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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C dependencies supplied by the surrounding kernel translation unit.
// MODULE_FIRMWARE("amdgpu/psp_15_0_8_toc.bin");
// MODULE_FIRMWARE("amdgpu/psp_15_0_8_toc_1.bin");

unsafe fn psp_v15_0_8_init_microcode(psp: *mut psp_context) -> i32 {
    let adev = (*psp).adev;
    let mut ucode_prefix = [0i8; 30];
    let mut err: i32 = 0;

    amdgpu_ucode_ip_version_decode(adev, MP0_HWIP, ucode_prefix.as_mut_ptr(), ucode_prefix.len());
    err = psp_init_toc_microcode(psp, ucode_prefix.as_mut_ptr());
    if err != 0 { return err; }
    0
}

unsafe fn psp_v15_0_8_ring_stop(psp: *mut psp_context, _ring_type: psp_ring_type) -> i32 {
    let mut ret: i32 = 0;
    let adev = (*psp).adev;

    if amdgpu_sriov_vf(adev) {
        // Write the ring destroy command
        WREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_DESTROY_GPCOM_RING);
        // there might be handshake issue with hardware which needs delay
        mdelay(20);
        // Wait for response flag (bit 31)
        ret = psp_wait_for(psp, SOC15_REG_OFFSET(MP0, 0, regMPASP_SMN_C2PMSG_101), 0x80000000, 0x80000000, false);
    } else {
        // Write the ring destroy command
        WREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_64, GFX_CTRL_CMD_ID_DESTROY_RINGS);
        // there might be handshake issue with hardware which needs delay
        mdelay(20);
        // Wait for response flag (bit 31)
        ret = psp_wait_for(psp, SOC15_REG_OFFSET(MP0, 0, regMPASP_SMN_C2PMSG_64), 0x80000000, 0x80000000, false);
    }
    ret
}

unsafe fn psp_v15_0_8_ring_create(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let mut ret: i32 = 0;
    let mut psp_ring_reg: u32 = 0;
    let ring = &mut (*psp).km_ring;
    let adev = (*psp).adev;

    if amdgpu_sriov_vf(adev) {
        ret = psp_v15_0_8_ring_stop(psp, ring_type);
        if ret != 0 { DRM_ERROR("psp_v14_0_ring_stop_sriov failed!\n"); return ret; }
        psp_ring_reg = lower_32_bits(ring.ring_mem_mc_addr);
        WREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_102, psp_ring_reg);
        psp_ring_reg = upper_32_bits(ring.ring_mem_mc_addr);
        WREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_103, psp_ring_reg);
        WREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_INIT_GPCOM_RING);
        mdelay(20);
        ret = psp_wait_for(psp, SOC15_REG_OFFSET(MP0, 0, regMPASP_SMN_C2PMSG_101), 0x80000000, 0x8000FFFF, false);
    } else {
        ret = psp_wait_for(psp, SOC15_REG_OFFSET(MP0, 0, regMPASP_SMN_C2PMSG_64), 0x80000000, 0x80000000, false);
        if ret != 0 { DRM_ERROR("Failed to wait for trust OS ready for ring creation\n"); return ret; }
        psp_ring_reg = lower_32_bits(ring.ring_mem_mc_addr);
        WREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_69, psp_ring_reg);
        psp_ring_reg = upper_32_bits(ring.ring_mem_mc_addr);
        WREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_70, psp_ring_reg);
        psp_ring_reg = ring.ring_size;
        WREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_71, psp_ring_reg);
        psp_ring_reg = ring_type as u32;
        psp_ring_reg <<= 16;
        WREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_64, psp_ring_reg);
        mdelay(20);
        ret = psp_wait_for(psp, SOC15_REG_OFFSET(MP0, 0, regMPASP_SMN_C2PMSG_64), 0x80000000, 0x8000FFFF, false);
    }
    ret
}

