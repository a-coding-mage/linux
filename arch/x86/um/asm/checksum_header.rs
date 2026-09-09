/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external: linux/string.h, linux/in6.h, and linux/uaccess.h.

extern "C" {
    pub fn csum_partial(buff: *const core::ffi::c_void, len: i32, sum: __wsum) -> __wsum;

    // Do not call this directly. Declared for export type visibility.
    pub fn csum_partial_copy_generic(
        src: *const core::ffi::c_void,
        dst: *mut core::ffi::c_void,
        len: i32,
    ) -> __wsum;
}

/**
 * csum_fold - Fold and invert a 32bit checksum.
 * sum: 32bit unfolded sum
 *
 * Fold a 32bit running checksum to 16bit and invert it. This is usually
 * the last step before putting a checksum into a packet.
 * Make sure not to mix with 64bit checksums.
 */
#[inline]
pub fn csum_fold(mut sum: __wsum) -> __sum16 {
    let upper = sum & 0xffff0000;
    let addend = sum << 16;
    let (value, carry) = upper.overflowing_add(addend);
    sum = value.wrapping_add(0xffff).wrapping_add(carry as u32);
    ((!sum) >> 16) as __sum16
}

/** Compute an IPv4 pseudo header checksum, without folding. */
#[inline]
pub fn csum_tcpudp_nofold(
    saddr: __be32,
    daddr: __be32,
    len: __u32,
    proto: __u8,
    mut sum: __wsum,
) -> __wsum {
    let (v1, c1) = sum.overflowing_add(daddr);
    sum = v1;
    let (v2, c2) = sum.overflowing_add(saddr);
    sum = v2;
    let value = (len.wrapping_add(proto as u32)) << 8;
    let (v3, c3) = sum.overflowing_add(value);
    sum = v3;
    sum = sum
        .wrapping_add((c1 as u32) + (c2 as u32) + (c3 as u32));
    sum
}

/* Computes the TCP/UDP pseudo-header checksum, already complemented. */
#[inline]
pub fn csum_tcpudp_magic(
    saddr: __be32,
    daddr: __be32,
    len: __u32,
    proto: __u8,
    sum: __wsum,
) -> __sum16 {
    csum_fold(csum_tcpudp_nofold(saddr, daddr, len, proto, sum))
}

/** Compute the IPv4 header checksum. */
#[inline]
pub unsafe fn ip_fast_csum(iph: *const u8, ihl: u32) -> __sum16 {
    let mut sum = core::ptr::read_unaligned(iph as *const u32);
    let mut words = ihl.wrapping_sub(1);
    if words > 0 {
        let mut p = iph.add(4);
        while words != 0 {
            let word = core::ptr::read_unaligned(p as *const u32);
            let (value, carry) = sum.overflowing_add(word);
            sum = value.wrapping_add(carry as u32);
            p = p.add(4);
            words = words.wrapping_sub(1);
        }
        let (value, carry) = sum.overflowing_add(0);
        sum = value.wrapping_add(carry as u32);
        sum = (sum & 0xffff).wrapping_add(sum >> 16);
        sum = sum.wrapping_add((sum >> 16) & 1);
        sum = !sum;
    }
    sum as __sum16
}

// CONFIG_X86_32 selects checksum_32.h; otherwise checksum_64.h supplies the
// architecture-specific declarations. Those dependencies are external here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
