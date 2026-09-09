// SPDX-License-Identifier: GPL-2.0+
/*
 * CAAM/SEC 4.x transport/backend driver
 * JobR backend functionality
 *
 * Copyright 2008-2012 Freescale Semiconductor, Inc.
 * Copyright 2019, 2023 NXP
 */

// C includes: linux/of_irq.h, linux/of_address.h, linux/platform_device.h,
// compat.h, ctrl.h, regs.h, jr.h, desc.h, intern.h

#[repr(C)]
struct jr_driver_data {
    /* List of Physical JobR's with the Driver */
    jr_list: list_head,
    jr_alloc_lock: spinlock_t, /* jr_list lock */
}

static mut driver_data: jr_driver_data = unsafe { core::mem::zeroed() };
static mut algs_lock: mutex = unsafe { core::mem::zeroed() };
static mut active_devs: c_uint = 0;

unsafe fn register_algs(jrpriv: *mut caam_drv_private_jr, dev: *mut device) {
    mutex_lock(&mut algs_lock);
    active_devs += 1;
    if active_devs != 1 { mutex_unlock(&mut algs_lock); return; }
    caam_algapi_init(dev);
    caam_algapi_hash_init(dev);
    caam_pkc_init(dev);
    (*jrpriv).hwrng = !caam_rng_init(dev);
    caam_qi_algapi_init(dev);
    mutex_unlock(&mut algs_lock);
}

unsafe fn unregister_algs() {
    mutex_lock(&mut algs_lock);
    active_devs -= 1;
    if active_devs != 0 { mutex_unlock(&mut algs_lock); return; }
    caam_qi_algapi_exit();
    caam_pkc_exit();
    caam_algapi_hash_exit();
    caam_algapi_exit();
    mutex_unlock(&mut algs_lock);
}

unsafe extern "C" fn caam_jr_crypto_engine_exit(data: *mut core::ffi::c_void) {
    let jrdev = data as *mut device;
    let jrpriv = dev_get_drvdata(jrdev) as *mut caam_drv_private_jr;
    /* Free the resources of crypto-engine */
    crypto_engine_exit((*jrpriv).engine);
}

/* Put the CAAM in quiesce, ie stop. Must be called with itr disabled */
unsafe fn caam_jr_stop_processing(dev: *mut device, jrcr_bits: u32) -> c_int {
    let jrp = dev_get_drvdata(dev) as *mut caam_drv_private_jr;
    let mut timeout: c_uint = 100000;
    if rd_reg32(&(*(*jrp).rregs).jrintstatus) & JRINT_ERR_HALT_INPROGRESS != 0 { }
    else {
        clrsetbits_32(&mut (*(*jrp).rregs).jrintstatus, JRINT_ERR_HALT_MASK, 0);
        wr_reg32(&mut (*(*jrp).rregs).jrcommand, jrcr_bits);
    }
    while rd_reg32(&(*(*jrp).rregs).jrintstatus) & JRINT_ERR_HALT_MASK == JRINT_ERR_HALT_INPROGRESS && { timeout -= 1; timeout != 0 } { cpu_relax(); }
    if rd_reg32(&(*(*jrp).rregs).jrintstatus) & JRINT_ERR_HALT_MASK != JRINT_ERR_HALT_COMPLETE || timeout == 0 {
        dev_err(dev, "failed to flush job ring %d\n", (*jrp).ridx); return -EIO;
    }
    0
}

unsafe fn caam_jr_flush(dev: *mut device) -> c_int { caam_jr_stop_processing(dev, JRCR_RESET) }

unsafe fn caam_jr_restart_processing(dev: *mut device) -> c_int {
    let jrp = dev_get_drvdata(dev) as *mut caam_drv_private_jr;
    let halt_status = rd_reg32(&(*(*jrp).rregs).jrintstatus) & JRINT_ERR_HALT_MASK;
    if halt_status != JRINT_ERR_HALT_COMPLETE { return -1; }
    clrsetbits_32(&mut (*(*jrp).rregs).jrintstatus, 0, JRINT_ERR_HALT_COMPLETE);
    0
}

unsafe fn caam_reset_hw_jr(dev: *mut device) -> c_int {
    let jrp = dev_get_drvdata(dev) as *mut caam_drv_private_jr;
    let mut timeout: c_uint = 100000;
    clrsetbits_32(&mut (*(*jrp).rregs).rconfig_lo, 0, JRCFG_IMSK);
    let err = caam_jr_flush(dev); if err != 0 { return err; }
    wr_reg32(&mut (*(*jrp).rregs).jrcommand, JRCR_RESET);
    while rd_reg32(&(*(*jrp).rregs).jrcommand) & JRCR_RESET != 0 && { timeout -= 1; timeout != 0 } { cpu_relax(); }
    if timeout == 0 { dev_err(dev, "failed to reset job ring %d\n", (*jrp).ridx); return -EIO; }
    clrsetbits_32(&mut (*(*jrp).rregs).rconfig_lo, JRCFG_IMSK, 0); 0
}

