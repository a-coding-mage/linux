/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

unsafe fn psp_v11_0_8_ring_stop(
    psp: *mut psp_context,
    ring_type: psp_ring_type,
) -> i32 {
    let mut ret: i32 = 0;
    let adev = unsafe { (*psp).adev };

    if unsafe { amdgpu_sriov_vf(adev) } {
        // Write the ring destroy command
        unsafe { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_DESTROY_GPCOM_RING); }
        // there might be handshake issue with hardware which needs delay
        unsafe { mdelay(20); }
        // Wait for response flag (bit 31)
        ret = unsafe { psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_101), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0) };
    } else {
        // Write the ring destroy command
        unsafe { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_64, GFX_CTRL_CMD_ID_DESTROY_RINGS); }
        // there might be handshake issue with hardware which needs delay
        unsafe { mdelay(20); }
        // Wait for response flag (bit 31)
        ret = unsafe { psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0) };
    }
    ret
}

unsafe fn psp_v11_0_8_ring_create(
    psp: *mut psp_context,
    ring_type: psp_ring_type,
) -> i32 {
    let mut ret: i32 = 0;
    let mut psp_ring_reg: u32 = 0;
    let ring = unsafe { &mut (*psp).km_ring };
    let adev = unsafe { (*psp).adev };

    if unsafe { amdgpu_sriov_vf(adev) } {
        ret = psp_v11_0_8_ring_stop(psp, ring_type);
        if ret != 0 { unsafe { DRM_ERROR!("psp_v11_0_8_ring_stop_sriov failed!\n"); } return ret; }
        psp_ring_reg = lower_32_bits(ring.ring_mem_mc_addr);
        unsafe { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_102, psp_ring_reg); }
        psp_ring_reg = upper_32_bits(ring.ring_mem_mc_addr);
        unsafe { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_103, psp_ring_reg); }
        unsafe { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_INIT_GPCOM_RING); }
        unsafe { mdelay(20); }
        ret = unsafe { psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_101), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0) };
    } else {
        ret = unsafe { psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), MBOX_TOS_READY_FLAG, MBOX_TOS_READY_MASK, 0) };
        if ret != 0 { unsafe { DRM_ERROR!("Failed to wait for trust OS ready for ring creation\n"); } return ret; }
        psp_ring_reg = lower_32_bits(ring.ring_mem_mc_addr);
        unsafe { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_69, psp_ring_reg); }
        psp_ring_reg = upper_32_bits(ring.ring_mem_mc_addr);
        unsafe { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_70, psp_ring_reg); }
        psp_ring_reg = ring.ring_size;
        unsafe { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_71, psp_ring_reg); }
        psp_ring_reg = (ring_type as u32) << 16;
        unsafe { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_64, psp_ring_reg); }
        unsafe { mdelay(20); }
        ret = unsafe { psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0) };
    }
    ret
}

unsafe fn psp_v11_0_8_ring_destroy(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let mut ret = psp_v11_0_8_ring_stop(psp, ring_type);
    let ring = unsafe { &mut (*psp).km_ring };
    let adev = unsafe { (*psp).adev };
    if ret != 0 { unsafe { DRM_ERROR!("Fail to stop psp ring\n"); } }
    unsafe { amdgpu_bo_free_kernel(&mut (*adev).firmware.rbuf, &mut ring.ring_mem_mc_addr, &mut ring.ring_mem as *mut _ as *mut *mut core::ffi::c_void); }
    ret
}

unsafe fn psp_v11_0_8_ring_get_wptr(psp: *mut psp_context) -> u32 {
    let adev = unsafe { (*psp).adev };
    if unsafe { amdgpu_sriov_vf(adev) } { unsafe { RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_102) } } else { unsafe { RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_67) } }
}

unsafe fn psp_v11_0_8_ring_set_wptr(psp: *mut psp_context, value: u32) {
    let adev = unsafe { (*psp).adev };
    if unsafe { amdgpu_sriov_vf(adev) } {
        unsafe { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_102, value); WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_CONSUME_CMD); }
    } else { unsafe { WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_67, value); } }
}

static psp_v11_0_8_funcs: psp_funcs = psp_funcs {
    ring_create: Some(psp_v11_0_8_ring_create),
    ring_stop: Some(psp_v11_0_8_ring_stop),
    ring_destroy: Some(psp_v11_0_8_ring_destroy),
    ring_get_wptr: Some(psp_v11_0_8_ring_get_wptr),
    ring_set_wptr: Some(psp_v11_0_8_ring_set_wptr),
};

pub unsafe fn psp_v11_0_8_set_psp_funcs(psp: *mut psp_context) {
    unsafe { (*psp).funcs = &psp_v11_0_8_funcs; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
