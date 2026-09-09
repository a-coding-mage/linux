// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2001,2002,2005 Broadcom Corporation
 * Copyright (C) 2004 by Ralf Baechle (ralf@linux-mips.org)
 */

/* BCM1x80/1x55-specific PCI support. */

// C dependencies supplied by the surrounding kernel translation unit.

const PCI_BUS_ENABLED: i32 = 1;
const PCI_DEVICE_MODE: i32 = 2;
const PCI_BRIDGE_DEVICE: i32 = 0;

static mut cfg_space: *mut core::ffi::c_void = core::ptr::null_mut();
static mut bcm1480_bus_status: i32 = 0;

#[inline]
unsafe fn READCFG32(addr: u32) -> u32 {
    *((cfg_space as *mut u8).add((addr & !3) as usize) as *mut u32)
}

#[inline]
unsafe fn WRITECFG32(addr: u32, data: u32) {
    *((cfg_space as *mut u8).add((addr & !3) as usize) as *mut u32) = data;
}

#[inline]
fn CFGOFFSET(bus: u32, devfn: u32, where_: u32) -> u32 {
    (bus << 16) + (devfn << 8) + where_
}

#[inline]
unsafe fn CFGADDR(bus: *const pci_bus, devfn: u32, where_: u32) -> u32 {
    CFGOFFSET((*bus).number as u32, devfn, where_)
}

pub unsafe extern "C" fn pcibios_map_irq(_dev: *const pci_dev, _slot: u8, pin: u8) -> i32 {
    if pin == 0 {
        return -1;
    }
    K_BCM1480_INT_PCI_INTA - 1 + pin as i32
}

pub unsafe extern "C" fn pcibios_plat_dev_init(_dev: *mut pci_dev) -> i32 {
    0
}

unsafe fn bcm1480_pci_can_access(bus: *const pci_bus, devfn: i32) -> i32 {
    let devno: i32;
    if (bcm1480_bus_status & (PCI_BUS_ENABLED | PCI_DEVICE_MODE)) == 0 {
        return 0;
    }
    if (*bus).number == 0 {
        devno = PCI_SLOT(devfn);
        let _ = devno;
        if (bcm1480_bus_status & PCI_DEVICE_MODE) != 0 {
            0
        } else {
            1
        }
    } else {
        1
    }
}

unsafe extern "C" fn bcm1480_pcibios_read(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: *mut u32,
) -> i32 {
    let mut data: u32 = 0;
    if size == 2 && (where_ & 1) != 0 || size == 4 && (where_ & 3) != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }
    if bcm1480_pci_can_access(bus, devfn as i32) != 0 {
        data = READCFG32(CFGADDR(bus, devfn, where_ as u32));
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

unsafe extern "C" fn bcm1480_pcibios_write(
    bus: *mut pci_bus,
    devfn: u32,
    where_: i32,
    size: i32,
    val: u32,
) -> i32 {
    let cfgaddr = CFGADDR(bus, devfn, where_ as u32);
    if size == 2 && (where_ & 1) != 0 || size == 4 && (where_ & 3) != 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }
    if bcm1480_pci_can_access(bus, devfn as i32) == 0 {
        return PCIBIOS_BAD_REGISTER_NUMBER;
    }
    let mut data = READCFG32(cfgaddr);
    if size == 1 {
        let shift = ((where_ & 3) << 3) as u32;
        data = (data & !(0xff << shift)) | (val << shift);
    } else if size == 2 {
        let shift = ((where_ & 3) << 3) as u32;
        data = (data & !(0xffff << shift)) | (val << shift);
    } else {
        data = val;
    }
    WRITECFG32(cfgaddr, data);
    PCIBIOS_SUCCESSFUL
}

#[repr(C)]
pub struct pci_ops {
    pub read: unsafe extern "C" fn(*mut pci_bus, u32, i32, i32, *mut u32) -> i32,
    pub write: unsafe extern "C" fn(*mut pci_bus, u32, i32, i32, u32) -> i32,
}

