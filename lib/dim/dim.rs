// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
/*
 * Copyright (c) 2019, Mellanox Technologies inc.  All rights reserved.
 */

// Dependency intent preserved from: #include <linux/dim.h>

pub unsafe fn dim_on_top(dim: *const dim) -> bool {
    match (*dim).tune_state {
        DIM_PARKING_ON_TOP | DIM_PARKING_TIRED => true,
        DIM_GOING_RIGHT => ((*dim).steps_left > 1) && ((*dim).steps_right == 1),
        _ => ((*dim).steps_right > 1) && ((*dim).steps_left == 1), // DIM_GOING_LEFT
    }
}

// EXPORT_SYMBOL(dim_on_top);

pub unsafe fn dim_turn(dim: *mut dim) {
    match (*dim).tune_state {
        DIM_PARKING_ON_TOP | DIM_PARKING_TIRED => {}
        DIM_GOING_RIGHT => {
            (*dim).tune_state = DIM_GOING_LEFT;
            (*dim).steps_left = 0;
        }
        DIM_GOING_LEFT => {
            (*dim).tune_state = DIM_GOING_RIGHT;
            (*dim).steps_right = 0;
        }
        _ => {}
    }
}

// EXPORT_SYMBOL(dim_turn);

pub unsafe fn dim_park_on_top(dim: *mut dim) {
    (*dim).steps_right = 0;
    (*dim).steps_left = 0;
    (*dim).tired = 0;
    (*dim).tune_state = DIM_PARKING_ON_TOP;
}

// EXPORT_SYMBOL(dim_park_on_top);

pub unsafe fn dim_park_tired(dim: *mut dim) {
    (*dim).steps_right = 0;
    (*dim).steps_left = 0;
    (*dim).tune_state = DIM_PARKING_TIRED;
}

// EXPORT_SYMBOL(dim_park_tired);

pub unsafe fn dim_calc_stats(
    start: *const dim_sample,
    end: *const dim_sample,
    curr_stats: *mut dim_stats,
) -> bool {
    // u32 holds up to 71 minutes, should be enough
    let delta_us: u32 = ktime_us_delta((*end).time, (*start).time) as u32;
    let npkts: u32 = BIT_GAP!(BITS_PER_TYPE!(u32), (*end).pkt_ctr, (*start).pkt_ctr);
    let nbytes: u32 = BIT_GAP!(BITS_PER_TYPE!(u32), (*end).byte_ctr, (*start).byte_ctr);
    let ncomps: u32 = BIT_GAP!(BITS_PER_TYPE!(u32), (*end).comp_ctr, (*start).comp_ctr);

    if delta_us == 0 {
        return false;
    }

    (*curr_stats).ppms = DIV_ROUND_UP!(npkts.wrapping_mul(USEC_PER_MSEC), delta_us);
    (*curr_stats).bpms = DIV_ROUND_UP!(nbytes.wrapping_mul(USEC_PER_MSEC), delta_us);
    (*curr_stats).epms = DIV_ROUND_UP!(DIM_NEVENTS.wrapping_mul(USEC_PER_MSEC), delta_us);
    (*curr_stats).cpms = DIV_ROUND_UP!(ncomps.wrapping_mul(USEC_PER_MSEC), delta_us);
    if (*curr_stats).epms != 0 {
        (*curr_stats).cpe_ratio = DIV_ROUND_DOWN_ULL!(
            (*curr_stats).cpms.wrapping_mul(100),
            (*curr_stats).epms,
        );
    } else {
        (*curr_stats).cpe_ratio = 0;
    }

    true
}

// EXPORT_SYMBOL(dim_calc_stats);

// MODULE_DESCRIPTION("Dynamic Interrupt Moderation (DIM) library");
// MODULE_LICENSE("Dual BSD/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
