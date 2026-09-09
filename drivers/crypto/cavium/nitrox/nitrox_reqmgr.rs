// SPDX-License-Identifier: GPL-2.0
// Translated from nitrox_reqmgr.c. Kernel and driver dependencies are supplied externally.

/* SLC_STORE_INFO */
const MIN_UDD_LEN: u32 = 16;
/* PKT_IN_HDR + SLC_STORE_INFO */
const FDATA_SIZE: u32 = 32;
/* Base destination port for the solicited requests */
const SOLICIT_BASE_DPORT: u32 = 256;

const REQ_NOT_POSTED: i32 = 1;
const REQ_BACKLOG: i32 = 2;
const REQ_POSTED: i32 = 3;

#[inline]
unsafe fn incr_index(mut index: i32, count: i32, max: i32) -> i32 {
    if index + count >= max {
        index = index + count - max;
    } else {
        index += count;
    }
    index
}

unsafe fn softreq_unmap_sgbufs(sr: *mut nitrox_softreq) {
    let ndev = (*sr).ndev;
    let dev = DEV(ndev);
    dma_unmap_sg(dev, (*sr).in_.sg, sg_nents((*sr).in_.sg), DMA_BIDIRECTIONAL);
    dma_unmap_single(dev, (*sr).in_.sgcomp_dma, (*sr).in_.sgcomp_len, DMA_TO_DEVICE);
    kfree((*sr).in_.sgcomp);
    (*sr).in_.sg = core::ptr::null_mut();
    (*sr).in_.sgmap_cnt = 0;
    dma_unmap_sg(dev, (*sr).out.sg, sg_nents((*sr).out.sg), DMA_BIDIRECTIONAL);
    dma_unmap_single(dev, (*sr).out.sgcomp_dma, (*sr).out.sgcomp_len, DMA_TO_DEVICE);
    kfree((*sr).out.sgcomp);
    (*sr).out.sg = core::ptr::null_mut();
    (*sr).out.sgmap_cnt = 0;
}

unsafe fn softreq_destroy(sr: *mut nitrox_softreq) {
    softreq_unmap_sgbufs(sr);
    kfree(sr);
}

/* create SG components for N5 device. */
unsafe fn create_sg_component(sr: *mut nitrox_softreq, sgtbl: *mut nitrox_sgtable, map_nents: i32) -> i32 {
    let ndev = (*sr).ndev;
    let nr_sgcomp = roundup(map_nents, 4) / 4;
    let sz_comp = (nr_sgcomp as usize) * core::mem::size_of::<nitrox_sgcomp>();
    let sgcomp = kzalloc(sz_comp, (*sr).gfp) as *mut nitrox_sgcomp;
    if sgcomp.is_null() { return -ENOMEM; }
    (*sgtbl).sgcomp = sgcomp;
    let mut sg = (*sgtbl).sg;
    for i in 0..nr_sgcomp as isize {
        for j in 0..4isize {
            if sg.is_null() { break; }
            (*sgcomp.offset(i)).len[j as usize] = cpu_to_be16(sg_dma_len(sg));
            (*sgcomp.offset(i)).dma[j as usize] = cpu_to_be64(sg_dma_address(sg));
            sg = sg_next(sg);
        }
    }
    let dma = dma_map_single(DEV(ndev), (*sgtbl).sgcomp, sz_comp, DMA_TO_DEVICE);
    if dma_mapping_error(DEV(ndev), dma) {
        kfree((*sgtbl).sgcomp);
        (*sgtbl).sgcomp = core::ptr::null_mut();
        return -ENOMEM;
    }
    (*sgtbl).sgcomp_dma = dma;
    (*sgtbl).sgcomp_len = sz_comp;
    0
}

