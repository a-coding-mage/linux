// SPDX-License-Identifier: GPL-2.0-only
//
// Mock DSP memory maps for cs_dsp KUnit tests.
//
// Copyright (C) 2024 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

// C dependencies supplied by the surrounding kernel sources.

#[no_mangle]
pub static cs_dsp_mock_halo_dsp1_regions: [cs_dsp_region; 5] = [
    cs_dsp_region { type_: WMFW_HALO_PM_PACKED, base: 0x3800000 },
    cs_dsp_region { type_: WMFW_HALO_XM_PACKED, base: 0x2000000 },
    cs_dsp_region { type_: WMFW_HALO_YM_PACKED, base: 0x2C00000 },
    cs_dsp_region { type_: WMFW_ADSP2_XM, base: 0x2800000 },
    cs_dsp_region { type_: WMFW_ADSP2_YM, base: 0x3400000 },
];
pub static cs_dsp_mock_halo_dsp1_region_sizes: [u32; 6] = [0x5000, 0x8fff4, 0x8fff4, 0xbfff8, 0xbfff8, 0];

pub static cs_dsp_mock_adsp2_32bit_dsp1_regions: [cs_dsp_region; 4] = [
    cs_dsp_region { type_: WMFW_ADSP2_PM, base: 0x080000 },
    cs_dsp_region { type_: WMFW_ADSP2_XM, base: 0x0a0000 },
    cs_dsp_region { type_: WMFW_ADSP2_YM, base: 0x0c0000 },
    cs_dsp_region { type_: WMFW_ADSP2_ZM, base: 0x0e0000 },
];
pub static cs_dsp_mock_adsp2_32bit_dsp1_region_sizes: [u32; 5] = [0x9000, 0xa000, 0x2000, 0x2000, 0];

pub static cs_dsp_mock_adsp2_16bit_dsp1_regions: [cs_dsp_region; 4] = [
    cs_dsp_region { type_: WMFW_ADSP2_PM, base: 0x100000 },
    cs_dsp_region { type_: WMFW_ADSP2_ZM, base: 0x180000 },
    cs_dsp_region { type_: WMFW_ADSP2_XM, base: 0x190000 },
    cs_dsp_region { type_: WMFW_ADSP2_YM, base: 0x1a8000 },
];
pub static cs_dsp_mock_adsp2_16bit_dsp1_region_sizes: [u32; 5] = [0x6000, 0x800, 0x800, 0x800, 0];

pub unsafe fn cs_dsp_mock_count_regions(region_sizes: *const u32) -> i32 {
    let mut i = 0;
    while *region_sizes.add(i as usize) != 0 { i += 1; }
    i
}

pub unsafe fn cs_dsp_mock_size_of_region(dsp: *const cs_dsp, mem_type: i32) -> u32 {
    let sizes: *const u32;
    if (*dsp).mem == cs_dsp_mock_halo_dsp1_regions.as_ptr() { sizes = cs_dsp_mock_halo_dsp1_region_sizes.as_ptr(); }
    else if (*dsp).mem == cs_dsp_mock_adsp2_32bit_dsp1_regions.as_ptr() { sizes = cs_dsp_mock_adsp2_32bit_dsp1_region_sizes.as_ptr(); }
    else if (*dsp).mem == cs_dsp_mock_adsp2_16bit_dsp1_regions.as_ptr() { sizes = cs_dsp_mock_adsp2_16bit_dsp1_region_sizes.as_ptr(); }
    else { return 0; }
    for i in 0..(*dsp).num_mems as usize { if (*dsp).mem.add(i).read().type_ == mem_type { return *sizes.add(i); } }
    0
}

pub unsafe fn cs_dsp_mock_base_addr_for_mem(priv_: *mut cs_dsp_test, mem_type: i32) -> u32 {
    for i in 0..(*(*priv_).dsp).num_mems as usize { if (*(*(*priv_).dsp).mem.add(i)).type_ == mem_type { return (*(*priv_).dsp).mem.add(i).read().base; } }
    KUNIT_FAIL((*priv_).test, b"Unexpected region %d\0".as_ptr() as *const _, mem_type); 0
}

