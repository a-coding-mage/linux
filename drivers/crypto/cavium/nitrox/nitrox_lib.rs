// SPDX-License-Identifier: GPL-2.0
// Translated from nitrox_lib.c; Linux and local header declarations are external dependencies.

const CRYPTO_CTX_SIZE: usize = 256;
const PKTIN_Q_ALIGN_BYTES: usize = 16;
const AQM_Q_ALIGN_BYTES: usize = 32;

unsafe fn nitrox_cmdq_init(cmdq: *mut nitrox_cmdq, align_bytes: usize) -> i32 {
    let ndev = (*cmdq).ndev;

    (*cmdq).qsize = ((*ndev).qlen * (*cmdq).instr_size) + align_bytes;
    (*cmdq).unalign_base = dma_alloc_coherent(
        DEV(ndev),
        (*cmdq).qsize,
        &mut (*cmdq).unalign_dma,
        GFP_KERNEL,
    );
    if (*cmdq).unalign_base.is_null() {
        return -ENOMEM;
    }

    (*cmdq).dma = ptr_align((*cmdq).unalign_dma, align_bytes);
    (*cmdq).base = (*cmdq).unalign_base.add(
        ((*cmdq).dma - (*cmdq).unalign_dma) as usize,
    );
    (*cmdq).write_idx = 0;

    spin_lock_init(&mut (*cmdq).cmd_qlock);
    spin_lock_init(&mut (*cmdq).resp_qlock);
    spin_lock_init(&mut (*cmdq).backlog_qlock);

    INIT_LIST_HEAD(&mut (*cmdq).response_head);
    INIT_LIST_HEAD(&mut (*cmdq).backlog_head);
    INIT_WORK(&mut (*cmdq).backlog_qflush, backlog_qflush_work);

    atomic_set(&mut (*cmdq).pending_count, 0);
    atomic_set(&mut (*cmdq).backlog_count, 0);
    0
}

unsafe fn nitrox_cmdq_reset(cmdq: *mut nitrox_cmdq) {
    (*cmdq).write_idx = 0;
    atomic_set(&mut (*cmdq).pending_count, 0);
    atomic_set(&mut (*cmdq).backlog_count, 0);
}

unsafe fn nitrox_cmdq_cleanup(cmdq: *mut nitrox_cmdq) {
    if cmdq.is_null() || (*cmdq).unalign_base.is_null() {
        return;
    }

    let ndev = (*cmdq).ndev;
    cancel_work_sync(&mut (*cmdq).backlog_qflush);
    dma_free_coherent(
        DEV(ndev),
        (*cmdq).qsize,
        (*cmdq).unalign_base,
        (*cmdq).unalign_dma,
    );
    nitrox_cmdq_reset(cmdq);

    (*cmdq).dbell_csr_addr = core::ptr::null_mut();
    (*cmdq).compl_cnt_csr_addr = core::ptr::null_mut();
    (*cmdq).unalign_base = core::ptr::null_mut();
    (*cmdq).base = core::ptr::null_mut();
    (*cmdq).unalign_dma = 0;
    (*cmdq).dma = 0;
    (*cmdq).qsize = 0;
    (*cmdq).instr_size = 0;
}

unsafe fn nitrox_free_aqm_queues(ndev: *mut nitrox_device) {
    for i in 0..(*ndev).nr_queues {
        nitrox_cmdq_cleanup((*ndev).aqmq[i]);
        kfree_sensitive((*ndev).aqmq[i]);
        (*ndev).aqmq[i] = core::ptr::null_mut();
    }
}

unsafe fn nitrox_alloc_aqm_queues(ndev: *mut nitrox_device) -> i32 {
    let mut err: i32;
    for i in 0..(*ndev).nr_queues {
        let cmdq = kzalloc_node(core::mem::size_of::<nitrox_cmdq>(), GFP_KERNEL, (*ndev).node);
        if cmdq.is_null() {
            err = -ENOMEM;
            nitrox_free_aqm_queues(ndev);
            return err;
        }
        (*cmdq).ndev = ndev;
        (*cmdq).qno = i;
        (*cmdq).instr_size = core::mem::size_of::<aqmq_command_s>();
        let offset = AQMQ_DRBLX(i);
        (*cmdq).dbell_csr_addr = NITROX_CSR_ADDR(ndev, offset);
        let offset = AQMQ_CMD_CNTX(i);
        (*cmdq).compl_cnt_csr_addr = NITROX_CSR_ADDR(ndev, offset);
        err = nitrox_cmdq_init(cmdq, AQM_Q_ALIGN_BYTES);
        if err != 0 {
            kfree_sensitive(cmdq);
            nitrox_free_aqm_queues(ndev);
            return err;
        }
        (*ndev).aqmq[i] = cmdq;
    }
    0
}

