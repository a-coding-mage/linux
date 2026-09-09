// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * the OHCI Virtual Support Module of AMD CS5536
 *
 * Copyright (C) 2007 Lemote, Inc.
 * Author : jlliu, liujl@lemote.com
 *
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// Dependencies supplied by the CS5536 headers are intentionally left external.

extern "C" {
    fn _rdmsr(reg: u32, hi: *mut u32, lo: *mut u32);
    fn _wrmsr(reg: u32, hi: u32, lo: u32);
}

pub unsafe fn pci_ohci_write_reg(reg: i32, mut value: u32) {
    let mut hi: u32 = 0;
    let mut lo: u32 = value;

    match reg {
        PCI_COMMAND => {
            _rdmsr(USB_MSR_REG(USB_OHCI), &mut hi, &mut lo);
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
            _wrmsr(USB_MSR_REG(USB_OHCI), hi, lo);
        }
        PCI_STATUS => {
            if value & PCI_STATUS_PARITY != 0 {
                _rdmsr(SB_MSR_REG(SB_ERROR), &mut hi, &mut lo);
                if lo & SB_PARE_ERR_FLAG != 0 {
                    lo = (lo & 0x0000ffff) | SB_PARE_ERR_FLAG;
                    _wrmsr(SB_MSR_REG(SB_ERROR), hi, lo);
                }
            }
        }
        PCI_BAR0_REG => {
            if value == PCI_BAR_RANGE_MASK {
                _rdmsr(GLCP_MSR_REG(GLCP_SOFT_COM), &mut hi, &mut lo);
                lo |= SOFT_BAR_OHCI_FLAG;
                _wrmsr(GLCP_MSR_REG(GLCP_SOFT_COM), hi, lo);
            } else if value & 0x01 == 0x00 {
                _rdmsr(USB_MSR_REG(USB_OHCI), &mut hi, &mut lo);
                lo = value;
                _wrmsr(USB_MSR_REG(USB_OHCI), hi, lo);

                value &= 0xfffffff0;
                hi = 0x40000000 | ((value & 0xff000000) >> 24);
                lo = 0x000fffff | ((value & 0x00fff000) << 8);
                _wrmsr(GLIU_MSR_REG(GLIU_P2D_BM3), hi, lo);
            }
        }
        PCI_OHCI_INT_REG => {
            _rdmsr(DIVIL_MSR_REG(PIC_YSEL_LOW), &mut hi, &mut lo);
            lo &= !(0xf << PIC_YSEL_LOW_USB_SHIFT);
            if value != 0 {
                // enable all the usb interrupt in PIC
                lo |= CS5536_USB_INTR << PIC_YSEL_LOW_USB_SHIFT;
            }
            _wrmsr(DIVIL_MSR_REG(PIC_YSEL_LOW), hi, lo);
        }
        _ => {}
    }
}

pub unsafe fn pci_ohci_read_reg(reg: i32) -> u32 {
    let mut conf_data: u32 = 0;
    let mut hi: u32;
    let mut lo: u32;

    match reg {
        PCI_VENDOR_ID => {
            conf_data = CFG_PCI_VENDOR_ID(CS5536_OHCI_DEVICE_ID, CS5536_VENDOR_ID);
        }
        PCI_COMMAND => {
            _rdmsr(USB_MSR_REG(USB_OHCI), &mut hi, &mut lo);
            if hi & PCI_COMMAND_MASTER != 0 {
                conf_data |= PCI_COMMAND_MASTER;
            }
            if hi & PCI_COMMAND_MEMORY != 0 {
                conf_data |= PCI_COMMAND_MEMORY;
            }
        }
        PCI_STATUS => {
            conf_data |= PCI_STATUS_66MHZ;
            conf_data |= PCI_STATUS_FAST_BACK;
            _rdmsr(SB_MSR_REG(SB_ERROR), &mut hi, &mut lo);
            if lo & SB_PARE_ERR_FLAG != 0 {
                conf_data |= PCI_STATUS_PARITY;
            }
            conf_data |= PCI_STATUS_DEVSEL_MEDIUM;
        }
        PCI_CLASS_REVISION => {
            _rdmsr(USB_MSR_REG(USB_CAP), &mut hi, &mut lo);
            conf_data = lo & 0x000000ff;
            conf_data |= CS5536_OHCI_CLASS_CODE << 8;
        }
        PCI_CACHE_LINE_SIZE => {
            conf_data = CFG_PCI_CACHE_LINE_SIZE(
                PCI_NORMAL_HEADER_TYPE,
                PCI_NORMAL_LATENCY_TIMER,
            );
        }
        PCI_BAR0_REG => {
            _rdmsr(GLCP_MSR_REG(GLCP_SOFT_COM), &mut hi, &mut lo);
            if lo & SOFT_BAR_OHCI_FLAG != 0 {
                conf_data = CS5536_OHCI_RANGE | PCI_BASE_ADDRESS_SPACE_MEMORY;
                lo &= !SOFT_BAR_OHCI_FLAG;
                _wrmsr(GLCP_MSR_REG(GLCP_SOFT_COM), hi, lo);
            } else {
                _rdmsr(USB_MSR_REG(USB_OHCI), &mut hi, &mut lo);
                conf_data = lo & 0xffffff00;
                conf_data &= !0x0000000f; // 32bit mem
            }
        }
        PCI_CARDBUS_CIS => {
            conf_data = PCI_CARDBUS_CIS_POINTER;
        }
        PCI_SUBSYSTEM_VENDOR_ID => {
            conf_data = CFG_PCI_VENDOR_ID(CS5536_OHCI_SUB_ID, CS5536_SUB_VENDOR_ID);
        }
        PCI_ROM_ADDRESS => {
            conf_data = PCI_EXPANSION_ROM_BAR;
        }
        PCI_CAPABILITY_LIST => {
            conf_data = PCI_CAPLIST_USB_POINTER;
        }
        PCI_INTERRUPT_LINE => {
            conf_data = CFG_PCI_INTERRUPT_LINE(PCI_DEFAULT_PIN, CS5536_USB_INTR);
        }
        PCI_OHCI_INT_REG => {
            _rdmsr(DIVIL_MSR_REG(PIC_YSEL_LOW), &mut hi, &mut lo);
            if ((lo >> PIC_YSEL_LOW_USB_SHIFT) & 0xf) == CS5536_USB_INTR {
                conf_data = 1;
            }
        }
        _ => {}
    }

    conf_data
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
