/* SPDX-License-Identifier: GPL-2.0-only */
/* industrial I/O data types needed both in and out of kernel
 *
 * Copyright (c) 2008 Jonathan Cameron
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum iio_event_info {
    IIO_EV_INFO_ENABLE,
    IIO_EV_INFO_VALUE,
    IIO_EV_INFO_HYSTERESIS,
    IIO_EV_INFO_PERIOD,
    IIO_EV_INFO_HIGH_PASS_FILTER_3DB,
    IIO_EV_INFO_LOW_PASS_FILTER_3DB,
    IIO_EV_INFO_TIMEOUT,
    IIO_EV_INFO_RESET_TIMEOUT,
    IIO_EV_INFO_TAP2_MIN_DELAY,
    IIO_EV_INFO_RUNNING_PERIOD,
    IIO_EV_INFO_RUNNING_COUNT,
    IIO_EV_INFO_SCALE,
}

pub const IIO_VAL_INT: i32 = 1;
pub const IIO_VAL_INT_PLUS_MICRO: i32 = 2;
pub const IIO_VAL_INT_PLUS_NANO: i32 = 3;
pub const IIO_VAL_INT_PLUS_MICRO_DB: i32 = 4;
pub const IIO_VAL_INT_MULTIPLE: i32 = 5;
pub const IIO_VAL_INT_64: i32 = 6; /* 64-bit data, val is lower 32 bits */
pub const IIO_VAL_FRACTIONAL: i32 = 10;
pub const IIO_VAL_FRACTIONAL_LOG2: i32 = 11;
pub const IIO_VAL_CHAR: i32 = 12;

pub const IIO_VAL_DECIMAL64_BASE: i32 = 32;
pub const IIO_VAL_DECIMAL64_MILLI: i32 = IIO_VAL_DECIMAL64_BASE + 3;
pub const IIO_VAL_DECIMAL64_MICRO: i32 = IIO_VAL_DECIMAL64_BASE + 6;
pub const IIO_VAL_DECIMAL64_NANO: i32 = IIO_VAL_DECIMAL64_BASE + 9;
pub const IIO_VAL_DECIMAL64_PICO: i32 = IIO_VAL_DECIMAL64_BASE + 12;
pub const IIO_VAL_DECIMAL64_FEMTO: i32 = IIO_VAL_DECIMAL64_BASE + 15;

#[inline]
pub fn iio_val_s64_compose(val0: i32, val1: i32) -> i64 {
    (((val1 as u32 as u64) << 32) | (val0 as u32 as u64)) as i64
}

#[inline]
pub unsafe fn iio_val_s64_decompose(dec64: i64, val0: *mut i32, val1: *mut i32) {
    *val0 = dec64 as u32 as i32;
    *val1 = ((dec64 as u64) >> 32) as u32 as i32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum iio_available_type {
    IIO_AVAIL_LIST,
    IIO_AVAIL_RANGE,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum iio_chan_info_enum {
    IIO_CHAN_INFO_RAW = 0,
    IIO_CHAN_INFO_PROCESSED,
    IIO_CHAN_INFO_SCALE,
    IIO_CHAN_INFO_OFFSET,
    IIO_CHAN_INFO_CALIBSCALE,
    IIO_CHAN_INFO_CALIBBIAS,
    IIO_CHAN_INFO_PEAK,
    IIO_CHAN_INFO_PEAK_SCALE,
    IIO_CHAN_INFO_QUADRATURE_CORRECTION_RAW,
    IIO_CHAN_INFO_AVERAGE_RAW,
    IIO_CHAN_INFO_LOW_PASS_FILTER_3DB_FREQUENCY,
    IIO_CHAN_INFO_HIGH_PASS_FILTER_3DB_FREQUENCY,
    IIO_CHAN_INFO_SAMP_FREQ,
    IIO_CHAN_INFO_FREQUENCY,
    IIO_CHAN_INFO_PHASE,
    IIO_CHAN_INFO_HARDWAREGAIN,
    IIO_CHAN_INFO_HYSTERESIS,
    IIO_CHAN_INFO_HYSTERESIS_RELATIVE,
    IIO_CHAN_INFO_INT_TIME,
    IIO_CHAN_INFO_ENABLE,
    IIO_CHAN_INFO_CALIBHEIGHT,
    IIO_CHAN_INFO_CALIBWEIGHT,
    IIO_CHAN_INFO_DEBOUNCE_COUNT,
    IIO_CHAN_INFO_DEBOUNCE_TIME,
    IIO_CHAN_INFO_CALIBEMISSIVITY,
    IIO_CHAN_INFO_OVERSAMPLING_RATIO,
    IIO_CHAN_INFO_THERMOCOUPLE_TYPE,
    IIO_CHAN_INFO_CALIBAMBIENT,
    IIO_CHAN_INFO_ZEROPOINT,
    IIO_CHAN_INFO_TROUGH,
    IIO_CHAN_INFO_CONVDELAY,
    IIO_CHAN_INFO_POWERFACTOR,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
