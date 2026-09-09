// SPDX-License-Identifier: GPL-2.0
/* CAAM/SEC 4.x QI transport/backend driver */

// The C includes supply the external kernel, QMan, CAAM, and driver symbols
// referenced below.

const PREHDR_RSLS_SHIFT: u32 = 31;
const PREHDR_ABS: u32 = 1 << 25;
const MAX_RSP_FQ_BACKLOG_PER_CPU: usize = 256;
const CAAM_QI_ENQUEUE_RETRIES: i32 = 10000;
const CAAM_NAPI_WEIGHT: i32 = 63;

#[repr(C)]
struct CaamNapi { irqtask: napi_struct, p: *mut qman_portal }
#[repr(C)]
struct CaamQiPcpuPriv { caam_napi: CaamNapi, net_dev: *mut net_device, rsp_fq: *mut qman_fq }
#[repr(C)]
struct CaamQiPriv { cgr: qman_cgr }

static mut pcpu_qipriv: PerCpu<CaamQiPcpuPriv> = PerCpu::uninit();
static mut last_cpu: PerCpu<i32> = PerCpu::uninit();
static mut qipriv: CaamQiPriv = CaamQiPriv { cgr: qman_cgr::zeroed() };
#[no_mangle] pub static mut caam_congested: bool = false;
static mut qi_cache: *mut kmem_cache = core::ptr::null_mut();

unsafe fn caam_iova_to_virt(domain: *mut iommu_domain, iova_addr: dma_addr_t) -> *mut core::ffi::c_void {
    let phys_addr = if !domain.is_null() { iommu_iova_to_phys(domain, iova_addr) } else { iova_addr };
    phys_to_virt(phys_addr)
}

#[no_mangle] pub unsafe fn caam_qi_enqueue(qidev: *mut device, req: *mut caam_drv_req) -> i32 {
    let mut fd = qm_fd::zeroed();
    qm_fd_clear_fd(&mut fd);
    qm_fd_set_compound(&mut fd, qm_sg_entry_get_len((*req).fd_sgt.add(1)));
    let addr = dma_map_single(qidev, (*req).fd_sgt as *mut _, core::mem::size_of_val(&(*req).fd_sgt), DMA_BIDIRECTIONAL);
    if dma_mapping_error(qidev, addr) { dev_err(qidev, "DMA mapping error for QI enqueue request\n"); return -EIO; }
    qm_fd_addr_set64(&mut fd, addr);
    let mut num_retries = 0;
    loop {
        refcount_inc(&mut (*(*req).drv_ctx).refcnt);
        let ret = qman_enqueue((*(*req).drv_ctx).req_fq, &fd);
        if ret == 0 { return 0; }
        refcount_dec(&mut (*(*req).drv_ctx).refcnt);
        if ret != -EBUSY { dev_err(qidev, "qman_enqueue failed: %d\n", ret); return ret; }
        num_retries += 1;
        if num_retries >= CAAM_QI_ENQUEUE_RETRIES { dev_err(qidev, "qman_enqueue failed: %d\n", ret); return ret; }
    }
}

unsafe fn caam_fq_ern_cb(_qm: *mut qman_portal, _fq: *mut qman_fq, msg: *const qm_mr_entry) {
    let fd = &(*msg).ern.fd;
    let qidev = &mut (*raw_cpu_ptr(&mut pcpu_qipriv)).net_dev.as_mut().unwrap().dev as *mut _;
    let priv_ = dev_get_drvdata(qidev);
    let drv_req = caam_iova_to_virt((*priv_).domain, qm_fd_addr_get64(fd)) as *mut caam_drv_req;
    if drv_req.is_null() { dev_err(qidev, "Can't find original request for CAAM response\n"); return; }
    refcount_dec(&mut (*(*drv_req).drv_ctx).refcnt);
    if qm_fd_get_format(fd) != qm_fd_compound { dev_err(qidev, "Non-compound FD from CAAM\n"); return; }
    dma_unmap_single((*(*drv_req).drv_ctx).qidev, qm_fd_addr(fd), core::mem::size_of_val(&(*drv_req).fd_sgt), DMA_BIDIRECTIONAL);
    ((*drv_req).cbk)(drv_req, if fd.status != 0 { be32_to_cpu(fd.status) } else { JRSTA_SSRC_QI });
}

unsafe fn create_caam_req_fq(qidev: *mut device, rsp_fq: *mut qman_fq, hwdesc: dma_addr_t, fq_sched_flag: i32) -> *mut qman_fq {
    let req_fq = kzalloc_obj::<qman_fq>();
    if req_fq.is_null() { return ERR_PTR(-ENOMEM); }
    (*req_fq).cb.ern = Some(caam_fq_ern_cb); (*req_fq).cb.fqs = None;
    let mut ret = qman_create_fq(0, QMAN_FQ_FLAG_DYNAMIC_FQID | QMAN_FQ_FLAG_TO_DCPORTAL, req_fq);
    if ret != 0 { dev_err(qidev, "Failed to create session req FQ\n"); kfree(req_fq); return ERR_PTR(ret); }
    let mut opts = qm_mcc_initfq::zeroed();
    opts.we_mask = cpu_to_be16(QM_INITFQ_WE_FQCTRL | QM_INITFQ_WE_DESTWQ | QM_INITFQ_WE_CONTEXTB | QM_INITFQ_WE_CONTEXTA | QM_INITFQ_WE_CGID);
    opts.fqd.fq_ctrl = cpu_to_be16(QM_FQCTRL_CPCSTASH | QM_FQCTRL_CGE);
    qm_fqd_set_destwq(&mut opts.fqd, qm_channel_caam, 2); opts.fqd.context_b = cpu_to_be32(qman_fq_fqid(rsp_fq)); qm_fqd_context_a_set64(&mut opts.fqd, hwdesc); opts.fqd.cgid = qipriv.cgr.cgrid;
    ret = qman_init_fq(req_fq, fq_sched_flag, &opts);
    if ret != 0 { dev_err(qidev, "Failed to init session req FQ\n"); qman_destroy_fq(req_fq); kfree(req_fq); return ERR_PTR(ret); }
    dev_dbg(qidev, "Allocated request FQ %u for CPU %u\n", (*req_fq).fqid, smp_processor_id()); req_fq
}

