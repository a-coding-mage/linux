// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Support for indirect PCI bridges.
 *
 * Copyright (C) 1998 Gabriel Paubert.
 */

// C header dependencies are supplied by the surrounding kernel translation.

pub unsafe fn __indirect_read_config(
    hose: *mut pci_controller,
    bus_number: u8,
    devfn: u32,
    offset: i32,
    len: i32,
    val: *mut u32,
) -> i32 {
    let mut cfg_data: *mut u8;
    let mut cfg_type: u8 = 0;
    let bus_no: u32;
    let reg: u32;

    if (*hose).indirect_type & PPC_INDIRECT_TYPE_NO_PCIE_LINK != 0 {
        if bus_number != (*hose).first_busno {
            return PCIBIOS_DEVICE_NOT_FOUND;
        }
        if devfn != 0 {
            return PCIBIOS_DEVICE_NOT_FOUND;
        }
    }

    if let Some(exclude_device) = ppc_md.pci_exclude_device {
        if exclude_device(hose, bus_number, devfn) != 0 {
            return PCIBIOS_DEVICE_NOT_FOUND;
        }
    }

    if (*hose).indirect_type & PPC_INDIRECT_TYPE_SET_CFG_TYPE != 0
        && bus_number != (*hose).first_busno
    {
        cfg_type = 1;
    }

    bus_no = if bus_number == (*hose).first_busno {
        (*hose).self_busno
    } else {
        bus_number as u32
    };

    if (*hose).indirect_type & PPC_INDIRECT_TYPE_EXT_REG != 0 {
        reg = (((offset as u32) & 0xf00) << 16) | ((offset as u32) & 0xfc);
    } else {
        reg = (offset as u32) & 0xfc;
    }

    let address = 0x80000000u32 | (bus_no << 16) | (devfn << 8) | reg | cfg_type as u32;
    if (*hose).indirect_type & PPC_INDIRECT_TYPE_BIG_ENDIAN != 0 {
        out_be32((*hose).cfg_addr, address);
    } else {
        out_le32((*hose).cfg_addr, address);
    }

    /*
     * Note: the caller has already checked that offset is
     * suitably aligned and that len is 1, 2 or 4.
     */
    cfg_data = (*hose).cfg_data.add((offset & 3) as usize);
    match len {
        1 => *val = in_8(cfg_data) as u32,
        2 => *val = in_le16(cfg_data) as u32,
        _ => *val = in_le32(cfg_data),
    }
    PCIBIOS_SUCCESSFUL
}

pub unsafe fn indirect_read_config(
    bus: *mut pci_bus,
    devfn: u32,
    offset: i32,
    len: i32,
    val: *mut u32,
) -> i32 {
    let hose = pci_bus_to_host(bus);
    __indirect_read_config(hose, (*bus).number, devfn, offset, len, val)
}

pub unsafe fn indirect_write_config(
    bus: *mut pci_bus,
    devfn: u32,
    offset: i32,
    len: i32,
    mut val: u32,
) -> i32 {
    let hose = pci_bus_to_host(bus);
    let mut cfg_data: *mut u8;
    let mut cfg_type: u8 = 0;
    let bus_no: u32;
    let reg: u32;

    if (*hose).indirect_type & PPC_INDIRECT_TYPE_NO_PCIE_LINK != 0 {
        if (*bus).number != (*hose).first_busno || devfn != 0 {
            return PCIBIOS_DEVICE_NOT_FOUND;
        }
    }
    if let Some(exclude_device) = ppc_md.pci_exclude_device {
        if exclude_device(hose, (*bus).number, devfn) != 0 {
            return PCIBIOS_DEVICE_NOT_FOUND;
        }
    }
    if (*hose).indirect_type & PPC_INDIRECT_TYPE_SET_CFG_TYPE != 0
        && (*bus).number != (*hose).first_busno
    {
        cfg_type = 1;
    }
    bus_no = if (*bus).number == (*hose).first_busno { (*hose).self_busno } else { (*bus).number as u32 };
    reg = if (*hose).indirect_type & PPC_INDIRECT_TYPE_EXT_REG != 0 {
        (((offset as u32) & 0xf00) << 16) | ((offset as u32) & 0xfc)
    } else { (offset as u32) & 0xfc };
    let address = 0x80000000u32 | (bus_no << 16) | (devfn << 8) | reg | cfg_type as u32;
    if (*hose).indirect_type & PPC_INDIRECT_TYPE_BIG_ENDIAN != 0 { out_be32((*hose).cfg_addr, address); } else { out_le32((*hose).cfg_addr, address); }

    /* suppress setting of PCI_PRIMARY_BUS */
    if (*hose).indirect_type & PPC_INDIRECT_TYPE_SURPRESS_PRIMARY_BUS != 0
        && offset == PCI_PRIMARY_BUS && (*bus).number == (*hose).first_busno
    { val &= 0xffffff00; }
    /* Workaround for PCI_28 Errata in 440EPx/GRx */
    if (*hose).indirect_type & PPC_INDIRECT_TYPE_BROKEN_MRM != 0 && offset == PCI_CACHE_LINE_SIZE { val = 0; }

    cfg_data = (*hose).cfg_data.add((offset & 3) as usize);
    match len { 1 => out_8(cfg_data, val), 2 => out_le16(cfg_data, val), _ => out_le32(cfg_data, val) }
    PCIBIOS_SUCCESSFUL
}

static mut indirect_pci_ops: pci_ops = pci_ops { read: indirect_read_config, write: indirect_write_config };

pub unsafe fn setup_indirect_pci(hose: *mut pci_controller, cfg_addr: resource_size_t, cfg_data: resource_size_t, flags: u32) {
    let base = cfg_addr & PAGE_MASK;
    let mut mbase = ioremap(base, PAGE_SIZE);
    (*hose).cfg_addr = mbase.add((cfg_addr & !PAGE_MASK) as usize);
    if (cfg_data & PAGE_MASK) != base { mbase = ioremap(cfg_data & PAGE_MASK, PAGE_SIZE); }
    (*hose).cfg_data = mbase.add((cfg_data & !PAGE_MASK) as usize);
    (*hose).ops = &mut indirect_pci_ops;
    (*hose).indirect_type = flags;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
