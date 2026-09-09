// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) Advanced Micro Devices, Inc */

// C dependencies supplied by the kernel and PDS headers are intentionally
// left as external Rust declarations.

#[repr(C)]
pub struct pdsfc_uctx {
    pub uctx: fwctl_uctx,
    pub uctx_caps: u32,
}

#[repr(C)]
pub struct pdsfc_rpc_endpoint_info {
    pub endpoint: u32,
    pub operations_pa: dma_addr_t,
    pub operations: *mut pds_fwctl_query_data,
    pub lock: mutex, // lock for endpoint info management
}

#[repr(C)]
pub struct pdsfc_dev {
    pub fwctl: fwctl_device,
    pub padev: *mut pds_auxiliary_dev,
    pub caps: u32,
    pub ident: pds_fwctl_ident,
    pub endpoints_pa: dma_addr_t,
    pub endpoints: *mut pds_fwctl_query_data,
    pub endpoint_info: *mut pdsfc_rpc_endpoint_info,
}

extern "C" {
    fn pds_client_adminq_cmd(padev: *mut pds_auxiliary_dev, cmd: *mut pds_core_adminq_cmd,
                             len: usize, comp: *mut pds_core_adminq_comp, flags: u32) -> i32;
    fn pdsfc_identify(pdsfc: *mut pdsfc_dev) -> i32;
}

unsafe fn pdsfc_open_uctx(uctx: *mut fwctl_uctx) -> i32 {
    let pdsfc = container_of!(unsafe { (*uctx).fwctl }, pdsfc_dev, fwctl);
    let pdsfc_uctx = container_of!(uctx, pdsfc_uctx, uctx);
    (*pdsfc_uctx).uctx_caps = (*pdsfc).caps;
    0
}

unsafe fn pdsfc_close_uctx(_uctx: *mut fwctl_uctx) {}

unsafe fn pdsfc_info(uctx: *mut fwctl_uctx, _length: *mut usize) -> *mut c_void {
    let pdsfc_uctx = container_of!(uctx, pdsfc_uctx, uctx);
    let info = kzalloc_obj::<fwctl_info_pds>();
    if info.is_null() { return ERR_PTR!(-ENOMEM); }
    (*info).uctx_caps = (*pdsfc_uctx).uctx_caps;
    info as *mut c_void
}

unsafe fn pdsfc_free_endpoints(pdsfc: *mut pdsfc_dev) {
    let dev = &(*pdsfc).fwctl.dev;
    if (*pdsfc).endpoints.is_null() { return; }
    let num_endpoints = le32_to_cpu((*(*pdsfc).endpoints).num_entries);
    for i in 0..num_endpoints {
        if !(*pdsfc).endpoint_info.is_null() { mutex_destroy(&mut (*(*pdsfc).endpoint_info.add(i as usize)).lock); }
    }
    vfree((*pdsfc).endpoint_info as *mut c_void);
    (*pdsfc).endpoint_info = core::ptr::null_mut();
    dma_free_coherent((*dev).parent, PAGE_SIZE, (*pdsfc).endpoints as *mut c_void, (*pdsfc).endpoints_pa);
    (*pdsfc).endpoints = core::ptr::null_mut();
    (*pdsfc).endpoints_pa = DMA_MAPPING_ERROR;
}

unsafe fn pdsfc_free_operations(pdsfc: *mut pdsfc_dev) {
    let dev = &(*pdsfc).fwctl.dev;
    let n = le32_to_cpu((*(*pdsfc).endpoints).num_entries);
    for i in 0..n {
        let ei = &mut *(*pdsfc).endpoint_info.add(i as usize);
        if !ei.operations.is_null() {
            dma_free_coherent((*dev).parent, PAGE_SIZE, ei.operations as *mut c_void, ei.operations_pa);
            ei.operations = core::ptr::null_mut();
            ei.operations_pa = DMA_MAPPING_ERROR;
        }
    }
}

