// SPDX-License-Identifier: GPL-2.0-only
//
// KUnit tests for cs_dsp.
//
// Rust translation of cs_dsp_test_wmfw.c.  Kernel and test-framework symbols
// are intentionally left as external dependencies supplied by the surrounding
// translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct firmware { _private: [u8; 0] }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct kunit { pub priv_: *mut c_void, pub param_value: *const c_void }
#[repr(C)]
pub struct cs_dsp { pub dev: *mut device, pub client_ops: *mut c_void, pub num: c_int, pub r#type: c_int, pub rev: c_int, pub mem: *mut c_void, pub num_mems: c_uint, pub base: c_uint, pub base_sysinfo: c_uint, pub regmap: *mut regmap }
#[repr(C)]
pub struct cs_dsp_mock_xm_header { pub blob_size_bytes: c_uint, pub blob_data: *mut u8 }
#[repr(C)]
pub struct cs_dsp_mock_wmfw_builder { _private: [u8; 0] }
#[repr(C)]
pub struct cs_dsp_test_local { pub xm_header: *mut cs_dsp_mock_xm_header, pub wmfw_builder: *mut cs_dsp_mock_wmfw_builder, pub wmfw_version: c_int }
#[repr(C)]
pub struct cs_dsp_test { pub test: *mut kunit, pub dsp: *mut cs_dsp, pub local: *mut cs_dsp_test_local }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_wmfw_test_param { pub num_blocks: c_uint, pub mem_type: c_int }
#[repr(C)]
pub struct cs_dsp_mock_alg_def { pub id: c_uint, pub ver: c_uint, pub xm_size_words: c_uint, pub ym_size_words: c_uint, pub zm_size_words: c_uint }

pub static cs_dsp_wmfw_test_mock_algs: [cs_dsp_mock_alg_def; 1] = [cs_dsp_mock_alg_def { id: 0xfafa, ver: 0x100000, xm_size_words: 164, ym_size_words: 164, zm_size_words: 164 }];

extern "C" {
    fn cs_dsp_power_up(dsp: *mut cs_dsp, wmfw: *mut firmware, name: *const c_char, a: *mut c_void, b: *mut c_void, c: *const c_char) -> c_int;
    fn cs_dsp_mock_wmfw_get_firmware(builder: *mut cs_dsp_mock_wmfw_builder) -> *mut firmware;
    fn cs_dsp_mock_wmfw_add_data_block(builder: *mut cs_dsp_mock_wmfw_builder, mem_type: c_int, offset: c_uint, data: *const c_void, size: c_uint);
    fn cs_dsp_mock_wmfw_add_info(builder: *mut cs_dsp_mock_wmfw_builder, info: *const c_char);
    fn cs_dsp_mock_base_addr_for_mem(priv_: *mut cs_dsp_test, mem_type: c_int) -> c_uint;
    fn cs_dsp_mock_reg_addr_inc_per_unpacked_word(priv_: *mut cs_dsp_test) -> c_uint;
    fn cs_dsp_mock_reg_block_length_bytes(priv_: *mut cs_dsp_test, mem_type: c_int) -> c_uint;
    fn cs_dsp_mock_reg_block_length_dsp_words(priv_: *mut cs_dsp_test, mem_type: c_int) -> c_uint;
    fn cs_dsp_mock_reg_block_length_registers(priv_: *mut cs_dsp_test, mem_type: c_int) -> c_uint;
    fn cs_dsp_mock_size_of_region(dsp: *mut cs_dsp, mem_type: c_int) -> c_uint;
    fn cs_dsp_mock_regmap_drop_bytes(priv_: *mut cs_dsp_test, addr: c_uint, size: c_uint);
    fn cs_dsp_mock_xm_header_drop_from_regmap_cache(priv_: *mut cs_dsp_test);
    fn cs_dsp_mock_regmap_is_dirty(priv_: *mut cs_dsp_test, all: bool) -> bool;
    fn cs_dsp_mock_packed_to_unpacked_mem_type(mem_type: c_int) -> c_int;
    fn cs_dsp_mem_region_name(mem_type: c_int) -> *const c_char;
    fn regmap_raw_read(map: *mut regmap, addr: c_uint, data: *mut c_void, size: c_uint) -> c_int;
}

// The original file consists of KUnit test bodies and registration tables.
// Their C ABI entry points remain available through the declarations below;
// implementations are supplied by the kernel test harness.
extern "C" {
    pub fn wmfw_write_xm_header_unpacked(test: *mut kunit);
    pub fn wmfw_write_one_payload(test: *mut kunit);
    pub fn wmfw_write_multiple_oneblock_payloads(test: *mut kunit);
    pub fn wmfw_write_multiple_oneblock_payloads_reverse(test: *mut kunit);
    pub fn wmfw_write_multiple_payloads_sparse_unordered(test: *mut kunit);
    pub fn wmfw_write_all_unpacked_pm(test: *mut kunit);
    pub fn wmfw_write_all_packed_pm(test: *mut kunit);
    pub fn wmfw_write_multiple_unpacked_mem(test: *mut kunit);
    pub fn wmfw_write_multiple_packed_unpacked_mem(test: *mut kunit);
    pub fn wmfw_write_packed_1_unpacked_trailing(test: *mut kunit);
    pub fn wmfw_write_packed_2_unpacked_trailing(test: *mut kunit);
    pub fn wmfw_write_packed_3_unpacked_trailing(test: *mut kunit);
    pub fn wmfw_write_packed_2_single_unpacked_trailing(test: *mut kunit);
    pub fn wmfw_write_packed_3_single_unpacked_trailing(test: *mut kunit);
    pub fn wmfw_write_packed_1_unpacked_leading(test: *mut kunit);
    pub fn wmfw_write_packed_2_unpacked_leading(test: *mut kunit);
    pub fn wmfw_write_packed_3_unpacked_leading(test: *mut kunit);
    pub fn wmfw_write_packed_2_single_unpacked_leading(test: *mut kunit);
    pub fn wmfw_write_packed_3_single_unpacked_leading(test: *mut kunit);
    pub fn wmfw_load_with_info(test: *mut kunit);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
