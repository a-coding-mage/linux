// SPDX-License-Identifier: GPL-2.0-only
/*
 * KUnit tests for TPE element handling
 *
 * Copyright (C) 2024 Intel Corporation
 */

// Dependencies supplied by the surrounding kernel translation unit:
// kunit/test.h, ../ieee80211_i.h
// MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");

static mut chan6g_1: ieee80211_channel = ieee80211_channel {
    band: NL80211_BAND_6GHZ,
    center_freq: 5955,
};

static mut chan6g_33: ieee80211_channel = ieee80211_channel {
    band: NL80211_BAND_6GHZ,
    center_freq: 6115,
};

static mut chan6g_61: ieee80211_channel = ieee80211_channel {
    band: NL80211_BAND_6GHZ,
    center_freq: 6255,
};

#[repr(C)]
struct subchan_test_case {
    desc: *const core::ffi::c_char,
    c: cfg80211_chan_def,
    n: u8,
    expect: i32,
}

static subchan_offset_cases: [subchan_test_case; 10] = [
    subchan_test_case { desc: c"identical 20 MHz".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_20, chan: unsafe { &raw mut chan6g_1 }, center_freq1: 5955, ..unsafe { core::mem::zeroed() } }, n: 1, expect: 0 },
    subchan_test_case { desc: c"identical 40 MHz".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_40, chan: unsafe { &raw mut chan6g_1 }, center_freq1: 5965, ..unsafe { core::mem::zeroed() } }, n: 2, expect: 0 },
    subchan_test_case { desc: c"identical 80+80 MHz".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_80P80, chan: unsafe { &raw mut chan6g_1 }, center_freq1: 5985, center_freq2: 6225, ..unsafe { core::mem::zeroed() } }, n: 16, expect: 0 },
    subchan_test_case { desc: c"identical 320 MHz".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &raw mut chan6g_1 }, center_freq1: 6105, ..unsafe { core::mem::zeroed() } }, n: 16, expect: 0 },
    subchan_test_case { desc: c"lower 160 MHz of 320 MHz".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &raw mut chan6g_1 }, center_freq1: 6105, ..unsafe { core::mem::zeroed() } }, n: 8, expect: 0 },
    subchan_test_case { desc: c"upper 160 MHz of 320 MHz".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &raw mut chan6g_61 }, center_freq1: 6105, ..unsafe { core::mem::zeroed() } }, n: 8, expect: 8 },
    subchan_test_case { desc: c"upper 160 MHz of 320 MHz, go to 40".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &raw mut chan6g_61 }, center_freq1: 6105, ..unsafe { core::mem::zeroed() } }, n: 2, expect: 8 + 4 + 2 },
    subchan_test_case { desc: c"secondary 80 above primary in 80+80 MHz".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_80P80, chan: unsafe { &raw mut chan6g_1 }, center_freq1: 5985, center_freq2: 6225, ..unsafe { core::mem::zeroed() } }, n: 4, expect: 0 },
    subchan_test_case { desc: c"secondary 80 below primary in 80+80 MHz".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_80P80, chan: unsafe { &raw mut chan6g_61 }, center_freq1: 6225, center_freq2: 5985, ..unsafe { core::mem::zeroed() } }, n: 4, expect: 4 },
    subchan_test_case { desc: c"secondary 80 below primary in 80+80 MHz, go to 20".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_80P80, chan: unsafe { &raw mut chan6g_61 }, center_freq1: 6225, center_freq2: 5985, ..unsafe { core::mem::zeroed() } }, n: 1, expect: 7 },
];

// KUNIT_ARRAY_PARAM_DESC(subchan_offset, subchan_offset_cases, desc);

unsafe fn subchan_offset(test: *mut kunit) {
    let params = (*test).param_value as *const subchan_test_case;
    let offset: i32;
    // KUNIT_ASSERT_EQ(test, cfg80211_chandef_valid(&params->c), true);
    offset = ieee80211_calc_chandef_subchan_offset(&(*params).c, (*params).n);
    // KUNIT_EXPECT_EQ(test, params->expect, offset);
    let _ = offset;
}

