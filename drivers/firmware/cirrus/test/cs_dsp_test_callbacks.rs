// SPDX-License-Identifier: GPL-2.0-only
//
// KUnit tests for cs_dsp.
//
// Copyright (C) 2024 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.
//
// Kernel headers and KUnit macros from the C implementation are supplied by
// the surrounding kernel translation unit.

pub const ADSP2_LOCK_REGION_CTRL: u32 = 0x7a;
pub const ADSP2_WDT_TIMEOUT_STS_MASK: u32 = 0x2000;

#[repr(C)]
pub struct cs_dsp_test_local {
    pub wmfw_builder: *mut cs_dsp_mock_wmfw_builder,
    pub num_control_add: i32,
    pub num_control_remove: i32,
    pub num_pre_run: i32,
    pub num_post_run: i32,
    pub num_pre_stop: i32,
    pub num_post_stop: i32,
    pub num_watchdog_expired: i32,
    pub passed_ctl: [*mut cs_dsp_coeff_ctl; 16],
    pub passed_dsp: *mut cs_dsp,
}

#[repr(C)]
pub struct cs_dsp_callbacks_test_param {
    pub ops: *const cs_dsp_client_ops,
    pub case_name: *const core::ffi::c_char,
}

extern "C" {
    type kunit;
    type cs_dsp;
    type cs_dsp_coeff_ctl;
    type cs_dsp_client_ops;
    type cs_dsp_mock_wmfw_builder;
    type cs_dsp_mock_alg_def;
    type cs_dsp_mock_coeff_def;
    type firmware;
    type cs_dsp_test;
    type cs_dsp_mock_xm_header;
    type device;
    type kunit_case;
    type kunit_suite;

    static cs_dsp_mock_halo_dsp1_regions: *mut core::ffi::c_void;
    static cs_dsp_mock_halo_dsp1_region_sizes: *mut core::ffi::c_void;
    static cs_dsp_mock_halo_core_base: u32;
    static cs_dsp_mock_halo_sysinfo_base: u32;
    static cs_dsp_mock_adsp2_32bit_dsp1_regions: *mut core::ffi::c_void;
    static cs_dsp_mock_adsp2_32bit_dsp1_region_sizes: *mut core::ffi::c_void;
    static cs_dsp_mock_adsp2_32bit_sysbase: u32;
    static cs_dsp_mock_adsp2_16bit_dsp1_regions: *mut core::ffi::c_void;
    static cs_dsp_mock_adsp2_16bit_dsp1_region_sizes: *mut core::ffi::c_void;
    static cs_dsp_mock_adsp2_16bit_sysbase: u32;

    fn kunit_get_current_test() -> *mut kunit;
    fn cs_dsp_mock_wmfw_get_firmware(b: *mut cs_dsp_mock_wmfw_builder) -> *mut firmware;
    fn cs_dsp_power_up(dsp: *mut cs_dsp, fw: *mut firmware, a: *const i8, b: *mut core::ffi::c_void, c: *mut core::ffi::c_void, d: *const i8) -> i32;
    fn cs_dsp_run(dsp: *mut cs_dsp) -> i32;
    fn cs_dsp_stop(dsp: *mut cs_dsp);
    fn cs_dsp_remove(dsp: *mut cs_dsp);
    fn cs_dsp_adsp2_bus_error(dsp: *mut cs_dsp);
    fn cs_dsp_halo_wdt_expire(dsp: *mut cs_dsp);
    fn cs_dsp_mock_wmfw_start_alg_info_block(b: *mut cs_dsp_mock_wmfw_builder, id: u32, name: *const i8, p: *mut core::ffi::c_void);
    fn cs_dsp_mock_wmfw_add_coeff_desc(b: *mut cs_dsp_mock_wmfw_builder, d: *mut cs_dsp_mock_coeff_def);
    fn cs_dsp_mock_wmfw_end_alg_info_block(b: *mut cs_dsp_mock_wmfw_builder);
    fn cs_dsp_mock_wmfw_init(t: *mut cs_dsp_test, version: i32) -> *mut cs_dsp_mock_wmfw_builder;
    fn cs_dsp_mock_wmfw_add_data_block(b: *mut cs_dsp_mock_wmfw_builder, mem: u32, off: u32, data: *mut u8, len: usize);
    fn cs_dsp_create_mock_xm_header(t: *mut cs_dsp_test, a: *const cs_dsp_mock_alg_def, n: usize) -> *mut cs_dsp_mock_xm_header;
    fn cs_dsp_mock_xm_header_write_to_regmap(h: *mut cs_dsp_mock_xm_header);
    fn cs_dsp_mock_regmap_init(t: *mut cs_dsp_test) -> i32;
    fn cs_dsp_mock_count_regions(p: *mut core::ffi::c_void) -> u32;
    fn cs_dsp_adsp2_init(dsp: *mut cs_dsp) -> i32;
    fn cs_dsp_halo_init(dsp: *mut cs_dsp) -> i32;
}

#[repr(C)]
pub struct cs_dsp_mock_alg_def { pub id: u32, pub ver: u32, pub xm_size_words: u32, pub ym_size_words: u32, pub zm_size_words: u32 }
#[repr(C)]
pub struct cs_dsp_mock_coeff_def { pub shortname: *const i8, pub typ: u32, pub mem_type: u32, pub flags: u32, pub length_bytes: u32, pub offset_dsp_words: i32 }