#[no_mangle]
pub static mut bcm1480_pci_ops: pci_ops = pci_ops {
    read: bcm1480_pcibios_read,
    write: bcm1480_pcibios_write,
};

#[no_mangle]
pub static mut bcm1480_mem_resource: resource = resource {
    name: b"BCM1480 PCI MEM\0".as_ptr() as *const i8,
    start: A_BCM1480_PHYS_PCI_MEM_MATCH_BYTES,
    end: A_BCM1480_PHYS_PCI_MEM_MATCH_BYTES + 0xfffffff,
    flags: IORESOURCE_MEM,
};

#[no_mangle]
pub static mut bcm1480_io_resource: resource = resource {
    name: b"BCM1480 PCI I/O\0".as_ptr() as *const i8,
    start: A_BCM1480_PHYS_PCI_IO_MATCH_BYTES,
    end: A_BCM1480_PHYS_PCI_IO_MATCH_BYTES + 0x1ffffff,
    flags: IORESOURCE_IO,
};

#[no_mangle]
pub static mut bcm1480_controller: pci_controller = pci_controller {
    pci_ops: &raw mut bcm1480_pci_ops,
    mem_resource: &raw mut bcm1480_mem_resource,
    io_resource: &raw mut bcm1480_io_resource,
    io_offset: A_BCM1480_PHYS_PCI_IO_MATCH_BYTES,
    io_map_base: 0,
};

unsafe extern "C" fn bcm1480_pcibios_init() -> i32 {
    let mut cmdreg: u32;
    let mut reg: u64;

    pci_set_flags(PCI_PROBE_ONLY);
    PCIBIOS_MIN_IO = 0x00008000;
    PCIBIOS_MIN_MEM = 0x01000000;
    ioport_resource.end = 0xffff_ffff;
    iomem_resource.end = 0xffff_ffff;

    cfg_space = ioremap(A_BCM1480_PHYS_PCI_CFG_MATCH_BITS, 16 * 1024 * 1024);
    reg = __raw_readq(IOADDR(A_SCD_SYSTEM_CFG));
    if (reg & M_BCM1480_SYS_PCI_HOST) == 0 {
        bcm1480_bus_status |= PCI_DEVICE_MODE;
    } else {
        cmdreg = READCFG32(CFGOFFSET(0, PCI_DEVFN(PCI_BRIDGE_DEVICE, 0), PCI_COMMAND));
        if (cmdreg & PCI_COMMAND_MASTER) == 0 {
            printk(b"PCI: Skipping PCI probe.\tBus is not initialized.\n\0".as_ptr() as *const i8);
            iounmap(cfg_space);
            return 1;
        }
        bcm1480_bus_status |= PCI_BUS_ENABLED;
    }

    cmdreg = READCFG32(CFGOFFSET(0, PCI_DEVFN(PCI_BRIDGE_DEVICE, 0), 0x40));
    WRITECFG32(CFGOFFSET(0, PCI_DEVFN(PCI_BRIDGE_DEVICE, 0), 0x40), cmdreg | 0x10);
    cmdreg = READCFG32(CFGOFFSET(0, PCI_DEVFN(PCI_BRIDGE_DEVICE, 0), 0x40));
    let _ = cmdreg;

    bcm1480_controller.io_map_base = ioremap(A_BCM1480_PHYS_PCI_IO_MATCH_BYTES, 65536) as unsigned_long;
    bcm1480_controller.io_map_base -= bcm1480_controller.io_offset;
    set_io_port_base(bcm1480_controller.io_map_base);
    register_pci_controller(&raw mut bcm1480_controller);

    #[cfg(CONFIG_VGA_CONSOLE)]
    {
        console_lock();
        do_take_over_console(&raw mut vga_con, 0, MAX_NR_CONSOLES - 1, 1);
        console_unlock();
    }
    0
}

arch_initcall!(bcm1480_pcibios_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
