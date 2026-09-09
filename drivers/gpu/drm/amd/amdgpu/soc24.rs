/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

static const soc24_common_ip_funcs: amd_ip_funcs;

static const vcn_5_0_0_video_codecs_encode_array_vcn0: [amdgpu_video_codec_info; 3] = [
    codec_info_build(AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4_AVC, 4096, 4096, 0),
    codec_info_build(AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_HEVC, 8192, 4352, 0),
    codec_info_build(AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_AV1, 8192, 4352, 0),
];

static const vcn_5_0_0_video_codecs_encode_vcn0: amdgpu_video_codecs = amdgpu_video_codecs {
    codec_count: vcn_5_0_0_video_codecs_encode_array_vcn0.len(),
    codec_array: vcn_5_0_0_video_codecs_encode_array_vcn0.as_ptr(),
};

static const vcn_5_0_0_video_codecs_decode_array_vcn0: [amdgpu_video_codec_info; 5] = [
    codec_info_build(AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4_AVC, 4096, 4096, 52),
    codec_info_build(AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_HEVC, 8192, 4352, 186),
    codec_info_build(AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_JPEG, 16384, 16384, 0),
    codec_info_build(AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VP9, 8192, 4352, 0),
    codec_info_build(AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_AV1, 8192, 4352, 0),
];

static const vcn_5_0_0_video_codecs_decode_vcn0: amdgpu_video_codecs = amdgpu_video_codecs {
    codec_count: vcn_5_0_0_video_codecs_decode_array_vcn0.len(),
    codec_array: vcn_5_0_0_video_codecs_decode_array_vcn0.as_ptr(),
};

unsafe fn soc24_query_video_codecs(adev: *mut amdgpu_device, encode: bool,
                                   codecs: *mut *const amdgpu_video_codecs) -> i32 {
    if (*adev).vcn.num_vcn_inst == hweight8((*adev).vcn.harvest_config) { return -EINVAL; }
    match amdgpu_ip_version(adev, UVD_HWIP, 0) {
        IP_VERSION(5, 0, 0) => {
            *codecs = if encode { &vcn_5_0_0_video_codecs_encode_vcn0 } else { &vcn_5_0_0_video_codecs_decode_vcn0 };
            0
        },
        _ => -EINVAL,
    }
}

unsafe fn soc24_get_config_memsize(adev: *mut amdgpu_device) -> u32 { ((*adev).nbio.funcs->get_memsize)(adev) }
unsafe fn soc24_get_xclk(adev: *mut amdgpu_device) -> u32 { (*adev).clock.spll.reference_freq }

unsafe fn soc24_grbm_select(adev: *mut amdgpu_device, me: u32, pipe: u32, queue: u32, vmid: u32) {
    let mut grbm_gfx_cntl = 0;
    grbm_gfx_cntl = REG_SET_FIELD(grbm_gfx_cntl, GRBM_GFX_CNTL, PIPEID, pipe);
    grbm_gfx_cntl = REG_SET_FIELD(grbm_gfx_cntl, GRBM_GFX_CNTL, MEID, me);
    grbm_gfx_cntl = REG_SET_FIELD(grbm_gfx_cntl, GRBM_GFX_CNTL, VMID, vmid);
    grbm_gfx_cntl = REG_SET_FIELD(grbm_gfx_cntl, GRBM_GFX_CNTL, QUEUEID, queue);
    WREG32_SOC15!(GC, 0, regGRBM_GFX_CNTL, grbm_gfx_cntl);
}

static mut soc24_allowed_read_registers: [soc15_allowed_register_entry; 19] = [
    SOC15_REG_ENTRY!(GC, 0, regGRBM_STATUS), SOC15_REG_ENTRY!(GC, 0, regGRBM_STATUS2),
    SOC15_REG_ENTRY!(GC, 0, regGRBM_STATUS_SE0), SOC15_REG_ENTRY!(GC, 0, regGRBM_STATUS_SE1),
    SOC15_REG_ENTRY!(GC, 0, regGRBM_STATUS_SE2), SOC15_REG_ENTRY!(GC, 0, regGRBM_STATUS_SE3),
    SOC15_REG_ENTRY!(SDMA0, 0, regSDMA0_STATUS_REG), SOC15_REG_ENTRY!(SDMA1, 0, regSDMA1_STATUS_REG),
    SOC15_REG_ENTRY!(GC, 0, regCP_STAT), SOC15_REG_ENTRY!(GC, 0, regCP_STALLED_STAT1),
    SOC15_REG_ENTRY!(GC, 0, regCP_STALLED_STAT2), SOC15_REG_ENTRY!(GC, 0, regCP_STALLED_STAT3),
    SOC15_REG_ENTRY!(GC, 0, regCP_CPF_BUSY_STAT), SOC15_REG_ENTRY!(GC, 0, regCP_CPF_STALLED_STAT1),
    SOC15_REG_ENTRY!(GC, 0, regCP_CPF_STATUS), SOC15_REG_ENTRY!(GC, 0, regCP_CPC_BUSY_STAT),
    SOC15_REG_ENTRY!(GC, 0, regCP_CPC_STALLED_STAT1), SOC15_REG_ENTRY!(GC, 0, regCP_CPC_STATUS),
    SOC15_REG_ENTRY!(GC, 0, regGB_ADDR_CONFIG),
];

