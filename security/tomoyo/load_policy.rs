// SPDX-License-Identifier: GPL-2.0
/*
 * security/tomoyo/load_policy.c
 *
 * Copyright (C) 2005-2011  NTT DATA CORPORATION
 */

// Dependency intent from C: #include "common.h"

use core::ffi::{c_char, c_int};
use core::ptr;

// C conditional intent: this file's contents are excluded when
// CONFIG_SECURITY_TOMOYO_OMIT_USERSPACE_LOADER is defined.

extern "C" {
    static mut CONFIG_SECURITY_TOMOYO_POLICY_LOADER: *const c_char;
    static mut CONFIG_SECURITY_TOMOYO_ACTIVATION_TRIGGER: *const c_char;
    static mut LOOKUP_FOLLOW: c_int;
    static mut UMH_WAIT_PROC: c_int;
    static mut tomoyo_policy_loaded: bool;

    fn kern_path(name: *const c_char, flags: c_int, path: *mut path) -> c_int;
    fn path_put(path: *mut path);
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn call_usermodehelper(
        path: *mut c_char,
        argv: *mut *mut c_char,
        envp: *mut *mut c_char,
        wait: c_int,
    ) -> c_int;
    fn tomoyo_check_profile();
    fn pr_info(fmt: *const c_char, ...);
}

// External kernel type from common/kernel headers.
#[repr(C)]
pub struct path {
    _data: [u8; 0],
}

/*
 * Path to the policy loader. (default = CONFIG_SECURITY_TOMOYO_POLICY_LOADER)
 */
static mut tomoyo_loader: *const c_char = ptr::null();

/**
 * tomoyo_loader_setup - Set policy loader.
 *
 * @str: Program to use as a policy loader (e.g. /sbin/tomoyo-init ).
 *
 * Returns 0.
 */
// C attribute intent: __init.
unsafe extern "C" fn tomoyo_loader_setup(str_: *mut c_char) -> c_int {
    tomoyo_loader = str_ as *const c_char;
    return 1;
}

// C setup hook intent: __setup("TOMOYO_loader=", tomoyo_loader_setup);

/**
 * tomoyo_policy_loader_exists - Check whether /sbin/tomoyo-init exists.
 *
 * Returns true if /sbin/tomoyo-init exists, false otherwise.
 */
unsafe fn tomoyo_policy_loader_exists() -> bool {
    let mut path: path = core::mem::zeroed();

    if tomoyo_loader.is_null() {
        tomoyo_loader = CONFIG_SECURITY_TOMOYO_POLICY_LOADER;
    }
    if kern_path(tomoyo_loader, LOOKUP_FOLLOW, &mut path) != 0 {
        pr_info(
            b"Not activating Mandatory Access Control as %s does not exist.\n\0".as_ptr()
                as *const c_char,
            tomoyo_loader,
        );
        return false;
    }
    path_put(&mut path);
    return true;
}

/*
 * Path to the trigger. (default = CONFIG_SECURITY_TOMOYO_ACTIVATION_TRIGGER)
 */
static mut tomoyo_trigger: *const c_char = ptr::null();

/**
 * tomoyo_trigger_setup - Set trigger for activation.
 *
 * @str: Program to use as an activation trigger (e.g. /sbin/init ).
 *
 * Returns 0.
 */
// C attribute intent: __init.
unsafe extern "C" fn tomoyo_trigger_setup(str_: *mut c_char) -> c_int {
    tomoyo_trigger = str_ as *const c_char;
    return 1;
}

// C setup hook intent: __setup("TOMOYO_trigger=", tomoyo_trigger_setup);

/**
 * tomoyo_load_policy - Run external policy loader to load policy.
 *
 * @filename: The program about to start.
 *
 * This function checks whether @filename is /sbin/init , and if so
 * invoke /sbin/tomoyo-init and wait for the termination of /sbin/tomoyo-init
 * and then continues invocation of /sbin/init.
 * /sbin/tomoyo-init reads policy files in /etc/tomoyo/ directory and
 * writes to /sys/kernel/security/tomoyo/ interfaces.
 *
 * Returns nothing.
 */
#[no_mangle]
pub unsafe extern "C" fn tomoyo_load_policy(filename: *const c_char) {
    static mut done: bool = false;
    let mut argv: [*mut c_char; 2] = [ptr::null_mut(); 2];
    let mut envp: [*mut c_char; 3] = [ptr::null_mut(); 3];

    if tomoyo_policy_loaded || done {
        return;
    }
    if tomoyo_trigger.is_null() {
        tomoyo_trigger = CONFIG_SECURITY_TOMOYO_ACTIVATION_TRIGGER;
    }
    if strcmp(filename, tomoyo_trigger) != 0 {
        return;
    }
    if !tomoyo_policy_loader_exists() {
        return;
    }
    done = true;
    pr_info(
        b"Calling %s to load policy. Please wait.\n\0".as_ptr() as *const c_char,
        tomoyo_loader,
    );
    argv[0] = tomoyo_loader as *mut c_char;
    argv[1] = ptr::null_mut();
    envp[0] = b"HOME=/\0".as_ptr() as *mut c_char;
    envp[1] = b"PATH=/sbin:/bin:/usr/sbin:/usr/bin\0".as_ptr() as *mut c_char;
    envp[2] = ptr::null_mut();
    call_usermodehelper(argv[0], argv.as_mut_ptr(), envp.as_mut_ptr(), UMH_WAIT_PROC);
    tomoyo_check_profile();
}



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
