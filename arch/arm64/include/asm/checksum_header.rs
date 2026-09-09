/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 ARM Ltd.
 */

// Dependency supplied by the surrounding kernel translation.

pub const _HAVE_ARCH_IPV6_CSUM: bool = true;

extern "C" {
    pub fn csum_ipv6_magic(
        saddr: *const in6_addr,
        daddr: *const in6_addr,
        len: __u32,
        proto: __u8,
        sum: __wsum,
    ) -> __sum16;
}

pub unsafe fn csum_fold(csum: __wsum) -> __sum16 {
    let mut sum: u32 = csum as u32;
    sum = sum.wrapping_add((sum >> 16) | (sum << 16));
    !(sum >> 16) as __sum16
}

pub unsafe fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: u32) -> __sum16 {
    let mut tmp: u128 = core::ptr::read_unaligned(iph as *const u128);
    let mut sum: u64;
    let mut n = ihl as i32; /* we want it signed */

    iph = (iph as *const u8).add(16) as *const core::ffi::c_void;
    n -= 4;
    tmp = tmp.wrapping_add((tmp >> 64) | (tmp << 64));
    sum = (tmp >> 64) as u64;
    loop {
        sum = sum.wrapping_add(core::ptr::read_unaligned(iph as *const u32) as u64);
        iph = (iph as *const u8).add(4) as *const core::ffi::c_void;
        n -= 1;
        if n <= 0 {
            break;
        }
    }

    sum = sum.wrapping_add((sum >> 32) | (sum << 32));
    csum_fold((sum >> 32) as __wsum)
}

extern "C" {
    pub fn do_csum(buff: *const u8, len: i32) -> u32;
}

// The declarations from <asm-generic/checksum.h> are supplied by another
// translated dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