unsafe fn caam_jr_shutdown(dev: *mut device) -> c_int {
    let jrp = dev_get_drvdata(dev) as *mut caam_drv_private_jr;
    let ret = caam_reset_hw_jr(dev); tasklet_kill(&mut (*jrp).irqtask); ret
}

unsafe extern "C" fn caam_jr_remove(pdev: *mut platform_device) {
    let jrdev = &mut (*pdev).dev; let jrpriv = dev_get_drvdata(jrdev) as *mut caam_drv_private_jr;
    if (*jrpriv).hwrng { caam_rng_exit((*jrdev).parent); }
    if atomic_read(&(*jrpriv).tfm_count) != 0 { dev_alert(jrdev, "Device is busy; consumers might start to crash\n"); return; }
    unregister_algs();
    spin_lock(&mut driver_data.jr_alloc_lock); list_del(&mut (*jrpriv).list_node); spin_unlock(&mut driver_data.jr_alloc_lock);
    if caam_jr_shutdown(jrdev) != 0 { dev_err(jrdev, "Failed to shut down job ring\n"); }
}

unsafe extern "C" fn caam_jr_interrupt(_irq: c_int, st_dev: *mut core::ffi::c_void) -> irqreturn_t {
    let dev = st_dev as *mut device; let jrp = dev_get_drvdata(dev) as *mut caam_drv_private_jr;
    let irqstate = rd_reg32(&(*(*jrp).rregs).jrintstatus);
    if irqstate & JRINT_JR_INT == 0 { return IRQ_NONE; }
    if irqstate & JRINT_JR_ERROR != 0 { dev_err(dev, "job ring error: irqstate: %08x\n", irqstate); BUG!(); }
    clrsetbits_32(&mut (*(*jrp).rregs).rconfig_lo, 0, JRCFG_IMSK);
    wr_reg32(&mut (*(*jrp).rregs).jrintstatus, irqstate);
    preempt_disable(); tasklet_schedule(&mut (*jrp).irqtask); preempt_enable(); IRQ_HANDLED
}

// Deferred service handler, run as interrupt-fired tasklet
unsafe extern "C" fn caam_jr_dequeue(devarg: c_ulong) {
    let params = devarg as *mut caam_jr_dequeue_params; let dev = (*params).dev;
    let jrp = dev_get_drvdata(dev) as *mut caam_drv_private_jr;
    let mut outring_used: u32 = 0;
    while outring_used != 0 || { outring_used = rd_reg32(&(*(*jrp).rregs).outring_used); outring_used != 0 } {
        let head = READ_ONCE((*jrp).head); let mut tail = (*jrp).tail; let mut sw_idx = tail; let hw_idx = (*jrp).out_ring_read_index;
        let mut i = 0; while CIRC_CNT(head, tail + i, JOBR_DEPTH) >= 1 {
            sw_idx = (tail + i) & (JOBR_DEPTH - 1);
            if jr_outentry_desc((*jrp).outring, hw_idx) == caam_dma_to_cpu((*jrp).entinfo[sw_idx].desc_addr_dma) { break; } i += 1;
        }
        BUG_ON(CIRC_CNT(head, tail + i, JOBR_DEPTH) <= 0);
        dma_unmap_single(dev, caam_dma_to_cpu(jr_outentry_desc((*jrp).outring, hw_idx)), (*jrp).entinfo[sw_idx].desc_size, DMA_TO_DEVICE);
        (*jrp).entinfo[sw_idx].desc_addr_dma = 0;
        let usercall = (*jrp).entinfo[sw_idx].callbk; let userarg = (*jrp).entinfo[sw_idx].cbkarg; let userdesc = (*jrp).entinfo[sw_idx].desc_addr_virt;
        let userstatus = caam32_to_cpu(jr_outentry_jrstatus((*jrp).outring, hw_idx)); mb();
        wr_reg32(&mut (*(*jrp).rregs).outring_rmvd, 1); (*jrp).out_ring_read_index = ((*jrp).out_ring_read_index + 1) & (JOBR_DEPTH - 1);
        if sw_idx == tail { loop { tail = (tail + 1) & (JOBR_DEPTH - 1); if !(CIRC_CNT(head, tail, JOBR_DEPTH) >= 1 && (*jrp).entinfo[tail].desc_addr_dma == 0) { break; } } (*jrp).tail = tail; }
        usercall(dev, userdesc, userstatus, userarg); outring_used -= 1;
    }
    if (*params).enable_itr { clrsetbits_32(&mut (*(*jrp).rregs).rconfig_lo, JRCFG_IMSK, 0); }
}

