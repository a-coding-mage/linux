/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2019-2021, The Linux Foundation. All rights reserved. */
/* Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries. */

// Linux and DRM dependencies supplied by other translation units.
use core::ffi::c_void;

pub const QAIC_DBC_BASE: usize = 128 * 1024;
pub const QAIC_DBC_SIZE: usize = 4 * 1024;
pub const QAIC_SSR_DBC_SENTINEL: u32 = u32::MAX; // No ongoing SSR sentinel
pub const QAIC_NO_PARTITION: i32 = -1;

#[inline]
pub const fn qaic_dbc_off(i: usize) -> usize { i * QAIC_DBC_SIZE + QAIC_DBC_BASE }

#[repr(C)] pub struct kref;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct srcu_struct;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct wait_queue_head_t;
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct completion;
#[repr(C)] pub struct pci_dev;
#[repr(C)] pub struct mhi_controller;
#[repr(C)] pub struct mhi_device;
#[repr(C)] pub struct mhi_result;
#[repr(C)] pub struct workqueue_struct;
#[repr(C)] pub struct drm_device;
#[repr(C)] pub struct drm_gem_object;
#[repr(C)] pub struct drm_file;
#[repr(C)] pub struct dma_buf;
#[repr(C)] pub struct sg_table;
#[repr(C)] pub struct dbc_req;
#[repr(C)] pub struct drm_accel;

