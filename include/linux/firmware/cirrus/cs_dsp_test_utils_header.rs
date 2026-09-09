/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Support utilities for cs_dsp testing.
 *
 * Copyright (C) 2024 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

// C dependencies: <linux/regmap.h> and <linux/firmware/cirrus/wmfw.h>

pub struct kunit;
pub struct cs_dsp;
pub struct cs_dsp_region;
pub struct firmware;
pub struct cs_dsp_test_local;
pub struct cs_dsp_mock_wmfw_builder;
pub struct cs_dsp_mock_bin_builder;

/**
 * struct cs_dsp_test - base class for test utilities
 *
 * @test: Pointer to struct kunit instance.
 * @dsp: Pointer to struct cs_dsp instance.
 * @local: Private data for each test suite.
 */
#[repr(C)]
pub struct cs_dsp_test {
    pub test: *mut kunit,
    pub dsp: *mut cs_dsp,
    pub local: *mut cs_dsp_test_local,
    /* private: Following members are private */
    pub saw_bus_write: bool,
}

/**
 * struct cs_dsp_mock_alg_def - Info for creating a mock algorithm entry.
 */
#[repr(C)]
pub struct cs_dsp_mock_alg_def {
    pub id: libc::c_uint,
    pub ver: libc::c_uint,
    pub xm_base_words: libc::c_uint,
    pub xm_size_words: libc::c_uint,
    pub ym_base_words: libc::c_uint,
    pub ym_size_words: libc::c_uint,
    pub zm_base_words: libc::c_uint,
    pub zm_size_words: libc::c_uint,
}

#[repr(C)]
pub struct cs_dsp_mock_coeff_def {
    pub shortname: *const libc::c_char,
    pub fullname: *const libc::c_char,
    pub description: *const libc::c_char,
    pub type_: u16,
    pub flags: u16,
    pub mem_type: u16,
    pub offset_dsp_words: libc::c_uint,
    pub length_bytes: libc::c_uint,
}

/**
 * struct cs_dsp_mock_xm_header - XM header builder
 */
#[repr(C)]
pub struct cs_dsp_mock_xm_header {
    pub test_priv: *mut cs_dsp_test,
    pub blob_data: *mut libc::c_void,
    pub blob_size_bytes: libc::size_t,
}

