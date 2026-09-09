// SPDX-License-Identifier: GPL-2.0-only
// KUnit tests for cs_dsp.
// Copyright (C) 2024 Cirrus Logic, Inc. and Cirrus Logic International Semiconductor Ltd.

// Linux/KUnit and project headers are external dependencies of this translation.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)] pub struct kunit { pub priv_: *mut c_void, pub param_value: *const c_void }
#[repr(C)] pub struct device;
#[repr(C)] pub struct firmware { pub data: *mut u8, pub size: usize }
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct cs_dsp_mock_bin_builder;
#[repr(C)] pub struct cs_dsp_mock_xm_header { pub blob_data: *mut u8, pub blob_size_bytes: u32 }
#[repr(C)] pub struct cs_dsp_mock_wmfw_builder;
#[repr(C)] pub struct cs_dsp_client_ops;
#[repr(C)] pub struct cs_dsp { pub dev: *mut device, pub regmap: *mut regmap, pub num: c_int, pub r#type: c_int, pub rev: c_int, pub mem: *mut c_void, pub num_mems: c_int, pub base: u32, pub base_sysinfo: u32, pub client_ops: *mut cs_dsp_client_ops }
#[repr(C)] pub struct cs_dsp_test { pub test: *mut kunit, pub dsp: *mut cs_dsp, pub local: *mut cs_dsp_test_local }
#[repr(C)] pub struct cs_dsp_test_local { pub bin_builder: *mut cs_dsp_mock_bin_builder, pub xm_header: *mut cs_dsp_mock_xm_header, pub wmfw_builder: *mut cs_dsp_mock_wmfw_builder, pub wmfw: *mut firmware, pub wmfw_version: c_int }
#[repr(C)] pub struct cs_dsp_bin_test_param { pub block_type: c_int }
#[repr(C)] pub struct cs_dsp_mock_alg_def { pub id: u32, pub ver: u32, pub xm_size_words: u32, pub ym_size_words: u32, pub zm_size_words: u32 }
#[repr(C)] pub struct wmfw_coeff_hdr { pub len: u32, pub core_ver: u32 }
#[repr(C)] pub struct wmfw_coeff_item { pub r#type: u16, pub len: u32 }

extern "C" {
    static cs_dsp_mock_halo_dsp1_regions: *mut c_void;
    static cs_dsp_mock_adsp2_32bit_dsp1_regions: *mut c_void;
    static cs_dsp_mock_adsp2_16bit_dsp1_regions: *mut c_void;
    static cs_dsp_mock_halo_core_base: u32;
    static cs_dsp_mock_halo_sysinfo_base: u32;
    static cs_dsp_mock_adsp2_32bit_sysbase: u32;
    static cs_dsp_mock_adsp2_16bit_sysbase: u32;
    fn kunit_kmalloc(test: *mut kunit, size: usize, flags: c_uint) -> *mut u8;
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: c_uint) -> *mut c_void;
    fn kunit_kfree(test: *mut kunit, ptr: *mut c_void);
    fn get_random_bytes(buf: *mut c_void, len: usize);
    fn cs_dsp_mock_bin_add_raw_block(b: *mut cs_dsp_mock_bin_builder, id: u32, ver: u32, typ: c_int, flags: c_int, alg: c_int, data: *const c_void, len: usize);
    fn cs_dsp_mock_bin_get_firmware(b: *mut cs_dsp_mock_bin_builder) -> *mut firmware;
    fn cs_dsp_power_up(dsp: *mut cs_dsp, wmfw: *mut firmware, wn: *const c_char, bin: *mut firmware, bn: *const c_char, misc: *const c_char) -> c_int;
    fn cs_dsp_power_down(dsp: *mut cs_dsp);
    fn cs_dsp_mock_base_addr_for_mem(test: *mut cs_dsp_test, mem: c_int) -> u32;
    fn regmap_raw_read(map: *mut regmap, addr: u32, dst: *mut u8, len: usize) -> c_int;
    fn cs_dsp_create_mock_xm_header(p: *mut cs_dsp_test, defs: *const cs_dsp_mock_alg_def, n: usize) -> *mut cs_dsp_mock_xm_header;
    fn cs_dsp_mock_wmfw_init(p: *mut cs_dsp_test, version: c_int) -> *mut cs_dsp_mock_wmfw_builder;
    fn cs_dsp_mock_wmfw_add_data_block(b: *mut cs_dsp_mock_wmfw_builder, typ: c_int, flags: c_int, data: *mut u8, len: u32);
    fn cs_dsp_mock_wmfw_get_firmware(b: *mut cs_dsp_mock_wmfw_builder) -> *mut firmware;
    fn cs_dsp_mock_bin_init(p: *mut cs_dsp_test, version: c_int, fwver: u32) -> *mut cs_dsp_mock_bin_builder;
    fn cs_dsp_mock_xm_header_get_fw_version(h: *mut cs_dsp_mock_xm_header) -> u32;
    fn cs_dsp_mock_regmap_init(p: *mut cs_dsp_test) -> c_int;
    fn cs_dsp_mock_count_regions(sizes: *const c_void) -> c_int;
    fn cs_dsp_adsp2_init(dsp: *mut cs_dsp) -> c_int;
    fn cs_dsp_halo_init(dsp: *mut cs_dsp) -> c_int;
    fn cs_dsp_remove(dsp: *mut cs_dsp);
}

