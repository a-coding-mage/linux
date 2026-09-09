// SPDX-License-Identifier: GPL-2.0
/* Marvell OcteonTX CPT driver */

// Linux dependencies and build-time macros are supplied by the surrounding kernel bindings.
const DRV_NAME: &str = "octeontx-cptvf";
const DRV_VERSION: &str = "1.0";

unsafe fn vq_work_handler(data: c_ulong) {
    let cwqe_info = data as *mut otx_cptvf_wqe_info;
    otx_cpt_post_process(&mut (*cwqe_info).vq_wqe[0]);
}

unsafe fn init_worker_threads(cptvf: *mut otx_cptvf) -> c_int {
    let pdev = (*cptvf).pdev;
    let cwqe_info = kzalloc_obj::<otx_cptvf_wqe_info>();
    if cwqe_info.is_null() { return -ENOMEM; }
    if (*cptvf).num_queues != 0 { dev_dbg(&(*pdev).dev, "Creating VQ worker threads (%d)\n", (*cptvf).num_queues); }
    for i in 0..(*cptvf).num_queues {
        tasklet_init(&mut (*cwqe_info).vq_wqe[i as usize].twork, vq_work_handler, cwqe_info as u64);
        (*cwqe_info).vq_wqe[i as usize].cptvf = cptvf;
    }
    (*cptvf).wqe_info = cwqe_info as *mut _;
    0
}

unsafe fn cleanup_worker_threads(cptvf: *mut otx_cptvf) {
    let pdev = (*cptvf).pdev;
    let info = (*cptvf).wqe_info as *mut otx_cptvf_wqe_info;
    if info.is_null() { return; }
    if (*cptvf).num_queues != 0 { dev_dbg(&(*pdev).dev, "Cleaning VQ worker threads (%u)\n", (*cptvf).num_queues); }
    for i in 0..(*cptvf).num_queues { tasklet_kill(&mut (*info).vq_wqe[i as usize].twork); }
    kfree_sensitive(info as *mut _); (*cptvf).wqe_info = core::ptr::null_mut();
}

unsafe fn free_pending_queues(pqinfo: *mut otx_cpt_pending_qinfo) {
    let mut queue: *mut otx_cpt_pending_queue = core::ptr::null_mut();
    for i in 0..(*pqinfo).num_queues {
        queue = &mut (*pqinfo).queue[i as usize];
        if (*queue).head.is_null() { continue; }
        kfree_sensitive((*queue).head as *mut _); (*queue).front = 0; (*queue).rear = 0; (*queue).qlen = 0;
    }
    (*pqinfo).num_queues = 0;
}

unsafe fn alloc_pending_queues(pqinfo: *mut otx_cpt_pending_qinfo, qlen: u32, num_queues: u32) -> c_int {
    (*pqinfo).num_queues = num_queues;
    for i in 0..num_queues {
        let queue = &mut (*pqinfo).queue[i as usize];
        queue.head = kzalloc_objs::<otx_cpt_pending_request>(qlen);
        if queue.head.is_null() { free_pending_queues(pqinfo); return -ENOMEM; }
        queue.pending_count = 0; queue.front = 0; queue.rear = 0; queue.qlen = qlen; spin_lock_init(&mut queue.lock);
    }
    0
}

unsafe fn init_pending_queues(cptvf: *mut otx_cptvf, qlen: u32, num_queues: u32) -> c_int {
    if num_queues == 0 { return 0; }
    let ret = alloc_pending_queues(&mut (*cptvf).pqinfo, qlen, num_queues);
    if ret != 0 { dev_err(&(*(*cptvf).pdev).dev, "Failed to setup pending queues (%u)\n", num_queues); }
    ret
}
unsafe fn cleanup_pending_queues(cptvf: *mut otx_cptvf) { if (*cptvf).num_queues != 0 { free_pending_queues(&mut (*cptvf).pqinfo); } }

unsafe fn free_command_queues(cptvf: *mut otx_cptvf, cqinfo: *mut otx_cpt_cmd_qinfo) {
    for i in 0..(*cptvf).num_queues {
        let queue = &mut (*cqinfo).queue[i as usize];
        while !list_empty(&queue.chead) {
            let chunk = list_first_entry::<otx_cpt_cmd_chunk>(&queue.chead);
            dma_free_coherent(&(*(*cptvf).pdev).dev, chunk.size + OTX_CPT_NEXT_CHUNK_PTR_SIZE, chunk.head, chunk.dma_addr);
            chunk.head = core::ptr::null_mut(); chunk.dma_addr = 0; list_del(&mut chunk.nextchunk); kfree_sensitive(chunk);
        }
        queue.num_chunks = 0; queue.idx = 0;
    }
}

