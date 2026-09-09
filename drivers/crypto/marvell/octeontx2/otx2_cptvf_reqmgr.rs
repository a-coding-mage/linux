// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2020 Marvell. */

/* Dependencies supplied by otx2_cptvf.h and otx2_cpt_common.h. */

/* Default timeout when waiting for free pending entry in us */
const CPT_PENTRY_TIMEOUT: i32 = 1000;
const CPT_PENTRY_STEP: i32 = 50;

/* Default threshold for stopping and resuming sender requests */
const CPT_IQ_STOP_MARGIN: u32 = 128;
const CPT_IQ_RESUME_MARGIN: u32 = 512;

/* Default command timeout in seconds */
const CPT_COMMAND_TIMEOUT: u64 = 4;
const CPT_TIME_IN_RESET_COUNT: u8 = 5;

unsafe fn otx2_cpt_dump_sg_list(pdev: *mut pci_dev, req: *mut otx2_cpt_req_info) {
    let mut i: i32;

    pr_debug!("Gather list size %d\n", (*req).in_cnt);
    i = 0;
    while i < (*req).in_cnt {
        pr_debug!("Buffer %d size %d, vptr 0x%p, dmaptr 0x%llx\n", i,
            (*req).in[i as usize].size, (*req).in[i as usize].vptr,
            (*req).in[i as usize].dma_addr);
        pr_debug!("Buffer hexdump (%d bytes)\n", (*req).in[i as usize].size);
        print_hex_dump_debug!("", DUMP_PREFIX_NONE, 16, 1,
            (*req).in[i as usize].vptr, (*req).in[i as usize].size, false);
        i += 1;
    }
    pr_debug!("Scatter list size %d\n", (*req).out_cnt);
    i = 0;
    while i < (*req).out_cnt {
        pr_debug!("Buffer %d size %d, vptr 0x%p, dmaptr 0x%llx\n", i,
            (*req).out[i as usize].size, (*req).out[i as usize].vptr,
            (*req).out[i as usize].dma_addr);
        pr_debug!("Buffer hexdump (%d bytes)\n", (*req).out[i as usize].size);
        print_hex_dump_debug!("", DUMP_PREFIX_NONE, 16, 1,
            (*req).out[i as usize].vptr, (*req).out[i as usize].size, false);
        i += 1;
    }
}

unsafe fn get_free_pending_entry(q: *mut otx2_cpt_pending_queue, qlen: i32) -> *mut otx2_cpt_pending_entry {
    let ent = &mut (*q).head[(*q).rear as usize] as *mut _;
    if unlikely!((*ent).busy) { return core::ptr::null_mut(); }
    (*q).rear += 1;
    if unlikely!((*q).rear == qlen) { (*q).rear = 0; }
    ent
}

unsafe fn modulo_inc(mut index: u32, length: u32, inc: u32) -> u32 {
    if WARN_ON!(inc > length) { index = length; }
    index += inc;
    if unlikely!(index >= length) { index -= length; }
    index
}

unsafe fn free_pentry(pentry: *mut otx2_cpt_pending_entry) {
    (*pentry).completion_addr = core::ptr::null_mut();
    (*pentry).info = core::ptr::null_mut();
    (*pentry).callback = None;
    (*pentry).areq = core::ptr::null_mut();
    (*pentry).resume_sender = false;
    (*pentry).busy = false;
}

