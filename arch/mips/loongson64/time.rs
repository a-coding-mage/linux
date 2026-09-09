// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007 Lemote, Inc. & Institute of Computing Technology
 * Author: Fuxin Zhang, zhangfx@lemote.com
 *
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct LoongsonSysconf {
    pub fw_interface: i32,
}

extern "C" {
    static mut loongson_sysconf: LoongsonSysconf;
    static mut cpu_clock_freq: u64;
    static mut mips_hpt_frequency: u64;

    fn of_clk_init(np: *mut core::ffi::c_void);
    fn of_get_cpu_node(cpu: i32, node: *mut core::ffi::c_void) -> *mut device_node;
    fn of_clk_get(np: *mut device_node, index: i32) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> u64;
    fn clk_put(clk: *mut clk);
    fn setup_hpet_timer();
    fn pr_err(format: *const core::ffi::c_char, ...);
}

pub const LOONGSON_DTB: i32 = 1;

#[inline]
unsafe fn is_err(ptr: *mut clk) -> bool {
    (ptr as isize) >= -4095 && (ptr as isize) < 0
}

#[inline]
unsafe fn ptr_err(ptr: *mut clk) -> isize {
    ptr as isize
}

pub unsafe extern "C" fn plat_time_init() {
    let mut clk: *mut clk;
    let mut np: *mut device_node;

    if loongson_sysconf.fw_interface == LOONGSON_DTB {
        of_clk_init(core::ptr::null_mut());

        np = of_get_cpu_node(0, core::ptr::null_mut());
        if np.is_null() {
            pr_err(b"Failed to get CPU node\0".as_ptr() as *const core::ffi::c_char);
            return;
        }

        clk = of_clk_get(np, 0);
        if is_err(clk) {
            pr_err(
                b"Failed to get CPU clock: %ld\n\0".as_ptr() as *const core::ffi::c_char,
                ptr_err(clk),
            );
            return;
        }

        cpu_clock_freq = clk_get_rate(clk);
        clk_put(clk);
    }

    /* setup mips r4k timer */
    mips_hpt_frequency = cpu_clock_freq / 2;

    // CONFIG_RS780_HPET controls this build-time conditional.
    #[cfg(CONFIG_RS780_HPET)]
    setup_hpet_timer();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
