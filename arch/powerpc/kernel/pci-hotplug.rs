// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Derived from "arch/powerpc/platforms/pseries/pci_dlpar.c"
 *
 * Copyright (C) 2003 Linda Xie <lxie@us.ibm.com>
 * Copyright (C) 2005 International Business Machines
 *
 * Updates, 2005, John Rose <johnrose@austin.ibm.com>
 * Updates, 2005, Linas Vepstas <linas@austin.ibm.com>
 * Updates, 2013, Gavin Shan <shangw@linux.vnet.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn find_bus_among_children(
    bus: *mut pci_bus,
    dn: *mut device_node,
) -> *mut pci_bus {
    let mut child: *mut pci_bus = core::ptr::null_mut();
    let mut tmp: *mut pci_bus;

    if pci_bus_to_OF_node(bus) == dn {
        return bus;
    }

    list_for_each_entry!(tmp, &(*bus).children, node) {
        child = find_bus_among_children(tmp, dn);
        if !child.is_null() {
            break;
        }
    }

    child
}

pub unsafe extern "C" fn pci_find_bus_by_node(dn: *mut device_node) -> *mut pci_bus {
    let pdn: *mut pci_dn = PCI_DN!(dn);

    if pdn.is_null() || (*pdn).phb.is_null() || (*(*pdn).phb).bus.is_null() {
        return core::ptr::null_mut();
    }

    find_bus_among_children((*pdn).phb.as_ptr(), dn)
}

// EXPORT_SYMBOL_GPL(pci_find_bus_by_node);

/**
 * pcibios_release_device - release PCI device
 * @dev: PCI device
 *
 * The function is called before releasing the indicated PCI device.
 */
pub unsafe extern "C" fn pcibios_release_device(dev: *mut pci_dev) {
    let phb: *mut pci_controller = pci_bus_to_host((*dev).bus);
    let pdn: *mut pci_dn = pci_get_pdn(dev);

    if (*phb).controller_ops.release_device.is_some() {
        ((*phb).controller_ops.release_device.unwrap())(dev);
    }

    /* free()ing the pci_dn has been deferred to us, do it now */
    if !pdn.is_null() && ((*pdn).flags & PCI_DN_FLAG_DEAD) != 0 {
        pci_dbg!(dev, "freeing dead pdn\n");
        kfree(pdn.cast());
    }
}

/**
 * pci_hp_remove_devices - remove all devices under this bus
 * @bus: the indicated PCI bus
 *
 * Remove all of the PCI devices under this bus both from the
 * linux pci device tree, and from the powerpc EEH address cache.
 */
pub unsafe extern "C" fn pci_hp_remove_devices(bus: *mut pci_bus) {
    let mut dev: *mut pci_dev;
    let mut tmp: *mut pci_dev;
    let mut child_bus: *mut pci_bus;

    /* First go down child busses */
    list_for_each_entry!(child_bus, &(*bus).children, node) {
        pci_hp_remove_devices(child_bus);
    }

    pr_debug!("PCI: Removing devices on bus {:04x}:{:02x}\n",
        pci_domain_nr(bus), (*bus).number);
    list_for_each_entry_safe_reverse!(dev, tmp, &(*bus).devices, bus_list) {
        pr_debug!("   Removing {}...\n", pci_name(dev));
        pci_stop_and_remove_bus_device(dev);
    }
}

// EXPORT_SYMBOL_GPL(pci_hp_remove_devices);

unsafe fn traverse_siblings_and_scan_slot(start: *mut device_node, bus: *mut pci_bus) {
    let mut dn: *mut device_node;
    let mut slotno: i32;
    let mut class: u32 = 0;

    if of_property_read_u32((*start).child, c"class-code", &mut class) == 0 {
        /* Call of pci_scan_slot for non-bridge/EP case */
        if !((class >> 8) == PCI_CLASS_BRIDGE_PCI) {
            slotno = PCI_SLOT!(PCI_DN!((*start).child).devfn);
            pci_scan_slot(bus, PCI_DEVFN!(slotno, 0));
            return;
        }
    }

    /* Iterate all siblings */
    for_each_child_of_node!(start, dn) {
        class = 0;

        if of_property_read_u32((*start).child, c"class-code", &mut class) == 0 {
            /* Call of pci_scan_slot on each sibling-nodes/bridge-ports */
            if (class >> 8) == PCI_CLASS_BRIDGE_PCI {
                slotno = PCI_SLOT!(PCI_DN!(dn).devfn);
                pci_scan_slot(bus, PCI_DEVFN!(slotno, 0));
            }
        }
    }
}

/**
 * pci_hp_add_devices - adds new pci devices to bus
 * @bus: the indicated PCI bus
 *
 * This routine will find and fixup new pci devices under
 * the indicated bus. This routine presumes that there
 * might already be some devices under this bridge, so it
 * carefully tries to add only new devices.  (And that
 * is how this routine differs from other, similar pcibios
 * routines.)
 */
pub unsafe extern "C" fn pci_hp_add_devices(bus: *mut pci_bus) {
    let mut mode: i32;
    let mut max: i32;
    let mut dev: *mut pci_dev;
    let phb: *mut pci_controller;
    let dn: *mut device_node = pci_bus_to_OF_node(bus);

    if dn.is_null() {
        return;
    }

    phb = pci_bus_to_host(bus);

    mode = PCI_PROBE_NORMAL;
    if (*phb).controller_ops.probe_mode.is_some() {
        mode = ((*phb).controller_ops.probe_mode.unwrap())(bus);
    }

    if mode == PCI_PROBE_DEVTREE {
        /* use ofdt-based probe */
        of_rescan_bus(dn, bus);
    } else if mode == PCI_PROBE_NORMAL && !(*dn).child.is_null() && !PCI_DN!((*dn).child).is_null() {
        /*
         * Use legacy probe. In the partial hotplug case, we
         * probably have grandchildren devices unplugged. So
         * we don't check the return value from pci_scan_slot() in
         * order for fully rescan all the way down to pick them up.
         * They can have been removed during partial hotplug.
         */
        traverse_siblings_and_scan_slot(dn, bus);
        max = (*bus).busn_res.start;
        /*
         * Scan bridges that are already configured. We don't touch
         * them unless they are misconfigured (which will be done in
         * the second scan below).
         */
        for_each_pci_bridge!(dev, bus) {
            max = pci_scan_bridge(bus, dev, max, 0);
        }

        /* Scan bridges that need to be reconfigured */
        for_each_pci_bridge!(dev, bus) {
            max = pci_scan_bridge(bus, dev, max, 1);
        }
    }
    pcibios_finish_adding_to_bus(bus);
}

// EXPORT_SYMBOL_GPL(pci_hp_add_devices);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
