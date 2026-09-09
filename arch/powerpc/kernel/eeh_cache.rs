// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PCI address cache; allows the lookup of PCI devices based on I/O address
 *
 * Copyright IBM Corporation 2004
 * Copyright Linas Vepstas <linas@austin.ibm.com> 2004
 */

/* Linux and architecture headers supplying the referenced kernel types and APIs. */

#[repr(C)]
pub struct pci_io_addr_range {
    pub rb_node: rb_node,
    pub addr_lo: resource_size_t,
    pub addr_hi: resource_size_t,
    pub edev: *mut eeh_dev,
    pub pcidev: *mut pci_dev,
    pub flags: c_ulong,
}

#[repr(C)]
pub struct pci_io_addr_cache {
    pub rb_root: rb_root,
    pub piar_lock: spinlock_t,
}

static mut pci_io_addr_cache_root: pci_io_addr_cache = pci_io_addr_cache {
    rb_root: RB_ROOT,
    piar_lock: spinlock_t { _bindgen_opaque_blob: [] },
};

/**
 * DOC: Overview
 *
 * The pci address cache subsystem.  This subsystem places
 * PCI device address resources into a red-black tree, sorted
 * according to the address range, so that given only an i/o
 * address, the corresponding PCI device can be **quickly**
 * found. It is safe to perform an address lookup in an interrupt
 * context; this ability is an important feature.
 *
 * Currently, the only customer of this code is the EEH subsystem;
 * thus, this code has been somewhat tailored to suit EEH better.
 * In particular, the cache does *not* hold the addresses of devices
 * for which EEH is not enabled.
 *
 * (Implementation Note: The RB tree seems to be better/faster
 * than any hash algo I could think of for this problem, even
 * with the penalty of slow pointer chases for d-cache misses).
 */

#[inline]
unsafe fn __eeh_addr_cache_get_device(addr: c_ulong) -> *mut eeh_dev {
    let mut n = pci_io_addr_cache_root.rb_root.rb_node;

    while !n.is_null() {
        let piar = rb_entry!(n, pci_io_addr_range, rb_node);

        if addr < (*piar).addr_lo {
            n = (*n).rb_left;
        } else if addr > (*piar).addr_hi {
            n = (*n).rb_right;
        } else {
            return (*piar).edev;
        }
    }

    core::ptr::null_mut()
}

/**
 * eeh_addr_cache_get_dev - Get device, given only address
 * @addr: mmio (PIO) phys address or i/o port number
 *
 * Given an mmio phys address, or a port number, find a pci device
 * that implements this address.  I/O port numbers are assumed to be offset
 * from zero (that is, they do *not* have pci_io_addr added in).
 * It is safe to call this function within an interrupt.
 */
pub unsafe fn eeh_addr_cache_get_dev(addr: c_ulong) -> *mut eeh_dev {
    let mut flags: c_ulong = 0;

    spin_lock_irqsave!(&mut pci_io_addr_cache_root.piar_lock, &mut flags);
    let edev = __eeh_addr_cache_get_device(addr);
    spin_unlock_irqrestore!(&mut pci_io_addr_cache_root.piar_lock, flags);
    edev
}

#[cfg(feature = "DEBUG")]
unsafe fn eeh_addr_cache_print(cache: *mut pci_io_addr_cache) {
    let mut n = rb_first(&mut (*cache).rb_root);
    let mut cnt: c_int = 0;

    while !n.is_null() {
        let piar = rb_entry!(n, pci_io_addr_range, rb_node);
        pr_info!(
            "PCI: %s addr range %d [%pap-%pap]: %s\n",
            if ((*piar).flags & IORESOURCE_IO) != 0 { "i/o" } else { "mem" },
            cnt,
            &(*piar).addr_lo,
            &(*piar).addr_hi,
            pci_name((*piar).pcidev)
        );
        cnt += 1;
        n = rb_next(n);
    }
}

/* Insert address range into the rb tree. */
unsafe fn eeh_addr_cache_insert(
    dev: *mut pci_dev,
    alo: resource_size_t,
    ahi: resource_size_t,
    flags: c_ulong,
) -> *mut pci_io_addr_range {
    let mut p = &mut pci_io_addr_cache_root.rb_root.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    let mut piar: *mut pci_io_addr_range;

    /* Walk tree, find a place to insert into tree */
    while !(*p).is_null() {
        parent = *p;
        piar = rb_entry!(parent, pci_io_addr_range, rb_node);
        if ahi < (*piar).addr_lo {
            p = &mut (*parent).rb_left;
        } else if alo > (*piar).addr_hi {
            p = &mut (*parent).rb_right;
        } else {
            if dev != (*piar).pcidev || alo != (*piar).addr_lo || ahi != (*piar).addr_hi {
                pr_warn!("PIAR: overlapping address range\n");
            }
            return piar;
        }
    }

    piar = kzalloc_obj!(pci_io_addr_range, GFP_ATOMIC);
    if piar.is_null() {
        return core::ptr::null_mut();
    }

    (*piar).addr_lo = alo;
    (*piar).addr_hi = ahi;
    (*piar).edev = pci_dev_to_eeh_dev(dev);
    (*piar).pcidev = dev;
    (*piar).flags = flags;

    eeh_edev_dbg!((*piar).edev, "PIAR: insert range=[%pap:%pap]\n", &alo, &ahi);

    rb_link_node(&mut (*piar).rb_node, parent, p);
    rb_insert_color(&mut (*piar).rb_node, &mut pci_io_addr_cache_root.rb_root);

    piar
}

