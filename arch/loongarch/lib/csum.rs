// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2019-2020 Arm Ltd.

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

pub type __u8 = u8;
pub type __u32 = u32;
pub type __wsum = u32;
pub type __sum16 = u16;

extern "C" {
    fn htonl(value: u32) -> u32;
    fn swab32(value: u32) -> u32;
    fn csum_fold(sum: __wsum) -> __sum16;
    fn kasan_check_read(addr: *const core::ffi::c_void, size: usize);
}

#[inline]
unsafe fn accumulate(mut sum: u64, data: u64) -> u64 {
    sum = sum.wrapping_add(data);
    if sum < data {
        sum = sum.wrapping_add(1);
    }
    sum
}

/*
 * We over-read the buffer and this makes KASAN unhappy. Instead, disable
 * instrumentation and call kasan explicitly.
 */
pub unsafe fn do_csum(buff: *const u8, mut len: i32) -> u32 {
    let mut offset: u32;
    let mut shift: u32;
    let sum: u32;
    let mut ptr: *const u64;
    let mut data: u64;
    let mut sum64: u64 = 0;

    if len <= 0 {
        return 0;
    }

    offset = (buff as usize & 7) as u32;
    kasan_check_read(buff as *const core::ffi::c_void, len as usize);
    ptr = buff.sub(offset as usize) as *const u64;
    len = len + offset as i32 - 8;

    shift = offset * 8;
    data = core::ptr::read_unaligned(ptr);
    ptr = ptr.add(1);
    data = (data >> shift) << shift;

    while len > 64 {
        let mut tmp1 = core::ptr::read_unaligned(ptr as *const u128);
        let mut tmp2 = core::ptr::read_unaligned(ptr.add(2) as *const u128);
        let mut tmp3 = core::ptr::read_unaligned(ptr.add(4) as *const u128);
        let mut tmp4 = core::ptr::read_unaligned(ptr.add(6) as *const u128);

        len -= 64;
        ptr = ptr.add(8);

        tmp1 = tmp1.wrapping_add((tmp1 >> 64) | (tmp1 << 64));
        tmp2 = tmp2.wrapping_add((tmp2 >> 64) | (tmp2 << 64));
        tmp3 = tmp3.wrapping_add((tmp3 >> 64) | (tmp3 << 64));
        tmp4 = tmp4.wrapping_add((tmp4 >> 64) | (tmp4 << 64));
        tmp1 = ((tmp1 >> 64) << 64) | (tmp2 >> 64);
        tmp1 = tmp1.wrapping_add((tmp1 >> 64) | (tmp1 << 64));
        tmp3 = ((tmp3 >> 64) << 64) | (tmp4 >> 64);
        tmp3 = tmp3.wrapping_add((tmp3 >> 64) | (tmp3 << 64));
        tmp1 = ((tmp1 >> 64) << 64) | (tmp3 >> 64);
        tmp1 = tmp1.wrapping_add((tmp1 >> 64) | (tmp1 << 64));
        tmp1 = ((tmp1 >> 64) << 64) | sum64 as u128;
        tmp1 = tmp1.wrapping_add((tmp1 >> 64) | (tmp1 << 64));
        sum64 = (tmp1 >> 64) as u64;
    }
    while len > 8 {
        sum64 = accumulate(sum64, data);
        let tmp = core::ptr::read_unaligned(ptr as *const u128);
        len -= 16;
        ptr = ptr.add(2);
        data = (tmp >> 64) as u64;
        sum64 = accumulate(sum64, tmp as u64);
    }
    if len > 0 {
        sum64 = accumulate(sum64, data);
        data = core::ptr::read_unaligned(ptr);
        len -= 8;
    }
    shift = (len * -8) as u32;
    data = (data << shift) >> shift;
    sum64 = accumulate(sum64, data);

    sum64 = sum64.wrapping_add((sum64 >> 32) | (sum64 << 32));
    sum = (sum64 >> 32) as u32;
    let mut folded = sum.wrapping_add((sum >> 16) | (sum << 16));
    if offset & 1 != 0 {
        return swab32(folded as u32);
    }
    folded >> 16
}

pub unsafe fn csum_ipv6_magic(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    len: __u32,
    proto: __u8,
    csum: __wsum,
) -> __sum16 {
    let mut src = core::ptr::read_unaligned((*saddr).s6_addr.as_ptr() as *const u128);
    let mut dst = core::ptr::read_unaligned((*daddr).s6_addr.as_ptr() as *const u128);
    let mut sum = csum as u64;

    sum = sum.wrapping_add(htonl(len) as u64);
    sum = sum.wrapping_add((proto as u32 as u64) << 24);
    src = src.wrapping_add((src >> 64) | (src << 64));
    dst = dst.wrapping_add((dst >> 64) | (dst << 64));
    sum = accumulate(sum, (src >> 64) as u64);
    sum = accumulate(sum, (dst >> 64) as u64);
    sum = sum.wrapping_add((sum >> 32) | (sum << 32));
    csum_fold((sum >> 32) as __wsum)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
