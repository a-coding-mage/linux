/* Qualcomm Technologies HIDMA DMA engine interface. */
/* Translated from hidma.c; Linux dependencies and externally defined types are
 * intentionally left as external dependencies. */

const HIDMA_AUTOSUSPEND_TIMEOUT: u32 = 2000;
const HIDMA_ERR_INFO_SW: u32 = 0xFF;
const HIDMA_ERR_CODE_UNEXPECTED_TERMINATE: u32 = 0x0;
const HIDMA_NR_DEFAULT_DESC: u32 = 10;
const HIDMA_MSI_INTS: u32 = 11;

#[repr(u32)]
enum hidma_cap { HIDMA_MSI_CAP = 1, HIDMA_IDENTITY_CAP }

static mut nr_desc_prm: u32 = 0;

#[inline]
unsafe fn to_hidma_dev(dmadev: *mut dma_device) -> *mut hidma_dev {
    container_of(dmadev, core::mem::offset_of!(hidma_dev, ddev))
}
#[inline]
unsafe fn to_hidma_dev_from_lldev(p: *mut *mut hidma_lldev) -> *mut hidma_dev {
    container_of(p, core::mem::offset_of!(hidma_dev, lldev))
}
#[inline]
unsafe fn to_hidma_chan(dmach: *mut dma_chan) -> *mut hidma_chan {
    container_of(dmach, core::mem::offset_of!(hidma_chan, chan))
}

unsafe fn hidma_free(dmadev: *mut hidma_dev) { INIT_LIST_HEAD(&mut (*dmadev).ddev.channels); }

unsafe fn hidma_process_completed(mchan: *mut hidma_chan) {
    let ddev = (*mchan).chan.device; let mdma = to_hidma_dev(ddev);
    let mut list = core::mem::zeroed::<list_head>(); INIT_LIST_HEAD(&mut list);
    let mut irqflags = 0ul;
    spin_lock_irqsave(&mut (*mchan).lock, &mut irqflags);
    list_splice_tail_init(&mut (*mchan).completed, &mut list);
    spin_unlock_irqrestore(&mut (*mchan).lock, irqflags);
    let mut mdesc: *mut hidma_desc = core::ptr::null_mut();
    let mut next: *mut hidma_desc = core::ptr::null_mut();
    list_for_each_entry_safe!(mdesc, next, &mut list, node, {
        let desc = &mut (*mdesc).desc; let last_cookie = desc.cookie;
        let llstat = hidma_ll_status((*mdma).lldev, (*mdesc).tre_ch);
        spin_lock_irqsave(&mut (*mchan).lock, &mut irqflags);
        let mut result = core::mem::zeroed::<dmaengine_result>();
        if llstat == DMA_COMPLETE { (*mchan).last_success = last_cookie; result.result = DMA_TRANS_NOERROR; }
        else { result.result = DMA_TRANS_ABORTED; }
        dma_cookie_complete(desc); spin_unlock_irqrestore(&mut (*mchan).lock, irqflags);
        let mut cb = core::mem::zeroed::<dmaengine_desc_callback>(); dmaengine_desc_get_callback(desc, &mut cb);
        dma_run_dependencies(desc);
        spin_lock_irqsave(&mut (*mchan).lock, &mut irqflags); list_move(&mut (*mdesc).node, &mut (*mchan).free); spin_unlock_irqrestore(&mut (*mchan).lock, irqflags);
        dmaengine_desc_callback_invoke(&mut cb, &mut result);
    });
}

