// SPDX-License-Identifier: GPL-2.0-only
/*
 * KUnit tests for channel helper functions
 *
 * Copyright (C) 2023-2024, 2026 Intel Corporation
 */
// C dependencies: <net/cfg80211.h> and <kunit/test.h>.
// MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");

static mut chan_2ghz_1: ieee80211_channel = ieee80211_channel {
    band: NL80211_BAND_2GHZ,
    center_freq: 2412,
};

static mut chan_6ghz_1: ieee80211_channel = ieee80211_channel {
    band: NL80211_BAND_6GHZ,
    center_freq: 5955,
};

static mut chan_6ghz_5: ieee80211_channel = ieee80211_channel {
    band: NL80211_BAND_6GHZ,
    center_freq: 5975,
};

static mut chan_6ghz_105: ieee80211_channel = ieee80211_channel {
    band: NL80211_BAND_6GHZ,
    center_freq: 6475,
};

struct chandef_compat_case {
    desc: *const core::ffi::c_char,
    // leave c1 empty for tests for identical
    c1: cfg80211_chan_def,
    c2: cfg80211_chan_def,
    // we test both ways around, so c2 should always be the compat one
    compat: bool,
}

static chandef_compat_cases: &[chandef_compat_case] = &[
    chandef_compat_case { desc: c"identical non-HT".as_ptr(), c1: cfg80211_chan_def::default(), c2: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_20_NOHT, chan: unsafe { &mut chan_6ghz_1 }, center_freq1: 5955, ..Default::default() }, compat: true },
    chandef_compat_case { desc: c"identical 20 MHz".as_ptr(), c1: cfg80211_chan_def::default(), c2: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_20, chan: unsafe { &mut chan_6ghz_1 }, center_freq1: 5955, ..Default::default() }, compat: true },
    chandef_compat_case { desc: c"identical 40 MHz".as_ptr(), c1: cfg80211_chan_def::default(), c2: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_40, chan: unsafe { &mut chan_6ghz_1 }, center_freq1: 5955 + 10, ..Default::default() }, compat: true },
    chandef_compat_case { desc: c"identical 80 MHz".as_ptr(), c1: cfg80211_chan_def::default(), c2: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_80, chan: unsafe { &mut chan_6ghz_1 }, center_freq1: 5955 + 10 + 20, ..Default::default() }, compat: true },
    chandef_compat_case { desc: c"identical 160 MHz".as_ptr(), c1: cfg80211_chan_def::default(), c2: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_1 }, center_freq1: 5955 + 10 + 20 + 40, ..Default::default() }, compat: true },
    chandef_compat_case { desc: c"identical 320 MHz".as_ptr(), c1: cfg80211_chan_def::default(), c2: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &mut chan_6ghz_1 }, center_freq1: 5955 + 10 + 20 + 40 + 80, ..Default::default() }, compat: true },
    chandef_compat_case { desc: c"20 MHz in 320 MHz\n".as_ptr(), c1: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_20, chan: unsafe { &mut chan_6ghz_1 }, center_freq1: 5955, ..Default::default() }, c2: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &mut chan_6ghz_1 }, center_freq1: 5955 + 10 + 20 + 40 + 80, ..Default::default() }, compat: true },
    chandef_compat_case { desc: c"different 20 MHz".as_ptr(), c1: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_20, chan: unsafe { &mut chan_6ghz_1 }, center_freq1: 5955, ..Default::default() }, c2: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_20, chan: unsafe { &mut chan_6ghz_5 }, center_freq1: 5975, ..Default::default() }, compat: false },
    chandef_compat_case { desc: c"different primary 320 MHz".as_ptr(), c1: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 110, ..Default::default() }, c2: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 - 50, ..Default::default() }, compat: false },
    chandef_compat_case { desc: c"matching primary 160 MHz".as_ptr(), c1: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 30, ..Default::default() }, c2: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 - 50, ..Default::default() }, compat: true },
    chandef_compat_case { desc: c"matching primary 160 MHz & punctured secondary 160 Mhz".as_ptr(), c1: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 30, ..Default::default() }, c2: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 - 50, punctured: 0xf, ..Default::default() }, compat: true },
    chandef_compat_case { desc: c"matching primary 160 MHz & punctured matching".as_ptr(), c1: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 30, punctured: 0xc0, ..Default::default() }, c2: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 - 50, punctured: 0xc000, ..Default::default() }, compat: true },
    chandef_compat_case { desc: c"matching primary 160 MHz & punctured not matching".as_ptr(), c1: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 30, punctured: 0x80, ..Default::default() }, c2: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 - 50, punctured: 0xc000, ..Default::default() }, compat: false },
];

// KUNIT_ARRAY_PARAM_DESC(chandef_compat, chandef_compat_cases, desc)

unsafe fn test_chandef_compat(test: *mut kunit) {
    let params = (*test).param_value as *const chandef_compat_case;
    let mut c1 = (*params).c1;
    if (*params).c1.chan.is_null() { c1 = (*params).c2; }
    KUNIT_EXPECT_EQ!(test, cfg80211_chandef_valid(&c1), true);
    KUNIT_EXPECT_EQ!(test, cfg80211_chandef_valid(&(*params).c2), true);
    let expect = if (*params).compat { &(*params).c2 as *const _ } else { core::ptr::null() };
    let ret = cfg80211_chandef_compatible(&c1, &(*params).c2);
    KUNIT_EXPECT_PTR_EQ!(test, ret, expect);
    let expect = if (*params).c1.chan.is_null() { &c1 as *const _ } else { expect };
    let ret = cfg80211_chandef_compatible(&(*params).c2, &c1);
    KUNIT_EXPECT_PTR_EQ!(test, ret, expect);
}

