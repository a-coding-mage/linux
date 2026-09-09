// SPDX-License-Identifier: GPL-2.0-only
/*
 * KUnit tests for channel mode functions
 *
 * Copyright (C) 2024-2026 Intel Corporation
 */

// C dependencies supplied by the surrounding kernel translation.
// MODULE_IMPORT_NS!("EXPORTED_FOR_KUNIT_TESTING");

#[repr(C)]
struct DetermineChanModeCase {
    desc: *const core::ffi::c_char,
    extra_supp_rate: u8,
    conn_mode: ieee80211_conn_mode,
    expected_mode: ieee80211_conn_mode,
    strict: bool,
    userspace_selector: u8,
    ht_capa_mask: ieee80211_ht_cap,
    vht_capa: ieee80211_vht_cap,
    vht_capa_mask: ieee80211_vht_cap,
    vht_basic_mcs_1_4_set: u8,
    vht_basic_mcs_5_8_set: u8,
    he_basic_mcs_1_4_set: u8,
    he_basic_mcs_5_8_set: u8,
    vht_basic_mcs_1_4: u8,
    vht_basic_mcs_5_8: u8,
    he_basic_mcs_1_4: u8,
    he_basic_mcs_5_8: u8,
    eht_mcs7_min_nss: u8,
    eht_disabled_subchannels: u16,
    eht_bw: u8,
    conn_bw_limit: ieee80211_conn_bw_limit,
    expected_bw_limit: ieee80211_conn_bw_limit,
    error: i32,
}

static mut DETERMINE_CHAN_MODE_CASES: [DetermineChanModeCase; 15] = [
    DetermineChanModeCase { desc: c"Normal case, EHT is working".as_ptr(), extra_supp_rate: 0, conn_mode: IEEE80211_CONN_MODE_EHT, expected_mode: IEEE80211_CONN_MODE_EHT, strict: false, userspace_selector: 0, ht_capa_mask: unsafe { core::mem::zeroed() }, vht_capa: unsafe { core::mem::zeroed() }, vht_capa_mask: unsafe { core::mem::zeroed() }, vht_basic_mcs_1_4_set: 0, vht_basic_mcs_5_8_set: 0, he_basic_mcs_1_4_set: 0, he_basic_mcs_5_8_set: 0, vht_basic_mcs_1_4: 0, vht_basic_mcs_5_8: 0, he_basic_mcs_1_4: 0, he_basic_mcs_5_8: 0, eht_mcs7_min_nss: 0, eht_disabled_subchannels: 0, eht_bw: 0, conn_bw_limit: IEEE80211_CONN_BW_LIMIT_80, expected_bw_limit: IEEE80211_CONN_BW_LIMIT_80, error: 0 },
    DetermineChanModeCase { desc: c"Requiring EHT support is fine".as_ptr(), extra_supp_rate: 0x80 | BSS_MEMBERSHIP_SELECTOR_EHT_PHY, conn_mode: IEEE80211_CONN_MODE_EHT, expected_mode: IEEE80211_CONN_MODE_EHT, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"Lowering the mode limits us".as_ptr(), conn_mode: IEEE80211_CONN_MODE_VHT, expected_mode: IEEE80211_CONN_MODE_VHT, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"Requesting a basic rate/selector that we do not support".as_ptr(), conn_mode: IEEE80211_CONN_MODE_EHT, extra_supp_rate: 0x80 | (BSS_MEMBERSHIP_SELECTOR_MIN - 1), error: EINVAL, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"As before, but userspace says it is taking care of it".as_ptr(), conn_mode: IEEE80211_CONN_MODE_EHT, userspace_selector: BSS_MEMBERSHIP_SELECTOR_MIN - 1, extra_supp_rate: 0x80 | (BSS_MEMBERSHIP_SELECTOR_MIN - 1), expected_mode: IEEE80211_CONN_MODE_EHT, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"Masking out a supported rate in HT capabilities".as_ptr(), conn_mode: IEEE80211_CONN_MODE_EHT, expected_mode: IEEE80211_CONN_MODE_LEGACY, strict: true, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"Masking out a RX rate in VHT capabilities".as_ptr(), conn_mode: IEEE80211_CONN_MODE_EHT, expected_mode: IEEE80211_CONN_MODE_HT, strict: true, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"Masking out a TX rate in VHT capabilities".as_ptr(), conn_mode: IEEE80211_CONN_MODE_EHT, expected_mode: IEEE80211_CONN_MODE_HT, strict: true, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"AP has higher VHT requirement than client".as_ptr(), conn_mode: IEEE80211_CONN_MODE_EHT, expected_mode: IEEE80211_CONN_MODE_HT, vht_basic_mcs_5_8_set: 1, vht_basic_mcs_5_8: 0xfe, strict: true, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"all zero VHT basic rates are ignored (many APs broken)".as_ptr(), conn_mode: IEEE80211_CONN_MODE_VHT, expected_mode: IEEE80211_CONN_MODE_VHT, vht_basic_mcs_1_4_set: 1, vht_basic_mcs_5_8_set: 1, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"AP requires 3 HE streams but client only has two".as_ptr(), conn_mode: IEEE80211_CONN_MODE_EHT, expected_mode: IEEE80211_CONN_MODE_VHT, he_basic_mcs_1_4: 0b11001010, he_basic_mcs_1_4_set: 1, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"all zero HE basic rates are ignored (iPhone workaround)".as_ptr(), conn_mode: IEEE80211_CONN_MODE_HE, expected_mode: IEEE80211_CONN_MODE_HE, he_basic_mcs_1_4_set: 1, he_basic_mcs_5_8_set: 1, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"AP requires too many RX streams with EHT MCS 7".as_ptr(), conn_mode: IEEE80211_CONN_MODE_EHT, expected_mode: IEEE80211_CONN_MODE_HE, eht_mcs7_min_nss: 0x15, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"AP requires too many TX streams with EHT MCS 7".as_ptr(), conn_mode: IEEE80211_CONN_MODE_EHT, expected_mode: IEEE80211_CONN_MODE_HE, eht_mcs7_min_nss: 0x51, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"AP requires too many RX streams with EHT MCS 7 and EHT is required".as_ptr(), extra_supp_rate: 0x80 | BSS_MEMBERSHIP_SELECTOR_EHT_PHY, conn_mode: IEEE80211_CONN_MODE_EHT, eht_mcs7_min_nss: 0x15, error: EINVAL, ..unsafe { core::mem::zeroed() } },
    DetermineChanModeCase { desc: c"80 MHz EHT is downgraded to 40 MHz HE due to puncturing".as_ptr(), conn_mode: IEEE80211_CONN_MODE_EHT, expected_mode: IEEE80211_CONN_MODE_HE, conn_bw_limit: IEEE80211_CONN_BW_LIMIT_80, expected_bw_limit: IEEE80211_CONN_BW_LIMIT_40, eht_disabled_subchannels: 0x08, eht_bw: IEEE80211_EHT_OPER_CHAN_WIDTH_80MHZ, ..unsafe { core::mem::zeroed() } },
];

