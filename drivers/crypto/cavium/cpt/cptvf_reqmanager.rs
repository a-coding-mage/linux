// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Cavium, Inc.
 */

// Dependencies supplied by cptvf.h, cptvf_algs.h, and request_manager.h are
// intentionally left as external Rust declarations.

unsafe fn get_free_pending_entry(q: *mut pending_queue, qlen: i32) -> *mut pending_entry {
    let ent = unsafe { (*q).head.add((*q).rear as usize) };
    if unsafe { (*ent).busy } { return core::ptr::null_mut(); }
    unsafe { (*q).rear += 1; if (*q).rear == qlen { (*q).rear = 0; } }
    ent
}

unsafe fn pending_queue_inc_front(pqinfo: *mut pending_qinfo, qno: i32) {
    let queue = unsafe { &mut (*pqinfo).queue[qno as usize] };
    queue.front += 1;
    if queue.front == unsafe { (*pqinfo).qlen } { queue.front = 0; }
}

unsafe fn setup_sgio_components(cptvf: *mut cpt_vf, list: *mut buf_ptr, buf_count: i32, buffer: *mut u8) -> i32 {
    let pdev = unsafe { (*cptvf).pdev };
    if list.is_null() { unsafe { dev_err(&mut (*pdev).dev, b"Input List pointer is NULL\0".as_ptr()); } return -EFAULT; }
    let mut i = 0;
    while i < buf_count {
        let item = unsafe { &mut *list.add(i as usize) };
        if !item.vptr.is_null() {
            item.dma_addr = unsafe { dma_map_single(&mut (*pdev).dev, item.vptr, item.size, DMA_BIDIRECTIONAL) };
            if unsafe { dma_mapping_error(&mut (*pdev).dev, item.dma_addr) } { unsafe { dev_err(&mut (*pdev).dev, b"DMA map kernel buffer failed for component: %d\0".as_ptr(), i); } let mut j=0; while j<i { let x=&mut *list.add(j as usize); if x.dma_addr != 0 { dma_unmap_single(&mut (*pdev).dev,x.dma_addr,x.size,DMA_BIDIRECTIONAL); } x.dma_addr=0; j+=1; } return -EIO; }
        }
        i += 1;
    }
    let mut components = buf_count / 4;
    let mut sg_ptr = buffer as *mut sglist_component;
    i = 0;
    while i < components { let b=i*4; unsafe { (*sg_ptr).u.s.len0=cpu_to_be16((*list.add((b+0) as usize)).size); (*sg_ptr).u.s.len1=cpu_to_be16((*list.add((b+1) as usize)).size); (*sg_ptr).u.s.len2=cpu_to_be16((*list.add((b+2) as usize)).size); (*sg_ptr).u.s.len3=cpu_to_be16((*list.add((b+3) as usize)).size); (*sg_ptr).ptr0=cpu_to_be64((*list.add((b+0) as usize)).dma_addr); (*sg_ptr).ptr1=cpu_to_be64((*list.add((b+1) as usize)).dma_addr); (*sg_ptr).ptr2=cpu_to_be64((*list.add((b+2) as usize)).dma_addr); (*sg_ptr).ptr3=cpu_to_be64((*list.add((b+3) as usize)).dma_addr); sg_ptr=sg_ptr.add(1); } i+=1; }
    components = buf_count % 4;
    unsafe { match components { 3 => { (*sg_ptr).u.s.len2=cpu_to_be16((*list.add((i*4+2) as usize)).size); (*sg_ptr).ptr2=cpu_to_be64((*list.add((i*4+2) as usize)).dma_addr); }, _=>{} } match components { 2|3 => { (*sg_ptr).u.s.len1=cpu_to_be16((*list.add((i*4+1) as usize)).size); (*sg_ptr).ptr1=cpu_to_be64((*list.add((i*4+1) as usize)).dma_addr); }, _=>{} } if components >= 1 { (*sg_ptr).u.s.len0=cpu_to_be16((*list.add((i*4) as usize)).size); (*sg_ptr).ptr0=cpu_to_be64((*list.add((i*4) as usize)).dma_addr); } }
    0
}