pub unsafe fn cs_dsp_mock_reg_addr_inc_per_unpacked_word(priv_: *mut cs_dsp_test) -> u32 {
    match (*(*priv_).dsp).type_ { WMFW_ADSP2 => 2, WMFW_HALO => 4, _ => { KUNIT_FAIL((*priv_).test, b"Unexpected DSP type\n\0".as_ptr() as *const _); u32::MAX } }
}

pub unsafe fn cs_dsp_mock_reg_block_length_bytes(priv_: *mut cs_dsp_test, mem_type: i32) -> u32 {
    let typ = (*(*priv_).dsp).type_;
    match typ {
        WMFW_ADSP2 => match mem_type { WMFW_ADSP2_PM => 3 * regmap_get_val_bytes((*(*priv_).dsp).regmap), WMFW_ADSP2_XM | WMFW_ADSP2_YM | WMFW_ADSP2_ZM => 4, _ => 0 },
        WMFW_HALO => match mem_type { WMFW_ADSP2_XM | WMFW_ADSP2_YM => 4, WMFW_HALO_PM_PACKED => 20, WMFW_HALO_XM_PACKED | WMFW_HALO_YM_PACKED => 12, _ => 0 },
        _ => { KUNIT_FAIL((*priv_).test, b"Unexpected DSP type\n\0".as_ptr() as *const _); 0 }
    }
}

pub unsafe fn cs_dsp_mock_reg_block_length_registers(p: *mut cs_dsp_test, m: i32) -> u32 { cs_dsp_mock_reg_block_length_bytes(p, m) / regmap_get_val_bytes((*(*p).dsp).regmap) }
pub unsafe fn cs_dsp_mock_reg_block_length_dsp_words(p: *mut cs_dsp_test, m: i32) -> u32 {
    match ((*(*p).dsp).type_, m) { (WMFW_ADSP2, WMFW_ADSP2_PM) => regmap_get_val_bytes((*(*p).dsp).regmap) / 2, (WMFW_ADSP2, WMFW_ADSP2_XM | WMFW_ADSP2_YM | WMFW_ADSP2_ZM) | (WMFW_HALO, WMFW_ADSP2_XM | WMFW_ADSP2_YM) => 1, (WMFW_HALO, WMFW_HALO_PM_PACKED | WMFW_HALO_XM_PACKED | WMFW_HALO_YM_PACKED) => 4, _ => 0 }
}
pub unsafe fn cs_dsp_mock_has_zm(p: *mut cs_dsp_test) -> bool { (*(*p).dsp).type_ == WMFW_ADSP2 }
pub fn cs_dsp_mock_packed_to_unpacked_mem_type(m: i32) -> i32 { match m { WMFW_HALO_XM_PACKED => WMFW_ADSP2_XM, WMFW_HALO_YM_PACKED => WMFW_ADSP2_YM, _ => -1 } }
pub fn cs_dsp_mock_num_dsp_words_to_num_packed_regs(n: u32) -> u32 { n.wrapping_mul(3) / 4 }

// The remaining declarations require the kernel's externally supplied C layout types and helpers.
// They are kept as extern declarations to preserve the source-level interface.
extern "C" {
    pub fn cs_dsp_mock_xm_header_get_alg_base_in_words(priv_: *mut cs_dsp_test, alg_id: u32, mem_type: i32) -> u32;
    pub fn cs_dsp_mock_xm_header_get_fw_version(header: *mut cs_dsp_mock_xm_header) -> u32;
    pub fn cs_dsp_mock_xm_header_drop_from_regmap_cache(priv_: *mut cs_dsp_test);
    pub fn cs_dsp_mock_xm_header_write_to_regmap(header: *mut cs_dsp_mock_xm_header) -> i32;
    pub fn cs_dsp_create_mock_xm_header(priv_: *mut cs_dsp_test, algs: *const cs_dsp_mock_alg_def, num_algs: usize) -> *mut cs_dsp_mock_xm_header;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