unsafe extern "C" fn caam_jr_alloc() -> *mut device {
    let mut min_jrpriv: *mut caam_drv_private_jr = core::ptr::null_mut(); let mut dev = ERR_PTR(-ENODEV); let mut min_tfm_cnt = INT_MAX;
    spin_lock(&mut driver_data.jr_alloc_lock);
    if list_empty(&mut driver_data.jr_list) { spin_unlock(&mut driver_data.jr_alloc_lock); return ERR_PTR(-ENODEV); }
    let mut jrpriv: *mut caam_drv_private_jr;
    list_for_each_entry!(jrpriv, &mut driver_data.jr_list, list_node) {
        let tfm_cnt = atomic_read(&(*jrpriv).tfm_count); if tfm_cnt < min_tfm_cnt { min_tfm_cnt = tfm_cnt; min_jrpriv = jrpriv; } if min_tfm_cnt == 0 { break; }
    }
    if !min_jrpriv.is_null() { atomic_inc(&mut (*min_jrpriv).tfm_count); dev = (*min_jrpriv).dev; }
    spin_unlock(&mut driver_data.jr_alloc_lock); dev
}

unsafe extern "C" fn caam_jr_free(rdev: *mut device) { let jrpriv = dev_get_drvdata(rdev) as *mut caam_drv_private_jr; atomic_dec(&mut (*jrpriv).tfm_count); }

unsafe extern "C" fn caam_jr_enqueue(dev: *mut device, desc: *mut u32, cbk: Option<unsafe extern "C" fn(*mut device,*mut u32,u32,*mut core::ffi::c_void)>, areq: *mut core::ffi::c_void) -> c_int {
    let jrp = dev_get_drvdata(dev) as *mut caam_drv_private_jr; let desc_size = (caam32_to_cpu(*desc) & HDR_JD_LENGTH_MASK) * core::mem::size_of::<u32>() as u32; let desc_dma = dma_map_single(dev, desc as _, desc_size as _, DMA_TO_DEVICE);
    if dma_mapping_error(dev, desc_dma) { dev_err(dev, "caam_jr_enqueue(): can't map jobdesc\n"); return -EIO; }
    spin_lock_bh(&mut (*jrp).inplock); let head = (*jrp).head; let tail = READ_ONCE((*jrp).tail);
    if !(*jrp).inpring_avail || CIRC_SPACE(head, tail, JOBR_DEPTH) <= 0 { spin_unlock_bh(&mut (*jrp).inplock); dma_unmap_single(dev, desc_dma, desc_size as _, DMA_TO_DEVICE); return -ENOSPC; }
    let entry = &mut (*jrp).entinfo[head]; entry.desc_addr_virt = desc; entry.desc_size = desc_size as _; entry.callbk = cbk; entry.cbkarg = areq; entry.desc_addr_dma = desc_dma;
    jr_inpentry_set((*jrp).inpring, head, cpu_to_caam_dma(desc_dma)); wmb(); (*jrp).head = (head + 1) & (JOBR_DEPTH - 1); wr_reg32(&mut (*(*jrp).rregs).inpring_jobadd, 1);
    (*jrp).inpring_avail -= 1; if !(*jrp).inpring_avail { (*jrp).inpring_avail = rd_reg32(&(*(*jrp).rregs).inpring_avail); } spin_unlock_bh(&mut (*jrp).inplock); -EINPROGRESS
}

unsafe fn caam_jr_init_hw(dev: *mut device, inpbusaddr: dma_addr_t, outbusaddr: dma_addr_t) { let jrp = dev_get_drvdata(dev) as *mut caam_drv_private_jr; wr_reg64(&mut (*(*jrp).rregs).inpring_base, inpbusaddr); wr_reg64(&mut (*(*jrp).rregs).outring_base, outbusaddr); wr_reg32(&mut (*(*jrp).rregs).inpring_size, JOBR_DEPTH); wr_reg32(&mut (*(*jrp).rregs).outring_size, JOBR_DEPTH); clrsetbits_32(&mut (*(*jrp).rregs).rconfig_lo, 0, JOBR_INTC | (JOBR_INTC_COUNT_THLD << JRCFG_ICDCT_SHIFT) | (JOBR_INTC_TIME_THLD << JRCFG_ICTT_SHIFT)); }
unsafe fn caam_jr_reset_index(jrp: *mut caam_drv_private_jr) { (*jrp).out_ring_read_index=0; (*jrp).head=0; (*jrp).tail=0; }

