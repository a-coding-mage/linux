// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2009 Gabor Juhos <juhosg@openwrt.org>
 *  Copyright (C) 2010 Joonas Lahtinen <joonas.lahtinen@gmail.com>
 *  Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Types and symbols supplied by the surrounding kernel headers.
#[repr(C)]
pub struct ralink_soc_info {
    pub sys_type: *const c_char,
}

pub type ralink_soc_type = c_int;

unsafe extern "C" {
    static mut fw_arg0: c_uint;
    static mut fw_arg1: c_uint;
    static mut fw_arg2: c_uint;
    static mut fw_arg3: c_uint;
    static mut arcs_cmdline: [c_char; 1024];

    fn prom_soc_init(info: *mut ralink_soc_info);
    fn strlcat(dst: *mut c_char, src: *const c_char, size: usize) -> usize;
    fn pr_debug(format: *const c_char, ...);
    fn pr_info(format: *const c_char, ...);
    fn KSEG1ADDR(address: c_uint) -> *mut c_void;
    fn CPHYSADDR(address: *const c_char) -> c_uint;
}

pub static mut soc_info: ralink_soc_info = ralink_soc_info {
    sys_type: core::ptr::null(),
};

pub static mut ralink_soc: ralink_soc_type = 0;

#[inline]
pub unsafe fn get_system_type() -> *const c_char {
    unsafe { soc_info.sys_type }
}

unsafe fn prom_init_cmdline() {
    let argc: c_int;
    let argv: *mut *mut c_char;
    let mut i: c_int;

    unsafe {
        pr_debug(
            b"prom: fw_arg0=%08x fw_arg1=%08x fw_arg2=%08x fw_arg3=%08x\0".as_ptr() as *const c_char,
            fw_arg0,
            fw_arg1,
            fw_arg2,
            fw_arg3,
        );

        argc = fw_arg0 as c_int;
        argv = KSEG1ADDR(fw_arg1) as *mut *mut c_char;

        if argv.is_null() {
            pr_debug(
                b"argv=%p is invalid, skipping\n\0".as_ptr() as *const c_char,
                argv,
            );
            return;
        }

        i = 0;
        while i < argc {
            let p = KSEG1ADDR(*argv.add(i as usize) as c_uint) as *mut c_char;

            if CPHYSADDR(p) != 0 && *p != 0 {
                pr_debug(
                    b"argv[%d]: %s\n\0".as_ptr() as *const c_char,
                    i,
                    p,
                );
                strlcat(arcs_cmdline.as_mut_ptr(), b" \0".as_ptr() as *const c_char, 1024);
                strlcat(arcs_cmdline.as_mut_ptr(), p, 1024);
            }
            i += 1;
        }
    }
}

pub unsafe fn prom_init() {
    unsafe {
        prom_soc_init(&raw mut soc_info);
        pr_info(
            b"SoC Type: %s\n\0".as_ptr() as *const c_char,
            get_system_type(),
        );
        prom_init_cmdline();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
