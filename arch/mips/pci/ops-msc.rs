// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 1999, 2000, 2004, 2005  MIPS Technologies, Inc.
 *    All rights reserved.
 *    Authors: Carsten Langgaard <carstenl@mips.com>
 *             Maciej W. Rozycki <macro@mips.com>
 * Copyright (C) 2005 Ralf Baechle (ralf@linux-mips.org)
 *
 * MIPS boards specific PCI support.
 */

// Dependencies supplied by the surrounding kernel translation.

const PCI_ACCESS_READ: u8 = 0;
const PCI_ACCESS_WRITE: u8 = 1;

/* PCI configuration cycle AD bus definition */
/* Type 0 */
const PCI_CFG_TYPE0_REG_SHF: u32 = 0;
const PCI_CFG_TYPE0_FUNC_SHF: u32 = 8;

/* Type 1 */
const PCI_CFG_TYPE1_REG_SHF: u32 = 0;
const PCI_CFG_TYPE1_FUNC_SHF: u32 = 8;
const PCI_CFG_TYPE1_DEV_SHF: u32 = 11;
const PCI_CFG_TYPE1_BUS_SHF: u32 = 16;

unsafe fn msc_pcibios_config_access(
    access_type: u8,
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    data: *mut u32,
) -> i32 {
    let busnum: u8 = (*bus).number;
    let mut intr: u32 = 0;

    /* Clear status register bits. */
    MSC_WRITE(
        MSC01_PCI_INTSTAT,
        MSC01_PCI_INTCFG_MA_BIT | MSC01_PCI_INTCFG_TA_BIT,
    );

    MSC_WRITE(
        MSC01_PCI_CFGADDR,
        ((busnum as u32) << MSC01_PCI_CFGADDR_BNUM_SHF)
            | (((devfn >> 3) & 0x1f) << MSC01_PCI_CFGADDR_DNUM_SHF)
            | ((devfn & 0x7) << MSC01_PCI_CFGADDR_FNUM_SHF)
            | (((where_ / 4) as u32) << MSC01_PCI_CFGADDR_RNUM_SHF),
    );

    /* Perform access */
    if access_type == PCI_ACCESS_WRITE {
        MSC_WRITE(MSC01_PCI_CFGDATA, *data);
    } else {
        MSC_READ(MSC01_PCI_CFGDATA, data);
    }

    /* Detect Master/Target abort */
    MSC_READ(MSC01_PCI_INTSTAT, &mut intr);
    if intr & (MSC01_PCI_INTCFG_MA_BIT | MSC01_PCI_INTCFG_TA_BIT) != 0 {
        /* Error occurred */

        /* Clear bits */
        MSC_WRITE(
            MSC01_PCI_INTSTAT,
            MSC01_PCI_INTCFG_MA_BIT | MSC01_PCI_INTCFG_TA_BIT,
        );

        return -1;
    }

    0
}

/*
 * We can't address 8 and 16 bit words directly. Instead we have to
 * read/write a 32bit word and mask/modify the data we actually want.
 */
unsafe fn msc_pcibios_read(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: *mut u32,
) -> i32 {
    let mut data: u32 = 0;

    if size == 2 && (where_ & 1) != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    } else if size == 4 && (where_ & 3) != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }

    if msc_pcibios_config_access(PCI_ACCESS_READ, bus, devfn, where_, &mut data) != 0 {
        return -1;
    }

    if size == 1 {
        *val = (data >> (((where_ & 3) as u32) << 3)) & 0xff;
    } else if size == 2 {
        *val = (data >> (((where_ & 3) as u32) << 3)) & 0xffff;
    } else {
        *val = data;
    }

    PCIBIOS_SUCCESSFUL
}

unsafe fn msc_pcibios_write(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: u32,
) -> i32 {
    let mut data: u32 = 0;

    if size == 2 && (where_ & 1) != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    } else if size == 4 && (where_ & 3) != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }

    if size == 4 {
        data = val;
    } else {
        if msc_pcibios_config_access(PCI_ACCESS_READ, bus, devfn, where_, &mut data) != 0 {
            return -1;
        }

        if size == 1 {
            let shift = ((where_ & 3) as u32) << 3;
            data = (data & !(0xff << shift)) | (val << shift);
        } else if size == 2 {
            let shift = ((where_ & 3) as u32) << 3;
            data = (data & !(0xffff << shift)) | (val << shift);
        }
    }

    if msc_pcibios_config_access(PCI_ACCESS_WRITE, bus, devfn, where_, &mut data) != 0 {
        return -1;
    }

    PCIBIOS_SUCCESSFUL
}

pub static mut msc_pci_ops: pci_ops = pci_ops {
    read: Some(msc_pcibios_read),
    write: Some(msc_pcibios_write),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
