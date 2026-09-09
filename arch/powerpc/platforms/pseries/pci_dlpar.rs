// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PCI Dynamic LPAR, PCI Hot Plug and PCI EEH recovery code
 * for RPA-compliant PPC64 platform.
 * Copyright (C) 2003 Linda Xie <lxie@us.ibm.com>
 * Copyright (C) 2005 International Business Machines
 *
 * Updates, 2005, John Rose <johnrose@austin.ibm.com>
 * Updates, 2005, Linas Vepstas <linas@austin.ibm.com>
 */

// C dependencies: linux/pci.h, linux/export.h, linux/node.h,
// asm/pci-bridge.h, asm/ppc-pci.h, asm/firmware.h, asm/eeh.h, and pseries.h.

pub unsafe fn init_phb_dynamic(dn: *mut device_node) -> *mut pci_controller {
    let phb: *mut pci_controller;
    let nid: i32;

    pr_debug!("PCI: Initializing new hotplug PHB %pOF\n", dn);

    nid = of_node_to_nid(dn);
    if nid >= 0 {
        if !node_online(nid) {
            if register_node(nid) != 0 {
                pr_err!("PCI: Failed to register node %d\n", nid);
            } else {
                update_numa_distance(dn);
                node_set_online(nid);
            }
        }
    }

    phb = pcibios_alloc_controller(dn);
    if phb.is_null() {
        return core::ptr::null_mut();
    }
    rtas_setup_phb(phb);
    pci_process_bridge_OF_ranges(phb, dn, 0);
    (*phb).controller_ops = pseries_pci_controller_ops;

    pci_devs_phb_init_dynamic(phb);

    pseries_msi_allocate_domains(phb);

    ppc_iommu_register_device(phb);

    /* Create EEH devices for the PHB */
    eeh_phb_pe_create(phb);

    if !(*dn).child.is_null() {
        pseries_eeh_init_edev_recursive(PCI_DN(dn));
    }

    pcibios_scan_phb(phb);
    pcibios_finish_adding_to_bus((*phb).bus);

    phb
}

// EXPORT_SYMBOL_GPL(init_phb_dynamic);

/* RPA-specific bits for removing PHBs */
pub unsafe fn remove_phb_dynamic(phb: *mut pci_controller) -> i32 {
    let b: *mut pci_bus = (*phb).bus;
    let host_bridge: *mut pci_host_bridge = to_pci_host_bridge((*b).bridge);
    let mut res: *mut resource;
    let rc: i32;
    let mut i: i32;

    pr_debug!(
        "PCI: Removing PHB %04x:%02x...\n",
        pci_domain_nr(b),
        (*b).number
    );

    /* We cannot to remove a root bus that has children */
    if !(list_empty(&(*b).children) && list_empty(&(*b).devices)) {
        return -EBUSY;
    }

    /* We -know- there aren't any child devices anymore at this stage
     * and thus, we can safely unmap the IO space as it's not in use
     */
    res = &mut (*phb).io_resource;
    if (*res).flags & IORESOURCE_IO != 0 {
        rc = pcibios_unmap_io_space(b);
        if rc != 0 {
            printk!(KERN_ERR "{}: failed to unmap IO on bus {}\n", __func__, (*b).name);
            return 1;
        }
    }

    ppc_iommu_unregister_device(phb);

    pseries_msi_free_domains(phb);

    /* Keep a reference so phb isn't freed yet */
    get_device(&mut (*host_bridge).dev);

    /* Remove the PCI bus and unregister the bridge device from sysfs */
    (*phb).bus = core::ptr::null_mut();
    pci_remove_bus(b);
    (*host_bridge).bus = core::ptr::null_mut();
    device_unregister(&mut (*host_bridge).dev);

    /* Now release the IO resource */
    if (*res).flags & IORESOURCE_IO != 0 {
        release_resource(res);
    }

    /* Release memory resources */
    i = 0;
    while i < 3 {
        res = &mut (*phb).mem_resources[i as usize];
        if (*res).flags & IORESOURCE_MEM != 0 {
            release_resource(res);
        }
        i += 1;
    }

    /*
     * The pci_controller data structure is freed by
     * the pcibios_free_controller_deferred() callback;
     * see pseries_root_bridge_prepare().
     */
    put_device(&mut (*host_bridge).dev);

    0
}

// EXPORT_SYMBOL_GPL(remove_phb_dynamic);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
