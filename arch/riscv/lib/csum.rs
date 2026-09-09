// SPDX-License-Identifier: GPL-2.0
/*
 * Checksum library
 *
 * Influenced by arch/arm64/lib/csum.c
 * Copyright (C) 2023-2024 Rivos Inc.
 */

// The definitions referenced below are supplied by the surrounding kernel.

#[cfg(not(CONFIG_32BIT))]
pub unsafe fn csum_ipv6_magic(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    len: u32,
    proto: u8,
    csum: __wsum,
) -> __sum16 {
    let mut sum = csum as libc::c_ulong;

    sum = sum.wrapping_add((*saddr).s6_addr32[0] as libc::c_ulong);
    sum = sum.wrapping_add((*saddr).s6_addr32[1] as libc::c_ulong);
    sum = sum.wrapping_add((*saddr).s6_addr32[2] as libc::c_ulong);
    sum = sum.wrapping_add((*saddr).s6_addr32[3] as libc::c_ulong);
    sum = sum.wrapping_add((*daddr).s6_addr32[0] as libc::c_ulong);
    sum = sum.wrapping_add((*daddr).s6_addr32[1] as libc::c_ulong);
    sum = sum.wrapping_add((*daddr).s6_addr32[2] as libc::c_ulong);
    sum = sum.wrapping_add((*daddr).s6_addr32[3] as libc::c_ulong);

    sum = sum.wrapping_add(u32::from_be(len) as libc::c_ulong);
    sum = sum.wrapping_add(u32::from_be(proto as u32) as libc::c_ulong);

    // The ZBB inline-assembly fast path is architecture/build dependent;
    // preserve its operation with the equivalent scalar folding below.
    sum = sum.wrapping_add(sum.rotate_right(32));
    sum >>= 32;
    csum_fold(sum as __wsum)
}

#[cfg(CONFIG_32BIT)]
const OFFSET_MASK: usize = 3;
#[cfg(CONFIG_64BIT)]
const OFFSET_MASK: usize = 7;

#[inline]
unsafe fn do_csum_common(mut ptr: *const libc::c_ulong, end: *const libc::c_ulong, mut data: libc::c_ulong) -> libc::c_ulong {
    let mut csum: libc::c_ulong = 0;
    let mut carry: libc::c_ulong = 0;
    while ptr < end {
        csum = csum.wrapping_add(data);
        carry = carry.wrapping_add((csum < data) as libc::c_ulong);
        data = *ptr;
        ptr = ptr.add(1);
    }
    let shift = ((ptr as isize - end as isize) * 8) as u32;
    #[cfg(target_endian = "little")]
    { data = (data << shift) >> shift; }
    #[cfg(target_endian = "big")]
    { data = (data >> shift) << shift; }
    csum = csum.wrapping_add(data);
    carry = carry.wrapping_add((csum < data) as libc::c_ulong);
    csum = csum.wrapping_add(carry);
    csum.wrapping_add((csum < carry) as libc::c_ulong)
}

#[inline]
unsafe fn do_csum_with_alignment(buff: *const u8, len: i32) -> u32 {
    let offset = (buff as usize) & OFFSET_MASK;
    kasan_check_read(buff, len);
    let mut ptr = (buff.sub(offset)) as *const libc::c_ulong;
    let shift = (offset * 8) as u32;
    let mut data = *ptr;
    ptr = ptr.add(1);
    #[cfg(target_endian = "little")]
    { data = (data >> shift) << shift; }
    #[cfg(target_endian = "big")]
    { data = (data << shift) >> shift; }
    let end = buff.add(len as usize) as *const libc::c_ulong;
    let mut csum = do_csum_common(ptr, end, data);
    #[cfg(not(CONFIG_32BIT))]
    { csum = csum.wrapping_add(csum.rotate_right(32)); csum >>= 32; }
    csum = (csum as u32).wrapping_add((csum as u32).rotate_right(16));
    if offset & 1 != 0 { swab32(csum) as u16 as u32 } else { csum >> 16 }
}

#[inline]
unsafe fn do_csum_no_alignment(buff: *const u8, len: i32) -> u32 {
    let mut ptr = buff as *const libc::c_ulong;
    let data = *ptr;
    ptr = ptr.add(1);
    kasan_check_read(buff, len);
    let end = buff.add(len as usize) as *const libc::c_ulong;
    let mut csum = do_csum_common(ptr, end, data);
    #[cfg(not(CONFIG_32BIT))]
    { csum = csum.wrapping_add(csum.rotate_right(32)); csum >>= 32; }
    csum = (csum as u32).wrapping_add((csum as u32).rotate_right(16));
    csum >> 16
}

pub unsafe fn do_csum(buff: *const u8, len: i32) -> u32 {
    if len <= 0 { return 0; }
    if has_fast_unaligned_accesses() || ((buff as usize & OFFSET_MASK) == 0) {
        do_csum_no_alignment(buff, len)
    } else {
        do_csum_with_alignment(buff, len)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