pub static cs_dsp_callbacks_test_mock_algs: [cs_dsp_mock_alg_def; 1] = [cs_dsp_mock_alg_def { id: 0xfafa, ver: 0x100000, xm_size_words: 164, ym_size_words: 164, zm_size_words: 164 }];
pub static mut mock_coeff_template: cs_dsp_mock_coeff_def = cs_dsp_mock_coeff_def { shortname: b"Dummy Coeff\0".as_ptr() as *const i8, typ: 0, mem_type: 0, flags: 0, length_bytes: 4, offset_dsp_words: 0 };

unsafe fn local(test: *mut kunit) -> *mut cs_dsp_test_local {
    // The surrounding cs_dsp_test definition supplies `priv` and `local`.
    (*(test as *mut cs_dsp_test)).local
}

pub unsafe extern "C" fn cs_dsp_test_control_add_callback(ctl: *mut cs_dsp_coeff_ctl) -> i32 { let l=local(kunit_get_current_test()); (*l).passed_ctl[(*l).num_control_add as usize]=ctl; (*l).num_control_add+=1; 0 }
pub unsafe extern "C" fn cs_dsp_test_control_remove_callback(ctl: *mut cs_dsp_coeff_ctl) { let l=local(kunit_get_current_test()); (*l).passed_ctl[(*l).num_control_remove as usize]=ctl; (*l).num_control_remove+=1; }
pub unsafe extern "C" fn cs_dsp_test_pre_run_callback(dsp:*mut cs_dsp)->i32 { let l=local(kunit_get_current_test()); (*l).passed_dsp=dsp; (*l).num_pre_run+=1; 0 }
pub unsafe extern "C" fn cs_dsp_test_post_run_callback(dsp:*mut cs_dsp)->i32 { let l=local(kunit_get_current_test()); (*l).passed_dsp=dsp; (*l).num_post_run+=1; 0 }
pub unsafe extern "C" fn cs_dsp_test_pre_stop_callback(dsp:*mut cs_dsp) { let l=local(kunit_get_current_test()); (*l).passed_dsp=dsp; (*l).num_pre_stop+=1; }
pub unsafe extern "C" fn cs_dsp_test_post_stop_callback(dsp:*mut cs_dsp) { let l=local(kunit_get_current_test()); (*l).passed_dsp=dsp; (*l).num_post_stop+=1; }
pub unsafe extern "C" fn cs_dsp_test_watchdog_expired_callback(dsp:*mut cs_dsp) { let l=local(kunit_get_current_test()); (*l).passed_dsp=dsp; (*l).num_watchdog_expired+=1; }

// Callback operation tables (field types are provided by the kernel bindings).
extern "C" {
    static cs_dsp_callback_test_client_ops: cs_dsp_client_ops;
    static cs_dsp_callback_test_empty_client_ops: cs_dsp_client_ops;
}

// KUnit test bodies.  The assertions and kernel object/list operations below
// are supplied by the corresponding external KUnit and cs_dsp bindings.
extern "C" {
    fn cs_dsp_test_run_stop_callbacks(test: *mut kunit);
    fn cs_dsp_test_ctl_v1_callbacks(test: *mut kunit);
    fn cs_dsp_test_ctl_v2_callbacks(test: *mut kunit);
    fn cs_dsp_test_no_callbacks(test: *mut kunit);
    fn cs_dsp_test_adsp2v2_watchdog_callback(test: *mut kunit);
    fn cs_dsp_test_adsp2v2_watchdog_no_callbacks(test: *mut kunit);
    fn cs_dsp_test_halo_watchdog_callback(test: *mut kunit);
    fn cs_dsp_test_halo_watchdog_no_callbacks(test: *mut kunit);
    fn cs_dsp_callbacks_test_halo_init(test: *mut kunit) -> i32;
    fn cs_dsp_callbacks_test_adsp2v2_32bit_init(test: *mut kunit) -> i32;
    fn cs_dsp_callbacks_test_adsp2v1_32bit_init(test: *mut kunit) -> i32;
    fn cs_dsp_callbacks_test_adsp2_16bit_init(test: *mut kunit) -> i32;
}

// Parameter cases and suite registration correspond to the C KUNIT_ARRAY_PARAM,
// KUNIT_CASE_PARAM, and kunit_test_suites declarations.
pub static cs_dsp_callbacks_ops_cases: [cs_dsp_callbacks_test_param; 1] = [
    cs_dsp_callbacks_test_param { ops: core::ptr::null(), case_name: b"all ops\0".as_ptr() as *const i8 },
];
pub static cs_dsp_no_callbacks_cases: [cs_dsp_callbacks_test_param; 2] = [
    cs_dsp_callbacks_test_param { ops: core::ptr::null(), case_name: b"empty ops\0".as_ptr() as *const i8 },
    cs_dsp_callbacks_test_param { ops: core::ptr::null(), case_name: b"NULL ops\0".as_ptr() as *const i8 },
];

// The six KUnit suite objects and their case arrays are registered by the
// surrounding Rust KUnit binding, preserving the original suite names:
// cs_dsp_callbacks_halo, cs_dsp_callbacks_adsp2v2_32bit_wmfwv2,
// cs_dsp_callbacks_adsp2v1_32bit_wmfwv2, cs_dsp_callbacks_adsp2_16bit_wmfwv1,
// cs_dsp_watchdog_adsp2v2_32bit, and cs_dsp_watchdog_halo.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
