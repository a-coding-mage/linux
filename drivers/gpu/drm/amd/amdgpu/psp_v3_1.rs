/*
 * Copyright 2016 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Author: Huang Rui
 */

// Firmware declarations and Linux/kernel dependencies are supplied externally.

const SMNMP1_FIRMWARE_FLAGS: u32 = 0x3010028;

unsafe fn psp_v3_1_init_microcode(psp: *mut psp_context) -> i32 {
    let adev = (*psp).adev;
    let mut ucode_prefix = [0i8; 30];
    let mut err: i32 = 0;

    DRM_DEBUG!("\n");
    amdgpu_ucode_ip_version_decode(adev, MP0_HWIP, ucode_prefix.as_mut_ptr(), ucode_prefix.len());
    err = psp_init_sos_microcode(psp, ucode_prefix.as_mut_ptr());
    if err != 0 { return err; }
    err = psp_init_asd_microcode(psp, ucode_prefix.as_mut_ptr());
    if err != 0 { return err; }
    0
}

unsafe fn psp_v3_1_bootloader_load_sysdrv(psp: *mut psp_context) -> i32 {
    let mut ret: i32;
    let mut psp_gfxdrv_command_reg: u32 = 0;
    let adev = (*psp).adev;
    let sol_reg: u32;
    sol_reg = RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_81);
    if sol_reg != 0 { return 0; }
    ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_35), 0x80000000, 0x80000000, 0);
    if ret != 0 { return ret; }
    ret = psp_copy_fw(psp, (*psp).sys.start_addr, (*psp).sys.size_bytes);
    if ret != 0 { return ret; }
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_36, ((*psp).fw_pri_mc_addr >> 20) as u32);
    psp_gfxdrv_command_reg = PSP_BL__LOAD_SYSDRV;
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_35, psp_gfxdrv_command_reg);
    mdelay(20);
    ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_35), 0x80000000, 0x80000000, 0);
    ret
}

unsafe fn psp_v3_1_bootloader_load_sos(psp: *mut psp_context) -> i32 {
    let mut ret: i32;
    let mut psp_gfxdrv_command_reg: u32 = 0;
    let sol_reg: u32;
    sol_reg = RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_81);
    if sol_reg != 0 { return 0; }
    ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_35), 0x80000000, 0x80000000, 0);
    if ret != 0 { return ret; }
    ret = psp_copy_fw(psp, (*psp).sos.start_addr, (*psp).sos.size_bytes);
    if ret != 0 { return ret; }
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_36, ((*psp).fw_pri_mc_addr >> 20) as u32);
    psp_gfxdrv_command_reg = PSP_BL__LOAD_SOSDRV;
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_35, psp_gfxdrv_command_reg);
    mdelay(20);
    psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_81), RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_81), 0, PSP_WAITREG_CHANGED)
}

unsafe fn psp_v3_1_reroute_ih(psp: *mut psp_context) {
    let mut tmp = REG_SET_FIELD!(0, IH_CLIENT_CFG_DATA, CREDIT_RETURN_ADDR, 0x1244b);
    tmp = REG_SET_FIELD!(tmp, IH_CLIENT_CFG_DATA, CLIENT_TYPE, 1);
    tmp = REG_SET_FIELD!(tmp, IH_CLIENT_CFG_DATA, RING_ID, 1);
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_69, 3); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_70, tmp); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_64, GFX_CTRL_CMD_ID_GBR_IH_SET);
    mdelay(20); psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), 0x80000000, 0x8000FFFF, 0);
    tmp = REG_SET_FIELD!(0, IH_CLIENT_CFG_DATA, CREDIT_RETURN_ADDR, 0x1216b);
    tmp = REG_SET_FIELD!(tmp, IH_CLIENT_CFG_DATA, RING_ID, 1);
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_69, 4); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_70, tmp); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_64, GFX_CTRL_CMD_ID_GBR_IH_SET);
    mdelay(20); psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), 0x80000000, 0x8000FFFF, 0);
}

