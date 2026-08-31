/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from include/tools/endian.h.
 * C dependency intent: <byteswap.h> supplies __bswap_16/32/64.
 */

#[cfg(target_endian = "little")]
#[inline]
pub const fn htole16(x: u16) -> u16 {
    x
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn htole32(x: u32) -> u32 {
    x
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn htole64(x: u64) -> u64 {
    x
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn le16toh(x: u16) -> u16 {
    x
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn le32toh(x: u32) -> u32 {
    x
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn le64toh(x: u64) -> u64 {
    x
}

#[cfg(not(target_endian = "little"))]
#[inline]
pub const fn htole16(x: u16) -> u16 {
    x.swap_bytes()
}

#[cfg(not(target_endian = "little"))]
#[inline]
pub const fn htole32(x: u32) -> u32 {
    x.swap_bytes()
}

#[cfg(not(target_endian = "little"))]
#[inline]
pub const fn htole64(x: u64) -> u64 {
    x.swap_bytes()
}

#[cfg(not(target_endian = "little"))]
#[inline]
pub const fn le16toh(x: u16) -> u16 {
    x.swap_bytes()
}

#[cfg(not(target_endian = "little"))]
#[inline]
pub const fn le32toh(x: u32) -> u32 {
    x.swap_bytes()
}

#[cfg(not(target_endian = "little"))]
#[inline]
pub const fn le64toh(x: u64) -> u64 {
    x.swap_bytes()
}
