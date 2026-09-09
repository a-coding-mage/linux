/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/bitops.h supplies the ror32 operation.

/*
 * computes the checksum of a memory block at buff, length len,
 * and adds in "sum" (32-bit)
 *
 * returns a 32-bit number suitable for feeding into itself
 * or csum_tcpudp_magic
 *
 * this function must be called with even lengths, except
 * for the last fragment, which may be odd
 *
 * it's best to have buff aligned on a 32-bit boundary
 */
unsafe extern "C" {
    pub fn csum_partial(buff: *const core::ffi::c_void, len: core::ffi::c_int, sum: __wsum) -> __wsum;
}

/*
 * This is a version of ip_compute_csum() optimized for IP headers,
 * which always checksum on 4 octet boundaries.
 */
unsafe extern "C" {
    pub fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: core::ffi::c_uint) -> __sum16;
}

/*
 * Fold a partial checksum
 */
#[inline]
pub fn csum_fold(csum: __wsum) -> __sum16 {
    let sum: u32 = csum as u32;
    ((!sum).wrapping_sub(sum.rotate_right(16)) >> 16) as __sum16
}

/*
 * computes the checksum of the TCP/UDP pseudo-header
 * returns a 16-bit checksum, already complemented
 */
unsafe extern "C" {
    pub fn csum_tcpudp_nofold(
        saddr: __be32,
        daddr: __be32,
        len: __u32,
        proto: __u8,
        sum: __wsum,
    ) -> __wsum;
}

#[inline]
pub unsafe fn csum_tcpudp_magic(
    saddr: __be32,
    daddr: __be32,
    len: __u32,
    proto: __u8,
    sum: __wsum,
) -> __sum16 {
    csum_fold(csum_tcpudp_nofold(saddr, daddr, len, proto, sum))
}

/*
 * this routine is used for miscellaneous IP-like checksums, mainly
 * in icmp.c
 */
unsafe extern "C" {
    pub fn ip_compute_csum(buff: *const core::ffi::c_void, len: core::ffi::c_int) -> __sum16;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
