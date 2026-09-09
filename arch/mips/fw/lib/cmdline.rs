/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2012 MIPS Technologies, Inc.  All rights reserved.
 */

use core::ffi::{c_char, c_int, c_ulong};

// External declarations supplied by the included kernel and firmware headers.
unsafe extern "C" {
    static mut fw_arg0: c_ulong;
    static mut fw_arg1: c_ulong;
    static mut fw_arg2: c_ulong;
    static mut arcs_cmdline: c_char;

    fn strlcat(dst: *mut c_char, src: *const c_char, size: usize) -> usize;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn kstrtoul(s: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int;
    fn fw_argv(index: c_int) -> *mut c_char;
    fn fw_envp(index: c_int) -> *mut c_char;
}

type c_uint = u32;

// These values are supplied by the target headers/build configuration.
const CKSEG0: c_ulong = 0;
const COMMAND_LINE_SIZE: usize = 0;

#[no_mangle]
pub static mut fw_argc: c_int = 0;
#[no_mangle]
pub static mut _fw_argv: *mut c_int = core::ptr::null_mut();
#[no_mangle]
pub static mut _fw_envp: *mut c_int = core::ptr::null_mut();

#[cfg(not(CONFIG_HAVE_PLAT_FW_INIT_CMDLINE))]
#[no_mangle]
pub unsafe extern "C" fn fw_init_cmdline() {
    let mut i: c_int;

    /* Validate command line parameters. */
    if fw_arg0 >= CKSEG0 || fw_arg1 < CKSEG0 {
        fw_argc = 0;
        _fw_argv = core::ptr::null_mut();
    } else {
        fw_argc = (fw_arg0 & 0x0000ffff) as c_int;
        _fw_argv = fw_arg1 as *mut c_int;
    }

    /* Validate environment pointer. */
    if fw_arg2 < CKSEG0 {
        _fw_envp = core::ptr::null_mut();
    } else {
        _fw_envp = fw_arg2 as *mut c_int;
    }

    i = 1;
    while i < fw_argc {
        strlcat(&mut arcs_cmdline as *mut c_char, fw_argv(i), COMMAND_LINE_SIZE);
        if i < fw_argc - 1 {
            strlcat(
                &mut arcs_cmdline as *mut c_char,
                b" \0".as_ptr() as *const c_char,
                COMMAND_LINE_SIZE,
            );
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn fw_getcmdline() -> *mut c_char {
    &mut arcs_cmdline as *mut c_char
}

#[no_mangle]
pub unsafe extern "C" fn fw_getenv(envname: *mut c_char) -> *mut c_char {
    let mut result: *mut c_char = core::ptr::null_mut();

    if !_fw_envp.is_null() && !fw_envp(0).is_null() {
        /*
         * Return a pointer to the given environment variable.
         * YAMON uses "name", "value" pairs, while U-Boot uses
         * "name=value".
         */
        let mut i: usize;
        let yamon: bool;
        let mut index: c_int = 0;

        yamon = strchr(fw_envp(index), '=' as c_int).is_null();
        i = strlen(envname);

        while !fw_envp(index).is_null() {
            if strncmp(envname, fw_envp(index), i) == 0 {
                if yamon {
                    result = fw_envp(index + 1);
                    break;
                } else if *fw_envp(index).add(i) == '=' as c_char {
                    result = fw_envp(index).add(i + 1);
                    break;
                }
            }

            /* Increment array index. */
            if yamon {
                index += 2;
            } else {
                index += 1;
            }
        }
    }

    result
}

#[no_mangle]
pub unsafe extern "C" fn fw_getenvl(envname: *mut c_char) -> c_ulong {
    let mut envl: c_ulong = 0;
    let str_: *mut c_char;
    let mut tmp: c_int;

    str_ = fw_getenv(envname);
    if !str_.is_null() {
        tmp = kstrtoul(str_, 0, &mut envl);
        if tmp != 0 {
            envl = 0;
        }
    }

    envl
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