unsafe fn __eeh_addr_cache_insert_dev(dev: *mut pci_dev) {
    let edev = pci_dev_to_eeh_dev(dev);
    if edev.is_null() {
        pr_warn!("PCI: no EEH dev found for %s\n", pci_name(dev));
        return;
    }

    /* Skip any devices for which EEH is not enabled. */
    if (*edev).pe.is_null() {
        dev_dbg!(&mut (*dev).dev, "EEH: Skip building address cache\n");
        return;
    }

    /*
     * Walk resources on this device, poke the first 7 (6 normal BAR and 1
     * ROM BAR) into the tree.
     */
    let mut i: c_int = 0;
    while i <= PCI_ROM_RESOURCE {
        let start = pci_resource_start(dev, i);
        let end = pci_resource_end(dev, i);
        let flags = pci_resource_flags(dev, i);

        /* We are interested only bus addresses, not dma or other stuff */
        if (flags & (IORESOURCE_IO | IORESOURCE_MEM)) == 0 {
            i += 1;
            continue;
        }
        if start == 0 || !start != 0 || end == 0 || !end != 0 {
            i += 1;
            continue;
        }
        eeh_addr_cache_insert(dev, start, end, flags);
        i += 1;
    }
}

/**
 * eeh_addr_cache_insert_dev - Add a device to the address cache
 * @dev: PCI device whose I/O addresses we are interested in.
 *
 * In order to support the fast lookup of devices based on addresses,
 * we maintain a cache of devices that can be quickly searched.
 * This routine adds a device to that cache.
 */
pub unsafe fn eeh_addr_cache_insert_dev(dev: *mut pci_dev) {
    let mut flags: c_ulong = 0;

    spin_lock_irqsave!(&mut pci_io_addr_cache_root.piar_lock, &mut flags);
    __eeh_addr_cache_insert_dev(dev);
    spin_unlock_irqrestore!(&mut pci_io_addr_cache_root.piar_lock, flags);
}

unsafe fn __eeh_addr_cache_rmv_dev(dev: *mut pci_dev) {
    'restart: loop {
        let mut n = rb_first(&mut pci_io_addr_cache_root.rb_root);
        while !n.is_null() {
            let piar = rb_entry!(n, pci_io_addr_range, rb_node);

            if (*piar).pcidev == dev {
                eeh_edev_dbg!((*piar).edev, "PIAR: remove range=[%pap:%pap]\n", &(*piar).addr_lo, &(*piar).addr_hi);
                rb_erase(n, &mut pci_io_addr_cache_root.rb_root);
                kfree(piar as *mut core::ffi::c_void);
                continue 'restart;
            }
            n = rb_next(n);
        }
        break;
    }
}

/**
 * eeh_addr_cache_rmv_dev - remove pci device from addr cache
 * @dev: device to remove
 *
 * Remove a device from the addr-cache tree.
 * This is potentially expensive, since it will walk
 * the tree multiple times (once per resource).
 * But so what; device removal doesn't need to be that fast.
 */
pub unsafe fn eeh_addr_cache_rmv_dev(dev: *mut pci_dev) {
    let mut flags: c_ulong = 0;

    spin_lock_irqsave!(&mut pci_io_addr_cache_root.piar_lock, &mut flags);
    __eeh_addr_cache_rmv_dev(dev);
    spin_unlock_irqrestore!(&mut pci_io_addr_cache_root.piar_lock, flags);
}

/**
 * eeh_addr_cache_init - Initialize a cache of I/O addresses
 *
 * Initialize a cache of pci i/o addresses.  This cache will be used to
 * find the pci device that corresponds to a given address.
 */
pub unsafe fn eeh_addr_cache_init() {
    spin_lock_init(&mut pci_io_addr_cache_root.piar_lock);
}

unsafe fn eeh_addr_cache_show(s: *mut seq_file, _v: *mut core::ffi::c_void) -> c_int {
    let mut flags: c_ulong = 0;

    spin_lock_irqsave!(&mut pci_io_addr_cache_root.piar_lock, &mut flags);
    let mut n = rb_first(&mut pci_io_addr_cache_root.rb_root);
    while !n.is_null() {
        let piar = rb_entry!(n, pci_io_addr_range, rb_node);
        seq_printf!(
            s,
            "%s addr range [%pap-%pap]: %s\n",
            if ((*piar).flags & IORESOURCE_IO) != 0 { "i/o" } else { "mem" },
            &(*piar).addr_lo,
            &(*piar).addr_hi,
            pci_name((*piar).pcidev)
        );
        n = rb_next(n);
    }
    spin_unlock_irqrestore!(&mut pci_io_addr_cache_root.piar_lock, flags);

    0
}

DEFINE_SHOW_ATTRIBUTE!(eeh_addr_cache);

pub unsafe fn eeh_cache_debugfs_init() {
    debugfs_create_file_unsafe!(
        "eeh_address_cache",
        0o400,
        arch_debugfs_dir,
        core::ptr::null_mut(),
        &eeh_addr_cache_fops
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
