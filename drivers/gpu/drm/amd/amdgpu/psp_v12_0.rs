/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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
 */

// Kernel and driver dependencies supplied by the surrounding translation unit.

/* address block */
const SMN_MP1_FIRMWARE_FLAGS: u32 = 0x3010024;

unsafe fn psp_v12_0_init_microcode(psp: *mut psp_context) -> i32 {
    let adev = (*psp).adev;
    let mut ucode_prefix = [0i8; 30];
    let mut err: i32 = 0;
    DRM_DEBUG!("\n");

    amdgpu_ucode_ip_version_decode(adev, MP0_HWIP, ucode_prefix.as_mut_ptr(), ucode_prefix.len());

    err = psp_init_asd_microcode(psp, ucode_prefix.as_mut_ptr());
    if err != 0 { return err; }

    err = psp_init_ta_microcode(psp, ucode_prefix.as_mut_ptr());
    if err != 0 { return err; }

    /* only supported on renoir */
    if ((*adev).apu_flags & AMD_APU_IS_RENOIR) == 0 {
        (*psp).securedisplay_context.context.bin_desc.size_bytes = 0;
    }

    0
}

unsafe fn psp_v12_0_bootloader_load_sysdrv(psp: *mut psp_context) -> i32 {
    let mut ret: i32;
    let mut psp_gfxdrv_command_reg: u32 = 0;
    let adev = (*psp).adev;
    let sol_reg: u32;

    /* Check sOS sign of life register to confirm sys driver and sOS
     * are already been loaded.
     */
    sol_reg = RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_81);
    if sol_reg != 0 { return 0; }

    /* Wait for bootloader to signify that is ready having bit 31 of C2PMSG_35 set to 1 */
    ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_35), 0x80000000, 0x80000000, 0);
    if ret != 0 { return ret; }

    /* Copy PSP System Driver binary to memory */
    ret = psp_copy_fw(psp, (*psp).sys.start_addr, (*psp).sys.size_bytes);
    if ret != 0 { return ret; }

    /* Provide the sys driver to bootloader */
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_36, ((*psp).fw_pri_mc_addr >> 20) as u32);
    psp_gfxdrv_command_reg = 1 << 16;
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_35, psp_gfxdrv_command_reg);

    ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_35), 0x80000000, 0x80000000, 0);
    ret
}

unsafe fn psp_v12_0_bootloader_load_sos(psp: *mut psp_context) -> i32 {
    let mut ret: i32;
    let mut psp_gfxdrv_command_reg: u32 = 0;
    let adev = (*psp).adev;
    let sol_reg: u32;

    /* Check sOS sign of life register to confirm sys driver and sOS
     * are already been loaded.
     */
    sol_reg = RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_81);
    if sol_reg != 0 { return 0; }

    /* Wait for bootloader to signify that is ready having bit 31 of C2PMSG_35 set to 1 */
    ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_35), 0x80000000, 0x80000000, 0);
    if ret != 0 { return ret; }

    /* Copy Secure OS binary to PSP memory */
    ret = psp_copy_fw(psp, (*psp).sos.start_addr, (*psp).sos.size_bytes);
    if ret != 0 { return ret; }

    /* Provide the PSP secure OS to bootloader */
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_36, ((*psp).fw_pri_mc_addr >> 20) as u32);
    psp_gfxdrv_command_reg = 2 << 16;
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_35, psp_gfxdrv_command_reg);

    ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_81), RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_81), 0, PSP_WAITREG_CHANGED);
    ret
}

unsafe fn psp_v12_0_ring_create(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let mut ret: i32 = 0;
    let mut psp_ring_reg: u32 = 0;
    let ring = &mut (*psp).km_ring;
    let _adev = (*psp).adev;

    psp_ring_reg = lower_32_bits(ring.ring_mem_mc_addr);
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_69, psp_ring_reg);
    psp_ring_reg = upper_32_bits(ring.ring_mem_mc_addr);
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_70, psp_ring_reg);
    psp_ring_reg = ring.ring_size;
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_71, psp_ring_reg);
    psp_ring_reg = (ring_type as u32) << 16;
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_64, psp_ring_reg);

    ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0);
    ret
}

unsafe fn psp_v12_0_ring_stop(psp: *mut psp_context, _ring_type: psp_ring_type) -> i32 {
    let mut ret: i32 = 0;
    let adev = (*psp).adev;

    if amdgpu_sriov_vf(adev) {
        WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_DESTROY_GPCOM_RING);
    } else {
        WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_64, GFX_CTRL_CMD_ID_DESTROY_RINGS);
    }

    if amdgpu_sriov_vf(adev) {
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_101), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0);
    } else {
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0);
    }
    ret
}

unsafe fn psp_v12_0_ring_destroy(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let ring = &mut (*psp).km_ring;
    let adev = (*psp).adev;
    let ret = psp_v12_0_ring_stop(psp, ring_type);
    if ret != 0 { DRM_ERROR!("Fail to stop psp ring\n"); }
    amdgpu_bo_free_kernel(&mut (*adev).firmware.rbuf, &mut ring.ring_mem_mc_addr, &mut ring.ring_mem as *mut _ as *mut *mut core::ffi::c_void);
    ret
}

unsafe fn psp_v12_0_mode1_reset(psp: *mut psp_context) -> i32 {
    let mut ret: i32;
    let offset = SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64);
    let adev = (*psp).adev;
    ret = psp_wait_for(psp, offset, MBOX_TOS_READY_FLAG, MBOX_TOS_READY_MASK, 0);
    if ret != 0 { drm_info!(adev_to_drm(adev), "psp is not working correctly before mode1 reset!\n"); return -EINVAL; }
    WREG32!(offset, GFX_CTRL_CMD_ID_MODE1_RST);
    msleep(500);
    let offset = SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_33);
    ret = psp_wait_for(psp, offset, MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0);
    if ret != 0 { drm_info!(adev_to_drm(adev), "psp mode 1 reset failed!\n"); return -EINVAL; }
    drm_info!(adev_to_drm(adev), "psp mode1 reset succeed\n");
    0
}

unsafe fn psp_v12_0_ring_get_wptr(psp: *mut psp_context) -> u32 {
    let adev = (*psp).adev;
    if amdgpu_sriov_vf(adev) { RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_102) } else { RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_67) }
}

unsafe fn psp_v12_0_ring_set_wptr(psp: *mut psp_context, value: u32) {
    let adev = (*psp).adev;
    if amdgpu_sriov_vf(adev) {
        WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_102, value);
        WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_CONSUME_CMD);
    } else { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_67, value); }
}

static PSP_V12_0_FUNCS: psp_funcs = psp_funcs {
    init_microcode: Some(psp_v12_0_init_microcode),
    bootloader_load_sysdrv: Some(psp_v12_0_bootloader_load_sysdrv),
    bootloader_load_sos: Some(psp_v12_0_bootloader_load_sos),
    ring_create: Some(psp_v12_0_ring_create),
    ring_stop: Some(psp_v12_0_ring_stop),
    ring_destroy: Some(psp_v12_0_ring_destroy),
    mode1_reset: Some(psp_v12_0_mode1_reset),
    ring_get_wptr: Some(psp_v12_0_ring_get_wptr),
    ring_set_wptr: Some(psp_v12_0_ring_set_wptr),
};

unsafe fn psp_v12_0_set_psp_funcs(psp: *mut psp_context) {
    (*psp).funcs = &PSP_V12_0_FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
