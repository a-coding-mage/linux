/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding translation unit: <linux/unaligned.h>
extern "C" {
    fn get_unaligned(ptr: *const u32) -> u32;
    fn put_unaligned(value: u32, ptr: *mut u32);
}

#[inline]
pub unsafe fn flat_get_addr_from_rp(
    rp: *mut u32,
    relval: u32,
    flags: u32,
    addr: *mut u32,
) -> i32 {
    let _ = relval;
    let _ = flags;
    *addr = get_unaligned(rp as *const u32);
    0
}

#[inline]
pub unsafe fn flat_put_addr_at_rp(rp: *mut u32, addr: u32, rel: u32) -> i32 {
    let _ = rel;
    put_unaligned(addr, rp);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
