/* SPDX-License-Identifier: GPL-2.0 */

#[inline]
fn swab16(x: u16) -> u16 {
    ((x & (0x00ffu16 as u16)) << 8) |
        ((x & (0xff00u16 as u16)) >> 8)
}

#[inline]
fn swab32(x: u32) -> u32 {
    ((x & (0x000000ffu32 as u32)) << 24) |
        ((x & (0x0000ff00u32 as u32)) << 8) |
        ((x & (0x00ff0000u32 as u32)) >> 8) |
        ((x & (0xff000000u32 as u32)) >> 24)
}

#[inline]
fn swab64(x: u64) -> u64 {
    ((x & (0x00000000000000ffu64 as u64)) << 56) |
        ((x & (0x000000000000ff00u64 as u64)) << 40) |
        ((x & (0x0000000000ff0000u64 as u64)) << 24) |
        ((x & (0x00000000ff000000u64 as u64)) << 8) |
        ((x & (0x000000ff00000000u64 as u64)) >> 8) |
        ((x & (0x0000ff0000000000u64 as u64)) >> 24) |
        ((x & (0x00ff000000000000u64 as u64)) >> 40) |
        ((x & (0xff00000000000000u64 as u64)) >> 56)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
