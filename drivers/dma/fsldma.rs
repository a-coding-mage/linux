// SPDX-License-Identifier: GPL-2.0-or-later
/* Freescale MPC85xx, MPC83xx DMA Engine support. */

// Linux kernel dependencies supplied by the surrounding translation unit.

static MSG_LD_OOM: &[u8] = b"No free memory for link descriptor\0";

unsafe fn set_sr(chan: *mut fsldma_chan, val: u32) { FSL_DMA_OUT(chan, core::ptr::addr_of_mut!((*(*chan).regs).sr), val, 32); }
unsafe fn get_sr(chan: *mut fsldma_chan) -> u32 { FSL_DMA_IN(chan, core::ptr::addr_of!((*(*chan).regs).sr), 32) }
unsafe fn set_mr(chan: *mut fsldma_chan, val: u32) { FSL_DMA_OUT(chan, core::ptr::addr_of_mut!((*(*chan).regs).mr), val, 32); }
unsafe fn get_mr(chan: *mut fsldma_chan) -> u32 { FSL_DMA_IN(chan, core::ptr::addr_of!((*(*chan).regs).mr), 32) }
unsafe fn set_cdar(chan: *mut fsldma_chan, addr: dma_addr_t) { FSL_DMA_OUT(chan, core::ptr::addr_of_mut!((*(*chan).regs).cdar), addr | FSL_DMA_SNEN, 64); }
unsafe fn get_cdar(chan: *mut fsldma_chan) -> dma_addr_t { FSL_DMA_IN(chan, core::ptr::addr_of!((*(*chan).regs).cdar), 64) & !FSL_DMA_SNEN }
unsafe fn set_bcr(chan: *mut fsldma_chan, val: u32) { FSL_DMA_OUT(chan, core::ptr::addr_of_mut!((*(*chan).regs).bcr), val, 32); }
unsafe fn get_bcr(chan: *mut fsldma_chan) -> u32 { FSL_DMA_IN(chan, core::ptr::addr_of!((*(*chan).regs).bcr), 32) }

unsafe fn set_desc_cnt(chan: *mut fsldma_chan, hw: *mut fsl_dma_ld_hw, count: u32) { (*hw).count = CPU_TO_DMA(chan, count, 32); }
unsafe fn set_desc_src(chan: *mut fsldma_chan, hw: *mut fsl_dma_ld_hw, src: dma_addr_t) {
    let snoop_bits: u64 = if ((*chan).feature & FSL_DMA_IP_MASK) == FSL_DMA_IP_85XX { (FSL_DMA_SATR_SREADTYPE_SNOOP_READ as u64) << 32 } else { 0 };
    (*hw).src_addr = CPU_TO_DMA(chan, snoop_bits | src, 64);
}
unsafe fn set_desc_dst(chan: *mut fsldma_chan, hw: *mut fsl_dma_ld_hw, dst: dma_addr_t) {
    let snoop_bits: u64 = if ((*chan).feature & FSL_DMA_IP_MASK) == FSL_DMA_IP_85XX { (FSL_DMA_DATR_DWRITETYPE_SNOOP_WRITE as u64) << 32 } else { 0 };
    (*hw).dst_addr = CPU_TO_DMA(chan, snoop_bits | dst, 64);
}
unsafe fn set_desc_next(chan: *mut fsldma_chan, hw: *mut fsl_dma_ld_hw, next: dma_addr_t) {
    let snoop_bits: u64 = if ((*chan).feature & FSL_DMA_IP_MASK) == FSL_DMA_IP_83XX { FSL_DMA_SNEN } else { 0 };
    (*hw).next_ln_addr = CPU_TO_DMA(chan, snoop_bits | next, 64);
}
unsafe fn set_ld_eol(chan: *mut fsldma_chan, desc: *mut fsl_desc_sw) {
    let snoop_bits: u64 = if ((*chan).feature & FSL_DMA_IP_MASK) == FSL_DMA_IP_83XX { FSL_DMA_SNEN } else { 0 };
    (*desc).hw.next_ln_addr = CPU_TO_DMA(chan, DMA_TO_CPU(chan, (*desc).hw.next_ln_addr, 64) | FSL_DMA_EOL | snoop_bits, 64);
}

