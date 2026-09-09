/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/iio/iio.h. Included dependency types are external. */

use core::ffi::{c_char, c_void};

#[repr(C)] pub struct fwnode_reference_args { _private: [u8; 0] }
#[repr(C)] pub struct device { pub parent: *mut device }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct iio_buffer { _private: [u8; 0] }
#[repr(C)] pub struct iio_trigger { _private: [u8; 0] }
#[repr(C)] pub struct iio_poll_func { _private: [u8; 0] }

pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type size_t = usize;
pub type clockid_t = i32;
pub type s64 = i64;
pub type u64 = u64;
pub type u8 = u8;

#[repr(C)] #[derive(Copy, Clone)] pub struct iio_chan_spec { pub type_: iio_chan_type, pub channel: i32, pub channel2: i32, pub address: usize, pub scan_index: i32, pub scan: iio_chan_scan_union, pub info_mask_separate: usize, pub info_mask_separate_available: usize, pub info_mask_shared_by_type: usize, pub info_mask_shared_by_type_available: usize, pub info_mask_shared_by_dir: usize, pub info_mask_shared_by_dir_available: usize, pub info_mask_shared_by_all: usize, pub info_mask_shared_by_all_available: usize, pub event_spec: *const iio_event_spec, pub num_event_specs: u32, pub ext_info: *const iio_chan_spec_ext_info, pub extend_name: *const c_char, pub datasheet_name: *const c_char, pub modified: u32, pub indexed: u32, pub output: u32, pub differential: u32, pub has_ext_scan_type: u32 }
#[repr(C)] pub union iio_chan_scan_union { pub scan_type: iio_scan_type, pub ext: iio_ext_scan_type }
#[repr(C)] pub struct iio_ext_scan_type { pub ext_scan_type: *const iio_scan_type, pub num_ext_scan_type: u32 }

#[repr(C)] #[derive(Copy, Clone)] pub struct iio_scan_type { pub sign_or_format: iio_scan_format_union, pub realbits: u8, pub storagebits: u8, pub shift: u8, pub repeat: u8, pub endianness: iio_endian }
#[repr(C)] pub union iio_scan_format_union { pub sign: c_char, pub format: c_char }
#[repr(C)] pub struct iio_chan_spec_ext_info { pub name: *const c_char, pub shared: iio_shared_by, pub read: Option<unsafe extern "C" fn(*mut iio_dev, uintptr_t, *const iio_chan_spec, *mut c_char) -> ssize_t>, pub write: Option<unsafe extern "C" fn(*mut iio_dev, uintptr_t, *const iio_chan_spec, *const c_char, size_t) -> ssize_t>, pub private: uintptr_t }
#[repr(C)] pub struct iio_enum { pub items: *const *const c_char, pub num_items: u32, pub set: Option<unsafe extern "C" fn(*mut iio_dev, *const iio_chan_spec, u32) -> i32>, pub get: Option<unsafe extern "C" fn(*mut iio_dev, *const iio_chan_spec) -> i32> }
#[repr(C)] pub struct iio_mount_matrix { pub rotation: [*const c_char; 9] }
#[repr(C)] pub struct iio_event_spec { pub type_: iio_event_type, pub dir: iio_event_direction, pub mask_separate: usize, pub mask_shared_by_type: usize, pub mask_shared_by_dir: usize, pub mask_shared_by_all: usize }

