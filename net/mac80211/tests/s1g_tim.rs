// SPDX-License-Identifier: GPL-2.0-only
/*
 * KUnit tests for S1G TIM PVB decoding. This test suite covers
 * IEEE80211-2024 Annex L figures 8, 9, 10, 12, 13, 14. ADE mode
 * is not covered as it is an optional encoding format and is not
 * currently supported by mac80211.
 *
 * Copyright (C) 2025 Morse Micro
 */

const MAX_AID: usize = 128;

#[inline]
fn bc(enc_mode: u8, inverse: bool, blk_off: u8) -> u8 {
    ((blk_off & 0x1f) << 3) | if inverse { 1 << 2 } else { 0 } | (enc_mode & 0x3)
}

unsafe fn byte_to_bitstr(v: u8, out: *mut u8) {
    for b in (0..=7).rev() {
        *out.add(7 - b) = if (v & (1 << b)) != 0 { b'1' } else { b'0' };
    }
    *out.add(8) = 0;
}

unsafe fn dump_tim_bits(test: *mut kunit, tim: *const ieee80211_tim_ie, tim_len: u8) {
    let mut ptr = (*tim).virtual_map;
    let end = (tim as *const u8).add(tim_len as usize);
    let mut oct: u32 = 1;
    let mut blk: u32 = 0;
    let mut bits = [0u8; 9];

    while ptr < end {
        let ctrl = *ptr;
        ptr = ptr.add(1);
        let mode = ctrl & 0x03;
        let inverse = (ctrl & (1 << 2)) != 0;
        let blk_off = ctrl >> 3;
        kunit_info!(test, "Block {} (ENC={}, blk_off={}, inverse={})", blk,
            if mode == IEEE80211_S1G_TIM_ENC_MODE_BLOCK { "BLOCK" }
            else if mode == IEEE80211_S1G_TIM_ENC_MODE_SINGLE { "SINGLE" } else { "OLB" },
            blk_off, inverse);
        byte_to_bitstr(ctrl, bits.as_mut_ptr());
        kunit_info!(test, "  octet {:2} (ctrl)    : {} (0x{:02x})", oct, cstr(bits.as_ptr()), ctrl);
        oct += 1;

        match mode {
            IEEE80211_S1G_TIM_ENC_MODE_BLOCK => {
                let blkmap = *ptr; ptr = ptr.add(1);
                byte_to_bitstr(blkmap, bits.as_mut_ptr());
                kunit_info!(test, "  octet {:2} (blk-map) : {} (0x{:02x})", oct, cstr(bits.as_ptr()), blkmap);
                oct += 1;
                for sb in 0u8..8 {
                    if (blkmap & (1 << sb)) == 0 { continue; }
                    let sub = *ptr; ptr = ptr.add(1);
                    byte_to_bitstr(sub, bits.as_mut_ptr());
                    kunit_info!(test, "  octet {:2} (SB {:2})   : {} (0x{:02x})", oct, sb, cstr(bits.as_ptr()), sub);
                    oct += 1;
                }
            }
            IEEE80211_S1G_TIM_ENC_MODE_SINGLE => {
                let single = *ptr; ptr = ptr.add(1);
                byte_to_bitstr(single, bits.as_mut_ptr());
                kunit_info!(test, "  octet {:2} (single)  : {} (0x{:02x})", oct, cstr(bits.as_ptr()), single);
                oct += 1;
            }
            IEEE80211_S1G_TIM_ENC_MODE_OLB => {
                let len = *ptr; ptr = ptr.add(1);
                byte_to_bitstr(len, bits.as_mut_ptr());
                kunit_info!(test, "  octet {:2} (len={:2})  : {} (0x{:02x})", oct, len, cstr(bits.as_ptr()), len);
                oct += 1;
                for i in 0..len {
                    if ptr >= end { break; }
                    let sub = *ptr; ptr = ptr.add(1);
                    byte_to_bitstr(sub, bits.as_mut_ptr());
                    kunit_info!(test, "  octet {:2} (SB {:2})   : {} (0x{:02x})", oct, i, cstr(bits.as_ptr()), sub);
                    oct += 1;
                }
            }
            _ => { kunit_info!(test, "  ** unknown encoding 0x{:x} **", mode); return; }
        }
        blk += 1;
    }
}

unsafe fn tim_push(p: *mut *mut u8, v: u8) { **p = v; *p = (*p).add(1); }
unsafe fn tim_begin(tim: *mut ieee80211_tim_ie, p: *mut *mut u8) {
    (*tim).dtim_count = 0; (*tim).dtim_period = 1; (*tim).bitmap_ctrl = 0; *p = (*tim).virtual_map;
}
unsafe fn tim_end(tim: *mut ieee80211_tim_ie, tail: *mut u8) -> u8 { tail.offset_from(tim as *mut u8) as u8 }