unsafe fn soc24_get_register_value(adev: *mut amdgpu_device, indexed: bool, se_num: u32, sh_num: u32, reg_offset: u32) -> u32 {
    if indexed { amdgpu_read_indexed_register(adev, se_num, sh_num, reg_offset) }
    else if reg_offset == SOC15_REG_OFFSET!(GC, 0, regGB_ADDR_CONFIG) && (*adev).gfx.config.gb_addr_config != 0 { (*adev).gfx.config.gb_addr_config }
    else { RREG32!(reg_offset) }
}

unsafe fn soc24_read_register(adev: *mut amdgpu_device, se_num: u32, sh_num: u32, reg_offset: u32, value: *mut u32) -> i32 {
    *value = 0;
    for i in 0..soc24_allowed_read_registers.len() {
        let en = &soc24_allowed_read_registers[i];
        if (*adev).reg_offset[en.hwip][en.inst].is_null() { continue; }
        if reg_offset != (*adev).reg_offset[en.hwip][en.inst][en.seg] + en.reg_offset { continue; }
        *value = soc24_get_register_value(adev, en.grbm_indexed, se_num, sh_num, reg_offset);
        return 0;
    }
    -EINVAL
}

unsafe fn soc24_asic_reset_method(adev: *mut amdgpu_device) -> amd_reset_method {
    if amdgpu_reset_method == AMD_RESET_METHOD_MODE1 || amdgpu_reset_method == AMD_RESET_METHOD_MODE2 || amdgpu_reset_method == AMD_RESET_METHOD_BACO { return amdgpu_reset_method; }
    if amdgpu_reset_method != -1 { dev_warn((*adev).dev, "Specified reset method:%d isn't supported, using AUTO instead.\n", amdgpu_reset_method); }
    match amdgpu_ip_version(adev, MP1_HWIP, 0) { IP_VERSION(14,0,2) | IP_VERSION(14,0,3) => AMD_RESET_METHOD_MODE1, _ => if amdgpu_dpm_is_baco_supported(adev) { AMD_RESET_METHOD_BACO } else { AMD_RESET_METHOD_MODE1 } }
}

unsafe fn soc24_asic_reset(adev: *mut amdgpu_device) -> i32 {
    let mut ret = 0;
    match soc24_asic_reset_method(adev) { AMD_RESET_METHOD_PCI => { dev_info((*adev).dev, "PCI reset\n"); ret = amdgpu_device_pci_reset(adev); }, AMD_RESET_METHOD_BACO => { dev_info((*adev).dev, "BACO reset\n"); ret = amdgpu_dpm_baco_reset(adev); }, AMD_RESET_METHOD_MODE2 => { dev_info((*adev).dev, "MODE2 reset\n"); ret = amdgpu_dpm_mode2_reset(adev); }, _ => { dev_info((*adev).dev, "MODE1 reset\n"); ret = amdgpu_device_mode1_reset(adev); } }
    ret
}

unsafe fn soc24_program_aspm(adev: *mut amdgpu_device) { if !amdgpu_device_should_use_aspm(adev) { return; } if (*adev).flags & AMD_IS_APU == 0 && (*adev).nbio.funcs->program_aspm != None { ((*adev).nbio.funcs->program_aspm)(adev); } }

const soc24_common_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_: AMD_IP_BLOCK_TYPE_COMMON, major: 1, minor: 0, rev: 0, funcs: &soc24_common_ip_funcs };

