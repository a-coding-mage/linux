// SPDX-License-Identifier: GPL-2.0-or-later

// Kernel dependencies supplied by the surrounding PowerNV PCI implementation.

unsafe fn pnv_pci_ioda_fixup_iov_resources(pdev: *mut pci_dev) {
    let phb = pci_bus_to_pnvhb((*pdev).bus);
    let mut iov: *mut pnv_iov_data = kzalloc_obj();
    if iov.is_null() { goto_disable_iov(pdev, iov); return; }
    (*pdev).dev.archdata.iov_data = iov;
    let mul = (*phb).ioda.total_pe_num;
    for i in 0..PCI_SRIOV_NUM_BARS {
        let res = &mut (*pdev).resource[i + PCI_IOV_RESOURCES];
        if res.flags == 0 || !res.parent.is_null() { continue; }
        if !pnv_pci_is_m64_flags(res.flags) {
            dev_warn(&mut (*pdev).dev, "Don't support SR-IOV with non M64 VF BAR%d: %pR. \n", i, res);
            goto_disable_iov(pdev, iov); return;
        }
        let vf_bar_sz = pci_iov_resource_size(pdev, i + PCI_IOV_RESOURCES);
        if vf_bar_sz > ((*phb).ioda.m64_segsize >> 2) {
            if vf_bar_sz < SZ_32M {
                pci_err(pdev, "VF BAR%d: %pR can't be mapped in single PE mode\n", i, res);
                goto_disable_iov(pdev, iov); return;
            }
            (*iov).m64_single_mode[i] = true;
            continue;
        }
        pci_dbg(pdev, " Fixing VF BAR%d: %pR to\n", i, res);
        res.end = res.start + vf_bar_sz * mul - 1;
        pci_dbg(pdev, "                       %pR\n", res);
        pci_info(pdev, "VF BAR%d: %pR (expanded to %d VFs for PE alignment)", i, res, mul);
        (*iov).need_shift = true;
    }
    return;
}

unsafe fn goto_disable_iov(pdev: *mut pci_dev, iov: *mut pnv_iov_data) {
    for i in 0..PCI_SRIOV_NUM_BARS {
        let res = &mut (*pdev).resource[i + PCI_IOV_RESOURCES];
        res.flags = 0;
        res.end = res.start - 1;
    }
    (*pdev).dev.archdata.iov_data = core::ptr::null_mut();
    if !iov.is_null() { kfree(iov); }
}

pub unsafe fn pnv_pci_ioda_fixup_iov(pdev: *mut pci_dev) {
    if (*pdev).is_virtfn {
        let pe = pnv_ioda_get_pe(pdev);
        (*pe).pdev = pdev;
        WARN_ON(((*pe).flags & PNV_IODA_PE_VF) == 0);
    } else if (*pdev).is_physfn { pnv_pci_ioda_fixup_iov_resources(pdev); }
}

pub unsafe fn pnv_pci_iov_resource_alignment(pdev: *const pci_dev, resno: i32) -> resource_size_t {
    let align = pci_iov_resource_size(pdev as *mut _, resno);
    let phb = pci_bus_to_pnvhb((*pdev).bus);
    let iov = pnv_iov_get(pdev as *mut _);
    if iov.is_null() { return align; }
    if (*iov).m64_single_mode[(resno - PCI_IOV_RESOURCES) as usize] { return align; }
    (*phb).ioda.total_pe_num * align
}

unsafe fn pnv_pci_vf_release_m64(pdev: *mut pci_dev, _num_vfs: u16) -> i32 {
    let iov = pnv_iov_get(pdev); let phb = pci_bus_to_pnvhb((*pdev).bus);
    for window_id in 0..MAX_M64_BARS {
        if test_bit(window_id, (*iov).used_m64_bar_mask) {
            opal_pci_phb_mmio_enable((*phb).opal_id, OPAL_M64_WINDOW_TYPE, window_id, 0);
            clear_bit(window_id, &mut (*phb).ioda.m64_bar_alloc);
        }
    } 0
}

unsafe fn pnv_ioda_map_m64_segmented(phb: *mut pnv_phb, window_id: i32, start: resource_size_t, size: resource_size_t) -> i64 {
    let mut rc = opal_pci_set_phb_mem_window((*phb).opal_id, OPAL_M64_WINDOW_TYPE, window_id, start, 0, size);
    if rc == 0 { rc = opal_pci_phb_mmio_enable((*phb).opal_id, OPAL_M64_WINDOW_TYPE, window_id, OPAL_ENABLE_M64_SPLIT); }
    if rc != 0 { pr_err("Failed to map M64 window #%d: %lld\n", window_id, rc); } rc
}

