// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018, NVIDIA CORPORATION.  All rights reserved.
 */

// Dependency: <asm/div64.h>
// Dependency: "clk.h"

#[inline]
fn div_mask(w: u8) -> u64 {
    ((1u64 << w) - 1)
}

pub fn div_frac_get(
    rate: u64,
    parent_rate: u32,
    width: u8,
    frac_width: u8,
    flags: u8,
) -> i32 {
    let mut divider_ux1 = parent_rate as u64;
    let mul: u64;

    if rate == 0 {
        return 0;
    }

    mul = 1u64 << frac_width;

    if (flags & TEGRA_DIVIDER_INT) == 0 {
        divider_ux1 = divider_ux1.wrapping_mul(mul);
    }

    if (flags & TEGRA_DIVIDER_ROUND_UP) != 0 {
        divider_ux1 = divider_ux1.wrapping_add(rate.wrapping_sub(1));
    }

    divider_ux1 /= rate;

    if (flags & TEGRA_DIVIDER_INT) != 0 {
        divider_ux1 = divider_ux1.wrapping_mul(mul);
    }

    if divider_ux1 < mul {
        return 0;
    }

    divider_ux1 = divider_ux1.wrapping_sub(mul);

    if divider_ux1 > div_mask(width) {
        return div_mask(width) as i32;
    }

    divider_ux1 as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