#[repr(C)] pub struct iio_val_int_plus_micro { pub integer: i32, pub micro: i32 }
#[repr(C)] pub struct iio_buffer_setup_ops { pub preenable: Option<unsafe extern "C" fn(*mut iio_dev)->i32>, pub postenable: Option<unsafe extern "C" fn(*mut iio_dev)->i32>, pub predisable: Option<unsafe extern "C" fn(*mut iio_dev)->i32>, pub postdisable: Option<unsafe extern "C" fn(*mut iio_dev)->i32>, pub validate_scan_mask: Option<unsafe extern "C" fn(*mut iio_dev,*const usize)->bool> }
#[repr(C)] pub struct iio_info { pub event_attrs: *const attribute_group, pub attrs: *const attribute_group, pub read_raw: Option<unsafe extern "C" fn(*mut iio_dev,*const iio_chan_spec,*mut i32,*mut i32,isize)->i32>, pub read_raw_multi: Option<unsafe extern "C" fn(*mut iio_dev,*const iio_chan_spec,i32,*mut i32,*mut i32,isize)->i32>, pub read_avail: Option<unsafe extern "C" fn(*mut iio_dev,*const iio_chan_spec,*mut *const i32,*mut i32,*mut i32,isize)->i32>, pub write_raw: Option<unsafe extern "C" fn(*mut iio_dev,*const iio_chan_spec,i32,i32,isize)->i32>, pub read_label: Option<unsafe extern "C" fn(*mut iio_dev,*const iio_chan_spec,*mut c_char)->i32>, pub write_raw_get_fmt: Option<unsafe extern "C" fn(*mut iio_dev,*const iio_chan_spec,isize)->i32>, pub read_event_config: Option<unsafe extern "C" fn(*mut iio_dev,*const iio_chan_spec,iio_event_type,iio_event_direction)->i32>, pub write_event_config: Option<unsafe extern "C" fn(*mut iio_dev,*const iio_chan_spec,iio_event_type,iio_event_direction,bool)->i32>, pub read_event_value: Option<unsafe extern "C" fn(*mut iio_dev,*const iio_chan_spec,iio_event_type,iio_event_direction,iio_event_info,*mut i32,*mut i32)->i32>, pub write_event_value: Option<unsafe extern "C" fn(*mut iio_dev,*const iio_chan_spec,iio_event_type,iio_event_direction,iio_event_info,i32,i32)->i32>, pub read_event_label: Option<unsafe extern "C" fn(*mut iio_dev,*const iio_chan_spec,iio_event_type,iio_event_direction,*mut c_char)->i32>, pub validate_trigger: Option<unsafe extern "C" fn(*mut iio_dev,*mut iio_trigger)->i32>, pub get_current_scan_type: Option<unsafe extern "C" fn(*const iio_dev,*const iio_chan_spec)->i32>, pub update_scan_mode: Option<unsafe extern "C" fn(*mut iio_dev,*const usize)->i32>, pub debugfs_reg_access: Option<unsafe extern "C" fn(*mut iio_dev,u32,u32,*mut u32)->i32>, pub fwnode_xlate: Option<unsafe extern "C" fn(*mut iio_dev,*const fwnode_reference_args)->i32>, pub hwfifo_set_watermark: Option<unsafe extern "C" fn(*mut iio_dev,u32)->i32>, pub hwfifo_flush_to_buffer: Option<unsafe extern "C" fn(*mut iio_dev,u32)->i32> }
#[repr(C)] pub struct iio_dev { pub modes: i32, pub dev: device, pub buffer: *mut iio_buffer, pub scan_bytes: i32, pub scan_timestamp_offset: u32, pub available_scan_masks: *const usize, pub masklength: u32, pub active_scan_mask: *const usize, pub scan_timestamp: bool, pub trig: *mut iio_trigger, pub pollfunc: *mut iio_poll_func, pub pollfunc_event: *mut iio_poll_func, pub channels: *const iio_chan_spec, pub num_channels: i32, pub name: *const c_char, pub label: *const c_char, pub info: *const iio_info, pub setup_ops: *const iio_buffer_setup_ops, pub priv_: *mut c_void }

#[repr(C)] pub enum iio_shared_by { IIO_SEPARATE, IIO_SHARED_BY_TYPE, IIO_SHARED_BY_DIR, IIO_SHARED_BY_ALL }
#[repr(C)] pub enum iio_endian { IIO_CPU, IIO_BE, IIO_LE }
/* These enums are supplied by linux/iio/types.h. */
#[repr(C)] pub enum iio_chan_type { IIO_TIMESTAMP }
#[repr(C)] pub enum iio_event_type { }
#[repr(C)] pub enum iio_event_direction { }
#[repr(C)] pub enum iio_event_info { }
#[repr(C)] pub enum iio_chan_info_enum { }

