/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2023 Intel Corporation */

// Dependency supplied by icp_qat_fw_init_admin.h is intentionally not
// redefined here.

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

extern "C" {
    pub fn adf_init_admin_comms(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_exit_admin_comms(accel_dev: *mut adf_accel_dev);
    pub fn adf_send_admin_init(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_get_ae_fw_counters(
        accel_dev: *mut adf_accel_dev,
        ae: u16,
        reqs: *mut u64,
        resps: *mut u64,
    ) -> i32;
    pub fn adf_init_admin_pm(accel_dev: *mut adf_accel_dev, idle_delay: u32) -> i32;
    pub fn adf_send_admin_tim_sync(accel_dev: *mut adf_accel_dev, cnt: u32) -> i32;
    pub fn adf_send_admin_hb_timer(accel_dev: *mut adf_accel_dev, ticks: u32) -> i32;
    pub fn adf_send_admin_rl_init(
        accel_dev: *mut adf_accel_dev,
        slices: *mut icp_qat_fw_init_admin_slice_cnt,
    ) -> i32;
    pub fn adf_send_admin_rl_add_update(
        accel_dev: *mut adf_accel_dev,
        req: *mut icp_qat_fw_init_admin_req,
    ) -> i32;
    pub fn adf_send_admin_rl_delete(
        accel_dev: *mut adf_accel_dev,
        node_id: u16,
        node_type: u8,
    ) -> i32;
    pub fn adf_get_fw_timestamp(accel_dev: *mut adf_accel_dev, timestamp: *mut u64) -> i32;
    pub fn adf_get_pm_info(
        accel_dev: *mut adf_accel_dev,
        p_state_addr: dma_addr_t,
        buff_size: usize,
    ) -> i32;
    pub fn adf_get_cnv_stats(
        accel_dev: *mut adf_accel_dev,
        ae: u16,
        err_cnt: *mut u16,
        latest_err: *mut u16,
    ) -> i32;
    pub fn adf_send_admin_tl_start(
        accel_dev: *mut adf_accel_dev,
        tl_dma_addr: dma_addr_t,
        layout_sz: usize,
        rp_indexes: *mut u8,
        slice_count: *mut icp_qat_fw_init_admin_slice_cnt,
    ) -> i32;
    pub fn adf_send_admin_tl_stop(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_send_admin_arb_query(
        accel_dev: *mut adf_accel_dev,
        cmd: i32,
        svn: *mut u8,
    ) -> i32;
    pub fn adf_send_admin_arb_commit(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_send_admin_kpt_init(
        accel_dev: *mut adf_accel_dev,
        init_cfg: *mut core::ffi::c_void,
        init_cfg_sz: usize,
        init_ptr: dma_addr_t,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
