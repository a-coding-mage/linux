// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  BSG helper library
 *
 *  Copyright (C) 2008   James Smart, Emulex Corporation
 *  Copyright (C) 2011   Red Hat, Inc.  All rights reserved.
 *  Copyright (C) 2011   Mike Christie
 */

// Dependencies supplied by the Linux kernel and other translation units.

unsafe fn uptr64(val: u64) -> *mut core::ffi::c_void { val as usize as *mut core::ffi::c_void }

#[repr(C)]
static bsg_mq_ops: blk_mq_ops = blk_mq_ops {
    queue_rq: Some(bsg_queue_rq),
    init_request: Some(bsg_init_rq),
    exit_request: Some(bsg_exit_rq),
    complete: Some(bsg_complete_request),
    timeout: Some(bsg_timeout),
};

#[repr(C)]
struct BsgSet {
    tag_set: blk_mq_tag_set,
    bd: *mut bsg_device,
    job_fn: Option<unsafe extern "C" fn(*mut bsg_job) -> i32>,
    timeout_fn: Option<unsafe extern "C" fn(*mut request) -> blk_eh_timer_return>,
}

unsafe fn bsg_transport_sg_io_fn(
    q: *mut request_queue,
    hdr: *mut sg_io_v4,
    open_for_write: bool,
    timeout: u32,
) -> i32 {
    let hdr_ref = &mut *hdr;
    let mut job: *mut bsg_job;
    let rq: *mut request;
    let bio: *mut bio;
    let reply: *mut core::ffi::c_void;
    let mut ret: i32;

    if hdr_ref.protocol != BSG_PROTOCOL_SCSI
        || hdr_ref.subprotocol != BSG_SUB_PROTOCOL_SCSI_TRANSPORT
    {
        return -EINVAL;
    }
    if !capable(CAP_SYS_RAWIO) {
        return -EPERM;
    }

    rq = blk_mq_alloc_request(
        q,
        if hdr_ref.dout_xfer_len != 0 { REQ_OP_DRV_OUT } else { REQ_OP_DRV_IN },
        0,
    );
    if IS_ERR(rq) {
        return PTR_ERR(rq);
    }
    (*rq).timeout = timeout;

    job = blk_mq_rq_to_pdu(rq);
    reply = (*job).reply;
    core::ptr::write_bytes(job as *mut u8, 0, core::mem::size_of::<bsg_job>());
    (*job).reply = reply;
    (*job).reply_len = SCSI_SENSE_BUFFERSIZE;
    (*job).dd_data = job.add(1) as *mut core::ffi::c_void;

    (*job).request_len = hdr_ref.request_len;
    (*job).request = memdup_user(uptr64(hdr_ref.request), hdr_ref.request_len);
    if IS_ERR((*job).request) {
        ret = PTR_ERR((*job).request);
        goto_out_free_rq!(rq, ret);
    }

    if hdr_ref.dout_xfer_len != 0 && hdr_ref.din_xfer_len != 0 {
        (*job).bidi_rq = blk_mq_alloc_request((*rq).q, REQ_OP_DRV_IN, 0);
        if IS_ERR((*job).bidi_rq) {
            ret = PTR_ERR((*job).bidi_rq);
            goto_out_free_job_request!(rq, job, ret);
        }
        ret = blk_rq_map_user((*rq).q, (*job).bidi_rq, core::ptr::null_mut(),
            uptr64(hdr_ref.din_xferp), hdr_ref.din_xfer_len, GFP_KERNEL);
        if ret != 0 { goto_out_free_bidi_rq!(rq, job, ret); }
        (*job).bidi_bio = (*(*job).bidi_rq).bio;
    } else {
        (*job).bidi_rq = core::ptr::null_mut();
        (*job).bidi_bio = core::ptr::null_mut();
    }

    ret = 0;
    if hdr_ref.dout_xfer_len != 0 {
        ret = blk_rq_map_user((*rq).q, rq, core::ptr::null_mut(), uptr64(hdr_ref.dout_xferp),
            hdr_ref.dout_xfer_len, GFP_KERNEL);
    } else if hdr_ref.din_xfer_len != 0 {
        ret = blk_rq_map_user((*rq).q, rq, core::ptr::null_mut(), uptr64(hdr_ref.din_xferp),
            hdr_ref.din_xfer_len, GFP_KERNEL);
    }
    if ret != 0 { goto_out_unmap_bidi_rq!(rq, job, ret); }

    bio = (*rq).bio;
    blk_execute_rq(rq, (hdr_ref.flags & BSG_FLAG_Q_AT_TAIL) == 0);

    hdr_ref.device_status = (*job).result & 0xff;
    hdr_ref.transport_status = host_byte((*job).result);
    hdr_ref.driver_status = 0;
    hdr_ref.info = 0;
    if hdr_ref.device_status != 0 || hdr_ref.transport_status != 0 || hdr_ref.driver_status != 0 {
        hdr_ref.info |= SG_INFO_CHECK;
    }
    hdr_ref.response_len = 0;
    if (*job).result < 0 {
        (*job).reply_len = core::mem::size_of::<u32>() as u32;
        ret = (*job).result;
    }
    if (*job).reply_len != 0 && hdr_ref.response != 0 {
        let len = core::cmp::min(hdr_ref.max_response_len, (*job).reply_len);
        if copy_to_user(uptr64(hdr_ref.response), (*job).reply, len) != 0 {
            ret = -EFAULT;
        } else { hdr_ref.response_len = len; }
    }
    hdr_ref.dout_resid = 0;
    if !(*job).bidi_rq.is_null() {
        let rsp_len = (*job).reply_payload.payload_len;
        if WARN_ON((*job).reply_payload_rcv_len > rsp_len) { hdr_ref.din_resid = 0; }
        else { hdr_ref.din_resid = rsp_len - (*job).reply_payload_rcv_len; }
    } else { hdr_ref.din_resid = 0; }
    blk_rq_unmap_user(bio);
    if !(*job).bidi_rq.is_null() { blk_rq_unmap_user((*job).bidi_bio); }
    if !(*job).bidi_rq.is_null() { blk_mq_free_request((*job).bidi_rq); }
    kfree((*job).request);
    blk_mq_free_request(rq);
    ret
}

