// SPDX-License-Identifier: GPL-2.0
/* Copyright IBM Corp. 2020 */

// Dependencies supplied by <linux/compiler.h>, <asm/timex.h>, and "vdso.h".

use core::ffi::c_void;

#[repr(C)]
pub union tod_clock {
    pub pf: u32,
}

unsafe extern "C" {
    fn store_tod_clock_ext(clk: *mut tod_clock);
}

#[no_mangle]
pub unsafe extern "C" fn __s390_vdso_getcpu(
    cpu: *mut u32,
    node: *mut u32,
    _unused: *mut c_void,
) -> i32 {
    let mut clk = tod_clock { pf: 0 };

    /* CPU number is stored in the programmable field of the TOD clock */
    unsafe {
        store_tod_clock_ext(&mut clk);
    }
    if !cpu.is_null() {
        unsafe {
            *cpu = clk.pf;
        }
    }
    /* NUMA node is always zero */
    if !node.is_null() {
        unsafe {
            *node = 0;
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