extern "C" {
    pub static cs_dsp_mock_adsp2_32bit_sysbase: libc::c_uint;
    pub static cs_dsp_mock_adsp2_16bit_sysbase: libc::c_uint;
    pub static cs_dsp_mock_halo_core_base: libc::c_uint;
    pub static cs_dsp_mock_halo_sysinfo_base: libc::c_uint;

    pub static cs_dsp_mock_halo_dsp1_regions: [cs_dsp_region; 0];
    pub static cs_dsp_mock_halo_dsp1_region_sizes: [libc::c_uint; 0];
    pub static cs_dsp_mock_adsp2_32bit_dsp1_regions: [cs_dsp_region; 0];
    pub static cs_dsp_mock_adsp2_32bit_dsp1_region_sizes: [libc::c_uint; 0];
    pub static cs_dsp_mock_adsp2_16bit_dsp1_regions: [cs_dsp_region; 0];
    pub static cs_dsp_mock_adsp2_16bit_dsp1_region_sizes: [libc::c_uint; 0];

    pub fn cs_dsp_mock_count_regions(region_sizes: *const libc::c_uint) -> libc::c_int;
    pub fn cs_dsp_mock_size_of_region(dsp: *const cs_dsp, mem_type: libc::c_int) -> libc::c_uint;
    pub fn cs_dsp_mock_base_addr_for_mem(priv_: *mut cs_dsp_test, mem_type: libc::c_int) -> libc::c_uint;
    pub fn cs_dsp_mock_reg_addr_inc_per_unpacked_word(priv_: *mut cs_dsp_test) -> libc::c_uint;
    pub fn cs_dsp_mock_reg_block_length_bytes(priv_: *mut cs_dsp_test, mem_type: libc::c_int) -> libc::c_uint;
    pub fn cs_dsp_mock_reg_block_length_registers(priv_: *mut cs_dsp_test, mem_type: libc::c_int) -> libc::c_uint;
    pub fn cs_dsp_mock_reg_block_length_dsp_words(priv_: *mut cs_dsp_test, mem_type: libc::c_int) -> libc::c_uint;
    pub fn cs_dsp_mock_has_zm(priv_: *mut cs_dsp_test) -> bool;
    pub fn cs_dsp_mock_packed_to_unpacked_mem_type(packed_mem_type: libc::c_int) -> libc::c_int;
    pub fn cs_dsp_mock_num_dsp_words_to_num_packed_regs(num_dsp_words: libc::c_uint) -> libc::c_uint;
    pub fn cs_dsp_mock_xm_header_get_alg_base_in_words(priv_: *mut cs_dsp_test, alg_id: libc::c_uint, mem_type: libc::c_int) -> libc::c_uint;
    pub fn cs_dsp_mock_xm_header_get_fw_version(header: *mut cs_dsp_mock_xm_header) -> libc::c_uint;
    pub fn cs_dsp_mock_xm_header_drop_from_regmap_cache(priv_: *mut cs_dsp_test);
    pub fn cs_dsp_mock_xm_header_write_to_regmap(header: *mut cs_dsp_mock_xm_header) -> libc::c_int;
    pub fn cs_dsp_create_mock_xm_header(priv_: *mut cs_dsp_test, algs: *const cs_dsp_mock_alg_def, num_algs: libc::size_t) -> *mut cs_dsp_mock_xm_header;

    pub fn cs_dsp_mock_regmap_init(priv_: *mut cs_dsp_test) -> libc::c_int;
    pub fn cs_dsp_mock_regmap_drop_range(priv_: *mut cs_dsp_test, first_reg: libc::c_uint, last_reg: libc::c_uint);
    pub fn cs_dsp_mock_regmap_drop_regs(priv_: *mut cs_dsp_test, first_reg: libc::c_uint, num_regs: libc::size_t);
    pub fn cs_dsp_mock_regmap_drop_bytes(priv_: *mut cs_dsp_test, first_reg: libc::c_uint, num_bytes: libc::size_t);
    pub fn cs_dsp_mock_regmap_drop_system_regs(priv_: *mut cs_dsp_test);
    pub fn cs_dsp_mock_regmap_is_dirty(priv_: *mut cs_dsp_test, drop_system_regs: bool) -> bool;

    pub fn cs_dsp_mock_bin_init(priv_: *mut cs_dsp_test, format_version: libc::c_int, fw_version: libc::c_uint) -> *mut cs_dsp_mock_bin_builder;
    pub fn cs_dsp_mock_bin_add_raw_block(builder: *mut cs_dsp_mock_bin_builder, alg_id: libc::c_uint, alg_ver: libc::c_uint, type_: libc::c_int, offset: u16, offset32: u32, payload_data: *const libc::c_void, payload_len_bytes: libc::size_t);
    pub fn cs_dsp_mock_bin_add_info(builder: *mut cs_dsp_mock_bin_builder, info: *const libc::c_char);
    pub fn cs_dsp_mock_bin_add_name(builder: *mut cs_dsp_mock_bin_builder, name: *const libc::c_char);
    pub fn cs_dsp_mock_bin_add_patch(builder: *mut cs_dsp_mock_bin_builder, alg_id: libc::c_uint, alg_ver: libc::c_uint, mem_region: libc::c_int, reg_addr_offset: libc::c_uint, payload_data: *const libc::c_void, payload_len_bytes: libc::size_t);
    pub fn cs_dsp_mock_bin_add_patch_off32(builder: *mut cs_dsp_mock_bin_builder, alg_id: libc::c_uint, alg_ver: libc::c_uint, mem_region: libc::c_int, reg_addr_offset: libc::c_uint, payload_data: *const libc::c_void, payload_len_bytes: libc::size_t);
    pub fn cs_dsp_mock_bin_get_firmware(builder: *mut cs_dsp_mock_bin_builder) -> *mut firmware;

    pub fn cs_dsp_mock_wmfw_init(priv_: *mut cs_dsp_test, format_version: libc::c_int) -> *mut cs_dsp_mock_wmfw_builder;
    pub fn cs_dsp_mock_wmfw_add_raw_block(builder: *mut cs_dsp_mock_wmfw_builder, mem_region: libc::c_int, mem_offset_dsp_words: libc::c_uint, payload_data: *const libc::c_void, payload_len_bytes: libc::size_t);
    pub fn cs_dsp_mock_wmfw_add_info(builder: *mut cs_dsp_mock_wmfw_builder, info: *const libc::c_char);
    pub fn cs_dsp_mock_wmfw_add_data_block(builder: *mut cs_dsp_mock_wmfw_builder, mem_region: libc::c_int, mem_offset_dsp_words: libc::c_uint, payload_data: *const libc::c_void, payload_len_bytes: libc::size_t);
    pub fn cs_dsp_mock_wmfw_start_alg_info_block(builder: *mut cs_dsp_mock_wmfw_builder, alg_id: libc::c_uint, name: *const libc::c_char, description: *const libc::c_char);
    pub fn cs_dsp_mock_wmfw_add_coeff_desc(builder: *mut cs_dsp_mock_wmfw_builder, def_: *const cs_dsp_mock_coeff_def);
    pub fn cs_dsp_mock_wmfw_end_alg_info_block(builder: *mut cs_dsp_mock_wmfw_builder);
    pub fn cs_dsp_mock_wmfw_get_firmware(builder: *mut cs_dsp_mock_wmfw_builder) -> *mut firmware;
    pub fn cs_dsp_mock_wmfw_format_version(builder: *mut cs_dsp_mock_wmfw_builder) -> libc::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