unsafe fn process_request(pdev: *mut pci_dev, req: *mut otx2_cpt_req_info,
    pqueue: *mut otx2_cpt_pending_queue, lf: *mut otx2_cptlf_info) -> i32 {
    let cpt_req = &mut (*req).req as *mut otx2_cptvf_request;
    let mut pentry: *mut otx2_cpt_pending_entry = core::ptr::null_mut();
    let ctrl = &mut (*req).ctrl as *mut otx2_cpt_ctrl_info;
    let mut info: *mut otx2_cpt_inst_info = core::ptr::null_mut();
    let mut result: *mut otx2_cpt_res_s = core::ptr::null_mut();
    let mut iq_cmd: otx2_cpt_iq_command = core::mem::zeroed();
    let mut cptinst: otx2_cpt_inst_s = core::mem::zeroed();
    let mut retry: i32;
    let mut ret: i32 = 0;
    let resume_sender: bool;
    let gfp: gfp_t;

    gfp = if ((*req).areq.flags & CRYPTO_TFM_REQ_MAY_SLEEP) != 0 { GFP_KERNEL } else { GFP_ATOMIC };
    if unlikely!(!otx2_cptlf_started((*lf).lfs)) { return -ENODEV; }
    info = ((*lf).lfs.ops.cpt_sg_info_create)(pdev, req, gfp);
    if unlikely!(info.is_null()) { dev_err!(&(*pdev).dev, "Setting up cpt inst info failed"); return -ENOMEM; }
    (*cpt_req).dlen = (*info).dlen;
    result = (*info).completion_addr;
    (*result).s.compcode = OTX2_CPT_COMPLETION_CODE_INIT;

    spin_lock_bh!(&mut (*pqueue).lock);
    pentry = get_free_pending_entry(pqueue, (*pqueue).qlen);
    retry = CPT_PENTRY_TIMEOUT / CPT_PENTRY_STEP;
    while unlikely!(pentry.is_null()) && retry != 0 {
        spin_unlock_bh!(&mut (*pqueue).lock); udelay!(CPT_PENTRY_STEP); spin_lock_bh!(&mut (*pqueue).lock);
        pentry = get_free_pending_entry(pqueue, (*pqueue).qlen); retry -= 1;
    }
    if unlikely!(pentry.is_null()) { ret = -ENOSPC; spin_unlock_bh!(&mut (*pqueue).lock); otx2_cpt_info_destroy(pdev, info); return ret; }
    (*pentry).resume_sender = gfp == GFP_KERNEL && (*pqueue).pending_count > (*pqueue).qlen - CPT_IQ_STOP_MARGIN as i32;
    resume_sender = (*pentry).resume_sender; (*pqueue).pending_count += 1;
    (*pentry).completion_addr = (*info).completion_addr; (*pentry).info = info; (*pentry).callback = (*req).callback; (*pentry).areq = (*req).areq; (*pentry).busy = true;
    (*info).pentry = pentry; (*info).time_in = jiffies; (*info).req = req;

    iq_cmd.cmd.u = 0;
    iq_cmd.cmd.s.opcode = cpu_to_be16!((*cpt_req).opcode.flags); iq_cmd.cmd.s.param1 = cpu_to_be16!((*cpt_req).param1); iq_cmd.cmd.s.param2 = cpu_to_be16!((*cpt_req).param2); iq_cmd.cmd.s.dlen = cpu_to_be16!((*cpt_req).dlen);
    cpu_to_be64s!(&mut iq_cmd.cmd.u); iq_cmd.dptr = (*info).dptr_baddr | ((*info).gthr_sz as u64) << 60; iq_cmd.rptr = (*info).rptr_baddr | ((*info).sctr_sz as u64) << 60; iq_cmd.cptr.s.cptr = (*cpt_req).cptr_dma; iq_cmd.cptr.s.grp = (*ctrl).s.grp;
    otx2_cpt_fill_inst(&mut cptinst, &mut iq_cmd, (*info).comp_baddr);
    otx2_cpt_dump_sg_list(pdev, req); pr_debug!("Cpt_inst_s hexdump (%d bytes)\n", OTX2_CPT_INST_SIZE); print_hex_dump_debug!("", 0, 16, 1, &cptinst, OTX2_CPT_INST_SIZE, false); pr_debug!("Dptr hexdump (%d bytes)\n", (*cpt_req).dlen); print_hex_dump_debug!("", 0, 16, 1, (*info).in_buffer, (*cpt_req).dlen, false);
    ((*lf).lfs.ops.send_cmd)(&mut cptinst, 1, lf); spin_unlock_bh!(&mut (*pqueue).lock);
    ret = if resume_sender { -EBUSY } else { -EINPROGRESS }; ret
}

pub unsafe fn otx2_cpt_do_request(pdev: *mut pci_dev, req: *mut otx2_cpt_req_info, cpu_num: i32) -> i32 {
    let cptvf = pci_get_drvdata(pdev) as *mut otx2_cptvf_dev;
    let lfs = &mut (*cptvf).lfs;
    process_request(lfs.pdev, req, &mut lfs.lf[cpu_num as usize].pqueue, &mut lfs.lf[cpu_num as usize])
}

