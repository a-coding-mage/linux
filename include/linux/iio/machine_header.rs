/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Industrial I/O in kernel access map definitions for board files.
 *
 * Copyright (c) 2011 Jonathan Cameron
 */

/// Description of link between consumer and device channels.
///
/// `adc_channel_label` is the label used to identify the channel on the
/// provider. This is matched against the `datasheet_name` element of
/// `struct iio_chan_spec`.
/// `consumer_dev_name` is the name to uniquely identify the consumer device.
/// `consumer_channel` is the unique name used to identify the channel on the
/// consumer side.
/// `consumer_data` is data about the channel for use by the consumer driver.
#[repr(C)]
pub struct iio_map {
    pub adc_channel_label: *const ::core::ffi::c_char,
    pub consumer_dev_name: *const ::core::ffi::c_char,
    pub consumer_channel: *const ::core::ffi::c_char,
    pub consumer_data: *mut ::core::ffi::c_void,
}

#[macro_export]
macro_rules! IIO_MAP {
    ($provider_channel:expr, $consumer_dev_name:expr, $consumer_channel:expr) => {
        $crate::iio_map {
            adc_channel_label: $provider_channel,
            consumer_dev_name: $consumer_dev_name,
            consumer_channel: $consumer_channel,
            ..unsafe { ::core::mem::zeroed() }
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
