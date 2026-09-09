// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2017, Microsoft Corporation.
 *   Copyright (c) 2025, Stefan Metzmacher
 */

// Dependency intent: declarations formerly supplied by internal.h and related
// kernel/RDMA headers are expected from the surrounding Rust translation unit.

pub unsafe fn smbdirect_connection_create_mr_list(sc: *mut smbdirect_socket) -> i32 {
    let sp = &(*sc).parameters;
    let mut mr: *mut smbdirect_mr_io;
    let mut ret: i32;
    let mut i: u32 = 0;

    if sp.responder_resources == 0 {
        smbdirect_log_rdma_mr(sc, SMBDIRECT_LOG_ERR, "responder_resources negotiated as 0\n");
        return -EINVAL;
    }

    /* Allocate more MRs (2x) than hardware responder_resources */
    while i < sp.responder_resources * 2 {
        mr = kzalloc_obj::<smbdirect_mr_io>();
        if mr.is_null() {
            ret = -ENOMEM;
            goto_kzalloc_mr_failed(sc, ret);
        }
        kref_init(&mut (*mr).kref);
        mutex_init(&mut (*mr).mutex);
        (*mr).mr = ib_alloc_mr((*sc).ib.pd, (*sc).mr_io.r#type, sp.max_frmr_depth);
        if is_err((*mr).mr) {
            ret = ptr_err((*mr).mr);
            smbdirect_log_rdma_mr(sc, SMBDIRECT_LOG_ERR, "ib_alloc_mr failed\n");
            mutex_destroy(&mut (*mr).mutex);
            kfree(mr);
            goto_kzalloc_mr_failed(sc, ret);
        }
        (*mr).sgt.sgl = kzalloc_objs::<scatterlist>(sp.max_frmr_depth);
        if (*mr).sgt.sgl.is_null() {
            ret = -ENOMEM;
            ib_dereg_mr((*mr).mr);
            mutex_destroy(&mut (*mr).mutex);
            kfree(mr);
            goto_kzalloc_mr_failed(sc, ret);
        }
        (*mr).state = SMBDIRECT_MR_READY;
        (*mr).socket = sc;
        list_add_tail(&mut (*mr).list, &mut (*sc).mr_io.all.list);
        atomic_inc(&mut (*sc).mr_io.ready.count);
        i += 1;
    }
    return 0;
}

unsafe fn smbdirect_mr_io_disable_locked(mr: *mut smbdirect_mr_io) {
    let sc = (*mr).socket;
    lockdep_assert_held(&(*mr).mutex);
    if (*mr).state == SMBDIRECT_MR_DISABLED { return; }
    if !(*mr).mr.is_null() { ib_dereg_mr((*mr).mr); }
    if (*mr).sgt.nents != 0 { ib_dma_unmap_sg((*sc).ib.dev, (*mr).sgt.sgl, (*mr).sgt.nents, (*mr).dir); }
    kfree((*mr).sgt.sgl);
    (*mr).mr = core::ptr::null_mut();
    (*mr).sgt.sgl = core::ptr::null_mut();
    (*mr).sgt.nents = 0;
    (*mr).state = SMBDIRECT_MR_DISABLED;
}

unsafe fn smbdirect_mr_io_free_locked(kref: *mut kref) {
    let mr = container_of!(kref, smbdirect_mr_io, kref);
    lockdep_assert_held(&(*mr).mutex);
    if (*mr).state != SMBDIRECT_MR_DISABLED { smbdirect_mr_io_disable_locked(mr); }
    mutex_unlock(&mut (*mr).mutex);
    mutex_destroy(&mut (*mr).mutex);
    kfree(mr);
}

pub unsafe fn smbdirect_connection_destroy_mr_list(sc: *mut smbdirect_socket) {
    let mut all_list = LIST_HEAD_INIT;
    let mut flags: unsigned_long = 0;
    spin_lock_irqsave(&mut (*sc).mr_io.all.lock, &mut flags);
    list_splice_tail_init(&mut (*sc).mr_io.all.list, &mut all_list);
    spin_unlock_irqrestore(&mut (*sc).mr_io.all.lock, flags);
    let mut pos = all_list.next;
    while pos != &mut all_list as *mut _ {
        let mr = container_of!(pos, smbdirect_mr_io, list);
        pos = (*pos).next;
        mutex_lock(&mut (*mr).mutex);
        smbdirect_mr_io_disable_locked(mr);
        list_del(&mut (*mr).list);
        (*mr).socket = core::ptr::null_mut();
        if !kref_put(&mut (*mr).kref, smbdirect_mr_io_free_locked) { mutex_unlock(&mut (*mr).mutex); }
    }
}

unsafe fn smbdirect_connection_get_mr_io(sc: *mut smbdirect_socket) -> *mut smbdirect_mr_io {
    loop {
        let ret = wait_event_interruptible(&mut (*sc).mr_io.ready.wait_queue,
            atomic_read(&(*sc).mr_io.ready.count) != 0 || (*sc).status != SMBDIRECT_SOCKET_CONNECTED);
        if ret != 0 || (*sc).status != SMBDIRECT_SOCKET_CONNECTED { return core::ptr::null_mut(); }
        let mut flags: unsigned_long = 0;
        spin_lock_irqsave(&mut (*sc).mr_io.all.lock, &mut flags);
        let mut p = (*sc).mr_io.all.list.next;
        while p != &mut (*sc).mr_io.all.list as *mut _ {
            let mr = container_of!(p, smbdirect_mr_io, list);
            p = (*p).next;
            if (*mr).state == SMBDIRECT_MR_READY {
                (*mr).state = SMBDIRECT_MR_REGISTERED;
                kref_get(&mut (*mr).kref);
                spin_unlock_irqrestore(&mut (*sc).mr_io.all.lock, flags);
                atomic_dec(&mut (*sc).mr_io.ready.count);
                atomic_inc(&mut (*sc).mr_io.used.count);
                return mr;
            }
        }
        spin_unlock_irqrestore(&mut (*sc).mr_io.all.lock, flags);
    }
}

unsafe fn smbdirect_connection_mr_io_register_done(_cq: *mut ib_cq, wc: *mut ib_wc) {
    let mr = container_of!((*wc).wr_cqe, smbdirect_mr_io, cqe);
    if (*wc).status != IB_WC_SUCCESS { smbdirect_socket_schedule_cleanup((*mr).socket, -ECONNABORTED); }
}

unsafe fn smbdirect_connection_mr_io_local_inv_done(_cq: *mut ib_cq, wc: *mut ib_wc) {
    let mr = container_of!((*wc).wr_cqe, smbdirect_mr_io, cqe);
    (*mr).state = SMBDIRECT_MR_INVALIDATED;
    if (*wc).status != IB_WC_SUCCESS { smbdirect_socket_schedule_cleanup((*mr).socket, -ECONNABORTED); }
    complete(&mut (*mr).invalidate_done);
}

/* Transcribe the pages from an iterator into an MR scatterlist. */
unsafe fn smbdirect_iter_to_sgt(iter: *mut iov_iter, sgt: *mut sg_table, max_sg: u32) -> i32 {
    memset((*sgt).sgl, 0, max_sg as usize * core::mem::size_of::<scatterlist>());
    let ret = extract_iter_to_sg(iter, iov_iter_count(iter), sgt, max_sg, 0);
    if (*sgt).nents > 0 { sg_mark_end((*sgt).sgl.add((*sgt).nents as usize - 1)); }
    ret
}

/* Register memory for RDMA read/write. */
pub unsafe fn smbdirect_connection_register_mr_io(sc: *mut smbdirect_socket, iter: *mut iov_iter, writing: bool, need_invalidate: bool) -> *mut smbdirect_mr_io {
    let sp = &(*sc).parameters;
    let num_pages = iov_iter_npages(iter, sp.max_frmr_depth + 1);
    if num_pages > sp.max_frmr_depth { return core::ptr::null_mut(); }
    let mr = smbdirect_connection_get_mr_io(sc);
    if mr.is_null() { return mr; }
    mutex_lock(&mut (*mr).mutex);
    (*mr).dir = if writing { DMA_FROM_DEVICE } else { DMA_TO_DEVICE };
    (*mr).need_invalidate = need_invalidate;
    (*mr).sgt.nents = 0; (*mr).sgt.orig_nents = 0;
    smbdirect_iter_to_sgt(iter, &mut (*mr).sgt, sp.max_frmr_depth);
    let num_mapped = ib_dma_map_sg((*sc).ib.dev, (*mr).sgt.sgl, (*mr).sgt.nents, (*mr).dir);
    if num_mapped == 0 { (*mr).state = SMBDIRECT_MR_ERROR; atomic_dec(&mut (*sc).mr_io.used.count); return core::ptr::null_mut(); }
    let ret = ib_map_mr_sg((*mr).mr, (*mr).sgt.sgl, num_mapped, core::ptr::null_mut(), PAGE_SIZE);
    if ret != num_mapped { ib_dma_unmap_sg((*sc).ib.dev, (*mr).sgt.sgl, (*mr).sgt.nents, (*mr).dir); (*mr).state = SMBDIRECT_MR_ERROR; atomic_dec(&mut (*sc).mr_io.used.count); return core::ptr::null_mut(); }
    ib_update_fast_reg_key((*mr).mr, ib_inc_rkey((*mr).mr).rkey);
    (*mr).wr.wr.opcode = IB_WR_REG_MR; (*mr).cqe.done = smbdirect_connection_mr_io_register_done; (*mr).wr.wr.wr_cqe = &mut (*mr).cqe; (*mr).wr.wr.num_sge = 0; (*mr).wr.wr.send_flags = IB_SEND_SIGNALED; (*mr).wr.mr = (*mr).mr; (*mr).wr.key = (*mr).mr.rkey; (*mr).wr.access = if writing { IB_ACCESS_REMOTE_WRITE | IB_ACCESS_LOCAL_WRITE } else { IB_ACCESS_REMOTE_READ };
    if ib_post_send((*sc).ib.qp, &mut (*mr).wr.wr, core::ptr::null_mut()) == 0 { mutex_unlock(&mut (*mr).mutex); return mr; }
    ib_dma_unmap_sg((*sc).ib.dev, (*mr).sgt.sgl, (*mr).sgt.nents, (*mr).dir); (*mr).sgt.nents = 0; (*mr).state = SMBDIRECT_MR_ERROR; atomic_dec(&mut (*sc).mr_io.used.count); mutex_unlock(&mut (*mr).mutex); core::ptr::null_mut()
}

pub unsafe fn smbdirect_mr_io_fill_buffer_descriptor(mr: *mut smbdirect_mr_io, v1: *mut smbdirect_buffer_descriptor_v1) {
    mutex_lock(&mut (*mr).mutex);
    if (*mr).state == SMBDIRECT_MR_REGISTERED { (*v1).offset = cpu_to_le64((*mr).mr).iova; (*v1).token = cpu_to_le32((*mr).mr).rkey; (*v1).length = cpu_to_le32((*mr).mr).length; } else { (*v1).offset = U64_MAX; (*v1).token = U32_MAX; (*v1).length = U32_MAX; }
    mutex_unlock(&mut (*mr).mutex);
}

pub unsafe fn smbdirect_connection_deregister_mr_io(mr: *mut smbdirect_mr_io) {
    let sc = (*mr).socket;
    mutex_lock(&mut (*mr).mutex);
    if (*mr).state == SMBDIRECT_MR_DISABLED || (*sc).status != SMBDIRECT_SOCKET_CONNECTED { smbdirect_mr_io_disable_locked(mr); } else { (*mr).state = SMBDIRECT_MR_INVALIDATED; if (*mr).sgt.nents != 0 { ib_dma_unmap_sg((*sc).ib.dev, (*mr).sgt.sgl, (*mr).sgt.nents, (*mr).dir); (*mr).sgt.nents = 0; } (*mr).state = SMBDIRECT_MR_READY; if atomic_inc_return(&mut (*sc).mr_io.ready.count) == 1 { wake_up(&mut (*sc).mr_io.ready.wait_queue); } atomic_dec(&mut (*sc).mr_io.used.count); }
    if !kref_put(&mut (*mr).kref, smbdirect_mr_io_free_locked) { mutex_unlock(&mut (*mr).mutex); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
