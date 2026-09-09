/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Ralink SoC register definitions
 *
 *  Copyright (C) 2013 John Crispin <john@phrozen.org>
 *  Copyright (C) 2008-2010 Gabor Juhos <juhosg@openwrt.org>
 *  Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 */

// C header guard: _RALINK_REGS_H_
// Dependency: <linux/io.h>

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ralink_soc_type {
    RALINK_UNKNOWN = 0,
    RT2880_SOC,
    RT3883_SOC,
    RT305X_SOC_RT3050,
    RT305X_SOC_RT3052,
    RT305X_SOC_RT3350,
    RT305X_SOC_RT3352,
    RT305X_SOC_RT5350,
    MT762X_SOC_MT7620A,
    MT762X_SOC_MT7620N,
    MT762X_SOC_MT7621AT,
    MT762X_SOC_MT7628AN,
    MT762X_SOC_MT7688,
}

unsafe extern "C" {
    pub static mut ralink_soc: ralink_soc_type;

    pub static mut rt_sysc_membase: *mut core::ffi::c_void;
    pub static mut rt_memc_membase: *mut core::ffi::c_void;

    // Dependency declarations corresponding to <linux/io.h>.
    pub fn __raw_writel(value: u32, address: *mut core::ffi::c_void);
    pub fn __raw_readl(address: *const core::ffi::c_void) -> u32;
}

#[inline]
pub unsafe fn rt_sysc_w32(val: u32, reg: u32) {
    __raw_writel(
        val,
        (rt_sysc_membase as *mut u8).add(reg as usize) as *mut core::ffi::c_void,
    );
}

#[inline]
pub unsafe fn rt_sysc_r32(reg: u32) -> u32 {
    __raw_readl(
        (rt_sysc_membase as *mut u8).add(reg as usize) as *const core::ffi::c_void,
    )
}

#[inline]
pub unsafe fn rt_sysc_m32(clr: u32, set: u32, reg: u32) {
    let val = rt_sysc_r32(reg) & !clr;

    __raw_writel(
        val | set,
        (rt_sysc_membase as *mut u8).add(reg as usize) as *mut core::ffi::c_void,
    );
}

#[inline]
pub unsafe fn rt_memc_w32(val: u32, reg: u32) {
    __raw_writel(
        val,
        (rt_memc_membase as *mut u8).add(reg as usize) as *mut core::ffi::c_void,
    );
}

#[inline]
pub unsafe fn rt_memc_r32(reg: u32) -> u32 {
    __raw_readl(
        (rt_memc_membase as *mut u8).add(reg as usize) as *const core::ffi::c_void,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
