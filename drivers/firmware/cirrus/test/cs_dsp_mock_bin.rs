// SPDX-License-Identifier: GPL-2.0-only
//
// bin file builder for cs_dsp KUnit tests.
//
// Copyright (C) 2024 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

// Kernel and project headers from the C translation unit provide the external
// types, constants, allocation routines, and byte-order helpers used below.

const CS_DSP_MOCK_BIN_BUF_SIZE: usize = 32768;

#[repr(C)]
pub struct cs_dsp_mock_bin_builder {
    pub test_priv: *mut cs_dsp_test,
    pub buf: *mut core::ffi::c_void,
    pub write_p: *mut core::ffi::c_void,
    pub bytes_used: usize,
}

extern "C" {
    pub fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut core::ffi::c_void;
    pub fn kunit_kfree(test: *mut kunit, ptr: *mut core::ffi::c_void);
    pub fn vmalloc(size: usize) -> *mut core::ffi::c_void;
    pub fn vfree(ptr: *mut core::ffi::c_void);
    pub fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    pub fn strlen(s: *const i8) -> usize;
}

#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct cs_dsp_test { pub test: *mut kunit, pub dsp: *mut cs_dsp }
#[repr(C)] pub struct cs_dsp { pub r#type: u32, pub rev: u32 }
#[repr(C)] pub struct wmfw_coeff_item {
    pub offset: u16, pub offset32: u32, pub r#type: u16, pub id: u32,
    pub ver: u32, pub len: u32,
    pub data: [u8; 0],
}
#[repr(C)] pub struct wmfw_coeff_hdr {
    pub magic: [u8; 4], pub len: u32, pub ver: u32, pub core_ver: u32,
    pub data: [u8; 0],
}
#[repr(C)] pub struct firmware { pub data: *const u8, pub size: usize }

const GFP_KERNEL: u32 = 0;
const WMFW_INFO_TEXT: i32 = 0;
const WMFW_NAME_TEXT: i32 = 1;

pub unsafe fn cs_dsp_mock_bin_get_firmware(builder: *mut cs_dsp_mock_bin_builder) -> *mut firmware {
    let fw = kunit_kzalloc((*(*builder).test_priv).test, core::mem::size_of::<firmware>(), GFP_KERNEL) as *mut firmware;
    assert!(!fw.is_null());
    (*fw).data = (*builder).buf as *const u8;
    (*fw).size = (*builder).bytes_used;
    fw
}

pub unsafe fn cs_dsp_mock_bin_add_raw_block(builder: *mut cs_dsp_mock_bin_builder,
    alg_id: u32, alg_ver: u32, r#type: i32, offset: u16, offset32: u32,
    payload_data: *const core::ffi::c_void, payload_len_bytes: usize) {
    let bytes_needed = core::mem::size_of::<wmfw_coeff_item>() + payload_len_bytes;
    assert!(((*builder).write_p as usize).wrapping_add(bytes_needed) <
            ((*builder).buf as usize).wrapping_add(CS_DSP_MOCK_BIN_BUF_SIZE));
    let item = (*builder).write_p as *mut wmfw_coeff_item;
    (*item).offset = offset.to_le();
    (*item).offset32 = offset32.to_le();
    (*item).r#type = (r#type as u16).to_le();
    (*item).id = alg_id.to_le();
    (*item).ver = (alg_ver << 8).to_le();
    (*item).len = (payload_len_bytes as u32).to_le();
    if payload_len_bytes != 0 { memcpy((*item).data.as_mut_ptr() as *mut _, payload_data, payload_len_bytes); }
    (*builder).write_p = ((*builder).write_p as *mut u8).add(bytes_needed) as *mut _;
    (*builder).bytes_used += bytes_needed;
}

unsafe fn cs_dsp_mock_bin_add_name_or_info(builder: *mut cs_dsp_mock_bin_builder, info: *const i8, r#type: i32) {
    let mut info_len = strlen(info);
    let mut tmp: *mut i8 = core::ptr::null_mut();
    let mut source = info;
    if info_len % 4 != 0 {
        let copy_len = info_len;
        info_len = (info_len + 3) & !3;
        tmp = kunit_kzalloc((*(*builder).test_priv).test, info_len, GFP_KERNEL) as *mut i8;
        assert!(!tmp.is_null());
        memcpy(tmp as *mut _, info as *const _, copy_len);
        source = tmp;
    }
    cs_dsp_mock_bin_add_raw_block(builder, 0, 0, r#type, 0, 0, source as *const _, info_len);
    kunit_kfree((*(*builder).test_priv).test, tmp as *mut _);
}

pub unsafe fn cs_dsp_mock_bin_add_info(builder: *mut cs_dsp_mock_bin_builder, info: *const i8) { cs_dsp_mock_bin_add_name_or_info(builder, info, WMFW_INFO_TEXT); }
pub unsafe fn cs_dsp_mock_bin_add_name(builder: *mut cs_dsp_mock_bin_builder, name: *const i8) { cs_dsp_mock_bin_add_name_or_info(builder, name, WMFW_NAME_TEXT); }

pub unsafe fn cs_dsp_mock_bin_add_patch(builder: *mut cs_dsp_mock_bin_builder, alg_id: u32, alg_ver: u32, mem_region: i32, reg_addr_offset: u32, payload_data: *const core::ffi::c_void, payload_len_bytes: usize) {
    assert_eq!(payload_len_bytes % 4, 0);
    cs_dsp_mock_bin_add_raw_block(builder, alg_id, alg_ver, mem_region, reg_addr_offset as u16, 0, payload_data, payload_len_bytes);
}

pub unsafe fn cs_dsp_mock_bin_add_patch_off32(builder: *mut cs_dsp_mock_bin_builder, alg_id: u32, alg_ver: u32, mut mem_region: i32, reg_addr_offset: u32, payload_data: *const core::ffi::c_void, payload_len_bytes: usize) {
    assert_eq!(payload_len_bytes % 4, 0);
    mem_region |= 0xf400;
    cs_dsp_mock_bin_add_raw_block(builder, alg_id, alg_ver, mem_region, 0, reg_addr_offset, payload_data, payload_len_bytes);
}

pub unsafe fn cs_dsp_mock_bin_init(priv_: *mut cs_dsp_test, format_version: i32, fw_version: u32) -> *mut cs_dsp_mock_bin_builder {
    assert!(format_version <= 0xff);
    assert!(fw_version <= 0xffffff);
    let builder = kunit_kzalloc((*priv_).test, core::mem::size_of::<cs_dsp_mock_bin_builder>(), GFP_KERNEL) as *mut cs_dsp_mock_bin_builder;
    assert!(!builder.is_null());
    (*builder).test_priv = priv_;
    (*builder).buf = vmalloc(CS_DSP_MOCK_BIN_BUF_SIZE);
    assert!(!(*builder).buf.is_null());
    let hdr = (*builder).buf as *mut wmfw_coeff_hdr;
    (*hdr).magic = *b"WMDR";
    (*hdr).len = (core::mem::size_of::<wmfw_coeff_hdr>() as u32).to_le();
    (*hdr).ver = (fw_version | ((format_version as u32) << 24)).to_le();
    (*hdr).core_ver = (((*(*priv_).dsp).r#type << 24) | (*(*priv_).dsp).rev).to_le();
    (*builder).write_p = (*hdr).data.as_mut_ptr() as *mut _;
    (*builder).bytes_used = core::mem::size_of::<wmfw_coeff_hdr>();
    builder
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