#[repr(C)]
struct psd_reorder_test_case {
    desc: *const core::ffi::c_char,
    ap: cfg80211_chan_def,
    used: cfg80211_chan_def,
    psd: ieee80211_parsed_tpe_psd,
    out: ieee80211_parsed_tpe_psd,
}

static psd_reorder_cases: [psd_reorder_test_case; 5] = [
    psd_reorder_test_case { desc: c"no changes, 320 MHz".as_ptr(), ap: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &raw mut chan6g_1 }, center_freq1: 6105, ..unsafe { core::mem::zeroed() } }, used: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &raw mut chan6g_1 }, center_freq1: 6105, ..unsafe { core::mem::zeroed() } }, psd: ieee80211_parsed_tpe_psd { valid: true, count: 16, n: 8, power: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15] }, out: ieee80211_parsed_tpe_psd { valid: true, count: 16, n: 8, power: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15] } },
    psd_reorder_test_case { desc: c"no changes, 320 MHz, 160 MHz used, n=0".as_ptr(), ap: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &raw mut chan6g_1 }, center_freq1: 6105, ..unsafe { core::mem::zeroed() } }, used: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &raw mut chan6g_1 }, center_freq1: 6025, ..unsafe { core::mem::zeroed() } }, psd: ieee80211_parsed_tpe_psd { valid: true, count: 16, n: 0, power: [1; 16] }, out: ieee80211_parsed_tpe_psd { valid: true, count: 8, n: 0, power: [1; 16] } },
    psd_reorder_test_case { desc: c"320 MHz, HE is 80, used 160, all lower".as_ptr(), ap: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &raw mut chan6g_1 }, center_freq1: 6105, ..unsafe { core::mem::zeroed() } }, used: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &raw mut chan6g_1 }, center_freq1: 6025, ..unsafe { core::mem::zeroed() } }, psd: ieee80211_parsed_tpe_psd { valid: true, count: 16, n: 4, power: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15] }, out: ieee80211_parsed_tpe_psd { valid: true, count: 8, n: 4, power: [0,1,2,3,4,5,6,7,127,127,127,127,127,127,127,127] } },
    psd_reorder_test_case { desc: c"320 MHz, HE is 80, used 160, all upper".as_ptr(), ap: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &raw mut chan6g_61 }, center_freq1: 6105, ..unsafe { core::mem::zeroed() } }, used: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &raw mut chan6g_61 }, center_freq1: 6185, ..unsafe { core::mem::zeroed() } }, psd: ieee80211_parsed_tpe_psd { valid: true, count: 16, n: 4, power: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15] }, out: ieee80211_parsed_tpe_psd { valid: true, count: 8, n: 4, power: [12,13,14,15,0,1,2,3,127,127,127,127,127,127,127,127] } },
    psd_reorder_test_case { desc: c"320 MHz, HE is 80, used 160, split".as_ptr(), ap: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &raw mut chan6g_33 }, center_freq1: 6105, ..unsafe { core::mem::zeroed() } }, used: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &raw mut chan6g_33 }, center_freq1: 6185, ..unsafe { core::mem::zeroed() } }, psd: ieee80211_parsed_tpe_psd { valid: true, count: 16, n: 4, power: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15] }, out: ieee80211_parsed_tpe_psd { valid: true, count: 8, n: 4, power: [0,1,2,3,12,13,14,15,127,127,127,127,127,127,127,127] } },
];

// KUNIT_ARRAY_PARAM_DESC(psd_reorder, psd_reorder_cases, desc);

unsafe fn psd_reorder(test: *mut kunit) {
    let params = (*test).param_value as *const psd_reorder_test_case;
    let mut tmp = (*params).psd;
    ieee80211_rearrange_tpe_psd(&mut tmp, &(*params).ap, &(*params).used);
    // KUNIT_EXPECT_MEMEQ(test, &tmp, &params->out, sizeof(tmp));
}

// static struct kunit_case tpe_test_cases[] = {
//     KUNIT_CASE_PARAM(subchan_offset, subchan_offset_gen_params),
//     KUNIT_CASE_PARAM(psd_reorder, psd_reorder_gen_params),
//     {},
// };
// static struct kunit_suite tpe = { .name = "mac80211-tpe", .test_cases = tpe_test_cases };
// kunit_test_suite(tpe);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
