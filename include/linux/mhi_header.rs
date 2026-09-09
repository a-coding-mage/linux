/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/mhi.h. Kernel-provided types and macros remain external dependencies. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

pub const MHI_MAX_OEM_PK_HASH_SEGMENTS: usize = 16;

pub type dma_addr_t = u64;
pub type u8 = core::primitive::u8;
pub type u32 = core::primitive::u32;
pub type dma_data_direction = u32;

pub const DMA_TO_DEVICE: dma_data_direction = 1;
pub const DMA_FROM_DEVICE: dma_data_direction = 2;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct mhi_device_id { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct mhi_chan { _private: [u8; 0] }
#[repr(C)] pub struct mhi_event { _private: [u8; 0] }
#[repr(C)] pub struct mhi_ctxt { _private: [u8; 0] }
#[repr(C)] pub struct mhi_cmd { _private: [u8; 0] }
#[repr(C)] pub struct mhi_buf_info { _private: [u8; 0] }
#[repr(C)] pub struct bhi_vec_entry { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct rwlock_t { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }

pub const fn BIT(n: u32) -> u32 { 1u32 << n }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mhi_callback { MHI_CB_IDLE, MHI_CB_PENDING_DATA, MHI_CB_LPM_ENTER, MHI_CB_LPM_EXIT, MHI_CB_EE_RDDM, MHI_CB_EE_MISSION_MODE, MHI_CB_SYS_ERROR, MHI_CB_FATAL_ERROR, MHI_CB_BW_REQ }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mhi_flags { MHI_EOB = 1, MHI_EOT = 2, MHI_CHAIN = 4 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mhi_device_type { MHI_DEVICE_XFER, MHI_DEVICE_CONTROLLER }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mhi_ch_type { MHI_CH_TYPE_INVALID = 0, MHI_CH_TYPE_OUTBOUND = DMA_TO_DEVICE as isize, MHI_CH_TYPE_INBOUND = DMA_FROM_DEVICE as isize, MHI_CH_TYPE_INBOUND_COALESCED = 3 }

#[repr(C)]
pub struct mhi_buf { pub buf: *mut c_void, pub name: *const core::ffi::c_char, pub dma_addr: dma_addr_t, pub len: usize }

#[repr(C)]
pub struct image_info { pub bhi_vec: *mut bhi_vec_entry, pub entries: u32, pub mhi_buf: [mhi_buf; 0] }

#[repr(C)] pub struct mhi_link_info { pub target_link_speed: core::ffi::c_uint, pub target_link_width: core::ffi::c_uint }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mhi_ee_type { MHI_EE_PBL, MHI_EE_SBL, MHI_EE_AMSS, MHI_EE_RDDM, MHI_EE_WFW, MHI_EE_PTHRU, MHI_EE_EDL, MHI_EE_FP, MHI_EE_MAX_SUPPORTED = 7, MHI_EE_DISABLE_TRANSITION, MHI_EE_NOT_SUPPORTED, MHI_EE_MAX }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mhi_state { MHI_STATE_RESET = 0, MHI_STATE_READY = 1, MHI_STATE_M0 = 2, MHI_STATE_M1 = 3, MHI_STATE_M2 = 4, MHI_STATE_M3 = 5, MHI_STATE_M3_FAST = 6, MHI_STATE_BHI = 7, MHI_STATE_SYS_ERR = 0xFF, MHI_STATE_MAX }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mhi_ch_ee_mask { MHI_CH_EE_PBL = 1, MHI_CH_EE_SBL = 2, MHI_CH_EE_AMSS = 4, MHI_CH_EE_RDDM = 8, MHI_CH_EE_PTHRU = 32, MHI_CH_EE_WFW = 16, MHI_CH_EE_EDL = 64 }

#[repr(C)] #[derive(Copy, Clone)] pub enum mhi_er_data_type { MHI_ER_DATA, MHI_ER_CTRL }
#[repr(C)] #[derive(Copy, Clone)] pub enum mhi_db_brst_mode { MHI_DB_BRST_DISABLE = 2, MHI_DB_BRST_ENABLE = 3 }

#[repr(C)]
pub struct mhi_channel_config { pub name: *mut core::ffi::c_char, pub num: u32, pub num_elements: u32, pub local_elements: u32, pub event_ring: u32, pub dir: dma_data_direction, pub type_: mhi_ch_type, pub ee_mask: u32, pub pollcfg: u32, pub doorbell: mhi_db_brst_mode, pub lpm_notify: bool, pub offload_channel: bool, pub doorbell_mode_switch: bool, pub wake_capable: bool }

#[repr(C)]
pub struct mhi_event_config { pub num_elements: u32, pub irq_moderation_ms: u32, pub irq: u32, pub channel: u32, pub priority: u32, pub mode: mhi_db_brst_mode, pub data_type: mhi_er_data_type, pub hardware_event: bool, pub client_managed: bool, pub offload_channel: bool }

#[repr(C)]
pub struct mhi_controller_config { pub max_channels: u32, pub timeout_ms: u32, pub ready_timeout_ms: u32, pub buf_len: u32, pub num_channels: u32, pub ch_cfg: *const mhi_channel_config, pub num_events: u32, pub event_cfg: *mut mhi_event_config, pub use_bounce_buf: bool, pub m2_no_db: bool }

#[repr(C)]
pub struct mhi_controller {
    pub name: *const core::ffi::c_char, pub cntrl_dev: *mut device, pub mhi_dev: *mut mhi_device, pub debugfs_dentry: *mut dentry, pub regs: *mut c_void, pub bhi: *mut c_void, pub bhie: *mut c_void, pub wake_db: *mut c_void,
    pub iova_start: dma_addr_t, pub iova_stop: dma_addr_t, pub fw_image: *const core::ffi::c_char, pub fw_data: *const u8, pub fw_sz: usize, pub edl_image: *const core::ffi::c_char, pub rddm_size: usize, pub sbl_size: usize, pub seg_len: usize, pub reg_len: usize, pub fbc_image: *mut image_info, pub rddm_image: *mut image_info, pub mhi_chan: *mut mhi_chan, pub lpm_chans: list_head, pub irq: *mut i32, pub max_chan: u32, pub total_ev_rings: u32, pub hw_ev_rings: u32, pub sw_ev_rings: u32, pub nr_irqs: u32, pub serial_number: u32,
    pub mhi_event: *mut mhi_event, pub mhi_cmd: *mut mhi_cmd, pub mhi_ctxt: *mut mhi_ctxt, pub pm_mutex: mutex, pub pm_lock: rwlock_t, pub timeout_ms: u32, pub ready_timeout_ms: u32, pub pm_state: u32, pub db_access: u32, pub ee: mhi_ee_type, pub dev_state: mhi_state, pub dev_wake: atomic_t, pub pending_pkts: atomic_t, pub M0: u32, pub M2: u32, pub M3: u32, pub transition_list: list_head, pub transition_lock: spinlock_t, pub wlock: spinlock_t, pub mhi_link_info: mhi_link_info, pub st_worker: work_struct, pub hiprio_wq: *mut workqueue_struct, pub state_event: wait_queue_head_t,
    pub status_cb: Option<unsafe extern "C" fn(*mut mhi_controller, mhi_callback)>, pub wake_get: Option<unsafe extern "C" fn(*mut mhi_controller, bool)>, pub wake_put: Option<unsafe extern "C" fn(*mut mhi_controller, bool)>, pub wake_toggle: Option<unsafe extern "C" fn(*mut mhi_controller)>, pub runtime_get: Option<unsafe extern "C" fn(*mut mhi_controller) -> i32>, pub runtime_put: Option<unsafe extern "C" fn(*mut mhi_controller)>, pub map_single: Option<unsafe extern "C" fn(*mut mhi_controller, *mut mhi_buf_info) -> i32>, pub unmap_single: Option<unsafe extern "C" fn(*mut mhi_controller, *mut mhi_buf_info)>, pub read_reg: Option<unsafe extern "C" fn(*mut mhi_controller, *mut c_void, *mut u32) -> i32>, pub write_reg: Option<unsafe extern "C" fn(*mut mhi_controller, *mut c_void, u32)>, pub reset: Option<unsafe extern "C" fn(*mut mhi_controller)>, pub edl_trigger: Option<unsafe extern "C" fn(*mut mhi_controller) -> i32>,
    pub buffer_len: usize, pub index: i32, pub bounce_buf: bool, pub fbc_download: bool, pub wake_set: bool, pub no_m3: bool, pub irq_flags: usize, pub mru: u32,
}

#[repr(C)]
pub struct mhi_device { pub id: *const mhi_device_id, pub name: *const core::ffi::c_char, pub mhi_cntrl: *mut mhi_controller, pub ul_chan: *mut mhi_chan, pub dl_chan: *mut mhi_chan, pub dev: device, pub dev_type: mhi_device_type, pub ul_chan_id: i32, pub dl_chan_id: i32, pub dev_wake: u32 }

#[repr(C)] pub struct mhi_result { pub buf_addr: *mut c_void, pub bytes_xferd: usize, pub dir: dma_data_direction, pub transaction_status: i32 }

#[repr(C)]
pub struct mhi_driver { pub id_table: *const mhi_device_id, pub probe: Option<unsafe extern "C" fn(*mut mhi_device, *const mhi_device_id) -> i32>, pub remove: Option<unsafe extern "C" fn(*mut mhi_device)>, pub ul_xfer_cb: Option<unsafe extern "C" fn(*mut mhi_device, *mut mhi_result)>, pub dl_xfer_cb: Option<unsafe extern "C" fn(*mut mhi_device, *mut mhi_result)>, pub status_cb: Option<unsafe extern "C" fn(*mut mhi_device, mhi_callback)>, pub driver: device_driver }

// C container_of/container_of_const helpers are kernel macros; preserve their intent.
pub unsafe fn to_mhi_driver(drv: *mut device_driver) -> *mut mhi_driver { (drv as *mut u8).sub(core::mem::offset_of!(mhi_driver, driver)) as *mut mhi_driver }
pub unsafe fn to_mhi_device(dev: *mut device) -> *mut mhi_device { (dev as *mut u8).sub(core::mem::offset_of!(mhi_device, dev)) as *mut mhi_device }

extern "C" {
    pub fn mhi_alloc_controller() -> *mut mhi_controller;
    pub fn mhi_free_controller(mhi_cntrl: *mut mhi_controller);
    pub fn mhi_register_controller(mhi_cntrl: *mut mhi_controller, config: *const mhi_controller_config) -> i32;
    pub fn mhi_unregister_controller(mhi_cntrl: *mut mhi_controller);
    pub fn __mhi_driver_register(mhi_drv: *mut mhi_driver, owner: *mut module) -> i32;
    pub fn mhi_driver_unregister(mhi_drv: *mut mhi_driver);
    pub fn mhi_set_mhi_state(mhi_cntrl: *mut mhi_controller, state: mhi_state);
    pub fn mhi_notify(mhi_dev: *mut mhi_device, cb_reason: mhi_callback);
    pub fn mhi_get_free_desc_count(mhi_dev: *mut mhi_device, dir: dma_data_direction) -> i32;
    pub fn mhi_prepare_for_power_up(mhi_cntrl: *mut mhi_controller) -> i32;
    pub fn mhi_async_power_up(mhi_cntrl: *mut mhi_controller) -> i32;
    pub fn mhi_sync_power_up(mhi_cntrl: *mut mhi_controller) -> i32;
    pub fn mhi_power_down(mhi_cntrl: *mut mhi_controller, graceful: bool);
    pub fn mhi_power_down_keep_dev(mhi_cntrl: *mut mhi_controller, graceful: bool);
    pub fn mhi_unprepare_after_power_down(mhi_cntrl: *mut mhi_controller);
    pub fn mhi_pm_suspend(mhi_cntrl: *mut mhi_controller) -> i32;
    pub fn mhi_pm_resume(mhi_cntrl: *mut mhi_controller) -> i32;
    pub fn mhi_pm_resume_force(mhi_cntrl: *mut mhi_controller) -> i32;
    pub fn mhi_download_rddm_image(mhi_cntrl: *mut mhi_controller, in_panic: bool) -> i32;
    pub fn mhi_force_rddm_mode(mhi_cntrl: *mut mhi_controller) -> i32;
    pub fn mhi_get_exec_env(mhi_cntrl: *mut mhi_controller) -> mhi_ee_type;
    pub fn mhi_get_mhi_state(mhi_cntrl: *mut mhi_controller) -> mhi_state;
    pub fn mhi_soc_reset(mhi_cntrl: *mut mhi_controller);
    pub fn mhi_device_get_sync(mhi_dev: *mut mhi_device) -> i32;
    pub fn mhi_device_put(mhi_dev: *mut mhi_device);
    pub fn mhi_prepare_for_transfer(mhi_dev: *mut mhi_device) -> i32;
    pub fn mhi_unprepare_from_transfer(mhi_dev: *mut mhi_device);
    pub fn mhi_queue_buf(mhi_dev: *mut mhi_device, dir: dma_data_direction, buf: *mut c_void, len: usize, mflags: mhi_flags) -> i32;
    pub fn mhi_queue_skb(mhi_dev: *mut mhi_device, dir: dma_data_direction, skb: *mut sk_buff, len: usize, mflags: mhi_flags) -> i32;
    pub fn mhi_queue_is_full(mhi_dev: *mut mhi_device, dir: dma_data_direction) -> bool;
    pub fn mhi_get_channel_doorbell_offset(mhi_cntrl: *mut mhi_controller, chdb_offset: *mut u32) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