pub const IIO_SCAN_FORMAT_SIGNED_INT: c_char = b's' as c_char;
pub const IIO_SCAN_FORMAT_UNSIGNED_INT: c_char = b'u' as c_char;
pub const IIO_SCAN_FORMAT_FLOAT: c_char = b'f' as c_char;
pub const INDIO_DIRECT_MODE: i32 = 0x01; pub const INDIO_BUFFER_TRIGGERED: i32 = 0x02; pub const INDIO_BUFFER_SOFTWARE: i32 = 0x04; pub const INDIO_BUFFER_HARDWARE: i32 = 0x08; pub const INDIO_EVENT_TRIGGERED: i32 = 0x10; pub const INDIO_HARDWARE_TRIGGERED: i32 = 0x20; pub const INDIO_ALL_BUFFER_MODES: i32 = 0x0e; pub const INDIO_ALL_TRIGGERED_MODES: i32 = 0x32; pub const INDIO_MAX_RAW_ELEMENTS: usize = 4;

extern "C" { pub fn iio_enum_available_read(*mut iio_dev, uintptr_t,*const iio_chan_spec,*mut c_char)->ssize_t; pub fn iio_enum_read(*mut iio_dev, uintptr_t,*const iio_chan_spec,*mut c_char)->ssize_t; pub fn iio_enum_write(*mut iio_dev, uintptr_t,*const iio_chan_spec,*const c_char,size_t)->ssize_t; pub fn iio_show_mount_matrix(*mut iio_dev,uintptr_t,*const iio_chan_spec,*mut c_char)->ssize_t; pub fn iio_read_mount_matrix(*mut device,*mut iio_mount_matrix)->i32; pub fn iio_get_time_ns(*const iio_dev)->s64; pub fn iio_device_id(*mut iio_dev)->i32; pub fn iio_device_get_current_mode(*mut iio_dev)->i32; pub fn iio_buffer_enabled(*mut iio_dev)->bool; pub fn iio_device_alloc(*mut device,i32)->*mut iio_dev; pub fn iio_device_free(*mut iio_dev); pub fn iio_device_suspend_triggering(*mut iio_dev)->i32; pub fn iio_device_resume_triggering(*mut iio_dev)->i32; pub fn iio_format_value(*mut c_char,u32,i32,*mut i32)->ssize_t; pub fn iio_str_to_fixpoint(*const c_char,i32,*mut i32,*mut i32)->i32 }

#[inline] pub unsafe fn iio_channel_has_info(c: *const iio_chan_spec, t: u32) -> bool { let b=1usize.wrapping_shl(t); ((*c).info_mask_separate|(*c).info_mask_shared_by_type|(*c).info_mask_shared_by_dir|(*c).info_mask_shared_by_all)&b != 0 }
#[inline] pub unsafe fn iio_channel_has_available(c: *const iio_chan_spec, t: u32) -> bool { let b=1usize.wrapping_shl(t); ((*c).info_mask_separate_available|(*c).info_mask_shared_by_type_available|(*c).info_mask_shared_by_dir_available|(*c).info_mask_shared_by_all_available)&b != 0 }
#[inline] pub unsafe fn iio_get_masklength(d: *const iio_dev)->u32 { (*d).masklength }
pub const fn iio_degree_to_rad(deg:u64)->u64 { (deg*314159+9000000)/18000000 }
pub const fn iio_rad_to_degree(rad:u64)->u64 { (rad*18000000+314159/2)/314159 }
pub const fn iio_g_to_m_s_2(g:u64)->u64 { g*980665/100000 }
pub const fn iio_m_s_2_to_g(ms2:u64)->u64 { (ms2*100000+980665/2)/980665 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
