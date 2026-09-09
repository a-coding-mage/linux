/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation unit. */

pub const BB_LEN_MASK: u64 = 0x0000_0000_0000_01FFu64;
pub const BB_OFFSET_MASK: u64 = 0x7FFF_FFFF_FFFF_FE00u64;
pub const BB_ACK_MASK: u64 = 0x8000_0000_0000_0000u64;
pub const BB_MAX_LEN: u64 = 512;

#[inline]
pub const fn bb_offset(x: u64) -> u64 {
    (x & BB_OFFSET_MASK) >> 9
}

#[inline]
pub const fn bb_len(x: u64) -> u64 {
    (x & BB_LEN_MASK) + 1
}

#[inline]
pub const fn bb_ack(x: u64) -> bool {
    (x & BB_ACK_MASK) != 0
}

#[inline]
pub const fn bb_end(x: u64) -> u64 {
    bb_offset(x) + bb_len(x)
}

#[inline]
pub const fn bb_make(a: u64, l: u64, ack: bool) -> u64 {
    (a << 9) | (l - 1) | ((ack as u64) << 63)
}

/* Bad block numbers are stored sorted in a single page.
 * 64bits is used for each block or extent.
 * 54 bits are sector number, 9 bits are extent size,
 * 1 bit is an 'acknowledged' flag.
 */
pub const MAX_BADBLOCKS: usize = PAGE_SIZE / 8;

#[repr(C)]
pub struct badblocks {
    pub dev: *mut device, /* set by devm_init_badblocks */
    pub count: i32,       /* count of bad blocks */
    pub unacked_exist: i32, /* there probably are unacknowledged
                              * bad blocks.  This is only cleared
                              * when a read discovers none */
    pub shift: i32,       /* shift from sectors to block size
                           * a -ve shift means badblocks are
                           * disabled. */
    pub page: *mut u64,   /* badblock list */
    pub changed: i32,
    pub lock: seqlock_t,
    pub sector: sector_t,
    pub size: sector_t,   /* in sectors */
}

#[repr(C)]
pub struct badblocks_context {
    pub start: sector_t,
    pub len: sector_t,
    pub ack: i32,
}

extern "C" {
    pub fn badblocks_check(
        bb: *mut badblocks,
        s: sector_t,
        sectors: sector_t,
        first_bad: *mut sector_t,
        bad_sectors: *mut sector_t,
    ) -> i32;
    pub fn badblocks_set(
        bb: *mut badblocks,
        s: sector_t,
        sectors: sector_t,
        acknowledged: i32,
    ) -> bool;
    pub fn badblocks_clear(bb: *mut badblocks, s: sector_t, sectors: sector_t) -> bool;
    pub fn ack_all_badblocks(bb: *mut badblocks);
    pub fn badblocks_show(bb: *mut badblocks, page: *mut c_char, unack: i32) -> ssize_t;
    pub fn badblocks_store(
        bb: *mut badblocks,
        page: *const c_char,
        len: size_t,
        unack: i32,
    ) -> ssize_t;
    pub fn badblocks_init(bb: *mut badblocks, enable: i32) -> i32;
    pub fn badblocks_exit(bb: *mut badblocks);
    pub fn devm_init_badblocks(dev: *mut device, bb: *mut badblocks) -> i32;
}

#[inline]
pub unsafe fn devm_exit_badblocks(dev: *mut device, bb: *mut badblocks) {
    if (*bb).dev != dev {
        /* dev_WARN_ONCE is a dependency-provided kernel diagnostic macro. */
        dev_WARN_ONCE!(dev, 1, "%s: badblocks instance not associated\n", "devm_exit_badblocks");
        return;
    }
    badblocks_exit(bb);
}

#[inline]
pub unsafe fn badblocks_full(bb: *mut badblocks) -> bool {
    (*bb).count as usize >= MAX_BADBLOCKS
}

#[inline]
pub unsafe fn badblocks_empty(bb: *mut badblocks) -> bool {
    (*bb).count == 0
}

#[inline]
pub unsafe fn set_changed(bb: *mut badblocks) {
    if (*bb).changed != 1 {
        (*bb).changed = 1;
    }
}

#[inline]
pub unsafe fn clear_changed(bb: *mut badblocks) {
    if (*bb).changed != 0 {
        (*bb).changed = 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