pub type dma_addr_t = usize;
pub type irqreturn_t = i32;
pub type atomic_t = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum aic_families { FAMILY_AIC100, FAMILY_AIC200, FAMILY_MAX }

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub enum dev_states {
    // Device is offline or will be very soon
    QAIC_OFFLINE,
    // Device is booting, not clear if it's in a usable state
    QAIC_BOOT,
    // Device is fully operational
    QAIC_ONLINE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dbc_states {
    // DBC is free and can be activated
    DBC_STATE_IDLE,
    // DBC is activated and a workload is running on device
    DBC_STATE_ASSIGNED,
    // Sub-system associated with this workload has crashed and it will shutdown soon
    DBC_STATE_BEFORE_SHUTDOWN,
    // Sub-system associated with this workload has crashed and it has shutdown
    DBC_STATE_AFTER_SHUTDOWN,
    // Sub-system associated with this workload is shutdown and it will be powered up soon
    DBC_STATE_BEFORE_POWER_UP,
    // Sub-system associated with this workload is now powered up
    DBC_STATE_AFTER_POWER_UP,
    DBC_STATE_MAX,
}

extern "C" { pub static mut datapath_polling: bool; }

#[repr(C)]
pub struct qaic_user {
    pub handle: i32,
    pub ref_count: kref,
    pub qddev: *mut qaic_drm_device,
    pub node: list_head,
    pub qddev_lock: srcu_struct,
    pub chunk_id: atomic_t,
}

#[repr(C)]
pub struct dma_bridge_chan {
    pub qdev: *mut qaic_device,
    pub id: u32,
    pub xfer_lock: spinlock_t,
    pub req_q_base: *mut c_void,
    pub rsp_q_base: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub total_size: u32,
    pub nelem: u32,
    pub usr: *mut qaic_user,
    pub next_req_id: u16,
    pub in_use: bool,
    pub dbc_base: *mut c_void,
    pub req_lock: mutex,
    pub xfer_list: list_head,
    pub ch_lock: srcu_struct,
    pub dbc_release: wait_queue_head_t,
    pub bo_lists: list_head,
    pub irq: u32,
    pub poll_work: work_struct,
    pub state: u32,
}

#[repr(C)]
pub struct qaic_device {
    pub pdev: *mut pci_dev,
    pub next_seq_num: u32,
    pub bar_mhi: *mut c_void,
    pub bar_dbc: *mut c_void,
    pub mhi_cntrl: *mut mhi_controller,
    pub cntl_ch: *mut mhi_device,
    pub cntl_xfer_list: list_head,
    pub cntl_mutex: mutex,
    pub cntl_wq: *mut workqueue_struct,
    pub dev_lock: srcu_struct,
    pub dev_state: dev_states,
    pub single_msi: bool,
    pub cntl_lost_buf: bool,
    pub num_dbc: u32,
    pub qddev: *mut qaic_drm_device,
    pub gen_crc: Option<unsafe extern "C" fn(*mut c_void) -> u32>,
    pub valid_crc: Option<unsafe extern "C" fn(*mut c_void) -> bool>,
    pub qts_ch: *mut mhi_device,
    pub qts_wq: *mut workqueue_struct,
    pub mqts_ch: *mut mhi_device,
    pub bootlog: list_head,
    pub bootlog_ch: *mut mhi_device,
    pub bootlog_wq: *mut workqueue_struct,
    pub bootlog_mutex: mutex,
    pub ras_ch: *mut mhi_device,
    pub ce_count: u32,
    pub ue_count: u32,
    pub ue_nf_count: u32,
    pub ssr_ch: *mut mhi_device,
    pub ssr_wq: *mut workqueue_struct,
    pub ssr_mhi_buf: *mut c_void,
    pub ssr_dbc: u32,
    pub dbc: [dma_bridge_chan; 0], // C flexible array member, __counted_by(num_dbc)
}

#[repr(C)]
pub struct qaic_drm_device {
    pub drm: drm_device,
    pub qdev: *mut qaic_device,
    pub partition_id: i32,
    pub users: list_head,
    pub users_mutex: mutex,
    pub sysfs_attrs: *mut c_void,
}

#[repr(C)]
pub struct qaic_bo {
    pub base: drm_gem_object,
    pub sgt: *mut sg_table,
    pub slices: list_head,
    pub total_slice_nents: i32,
    pub dir: i32,
    pub dbc: *mut dma_bridge_chan,
    pub nr_slice: u32,
    pub nr_slice_xfer_done: u32,
    pub sliced: bool,
    pub req_id: u16,
    pub xfer_done: completion,
    pub xfer_list: list_head,
    pub bo_list: list_head,
    pub perf_stats: qaic_bo_perf_stats,
    pub lock: mutex,
}

#[repr(C)]
pub struct qaic_bo_perf_stats {
    pub req_received_ts: u64,
    pub req_submit_ts: u64,
    pub req_processed_ts: u64,
    pub queue_level_before: u32,
}

#[repr(C)]
pub struct bo_slice {
    pub sgt: *mut sg_table,
    pub nents: i32,
    pub dir: i32,
    pub reqs: *mut dbc_req,
    pub ref_count: kref,
    pub no_xfer: bool,
    pub bo: *mut qaic_bo,
    pub slice: list_head,
    pub size: u64,
    pub offset: u64,
}

extern "C" {
    pub fn get_dbc_req_elem_size() -> i32;
    pub fn get_dbc_rsp_elem_size() -> i32;
    pub fn get_cntl_version(qdev: *mut qaic_device, usr: *mut qaic_user, major: *mut u16, minor: *mut u16) -> i32;
    pub fn qaic_manage_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> i32;
    pub fn qaic_mhi_ul_xfer_cb(mhi_dev: *mut mhi_device, mhi_result: *mut mhi_result);
    pub fn qaic_mhi_dl_xfer_cb(mhi_dev: *mut mhi_device, mhi_result: *mut mhi_result);
    pub fn qaic_control_open(qdev: *mut qaic_device) -> i32;
    pub fn qaic_control_close(qdev: *mut qaic_device);
    pub fn qaic_release_usr(qdev: *mut qaic_device, usr: *mut qaic_user);
    pub fn dbc_irq_threaded_fn(irq: i32, data: *mut c_void) -> irqreturn_t;
    pub fn dbc_irq_handler(irq: i32, data: *mut c_void) -> irqreturn_t;
    pub fn disable_dbc(qdev: *mut qaic_device, dbc_id: u32, usr: *mut qaic_user) -> i32;
    pub fn enable_dbc(qdev: *mut qaic_device, dbc_id: u32, usr: *mut qaic_user);
    pub fn wakeup_dbc(qdev: *mut qaic_device, dbc_id: u32);
    pub fn release_dbc(qdev: *mut qaic_device, dbc_id: u32);
    pub fn qaic_data_get_fifo_info(dbc: *mut dma_bridge_chan, head: *mut u32, tail: *mut u32);
    pub fn wake_all_cntl(qdev: *mut qaic_device);
    pub fn qaic_dev_reset_clean_local_state(qdev: *mut qaic_device);
    pub fn qaic_gem_prime_import(dev: *mut drm_device, dma_buf: *mut dma_buf) -> *mut drm_gem_object;
    pub fn qaic_create_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> i32;
    pub fn qaic_mmap_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> i32;
    pub fn qaic_attach_slice_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> i32;
    pub fn qaic_execute_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> i32;
    pub fn qaic_partial_execute_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> i32;
    pub fn qaic_wait_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> i32;
    pub fn qaic_perf_stats_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> i32;
    pub fn qaic_detach_slice_bo_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> i32;
    pub fn qaic_irq_polling_work(work: *mut work_struct);
    pub fn qaic_dbc_enter_ssr(qdev: *mut qaic_device, dbc_id: u32);
    pub fn qaic_dbc_exit_ssr(qdev: *mut qaic_device);
    pub fn qaic_sysfs_init(qddev: *mut qaic_drm_device) -> i32;
    pub fn qaic_sysfs_remove(qddev: *mut qaic_drm_device);
    pub fn set_dbc_state(qdev: *mut qaic_device, dbc_id: u32, state: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
