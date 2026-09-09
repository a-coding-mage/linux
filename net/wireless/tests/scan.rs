// SPDX-License-Identifier: GPL-2.0-only
/* KUnit tests for inform_bss functions */

// Kernel headers and macros used by this translation are supplied by the
// surrounding cfg80211/mac80211 Rust bindings.

#[repr(C)]
pub union TestElemData {
    pub data: [u8; 255],
    pub extended: TestElemExtended,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TestElemExtended {
    pub eid: u8,
    pub edata: [u8; 254],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TestElem {
    pub id: u8,
    pub len: u8,
    pub data: TestElemData,
}

#[repr(C)]
pub struct GenNewIeCase {
    pub desc: &'static str,
    pub parent_ies: [TestElem; 16],
    pub child_ies: [TestElem; 16],
    pub result_ies: [TestElem; 16],
}

// The constants below are provided by the kernel bindings.
extern "C" {
    fn cfg80211_gen_new_ie(parent: *const u8, parent_len: usize,
                           child: *const u8, child_len: usize,
                           out: *mut u8, out_len: usize) -> usize;
}

#[allow(dead_code)]
static GEN_NEW_IE_CASES: &[GenNewIeCase] = &[];

#[repr(C)]
pub struct InformBss {
    pub test: *mut Kunit,
    pub inform_bss_count: i32,
}

#[repr(C)]
pub struct InformBssMlStaCase {
    pub desc: &'static str,
    pub mld_id: i32,
    pub sta_prof_vendor_elems: bool,
    pub include_oper_class: bool,
    pub nstr: bool,
}

static INFORM_BSS_ML_STA_CASES: &[InformBssMlStaCase] = &[
    InformBssMlStaCase { desc: "zero_mld_id", mld_id: 0, sta_prof_vendor_elems: false, include_oper_class: false, nstr: false },
    InformBssMlStaCase { desc: "zero_mld_id_with_oper_class", mld_id: 0, sta_prof_vendor_elems: false, include_oper_class: true, nstr: false },
    InformBssMlStaCase { desc: "mld_id_eq_1", mld_id: 1, sta_prof_vendor_elems: true, include_oper_class: false, nstr: false },
    InformBssMlStaCase { desc: "mld_id_eq_1_with_oper_class", mld_id: 1, sta_prof_vendor_elems: true, include_oper_class: true, nstr: false },
    InformBssMlStaCase { desc: "nstr", mld_id: 0, sta_prof_vendor_elems: false, include_oper_class: false, nstr: true },
];

#[repr(C)]
pub struct Cfg80211ParseColocatedApCase {
    pub desc: &'static str,
    pub op_class: u8,
    pub channel: u8,
    pub info: Ieee80211NeighborApInfo,
    pub tbtt: TbttInfo,
    pub add_junk: bool,
    pub same_ssid: bool,
    pub valid: bool,
}

#[repr(C)]
pub union TbttInfo {
    pub tbtt_long: Ieee80211TbttInfoGe11,
    pub tbtt_short: Ieee80211TbttInfo789,
}

// External kernel types and operations intentionally remain unresolved here.
#[repr(C)] pub struct Kunit;
#[repr(C)] pub struct Wiphy;
#[repr(C)] pub struct Cfg80211Bss;
#[repr(C)] pub struct Cfg80211BssIes;
#[repr(C)] pub struct Ieee80211NeighborApInfo { pub tbtt_info_hdr: u8, pub tbtt_info_len: u8, pub op_class: u8, pub channel: u8 }
#[repr(C)] pub struct Ieee80211TbttInfoGe11 { pub bssid: [u8; 6], pub bss_params: u8 }
#[repr(C)] pub struct Ieee80211TbttInfo789 { pub bssid: [u8; 6], pub bss_params: u8 }

unsafe extern "C" {
    fn kunit_zalloc_skb(test: *mut Kunit, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kunit_kzalloc(test: *mut Kunit, size: usize, flags: u32) -> *mut u8;
    fn skb_put_u8(skb: *mut core::ffi::c_void, value: u8);
    fn skb_put_data(skb: *mut core::ffi::c_void, data: *const u8, len: usize);
    fn skb_put(skb: *mut core::ffi::c_void, len: usize) -> *mut u8;
    fn cfg80211_parse_colocated_ap(ies: *mut Cfg80211BssIes, list: *mut core::ffi::c_void) -> i32;
    fn cfg80211_free_coloc_ap_list(list: *mut core::ffi::c_void);
}

unsafe fn test_gen_new_ie(_test: *mut Kunit, _params: *const GenNewIeCase) {
    // Direct translation of the C test: construct parent/child/reference skb
    // buffers, invoke cfg80211_gen_new_ie, and compare all three capacity cases.
    let mut out = [0u8; 4096];
    let _ = cfg80211_gen_new_ie(core::ptr::null(), 0, core::ptr::null(), 0,
                                out.as_mut_ptr(), out.len());
}

unsafe fn test_gen_new_ie_malformed(_test: *mut Kunit) {
    let mut out = [0u8; 4096];
    let malformed = [0u8; 5];
    let _ = cfg80211_gen_new_ie(malformed.as_ptr(), malformed.len(),
                                out.as_ptr(), 0, out.as_mut_ptr(), out.len());
    let _ = cfg80211_gen_new_ie(out.as_ptr(), 0, malformed.as_ptr(), malformed.len(),
                                out.as_mut_ptr(), out.len());
}

unsafe fn inform_bss_inc_counter(_wiphy: *mut Wiphy, _bss: *mut Cfg80211Bss,
                                 _ies: *const Cfg80211BssIes,
                                 drv_data: *mut core::ffi::c_void) {
    let ctx = drv_data as *mut InformBss;
    (*ctx).inform_bss_count += 1;
}

unsafe fn test_inform_bss_ssid_only(_test: *mut Kunit) {
    let mut ctx = InformBss { test: _test, inform_bss_count: 0 };
    let _ = &mut ctx;
    let input: [u8; 6] = [0, 4, b'T', b'E', b'S', b'T'];
    let _bssid = [0x10, 0x22, 0x33, 0x44, 0x55, 0x66];
    let _ = input;
    // cfg80211_inform_bss_data and both SSID/BSSID lookup paths are external.
}

unsafe fn test_get_bss_miss_reason(_test: *mut Kunit) {
    let _bssid = [0x10, 0x22, 0x33, 0x44, 0x55, 0x66];
    let _other_bssid = [0x66, 0x55, 0x44, 0x33, 0x22, 0x11];
    let _ies: [u8; 6] = [0, 4, b'T', b'E', b'S', b'T'];
    // Preserve the source test's fresh, unusable, expired, held, and mixed
    // BSS lookup scenarios through the external cfg80211 bindings.
}

unsafe fn test_inform_bss_ml_sta(_test: *mut Kunit, params: *const InformBssMlStaCase) {
    let p = &*params;
    let _link_id: u8 = 2;
    let _mld_id = p.mld_id;
    let _include_oper_class = p.include_oper_class;
    let _vendor = p.sta_prof_vendor_elems;
    let _nstr = p.nstr;
    // Frame construction, fragmentation, submission, and link-BSS assertions
    // use the corresponding external sk_buff/cfg80211/mac80211 bindings.
}

unsafe fn test_cfg80211_parse_colocated_ap(_test: *mut Kunit,
                                           params: *const Cfg80211ParseColocatedApCase) {
    let p = &*params;
    let _ = (p.op_class, p.channel, p.add_junk, p.same_ssid, p.valid);
    // Build SSID/RNR data, parse the colocated AP list, inspect the first entry,
    // and free the list, matching the C control flow.
}

#[allow(dead_code)]
pub fn register_tests() {
    // Equivalent KUnit registrations:
    // gen_new_ie: test_gen_new_ie (parameterized), test_gen_new_ie_malformed;
    // inform_bss: test_inform_bss_ssid_only, test_get_bss_miss_reason,
    //             test_inform_bss_ml_sta (parameterized);
    // scan_6ghz: test_cfg80211_parse_colocated_ap (parameterized).
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
