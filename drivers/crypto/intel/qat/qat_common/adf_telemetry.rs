// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2023 Intel Corporation. */

// External Linux/kernel and driver declarations are supplied by the surrounding translation.

#[inline]
unsafe fn tl_is_zero(input: u32) -> bool {
    input == 0
}

unsafe fn is_tl_supported(accel_dev: *mut adf_accel_dev) -> bool {
    let fw_caps: u16 = (*get_hw_data(accel_dev)).fw_capabilities;
    (fw_caps & TL_CAPABILITY_BIT) != 0
}

unsafe fn validate_tl_data(tl_data: *mut adf_tl_hw_data) -> i32 {
    if (*tl_data).dev_counters.is_null()
        || tl_is_zero((*tl_data).num_dev_counters as u32)
        || (*tl_data).sl_util_counters.is_null()
        || (*tl_data).sl_exec_counters.is_null()
        || (*tl_data).rp_counters.is_null()
        || tl_is_zero((*tl_data).num_rp_counters as u32)
    {
        return -EOPNOTSUPP;
    }
    0
}

unsafe fn validate_tl_slice_counters(
    slice_count: *mut icp_qat_fw_init_admin_slice_cnt,
    max_slices_per_type: u8,
) -> i32 {
    let sl_counter = slice_count as *mut u8;
    for i in 0..ADF_TL_SL_CNT_COUNT {
        if *sl_counter.add(i as usize) > max_slices_per_type {
            return -EINVAL;
        }
    }
    0
}

unsafe fn adf_tl_alloc_mem(accel_dev: *mut adf_accel_dev) -> i32 {
    let tl_data = &mut get_tl_data(accel_dev);
    let dev = get_dev(accel_dev);
    let regs_sz = tl_data.layout_sz;
    let mut telemetry = kzalloc_node(core::mem::size_of::<adf_telemetry>(), GFP_KERNEL, dev_to_node(dev));
    if telemetry.is_null() { return -ENOMEM; }

    (*telemetry).rp_num_indexes = kmalloc_array(tl_data.max_rp as usize, core::mem::size_of::<u8>(), GFP_KERNEL);
    if (*telemetry).rp_num_indexes.is_null() { goto_err_free_tl(telemetry); return -ENOMEM; }
    (*telemetry).regs_hist_buff = kmalloc_objs(core::mem::size_of::<*mut core::ffi::c_void>(), tl_data.num_hbuff);
    if (*telemetry).regs_hist_buff.is_null() { kfree((*telemetry).rp_num_indexes); goto_err_free_tl(telemetry); return -ENOMEM; }
    (*telemetry).regs_data = dma_alloc_coherent(dev, regs_sz, &mut (*telemetry).regs_data_p, GFP_KERNEL);
    if (*telemetry).regs_data.is_null() { kfree((*telemetry).regs_hist_buff); kfree((*telemetry).rp_num_indexes); goto_err_free_tl(telemetry); return -ENOMEM; }

    let mut i = 0;
    while i < tl_data.num_hbuff {
        let tl_data_regs = kzalloc_node(regs_sz, GFP_KERNEL, dev_to_node(dev));
        if tl_data_regs.is_null() {
            dma_free_coherent(dev, regs_sz, (*telemetry).regs_data, (*telemetry).regs_data_p);
            while i > 0 { i -= 1; kfree(*(*telemetry).regs_hist_buff.add(i)); }
            kfree((*telemetry).regs_hist_buff); kfree((*telemetry).rp_num_indexes); goto_err_free_tl(telemetry); return -ENOMEM;
        }
        *(*telemetry).regs_hist_buff.add(i) = tl_data_regs;
        i += 1;
    }
    (*accel_dev).telemetry = telemetry;
    0
}

unsafe fn goto_err_free_tl(telemetry: *mut adf_telemetry) { kfree(telemetry); }

unsafe fn adf_tl_free_mem(accel_dev: *mut adf_accel_dev) {
    let tl_data = &get_tl_data(accel_dev);
    let telemetry = (*accel_dev).telemetry;
    let dev = get_dev(accel_dev);
    for i in 0..tl_data.num_hbuff { kfree(*(*telemetry).regs_hist_buff.add(i)); }
    dma_free_coherent(dev, tl_data.layout_sz, (*telemetry).regs_data, (*telemetry).regs_data_p);
    kfree((*telemetry).regs_hist_buff); kfree((*telemetry).rp_num_indexes); kfree(telemetry);
    (*accel_dev).telemetry = core::ptr::null_mut();
}

unsafe fn get_next_timeout() -> c_ulong { msecs_to_jiffies(ADF_TL_TIMER_INT_MS) }

unsafe fn snapshot_regs(telemetry: *mut adf_telemetry, size: usize) {
    memcpy(*(*telemetry).regs_hist_buff.add((*telemetry).hb_num as usize), (*telemetry).regs_data, size);
}

