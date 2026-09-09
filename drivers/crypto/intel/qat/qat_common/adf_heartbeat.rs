// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// Kernel and local header dependencies are supplied by the surrounding translation unit.

const ADF_HB_EMPTY_SIG: u32 = 0xA5A5A5A5;

unsafe fn adf_hb_check_polling_freq(accel_dev: *mut adf_accel_dev) -> i32 {
    let curr_time: u64 = adf_clock_get_current_time();
    let polling_time: u64 = curr_time - (*(*accel_dev).heartbeat).last_hb_check_time;

    if polling_time < (*(*accel_dev).heartbeat).hb_timer as u64 {
        dev_warn(&GET_DEV(accel_dev), "HB polling too frequent. Configured HB timer %d ms\n", (*(*accel_dev).heartbeat).hb_timer);
        return -EINVAL;
    }
    (*(*accel_dev).heartbeat).last_hb_check_time = curr_time;
    0
}

unsafe fn validate_hb_ctrs_cnt(accel_dev: *mut adf_accel_dev) -> bool {
    let hb_ctrs: usize = (*(*accel_dev).hw_device).num_hb_ctrs;
    let max_aes: usize = (*(*accel_dev).hw_device).num_engines;
    let hb_struct_size: usize = core::mem::size_of::<hb_cnt_pair>();
    let exp_diff_size: usize = array3_size(ADF_NUM_PKE_STRAND, max_aes, hb_struct_size);
    let dev_ctrs: usize = size_mul(max_aes, hb_ctrs);
    let stats_size: usize = size_mul(dev_ctrs, hb_struct_size);
    let exp_diff_cnt: u32 = (exp_diff_size / core::mem::size_of::<u32>()) as u32;
    let stats_el_cnt: u32 = (stats_size / core::mem::size_of::<u32>()) as u32;
    let hb_stats: *mut hb_cnt_pair = (*(*accel_dev).heartbeat).dma.virt_addr;
    let mem_to_chk: *const u32 = hb_stats.add(dev_ctrs) as *const u32;
    let mut el_diff_cnt: u32 = 0;

    for i in 0..stats_el_cnt {
        if *mem_to_chk.add(i as usize) == ADF_HB_EMPTY_SIG { break; }
        el_diff_cnt += 1;
    }
    el_diff_cnt != 0 && el_diff_cnt == exp_diff_cnt
}

pub unsafe fn adf_heartbeat_check_ctrs(accel_dev: *mut adf_accel_dev) {
    let hb_stats: *mut hb_cnt_pair = (*(*accel_dev).heartbeat).dma.virt_addr;
    let hb_ctrs = (*(*accel_dev).hw_device).num_hb_ctrs;
    let max_aes = (*(*accel_dev).hw_device).num_engines;
    let dev_ctrs = size_mul(max_aes, hb_ctrs);
    let stats_size = size_mul(dev_ctrs, core::mem::size_of::<hb_cnt_pair>());
    let mem_items_to_fill = size_mul(stats_size, 2) / core::mem::size_of::<u32>();
    memset32(hb_stats as *mut u32, ADF_HB_EMPTY_SIG, mem_items_to_fill);
    (*(*accel_dev).heartbeat).ctrs_cnt_checked = false;
}

unsafe fn get_timer_ticks(accel_dev: *mut adf_accel_dev, value: *mut u32) -> i32 {
    let mut timer_str = [0i8; ADF_CFG_MAX_VAL_LEN_IN_BYTES];
    let mut timer_ms: u32 = ADF_CFG_HB_TIMER_DEFAULT_MS;
    let cfg_read_status = adf_cfg_get_param_value(accel_dev, ADF_GENERAL_SEC, ADF_HEARTBEAT_TIMER, timer_str.as_mut_ptr());
    if cfg_read_status == 0 && kstrtouint(timer_str.as_ptr(), 10, &mut timer_ms) != 0 {
        dev_dbg(&GET_DEV(accel_dev), "kstrtouint failed to parse the %s, param value", ADF_HEARTBEAT_TIMER);
    }
    if timer_ms < ADF_CFG_HB_TIMER_MIN_MS { dev_err(&GET_DEV(accel_dev), "Timer cannot be less than %u\n", ADF_CFG_HB_TIMER_MIN_MS); return -EINVAL; }
    if (*accel_dev).timer { timer_ms = ADF_CFG_HB_TIMER_MIN_MS; }
    let mut ticks = 0;
    let ret = adf_heartbeat_ms_to_ticks(accel_dev, timer_ms, &mut ticks);
    if ret != 0 { return ret; }
    adf_heartbeat_save_cfg_param(accel_dev, timer_ms);
    (*(*accel_dev).heartbeat).hb_timer = timer_ms;
    *value = ticks;
    0
}

unsafe fn check_ae(curr: *mut hb_cnt_pair, prev: *mut hb_cnt_pair, count: *mut u16, hb_ctrs: usize) -> i32 {
    for thr in 0..hb_ctrs {
        let req = (*curr.add(thr)).req_heartbeat_cnt;
        let resp = (*curr.add(thr)).resp_heartbeat_cnt;
        let last = (*prev.add(thr)).resp_heartbeat_cnt;
        if (thr == ADF_AE_ADMIN_THREAD || req != resp) && resp == last {
            *count.add(thr) += 1;
            if *count.add(thr) >= ADF_CFG_HB_COUNT_THRESHOLD { return -EIO; }
        } else { *count.add(thr) = 0; }
    }
    0
}

