/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <linux/in6.h>, <asm/byteorder.h>, and
// <asm-generic/checksum.h>; their declarations are supplied by the surrounding
// translation unit.

#[inline]
pub unsafe fn csum_fold(mut csum: __wsum) -> __sum16 {
    let tmp: u32 = csum as u32;
    csum = (((csum as u32).rotate_right(16)).wrapping_add(tmp) >> 16) as __wsum;
    (!csum as u16) as __sum16
}

#[inline]
pub unsafe fn csum_tcpudp_nofold(
    saddr: __be32,
    daddr: __be32,
    len: u16,
    proto: u16,
    mut sum: __wsum,
) -> __wsum {
    let mut value = sum as u32;
    let mut carry: u32 = 0;

    let (v, c) = value.overflowing_add(saddr as u32);
    value = v;
    carry = c as u32;

    let (v, c) = value.overflowing_add(daddr as u32);
    let (v, c2) = v.overflowing_add(carry);
    value = v;
    carry = (c || c2) as u32;

    #[cfg(target_endian = "big")]
    let pseudo = (proto as u32).wrapping_add(len as u32);
    #[cfg(not(target_endian = "big"))]
    let pseudo = (proto as u32).wrapping_add(len as u32).wrapping_shl(8);

    let (v, c) = value.overflowing_add(pseudo);
    let (v, c2) = v.overflowing_add(carry);
    value = v;
    carry = (c || c2) as u32;

    value = value.wrapping_add(carry);
    sum = value as __wsum;
    sum
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