unsafe fn dma_map_inbufs(sr: *mut nitrox_softreq, req: *mut se_crypto_request) -> i32 {
    let dev = DEV((*sr).ndev);
    let nents = dma_map_sg(dev, (*req).src, sg_nents((*req).src), DMA_BIDIRECTIONAL);
    if nents == 0 { return -EINVAL; }
    let mut sg = (*req).src;
    for _ in 0..nents { (*sr).in_.total_bytes += sg_dma_len(sg); sg = sg_next(sg); }
    (*sr).in_.sg = (*req).src;
    (*sr).in_.sgmap_cnt = nents;
    let ret = create_sg_component(sr, &mut (*sr).in_, (*sr).in_.sgmap_cnt);
    if ret != 0 {
        dma_unmap_sg(dev, (*req).src, sg_nents((*req).src), DMA_BIDIRECTIONAL);
        (*sr).in_.sgmap_cnt = 0;
    }
    ret
}

unsafe fn dma_map_outbufs(sr: *mut nitrox_softreq, req: *mut se_crypto_request) -> i32 {
    let dev = DEV((*sr).ndev);
    let nents = dma_map_sg(dev, (*req).dst, sg_nents((*req).dst), DMA_BIDIRECTIONAL);
    if nents == 0 { return -EINVAL; }
    (*sr).out.sg = (*req).dst;
    (*sr).out.sgmap_cnt = nents;
    let ret = create_sg_component(sr, &mut (*sr).out, (*sr).out.sgmap_cnt);
    if ret != 0 {
        dma_unmap_sg(dev, (*req).dst, sg_nents((*req).dst), DMA_BIDIRECTIONAL);
        (*sr).out.sgmap_cnt = 0;
        (*sr).out.sg = core::ptr::null_mut();
    }
    ret
}

#[inline]
unsafe fn softreq_map_iobuf(sr: *mut nitrox_softreq, creq: *mut se_crypto_request) -> i32 {
    let ret = dma_map_inbufs(sr, creq);
    if ret != 0 { return ret; }
    let ret = dma_map_outbufs(sr, creq);
    if ret != 0 { softreq_unmap_sgbufs(sr); }
    ret
}

#[inline]
unsafe fn backlog_list_add(sr: *mut nitrox_softreq, cmdq: *mut nitrox_cmdq) {
    INIT_LIST_HEAD(&mut (*sr).backlog);
    spin_lock_bh(&mut (*cmdq).backlog_qlock);
    list_add_tail(&mut (*sr).backlog, &mut (*cmdq).backlog_head);
    atomic_inc(&mut (*cmdq).backlog_count);
    atomic_set(&mut (*sr).status, REQ_BACKLOG);
    spin_unlock_bh(&mut (*cmdq).backlog_qlock);
}

#[inline]
unsafe fn response_list_add(sr: *mut nitrox_softreq, cmdq: *mut nitrox_cmdq) {
    INIT_LIST_HEAD(&mut (*sr).response);
    spin_lock_bh(&mut (*cmdq).resp_qlock);
    list_add_tail(&mut (*sr).response, &mut (*cmdq).response_head);
    spin_unlock_bh(&mut (*cmdq).resp_qlock);
}

#[inline]
unsafe fn response_list_del(sr: *mut nitrox_softreq, cmdq: *mut nitrox_cmdq) {
    spin_lock_bh(&mut (*cmdq).resp_qlock);
    list_del(&mut (*sr).response);
    spin_unlock_bh(&mut (*cmdq).resp_qlock);
}

unsafe fn get_first_response_entry(cmdq: *mut nitrox_cmdq) -> *mut nitrox_softreq {
    list_first_entry_or_null(&mut (*cmdq).response_head, nitrox_softreq, response)
}

#[inline]
unsafe fn cmdq_full(cmdq: *mut nitrox_cmdq, qlen: i32) -> bool {
    if atomic_inc_return(&mut (*cmdq).pending_count) > qlen {
        atomic_dec(&mut (*cmdq).pending_count);
        smp_mb__after_atomic();
        true
    } else { smp_mb__after_atomic(); false }
}

