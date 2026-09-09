// SPDX-License-Identifier: GPL-2.0-only
/*
 * Utility functions for parsing Tegra CVB voltage tables
 *
 * Copyright (C) 2012-2019 NVIDIA Corporation.  All rights reserved.
 */

// Linux kernel dependencies and cvb.h are supplied by the surrounding crate.

#[inline]
unsafe fn get_cvb_voltage(
    speedo: i32,
    s_scale: i32,
    cvb: *const cvb_coefficients,
) -> i32 {
    let mut mv: i32;

    // apply only speedo scale: output mv = cvb_mv * v_scale
    mv = div_round_closest((*cvb).c2.wrapping_mul(speedo), s_scale);
    mv = div_round_closest(
        (mv.wrapping_add((*cvb).c1)).wrapping_mul(speedo),
        s_scale,
    )
    .wrapping_add((*cvb).c0);
    mv
}

unsafe fn round_cvb_voltage(mv: i32, v_scale: i32, align: *const rail_alignment) -> i32 {
    // combined: apply voltage scale and round to cvb alignment step
    let step = (if (*align).step_uv != 0 {
        (*align).step_uv
    } else {
        1000
    })
    .wrapping_mul(v_scale);
    let offset = (*align).offset_uv.wrapping_mul(v_scale);

    let mut uv = core::cmp::max(mv.wrapping_mul(1000), offset).wrapping_sub(offset);
    uv = div_round_up(uv, step)
        .wrapping_mul((*align).step_uv)
        .wrapping_add((*align).offset_uv);
    uv / 1000
}

const DOWN: i32 = 0;
const UP: i32 = 1;

unsafe fn round_voltage(mv: i32, align: *const rail_alignment, up: i32) -> i32 {
    if (*align).step_uv != 0 {
        let uv = core::cmp::max(mv.wrapping_mul(1000), (*align).offset_uv)
            .wrapping_sub((*align).offset_uv);
        let uv = (uv.wrapping_add(if up != 0 {
            (*align).step_uv - 1
        } else {
            0
        })) / (*align).step_uv;
        return (uv.wrapping_mul((*align).step_uv).wrapping_add((*align).offset_uv)) / 1000;
    }
    mv
}

unsafe fn build_opp_table(
    dev: *mut device,
    table: *const cvb_table,
    align: *mut rail_alignment,
    speedo_value: i32,
    max_freq: usize,
) -> i32 {
    let min_mv = round_voltage((*table).min_millivolts, align, UP);
    let max_mv = round_voltage((*table).max_millivolts, align, DOWN);

    for i in 0..MAX_DVFS_FREQS {
        let entry = &(*table).entries[i];

        if entry.freq == 0 || entry.freq > max_freq {
            break;
        }

        let mut dfll_mv = get_cvb_voltage(
            speedo_value,
            (*table).speedo_scale,
            &entry.coefficients,
        );
        dfll_mv = round_cvb_voltage(dfll_mv, (*table).voltage_scale, align);
        dfll_mv = core::cmp::max(min_mv, core::cmp::min(dfll_mv, max_mv));

        let ret = dev_pm_opp_add(dev, entry.freq, (dfll_mv as u64).wrapping_mul(1000));
        if ret != 0 {
            return ret;
        }
    }

    0
}

/**
 * tegra_cvb_add_opp_table - build OPP table from Tegra CVB tables
 */
pub unsafe fn tegra_cvb_add_opp_table(
    dev: *mut device,
    tables: *const cvb_table,
    count: usize,
    align: *mut rail_alignment,
    process_id: i32,
    speedo_id: i32,
    speedo_value: i32,
    max_freq: usize,
) -> *const cvb_table {
    for i in 0..count {
        let table = tables.add(i);

        if (*table).speedo_id != -1 && (*table).speedo_id != speedo_id {
            continue;
        }
        if (*table).process_id != -1 && (*table).process_id != process_id {
            continue;
        }

        let ret = build_opp_table(dev, table, align, speedo_value, max_freq);
        return if ret != 0 {
            err_ptr(ret)
        } else {
            table
        };
    }

    err_ptr(-22)
}

pub unsafe fn tegra_cvb_remove_opp_table(
    dev: *mut device,
    table: *const cvb_table,
    max_freq: usize,
) {
    for i in 0..MAX_DVFS_FREQS {
        let entry = &(*table).entries[i];
        if entry.freq == 0 || entry.freq > max_freq {
            break;
        }
        dev_pm_opp_remove(dev, entry.freq);
    }
}

// These helpers and types are provided by the surrounding kernel translation.
extern "C" {
    fn div_round_closest(x: i32, divisor: i32) -> i32;
    fn div_round_up(x: i32, divisor: i32) -> i32;
    fn err_ptr(error: i32) -> *const cvb_table;
    fn dev_pm_opp_add(dev: *mut device, freq: usize, voltage: u64) -> i32;
    fn dev_pm_opp_remove(dev: *mut device, freq: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
