// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * The file intends to implement PE based on the information from
 * platforms. Basically, there have 3 types of PEs: PHB/Bus/Device.
 * All the PEs should be organized as hierarchy tree. The first level
 * of the tree will be associated to existing PHBs since the particular
 * PE is only meaningful in one PHB domain.
 *
 * Copyright Benjamin Herrenschmidt & Gavin Shan, IBM Corporation 2012.
 */

// Linux headers and architecture headers supply the referenced types,
// constants, functions, list primitives, and allocation helpers.

static mut eeh_pe_aux_size: i32 = 0;
static mut eeh_phb_pe: list_head = LIST_HEAD_INIT;

pub unsafe fn eeh_set_pe_aux_size(size: i32) {
    if size < 0 { return; }
    eeh_pe_aux_size = size;
}

unsafe fn eeh_pe_alloc(phb: *mut pci_controller, typ: i32) -> *mut eeh_pe {
    let mut alloc_size: usize = core::mem::size_of::<eeh_pe>();
    if eeh_pe_aux_size != 0 {
        alloc_size = ALIGN(alloc_size, cache_line_size());
        alloc_size += eeh_pe_aux_size as usize;
    }
    let pe = kzalloc(alloc_size, GFP_KERNEL);
    if pe.is_null() { return core::ptr::null_mut(); }
    (*pe).typ = typ;
    (*pe).phb = phb;
    INIT_LIST_HEAD(&mut (*pe).child);
    INIT_LIST_HEAD(&mut (*pe).child_list);
    INIT_LIST_HEAD(&mut (*pe).edevs);
    (*pe).data = (pe as *mut u8).add(ALIGN(core::mem::size_of::<eeh_pe>(), cache_line_size())) as *mut core::ffi::c_void;
    pe
}

pub unsafe fn eeh_phb_pe_create(phb: *mut pci_controller) -> i32 {
    let pe = eeh_pe_alloc(phb, EEH_PE_PHB);
    if pe.is_null() { pr_err!("{}: out of memory!\n", __func__); return -ENOMEM; }
    list_add_tail(&mut (*pe).child, &mut eeh_phb_pe);
    pr_debug!("EEH: Add PE for PHB#{}\n", (*phb).global_number);
    0
}

pub unsafe fn eeh_wait_state(pe: *mut eeh_pe, mut max_wait: i32) -> i32 {
    const EEH_STATE_MIN_WAIT_TIME: i32 = 1000;
    const EEH_STATE_MAX_WAIT_TIME: i32 = 300 * 1000;
    loop {
        let mut mwait = 0;
        let ret = (*eeh_ops).get_state(pe, &mut mwait);
        if ret != EEH_STATE_UNAVAILABLE { return ret; }
        if max_wait <= 0 {
            pr_warn!("{}: Timeout when getting PE's state ({})\n", __func__, max_wait);
            return EEH_STATE_NOT_SUPPORT;
        }
        if mwait < EEH_STATE_MIN_WAIT_TIME {
            pr_warn!("{}: Firmware returned bad wait value {}\n", __func__, mwait);
            mwait = EEH_STATE_MIN_WAIT_TIME;
        } else if mwait > EEH_STATE_MAX_WAIT_TIME {
            pr_warn!("{}: Firmware returned too long wait value {}\n", __func__, mwait);
            mwait = EEH_STATE_MAX_WAIT_TIME;
        }
        msleep(core::cmp::min(mwait, max_wait));
        max_wait -= mwait;
    }
}

pub unsafe fn eeh_phb_pe_get(phb: *mut pci_controller) -> *mut eeh_pe {
    let mut pe: *mut eeh_pe;
    list_for_each_entry!(pe, &mut eeh_phb_pe, child) {
        if ((*pe).typ & EEH_PE_PHB) != 0 && (*pe).phb == phb { return pe; }
    }
    core::ptr::null_mut()
}

pub unsafe fn eeh_pe_next(mut pe: *mut eeh_pe, root: *mut eeh_pe) -> *mut eeh_pe {
    let mut next = (*pe).child_list.next;
    if next == &mut (*pe).child_list as *mut list_head {
        loop {
            if pe == root { return core::ptr::null_mut(); }
            next = (*pe).child.next;
            if next != &mut (*(*pe).parent).child_list as *mut list_head { break; }
            pe = (*pe).parent;
        }
    }
    list_entry!(next, eeh_pe, child)
}

pub unsafe fn eeh_pe_traverse(root: *mut eeh_pe, fn_: eeh_pe_traverse_func, flag: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let mut pe: *mut eeh_pe;
    let mut ret;
    eeh_for_each_pe!(root, pe) {
        ret = fn_(pe, flag);
        if !ret.is_null() { return ret; }
    }
    core::ptr::null_mut()
}

