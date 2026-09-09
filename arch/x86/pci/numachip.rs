// SPDX-License-Identifier: GPL-2.0
/*
 * Numascale NumaConnect-specific PCI code
 *
 * Copyright (C) 2012 Numascale AS. All rights reserved.
 *
 * Send feedback to <support@numascale.com>
 *
 * PCI accessor functions derived from mmconfig_64.c
 */

// Dependencies supplied by the kernel PCI, x86 PCI, and Numachip headers.

extern "C" {
    fn pci_mmconfig_lookup(seg: u32, bus: u32) -> *mut pci_mmcfg_region;
    fn mmio_config_readb(addr: *mut u8) -> u8;
    fn mmio_config_readw(addr: *mut u8) -> u16;
    fn mmio_config_readl(addr: *mut u8) -> u32;
    fn mmio_config_writeb(addr: *mut u8, value: u32);
    fn mmio_config_writew(addr: *mut u8, value: u32);
    fn mmio_config_writel(addr: *mut u8, value: u32);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn raw_pci_read(seg: u32, bus: u32, devfn: u32, reg: u32, len: u32, value: *mut u32) -> i32;
}

#[repr(C)]
pub struct pci_mmcfg_region {
    pub virt: *mut u8,
}

#[repr(C)]
pub struct pci_raw_ops {
    pub read: Option<unsafe extern "C" fn(u32, u32, u32, i32, i32, *mut u32) -> i32>,
    pub write: Option<unsafe extern "C" fn(u32, u32, u32, i32, i32, u32) -> i32>,
}

extern "C" {
    static mut raw_pci_ops: *const pci_raw_ops;
    static mut raw_pci_ext_ops: *const pci_raw_ops;
}

extern "C" {
    fn pci_mmconfig_bus_offset(bus: u32) -> usize;
}

static mut limit: u8 = 0;

unsafe fn pci_dev_base(seg: u32, bus: u32, devfn: u32) -> *mut u8 {
    let cfg = pci_mmconfig_lookup(seg, bus);

    if !cfg.is_null() && !(*cfg).virt.is_null() {
        return (*cfg).virt.add(pci_mmconfig_bus_offset(bus) | ((devfn as usize) << 12));
    }
    core::ptr::null_mut()
}

unsafe extern "C" fn pci_mmcfg_read_numachip(
    seg: u32,
    bus: u32,
    devfn: u32,
    reg: i32,
    len: i32,
    value: *mut u32,
) -> i32 {
    let addr: *mut u8;

    /* Why do we have this when nobody checks it. How about a BUG()!? -AK */
    if bus > 255 || devfn > 255 || reg > 4095 {
        *value = u32::MAX;
        return -22;
    }

    /* Ensure AMD Northbridges don't decode reads to other devices */
    if bus == 0 && devfn >= limit as u32 {
        *value = u32::MAX;
        return 0;
    }

    rcu_read_lock();
    addr = pci_dev_base(seg, bus, devfn);
    if addr.is_null() {
        rcu_read_unlock();
        *value = u32::MAX;
        return -22;
    }

    match len {
        1 => *value = mmio_config_readb(addr.add(reg as usize)) as u32,
        2 => *value = mmio_config_readw(addr.add(reg as usize)) as u32,
        4 => *value = mmio_config_readl(addr.add(reg as usize)),
        _ => {}
    }
    rcu_read_unlock();

    0
}

unsafe extern "C" fn pci_mmcfg_write_numachip(
    seg: u32,
    bus: u32,
    devfn: u32,
    reg: i32,
    len: i32,
    value: u32,
) -> i32 {
    let addr: *mut u8;

    /* Why do we have this when nobody checks it. How about a BUG()!? -AK */
    if bus > 255 || devfn > 255 || reg > 4095 {
        return -22;
    }

    /* Ensure AMD Northbridges don't decode writes to other devices */
    if bus == 0 && devfn >= limit as u32 {
        return 0;
    }

    rcu_read_lock();
    addr = pci_dev_base(seg, bus, devfn);
    if addr.is_null() {
        rcu_read_unlock();
        return -22;
    }

    match len {
        1 => mmio_config_writeb(addr.add(reg as usize), value),
        2 => mmio_config_writew(addr.add(reg as usize), value),
        4 => mmio_config_writel(addr.add(reg as usize), value),
        _ => {}
    }
    rcu_read_unlock();

    0
}

static pci_mmcfg_numachip: pci_raw_ops = pci_raw_ops {
    read: Some(pci_mmcfg_read_numachip),
    write: Some(pci_mmcfg_write_numachip),
};

pub unsafe extern "C" fn pci_numachip_init() -> i32 {
    let mut ret: i32 = 0;
    let mut val: u32 = 0;

    /* For remote I/O, restrict bus 0 access to the actual number of AMD
       Northbridges, which starts at device number 0x18 */
    ret = raw_pci_read(0, 0, ((0x18 << 3) | 0), 0x60, core::mem::size_of::<u32>() as u32, &mut val);
    if ret != 0 {
        return ret;
    }

    /* HyperTransport fabric size in bits 6:4 */
    limit = (0x18 + ((val >> 4) & 7) + 1) as u8;

    /* Use NumaChip PCI accessors for non-extended and extended access */
    raw_pci_ops = &pci_mmcfg_numachip;
    raw_pci_ext_ops = &pci_mmcfg_numachip;
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