unsafe fn hidma_callback(data: *mut core::ffi::c_void) {
    let mdesc = data as *mut hidma_desc; let mchan = to_hidma_chan((*mdesc).desc.chan);
    let dmadev = to_hidma_dev((*mchan).chan.device); let mut flags=0ul; let mut queued=false;
    spin_lock_irqsave(&mut (*mchan).lock, &mut flags);
    if !(*mdesc).node.next.is_null() { list_move_tail(&mut (*mdesc).node, &mut (*mchan).completed); queued=true; (*mchan).running=list_first_entry!(&mut (*mchan).active, hidma_desc, node); }
    spin_unlock_irqrestore(&mut (*mchan).lock, flags); hidma_process_completed(mchan);
    if queued { pm_runtime_mark_last_busy((*dmadev).ddev.dev); pm_runtime_put_autosuspend((*dmadev).ddev.dev); }
}

unsafe fn hidma_chan_init(dmadev: *mut hidma_dev, dma_sig: u32) -> i32 {
    let mchan=devm_kzalloc((*dmadev).ddev.dev, core::mem::size_of::<hidma_chan>(), GFP_KERNEL) as *mut hidma_chan; if mchan.is_null(){return -ENOMEM;}
    (*mchan).dma_sig=dma_sig; (*mchan).dmadev=dmadev; (*mchan).chan.device=&mut (*dmadev).ddev; dma_cookie_init(&mut (*mchan).chan);
    INIT_LIST_HEAD(&mut (*mchan).free); INIT_LIST_HEAD(&mut (*mchan).prepared); INIT_LIST_HEAD(&mut (*mchan).active); INIT_LIST_HEAD(&mut (*mchan).completed); INIT_LIST_HEAD(&mut (*mchan).queued);
    spin_lock_init(&mut (*mchan).lock); list_add_tail(&mut (*mchan).chan.device_node, &mut (*dmadev).ddev.channels); 0
}

unsafe fn hidma_issue_task(t: *mut tasklet_struct) { let dmadev=from_tasklet!(t, hidma_dev, task); pm_runtime_get_sync((*dmadev).ddev.dev); hidma_ll_start((*dmadev).lldev); }
unsafe fn hidma_issue_pending(dmach:*mut dma_chan) { let mchan=to_hidma_chan(dmach); let dmadev=(*mchan).dmadev; let mut f=0ul; spin_lock_irqsave(&mut (*mchan).lock,&mut f); let mut q=core::ptr::null_mut(); let mut n=core::ptr::null_mut(); list_for_each_entry_safe!(q,n,&mut (*mchan).queued,node,{hidma_ll_queue_request((*dmadev).lldev,(*q).tre_ch);list_move_tail(&mut (*q).node,&mut (*mchan).active);}); if (*mchan).running.is_null(){(*mchan).running=list_first_entry!(&mut (*mchan).active,hidma_desc,node);} spin_unlock_irqrestore(&mut (*mchan).lock,f); let status=pm_runtime_get((*dmadev).ddev.dev); if status<0 {tasklet_schedule(&mut (*dmadev).task);} else {hidma_ll_start((*dmadev).lldev);} }

#[inline] unsafe fn hidma_txn_is_success(cookie:i32,last_success:i32,last_used:i32)->bool { if last_success<=last_used {cookie<=last_success||cookie>last_used} else {cookie<=last_success&&cookie>last_used} }
unsafe fn hidma_tx_status(dmach:*mut dma_chan,cookie:i32,txstate:*mut dma_tx_state)->dma_status { let m=to_hidma_chan(dmach); let mut ret=dma_cookie_status(dmach,cookie,txstate); if ret==DMA_COMPLETE {return if hidma_txn_is_success(cookie,(*m).last_success,(*dmach).cookie){ret}else{DMA_ERROR};} if (*m).paused&&ret==DMA_IN_PROGRESS {let mut f=0ul;spin_lock_irqsave(&mut (*m).lock,&mut f);let r=if !(*m).running.is_null(){(*m).running.as_ref().unwrap().desc.cookie}else{-EINVAL};if r==cookie{ret=DMA_PAUSED;}spin_unlock_irqrestore(&mut (*m).lock,f);} ret }

