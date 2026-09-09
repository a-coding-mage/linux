// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * pci_dn.c
 *
 * Copyright (C) 2001 Todd Inglett, IBM Corporation
 *
 * PCI manipulation via device_nodes.
 *
 * Translated from the C implementation; kernel-provided types, functions,
 * constants, and list primitives are intentionally left as external dependencies.
 */

unsafe fn pci_bus_to_pdn(bus: *mut pci_bus) -> *mut pci_dn {
    let mut pbus = bus;
    while !pbus.is_null() {
        if pci_is_root_bus(pbus) || !(*pbus).self_.is_null() { break; }
        pbus = (*pbus).parent;
    }
    let dn = pci_bus_to_OF_node(pbus);
    if dn.is_null() { core::ptr::null_mut() } else { PCI_DN(dn) }
}

pub unsafe fn pci_get_pdn_by_devfn(bus: *mut pci_bus, devfn: i32) -> *mut pci_dn {
    let mut dn: *mut device_node = core::ptr::null_mut();
    let mut pdev: *mut pci_dev = core::ptr::null_mut();
    // list_for_each_entry(pdev, &bus->devices, bus_list)
    list_for_each_entry!(pdev, &(*bus).devices, bus_list, {
        if (*pdev).devfn == devfn {
            if !(*pdev).dev.archdata.pci_data.is_null() { return (*pdev).dev.archdata.pci_data; }
            dn = pci_device_to_OF_node(pdev);
            break;
        }
    });
    let pdn = if dn.is_null() { core::ptr::null_mut() } else { PCI_DN(dn) };
    if !pdn.is_null() { return pdn; }
    let parent = pci_bus_to_pdn(bus);
    if parent.is_null() { return core::ptr::null_mut(); }
    list_for_each_entry!(pdn, &(*parent).child_list, list, {
        if (*pdn).busno == (*bus).number && (*pdn).devfn == devfn { return pdn; }
    });
    core::ptr::null_mut()
}

pub unsafe fn pci_get_pdn(pdev: *mut pci_dev) -> *mut pci_dn {
    if !(*pdev).dev.archdata.pci_data.is_null() { return (*pdev).dev.archdata.pci_data; }
    let dn = pci_device_to_OF_node(pdev);
    let pdn = if dn.is_null() { core::ptr::null_mut() } else { PCI_DN(dn) };
    if !pdn.is_null() { return pdn; }
    let parent = pci_bus_to_pdn((*pdev).bus);
    if parent.is_null() { return core::ptr::null_mut(); }
    list_for_each_entry!(pdn, &(*parent).child_list, list, {
        if (*pdn).busno == (*(*pdev).bus).number && (*pdn).devfn == (*pdev).devfn { return pdn; }
    });
    core::ptr::null_mut()
}

#[cfg(CONFIG_EEH)]
unsafe fn eeh_dev_init(pdn: *mut pci_dn) -> *mut eeh_dev {
    let edev = kzalloc_obj!(*edev);
    if edev.is_null() { return core::ptr::null_mut(); }
    (*pdn).edev = edev;
    (*edev).pdn = pdn;
    (*edev).bdfn = ((*pdn).busno << 8) | (*pdn).devfn;
    (*edev).controller = (*pdn).phb;
    edev
}

#[cfg(CONFIG_PCI_IOV)]
unsafe fn add_one_sriov_vf_pdn(parent: *mut pci_dn, busno: i32, devfn: i32) -> *mut pci_dn {
    if parent.is_null() { return core::ptr::null_mut(); }
    let pdn = kzalloc_obj!(*pdn);
    if pdn.is_null() { return core::ptr::null_mut(); }
    (*pdn).phb = (*parent).phb;
    (*pdn).parent = parent;
    (*pdn).busno = busno;
    (*pdn).devfn = devfn;
    (*pdn).pe_number = IODA_INVALID_PE;
    INIT_LIST_HEAD!(&mut (*pdn).child_list);
    INIT_LIST_HEAD!(&mut (*pdn).list);
    list_add_tail!(&mut (*pdn).list, &mut (*parent).child_list);
    pdn
}

