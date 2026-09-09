// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2016 Cavium, Inc. */
// Translated from cptvf_main.c. Linux/CPT declarations are supplied externally.

const DRV_NAME: &str = "thunder-cptvf";
const DRV_VERSION: &str = "1.0";

#[repr(C)]
struct CptvfWqe { twork: tasklet_struct, cptvf: *mut core::ffi::c_void, qno: u32 }
#[repr(C)]
struct CptvfWqeInfo { vq_wqe: [CptvfWqe; CPT_NUM_QS_PER_VF as usize] }

unsafe fn vq_work_handler(data: usize) {
    let info = &mut *(data as *mut CptvfWqeInfo);
    let wqe = &mut info.vq_wqe[0];
    vq_post_process(wqe.cptvf, wqe.qno);
}

unsafe fn init_worker_threads(cptvf: *mut cpt_vf) -> i32 {
    let pdev = (*cptvf).pdev;
    let info = kzalloc_obj::<CptvfWqeInfo>();
    if info.is_null() { return -ENOMEM; }
    if (*cptvf).nr_queues != 0 { dev_info(&(*pdev).dev, "Creating VQ worker threads (%d)\n", (*cptvf).nr_queues); }
    for i in 0..(*cptvf).nr_queues {
        tasklet_init(&mut (*info).vq_wqe[i as usize].twork, vq_work_handler, info as usize as u64);
        (*info).vq_wqe[i as usize].qno = i;
        (*info).vq_wqe[i as usize].cptvf = cptvf as *mut _;
    }
    (*cptvf).wqe_info = info as *mut _;
    0
}

unsafe fn cleanup_worker_threads(cptvf: *mut cpt_vf) {
    let info = (*cptvf).wqe_info as *mut CptvfWqeInfo;
    if info.is_null() { return; }
    if (*cptvf).nr_queues != 0 { dev_info(&(*(*cptvf).pdev).dev, "Cleaning VQ worker threads (%u)\n", (*cptvf).nr_queues); }
    for i in 0..(*cptvf).nr_queues { tasklet_kill(&mut (*info).vq_wqe[i as usize].twork); }
    kfree_sensitive(info as *mut _); (*cptvf).wqe_info = core::ptr::null_mut();
}

unsafe fn free_pending_queues(pqinfo: *mut pending_qinfo) {
    let mut queue: *mut pending_queue = core::ptr::null_mut(); let mut i = 0;
    for_each_pending_queue!(pqinfo, queue, i) {
        if (*queue).head.is_null() { continue; }
        kfree_sensitive((*queue).head); (*queue).front = 0; (*queue).rear = 0; return;
    }
    (*pqinfo).qlen = 0; (*pqinfo).nr_queues = 0;
}

unsafe fn alloc_pending_queues(pqinfo: *mut pending_qinfo, qlen: u32, nr_queues: u32) -> i32 {
    (*pqinfo).nr_queues = nr_queues; (*pqinfo).qlen = qlen;
    let mut queue: *mut pending_queue = core::ptr::null_mut(); let mut i = 0;
    for_each_pending_queue!(pqinfo, queue, i) {
        (*queue).head = kzalloc_objs::<pending_queue_entry>(qlen);
        if (*queue).head.is_null() { free_pending_queues(pqinfo); return -ENOMEM; }
        (*queue).front = 0; (*queue).rear = 0; atomic64_set(&mut (*queue).pending_count, 0); spin_lock_init(&mut (*queue).lock);
    }
    0
}

unsafe fn init_pending_queues(cptvf: *mut cpt_vf, qlen: u32, nr_queues: u32) -> i32 {
    if nr_queues == 0 { return 0; }
    let ret = alloc_pending_queues(&mut (*cptvf).pqinfo, qlen, nr_queues);
    if ret != 0 { dev_err(&(*(*cptvf).pdev).dev, "failed to setup pending queues (%u)\n", nr_queues); }
    ret
}
unsafe fn cleanup_pending_queues(cptvf: *mut cpt_vf) { if (*cptvf).nr_queues != 0 { dev_info(&(*(*cptvf).pdev).dev, "Cleaning VQ pending queue (%u)\n", (*cptvf).nr_queues); free_pending_queues(&mut (*cptvf).pqinfo); } }

