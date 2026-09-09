/* SPDX-License-Identifier: GPL-2.0 */
/*
 * the read/write interfaces for Virtual Support Module(VSM)
 *
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Wu Zhangjin <wuzhangjin@gmail.com>
 */

// Dependency equivalent: Linux `u32` is represented by Rust `u32`.

pub type Cs5536PciVsmWrite = unsafe extern "C" fn(reg: i32, value: u32);
pub type Cs5536PciVsmRead = unsafe extern "C" fn(reg: i32) -> u32;

// DECLARE_CS5536_MODULE(ide)
// DECLARE_CS5536_MODULE(acc)
// DECLARE_CS5536_MODULE(ohci)
// DECLARE_CS5536_MODULE(isa)
// DECLARE_CS5536_MODULE(ehci)
extern "C" {
    pub fn pci_ide_write_reg(reg: i32, value: u32);
    pub fn pci_ide_read_reg(reg: i32) -> u32;

    pub fn pci_acc_write_reg(reg: i32, value: u32);
    pub fn pci_acc_read_reg(reg: i32) -> u32;

    pub fn pci_ohci_write_reg(reg: i32, value: u32);
    pub fn pci_ohci_read_reg(reg: i32) -> u32;

    pub fn pci_isa_write_reg(reg: i32, value: u32);
    pub fn pci_isa_read_reg(reg: i32) -> u32;

    pub fn pci_ehci_write_reg(reg: i32, value: u32);
    pub fn pci_ehci_read_reg(reg: i32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
