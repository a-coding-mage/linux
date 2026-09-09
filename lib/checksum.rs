// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		IP/TCP/UDP checksumming routines
 *
 * Authors:	Jorge Cwik, <jorge@laser.satlink.net>
 *		Arnt Gulbrandsen, <agulbra@nvg.unit.no>
 *		Tom May, <ftom@netcom.com>
 *		Andreas Schwab, <schwab@issan.informatik.uni-dortmund.de>
 *		Lots of code moved from tcp.c and ip.c; see those files
 *		for more names.
 *
 * Revised by Kenneth Albanowski for m68knommu. Basic problem: unaligned access
 * kills, so most of the assembly has to go.
 */

unsafe fn do_csum(buff: *const u8, mut len: i32) -> u32 {
    let mut odd: i32;
    let mut result: u32 = 0;

    if len <= 0 {
        return result;
    }
    odd = (1 & (buff as usize)) as i32;
    if odd != 0 {
        #[cfg(target_endian = "little")]
        {
            result = result.wrapping_add((*buff as u32) << 8);
        }
        #[cfg(target_endian = "big")]
        {
            result = *buff as u32;
        }
        len -= 1;
        buff = buff.add(1);
    }
    if len >= 2 {
        if 2 & (buff as usize) != 0 {
            result = result.wrapping_add((*(buff as *const u16)) as u32);
            len -= 2;
            buff = buff.add(2);
        }
        if len >= 4 {
            let end = buff.add((len as u32 & !3) as usize);
            let mut carry: u32 = 0;
            loop {
                let w = *(buff as *const u32);
                buff = buff.add(4);
                result = result.wrapping_add(carry);
                result = result.wrapping_add(w);
                carry = (w > result) as u32;
                if buff >= end {
                    break;
                }
            }
            result = result.wrapping_add(carry);
            result = (result & 0xffff).wrapping_add(result >> 16);
        }
        if len & 2 != 0 {
            result = result.wrapping_add((*(buff as *const u16)) as u32);
            buff = buff.add(2);
        }
    }
    if len & 1 != 0 {
        #[cfg(target_endian = "little")]
        {
            result = result.wrapping_add(*buff as u32);
        }
        #[cfg(target_endian = "big")]
        {
            result = result.wrapping_add((*buff as u32) << 8);
        }
    }
    result = csum_from32to16(result);
    if odd != 0 {
        result = ((result >> 8) & 0xff) | ((result & 0xff) << 8);
    }
    result
}

pub unsafe fn ip_fast_csum(iph: *const core::ffi::c_void, ihl: u32) -> u16 {
    (!do_csum(iph as *const u8, ihl.wrapping_mul(4))) as u16
}

pub unsafe fn csum_partial(
    buff: *const core::ffi::c_void,
    len: i32,
    wsum: u32,
) -> u32 {
    let sum = wsum;
    let mut result = do_csum(buff as *const u8, len);

    /* add in old sum, and carry.. */
    result = result.wrapping_add(sum);
    if sum > result {
        result = result.wrapping_add(1);
    }
    result
}

/*
 * this routine is used for miscellaneous IP-like checksums, mainly
 * in icmp.c
 */
pub unsafe fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> u16 {
    (!do_csum(buff as *const u8, len)) as u16
}

unsafe fn from64to32(mut x: u64) -> u32 {
    /* add up 32-bit and 32-bit for 32+c bit */
    x = (x & 0xffffffff).wrapping_add(x >> 32);
    /* add up carry.. */
    x = (x & 0xffffffff).wrapping_add(x >> 32);
    x as u32
}

pub unsafe fn csum_tcpudp_nofold(
    saddr: u32,
    daddr: u32,
    len: u32,
    proto: u8,
    sum: u32,
) -> u32 {
    let mut s = sum as u64;

    s = s.wrapping_add(saddr as u64);
    s = s.wrapping_add(daddr as u64);
    #[cfg(target_endian = "big")]
    {
        s = s.wrapping_add(proto as u64 + len as u64);
    }
    #[cfg(target_endian = "little")]
    {
        s = s.wrapping_add((proto as u32).wrapping_add(len) as u64 * 256);
    }
    from64to32(s)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