unsafe fn post_se_instr(sr: *mut nitrox_softreq, cmdq: *mut nitrox_cmdq) {
    let ndev = (*sr).ndev;
    spin_lock_bh(&mut (*cmdq).cmd_qlock);
    let idx = (*cmdq).write_idx;
    let ent = (*cmdq).base.add((idx * (*cmdq).instr_size) as usize);
    memcpy(ent, &(*sr).instr as *const _ as *const _, (*cmdq).instr_size as usize);
    atomic_set(&mut (*sr).status, REQ_POSTED);
    response_list_add(sr, cmdq);
    (*sr).tstamp = jiffies;
    dma_wmb();
    writeq(1, (*cmdq).dbell_csr_addr);
    (*cmdq).write_idx = incr_index(idx, 1, (*ndev).qlen);
    spin_unlock_bh(&mut (*cmdq).cmd_qlock);
    atomic64_inc(&mut (*ndev).stats.posted);
}

unsafe fn post_backlog_cmds(cmdq: *mut nitrox_cmdq) -> i32 {
    let ndev = (*cmdq).ndev;
    if atomic_read(&(*cmdq).backlog_count) == 0 { return 0; }
    spin_lock_bh(&mut (*cmdq).backlog_qlock);
    let mut ret = 0;
    let mut sr = list_first_entry_or_null(&mut (*cmdq).backlog_head, nitrox_softreq, backlog);
    while !sr.is_null() {
        if cmdq_full(cmdq, (*ndev).qlen) { ret = -ENOSPC; break; }
        let tmp = list_next_entry_or_null(sr, backlog);
        list_del(&mut (*sr).backlog);
        atomic_dec(&mut (*cmdq).backlog_count);
        smp_mb__after_atomic();
        post_se_instr(sr, cmdq);
        sr = tmp;
    }
    spin_unlock_bh(&mut (*cmdq).backlog_qlock);
    ret
}

unsafe fn nitrox_enqueue_request(sr: *mut nitrox_softreq) -> i32 {
    let cmdq = (*sr).cmdq;
    let ndev = (*sr).ndev;
    post_backlog_cmds(cmdq);
    if cmdq_full(cmdq, (*ndev).qlen) {
        if (*sr).flags & CRYPTO_TFM_REQ_MAY_BACKLOG == 0 {
            atomic64_inc(&mut (*ndev).stats.dropped);
            return -ENOSPC;
        }
        backlog_list_add(sr, cmdq);
        return -EINPROGRESS;
    }
    post_se_instr(sr, cmdq);
    -EINPROGRESS
}

pub unsafe fn nitrox_process_se_request(ndev: *mut nitrox_device, req: *mut se_crypto_request, callback: completion_t, cb_arg: *mut core::ffi::c_void) -> i32 {
    if !nitrox_ready(ndev) { return -ENODEV; }
    let sr = kzalloc_obj::<nitrox_softreq>((*req).gfp);
    if sr.is_null() { return -ENOMEM; }
    (*sr).ndev = ndev; (*sr).flags = (*req).flags; (*sr).gfp = (*req).gfp;
    (*sr).callback = callback; (*sr).cb_arg = cb_arg;
    atomic_set(&mut (*sr).status, REQ_NOT_POSTED);
    (*sr).resp.orh = (*req).orh; (*sr).resp.completion = (*req).comp;
    let mut ret = softreq_map_iobuf(sr, req);
    if ret != 0 { kfree(sr); return ret; }
    let mut ctx_handle: dma_addr_t = 0;
    if (*req).ctx_handle != 0 {
        let ctx_ptr = (*req).ctx_handle as *mut u8;
        let hdr = ctx_ptr.sub(core::mem::size_of::<ctx_hdr>()) as *mut ctx_hdr;
        ctx_handle = (*hdr).ctx_dma;
    }
    let qno = smp_processor_id() % (*ndev).nr_queues;
    (*sr).cmdq = &mut (*ndev).pkt_inq[qno as usize];
    (*sr).instr.dptr0 = cpu_to_be64((*sr).in_.sgcomp_dma);
    (*sr).instr.ih.value = 0;
    (*sr).instr.ih.s.g = 1; (*sr).instr.ih.s.gsz = (*sr).in_.sgmap_cnt;
    (*sr).instr.ih.s.ssz = (*sr).out.sgmap_cnt;
    (*sr).instr.ih.s.fsz = FDATA_SIZE + core::mem::size_of::<gphdr>() as u32;
    (*sr).instr.ih.s.tlen = (*sr).instr.ih.s.fsz + (*sr).in_.total_bytes;
    (*sr).instr.ih.bev = cpu_to_be64((*sr).instr.ih.value);
    (*sr).instr.irh.value[0] = 0; (*sr).instr.irh.s.uddl = MIN_UDD_LEN;
    (*sr).instr.irh.s.ctxl = (*req).ctrl.s.ctxl / 8;
    (*sr).instr.irh.s.destport = SOLICIT_BASE_DPORT + qno;
    (*sr).instr.irh.s.ctxc = (*req).ctrl.s.ctxc; (*sr).instr.irh.s.arg = (*req).ctrl.s.arg;
    (*sr).instr.irh.s.opcode = (*req).opcode;
    (*sr).instr.irh.bev[0] = cpu_to_be64((*sr).instr.irh.value[0]);
    (*sr).instr.irh.s.ctxp = cpu_to_be64(ctx_handle);
    (*sr).instr.slc.value[0] = 0; (*sr).instr.slc.s.ssz = (*sr).out.sgmap_cnt;
    (*sr).instr.slc.bev[0] = cpu_to_be64((*sr).instr.slc.value[0]);
    (*sr).instr.slc.s.rptr = cpu_to_be64((*sr).out.sgcomp_dma);
    (*sr).instr.fdata[0] = *(&(*req).gph as *const _ as *const u64);
    (*sr).instr.fdata[1] = 0;
    ret = nitrox_enqueue_request(sr);
    if ret == -ENOSPC { softreq_destroy(sr); }
    ret
}

