// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * the ISA Virtual Support Module of AMD CS5536
 *
 * Copyright (C) 2007 Lemote, Inc.
 * Author : jlliu, liujl@lemote.com
 *
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// External kernel and CS5536 definitions are supplied by other translation units.

static DIVIL_MSR_REGS: [u32; 6] = [
    DIVIL_MSR_REG(DIVIL_LBAR_SMB), DIVIL_MSR_REG(DIVIL_LBAR_GPIO),
    DIVIL_MSR_REG(DIVIL_LBAR_MFGPT), DIVIL_MSR_REG(DIVIL_LBAR_IRQ),
    DIVIL_MSR_REG(DIVIL_LBAR_PMS), DIVIL_MSR_REG(DIVIL_LBAR_ACPI),
];

static SOFT_BAR_FLAGS: [u32; 6] = [
    SOFT_BAR_SMB_FLAG, SOFT_BAR_GPIO_FLAG, SOFT_BAR_MFGPT_FLAG,
    SOFT_BAR_IRQ_FLAG, SOFT_BAR_PMS_FLAG, SOFT_BAR_ACPI_FLAG,
];

static SB_MSR_REGS: [u32; 6] = [
    SB_MSR_REG(SB_R0), SB_MSR_REG(SB_R1), SB_MSR_REG(SB_R2),
    SB_MSR_REG(SB_R3), SB_MSR_REG(SB_R4), SB_MSR_REG(SB_R5),
];

static BAR_SPACE_RANGES: [u32; 6] = [
    CS5536_SMB_RANGE, CS5536_GPIO_RANGE, CS5536_MFGPT_RANGE,
    CS5536_IRQ_RANGE, CS5536_PMS_RANGE, CS5536_ACPI_RANGE,
];

static BAR_SPACE_LENGTHS: [i32; 6] = [
    CS5536_SMB_LENGTH, CS5536_GPIO_LENGTH, CS5536_MFGPT_LENGTH,
    CS5536_IRQ_LENGTH, CS5536_PMS_LENGTH, CS5536_ACPI_LENGTH,
];

unsafe fn divil_lbar_enable() {
    let (mut hi, mut lo): (u32, u32);
    for offset in DIVIL_LBAR_SMB..=DIVIL_LBAR_PMS {
        _rdmsr(DIVIL_MSR_REG(offset), &mut hi, &mut lo);
        hi |= 0x01;
        _wrmsr(DIVIL_MSR_REG(offset), hi, lo);
    }
}

unsafe fn divil_lbar_disable() {
    let (mut hi, mut lo): (u32, u32);
    for offset in DIVIL_LBAR_SMB..=DIVIL_LBAR_PMS {
        _rdmsr(DIVIL_MSR_REG(offset), &mut hi, &mut lo);
        hi &= !0x01;
        _wrmsr(DIVIL_MSR_REG(offset), hi, lo);
    }
}

pub unsafe fn pci_isa_write_bar(n: usize, value: u32) {
    let (mut hi, mut lo) = (0u32, value);
    if value == PCI_BAR_RANGE_MASK {
        _rdmsr(GLCP_MSR_REG(GLCP_SOFT_COM), &mut hi, &mut lo);
        lo |= SOFT_BAR_FLAGS[n];
        _wrmsr(GLCP_MSR_REG(GLCP_SOFT_COM), hi, lo);
    } else if value & 0x01 != 0 {
        hi = 0x0000f001;
        lo &= BAR_SPACE_RANGES[n];
        _wrmsr(DIVIL_MSR_REGS[n], hi, lo);
        hi = ((value & 0x000ffffc) << 12)
            | (((BAR_SPACE_LENGTHS[n] - 4) as u32) << 12) | 0x01;
        lo = ((value & 0x000ffffc) << 12) | 0x01;
        _wrmsr(SB_MSR_REGS[n], hi, lo);
    }
}

pub unsafe fn pci_isa_read_bar(n: usize) -> u32 {
    let (mut hi, mut lo) = (0u32, 0u32);
    let mut conf_data = 0u32;
    _rdmsr(GLCP_MSR_REG(GLCP_SOFT_COM), &mut hi, &mut lo);
    if lo & SOFT_BAR_FLAGS[n] != 0 {
        conf_data = BAR_SPACE_RANGES[n] | PCI_BASE_ADDRESS_SPACE_IO;
        lo &= !SOFT_BAR_FLAGS[n];
        _wrmsr(GLCP_MSR_REG(GLCP_SOFT_COM), hi, lo);
    } else {
        _rdmsr(DIVIL_MSR_REGS[n], &mut hi, &mut lo);
        conf_data = lo & BAR_SPACE_RANGES[n];
        conf_data |= 0x01;
        conf_data &= !0x02;
    }
    conf_data
}