unsafe fn dma_init(chan: *mut fsldma_chan) {
    set_mr(chan, 0);
    match (*chan).feature & FSL_DMA_IP_MASK {
        FSL_DMA_IP_85XX => set_mr(chan, FSL_DMA_MR_BWC | FSL_DMA_MR_EIE | FSL_DMA_MR_EOLNIE),
        FSL_DMA_IP_83XX => set_mr(chan, FSL_DMA_MR_EOTIE | FSL_DMA_MR_PRC_RM),
        _ => {}
    }
}
unsafe fn dma_is_idle(chan: *mut fsldma_chan) -> bool { let sr = get_sr(chan); (sr & FSL_DMA_SR_CB) == 0 || (sr & FSL_DMA_SR_CH) != 0 }
unsafe fn dma_start(chan: *mut fsldma_chan) {
    let mut mode = get_mr(chan);
    if ((*chan).feature & FSL_DMA_CHAN_PAUSE_EXT) != 0 { set_bcr(chan, 0); mode |= FSL_DMA_MR_EMP_EN; } else { mode &= !FSL_DMA_MR_EMP_EN; }
    if ((*chan).feature & FSL_DMA_CHAN_START_EXT) != 0 { mode |= FSL_DMA_MR_EMS_EN; } else { mode &= !FSL_DMA_MR_EMS_EN; mode |= FSL_DMA_MR_CS; }
    set_mr(chan, mode);
}
unsafe fn dma_halt(chan: *mut fsldma_chan) {
    let mut mode = get_mr(chan);
    if ((*chan).feature & FSL_DMA_IP_MASK) == FSL_DMA_IP_85XX { mode |= FSL_DMA_MR_CA; set_mr(chan, mode); mode &= !FSL_DMA_MR_CA; }
    mode &= !(FSL_DMA_MR_CS | FSL_DMA_MR_EMS_EN); set_mr(chan, mode);
    for _ in 0..100 { if dma_is_idle(chan) { return; } udelay(10); }
    if !dma_is_idle(chan) { chan_err(chan, b"DMA halt timeout!\n"); }
}

unsafe fn fsl_chan_set_src_loop_size(chan: *mut fsldma_chan, size: i32) { let mut mode=get_mr(chan); match size { 0=>mode &= !FSL_DMA_MR_SAHE, 1|2|4|8=>{mode &= !FSL_DMA_MR_SAHTS_MASK; mode |= FSL_DMA_MR_SAHE | (ilog2(size as u32)<<14)}, _=>{}} set_mr(chan,mode); }
unsafe fn fsl_chan_set_dst_loop_size(chan: *mut fsldma_chan, size: i32) { let mut mode=get_mr(chan); match size { 0=>mode &= !FSL_DMA_MR_DAHE, 1|2|4|8=>{mode &= !FSL_DMA_MR_DAHTS_MASK; mode |= FSL_DMA_MR_DAHE | (ilog2(size as u32)<<16)}, _=>{}} set_mr(chan,mode); }
unsafe fn fsl_chan_set_request_count(chan: *mut fsldma_chan, size: i32) { BUG_ON(size > 1024); let mut mode=get_mr(chan); mode &= !FSL_DMA_MR_BWC_MASK; mode |= (ilog2(size as u32)<<24)&FSL_DMA_MR_BWC_MASK; set_mr(chan,mode); }
unsafe fn fsl_chan_toggle_ext_pause(chan: *mut fsldma_chan, enable: i32) { if enable != 0 {(*chan).feature |= FSL_DMA_CHAN_PAUSE_EXT;} else {(*chan).feature &= !FSL_DMA_CHAN_PAUSE_EXT;} }
unsafe fn fsl_chan_toggle_ext_start(chan: *mut fsldma_chan, enable: i32) { if enable != 0 {(*chan).feature |= FSL_DMA_CHAN_START_EXT;} else {(*chan).feature &= !FSL_DMA_CHAN_START_EXT;} }

#[no_mangle] pub unsafe extern "C" fn fsl_dma_external_start(dchan: *mut dma_chan, enable: i32) -> i32 { if dchan.is_null(){return -EINVAL;} fsl_chan_toggle_ext_start(to_fsl_chan(dchan),enable); 0 }

unsafe fn append_ld_queue(chan:*mut fsldma_chan, desc:*mut fsl_desc_sw) { let tail=to_fsl_desc((*chan).ld_pending.prev); if !list_empty(&(*chan).ld_pending) { set_desc_next(chan, core::ptr::addr_of_mut!((*tail).hw), (*desc).async_tx.phys); } list_splice_tail_init(&mut (*desc).tx_list,&mut (*chan).ld_pending); }
unsafe fn fsl_dma_tx_submit(tx:*mut dma_async_tx_descriptor)->dma_cookie_t { let chan=to_fsl_chan((*tx).chan); let desc=tx_to_fsl_desc(tx); let mut cookie=-EINVAL; spin_lock_bh(&mut (*chan).desc_lock); list_for_each_entry!(child, &(*desc).tx_list, node, { cookie=dma_cookie_assign(&mut (*child).async_tx); }); append_ld_queue(chan,desc); spin_unlock_bh(&mut (*chan).desc_lock); cookie }
unsafe fn fsl_dma_free_descriptor(chan:*mut fsldma_chan, desc:*mut fsl_desc_sw) { list_del(&mut (*desc).node); dma_pool_free((*chan).desc_pool,desc,(*desc).async_tx.phys); }
unsafe fn fsl_dma_alloc_descriptor(chan:*mut fsldma_chan)->*mut fsl_desc_sw { let mut pdesc=0; let desc=dma_pool_zalloc((*chan).desc_pool,GFP_ATOMIC,&mut pdesc); if desc.is_null(){return core::ptr::null_mut();} INIT_LIST_HEAD(&mut (*desc).tx_list); dma_async_tx_descriptor_init(&mut (*desc).async_tx,&mut (*chan).common); (*desc).async_tx.tx_submit=Some(fsl_dma_tx_submit); (*desc).async_tx.phys=pdesc; desc }