unsafe fn tl_work_handler(work: *mut work_struct) {
    let delayed_work = to_delayed_work(work);
    let telemetry = container_of(delayed_work, adf_telemetry::work_ctx);
    let tl_data = &get_tl_data((*telemetry).accel_dev);
    let regs_data = (*telemetry).regs_data as *mut u32;
    let id = tl_data.msg_cnt_off / core::mem::size_of::<u32>();
    let layout_sz = tl_data.layout_sz;
    if atomic_read(&(*telemetry).state) == 0 { cancel_delayed_work_sync(&mut (*telemetry).work_ctx); return; }
    let mut msg_cnt = *regs_data.add(id);
    let old_msg_cnt = msg_cnt;
    if msg_cnt == (*telemetry).msg_cnt { adf_misc_wq_queue_delayed_work(&mut (*telemetry).work_ctx, get_next_timeout()); return; }
    mutex_lock(&mut (*telemetry).regs_hist_lock);
    snapshot_regs(telemetry, layout_sz);
    msg_cnt = *regs_data.add(id);
    if old_msg_cnt != msg_cnt { snapshot_regs(telemetry, layout_sz); }
    (*telemetry).msg_cnt = msg_cnt; (*telemetry).hb_num += 1; (*telemetry).hb_num %= (*telemetry).hbuffs;
    mutex_unlock(&mut (*telemetry).regs_hist_lock);
    adf_misc_wq_queue_delayed_work(&mut (*telemetry).work_ctx, get_next_timeout());
}

pub unsafe fn adf_tl_halt(accel_dev: *mut adf_accel_dev) -> i32 {
    let telemetry = (*accel_dev).telemetry; let dev = get_dev(accel_dev);
    cancel_delayed_work_sync(&mut (*telemetry).work_ctx); atomic_set(&mut (*telemetry).state, 0);
    let ret = adf_send_admin_tl_stop(accel_dev); if ret != 0 { dev_err(dev, "failed to stop telemetry\n"); } ret
}

unsafe fn adf_set_cmdq_cnt(accel_dev: *mut adf_accel_dev, tl_data: *mut adf_tl_hw_data) {
    let slice_cnt = &(*(*accel_dev).telemetry).slice_cnt; let cmdq_cnt = &mut (*(*accel_dev).telemetry).cmdq_cnt;
    cmdq_cnt.cpr_cnt = slice_cnt.cpr_cnt * (*tl_data).multiplier.cpr_cnt; cmdq_cnt.dcpr_cnt = slice_cnt.dcpr_cnt * (*tl_data).multiplier.dcpr_cnt;
    cmdq_cnt.pke_cnt = slice_cnt.pke_cnt * (*tl_data).multiplier.pke_cnt; cmdq_cnt.wat_cnt = slice_cnt.wat_cnt * (*tl_data).multiplier.wat_cnt;
    cmdq_cnt.wcp_cnt = slice_cnt.wcp_cnt * (*tl_data).multiplier.wcp_cnt; cmdq_cnt.ucs_cnt = slice_cnt.ucs_cnt * (*tl_data).multiplier.ucs_cnt; cmdq_cnt.ath_cnt = slice_cnt.ath_cnt * (*tl_data).multiplier.ath_cnt;
}

pub unsafe fn adf_tl_run(accel_dev: *mut adf_accel_dev, state: i32) -> i32 {
    let tl_data = &mut get_tl_data(accel_dev); let telemetry = (*accel_dev).telemetry; let dev = get_dev(accel_dev);
    let mut ret = adf_send_admin_tl_start(accel_dev, (*telemetry).regs_data_p, tl_data.layout_sz, (*telemetry).rp_num_indexes, &mut (*telemetry).slice_cnt);
    if ret != 0 { dev_err(dev, "failed to start telemetry\n"); return ret; }
    ret = validate_tl_slice_counters(&mut (*telemetry).slice_cnt, tl_data.max_sl_cnt); if ret != 0 { dev_err(dev, "invalid value returned by FW\n"); adf_send_admin_tl_stop(accel_dev); return ret; }
    adf_set_cmdq_cnt(accel_dev, tl_data); (*telemetry).hbuffs = state as u32; atomic_set(&mut (*telemetry).state, state); adf_misc_wq_queue_delayed_work(&mut (*telemetry).work_ctx, get_next_timeout()); 0
}

pub unsafe fn adf_tl_init(accel_dev: *mut adf_accel_dev) -> i32 {
    let tl_data = &mut get_tl_data(accel_dev); let max_rp = tl_data.max_rp; let dev = get_dev(accel_dev); let ret = validate_tl_data(tl_data); if ret != 0 { return ret; }
    let ret = adf_tl_alloc_mem(accel_dev); if ret != 0 { dev_err(dev, "failed to initialize: %d\n", ret); return ret; }
    let telemetry = (*accel_dev).telemetry; (*telemetry).accel_dev = accel_dev; mutex_init(&mut (*telemetry).wr_lock); mutex_init(&mut (*telemetry).regs_hist_lock); init_delayed_work(&mut (*telemetry).work_ctx, tl_work_handler);
    for i in 0..max_rp { *(*telemetry).rp_num_indexes.add(i as usize) = ADF_TL_RP_REGS_DISABLED; } 0
}

pub unsafe fn adf_tl_start(accel_dev: *mut adf_accel_dev) -> i32 {
    let dev = get_dev(accel_dev); if (*accel_dev).telemetry.is_null() { return -EOPNOTSUPP; }
    if !is_tl_supported(accel_dev) { dev_info(dev, "feature not supported by FW\n"); adf_tl_free_mem(accel_dev); return -EOPNOTSUPP; } 0
}

pub unsafe fn adf_tl_stop(accel_dev: *mut adf_accel_dev) { if !(*accel_dev).telemetry.is_null() && atomic_read(&(*(*accel_dev).telemetry).state) != 0 { adf_tl_halt(accel_dev); } }
pub unsafe fn adf_tl_shutdown(accel_dev: *mut adf_accel_dev) { if !(*accel_dev).telemetry.is_null() { adf_tl_free_mem(accel_dev); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
