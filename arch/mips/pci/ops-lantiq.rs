// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2010 John Crispin <john@phrozen.org>
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct pci_bus {
    pub number: u8,
}

pub type u32_t = u32;

pub const LTQ_PCI_CFG_BUSNUM_SHF: u32 = 16;
pub const LTQ_PCI_CFG_DEVNUM_SHF: u32 = 11;
pub const LTQ_PCI_CFG_FUNNUM_SHF: u32 = 8;

pub const PCI_ACCESS_READ: u8 = 0;
pub const PCI_ACCESS_WRITE: u8 = 1;

extern "C" {
    static mut ebu_lock: u8;
    static mut ltq_pci_mapped_cfg: *mut u8;

    fn spin_lock_irqsave(lock: *mut u8, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut u8, flags: usize);
    fn ltq_w32(value: u32, address: *mut u32);
    fn ltq_r32(address: *const u32) -> u32;
    fn swab32(value: u32) -> u32;
    fn wmb();
}

pub const PCIBIOS_DEVICE_NOT_FOUND: i32 = 0x86;
pub const PCIBIOS_SUCCESSFUL: i32 = 0;

unsafe fn ltq_pci_config_access(
    access_type: u8,
    bus: *mut pci_bus,
    devfn: u32,
    where_: u32,
    data: *mut u32,
) -> i32 {
    let mut cfg_base: usize;
    let mut flags: usize = 0;
    let mut temp: u32;

    /* we support slot from 0 to 15 dev_fn & 0x68 (AD29) is the
       SoC itself */
    if ((*bus).number != 0)
        || ((devfn & 0xf8) > 0x78)
        || ((devfn & 0xf8) == 0)
        || ((devfn & 0xf8) == 0x68)
    {
        return 1;
    }

    spin_lock_irqsave((&mut ebu_lock) as *mut u8, &mut flags);

    cfg_base = ltq_pci_mapped_cfg as usize;
    cfg_base |= (((*bus).number as u32) << LTQ_PCI_CFG_BUSNUM_SHF) as usize
        | (devfn << LTQ_PCI_CFG_FUNNUM_SHF) as usize
        | (where_ & !0x3) as usize;

    /* Perform access */
    if access_type == PCI_ACCESS_WRITE {
        ltq_w32(swab32(*data), cfg_base as *mut u32);
    } else {
        *data = ltq_r32(cfg_base as *const u32);
        *data = swab32(*data);
    }
    wmb();

    /* clean possible Master abort */
    cfg_base = ltq_pci_mapped_cfg as usize;
    cfg_base |= (0u32 << LTQ_PCI_CFG_FUNNUM_SHF) as usize + 4;
    temp = ltq_r32(cfg_base as *const u32);
    temp = swab32(temp);
    cfg_base = ltq_pci_mapped_cfg as usize;
    cfg_base |= (0x68u32 << LTQ_PCI_CFG_FUNNUM_SHF) as usize + 4;
    ltq_w32(temp, cfg_base as *mut u32);

    spin_unlock_irqrestore((&mut ebu_lock) as *mut u8, flags);

    if *data == 0xffff_ffff && access_type == PCI_ACCESS_READ {
        return 1;
    }

    0
}

pub unsafe fn ltq_pci_read_config_dword(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: *mut u32,
) -> i32 {
    let mut data: u32 = 0;

    if ltq_pci_config_access(PCI_ACCESS_READ, bus, devfn, where_ as u32, &mut data) != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }

    if size == 1 {
        *val = (data >> (((where_ as u32) & 3) << 3)) & 0xff;
    } else if size == 2 {
        *val = (data >> (((where_ as u32) & 3) << 3)) & 0xffff;
    } else {
        *val = data;
    }

    PCIBIOS_SUCCESSFUL
}

pub unsafe fn ltq_pci_write_config_dword(
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
        if ltq_pci_config_access(PCI_ACCESS_READ, bus, devfn, where_ as u32, &mut data) != 0 {
            return PCIBIOS_DEVICE_NOT_FOUND;
        }

        if size == 1 {
            data = (data & !(0xff << (((where_ as u32) & 3) << 3)))
                | (val << (((where_ as u32) & 3) << 3));
        } else if size == 2 {
            data = (data & !(0xffff << (((where_ as u32) & 3) << 3)))
                | (val << (((where_ as u32) & 3) << 3));
        }
    }

    if ltq_pci_config_access(PCI_ACCESS_WRITE, bus, devfn, where_ as u32, &mut data) != 0 {
        return PCIBIOS_DEVICE_NOT_FOUND;
    }

    PCIBIOS_SUCCESSFUL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
