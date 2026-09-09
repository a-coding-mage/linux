/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    S390 fast network checksum routines
 *
 *  S390 version
 *    Copyright IBM Corp. 1999
 *    Author(s): Ulrich Hild        (first version)
 *               Martin Schwidefsky (heavily optimized CKSM version)
 *               D.J. Barrow        (third attempt)
 */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
pub unsafe fn cksm(buff: *const core::ffi::c_void, len: i32, mut sum: __wsum) -> __wsum {
    let mut rp = register_pair {
        even: buff as usize,
        odd: len as usize,
    };

    instrument_read(buff, len);
    kmsan_check_memory(buff, len);
    core::arch::asm!(
        "0: cksm {sum}, {rp}",
        "jo 0b",
        sum = inout(reg) sum,
        rp = inout(reg) rp.pair,
        options(preserves_flags)
    );
    sum
}

extern "C" {
    pub fn csum_partial(buff: *const core::ffi::c_void, len: i32, sum: __wsum) -> __wsum;
    pub fn csum_partial_copy_nocheck(
        src: *const core::ffi::c_void,
        dst: *mut core::ffi::c_void,
        len: i32,
    ) -> __wsum;
}

pub const _HAVE_ARCH_CSUM_AND_COPY: bool = true;

/*
 * Fold a partial checksum without adding pseudo headers.
 */
#[inline]
pub fn csum_fold(sum: __wsum) -> __sum16 {
    let mut csum = sum as u32;

    csum = csum.wrapping_add((csum >> 16) | (csum << 16));
    csum >>= 16;
    (!csum) as __sum16
}

/*
 * This is a version of ip_compute_csum() optimized for IP headers,
 * which always checksums on 4 octet boundaries.
 */
#[inline]
pub unsafe fn ip_fast_csum(iph: *const core::ffi::c_void, mut ihl: u32) -> __sum16 {
    let mut csum: u64 = 0;
    let mut ptr = iph as *const u32;

    csum = csum.wrapping_add(*ptr); ptr = ptr.add(1);
    csum = csum.wrapping_add(*ptr); ptr = ptr.add(1);
    csum = csum.wrapping_add(*ptr); ptr = ptr.add(1);
    csum = csum.wrapping_add(*ptr); ptr = ptr.add(1);
    ihl -= 4;
    while ihl != 0 {
        csum = csum.wrapping_add(*ptr); ptr = ptr.add(1);
        ihl -= 1;
    }
    csum = csum.wrapping_add((csum >> 32) | (csum << 32));
    csum_fold((csum >> 32) as __wsum)
}

/*
 * Computes the checksum of the TCP/UDP pseudo-header.
 * Returns a 32-bit checksum.
 */
#[inline]
pub fn csum_tcpudp_nofold(saddr: __be32, daddr: __be32, len: u32, proto: u8, sum: __wsum) -> __wsum {
    let mut csum = sum as u64;

    csum = csum.wrapping_add(saddr as u32 as u64);
    csum = csum.wrapping_add(daddr as u32 as u64);
    csum = csum.wrapping_add(len as u64);
    csum = csum.wrapping_add(proto as u64);
    csum = csum.wrapping_add((csum >> 32) | (csum << 32));
    (csum >> 32) as __wsum
}

/*
 * Computes the checksum of the TCP/UDP pseudo-header.
 * Returns a 16-bit checksum, already complemented.
 */
#[inline]
pub fn csum_tcpudp_magic(saddr: __be32, daddr: __be32, len: u32, proto: u8, sum: __wsum) -> __sum16 {
    csum_fold(csum_tcpudp_nofold(saddr, daddr, len, proto, sum))
}

/*
 * Used for miscellaneous IP-like checksums, mainly icmp.
 */
#[inline]
pub unsafe fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> __sum16 {
    csum_fold(csum_partial(buff, len, 0))
}

pub const _HAVE_ARCH_IPV6_CSUM: bool = true;

#[inline]
pub unsafe fn csum_ipv6_magic(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    len: u32,
    proto: u8,
    csum: __wsum,
) -> __sum16 {
    let mut sum = csum as u64;

    sum = sum.wrapping_add((*saddr).s6_addr32[0] as u32 as u64);
    sum = sum.wrapping_add((*saddr).s6_addr32[1] as u32 as u64);
    sum = sum.wrapping_add((*saddr).s6_addr32[2] as u32 as u64);
    sum = sum.wrapping_add((*saddr).s6_addr32[3] as u32 as u64);
    sum = sum.wrapping_add((*daddr).s6_addr32[0] as u32 as u64);
    sum = sum.wrapping_add((*daddr).s6_addr32[1] as u32 as u64);
    sum = sum.wrapping_add((*daddr).s6_addr32[2] as u32 as u64);
    sum = sum.wrapping_add((*daddr).s6_addr32[3] as u32 as u64);
    sum = sum.wrapping_add(len as u64);
    sum = sum.wrapping_add(proto as u64);
    sum = sum.wrapping_add((sum >> 32) | (sum << 32));
    csum_fold((sum >> 32) as __wsum)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