unsafe fn psp_v3_1_ring_create(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let mut ret = 0; let mut psp_ring_reg = 0; let ring = &mut (*psp).km_ring; let adev = (*psp).adev;
    psp_v3_1_reroute_ih(psp);
    if amdgpu_sriov_vf(adev) { ring.ring_wptr = 0; ret = psp_v3_1_ring_stop(psp, ring_type); if ret != 0 { DRM_ERROR!("psp_v3_1_ring_stop_sriov failed!\n"); return ret; }
        psp_ring_reg = lower_32_bits(ring.ring_mem_mc_addr); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_102, psp_ring_reg);
        psp_ring_reg = upper_32_bits(ring.ring_mem_mc_addr); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_103, psp_ring_reg);
        psp_ring_reg = (ring_type as u32) << 16; WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_101, psp_ring_reg); mdelay(20);
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_101), 0x80000000, 0x8000FFFF, 0);
    } else { psp_ring_reg = lower_32_bits(ring.ring_mem_mc_addr); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_69, psp_ring_reg); psp_ring_reg = upper_32_bits(ring.ring_mem_mc_addr); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_70, psp_ring_reg); psp_ring_reg = ring.ring_size; WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_71, psp_ring_reg); psp_ring_reg = (ring_type as u32) << 16; WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_64, psp_ring_reg); mdelay(20); ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), 0x80000000, 0x8000FFFF, 0); }
    ret
}

unsafe fn psp_v3_1_ring_stop(psp: *mut psp_context, _ring_type: psp_ring_type) -> i32 {
    let adev = (*psp).adev;
    if amdgpu_sriov_vf(adev) { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_DESTROY_GPCOM_RING); mdelay(20); psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_101), 0x80000000, 0x80000000, 0) } else { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_64, GFX_CTRL_CMD_ID_DESTROY_RINGS); mdelay(20); psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), 0x80000000, 0x80000000, 0) }
}

unsafe fn psp_v3_1_ring_destroy(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let ring = &mut (*psp).km_ring; let adev = (*psp).adev; let ret = psp_v3_1_ring_stop(psp, ring_type); if ret != 0 { DRM_ERROR!("Fail to stop psp ring\n"); }
    amdgpu_bo_free_kernel(&mut (*adev).firmware.rbuf, &mut ring.ring_mem_mc_addr, &mut ring.ring_mem as *mut _ as *mut *mut core::ffi::c_void); ret
}

unsafe fn psp_v3_1_smu_reload_quirk(psp: *mut psp_context) -> bool { let adev = (*psp).adev; let reg = RREG32_PCIE!(SMNMP1_FIRMWARE_FLAGS | 0x03b00000); (reg & MP1_FIRMWARE_FLAGS__INTERRUPTS_ENABLED_MASK) != 0 }

unsafe fn psp_v3_1_mode1_reset(psp: *mut psp_context) -> i32 {
    let adev = (*psp).adev; let mut offset = SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64); let mut ret = psp_wait_for(psp, offset, 0x80000000, 0x8000FFFF, 0);
    if ret != 0 { drm_info!(adev_to_drm(adev), "psp is not working correctly before mode1 reset!\n"); return -EINVAL; }
    WREG32!(offset, GFX_CTRL_CMD_ID_MODE1_RST); msleep(500); offset = SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_33); ret = psp_wait_for(psp, offset, 0x80000000, 0x80000000, 0);
    if ret != 0 { drm_info!(adev_to_drm(adev), "psp mode 1 reset failed!\n"); return -EINVAL; }
    drm_info!(adev_to_drm(adev), "psp mode1 reset succeed\n"); 0
}

unsafe fn psp_v3_1_ring_get_wptr(psp: *mut psp_context) -> u32 { let adev = (*psp).adev; if amdgpu_sriov_vf(adev) { (*psp).km_ring.ring_wptr } else { RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_67) } }
unsafe fn psp_v3_1_ring_set_wptr(psp: *mut psp_context, value: u32) { let adev = (*psp).adev; if amdgpu_sriov_vf(adev) { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_102, value); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_CONSUME_CMD); (*psp).km_ring.ring_wptr = value; } else { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_67, value); } }

static psp_v3_1_funcs: psp_funcs = psp_funcs { init_microcode: Some(psp_v3_1_init_microcode), bootloader_load_sysdrv: Some(psp_v3_1_bootloader_load_sysdrv), bootloader_load_sos: Some(psp_v3_1_bootloader_load_sos), ring_create: Some(psp_v3_1_ring_create), ring_stop: Some(psp_v3_1_ring_stop), ring_destroy: Some(psp_v3_1_ring_destroy), smu_reload_quirk: Some(psp_v3_1_smu_reload_quirk), mode1_reset: Some(psp_v3_1_mode1_reset), ring_get_wptr: Some(psp_v3_1_ring_get_wptr), ring_set_wptr: Some(psp_v3_1_ring_set_wptr) };

unsafe fn psp_v3_1_set_psp_funcs(psp: *mut psp_context) { (*psp).funcs = &psp_v3_1_funcs; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