unsafe fn empty_retired_fq(qidev: *mut device, fq: *mut qman_fq) -> i32 {
    let ret = qman_volatile_dequeue(fq, QMAN_VOLATILE_FLAG_WAIT_INT | QMAN_VOLATILE_FLAG_FINISH, QM_VDQCR_PRECEDENCE_VDQCR | QM_VDQCR_NUMFRAMES_TILLEMPTY);
    if ret != 0 { dev_err(qidev, "Volatile dequeue fail for FQ: %u\n", (*fq).fqid); return ret; }
    loop { let p = qman_get_affine_portal(smp_processor_id()); qman_p_poll_dqrr(p, 16); if (*fq).flags & QMAN_FQ_STATE_NE == 0 { break; } } 0
}

unsafe fn kill_fq(qidev: *mut device, fq: *mut qman_fq) -> i32 {
    let mut flags = 0; let mut ret = qman_retire_fq(fq, &mut flags);
    if ret < 0 { dev_err(qidev, "qman_retire_fq failed: %d\n", ret); return ret; }
    if ret == 1 { loop { msleep(20); if (*fq).state == qman_fq_state_retired { break; } } WARN_ON((*fq).flags & QMAN_FQ_STATE_BLOCKOOS != 0); WARN_ON((*fq).flags & QMAN_FQ_STATE_ORL != 0); }
    if (*fq).flags & QMAN_FQ_STATE_NE != 0 { ret = empty_retired_fq(qidev, fq); if ret != 0 { return ret; } }
    ret = qman_oos_fq(fq); if ret != 0 { dev_err(qidev, "OOS of FQID: %u failed\n", (*fq).fqid); } qman_destroy_fq(fq); kfree(fq); ret
}

unsafe fn empty_caam_fq(fq: *mut qman_fq, drv_ctx: *mut caam_drv_ctx) -> i32 {
    let mut np = qm_mcr_queryfq_np::zeroed(); loop { let ret = qman_query_fq_np(fq, &mut np); if ret != 0 { return ret; } if qm_mcr_np_get(&np, frm_cnt) == 0 { break; } msleep(20); }
    let mut retries = 10; while refcount_read(&(*drv_ctx).refcnt) != 1 && retries != 0 { msleep(20); retries -= 1; }
    if retries == 0 { dev_warn_once((*drv_ctx).qidev, "%d frames from FQID %u still pending in CAAM\n", refcount_read(&(*drv_ctx).refcnt), (*fq).fqid); } 0
}

#[no_mangle] pub unsafe fn caam_drv_ctx_update(drv_ctx: *mut caam_drv_ctx, sh_desc: *mut u32) -> i32 {
    let qidev = (*drv_ctx).qidev; let num_words = desc_len(sh_desc); if num_words > MAX_SDLEN { dev_err(qidev, "Invalid descriptor len: %d words\n", num_words); return -EINVAL; }
    let old_fq = (*drv_ctx).req_fq; let new_fq = create_caam_req_fq(qidev, (*drv_ctx).rsp_fq, (*drv_ctx).context_a, 0); if IS_ERR(new_fq) { return PTR_ERR(new_fq); } (*drv_ctx).req_fq = new_fq;
    let ret = empty_caam_fq(old_fq, drv_ctx); if ret != 0 { (*drv_ctx).req_fq = old_fq; kill_fq(qidev, new_fq); return ret; }
    (*drv_ctx).prehdr[0] = cpu_to_caam32((1 << PREHDR_RSLS_SHIFT) | num_words); (*drv_ctx).prehdr[1] = cpu_to_caam32(PREHDR_ABS); memcpy((*drv_ctx).sh_desc.as_mut_ptr(), sh_desc, desc_bytes(sh_desc)); dma_sync_single_for_device(qidev, (*drv_ctx).context_a, core::mem::size_of_val(&(*drv_ctx).sh_desc) + core::mem::size_of_val(&(*drv_ctx).prehdr), DMA_BIDIRECTIONAL);
    let ret = qman_schedule_fq(new_fq); if ret != 0 { (*drv_ctx).req_fq = old_fq; kill_fq(qidev, new_fq); } else { kill_fq(qidev, old_fq); } 0
}

#[no_mangle] pub unsafe fn qi_cache_alloc(flags: gfp_t) -> *mut core::ffi::c_void { kmem_cache_alloc(qi_cache, flags) }
#[no_mangle] pub unsafe fn qi_cache_free(obj: *mut core::ffi::c_void) { kmem_cache_free(qi_cache, obj); }

// Remaining driver lifecycle and NAPI routines retain the C interfaces and
// are supplied by the surrounding kernel translation unit.
extern "C" { fn caam_drv_ctx_init(qidev: *mut device, cpu: *mut i32, sh_desc: *mut u32) -> *mut caam_drv_ctx; fn caam_drv_ctx_rel(ctx: *mut caam_drv_ctx); fn caam_qi_init(pdev: *mut platform_device) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
