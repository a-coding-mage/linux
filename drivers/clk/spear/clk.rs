// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 *
 * SPEAr clk - Common routines
 */

// Dependencies supplied by the Linux clock-provider/types headers and clk.h.

use core::ffi::{c_int, c_long, c_ulong, c_uchar};

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

pub type clk_calc_rate = unsafe extern "C" fn(
    hw: *mut clk_hw,
    parent_rate: c_ulong,
    index: c_int,
) -> c_ulong;

pub unsafe extern "C" fn clk_round_rate_index(
    hw: *mut clk_hw,
    drate: c_ulong,
    parent_rate: c_ulong,
    calc_rate: clk_calc_rate,
    rtbl_cnt: c_uchar,
    index: *mut c_int,
) -> c_long {
    let mut prev_rate: c_ulong;
    let mut rate: c_ulong = 0;

    while *index < rtbl_cnt as c_int {
        prev_rate = rate;
        rate = calc_rate(hw, parent_rate, *index);
        if drate < rate {
            /* previous clock was best */
            if *index != 0 {
                rate = prev_rate;
                *index -= 1;
            }
            break;
        }
        *index += 1;
    }

    if *index == rtbl_cnt as c_int {
        *index -= 1;
    }

    rate as c_long
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