unsafe fn psp_v15_0_8_ring_destroy(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let ring = &mut (*psp).km_ring;
    let adev = (*psp).adev;
    let ret = psp_v15_0_8_ring_stop(psp, ring_type);
    if ret != 0 { DRM_ERROR("Fail to stop psp ring\n"); }
    amdgpu_bo_free_kernel(&mut (*adev).firmware.rbuf, &mut ring.ring_mem_mc_addr, &mut ring.ring_mem as *mut _ as *mut *mut core::ffi::c_void);
    ret
}

unsafe fn psp_v15_0_8_ring_get_wptr(psp: *mut psp_context) -> u32 {
    let adev = (*psp).adev;
    if amdgpu_sriov_vf(adev) { RREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_102) } else { RREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_67) }
}

unsafe fn psp_v15_0_8_ring_set_wptr(psp: *mut psp_context, value: u32) {
    let adev = (*psp).adev;
    if amdgpu_sriov_vf(adev) {
        WREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_102, value);
        WREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_CONSUME_CMD);
    } else { WREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_67, value); }
}

unsafe fn psp_v15_0_8_get_ras_capability(psp: *mut psp_context) -> bool {
    let adev = (*psp).adev;
    let con = amdgpu_ras_get_context(adev);
    if amdgpu_sriov_vf(adev) || con.is_null() { return false; }
    let reg_data = RREG32_SOC15(MP0, 0, regMPASP_SMN_C2PMSG_127);
    (*adev).ras_hw_enabled = reg_data & GENMASK_ULL(23, 0);
    (*con).poison_supported = ((reg_data & GENMASK_ULL(24, 24)) >> 24) != 0;
    true
}

