// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * the ACC Virtual Support Module of AMD CS5536
 *
 * Copyright (C) 2007 Lemote, Inc.
 * Author : jlliu, liujl@lemote.com
 *
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// Dependencies supplied by the corresponding CS5536 headers are referenced by
// name below; C preprocessor includes are intentionally omitted.

extern "C" {
    fn _rdmsr(reg: u32, hi: *mut u32, lo: *mut u32);
    fn _wrmsr(reg: u32, hi: u32, lo: u32);
}

pub unsafe fn pci_acc_write_reg(reg: i32, mut value: u32) {
    let mut hi: u32 = 0;
    let mut lo: u32 = value;

    match reg {
        PCI_COMMAND => {
            _rdmsr(GLIU_MSR_REG(GLIU_PAE), &mut hi, &mut lo);
            if value & PCI_COMMAND_MASTER != 0 {
                lo |= 0x03 << 8;
            } else {
                lo &= !(0x03 << 8);
            }
            _wrmsr(GLIU_MSR_REG(GLIU_PAE), hi, lo);
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
                lo |= SOFT_BAR_ACC_FLAG;
                _wrmsr(GLCP_MSR_REG(GLCP_SOFT_COM), hi, lo);
            } else if value & 0x01 != 0 {
                value &= 0xfffffffc;
                hi = 0xA0000000 | ((value & 0x000ff000) >> 12);
                lo = 0x000fff80 | ((value & 0x00000fff) << 20);
                _wrmsr(GLIU_MSR_REG(GLIU_IOD_BM1), hi, lo);
            }
        }
        PCI_ACC_INT_REG => {
            _rdmsr(DIVIL_MSR_REG(PIC_YSEL_LOW), &mut hi, &mut lo);
            // disable all the usb interrupt in PIC
            lo &= !(0xf << PIC_YSEL_LOW_ACC_SHIFT);
            if value != 0 {
                // enable all the acc interrupt in PIC
                lo |= CS5536_ACC_INTR << PIC_YSEL_LOW_ACC_SHIFT;
            }
            _wrmsr(DIVIL_MSR_REG(PIC_YSEL_LOW), hi, lo);
        }
        _ => {}
    }
}

pub unsafe fn pci_acc_read_reg(reg: i32) -> u32 {
    let mut hi: u32;
    let mut lo: u32;
    let mut conf_data: u32 = 0;

    match reg {
        PCI_VENDOR_ID => {
            conf_data = CFG_PCI_VENDOR_ID(CS5536_ACC_DEVICE_ID, CS5536_VENDOR_ID);
        }
        PCI_COMMAND => {
            _rdmsr(GLIU_MSR_REG(GLIU_IOD_BM1), &mut hi, &mut lo);
            if ((lo & 0xfff00000) != 0 || (hi & 0x000000ff) != 0)
                && (hi & 0xf0000000) == 0xa0000000
            {
                conf_data |= PCI_COMMAND_IO;
            }
            _rdmsr(GLIU_MSR_REG(GLIU_PAE), &mut hi, &mut lo);
            if lo & 0x300 == 0x300 {
                conf_data |= PCI_COMMAND_MASTER;
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
            _rdmsr(ACC_MSR_REG(ACC_CAP), &mut hi, &mut lo);
            conf_data = lo & 0x000000ff;
            conf_data |= CS5536_ACC_CLASS_CODE << 8;
        }
        PCI_CACHE_LINE_SIZE => {
            conf_data = CFG_PCI_CACHE_LINE_SIZE(
                PCI_NORMAL_HEADER_TYPE,
                PCI_NORMAL_LATENCY_TIMER,
            );
        }
        PCI_BAR0_REG => {
            _rdmsr(GLCP_MSR_REG(GLCP_SOFT_COM), &mut hi, &mut lo);
            if lo & SOFT_BAR_ACC_FLAG != 0 {
                conf_data = CS5536_ACC_RANGE | PCI_BASE_ADDRESS_SPACE_IO;
                lo &= !SOFT_BAR_ACC_FLAG;
                _wrmsr(GLCP_MSR_REG(GLCP_SOFT_COM), hi, lo);
            } else {
                _rdmsr(GLIU_MSR_REG(GLIU_IOD_BM1), &mut hi, &mut lo);
                conf_data = (hi & 0x000000ff) << 12;
                conf_data |= (lo & 0xfff00000) >> 20;
                conf_data |= 0x01;
                conf_data &= !0x02;
            }
        }
        PCI_CARDBUS_CIS => {
            conf_data = PCI_CARDBUS_CIS_POINTER;
        }
        PCI_SUBSYSTEM_VENDOR_ID => {
            conf_data = CFG_PCI_VENDOR_ID(CS5536_ACC_SUB_ID, CS5536_SUB_VENDOR_ID);
        }
        PCI_ROM_ADDRESS => {
            conf_data = PCI_EXPANSION_ROM_BAR;
        }
        PCI_CAPABILITY_LIST => {
            conf_data = PCI_CAPLIST_USB_POINTER;
        }
        PCI_INTERRUPT_LINE => {
            conf_data = CFG_PCI_INTERRUPT_LINE(PCI_DEFAULT_PIN, CS5536_ACC_INTR);
        }
        _ => {}
    }

    conf_data
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
