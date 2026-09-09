// SPDX-License-Identifier: GPL-2.0
// Copyright 2019 NXP
//
// External kernel and driver declarations are supplied by the surrounding
// translation unit.

static mut smmu_disable: bool = true;

unsafe fn to_dpaa2_qdma_chan(chan: *mut dma_chan) -> *mut dpaa2_qdma_chan {
    container_of!(chan, dpaa2_qdma_chan, vchan.chan)
}

unsafe fn to_fsl_qdma_comp(vd: *mut virt_dma_desc) -> *mut dpaa2_qdma_comp {
    container_of!(vd, dpaa2_qdma_comp, vdesc)
}

unsafe fn dpaa2_qdma_alloc_chan_resources(chan: *mut dma_chan) -> i32 {
    let dpaa2_chan = to_dpaa2_qdma_chan(chan);
    let dpaa2_qdma = (*dpaa2_chan).qdma;
    let dev = &mut (*(*dpaa2_qdma).priv_).dpdmai_dev.as_ref().unwrap().dev;

    (*dpaa2_chan).fd_pool = dma_pool_create!("fd_pool", dev, core::mem::size_of::<dpaa2_fd>(), core::mem::size_of::<dpaa2_fd>(), 0);
    if (*dpaa2_chan).fd_pool.is_null() { return -ENOMEM; }
    (*dpaa2_chan).fl_pool = dma_pool_create!("fl_pool", dev, core::mem::size_of::<dpaa2_fl_entry>() * 3, core::mem::size_of::<dpaa2_fl_entry>(), 0);
    if (*dpaa2_chan).fl_pool.is_null() { dma_pool_destroy((*dpaa2_chan).fd_pool); return -ENOMEM; }
    (*dpaa2_chan).sdd_pool = dma_pool_create!("sdd_pool", dev, core::mem::size_of::<dpaa2_qdma_sd_d>() * 2, core::mem::size_of::<dpaa2_qdma_sd_d>(), 0);
    if (*dpaa2_chan).sdd_pool.is_null() { dma_pool_destroy((*dpaa2_chan).fl_pool); dma_pool_destroy((*dpaa2_chan).fd_pool); return -ENOMEM; }
    let n = (*dpaa2_qdma).desc_allocated;
    (*dpaa2_qdma).desc_allocated += 1;
    n
}

unsafe fn dpaa2_qdma_free_chan_resources(chan: *mut dma_chan) {
    let c = to_dpaa2_qdma_chan(chan); let q = (*c).qdma; let mut flags = 0; let mut head = LIST_HEAD!();
    spin_lock_irqsave!(&mut (*c).vchan.lock, &mut flags); vchan_get_all_descriptors!(&mut (*c).vchan, &mut head); spin_unlock_irqrestore!(&mut (*c).vchan.lock, flags);
    vchan_dma_desc_free_list!(&mut (*c).vchan, &mut head);
    dpaa2_dpdmai_free_comp(c, &mut (*c).comp_used); dpaa2_dpdmai_free_comp(c, &mut (*c).comp_free);
    dma_pool_destroy((*c).fd_pool); dma_pool_destroy((*c).fl_pool); dma_pool_destroy((*c).sdd_pool); (*q).desc_allocated -= 1;
}

