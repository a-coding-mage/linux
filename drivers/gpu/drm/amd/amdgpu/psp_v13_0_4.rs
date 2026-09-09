/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// External kernel types, functions, constants, and register macros are supplied by dependencies.

unsafe fn psp_v13_0_4_init_microcode(psp: *mut psp_context) -> i32 {
    let adev = (*psp).adev;
    let mut ucode_prefix = [0i8; 30];
    let mut err: i32 = 0;

    amdgpu_ucode_ip_version_decode(adev, MP0_HWIP, ucode_prefix.as_mut_ptr(), ucode_prefix.len());

    match amdgpu_ip_version(adev, MP0_HWIP, 0) {
        IP_VERSION(13, 0, 4) => {
            err = psp_init_toc_microcode(psp, ucode_prefix.as_mut_ptr());
            if err != 0 { return err; }
            err = psp_init_ta_microcode(psp, ucode_prefix.as_mut_ptr());
            if err != 0 { return err; }
        }
        _ => {
            dev_warn((*adev).dev, "Unsupported MP0 version 0x%08x\n", amdgpu_ip_version(adev, MP0_HWIP, 0));
            return -EINVAL;
        }
    }
    0
}

unsafe fn psp_v13_0_4_is_sos_alive(psp: *mut psp_context) -> bool {
    let adev = (*psp).adev;
    let sol_reg: u32 = RREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_81);
    sol_reg != 0
}

unsafe fn psp_v13_0_4_wait_for_bootloader(psp: *mut psp_context) -> i32 {
    let mut ret = 0;
    for _retry_loop in 0..10 {
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, regMP0_SMN_C2PMSG_35),
                           0x80000000, 0x80000000, PSP_WAITREG_NOVERBOSE);
        if ret == 0 { return 0; }
    }
    ret
}

unsafe fn psp_v13_0_4_bootloader_load_component(psp: *mut psp_context, bin_desc: *mut psp_bin_desc, bl_cmd: psp_bootloader_cmd) -> i32 {
    let mut psp_gfxdrv_command_reg: u32 = 0;
    if psp_v13_0_4_is_sos_alive(psp) { return 0; }
    let mut ret = psp_v13_0_4_wait_for_bootloader(psp);
    if ret != 0 { return ret; }
    ret = psp_copy_fw(psp, (*bin_desc).start_addr, (*bin_desc).size_bytes);
    if ret != 0 { return ret; }
    WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_36, ((*psp).fw_pri_mc_addr >> 20) as u32);
    psp_gfxdrv_command_reg = bl_cmd as u32;
    WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_35, psp_gfxdrv_command_reg);
    psp_v13_0_4_wait_for_bootloader(psp)
}

unsafe fn psp_v13_0_4_bootloader_load_kdb(psp: *mut psp_context) -> i32 { psp_v13_0_4_bootloader_load_component(psp, &mut (*psp).kdb, PSP_BL__LOAD_KEY_DATABASE) }
unsafe fn psp_v13_0_4_bootloader_load_spl(psp: *mut psp_context) -> i32 { psp_v13_0_4_bootloader_load_component(psp, &mut (*psp).kdb, PSP_BL__LOAD_TOS_SPL_TABLE) }
unsafe fn psp_v13_0_4_bootloader_load_sysdrv(psp: *mut psp_context) -> i32 { psp_v13_0_4_bootloader_load_component(psp, &mut (*psp).sys, PSP_BL__LOAD_SYSDRV) }
unsafe fn psp_v13_0_4_bootloader_load_soc_drv(psp: *mut psp_context) -> i32 { psp_v13_0_4_bootloader_load_component(psp, &mut (*psp).soc_drv, PSP_BL__LOAD_SOCDRV) }
unsafe fn psp_v13_0_4_bootloader_load_intf_drv(psp: *mut psp_context) -> i32 { psp_v13_0_4_bootloader_load_component(psp, &mut (*psp).intf_drv, PSP_BL__LOAD_INTFDRV) }
unsafe fn psp_v13_0_4_bootloader_load_dbg_drv(psp: *mut psp_context) -> i32 { psp_v13_0_4_bootloader_load_component(psp, &mut (*psp).dbg_drv, PSP_BL__LOAD_DBGDRV) }