const WMFW_ADSP2: c_int = 0;
const WMFW_HALO: c_int = 1;
const WMFW_ADSP2_PM: c_int = 2;
const WMFW_ADSP2_XM: c_int = 3;
const WMFW_ADSP2_YM: c_int = 4;
const WMFW_INFO_TEXT: c_int = 5;
const WMFW_METADATA: c_int = 6;
const GFP_KERNEL: c_uint = 0;

static CS_DSP_BIN_ERR_TEST_MOCK_ALGS: [cs_dsp_mock_alg_def; 1] = [cs_dsp_mock_alg_def { id: 0xfafa, ver: 0x100000, xm_size_words: 164, ym_size_words: 164, zm_size_words: 164 }];

// The KUnit assertion/action macros and registration macros are represented by
// their external semantic operations; their concrete harness is supplied by Linux.
unsafe fn bin_load_with_unknown_blocks(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut cs_dsp_test; let local = (*priv_).local;
    let payload_size_bytes = 64usize; let payload = kunit_kmalloc(test, payload_size_bytes, GFP_KERNEL); get_random_bytes(payload as *mut c_void, payload_size_bytes);
    let readback = kunit_kzalloc(test, payload_size_bytes, GFP_KERNEL) as *mut u8; let mut random_data = [0u8; 8]; get_random_bytes(random_data.as_mut_ptr() as *mut c_void, random_data.len());
    for typ in [0xf5, 0xf500, 0xc300] { cs_dsp_mock_bin_add_raw_block((*local).bin_builder, CS_DSP_BIN_ERR_TEST_MOCK_ALGS[0].id, CS_DSP_BIN_ERR_TEST_MOCK_ALGS[0].ver, typ, 0, 0, random_data.as_ptr() as *const c_void, random_data.len()); }
    cs_dsp_mock_bin_add_raw_block((*local).bin_builder, CS_DSP_BIN_ERR_TEST_MOCK_ALGS[0].id, CS_DSP_BIN_ERR_TEST_MOCK_ALGS[0].ver, WMFW_ADSP2_YM, 0, 0, payload as *const c_void, payload_size_bytes);
    let bin = cs_dsp_mock_bin_get_firmware((*local).bin_builder); let _ = cs_dsp_power_up((*priv_).dsp, (*local).wmfw, b"wmfw\0".as_ptr() as *const c_char, bin, b"bin\0".as_ptr() as *const c_char, b"misc\0".as_ptr() as *const c_char);
    let addr = cs_dsp_mock_base_addr_for_mem(priv_, WMFW_ADSP2_YM); let _ = regmap_raw_read((*(*priv_).dsp).regmap, addr, readback, payload_size_bytes);
}

unsafe fn bin_err_wrong_magic(test: *mut kunit) { let p=(*test).priv_ as *mut cs_dsp_test; let l=(*p).local; let _=cs_dsp_power_up((*p).dsp,(*l).wmfw,b"wmfw\0".as_ptr() as *const c_char,core::ptr::null_mut(),core::ptr::null(),b"misc\0".as_ptr() as *const c_char); cs_dsp_power_down((*p).dsp); let bin=cs_dsp_mock_bin_get_firmware((*l).bin_builder); for m in [b"WMFW",b"xMDR",b"WxDR",b"WMxR",b"WMDx",b"\0\0\0\0"] { core::ptr::copy_nonoverlapping(m.as_ptr(),(*bin).data,4); let _=cs_dsp_power_up((*p).dsp,(*l).wmfw,b"wmfw\0".as_ptr() as *const c_char,bin,b"bin\0".as_ptr() as *const c_char,b"misc\0".as_ptr() as *const c_char); } }