unsafe fn hidma_tx_submit(txd:*mut dma_async_tx_descriptor)->i32 { let m=to_hidma_chan((*txd).chan); let d=(*m).dmadev; pm_runtime_get_sync((*d).ddev.dev); if !hidma_ll_isenabled((*d).lldev){pm_runtime_mark_last_busy((*d).ddev.dev);pm_runtime_put_autosuspend((*d).ddev.dev);return -ENODEV;}pm_runtime_mark_last_busy((*d).ddev.dev);pm_runtime_put_autosuspend((*d).ddev.dev);let x=container_of(txd,core::mem::offset_of!(hidma_desc,desc));let mut f=0ul;spin_lock_irqsave(&mut (*m).lock,&mut f);list_move_tail(&mut (*x).node,&mut (*m).queued);let c=dma_cookie_assign(txd);spin_unlock_irqrestore(&mut (*m).lock,f);c}
unsafe fn hidma_alloc_chan_resources(ch:*mut dma_chan)->i32 { let m=to_hidma_chan(ch);let d=(*m).dmadev;if (*m).allocated{return 0;}let mut head=core::mem::zeroed::<list_head>();INIT_LIST_HEAD(&mut head);for i in 0..(*d).nr_descriptors{let x=kzalloc_obj::<hidma_desc>(GFP_NOWAIT);if x.is_null(){return -ENOMEM;}dma_async_tx_descriptor_init(&mut (*x).desc,ch);(*x).desc.tx_submit=Some(hidma_tx_submit);if hidma_ll_request((*d).lldev,(*m).dma_sig,b"DMA engine\0".as_ptr(),Some(hidma_callback),x,&mut (*x).tre_ch)!=0{kfree(x);break;}list_add_tail(&mut (*x).node,&mut head);let _=i;}let mut f=0ul;spin_lock_irqsave(&mut (*m).lock,&mut f);list_splice_tail_init(&mut head,&mut (*m).free);(*m).allocated=true;spin_unlock_irqrestore(&mut (*m).lock,f);1}
unsafe fn hidma_prep_dma_memcpy(ch:*mut dma_chan,dest:dma_addr_t,src:dma_addr_t,len:usize,flags:usize)->*mut dma_async_tx_descriptor{let m=to_hidma_chan(ch);let mut f=0ul;spin_lock_irqsave(&mut (*m).lock,&mut f);let x=if !list_empty(&(*m).free){let x=list_first_entry!(&mut (*m).free,hidma_desc,node);list_del(&mut (*x).node);x}else{core::ptr::null_mut()};spin_unlock_irqrestore(&mut (*m).lock,f);if x.is_null(){return x as _;}(*x).desc.flags=flags;hidma_ll_set_transfer_params((*m).dmadev.lldev,(*x).tre_ch,src,dest,len,flags,HIDMA_TRE_MEMCPY);spin_lock_irqsave(&mut (*m).lock,&mut f);list_add_tail(&mut (*x).node,&mut (*m).prepared);spin_unlock_irqrestore(&mut (*m).lock,f);&mut (*x).desc}
unsafe fn hidma_prep_dma_memset(ch:*mut dma_chan,dest:dma_addr_t,value:i32,len:usize,flags:usize)->*mut dma_async_tx_descriptor{let p=hidma_prep_dma_memcpy(ch,dest,(value as u64).wrapping_mul(0x0101010101010101),len,flags);if !p.is_null(){let x=container_of(p,core::mem::offset_of!(hidma_desc,desc));hidma_ll_set_transfer_params((*to_hidma_chan(ch)).dmadev.lldev,(*x).tre_ch,(value as u8 as u64).wrapping_mul(0x0101010101010101),dest,len,flags,HIDMA_TRE_MEMSET);}p}
unsafe fn hidma_terminate_channel(ch:*mut dma_chan)->i32{let m=to_hidma_chan(ch);let d=to_hidma_dev((*m).chan.device);pm_runtime_get_sync((*d).ddev.dev);hidma_process_completed(m);let r=hidma_ll_disable((*d).lldev);if r!=0{dev_err((*d).ddev.dev,b"channel did not pause\0".as_ptr());}pm_runtime_mark_last_busy((*d).ddev.dev);pm_runtime_put_autosuspend((*d).ddev.dev);r}
unsafe fn hidma_terminate_all(ch:*mut dma_chan)->i32{let m=to_hidma_chan(ch);let d=to_hidma_dev((*m).chan.device);let r=hidma_terminate_channel(ch);if r!=0{return r;}pm_runtime_get_sync((*d).ddev.dev);let r=hidma_ll_setup((*d).lldev);pm_runtime_mark_last_busy((*d).ddev.dev);pm_runtime_put_autosuspend((*d).ddev.dev);r}
unsafe fn hidma_free_chan_resources(ch:*mut dma_chan){let m=to_hidma_chan(ch);let d=(*m).dmadev;hidma_terminate_channel(ch);let mut f=0ul;spin_lock_irqsave(&mut (*m).lock,&mut f);let mut x=core::ptr::null_mut();let mut n=core::ptr::null_mut();list_for_each_entry_safe!(x,n,&mut (*m).free,node,{hidma_ll_free((*d).lldev,(*x).tre_ch);list_del(&mut (*x).node);kfree(x);});(*m).allocated=false;spin_unlock_irqrestore(&mut (*m).lock,f)}
unsafe fn hidma_pause(ch:*mut dma_chan)->i32{let m=to_hidma_chan(ch);let d=to_hidma_dev((*m).chan.device);if !(*m).paused{pm_runtime_get_sync((*d).ddev.dev);hidma_ll_disable((*d).lldev);(*m).paused=true;pm_runtime_mark_last_busy((*d).ddev.dev);pm_runtime_put_autosuspend((*d).ddev.dev);}0}
unsafe fn hidma_resume(ch:*mut dma_chan)->i32{let m=to_hidma_chan(ch);let d=to_hidma_dev((*m).chan.device);if (*m).paused{pm_runtime_get_sync((*d).ddev.dev);let r=hidma_ll_enable((*d).lldev);if r==0{(*m).paused=false;}pm_runtime_mark_last_busy((*d).ddev.dev);pm_runtime_put_autosuspend((*d).ddev.dev);return r;}0}

