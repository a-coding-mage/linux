/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const EBT_AMONG_DST: u32 = 0x01;
pub const EBT_AMONG_SRC: u32 = 0x02;

/* Grzegorz Borowiak <grzes@gnu.univ.gda.pl> 2003
 *
 * Write-once-read-many hash table, used for checking if a given
 * MAC address belongs to a set or not and possibly for checking
 * if it is related with a given IPv4 address.
 *
 * The hash value of an address is its last byte.
 *
 * In real-world ethernet addresses, values of the last byte are
 * evenly distributed and there is no need to consider other bytes.
 * It would only slow the routines down.
 *
 * For MAC address comparison speedup reasons, we introduce a trick.
 * MAC address is mapped onto an array of two 32-bit integers.
 * This pair of integers is compared with MAC addresses in the
 * hash table, which are stored also in form of pairs of integers
 * (in `cmp' array). This is quick as it requires only two elementary
 * number comparisons in worst case. Further, we take advantage of
 * fact that entropy of 3 last bytes of address is larger than entropy
 * of 3 first bytes. So first we compare 4 last bytes of addresses and
 * if they are the same we compare 2 first.
 *
 * Yes, it is a memory overhead, but in 2003 AD, who cares?
 */

#[repr(C)]
pub struct ebt_mac_wormhash_tuple {
    pub cmp: [u32; 2],
    pub ip: u32,
}

#[repr(C)]
pub struct ebt_mac_wormhash {
    pub table: [i32; 257],
    pub poolsize: i32,
    pub pool: [ebt_mac_wormhash_tuple; 0],
}

#[inline]
pub unsafe fn ebt_mac_wormhash_size(x: *const ebt_mac_wormhash) -> usize {
    if !x.is_null() {
        core::mem::size_of::<ebt_mac_wormhash>()
            + ((*x).poolsize as usize)
                * core::mem::size_of::<ebt_mac_wormhash_tuple>()
    } else {
        0
    }
}

#[repr(C)]
pub struct ebt_among_info {
    pub wh_dst_ofs: i32,
    pub wh_src_ofs: i32,
    pub bitmask: i32,
}

pub const EBT_AMONG_DST_NEG: u32 = 0x1;
pub const EBT_AMONG_SRC_NEG: u32 = 0x2;

#[inline]
pub unsafe fn ebt_among_wh_dst(
    x: *const ebt_among_info,
) -> *mut ebt_mac_wormhash {
    if !x.is_null() && (*x).wh_dst_ofs != 0 {
        (x as *const u8).offset((*x).wh_dst_ofs as isize) as *mut ebt_mac_wormhash
    } else {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn ebt_among_wh_src(
    x: *const ebt_among_info,
) -> *mut ebt_mac_wormhash {
    if !x.is_null() && (*x).wh_src_ofs != 0 {
        (x as *const u8).offset((*x).wh_src_ofs as isize) as *mut ebt_mac_wormhash
    } else {
        core::ptr::null_mut()
    }
}

pub const EBT_AMONG_MATCH: &str = "among";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
