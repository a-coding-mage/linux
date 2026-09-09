// SPDX-License-Identifier: GPL-2.0-only
/*
 * KUnit tests for element parsing
 *
 * Copyright (C) 2023-2025 Intel Corporation
 */

// Dependencies supplied by the surrounding mac80211 and KUnit environment.

#[repr(C)]
struct MeshPreqParseTestCase {
    desc: *const core::ffi::c_char,
    len: u8,
    ae_enabled: bool,
    target_count: u8,
    result: bool,
}

static MESH_PREQ_PARSE_CASES: [MeshPreqParseTestCase; 11] = [
    MeshPreqParseTestCase { desc: c"shorter than header".as_ptr(), len: 16, ae_enabled: false, target_count: 1, result: false },
    MeshPreqParseTestCase { desc: c"too short non AE, target count is not included".as_ptr(), len: 29, ae_enabled: false, target_count: 1, result: false },
    MeshPreqParseTestCase { desc: c"too short non AE, target count is 1".as_ptr(), len: 36, ae_enabled: false, target_count: 1, result: false },
    MeshPreqParseTestCase { desc: c"too short AE, target count is not included".as_ptr(), len: 35, ae_enabled: true, target_count: 1, result: false },
    MeshPreqParseTestCase { desc: c"too short AE, target count is 1".as_ptr(), len: 42, ae_enabled: true, target_count: 1, result: false },
    MeshPreqParseTestCase { desc: c"target count is zero".as_ptr(), len: 26, ae_enabled: false, target_count: 0, result: false },
    MeshPreqParseTestCase { desc: c"target count is 21".as_ptr(), len: 255, ae_enabled: false, target_count: 21, result: false },
    MeshPreqParseTestCase { desc: c"non AE, target count is 1".as_ptr(), len: 37, ae_enabled: false, target_count: 1, result: true },
    MeshPreqParseTestCase { desc: c"non AE, target count is 20".as_ptr(), len: 246, ae_enabled: false, target_count: 20, result: true },
    MeshPreqParseTestCase { desc: c"AE, target count is 1".as_ptr(), len: 43, ae_enabled: true, target_count: 1, result: true },
    MeshPreqParseTestCase { desc: c"AE, target count is 20".as_ptr(), len: 252, ae_enabled: true, target_count: 20, result: true },
];

#[repr(C)]
struct MeshPrepParseTestCase { desc: *const core::ffi::c_char, len: u8, ae_enabled: bool, result: bool }
static MESH_PREP_PARSE_CASES: [MeshPrepParseTestCase; 5] = [
    MeshPrepParseTestCase { desc: c"shorter than header".as_ptr(), len: 12, ae_enabled: false, result: false },
    MeshPrepParseTestCase { desc: c"non AE short".as_ptr(), len: 30, ae_enabled: false, result: false },
    MeshPrepParseTestCase { desc: c"non AE".as_ptr(), len: 31, ae_enabled: false, result: true },
    MeshPrepParseTestCase { desc: c"AE short".as_ptr(), len: 36, ae_enabled: true, result: false },
    MeshPrepParseTestCase { desc: c"AE".as_ptr(), len: 37, ae_enabled: true, result: true },
];

#[repr(C)]
struct MeshPerrParseTestCase { desc: *const core::ffi::c_char, len: u8, number_of_dst: u8, ae_enabled_idx: i32, result: bool }
static MESH_PERR_PARSE_CASES: [MeshPerrParseTestCase; 13] = [
    MeshPerrParseTestCase { desc: c"shorter than header".as_ptr(), len: 1, number_of_dst: 1, ae_enabled_idx: -1, result: false },
    MeshPerrParseTestCase { desc: c"number_of_dst is 0".as_ptr(), len: 2, number_of_dst: 0, ae_enabled_idx: -1, result: true },
    MeshPerrParseTestCase { desc: c"number_of_dst is 20".as_ptr(), len: 255, number_of_dst: 20, ae_enabled_idx: -1, result: false },
    MeshPerrParseTestCase { desc: c"number_of_dst is 1, non AE, short".as_ptr(), len: 14, number_of_dst: 1, ae_enabled_idx: -1, result: false },
    MeshPerrParseTestCase { desc: c"number_of_dst is 1, non AE".as_ptr(), len: 15, number_of_dst: 1, ae_enabled_idx: -1, result: true },
    MeshPerrParseTestCase { desc: c"number_of_dst is 1, non AE, extra short dst header".as_ptr(), len: 25, number_of_dst: 1, ae_enabled_idx: -1, result: false },
    MeshPerrParseTestCase { desc: c"number_of_dst is 1, non AE, extra dst header".as_ptr(), len: 26, number_of_dst: 1, ae_enabled_idx: -1, result: false },
    MeshPerrParseTestCase { desc: c"number_of_dst is 1, AE, short".as_ptr(), len: 20, number_of_dst: 1, ae_enabled_idx: 0, result: false },
    MeshPerrParseTestCase { desc: c"number_of_dst is 1, AE".as_ptr(), len: 21, number_of_dst: 1, ae_enabled_idx: 0, result: true },
    MeshPerrParseTestCase { desc: c"number_of_dst is 19, non AE, short".as_ptr(), len: 2 + 13 * 19 - 1, number_of_dst: 19, ae_enabled_idx: -1, result: false },
    MeshPerrParseTestCase { desc: c"number_of_dst is 19, non AE".as_ptr(), len: 2 + 13 * 19, number_of_dst: 19, ae_enabled_idx: -1, result: true },
    MeshPerrParseTestCase { desc: c"number_of_dst is 19, AE, short".as_ptr(), len: 2 + 13 * 19 + 6 - 1, number_of_dst: 19, ae_enabled_idx: 18, result: false },
    MeshPerrParseTestCase { desc: c"number_of_dst is 19, AE".as_ptr(), len: 2 + 13 * 19 + 6, number_of_dst: 19, ae_enabled_idx: 18, result: true },
];