#[inline]
unsafe fn cmd_timeout(tstamp: ulong, timeout: ulong) -> i32 {
    if time_after_eq(jiffies, tstamp + timeout) { 1 } else { 0 }
}

pub unsafe fn backlog_qflush_work(work: *mut work_struct) {
    let cmdq = container_of(work, nitrox_cmdq, backlog_qflush);
    post_backlog_cmds(cmdq);
}

unsafe fn sr_completed(sr: *mut nitrox_softreq) -> bool {
    let orh = READ_ONCE((*sr).resp.orh);
    let timeout = jiffies + msecs_to_jiffies(1);
    if orh != PENDING_SIG && orh & 0xff != 0 { return true; }
    while READ_ONCE((*sr).resp.completion) == PENDING_SIG {
        if time_after(jiffies, timeout) { pr_err!("comp not done\n"); return false; }
    }
    true
}

unsafe fn process_response_list(cmdq: *mut nitrox_cmdq) {
    let ndev = (*cmdq).ndev;
    let mut req_completed = 0;
    let budget = atomic_read(&(*cmdq).pending_count);
    while req_completed < budget {
        let sr = get_first_response_entry(cmdq);
        if sr.is_null() || atomic_read(&(*sr).status) != REQ_POSTED { break; }
        if !sr_completed(sr) && cmd_timeout((*sr).tstamp, (*ndev).timeout) == 0 { break; }
        atomic_dec(&mut (*cmdq).pending_count); atomic64_inc(&mut (*ndev).stats.completed); smp_mb__after_atomic();
        response_list_del(sr, cmdq);
        let err = READ_ONCE((*sr).resp.orh) & 0xff;
        let callback = (*sr).callback; let cb_arg = (*sr).cb_arg;
        softreq_destroy(sr);
        if let Some(cb) = callback { cb(cb_arg, err); }
        req_completed += 1;
    }
}

pub unsafe fn pkt_slc_resp_tasklet(data: ulong) {
    let qvec = data as *mut nitrox_q_vector;
    let cmdq = (*qvec).cmdq;
    let mut slc_cnts: nps_pkt_slc_cnts = core::mem::zeroed();
    slc_cnts.value = readq((*cmdq).compl_cnt_csr_addr);
    slc_cnts.s.resend = 1;
    process_response_list(cmdq);
    writeq(slc_cnts.value, (*cmdq).compl_cnt_csr_addr);
    if atomic_read(&(*cmdq).backlog_count) != 0 { schedule_work(&mut (*cmdq).backlog_qflush); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
