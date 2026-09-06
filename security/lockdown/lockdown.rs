// SPDX-License-Identifier: GPL-2.0
/* Lock down the kernel
 *
 * Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public Licence
 * as published by the Free Software Foundation; either version
 * 2 of the Licence, or (at your option) any later version.
 */

/* Dependencies from:
 * #include <linux/security.h>
 * #include <linux/export.h>
 * #include <linux/lsm_hooks.h>
 * #include <uapi/linux/lsm.h>
 */

use core::ffi::{c_char, c_int, c_void};

type ssize_t = isize;
type size_t = usize;
type loff_t = i64;

const EPERM: c_int = 1;
const EINVAL: c_int = 22;
const LOCKDOWN_NONE: lockdown_reason = 0;
const LOCKDOWN_INTEGRITY_MAX: lockdown_reason = 1;
const LOCKDOWN_CONFIDENTIALITY_MAX: lockdown_reason = 2;
const LSM_ID_LOCKDOWN: c_int = 0;

type lockdown_reason = c_int;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub comm: *const c_char,
}

#[repr(C)]
pub struct security_hook_list {
    pub locked_down: Option<unsafe extern "C" fn(lockdown_reason) -> c_int>,
}

#[repr(C)]
pub struct lsm_id {
    pub name: *const c_char,
    pub id: c_int,
}

#[repr(C)]
pub struct file_operations {
    pub read: Option<
        unsafe extern "C" fn(
            *mut file,
            *mut c_char,
            size_t,
            *mut loff_t,
        ) -> ssize_t,
    >,
    pub write: Option<
        unsafe extern "C" fn(
            *mut file,
            *const c_char,
            size_t,
            *mut loff_t,
        ) -> ssize_t,
    >,
}

#[repr(C)]
pub struct lsm_info {
    pub id: *const lsm_id,
    pub init: Option<unsafe extern "C" fn() -> c_int>,
    pub initcall_core: Option<unsafe extern "C" fn() -> c_int>,
}

