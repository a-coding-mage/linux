/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2014 Marvell Technology Group Ltd.
 *
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 * Alexandre Belloni <alexandre.belloni@free-electrons.com>
 */

use std::os::raw::{c_char, c_ulong};

#[repr(C)]
pub struct berlin2_gate_data {
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub bit_idx: u8,
    pub flags: c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
