/* SPDX-License-Identifier: GPL-2.0 */
/*
 * the definition file of cs5536 Virtual Support Module(VSM).
 * pci configuration space can be accessed through the VSM, so
 * there is no need of the MSR read/write now, except the spec.
 * MSR registers which are not implemented yet.
 *
 * Copyright (C) 2007 Lemote Inc.
 * Author : jlliu, liujl@lemote.com
 */

// C header dependencies: linux/init.h, linux/types.h, linux/pci_regs.h.

extern "C" {
    pub fn cs5536_pci_conf_write4(function: i32, reg: i32, value: u32);
    pub fn cs5536_pci_conf_read4(function: i32, reg: i32) -> u32;

    pub fn pci_ehci_write_reg(reg: i32, value: u32);
    pub fn pci_ehci_read_reg(reg: i32) -> u32;

    pub fn pci_ide_write_reg(reg: i32, value: u32);
    pub fn pci_ide_read_reg(reg: i32) -> u32;

    pub fn pci_acc_write_reg(reg: i32, value: u32);
    pub fn pci_acc_read_reg(reg: i32) -> u32;

    pub fn pci_ohci_write_reg(reg: i32, value: u32);
    pub fn pci_ohci_read_reg(reg: i32) -> u32;

    pub fn pci_isa_write_bar(n: i32, value: u32);
    pub fn pci_isa_read_bar(n: i32) -> u32;
    pub fn pci_isa_write_reg(reg: i32, value: u32);
    pub fn pci_isa_read_reg(reg: i32) -> u32;

    // C declaration has the __init annotation.
    pub fn init_mfgpt_clocksource() -> i32;
}

pub const CS5536_ACC_INTR: i32 = 9;
pub const CS5536_IDE_INTR: i32 = 14;
pub const CS5536_USB_INTR: i32 = 11;
pub const CS5536_MFGPT_INTR: i32 = 5;
pub const CS5536_UART1_INTR: i32 = 4;
pub const CS5536_UART2_INTR: i32 = 3;

/* PCI BUS DEVICE FUNCTION */
pub const PCI_BUS_CS5536: i32 = 0;
pub const PCI_IDSEL_CS5536: i32 = 14;

/* CONFIG of PCI VENDOR ID */
pub const fn CFG_PCI_VENDOR_ID(mod_dev_id: u32, sys_vendor_id: u32) -> u32 {
    (mod_dev_id << 16) | sys_vendor_id
}

pub const CS5536_VENDOR_ID: u32 = 0x1022;

pub const CS5536_ISA_DEVICE_ID: u32 = 0x2090;
pub const CS5536_IDE_DEVICE_ID: u32 = 0x209a;
pub const CS5536_ACC_DEVICE_ID: u32 = 0x2093;
pub const CS5536_OHCI_DEVICE_ID: u32 = 0x2094;
pub const CS5536_EHCI_DEVICE_ID: u32 = 0x2095;

pub const CS5536_ISA_CLASS_CODE: u32 = 0x060100;
pub const CS5536_IDE_CLASS_CODE: u32 = 0x010180;
pub const CS5536_ACC_CLASS_CODE: u32 = 0x040100;
pub const CS5536_OHCI_CLASS_CODE: u32 = 0x0C0310;
pub const CS5536_EHCI_CLASS_CODE: u32 = 0x0C0320;

/* BHLC : BIST HEADER-TYPE LATENCY-TIMER CACHE-LINE-SIZE */
pub const fn CFG_PCI_CACHE_LINE_SIZE(header_type: u32, latency_timer: u32) -> u32 {
    (PCI_NONE_BIST << 24) | (header_type << 16) | (latency_timer << 8) | PCI_NORMAL_CACHE_LINE_SIZE
}

pub const PCI_NONE_BIST: u32 = 0x00; /* RO not implemented yet. */
pub const PCI_BRIDGE_HEADER_TYPE: u32 = 0x80; /* RO */
pub const PCI_NORMAL_HEADER_TYPE: u32 = 0x00;
pub const PCI_NORMAL_LATENCY_TIMER: u32 = 0x00;
pub const PCI_NORMAL_CACHE_LINE_SIZE: u32 = 0x08; /* RW */

pub const PCI_BAR0_REG: u32 = 0x10;
pub const PCI_BAR1_REG: u32 = 0x14;
pub const PCI_BAR2_REG: u32 = 0x18;
pub const PCI_BAR3_REG: u32 = 0x1c;
pub const PCI_BAR4_REG: u32 = 0x20;
pub const PCI_BAR5_REG: u32 = 0x24;
pub const PCI_BAR_RANGE_MASK: u32 = 0xFFFFFFFF;

pub const PCI_CARDBUS_CIS_POINTER: u32 = 0x00000000;
pub const CS5536_SUB_VENDOR_ID: u32 = CS5536_VENDOR_ID;
pub const CS5536_ISA_SUB_ID: u32 = CS5536_ISA_DEVICE_ID;
pub const CS5536_IDE_SUB_ID: u32 = CS5536_IDE_DEVICE_ID;
pub const CS5536_ACC_SUB_ID: u32 = CS5536_ACC_DEVICE_ID;
pub const CS5536_OHCI_SUB_ID: u32 = CS5536_OHCI_DEVICE_ID;
pub const CS5536_EHCI_SUB_ID: u32 = CS5536_EHCI_DEVICE_ID;
pub const PCI_EXPANSION_ROM_BAR: u32 = 0x00000000;
pub const PCI_CAPLIST_POINTER: u32 = 0x00000000;
pub const PCI_CAPLIST_USB_POINTER: u32 = 0x40;

pub const fn CFG_PCI_INTERRUPT_LINE(pin: u32, mod_intr: u32) -> u32 {
    (PCI_MAX_LATENCY << 24) | (PCI_MIN_GRANT << 16) | (pin << 8) | mod_intr
}

pub const PCI_MAX_LATENCY: u32 = 0x40;
pub const PCI_MIN_GRANT: u32 = 0x00;
pub const PCI_DEFAULT_PIN: u32 = 0x01;

pub const PCI_UART1_INT_REG: u32 = 0x50;
pub const PCI_UART2_INT_REG: u32 = 0x54;
pub const PCI_ISA_FIXUP_REG: u32 = 0x58;

pub const PCI_IDE_CFG_REG: u32 = 0x40;
pub const CS5536_IDE_FLASH_SIGNATURE: u32 = 0xDEADBEEF;
pub const PCI_IDE_DTC_REG: u32 = 0x48;
pub const PCI_IDE_CAST_REG: u32 = 0x4C;
pub const PCI_IDE_ETC_REG: u32 = 0x50;
pub const PCI_IDE_PM_REG: u32 = 0x54;
pub const PCI_IDE_INT_REG: u32 = 0x60;

pub const PCI_ACC_INT_REG: u32 = 0x50;
pub const PCI_OHCI_PM_REG: u32 = 0x40;
pub const PCI_OHCI_INT_REG: u32 = 0x50;
pub const PCI_EHCI_LEGSMIEN_REG: u32 = 0x50;
pub const PCI_EHCI_LEGSMISTS_REG: u32 = 0x54;
pub const PCI_EHCI_FLADJ_REG: u32 = 0x60;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
