// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2004 ICT CAS
 * Author: Li xiaoyu, ICT CAS
 *   lixy@ict.ac.cn
 *
 * Copyright (C) 2007 Lemote, Inc. & Institute of Computing Technology
 * Author: Fuxin Zhang, zhangfx@lemote.com
 */

// Dependencies supplied by the surrounding kernel environment.
use crate::linux::pci::{pci_dev, PCI_COMMAND, PCI_COMMAND_IO, PCI_COMMAND_MEMORY,
    PCI_COMMAND_MASTER, PCI_INTERRUPT_LINE, PCI_LATENCY_TIMER, PCI_DEVICE_ID_NEC_USB,
    PCI_DEVICE_ID_VIA_82C586_1, PCI_DEVICE_ID_VIA_82C586_2, PCI_DEVICE_ID_VIA_82C586_3,
    PCI_DEVICE_ID_VIA_82C686, PCI_DEVICE_ID_VIA_82C686_5, PCI_FUNC, PCI_SLOT,
    PCI_VENDOR_ID_NEC, PCI_VENDOR_ID_VIA};
use crate::loongson::LOONGSON_IRQ_BASE;

extern "C" {
    fn pci_read_config_byte(dev: *mut pci_dev, where_: u32, val: *mut u8);
    fn pci_read_config_word(dev: *mut pci_dev, where_: u32, val: *mut u16);
    fn pci_read_config_dword(dev: *mut pci_dev, where_: u32, val: *mut u32);
    fn pci_write_config_byte(dev: *mut pci_dev, where_: u32, val: u8);
    fn pci_write_config_word(dev: *mut pci_dev, where_: u32, val: u16);
    fn pci_write_config_dword(dev: *mut pci_dev, where_: u32, val: u32);
    fn outb(value: u8, port: u16);
    fn printk(level: u32, message: *const u8, ...);
}

static mut sb_slot: u8 = 5;

pub unsafe fn pcibios_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32 {
    let mut irq: i32 = 0;
    if slot == sb_slot {
        match PCI_FUNC((*dev).devfn) {
            2 => irq = 10,
            3 => irq = 11,
            5 => irq = 9,
            _ => {}
        }
    } else {
        irq = LOONGSON_IRQ_BASE as i32 + 25 + pin as i32;
    }
    irq
}

pub unsafe fn pcibios_plat_dev_init(_dev: *mut pci_dev) -> i32 { 0 }

unsafe fn loongson2e_nec_fixup(pdev: *mut pci_dev) {
    let mut val: u32 = 0;
    pci_read_config_dword(pdev, 0xe0, &mut val);
    pci_write_config_dword(pdev, 0xe0, (val & !7) | 0x4);
    pci_write_config_dword(pdev, 0xe4, 1 << 5);
}

unsafe fn loongson2e_686b_func0_fixup(pdev: *mut pci_dev) {
    let mut c: u8 = 0;
    sb_slot = PCI_SLOT((*pdev).devfn) as u8;
    pci_write_config_byte(pdev, 0x40, 0x08);
    pci_write_config_byte(pdev, 0x41, 0x01);
    pci_write_config_byte(pdev, 0x45, 0x00);
    pci_write_config_byte(pdev, 0x46, 0xe0);
    pci_write_config_byte(pdev, 0x47, 0xe6);
    outb(0x2e, 0x4d1);
    pci_write_config_byte(pdev, 0x48, 0x01);
    pci_write_config_byte(pdev, 0x4a, 0x84);
    pci_write_config_byte(pdev, 0x50, 0x0e);
    pci_write_config_byte(pdev, 0x51, 0x76);
    pci_write_config_byte(pdev, 0x52, 0x34);
    pci_write_config_byte(pdev, 0x54, 0x00);
    pci_write_config_byte(pdev, 0x55, 0x90);
    pci_write_config_byte(pdev, 0x56, 0xba);
    pci_write_config_byte(pdev, 0x57, 0xd0);
    pci_read_config_byte(pdev, 0x85, &mut c);
    c &= !(0x3 << 2);
    pci_write_config_byte(pdev, 0x85, c);
}

unsafe fn loongson2e_686b_func1_fixup(pdev: *mut pci_dev) {
    pci_write_config_byte(pdev, PCI_LATENCY_TIMER, 48);
    pci_write_config_byte(pdev, PCI_COMMAND, PCI_COMMAND_IO | PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER);
    pci_write_config_byte(pdev, 0x40, 0x0b);
    pci_write_config_byte(pdev, 0x42, 0x09);
    pci_write_config_byte(pdev, 0x41, 0x02);
    pci_write_config_byte(pdev, 0x43, 0x0a);
    pci_write_config_byte(pdev, 0x44, 0x00);
    pci_write_config_byte(pdev, 0x45, 0x00);
}

unsafe fn loongson2e_686b_func2_fixup(pdev: *mut pci_dev) { pci_write_config_byte(pdev, PCI_INTERRUPT_LINE, 10); }
unsafe fn loongson2e_686b_func3_fixup(pdev: *mut pci_dev) { pci_write_config_byte(pdev, PCI_INTERRUPT_LINE, 11); }

unsafe fn loongson2e_686b_func5_fixup(pdev: *mut pci_dev) {
    let mut val: u32 = 0;
    let mut c: u8 = 0;
    pci_write_config_byte(pdev, PCI_COMMAND, PCI_COMMAND_IO | PCI_COMMAND_MEMORY | PCI_COMMAND_MASTER);
    pci_read_config_dword(pdev, 0x4, &mut val);
    pci_write_config_dword(pdev, 0x4, val | 1);
    pci_write_config_byte(pdev, 0x3c, 9);
    pci_read_config_byte(pdev, 0x8, &mut c);
    pci_write_config_byte(pdev, 0x41, 0xcc);
    pci_write_config_byte(pdev, 0x42, 0x20);
    pci_write_config_word(pdev, 0x2c, 0x1005);
    pci_write_config_word(pdev, 0x2e, 0x4710);
    pci_read_config_dword(pdev, 0x2c, &mut val);
    pci_write_config_byte(pdev, 0x42, 0x0);
}

// DECLARE_PCI_FIXUP_HEADER registrations:
// (PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_82C686, loongson2e_686b_func0_fixup)
// (PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_82C586_1, loongson2e_686b_func1_fixup)
// (PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_82C586_2, loongson2e_686b_func2_fixup)
// (PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_82C586_3, loongson2e_686b_func3_fixup)
// (PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_82C686_5, loongson2e_686b_func5_fixup)
// (PCI_VENDOR_ID_NEC, PCI_DEVICE_ID_NEC_USB, loongson2e_nec_fixup)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