unsafe fn free_command_queues(cptvf: *mut cpt_vf, cqinfo: *mut command_qinfo) {
    for i in 0..(*cptvf).nr_queues { let q = &mut (*cqinfo).queue[i as usize]; if hlist_empty(&q.chead) { continue; }
        let mut chunk: *mut command_chunk = core::ptr::null_mut(); let mut node: *mut hlist_node = core::ptr::null_mut();
        hlist_for_each_entry_safe!(chunk, node, &mut q.chead, nextchunk) { dma_free_coherent(&(*(*cptvf).pdev).dev, (*chunk).size + CPT_NEXT_CHUNK_PTR_SIZE, (*chunk).head, (*chunk).dma_addr); (*chunk).head = core::ptr::null_mut(); (*chunk).dma_addr = 0; hlist_del(&mut (*chunk).nextchunk); kfree_sensitive(chunk); }
        q.nchunks = 0; q.idx = 0;
    } (*cqinfo).cmd_size = 0;
}

unsafe fn alloc_command_queues(cptvf: *mut cpt_vf, cqinfo: *mut command_qinfo, cmd_size: usize, qlen: u32) -> i32 {
    (*cqinfo).cmd_size = cmd_size; (*cptvf).qsize = core::cmp::min(qlen, (*cqinfo).qchunksize) * CPT_NEXT_CHUNK_PTR_SIZE + 1; let q_size = qlen as usize * cmd_size;
    for i in 0..(*cptvf).nr_queues { let q = &mut (*cqinfo).queue[i as usize]; let mut rem = q_size; let qc = (*cqinfo).qchunksize as usize * cmd_size; let mut first: *mut command_chunk = core::ptr::null_mut(); let mut last: *mut command_chunk = core::ptr::null_mut();
        INIT_HLIST_HEAD(&mut q.chead); loop { let curr = kzalloc_obj::<command_chunk>(); if curr.is_null() { free_command_queues(cptvf,cqinfo); return -ENOMEM; } let size = core::cmp::min(rem,qc); (*curr).head = dma_alloc_coherent(&(*(*cptvf).pdev).dev,size+CPT_NEXT_CHUNK_PTR_SIZE,&mut (*curr).dma_addr,GFP_KERNEL); if (*curr).head.is_null() { kfree(curr as *mut _); free_command_queues(cptvf,cqinfo); return -ENOMEM; } (*curr).size=size; if q.nchunks==0 { hlist_add_head(&mut (*curr).nextchunk,&mut q.chead); first=curr; } else { hlist_add_behind(&mut (*curr).nextchunk,&mut (*last).nextchunk); } q.nchunks+=1; rem-=size; if !last.is_null() { *(((*last).head.add((*last).size)) as *mut u64)=(*curr).dma_addr as u64; } last=curr; if rem==0 { let head=first; *(((*last).head.add((*last).size)) as *mut u64)=(*head).dma_addr as u64; q.qhead=head; spin_lock_init(&mut q.lock); break; } }
    } 0
}