#[cfg(CONFIG_PCI_IOV)]
pub unsafe fn add_sriov_vf_pdns(pdev: *mut pci_dev) -> *mut pci_dn {
    if WARN_ON!(!(*pdev).is_physfn) { return core::ptr::null_mut(); }
    let mut pdn = pci_get_pdn(pdev);
    if pdn.is_null() || ((*pdn).flags & PCI_DN_FLAG_IOV_VF) != 0 { return core::ptr::null_mut(); }
    (*pdn).flags |= PCI_DN_FLAG_IOV_VF;
    let parent = pci_bus_to_pdn((*pdev).bus);
    if parent.is_null() { return core::ptr::null_mut(); }
    for i in 0..pci_sriov_get_totalvfs(pdev) {
        pdn = add_one_sriov_vf_pdn(parent, pci_iov_virtfn_bus(pdev, i), pci_iov_virtfn_devfn(pdev, i));
        if pdn.is_null() { dev_warn!(&(*pdev).dev, "%s: Cannot create firmware data for VF#%d\n", __func__, i); return core::ptr::null_mut(); }
        #[cfg(CONFIG_EEH)] { let edev = eeh_dev_init(pdn); BUG_ON!(edev.is_null()); (*edev).physfn = pdev; (*edev).vf_index = i; }
    }
    pci_get_pdn(pdev)
}

#[cfg(CONFIG_PCI_IOV)]
pub unsafe fn remove_sriov_vf_pdns(pdev: *mut pci_dev) {
    if WARN_ON!(!(*pdev).is_physfn) { return; }
    let pdn = pci_get_pdn(pdev);
    if pdn.is_null() || ((*pdn).flags & PCI_DN_FLAG_IOV_VF) == 0 { return; }
    (*pdn).flags &= !PCI_DN_FLAG_IOV_VF;
    let parent = pci_bus_to_pdn((*pdev).bus);
    if parent.is_null() { return; }
    for i in 0..pci_sriov_get_totalvfs(pdev) {
        list_for_each_entry_safe!(pdn, tmp, &mut (*parent).child_list, list, {
            if (*pdn).busno != pci_iov_virtfn_bus(pdev, i) || (*pdn).devfn != pci_iov_virtfn_devfn(pdev, i) { continue; }
            #[cfg(CONFIG_EEH)] { let edev = pdn_to_eeh_dev(pdn); if !edev.is_null() { if !(*edev).pe.is_null() { eeh_pe_tree_remove(edev); } (*pdn).edev = core::ptr::null_mut(); kfree(edev); } }
            if !list_empty!(&(*pdn).list) { list_del!(&mut (*pdn).list); }
            kfree(pdn);
        });
    }
}

pub unsafe fn pci_add_device_node_info(hose: *mut pci_controller, dn: *mut device_node) -> *mut pci_dn {
    let type_ = of_get_property(dn, c"ibm,pci-config-space-type".as_ptr(), core::ptr::null_mut());
    let pdn = kzalloc_obj!(*pdn);
    if pdn.is_null() { return core::ptr::null_mut(); }
    (*dn).data = pdn; (*pdn).phb = hose; (*pdn).pe_number = IODA_INVALID_PE;
    let regs = of_get_property(dn, c"reg".as_ptr(), core::ptr::null_mut());
    if !regs.is_null() { let addr = of_read_number(regs, 1); (*pdn).busno = (addr >> 16) & 0xff; (*pdn).devfn = (addr >> 8) & 0xff; }
    let regs = of_get_property(dn, c"vendor-id".as_ptr(), core::ptr::null_mut()); (*pdn).vendor_id = if regs.is_null() { 0 } else { of_read_number(regs, 1) };
    let regs = of_get_property(dn, c"device-id".as_ptr(), core::ptr::null_mut()); (*pdn).device_id = if regs.is_null() { 0 } else { of_read_number(regs, 1) };
    let regs = of_get_property(dn, c"class-code".as_ptr(), core::ptr::null_mut()); (*pdn).class_code = if regs.is_null() { 0 } else { of_read_number(regs, 1) };
    (*pdn).pci_ext_config_space = !type_.is_null() && of_read_number(type_, 1) == 1;
    #[cfg(CONFIG_EEH)] { if eeh_dev_init(pdn).is_null() { kfree(pdn); return core::ptr::null_mut(); } }
    INIT_LIST_HEAD!(&mut (*pdn).child_list); INIT_LIST_HEAD!(&mut (*pdn).list);
    let parent = of_get_parent(dn); (*pdn).parent = if parent.is_null() { core::ptr::null_mut() } else { PCI_DN(parent) }; of_node_put(parent);
    if !(*pdn).parent.is_null() { list_add_tail!(&mut (*pdn).list, &mut (*(*pdn).parent).child_list); }
    pdn
}

