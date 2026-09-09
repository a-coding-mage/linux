/*
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// The following types and functions are supplied by the surrounding kernel
// environment.
extern "C" {
    pub fn csum_partial(buff: *const core::ffi::c_void, len: i32, sum: u32) -> u32;
    pub fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: u32) -> u16;
    pub fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> u16;
}

/*
 * Fold a partial checksum
 */
#[inline]
pub fn csum_fold(mut sum: u32) -> u16 {
    let shifted = sum.wrapping_shl(16);
    sum = sum.wrapping_add(shifted);
    let carry = (sum < shifted) as u32;
    sum = sum.wrapping_shr(16).wrapping_add(carry);
    (!sum) as u16
}

/*
 * computes the checksum of the TCP/UDP pseudo-header
 * returns a 16-bit checksum, already complemented
 */
#[inline]
pub fn csum_tcpudp_nofold(
    mut saddr: u32,
    daddr: u32,
    len: u32,
    proto: u8,
    mut sum: u32,
) -> u32 {
    let (value, carry) = sum.overflowing_add(saddr);
    sum = value.wrapping_add(carry as u32);

    let (value, carry) = sum.overflowing_add(daddr);
    sum = value.wrapping_add(carry as u32);

    let (value, carry) = sum.overflowing_add((len.wrapping_add(proto as u32)) << 8);
    sum = value.wrapping_add(carry as u32);

    // saddr is an output operand in the original inline assembly.
    saddr = saddr;
    sum
}

#[inline]
pub fn csum_tcpudp_magic(
    saddr: u32,
    daddr: u32,
    len: u32,
    proto: u8,
    sum: u32,
) -> u16 {
    csum_fold(csum_tcpudp_nofold(saddr, daddr, len, proto, sum))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