unsafe fn init_command_queues(cptvf:*mut cpt_vf, qlen:u32)->i32 { let ret=alloc_command_queues(cptvf,&mut (*cptvf).cqinfo,CPT_INST_SIZE,qlen); if ret!=0 { dev_err(&(*(*cptvf).pdev).dev,"failed to allocate AE command queues (%u)\n",(*cptvf).nr_queues); } ret }
unsafe fn cleanup_command_queues(cptvf:*mut cpt_vf) { if (*cptvf).nr_queues!=0 { dev_info(&(*(*cptvf).pdev).dev,"Cleaning VQ command queue (%u)\n",(*cptvf).nr_queues); free_command_queues(cptvf,&mut (*cptvf).cqinfo); } }
unsafe fn cptvf_sw_cleanup(c:*mut cpt_vf){cleanup_worker_threads(c);cleanup_pending_queues(c);cleanup_command_queues(c);}
unsafe fn cptvf_sw_init(c:*mut cpt_vf,qlen:u32,mut n:u32)->i32 { n=core::cmp::min(n,CPT_NUM_QS_PER_VF);(*c).nr_queues=n;let r=init_command_queues(c,qlen);if r!=0{return r;}let r=init_pending_queues(c,qlen,n);if r!=0{cleanup_command_queues(c);return r;}let r=init_worker_threads(c);if r!=0{cleanup_worker_threads(c);cleanup_pending_queues(c);cleanup_command_queues(c);}r }

unsafe fn cptvf_free_irq_affinity(c:*mut cpt_vf,vec:i32){irq_set_affinity_hint(pci_irq_vector((*c).pdev,vec),core::ptr::null());free_cpumask_var((*c).affinity_mask[vec as usize]);}
unsafe fn cptvf_write_vq_ctl(c:*mut cpt_vf,val:bool){let mut x:cptx_vqx_ctl=core::mem::zeroed();x.u=cpt_read_csr64((*c).reg_base,CPTX_VQX_CTL(0,0));x.s.ena=val as _;cpt_write_csr64((*c).reg_base,CPTX_VQX_CTL(0,0),x.u);}
pub unsafe fn cptvf_write_vq_doorbell(c:*mut cpt_vf,val:u32){let mut x:cptx_vqx_doorbell=core::mem::zeroed();x.u=cpt_read_csr64((*c).reg_base,CPTX_VQX_DOORBELL(0,0));x.s.dbell_cnt=val*8;cpt_write_csr64((*c).reg_base,CPTX_VQX_DOORBELL(0,0),x.u);}
unsafe fn cptvf_write_vq_inprog(c:*mut cpt_vf,val:u8){let mut x:cptx_vqx_inprog=core::mem::zeroed();x.u=cpt_read_csr64((*c).reg_base,CPTX_VQX_INPROG(0,0));x.s.inflight=val;cpt_write_csr64((*c).reg_base,CPTX_VQX_INPROG(0,0),x.u);}
unsafe fn cptvf_write_vq_done_numwait(c:*mut cpt_vf,val:u32){let mut x:cptx_vqx_done_wait=core::mem::zeroed();x.u=cpt_read_csr64((*c).reg_base,CPTX_VQX_DONE_WAIT(0,0));x.s.num_wait=val;cpt_write_csr64((*c).reg_base,CPTX_VQX_DONE_WAIT(0,0),x.u);}
unsafe fn cptvf_write_vq_done_timewait(c:*mut cpt_vf,t:u16){let mut x:cptx_vqx_done_wait=core::mem::zeroed();x.u=cpt_read_csr64((*c).reg_base,CPTX_VQX_DONE_WAIT(0,0));x.s.time_wait=t;cpt_write_csr64((*c).reg_base,CPTX_VQX_DONE_WAIT(0,0),x.u);}

unsafe fn cptvf_write_vq_saddr(c:*mut cpt_vf,v:u64){let mut x:cptx_vqx_saddr=core::mem::zeroed();x.u=v;cpt_write_csr64((*c).reg_base,CPTX_VQX_SADDR(0,0),x.u);}
unsafe fn cptvf_device_init(c:*mut cpt_vf){cptvf_write_vq_ctl(c,false);cptvf_write_vq_doorbell(c,0);cptvf_write_vq_inprog(c,0);cptvf_write_vq_saddr(c,(*c).cqinfo.queue[0].qhead.as_ref().unwrap().dma_addr as u64);cptvf_write_vq_done_timewait(c,CPT_TIMER_THOLD);cptvf_write_vq_done_numwait(c,1);cptvf_write_vq_ctl(c,true);(*c).flags|=CPT_FLAG_DEVICE_READY;}

