// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004 Matthew Wilcox <matthew@wil.cx>
 * Copyright (C) 2004 Intel Corp.
 */

/*
 * mmconfig.c - Low-level direct PCI config space access via MMCONFIG
 */

// C dependencies: linux/pci.h, linux/init.h, linux/rcupdate.h,
// asm/e820/api.h, and asm/pci_x86.h.

/* Assume systems with more busses have correct MCFG */
// #define mmcfg_virt_addr ((void __iomem *) fix_to_virt(FIX_PCIE_MCFG))

/* The base address of the last MMCONFIG device accessed */
static mut mmcfg_last_accessed_device: u32 = 0;
static mut mmcfg_last_accessed_cpu: i32 = 0;

/*
 * Functions for accessing PCI configuration space with MMCONFIG accesses
 */
unsafe fn get_base_addr(seg: u32, bus: i32, _devfn: u32) -> u32 {
    let cfg = pci_mmconfig_lookup(seg, bus);

    if !cfg.is_null() {
        return (*cfg).address;
    }
    0
}

/*
 * This is always called under pci_config_lock
 */
unsafe fn pci_exp_set_dev_base(base: u32, bus: i32, devfn: u32) {
    let dev_base = base | PCI_MMCFG_BUS_OFFSET(bus) | (devfn << 12);
    let cpu = smp_processor_id();
    if dev_base != mmcfg_last_accessed_device || cpu != mmcfg_last_accessed_cpu {
        mmcfg_last_accessed_device = dev_base;
        mmcfg_last_accessed_cpu = cpu;
        set_fixmap_nocache(FIX_PCIE_MCFG, dev_base);
    }
}

unsafe fn pci_mmcfg_read(
    seg: u32,
    bus: u32,
    devfn: u32,
    reg: i32,
    len: i32,
    value: *mut u32,
) -> i32 {
    let mut flags: usize = 0;
    let base: u32;

    if bus > 255 || devfn > 255 || reg > 4095 {
        *value = u32::MAX;
        return -EINVAL;
    }

    rcu_read_lock();
    base = get_base_addr(seg, bus as i32, devfn);
    if base == 0 {
        rcu_read_unlock();
        *value = u32::MAX;
        return -EINVAL;
    }

    raw_spin_lock_irqsave(&pci_config_lock, &mut flags);

    pci_exp_set_dev_base(base, bus as i32, devfn);

    let addr = (fix_to_virt(FIX_PCIE_MCFG) as *mut u8).offset(reg as isize);
    match len {
        1 => *value = mmio_config_readb(addr),
        2 => *value = mmio_config_readw(addr),
        4 => *value = mmio_config_readl(addr),
        _ => {}
    }
    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
    rcu_read_unlock();

    0
}

unsafe fn pci_mmcfg_write(
    seg: u32,
    bus: u32,
    devfn: u32,
    reg: i32,
    len: i32,
    value: u32,
) -> i32 {
    let mut flags: usize = 0;
    let base: u32;

    if bus > 255 || devfn > 255 || reg > 4095 {
        return -EINVAL;
    }

    rcu_read_lock();
    base = get_base_addr(seg, bus as i32, devfn);
    if base == 0 {
        rcu_read_unlock();
        return -EINVAL;
    }

    raw_spin_lock_irqsave(&pci_config_lock, &mut flags);

    pci_exp_set_dev_base(base, bus as i32, devfn);

    let addr = (fix_to_virt(FIX_PCIE_MCFG) as *mut u8).offset(reg as isize);
    match len {
        1 => mmio_config_writeb(addr, value),
        2 => mmio_config_writew(addr, value),
        4 => mmio_config_writel(addr, value),
        _ => {}
    }
    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
    rcu_read_unlock();

    0
}

const pci_mmcfg: pci_raw_ops = pci_raw_ops {
    read: pci_mmcfg_read,
    write: pci_mmcfg_write,
};

unsafe fn pci_mmcfg_arch_init() -> i32 {
    printk(KERN_INFO, "PCI: Using ECAM for extended config space\n");
    raw_pci_ext_ops = &pci_mmcfg;
    1
}

unsafe fn pci_mmcfg_arch_free() {}

unsafe fn pci_mmcfg_arch_map(_cfg: *mut pci_mmcfg_region) -> i32 {
    0
}

unsafe fn pci_mmcfg_arch_unmap(_cfg: *mut pci_mmcfg_region) {
    let mut flags: usize = 0;

    /* Invalidate the cached mmcfg map entry. */
    raw_spin_lock_irqsave(&pci_config_lock, &mut flags);
    mmcfg_last_accessed_device = 0;
    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
