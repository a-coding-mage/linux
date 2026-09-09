/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * Joern Rennecke  <joern.rennecke@embecosm.com>: Jan 2012
 *  -Insn Scheduling improvements to csum core routines.
 *      = csum_fold( ) largely derived from ARM version.
 *      = ip_fast_cum( ) to have module scheduling
 *  -gcc 4.4.x broke networking. Alias analysis needed to be primed.
 *   worked around by adding memory clobber to ip_fast_csum( )
 *
 * vineetg: May 2010
 *  -Rewrote ip_fast_cscum( ) and csum_fold( ) with fast inline asm
 */

/* The C header guard and include of asm-generic/checksum.h are intentionally
 * omitted; required types and external symbols are supplied by dependencies. */

/*
 * Fold a partial checksum
 *
 *  The 2 swords comprising the 32bit sum are added, any carry to 16th bit
 *  added back and final sword result inverted.
 */
#[inline]
pub fn csum_fold(mut s: __wsum) -> __sum16 {
    let r: u32 = (s as u32).wrapping_shl(16) | (s as u32).wrapping_shr(16); // ror
    s = (!s).wrapping_sub(r as __wsum);
    (s as u32).wrapping_shr(16) as __sum16
}

/*
 * This is a version of ip_compute_csum() optimized for IP headers,
 * which always checksum on 4 octet boundaries.
 *
 * The original implementation uses ARC inline assembly. The following keeps
 * its word loads, carry propagation, pointer advancement, and checksum fold.
 */
#[inline]
pub unsafe fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: u32) -> __sum16 {
    let mut ptr = iph as *const u8;
    let words = ihl as usize;
    let mut sum: u32 = 0;

    for _ in 0..words {
        let word = core::ptr::read_unaligned(ptr as *const u32);
        ptr = ptr.add(4);
        let (v, carry) = sum.overflowing_add(word);
        sum = v.wrapping_add(carry as u32);
    }

    csum_fold(sum as __wsum)
}

/*
 * TCP pseudo Header is 12 bytes:
 * SA [4], DA [4], zeroes [1], Proto[1], TCP Seg(hdr+data) Len [2]
 */
#[inline]
pub fn csum_tcpudp_nofold(
    saddr: __be32,
    daddr: __be32,
    len: __u32,
    proto: __u8,
    mut sum: __wsum,
) -> __wsum {
    let len_word = if cfg!(target_endian = "big") {
        len
    } else {
        len.wrapping_shl(8)
    };

    for value in [saddr as __wsum, daddr as __wsum, len_word as __wsum,
                  htons(proto) as __wsum] {
        let (v, carry) = sum.overflowing_add(value);
        sum = v.wrapping_add(carry as __wsum);
    }
    sum
}

/* C self-referential macro definitions: the Rust functions above are the
 * corresponding exported names. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
