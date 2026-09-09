/*
 * Copyright 2025 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependencies supplied by the surrounding driver translation.
const XCC_REG_RANGE_0_LOW: u32 = 0x1260;
const XCC_REG_RANGE_0_HIGH: u32 = 0x3C00;
const XCC_REG_RANGE_1_LOW: u32 = 0xA000;
const XCC_REG_RANGE_1_HIGH: u32 = 0x10000;
const MID1_REG_RANGE_0_LOW: u32 = 0x40000;
const MID1_REG_RANGE_0_HIGH: u32 = 0x80000;

static mut vcn_5_0_2_video_codecs_encode_vcn0: amdgpu_video_codecs = amdgpu_video_codecs { codec_count: 0, codec_array: core::ptr::null() };
static vcn_5_0_2_video_codecs_decode_array_vcn0: [amdgpu_video_codec_info; 5] = [
    codec_info_build(AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4_AVC, 4096, 4096, 52),
    codec_info_build(AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_HEVC, 8192, 4352, 186),
    codec_info_build(AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_JPEG, 16384, 16384, 0),
    codec_info_build(AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VP9, 8192, 4352, 0),
    codec_info_build(AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_AV1, 8192, 4352, 0),
];
static vcn_5_0_2_video_codecs_decode_vcn0: amdgpu_video_codecs = amdgpu_video_codecs {
    codec_count: vcn_5_0_2_video_codecs_decode_array_vcn0.len(),
    codec_array: vcn_5_0_2_video_codecs_decode_array_vcn0.as_ptr(),
};

unsafe fn soc_v1_0_query_video_codecs(adev: *mut amdgpu_device, encode: bool, codecs: *mut *const amdgpu_video_codecs) -> i32 {
    match amdgpu_ip_version(adev, UVD_HWIP, 0) {
        IP_VERSION(5, 0, 2) => { *codecs = if encode { &raw const vcn_5_0_2_video_codecs_encode_vcn0 } else { &raw const vcn_5_0_2_video_codecs_decode_vcn0 }; 0 }
        _ => -EINVAL,
    }
}

unsafe fn soc_v1_0_doorbell_index_init(adev: *mut amdgpu_device) {
    (*adev).doorbell_index.kiq = AMDGPU_SOC_V1_0_DOORBELL_KIQ_START;
    (*adev).doorbell_index.mec_ring0 = AMDGPU_SOC_V1_0_DOORBELL_MEC_RING_START;
    (*adev).doorbell_index.mes_ring0 = AMDGPU_SOC_V1_0_DOORBELL_MES_RING0;
    (*adev).doorbell_index.mes_ring1 = AMDGPU_SOC_V1_0_DOORBELL_MES_RING1;
    (*adev).doorbell_index.userqueue_start = AMDGPU_SOC_V1_0_DOORBELL_USERQUEUE_START;
    (*adev).doorbell_index.userqueue_end = AMDGPU_SOC_V1_0_DOORBELL_USERQUEUE_END;
    (*adev).doorbell_index.xcc_doorbell_range = AMDGPU_SOC_V1_0_DOORBELL_XCC_RANGE;
    (*adev).doorbell_index.sdma_doorbell_range = 14;
    for i in 0..(*adev).sdma.num_instances { (*adev).doorbell_index.sdma_engine[i as usize] = AMDGPU_SOC_V1_0_DOORBELL_sDMA_ENGINE_START + i * ((*adev).doorbell_index.sdma_doorbell_range >> 1); }
    (*adev).doorbell_index.ih = AMDGPU_SOC_V1_0_DOORBELL_IH;
    (*adev).doorbell_index.vcn.vcn_ring0_1 = AMDGPU_SOC_V1_0_DOORBELL_VCN_START;
    (*adev).doorbell_index.first_non_cp = AMDGPU_SOC_V1_0_DOORBELL_FIRST_NON_CP;
    (*adev).doorbell_index.last_non_cp = AMDGPU_SOC_V1_0_DOORBELL_LAST_NON_CP;
    (*adev).doorbell_index.max_assignment = AMDGPU_SOC_V1_0_DOORBELL_MAX_ASSIGNMENT << 1;
}

pub unsafe fn soc_v1_0_encode_ext_smn_addressing(ext_id: i32) -> u64 {
    if ext_id == 0 { return 0; }
    let die_id = ext_id & 3; let socket_id = (ext_id >> 6) & 0xff;
    if socket_id == 0 { ((die_id as u64) << 34) | (1u64 << 32) }
    else if die_id != 0 { ((socket_id as u64) << 40) | ((die_id as u64) << 34) | (3u64 << 32) }
    else { ((socket_id as u64) << 40) | (1u64 << 33) }
}

unsafe fn soc_v1_0_get_config_memsize(adev: *mut amdgpu_device) -> u32 { ((*(*adev).nbio.funcs).get_memsize)(adev) }
unsafe fn soc_v1_0_get_xclk(adev: *mut amdgpu_device) -> u32 { (*adev).clock.spll.reference_freq }

pub unsafe fn soc_v1_0_grbm_select(adev: *mut amdgpu_device, me: u32, pipe: u32, queue: u32, vmid: u32, xcc_id: i32) {
    let mut v = 0; v = REG_SET_FIELD(v, GRBM_GFX_CNTL, PIPEID, pipe); v = REG_SET_FIELD(v, GRBM_GFX_CNTL, MEID, me); v = REG_SET_FIELD(v, GRBM_GFX_CNTL, VMID, vmid); v = REG_SET_FIELD(v, GRBM_GFX_CNTL, QUEUEID, queue); WREG32_SOC15_RLC_SHADOW(GC, xcc_id, regGRBM_GFX_CNTL, v);
}

pub fn soc_v1_0_normalize_xcc_reg_range(reg: u32) -> bool { (reg >= XCC_REG_RANGE_0_LOW && reg < XCC_REG_RANGE_0_HIGH) || (reg >= XCC_REG_RANGE_1_LOW && reg < XCC_REG_RANGE_1_HIGH) }
pub fn soc_v1_0_normalize_xcc_reg_offset(reg: u32) -> u32 { let n = reg & 0xffff; if soc_v1_0_normalize_xcc_reg_range(n) { n } else { reg } }
pub fn soc_v1_0_mid1_reg_range(reg: u32) -> bool { !soc_v1_0_normalize_xcc_reg_range(soc_v1_0_normalize_xcc_reg_offset(reg)) && reg >= MID1_REG_RANGE_0_LOW && reg < MID1_REG_RANGE_0_HIGH }
pub fn soc_v1_0_normalize_reg_offset(reg: u32) -> u32 { let n = soc_v1_0_normalize_xcc_reg_offset(reg); if soc_v1_0_normalize_xcc_reg_range(n) { soc_v1_0_normalize_xcc_reg_offset(reg) } else if soc_v1_0_mid1_reg_range(reg) { reg & 0x3ffff } else { reg } }

// The remaining callbacks retain the C ABI-facing objects and delegate to the
// corresponding driver-provided operations.
unsafe fn soc_v1_0_get_register_value(adev: *mut amdgpu_device, indexed: bool, se: u32, sh: u32, off: u32) -> u32 {
    if indexed { amdgpu_read_indexed_register(adev, se, sh, off) }
    else if off == SOC15_REG_OFFSET(GC, 0, regGB_ADDR_CONFIG_1) && (*adev).gfx.config.gb_addr_config != 0 { (*adev).gfx.config.gb_addr_config } else { RREG32(off) }
}
unsafe fn soc_v1_0_need_reset_on_init(_adev: *mut amdgpu_device) -> bool { false }
unsafe fn soc_v1_0_asic_reset_method(adev: *mut amdgpu_device) -> amd_reset_method {
    if ((*adev).gmc.xgmi.supported && (*adev).gmc.xgmi.connected_to_cpu) || amdgpu_ip_version(adev, MP1_HWIP, 0) == IP_VERSION(15, 0, 8) { AMD_RESET_METHOD_MODE2 } else { amdgpu_reset_method }
}
unsafe fn soc_v1_0_asic_reset(adev: *mut amdgpu_device) -> i32 {
    match soc_v1_0_asic_reset_method(adev) { AMD_RESET_METHOD_MODE2 => amdgpu_dpm_mode2_reset(adev), _ => -EOPNOTSUPP }
}

unsafe fn soc_v1_0_common_early_init(ip: *mut amdgpu_ip_block) -> i32 {
    let adev = (*ip).adev; (*adev).rev_id = amdgpu_device_get_rev_id(adev); (*adev).external_rev_id = 0xff;
    match amdgpu_ip_version(adev, GC_HWIP, 0) { IP_VERSION(12, 1, 0) => { (*adev).cg_flags = AMD_CG_SUPPORT_GFX_CGCG | AMD_CG_SUPPORT_GFX_CGLS; (*adev).pg_flags = AMD_PG_SUPPORT_VCN_DPG; (*adev).external_rev_id = (*adev).rev_id + 0x50; 0 }, _ => -EINVAL }
}
unsafe fn soc_v1_0_common_late_init(ip: *mut amdgpu_ip_block) -> i32 { (*(*ip).adev).nbio.funcs.enable_doorbell_selfring_aperture((*ip).adev, true); 0 }
unsafe fn soc_v1_0_common_sw_init(_ip: *mut amdgpu_ip_block) -> i32 { 0 }
unsafe fn soc_v1_0_common_hw_init(ip: *mut amdgpu_ip_block) -> i32 { (*(*ip).adev).nbio.funcs.enable_doorbell_aperture((*ip).adev, true); 0 }
unsafe fn soc_v1_0_common_hw_fini(ip: *mut amdgpu_ip_block) -> i32 { let a=(*ip).adev; (*a).nbio.funcs.enable_doorbell_aperture(a,false); (*a).nbio.funcs.enable_doorbell_selfring_aperture(a,false); 0 }
unsafe fn soc_v1_0_common_suspend(ip: *mut amdgpu_ip_block) -> i32 { soc_v1_0_common_hw_fini(ip) }
unsafe fn soc_v1_0_common_resume(ip: *mut amdgpu_ip_block) -> i32 { soc_v1_0_common_hw_init(ip) }
unsafe fn soc_v1_0_common_is_idle(_ip: *mut amdgpu_ip_block) -> bool { true }
unsafe fn soc_v1_0_common_set_clockgating_state(_ip: *mut amdgpu_ip_block, _state: amd_clockgating_state) -> i32 { 0 }
unsafe fn soc_v1_0_common_set_powergating_state(_ip: *mut amdgpu_ip_block, _state: amd_powergating_state) -> i32 { 0 }
unsafe fn soc_v1_0_common_get_clockgating_state(_ip: *mut amdgpu_ip_block, _flags: *mut u64) {}

pub unsafe fn soc_v1_0_init_soc_config(adev: *mut amdgpu_device) -> i32 {
    let mut mask = (*adev).gfx.xcc_mask; (*adev).aid_mask = 0; let mut i=0; while mask != 0 { if mask & 0xf != 0 { (*adev).aid_mask |= 1 << i; } mask >>= 4; i+=1; }
    (*adev).sdma.num_inst_per_xcc=2; (*adev).sdma.num_instances=NUM_XCC((*adev).gfx.xcc_mask) * 2; (*adev).sdma.sdma_mask = GENMASK(1,0) * (*adev).gfx.xcc_mask;
    amdgpu_ip_map_init(adev); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
