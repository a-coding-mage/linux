/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the corresponding Linux headers.

#[repr(C)]
pub struct iio_chan_spec {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}
#[repr(C)]
pub struct iio_backend {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct iio_dev {
    _private: [u8; 0],
}
#[repr(C)]
pub struct iio_buffer {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum iio_backend_data_type {
    IIO_BACKEND_TWOS_COMPLEMENT = 0,
    IIO_BACKEND_OFFSET_BINARY = 1,
    IIO_BACKEND_DATA_UNSIGNED = 2,
    IIO_BACKEND_DATA_TYPE_MAX = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum iio_backend_data_source {
    IIO_BACKEND_INTERNAL_CONTINUOUS_WAVE = 0,
    IIO_BACKEND_EXTERNAL = 1,
    IIO_BACKEND_INTERNAL_RAMP_16BIT = 2,
    IIO_BACKEND_DATA_SOURCE_MAX = 3,
}

#[repr(C)]
pub struct iio_backend_data_fmt {
    pub type_: iio_backend_data_type,
    pub sign_extend: bool,
    pub enable: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum iio_backend_test_pattern {
    IIO_BACKEND_NO_TEST_PATTERN = 0,
    IIO_BACKEND_ADI_PRBS_9A = 32,
    IIO_BACKEND_ADI_PRBS_23A = 33,
    IIO_BACKEND_TEST_PATTERN_MAX = 34,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum iio_backend_sample_trigger {
    IIO_BACKEND_SAMPLE_TRIGGER_EDGE_FALLING = 0,
    IIO_BACKEND_SAMPLE_TRIGGER_EDGE_RISING = 1,
    IIO_BACKEND_SAMPLE_TRIGGER_MAX = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum iio_backend_interface_type {
    IIO_BACKEND_INTERFACE_SERIAL_LVDS = 0,
    IIO_BACKEND_INTERFACE_SERIAL_CMOS = 1,
    IIO_BACKEND_INTERFACE_MAX = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum iio_backend_filter_type {
    IIO_BACKEND_FILTER_TYPE_DISABLED = 0,
    IIO_BACKEND_FILTER_TYPE_SINC1 = 1,
    IIO_BACKEND_FILTER_TYPE_SINC5 = 2,
    IIO_BACKEND_FILTER_TYPE_SINC5_PLUS_COMP = 3,
    IIO_BACKEND_FILTER_TYPE_MAX = 4,
}

pub const IIO_BACKEND_CAP_CALIBRATION: u32 = 1 << 0;
pub const IIO_BACKEND_CAP_BUFFER: u32 = 1 << 1;
pub const IIO_BACKEND_CAP_ENABLE: u32 = 1 << 2;

#[repr(C)]
pub struct iio_backend_ops {
    pub enable: Option<unsafe extern "C" fn(*mut iio_backend) -> i32>,
    pub disable: Option<unsafe extern "C" fn(*mut iio_backend)>,
    pub chan_enable: Option<unsafe extern "C" fn(*mut iio_backend, u32) -> i32>,
    pub chan_disable: Option<unsafe extern "C" fn(*mut iio_backend, u32) -> i32>,
    pub data_format_set: Option<unsafe extern "C" fn(*mut iio_backend, u32, *const iio_backend_data_fmt) -> i32>,
    pub data_source_set: Option<unsafe extern "C" fn(*mut iio_backend, u32, iio_backend_data_source) -> i32>,
    pub data_source_get: Option<unsafe extern "C" fn(*mut iio_backend, u32, *mut iio_backend_data_source) -> i32>,
    pub set_sample_rate: Option<unsafe extern "C" fn(*mut iio_backend, u32, u64) -> i32>,
    pub test_pattern_set: Option<unsafe extern "C" fn(*mut iio_backend, u32, iio_backend_test_pattern) -> i32>,
    pub chan_status: Option<unsafe extern "C" fn(*mut iio_backend, u32, *mut bool) -> i32>,
    pub iodelay_set: Option<unsafe extern "C" fn(*mut iio_backend, u32, u32) -> i32>,
    pub data_sample_trigger: Option<unsafe extern "C" fn(*mut iio_backend, iio_backend_sample_trigger) -> i32>,
    pub request_buffer: Option<unsafe extern "C" fn(*mut iio_backend, *mut iio_dev) -> *mut iio_buffer>,
    pub free_buffer: Option<unsafe extern "C" fn(*mut iio_backend, *mut iio_buffer)>,
    pub extend_chan_spec: Option<unsafe extern "C" fn(*mut iio_backend, *mut iio_chan_spec) -> i32>,
    pub ext_info_set: Option<unsafe extern "C" fn(*mut iio_backend, usize, *const iio_chan_spec, *const i8, usize) -> i32>,
    pub ext_info_get: Option<unsafe extern "C" fn(*mut iio_backend, usize, *const iio_chan_spec, *mut i8) -> i32>,
    pub interface_type_get: Option<unsafe extern "C" fn(*mut iio_backend, *mut iio_backend_interface_type) -> i32>,
    pub data_size_set: Option<unsafe extern "C" fn(*mut iio_backend, u32) -> i32>,
    pub oversampling_ratio_set: Option<unsafe extern "C" fn(*mut iio_backend, u32, u32) -> i32>,
    pub read_raw: Option<unsafe extern "C" fn(*mut iio_backend, *const iio_chan_spec, *mut i32, *mut i32, isize) -> i32>,
    pub debugfs_print_chan_status: Option<unsafe extern "C" fn(*mut iio_backend, u32, *mut i8, usize) -> i32>,
    pub debugfs_reg_access: Option<unsafe extern "C" fn(*mut iio_backend, u32, u32, *mut u32) -> i32>,
    pub filter_type_set: Option<unsafe extern "C" fn(*mut iio_backend, iio_backend_filter_type) -> i32>,
    pub interface_data_align: Option<unsafe extern "C" fn(*mut iio_backend, u32) -> i32>,
    pub num_lanes_set: Option<unsafe extern "C" fn(*mut iio_backend, u32) -> i32>,
    pub ddr_enable: Option<unsafe extern "C" fn(*mut iio_backend) -> i32>,
    pub ddr_disable: Option<unsafe extern "C" fn(*mut iio_backend) -> i32>,
    pub data_stream_enable: Option<unsafe extern "C" fn(*mut iio_backend) -> i32>,
    pub data_stream_disable: Option<unsafe extern "C" fn(*mut iio_backend) -> i32>,
    pub data_transfer_addr: Option<unsafe extern "C" fn(*mut iio_backend, u32) -> i32>,
}

#[repr(C)]
pub struct iio_backend_info {
    pub name: *const i8,
    pub ops: *const iio_backend_ops,
    pub caps: u32,
}

extern "C" {
    pub fn iio_backend_chan_enable(back: *mut iio_backend, chan: u32) -> i32;
    pub fn iio_backend_chan_disable(back: *mut iio_backend, chan: u32) -> i32;
    pub fn devm_iio_backend_enable(dev: *mut device, back: *mut iio_backend) -> i32;
    pub fn iio_backend_enable(back: *mut iio_backend) -> i32;
    pub fn iio_backend_disable(back: *mut iio_backend);
    pub fn iio_backend_data_format_set(back: *mut iio_backend, chan: u32, data: *const iio_backend_data_fmt) -> i32;
    pub fn iio_backend_data_source_set(back: *mut iio_backend, chan: u32, data: iio_backend_data_source) -> i32;
    pub fn iio_backend_data_source_get(back: *mut iio_backend, chan: u32, data: *mut iio_backend_data_source) -> i32;
    pub fn iio_backend_set_sampling_freq(back: *mut iio_backend, chan: u32, sample_rate_hz: u64) -> i32;
    pub fn iio_backend_test_pattern_set(back: *mut iio_backend, chan: u32, pattern: iio_backend_test_pattern) -> i32;
    pub fn iio_backend_chan_status(back: *mut iio_backend, chan: u32, error: *mut bool) -> i32;
    pub fn iio_backend_iodelay_set(back: *mut iio_backend, lane: u32, taps: u32) -> i32;
    pub fn iio_backend_data_sample_trigger(back: *mut iio_backend, trigger: iio_backend_sample_trigger) -> i32;
    pub fn devm_iio_backend_request_buffer(dev: *mut device, back: *mut iio_backend, indio_dev: *mut iio_dev) -> i32;
    pub fn iio_backend_filter_type_set(back: *mut iio_backend, type_: iio_backend_filter_type) -> i32;
    pub fn iio_backend_interface_data_align(back: *mut iio_backend, timeout_us: u32) -> i32;
    pub fn iio_backend_num_lanes_set(back: *mut iio_backend, num_lanes: u32) -> i32;
    pub fn iio_backend_ddr_enable(back: *mut iio_backend) -> i32;
    pub fn iio_backend_ddr_disable(back: *mut iio_backend) -> i32;
    pub fn iio_backend_data_stream_enable(back: *mut iio_backend) -> i32;
    pub fn iio_backend_data_stream_disable(back: *mut iio_backend) -> i32;
    pub fn iio_backend_data_transfer_addr(back: *mut iio_backend, address: u32) -> i32;
    pub fn iio_backend_ext_info_set(indio_dev: *mut iio_dev, private: usize, chan: *const iio_chan_spec, buf: *const i8, len: usize) -> isize;
    pub fn iio_backend_ext_info_get(indio_dev: *mut iio_dev, private: usize, chan: *const iio_chan_spec, buf: *mut i8) -> isize;
    pub fn iio_backend_interface_type_get(back: *mut iio_backend, type_: *mut iio_backend_interface_type) -> i32;
    pub fn iio_backend_data_size_set(back: *mut iio_backend, size: u32) -> i32;
    pub fn iio_backend_oversampling_ratio_set(back: *mut iio_backend, chan: u32, ratio: u32) -> i32;
    pub fn iio_backend_read_raw(back: *mut iio_backend, chan: *const iio_chan_spec, val: *mut i32, val2: *mut i32, mask: isize) -> i32;
    pub fn iio_backend_extend_chan_spec(back: *mut iio_backend, chan: *mut iio_chan_spec) -> i32;
    pub fn iio_backend_has_caps(back: *mut iio_backend, caps: u32) -> bool;
    pub fn iio_backend_get_priv(conv: *const iio_backend) -> *mut core::ffi::c_void;
    pub fn devm_iio_backend_get(dev: *mut device, name: *const i8) -> *mut iio_backend;
    pub fn devm_iio_backend_get_by_index(dev: *mut device, index: u32) -> *mut iio_backend;
    pub fn devm_iio_backend_fwnode_get(dev: *mut device, name: *const i8, fwnode: *mut fwnode_handle) -> *mut iio_backend;
    pub fn __devm_iio_backend_get_from_fwnode_lookup(dev: *mut device, fwnode: *mut fwnode_handle) -> *mut iio_backend;
    pub fn devm_iio_backend_register(dev: *mut device, info: *const iio_backend_info, priv_: *mut core::ffi::c_void) -> i32;
    pub fn iio_backend_debugfs_print_chan_status(back: *mut iio_backend, chan: u32, buf: *mut i8, len: usize) -> isize;
    pub fn iio_backend_debugfs_add(back: *mut iio_backend, indio_dev: *mut iio_dev);
}

pub unsafe fn iio_backend_read_scale(back: *mut iio_backend, chan: *const iio_chan_spec, val: *mut i32, val2: *mut i32) -> i32 {
    iio_backend_read_raw(back, chan, val, val2, IIO_CHAN_INFO_SCALE as isize)
}

pub unsafe fn iio_backend_read_offset(back: *mut iio_backend, chan: *const iio_chan_spec, val: *mut i32, val2: *mut i32) -> i32 {
    iio_backend_read_raw(back, chan, val, val2, IIO_CHAN_INFO_OFFSET as isize)
}

// Supplied by <linux/iio/iio.h>.
extern "C" {
    pub static IIO_CHAN_INFO_SCALE: u32;
    pub static IIO_CHAN_INFO_OFFSET: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
