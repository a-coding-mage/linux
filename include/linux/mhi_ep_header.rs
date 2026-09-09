/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2022, Linaro Ltd.
 *
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented in this header translation.

pub const MHI_EP_DEFAULT_MTU: u32 = 0x8000;

#[repr(C)]
pub struct mhi_ep_channel_config {
    pub name: *mut ::std::ffi::c_char,
    pub num: u32,
    pub num_elements: u32,
    pub dir: dma_data_direction,
}

#[repr(C)]
pub struct mhi_ep_cntrl_config {
    pub mhi_version: u32,
    pub max_channels: u32,
    pub num_channels: u32,
    pub ch_cfg: *const mhi_ep_channel_config,
}

#[repr(C)]
pub struct mhi_ep_db_info {
    pub mask: u32,
    pub status: u32,
}

#[repr(C)]
pub struct mhi_ep_buf_info {
    pub mhi_dev: *mut mhi_ep_device,
    pub dev_addr: *mut ::std::ffi::c_void,
    pub host_addr: u64,
    pub size: usize,
    pub code: ::std::ffi::c_int,
    pub cb: Option<unsafe extern "C" fn(buf_info: *mut mhi_ep_buf_info)>,
    pub cb_buf: *mut ::std::ffi::c_void,
}

#[repr(C)]
pub struct mhi_ep_cntrl {
    pub cntrl_dev: *mut device,
    pub mhi_dev: *mut mhi_ep_device,
    pub mmio: *mut ::std::ffi::c_void,
    pub mhi_chan: *mut mhi_ep_chan,
    pub mhi_event: *mut mhi_ep_event,
    pub mhi_cmd: *mut mhi_ep_cmd,
    pub sm: *mut mhi_ep_sm,
    pub ch_ctx_cache: *mut mhi_chan_ctxt,
    pub ev_ctx_cache: *mut mhi_event_ctxt,
    pub cmd_ctx_cache: *mut mhi_cmd_ctxt,
    pub ch_ctx_host_pa: u64,
    pub ev_ctx_host_pa: u64,
    pub cmd_ctx_host_pa: u64,
    pub ch_ctx_cache_phys: phys_addr_t,
    pub ev_ctx_cache_phys: phys_addr_t,
    pub cmd_ctx_cache_phys: phys_addr_t,
    pub chdb: [mhi_ep_db_info; 4],
    pub event_lock: mutex,
    pub state_lock: mutex,
    pub list_lock: spinlock_t,
    pub st_transition_list: list_head,
    pub ch_db_list: list_head,
    pub wq: *mut workqueue_struct,
    pub state_work: work_struct,
    pub reset_work: work_struct,
    pub cmd_ring_work: work_struct,
    pub ch_ring_work: work_struct,
    pub ring_item_cache: *mut kmem_cache,
    pub ev_ring_el_cache: *mut kmem_cache,
    pub tre_buf_cache: *mut kmem_cache,
    pub raise_irq: Option<unsafe extern "C" fn(*mut mhi_ep_cntrl, u32)>,
    pub alloc_map: Option<unsafe extern "C" fn(*mut mhi_ep_cntrl, u64, *mut phys_addr_t, *mut *mut ::std::ffi::c_void, usize) -> ::std::ffi::c_int>,
    pub unmap_free: Option<unsafe extern "C" fn(*mut mhi_ep_cntrl, u64, phys_addr_t, *mut ::std::ffi::c_void, usize)>,
    pub read_sync: Option<unsafe extern "C" fn(*mut mhi_ep_cntrl, *mut mhi_ep_buf_info) -> ::std::ffi::c_int>,
    pub write_sync: Option<unsafe extern "C" fn(*mut mhi_ep_cntrl, *mut mhi_ep_buf_info) -> ::std::ffi::c_int>,
    pub read_async: Option<unsafe extern "C" fn(*mut mhi_ep_cntrl, *mut mhi_ep_buf_info) -> ::std::ffi::c_int>,
    pub write_async: Option<unsafe extern "C" fn(*mut mhi_ep_cntrl, *mut mhi_ep_buf_info) -> ::std::ffi::c_int>,
    pub flush_async: Option<unsafe extern "C" fn(*mut mhi_ep_cntrl)>,
    pub mhi_state: mhi_state,
    pub max_chan: u32,
    pub mru: u32,
    pub event_rings: u32,
    pub hw_event_rings: u32,
    pub chdb_offset: u32,
    pub erdb_offset: u32,
    pub index: u32,
    pub irq: ::std::ffi::c_int,
    pub enabled: bool,
}

#[repr(C)]
pub struct mhi_ep_device {
    pub dev: device,
    pub mhi_cntrl: *mut mhi_ep_cntrl,
    pub id: *const mhi_device_id,
    pub name: *const ::std::ffi::c_char,
    pub ul_chan: *mut mhi_ep_chan,
    pub dl_chan: *mut mhi_ep_chan,
    pub dev_type: mhi_device_type,
}

#[repr(C)]
pub struct mhi_ep_driver {
    pub id_table: *const mhi_device_id,
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut mhi_ep_device, *const mhi_device_id) -> ::std::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut mhi_ep_device)>,
    pub ul_xfer_cb: Option<unsafe extern "C" fn(*mut mhi_ep_device, *mut mhi_result)>,
    pub dl_xfer_cb: Option<unsafe extern "C" fn(*mut mhi_ep_device, *mut mhi_result)>,
}

// Equivalent helper macros; container_of semantics are provided by the kernel bindings.
#[macro_export]
macro_rules! to_mhi_ep_device { ($dev:expr) => { container_of!($dev, mhi_ep_device, dev) }; }
#[macro_export]
macro_rules! to_mhi_ep_driver { ($drv:expr) => { container_of_const!($drv, mhi_ep_driver, driver) }; }

// Helper macro for drivers using the default registration and unregistration.
#[macro_export]
macro_rules! module_mhi_ep_driver {
    ($mhi_drv:expr) => { module_driver!($mhi_drv, mhi_ep_driver_register, mhi_ep_driver_unregister) };
}

// Macro equivalent of mhi_ep_driver_register(mhi_drv), with THIS_MODULE supplied
// by the surrounding kernel environment.
#[macro_export]
macro_rules! mhi_ep_driver_register {
    ($mhi_drv:expr) => { __mhi_ep_driver_register($mhi_drv, THIS_MODULE) };
}

extern "C" {
    pub fn __mhi_ep_driver_register(mhi_drv: *mut mhi_ep_driver, owner: *mut module) -> ::std::ffi::c_int;
    pub fn mhi_ep_driver_unregister(mhi_drv: *mut mhi_ep_driver);
    pub fn mhi_ep_register_controller(mhi_cntrl: *mut mhi_ep_cntrl, config: *const mhi_ep_cntrl_config) -> ::std::ffi::c_int;
    pub fn mhi_ep_unregister_controller(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_power_up(mhi_cntrl: *mut mhi_ep_cntrl) -> ::std::ffi::c_int;
    pub fn mhi_ep_power_down(mhi_cntrl: *mut mhi_ep_cntrl);
    pub fn mhi_ep_queue_is_empty(mhi_dev: *mut mhi_ep_device, dir: dma_data_direction) -> bool;
    pub fn mhi_ep_queue_skb(mhi_dev: *mut mhi_ep_device, skb: *mut sk_buff) -> ::std::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