unsafe fn soc24_need_reset_on_init(adev: *mut amdgpu_device) -> bool { if (*adev).flags & AMD_IS_APU != 0 { return false; } RREG32_SOC15!(MP0, 0, regMPASP_SMN_C2PMSG_81) != 0 }
unsafe fn soc24_get_pcie_replay_count(_: *mut amdgpu_device) -> u64 { /* dummy implement for pcie_replay_count sysfs interface */ 0 }

unsafe fn soc24_init_doorbell_index(adev: *mut amdgpu_device) {
    (*adev).doorbell_index.kiq = AMDGPU_NAVI10_DOORBELL_KIQ; (*adev).doorbell_index.mec_ring0 = AMDGPU_NAVI10_DOORBELL_MEC_RING0; (*adev).doorbell_index.mec_ring1 = AMDGPU_NAVI10_DOORBELL_MEC_RING1; (*adev).doorbell_index.mec_ring2 = AMDGPU_NAVI10_DOORBELL_MEC_RING2; (*adev).doorbell_index.mec_ring3 = AMDGPU_NAVI10_DOORBELL_MEC_RING3; (*adev).doorbell_index.mec_ring4 = AMDGPU_NAVI10_DOORBELL_MEC_RING4; (*adev).doorbell_index.mec_ring5 = AMDGPU_NAVI10_DOORBELL_MEC_RING5; (*adev).doorbell_index.mec_ring6 = AMDGPU_NAVI10_DOORBELL_MEC_RING6; (*adev).doorbell_index.mec_ring7 = AMDGPU_NAVI10_DOORBELL_MEC_RING7;
    (*adev).doorbell_index.userqueue_start = AMDGPU_NAVI10_DOORBELL_USERQUEUE_START; (*adev).doorbell_index.userqueue_end = AMDGPU_NAVI10_DOORBELL_USERQUEUE_END; (*adev).doorbell_index.gfx_ring0 = AMDGPU_NAVI10_DOORBELL_GFX_RING0; (*adev).doorbell_index.gfx_ring1 = AMDGPU_NAVI10_DOORBELL_GFX_RING1; (*adev).doorbell_index.gfx_userqueue_start = AMDGPU_NAVI10_DOORBELL_GFX_USERQUEUE_START; (*adev).doorbell_index.gfx_userqueue_end = AMDGPU_NAVI10_DOORBELL_GFX_USERQUEUE_END;
    (*adev).doorbell_index.mes_ring0 = AMDGPU_NAVI10_DOORBELL_MES_RING0; (*adev).doorbell_index.mes_ring1 = AMDGPU_NAVI10_DOORBELL_MES_RING1; (*adev).doorbell_index.sdma_engine[0] = AMDGPU_NAVI10_DOORBELL_sDMA_ENGINE0; (*adev).doorbell_index.sdma_engine[1] = AMDGPU_NAVI10_DOORBELL_sDMA_ENGINE1; (*adev).doorbell_index.ih = AMDGPU_NAVI10_DOORBELL_IH; (*adev).doorbell_index.vcn.vcn_ring0_1 = AMDGPU_NAVI10_DOORBELL64_VCN0_1; (*adev).doorbell_index.vcn.vcn_ring2_3 = AMDGPU_NAVI10_DOORBELL64_VCN2_3; (*adev).doorbell_index.vcn.vcn_ring4_5 = AMDGPU_NAVI10_DOORBELL64_VCN4_5; (*adev).doorbell_index.vcn.vcn_ring6_7 = AMDGPU_NAVI10_DOORBELL64_VCN6_7; (*adev).doorbell_index.first_non_cp = AMDGPU_NAVI10_DOORBELL64_FIRST_NON_CP; (*adev).doorbell_index.last_non_cp = AMDGPU_NAVI10_DOORBELL64_LAST_NON_CP; (*adev).doorbell_index.max_assignment = AMDGPU_NAVI10_DOORBELL_MAX_ASSIGNMENT << 1; (*adev).doorbell_index.sdma_doorbell_range = 20;
}

unsafe fn soc24_update_umd_stable_pstate(adev: *mut amdgpu_device, enter: bool) -> i32 { if enter { amdgpu_gfx_rlc_enter_safe_mode(adev, 0); } else { amdgpu_gfx_rlc_exit_safe_mode(adev, 0); } if (*adev).gfx.funcs->update_perfmon_mgcg != None { ((*adev).gfx.funcs->update_perfmon_mgcg)(adev, !enter); } 0 }

