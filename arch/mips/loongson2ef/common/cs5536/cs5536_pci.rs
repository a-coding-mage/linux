// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * read/write operation to the PCI config space of CS5536
 *
 * Copyright (C) 2007 Lemote, Inc.
 * Author : jlliu, liujl@lemote.com
 *
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 *
 *	the Virtual Support Module(VSM) for virtulizing the PCI
 *	configure space are defined in cs5536_modulename.c respectively,
 *
 *	after this virtulizing, user can access the PCI configure space
 *	directly as a normal multi-function PCI device which follows
 *	the PCI-2.2 spec.
 */

// Declarations supplied by cs5536_pci.h and cs5536_vsm.h.
type U32 = u32;
type Cs5536PciVsmWrite = unsafe extern "C" fn(i32, U32);
type Cs5536PciVsmRead = unsafe extern "C" fn(i32) -> U32;

unsafe extern "C" {
    fn pci_isa_write_reg(reg: i32, value: U32);
    fn pci_ide_write_reg(reg: i32, value: U32);
    fn pci_acc_write_reg(reg: i32, value: U32);
    fn pci_ohci_write_reg(reg: i32, value: U32);
    fn pci_ehci_write_reg(reg: i32, value: U32);

    fn pci_isa_read_reg(reg: i32) -> U32;
    fn pci_ide_read_reg(reg: i32) -> U32;
    fn pci_acc_read_reg(reg: i32) -> U32;
    fn pci_ohci_read_reg(reg: i32) -> U32;
    fn pci_ehci_read_reg(reg: i32) -> U32;
}

const CS5536_FUNC_START: i32 = -1;
const CS5536_ISA_FUNC: usize = 0;
const RESERVED_FUNC: usize = 1;
const CS5536_IDE_FUNC: usize = 2;
const CS5536_ACC_FUNC: usize = 3;
const CS5536_OHCI_FUNC: usize = 4;
const CS5536_EHCI_FUNC: usize = 5;
const CS5536_FUNC_END: i32 = 6;

static VSM_CONF_WRITE: [Option<Cs5536PciVsmWrite>; 6] = [
    Some(pci_isa_write_reg),
    None,
    Some(pci_ide_write_reg),
    Some(pci_acc_write_reg),
    Some(pci_ohci_write_reg),
    Some(pci_ehci_write_reg),
];

static VSM_CONF_READ: [Option<Cs5536PciVsmRead>; 6] = [
    Some(pci_isa_read_reg),
    None,
    Some(pci_ide_read_reg),
    Some(pci_acc_read_reg),
    Some(pci_ohci_read_reg),
    Some(pci_ehci_read_reg),
];

/*
 * write to PCI config space and transfer it to MSR write.
 */
pub unsafe extern "C" fn cs5536_pci_conf_write4(function: i32, reg: i32, value: U32) {
    if (function <= CS5536_FUNC_START) || (function >= CS5536_FUNC_END) {
        return;
    }
    if (reg < 0) || (reg > 0x100) || ((reg & 0x03) != 0) {
        return;
    }

    if let Some(write_reg) = VSM_CONF_WRITE[function as usize] {
        write_reg(reg, value);
    }
}

/*
 * read PCI config space and transfer it to MSR access.
 */
pub unsafe extern "C" fn cs5536_pci_conf_read4(function: i32, reg: i32) -> U32 {
    let mut data: U32 = 0;

    if (function <= CS5536_FUNC_START) || (function >= CS5536_FUNC_END) {
        return 0;
    }
    if (reg < 0) || ((reg & 0x03) != 0) {
        return 0;
    }
    if reg > 0x100 {
        return 0xffff_ffff;
    }

    if let Some(read_reg) = VSM_CONF_READ[function as usize] {
        data = read_reg(reg);
    }

    data
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
