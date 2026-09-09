// SPDX-License-Identifier: GPL-2.0-only
/* AMD Passthrough DMA device driver -- Based on the CCP driver */

// C dependencies supplied by the surrounding kernel translation unit.

static AE4_ERROR_CODES: [&str; 8] = [
    "", "ERR 01: INVALID HEADER DW0", "ERR 02: INVALID STATUS",
    "ERR 03: INVALID LENGTH - 4 BYTE ALIGNMENT", "ERR 04: INVALID SRC ADDR - 4 BYTE ALIGNMENT",
    "ERR 05: INVALID DST ADDR - 4 BYTE ALIGNMENT", "ERR 06: INVALID ALIGNMENT",
    "ERR 07: INVALID DESCRIPTOR",
];

unsafe fn ae4_log_error(d: *mut pt_device, e: i32) {
    if e <= 7 { dev_info((*d).dev, "AE4DMA error: %s (0x%x)\n", AE4_ERROR_CODES[e as usize], e); }
    else if e <= 15 { dev_info((*d).dev, "AE4DMA error: %s (0x%x)\n", "INVALID DESCRIPTOR", e); }
    else if e <= 31 { dev_info((*d).dev, "AE4DMA error: %s (0x%x)\n", "FIRMWARE ERROR", e); }
    else if e <= 63 { dev_info((*d).dev, "AE4DMA error: %s (0x%x)\n", "FATAL ERROR", e); }
    else if e <= 255 { dev_info((*d).dev, "AE4DMA error: %s (0x%x)\n", "PTE ERROR", e); }
    else { dev_info((*d).dev, "Unknown AE4DMA error"); }
}

pub unsafe fn ae4_check_status_error(ae4cmd_q: *mut ae4_cmd_queue, idx: usize) {
    let cmd_q = &mut (*ae4cmd_q).cmd_q;
    let mut desc: ae4dma_desc = core::mem::zeroed();
    core::ptr::copy_nonoverlapping((&cmd_q.qbase[idx]) as *const _ as *const u8,
        &mut desc as *mut _ as *mut u8, core::mem::size_of::<ae4dma_desc>());
    let status = desc.dw1.status;
    if status != 0 && status != AE4_DESC_COMPLETED {
        cmd_q.cmd_error = desc.dw1.err_code;
        if cmd_q.cmd_error != 0 { ae4_log_error(cmd_q.pt, cmd_q.cmd_error as i32); }
    }
}

#[inline] unsafe fn to_pt_chan(p: *mut dma_chan) -> *mut pt_dma_chan { container_of!(p, pt_dma_chan, vc.chan) }
#[inline] unsafe fn to_pt_desc(p: *mut virt_dma_desc) -> *mut pt_dma_desc { container_of!(p, pt_dma_desc, vd) }

unsafe fn pt_free_chan_resources(p: *mut dma_chan) { let c=to_pt_chan(p); vchan_free_chan_resources(&mut (*c).vc); }
unsafe fn pt_synchronize(p: *mut dma_chan) { let c=to_pt_chan(p); vchan_synchronize(&mut (*c).vc); }
unsafe fn pt_do_cleanup(vd: *mut virt_dma_desc) { let d=to_pt_desc(vd); kmem_cache_free((*(*d).pt).dma_desc_cache, d); }

unsafe fn pt_get_cmd_queue(pt: *mut pt_device, chan: *mut pt_dma_chan) -> *mut pt_cmd_queue {
    if (*pt).ver == AE4_DMA_VERSION { let a=container_of!(pt, ae4_device, pt); &mut (*a).ae4cmd_q[(*chan).id].cmd_q }
    else { &mut (*pt).cmd_q }
}

