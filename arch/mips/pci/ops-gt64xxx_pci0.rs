// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 1999, 2000, 2004  MIPS Technologies, Inc.
 *	All rights reserved.
 *	Authors: Carsten Langgaard <carstenl@mips.com>
 *		 Maciej W. Rozycki <macro@mips.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const PCI_ACCESS_READ: u8 = 0;
const PCI_ACCESS_WRITE: u8 = 1;

/*
 *  PCI configuration cycle AD bus definition
 */
/* Type 0 */
const PCI_CFG_TYPE0_REG_SHF: u32 = 0;
const PCI_CFG_TYPE0_FUNC_SHF: u32 = 8;

/* Type 1 */
const PCI_CFG_TYPE1_REG_SHF: u32 = 0;
const PCI_CFG_TYPE1_FUNC_SHF: u32 = 8;
const PCI_CFG_TYPE1_DEV_SHF: u32 = 11;
const PCI_CFG_TYPE1_BUS_SHF: u32 = 16;

unsafe fn gt64xxx_pci0_pcibios_config_access(
    access_type: u8,
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    data: *mut u32,
) -> i32 {
    let busnum: u8 = (*bus).number;
    let mut intr: u32;

    if busnum == 0 && devfn >= PCI_DEVFN(31, 0) {
        return -1; /* Because of a bug in the galileo (for slot 31). */
    }

    /* Clear cause register bits */
    GT_WRITE(
        GT_INTRCAUSE_OFS,
        !(GT_INTRCAUSE_MASABORT0_BIT | GT_INTRCAUSE_TARABORT0_BIT),
    );

    /* Setup address */
    GT_WRITE(
        GT_PCI0_CFGADDR_OFS,
        ((busnum as u32) << GT_PCI0_CFGADDR_BUSNUM_SHF)
            | (devfn << GT_PCI0_CFGADDR_FUNCTNUM_SHF)
            | (((where_ / 4) as u32) << GT_PCI0_CFGADDR_REGNUM_SHF)
            | GT_PCI0_CFGADDR_CONFIGEN_BIT,
    );

    if access_type == PCI_ACCESS_WRITE {
        if busnum == 0 && PCI_SLOT(devfn) == 0 {
            /*
             * The Galileo system controller is acting
             * differently than other devices.
             */
            GT_WRITE(GT_PCI0_CFGDATA_OFS, *data);
        } else {
            __GT_WRITE(GT_PCI0_CFGDATA_OFS, *data);
        }
    } else if busnum == 0 && PCI_SLOT(devfn) == 0 {
        /*
         * The Galileo system controller is acting
         * differently than other devices.
         */
        *data = GT_READ(GT_PCI0_CFGDATA_OFS);
    } else {
        *data = __GT_READ(GT_PCI0_CFGDATA_OFS);
    }

    /* Check for master or target abort */
    intr = GT_READ(GT_INTRCAUSE_OFS);

    if intr & (GT_INTRCAUSE_MASABORT0_BIT | GT_INTRCAUSE_TARABORT0_BIT) != 0 {
        /* Error occurred */

        /* Clear bits */
        GT_WRITE(
            GT_INTRCAUSE_OFS,
            !(GT_INTRCAUSE_MASABORT0_BIT | GT_INTRCAUSE_TARABORT0_BIT),
        );

        return -1;
    }

    0
}

/*
 * We can't address 8 and 16 bit words directly.  Instead we have to
 * read/write a 32bit word and mask/modify the data we actually want.
 */
unsafe fn gt64xxx_pci0_pcibios_read(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: *mut u32,
) -> i32 {
    let mut data: u32 = 0;

    if gt64xxx_pci0_pcibios_config_access(PCI_ACCESS_READ, bus, devfn, where_, &mut data) != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }

    if size == 1 {
        *val = (data >> (((where_ & 3) << 3) as u32)) & 0xff;
    } else if size == 2 {
        *val = (data >> (((where_ & 3) << 3) as u32)) & 0xffff;
    } else {
        *val = data;
    }

    PCIBIOS_SUCCESSFUL
}

unsafe fn gt64xxx_pci0_pcibios_write(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: u32,
) -> i32 {
    let mut data: u32 = 0;

    if size == 4 {
        data = val;
    } else {
        if gt64xxx_pci0_pcibios_config_access(PCI_ACCESS_READ, bus, devfn, where_, &mut data) != 0 {
            return PCIBIOS_DEVICE_NOT_FOUND;
        }

        if size == 1 {
            let shift = ((where_ & 3) << 3) as u32;
            data = (data & !(0xffu32 << shift)) | (val << shift);
        } else if size == 2 {
            let shift = ((where_ & 3) << 3) as u32;
            data = (data & !(0xffffu32 << shift)) | (val << shift);
        }
    }

    if gt64xxx_pci0_pcibios_config_access(PCI_ACCESS_WRITE, bus, devfn, where_, &mut data) != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }

    PCIBIOS_SUCCESSFUL
}

pub static mut gt64xxx_pci0_ops: pci_ops = pci_ops {
    read: Some(gt64xxx_pci0_pcibios_read),
    write: Some(gt64xxx_pci0_pcibios_write),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