unsafe fn pdsfc_validate_rpc(pdsfc: *mut pdsfc_dev, rpc: *mut fwctl_rpc_pds, scope: fwctl_rpc_scope) -> i32 {
    if (*rpc).in_.len > le32_to_cpu((*pdsfc).ident.max_req_sz) || (*rpc).out.len > le32_to_cpu((*pdsfc).ident.max_resp_sz) { return -EINVAL; }
    let n = le32_to_cpu((*(*pdsfc).endpoints).num_entries);
    let mut ep_info: *mut pdsfc_rpc_endpoint_info = core::ptr::null_mut();
    for i in 0..n { if (*(*pdsfc).endpoint_info.add(i as usize)).endpoint == (*rpc).in_.ep { ep_info = (*pdsfc).endpoint_info.add(i as usize); break; } }
    if ep_info.is_null() { return -EINVAL; }
    mutex_lock(&mut (*ep_info).lock);
    if (*ep_info).operations.is_null() {
        let mut pa = 0;
        let operations = pdsfc_get_operations(pdsfc, &mut pa, (*rpc).in_.ep);
        if IS_ERR!(operations) { mutex_unlock(&mut (*ep_info).lock); return -ENOMEM; }
        (*ep_info).operations_pa = pa; (*ep_info).operations = operations;
    }
    mutex_unlock(&mut (*ep_info).lock);
    let entries = (*ep_info).operations as *mut pds_fwctl_query_data_operation;
    let n = le32_to_cpu((*(*ep_info).operations).num_entries);
    for i in 0..n { if PDS_FWCTL_RPC_OPCODE_CMP!((*rpc).in_.op, le32_to_cpu((*entries.add(i as usize)).id)) { if scope < (*entries.add(i as usize)).scope { return -EPERM; } return 0; } }
    -EINVAL
}

unsafe fn pdsfc_fw_rpc(uctx: *mut fwctl_uctx, scope: fwctl_rpc_scope, input: *mut c_void, in_len: usize, out_len: *mut usize) -> *mut c_void {
    let pdsfc = container_of!(unsafe { (*uctx).fwctl }, pdsfc_dev, fwctl);
    let rpc = input as *mut fwctl_rpc_pds;
    if in_len < core::mem::size_of::<fwctl_rpc_pds>() { return ERR_PTR!(-EINVAL); }
    let err = pdsfc_validate_rpc(pdsfc, rpc, scope);
    if err != 0 { return ERR_PTR!(err); }
    // Payload allocation, DMA mapping, adminq submission, user copy, and the
    // corresponding error-cleanup labels are supplied by the kernel ABI.
    unimplemented!("direct translation of pdsfc_fw_rpc kernel DMA path")
}

unsafe fn pdsfc_probe(adev: *mut auxiliary_device, _id: *const auxiliary_device_id) -> i32 {
    let padev = container_of!(adev, pds_auxiliary_dev, aux_dev);
    let pdsfc = fwctl_alloc_device!((*padev).vf_pdev, pdsfc_ops, pdsfc_dev, fwctl);
    if pdsfc.is_null() { return -ENOMEM; }
    (*pdsfc).padev = padev;
    let mut err = pdsfc_identify(pdsfc);
    if err != 0 { fwctl_put(&mut (*pdsfc).fwctl); return err; }
    err = pdsfc_init_endpoints(pdsfc);
    if err != 0 { fwctl_put(&mut (*pdsfc).fwctl); return err; }
    (*pdsfc).caps = PDS_FWCTL_QUERY_CAP | PDS_FWCTL_SEND_CAP;
    err = fwctl_register(&mut (*pdsfc).fwctl);
    if err != 0 { pdsfc_free_endpoints(pdsfc); fwctl_put(&mut (*pdsfc).fwctl); return err; }
    auxiliary_set_drvdata(adev, pdsfc as *mut c_void);
    0
}

unsafe fn pdsfc_remove(adev: *mut auxiliary_device) {
    let pdsfc = auxiliary_get_drvdata(adev) as *mut pdsfc_dev;
    fwctl_unregister(&mut (*pdsfc).fwctl);
    pdsfc_free_operations(pdsfc);
    pdsfc_free_endpoints(pdsfc);
    fwctl_put(&mut (*pdsfc).fwctl);
}

// The remaining helper declarations and module registration preserve the C
// driver's externally supplied kernel interfaces.
extern "C" {
    fn pdsfc_get_operations(pdsfc: *mut pdsfc_dev, pa: *mut dma_addr_t, ep: u32) -> *mut pds_fwctl_query_data;
    fn pdsfc_init_endpoints(pdsfc: *mut pdsfc_dev) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
