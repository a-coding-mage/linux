/* SPDX-License-Identifier: GPL-2.0-only */
/* Industrial I/O in kernel consumer interface */

/* Declarations corresponding to linux/types.h and linux/iio/types.h are
 * supplied by the surrounding translation. */

use core::ffi::c_void;

pub enum iio_dev {}
pub enum iio_chan_spec {}
pub enum device {}
pub enum fwnode_handle {}
pub enum iio_cb_buffer {}

#[repr(C)]
pub struct iio_channel {
    pub indio_dev: *mut iio_dev,
    pub channel: *const iio_chan_spec,
    pub data: *mut c_void,
}

extern "C" {
    pub fn iio_channel_get(dev: *mut device, consumer_channel: *const i8) -> *mut iio_channel;
    pub fn iio_channel_release(chan: *mut iio_channel);
    pub fn devm_iio_channel_get(dev: *mut device, consumer_channel: *const i8) -> *mut iio_channel;
    pub fn iio_channel_get_all(dev: *mut device) -> *mut iio_channel;
    pub fn iio_channel_release_all(chan: *mut iio_channel);
    pub fn devm_iio_channel_get_all(dev: *mut device) -> *mut iio_channel;
    pub fn fwnode_iio_channel_get_by_name(
        fwnode: *mut fwnode_handle,
        name: *const i8,
    ) -> *mut iio_channel;
    pub fn devm_fwnode_iio_channel_get_by_name(
        dev: *mut device,
        fwnode: *mut fwnode_handle,
        consumer_channel: *const i8,
    ) -> *mut iio_channel;

    pub fn iio_channel_get_all_cb(
        dev: *mut device,
        cb: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> i32>,
        private: *mut c_void,
    ) -> *mut iio_cb_buffer;
    pub fn iio_channel_cb_set_buffer_watermark(cb_buffer: *mut iio_cb_buffer, watermark: usize) -> i32;
    pub fn iio_channel_release_all_cb(cb_buffer: *mut iio_cb_buffer);
    pub fn iio_channel_start_all_cb(cb_buff: *mut iio_cb_buffer) -> i32;
    pub fn iio_channel_stop_all_cb(cb_buff: *mut iio_cb_buffer);
    pub fn iio_channel_cb_get_channels(cb_buffer: *const iio_cb_buffer) -> *mut iio_channel;
    pub fn iio_channel_cb_get_iio_dev(cb_buffer: *const iio_cb_buffer) -> *mut iio_dev;

    pub fn iio_read_channel_raw(chan: *mut iio_channel, val: *mut i32) -> i32;
    pub fn iio_read_channel_average_raw(chan: *mut iio_channel, val: *mut i32) -> i32;
    pub fn iio_read_channel_processed(chan: *mut iio_channel, val: *mut i32) -> i32;
    pub fn iio_read_channel_processed_scale(chan: *mut iio_channel, val: *mut i32, scale: u32) -> i32;
    pub fn iio_write_channel_attribute(
        chan: *mut iio_channel,
        val: i32,
        val2: i32,
        attribute: iio_chan_info_enum,
    ) -> i32;
    pub fn iio_read_channel_attribute(
        chan: *mut iio_channel,
        val: *mut i32,
        val2: *mut i32,
        attribute: iio_chan_info_enum,
    ) -> i32;
    pub fn iio_write_channel_raw(chan: *mut iio_channel, val: i32) -> i32;
    pub fn iio_read_max_channel_raw(chan: *mut iio_channel, val: *mut i32) -> i32;
    pub fn iio_read_min_channel_raw(chan: *mut iio_channel, val: *mut i32) -> i32;
    pub fn iio_read_avail_channel_raw(chan: *mut iio_channel, vals: *mut *const i32, length: *mut i32) -> i32;
    pub fn iio_read_avail_channel_attribute(
        chan: *mut iio_channel,
        vals: *mut *const i32,
        type_: *mut i32,
        length: *mut i32,
        attribute: iio_chan_info_enum,
    ) -> i32;
    pub fn iio_get_channel_type(channel: *mut iio_channel, type_: *mut iio_chan_type) -> i32;
    pub fn iio_read_channel_offset(chan: *mut iio_channel, val: *mut i32, val2: *mut i32) -> i32;
    pub fn iio_read_channel_scale(chan: *mut iio_channel, val: *mut i32, val2: *mut i32) -> i32;
    pub fn iio_multiply_value(result: *mut i32, multiplier: i64, type_: u32, val: i32, val2: i32) -> i32;
    pub fn iio_convert_raw_to_processed(chan: *mut iio_channel, raw: i32, processed: *mut i32, scale: u32) -> i32;
    pub fn iio_get_channel_ext_info_count(chan: *mut iio_channel) -> u32;
    pub fn iio_read_channel_ext_info(chan: *mut iio_channel, attr: *const i8, buf: *mut i8) -> isize;
    pub fn iio_write_channel_ext_info(chan: *mut iio_channel, attr: *const i8, buf: *const i8, len: usize) -> isize;
    pub fn iio_read_channel_label(chan: *mut iio_channel, buf: *mut i8) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