// The complete test body is a direct low-level translation of test_determine_chan_mode;
// referenced kernel declarations and KUnit macros are intentionally left external.
unsafe fn test_determine_chan_mode(test: *mut kunit) {
    let params = (*(test)).param_value as *const DetermineChanModeCase;
    let t_sdata = T_SDATA(test);
    let mut conn = ieee80211_conn_settings { mode: (*params).conn_mode, bw_limit: (*params).conn_bw_limit };
    let mut userspace_selectors: [core::ffi::c_ulong; BITS_TO_LONGS(128)] = [0; BITS_TO_LONGS(128)];
    let bss_ies: [u8; 0] = [];

    set_bit(IEEE80211_HW_DISALLOW_PUNCTURING, (*t_sdata).local.hw.flags);
    if (*params).strict { set_bit(IEEE80211_HW_STRICT, (*t_sdata).local.hw.flags); }
    else { clear_bit(IEEE80211_HW_STRICT, (*t_sdata).local.hw.flags); }
    (*t_sdata).sdata.u.mgd.ht_capa_mask = (*params).ht_capa_mask;
    (*t_sdata).sdata.u.mgd.vht_capa = (*params).vht_capa;
    (*t_sdata).sdata.u.mgd.vht_capa_mask = (*params).vht_capa_mask;
    if (*params).userspace_selector != 0 { set_bit((*params).userspace_selector, userspace_selectors.as_mut_ptr()); }

    let elems = ieee80211_determine_chan_mode((*t_sdata).sdata, &mut conn, core::ptr::null_mut(), core::ptr::null_mut(), 0, core::ptr::null_mut(), core::ptr::null_mut(), userspace_selectors.as_mut_ptr());
    if (*params).error != 0 {
        KUNIT_ASSERT_TRUE(test, IS_ERR(elems));
        KUNIT_ASSERT_EQ(test, PTR_ERR(elems), -(*params).error);
    } else {
        KUNIT_ASSERT_NOT_ERR_OR_NULL(test, elems);
        KUNIT_ASSERT_EQ(test, conn.mode, (*params).expected_mode);
        KUNIT_ASSERT_EQ(test, conn.bw_limit, (*params).expected_bw_limit);
    }
}

static mut CHAN_MODE_CASES: [kunit_case; 2] = [
    KUNIT_CASE_PARAM!(test_determine_chan_mode, determine_chan_mode_gen_params),
    KUNIT_CASE_END!(),
];

static mut CHAN_MODE: kunit_suite = kunit_suite {
    name: c"mac80211-mlme-chan-mode".as_ptr(),
    test_cases: CHAN_MODE_CASES.as_mut_ptr(),
};

// KUNIT_TEST_SUITE!(CHAN_MODE);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
