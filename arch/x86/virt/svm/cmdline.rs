// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD SVM-SEV command line parsing support
 *
 * Copyright (C) 2023 - 2024 Advanced Micro Devices, Inc.
 *
 * Author: Michael Roth <michael.roth@amd.com>
 */

use core::ffi::{c_char, c_int};

// The following symbols are supplied by the surrounding kernel sources.
#[repr(C)]
pub struct sev_config {
    pub debug: bool,
}

extern "C" {
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn cpu_feature_enabled(feature: c_int) -> bool;
    fn setup_clear_cpu_cap(feature: c_int);
    fn cc_platform_clear(attr: c_int);
    fn pr_info(format: *const c_char, ...);
}

// Build-time kernel constants supplied by other translation units.
extern "C" {
    static X86_FEATURE_HYPERVISOR: c_int;
    static X86_FEATURE_SEV_SNP: c_int;
    static CC_ATTR_HOST_SEV_SNP: c_int;
}

#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut sev_cfg: sev_config = sev_config { debug: false };

unsafe fn init_sev_config(mut str_: *mut c_char) -> c_int {
    let delim = b",\0".as_ptr() as *const c_char;
    let debug = b"debug\0".as_ptr() as *const c_char;
    let nosnp = b"nosnp\0".as_ptr() as *const c_char;
    let format = b"SEV command-line option '%s' was not recognized\n\0".as_ptr()
        as *const c_char;

    loop {
        let s = strsep(&mut str_, delim);
        if s.is_null() {
            break;
        }

        if strcmp(s, debug) == 0 {
            sev_cfg.debug = true;
            continue;
        }

        if strcmp(s, nosnp) == 0 {
            if !cpu_feature_enabled(X86_FEATURE_HYPERVISOR) {
                setup_clear_cpu_cap(X86_FEATURE_SEV_SNP);
                cc_platform_clear(CC_ATTR_HOST_SEV_SNP);
                continue;
            } else {
                // Equivalent to the C goto warn.
            }
        }

        pr_info(format, s);
    }

    1
}

// __setup("sev=", init_sev_config); registers the command-line parser with
// the kernel's init-time setup framework.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
