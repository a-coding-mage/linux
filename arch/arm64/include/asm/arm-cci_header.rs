/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm64/include/asm/arm-cci.h
 *
 * Copyright (C) 2015 ARM Ltd.
 */

#[inline]
pub fn platform_has_secure_cci_access() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
