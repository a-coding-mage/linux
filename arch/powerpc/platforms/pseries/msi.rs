// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2006 Jake Moilanen <moilanen@austin.ibm.com>, IBM Corp.
 * Copyright 2006-2007 Michael Ellerman, IBM Corp.
 */

// Dependencies are supplied by the surrounding kernel translation.

#[repr(C)]
struct pseries_msi_device {
    msi_quota: u32,
    msi_used: u32,
}

static mut query_token: i32 = 0;
static mut change_token: i32 = 0;

const RTAS_QUERY_FN: u32 = 0;
const RTAS_CHANGE_FN: u32 = 1;
const RTAS_RESET_FN: u32 = 2;
const RTAS_CHANGE_MSI_FN: u32 = 3;
const RTAS_CHANGE_MSIX_FN: u32 = 4;
const RTAS_CHANGE_32MSI_FN: u32 = 5;
const RTAS_CHANGE_32MSIX_FN: u32 = 6;

unsafe fn rtas_change_msi(pdn: *mut pci_dn, func: u32, num_irqs: u32) -> i32 {
    let addr = rtas_config_addr((*pdn).busno, (*pdn).devfn, 0);
    let buid = (*(*pdn).phb).buid;
    let mut seq_num: u32 = 1;
    let mut rtas_ret = [0u32; 3];
    let mut rc;
    loop {
        if func == RTAS_CHANGE_MSI_FN || func == RTAS_CHANGE_MSIX_FN ||
           func == RTAS_CHANGE_32MSI_FN || func == RTAS_CHANGE_32MSIX_FN {
            rc = rtas_call(change_token, 6, 4, rtas_ret.as_mut_ptr(), addr,
                BUID_HI(buid), BUID_LO(buid), func, num_irqs, seq_num);
        } else {
            rc = rtas_call(change_token, 6, 3, rtas_ret.as_mut_ptr(), addr,
                BUID_HI(buid), BUID_LO(buid), func, num_irqs, seq_num);
        }
        seq_num = rtas_ret[1];
        if !rtas_busy_delay(rc) { break; }
    }
    if rc == 0 { rc = rtas_ret[0] as i32; }
    else if rc > 0 { rc = -rc; }
    pr_debug!("rtas_msi: ibm,change_msi(func=%d,num=%d), got %d rc = %d\n", func, num_irqs, rtas_ret[0], rc);
    rc
}

unsafe fn rtas_disable_msi(pdev: *mut pci_dev) {
    let pdn = pci_get_pdn(pdev);
    if pdn.is_null() { return; }
    if rtas_change_msi(pdn, RTAS_CHANGE_MSI_FN, 0) != 0 &&
       rtas_change_msi(pdn, RTAS_CHANGE_FN, 0) != 0 {
        pr_debug!("rtas_msi: Setting MSIs to 0 failed!\n");
    }
}

unsafe fn rtas_query_irq_number(pdn: *mut pci_dn, offset: i32) -> i32 {
    let addr = rtas_config_addr((*pdn).busno, (*pdn).devfn, 0);
    let buid = (*(*pdn).phb).buid;
    let mut ret = [0u32; 2];
    let rc;
    loop {
        rc = rtas_call(query_token, 4, 3, ret.as_mut_ptr(), addr,
            BUID_HI(buid), BUID_LO(buid), offset);
        if !rtas_busy_delay(rc) { break; }
    }
    if rc != 0 { pr_debug!("rtas_msi: error (%d) querying source number\n", rc); return rc; }
    ret[0] as i32
}

unsafe fn check_req(pdev: *mut pci_dev, nvec: i32, prop_name: *const i8) -> i32 {
    let dn = pci_device_to_OF_node(pdev);
    let p = of_get_property(dn, prop_name, core::ptr::null_mut());
    if p.is_null() { pr_debug!("rtas_msi: No %s on %pOF\n", prop_name, dn); return -ENOENT; }
    let req = be32_to_cpup(p);
    if req < nvec as u32 {
        pr_debug!("rtas_msi: %s requests < %d MSIs\n", prop_name, nvec);
        if req == 0 { return -ENOSPC; }
        return req as i32;
    }
    0
}

