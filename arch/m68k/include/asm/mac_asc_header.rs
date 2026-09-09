/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	Apple Sound Chip
 */

/*
 *	ASC offsets and controls
 */

pub const ASC_BUF_BASE: i32 = 0x00; // RAM buffer offset
pub const ASC_BUF_SIZE: i32 = 0x800;

pub const ASC_CONTROL: i32 = 0x800;
pub const ASC_CONTROL_OFF: i32 = 0x00;
#[inline]
pub const fn asc_freq(chan: i32, byte: i32) -> i32 {
    (0x810) + (chan << 3) + byte
}
pub const ASC_ENABLE: i32 = 0x801;
pub const ASC_ENABLE_SAMPLE: i32 = 0x02;
pub const ASC_MODE: i32 = 0x802;
pub const ASC_MODE_SAMPLE: i32 = 0x02;

pub const ASC_VOLUME: i32 = 0x806;
pub const ASC_CHAN: i32 = 0x807; // ???

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
