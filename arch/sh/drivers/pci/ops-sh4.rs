// SPDX-License-Identifier: GPL-2.0
/*
 * Generic SH-4 / SH-4A PCIC operations (SH7751, SH7780).
 *
 * Copyright (C) 2002 - 2009  Paul Mundt
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/pci.h, linux/io.h, linux/spinlock.h, asm/addrspace.h, pci-sh4.h

/*
 * Direct access to PCI hardware...
 */
const fn config_cmd(bus: *mut pci_bus, devfn: u32, where_: i32) -> u32 {
    0x8000_0000
        | ((*bus).number << 16)
        | (devfn << 8)
        | ((where_ as u32) & !3)
}

/*
 * Functions for accessing PCI configuration space with type 1 accesses
 */
unsafe fn sh4_pci_read(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: *mut u32,
) -> i32 {
    let chan = (*bus).sysdata;
    let mut flags: usize = 0;
    let mut data: u32;

    /*
     * PCIPDR may only be accessed as 32 bit words,
     * so we must do byte alignment by hand
     */
    raw_spin_lock_irqsave(&raw mut pci_config_lock, &mut flags);
    pci_write_reg(chan, config_cmd(bus, devfn, where_), SH4_PCIPAR);
    data = pci_read_reg(chan, SH4_PCIPDR);
    raw_spin_unlock_irqrestore(&raw mut pci_config_lock, flags);

    match size {
        1 => {
            *val = (data >> (((where_ & 3) << 3) as u32)) & 0xff;
        }
        2 => {
            *val = (data >> (((where_ & 2) << 3) as u32)) & 0xffff;
        }
        4 => {
            *val = data;
        }
        _ => return PCIBIOS_FUNC_NOT_SUPPORTED,
    }

    PCIBIOS_SUCCESSFUL
}

/*
 * Since SH4 only does 32bit access we'll have to do a read,
 * mask,write operation.
 * We'll allow an odd byte offset, though it should be illegal.
 */
unsafe fn sh4_pci_write(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: u32,
) -> i32 {
    let chan = (*bus).sysdata;
    let mut flags: usize = 0;
    let mut shift: u32;
    let mut data: u32;

    raw_spin_lock_irqsave(&raw mut pci_config_lock, &mut flags);
    pci_write_reg(chan, config_cmd(bus, devfn, where_), SH4_PCIPAR);
    data = pci_read_reg(chan, SH4_PCIPDR);
    raw_spin_unlock_irqrestore(&raw mut pci_config_lock, flags);

    match size {
        1 => {
            shift = ((where_ & 3) << 3) as u32;
            data &= !(0xff << shift);
            data |= (val & 0xff) << shift;
        }
        2 => {
            shift = ((where_ & 2) << 3) as u32;
            data &= !(0xffff << shift);
            data |= (val & 0xffff) << shift;
        }
        4 => {
            data = val;
        }
        _ => return PCIBIOS_FUNC_NOT_SUPPORTED,
    }

    pci_write_reg(chan, data, SH4_PCIPDR);

    PCIBIOS_SUCCESSFUL
}

pub static mut sh4_pci_ops: pci_ops = pci_ops {
    read: Some(sh4_pci_read),
    write: Some(sh4_pci_write),
};

pub unsafe extern "C" fn pci_fixup_pcic(_chan: *mut pci_channel) -> i32 {
    /* Nothing to do. */
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