unsafe fn nitrox_free_pktin_queues(ndev: *mut nitrox_device) {
    for i in 0..(*ndev).nr_queues {
        nitrox_cmdq_cleanup(&mut (*ndev).pkt_inq[i]);
    }
    kfree((*ndev).pkt_inq);
    (*ndev).pkt_inq = core::ptr::null_mut();
}

unsafe fn nitrox_alloc_pktin_queues(ndev: *mut nitrox_device) -> i32 {
    (*ndev).pkt_inq = kcalloc_node((*ndev).nr_queues, core::mem::size_of::<nitrox_cmdq>(), GFP_KERNEL, (*ndev).node);
    if (*ndev).pkt_inq.is_null() { return -ENOMEM; }
    for i in 0..(*ndev).nr_queues {
        let cmdq = &mut (*ndev).pkt_inq[i];
        (*cmdq).ndev = ndev;
        (*cmdq).qno = i;
        (*cmdq).instr_size = core::mem::size_of::<nps_pkt_instr>();
        let offset = NPS_PKT_IN_INSTR_BAOFF_DBELLX(i);
        (*cmdq).dbell_csr_addr = NITROX_CSR_ADDR(ndev, offset);
        let offset = NPS_PKT_SLC_CNTSX(i);
        (*cmdq).compl_cnt_csr_addr = NITROX_CSR_ADDR(ndev, offset);
        let err = nitrox_cmdq_init(cmdq, PKTIN_Q_ALIGN_BYTES);
        if err != 0 { nitrox_free_pktin_queues(ndev); return err; }
    }
    0
}

unsafe fn create_crypto_dma_pool(ndev: *mut nitrox_device) -> i32 {
    let size = CRYPTO_CTX_SIZE + core::mem::size_of::<ctx_hdr>();
    (*ndev).ctx_pool = dma_pool_create(c"nitrox-context", DEV(ndev), size, 16, 0);
    if (*ndev).ctx_pool.is_null() { return -ENOMEM; }
    0
}

unsafe fn destroy_crypto_dma_pool(ndev: *mut nitrox_device) {
    if (*ndev).ctx_pool.is_null() { return; }
    dma_pool_destroy((*ndev).ctx_pool);
    (*ndev).ctx_pool = core::ptr::null_mut();
}

pub unsafe fn crypto_alloc_context(ndev: *mut nitrox_device) -> *mut core::ffi::c_void {
    let chdr = kmalloc_obj::<crypto_ctx_hdr>();
    if chdr.is_null() { return core::ptr::null_mut(); }
    let mut dma: dma_addr_t = 0;
    let vaddr = dma_pool_zalloc((*ndev).ctx_pool, GFP_KERNEL, &mut dma);
    if vaddr.is_null() { kfree(chdr); return core::ptr::null_mut(); }
    let ctx = vaddr as *mut ctx_hdr;
    (*ctx).pool = (*ndev).ctx_pool;
    (*ctx).dma = dma;
    (*ctx).ctx_dma = dma + core::mem::size_of::<ctx_hdr>() as u64;
    (*chdr).pool = (*ndev).ctx_pool;
    (*chdr).dma = dma;
    (*chdr).vaddr = vaddr;
    chdr as *mut core::ffi::c_void
}

pub unsafe fn crypto_free_context(ctx: *mut core::ffi::c_void) {
    if ctx.is_null() { return; }
    let ctxp = ctx as *mut crypto_ctx_hdr;
    dma_pool_free((*ctxp).pool, (*ctxp).vaddr, (*ctxp).dma);
    kfree(ctxp);
}

pub unsafe fn nitrox_common_sw_init(ndev: *mut nitrox_device) -> i32 {
    let mut err = create_crypto_dma_pool(ndev);
    if err != 0 { return err; }
    err = nitrox_alloc_pktin_queues(ndev);
    if err != 0 { destroy_crypto_dma_pool(ndev); }
    err = nitrox_alloc_aqm_queues(ndev);
    if err != 0 { nitrox_free_pktin_queues(ndev); destroy_crypto_dma_pool(ndev); }
    err
}

pub unsafe fn nitrox_common_sw_cleanup(ndev: *mut nitrox_device) {
    nitrox_free_aqm_queues(ndev);
    nitrox_free_pktin_queues(ndev);
    destroy_crypto_dma_pool(ndev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
