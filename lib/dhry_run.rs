// SPDX-License-Identifier: GPL-2.0-only
/*
 * Dhrystone benchmark test module
 *
 * Copyright (C) 2022 Glider bv
 */

// Dependencies supplied by the surrounding kernel/module implementation:
// dhry, kernel parameter helpers, CPU helpers, logging, module metadata, and
// the kernel state/error-pointer definitions.

const DHRY_VAX: i32 = 1757;

extern "C" {
    fn dhry(iterations: i32) -> i32;
    fn get_cpu() -> u32;
    fn put_cpu();
    fn param_set_bool(val: *const core::ffi::c_char, kp: *const kernel_param) -> i32;
    fn system_state_value() -> i32;
    fn pr_info(format: *const core::ffi::c_char, ...);
    fn pr_err(format: *const core::ffi::c_char, ...);
    fn err_ptr(value: i32) -> *mut core::ffi::c_void;
}

#[repr(C)]
pub struct kernel_param {
    _private: [u8; 0],
}

static mut dhry_run: bool = false;
static mut iterations: i32 = -1;

// module_param_cb(run, &run_ops, &dhry_run, 0200);
// MODULE_PARM_DESC(run, "Run the test (default: false)");
// module_param(iterations, int, 0644);
// MODULE_PARM_DESC(iterations,
//                 "Number of iterations through the benchmark (default: auto)");

unsafe fn dhry_benchmark() {
    let cpu: u32 = get_cpu();
    let mut i: i32;
    let n: i32;

    if iterations > 0 {
        n = dhry(iterations);
        goto_report(cpu, n);
        return;
    }

    i = DHRY_VAX;
    loop {
        n = dhry(i);
        if n != -11 {
            break;
        }
        i = i.wrapping_shl(1);
        if !(i > 0) {
            break;
        }
    }

    goto_report(cpu, n);
}

unsafe fn goto_report(cpu: u32, n: i32) {
    put_cpu();
    if n >= 0 {
        pr_info(
            b"CPU%u: Dhrystones per Second: %d (%d DMIPS)\0".as_ptr() as *const core::ffi::c_char,
            cpu,
            n,
            n / DHRY_VAX,
        );
    } else if n == -11 {
        pr_err(b"Please increase the number of iterations\n\0".as_ptr() as *const core::ffi::c_char);
    } else {
        pr_err(
            b"Dhrystone benchmark failed error %pe\n\0".as_ptr() as *const core::ffi::c_char,
            err_ptr(n),
        );
    }
}

unsafe fn dhry_run_set(val: *const core::ffi::c_char, kp: *const kernel_param) -> i32 {
    let ret: i32;

    if !val.is_null() {
        ret = param_set_bool(val, kp);
        if ret != 0 {
            return ret;
        }
    } else {
        dhry_run = true;
    }

    if dhry_run && system_state_value() == 1 {
        dhry_benchmark();
    }

    0
}

unsafe fn dhry_init() -> i32 {
    if dhry_run {
        dhry_benchmark();
    }

    0
}

// module_init(dhry_init);
// MODULE_AUTHOR("Geert Uytterhoeven <geert+renesas@glider.be>");
// MODULE_DESCRIPTION("Dhrystone benchmark test module");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
