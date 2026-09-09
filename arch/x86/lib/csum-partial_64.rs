// SPDX-License-Identifier: GPL-2.0
/*
 * arch/x86_64/lib/csum-partial.c
 *
 * This file contains network checksum routines that are better done
 * in an architecture-specific manner due to speed.
 */

use core::ffi::c_void;

// These types and functions are supplied by the surrounding kernel code.
extern "C" {
    fn load_unaligned_zeropad(addr: *const c_void) -> u64;
    fn csum_fold(sum: __wsum) -> __sum16;
}

#[allow(non_camel_case_types)]
type __wsum = u32;
#[allow(non_camel_case_types)]
type __sum16 = u16;

#[inline]
fn csum_finalize_sum(temp64: u64) -> __wsum {
    temp64.wrapping_add(temp64.rotate_right(32)).wrapping_shr(32) as __wsum
}

#[inline]
unsafe fn add_with_carry(sum: u64, value: u64, carry: &mut bool) -> u64 {
    let (result, c1) = sum.overflowing_add(value);
    let (result, c2) = result.overflowing_add(*carry as u64);
    *carry = c1 || c2;
    result
}

#[inline]
unsafe fn update_csum_40b(mut sum: u64, m: *const u64) -> u64 {
    let mut carry = false;
    sum = add_with_carry(sum, core::ptr::read_unaligned(m.add(0)), &mut carry);
    sum = add_with_carry(sum, core::ptr::read_unaligned(m.add(1)), &mut carry);
    sum = add_with_carry(sum, core::ptr::read_unaligned(m.add(2)), &mut carry);
    sum = add_with_carry(sum, core::ptr::read_unaligned(m.add(3)), &mut carry);
    sum = add_with_carry(sum, core::ptr::read_unaligned(m.add(4)), &mut carry);
    let (sum2, _) = sum.overflowing_add(carry as u64);
    sum2
}

/*
 * Do a checksum on an arbitrary memory area.
 * Returns a 32bit checksum.
 *
 * This isn't as time critical as it used to be because many NICs
 * do hardware checksumming these days.
 *
 * Still, with CHECKSUM_COMPLETE this is called to compute
 * checksums on IPv6 headers (40 bytes) and other small parts.
 * it's best to have buff aligned on a 64-bit boundary
 */
#[no_mangle]
pub unsafe extern "C" fn csum_partial(mut buff: *const c_void, mut len: i32, sum: __wsum) -> __wsum {
    let mut temp64 = sum as u64;

    /* Do two 40-byte chunks in parallel to get better ILP */
    if len >= 80 {
        let mut temp64_2 = 0u64;
        loop {
            temp64 = update_csum_40b(temp64, buff as *const u64);
            temp64_2 = update_csum_40b(temp64_2, buff.add(40) as *const u64);
            buff = buff.add(80);
            len -= 80;
            if len < 80 { break; }
        }
        let (v, c) = temp64.overflowing_add(temp64_2);
        temp64 = v.wrapping_add(c as u64);
    }

    /*
     * len == 40 is the hot case due to IPv6 headers, so return
     * early for that exact case without checking the tail bytes.
     */
    if len >= 40 {
        temp64 = update_csum_40b(temp64, buff as *const u64);
        len -= 40;
        if len == 0 { return csum_finalize_sum(temp64); }
        buff = buff.add(40);
    }

    let mut carry = false;
    if (len & 32) != 0 {
        for i in 0..4 { temp64 = add_with_carry(temp64, core::ptr::read_unaligned((buff as *const u64).add(i)), &mut carry); }
        temp64 = temp64.wrapping_add(carry as u64); carry = false;
        buff = buff.add(32);
    }
    if (len & 16) != 0 {
        for i in 0..2 { temp64 = add_with_carry(temp64, core::ptr::read_unaligned((buff as *const u64).add(i)), &mut carry); }
        temp64 = temp64.wrapping_add(carry as u64); carry = false;
        buff = buff.add(16);
    }
    if (len & 8) != 0 {
        temp64 = add_with_carry(temp64, core::ptr::read_unaligned(buff as *const u64), &mut carry);
        temp64 = temp64.wrapping_add(carry as u64); carry = false;
        buff = buff.add(8);
    }
    if (len & 7) != 0 {
        let shift = ((-len << 3) & 63) as u32;
        let trail = (load_unaligned_zeropad(buff) << shift) >> shift;
        temp64 = add_with_carry(temp64, trail, &mut carry);
        temp64 = temp64.wrapping_add(carry as u64);
    }
    csum_finalize_sum(temp64)
}

#[no_mangle]
pub unsafe extern "C" fn ip_compute_csum(buff: *const c_void, len: i32) -> __sum16 {
    csum_fold(csum_partial(buff, len, 0))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
