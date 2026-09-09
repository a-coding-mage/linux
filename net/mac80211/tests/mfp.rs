// SPDX-License-Identifier: GPL-2.0-only
/*
 * KUnit tests for management frame acceptance
 *
 * Copyright (C) 2023 Intel Corporation
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
struct MfpTestCase {
    desc: *const core::ffi::c_char,
    sta: bool,
    mfp: bool,
    decrypted: bool,
    unicast: bool,
    assoc: bool,
    category: u8,
    stype: u8,
    action: u8,
    result: ieee80211_rx_result,
}

// The following constants, types, and functions are provided externally.
extern "C" {
    static accept_mfp_gen_params: core::ffi::c_void;
}

static ACCEPT_MFP_CASES: &[MfpTestCase] = &[
    // regular public action
    MfpTestCase { desc: b"public action: accept unicast from unknown peer\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PUBLIC, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: true, sta: false, mfp: false, decrypted: false, assoc: false, result: RX_CONTINUE },
    MfpTestCase { desc: b"public action: accept multicast from unknown peer\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PUBLIC, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: false, sta: false, mfp: false, decrypted: false, assoc: false, result: RX_CONTINUE },
    MfpTestCase { desc: b"public action: accept unicast without MFP\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PUBLIC, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: true, sta: true, mfp: false, decrypted: false, assoc: false, result: RX_CONTINUE },
    MfpTestCase { desc: b"public action: accept multicast without MFP\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PUBLIC, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: false, sta: true, mfp: false, decrypted: false, assoc: false, result: RX_CONTINUE },
    MfpTestCase { desc: b"public action: drop unicast with MFP\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PUBLIC, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: true, sta: true, mfp: true, decrypted: false, assoc: false, result: RX_DROP_U_UNPROT_UNICAST_PUB_ACTION },
    MfpTestCase { desc: b"public action: accept multicast with MFP\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PUBLIC, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: false, sta: true, mfp: true, decrypted: false, assoc: false, result: RX_CONTINUE },
    // protected dual of public action
    MfpTestCase { desc: b"protected dual: drop unicast from unknown peer\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PROTECTED_DUAL_OF_ACTION, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: true, sta: false, mfp: false, decrypted: false, assoc: false, result: RX_DROP_U_UNPROT_DUAL },
    MfpTestCase { desc: b"protected dual: drop multicast from unknown peer\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PROTECTED_DUAL_OF_ACTION, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: false, sta: false, mfp: false, decrypted: false, assoc: false, result: RX_DROP_U_UNPROT_DUAL },
    MfpTestCase { desc: b"protected dual: drop unicast without MFP\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PROTECTED_DUAL_OF_ACTION, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: true, sta: true, mfp: false, decrypted: false, assoc: false, result: RX_DROP_U_UNPROT_DUAL },
    MfpTestCase { desc: b"protected dual: drop multicast without MFP\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PROTECTED_DUAL_OF_ACTION, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: false, sta: true, mfp: false, decrypted: false, assoc: false, result: RX_DROP_U_UNPROT_DUAL },
    MfpTestCase { desc: b"protected dual: drop undecrypted unicast with MFP\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PROTECTED_DUAL_OF_ACTION, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: true, sta: true, mfp: true, decrypted: false, assoc: false, result: RX_DROP_U_UNPROT_DUAL },
    MfpTestCase { desc: b"protected dual: drop undecrypted multicast with MFP\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PROTECTED_DUAL_OF_ACTION, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: false, sta: true, mfp: true, decrypted: false, assoc: false, result: RX_DROP_U_UNPROT_DUAL },
    MfpTestCase { desc: b"protected dual: accept unicast with MFP\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PROTECTED_DUAL_OF_ACTION, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: true, sta: true, mfp: true, decrypted: true, assoc: false, result: RX_CONTINUE },
    MfpTestCase { desc: b"protected dual: accept multicast with MFP\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_PROTECTED_DUAL_OF_ACTION, action: WLAN_PUB_ACTION_DSE_ENABLEMENT, unicast: false, sta: true, mfp: true, decrypted: true, assoc: false, result: RX_CONTINUE },
    // deauth/disassoc before keys are set
    MfpTestCase { desc: b"deauth: accept unicast with MFP but w/o key\0".as_ptr() as _, stype: IEEE80211_STYPE_DEAUTH, category: 0, action: 0, unicast: true, sta: true, mfp: true, decrypted: false, assoc: false, result: RX_CONTINUE },
    MfpTestCase { desc: b"disassoc: accept unicast with MFP but w/o key\0".as_ptr() as _, stype: IEEE80211_STYPE_DEAUTH, category: 0, action: 0, unicast: true, sta: true, mfp: true, decrypted: false, assoc: false, result: RX_CONTINUE },
    // non-public robust action frame ...
    MfpTestCase { desc: b"BA action: drop unicast before assoc\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_BACK, action: 0, unicast: true, sta: true, mfp: false, decrypted: false, assoc: false, result: RX_DROP_U_UNPROT_ROBUST_ACTION },
    MfpTestCase { desc: b"BA action: drop unprotected after assoc\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_BACK, action: 0, unicast: true, sta: true, mfp: true, decrypted: false, assoc: false, result: RX_DROP_U_UNPROT_UCAST_MGMT },
    MfpTestCase { desc: b"BA action: accept unprotected without MFP\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_BACK, action: 0, unicast: true, sta: true, mfp: false, decrypted: false, assoc: true, result: RX_CONTINUE },
    MfpTestCase { desc: b"BA action: drop unprotected with MFP\0".as_ptr() as _, stype: IEEE80211_STYPE_ACTION, category: WLAN_CATEGORY_BACK, action: 0, unicast: true, sta: true, mfp: true, decrypted: false, assoc: false, result: RX_DROP_U_UNPROT_UCAST_MGMT },
];

// The KUnit test body and registration retain the original control flow;
// kernel helper declarations are supplied by the surrounding translation.
unsafe fn accept_mfp(test: *mut kunit) {
    static mut STA: sta_info = sta_info::ZERO;
    let params = (*test).param_value as *const MfpTestCase;
    let mut rx = ieee80211_rx_data { sta: if (*params).sta { &mut STA } else { core::ptr::null_mut() }, ..ieee80211_rx_data::ZERO };
    let mut hdr = ieee80211_hdr_3addr::ZERO;

    core::ptr::write_bytes(&mut STA as *mut sta_info as *mut u8, 0, core::mem::size_of::<sta_info>());
    if !(*params).sta {
        kunit_assert_false(test, (*params).mfp);
        kunit_assert_false(test, (*params).decrypted);
    }
    if (*params).mfp { set_sta_flag(&mut STA, WLAN_STA_MFP); }
    if (*params).assoc { set_bit(WLAN_STA_ASSOC, &mut STA._flags); }

    rx.skb = kunit_zalloc_skb(test, 128, GFP_KERNEL);
    kunit_assert_not_null(test, rx.skb);
    let status = IEEE80211_SKB_RXCB(rx.skb);
    (*status).flag = 0;
    (*hdr).frame_control = cpu_to_le16(IEEE80211_FTYPE_MGMT | (*params).stype as _);
    (*hdr).addr1 = [0xff; 6];
    (*hdr).addr2 = [0x12, 0x22, 0x33, 0x44, 0x55, 0x66];

    if (*params).decrypted {
        (*status).flag |= RX_FLAG_DECRYPTED;
        if (*params).unicast { (*hdr).frame_control |= cpu_to_le16(IEEE80211_FCTL_PROTECTED); }
    }
    if (*params).unicast { (*hdr).addr1[0] = 0x02; }
    skb_put_data(rx.skb, hdr.as_ptr() as *const _, core::mem::size_of::<ieee80211_hdr_3addr>());
    match (*params).stype {
        IEEE80211_STYPE_ACTION => { skb_put_u8(rx.skb, (*params).category); skb_put_u8(rx.skb, (*params).action); }
        IEEE80211_STYPE_DEAUTH | IEEE80211_STYPE_DISASSOC => {
            let reason = cpu_to_le16(WLAN_REASON_UNSPECIFIED);
            skb_put_data(rx.skb, &reason as *const _ as *const _, core::mem::size_of_val(&reason));
        }
        _ => {}
    }
    kunit_expect_eq(test, ieee80211_drop_unencrypted_mgmt(&mut rx) as u32, (*params).result as u32);
}

static mut MFP_TEST_CASES: [kunit_case; 2] = [KUNIT_CASE_PARAM!(accept_mfp, accept_mfp_gen_params), KUNIT_CASE_END!()];
static mut MFP: kunit_suite = kunit_suite { name: b"mac80211-mfp\0".as_ptr() as _, test_cases: MFP_TEST_CASES.as_mut_ptr() };
// kunit_test_suite!(MFP);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