struct chandef_dbe_case {
    desc: *const core::ffi::c_char,
    c: cfg80211_chan_def,
    dbe: [u8; 3],
    fails: bool,
    cf1: u16,
}

static chandef_dbe_cases: &[chandef_dbe_case] = &[
    chandef_dbe_case { desc: c"non-HT failure".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_20_NOHT, chan: unsafe { &mut chan_6ghz_1 }, center_freq1: 5955, ..Default::default() }, dbe: [IEEE80211_UHR_DBE_OPER_BW_40, 0, 0], fails: true, cf1: 0 },
    chandef_dbe_case { desc: c"2.4 GHz fails".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_20, chan: unsafe { &mut chan_2ghz_1 }, center_freq1: 2412, ..Default::default() }, dbe: [IEEE80211_UHR_DBE_OPER_BW_40, 0, 0], fails: true, cf1: 0 },
    chandef_dbe_case { desc: c"DBE narrower".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_320, chan: unsafe { &mut chan_6ghz_1 }, center_freq1: 5955 + 10 + 20 + 40 + 80, ..Default::default() }, dbe: [IEEE80211_UHR_DBE_OPER_BW_160, 0, 0], fails: true, cf1: 0 },
    chandef_dbe_case { desc: c"DBE to 320-1".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 30, ..Default::default() }, dbe: [IEEE80211_UHR_DBE_OPER_BW_320_1, 0, 0], fails: false, cf1: 6425 },
    chandef_dbe_case { desc: c"DBE to 320-2".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 30, ..Default::default() }, dbe: [IEEE80211_UHR_DBE_OPER_BW_320_2, 0, 0], fails: false, cf1: 6585 },
    chandef_dbe_case { desc: c"bad disabled subchannel bitmap - not enough in BSS (1)".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 30, punctured: 1, ..Default::default() }, dbe: [IEEE80211_UHR_DBE_OPER_BW_320_1 | IEEE80211_UHR_DBE_OPER_DIS_SUBCHANNEL_BITMAP_PRES, 0, 0], fails: true, cf1: 0 },
    chandef_dbe_case { desc: c"bad disabled subchannel bitmap - too much in BSS (1)".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 30, punctured: 1, ..Default::default() }, dbe: [IEEE80211_UHR_DBE_OPER_BW_320_1 | IEEE80211_UHR_DBE_OPER_DIS_SUBCHANNEL_BITMAP_PRES, 0, 3], fails: true, cf1: 0 },
    chandef_dbe_case { desc: c"bad disabled subchannel bitmap - not enough in BSS (2)".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 30, punctured: 1, ..Default::default() }, dbe: [IEEE80211_UHR_DBE_OPER_BW_320_2 | IEEE80211_UHR_DBE_OPER_DIS_SUBCHANNEL_BITMAP_PRES, 0, 0], fails: true, cf1: 0 },
    chandef_dbe_case { desc: c"bad disabled subchannel bitmap - too much in BSS (2)".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 30, punctured: 1, ..Default::default() }, dbe: [IEEE80211_UHR_DBE_OPER_BW_320_2 | IEEE80211_UHR_DBE_OPER_DIS_SUBCHANNEL_BITMAP_PRES, 3, 0], fails: true, cf1: 0 },
    chandef_dbe_case { desc: c"bad disabled subchannel bitmap - bad bitmap".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 30, punctured: 1, ..Default::default() }, dbe: [IEEE80211_UHR_DBE_OPER_BW_320_1 | IEEE80211_UHR_DBE_OPER_DIS_SUBCHANNEL_BITMAP_PRES, 0, 0x11], fails: true, cf1: 0 },
    chandef_dbe_case { desc: c"good disabled subchannel bitmap (1)".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 30, punctured: 3, ..Default::default() }, dbe: [IEEE80211_UHR_DBE_OPER_BW_320_1 | IEEE80211_UHR_DBE_OPER_DIS_SUBCHANNEL_BITMAP_PRES, 0, 3], fails: false, cf1: 6425 },
    chandef_dbe_case { desc: c"good disabled subchannel bitmap (2)".as_ptr(), c: cfg80211_chan_def { width: NL80211_CHAN_WIDTH_160, chan: unsafe { &mut chan_6ghz_105 }, center_freq1: 6475 + 30, punctured: 3, ..Default::default() }, dbe: [IEEE80211_UHR_DBE_OPER_BW_320_2 | IEEE80211_UHR_DBE_OPER_DIS_SUBCHANNEL_BITMAP_PRES, 3, 0], fails: false, cf1: 6585 },
];

// KUNIT_ARRAY_PARAM_DESC(chandef_dbe, chandef_dbe_cases, desc)

unsafe fn test_chandef_dbe(test: *mut kunit) {
    let params = (*test).param_value as *const chandef_dbe_case;
    let mut c = (*params).c;
    KUNIT_EXPECT_EQ!(test, cfg80211_chandef_valid(&(*params).c), true);
    let ret = cfg80211_chandef_add_dbe(&mut c, (*params).dbe.as_ptr() as *const core::ffi::c_void);
    KUNIT_EXPECT_EQ!(test, ret != 0, (*params).fails);
    if (*params).fails { return; }
    KUNIT_EXPECT_EQ!(test, c.center_freq1, (*params).cf1);
}

// KUNIT_CASE_PARAM(test_chandef_compat, chandef_compat_gen_params)
// KUNIT_CASE_PARAM(test_chandef_dbe, chandef_dbe_gen_params)
// kunit_test_suite(chandef)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
