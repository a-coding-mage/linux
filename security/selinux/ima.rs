// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2021 Microsoft Corporation
 *
 * Author: Lakshmi Ramasubramanian (nramas@linux.microsoft.com)
 *
 * Measure critical data structures maintained by SELinux
 * using IMA subsystem.
 */

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut selinux_state: selinux_state;
    static selinux_policycap_names: [*const c_char; __POLICYDB_CAP_MAX as usize];

    fn strlen(s: *const c_char) -> usize;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_char;
    fn kfree(ptr: *const c_void);
    fn vfree(addr: *const c_void);

    fn seq_buf_init(s: *mut seq_buf, buf: *mut c_char, size: usize);
    fn seq_buf_printf(s: *mut seq_buf, fmt: *const c_char, ...);
    fn seq_buf_has_overflowed(s: *const seq_buf) -> bool;

    fn selinux_initialized() -> c_int;
    fn enforcing_enabled() -> c_int;
    fn checkreqprot_get() -> c_int;
    fn security_read_state_kernel(policy: *mut *mut c_void, policy_len: *mut usize) -> c_int;

    fn ima_measure_critical_data(
        event_label: *const c_char,
        event_name: *const c_char,
        buf: *const c_void,
        buf_len: usize,
        hash: bool,
        func_data: *const c_void,
        func_data_len: usize,
    );

    fn lockdep_assert_held(lock: *const mutex);
    fn lockdep_assert_not_held(lock: *const mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn pr_err(fmt: *const c_char, ...);
    fn WARN_ON(condition: bool) -> c_int;
}

type c_uint = u32;

// Types and constants are supplied by the translated kernel dependencies.
type seq_buf = crate::seq_buf;
type selinux_state = crate::selinux_state;
type mutex = crate::mutex;

const GFP_KERNEL: c_uint = crate::GFP_KERNEL;
const __POLICYDB_CAP_MAX: c_int = crate::__POLICYDB_CAP_MAX;

static mut selinux_ima_config_len: c_int = 0; // __ro_after_init

/*
 * selinux_ima_config_len_init - Compute the configuration settings string length
 *
 * The string is fixed text plus one digit per setting, so its length
 * is known at boot.
 */
pub unsafe extern "C" fn selinux_ima_config_len_init() {
    let mut buf_len: c_int;
    let suffix_len: c_int;
    let mut i: c_int;

    buf_len = strlen(b"initialized=0;enforcing=0;checkreqprot=0;\0".as_ptr() as *const c_char)
        as c_int
        + 1;
    suffix_len = strlen(b"=0;\0".as_ptr() as *const c_char) as c_int;

    i = 0;
    while i < __POLICYDB_CAP_MAX {
        buf_len += strlen(selinux_policycap_names[i as usize]) as c_int + suffix_len;
        i += 1;
    }

    selinux_ima_config_len = buf_len;
}

/*
 * selinux_ima_collect_state - Read selinux configuration settings
 *
 * On success returns the configuration settings string.
 * On error, returns NULL.
 */
unsafe fn selinux_ima_collect_state() -> *mut c_char {
    let mut s: seq_buf = core::mem::zeroed();
    let buf: *mut c_char;
    let mut i: c_int;

    buf = kzalloc(selinux_ima_config_len as usize, GFP_KERNEL);
    if buf.is_null() {
        return core::ptr::null_mut();
    }

    seq_buf_init(&mut s, buf, selinux_ima_config_len as usize);

    seq_buf_printf(
        &mut s,
        b"initialized=%d;enforcing=%d;checkreqprot=%d;\0".as_ptr() as *const c_char,
        selinux_initialized(),
        enforcing_enabled(),
        checkreqprot_get(),
    );

    i = 0;
    while i < __POLICYDB_CAP_MAX {
        seq_buf_printf(
            &mut s,
            b"%s=%d;\0".as_ptr() as *const c_char,
            selinux_policycap_names[i as usize],
            selinux_state.policycap[i as usize],
        );
        i += 1;
    }

    WARN_ON(seq_buf_has_overflowed(&s));

    buf
}

/*
 * selinux_ima_measure_state_locked - Measure SELinux state and hash of policy
 */
pub unsafe extern "C" fn selinux_ima_measure_state_locked() {
    let mut state_str: *mut c_char = core::ptr::null_mut();
    let mut policy: *mut c_void = core::ptr::null_mut();
    let mut policy_len: usize = 0;
    let mut rc: c_int = 0;

    lockdep_assert_held(&selinux_state.policy_mutex);

    state_str = selinux_ima_collect_state();
    if state_str.is_null() {
        pr_err(
            b"SELinux: %s: failed to read state.\n\0".as_ptr() as *const c_char,
            b"selinux_ima_measure_state_locked\0".as_ptr() as *const c_char,
        );
        return;
    }

    ima_measure_critical_data(
        b"selinux\0".as_ptr() as *const c_char,
        b"selinux-state\0".as_ptr() as *const c_char,
        state_str as *const c_void,
        strlen(state_str as *const c_char),
        false,
        core::ptr::null(),
        0,
    );

    kfree(state_str as *const c_void);

    /*
     * Measure SELinux policy only after initialization is completed.
     */
    if selinux_initialized() == 0 {
        return;
    }

    rc = security_read_state_kernel(&mut policy, &mut policy_len);
    if rc != 0 {
        pr_err(
            b"SELinux: %s: failed to read policy %d.\n\0".as_ptr() as *const c_char,
            b"selinux_ima_measure_state_locked\0".as_ptr() as *const c_char,
            rc,
        );
        return;
    }

    ima_measure_critical_data(
        b"selinux\0".as_ptr() as *const c_char,
        b"selinux-policy-hash\0".as_ptr() as *const c_char,
        policy,
        policy_len,
        true,
        core::ptr::null(),
        0,
    );

    vfree(policy);
}

/*
 * selinux_ima_measure_state - Measure SELinux state and hash of policy
 */
pub unsafe extern "C" fn selinux_ima_measure_state() {
    lockdep_assert_not_held(&selinux_state.policy_mutex);

    mutex_lock(&mut selinux_state.policy_mutex);
    selinux_ima_measure_state_locked();
    mutex_unlock(&mut selinux_state.policy_mutex);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