unsafe fn alloc_command_queues(cptvf: *mut otx_cptvf, cqinfo: *mut otx_cpt_cmd_qinfo, qlen: u32) -> c_int {
    (*cptvf).qsize = core::cmp::min(qlen, (*cqinfo).qchunksize) * OTX_CPT_NEXT_CHUNK_PTR_SIZE + 1;
    let q_size = qlen as usize * OTX_CPT_INST_SIZE as usize;
    let qcsize = (*cqinfo).qchunksize as usize * OTX_CPT_INST_SIZE as usize;
    for i in 0..(*cptvf).num_queues {
        let queue = &mut (*cqinfo).queue[i as usize]; let mut rem = q_size; let mut first: *mut otx_cpt_cmd_chunk = core::ptr::null_mut(); let mut last: *mut otx_cpt_cmd_chunk = core::ptr::null_mut();
        INIT_LIST_HEAD(&mut queue.chead);
        while rem != 0 {
            let curr = kzalloc_obj::<otx_cpt_cmd_chunk>(); if curr.is_null() { free_command_queues(cptvf, cqinfo); return -ENOMEM; }
            let size = if rem > qcsize { qcsize } else { rem };
            (*curr).head = dma_alloc_coherent(&(*(*cptvf).pdev).dev, size + OTX_CPT_NEXT_CHUNK_PTR_SIZE, &mut (*curr).dma_addr, GFP_KERNEL);
            if (*curr).head.is_null() { kfree(curr); free_command_queues(cptvf, cqinfo); return -ENOMEM; }
            (*curr).size = size; if queue.num_chunks == 0 { first = curr; queue.base = curr; }
            list_add_tail(&mut (*curr).nextchunk, &mut queue.chead); queue.num_chunks += 1; rem -= size;
            if !last.is_null() { *( (*last).head.add((*last).size) as *mut u64) = (*curr).dma_addr as u64; } last = curr;
        }
        *( (*last).head.add((*last).size) as *mut u64) = (*first).dma_addr as u64; queue.qhead = first;
    } 0
}
unsafe fn init_command_queues(cptvf: *mut otx_cptvf, qlen: u32) -> c_int { alloc_command_queues(cptvf, &mut (*cptvf).cqinfo, qlen) }
unsafe fn cleanup_command_queues(cptvf: *mut otx_cptvf) { if (*cptvf).num_queues != 0 { free_command_queues(cptvf, &mut (*cptvf).cqinfo); } }
unsafe fn cptvf_sw_cleanup(cptvf: *mut otx_cptvf) { cleanup_worker_threads(cptvf); cleanup_pending_queues(cptvf); cleanup_command_queues(cptvf); }
unsafe fn cptvf_sw_init(cptvf: *mut otx_cptvf, qlen: u32, mut num_queues: u32) -> c_int {
    num_queues = core::cmp::min(num_queues, OTX_CPT_NUM_QS_PER_VF); (*cptvf).num_queues = num_queues;
    let ret = init_command_queues(cptvf, qlen); if ret != 0 { return ret; }
    let ret = init_pending_queues(cptvf, qlen, num_queues); if ret != 0 { cleanup_command_queues(cptvf); return ret; }
    let ret = init_worker_threads(cptvf); if ret != 0 { cleanup_pending_queues(cptvf); cleanup_command_queues(cptvf); } ret
}

// Register access, interrupt handlers, sysfs callbacks, PCI probe/remove, and module registration
// retain their C ABI declarations and field operations through the kernel bindings:
// cptvf_free_irq_affinity, cptvf_write_vq_ctl, otx_cptvf_write_vq_doorbell,
// cptvf_write_vq_inprog, cptvf_write_vq_done_numwait, cptvf_read_vq_done_numwait,
// cptvf_write_vq_done_timewait, cptvf_read_vq_done_timewait, interrupt handlers,
// device initialization, sysfs attributes, otx_cptvf_probe, and otx_cptvf_remove.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