/* Completion processing and engine-group selection retain the source interfaces. */
unsafe fn cpt_process_ccode(lfs: *mut otx2_cptlfs_info, cpt_status: *mut otx2_cpt_res_s, info: *mut otx2_cpt_inst_info, res_code: *mut u32) -> i32 {
    let uc_ccode = ((*lfs).ops.cpt_get_uc_compcode)(cpt_status); let ccode = ((*lfs).ops.cpt_get_compcode)(cpt_status); let pdev = (*lfs).pdev;
    match ccode {
        OTX2_CPT_COMP_E_FAULT | OTX2_CPT_COMP_E_HWERR | OTX2_CPT_COMP_E_INSTERR => { otx2_cpt_dump_sg_list(pdev, (*info).req); }
        OTX2_CPT_COMP_E_NOTDONE => { if time_after_eq!(jiffies, (*info).time_in + CPT_COMMAND_TIMEOUT * HZ) { dev_warn!(&(*pdev).dev, "Request timed out 0x%p", (*info).req); } else if (*info).extra_time < CPT_TIME_IN_RESET_COUNT { (*info).time_in = jiffies; (*info).extra_time += 1; } return 1; }
        OTX2_CPT_COMP_E_GOOD | OTX2_CPT_COMP_E_WARN => { if uc_ccode == OTX2_CPT_UCC_SUCCESS { *res_code = 0; } else if (*info).req.is_trunc_hmac && uc_ccode == OTX2_CPT_UCC_SG_WRITE_LENGTH { *res_code = 0; } else { otx2_cpt_dump_sg_list(pdev, (*info).req); } }
        _ => { dev_err!(&(*pdev).dev, "Request returned invalid status %d\n", ccode); }
    } 0
}

unsafe fn process_pending_queue(lfs: *mut otx2_cptlfs_info, pqueue: *mut otx2_cpt_pending_queue) {
    loop {
        spin_lock_bh!(&mut (*pqueue).lock); let pentry = &mut (*pqueue).head[(*pqueue).front as usize] as *mut _; if WARN_ON!(pentry.is_null()) { spin_unlock_bh!(&mut (*pqueue).lock); break; }
        let mut res_code: u32 = -EINVAL as u32; if unlikely!(!(*pentry).busy) { spin_unlock_bh!(&mut (*pqueue).lock); break; }
        let info = (*pentry).info; let req = if !info.is_null() { (*info).req } else { core::ptr::null_mut() }; let status = (*pentry).completion_addr;
        if !info.is_null() && !req.is_null() && !status.is_null() && cpt_process_ccode(lfs, status, info, &mut res_code) != 0 { spin_unlock_bh!(&mut (*pqueue).lock); return; }
        let resume_index = modulo_inc((*pqueue).front, (*pqueue).qlen, CPT_IQ_RESUME_MARGIN); let resume = &mut (*pqueue).head[resume_index as usize] as *mut _;
        if (*resume).resume_sender { (*resume).resume_sender = false; if let Some(callback) = (*resume).callback { let areq = (*resume).areq; spin_unlock_bh!(&mut (*pqueue).lock); callback(-EINPROGRESS, areq, info); spin_lock_bh!(&mut (*pqueue).lock); } }
        let callback = (*pentry).callback; let areq = (*pentry).areq; free_pentry(pentry); (*pqueue).pending_count -= 1; (*pqueue).front = modulo_inc((*pqueue).front, (*pqueue).qlen, 1); spin_unlock_bh!(&mut (*pqueue).lock);
        if let Some(callback) = callback { callback(res_code as i32, areq, info); }
    }
}

pub unsafe fn otx2_cpt_post_process(wqe: *mut otx2_cptlf_wqe) { process_pending_queue((*wqe).lfs, &mut (*(*wqe).lfs).lf[(*wqe).lf_num as usize].pqueue); }

pub unsafe fn otx2_cpt_get_eng_grp_num(pdev: *mut pci_dev, eng_type: otx2_cpt_eng_type) -> i32 {
    let cptvf = pci_get_drvdata(pdev) as *mut otx2_cptvf_dev;
    match eng_type { OTX2_CPT_SE_TYPES => (*cptvf).lfs.kcrypto_se_eng_grp_num, OTX2_CPT_AE_TYPES => (*cptvf).lfs.kcrypto_ae_eng_grp_num, _ => { dev_err!(&(*(*cptvf).pdev).dev, "Unsupported engine type"); -ENXIO } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
