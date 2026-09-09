// SPDX-License-Identifier: GPL-2.0
/*
 * mmconfig.c - Low-level direct PCI config space access via MMCONFIG
 *
 * This is an 64bit optimized version that always keeps the full mmconfig
 * space mapped. This allows lockless config space operation.
 */

// Linux headers and architecture dependencies are supplied by other files.

#[repr(C)]
pub struct pci_mmcfg_region {
    pub address: u64,
    pub start_bus: u8,
    pub end_bus: u8,
    pub virt: *mut core::ffi::c_void,
    pub res: core::ffi::c_void,
    pub list: core::ffi::c_void,
}

#[repr(C)]
pub struct pci_raw_ops {
    pub read: Option<unsafe extern "C" fn(u32, u32, u32, i32, i32, *mut u32) -> i32>,
    pub write: Option<unsafe extern "C" fn(u32, u32, u32, i32, i32, u32) -> i32>,
}

extern "C" {
    fn pci_mmconfig_lookup(seg: u32, bus: u32) -> *mut pci_mmcfg_region;
    fn mmio_config_readb(addr: *mut u8) -> u32;
    fn mmio_config_readw(addr: *mut u8) -> u32;
    fn mmio_config_readl(addr: *mut u8) -> u32;
    fn mmio_config_writeb(addr: *mut u8, value: u32);
    fn mmio_config_writew(addr: *mut u8, value: u32);
    fn mmio_config_writel(addr: *mut u8, value: u32);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn ioremap(start: u64, size: u64) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
}

extern "C" {
    static mut pci_mmcfg_list: core::ffi::c_void;
    static mut raw_pci_ext_ops: *const pci_raw_ops;
}

// PCI_MMCFG_BUS_OFFSET(x) is supplied by the architecture headers.
extern "C" {
    fn PCI_MMCFG_BUS_OFFSET(value: u32) -> u64;
}

unsafe fn pci_dev_base(seg: u32, bus: u32, devfn: u32) -> *mut u8 {
    let cfg = pci_mmconfig_lookup(seg, bus);

    if !cfg.is_null() && !(*cfg).virt.is_null() {
        return ((*cfg).virt as *mut u8)
            .add((PCI_MMCFG_BUS_OFFSET(bus) | ((devfn as u64) << 12)) as usize);
    }
    core::ptr::null_mut()
}

unsafe extern "C" fn pci_mmcfg_read(
    seg: u32,
    bus: u32,
    devfn: u32,
    reg: i32,
    len: i32,
    value: *mut u32,
) -> i32 {
    let mut addr: *mut u8;

    /* Why do we have this when nobody checks it. How about a BUG()!? -AK */
    if bus > 255 || devfn > 255 || reg > 4095 {
        *value = u32::MAX;
        return -22;
    }

    rcu_read_lock();
    addr = pci_dev_base(seg, bus, devfn);
    if addr.is_null() {
        rcu_read_unlock();
        *value = u32::MAX;
        return -22;
    }

    match len {
        1 => *value = mmio_config_readb(addr.add(reg as usize)),
        2 => *value = mmio_config_readw(addr.add(reg as usize)),
        4 => *value = mmio_config_readl(addr.add(reg as usize)),
        _ => {}
    }
    rcu_read_unlock();

    0
}

unsafe extern "C" fn pci_mmcfg_write(
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

#[no_mangle]
pub static pci_mmcfg: pci_raw_ops = pci_raw_ops {
    read: Some(pci_mmcfg_read),
    write: Some(pci_mmcfg_write),
};

unsafe fn mcfg_ioremap(cfg: *mut pci_mmcfg_region) -> *mut core::ffi::c_void {
    let mut addr: *mut core::ffi::c_void;
    let start: u64;
    let size: u64;
    let num_buses: u32;

    start = (*cfg).address + PCI_MMCFG_BUS_OFFSET((*cfg).start_bus as u32);
    num_buses = (*cfg).end_bus as u32 - (*cfg).start_bus as u32 + 1;
    size = PCI_MMCFG_BUS_OFFSET(num_buses);
    addr = ioremap(start, size);
    if !addr.is_null() {
        addr = (addr as *mut u8).sub(PCI_MMCFG_BUS_OFFSET((*cfg).start_bus as u32) as usize)
            as *mut core::ffi::c_void;
    }
    addr
}

#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_arch_map(cfg: *mut pci_mmcfg_region) -> i32 {
    (*cfg).virt = mcfg_ioremap(cfg);
    if (*cfg).virt.is_null() {
        // pr_err("can't map ECAM at %pR\n", &cfg->res);
        return -12;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_arch_unmap(cfg: *mut pci_mmcfg_region) {
    if !cfg.is_null() && !(*cfg).virt.is_null() {
        iounmap(
            ((*cfg).virt as *mut u8)
                .add(PCI_MMCFG_BUS_OFFSET((*cfg).start_bus as u32) as usize)
                as *mut core::ffi::c_void,
        );
        (*cfg).virt = core::ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_arch_init() -> i32 {
    // list_for_each_entry(cfg, &pci_mmcfg_list, list)
    //     if (pci_mmcfg_arch_map(cfg)) { pci_mmcfg_arch_free(); return 0; }
    // The list traversal is supplied by the Linux list implementation.
    raw_pci_ext_ops = &pci_mmcfg;
    1
}

#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_arch_free() {
    // list_for_each_entry(cfg, &pci_mmcfg_list, list)
    //     pci_mmcfg_arch_unmap(cfg);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
