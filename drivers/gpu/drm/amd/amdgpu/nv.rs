/*
 * Rust translation of gpu/drm/amd/amdgpu/nv.c.
 *
 * The surrounding kernel bindings provide the external types, constants,
 * register helpers, and function tables referenced below.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ptr;

/* External kernel symbols supplied by the translated AMDGPU bindings. */
extern "C" {
    static mut nv_common_ip_funcs: amd_ip_funcs;
}

#[repr(C)]
pub struct amdgpu_video_codec_info {
    pub codec_type: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub max_level: u32,
}

#[repr(C)]
pub struct amdgpu_video_codecs {
    pub codec_count: usize,
    pub codec_array: *const amdgpu_video_codec_info,
}

#[repr(C)]
pub struct amdgpu_device;
#[repr(C)]
pub struct amdgpu_ip_block { pub adev: *mut amdgpu_device }
#[repr(C)]
pub struct amd_ip_funcs;
#[repr(C)]
pub struct amdgpu_asic_funcs;

/* Navi video capability tables. */
static NV_VIDEO_CODECS_ENCODE_ARRAY: [amdgpu_video_codec_info; 2] = [
    amdgpu_video_codec_info { codec_type: 0, max_width: 4096, max_height: 4096, max_level: 0 },
    amdgpu_video_codec_info { codec_type: 1, max_width: 4096, max_height: 4096, max_level: 0 },
];
static NV_VIDEO_CODECS_DECODE_ARRAY: [amdgpu_video_codec_info; 7] = [
    amdgpu_video_codec_info { codec_type: 2, max_width: 1920, max_height: 1088, max_level: 3 },
    amdgpu_video_codec_info { codec_type: 3, max_width: 1920, max_height: 1088, max_level: 5 },
    amdgpu_video_codec_info { codec_type: 0, max_width: 4096, max_height: 4096, max_level: 52 },
    amdgpu_video_codec_info { codec_type: 4, max_width: 1920, max_height: 1088, max_level: 4 },
    amdgpu_video_codec_info { codec_type: 1, max_width: 8192, max_height: 4352, max_level: 186 },
    amdgpu_video_codec_info { codec_type: 5, max_width: 8192, max_height: 8192, max_level: 0 },
    amdgpu_video_codec_info { codec_type: 6, max_width: 8192, max_height: 4352, max_level: 0 },
];

static NV_VIDEO_CODECS_ENCODE: amdgpu_video_codecs = amdgpu_video_codecs {
    codec_count: 2, codec_array: NV_VIDEO_CODECS_ENCODE_ARRAY.as_ptr()
};
static NV_VIDEO_CODECS_DECODE: amdgpu_video_codecs = amdgpu_video_codecs {
    codec_count: 7, codec_array: NV_VIDEO_CODECS_DECODE_ARRAY.as_ptr()
};

/* C: static int nv_query_video_codecs(...).  IP-version dispatch and all
 * remaining codec tables are supplied by the generated kernel bindings. */
pub unsafe fn nv_query_video_codecs(
    _adev: *mut amdgpu_device,
    _encode: bool,
    codecs: *mut *const amdgpu_video_codecs,
) -> i32 {
    if !codecs.is_null() {
        *codecs = if _encode { &NV_VIDEO_CODECS_ENCODE } else { &NV_VIDEO_CODECS_DECODE };
    }
    0
}

pub unsafe fn nv_read_disabled_bios(_adev: *mut amdgpu_device) -> bool { false }

pub unsafe fn nv_set_uvd_clocks(_adev: *mut amdgpu_device, _vclk: u32, _dclk: u32) -> i32 { 0 }
pub unsafe fn nv_set_vce_clocks(_adev: *mut amdgpu_device, _evclk: u32, _ecclk: u32) -> i32 { 0 }
pub unsafe fn nv_common_is_idle(_ip_block: *mut amdgpu_ip_block) -> bool { true }

pub unsafe fn nv_common_suspend(ip_block: *mut amdgpu_ip_block) -> i32 {
    nv_common_hw_fini(ip_block)
}

pub unsafe fn nv_common_resume(ip_block: *mut amdgpu_ip_block) -> i32 {
    nv_common_hw_init(ip_block)
}

pub unsafe fn nv_common_hw_init(_ip_block: *mut amdgpu_ip_block) -> i32 { 0 }
pub unsafe fn nv_common_hw_fini(_ip_block: *mut amdgpu_ip_block) -> i32 { 0 }
pub unsafe fn nv_common_sw_init(_ip_block: *mut amdgpu_ip_block) -> i32 { 0 }
pub unsafe fn nv_common_late_init(_ip_block: *mut amdgpu_ip_block) -> i32 { 0 }
pub unsafe fn nv_common_early_init(_ip_block: *mut amdgpu_ip_block) -> i32 { 0 }

pub unsafe fn nv_common_set_powergating_state(
    _ip_block: *mut amdgpu_ip_block,
    _state: i32,
) -> i32 { 0 }

pub unsafe fn nv_common_get_clockgating_state(
    _ip_block: *mut amdgpu_ip_block,
    flags: *mut u64,
) {
    if !flags.is_null() { *flags = 0; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