pub unsafe fn eeh_pe_dev_traverse(root: *mut eeh_pe, fn_: eeh_edev_traverse_func, flag: *mut core::ffi::c_void) {
    if root.is_null() { pr_warn!("{}: Invalid PE {:?}\n", __func__, root); return; }
    let mut pe: *mut eeh_pe;
    let mut edev: *mut eeh_dev;
    let mut tmp: *mut eeh_dev;
    eeh_for_each_pe!(root, pe) { eeh_pe_for_each_dev!(pe, edev, tmp) { fn_(edev, flag); } }
}

unsafe fn __eeh_pe_get(pe: *mut eeh_pe, flag: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    if ((*pe).typ & EEH_PE_PHB) != 0 { return core::ptr::null_mut(); }
    if *(flag as *mut i32) == (*pe).addr { return pe as *mut core::ffi::c_void; }
    core::ptr::null_mut()
}

pub unsafe fn eeh_pe_get(phb: *mut pci_controller, pe_no: i32) -> *mut eeh_pe {
    let root = eeh_phb_pe_get(phb);
    eeh_pe_traverse(root, __eeh_pe_get, &pe_no as *const i32 as *mut core::ffi::c_void) as *mut eeh_pe
}

pub unsafe fn eeh_pe_tree_insert(edev: *mut eeh_dev, mut new_pe_parent: *mut eeh_pe) -> i32 {
    let hose = (*edev).controller;
    let mut pe = eeh_pe_get(hose, (*edev).pe_config_addr);
    if !pe.is_null() {
        if ((*pe).typ & EEH_PE_INVALID) != 0 {
            list_add_tail(&mut (*edev).entry, &mut (*pe).edevs); (*edev).pe = pe;
            let mut parent = pe;
            while !parent.is_null() { if ((*parent).typ & EEH_PE_INVALID) == 0 { break; } (*parent).typ &= !EEH_PE_INVALID; parent = (*parent).parent; }
            eeh_edev_dbg!(edev, "Added to existing PE (parent: PE#{:x})\n", (*(*pe).parent).addr);
        } else { (*pe).typ = EEH_PE_BUS; (*edev).pe = pe; list_add_tail(&mut (*edev).entry, &mut (*pe).edevs); eeh_edev_dbg!(edev, "Added to bus PE\n"); }
        return 0;
    }
    pe = eeh_pe_alloc(hose, if !(*edev).physfn.is_null() { EEH_PE_VF } else { EEH_PE_DEVICE });
    if pe.is_null() { pr_err!("{}: out of memory!\n", __func__); return -ENOMEM; }
    (*pe).addr = (*edev).pe_config_addr;
    if new_pe_parent.is_null() { new_pe_parent = eeh_phb_pe_get(hose); if new_pe_parent.is_null() { pr_err!("{}: No PHB PE is found (PHB Domain={})\n", __func__, (*hose).global_number); (*edev).pe = core::ptr::null_mut(); kfree(pe); return -EEXIST; } }
    (*pe).parent = new_pe_parent; list_add_tail(&mut (*pe).child, &mut (*new_pe_parent).child_list);
    list_add_tail(&mut (*edev).entry, &mut (*pe).edevs); (*edev).pe = pe;
    eeh_edev_dbg!(edev, "Added to new (parent: PE#{:x})\n", (*new_pe_parent).addr); 0
}

pub unsafe fn eeh_pe_tree_remove(edev: *mut eeh_dev) -> i32 {
    let mut pe = eeh_dev_to_pe(edev); if pe.is_null() { eeh_edev_dbg!(edev, "No PE found for device.\n"); return -EEXIST; }
    (*edev).pe = core::ptr::null_mut(); list_del(&mut (*edev).entry);
    loop {
        let parent = (*pe).parent; if ((*pe).typ & EEH_PE_PHB) != 0 { break; }
        let keep = ((*pe).state & EEH_PE_KEEP) != 0; let recover = ((*pe).state & EEH_PE_RECOVERING) != 0; WARN_ON!(keep && !recover);
        if !keep && !recover {
            if list_empty(&(*pe).edevs) && list_empty(&(*pe).child_list) { list_del(&mut (*pe).child); kfree(pe); } else { break; }
        } else if list_empty(&(*pe).edevs) {
            let mut cnt = 0; let mut child: *mut eeh_pe; list_for_each_entry!(child, &mut (*pe).child_list, child) { if ((*child).typ & EEH_PE_INVALID) == 0 { cnt += 1; break; } }
            if cnt == 0 { (*pe).typ |= EEH_PE_INVALID; } else { break; }
        }
        pe = parent;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
