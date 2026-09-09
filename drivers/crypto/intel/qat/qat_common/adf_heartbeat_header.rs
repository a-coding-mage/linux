/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2023 Intel Corporation */

/* Linux dependency declarations are supplied by the surrounding translation. */

pub struct adf_accel_dev;
pub struct dentry;

pub const ADF_CFG_HB_TIMER_MIN_MS: u32 = 200;
pub const ADF_CFG_HB_TIMER_DEFAULT_MS: u32 = 500;
pub const ADF_CFG_HB_COUNT_THRESHOLD: u32 = 3;

pub const ADF_CFG_HB_RESET_MS: u32 = 5000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_device_heartbeat_status {
    HB_DEV_UNRESPONSIVE = 0,
    HB_DEV_ALIVE,
    HB_DEV_UNSUPPORTED,
}

/* Heartbeat counter pair */
#[repr(C)]
pub struct hb_cnt_pair {
    pub resp_heartbeat_cnt: u16,
    pub req_heartbeat_cnt: u16,
}

#[repr(C)]
pub struct hb_dma_addr {
    pub phy_addr: dma_addr_t,
    pub virt_addr: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct adf_heartbeat {
    pub hb_sent_counter: u32,
    pub hb_failed_counter: u32,
    pub hb_timer: u32,
    pub last_hb_check_time: u64,
    pub last_hb_reset_time: u64,
    pub ctrs_cnt_checked: bool,
    pub dma: hb_dma_addr,
    pub dbgfs: adf_heartbeat_dbgfs,
}

#[repr(C)]
pub struct adf_heartbeat_dbgfs {
    pub base_dir: *mut dentry,
    pub status: *mut dentry,
    pub cfg: *mut dentry,
    pub sent: *mut dentry,
    pub failed: *mut dentry,
    /* CONFIG_CRYPTO_DEV_QAT_ERROR_INJECTION */
    #[cfg(feature = "CONFIG_CRYPTO_DEV_QAT_ERROR_INJECTION")]
    pub inject_error: *mut dentry,
}

/* CONFIG_DEBUG_FS */
#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe extern "C" {
    pub fn adf_heartbeat_init(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_heartbeat_start(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_heartbeat_shutdown(accel_dev: *mut adf_accel_dev);

    pub fn adf_heartbeat_ms_to_ticks(
        accel_dev: *mut adf_accel_dev,
        time_ms: u32,
        value: *mut u32,
    ) -> i32;
    pub fn adf_heartbeat_save_cfg_param(
        accel_dev: *mut adf_accel_dev,
        timer_ms: u32,
    ) -> i32;
    pub fn adf_heartbeat_status(
        accel_dev: *mut adf_accel_dev,
        hb_status: *mut adf_device_heartbeat_status,
    );
    pub fn adf_heartbeat_check_ctrs(accel_dev: *mut adf_accel_dev);

    /* CONFIG_CRYPTO_DEV_QAT_ERROR_INJECTION */
    #[cfg(feature = "CONFIG_CRYPTO_DEV_QAT_ERROR_INJECTION")]
    pub fn adf_heartbeat_inject_error(accel_dev: *mut adf_accel_dev) -> i32;
}

#[cfg(all(
    feature = "CONFIG_DEBUG_FS",
    not(feature = "CONFIG_CRYPTO_DEV_QAT_ERROR_INJECTION")
))]
pub unsafe fn adf_heartbeat_inject_error(_accel_dev: *mut adf_accel_dev) -> i32 {
    -EPERM
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn adf_heartbeat_init(_accel_dev: *mut adf_accel_dev) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn adf_heartbeat_start(_accel_dev: *mut adf_accel_dev) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn adf_heartbeat_shutdown(_accel_dev: *mut adf_accel_dev) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn adf_heartbeat_save_cfg_param(
    _accel_dev: *mut adf_accel_dev,
    _timer_ms: u32,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn adf_heartbeat_check_ctrs(_accel_dev: *mut adf_accel_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