static const soc24_asic_funcs: amdgpu_asic_funcs = amdgpu_asic_funcs {
    read_bios_from_rom: Some(amdgpu_soc15_read_bios_from_rom), read_register: Some(soc24_read_register), reset: Some(soc24_asic_reset), reset_method: Some(soc24_asic_reset_method), get_xclk: Some(soc24_get_xclk), get_config_memsize: Some(soc24_get_config_memsize), init_doorbell_index: Some(soc24_init_doorbell_index), need_reset_on_init: Some(soc24_need_reset_on_init), get_pcie_replay_count: Some(soc24_get_pcie_replay_count), supports_baco: Some(amdgpu_dpm_is_baco_supported), query_video_codecs: Some(soc24_query_video_codecs), update_umd_stable_pstate: Some(soc24_update_umd_stable_pstate),
};

unsafe fn soc24_common_early_init(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; ((*adev).nbio.funcs->set_reg_remap)(adev); (*adev).reg.pcie.rreg = Some(amdgpu_device_indirect_rreg); (*adev).reg.pcie.wreg = Some(amdgpu_device_indirect_wreg); (*adev).reg.pcie.rreg64 = Some(amdgpu_device_indirect_rreg64); (*adev).reg.pcie.wreg64 = Some(amdgpu_device_indirect_wreg64); (*adev).reg.pcie.port_rreg = Some(amdgpu_device_pcie_port_rreg); (*adev).reg.pcie.port_wreg = Some(amdgpu_device_pcie_port_wreg); (*adev).asic_funcs = &soc24_asic_funcs; (*adev).rev_id = amdgpu_device_get_rev_id(adev); (*adev).external_rev_id = 0xff;
    match amdgpu_ip_version(adev, GC_HWIP, 0) { IP_VERSION(12,0,0) => { (*adev).cg_flags = AMD_CG_SUPPORT_GFX_CGCG | AMD_CG_SUPPORT_GFX_CGLS | AMD_CG_SUPPORT_GFX_MGCG | AMD_CG_SUPPORT_GFX_3D_CGCG | AMD_CG_SUPPORT_GFX_3D_CGLS | AMD_CG_SUPPORT_REPEATER_FGCG | AMD_CG_SUPPORT_GFX_FGCG | AMD_CG_SUPPORT_GFX_PERF_CLK | AMD_CG_SUPPORT_ATHUB_MGCG | AMD_CG_SUPPORT_ATHUB_LS | AMD_CG_SUPPORT_MC_MGCG | AMD_CG_SUPPORT_HDP_SD | AMD_CG_SUPPORT_MC_LS; (*adev).pg_flags = AMD_PG_SUPPORT_VCN | AMD_PG_SUPPORT_JPEG | AMD_PG_SUPPORT_VCN_DPG; (*adev).external_rev_id = (*adev).rev_id + 0x40; }, IP_VERSION(12,0,1) => { (*adev).cg_flags = AMD_CG_SUPPORT_GFX_CGCG | AMD_CG_SUPPORT_GFX_CGLS | AMD_CG_SUPPORT_GFX_MGCG | AMD_CG_SUPPORT_GFX_3D_CGCG | AMD_CG_SUPPORT_GFX_3D_CGLS | AMD_CG_SUPPORT_REPEATER_FGCG | AMD_CG_SUPPORT_GFX_FGCG | AMD_CG_SUPPORT_GFX_PERF_CLK | AMD_CG_SUPPORT_ATHUB_MGCG | AMD_CG_SUPPORT_ATHUB_LS | AMD_CG_SUPPORT_MC_MGCG | AMD_CG_SUPPORT_HDP_SD | AMD_CG_SUPPORT_MC_LS; (*adev).pg_flags = AMD_PG_SUPPORT_VCN | AMD_PG_SUPPORT_JPEG | AMD_PG_SUPPORT_JPEG_DPG | AMD_PG_SUPPORT_VCN_DPG; (*adev).external_rev_id = (*adev).rev_id + 0x50; }, _ => return -EINVAL }
    if amdgpu_sriov_vf(adev) { amdgpu_virt_init_setting(adev); xgpu_nv_mailbox_set_irq_funcs(adev); } 0 }