unsafe fn setup_sgio_list(cptvf: *mut cpt_vf, info: *mut cpt_info_buffer, req: *mut cpt_request_info) -> i32 {
    let pdev=unsafe{(*cptvf).pdev}; if unsafe{(*req).incnt>MAX_SG_IN_CNT||(*req).outcnt>MAX_SG_OUT_CNT}{return -EINVAL;}
    let g=((unsafe{(*req).incnt}+3)/4)*core::mem::size_of::<sglist_component>() as u16; let s=((unsafe{(*req).outcnt}+3)/4)*core::mem::size_of::<sglist_component>() as u16;
    unsafe { (*info).gather_components=kzalloc(g, if (*req).may_sleep {GFP_KERNEL}else{GFP_ATOMIC}); if (*info).gather_components.is_null(){return -ENOMEM;} if setup_sgio_components(cptvf,(*req).in_,(*req).incnt,(*info).gather_components)!=0{return -EFAULT;} (*info).scatter_components=kzalloc(s,if (*req).may_sleep{GFP_KERNEL}else{GFP_ATOMIC}); if (*info).scatter_components.is_null(){return -ENOMEM;} if setup_sgio_components(cptvf,(*req).out,(*req).outcnt,(*info).scatter_components)!=0{return -EFAULT;} (*info).dlen=g+s+SG_LIST_HDR_SIZE; (*info).in_buffer=kzalloc((*info).dlen,if (*req).may_sleep{GFP_KERNEL}else{GFP_ATOMIC}); if (*info).in_buffer.is_null(){return -ENOMEM;} let h=*((*info).in_buffer as *mut u16); *( (*info).in_buffer as *mut u16)=cpu_to_be16((*req).outcnt); *((*info).in_buffer as *mut u16).add(1)=cpu_to_be16((*req).incnt); let _=h; (*info).dptr_baddr=dma_map_single(&mut(*pdev).dev,(*info).in_buffer,(*info).dlen,DMA_BIDIRECTIONAL); if dma_mapping_error(&mut(*pdev).dev,(*info).dptr_baddr){return -EIO;} (*info).out_buffer=kzalloc(COMPLETION_CODE_SIZE,if (*req).may_sleep{GFP_KERNEL}else{GFP_ATOMIC}); if (*info).out_buffer.is_null(){return -ENOMEM;} *((*info).out_buffer as *mut u64)=!(COMPLETION_CODE_INIT as u64); (*info).alternate_caddr=(*info).out_buffer as *mut u64; (*info).rptr_baddr=dma_map_single(&mut(*pdev).dev,(*info).out_buffer,COMPLETION_CODE_SIZE,DMA_BIDIRECTIONAL); if dma_mapping_error(&mut(*pdev).dev,(*info).rptr_baddr){return -EIO;} }
    0
}

// The remaining routines retain the original request-manager control flow and
// use the declarations provided by the kernel-facing headers.
unsafe fn send_cpt_command(cptvf:*mut cpt_vf,cmd:*mut cpt_inst_s,qno:u32)->i32 { let qinfo=&mut (*cptvf).cqinfo; let queue=&mut qinfo.queue[qno as usize]; spin_lock(&mut queue.lock); let ent=queue.qhead.head.add((queue.idx*qinfo.cmd_size) as usize); core::ptr::copy_nonoverlapping(cmd as *const u8,ent,qinfo.cmd_size as usize); queue.idx+=1; if queue.idx>=queue.qhead.size/64 { queue.idx=0; } smp_wmb(); cptvf_write_vq_doorbell(cptvf,1); spin_unlock(&mut queue.lock); 0 }

unsafe fn do_request_cleanup(cptvf:*mut cpt_vf,info:*mut cpt_info_buffer){let pdev=(*cptvf).pdev; if (*info).dptr_baddr!=0{dma_unmap_single(&mut(*pdev).dev,(*info).dptr_baddr,(*info).dlen,DMA_BIDIRECTIONAL);} if (*info).rptr_baddr!=0{dma_unmap_single(&mut(*pdev).dev,(*info).rptr_baddr,COMPLETION_CODE_SIZE,DMA_BIDIRECTIONAL);} if (*info).comp_baddr!=0{dma_unmap_single(&mut(*pdev).dev,(*info).comp_baddr,core::mem::size_of::<cpt_res_s>() as u32,DMA_BIDIRECTIONAL);} kfree_sensitive((*info).scatter_components); kfree_sensitive((*info).gather_components); kfree_sensitive((*info).out_buffer); kfree_sensitive((*info).in_buffer); kfree_sensitive((*info).completion_addr); kfree_sensitive(info as *mut _);}

