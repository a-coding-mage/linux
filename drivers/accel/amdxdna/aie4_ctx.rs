// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// External kernel, DRM, and AMD XDNA dependencies are supplied by the surrounding crate.

unsafe fn cert_comp_isr(_irq: i32, p: *mut core::ffi::c_void) -> irqreturn_t {
    let cert_comp = p as *mut cert_comp;

    wake_up_all(unsafe { &mut (*cert_comp).waitq });
    IRQ_HANDLED
}

unsafe fn aie4_lookup_cert_comp(
    ndev: *mut amdxdna_dev_hdl,
    msix_idx: u32,
) -> *mut cert_comp {
    let xdna = (*ndev).aie.xdna;
    let pdev = to_pci_dev((*xdna).ddev.dev);
    let mut cert_comp: *mut cert_comp;
    let ret: i32;

    // guard(mutex)(&ndev->cert_comp_lock);
    let _cert_comp_lock = mutex_guard(&mut (*ndev).cert_comp_lock);

    cert_comp = xa_load(&mut (*ndev).cert_comp_xa, msix_idx);
    if !cert_comp.is_null() {
        kref_get(&mut (*cert_comp).kref);
        return cert_comp;
    }

    cert_comp = kzalloc_obj::<cert_comp>();
    if cert_comp.is_null() {
        return core::ptr::null_mut();
    }

    (*cert_comp).ndev = ndev;
    (*cert_comp).msix_idx = msix_idx;
    init_waitqueue_head(&mut (*cert_comp).waitq);
    kref_init(&mut (*cert_comp).kref);

    ret = pci_irq_vector(pdev, (*cert_comp).msix_idx);
    if ret < 0 {
        XDNA_ERR!(xdna, "MSI-X idx %u is invalid, ret:%d", msix_idx, ret);
        kfree(cert_comp);
        return core::ptr::null_mut();
    }
    (*cert_comp).irq = ret;

    ret = request_irq((*cert_comp).irq, cert_comp_isr, 0, "xdna_hsa", cert_comp);
    if ret != 0 {
        XDNA_ERR!(xdna, "request irq %d failed %d", (*cert_comp).irq, ret);
        kfree(cert_comp);
        return core::ptr::null_mut();
    }

    ret = xa_err(xa_store(
        &mut (*ndev).cert_comp_xa,
        msix_idx,
        cert_comp,
        GFP_KERNEL,
    ));
    if ret != 0 {
        XDNA_ERR!(xdna, "store cert_comp for msix idx %d failed %d", msix_idx, ret);
        free_irq((*cert_comp).irq, cert_comp);
        kfree(cert_comp);
        return core::ptr::null_mut();
    }

    cert_comp
}

unsafe extern "C" fn cert_comp_release(kref: *mut kref) {
    let cert_comp = container_of!(kref, cert_comp, kref);
    let ndev = (*cert_comp).ndev;

    drm_WARN_ON!(&(*ndev).aie.xdna.ddev, !mutex_is_locked(&(*ndev).cert_comp_lock));

    xa_erase(&mut (*ndev).cert_comp_xa, (*cert_comp).msix_idx);
    free_irq((*cert_comp).irq, cert_comp);
    kfree(cert_comp);
}

unsafe fn aie4_put_cert_comp(cert_comp: *mut cert_comp) {
    let ndev = (*cert_comp).ndev;
    let _cert_comp_lock = mutex_guard(&mut (*ndev).cert_comp_lock);
    kref_put(&mut (*cert_comp).kref, cert_comp_release);
}

unsafe fn aie4_msg_destroy_context(ndev: *mut amdxdna_dev_hdl, hw_context_id: u32) -> i32 {
    // DECLARE_AIE_MSG(aie4_msg_destroy_hw_context, AIE4_MSG_OP_DESTROY_HW_CONTEXT);
    let mut msg = aie4_msg_destroy_hw_context::new(AIE4_MSG_OP_DESTROY_HW_CONTEXT);
    msg.req.hw_context_id = hw_context_id;
    aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg)
}