unsafe fn pvb_add_block_bitmap(p: *mut *mut u8, blk_off: u8, inverse: bool, blk_bmap: u8, subblocks: *const u8) {
    tim_push(p, bc(IEEE80211_S1G_TIM_ENC_MODE_BLOCK, inverse, blk_off)); tim_push(p, blk_bmap);
    for i in 0..blk_bmap.count_ones() { tim_push(p, *subblocks.add(i as usize)); }
}
unsafe fn pvb_add_single_aid(p: *mut *mut u8, blk_off: u8, inverse: bool, single6: u8) {
    tim_push(p, bc(IEEE80211_S1G_TIM_ENC_MODE_SINGLE, inverse, blk_off)); tim_push(p, single6 & 0x3f);
}
unsafe fn pvb_add_olb(p: *mut *mut u8, blk_off: u8, inverse: bool, subblocks: *const u8, len: u8) {
    tim_push(p, bc(IEEE80211_S1G_TIM_ENC_MODE_OLB, inverse, blk_off)); tim_push(p, len);
    for i in 0..len { tim_push(p, *subblocks.add(i as usize)); }
}

unsafe fn check_all_aids(test: *mut kunit, tim: *const ieee80211_tim_ie, tim_len: u8, expected: *const usize) {
    for aid in 1u16..=MAX_AID as u16 {
        let want = test_bit!(aid as usize, expected); let got = ieee80211_s1g_check_tim(tim, tim_len, aid);
        kunit_assert_eq_msg!(test, got, want, "AID {} mismatch (got={} want={})", aid, got, want);
    }
}
unsafe fn fill_bitmap(bm: *mut usize, list: *const u16, n: usize) { bitmap_zero!(bm, MAX_AID + 1); for i in 0..n { set_bit!(*list.add(i) as usize, bm); } }
unsafe fn fill_bitmap_inverse(bm: *mut usize, max_aid: u16, except: *const u16, n_except: usize) {
    bitmap_zero!(bm, MAX_AID + 1); for aid in 1..=max_aid { set_bit!(aid as usize, bm); }
    for i in 0..n_except { let aid = *except.add(i); if aid <= max_aid { clear_bit!(aid as usize, bm); } }
}

unsafe fn s1g_tim_block_test(test: *mut kunit) { s1g_tim_basic(test, false, 0); }
unsafe fn s1g_tim_single_test(test: *mut kunit) { s1g_tim_basic(test, false, 1); }
unsafe fn s1g_tim_olb_test(test: *mut kunit) { s1g_tim_basic(test, false, 2); }
unsafe fn s1g_tim_inverse_block_test(test: *mut kunit) { s1g_tim_basic(test, true, 0); }
unsafe fn s1g_tim_inverse_single_test(test: *mut kunit) { s1g_tim_basic(test, true, 1); }
unsafe fn s1g_tim_inverse_olb_test(test: *mut kunit) { s1g_tim_basic(test, true, 2); }

unsafe fn s1g_tim_basic(test: *mut kunit, inverse: bool, kind: u8) {
    let mut buf = [0u8; 256];
    let tim = buf.as_mut_ptr() as *mut ieee80211_tim_ie;
    let mut p = core::ptr::null_mut();
    let subblocks = [0x42u8, 0xa0, 0x42, 0xa0, 0x42, 0xa0, 0x42, 0xa0, 0x42];
    let list = [1u16, 6, 13, 15, 17, 22, 29, 31, 33, 38, 45, 47, 49, 54, 61, 63, 65, 70];
    let block_list = [1u16, 6, 21, 23];
    let mut exp = [0usize; 3];
    tim_begin(tim, &mut p);
    match kind {
        0 => pvb_add_block_bitmap(&mut p, 0, inverse, 0x05, subblocks.as_ptr()),
        1 => pvb_add_single_aid(&mut p, 0, inverse, 0x1f),
        _ => pvb_add_olb(&mut p, 0, inverse, subblocks.as_ptr(), subblocks.len() as u8),
    }
    let tim_len = tim_end(tim, p);
    if !inverse {
        if kind == 0 { fill_bitmap(exp.as_mut_ptr(), block_list.as_ptr(), block_list.len()); }
        else if kind == 1 { fill_bitmap(exp.as_mut_ptr(), [31u16].as_ptr(), 1); }
        else { fill_bitmap(exp.as_mut_ptr(), list.as_ptr(), list.len()); }
    } else if kind == 0 {
        fill_bitmap_inverse(exp.as_mut_ptr(), 63, block_list.as_ptr(), block_list.len());
    } else if kind == 1 {
        fill_bitmap_inverse(exp.as_mut_ptr(), 63, [31u16].as_ptr(), 1);
    } else {
        fill_bitmap_inverse(exp.as_mut_ptr(), 127, list.as_ptr(), list.len());
    }
    dump_tim_bits(test, tim, tim_len);
    check_all_aids(test, tim, tim_len, exp.as_ptr());
}

// The KUnit registration and external kernel definitions are supplied by the surrounding translation unit.
extern "C" {
    type kunit;
    type ieee80211_tim_ie;
    static IEEE80211_S1G_TIM_ENC_MODE_BLOCK: u8;
    static IEEE80211_S1G_TIM_ENC_MODE_SINGLE: u8;
    static IEEE80211_S1G_TIM_ENC_MODE_OLB: u8;
    fn ieee80211_s1g_check_tim(tim: *const ieee80211_tim_ie, len: u8, aid: u16) -> bool;
}

// The original file's six KUnit cases are retained as registration metadata.
static S1G_TIM_TEST_CASES: [Option<unsafe fn(*mut kunit)>; 6] = [
    Some(s1g_tim_block_test), Some(s1g_tim_single_test), Some(s1g_tim_olb_test),
    Some(s1g_tim_inverse_block_test), Some(s1g_tim_inverse_single_test), Some(s1g_tim_inverse_olb_test),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
