// SPDX-License-Identifier: GPL-2.0
/*
 * arch/alpha/lib/checksum.c
 *
 * This file contains network checksum routines that are better done
 * in an architecture-specific manner due to speed..
 * Comments in other versions indicate that the algorithms are from RFC1071
 *
 * accelerated versions (and 21264 assembly versions ) contributed by
 *	Rick Gorton	<rick.gorton@alpha-processor.com>
 */

#[repr(C)]
union From64Value {
    ul: u64,
    ui: [u32; 2],
    us: [u16; 4],
}

unsafe fn from64to16(x: u64) -> u16 {
    /* Using extract instructions is a bit more efficient
       than the original shift/bitmask version.  */
    let mut in_v = From64Value { ul: x };
    let mut tmp_v = From64Value { ul: 0 };
    let mut out_v = From64Value { ul: 0 };

    tmp_v.ul = (in_v.ui[0] as u64).wrapping_add(in_v.ui[1] as u64);

    /* Since the bits of tmp_v.sh[3] are going to always be zero,
       we don't have to bother to add that in.  */
    out_v.ul = (tmp_v.us[0] as u64)
        .wrapping_add(tmp_v.us[1] as u64)
        .wrapping_add(tmp_v.us[2] as u64);

    /* Similarly, out_v.us[2] is always zero for the final add.  */
    out_v.us[0].wrapping_add(out_v.us[1])
}

/*
 * computes the checksum of the TCP/UDP pseudo-header
 * returns a 16-bit checksum, already complemented.
 */
pub unsafe fn csum_tcpudp_magic(
    saddr: __be32,
    daddr: __be32,
    len: __u32,
    proto: __u8,
    sum: __wsum,
) -> __sum16 {
    (!from64to16(
        (saddr as u64)
            .wrapping_add(daddr as u64)
            .wrapping_add(sum as u64)
            .wrapping_add(((len.wrapping_add(proto as __u32)) << 8) as u64),
    )) as __sum16
}

pub unsafe fn csum_tcpudp_nofold(
    saddr: __be32,
    daddr: __be32,
    len: __u32,
    proto: __u8,
    sum: __wsum,
) -> __wsum {
    let mut result = (saddr as u64)
        .wrapping_add(daddr as u64)
        .wrapping_add(sum as u64)
        .wrapping_add(((len.wrapping_add(proto as __u32)) << 8) as u64);

    /* Fold down to 32-bits so we don't lose in the typedef-less
       network stack.  */
    /* 64 to 33 */
    result = (result & 0xffff_ffff).wrapping_add(result >> 32);
    /* 33 to 32 */
    result = (result & 0xffff_ffff).wrapping_add(result >> 32);
    result as __wsum
}

/*
 * Do a 64-bit checksum on an arbitrary memory area..
 *
 * This isn't a great routine, but it's not _horrible_ either. The
 * inner loop could be unrolled a bit further, and there are better
 * ways to do the carry, but this is reasonable.
 */
unsafe fn do_csum(mut buff: *const u8, mut len: i32) -> u64 {
    let mut odd: u64;
    let mut count: i32;
    let mut result: u64 = 0;

    if len <= 0 {
        return result;
    }
    odd = 1 & (buff as usize as u64);
    if odd != 0 {
        result = ((*buff as u64) << 8);
        len -= 1;
        buff = buff.add(1);
    }
    count = len >> 1; /* nr of 16-bit words.. */
    if count != 0 {
        if 2 & (buff as usize) != 0 {
            result = result.wrapping_add(*(buff as *const u16) as u64);
            count -= 1;
            len -= 2;
            buff = buff.add(2);
        }
        count >>= 1; /* nr of 32-bit words.. */
        if count != 0 {
            if 4 & (buff as usize) != 0 {
                result = result.wrapping_add(*(buff as *const u32) as u64);
                count -= 1;
                len -= 4;
                buff = buff.add(4);
            }
            count >>= 1; /* nr of 64-bit words.. */
            if count != 0 {
                let mut carry: u64 = 0;
                loop {
                    let w = *(buff as *const u64);
                    count -= 1;
                    buff = buff.add(8);
                    result = result.wrapping_add(carry).wrapping_add(w);
                    carry = (w > result) as u64;
                    if count == 0 {
                        break;
                    }
                }
                result = result.wrapping_add(carry);
                result = (result & 0xffff_ffff).wrapping_add(result >> 32);
            }
            if len & 4 != 0 {
                result = result.wrapping_add(*(buff as *const u32) as u64);
                buff = buff.add(4);
            }
        }
        if len & 2 != 0 {
            result = result.wrapping_add(*(buff as *const u16) as u64);
            buff = buff.add(2);
        }
    }
    if len & 1 != 0 {
        result = result.wrapping_add(*buff as u64);
    }
    result = from64to16(result) as u64;
    if odd != 0 {
        result = ((result >> 8) & 0xff) | ((result & 0xff) << 8);
    }
    result
}

/*
 *	This is a version of ip_compute_csum() optimized for IP headers,
 *	which always checksum on 4 octet boundaries.
 */
pub unsafe fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: __u32) -> __sum16 {
    (!do_csum(iph as *const u8, ihl.wrapping_mul(4) as i32)) as __sum16
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
pub unsafe fn csum_partial(
    buff: *const core::ffi::c_void,
    len: i32,
    sum: __wsum,
) -> __wsum {
    let mut result = do_csum(buff as *const u8, len);

    /* add in old sum, and carry.. */
    result = result.wrapping_add(sum as u32 as u64);
    /* 32+c bits -> 32 bits */
    result = (result & 0xffff_ffff).wrapping_add(result >> 32);
    result as __wsum
}

/*
 * this routine is used for miscellaneous IP-like checksums, mainly
 * in icmp.c
 */
pub unsafe fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> __sum16 {
    (!from64to16(do_csum(buff as *const u8, len))) as __sum16
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