unsafe extern "C" {
    static lockdown_reasons: [*const c_char; 0];
    static current: *mut task_struct;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn pr_notice(format: *const c_char, ...);
    fn pr_notice_ratelimited(format: *const c_char, ...);
    fn memdup_user_nul(src: *const c_char, len: size_t) -> *mut c_char;
    fn kfree(ptr: *const c_void);
    fn simple_read_from_buffer(
        to: *mut c_char,
        count: size_t,
        ppos: *mut loff_t,
        from: *const c_void,
        available: size_t,
    ) -> ssize_t;
    fn security_add_hooks(
        hooks: *mut security_hook_list,
        count: size_t,
        lsmid: *const lsm_id,
    );
    fn securityfs_create_file(
        name: *const c_char,
        mode: u16,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR_OR_ZERO(ptr: *const c_void) -> c_int;
    fn WARN(condition: bool, message: *const c_char) -> bool;
}

static mut kernel_locked_down: lockdown_reason = 0;

static lockdown_levels: [lockdown_reason; 3] = [
    LOCKDOWN_NONE,
    LOCKDOWN_INTEGRITY_MAX,
    LOCKDOWN_CONFIDENTIALITY_MAX,
];

/*
 * Put the kernel into lock-down mode.
 */
unsafe fn lock_kernel_down(where_: *const c_char, level: lockdown_reason) -> c_int {
    if unsafe { kernel_locked_down >= level } {
        return -EPERM;
    }

    unsafe {
        kernel_locked_down = level;
        pr_notice(
            c"Kernel is locked down from %s; see man kernel_lockdown.7\n".as_ptr(),
            where_,
        );
    }
    0
}

unsafe fn lockdown_param(level: *mut c_char) -> c_int {
    if level.is_null() {
        return -EINVAL;
    }

    unsafe {
        if strcmp(level, c"integrity".as_ptr()) == 0 {
            lock_kernel_down(c"command line".as_ptr(), LOCKDOWN_INTEGRITY_MAX);
        } else if strcmp(level, c"confidentiality".as_ptr()) == 0 {
            lock_kernel_down(c"command line".as_ptr(), LOCKDOWN_CONFIDENTIALITY_MAX);
        } else {
            return -EINVAL;
        }
    }

    0
}

/* early_param("lockdown", lockdown_param); */

/**
 * lockdown_is_locked_down - Find out if the kernel is locked down
 * @what: Tag to use in notice generated if lockdown is in effect
 */
unsafe fn lockdown_is_locked_down(what: lockdown_reason) -> c_int {
    if unsafe {
        WARN(
            what >= LOCKDOWN_CONFIDENTIALITY_MAX,
            c"Invalid lockdown reason".as_ptr(),
        )
    } {
        return -EPERM;
    }

    if unsafe { kernel_locked_down >= what } {
        unsafe {
            let reason = *lockdown_reasons.as_ptr().add(what as usize);
            if !reason.is_null() {
                pr_notice_ratelimited(
                    c"Lockdown: %s: %s is restricted; see man kernel_lockdown.7\n".as_ptr(),
                    (*current).comm,
                    reason,
                );
            }
        }
        return -EPERM;
    }

    0
}

static mut lockdown_hooks: [security_hook_list; 1] = [security_hook_list {
    locked_down: Some(lockdown_is_locked_down),
}];

static lockdown_lsmid: lsm_id = lsm_id {
    name: c"lockdown".as_ptr(),
    id: LSM_ID_LOCKDOWN,
};

unsafe fn lockdown_lsm_init() -> c_int {
    /* CONFIG_LOCK_DOWN_KERNEL_FORCE_INTEGRITY:
     * lock_kernel_down("Kernel configuration", LOCKDOWN_INTEGRITY_MAX);
     *
     * CONFIG_LOCK_DOWN_KERNEL_FORCE_CONFIDENTIALITY:
     * lock_kernel_down("Kernel configuration", LOCKDOWN_CONFIDENTIALITY_MAX);
     */
    unsafe {
        security_add_hooks(
            core::ptr::addr_of_mut!(lockdown_hooks) as *mut security_hook_list,
            lockdown_hooks.len(),
            &lockdown_lsmid,
        );
    }
    0
}

unsafe fn lockdown_read(
    _filp: *mut file,
    buf: *mut c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let mut temp = [0 as c_char; 80];
    let mut i: c_int;
    let mut offset: c_int = 0;

    i = 0;
    while (i as usize) < lockdown_levels.len() {
        let level = lockdown_levels[i as usize];

        unsafe {
            let label = *lockdown_reasons.as_ptr().add(level as usize);
            if !label.is_null() {
                if kernel_locked_down == level {
                    offset += sprintf(
                        temp.as_mut_ptr().add(offset as usize),
                        c"[%s] ".as_ptr(),
                        label,
                    );
                } else {
                    offset += sprintf(
                        temp.as_mut_ptr().add(offset as usize),
                        c"%s ".as_ptr(),
                        label,
                    );
                }
            }
        }
        i += 1;
    }

    /* Convert the last space to a newline if needed. */
    if offset > 0 {
        temp[(offset - 1) as usize] = b'\n' as c_char;
    }

    unsafe {
        simple_read_from_buffer(
            buf,
            count,
            ppos,
            temp.as_ptr() as *const c_void,
            strlen(temp.as_ptr()),
        )
    }
}

unsafe fn lockdown_write(
    _file: *mut file,
    buf: *const c_char,
    n: size_t,
    _ppos: *mut loff_t,
) -> ssize_t {
    let state: *mut c_char;
    let mut i: c_int;
    let mut len: c_int;
    let mut err: c_int = -EINVAL;

    unsafe {
        state = memdup_user_nul(buf, n);
        if IS_ERR(state as *const c_void) {
            return PTR_ERR(state as *const c_void);
        }

        len = strlen(state) as c_int;
        if len != 0 && *state.add((len - 1) as usize) == b'\n' as c_char {
            *state.add((len - 1) as usize) = b'\0' as c_char;
            len -= 1;
        }

        i = 0;
        while (i as usize) < lockdown_levels.len() {
            let level = lockdown_levels[i as usize];
            let label = *lockdown_reasons.as_ptr().add(level as usize);

            if !label.is_null() && strcmp(state, label) == 0 {
                err = lock_kernel_down(c"securityfs".as_ptr(), level);
            }
            i += 1;
        }

        kfree(state as *const c_void);
    }
    if err != 0 {
        err as ssize_t
    } else {
        n as ssize_t
    }
}

static lockdown_ops: file_operations = file_operations {
    read: Some(lockdown_read),
    write: Some(lockdown_write),
};

unsafe fn lockdown_secfs_init() -> c_int {
    let dentry: *mut dentry;

    unsafe {
        dentry = securityfs_create_file(
            c"lockdown".as_ptr(),
            0o644,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            &lockdown_ops,
        );
        PTR_ERR_OR_ZERO(dentry as *const c_void)
    }
}

/* CONFIG_SECURITY_LOCKDOWN_LSM_EARLY selects DEFINE_EARLY_LSM(lockdown);
 * otherwise DEFINE_LSM(lockdown) is used.
 */
static lockdown: lsm_info = lsm_info {
    id: &lockdown_lsmid,
    init: Some(lockdown_lsm_init),
    initcall_core: Some(lockdown_secfs_init),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