unsafe fn pnv_ioda_map_m64_single(phb: *mut pnv_phb, pe_num: i32, window_id: i32, start: resource_size_t, size: resource_size_t) -> i64 {
    let mut rc = opal_pci_map_pe_mmio_window((*phb).opal_id, pe_num, OPAL_M64_WINDOW_TYPE, window_id, 0);
    if rc == 0 { rc = opal_pci_set_phb_mem_window((*phb).opal_id, OPAL_M64_WINDOW_TYPE, window_id, start, 0, size); }
    if rc == 0 { rc = opal_pci_phb_mmio_enable((*phb).opal_id, OPAL_M64_WINDOW_TYPE, window_id, OPAL_ENABLE_M64_NON_SPLIT); }
    if rc != 0 { pr_err("Error mapping single PE BAR\n"); } rc
}

unsafe fn pnv_pci_alloc_m64_bar(phb: *mut pnv_phb, iov: *mut pnv_iov_data) -> i32 {
    loop {
        let win = find_next_zero_bit(&(*phb).ioda.m64_bar_alloc, (*phb).ioda.m64_bar_idx + 1, 0);
        if win >= (*phb).ioda.m64_bar_idx + 1 { return -1; }
        if !test_and_set_bit(win, &mut (*phb).ioda.m64_bar_alloc) { set_bit(win, (*iov).used_m64_bar_mask); return win; }
    }
}

unsafe fn pnv_pci_vf_assign_m64(pdev: *mut pci_dev, num_vfs: u16) -> i32 {
    let iov = pnv_iov_get(pdev); let phb = pci_bus_to_pnvhb((*pdev).bus);
    for i in 0..PCI_SRIOV_NUM_BARS {
        let res = &mut (*pdev).resource[i + PCI_IOV_RESOURCES];
        if res.flags == 0 || res.parent.is_null() { continue; }
        if !(*iov).m64_single_mode[i] {
            let win = pnv_pci_alloc_m64_bar(phb, iov); if win < 0 { pnv_pci_vf_release_m64(pdev, num_vfs); return -EBUSY; }
            if pnv_ioda_map_m64_segmented(phb, win, res.start, resource_size(res)) != 0 { pnv_pci_vf_release_m64(pdev, num_vfs); return -EBUSY; }
        } else {
            let size = pci_iov_resource_size(pdev, PCI_IOV_RESOURCES + i); let base = (*iov).vf_pe_arr[0].pe_number;
            for j in 0..num_vfs {
                let win = pnv_pci_alloc_m64_bar(phb, iov); if win < 0 { pnv_pci_vf_release_m64(pdev, num_vfs); return -EBUSY; }
                if pnv_ioda_map_m64_single(phb, win, base + j as i32, res.start + size * j as u64, size) != 0 { pnv_pci_vf_release_m64(pdev, num_vfs); return -EBUSY; }
            }
        }
    } 0
}

unsafe fn pnv_ioda_release_vf_PE(pdev: *mut pci_dev) {
    if !(*pdev).is_physfn { return; }
    let phb = pci_bus_to_pnvhb((*pdev).bus);
    // FIXME: Use pnv_ioda_release_pe()?
    list_for_each_entry_safe!(pe, pe_n, &(*phb).ioda.pe_list, list, {
        if pe.parent_dev != pdev { continue; }
        pnv_pci_ioda2_release_pe_dma(pe);
        mutex_lock(&mut (*phb).ioda.pe_list_mutex); list_del(&mut pe.list); mutex_unlock(&mut (*phb).ioda.pe_list_mutex);
        pnv_ioda_deconfigure_pe(phb, pe); pnv_ioda_free_pe(pe);
    });
}

unsafe fn pnv_pci_vf_resource_shift(dev: *mut pci_dev, offset: i32) -> i32 {
    if !(*dev).is_physfn { return -EINVAL; }
    let iov = pnv_iov_get(dev); let num_vfs = (*iov).num_vfs;
    for i in 0..PCI_SRIOV_NUM_BARS {
        let res = &mut (*dev).resource[i + PCI_IOV_RESOURCES]; if res.flags == 0 || res.parent.is_null() || (*iov).m64_single_mode[i] { continue; }
        let size = pci_iov_resource_size(dev, i + PCI_IOV_RESOURCES); let mut res2 = *res; res2.start = res.start + size * offset as u64; res2.end = res2.start + size * num_vfs as u64 - 1;
        if res2.end > res.end { dev_err(&mut (*dev).dev, "VF BAR%d: %pR would extend past %pR (trying to enable %d VFs shifted by %d)\n", i, &res2, res, num_vfs, offset); return -EBUSY; }
    }
    for i in 0..PCI_SRIOV_NUM_BARS {
        let res = &mut (*dev).resource[i + PCI_IOV_RESOURCES]; if res.flags == 0 || res.parent.is_null() || (*iov).m64_single_mode[i] { continue; }
        let size = pci_iov_resource_size(dev, i + PCI_IOV_RESOURCES); let old = *res; res.start += size * offset as u64;
        dev_info(&mut (*dev).dev, "VF BAR%d: %pR shifted to %pR (%sabling %d VFs shifted by %d)\n", i, &old, res, if offset > 0 { "En" } else { "Dis" }, num_vfs, offset);
        if offset < 0 { devm_release_resource(&mut (*dev).dev, &mut (*iov).holes[i]); memset(&mut (*iov).holes[i], 0, core::mem::size_of::<resource>()); }
        pci_update_resource(dev, i + PCI_IOV_RESOURCES);
        if offset > 0 { (*iov).holes[i].start = old.start; (*iov).holes[i].end = old.start + size * offset as u64 - 1; (*iov).holes[i].flags = IORESOURCE_BUS; (*iov).holes[i].name = c"pnv_iov_reserved".as_ptr(); devm_request_resource(&mut (*dev).dev, res.parent, &mut (*iov).holes[i]); }
    } 0
}