unsafe fn caam_jr_init(dev: *mut device) -> c_int {
    let jrp = dev_get_drvdata(dev) as *mut caam_drv_private_jr; let mut inp=dma_addr_t::default(); let mut out=dma_addr_t::default();
    let error=caam_reset_hw_jr(dev); if error!=0{return error;}
    (*jrp).inpring=dmam_alloc_coherent(dev,SIZEOF_JR_INPENTRY*JOBR_DEPTH,&mut inp,GFP_KERNEL); if (*jrp).inpring.is_null(){return -ENOMEM;}
    (*jrp).outring=dmam_alloc_coherent(dev,SIZEOF_JR_OUTENTRY*JOBR_DEPTH,&mut out,GFP_KERNEL); if (*jrp).outring.is_null(){return -ENOMEM;}
    (*jrp).entinfo=devm_kcalloc(dev,JOBR_DEPTH,core::mem::size_of::<caam_jrentry_info>(),GFP_KERNEL); if (*jrp).entinfo.is_null(){return -ENOMEM;}
    for i in 0..JOBR_DEPTH {(*jrp).entinfo[i].desc_addr_dma=!0;}
    caam_jr_reset_index(jrp); (*jrp).inpring_avail=JOBR_DEPTH; caam_jr_init_hw(dev,inp,out); spin_lock_init(&mut (*jrp).inplock);
    (*jrp).tasklet_params.dev=dev; (*jrp).tasklet_params.enable_itr=1; tasklet_init(&mut (*jrp).irqtask,caam_jr_dequeue,&mut (*jrp).tasklet_params as *mut _ as c_ulong);
    let error=devm_request_irq(dev,(*jrp).irq,Some(caam_jr_interrupt),IRQF_SHARED,dev_name(dev),dev); if error!=0 {dev_err(dev,"can't connect JobR %d interrupt (%d)\n",(*jrp).ridx,(*jrp).irq);tasklet_kill(&mut (*jrp).irqtask);} error
}

unsafe fn caam_jr_get_hw_state(dev:*mut device){let jrp=dev_get_drvdata(dev) as *mut caam_drv_private_jr;(*jrp).state.inpbusaddr=rd_reg64(&(*(*jrp).rregs).inpring_base);(*jrp).state.outbusaddr=rd_reg64(&(*(*jrp).rregs).outring_base);}
unsafe fn caam_jr_suspend(dev:*mut device)->c_int{let p=to_platform_device(dev);let j=platform_get_drvdata(p) as *mut caam_drv_private_jr;spin_lock(&mut driver_data.jr_alloc_lock);list_del(&mut (*j).list_node);spin_unlock(&mut driver_data.jr_alloc_lock);if (*j).hwrng{caam_rng_exit((*dev).parent);}let c=dev_get_drvdata((*dev).parent) as *mut caam_drv_private;if (*c).caam_off_during_pm{tasklet_disable(&mut (*j).irqtask);clrsetbits_32(&mut (*(*j).rregs).rconfig_lo,0,JRCFG_IMSK);let e=caam_jr_flush(dev);if e!=0{return e;}let q=caam_jr_dequeue as unsafe extern "C" fn(c_ulong);q(&mut (*j).tasklet_params as *mut _ as c_ulong);caam_jr_get_hw_state(dev);}else if device_may_wakeup(&(*p).dev){enable_irq_wake((*j).irq);}0}
unsafe fn caam_jr_resume(dev:*mut device)->c_int{let p=to_platform_device(dev);let j=platform_get_drvdata(p) as *mut caam_drv_private_jr;let c=dev_get_drvdata((*dev).parent) as *mut caam_drv_private;if (*c).caam_off_during_pm{let inp=rd_reg64(&(*(*j).rregs).inpring_base);if inp!=0&&inp==(*j).state.inpbusaddr{let e=caam_jr_restart_processing(dev);if e!=0{return e;}tasklet_enable(&mut (*j).irqtask);clrsetbits_32(&mut (*(*j).rregs).rconfig_lo,JRCFG_IMSK,0);}else{caam_jr_reset_index(j);caam_jr_init_hw(dev,(*j).state.inpbusaddr,(*j).state.outbusaddr);tasklet_enable(&mut (*j).irqtask);}}spin_lock(&mut driver_data.jr_alloc_lock);list_add_tail(&mut (*j).list_node,&mut driver_data.jr_list);spin_unlock(&mut driver_data.jr_alloc_lock);if (*j).hwrng{(*j).hwrng=!caam_rng_init((*dev).parent);}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
