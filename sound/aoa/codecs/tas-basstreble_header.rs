/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This file is only included exactly once!
 *
 * The tables here are derived from the tas3004 datasheet,
 * modulo typo corrections and some smoothing...
 */

pub const TAS3004_TREBLE_MIN: i32 = 0;
pub const TAS3004_TREBLE_MAX: i32 = 72;
pub const TAS3004_BASS_MIN: i32 = 0;
pub const TAS3004_BASS_MAX: i32 = 72;
pub const TAS3004_TREBLE_ZERO: i32 = 36;
pub const TAS3004_BASS_ZERO: i32 = 36;

static TAS3004_TREBLE_TABLE: [u8; 73] = [
    150, /* -18 dB */
    149,
    148,
    147,
    146,
    145,
    144,
    143,
    142,
    141,
    140,
    139,
    138,
    137,
    136,
    135,
    134,
    133,
    132,
    131,
    130,
    129,
    128,
    127,
    126,
    125,
    124,
    123,
    122,
    121,
    120,
    119,
    118,
    117,
    116,
    115,
    114, /* 0 dB */
    113,
    112,
    111,
    109,
    108,
    107,
    105,
    104,
    103,
    101,
    99,
    98,
    96,
    93,
    91,
    89,
    86,
    83,
    81,
    77,
    74,
    71,
    67,
    63,
    59,
    54,
    49,
    44,
    38,
    32,
    26,
    19,
    10,
    4,
    2,
    1, /* +18 dB */
];

#[inline]
pub unsafe fn tas3004_treble(idx: core::ffi::c_int) -> u8 {
    unsafe { *TAS3004_TREBLE_TABLE.get_unchecked(idx as usize) }
}

/* I only save the difference here to the treble table
 * so that the binary is smaller...
 * I have also ignored completely differences of
 * +/- 1
 */
static TAS3004_BASS_DIFF_TO_TREBLE: [i8; 23] = [
    2, /* 7 dB, offset 50 */
    2,
    2,
    2,
    2,
    1,
    2,
    2,
    2,
    3,
    4,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    14,
    13,
    8,
    1, /* 18 dB */
];

#[inline]
pub unsafe fn tas3004_bass(idx: core::ffi::c_int) -> u8 {
    let mut result = unsafe { *TAS3004_TREBLE_TABLE.get_unchecked(idx as usize) };

    if idx >= 50 {
        result = result.wrapping_add(unsafe {
            *TAS3004_BASS_DIFF_TO_TREBLE.get_unchecked((idx - 50) as usize) as u8
        });
    }
    result
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
