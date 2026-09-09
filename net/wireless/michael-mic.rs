// SPDX-License-Identifier: GPL-2.0-only
/*
 * Michael MIC implementation - optimized for TKIP MIC operations
 * Copyright 2002-2003, Instant802 Networks, Inc.
 */

use core::ptr;

#[repr(C)]
pub struct ieee80211_hdr {
    pub frame_control: u16,
}

extern "C" {
    fn ieee80211_get_DA(hdr: *mut ieee80211_hdr) -> *mut u8;
    fn ieee80211_get_SA(hdr: *mut ieee80211_hdr) -> *mut u8;
    fn ieee80211_is_data_qos(frame_control: u16) -> bool;
    fn ieee80211_get_tid(hdr: *mut ieee80211_hdr) -> u8;
}

#[repr(C)]
struct michael_mic_ctx {
    l: u32,
    r: u32,
}

#[inline]
unsafe fn get_unaligned_le16(p: *const u8) -> u32 {
    u16::from_le(ptr::read_unaligned(p as *const u16)) as u32
}

#[inline]
unsafe fn get_unaligned_le32(p: *const u8) -> u32 {
    u32::from_le(ptr::read_unaligned(p as *const u32))
}

#[inline]
unsafe fn put_unaligned_le32(value: u32, p: *mut u8) {
    ptr::write_unaligned(p as *mut u32, value.to_le());
}

#[inline]
fn rol32(value: u32, shift: u32) -> u32 {
    value.rotate_left(shift)
}

#[inline]
fn ror32(value: u32, shift: u32) -> u32 {
    value.rotate_right(shift)
}

unsafe fn michael_block(mctx: *mut michael_mic_ctx, val: u32) {
    (*mctx).l ^= val;
    (*mctx).r ^= rol32((*mctx).l, 17);
    (*mctx).l = (*mctx).l.wrapping_add((*mctx).r);
    (*mctx).r ^= (((*mctx).l & 0xff00ff00) >> 8) |
        (((*mctx).l & 0x00ff00ff) << 8);
    (*mctx).l = (*mctx).l.wrapping_add((*mctx).r);
    (*mctx).r ^= rol32((*mctx).l, 3);
    (*mctx).l = (*mctx).l.wrapping_add((*mctx).r);
    (*mctx).r ^= ror32((*mctx).l, 2);
    (*mctx).l = (*mctx).l.wrapping_add((*mctx).r);
}

unsafe fn michael_mic_hdr(
    mctx: *mut michael_mic_ctx,
    key: *const u8,
    hdr: *mut ieee80211_hdr,
) {
    let da: *mut u8;
    let sa: *mut u8;
    let tid: u8;

    da = ieee80211_get_DA(hdr);
    sa = ieee80211_get_SA(hdr);
    if ieee80211_is_data_qos((*hdr).frame_control) {
        tid = ieee80211_get_tid(hdr);
    } else {
        tid = 0;
    }

    (*mctx).l = get_unaligned_le32(key);
    (*mctx).r = get_unaligned_le32(key.add(4));

    /*
     * A pseudo header (DA, SA, Priority, 0, 0, 0) is used in Michael MIC
     * calculation, but it is _not_ transmitted
     */
    michael_block(mctx, get_unaligned_le32(da));
    michael_block(
        mctx,
        get_unaligned_le16(da.add(4)) |
            (get_unaligned_le16(sa) << 16),
    );
    michael_block(mctx, get_unaligned_le32(sa.add(2)));
    michael_block(mctx, tid as u32);
}

pub unsafe fn michael_mic(
    key: *const u8,
    hdr: *mut ieee80211_hdr,
    data: *const u8,
    data_len: usize,
    mic: *mut u8,
) {
    let mut val: u32;
    let block: usize;
    let blocks: usize;
    let mut left: usize;
    let mut mctx: michael_mic_ctx = core::mem::zeroed();

    michael_mic_hdr(&mut mctx, key, hdr);

    /* Real data */
    blocks = data_len / 4;
    left = data_len % 4;

    block = 0;
    while block < blocks {
        michael_block(&mut mctx, get_unaligned_le32(data.add(block * 4)));
        block += 1;
    }

    /* Partial block of 0..3 bytes and padding: 0x5a + 4..7 zeros to make
     * total length a multiple of 4. */
    val = 0x5a;
    while left > 0 {
        val <<= 8;
        left -= 1;
        val |= *data.add(blocks * 4 + left) as u32;
    }

    michael_block(&mut mctx, val);
    michael_block(&mut mctx, 0);

    put_unaligned_le32(mctx.l, mic);
    put_unaligned_le32(mctx.r, mic.add(4));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