pub unsafe fn pci_remove_device_node_info(dn: *mut device_node) {
    let pdn = if dn.is_null() { core::ptr::null_mut() } else { PCI_DN(dn) };
    #[cfg(CONFIG_EEH)] { let edev = pdn_to_eeh_dev(pdn); if !edev.is_null() { (*edev).pdn = core::ptr::null_mut(); } }
    if pdn.is_null() { return; }
    WARN_ON!(!list_empty!(&(*pdn).child_list)); list_del!(&mut (*pdn).list);
    let parent = of_get_parent(dn); if !parent.is_null() { of_node_put(parent); }
    let pdev = pci_get_domain_bus_and_slot((*(*pdn).phb).global_number, (*pdn).busno, (*pdn).devfn);
    if !pdev.is_null() { pci_dbg!(pdev, "marked pdn (from %pOF) as dead\n", dn); (*pdn).flags |= PCI_DN_FLAG_DEAD; } else { (*dn).data = core::ptr::null_mut(); kfree(pdn); }
    pci_dev_put(pdev);
}

pub unsafe fn pci_traverse_device_nodes(start: *mut device_node, fn_: Option<unsafe fn(*mut device_node, *mut core::ffi::c_void) -> *mut core::ffi::c_void>, data: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let mut dn = (*start).child; let mut nextdn;
    while !dn.is_null() {
        let classp = of_get_property(dn, c"class-code".as_ptr(), core::ptr::null_mut()); let class = if classp.is_null() { 0 } else { of_read_number(classp, 1) };
        if let Some(f) = fn_ { let ret = f(dn, data); if !ret.is_null() { return ret; } }
        if !(*dn).child.is_null() && ((class >> 8) == PCI_CLASS_BRIDGE_PCI || (class >> 8) == PCI_CLASS_BRIDGE_CARDBUS) { nextdn = (*dn).child; }
        else if !(*dn).sibling.is_null() { nextdn = (*dn).sibling; }
        else { loop { dn = (*dn).parent; if dn == start { return core::ptr::null_mut(); } if !(*dn).sibling.is_null() { break; } } nextdn = (*dn).sibling; }
        dn = nextdn;
    }
    core::ptr::null_mut()
}

unsafe fn add_pdn(dn: *mut device_node, data: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    if pci_add_device_node_info(data as *mut pci_controller, dn).is_null() { return ERR_PTR!(-ENOMEM); }
    core::ptr::null_mut()
}

pub unsafe fn pci_devs_phb_init_dynamic(phb: *mut pci_controller) {
    let dn = (*phb).dn; let pdn = pci_add_device_node_info(phb, dn);
    if !pdn.is_null() { (*pdn).devfn = -1; (*pdn).busno = -1; (*pdn).vendor_id = 0; (*pdn).device_id = 0; (*pdn).class_code = 0; (*pdn).phb = phb; (*phb).pci_data = pdn; }
    pci_traverse_device_nodes(dn, Some(add_pdn), phb as *mut core::ffi::c_void);
}

unsafe fn pci_dev_pdn_setup(pdev: *mut pci_dev) {
    if !(*pdev).dev.archdata.pci_data.is_null() { return; }
    (*pdev).dev.archdata.pci_data = pci_get_pdn(pdev);
}

DECLARE_PCI_FIXUP_EARLY!(PCI_ANY_ID, PCI_ANY_ID, pci_dev_pdn_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
