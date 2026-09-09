/* SPDX-License-Identifier: GPL-2.0 */
/* most.h - API for component and adapter drivers */

// C dependencies supplied by the surrounding kernel translation:
// linux/types.h, linux/device.h

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum most_interface_type {
    ITYPE_LOOPBACK = 1,
    ITYPE_I2C,
    ITYPE_I2S,
    ITYPE_TSI,
    ITYPE_HBI,
    ITYPE_MEDIALB_DIM,
    ITYPE_MEDIALB_DIM2,
    ITYPE_USB,
    ITYPE_PCIE,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum most_channel_direction {
    MOST_CH_RX = 1 << 0,
    MOST_CH_TX = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum most_channel_data_type {
    MOST_CH_CONTROL = 1 << 0,
    MOST_CH_ASYNC = 1 << 1,
    MOST_CH_ISOC = 1 << 2,
    MOST_CH_SYNC = 1 << 5,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum most_status_flags {
    MBO_SUCCESS = 0,
    MBO_E_INVAL,
    MBO_E_CLOSE,
}

#[repr(C)]
pub struct most_channel_capability {
    pub direction: u16,
    pub data_type: u16,
    pub num_buffers_packet: u16,
    pub buffer_size_packet: u16,
    pub num_buffers_streaming: u16,
    pub buffer_size_streaming: u16,
    pub name_suffix: *const core::ffi::c_char,
}

#[repr(C)]
pub struct most_channel_config {
    pub direction: most_channel_direction,
    pub data_type: most_channel_data_type,
    pub num_buffers: u16,
    pub buffer_size: u16,
    pub extra_len: u16,
    pub subbuffer_size: u16,
    pub packets_per_xact: u16,
    pub dbr_size: u16,
}

#[repr(C)]
pub struct mbo {
    pub context: *mut core::ffi::c_void,
    pub priv_: *mut core::ffi::c_void,
    pub list: list_head,
    pub ifp: *mut most_interface,
    pub num_buffers_ptr: *mut core::ffi::c_int,
    pub hdm_channel_id: u16,
    pub virt_address: *mut core::ffi::c_void,
    pub bus_address: dma_addr_t,
    pub buffer_length: u16,
    pub processed_length: u16,
    pub status: most_status_flags,
    pub complete: Option<unsafe extern "C" fn(mbo: *mut mbo)>,
}

#[repr(C)]
pub struct most_interface {
    pub dev: *mut device,
    pub driver_dev: *mut device,
    pub mod_: *mut module,
    pub interface: most_interface_type,
    pub description: *const core::ffi::c_char,
    pub num_channels: core::ffi::c_uint,
    pub channel_vector: *mut most_channel_capability,
    pub dma_alloc: Option<unsafe extern "C" fn(*mut mbo, u32) -> *mut core::ffi::c_void>,
    pub dma_free: Option<unsafe extern "C" fn(*mut mbo, u32)>,
    pub configure: Option<unsafe extern "C" fn(*mut most_interface, core::ffi::c_int, *mut most_channel_config) -> core::ffi::c_int>,
    pub enqueue: Option<unsafe extern "C" fn(*mut most_interface, core::ffi::c_int, *mut mbo) -> core::ffi::c_int>,
    pub poison_channel: Option<unsafe extern "C" fn(*mut most_interface, core::ffi::c_int) -> core::ffi::c_int>,
    pub request_netinfo: Option<unsafe extern "C" fn(*mut most_interface, core::ffi::c_int, Option<unsafe extern "C" fn(*mut most_interface, u8, *mut u8)>)>,
    pub priv_: *mut core::ffi::c_void,
    pub p: *mut interface_private,
}

#[repr(C)]
pub struct most_component {
    pub list: list_head,
    pub name: *const core::ffi::c_char,
    pub mod_: *mut module,
    pub probe_channel: Option<unsafe extern "C" fn(*mut most_interface, core::ffi::c_int, *mut most_channel_config, *mut core::ffi::c_char, *mut core::ffi::c_char) -> core::ffi::c_int>,
    pub disconnect_channel: Option<unsafe extern "C" fn(*mut most_interface, core::ffi::c_int) -> core::ffi::c_int>,
    pub rx_completion: Option<unsafe extern "C" fn(*mut mbo) -> core::ffi::c_int>,
    pub tx_completion: Option<unsafe extern "C" fn(*mut most_interface, core::ffi::c_int) -> core::ffi::c_int>,
    pub cfg_complete: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
}

extern "C" {
    pub fn most_register_interface(iface: *mut most_interface) -> core::ffi::c_int;
    pub fn most_deregister_interface(iface: *mut most_interface);
    pub fn most_submit_mbo(mbo: *mut mbo);
    pub fn most_stop_enqueue(iface: *mut most_interface, channel_idx: core::ffi::c_int);
    pub fn most_resume_enqueue(iface: *mut most_interface, channel_idx: core::ffi::c_int);
    pub fn most_register_component(comp: *mut most_component) -> core::ffi::c_int;
    pub fn most_deregister_component(comp: *mut most_component) -> core::ffi::c_int;
    pub fn most_get_mbo(iface: *mut most_interface, channel_idx: core::ffi::c_int, comp: *mut most_component) -> *mut mbo;
    pub fn most_put_mbo(mbo: *mut mbo);
    pub fn channel_has_mbo(iface: *mut most_interface, channel_idx: core::ffi::c_int, comp: *mut most_component) -> core::ffi::c_int;
    pub fn most_start_channel(iface: *mut most_interface, channel_idx: core::ffi::c_int, comp: *mut most_component) -> core::ffi::c_int;
    pub fn most_stop_channel(iface: *mut most_interface, channel_idx: core::ffi::c_int, comp: *mut most_component) -> core::ffi::c_int;
    pub fn configfs_init() -> core::ffi::c_int;
    pub fn most_register_configfs_subsys(comp: *mut most_component) -> core::ffi::c_int;
    pub fn most_deregister_configfs_subsys(comp: *mut most_component);
    pub fn most_add_link(mdev: *mut core::ffi::c_char, mdev_ch: *mut core::ffi::c_char, comp_name: *mut core::ffi::c_char, link_name: *mut core::ffi::c_char, comp_param: *mut core::ffi::c_char) -> core::ffi::c_int;
    pub fn most_remove_link(mdev: *mut core::ffi::c_char, mdev_ch: *mut core::ffi::c_char, comp_name: *mut core::ffi::c_char) -> core::ffi::c_int;
    pub fn most_set_cfg_buffer_size(mdev: *mut core::ffi::c_char, mdev_ch: *mut core::ffi::c_char, val: u16) -> core::ffi::c_int;
    pub fn most_set_cfg_subbuffer_size(mdev: *mut core::ffi::c_char, mdev_ch: *mut core::ffi::c_char, val: u16) -> core::ffi::c_int;
    pub fn most_set_cfg_dbr_size(mdev: *mut core::ffi::c_char, mdev_ch: *mut core::ffi::c_char, val: u16) -> core::ffi::c_int;
    pub fn most_set_cfg_num_buffers(mdev: *mut core::ffi::c_char, mdev_ch: *mut core::ffi::c_char, val: u16) -> core::ffi::c_int;
    pub fn most_set_cfg_datatype(mdev: *mut core::ffi::c_char, mdev_ch: *mut core::ffi::c_char, buf: *mut core::ffi::c_char) -> core::ffi::c_int;
    pub fn most_set_cfg_direction(mdev: *mut core::ffi::c_char, mdev_ch: *mut core::ffi::c_char, buf: *mut core::ffi::c_char) -> core::ffi::c_int;
    pub fn most_set_cfg_packets_xact(mdev: *mut core::ffi::c_char, mdev_ch: *mut core::ffi::c_char, val: u16) -> core::ffi::c_int;
    pub fn most_cfg_complete(comp_name: *mut core::ffi::c_char) -> core::ffi::c_int;
    pub fn most_interface_register_notify(mdev_name: *const core::ffi::c_char);
}

// External C/kernel types supplied by other translated headers.
#[allow(non_camel_case_types)]
pub type dma_addr_t = usize;
pub enum list_head {}
pub enum device {}
pub enum module {}
pub enum interface_private {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
