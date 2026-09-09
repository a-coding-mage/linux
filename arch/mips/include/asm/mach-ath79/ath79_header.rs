/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Atheros AR71XX/AR724X/AR913X common definitions
 *
 * Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 *
 * Parts of this file are based on Atheros' 2.6.15 BSP
 */

use core::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath79_soc_type {
    ATH79_SOC_UNKNOWN,
    ATH79_SOC_AR7130,
    ATH79_SOC_AR7141,
    ATH79_SOC_AR7161,
    ATH79_SOC_AR7240,
    ATH79_SOC_AR7241,
    ATH79_SOC_AR7242,
    ATH79_SOC_AR9130,
    ATH79_SOC_AR9132,
    ATH79_SOC_AR9330,
    ATH79_SOC_AR9331,
    ATH79_SOC_AR9341,
    ATH79_SOC_AR9342,
    ATH79_SOC_AR9344,
    ATH79_SOC_QCA9533,
    ATH79_SOC_QCA9556,
    ATH79_SOC_QCA9558,
    ATH79_SOC_TP9343,
    ATH79_SOC_QCA956X,
}

extern "C" {
    pub static mut ath79_soc: ath79_soc_type;
    pub static mut ath79_soc_rev: u32;

    pub fn ath79_ddr_wb_flush(reg: u32);
    pub fn ath79_ddr_set_pci_windows();

    pub static mut ath79_pll_base: *mut c_void;
    pub static mut ath79_reset_base: *mut c_void;

    pub fn __raw_writel(value: u32, address: *mut c_void);
    pub fn __raw_readl(address: *mut c_void) -> u32;

    pub fn ath79_device_reset_set(mask: u32);
    pub fn ath79_device_reset_clear(mask: u32);
}

#[inline]
pub unsafe fn soc_is_ar71xx() -> i32 {
    (ath79_soc == ath79_soc_type::ATH79_SOC_AR7130
        || ath79_soc == ath79_soc_type::ATH79_SOC_AR7141
        || ath79_soc == ath79_soc_type::ATH79_SOC_AR7161) as i32
}

#[inline]
pub unsafe fn soc_is_ar724x() -> i32 {
    (ath79_soc == ath79_soc_type::ATH79_SOC_AR7240
        || ath79_soc == ath79_soc_type::ATH79_SOC_AR7241
        || ath79_soc == ath79_soc_type::ATH79_SOC_AR7242) as i32
}

#[inline] pub unsafe fn soc_is_ar7240() -> i32 { (ath79_soc == ath79_soc_type::ATH79_SOC_AR7240) as i32 }
#[inline] pub unsafe fn soc_is_ar7241() -> i32 { (ath79_soc == ath79_soc_type::ATH79_SOC_AR7241) as i32 }
#[inline] pub unsafe fn soc_is_ar7242() -> i32 { (ath79_soc == ath79_soc_type::ATH79_SOC_AR7242) as i32 }

#[inline]
pub unsafe fn soc_is_ar913x() -> i32 {
    (ath79_soc == ath79_soc_type::ATH79_SOC_AR9130
        || ath79_soc == ath79_soc_type::ATH79_SOC_AR9132) as i32
}

#[inline]
pub unsafe fn soc_is_ar933x() -> i32 {
    (ath79_soc == ath79_soc_type::ATH79_SOC_AR9330
        || ath79_soc == ath79_soc_type::ATH79_SOC_AR9331) as i32
}

#[inline] pub unsafe fn soc_is_ar9341() -> i32 { (ath79_soc == ath79_soc_type::ATH79_SOC_AR9341) as i32 }
#[inline] pub unsafe fn soc_is_ar9342() -> i32 { (ath79_soc == ath79_soc_type::ATH79_SOC_AR9342) as i32 }
#[inline] pub unsafe fn soc_is_ar9344() -> i32 { (ath79_soc == ath79_soc_type::ATH79_SOC_AR9344) as i32 }
#[inline] pub unsafe fn soc_is_ar934x() -> i32 { (soc_is_ar9341() != 0 || soc_is_ar9342() != 0 || soc_is_ar9344() != 0) as i32 }

#[inline] pub unsafe fn soc_is_qca9533() -> i32 { (ath79_soc == ath79_soc_type::ATH79_SOC_QCA9533) as i32 }
#[inline] pub unsafe fn soc_is_qca953x() -> i32 { soc_is_qca9533() }
#[inline] pub unsafe fn soc_is_qca9556() -> i32 { (ath79_soc == ath79_soc_type::ATH79_SOC_QCA9556) as i32 }
#[inline] pub unsafe fn soc_is_qca9558() -> i32 { (ath79_soc == ath79_soc_type::ATH79_SOC_QCA9558) as i32 }
#[inline] pub unsafe fn soc_is_qca955x() -> i32 { (soc_is_qca9556() != 0 || soc_is_qca9558() != 0) as i32 }
#[inline] pub unsafe fn soc_is_tp9343() -> i32 { (ath79_soc == ath79_soc_type::ATH79_SOC_TP9343) as i32 }
#[inline] pub unsafe fn soc_is_qca9561() -> i32 { (ath79_soc == ath79_soc_type::ATH79_SOC_QCA956X) as i32 }
#[inline] pub unsafe fn soc_is_qca9563() -> i32 { (ath79_soc == ath79_soc_type::ATH79_SOC_QCA956X) as i32 }
#[inline] pub unsafe fn soc_is_qca956x() -> i32 { (soc_is_qca9561() != 0 || soc_is_qca9563() != 0) as i32 }

#[inline]
pub unsafe fn ath79_pll_wr(reg: u32, val: u32) {
    __raw_writel(val, (ath79_pll_base as *mut u8).add(reg as usize) as *mut c_void);
}

#[inline]
pub unsafe fn ath79_pll_rr(reg: u32) -> u32 {
    __raw_readl((ath79_pll_base as *mut u8).add(reg as usize) as *mut c_void)
}

#[inline]
pub unsafe fn ath79_reset_wr(reg: u32, val: u32) {
    let address = (ath79_reset_base as *mut u8).add(reg as usize) as *mut c_void;
    __raw_writel(val, address);
    let _ = __raw_readl(address); /* flush */
}

#[inline]
pub unsafe fn ath79_reset_rr(reg: u32) -> u32 {
    __raw_readl((ath79_reset_base as *mut u8).add(reg as usize) as *mut c_void)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