unsafe fn aie4_hwctx_create(hwctx: *mut amdxdna_hwctx) -> i32 {
    // DECLARE_AIE_MSG(aie4_msg_create_hw_context, AIE4_MSG_OP_CREATE_HW_CONTEXT);
    let mut msg = aie4_msg_create_hw_context::new(AIE4_MSG_OP_CREATE_HW_CONTEXT);
    let client = (*hwctx).client;
    let priv_ = (*hwctx).priv_;
    let xdna = (*client).xdna;
    let ndev = (*xdna).dev_handle;

    drm_WARN_ON!(&(*xdna).ddev, !mutex_is_locked(&(*xdna).dev_lock));

    if (*ndev).partition_id == 0 || (*hwctx).num_tiles == 0 {
        XDNA_ERR!(xdna, "invalid request partition_id %d, num_tiles %d", (*ndev).partition_id, (*hwctx).num_tiles);
        return -EINVAL;
    }

    msg.req.partition_id = (*ndev).partition_id;
    msg.req.request_num_tiles = (*hwctx).num_tiles;
    msg.req.pasid = FIELD_PREP(AIE4_MSG_PASID, (*client).pasid) | FIELD_PREP(AIE4_MSG_PASID_VLD, 1);
    msg.req.priority_band = (*hwctx).qos.priority;
    msg.req.hsa_addr_high = upper_32_bits(amdxdna_gem_dev_addr((*priv_).umq_bo));
    msg.req.hsa_addr_low = lower_32_bits(amdxdna_gem_dev_addr((*priv_).umq_bo));

    XDNA_DBG!(xdna, "pasid 0x%x, num_tiles %d, hsa[0x%x 0x%x]", msg.req.pasid, msg.req.request_num_tiles, msg.req.hsa_addr_high, msg.req.hsa_addr_low);

    let ret = aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg);
    if ret != 0 {
        XDNA_ERR!(xdna, "create ctx failed: %d", ret);
        return ret;
    }

    XDNA_DBG!(xdna, "resp msix: %d, ctx id: %d, doorbell: %d", msg.resp.job_complete_msix_idx, msg.resp.hw_context_id, msg.resp.doorbell_offset);
    (*priv_).cert_comp = aie4_lookup_cert_comp(ndev, msg.resp.job_complete_msix_idx);
    if (*priv_).cert_comp.is_null() {
        aie4_msg_destroy_context(ndev, msg.resp.hw_context_id);
        return -EINVAL;
    }
    (*priv_).hw_ctx_id = msg.resp.hw_context_id;
    (*hwctx).doorbell_offset = msg.resp.doorbell_offset;
    0
}

unsafe fn aie4_hwctx_destroy(hwctx: *mut amdxdna_hwctx) {
    let client = (*hwctx).client;
    let priv_ = (*hwctx).priv_;
    let xdna = (*client).xdna;
    let ndev = (*xdna).dev_handle;
    drm_WARN_ON!(&(*xdna).ddev, !mutex_is_locked(&(*xdna).dev_lock));
    aie4_msg_destroy_context(ndev, (*priv_).hw_ctx_id);
    aie4_put_cert_comp((*priv_).cert_comp);
}

unsafe fn aie4_hwctx_umq_fini(hwctx: *mut amdxdna_hwctx) {
    if !(*hwctx).priv_.is_null() && !(*(*hwctx).priv_).umq_bo.is_null() {
        amdxdna_gem_put_obj((*(*hwctx).priv_).umq_bo);
    }
}

unsafe fn aie4_hwctx_umq_init(hwctx: *mut amdxdna_hwctx) -> i32 {
    let priv_ = (*hwctx).priv_;
    let xdna = (*(*hwctx).client).xdna;
    let umq_bo = amdxdna_gem_get_obj((*hwctx).client, (*hwctx).umq_bo_hdl, AMDXDNA_BO_SHARE);
    if umq_bo.is_null() { XDNA_ERR!(xdna, "cannot find umq_bo handle %d", (*hwctx).umq_bo_hdl); return -ENOENT; }
    if (*umq_bo).mem.size < core::mem::size_of::<host_queue_header>() { XDNA_ERR!(xdna, "umq_bo size is too small"); amdxdna_gem_put_obj(umq_bo); return -EINVAL; }
    let qhdr = amdxdna_gem_vmap(umq_bo);
    if qhdr.is_null() { amdxdna_gem_put_obj(umq_bo); return -ENOMEM; }
    (*priv_).umq_bo = umq_bo;
    (*priv_).umq_read_index = &mut (*qhdr).read_index;
    (*priv_).umq_write_index = &mut (*qhdr).write_index;
    0
}

