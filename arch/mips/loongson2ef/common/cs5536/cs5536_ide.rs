// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * the IDE Virtual Support Module of AMD CS5536
 *
 * Copyright (C) 2007 Lemote, Inc.
 * Author : jlliu, liujl@lemote.com
 *
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

pub unsafe fn pci_ide_write_reg(reg: i32, mut value: u32) {
    let mut hi: u32 = 0;
    let mut lo: u32 = value;

    match reg {
        PCI_COMMAND => {
            _rdmsr(GLIU_MSR_REG(GLIU_PAE), &mut hi, &mut lo);
            if value & PCI_COMMAND_MASTER != 0 {
                lo |= 0x03 << 4;
            } else {
                lo &= !(0x03 << 4);
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
        PCI_CACHE_LINE_SIZE => {
            value &= 0x0000ff00;
            _rdmsr(SB_MSR_REG(SB_CTRL), &mut hi, &mut lo);
            hi &= 0xffffff00;
            hi |= value >> 8;
            _wrmsr(SB_MSR_REG(SB_CTRL), hi, lo);
        }
        PCI_BAR4_REG => {
            if value == PCI_BAR_RANGE_MASK {
                _rdmsr(GLCP_MSR_REG(GLCP_SOFT_COM), &mut hi, &mut lo);
                lo |= SOFT_BAR_IDE_FLAG;
                _wrmsr(GLCP_MSR_REG(GLCP_SOFT_COM), hi, lo);
            } else if value & 0x01 != 0 {
                _rdmsr(IDE_MSR_REG(IDE_IO_BAR), &mut hi, &mut lo);
                lo = (value & 0xfffffff0) | 0x1;
                _wrmsr(IDE_MSR_REG(IDE_IO_BAR), hi, lo);

                value &= 0xfffffffc;
                hi = 0x60000000 | ((value & 0x000ff000) >> 12);
                lo = 0x000ffff0 | ((value & 0x00000fff) << 20);
                _wrmsr(GLIU_MSR_REG(GLIU_IOD_BM2), hi, lo);
            }
        }
        PCI_IDE_CFG_REG => {
            if value == CS5536_IDE_FLASH_SIGNATURE {
                _rdmsr(DIVIL_MSR_REG(DIVIL_BALL_OPTS), &mut hi, &mut lo);
                lo |= 0x01;
                _wrmsr(DIVIL_MSR_REG(DIVIL_BALL_OPTS), hi, lo);
            } else {
                _rdmsr(IDE_MSR_REG(IDE_CFG), &mut hi, &mut lo);
                lo = value;
                _wrmsr(IDE_MSR_REG(IDE_CFG), hi, lo);
            }
        }
        PCI_IDE_DTC_REG => {
            _rdmsr(IDE_MSR_REG(IDE_DTC), &mut hi, &mut lo);
            lo = value;
            _wrmsr(IDE_MSR_REG(IDE_DTC), hi, lo);
        }
        PCI_IDE_CAST_REG => {
            _rdmsr(IDE_MSR_REG(IDE_CAST), &mut hi, &mut lo);
            lo = value;
            _wrmsr(IDE_MSR_REG(IDE_CAST), hi, lo);
        }
        PCI_IDE_ETC_REG => {
            _rdmsr(IDE_MSR_REG(IDE_ETC), &mut hi, &mut lo);
            lo = value;
            _wrmsr(IDE_MSR_REG(IDE_ETC), hi, lo);
        }
        PCI_IDE_PM_REG => {
            _rdmsr(IDE_MSR_REG(IDE_INTERNAL_PM), &mut hi, &mut lo);
            lo = value;
            _wrmsr(IDE_MSR_REG(IDE_INTERNAL_PM), hi, lo);
        }
        _ => {}
    }
}

pub unsafe fn pci_ide_read_reg(reg: i32) -> u32 {
    let mut conf_data: u32 = 0;
    let mut hi: u32;
    let mut lo: u32;

    match reg {
        PCI_VENDOR_ID => {
            conf_data = CFG_PCI_VENDOR_ID(CS5536_IDE_DEVICE_ID, CS5536_VENDOR_ID);
        }
        PCI_COMMAND => {
            _rdmsr(IDE_MSR_REG(IDE_IO_BAR), &mut hi, &mut lo);
            if lo & 0xfffffff0 != 0 { conf_data |= PCI_COMMAND_IO; }
            _rdmsr(GLIU_MSR_REG(GLIU_PAE), &mut hi, &mut lo);
            if lo & 0x30 == 0x30 { conf_data |= PCI_COMMAND_MASTER; }
        }
        PCI_STATUS => {
            conf_data |= PCI_STATUS_66MHZ | PCI_STATUS_FAST_BACK;
            _rdmsr(SB_MSR_REG(SB_ERROR), &mut hi, &mut lo);
            if lo & SB_PARE_ERR_FLAG != 0 { conf_data |= PCI_STATUS_PARITY; }
            conf_data |= PCI_STATUS_DEVSEL_MEDIUM;
        }
        PCI_CLASS_REVISION => {
            _rdmsr(IDE_MSR_REG(IDE_CAP), &mut hi, &mut lo);
            conf_data = lo & 0x000000ff;
            conf_data |= CS5536_IDE_CLASS_CODE << 8;
        }
        PCI_CACHE_LINE_SIZE => {
            _rdmsr(SB_MSR_REG(SB_CTRL), &mut hi, &mut lo);
            hi &= 0x000000f8;
            conf_data = CFG_PCI_CACHE_LINE_SIZE(PCI_NORMAL_HEADER_TYPE, hi);
        }
        PCI_BAR4_REG => {
            _rdmsr(GLCP_MSR_REG(GLCP_SOFT_COM), &mut hi, &mut lo);
            if lo & SOFT_BAR_IDE_FLAG != 0 {
                conf_data = CS5536_IDE_RANGE | PCI_BASE_ADDRESS_SPACE_IO;
                lo &= !SOFT_BAR_IDE_FLAG;
                _wrmsr(GLCP_MSR_REG(GLCP_SOFT_COM), hi, lo);
            } else {
                _rdmsr(IDE_MSR_REG(IDE_IO_BAR), &mut hi, &mut lo);
                conf_data = (lo & 0xfffffff0) | 0x01;
                conf_data &= !0x02;
            }
        }
        PCI_CARDBUS_CIS => conf_data = PCI_CARDBUS_CIS_POINTER,
        PCI_SUBSYSTEM_VENDOR_ID => conf_data = CFG_PCI_VENDOR_ID(CS5536_IDE_SUB_ID, CS5536_SUB_VENDOR_ID),
        PCI_ROM_ADDRESS => conf_data = PCI_EXPANSION_ROM_BAR,
        PCI_CAPABILITY_LIST => conf_data = PCI_CAPLIST_POINTER,
        PCI_INTERRUPT_LINE => conf_data = CFG_PCI_INTERRUPT_LINE(PCI_DEFAULT_PIN, CS5536_IDE_INTR),
        PCI_IDE_CFG_REG => { _rdmsr(IDE_MSR_REG(IDE_CFG), &mut hi, &mut lo); conf_data = lo; }
        PCI_IDE_DTC_REG => { _rdmsr(IDE_MSR_REG(IDE_DTC), &mut hi, &mut lo); conf_data = lo; }
        PCI_IDE_CAST_REG => { _rdmsr(IDE_MSR_REG(IDE_CAST), &mut hi, &mut lo); conf_data = lo; }
        PCI_IDE_ETC_REG => { _rdmsr(IDE_MSR_REG(IDE_ETC), &mut hi, &mut lo); conf_data = lo; }
        PCI_IDE_PM_REG => { _rdmsr(IDE_MSR_REG(IDE_INTERNAL_PM), &mut hi, &mut lo); conf_data = lo; }
        _ => {}
    }

    conf_data
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