unsafe fn check_req_msi(pdev: *mut pci_dev, nvec: i32) -> i32 { check_req(pdev, nvec, c"ibm,req#msi".as_ptr()) }
unsafe fn check_req_msix(pdev: *mut pci_dev, nvec: i32) -> i32 { check_req(pdev, nvec, c"ibm,req#msi-x".as_ptr()) }

unsafe fn __find_pe_total_msi(mut node: *mut device_node, total: *mut i32) -> *mut device_node {
    let mut dn = of_node_get(node);
    while !dn.is_null() {
        let p = of_get_property(dn, c"ibm,pe-total-#msi".as_ptr(), core::ptr::null_mut());
        if !p.is_null() { *total = be32_to_cpup(p) as i32; return dn; }
        dn = of_get_next_parent(dn);
    }
    core::ptr::null_mut()
}
unsafe fn find_pe_total_msi(dev: *mut pci_dev, total: *mut i32) -> *mut device_node {
    __find_pe_total_msi(pci_device_to_OF_node(dev), total)
}
unsafe fn find_pe_dn(dev: *mut pci_dev, total: *mut i32) -> *mut device_node {
    let dn = pci_device_to_OF_node(dev); if dn.is_null() { return core::ptr::null_mut(); }
    let edev = pdn_to_eeh_dev(PCI_DN(dn));
    let top = if !(*edev).pe.is_null() { list_first_entry(&(*(*edev).pe).edevs, eeh_dev, entry) } else { edev };
    let dn = pci_device_to_OF_node((*top).pdev); if dn.is_null() { return core::ptr::null_mut(); }
    let dn = of_get_parent(dn); if dn.is_null() { return core::ptr::null_mut(); }
    *total = 8; dn
}

#[repr(C)] struct msi_counts { requestor: *mut device_node, num_devices: i32, request: i32, quota: i32, spare: i32, over_quota: i32 }

unsafe extern "C" fn count_non_bridge_devices(dn: *mut device_node, data: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let c = &mut *(data as *mut msi_counts);
    let p = of_get_property(dn, c"class-code".as_ptr(), core::ptr::null_mut());
    let class = if p.is_null() { 0 } else { be32_to_cpup(p) };
    if (class >> 8) != PCI_CLASS_BRIDGE_PCI { c.num_devices += 1; }
    core::ptr::null_mut()
}
unsafe extern "C" fn count_spare_msis(dn: *mut device_node, data: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let c = &mut *(data as *mut msi_counts);
    let mut req = if dn == c.requestor { c.request } else { 0 };
    if dn != c.requestor {
        let p = of_get_property(dn, c"ibm,req#msi".as_ptr(), core::ptr::null_mut());
        if !p.is_null() { req = be32_to_cpup(p) as i32; }
        let p = of_get_property(dn, c"ibm,req#msi-x".as_ptr(), core::ptr::null_mut());
        if !p.is_null() { req = core::cmp::max(req, be32_to_cpup(p) as i32); }
    }
    if req < c.quota { c.spare += c.quota - req; } else if req > c.quota { c.over_quota += 1; }
    core::ptr::null_mut()
}

unsafe fn msi_quota_for_device(dev: *mut pci_dev, mut request: i32) -> i32 {
    let mut total = 0;
    let mut pe_dn = find_pe_total_msi(dev, &mut total);
    if pe_dn.is_null() { pe_dn = find_pe_dn(dev, &mut total); }
    if pe_dn.is_null() { pr_err!("rtas_msi: couldn't find PE for %s\n", pci_name(dev)); return request; }
    let mut counts = msi_counts { requestor: core::ptr::null_mut(), num_devices: 0, request: 0, quota: 0, spare: 0, over_quota: 0 };
    pci_traverse_device_nodes(pe_dn, Some(count_non_bridge_devices), &mut counts as *mut _ as *mut _);
    if counts.num_devices == 0 { return request; }
    counts.quota = total / counts.num_devices;
    if request > counts.quota {
        counts.requestor = pci_device_to_OF_node(dev); counts.request = request;
        pci_traverse_device_nodes(pe_dn, Some(count_spare_msis), &mut counts as *mut _ as *mut _);
        counts.spare += total % counts.num_devices;
        if counts.over_quota != 0 { counts.quota += counts.spare / counts.over_quota; }
        request = core::cmp::min(counts.quota, request);
    }
    of_node_put(pe_dn); request
}