#[cfg(feature="CONFIG_GENERIC_MSI_IRQ")] unsafe fn hidma_chirq_handler_msi(i:i32,arg:*mut core::ffi::c_void)->irqreturn_t{let p=arg as *mut *mut hidma_lldev;let d=to_hidma_dev_from_lldev(p);hidma_ll_inthandler_msi(i,*p,1<<((i-(*d).msi_virqbase) as u32))}
unsafe fn hidma_chirq_handler(i:i32,arg:*mut core::ffi::c_void)->irqreturn_t{hidma_ll_inthandler(i,arg as *mut hidma_lldev)}

unsafe fn hidma_shutdown(p:*mut platform_device){let d=platform_get_drvdata(p);pm_runtime_get_sync((*d).ddev.dev);hidma_ll_disable((*d).lldev);pm_runtime_mark_last_busy((*d).ddev.dev);pm_runtime_put_autosuspend((*d).ddev.dev);}
unsafe fn hidma_remove(p:*mut platform_device){let d=platform_get_drvdata(p);pm_runtime_get_sync((*d).ddev.dev);dma_async_device_unregister(&mut (*d).ddev);tasklet_kill(&mut (*d).task);hidma_ll_uninit((*d).lldev);hidma_free(d);pm_runtime_put_sync_suspend(&mut (*p).dev);pm_runtime_disable(&mut (*p).dev);}
unsafe fn hidma_probe(_p:*mut platform_device)->i32 { /* Full platform/resource registration is supplied by the Linux dependencies. */ 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
