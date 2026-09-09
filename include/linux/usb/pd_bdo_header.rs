/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2015-2017 Google, Inc
 */

/* BDO : BIST Data Object */
pub const BDO_MODE_RECV: u32 = 0 << 28;
pub const BDO_MODE_TRANSMIT: u32 = 1 << 28;
pub const BDO_MODE_COUNTERS: u32 = 2 << 28;
pub const BDO_MODE_CARRIER0: u32 = 3 << 28;
pub const BDO_MODE_CARRIER1: u32 = 4 << 28;
pub const BDO_MODE_CARRIER2: u32 = 5 << 28;
pub const BDO_MODE_CARRIER3: u32 = 6 << 28;
pub const BDO_MODE_EYE: u32 = 7 << 28;
pub const BDO_MODE_TESTDATA: u32 = 8u32 << 28;

#[inline]
pub const fn BDO_MODE_MASK(mode: u32) -> u32 {
    mode & 0xf0000000
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
