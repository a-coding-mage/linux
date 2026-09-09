// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 1999, 2000, 2004  MIPS Technologies, Inc.
 *	All rights reserved.
 *	Authors: Carsten Langgaard <carstenl@mips.com>
 *		 Maciej W. Rozycki <macro@mips.com>
 *
 * MIPS boards specific PCI support.
 */

// Dependencies supplied by the Linux/MIPS environment are intentionally left external.

const PCI_ACCESS_READ: u8 = 0;
const PCI_ACCESS_WRITE: u8 = 1;
const ID_SEL_BEGIN: i32 = 10;
const MAX_DEV_NUM: i32 = 31 - ID_SEL_BEGIN;

unsafe fn cfg_space_reg(offset: u32) -> *mut core::ffi::c_void {
    CKSEG1ADDR(_pcictrl_bonito_pcicfg.wrapping_add(offset)) as *mut core::ffi::c_void
}

unsafe fn bonito64_pcibios_config_access(
    access_type: u8,
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    data: *mut u32,
) -> i32 {
    let busnum: u32 = (*bus).number;
    let mut addr: u32;
    let type_: u32;
    let mut dummy: u32;
    let addrp: *mut core::ffi::c_void;
    let device: i32 = PCI_SLOT(devfn);
    let function: i32 = PCI_FUNC(devfn);
    let reg: i32 = where_ & !3;

    if busnum == 0 {
        /* Type 0 configuration for onboard PCI bus */
        if device > MAX_DEV_NUM {
            return -1;
        }

        addr = (1u32 << (device + ID_SEL_BEGIN))
            | ((function as u32) << 8)
            | (reg as u32);
        type_ = 0;
    } else {
        /* Type 1 configuration for offboard PCI bus */
        addr = (busnum << 16)
            | ((device as u32) << 11)
            | ((function as u32) << 8)
            | (reg as u32);
        type_ = 0x10000;
    }

    /* Clear aborts */
    BONITO_PCICMD |= BONITO_PCICMD_MABORT_CLR | BONITO_PCICMD_MTABORT_CLR;

    BONITO_PCIMAP_CFG = (addr >> 16) | type_;

    /* Flush Bonito register block */
    dummy = BONITO_PCIMAP_CFG;
    let _ = dummy;
    mmiowb();

    addrp = cfg_space_reg(addr & 0xffff);
    if access_type == PCI_ACCESS_WRITE {
        writel(cpu_to_le32(*data), addrp);
        /* Wait till done */
        while BONITO_PCIMSTAT & 0xF != 0 {}
    } else {
        *data = le32_to_cpu(readl(addrp));
    }

    /* Detect Master/Target abort */
    if BONITO_PCICMD & (BONITO_PCICMD_MABORT_CLR | BONITO_PCICMD_MTABORT_CLR) != 0 {
        /* Error occurred */

        /* Clear bits */
        BONITO_PCICMD |= BONITO_PCICMD_MABORT_CLR | BONITO_PCICMD_MTABORT_CLR;

        return -1;
    }

    0
}

/*
 * We can't address 8 and 16 bit words directly.  Instead we have to
 * read/write a 32bit word and mask/modify the data we actually want.
 */
unsafe fn bonito64_pcibios_read(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: *mut u32,
) -> i32 {
    let mut data: u32 = 0;

    if size == 2 && where_ & 1 != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    } else if size == 4 && where_ & 3 != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }

    if bonito64_pcibios_config_access(PCI_ACCESS_READ, bus, devfn, where_, &mut data) != 0 {
        return -1;
    }

    if size == 1 {
        *val = (data >> ((where_ & 3) << 3)) & 0xff;
    } else if size == 2 {
        *val = (data >> ((where_ & 3) << 3)) & 0xffff;
    } else {
        *val = data;
    }

    PCIBIOS_SUCCESSFUL
}

unsafe fn bonito64_pcibios_write(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: u32,
) -> i32 {
    let mut data: u32 = 0;

    if size == 2 && where_ & 1 != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    } else if size == 4 && where_ & 3 != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }

    if size == 4 {
        data = val;
    } else {
        if bonito64_pcibios_config_access(PCI_ACCESS_READ, bus, devfn, where_, &mut data) != 0 {
            return -1;
        }

        if size == 1 {
            data = (data & !(0xff << ((where_ & 3) << 3)))
                | (val << ((where_ & 3) << 3));
        } else if size == 2 {
            data = (data & !(0xffff << ((where_ & 3) << 3)))
                | (val << ((where_ & 3) << 3));
        }
    }

    if bonito64_pcibios_config_access(PCI_ACCESS_WRITE, bus, devfn, where_, &mut data) != 0 {
        return -1;
    }

    PCIBIOS_SUCCESSFUL
}

#[repr(C)]
pub struct pci_ops {
    pub read: unsafe fn(*mut pci_bus, u32, i32, i32, *mut u32) -> i32,
    pub write: unsafe fn(*mut pci_bus, u32, i32, i32, u32) -> i32,
}

pub static mut bonito64_pci_ops: pci_ops = pci_ops {
    read: bonito64_pcibios_read,
    write: bonito64_pcibios_write,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
