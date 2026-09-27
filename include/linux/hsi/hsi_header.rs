/* SPDX-License-Identifier: GPL-2.0-only */
/* HSI core header file. */

// C dependencies: linux/device.h, linux/mutex.h, linux/scatterlist.h,
// linux/list.h, linux/module.h, and linux/notifier.h.

pub const HSI_MSG_READ: u32 = 0;
pub const HSI_MSG_WRITE: u32 = 1;

pub const HSI_MODE_STREAM: u32 = 1;
pub const HSI_MODE_FRAME: u32 = 2;

pub const HSI_FLOW_SYNC: u32 = 0;
pub const HSI_FLOW_PIPE: u32 = 1;

pub const HSI_ARB_RR: u32 = 0;
pub const HSI_ARB_PRIO: u32 = 1;

pub const HSI_MAX_CHANNELS: usize = 16;

pub const HSI_STATUS_COMPLETED: u32 = 0;
pub const HSI_STATUS_PENDING: u32 = 1;
pub const HSI_STATUS_PROCEEDING: u32 = 2;
pub const HSI_STATUS_QUEUED: u32 = 3;
pub const HSI_STATUS_ERROR: u32 = 4;

pub const HSI_EVENT_START_RX: u32 = 0;
pub const HSI_EVENT_STOP_RX: u32 = 1;

#[repr(C)]
pub struct hsi_channel {
    pub id: ::kernel::ffi::c_uint,
    pub name: *const ::kernel::ffi::c_char,
}

#[repr(C)]
pub union hsi_config__bindgen_ty_1 {
    pub flow: ::kernel::ffi::c_uint,
    pub arb_mode: ::kernel::ffi::c_uint,
}

#[repr(C)]
pub struct hsi_config {
    pub mode: ::kernel::ffi::c_uint,
    pub channels: *mut hsi_channel,
    pub num_channels: ::kernel::ffi::c_uint,
    pub num_hw_channels: ::kernel::ffi::c_uint,
    pub speed: ::kernel::ffi::c_uint,
    pub __bindgen_anon_1: hsi_config__bindgen_ty_1,
}

#[repr(C)]
pub struct hsi_board_info {
    pub name: *const ::kernel::ffi::c_char,
    pub hsi_id: ::kernel::ffi::c_uint,
    pub port: ::kernel::ffi::c_uint,
    pub tx_cfg: hsi_config,
    pub rx_cfg: hsi_config,
    pub platform_data: *mut ::kernel::ffi::c_void,
    pub archdata: *mut dev_archdata,
}

#[cfg(CONFIG_HSI_BOARDINFO)]
unsafe extern "C" {
    pub fn hsi_register_board_info(
        info: *const hsi_board_info,
        len: ::kernel::ffi::c_uint,
    ) -> ::kernel::ffi::c_int;
}

#[cfg(not(CONFIG_HSI_BOARDINFO))]
#[inline]
pub unsafe fn hsi_register_board_info(
    _info: *const hsi_board_info,
    _len: ::kernel::ffi::c_uint,
) -> ::kernel::ffi::c_int {
    0
}

#[repr(C)]
pub struct hsi_client {
    pub device: device,
    pub tx_cfg: hsi_config,
    pub rx_cfg: hsi_config,
    pub ehandler: Option<unsafe extern "C" fn(*mut hsi_client, ::kernel::ffi::c_ulong)>,
    pub pclaimed: ::kernel::ffi::c_uint,
    pub nb: notifier_block,
}

#[macro_export]
macro_rules! to_hsi_client {
    ($dev:expr) => { container_of!($dev, hsi_client, device) };
}

#[inline]
pub unsafe fn hsi_client_set_drvdata(cl: *mut hsi_client, data: *mut ::kernel::ffi::c_void) {
    dev_set_drvdata(&mut (*cl).device, data);
}

#[inline]
pub unsafe fn hsi_client_drvdata(cl: *mut hsi_client) -> *mut ::kernel::ffi::c_void {
    dev_get_drvdata(&mut (*cl).device)
}

unsafe extern "C" {
    pub fn hsi_register_port_event(
        cl: *mut hsi_client,
        handler: Option<unsafe extern "C" fn(*mut hsi_client, ::kernel::ffi::c_ulong)>,
    ) -> ::kernel::ffi::c_int;
    pub fn hsi_unregister_port_event(cl: *mut hsi_client) -> ::kernel::ffi::c_int;
}

#[repr(C)]
pub struct hsi_client_driver {
    pub driver: device_driver,
}

#[macro_export]
macro_rules! to_hsi_client_driver {
    ($drv:expr) => { container_of!($drv, hsi_client_driver, driver) };
}

unsafe extern "C" {
    pub fn hsi_register_client_driver(drv: *mut hsi_client_driver) -> ::kernel::ffi::c_int;
}

#[inline]
pub unsafe fn hsi_unregister_client_driver(drv: *mut hsi_client_driver) {
    driver_unregister(&mut (*drv).driver);
}

#[repr(C)]
pub struct hsi_msg {
    pub link: list_head,
    pub cl: *mut hsi_client,
    pub sgt: sg_table,
    pub context: *mut ::kernel::ffi::c_void,
    pub complete: Option<unsafe extern "C" fn(*mut hsi_msg)>,
    pub destructor: Option<unsafe extern "C" fn(*mut hsi_msg)>,
    pub status: ::kernel::ffi::c_int,
    pub actual_len: ::kernel::ffi::c_uint,
    pub channel: ::kernel::ffi::c_uint,
    pub ttype: ::kernel::ffi::c_uint,
    pub break_frame: ::kernel::ffi::c_uint,
}

unsafe extern "C" {
    pub fn hsi_alloc_msg(n_frag: ::kernel::ffi::c_uint, flags: gfp_t) -> *mut hsi_msg;
    pub fn hsi_free_msg(msg: *mut hsi_msg);
}

#[repr(C)]
pub struct hsi_port {
    pub device: device,
    pub tx_cfg: hsi_config,
    pub rx_cfg: hsi_config,
    pub num: ::kernel::ffi::c_uint,
    pub shared: ::kernel::ffi::c_uint,
    pub claimed: ::kernel::ffi::c_int,
    pub lock: mutex,
    pub async_: Option<unsafe extern "C" fn(*mut hsi_msg) -> ::kernel::ffi::c_int>,
    pub setup: Option<unsafe extern "C" fn(*mut hsi_client) -> ::kernel::ffi::c_int>,
    pub flush: Option<unsafe extern "C" fn(*mut hsi_client) -> ::kernel::ffi::c_int>,
    pub start_tx: Option<unsafe extern "C" fn(*mut hsi_client) -> ::kernel::ffi::c_int>,
    pub stop_tx: Option<unsafe extern "C" fn(*mut hsi_client) -> ::kernel::ffi::c_int>,
    pub release: Option<unsafe extern "C" fn(*mut hsi_client) -> ::kernel::ffi::c_int>,
    pub n_head: blocking_notifier_head,
}

#[macro_export]
macro_rules! to_hsi_port {
    ($dev:expr) => { container_of!($dev, hsi_port, device) };
}

#[macro_export]
macro_rules! hsi_get_port {
    ($cl:expr) => { to_hsi_port!((*$cl).device.parent) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