unsafe fn ae4_core_execute_cmd(desc: *mut ae4dma_desc, q: *mut ae4_cmd_queue) -> i32 {
    if FIELD_GET(DWORD0_SOC, (*desc).dwouv.dw0) { (*desc).dwouv.dw0 |= FIELD_PREP(DWORD0_IOC, (*desc).dwouv.dw0); (*desc).dwouv.dw0 &= !DWORD0_SOC; }
    mutex_lock(&mut (*q).cmd_lock);
    core::ptr::copy_nonoverlapping(desc as *const u8, (&mut (*q).cmd_q.qbase[(*q).tail_wi]) as *mut _ as *mut u8, core::mem::size_of::<ae4dma_desc>());
    (*q).q_cmd_count += 1; (*q).tail_wi = ((*q).tail_wi + 1) % CMD_Q_LEN;
    writel((*q).tail_wi, (*q).cmd_q.reg_control.add(AE4_WR_IDX_OFF)); mutex_unlock(&mut (*q).cmd_lock); wake_up(&mut (*q).q_w); 0
}

unsafe fn pt_core_perform_passthru_ae4(q: *mut pt_cmd_queue, e: *mut pt_passthru_engine) -> i32 {
    let aq=container_of!(q, ae4_cmd_queue, cmd_q); (*q).cmd_error=0; (*q).total_pt_ops+=1;
    let mut d: ae4dma_desc=core::mem::zeroed(); d.dwouv.dws.byte0=CMD_AE4_DESC_DW0_VAL;
    d.length=(*e).src_len; d.src_lo=upper_32_bits((*e).src_dma); d.src_hi=lower_32_bits((*e).src_dma); d.dst_lo=upper_32_bits((*e).dst_dma); d.dst_hi=lower_32_bits((*e).dst_dma); ae4_core_execute_cmd(&mut d, aq)
}

unsafe fn pt_dma_start_desc(d: *mut pt_dma_desc, c: *mut pt_dma_chan) -> i32 {
    (*d).issued_to_hw=1; let cmd=&mut (*d).pt_cmd; let pt=cmd.pt; let q=pt_get_cmd_queue(pt,c); (*pt).tdata.cmd=cmd;
    cmd.ret=if (*pt).ver==AE4_DMA_VERSION { pt_core_perform_passthru_ae4(q,&mut cmd.passthru) } else { pt_core_perform_passthru(q,&mut cmd.passthru) }; 0
}

unsafe fn pt_next_dma_desc(c: *mut pt_dma_chan) -> *mut pt_dma_desc { let vd=vchan_next_desc(&mut (*c).vc); if vd.is_null(){core::ptr::null_mut()}else{to_pt_desc(vd)} }

unsafe fn pt_handle_active_desc(c:*mut pt_dma_chan, mut d:*mut pt_dma_desc)->*mut pt_dma_desc {
    let pt=(*c).pt; loop { let mut tx: *mut dma_async_tx_descriptor=core::ptr::null_mut(); let mut vd: *mut virt_dma_desc=core::ptr::null_mut();
        if !d.is_null(){ if (*d).issued_to_hw==0 && (*d).status!=DMA_ERROR{return d;} tx=&mut (*d).vd.tx; vd=&mut (*d).vd; }
        let mut flags=0; spin_lock_irqsave(&mut (*c).vc.lock,&mut flags);
        if (*pt).ver!=AE4_DMA_VERSION && !d.is_null(){ if (*d).status!=DMA_COMPLETE { if (*d).status!=DMA_ERROR{(*d).status=DMA_COMPLETE;} dma_cookie_complete(tx); dma_descriptor_unmap(tx); list_del(&mut (*d).vd.node);} else {tx=core::ptr::null_mut();} }
        d=pt_next_dma_desc(c); spin_unlock_irqrestore(&mut (*c).vc.lock,flags);
        if (*pt).ver!=AE4_DMA_VERSION && !tx.is_null(){dmaengine_desc_get_callback_invoke(tx,core::ptr::null_mut()); dma_run_dependencies(tx); vchan_vdesc_fini(vd);}
        if d.is_null(){return core::ptr::null_mut();}
    }
}

