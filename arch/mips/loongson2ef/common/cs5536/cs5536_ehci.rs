// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * the EHCI Virtual Support Module of AMD CS5536
 *
 * Copyright (C) 2007 Lemote, Inc.
 * Author : jlliu, liujl@lemote.com
 *
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// Dependencies supplied by the CS5536 headers are intentionally external.

pub unsafe fn pci_ehci_write_reg(reg: i32, mut value: u32) {
    let mut hi: u32 = 0;
    let mut lo: u32 = value;

    match reg {
        PCI_COMMAND => {
            _rdmsr(USB_MSR_REG(USB_EHCI), &mut hi, &mut lo);
            if value & PCI_COMMAND_MASTER != 0 {
                hi |= PCI_COMMAND_MASTER;
            } else {
                hi &= !PCI_COMMAND_MASTER;
            }

            if value & PCI_COMMAND_MEMORY != 0 {
                hi |= PCI_COMMAND_MEMORY;
            } else {
                hi &= !PCI_COMMAND_MEMORY;
            }
            _wrmsr(USB_MSR_REG(USB_EHCI), hi, lo);
        }
        PCI_STATUS => {
            if value & PCI_STATUS_PARITY != 0 {
                _rdmsr(SB_MSR_REG(SB_ERROR), &mut hi, &mut lo);
                if lo & SB_PARE_ERR_FLAG != 0 {
                    lo = (lo & 0x0000_ffff) | SB_PARE_ERR_FLAG;
                    _wrmsr(SB_MSR_REG(SB_ERROR), hi, lo);
                }
            }
        }
        PCI_BAR0_REG => {
            if value == PCI_BAR_RANGE_MASK {
                _rdmsr(GLCP_MSR_REG(GLCP_SOFT_COM), &mut hi, &mut lo);
                lo |= SOFT_BAR_EHCI_FLAG;
                _wrmsr(GLCP_MSR_REG(GLCP_SOFT_COM), hi, lo);
            } else if value & 0x01 == 0x00 {
                _rdmsr(USB_MSR_REG(USB_EHCI), &mut hi, &mut lo);
                lo = value;
                _wrmsr(USB_MSR_REG(USB_EHCI), hi, lo);

                value &= 0xffff_fff0;
                hi = 0x4000_0000 | ((value & 0xff00_0000) >> 24);
                lo = 0x000f_ffff | ((value & 0x00ff_f000) << 8);
                _wrmsr(GLIU_MSR_REG(GLIU_P2D_BM4), hi, lo);
            }
        }
        PCI_EHCI_LEGSMIEN_REG => {
            _rdmsr(USB_MSR_REG(USB_EHCI), &mut hi, &mut lo);
            hi &= 0x003f_0000;
            hi |= (value & 0x3f) << 16;
            _wrmsr(USB_MSR_REG(USB_EHCI), hi, lo);
        }
        PCI_EHCI_FLADJ_REG => {
            _rdmsr(USB_MSR_REG(USB_EHCI), &mut hi, &mut lo);
            hi &= !0x0000_3f00;
            hi |= value & 0x0000_3f00;
            _wrmsr(USB_MSR_REG(USB_EHCI), hi, lo);
        }
        _ => {}
    }
}

pub unsafe fn pci_ehci_read_reg(reg: i32) -> u32 {
    let mut conf_data: u32 = 0;
    let mut hi: u32;
    let mut lo: u32;

    match reg {
        PCI_VENDOR_ID => {
            conf_data = CFG_PCI_VENDOR_ID(CS5536_EHCI_DEVICE_ID, CS5536_VENDOR_ID);
        }
        PCI_COMMAND => {
            _rdmsr(USB_MSR_REG(USB_EHCI), &mut hi, &mut lo);
            if hi & PCI_COMMAND_MASTER != 0 { conf_data |= PCI_COMMAND_MASTER; }
            if hi & PCI_COMMAND_MEMORY != 0 { conf_data |= PCI_COMMAND_MEMORY; }
        }
        PCI_STATUS => {
            conf_data |= PCI_STATUS_66MHZ;
            conf_data |= PCI_STATUS_FAST_BACK;
            _rdmsr(SB_MSR_REG(SB_ERROR), &mut hi, &mut lo);
            if lo & SB_PARE_ERR_FLAG != 0 { conf_data |= PCI_STATUS_PARITY; }
            conf_data |= PCI_STATUS_DEVSEL_MEDIUM;
        }
        PCI_CLASS_REVISION => {
            _rdmsr(USB_MSR_REG(USB_CAP), &mut hi, &mut lo);
            conf_data = lo & 0x0000_00ff;
            conf_data |= CS5536_EHCI_CLASS_CODE << 8;
        }
        PCI_CACHE_LINE_SIZE => {
            conf_data = CFG_PCI_CACHE_LINE_SIZE(PCI_NORMAL_HEADER_TYPE, PCI_NORMAL_LATENCY_TIMER);
        }
        PCI_BAR0_REG => {
            _rdmsr(GLCP_MSR_REG(GLCP_SOFT_COM), &mut hi, &mut lo);
            if lo & SOFT_BAR_EHCI_FLAG != 0 {
                conf_data = CS5536_EHCI_RANGE | PCI_BASE_ADDRESS_SPACE_MEMORY;
                lo &= !SOFT_BAR_EHCI_FLAG;
                _wrmsr(GLCP_MSR_REG(GLCP_SOFT_COM), hi, lo);
            } else {
                _rdmsr(USB_MSR_REG(USB_EHCI), &mut hi, &mut lo);
                conf_data = lo & 0xffff_f000;
            }
        }
        PCI_CARDBUS_CIS => { conf_data = PCI_CARDBUS_CIS_POINTER; }
        PCI_SUBSYSTEM_VENDOR_ID => {
            conf_data = CFG_PCI_VENDOR_ID(CS5536_EHCI_SUB_ID, CS5536_SUB_VENDOR_ID);
        }
        PCI_ROM_ADDRESS => { conf_data = PCI_EXPANSION_ROM_BAR; }
        PCI_CAPABILITY_LIST => { conf_data = PCI_CAPLIST_USB_POINTER; }
        PCI_INTERRUPT_LINE => {
            conf_data = CFG_PCI_INTERRUPT_LINE(PCI_DEFAULT_PIN, CS5536_USB_INTR);
        }
        PCI_EHCI_LEGSMIEN_REG => {
            _rdmsr(USB_MSR_REG(USB_EHCI), &mut hi, &mut lo);
            conf_data = (hi & 0x003f_0000) >> 16;
        }
        PCI_EHCI_LEGSMISTS_REG => {
            _rdmsr(USB_MSR_REG(USB_EHCI), &mut hi, &mut lo);
            conf_data = (hi & 0x3f00_0000) >> 24;
        }
        PCI_EHCI_FLADJ_REG => {
            _rdmsr(USB_MSR_REG(USB_EHCI), &mut hi, &mut lo);
            conf_data = hi & 0x0000_3f00;
        }
        _ => {}
    }

    conf_data
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