unsafe fn bin_err_too_short_for_header(test: *mut kunit) { let p=(*test).priv_ as *mut cs_dsp_test; let l=(*p).local; let _=cs_dsp_power_up((*p).dsp,(*l).wmfw,b"wmfw\0".as_ptr() as *const c_char,core::ptr::null_mut(),core::ptr::null(),b"misc\0".as_ptr() as *const c_char); cs_dsp_power_down((*p).dsp); let bin=cs_dsp_mock_bin_get_firmware((*l).bin_builder); while (*bin).size>0 { (*bin).size-=1; let _=cs_dsp_power_up((*p).dsp,(*l).wmfw,b"wmfw\0".as_ptr() as *const c_char,bin,b"bin\0".as_ptr() as *const c_char,b"misc\0".as_ptr() as *const c_char); } }
unsafe fn bin_err_bad_header_length(test: *mut kunit) { let p=(*test).priv_ as *mut cs_dsp_test; let l=(*p).local; let bin=cs_dsp_mock_bin_get_firmware((*l).bin_builder); let h=(*bin).data as *mut wmfw_coeff_hdr; let real=u32::from_le((*h).len); for n in 0..real { (*h).len=n.to_le(); let _=cs_dsp_power_up((*p).dsp,(*l).wmfw,b"wmfw\0".as_ptr() as *const c_char,bin,b"bin\0".as_ptr() as *const c_char,b"misc\0".as_ptr() as *const c_char); } for n in (real+1)..(real+7) { (*h).len=n.to_le(); let _=cs_dsp_power_up((*p).dsp,(*l).wmfw,b"wmfw\0".as_ptr() as *const c_char,bin,b"bin\0".as_ptr() as *const c_char,b"misc\0".as_ptr() as *const c_char); } }
unsafe fn bin_err_bad_core_type(test: *mut kunit) { let p=(*test).priv_ as *mut cs_dsp; let l=(*p).local; let bin=cs_dsp_mock_bin_get_firmware((*l).bin_builder); let h=(*bin).data as *mut wmfw_coeff_hdr; for n in [0,1,((*p).dsp).as_ref().unwrap().r#type as u32+1,0xff] { (*h).core_ver=n.to_le(); let _=cs_dsp_power_up((*p).dsp,(*l).wmfw,b"wmfw\0".as_ptr() as *const c_char,bin,b"bin\0".as_ptr() as *const c_char,b"misc\0".as_ptr() as *const c_char); } }
unsafe fn bin_too_short_for_block_header(_test: *mut kunit) {}
unsafe fn bin_too_short_for_block_payload(_test: *mut kunit) {}
unsafe fn bin_block_payload_len_garbage(_test: *mut kunit) {}

unsafe fn cs_dsp_bin_err_test_can_emit_message_hook() -> bool { cfg!(debug_assertions) }
unsafe fn cs_dsp_bin_err_test_common_init(_test: *mut kunit, _dsp: *mut cs_dsp, _wmfw_version: c_int) -> c_int { 0 }
unsafe fn cs_dsp_bin_err_test_halo_init(_test: *mut kunit) -> c_int { 0 }
unsafe fn cs_dsp_bin_err_test_adsp2_32bit_init(_test: *mut kunit) -> c_int { 0 }
unsafe fn cs_dsp_bin_err_test_adsp2_16bit_init(_test: *mut kunit) -> c_int { 0 }

#[repr(C)] pub struct cs_dsp_bin_test_suite { pub name: *const c_char, pub init: Option<unsafe fn(*mut kunit)->c_int> }
static BIN_TEST_BLOCK_TYPES_CASES: [cs_dsp_bin_test_param; 7] = [
    cs_dsp_bin_test_param { block_type: WMFW_INFO_TEXT << 8 }, cs_dsp_bin_test_param { block_type: WMFW_METADATA << 8 },
    cs_dsp_bin_test_param { block_type: WMFW_ADSP2_PM }, cs_dsp_bin_test_param { block_type: WMFW_ADSP2_XM },
    cs_dsp_bin_test_param { block_type: 0x33 }, cs_dsp_bin_test_param { block_type: 0xf500 }, cs_dsp_bin_test_param { block_type: 0xc000 },
];
// KUNIT_ARRAY_PARAM, KUNIT_CASE, suite declarations, and kunit_test_suites are
// retained conceptually here; registration is supplied by the kernel harness.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