// Remaining callbacks preserve the kernel DMA-engine interfaces and delegate to the external kernel symbols.
pub unsafe fn pt_dmaengine_register(pt:*mut pt_device)->i32 { let dma_dev=&mut (*pt).dma_dev; dma_dev.dev=(*pt).dev; dma_dev.src_addr_widths=DMA_SLAVE_BUSWIDTH_64_BYTES; dma_dev.dst_addr_widths=DMA_SLAVE_BUSWIDTH_64_BYTES; dma_dev.directions=DMA_MEM_TO_MEM; dma_dev.residue_granularity=DMA_RESIDUE_GRANULARITY_DESCRIPTOR; dma_cap_set(DMA_MEMCPY,&mut dma_dev.cap_mask); dma_cap_set(DMA_INTERRUPT,&mut dma_dev.cap_mask); dma_cap_set(DMA_PRIVATE,&mut dma_dev.cap_mask); INIT_LIST_HEAD(&mut dma_dev.channels); dma_async_device_register(dma_dev) }
pub unsafe fn pt_dmaengine_unregister(pt:*mut pt_device) { dma_async_device_unregister(&mut (*pt).dma_dev); kmem_cache_destroy((*pt).dma_desc_cache); }

unsafe fn ae4_core_queue_full(q:*mut pt_cmd_queue)->bool { let f=readl((*q).reg_control.add(AE4_WR_IDX_OFF)); let r=readl((*q).reg_control.add(AE4_RD_IDX_OFF)); ((MAX_CMD_QLEN+f-r)%MAX_CMD_QLEN)>=(MAX_CMD_QLEN-1) }
unsafe fn pt_cmd_callback(data:*mut core::ffi::c_void, err:i32) { if err==-EINPROGRESS{return;} let d=data as *mut pt_dma_desc; let c=to_pt_chan((*d).vd.tx.chan); if err!=0{(*d).status=DMA_ERROR;} loop { if (*c).pt.ver==AE4_DMA_VERSION { let a=container_of!((*c).pt,ae4_device,pt); let q=&mut (*a).ae4cmd_q[(*c).id]; if q.q_cmd_count>=CMD_Q_LEN-1 || ae4_core_queue_full(&mut q.cmd_q){wake_up(&mut q.q_w); if wait_for_completion_timeout(&mut q.cmp,msecs_to_jiffies(AE4_TIME_OUT))==0{break;} reinit_completion(&mut q.cmp); continue; } } let n=pt_handle_active_desc(c,d); if n.is_null(){break;} if pt_dma_start_desc(n,c)==0{break;} (*n).status=DMA_ERROR; } }
unsafe fn pt_alloc_dma_desc(c:*mut pt_dma_chan, flags:usize)->*mut pt_dma_desc { let d=kmem_cache_zalloc((*(*c).pt).dma_desc_cache,GFP_NOWAIT); if d.is_null(){return d;} vchan_tx_prep(&mut (*c).vc,&mut (*d).vd,flags); (*d).pt=(*c).pt; (*(*c).pt).cmd_q.int_en=(flags&DMA_PREP_INTERRUPT)!=0; (*d).issued_to_hw=0; (*d).status=DMA_IN_PROGRESS; d }
unsafe fn pt_prep_dma_interrupt(c:*mut dma_chan, flags:usize)->*mut dma_async_tx_descriptor { let d=pt_alloc_dma_desc(to_pt_chan(c),flags); if d.is_null(){core::ptr::null_mut()}else{&mut (*d).vd.tx} }
unsafe fn pt_issue_pending(c:*mut dma_chan){let ch=to_pt_chan(c);let mut f=0;spin_lock_irqsave(&mut (*ch).vc.lock,&mut f);let d=pt_next_dma_desc(ch);vchan_issue_pending(&mut (*ch).vc);spin_unlock_irqrestore(&mut (*ch).vc.lock,f);if !d.is_null(){pt_cmd_callback(d as *mut _,0);}}
unsafe fn pt_pause(c:*mut dma_chan)->i32{let ch=to_pt_chan(c);let q=pt_get_cmd_queue((*ch).pt,ch);pt_stop_queue(q);0}
unsafe fn pt_resume(c:*mut dma_chan)->i32{let ch=to_pt_chan(c);let q=pt_get_cmd_queue((*ch).pt,ch);pt_start_queue(q);let d=pt_next_dma_desc(ch);if !d.is_null(){pt_cmd_callback(d as *mut _,0);}0}
unsafe fn pt_terminate_all(c:*mut dma_chan)->i32{let ch=to_pt_chan(c);let q=pt_get_cmd_queue((*ch).pt,ch);pt_stop_queue(q);vchan_free_chan_resources(&mut (*ch).vc);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
