// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/xtensa/platform-iss/setup.c
 *
 * Platform specific initialization.
 *
 * Authors: Chris Zankel <chris@zankel.net>
 *          Joe Taylor <joe@tensilica.com>
 *
 * Copyright 2001 - 2005 Tensilica Inc.
 * Copyright 2017 Cadence Design Systems Inc.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// COMMAND_LINE_SIZE is supplied by the kernel build environment.
const COMMAND_LINE_SIZE: usize = 4096;

const NOTIFY_DONE: c_int = 0;
const SYS_OFF_MODE_RESTART: c_int = 0;
const SYS_OFF_MODE_POWER_OFF: c_int = 1;
const SYS_OFF_PRIO_PLATFORM: c_int = 0;

#[repr(C)]
pub struct sys_off_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call:
        Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}

unsafe extern "C" {
    fn simc_exit(code: c_int);
    fn cpu_reset();
    fn simc_argc() -> c_int;
    fn simc_argv_size() -> c_int;
    fn simc_argv(argv: *mut c_void);
    fn atomic_notifier_chain_register(
        list: *mut c_void,
        block: *mut notifier_block,
    ) -> c_int;
    fn register_sys_off_handler(
        mode: c_int,
        priority: c_int,
        callback: unsafe extern "C" fn(*mut sys_off_data) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    fn pr_info(format: *const c_char, ...);
    fn pr_err(format: *const c_char, ...);
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    static mut panic_notifier_list: c_void;
}

unsafe extern "C" fn iss_power_off(_unused: *mut sys_off_data) -> c_int {
    pr_info(b" ** Called platform_power_off() **\n\0".as_ptr() as *const c_char);
    simc_exit(0);
    NOTIFY_DONE
}

unsafe extern "C" fn iss_restart(_unused: *mut sys_off_data) -> c_int {
    /* Flush and reset the mmu, simulate a processor reset, and
     * jump to the reset vector. */
    cpu_reset();

    NOTIFY_DONE
}

unsafe extern "C" fn iss_panic_event(
    _this: *mut notifier_block,
    _event: c_ulong,
    _ptr: *mut c_void,
) -> c_int {
    simc_exit(1);
    NOTIFY_DONE
}

static mut iss_panic_block: notifier_block = notifier_block {
    notifier_call: Some(iss_panic_event),
};

pub unsafe extern "C" fn platform_setup(p_cmdline: *mut *mut c_char) {
    static mut argv: [*mut c_void; COMMAND_LINE_SIZE / core::mem::size_of::<*mut c_void>()] =
        [core::ptr::null_mut(); COMMAND_LINE_SIZE / core::mem::size_of::<*mut c_void>()];
    static mut cmdline: [c_char; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];
    let argc = simc_argc();
    let argv_size = simc_argv_size();

    if argc > 1 {
        if argv_size as usize > core::mem::size_of_val(&argv) {
            pr_err(
                b"%s: command line too long: argv_size = %d\n\0".as_ptr() as *const c_char,
                b"platform_setup\0".as_ptr(),
                argv_size,
            );
        } else {
            let mut i: c_int;

            cmdline[0] = 0;
            simc_argv(argv.as_mut_ptr() as *mut c_void);

            i = 1;
            while i < argc {
                if i > 1 {
                    strcat(cmdline.as_mut_ptr(), b" \0".as_ptr() as *const c_char);
                }
                strcat(
                    cmdline.as_mut_ptr(),
                    argv[i as usize] as *const c_char,
                );
                i += 1;
            }
            *p_cmdline = cmdline.as_mut_ptr();
        }
    }

    atomic_notifier_chain_register(
        &raw mut panic_notifier_list,
        &raw mut iss_panic_block,
    );
    register_sys_off_handler(
        SYS_OFF_MODE_RESTART,
        SYS_OFF_PRIO_PLATFORM,
        iss_restart,
        core::ptr::null_mut(),
    );
    register_sys_off_handler(
        SYS_OFF_MODE_POWER_OFF,
        SYS_OFF_PRIO_PLATFORM,
        iss_power_off,
        core::ptr::null_mut(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