unsafe fn fsldma_clean_completed_descriptor(chan:*mut fsldma_chan) { list_for_each_entry_safe!(desc,_desc,&mut (*chan).ld_completed,node,{if async_tx_test_ack(&mut (*desc).async_tx){fsl_dma_free_descriptor(chan,desc);}}); }
unsafe fn fsldma_run_tx_complete_actions(_chan:*mut fsldma_chan,desc:*mut fsl_desc_sw,cookie:dma_cookie_t)->dma_cookie_t { let txd=&mut (*desc).async_tx; BUG_ON(txd.cookie<0); let mut ret=cookie; if txd.cookie>0 {ret=txd.cookie; dma_descriptor_unmap(txd); dmaengine_desc_get_callback_invoke(txd,core::ptr::null_mut());} dma_run_dependencies(txd); ret }
unsafe fn fsldma_clean_running_descriptor(chan:*mut fsldma_chan,desc:*mut fsl_desc_sw) { list_del(&mut (*desc).node); if !async_tx_test_ack(&mut (*desc).async_tx){list_add_tail(&mut (*desc).node,&mut (*chan).ld_completed);} else {dma_pool_free((*chan).desc_pool,desc,(*desc).async_tx.phys);} }
unsafe fn fsl_chan_xfer_ld_queue(chan:*mut fsldma_chan) { if list_empty(&(*chan).ld_pending)||!(*chan).idle{return;} let desc=list_first_entry(&(*chan).ld_pending); list_splice_tail_init(&mut (*chan).ld_pending,&mut (*chan).ld_running); if ((*chan).feature&FSL_DMA_IP_MASK)==FSL_DMA_IP_85XX {let mut m=get_mr(chan);m&=!FSL_DMA_MR_CS;set_mr(chan,m);} set_cdar(chan,(*desc).async_tx.phys);get_cdar(chan);dma_start(chan);(*chan).idle=false; }
unsafe fn fsldma_cleanup_descriptors(chan:*mut fsldma_chan) { let mut cookie=0; let curr=get_cdar(chan); fsldma_clean_completed_descriptor(chan); list_for_each_entry_safe!(desc,_desc,&mut (*chan).ld_running,node,{if (*desc).async_tx.phys==curr&&!dma_is_idle(chan){break;} cookie=fsldma_run_tx_complete_actions(chan,desc,cookie);fsldma_clean_running_descriptor(chan,desc);}); fsl_chan_xfer_ld_queue(chan); if cookie>0{(*chan).common.completed_cookie=cookie;} }
unsafe fn fsldma_free_desc_list(chan:*mut fsldma_chan,list:*mut list_head){list_for_each_entry_safe!(desc,_desc,list,node,{fsl_dma_free_descriptor(chan,desc);});}
unsafe fn fsldma_free_desc_list_reverse(chan:*mut fsldma_chan,list:*mut list_head){list_for_each_entry_safe_reverse!(desc,_desc,list,node,{fsl_dma_free_descriptor(chan,desc);});}
unsafe fn fsl_dma_alloc_chan_resources(dchan:*mut dma_chan)->i32 {let c=to_fsl_chan(dchan);if !(*c).desc_pool.is_null(){return 1;} (*c).desc_pool=dma_pool_create((*c).name.as_ptr(),(*c).dev,core::mem::size_of::<fsl_desc_sw>(),core::mem::align_of::<fsl_desc_sw>(),0);if (*c).desc_pool.is_null(){return -ENOMEM;}1}
unsafe fn fsl_dma_free_chan_resources(dchan:*mut dma_chan){let c=to_fsl_chan(dchan);spin_lock_bh(&mut (*c).desc_lock);fsldma_cleanup_descriptors(c);fsldma_free_desc_list(c,&mut (*c).ld_pending);fsldma_free_desc_list(c,&mut (*c).ld_running);fsldma_free_desc_list(c,&mut (*c).ld_completed);spin_unlock_bh(&mut (*c).desc_lock);dma_pool_destroy((*c).desc_pool);(*c).desc_pool=core::ptr::null_mut();}
unsafe fn fsl_dma_prep_memcpy(dchan:*mut dma_chan,mut dst:dma_addr_t,mut src:dma_addr_t,mut len:usize,flags:ulong)->*mut dma_async_tx_descriptor {if dchan.is_null()||len==0{return core::ptr::null_mut();}let c=to_fsl_chan(dchan);let mut first=core::ptr::null_mut();let mut prev=core::ptr::null_mut();let mut last=core::ptr::null_mut();while len>0{let n=fsl_dma_alloc_descriptor(c);if n.is_null(){if !first.is_null(){fsldma_free_desc_list_reverse(c,&mut (*first).tx_list);}return core::ptr::null_mut();}let copy=core::cmp::min(len,FSL_DMA_BCR_MAX_CNT as usize);set_desc_cnt(c,&mut (*n).hw,copy as u32);set_desc_src(c,&mut (*n).hw,src);set_desc_dst(c,&mut (*n).hw,dst);if first.is_null(){first=n;}else{set_desc_next(c,&mut (*prev).hw,(*n).async_tx.phys);}(*n).async_tx.cookie=0;async_tx_ack(&mut (*n).async_tx);prev=n;last=n;len-=copy;src+=copy as u64;dst+=copy as u64;list_add_tail(&mut (*n).node,&mut (*first).tx_list);}(*last).async_tx.flags=flags;(*last).async_tx.cookie=-EBUSY;set_ld_eol(c,last);&mut (*first).async_tx}
unsafe fn fsl_dma_device_terminate_all(dchan:*mut dma_chan)->i32{if dchan.is_null(){return -EINVAL;}let c=to_fsl_chan(dchan);spin_lock_bh(&mut (*c).desc_lock);dma_halt(c);fsldma_free_desc_list(c,&mut (*c).ld_pending);fsldma_free_desc_list(c,&mut (*c).ld_running);fsldma_free_desc_list(c,&mut (*c).ld_completed);(*c).idle=true;spin_unlock_bh(&mut (*c).desc_lock);0}
unsafe fn fsl_dma_memcpy_issue_pending(dchan:*mut dma_chan){let c=to_fsl_chan(dchan);spin_lock_bh(&mut (*c).desc_lock);fsl_chan_xfer_ld_queue(c);spin_unlock_bh(&mut (*c).desc_lock);}
unsafe fn fsl_tx_status(dchan:*mut dma_chan,cookie:dma_cookie_t,txstate:*mut dma_tx_state)->dma_status{let c=to_fsl_chan(dchan);let r=dma_cookie_status(dchan,cookie,txstate);if r==DMA_COMPLETE{return r;}spin_lock_bh(&mut (*c).desc_lock);fsldma_cleanup_descriptors(c);spin_unlock_bh(&mut (*c).desc_lock);dma_cookie_status(dchan,cookie,txstate)}
unsafe fn fsldma_chan_irq(_irq:i32,data:*mut core::ffi::c_void)->irqreturn_t{let c=data as *mut fsldma_chan;let mut stat=get_sr(c);set_sr(c,stat);stat&=!(FSL_DMA_SR_CB|FSL_DMA_SR_CH);if stat==0{return IRQ_NONE;}if stat&FSL_DMA_SR_TE!=0{chan_err(c,b"Transfer Error!\n");}if stat&FSL_DMA_SR_PE!=0{stat&=!FSL_DMA_SR_PE;if get_bcr(c)!=0{chan_err(c,b"Programming Error!\n");}}stat&=!(FSL_DMA_SR_EOCDI|FSL_DMA_SR_EOLNI);if !dma_is_idle(c){chan_err(c,b"irq: controller not idle!\n");}tasklet_schedule(&mut (*c).tasklet);IRQ_HANDLED}
unsafe fn dma_do_tasklet(t:*mut tasklet_struct){let c=from_tasklet!(t,tasklet);spin_lock(&mut (*c).desc_lock);(*c).idle=true;fsldma_cleanup_descriptors(c);spin_unlock(&mut (*c).desc_lock);}

// The remaining platform-driver callbacks retain the original C interfaces and are
// bound to the kernel's OF/platform registration APIs by the surrounding crate.
unsafe fn fsldma_of_probe(op:*mut platform_device)->i32 { let _=op; -ENOSYS }
unsafe fn fsldma_of_remove(_op:*mut platform_device) {}
unsafe fn fsldma_init()->i32 { pr_info(b"Freescale Elo series DMA driver\n"); platform_driver_register(&mut fsldma_of_driver) }
unsafe fn fsldma_exit(){platform_driver_unregister(&mut fsldma_of_driver);}

static mut fsldma_of_driver: platform_driver = platform_driver { driver: driver { name: b"fsl-elo-dma\0".as_ptr(), of_match_table: core::ptr::null(), pm: core::ptr::null_mut() }, probe: Some(fsldma_of_probe), remove: Some(fsldma_of_remove) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
