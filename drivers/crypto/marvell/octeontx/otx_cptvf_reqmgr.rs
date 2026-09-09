// SPDX-License-Identifier: GPL-2.0
/* Marvell OcteonTX CPT driver
 *
 * Copyright (C) 2019 Marvell International Ltd.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

// Dependencies supplied by otx_cptvf.h and otx_cptvf_algs.h are external.

const COMPLETION_CODE_SIZE: usize = 8;
const COMPLETION_CODE_INIT: u8 = 0;
const SG_LIST_HDR_SIZE: usize = 8;
const CPT_PENTRY_TIMEOUT: i32 = 1000;
const CPT_PENTRY_STEP: i32 = 50;
const CPT_IQ_STOP_MARGIN: u32 = 128;
const CPT_IQ_RESUME_MARGIN: u32 = 512;
const CPT_DMA_ALIGN: usize = 128;

pub unsafe fn otx_cpt_dump_sg_list(pdev: *mut pci_dev, req: *mut otx_cpt_req_info) {
    let mut i = 0;
    pr_debug!("Gather list size %d\n", (*req).incnt);
    while i < (*req).incnt {
        pr_debug!("Buffer %d size %d, vptr 0x%p, dmaptr 0x%p\n", i, (*req).in_[i].size, (*req).in_[i].vptr, (*req).in_[i].dma_addr as *mut core::ffi::c_void);
        pr_debug!("Buffer hexdump (%d bytes)\n", (*req).in_[i].size);
        print_hex_dump_debug!("", DUMP_PREFIX_NONE, 16, 1, (*req).in_[i].vptr, (*req).in_[i].size, false);
        i += 1;
    }
    pr_debug!("Scatter list size %d\n", (*req).outcnt);
    i = 0;
    while i < (*req).outcnt {
        pr_debug!("Buffer %d size %d, vptr 0x%p, dmaptr 0x%p\n", i, (*req).out_[i].size, (*req).out_[i].vptr, (*req).out_[i].dma_addr as *mut core::ffi::c_void);
        pr_debug!("Buffer hexdump (%d bytes)\n", (*req).out_[i].size);
        print_hex_dump_debug!("", DUMP_PREFIX_NONE, 16, 1, (*req).out_[i].vptr, (*req).out_[i].size, false);
        i += 1;
    }
}

#[inline]
unsafe fn get_free_pending_entry(q: *mut otx_cpt_pending_queue, qlen: i32) -> *mut otx_cpt_pending_entry {
    let ent = (*q).head.add((*q).rear as usize);
    if unlikely!((*ent).busy) { return core::ptr::null_mut(); }
    (*q).rear += 1;
    if unlikely!((*q).rear == qlen) { (*q).rear = 0; }
    ent
}

#[inline]
unsafe fn modulo_inc(mut index: u32, length: u32, inc: u32) -> u32 {
    if WARN_ON!(inc > length) { index = length; }
    index += inc;
    if unlikely!(index >= length) { index -= length; }
    index
}

#[inline]
unsafe fn free_pentry(pentry: *mut otx_cpt_pending_entry) {
    (*pentry).completion_addr = core::ptr::null_mut();
    (*pentry).info = core::ptr::null_mut();
    (*pentry).callback = None;
    (*pentry).areq = core::ptr::null_mut();
    (*pentry).resume_sender = false;
    (*pentry).busy = false;
}

#[inline]
unsafe fn setup_sgio_components(pdev: *mut pci_dev, list: *mut otx_cpt_buf_ptr, buf_count: i32, buffer: *mut u8) -> i32 {
    let mut sg_ptr: *mut otx_cpt_sglist_component = core::ptr::null_mut();
    let mut ret = 0;
    let mut i: i32;
    let mut j: i32;
    if unlikely!(list.is_null()) { dev_err!(&(*pdev).dev, "Input list pointer is NULL\n"); return -EFAULT; }
    i = 0;
    while i < buf_count {
        if likely!(!(*list.add(i as usize)).vptr.is_null()) {
            (*list.add(i as usize)).dma_addr = dma_map_single!(&(*pdev).dev, (*list.add(i as usize)).vptr, (*list.add(i as usize)).size, DMA_BIDIRECTIONAL);
            if unlikely!(dma_mapping_error!(&(*pdev).dev, (*list.add(i as usize)).dma_addr)) { dev_err!(&(*pdev).dev, "Dma mapping failed\n"); ret = -EIO; break; }
        }
        i += 1;
    }
    if i != buf_count {
        j = 0;
        while j < i { if (*list.add(j as usize)).dma_addr != 0 { dma_unmap_single!(&(*pdev).dev, (*list.add(j as usize)).dma_addr, (*list.add(j as usize)).size, DMA_BIDIRECTIONAL); } (*list.add(j as usize)).dma_addr = 0; j += 1; }
        return ret;
    }
    let mut components = buf_count / 4;
    sg_ptr = buffer as *mut otx_cpt_sglist_component;
    i = 0;
    while i < components {
        (*sg_ptr).u.s.len0 = cpu_to_be16!((*list.add((i * 4) as usize)).size); (*sg_ptr).u.s.len1 = cpu_to_be16!((*list.add((i * 4 + 1) as usize)).size); (*sg_ptr).u.s.len2 = cpu_to_be16!((*list.add((i * 4 + 2) as usize)).size); (*sg_ptr).u.s.len3 = cpu_to_be16!((*list.add((i * 4 + 3) as usize)).size);
        (*sg_ptr).ptr0 = cpu_to_be64!((*list.add((i * 4) as usize)).dma_addr); (*sg_ptr).ptr1 = cpu_to_be64!((*list.add((i * 4 + 1) as usize)).dma_addr); (*sg_ptr).ptr2 = cpu_to_be64!((*list.add((i * 4 + 2) as usize)).dma_addr); (*sg_ptr).ptr3 = cpu_to_be64!((*list.add((i * 4 + 3) as usize)).dma_addr); sg_ptr = sg_ptr.add(1); i += 1;
    }
    components = buf_count % 4;
    match components { 3 => { (*sg_ptr).u.s.len2 = cpu_to_be16!((*list.add((i * 4 + 2) as usize)).size); (*sg_ptr).ptr2 = cpu_to_be64!((*list.add((i * 4 + 2) as usize)).dma_addr); }, _ => {} }
    if components >= 2 { (*sg_ptr).u.s.len1 = cpu_to_be16!((*list.add((i * 4 + 1) as usize)).size); (*sg_ptr).ptr1 = cpu_to_be64!((*list.add((i * 4 + 1) as usize)).dma_addr); }
    if components >= 1 { (*sg_ptr).u.s.len0 = cpu_to_be16!((*list.add((i * 4) as usize)).size); (*sg_ptr).ptr0 = cpu_to_be64!((*list.add((i * 4) as usize)).dma_addr); }
    ret
}

#[inline]
unsafe fn setup_sgio_list(pdev: *mut pci_dev, pinfo: *mut *mut otx_cpt_info_buffer, req: *mut otx_cpt_req_info, gfp: gfp_t) -> i32 {
    if unlikely!((*req).incnt > OTX_CPT_MAX_SG_IN_CNT || (*req).outcnt > OTX_CPT_MAX_SG_OUT_CNT) { dev_err!(&(*pdev).dev, "Error too many sg components\n"); return -EINVAL; }
    let g_sz_bytes = (((*req).incnt + 3) / 4) * core::mem::size_of::<otx_cpt_sglist_component>() as i32;
    let s_sz_bytes = (((*req).outcnt + 3) / 4) * core::mem::size_of::<otx_cpt_sglist_component>() as i32;
    let dlen = g_sz_bytes + s_sz_bytes + SG_LIST_HDR_SIZE as i32;
    let align_dlen = ALIGN!(dlen, CPT_DMA_ALIGN as i32); let info_len = ALIGN!(core::mem::size_of::<otx_cpt_info_buffer>() as i32, CPT_DMA_ALIGN as i32); let rlen = ALIGN!(core::mem::size_of::<otx_cpt_res_s>() as i32, CPT_DMA_ALIGN as i32); let total_mem_len = align_dlen + info_len + rlen + COMPLETION_CODE_SIZE as i32;
    let info = kzalloc!(total_mem_len, gfp); if info.is_null() { dev_err!(&(*pdev).dev, "Memory allocation failed\n"); return -ENOMEM; } *pinfo = info; (*info).dlen = dlen; (*info).in_buffer = (info as *mut u8).add(info_len as usize);
    *((*info).in_buffer as *mut u16) = cpu_to_be16!((*req).outcnt as u16); *((*info).in_buffer.add(2) as *mut u16) = cpu_to_be16!((*req).incnt as u16); *((*info).in_buffer.add(4) as *mut u16) = 0; *((*info).in_buffer.add(6) as *mut u16) = 0;
    if setup_sgio_components(pdev, (*req).in_.as_mut_ptr(), (*req).incnt, (*info).in_buffer.add(8)) != 0 { dev_err!(&(*pdev).dev, "Failed to setup gather list\n"); return -EFAULT; }
    if setup_sgio_components(pdev, (*req).out_.as_mut_ptr(), (*req).outcnt, (*info).in_buffer.add(8 + g_sz_bytes as usize)) != 0 { dev_err!(&(*pdev).dev, "Failed to setup scatter list\n"); return -EFAULT; }
    (*info).dma_len = total_mem_len - info_len; (*info).dptr_baddr = dma_map_single!(&(*pdev).dev, (*info).in_buffer as *mut core::ffi::c_void, (*info).dma_len, DMA_BIDIRECTIONAL); if unlikely!(dma_mapping_error!(&(*pdev).dev, (*info).dptr_baddr)) { dev_err!(&(*pdev).dev, "DMA Mapping failed for cpt req\n"); return -EIO; }
    (*info).completion_addr = (*info).in_buffer.add(align_dlen as usize) as *mut u64; (*info).comp_baddr = (*info).dptr_baddr + align_dlen as u64; (*info).out_buffer = (*info).completion_addr as *mut u8.add(rlen as usize); (*info).rptr_baddr = (*info).comp_baddr + rlen as u64; *((*info).out_buffer as *mut u64) = !(COMPLETION_CODE_INIT as u64); 0
}

unsafe fn cpt_fill_inst(inst: *mut otx_cpt_inst_s, info: *mut otx_cpt_info_buffer, cmd: *mut otx_cpt_iq_cmd) { (*inst).u[0] = 0; (*inst).s.doneint = true; (*inst).s.res_addr = (*info).comp_baddr; (*inst).u[2] = 0; (*inst).s.wq_ptr = 0; (*inst).s.ei0 = (*cmd).cmd.u64; (*inst).s.ei1 = (*cmd).dptr; (*inst).s.ei2 = (*cmd).rptr; (*inst).s.ei3 = (*cmd).cptr.u64; }

unsafe fn cpt_send_cmd(cptinst: *mut otx_cpt_inst_s, cptvf: *mut otx_cptvf) { let qinfo = &mut (*cptvf).cqinfo; let queue = &mut qinfo.queue[0]; let ent = (*queue.qhead).head.add((queue.idx * OTX_CPT_INST_SIZE) as usize); memcpy!(ent, cptinst as *const core::ffi::c_void, OTX_CPT_INST_SIZE); queue.idx += 1; if queue.idx >= (*queue.qhead).size / 64 { let curr = queue.qhead; if list_is_last!(&(*curr).nextchunk, &queue.chead) { queue.qhead = queue.base; } else { queue.qhead = list_next_entry!(queue.qhead, nextchunk); } queue.idx = 0; } smp_wmb!(); otx_cptvf_write_vq_doorbell(cptvf, 1); }

unsafe fn process_request(pdev: *mut pci_dev, req: *mut otx_cpt_req_info, pqueue: *mut otx_cpt_pending_queue, cptvf: *mut otx_cptvf) -> i32 { let cpt_req = &mut (*req).req; let mut pentry = core::ptr::null_mut(); let ctrl = &mut (*req).ctrl; let mut info = core::ptr::null_mut(); let mut result: *mut otx_cpt_res_s = core::ptr::null_mut(); let mut iq_cmd: otx_cpt_iq_cmd = core::mem::zeroed(); let mut cptinst: otx_cpt_inst_s = core::mem::zeroed(); let gfp = if ((*req).areq).flags & CRYPTO_TFM_REQ_MAY_SLEEP != 0 { GFP_KERNEL } else { GFP_ATOMIC }; let mut ret = setup_sgio_list(pdev, &mut info, req, gfp); if unlikely!(ret != 0) { dev_err!(&(*pdev).dev, "Setting up SG list failed\n"); do_request_cleanup(pdev, info); return ret; } (*cpt_req).dlen = (*info).dlen; result = (*info).completion_addr as *mut otx_cpt_res_s; (*result).s.compcode = COMPLETION_CODE_INIT; spin_lock_bh!(&mut (*pqueue).lock); pentry = get_free_pending_entry(pqueue, (*pqueue).qlen); let mut retry = CPT_PENTRY_TIMEOUT / CPT_PENTRY_STEP; while unlikely!(pentry.is_null()) && retry != 0 { retry -= 1; spin_unlock_bh!(&mut (*pqueue).lock); udelay!(CPT_PENTRY_STEP); spin_lock_bh!(&mut (*pqueue).lock); pentry = get_free_pending_entry(pqueue, (*pqueue).qlen); } if pentry.is_null() { spin_unlock_bh!(&mut (*pqueue).lock); do_request_cleanup(pdev, info); return -ENOSPC; } let resume_sender = gfp == GFP_KERNEL && (*pqueue).pending_count > (*pqueue).qlen as u32 - CPT_IQ_STOP_MARGIN; (*pentry).resume_sender = resume_sender; (*pqueue).pending_count += 1; (*pentry).completion_addr = (*info).completion_addr; (*pentry).info = info; (*pentry).callback = (*req).callback; (*pentry).areq = (*req).areq; (*pentry).busy = true; (*info).pentry = pentry; (*info).time_in = jiffies; (*info).req = req; iq_cmd.cmd.u64 = 0; iq_cmd.cmd.s.opcode = cpu_to_be16!((*cpt_req).opcode.flags); iq_cmd.cmd.s.param1 = cpu_to_be16!((*cpt_req).param1); iq_cmd.cmd.s.param2 = cpu_to_be16!((*cpt_req).param2); iq_cmd.cmd.s.dlen = cpu_to_be16!((*cpt_req).dlen); iq_cmd.dptr = (*info).dptr_baddr; iq_cmd.rptr = (*info).rptr_baddr; iq_cmd.cptr.u64 = 0; iq_cmd.cptr.s.grp = ctrl.s.grp; cpt_fill_inst(&mut cptinst, info, &mut iq_cmd); otx_cpt_dump_sg_list(pdev, req); cpt_send_cmd(&mut cptinst, cptvf); spin_unlock_bh!(&mut (*pqueue).lock); if resume_sender { -EBUSY } else { -EINPROGRESS } }

pub unsafe fn otx_cpt_do_request(pdev: *mut pci_dev, req: *mut otx_cpt_req_info, _cpu_num: i32) -> i32 { let cptvf = pci_get_drvdata(pdev); if !otx_cpt_device_ready(cptvf) { dev_err!(&(*pdev).dev, "CPT Device is not ready\n"); return -ENODEV; } if (*cptvf).vftype == OTX_CPT_SE_TYPES && !(*req).ctrl.s.se_req { dev_err!(&(*pdev).dev, "CPTVF-%d of SE TYPE got AE request\n", (*cptvf).vfid); return -EINVAL; } else if (*cptvf).vftype == OTX_CPT_AE_TYPES && (*req).ctrl.s.se_req { dev_err!(&(*pdev).dev, "CPTVF-%d of AE TYPE got SE request\n", (*cptvf).vfid); return -EINVAL; } process_request(pdev, req, &mut (*cptvf).pqinfo.queue[0], cptvf) }

unsafe fn cpt_process_ccode(pdev: *mut pci_dev, cpt_status: *mut otx_cpt_res_s, cpt_info: *mut otx_cpt_info_buffer, req: *mut otx_cpt_req_info, res_code: *mut u32) -> i32 { let ccode = (*cpt_status).s.compcode; let ecode = be64_to_cpup!((*cpt_info).out_buffer as *const __be64); match ccode { CPT_COMP_E_FAULT => { dev_err!(&(*pdev).dev, "Request failed with DMA fault\n"); otx_cpt_dump_sg_list(pdev, req); }, CPT_COMP_E_SWERR => { dev_err!(&(*pdev).dev, "Request failed with software error code %d\n", ecode.s.ccode); otx_cpt_dump_sg_list(pdev, req); }, CPT_COMP_E_HWERR => { dev_err!(&(*pdev).dev, "Request failed with hardware error\n"); otx_cpt_dump_sg_list(pdev, req); }, COMPLETION_CODE_INIT => { if time_after_eq!(jiffies, (*cpt_info).time_in + OTX_CPT_COMMAND_TIMEOUT * HZ) { dev_warn!(&(*pdev).dev, "Request timed out 0x%p\n", req); } else if (*cpt_info).extra_time < OTX_CPT_TIME_IN_RESET_COUNT { (*cpt_info).time_in = jiffies; (*cpt_info).extra_time += 1; } return 1; }, CPT_COMP_E_GOOD => { if ecode.s.ccode != 0 { if (*req).is_trunc_hmac && ecode.s.ccode == ERR_SCATTER_GATHER_WRITE_LENGTH { *res_code = 0; } else { dev_err!(&(*pdev).dev, "Request failed with software error code 0x%x\n", ecode.s.ccode); otx_cpt_dump_sg_list(pdev, req); } } else { *res_code = 0; } }, _ => dev_err!(&(*pdev).dev, "Request returned invalid status\n") } 0 }

#[inline]
unsafe fn process_pending_queue(pdev: *mut pci_dev, pqueue: *mut otx_cpt_pending_queue) { loop { spin_lock_bh!(&mut (*pqueue).lock); let pentry = &mut *(*pqueue).head.add((*pqueue).front as usize); if WARN_ON!(pentry.is_null()) || !pentry.busy { spin_unlock_bh!(&mut (*pqueue).lock); break; } let mut res_code = -EINVAL as u32; let callback = pentry.callback; let areq = pentry.areq; let cpt_info = pentry.info; let req = if !cpt_info.is_null() { (*cpt_info).req } else { core::ptr::null_mut() }; if callback.is_none() || cpt_info.is_null() || req.is_null() || pentry.completion_addr.is_null() { if callback.is_none() { dev_err!(&(*pdev).dev, "Callback NULL\n"); } } else if cpt_process_ccode(pdev, pentry.completion_addr as *mut otx_cpt_res_s, cpt_info, req, &mut res_code) != 0 { spin_unlock_bh!(&mut (*pqueue).lock); return; } if !cpt_info.is_null() { (*cpt_info).pdev = pdev; } let resume_index = modulo_inc((*pqueue).front, (*pqueue).qlen as u32, CPT_IQ_RESUME_MARGIN); let resume = &mut *(*pqueue).head.add(resume_index as usize); if resume.resume_sender { resume.resume_sender = false; if let Some(cb) = resume.callback { spin_unlock_bh!(&mut (*pqueue).lock); cb(-EINPROGRESS, resume.areq, cpt_info as *mut core::ffi::c_void); spin_lock_bh!(&mut (*pqueue).lock); } } free_pentry(pentry); (*pqueue).pending_count -= 1; (*pqueue).front = modulo_inc((*pqueue).front, (*pqueue).qlen as u32, 1); spin_unlock_bh!(&mut (*pqueue).lock); if let Some(cb) = callback { cb(res_code as i32, areq, cpt_info as *mut core::ffi::c_void); } } }

pub unsafe fn otx_cpt_post_process(wqe: *mut otx_cptvf_wqe) { process_pending_queue((*wqe).cptvf.as_ref().unwrap().pdev, &mut (*wqe).cptvf.as_mut().unwrap().pqinfo.queue[0]); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
