// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Atheros AR71XX/AR724X/AR913X common routines
 *
 *  Copyright (C) 2010-2011 Jaiganesh Narayanan <jnarayanan@atheros.com>
 *  Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 *  Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 *
 *  Parts of this file are based on Atheros' 2.6.15/2.6.31 BSP
 */

use core::ffi::c_void;

// Linux and platform headers provide these declarations and constants.
extern "C" {
    fn ioremap(phys_addr: usize, size: usize) -> *mut c_void;
    fn __raw_writel(value: u32, address: *mut c_void);
    fn __raw_readl(address: *mut c_void) -> u32;
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: usize);
    fn ath79_reset_rr(reg: u32) -> u32;
    fn ath79_reset_wr(reg: u32, value: u32);
    fn soc_is_ar913x() -> bool;
    fn soc_is_ar724x() -> bool;
    fn soc_is_ar933x() -> bool;
    fn soc_is_ar71xx() -> bool;
    fn soc_is_ar934x() -> bool;
    fn soc_is_qca953x() -> bool;
    fn soc_is_qca955x() -> bool;
    fn soc_is_qca956x() -> bool;
    fn soc_is_tp9343() -> bool;
    fn BUG();
    fn BUG_ON(condition: bool);
}

type ath79_soc_type = u32;

const AR71XX_DDR_CTRL_BASE: usize = 0;
const AR71XX_DDR_CTRL_SIZE: usize = 0;
const AR71XX_PCI_WIN0_OFFS: u32 = 0;
const AR71XX_PCI_WIN1_OFFS: u32 = 0;
const AR71XX_PCI_WIN2_OFFS: u32 = 0;
const AR71XX_PCI_WIN3_OFFS: u32 = 0;
const AR71XX_PCI_WIN4_OFFS: u32 = 0;
const AR71XX_PCI_WIN5_OFFS: u32 = 0;
const AR71XX_PCI_WIN6_OFFS: u32 = 0;
const AR71XX_PCI_WIN7_OFFS: u32 = 0;
const AR71XX_RESET_REG_RESET_MODULE: u32 = 0;
const AR724X_RESET_REG_RESET_MODULE: u32 = 0;
const AR913X_RESET_REG_RESET_MODULE: u32 = 0;
const AR933X_RESET_REG_RESET_MODULE: u32 = 0;
const AR934X_RESET_REG_RESET_MODULE: u32 = 0;
const QCA953X_RESET_REG_RESET_MODULE: u32 = 0;
const QCA955X_RESET_REG_RESET_MODULE: u32 = 0;
const QCA956X_RESET_REG_RESET_MODULE: u32 = 0;

static mut ath79_device_reset_lock: u8 = 0;

pub static mut ath79_cpu_freq: u32 = 0;
pub static mut ath79_ahb_freq: u32 = 0;
pub static mut ath79_ddr_freq: u32 = 0;
pub static mut ath79_soc: ath79_soc_type = 0;
pub static mut ath79_soc_rev: u32 = 0;
pub static mut ath79_pll_base: *mut c_void = core::ptr::null_mut();
pub static mut ath79_reset_base: *mut c_void = core::ptr::null_mut();
static mut ath79_ddr_base: *mut c_void = core::ptr::null_mut();
static mut ath79_ddr_wb_flush_base: *mut c_void = core::ptr::null_mut();
static mut ath79_ddr_pci_win_base: *mut c_void = core::ptr::null_mut();

pub unsafe fn ath79_ddr_ctrl_init() {
    ath79_ddr_base = ioremap(AR71XX_DDR_CTRL_BASE, AR71XX_DDR_CTRL_SIZE);
    if soc_is_ar913x() || soc_is_ar724x() || soc_is_ar933x() {
        ath79_ddr_wb_flush_base = ath79_ddr_base.add(0x7c);
        ath79_ddr_pci_win_base = core::ptr::null_mut();
    } else {
        ath79_ddr_wb_flush_base = ath79_ddr_base.add(0x9c);
        ath79_ddr_pci_win_base = ath79_ddr_base.add(0x7c);
    }
}

pub unsafe fn ath79_ddr_wb_flush(reg: u32) {
    let flush_reg = ath79_ddr_wb_flush_base.add((reg * 4) as usize);
    __raw_writel(0x1, flush_reg);
    while (__raw_readl(flush_reg) & 0x1) != 0 {}
    __raw_writel(0x1, flush_reg);
    while (__raw_readl(flush_reg) & 0x1) != 0 {}
}

pub unsafe fn ath79_ddr_set_pci_windows() {
    BUG_ON(ath79_ddr_pci_win_base.is_null());
    __raw_writel(AR71XX_PCI_WIN0_OFFS, ath79_ddr_pci_win_base.add(0x0));
    __raw_writel(AR71XX_PCI_WIN1_OFFS, ath79_ddr_pci_win_base.add(0x4));
    __raw_writel(AR71XX_PCI_WIN2_OFFS, ath79_ddr_pci_win_base.add(0x8));
    __raw_writel(AR71XX_PCI_WIN3_OFFS, ath79_ddr_pci_win_base.add(0xc));
    __raw_writel(AR71XX_PCI_WIN4_OFFS, ath79_ddr_pci_win_base.add(0x10));
    __raw_writel(AR71XX_PCI_WIN5_OFFS, ath79_ddr_pci_win_base.add(0x14));
    __raw_writel(AR71XX_PCI_WIN6_OFFS, ath79_ddr_pci_win_base.add(0x18));
    __raw_writel(AR71XX_PCI_WIN7_OFFS, ath79_ddr_pci_win_base.add(0x1c));
}

pub unsafe fn ath79_device_reset_set(mask: u32) {
    let mut flags = 0usize;
    let reg = reset_module_reg();
    spin_lock_irqsave(&mut ath79_device_reset_lock as *mut u8 as *mut c_void, &mut flags);
    let t = ath79_reset_rr(reg);
    ath79_reset_wr(reg, t | mask);
    spin_unlock_irqrestore(&mut ath79_device_reset_lock as *mut u8 as *mut c_void, flags);
}

pub unsafe fn ath79_device_reset_clear(mask: u32) {
    let mut flags = 0usize;
    let reg = reset_module_reg();
    spin_lock_irqsave(&mut ath79_device_reset_lock as *mut u8 as *mut c_void, &mut flags);
    let t = ath79_reset_rr(reg);
    ath79_reset_wr(reg, t & !mask);
    spin_unlock_irqrestore(&mut ath79_device_reset_lock as *mut u8 as *mut c_void, flags);
}

unsafe fn reset_module_reg() -> u32 {
    if soc_is_ar71xx() { AR71XX_RESET_REG_RESET_MODULE }
    else if soc_is_ar724x() { AR724X_RESET_REG_RESET_MODULE }
    else if soc_is_ar913x() { AR913X_RESET_REG_RESET_MODULE }
    else if soc_is_ar933x() { AR933X_RESET_REG_RESET_MODULE }
    else if soc_is_ar934x() { AR934X_RESET_REG_RESET_MODULE }
    else if soc_is_qca953x() { QCA953X_RESET_REG_RESET_MODULE }
    else if soc_is_qca955x() { QCA955X_RESET_REG_RESET_MODULE }
    else if soc_is_qca956x() || soc_is_tp9343() { QCA956X_RESET_REG_RESET_MODULE }
    else { BUG(); 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