unsafe fn psp_v13_0_4_bootloader_load_sos(psp: *mut psp_context) -> i32 {
    if psp_v13_0_4_is_sos_alive(psp) { return 0; }
    let mut ret = psp_v13_0_4_wait_for_bootloader(psp);
    if ret != 0 { return ret; }
    ret = psp_copy_fw(psp, (*psp).sos.start_addr, (*psp).sos.size_bytes);
    if ret != 0 { return ret; }
    WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_36, ((*psp).fw_pri_mc_addr >> 20) as u32);
    WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_35, PSP_BL__LOAD_SOSDRV);
    mdelay(20);
    ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, regMP0_SMN_C2PMSG_81),
                       RREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_81), 0, PSP_WAITREG_CHANGED);
    ret
}

unsafe fn psp_v13_0_4_ring_stop(psp: *mut psp_context, _ring_type: psp_ring_type) -> i32 {
    let adev = (*psp).adev;
    let (reg, cmd) = if amdgpu_sriov_vf(adev) { (regMP0_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_DESTROY_GPCOM_RING) } else { (regMP0_SMN_C2PMSG_64, GFX_CTRL_CMD_ID_DESTROY_RINGS) };
    WREG32_SOC15!(MP0, 0, reg, cmd); mdelay(20);
    psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, reg), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0)
}

unsafe fn psp_v13_0_4_ring_create(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let adev = (*psp).adev; let ring = &mut (*psp).km_ring;
    if amdgpu_sriov_vf(adev) {
        let mut ret = psp_v13_0_4_ring_stop(psp, ring_type); if ret != 0 { DRM_ERROR!("psp_v13_0_ring_stop_sriov failed!\n"); return ret; }
        WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_102, lower_32_bits(ring.ring_mem_mc_addr));
        WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_103, upper_32_bits(ring.ring_mem_mc_addr));
        WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_INIT_GPCOM_RING); mdelay(20);
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, regMP0_SMN_C2PMSG_101), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0); ret
    } else {
        let mut ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, regMP0_SMN_C2PMSG_64), MBOX_TOS_READY_FLAG, MBOX_TOS_READY_MASK, 0);
        if ret != 0 { DRM_ERROR!("Failed to wait for trust OS ready for ring creation\n"); return ret; }
        WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_69, lower_32_bits(ring.ring_mem_mc_addr));
        WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_70, upper_32_bits(ring.ring_mem_mc_addr));
        WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_71, ring.ring_size);
        WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_64, (ring_type as u32) << 16); mdelay(20);
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, regMP0_SMN_C2PMSG_64), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0); ret
    }
}

unsafe fn psp_v13_0_4_ring_destroy(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let ring = &mut (*psp).km_ring; let adev = (*psp).adev;
    let ret = psp_v13_0_4_ring_stop(psp, ring_type); if ret != 0 { DRM_ERROR!("Fail to stop psp ring\n"); }
    amdgpu_bo_free_kernel(&mut (*adev).firmware.rbuf, &mut ring.ring_mem_mc_addr, &mut ring.ring_mem as *mut _ as *mut *mut core::ffi::c_void); ret
}

unsafe fn psp_v13_0_4_ring_get_wptr(psp: *mut psp_context) -> u32 {
    if amdgpu_sriov_vf((*psp).adev) { RREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_102) } else { RREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_67) }
}

unsafe fn psp_v13_0_4_ring_set_wptr(psp: *mut psp_context, value: u32) {
    if amdgpu_sriov_vf((*psp).adev) { WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_102, value); WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_CONSUME_CMD); } else { WREG32_SOC15!(MP0, 0, regMP0_SMN_C2PMSG_67, value); }
}

static psp_v13_0_4_funcs: psp_funcs = psp_funcs {
    init_microcode: Some(psp_v13_0_4_init_microcode), bootloader_load_kdb: Some(psp_v13_0_4_bootloader_load_kdb), bootloader_load_spl: Some(psp_v13_0_4_bootloader_load_spl), bootloader_load_sysdrv: Some(psp_v13_0_4_bootloader_load_sysdrv), bootloader_load_soc_drv: Some(psp_v13_0_4_bootloader_load_soc_drv), bootloader_load_intf_drv: Some(psp_v13_0_4_bootloader_load_intf_drv), bootloader_load_dbg_drv: Some(psp_v13_0_4_bootloader_load_dbg_drv), bootloader_load_sos: Some(psp_v13_0_4_bootloader_load_sos), ring_create: Some(psp_v13_0_4_ring_create), ring_stop: Some(psp_v13_0_4_ring_stop), ring_destroy: Some(psp_v13_0_4_ring_destroy), ring_get_wptr: Some(psp_v13_0_4_ring_get_wptr), ring_set_wptr: Some(psp_v13_0_4_ring_set_wptr),
};

unsafe fn psp_v13_0_4_set_psp_funcs(psp: *mut psp_context) { (*psp).funcs = &psp_v13_0_4_funcs; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
