/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2020 Invensense, Inc.
 */

// Translated from inv_sensors_timestamp.h.

#[repr(C)]
pub struct inv_sensors_timestamp_chip {
    pub clock_period: u32,
    pub jitter: u32,
    pub init_period: u32,
}

#[repr(C)]
pub struct inv_sensors_timestamp_interval {
    pub lo: i64,
    pub up: i64,
}

#[repr(C)]
pub struct inv_sensors_timestamp_acc {
    pub val: u32,
    pub idx: usize,
    pub values: [u32; 32],
}

#[repr(C)]
pub struct inv_sensors_timestamp {
    pub chip: inv_sensors_timestamp_chip,
    pub min_period: u32,
    pub max_period: u32,
    pub it: inv_sensors_timestamp_interval,
    pub delta: inv_sensors_timestamp_interval,
    pub delta_counter: u32,
    pub timestamp: i64,
    pub mult: u32,
    pub new_mult: u32,
    pub period: u32,
    pub chip_period: inv_sensors_timestamp_acc,
}

extern "C" {
    pub fn inv_sensors_timestamp_init(
        ts: *mut inv_sensors_timestamp,
        chip: *const inv_sensors_timestamp_chip,
    );

    pub fn inv_sensors_timestamp_update_odr(
        ts: *mut inv_sensors_timestamp,
        period: u32,
        fifo: bool,
    ) -> i32;

    pub fn inv_sensors_timestamp_interrupt(
        ts: *mut inv_sensors_timestamp,
        sample_nb: usize,
        timestamp: i64,
    );
}

#[inline]
pub unsafe fn inv_sensors_timestamp_pop(ts: *mut inv_sensors_timestamp) -> i64 {
    (*ts).timestamp = (*ts).timestamp.wrapping_add((*ts).period as i64);
    (*ts).timestamp
}

extern "C" {
    pub fn inv_sensors_timestamp_apply_odr(
        ts: *mut inv_sensors_timestamp,
        fifo_period: u32,
        fifo_nb: usize,
        fifo_no: u32,
    );
}

#[inline]
pub unsafe fn inv_sensors_timestamp_reset(ts: *mut inv_sensors_timestamp) {
    let interval_init = inv_sensors_timestamp_interval { lo: 0, up: 0 };

    (*ts).it = interval_init;
    (*ts).delta = interval_init;
    (*ts).delta_counter = 0;
    (*ts).timestamp = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