unsafe fn pnv_pci_sriov_disable(pdev: *mut pci_dev) {
    let iov = pnv_iov_get(pdev); if WARN_ON(iov.is_null()) { return; }
    let num_vfs = (*iov).num_vfs; let base_pe = (*iov).vf_pe_arr[0].pe_number;
    pnv_ioda_release_vf_PE(pdev); if (*iov).need_shift { pnv_pci_vf_resource_shift(pdev, -base_pe); } pnv_pci_vf_release_m64(pdev, num_vfs);
}

unsafe fn pnv_ioda_setup_vf_PE(pdev: *mut pci_dev, num_vfs: u16) {
    if !(*pdev).is_physfn { return; }
    let phb = pci_bus_to_pnvhb((*pdev).bus); let pdn = pci_get_pdn(pdev); let iov = pnv_iov_get(pdev);
    for vf_index in 0..num_vfs {
        let vf_devfn = pci_iov_virtfn_devfn(pdev, vf_index); let vf_bus = pci_iov_virtfn_bus(pdev, vf_index); let pe = &mut (*iov).vf_pe_arr[vf_index as usize];
        pe.phb = phb; pe.flags = PNV_IODA_PE_VF; pe.pbus = core::ptr::null_mut(); pe.parent_dev = pdev; pe.mve_number = -1; pe.rid = (vf_bus << 8) | vf_devfn; let pe_num = pe.pe_number;
        pe_info(pe, "VF %04d:%02d:%02d.%d associated with PE#%x\n", pci_domain_nr((*pdev).bus), (*pdev).bus.number, PCI_SLOT(vf_devfn), PCI_FUNC(vf_devfn), pe_num);
        if pnv_ioda_configure_pe(phb, pe) != 0 { pnv_ioda_free_pe(pe); pe.pdev = core::ptr::null_mut(); continue; }
        mutex_lock(&mut (*phb).ioda.pe_list_mutex); list_add_tail(&mut pe.list, &mut (*phb).ioda.pe_list); mutex_unlock(&mut (*phb).ioda.pe_list_mutex);
        list_for_each_entry!(vf_pdn, &mut (*(*pdn).parent).child_list, list, { if vf_pdn.busno == vf_bus && vf_pdn.devfn == vf_devfn { vf_pdn.pe_number = pe_num; break; } });
        pnv_pci_ioda2_setup_dma_pe(phb, pe);
    }
}

unsafe fn pnv_pci_sriov_enable(pdev: *mut pci_dev, num_vfs: u16) -> i32 {
    let phb = pci_bus_to_pnvhb((*pdev).bus); let iov = pnv_iov_get(pdev);
    if (*phb).type != PNV_PHB_IODA2 { pci_err(pdev, "SR-IOV is not supported on this PHB\n"); return -ENXIO; }
    if iov.is_null() { dev_info(&mut (*pdev).dev, "don't support this SRIOV device with non 64bit-prefetchable IOV BAR\n"); return -ENOSPC; }
    let base_pe = pnv_ioda_alloc_pe(phb, num_vfs); if base_pe.is_null() { pci_err(pdev, "Unable to allocate PEs for %d VFs\n", num_vfs); return -EBUSY; }
    (*iov).vf_pe_arr = base_pe; (*iov).num_vfs = num_vfs;
    let ret = pnv_pci_vf_assign_m64(pdev, num_vfs); if ret != 0 { dev_info(&mut (*pdev).dev, "Not enough M64 window resources\n"); for i in 0..num_vfs { pnv_ioda_free_pe(&mut (*iov).vf_pe_arr[i as usize]); } return ret; }
    if (*iov).need_shift { let ret = pnv_pci_vf_resource_shift(pdev, (*base_pe).pe_number); if ret != 0 { pnv_pci_vf_release_m64(pdev, num_vfs); for i in 0..num_vfs { pnv_ioda_free_pe(&mut (*iov).vf_pe_arr[i as usize]); } return ret; } }
    pnv_ioda_setup_vf_PE(pdev, num_vfs); 0
}

pub unsafe fn pnv_pcibios_sriov_disable(pdev: *mut pci_dev) -> i32 { pnv_pci_sriov_disable(pdev); remove_sriov_vf_pdns(pdev); 0 }
pub unsafe fn pnv_pcibios_sriov_enable(pdev: *mut pci_dev, num_vfs: u16) -> i32 { add_sriov_vf_pdns(pdev); pnv_pci_sriov_enable(pdev, num_vfs) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