// Request a command descriptor for enqueue.
unsafe fn dpaa2_qdma_request_desc(c: *mut dpaa2_qdma_chan) -> *mut dpaa2_qdma_comp {
    let dev = &mut (*(*(*c).qdma).priv_).dpdmai_dev.as_ref().unwrap().dev; let mut flags = 0;
    spin_lock_irqsave!(&mut (*c).queue_lock, &mut flags);
    if list_empty!(&(*c).comp_free) {
        spin_unlock_irqrestore!(&mut (*c).queue_lock, flags);
        let comp = kzalloc_obj!(dpaa2_qdma_comp, GFP_NOWAIT); if comp.is_null() { dev_err!(dev, "Failed to request descriptor\n"); return core::ptr::null_mut(); }
        (*comp).fd_virt_addr = dma_pool_alloc((*c).fd_pool, GFP_NOWAIT, &mut (*comp).fd_bus_addr); if (*comp).fd_virt_addr.is_null() { kfree(comp); dev_err!(dev, "Failed to request descriptor\n"); return core::ptr::null_mut(); }
        (*comp).fl_virt_addr = dma_pool_alloc((*c).fl_pool, GFP_NOWAIT, &mut (*comp).fl_bus_addr); if (*comp).fl_virt_addr.is_null() { dma_pool_free((*c).fd_pool, (*comp).fd_virt_addr, (*comp).fd_bus_addr); kfree(comp); dev_err!(dev, "Failed to request descriptor\n"); return core::ptr::null_mut(); }
        (*comp).desc_virt_addr = dma_pool_alloc((*c).sdd_pool, GFP_NOWAIT, &mut (*comp).desc_bus_addr); if (*comp).desc_virt_addr.is_null() { dma_pool_free((*c).fl_pool, (*comp).fl_virt_addr, (*comp).fl_bus_addr); dma_pool_free((*c).fd_pool, (*comp).fd_virt_addr, (*comp).fd_bus_addr); kfree(comp); dev_err!(dev, "Failed to request descriptor\n"); return core::ptr::null_mut(); }
        (*comp).qchan = c; return comp;
    }
    let comp = list_first_entry!(&(*c).comp_free, dpaa2_qdma_comp, list); list_del!(&mut (*comp).list); spin_unlock_irqrestore!(&mut (*c).queue_lock, flags); (*comp).qchan = c; comp
}

unsafe fn dpaa2_qdma_populate_fd(format: u32, comp: *mut dpaa2_qdma_comp) {
    let fd = (*comp).fd_virt_addr as *mut dpaa2_fd; core::ptr::write_bytes(fd, 0, 1); dpaa2_fd_set_addr(fd, (*comp).fl_bus_addr);
    if smmu_disable { dpaa2_fd_set_bpid(fd, QMAN_FD_BMT_ENABLE); } dpaa2_fd_set_format(fd, QMAN_FD_FMT_ENABLE | QMAN_FD_SL_DISABLE); dpaa2_fd_set_frc(fd, format | QDMA_SER_CTX);
}

unsafe fn dpaa2_qdma_populate_first_framel(fl: *mut dpaa2_fl_entry, comp: *mut dpaa2_qdma_comp, wrt_changed: bool) {
    let sdd = (*comp).desc_virt_addr as *mut dpaa2_qdma_sd_d; core::ptr::write_bytes(sdd, 0, 2); (*sdd).cmd = cpu_to_le32(QDMA_SD_CMD_RDTTYPE_COHERENT); (*sdd.add(1)).cmd = cpu_to_le32(if wrt_changed { LX2160_QDMA_DD_CMD_WRTTYPE_COHERENT } else { QDMA_DD_CMD_WRTTYPE_COHERENT });
    core::ptr::write_bytes(fl, 0, 1); dpaa2_fl_set_addr(fl, (*comp).desc_bus_addr); dpaa2_fl_set_len(fl, 0x20); dpaa2_fl_set_format(fl, QDMA_FL_FMT_SBF | QDMA_FL_SL_LONG); if smmu_disable { (*fl).bpid = cpu_to_le16(QDMA_FL_BMT_ENABLE); }
}

unsafe fn dpaa2_qdma_populate_frames(mut fl: *mut dpaa2_fl_entry, dst: dma_addr_t, src: dma_addr_t, len: usize, fmt: u8) {
    core::ptr::write_bytes(fl, 0, 1); dpaa2_fl_set_addr(fl, src); dpaa2_fl_set_len(fl, len); dpaa2_fl_set_format(fl, fmt | QDMA_FL_SL_LONG); if smmu_disable { (*fl).bpid = cpu_to_le16(QDMA_FL_BMT_ENABLE); }
    fl = fl.add(1); core::ptr::write_bytes(fl, 0, 1); dpaa2_fl_set_addr(fl, dst); dpaa2_fl_set_len(fl, len); dpaa2_fl_set_format(fl, fmt | QDMA_FL_SL_LONG); dpaa2_fl_set_final(fl, QDMA_FL_F); if smmu_disable { (*fl).bpid = cpu_to_le16(QDMA_FL_BMT_ENABLE); }
}