unsafe fn adf_hb_get_status(accel_dev: *mut adf_accel_dev) -> i32 {
    let hw_device = (*accel_dev).hw_device;
    let hb_ctrs = (*hw_device).num_hb_ctrs;
    let max_aes = (*hw_device).num_engines;
    let dev_ctrs = size_mul(max_aes, hb_ctrs);
    let stats_size = size_mul(dev_ctrs, core::mem::size_of::<hb_cnt_pair>());
    if !(*(*accel_dev).heartbeat).ctrs_cnt_checked {
        if validate_hb_ctrs_cnt(accel_dev) { (*hw_device).num_hb_ctrs += ADF_NUM_PKE_STRAND; }
        (*(*accel_dev).heartbeat).ctrs_cnt_checked = true;
    }
    let live_stats = (*(*accel_dev).heartbeat).dma.virt_addr;
    let last_stats = live_stats.add(dev_ctrs);
    let count_fails = (last_stats.add(dev_ctrs)) as *mut u16;
    let curr_stats = kmemdup(live_stats, stats_size, GFP_KERNEL);
    if curr_stats.is_null() { return -ENOMEM; }
    let mut ret = 0;
    let ae_mask = (*hw_device).ae_mask;
    for ae in 0..max_aes { if (ae_mask & (1usize << ae)) != 0 { ret = check_ae(curr_stats.add(ae * hb_ctrs), last_stats.add(ae * hb_ctrs), count_fails.add(ae * hb_ctrs), hb_ctrs); if ret != 0 { break; } } }
    memcpy(last_stats, curr_stats, stats_size);
    kfree(curr_stats);
    ret
}

unsafe fn adf_heartbeat_reset(accel_dev: *mut adf_accel_dev) {
    let curr_time = adf_clock_get_current_time();
    if curr_time - (*(*accel_dev).heartbeat).last_hb_reset_time < ADF_CFG_HB_RESET_MS as u64 { return; }
    (*(*accel_dev).heartbeat).last_hb_reset_time = curr_time;
    if adf_notify_fatal_error(accel_dev) != 0 { dev_err(&GET_DEV(accel_dev), "Failed to notify fatal error\n"); }
}

pub unsafe fn adf_heartbeat_status(accel_dev: *mut adf_accel_dev, hb_status: *mut adf_device_heartbeat_status) {
    if !adf_dev_started(accel_dev) || test_bit(ADF_STATUS_RESTARTING, &(*accel_dev).status) { *hb_status = HB_DEV_UNRESPONSIVE; return; }
    if adf_hb_check_polling_freq(accel_dev) == -EINVAL { *hb_status = HB_DEV_UNSUPPORTED; return; }
    let hb = (*accel_dev).heartbeat;
    (*hb).hb_sent_counter += 1;
    if adf_hb_get_status(accel_dev) != 0 { dev_err(&GET_DEV(accel_dev), "Heartbeat ERROR: QAT is not responding.\n"); *hb_status = HB_DEV_UNRESPONSIVE; (*hb).hb_failed_counter += 1; adf_heartbeat_reset(accel_dev); return; }
    *hb_status = HB_DEV_ALIVE;
}

pub unsafe fn adf_heartbeat_ms_to_ticks(accel_dev: *mut adf_accel_dev, time_ms: u32, value: *mut u32) -> i32 {
    let hw_data = (*accel_dev).hw_device;
    if (*hw_data).get_hb_clock.is_none() { return -EINVAL; }
    let clk_per_sec = ((*hw_data).get_hb_clock.unwrap())(hw_data);
    *value = time_ms * (clk_per_sec / MSEC_PER_SEC);
    0
}

pub unsafe fn adf_heartbeat_save_cfg_param(accel_dev: *mut adf_accel_dev, timer_ms: u32) -> i32 {
    let mut timer_str = [0i8; ADF_CFG_MAX_VAL_LEN_IN_BYTES];
    snprintf(timer_str.as_mut_ptr(), timer_str.len(), "%u", timer_ms);
    adf_cfg_add_key_value_param(accel_dev, ADF_GENERAL_SEC, ADF_HEARTBEAT_TIMER, timer_str.as_ptr(), ADF_STR)
}

pub unsafe fn adf_heartbeat_init(accel_dev: *mut adf_accel_dev) -> i32 {
    let hb = kzalloc_obj::<adf_heartbeat>();
    if hb.is_null() { return -ENOMEM; }
    (*hb).dma.virt_addr = dma_alloc_coherent(&GET_DEV(accel_dev), PAGE_SIZE, &mut (*hb).dma.phy_addr, GFP_KERNEL);
    if (*hb).dma.virt_addr.is_null() { kfree(hb); return -ENOMEM; }
    (*hb).ctrs_cnt_checked = true;
    (*accel_dev).heartbeat = hb;
    0
}

pub unsafe fn adf_heartbeat_start(accel_dev: *mut adf_accel_dev) -> i32 {
    if (*accel_dev).heartbeat.is_null() { dev_warn(&GET_DEV(accel_dev), "Heartbeat instance not found!"); return -EFAULT; }
    if let Some(check) = (*(*accel_dev).hw_device).check_hb_ctrs { check(accel_dev); }
    let mut timer_ticks = 0;
    let ret = get_timer_ticks(accel_dev, &mut timer_ticks);
    if ret != 0 { return ret; }
    let ret = adf_send_admin_hb_timer(accel_dev, timer_ticks);
    if ret != 0 { dev_warn(&GET_DEV(accel_dev), "Heartbeat not supported!"); }
    ret
}

pub unsafe fn adf_heartbeat_shutdown(accel_dev: *mut adf_accel_dev) {
    let hb = (*accel_dev).heartbeat;
    if hb.is_null() { return; }
    if !(*hb).dma.virt_addr.is_null() { dma_free_coherent(&GET_DEV(accel_dev), PAGE_SIZE, (*hb).dma.virt_addr, (*hb).dma.phy_addr); }
    kfree(hb);
    (*accel_dev).heartbeat = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
