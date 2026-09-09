/* SPDX-License-Identifier: GPL-2.0-only */
/* The industrial I/O core
 *
 * Copyright (c) 2008 Jonathan Cameron
 *
 * General attributes
 */

// Forward declarations supplied by other headers.
pub struct device;
pub struct device_attribute;
pub struct list_head;
pub struct iio_buffer;
pub struct iio_chan_spec;

#[repr(C)]
pub struct iio_dev_attr {
    pub dev_attr: device_attribute,
    pub address: u64,
    pub l: list_head,
    pub c: *const iio_chan_spec,
    pub buffer: *mut iio_buffer,
}

// Equivalent of container_of(_dev_attr, struct iio_dev_attr, dev_attr).
#[macro_export]
macro_rules! to_iio_dev_attr {
    ($dev_attr:expr) => {
        unsafe {
            &mut *((($dev_attr as *mut u8).sub(core::mem::offset_of!(iio_dev_attr, dev_attr)))
                as *mut iio_dev_attr)
        }
    };
}

extern "C" {
    pub fn iio_read_const_attr(
        dev: *mut device,
        attr: *mut device_attribute,
        len: *mut core::ffi::c_char,
    ) -> isize;
}

#[repr(C)]
pub struct iio_const_attr {
    pub string: *const core::ffi::c_char,
    pub dev_attr: device_attribute,
}

#[macro_export]
macro_rules! to_iio_const_attr {
    ($dev_attr:expr) => {
        unsafe {
            &mut *((($dev_attr as *mut u8).sub(core::mem::offset_of!(iio_const_attr, dev_attr)))
                as *mut iio_const_attr)
        }
    };
}

// Some attributes are hard coded and do not require an address; in these
// cases pass a negative value.
#[macro_export]
macro_rules! IIO_ATTR {
    ($name:ident, $mode:expr, $show:expr, $store:expr, $addr:expr) => {
        iio_dev_attr {
            dev_attr: __ATTR!($name, $mode, $show, $store),
            address: $addr,
            l: unsafe { core::mem::zeroed() },
            c: core::ptr::null(),
            buffer: core::ptr::null_mut(),
        }
    };
}

#[macro_export]
macro_rules! IIO_ATTR_RO {
    ($name:ident, $addr:expr) => { iio_dev_attr { dev_attr: __ATTR_RO!($name), address: $addr, l: unsafe { core::mem::zeroed() }, c: core::ptr::null(), buffer: core::ptr::null_mut() } };
}
#[macro_export]
macro_rules! IIO_ATTR_WO {
    ($name:ident, $addr:expr) => { iio_dev_attr { dev_attr: __ATTR_WO!($name), address: $addr, l: unsafe { core::mem::zeroed() }, c: core::ptr::null(), buffer: core::ptr::null_mut() } };
}
#[macro_export]
macro_rules! IIO_ATTR_RW {
    ($name:ident, $addr:expr) => { iio_dev_attr { dev_attr: __ATTR_RW!($name), address: $addr, l: unsafe { core::mem::zeroed() }, c: core::ptr::null(), buffer: core::ptr::null_mut() } };
}

// C token-pasting in the declaration macros is represented by an explicit
// variable identifier argument in Rust.
#[macro_export]
macro_rules! IIO_DEVICE_ATTR {
    ($var:ident, $name:ident, $mode:expr, $show:expr, $store:expr, $addr:expr) => {
        static mut $var: iio_dev_attr = IIO_ATTR!($name, $mode, $show, $store, $addr);
    };
}
#[macro_export]
macro_rules! IIO_DEVICE_ATTR_RO { ($var:ident, $name:ident, $addr:expr) => { static mut $var: iio_dev_attr = IIO_ATTR_RO!($name, $addr); }; }
#[macro_export]
macro_rules! IIO_DEVICE_ATTR_WO { ($var:ident, $name:ident, $addr:expr) => { static mut $var: iio_dev_attr = IIO_ATTR_WO!($name, $addr); }; }
#[macro_export]
macro_rules! IIO_DEVICE_ATTR_RW { ($var:ident, $name:ident, $addr:expr) => { static mut $var: iio_dev_attr = IIO_ATTR_RW!($name, $addr); }; }
#[macro_export]
macro_rules! IIO_DEVICE_ATTR_NAMED { ($var:ident, $name:ident, $mode:expr, $show:expr, $store:expr, $addr:expr) => { static mut $var: iio_dev_attr = IIO_ATTR!($name, $mode, $show, $store, $addr); }; }

#[macro_export]
macro_rules! IIO_CONST_ATTR {
    ($var:ident, $name:ident, $string:expr) => { static mut $var: iio_const_attr = iio_const_attr { string: $string, dev_attr: __ATTR!($name, S_IRUGO, iio_read_const_attr, core::ptr::null_mut()) }; };
}
#[macro_export]
macro_rules! IIO_CONST_ATTR_NAMED { ($var:ident, $name:ident, $string:expr) => { IIO_CONST_ATTR!($var, $name, $string); }; }

#[macro_export]
macro_rules! IIO_DEV_ATTR_SAMP_FREQ { ($var:ident, $mode:expr, $show:expr, $store:expr) => { IIO_DEVICE_ATTR!($var, sampling_frequency, $mode, $show, $store, 0); }; }
#[macro_export]
macro_rules! IIO_DEV_ATTR_SAMP_FREQ_AVAIL { ($var:ident, $show:expr) => { IIO_DEVICE_ATTR!($var, sampling_frequency_available, S_IRUGO, $show, core::ptr::null_mut(), 0); }; }
#[macro_export]
macro_rules! IIO_CONST_ATTR_SAMP_FREQ_AVAIL { ($var:ident, $string:expr) => { IIO_CONST_ATTR!($var, sampling_frequency_available, $string); }; }
#[macro_export]
macro_rules! IIO_DEV_ATTR_INT_TIME_AVAIL { ($var:ident, $show:expr) => { IIO_DEVICE_ATTR!($var, integration_time_available, S_IRUGO, $show, core::ptr::null_mut(), 0); }; }
#[macro_export]
macro_rules! IIO_CONST_ATTR_INT_TIME_AVAIL { ($var:ident, $string:expr) => { IIO_CONST_ATTR!($var, integration_time_available, $string); }; }
#[macro_export]
macro_rules! IIO_DEV_ATTR_TEMP_RAW { ($var:ident, $show:expr) => { IIO_DEVICE_ATTR!($var, in_temp_raw, S_IRUGO, $show, core::ptr::null_mut(), 0); }; }
#[macro_export]
macro_rules! IIO_CONST_ATTR_TEMP_OFFSET { ($var:ident, $string:expr) => { IIO_CONST_ATTR!($var, in_temp_offset, $string); }; }
#[macro_export]
macro_rules! IIO_CONST_ATTR_TEMP_SCALE { ($var:ident, $string:expr) => { IIO_CONST_ATTR!($var, in_temp_scale, $string); }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