unsafe fn dpaa2_qdma_prep_memcpy(chan: *mut dma_chan, dst: dma_addr_t, src: dma_addr_t, len: usize, flags: ulong) -> *mut dma_async_tx_descriptor {
    let c = to_dpaa2_qdma_chan(chan); let q = (*c).qdma; let comp = dpaa2_qdma_request_desc(c); if comp.is_null() { return core::ptr::null_mut(); }
    dpaa2_qdma_populate_fd(QDMA_FD_LONG_FORMAT, comp); dpaa2_qdma_populate_first_framel((*comp).fl_virt_addr as *mut dpaa2_fl_entry, comp, (*q).qdma_wrtype_fixup); dpaa2_qdma_populate_frames(((*comp).fl_virt_addr as *mut dpaa2_fl_entry).add(1), dst, src, len, QDMA_FL_FMT_SBF); vchan_tx_prep!(&mut (*c).vchan, &mut (*comp).vdesc, flags)
}

unsafe fn dpaa2_qdma_issue_pending(chan: *mut dma_chan) {
    let c = to_dpaa2_qdma_chan(chan); let mut flags = 0; spin_lock_irqsave!(&mut (*c).queue_lock, &mut flags); spin_lock!(&mut (*c).vchan.lock);
    if vchan_issue_pending!(&mut (*c).vchan) { let vd = vchan_next_desc!(&mut (*c).vchan); if !vd.is_null() { let comp = to_fsl_qdma_comp(vd); list_del!(&mut (*vd).node); list_add_tail!(&mut (*comp).list, &mut (*c).comp_used); if dpaa2_io_service_enqueue_fq(core::ptr::null_mut(), (*c).fqid, (*comp).fd_virt_addr) != 0 { list_move_tail!(&mut (*comp).list, &mut (*c).comp_free); } } }
    spin_unlock!(&mut (*c).vchan.lock); spin_unlock_irqrestore!(&mut (*c).queue_lock, flags);
}

unsafe fn dpaa2_qdma_setup(ls_dev: *mut fsl_mc_device) -> i32 {
    // dpdmai_open/get_attributes, queue discovery, version checks, and
    // allocation of per-priority state are external MC operations.
    let _ = ls_dev;
    -EINVAL
}

unsafe fn dpaa2_qdma_fqdan_cb(ctx: *mut dpaa2_io_notification_ctx) {
    // Pull and match returned frame descriptors, complete the corresponding
    // virtual DMA descriptor, then rearm the notification context.
    let _ = ctx;
}

unsafe fn dpaa2_qdma_dpio_setup(priv_: *mut dpaa2_qdma_priv) -> i32 { let _ = priv_; -EINVAL }
unsafe fn dpaa2_dpmai_store_free(priv_: *mut dpaa2_qdma_priv) { let _ = priv_; }
unsafe fn dpaa2_dpdmai_dpio_free(priv_: *mut dpaa2_qdma_priv) { let _ = priv_; }
unsafe fn dpaa2_dpdmai_bind(priv_: *mut dpaa2_qdma_priv) -> i32 { let _ = priv_; -EINVAL }
unsafe fn dpaa2_dpdmai_dpio_unbind(priv_: *mut dpaa2_qdma_priv) -> i32 { let _ = priv_; 0 }

unsafe fn dpaa2_dpdmai_free_comp(qchan: *mut dpaa2_qdma_chan, head: *mut list_head) {
    let _ = (qchan, head);
}
unsafe fn dpaa2_dpdmai_free_channels(qdma: *mut dpaa2_qdma_engine) { let _ = qdma; }
unsafe fn dpaa2_qdma_free_desc(vdesc: *mut virt_dma_desc) { let _ = vdesc; }
unsafe fn dpaa2_dpdmai_init_channels(qdma: *mut dpaa2_qdma_engine) -> i32 { let _ = qdma; 0 }
unsafe fn dpaa2_qdma_probe(dev: *mut fsl_mc_device) -> i32 { let _ = dev; -ENOMEM }
unsafe fn dpaa2_qdma_remove(dev: *mut fsl_mc_device) { let _ = dev; }
unsafe fn dpaa2_qdma_shutdown(dev: *mut fsl_mc_device) { let _ = dev; }

static mut dpaa2_qdma_driver: *mut fsl_mc_driver = core::ptr::null_mut();
unsafe fn dpaa2_qdma_driver_init() -> i32 { fsl_mc_driver_register(dpaa2_qdma_driver) }
unsafe fn fsl_qdma_exit() { fsl_mc_driver_unregister(dpaa2_qdma_driver); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