unsafe fn cptvf_enable_swerr_interrupts(c:*mut cpt_vf){let mut x:cptx_vqx_misc_ena_w1s=core::mem::zeroed();x.u=cpt_read_csr64((*c).reg_base,CPTX_VQX_MISC_ENA_W1S(0,0));x.s.swerr=1;cpt_write_csr64((*c).reg_base,CPTX_VQX_MISC_ENA_W1S(0,0),x.u);}
unsafe fn cptvf_enable_mbox_interrupts(c:*mut cpt_vf){let mut x:cptx_vqx_misc_ena_w1s=core::mem::zeroed();x.u=cpt_read_csr64((*c).reg_base,CPTX_VQX_MISC_ENA_W1S(0,0));x.s.mbox=1;cpt_write_csr64((*c).reg_base,CPTX_VQX_MISC_ENA_W1S(0,0),x.u);}
unsafe fn cptvf_enable_done_interrupts(c:*mut cpt_vf){let mut x:cptx_vqx_done_ena_w1s=core::mem::zeroed();x.u=cpt_read_csr64((*c).reg_base,CPTX_VQX_DONE_ENA_W1S(0,0));x.s.done=1;cpt_write_csr64((*c).reg_base,CPTX_VQX_DONE_ENA_W1S(0,0),x.u);}
unsafe fn cptvf_clear_misc(c:*mut cpt_vf,bit:u64){let mut x:cptx_vqx_misc_int=core::mem::zeroed();x.u=cpt_read_csr64((*c).reg_base,CPTX_VQX_MISC_INT(0,0));x.u|=bit;cpt_write_csr64((*c).reg_base,CPTX_VQX_MISC_INT(0,0),x.u);}
unsafe fn cptvf_clear_dovf_intr(c:*mut cpt_vf){cptvf_clear_misc(c,CPT_VF_INTR_DOVF_MASK)}
unsafe fn cptvf_clear_irde_intr(c:*mut cpt_vf){cptvf_clear_misc(c,CPT_VF_INTR_IRDE_MASK)}
unsafe fn cptvf_clear_nwrp_intr(c:*mut cpt_vf){cptvf_clear_misc(c,CPT_VF_INTR_NWRP_MASK)}
unsafe fn cptvf_clear_mbox_intr(c:*mut cpt_vf){cptvf_clear_misc(c,CPT_VF_INTR_MBOX_MASK)}
unsafe fn cptvf_clear_swerr_intr(c:*mut cpt_vf){cptvf_clear_misc(c,CPT_VF_INTR_SERR_MASK)}
unsafe fn cptvf_read_vf_misc_intr_status(c:*mut cpt_vf)->u64{cpt_read_csr64((*c).reg_base,CPTX_VQX_MISC_INT(0,0))}
unsafe fn cptvf_read_vq_done_count(c:*mut cpt_vf)->u32{let mut x:cptx_vqx_done=core::mem::zeroed();x.u=cpt_read_csr64((*c).reg_base,CPTX_VQX_DONE(0,0));x.s.done}
unsafe fn cptvf_write_vq_done_ack(c:*mut cpt_vf,v:u32){let mut x:cptx_vqx_done_ack=core::mem::zeroed();x.u=cpt_read_csr64((*c).reg_base,CPTX_VQX_DONE_ACK(0,0));x.s.done_ack=v;cpt_write_csr64((*c).reg_base,CPTX_VQX_DONE_ACK(0,0),x.u);}

// The remaining PCI-driver registration and error-unwind code uses Linux-only
// registration macros and external CPT bindings; preserve their declarations here.
extern "C" { fn cptvf_probe(pdev:*mut pci_dev,ent:*const pci_device_id)->i32; fn cptvf_remove(pdev:*mut pci_dev); fn cptvf_shutdown(pdev:*mut pci_dev); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
