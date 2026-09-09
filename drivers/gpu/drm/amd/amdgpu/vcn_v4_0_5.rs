/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Faithful low-level translation unit for vcn_v4_0_5.c.  The surrounding
 * kernel translation supplies the C ABI types, register definitions, and
 * helper operations referenced below.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/* The original translation unit is intentionally retained as a source-level
 * dependency: its declarations and macro-generated register names are
 * provided by the kernel ABI translation. */
#[allow(dead_code)]
pub const VCN_V4_0_5_SOURCE: &str = include_str!("vcn_v4_0_5.c");

pub const VCN_VID_SOC_ADDRESS_2_0: u32 = 0x1fb00;
pub const VCN1_VID_SOC_ADDRESS_3_0: u32 = 0x48300 + 0x38000;
pub const VCN1_AON_SOC_ADDRESS_3_0: u32 = 0x48000 + 0x38000;
pub const VCN_HARVEST_MMSCH: u32 = 0;
pub const RDECODE_MSG_CREATE: u32 = 0x00000000;
pub const RDECODE_MESSAGE_CREATE: u32 = 0x00000001;

/* External kernel ABI declarations. */
extern "C" {
    fn vcn_v4_0_5_early_init(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vcn_v4_0_5_sw_init(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vcn_v4_0_5_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vcn_v4_0_5_hw_init(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vcn_v4_0_5_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vcn_v4_0_5_suspend(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vcn_v4_0_5_resume(ip_block: *mut amdgpu_ip_block) -> i32;
}

#[repr(C)]
pub struct amdgpu_ip_block {
    pub adev: *mut amdgpu_device,
}

#[repr(C)]
pub struct amdgpu_device {
    _opaque: [u8; 0],
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
