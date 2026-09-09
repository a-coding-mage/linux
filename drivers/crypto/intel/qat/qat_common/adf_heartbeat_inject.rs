// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// Dependencies supplied by the surrounding kernel/driver translation.

const MAX_HB_TICKS: u32 = 0xFFFF_FFFF;

unsafe fn adf_hb_set_timer_to_max(accel_dev: *mut adf_accel_dev) -> i32 {
    let hw_data = (*accel_dev).hw_device;

    (*(*accel_dev).heartbeat).hb_timer = 0;

    if (*hw_data).stop_timer.is_some() {
        ((*hw_data).stop_timer.unwrap())(accel_dev);
    }

    adf_send_admin_hb_timer(accel_dev, MAX_HB_TICKS)
}

unsafe fn adf_set_hb_counters_fail(
    accel_dev: *mut adf_accel_dev,
    ae: u32,
    thr: u32,
) {
    let stats = (*(*accel_dev).heartbeat).dma.virt_addr as *mut hb_cnt_pair;
    let hw_device = (*accel_dev).hw_device;
    let max_aes: usize = ((*hw_device).get_num_aes.unwrap())(hw_device) as usize;
    let hb_ctrs: usize = (*hw_device).num_hb_ctrs as usize;
    let thr_id: usize = ae as usize * hb_ctrs + thr as usize;
    let num_rsp: u16 = (*stats.add(thr_id)).resp_heartbeat_cnt;

    /*
     * Inject live.req != live.rsp and live.rsp == last.rsp
     * to trigger the heartbeat error detection
     */
    (*stats.add(thr_id)).req_heartbeat_cnt =
        (*stats.add(thr_id)).req_heartbeat_cnt.wrapping_add(1);
    let stats = stats.add(max_aes * hb_ctrs);
    (*stats.add(thr_id)).resp_heartbeat_cnt = num_rsp;
}

pub unsafe fn adf_heartbeat_inject_error(accel_dev: *mut adf_accel_dev) -> i32 {
    let hw_device = (*accel_dev).hw_device;
    let max_aes: usize = ((*hw_device).get_num_aes.unwrap())(hw_device) as usize;
    let hb_ctrs: usize = (*hw_device).num_hb_ctrs as usize;
    let mut rand: u32;
    let mut rand_ae: u32;
    let mut rand_thr: u32;
    let ae_mask: ::core::ffi::c_ulong = (*hw_device).ae_mask;
    let ret: i32;

    loop {
        /* Ensure we have a valid ae */
        get_random_bytes(
            &mut rand as *mut u32 as *mut ::core::ffi::c_void,
            core::mem::size_of::<u32>(),
        );
        rand_ae = rand % max_aes as u32;
        if test_bit(rand_ae as usize, &ae_mask as *const _ as *const ::core::ffi::c_ulong) {
            break;
        }
    }

    get_random_bytes(
        &mut rand as *mut u32 as *mut ::core::ffi::c_void,
        core::mem::size_of::<u32>(),
    );
    rand_thr = rand % hb_ctrs as u32;

    /* Increase the heartbeat timer to prevent FW updating HB counters */
    ret = adf_hb_set_timer_to_max(accel_dev);
    if ret != 0 {
        return ret;
    }

    /* Disable arbiter to stop processing any packet */
    ((*hw_device).exit_arb.unwrap())(accel_dev);

    /* Change HB counters memory to simulate a hang */
    adf_set_hb_counters_fail(accel_dev, rand_ae, rand_thr);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