unsafe fn soc24_common_late_init(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; if amdgpu_sriov_vf(adev) { xgpu_nv_mailbox_get_irq(adev); } else if (*adev).nbio.ras && (*adev).nbio.ras_err_event_athub_irq.funcs { amdgpu_irq_get(adev, &mut (*adev).nbio.ras_err_event_athub_irq, 0); } ((*adev).nbio.funcs->enable_doorbell_selfring_aperture)(adev, true); 0 }
unsafe fn soc24_common_sw_init(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; if amdgpu_sriov_vf(adev) { xgpu_nv_mailbox_add_irq_id(adev); } 0 }
unsafe fn soc24_common_hw_init(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; soc24_program_aspm(adev); ((*adev).nbio.funcs->init_registers)(adev); if (*adev).nbio.funcs->remap_hdp_registers != None { ((*adev).nbio.funcs->remap_hdp_registers)(adev); } if (*adev).df.funcs != None && (*adev).df.funcs->hw_init != None { ((*adev).df.funcs->hw_init)(adev); } ((*adev).nbio.funcs->enable_doorbell_aperture)(adev, true); 0 }
unsafe fn soc24_common_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; ((*adev).nbio.funcs->enable_doorbell_aperture)(adev, false); ((*adev).nbio.funcs->enable_doorbell_selfring_aperture)(adev, false); if amdgpu_sriov_vf(adev) { xgpu_nv_mailbox_put_irq(adev); } else if (*adev).nbio.ras && (*adev).nbio.ras_err_event_athub_irq.funcs { amdgpu_irq_put(adev, &mut (*adev).nbio.ras_err_event_athub_irq, 0); } 0 }
unsafe fn soc24_common_suspend(ip_block: *mut amdgpu_ip_block) -> i32 { soc24_common_hw_fini(ip_block) }
unsafe fn soc24_need_reset_on_resume(adev: *mut amdgpu_device) -> bool { if (*adev).flags & AMD_IS_APU == 0 && (*adev).in_s3 { let sol_reg1 = RREG32_SOC15!(MP0, 0, regMPASP_SMN_C2PMSG_81); msleep(100); let sol_reg2 = RREG32_SOC15!(MP0, 0, regMPASP_SMN_C2PMSG_81); return sol_reg1 != sol_reg2; } false }
unsafe fn soc24_common_resume(ip_block: *mut amdgpu_ip_block) -> i32 { let adev = (*ip_block).adev; if soc24_need_reset_on_resume(adev) { dev_info((*adev).dev, "S3 suspend aborted, resetting..."); soc24_asic_reset(adev); } soc24_common_hw_init(ip_block) }
unsafe fn soc24_common_is_idle(_: *mut amdgpu_ip_block) -> bool { true }
unsafe fn soc24_common_set_clockgating_state(ip_block: *mut amdgpu_ip_block, state: amd_clockgating_state) -> i32 { let adev = (*ip_block).adev; if amdgpu_ip_version(adev, NBIO_HWIP, 0) == IP_VERSION(6,3,1) { ((*adev).nbio.funcs->update_medium_grain_clock_gating)(adev, state == AMD_CG_STATE_GATE); ((*adev).nbio.funcs->update_medium_grain_light_sleep)(adev, state == AMD_CG_STATE_GATE); ((*adev).hdp.funcs->update_clock_gating)(adev, state == AMD_CG_STATE_GATE); } 0 }
unsafe fn soc24_common_set_powergating_state(ip_block: *mut amdgpu_ip_block, state: amd_powergating_state) -> i32 { let adev = (*ip_block).adev; match amdgpu_ip_version(adev, LSDMA_HWIP, 0) { IP_VERSION(7,0,0) | IP_VERSION(7,0,1) => ((*adev).lsdma.funcs->update_memory_power_gating)(adev, state == AMD_PG_STATE_GATE), _ => {} } 0 }
unsafe fn soc24_common_get_clockgating_state(ip_block: *mut amdgpu_ip_block, flags: *mut u64) { let adev = (*ip_block).adev; ((*adev).nbio.funcs->get_clockgating_state)(adev, flags); ((*adev).hdp.funcs->get_clock_gating_state)(adev, flags); }

static const soc24_common_ip_funcs: amd_ip_funcs = amd_ip_funcs { name: "soc24_common", early_init: Some(soc24_common_early_init), late_init: Some(soc24_common_late_init), sw_init: Some(soc24_common_sw_init), hw_init: Some(soc24_common_hw_init), hw_fini: Some(soc24_common_hw_fini), suspend: Some(soc24_common_suspend), resume: Some(soc24_common_resume), is_idle: Some(soc24_common_is_idle), set_clockgating_state: Some(soc24_common_set_clockgating_state), set_powergating_state: Some(soc24_common_set_powergating_state), get_clockgating_state: Some(soc24_common_get_clockgating_state) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