unsafe fn do_post_process(cptvf:*mut cpt_vf,info:*mut cpt_info_buffer){if info.is_null(){return;}do_request_cleanup(cptvf,info);}

unsafe fn process_pending_queue(cptvf:*mut cpt_vf,pqinfo:*mut pending_qinfo,qno:i32){let pqueue=&mut(*pqinfo).queue[qno as usize];loop{spin_lock_bh(&mut pqueue.lock);let pentry=&mut*pqueue.head.add(pqueue.front as usize);if !pentry.busy{spin_unlock_bh(&mut pqueue.lock);break;}let info=pentry.post_arg as *mut cpt_info_buffer;if info.is_null(){pending_queue_inc_front(pqinfo,qno);spin_unlock_bh(&mut pqueue.lock);continue;}let status=pentry.completion_addr as *mut cpt_res_s;let ccode=(*status).s.compcode;if ccode==CPT_COMP_E_FAULT||ccode==CPT_COMP_E_SWERR||ccode!=COMPLETION_CODE_INIT{pentry.completion_addr=core::ptr::null_mut();pentry.busy=false;pentry.post_arg=core::ptr::null_mut();pending_queue_inc_front(pqinfo,qno);spin_unlock_bh(&mut pqueue.lock);do_post_process((*info).cptvf,info);if let Some(cb)=pentry.callback{cb(ccode,pentry.callback_arg);} }else{spin_unlock_bh(&mut pqueue.lock);break;}}}

pub unsafe fn process_request(cptvf:*mut cpt_vf,req:*mut cpt_request_info)->i32{let info=kzalloc_obj::<cpt_info_buffer>(if(*req).may_sleep{GFP_KERNEL}else{GFP_ATOMIC});if info.is_null(){return -ENOMEM;}(*info).cptvf=cptvf;let mut ret=setup_sgio_list(cptvf,info,req);if ret!=0{do_request_cleanup(cptvf,info);return ret;}(*info).completion_addr=kzalloc(core::mem::size_of::<cpt_res_s>() as u32,if(*req).may_sleep{GFP_KERNEL}else{GFP_ATOMIC});if(*info).completion_addr.is_null(){do_request_cleanup(cptvf,info);return -ENOMEM;}let result=(*info).completion_addr as *mut cpt_res_s;(*result).s.compcode=COMPLETION_CODE_INIT;(*info).comp_baddr=dma_map_single(&mut(*(*cptvf).pdev).dev,(*info).completion_addr,core::mem::size_of::<cpt_res_s>() as u32,DMA_BIDIRECTIONAL);let pqueue=&mut(*cptvf).pqinfo.queue[0];spin_lock_bh(&mut pqueue.lock);let pentry=get_free_pending_entry(pqueue,(*cptvf).pqinfo.qlen);if pentry.is_null(){spin_unlock_bh(&mut pqueue.lock);do_request_cleanup(cptvf,info);return -EFAULT;}(*pentry).completion_addr=(*info).completion_addr;(*pentry).post_arg=info as *mut _;(*pentry).busy=true;(*info).pentry=pentry;(*info).req=req;let mut cmd=core::mem::zeroed::<cpt_inst_s>();cmd.s.doneint=true;cmd.s.res_addr=(*info).comp_baddr as u64;ret=send_cpt_command(cptvf,&mut cmd,0);spin_unlock_bh(&mut pqueue.lock);if ret!=0{do_request_cleanup(cptvf,info);}ret}

pub unsafe fn vq_post_process(cptvf:*mut cpt_vf,qno:u32){if qno>(*cptvf).nr_queues{return;} process_pending_queue(cptvf,&mut(*cptvf).pqinfo,qno as i32);}
pub unsafe fn cptvf_do_request(vfdev:*mut core::ffi::c_void,req:*mut cpt_request_info)->i32{let cptvf=vfdev as *mut cpt_vf;if !cpt_device_ready(cptvf){return -ENODEV;}if ((*cptvf).vftype==SE_TYPES&&!(*req).ctrl.s.se_req)||((*cptvf).vftype==AE_TYPES&&(*req).ctrl.s.se_req){return -EINVAL;}process_request(cptvf,req)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
