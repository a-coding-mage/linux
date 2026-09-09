// SPDX-License-Identifier: GPL-2.0-only
/*
 * KUnit tests for element fragmentation
 *
 * Copyright (C) 2023-2024 Intel Corporation
 */
// Dependencies supplied by the Linux headers and KUnit environment.

extern "C" {
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut u8;
    fn cfg80211_defragment_element(
        element: *const core::ffi::c_void,
        data: *const u8,
        len: usize,
        out: *mut u8,
        out_len: isize,
        frag_id: u8,
    ) -> isize;
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct element {
    _private: [u8; 0],
}

const GFP_KERNEL: u32 = 0;
const WLAN_EID_EXTENSION: u8 = 255;
const WLAN_EID_EXT_EHT_MULTI_LINK: u8 = 107;
const WLAN_EID_FRAGMENT: u8 = 242;

unsafe fn defragment_0(test: *mut kunit) {
    let mut input = [0u8; 265];
    input[0] = WLAN_EID_EXTENSION;
    input[1] = 254;
    input[2] = WLAN_EID_EXT_EHT_MULTI_LINK;
    input[27] = 27;
    input[123] = 123;
    input[254 + 2] = WLAN_EID_FRAGMENT;
    input[254 + 3] = 7;
    input[254 + 3 + 7] = 0; // for size
    let data = kunit_kzalloc(test, input.len(), GFP_KERNEL);

    kunit_assert_not_null!(test, data);
    let mut ret = cfg80211_defragment_element(input.as_ptr() as *const _, input.as_ptr(), input.len(), core::ptr::null_mut(), 0, WLAN_EID_FRAGMENT);
    kunit_expect_eq!(test, ret, 253);
    ret = cfg80211_defragment_element(input.as_ptr() as *const _, input.as_ptr(), input.len(), data, ret, WLAN_EID_FRAGMENT);
    kunit_expect_eq!(test, ret, 253);
    kunit_expect_memeq!(test, data, input.as_ptr().add(3), 253);
}

unsafe fn defragment_1(test: *mut kunit) {
    let mut input = [0u8; 276];
    input[0] = WLAN_EID_EXTENSION; input[1] = 255; input[2] = WLAN_EID_EXT_EHT_MULTI_LINK;
    input[27] = 27; input[123] = 123;
    input[255 + 2] = WLAN_EID_FRAGMENT; input[255 + 3] = 7; input[255 + 3 + 1] = 0xaa;
    input[255 + 3 + 8] = WLAN_EID_FRAGMENT; input[255 + 3 + 9] = 1; input[255 + 3 + 10] = 0; // for size
    let data = kunit_kzalloc(test, input.len(), GFP_KERNEL);
    let mut count = 0;
    kunit_assert_not_null!(test, data);
    for_each_element!(elem, input.as_ptr(), input.len(), { count += 1; });
    kunit_assert_eq!(test, count, 3);
    let mut ret = cfg80211_defragment_element(input.as_ptr() as *const _, input.as_ptr(), input.len(), core::ptr::null_mut(), 0, WLAN_EID_FRAGMENT);
    kunit_expect_eq!(test, ret, 254 + 7);
    ret = cfg80211_defragment_element(input.as_ptr() as *const _, input.as_ptr(), input.len(), data, ret, WLAN_EID_FRAGMENT);
    kunit_expect_eq!(test, ret, 254 + 7); // this means the last fragment was not used
    kunit_expect_memeq!(test, data, input.as_ptr().add(3), 254);
    kunit_expect_memeq!(test, data.add(254), input.as_ptr().add(255 + 4), 7);
}

unsafe fn defragment_2(test: *mut kunit) {
    let mut input = [0u8; 520];
    input[0] = WLAN_EID_EXTENSION; input[1] = 255; input[2] = WLAN_EID_EXT_EHT_MULTI_LINK;
    input[27] = 27; input[123] = 123;
    input[257] = WLAN_EID_FRAGMENT; input[258] = 255; input[277] = 0xaa;
    input[514] = WLAN_EID_FRAGMENT; input[515] = 1; input[516] = 0xcc; input[517] = WLAN_EID_FRAGMENT; input[518] = 1; input[519] = 0;
    let data = kunit_kzalloc(test, input.len(), GFP_KERNEL);
    let mut count = 0;
    kunit_assert_not_null!(test, data);
    for_each_element!(elem, input.as_ptr(), input.len(), { count += 1; });
    kunit_assert_eq!(test, count, 4);
    let mut ret = cfg80211_defragment_element(input.as_ptr() as *const _, input.as_ptr(), input.len(), core::ptr::null_mut(), 0, WLAN_EID_FRAGMENT);
    kunit_expect_eq!(test, ret, 254 + 255 + 1);
    ret = cfg80211_defragment_element(input.as_ptr() as *const _, input.as_ptr(), input.len(), data, ret, WLAN_EID_FRAGMENT);
    kunit_expect_eq!(test, ret, 254 + 255 + 1);
    kunit_expect_memeq!(test, data, input.as_ptr().add(3), 254);
    kunit_expect_memeq!(test, data.add(254), input.as_ptr().add(257 + 2), 255);
    kunit_expect_memeq!(test, data.add(254 + 255), input.as_ptr().add(2 * 257 + 2), 1);
}

unsafe fn defragment_at_end(test: *mut kunit) {
    let mut input = [0u8; 266];
    input[0] = WLAN_EID_EXTENSION; input[1] = 255; input[2] = WLAN_EID_EXT_EHT_MULTI_LINK;
    input[27] = 27; input[123] = 123; input[257] = WLAN_EID_FRAGMENT; input[258] = 7; input[265] = 0;
    let data = kunit_kzalloc(test, input.len(), GFP_KERNEL);
    kunit_assert_not_null!(test, data);
    let mut ret = cfg80211_defragment_element(input.as_ptr() as *const _, input.as_ptr(), input.len(), core::ptr::null_mut(), 0, WLAN_EID_FRAGMENT);
    kunit_expect_eq!(test, ret, 254 + 7);
    ret = cfg80211_defragment_element(input.as_ptr() as *const _, input.as_ptr(), input.len(), data, ret, WLAN_EID_FRAGMENT);
    kunit_expect_eq!(test, ret, 254 + 7);
    kunit_expect_memeq!(test, data, input.as_ptr().add(3), 254);
    kunit_expect_memeq!(test, data.add(254), input.as_ptr().add(255 + 4), 7);
}

static ELEMENT_FRAGMENTATION_TEST_CASES: &[unsafe fn(*mut kunit)] = &[defragment_0, defragment_1, defragment_2, defragment_at_end];
static ELEMENT_FRAGMENTATION: &str = "cfg80211-element-defragmentation";
// kunit_test_suite(element_fragmentation);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