// The remaining helpers retain the kernel ABI and are expressed as direct unsafe wrappers.
unsafe fn bsg_teardown_job(kref: *mut kref) {
    let job = container_of!(kref, bsg_job, kref);
    let rq = blk_mq_rq_from_pdu(job);
    put_device((*job).dev);
    kfree((*job).request_payload.sg_list);
    kfree((*job).reply_payload.sg_list);
    blk_mq_end_request(rq, BLK_STS_OK);
}

pub unsafe fn bsg_job_put(job: *mut bsg_job) { kref_put(&mut (*job).kref, bsg_teardown_job); }
pub unsafe fn bsg_job_get(job: *mut bsg_job) -> i32 { kref_get_unless_zero(&mut (*job).kref) }

pub unsafe fn bsg_job_done(job: *mut bsg_job, result: i32, reply_payload_rcv_len: u32) {
    let rq = blk_mq_rq_from_pdu(job);
    (*job).result = result;
    (*job).reply_payload_rcv_len = reply_payload_rcv_len;
    if likely(!blk_should_fake_timeout((*rq).q)) { blk_mq_complete_request(rq); }
}

unsafe fn bsg_complete(rq: *mut request) { bsg_job_put(blk_mq_rq_to_pdu(rq)); }

unsafe fn bsg_map_buffer(buf: *mut bsg_buffer, req: *mut request) -> i32 {
    let sz = core::mem::size_of::<scatterlist>() * (*req).nr_phys_segments as usize;
    BUG_ON((*req).nr_phys_segments == 0);
    (*buf).sg_list = kmalloc(sz, GFP_KERNEL);
    if (*buf).sg_list.is_null() { return -ENOMEM; }
    sg_init_table((*buf).sg_list, (*req).nr_phys_segments);
    (*buf).sg_cnt = blk_rq_map_sg((*req).q, req, (*buf).sg_list);
    (*buf).payload_len = blk_rq_bytes(req);
    0
}

unsafe fn bsg_prepare_job(dev: *mut device, req: *mut request) -> bool {
    let job = blk_mq_rq_to_pdu(req);
    (*job).timeout = (*req).timeout;
    if !(*req).bio.is_null() && bsg_map_buffer(&mut (*job).request_payload, req) != 0 { (*job).result = -ENOMEM; return false; }
    if !(*job).bidi_rq.is_null() && bsg_map_buffer(&mut (*job).reply_payload, (*job).bidi_rq) != 0 { kfree((*job).request_payload.sg_list); (*job).result = -ENOMEM; return false; }
    (*job).dev = dev; get_device((*job).dev); kref_init(&mut (*job).kref); true
}