pub unsafe fn pci_isa_write_reg(reg: u32, mut value: u32) {
    let (mut hi, mut lo) = (0u32, value);
    let mut temp: u32;
    match reg {
        PCI_COMMAND => if value & PCI_COMMAND_IO != 0 { divil_lbar_enable(); } else { divil_lbar_disable(); },
        PCI_STATUS => {
            _rdmsr(SB_MSR_REG(SB_ERROR), &mut hi, &mut lo);
            temp = lo & 0x0000ffff;
            if value & PCI_STATUS_SIG_TARGET_ABORT != 0 && lo & SB_TAS_ERR_EN != 0 { temp |= SB_TAS_ERR_FLAG; }
            if value & PCI_STATUS_REC_TARGET_ABORT != 0 && lo & SB_TAR_ERR_EN != 0 { temp |= SB_TAR_ERR_FLAG; }
            if value & PCI_STATUS_REC_MASTER_ABORT != 0 && lo & SB_MAR_ERR_EN != 0 { temp |= SB_MAR_ERR_FLAG; }
            if value & PCI_STATUS_DETECTED_PARITY != 0 && lo & SB_PARE_ERR_EN != 0 { temp |= SB_PARE_ERR_FLAG; }
            _wrmsr(SB_MSR_REG(SB_ERROR), hi, temp);
        }
        PCI_CACHE_LINE_SIZE => {
            value &= 0x0000ff00;
            _rdmsr(SB_MSR_REG(SB_CTRL), &mut hi, &mut lo);
            hi &= 0xffffff00; hi |= value >> 8;
            _wrmsr(SB_MSR_REG(SB_CTRL), hi, lo);
        }
        PCI_BAR0_REG..=PCI_BAR5_REG => pci_isa_write_bar((reg - PCI_BAR0_REG) as usize, value),
        PCI_UART1_INT_REG => { _rdmsr(DIVIL_MSR_REG(PIC_YSEL_HIGH), &mut hi, &mut lo); lo &= !(0xf << 24); if value != 0 { lo |= CS5536_UART1_INTR << 24; } _wrmsr(DIVIL_MSR_REG(PIC_YSEL_HIGH), hi, lo); }
        PCI_UART2_INT_REG => { _rdmsr(DIVIL_MSR_REG(PIC_YSEL_HIGH), &mut hi, &mut lo); lo &= !(0xf << 28); if value != 0 { lo |= CS5536_UART2_INTR << 28; } _wrmsr(DIVIL_MSR_REG(PIC_YSEL_HIGH), hi, lo); }
        PCI_ISA_FIXUP_REG => if value != 0 { _rdmsr(SB_MSR_REG(SB_ERROR), &mut hi, &mut lo); lo |= 0x00000063; _wrmsr(SB_MSR_REG(SB_ERROR), hi, lo); },
        _ => (),
    }
}

pub unsafe fn pci_isa_read_reg(reg: u32) -> u32 {
    let (mut hi, mut lo) = (0u32, 0u32);
    let mut conf_data = 0u32;
    match reg {
        PCI_VENDOR_ID => conf_data = CFG_PCI_VENDOR_ID(CS5536_ISA_DEVICE_ID, CS5536_VENDOR_ID),
        PCI_COMMAND => { _rdmsr(DIVIL_MSR_REG(DIVIL_LBAR_SMB), &mut hi, &mut lo); if hi & 0x01 != 0 { conf_data |= PCI_COMMAND_IO; } }
        PCI_STATUS => { conf_data |= PCI_STATUS_66MHZ | PCI_STATUS_DEVSEL_MEDIUM | PCI_STATUS_FAST_BACK; _rdmsr(SB_MSR_REG(SB_ERROR), &mut hi, &mut lo); if lo & SB_TAS_ERR_FLAG != 0 { conf_data |= PCI_STATUS_SIG_TARGET_ABORT; } if lo & SB_TAR_ERR_FLAG != 0 { conf_data |= PCI_STATUS_REC_TARGET_ABORT; } if lo & SB_MAR_ERR_FLAG != 0 { conf_data |= PCI_STATUS_REC_MASTER_ABORT; } if lo & SB_PARE_ERR_FLAG != 0 { conf_data |= PCI_STATUS_DETECTED_PARITY; } }
        PCI_CLASS_REVISION => { _rdmsr(GLCP_MSR_REG(GLCP_CHIP_REV_ID), &mut hi, &mut lo); conf_data = (lo & 0xff) | (CS5536_ISA_CLASS_CODE << 8); }
        PCI_CACHE_LINE_SIZE => { _rdmsr(SB_MSR_REG(SB_CTRL), &mut hi, &mut lo); hi &= 0x000000f8; conf_data = CFG_PCI_CACHE_LINE_SIZE(PCI_BRIDGE_HEADER_TYPE, hi); }
        PCI_BAR0_REG..=PCI_BAR2_REG | PCI_BAR4_REG..=PCI_BAR5_REG => return pci_isa_read_bar((reg - PCI_BAR0_REG) as usize),
        PCI_CARDBUS_CIS => conf_data = PCI_CARDBUS_CIS_POINTER,
        PCI_SUBSYSTEM_VENDOR_ID => conf_data = CFG_PCI_VENDOR_ID(CS5536_ISA_SUB_ID, CS5536_SUB_VENDOR_ID),
        PCI_ROM_ADDRESS => conf_data = PCI_EXPANSION_ROM_BAR,
        PCI_CAPABILITY_LIST => conf_data = PCI_CAPLIST_POINTER,
        PCI_INTERRUPT_LINE => conf_data = CFG_PCI_INTERRUPT_LINE(0x00, 0x00),
        _ => (),
    }
    conf_data
}

unsafe fn cs5536_isa_mmio_always_on(dev: *mut pci_dev) {
    (*dev).mmio_always_on = 1;
}

// The C DECLARE_PCI_FIXUP_CLASS_EARLY registration is preserved as external
// integration intent; the corresponding kernel macro is supplied elsewhere.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