// KUNIT_ARRAY_PARAM_DESC(mesh_preq_parse, mesh_preq_parse_cases, desc);
// KUNIT_ARRAY_PARAM_DESC(mesh_prep_parse, mesh_prep_parse_cases, desc);
// KUNIT_ARRAY_PARAM_DESC(mesh_perr_parse, mesh_perr_parse_cases, desc);

unsafe fn mle_defrag(test: *mut kunit) {
    let mut parse_params = ieee80211_elems_parse_params { link_id: 12, from_ap: true, mode: IEEE80211_CONN_MODE_EHT, type_: IEEE80211_FTYPE_MGMT | IEEE80211_STYPE_BEACON, start: core::ptr::null(), len: 0 };
    let skb = alloc_skb(1024, GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, skb);
    if skb_pad(skb, skb_tailroom(skb)) != 0 { KUNIT_FAIL(test, c"failed to pad skb".as_ptr()); return; }
    skb_put_u8(skb, WLAN_EID_EXTENSION);
    let len_mle = skb_put(skb, 1);
    skb_put_u8(skb, WLAN_EID_EXT_EHT_MULTI_LINK);
    put_unaligned_le16(IEEE80211_ML_CONTROL_TYPE_BASIC, skb_put(skb, 2));
    skb_put_u8(skb, 7);
    skb_put_data(skb, c"\0\0\0\0\0\0".as_ptr() as *const _, ETH_ALEN);
    skb_put_u8(skb, IEEE80211_MLE_SUBELEM_PER_STA_PROFILE);
    let len_prof = skb_put(skb, 1);
    put_unaligned_le16(IEEE80211_MLE_STA_CONTROL_COMPLETE_PROFILE | parse_params.link_id, skb_put(skb, 2));
    skb_put_u8(skb, 1);
    for _i in 0..20 { skb_put_u8(skb, WLAN_EID_SSID); skb_put_u8(skb, 20); skb_put(skb, 20); }
    ieee80211_fragment_element(skb, len_prof, IEEE80211_MLE_SUBELEM_FRAGMENT);
    ieee80211_fragment_element(skb, len_mle, WLAN_EID_FRAGMENT);
    parse_params.start = (*skb).data;
    parse_params.len = (*skb).len;
    let parsed = ieee802_11_parse_elems_full(&mut parse_params);
    KUNIT_EXPECT_NOT_NULL(test, parsed);
    if IS_ERR_OR_NULL(parsed) { kfree_skb(skb); return; }
    KUNIT_EXPECT_NOT_NULL(test, (*parsed).ml_basic);
    KUNIT_EXPECT_EQ(test, (*parsed).ml_basic_len, 2 + 7 + 2 + 3 + 20 * 22 + 2);
    KUNIT_EXPECT_NOT_NULL(test, (*parsed).prof);
    KUNIT_EXPECT_EQ(test, (*parsed).sta_prof_len, 3 + 20 * 22);
    kfree(parsed as *mut _);
    kfree_skb(skb);
}

unsafe fn mesh_preq_parse(test: *mut kunit) { let params = test_param_value::<MeshPreqParseTestCase>(test); let mut data = [0u8; 64]; let top = data.as_mut_ptr() as *mut ieee80211_mesh_hwmp_preq_top; (*top).flags = if params.ae_enabled { AE_F } else { 0 }; let bottom = ieee80211_mesh_hwmp_preq_get_bottom(data.as_mut_ptr()); (*bottom).target_count = params.target_count; KUNIT_EXPECT_EQ(test, ieee80211_mesh_preq_size_ok(data.as_mut_ptr(), params.len), params.result); }
unsafe fn mesh_prep_parse(test: *mut kunit) { let params = test_param_value::<MeshPrepParseTestCase>(test); let mut data = [0u8; 64]; let top = data.as_mut_ptr() as *mut ieee80211_mesh_hwmp_prep_top; (*top).flags = if params.ae_enabled { AE_F } else { 0 }; KUNIT_EXPECT_EQ(test, ieee80211_mesh_prep_size_ok(data.as_mut_ptr(), params.len), params.result); }
unsafe fn mesh_perr_parse(test: *mut kunit) { let params = test_param_value::<MeshPerrParseTestCase>(test); let mut data = [0u8; 256]; let perr = data.as_mut_ptr() as *mut ieee80211_mesh_hwmp_perr; (*perr).number_of_dst = params.number_of_dst; if params.ae_enabled_idx > -1 { let dst = ieee80211_mesh_hwmp_perr_get_dst(data.as_mut_ptr(), params.ae_enabled_idx); (*dst).flags = AE_F; } KUNIT_EXPECT_EQ(test, ieee80211_mesh_perr_size_ok(data.as_mut_ptr(), params.len), params.result); }

// KUNIT_CASE(mle_defrag);
// KUNIT_CASE_PARAM(mesh_preq_parse, mesh_preq_parse_gen_params);
// KUNIT_CASE_PARAM(mesh_prep_parse, mesh_prep_parse_gen_params);
// KUNIT_CASE_PARAM(mesh_perr_parse, mesh_perr_parse_gen_params);
// kunit_test_suite(element_parsing);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