unsafe fn rtas_hack_32bit_msi_gen2(pdev: *mut pci_dev) {
    let mut hi = 0u32; pci_read_config_dword(pdev, (*pdev).msi_cap + PCI_MSI_ADDRESS_HI, &mut hi);
    let lo = 0xffff0000 | ((hi >> (48 - 32)) << 4);
    pci_write_config_dword(pdev, (*pdev).msi_cap + PCI_MSI_ADDRESS_LO, lo);
    pci_write_config_dword(pdev, (*pdev).msi_cap + PCI_MSI_ADDRESS_HI, 0);
}

unsafe fn rtas_prepare_msi_irqs(pdev: *mut pci_dev, nvec_in: i32, kind: i32, _arg: *mut msi_alloc_info_t) -> i32 {
    let mut nvec = nvec_in;
    let mut rc = if kind == PCI_CAP_ID_MSIX { check_req_msix(pdev, nvec) } else { check_req_msi(pdev, nvec) };
    if rc != 0 { return rc; }
    let quota = msi_quota_for_device(pdev, nvec);
    if quota != 0 && quota < nvec { return quota; }
    if kind == PCI_CAP_ID_MSIX { let m = roundup_pow_of_two(nvec); if msi_quota_for_device(pdev, m) >= m { nvec = m; } }
    let pdn = pci_get_pdn(pdev);
    if kind == PCI_CAP_ID_MSI {
        if (*pdev).msi_addr_mask < DMA_BIT_MASK(64) { rc = rtas_change_msi(pdn, RTAS_CHANGE_32MSI_FN, nvec as u32); } else { rc = -1; }
        if rc < 0 { rc = rtas_change_msi(pdn, RTAS_CHANGE_MSI_FN, nvec as u32); }
        if rc < 0 { rc = rtas_change_msi(pdn, RTAS_CHANGE_FN, nvec as u32); }
    } else { rc = rtas_change_msi(pdn, if (*pdev).msi_addr_mask < DMA_BIT_MASK(64) { RTAS_CHANGE_32MSIX_FN } else { RTAS_CHANGE_MSIX_FN }, nvec as u32); }
    if rc != nvec { if nvec != nvec_in { nvec = nvec_in; return rtas_prepare_msi_irqs(pdev, nvec, kind, _arg); } return rc; }
    0
}

unsafe extern "C" fn pseries_msi_shutdown(d: *mut irq_data) {
    let d = (*d).parent_data; if !d.is_null() && !(*d).chip.is_null() {
        if let Some(f) = (*(*d).chip).irq_shutdown { f(d); }
    }
}
unsafe extern "C" fn pseries_msi_write_msg(data: *mut irq_data, msg: *mut msi_msg) {
    let entry = irq_data_get_msi_desc(data); (*entry).msg = *msg;
}
unsafe extern "C" fn pseries_init_dev_msi_info(dev: *mut device, domain: *mut irq_domain, parent: *mut irq_domain, info: *mut msi_domain_info) -> bool {
    if !msi_lib_init_dev_msi_info(dev, domain, parent, info) { return false; }
    (*(*info).chip).irq_shutdown = Some(pseries_msi_shutdown); (*(*info).chip).irq_write_msi_msg = Some(pseries_msi_write_msg); true
}
unsafe extern "C" fn rtas_msi_pci_irq_fixup(pdev: *mut pci_dev) {
    if (*pdev).irq == 0 { return; }
    if check_req_msi(pdev, 1) != 0 && check_req_msix(pdev, 1) != 0 { return; }
    rtas_disable_msi(pdev);
}
unsafe fn rtas_msi_init() -> i32 {
    query_token = rtas_function_token(RTAS_FN_IBM_QUERY_INTERRUPT_SOURCE_NUMBER);
    change_token = rtas_function_token(RTAS_FN_IBM_CHANGE_MSI);
    if query_token == RTAS_UNKNOWN_SERVICE || change_token == RTAS_UNKNOWN_SERVICE { return -1; }
    if ppc_md.pci_irq_fixup.is_some() { WARN_ON(true); }
    ppc_md.pci_irq_fixup = Some(rtas_msi_pci_irq_fixup); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