unsafe fn psp_v15_0_8_get_fw_type(ucode: *mut amdgpu_firmware_info, kind: *mut psp_gfx_fw_type) -> i32 {
    *kind = match (*ucode).ucode_id {
        AMDGPU_UCODE_ID_CAP => GFX_FW_TYPE_CAP,
        AMDGPU_UCODE_ID_SDMA0 => GFX_FW_TYPE_SDMA0,
        AMDGPU_UCODE_ID_SDMA1 => GFX_FW_TYPE_SDMA1,
        AMDGPU_UCODE_ID_SDMA2 => GFX_FW_TYPE_SDMA2,
        AMDGPU_UCODE_ID_SDMA3 => GFX_FW_TYPE_SDMA3,
        AMDGPU_UCODE_ID_SDMA4 => GFX_FW_TYPE_SDMA4,
        AMDGPU_UCODE_ID_SDMA5 => GFX_FW_TYPE_SDMA5,
        AMDGPU_UCODE_ID_SDMA6 => GFX_FW_TYPE_SDMA6,
        AMDGPU_UCODE_ID_SDMA7 => GFX_FW_TYPE_SDMA7,
        AMDGPU_UCODE_ID_CP_MES => GFX_FW_TYPE_RS64_MES,
        AMDGPU_UCODE_ID_CP_MES_DATA => GFX_FW_TYPE_RS64_MES_STACK,
        AMDGPU_UCODE_ID_CP_MES1 => GFX_FW_TYPE_RS64_KIQ,
        AMDGPU_UCODE_ID_CP_MES1_DATA => GFX_FW_TYPE_RS64_KIQ_STACK,
        AMDGPU_UCODE_ID_RLC_P => GFX_FW_TYPE_RLC_P,
        AMDGPU_UCODE_ID_RLC_V => GFX_FW_TYPE_RLC_V,
        AMDGPU_UCODE_ID_RLC_G => GFX_FW_TYPE_RLC_G,
        AMDGPU_UCODE_ID_RLC_RESTORE_LIST_CNTL => GFX_FW_TYPE_RLC_RESTORE_LIST_SRM_CNTL,
        AMDGPU_UCODE_ID_RLC_RESTORE_LIST_GPM_MEM => GFX_FW_TYPE_RLC_RESTORE_LIST_GPM_MEM,
        AMDGPU_UCODE_ID_RLC_RESTORE_LIST_SRM_MEM => GFX_FW_TYPE_RLC_RESTORE_LIST_SRM_MEM,
        AMDGPU_UCODE_ID_RLC_IRAM => GFX_FW_TYPE_RLC_IRAM,
        AMDGPU_UCODE_ID_RLC_DRAM => GFX_FW_TYPE_RLC_DRAM_BOOT,
        AMDGPU_UCODE_ID_RLC_IRAM_1 => GFX_FW_TYPE_RLX6_UCODE_CORE1,
        AMDGPU_UCODE_ID_RLC_DRAM_1 => GFX_FW_TYPE_RLX6_DRAM_BOOT_CORE1,
        AMDGPU_UCODE_ID_SMC => GFX_FW_TYPE_SMU,
        AMDGPU_UCODE_ID_PPTABLE => GFX_FW_TYPE_PPTABLE,
        AMDGPU_UCODE_ID_VCN => GFX_FW_TYPE_VCN,
        AMDGPU_UCODE_ID_VCN1 => GFX_FW_TYPE_VCN1,
        AMDGPU_UCODE_ID_VCN0_RAM => GFX_FW_TYPE_VCN0_RAM,
        AMDGPU_UCODE_ID_VCN1_RAM => GFX_FW_TYPE_VCN1_RAM,
        AMDGPU_UCODE_ID_SDMA_UCODE_TH0 | AMDGPU_UCODE_ID_SDMA_RS64 => GFX_FW_TYPE_SDMA0,
        AMDGPU_UCODE_ID_SDMA_UCODE_TH1 => GFX_FW_TYPE_SDMA_UCODE_TH1,
        AMDGPU_UCODE_ID_IMU_I => GFX_FW_TYPE_IMU_I,
        AMDGPU_UCODE_ID_IMU_D => GFX_FW_TYPE_IMU_D,
        AMDGPU_UCODE_ID_CP_RS64_MEC => GFX_FW_TYPE_RS64_MEC,
        AMDGPU_UCODE_ID_CP_RS64_MEC_P0_STACK => GFX_FW_TYPE_RS64_MEC_P0_STACK,
        AMDGPU_UCODE_ID_CP_RS64_MEC_P1_STACK => GFX_FW_TYPE_RS64_MEC_P1_STACK,
        AMDGPU_UCODE_ID_CP_RS64_MEC_P2_STACK => GFX_FW_TYPE_RS64_MEC_P2_STACK,
        AMDGPU_UCODE_ID_CP_RS64_MEC_P3_STACK => GFX_FW_TYPE_RS64_MEC_P3_STACK,
        AMDGPU_UCODE_ID_UMSCH_MM_UCODE => GFX_FW_TYPE_UMSCH_UCODE,
        AMDGPU_UCODE_ID_UMSCH_MM_DATA => GFX_FW_TYPE_UMSCH_DATA,
        AMDGPU_UCODE_ID_UMSCH_MM_CMD_BUFFER => GFX_FW_TYPE_UMSCH_CMD_BUFFER,
        AMDGPU_UCODE_ID_P2S_TABLE => GFX_FW_TYPE_P2S_TABLE,
        _ => return -EINVAL,
    };
    0
}

static psp_v15_0_8_funcs: psp_funcs = psp_funcs {
    init_microcode: Some(psp_v15_0_8_init_microcode),
    ring_create: Some(psp_v15_0_8_ring_create),
    ring_stop: Some(psp_v15_0_8_ring_stop),
    ring_destroy: Some(psp_v15_0_8_ring_destroy),
    ring_get_wptr: Some(psp_v15_0_8_ring_get_wptr),
    ring_set_wptr: Some(psp_v15_0_8_ring_set_wptr),
    get_fw_type: Some(psp_v15_0_8_get_fw_type),
    get_ras_capability: Some(psp_v15_0_8_get_ras_capability),
};

pub unsafe fn psp_v15_0_8_set_psp_funcs(psp: *mut psp_context) {
    (*psp).funcs = &mut psp_v15_0_8_funcs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
