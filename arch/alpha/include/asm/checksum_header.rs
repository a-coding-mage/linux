/* SPDX-License-Identifier: GPL-2.0 */

// Dependency preserved from <linux/in6.h>.

/*
 *	This is a version of ip_compute_csum() optimized for IP headers,
 *	which always checksum on 4 octet boundaries.
 */
unsafe extern "C" {
    pub fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: core::ffi::c_uint) -> __sum16;
}

/*
 * computes the checksum of the TCP/UDP pseudo-header
 * returns a 16-bit checksum, already complemented
 */
unsafe extern "C" {
    pub fn csum_tcpudp_magic(
        saddr: __be32,
        daddr: __be32,
        len: __u32,
        proto: __u8,
        sum: __wsum,
    ) -> __sum16;

    pub fn csum_tcpudp_nofold(
        saddr: __be32,
        daddr: __be32,
        len: __u32,
        proto: __u8,
        sum: __wsum,
    ) -> __wsum;
}

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
    pub fn csum_partial(
        buff: *const core::ffi::c_void,
        len: core::ffi::c_int,
        sum: __wsum,
    ) -> __wsum;
}

/*
 * the same as csum_partial, but copies from src while it
 * checksums
 *
 * here even more important to align src and dst on a 32-bit (or even
 * better 64-bit) boundary
 */
pub const _HAVE_ARCH_COPY_AND_CSUM_FROM_USER: bool = true;
pub const _HAVE_ARCH_CSUM_AND_COPY: bool = true;

unsafe extern "C" {
    pub fn csum_and_copy_from_user(
        src: *const core::ffi::c_void,
        dst: *mut core::ffi::c_void,
        len: core::ffi::c_int,
    ) -> __wsum;

    pub fn csum_partial_copy_nocheck(
        src: *const core::ffi::c_void,
        dst: *mut core::ffi::c_void,
        len: core::ffi::c_int,
    ) -> __wsum;
}

/*
 * this routine is used for miscellaneous IP-like checksums, mainly
 * in icmp.c
 */
unsafe extern "C" {
    pub fn ip_compute_csum(buff: *const core::ffi::c_void, len: core::ffi::c_int) -> __sum16;
}

/*
 *	Fold a partial checksum without adding pseudo headers
 */
pub unsafe fn csum_fold(csum: __wsum) -> __sum16 {
    let mut sum: u32 = csum as u32;
    sum = (sum & 0xffff) + (sum >> 16);
    sum = (sum & 0xffff) + (sum >> 16);
    (!sum) as __sum16
}

pub const _HAVE_ARCH_IPV6_CSUM: bool = true;

unsafe extern "C" {
    pub fn csum_ipv6_magic(
        saddr: *const in6_addr,
        daddr: *const in6_addr,
        len: __u32,
        proto: __u8,
        sum: __wsum,
    ) -> __sum16;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
