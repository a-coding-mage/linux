// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2001,2002,2005 Broadcom Corporation
 * Copyright (C) 2004 by Ralf Baechle (ralf@linux-mips.org)
 */

/*
 * BCM1480/1455-specific HT support (looking like PCI)
 *
 * This module provides the glue between Linux's PCI subsystem
 * and the hardware.  We basically provide glue for accessing
 * configuration space, and set up the translation for I/O
 * space accesses.
 */

// Macros for calculating offsets into config space given a device structure or dev/fun/reg.
#[inline]
const fn cfgoffset(bus: u32, devfn: u32, where_: u32) -> u32 {
    (bus << 16) + (devfn << 8) + where_
}

#[inline]
unsafe fn cfgaddr(bus: *mut pci_bus, devfn: u32, where_: u32) -> u32 {
    cfgoffset((*bus).number as u32, devfn, where_)
}

static mut ht_cfg_space: *mut core::ffi::c_void = core::ptr::null_mut();

const PCI_BUS_ENABLED: i32 = 1;
const PCI_DEVICE_MODE: i32 = 2;

static mut bcm1480ht_bus_status: i32 = 0;

const PCI_BRIDGE_DEVICE: i32 = 0;
const HT_BRIDGE_DEVICE: i32 = 1;

/* HT's level-sensitive interrupts require EOI, generated through a 4MB mapped region. */
pub static mut ht_eoi_space: u64 = 0;

/* Read/write 32-bit values in config space. */
#[inline]
unsafe fn readcfg32(addr: u32) -> u32 {
    *((ht_cfg_space as *mut u8).add((addr & !3) as usize) as *mut u32)
}

#[inline]
unsafe fn writecfg32(addr: u32, data: u32) {
    *((ht_cfg_space as *mut u8).add((addr & !3) as usize) as *mut u32) = data;
}

/* Some checks before doing config cycles. */
unsafe fn bcm1480ht_can_access(bus: *mut pci_bus, devfn: i32) -> i32 {
    let mut devno: u32;

    if (bcm1480ht_bus_status & (PCI_BUS_ENABLED | PCI_DEVICE_MODE)) == 0 {
        return 0;
    }

    if (*bus).number == 0 {
        devno = ((devfn as u32) >> 3) & 0x1f;
        let _ = devno;
        if (bcm1480ht_bus_status & PCI_DEVICE_MODE) != 0 {
            return 0;
        }
    }
    1
}

/* Read/write access functions for various sizes of values in config space. */
unsafe fn bcm1480ht_pcibios_read(
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

    if bcm1480ht_can_access(bus, devfn as i32) != 0 {
        data = readcfg32(cfgaddr(bus, devfn, where_ as u32));
    } else {
        data = 0xffff_ffff;
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

unsafe fn bcm1480ht_pcibios_write(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: u32,
) -> i32 {
    let cfgaddr_ = cfgaddr(bus, devfn, where_ as u32);
    let mut data: u32 = 0;

    if size == 2 && (where_ & 1) != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    } else if size == 4 && (where_ & 3) != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }

    if bcm1480ht_can_access(bus, devfn as i32) == 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }

    data = readcfg32(cfgaddr_);
    let shift = ((where_ & 3) << 3) as u32;
    if size == 1 {
        data = (data & !(0xff << shift)) | (val << shift);
    } else if size == 2 {
        data = (data & !(0xffff << shift)) | (val << shift);
    } else {
        data = val;
    }
    writecfg32(cfgaddr_, data);
    PCIBIOS_SUCCESSFUL
}

unsafe fn bcm1480ht_pcibios_get_busno() -> i32 { 0 }

pub static mut bcm1480ht_pci_ops: pci_ops = pci_ops {
    read: Some(bcm1480ht_pcibios_read),
    write: Some(bcm1480ht_pcibios_write),
};

static mut bcm1480ht_mem_resource: resource = resource {
    name: b"BCM1480 HT MEM\0".as_ptr() as *const i8,
    start: A_BCM1480_PHYS_HT_MEM_MATCH_BYTES,
    end: A_BCM1480_PHYS_HT_MEM_MATCH_BYTES + 0x1fffffff,
    flags: IORESOURCE_MEM,
};

static mut bcm1480ht_io_resource: resource = resource {
    name: b"BCM1480 HT I/O\0".as_ptr() as *const i8,
    start: A_BCM1480_PHYS_HT_IO_MATCH_BYTES,
    end: A_BCM1480_PHYS_HT_IO_MATCH_BYTES + 0x01ffffff,
    flags: IORESOURCE_IO,
};

pub static mut bcm1480ht_controller: pci_controller = pci_controller {
    pci_ops: &raw mut bcm1480ht_pci_ops,
    mem_resource: &raw mut bcm1480ht_mem_resource,
    io_resource: &raw mut bcm1480ht_io_resource,
    index: 1,
    get_busno: Some(bcm1480ht_pcibios_get_busno),
    io_offset: A_BCM1480_PHYS_HT_IO_MATCH_BYTES,
    ..pci_controller::ZERO
};

unsafe fn bcm1480ht_pcibios_init() -> i32 {
    ht_cfg_space = ioremap(A_BCM1480_PHYS_HT_CFG_MATCH_BITS, 16 * 1024 * 1024);

    /* CFE doesn't always init all HT paths, so we always scan. */
    bcm1480ht_bus_status |= PCI_BUS_ENABLED;

    ht_eoi_space = ioremap(A_BCM1480_PHYS_HT_SPECIAL_MATCH_BYTES, 4 * 1024 * 1024) as u64;
    bcm1480ht_controller.io_map_base =
        ioremap(A_BCM1480_PHYS_HT_IO_MATCH_BYTES, 65536) as u64;
    bcm1480ht_controller.io_map_base -= bcm1480ht_controller.io_offset;

    register_pci_controller(&raw mut bcm1480ht_controller);
    0
}

// arch_initcall(bcm1480ht_pcibios_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