// Queue setup and teardown retain their original external kernel calls and callback wiring.
pub unsafe fn bsg_remove_queue(q: *mut request_queue) {
    if !q.is_null() { let bset = container_of!((*q).tag_set, BsgSet, tag_set); bsg_unregister_queue((*bset).bd); blk_mq_destroy_queue(q); blk_put_queue(q); blk_mq_free_tag_set(&mut (*bset).tag_set); kfree(bset); }
}

unsafe fn bsg_timeout(rq: *mut request) -> blk_eh_timer_return {
    let bset = container_of!((*(*rq).q).tag_set, BsgSet, tag_set);
    match (*bset).timeout_fn { Some(f) => f(rq), None => BLK_EH_DONE }
}

unsafe fn bsg_init_rq(_set: *mut blk_mq_tag_set, req: *mut request, _hctx_idx: u32, _numa_node: i32) -> i32 {
    let job = blk_mq_rq_to_pdu(req);
    (*job).reply = kzalloc(SCSI_SENSE_BUFFERSIZE, GFP_KERNEL);
    if (*job).reply.is_null() { -ENOMEM } else { 0 }
}

unsafe fn bsg_exit_rq(_set: *mut blk_mq_tag_set, req: *mut request, _hctx_idx: u32) {
    kfree((*blk_mq_rq_to_pdu(req)).reply);
}

unsafe fn bsg_queue_rq(hctx: *mut blk_mq_hw_ctx, bd: *const blk_mq_queue_data) -> blk_status_t {
    let q = (*hctx).queue;
    let dev = (*q).queuedata;
    let req = (*bd).rq;
    let bset = container_of!((*q).tag_set, BsgSet, tag_set);
    let mut sts = BLK_STS_IOERR;
    blk_mq_start_request(req);
    if !get_device(dev) { return sts; }
    if bsg_prepare_job(dev, req) {
        if let Some(f) = (*bset).job_fn { if f(blk_mq_rq_to_pdu(req)) == 0 { sts = BLK_STS_OK; } }
    }
    put_device(dev);
    sts
}

unsafe fn bsg_complete_request(rq: *mut request) { bsg_job_put(blk_mq_rq_to_pdu(rq)); }

unsafe fn bsg_setup_queue(dev: *mut device, name: *const i8, lim: *mut queue_limits,
    job_fn: Option<unsafe extern "C" fn(*mut bsg_job) -> i32>,
    timeout: Option<unsafe extern "C" fn(*mut request) -> blk_eh_timer_return>,
    dd_job_size: i32) -> *mut request_queue {
    let bset = kzalloc(core::mem::size_of::<BsgSet>(), GFP_KERNEL) as *mut BsgSet;
    if bset.is_null() { return ERR_PTR(-ENOMEM); }
    (*bset).job_fn = job_fn;
    (*bset).timeout_fn = timeout;
    let set = &mut (*bset).tag_set;
    (*set).ops = &bsg_mq_ops;
    (*set).nr_hw_queues = 1;
    (*set).queue_depth = 128;
    (*set).numa_node = NUMA_NO_NODE;
    (*set).cmd_size = (core::mem::size_of::<bsg_job>() as i32) + dd_job_size;
    (*set).flags = BLK_MQ_F_BLOCKING;
    if blk_mq_alloc_tag_set(set) != 0 { kfree(bset as *mut _); return ERR_PTR(-ENOMEM); }
    let q = blk_mq_alloc_queue(set, lim, dev);
    if IS_ERR(q) { blk_mq_free_tag_set(set); kfree(bset as *mut _); return q; }
    blk_queue_rq_timeout(q, BLK_DEFAULT_SG_TIMEOUT);
    (*bset).bd = bsg_register_queue(q, dev, name, bsg_transport_sg_io_fn, core::ptr::null_mut());
    if IS_ERR((*bset).bd) { let ret = PTR_ERR((*bset).bd); blk_mq_destroy_queue(q); blk_put_queue(q); blk_mq_free_tag_set(set); kfree(bset as *mut _); return ERR_PTR(ret); }
    q
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
