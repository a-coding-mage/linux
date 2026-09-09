// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Device Tree support for Rockchip SoCs
 *
 * Copyright (c) 2013 MundoReader S.L.
 * Author: Heiko Stuebner <heiko@sntech.de>
 */

// Dependency intent from the original Linux headers:
// linux/kernel.h, linux/init.h, linux/io.h, linux/of.h, linux/of_clk.h,
// linux/clocksource.h, asm/mach/arch.h, asm/mach/map.h, "core.h", "pm.h"

use core::ffi::{c_char, c_void};

const RK3288_TIMER6_7_PHYS: usize = 0xff810000;
const SZ_16K: usize = 16 * 1024;

extern "C" {
    fn of_machine_is_compatible(compat: *const c_char) -> bool;
    fn ioremap(phys_addr: usize, size: usize) -> *mut c_void;
    fn writel(value: u32, address: *mut c_void);
    fn dsb();
    fn iounmap(address: *mut c_void);
    fn of_clk_init(clk_init: *const c_void);
    fn timer_probe();
    fn rockchip_suspend_init();
    fn pr_err(format: *const c_char, ...);
}

unsafe fn rockchip_timer_init() {
    if of_machine_is_compatible(b"rockchip,rk3288\0".as_ptr() as *const c_char) {
        let mut reg_base: *mut c_void;

        /*
         * Most/all uboot versions for rk3288 don't enable timer7
         * which is needed for the architected timer to work.
         * So make sure it is running during early boot.
         */
        reg_base = ioremap(RK3288_TIMER6_7_PHYS, SZ_16K);
        if !reg_base.is_null() {
            writel(0, reg_base.add(0x30));
            writel(0xffffffff, reg_base.add(0x20));
            writel(0xffffffff, reg_base.add(0x24));
            writel(1, reg_base.add(0x30));
            dsb();
            iounmap(reg_base);
        } else {
            pr_err(b"rockchip: could not map timer7 registers\n\0".as_ptr() as *const c_char);
        }
    }

    of_clk_init(core::ptr::null());
    timer_probe();
}

unsafe fn rockchip_dt_init() {
    rockchip_suspend_init();
}

static ROCKCHIP_BOARD_DT_COMPAT: [*const c_char; 8] = [
    b"rockchip,rk2928\0".as_ptr() as *const c_char,
    b"rockchip,rk3066a\0".as_ptr() as *const c_char,
    b"rockchip,rk3066b\0".as_ptr() as *const c_char,
    b"rockchip,rk3188\0".as_ptr() as *const c_char,
    b"rockchip,rk3228\0".as_ptr() as *const c_char,
    b"rockchip,rk3288\0".as_ptr() as *const c_char,
    b"rockchip,rv1108\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// Direct translation of the DT_MACHINE_START(ROCKCHIP_DT,
// "Rockchip (Device Tree)") machine descriptor.
#[repr(C)]
pub struct MachineDesc {
    pub name: *const c_char,
    pub l2c_aux_val: usize,
    pub l2c_aux_mask: usize,
    pub init_time: unsafe fn(),
    pub dt_compat: *const *const c_char,
    pub init_machine: unsafe fn(),
}

#[no_mangle]
pub static ROCKCHIP_DT: MachineDesc = MachineDesc {
    name: b"Rockchip (Device Tree)\0".as_ptr() as *const c_char,
    l2c_aux_val: 0,
    l2c_aux_mask: !0,
    init_time: rockchip_timer_init,
    dt_compat: ROCKCHIP_BOARD_DT_COMPAT.as_ptr(),
    init_machine: rockchip_dt_init,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