pub unsafe fn aie4_hwctx_init(hwctx: *mut amdxdna_hwctx) -> i32 {
    let xdna = (*(*hwctx).client).xdna;
    let priv_ = kzalloc_obj::<amdxdna_hwctx_priv>();
    if priv_.is_null() { return -ENOMEM; }
    (*hwctx).priv_ = priv_;
    let ret = aie4_hwctx_umq_init(hwctx);
    if ret != 0 { kfree(priv_); (*hwctx).priv_ = core::ptr::null_mut(); return ret; }
    let ret = aie4_hwctx_create(hwctx);
    if ret != 0 { aie4_hwctx_umq_fini(hwctx); kfree(priv_); (*hwctx).priv_ = core::ptr::null_mut(); return ret; }
    XDNA_DBG!(xdna, "hwctx %s init completed", (*hwctx).name);
    0
}

pub unsafe fn aie4_hwctx_fini(hwctx: *mut amdxdna_hwctx) {
    aie4_hwctx_destroy(hwctx);
    aie4_hwctx_umq_fini(hwctx);
    kfree((*hwctx).priv_);
}

#[inline]
unsafe fn valid_queue_index(read: u64, write: u64, capacity: u32) -> bool { write >= read && write - read <= capacity as u64 }

unsafe fn get_read_index(hwctx: *mut amdxdna_hwctx) -> u64 {
    let mut wi = READ_ONCE!((*(*(*hwctx).priv_).umq_write_index));
    let mut ri = READ_ONCE!((*(*(*hwctx).priv_).umq_read_index));
    let xdna = (*(*hwctx).client).xdna;
    if !valid_queue_index(ri, wi, CTX_MAX_CMDS) {
        XDNA_WARN!(xdna, "Invalid index, ri %llu, wi %llu", ri, wi);
        usleep_range(100, 200);
        ri = READ_ONCE!((*(*(*hwctx).priv_).umq_read_index));
        if !valid_queue_index(ri, wi, CTX_MAX_CMDS) { XDNA_ERR!(xdna, "Invalid index after retry, ri %llu, wi %llu", ri, wi); ri = 0; }
    }
    ri
}

#[inline]
unsafe fn check_cmd_done(hwctx: *mut amdxdna_hwctx, seq: u64) -> bool { get_read_index(hwctx) > seq }

pub unsafe fn aie4_cmd_wait(hwctx: *mut amdxdna_hwctx, seq: u64, timeout: u32) -> i32 {
    let wait_jifs = if timeout != 0 { msecs_to_jiffies(timeout) } else { MAX_SCHEDULE_TIMEOUT };
    let cert_comp = (*(*hwctx).priv_).cert_comp;
    let mut ret = wait_event_interruptible_timeout!((*cert_comp).waitq, check_cmd_done(hwctx, seq), wait_jifs);
    if ret == 0 { ret = -ETIME; }
    if ret <= 0 { ret as i32 } else { 0 }
}

pub unsafe fn aie4_hwctx_valid_doorbell(client: *mut amdxdna_client, vm_pgoff: u32) -> i32 {
    let idx = srcu_read_lock(&mut (*client).hwctx_srcu);
    let mut hwctx_id: usize = 0;
    let mut hwctx: *mut amdxdna_hwctx = core::ptr::null_mut();
    amdxdna_for_each_hwctx!(client, hwctx_id, hwctx);
    while !hwctx.is_null() {
        if vm_pgoff == ((*hwctx).doorbell_offset >> PAGE_SHIFT) { srcu_read_unlock(&(*client).hwctx_srcu, idx); return 1; }
        break;
    }
    srcu_read_unlock(&(*client).hwctx_srcu, idx);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
