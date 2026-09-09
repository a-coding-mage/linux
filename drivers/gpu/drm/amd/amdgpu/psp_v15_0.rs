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

// C headers and firmware declarations are supplied by the surrounding kernel bindings.

const REG_MPASP_PCRU0_MPASP_C2PMSG_64: u32 = 0x4280;
const REG_MPASP_PCRU0_MPASP_C2PMSG_64_BASE_IDX: u32 = 2;
const REG_MPASP_PCRU0_MPASP_C2PMSG_67: u32 = 0x4283;
const REG_MPASP_PCRU0_MPASP_C2PMSG_67_BASE_IDX: u32 = 2;
const REG_MPASP_PCRU0_MPASP_C2PMSG_69: u32 = 0x4285;
const REG_MPASP_PCRU0_MPASP_C2PMSG_69_BASE_IDX: u32 = 2;
const REG_MPASP_PCRU0_MPASP_C2PMSG_70: u32 = 0x4286;
const REG_MPASP_PCRU0_MPASP_C2PMSG_70_BASE_IDX: u32 = 2;
const REG_MPASP_PCRU0_MPASP_C2PMSG_71: u32 = 0x4287;
const REG_MPASP_PCRU0_MPASP_C2PMSG_71_BASE_IDX: u32 = 2;

unsafe fn psp_v15_0_0_init_microcode(psp: *mut psp_context) -> i32 {
    let adev = (*psp).adev;
    let mut ucode_prefix = [0i8; 30];
    let mut err: i32 = 0;

    amdgpu_ucode_ip_version_decode(adev, MP0_HWIP, ucode_prefix.as_mut_ptr(), ucode_prefix.len());

    err = psp_init_toc_microcode(psp, ucode_prefix.as_mut_ptr());
    if err != 0 {
        return err;
    }

    err = psp_init_ta_microcode(psp, ucode_prefix.as_mut_ptr());
    if err != 0 {
        return err;
    }

    0
}

unsafe fn psp_v15_0_0_ring_stop(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let mut ret: i32 = 0;
    let adev = (*psp).adev;

    if amdgpu_sriov_vf(adev) {
        WREG32_SOC15!(MP0, 0, regMPASP_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_DESTROY_GPCOM_RING);
        mdelay(20);
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, regMPASP_SMN_C2PMSG_101), 0x80000000, 0x80000000, false);
    } else if amdgpu_ip_version(adev, MP0_HWIP, 0) == IP_VERSION!(15, 0, 5) {
        WREG32_SOC15!(MP0, 0, REG_MPASP_PCRU0_MPASP_C2PMSG_64, GFX_CTRL_CMD_ID_DESTROY_RINGS);
        mdelay(20);
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, REG_MPASP_PCRU0_MPASP_C2PMSG_64), 0x80000000, 0x80000000, false);
    } else {
        WREG32_SOC15!(MP0, 0, regMPASP_PCRU1_MPASP_C2PMSG_64, GFX_CTRL_CMD_ID_DESTROY_RINGS);
        mdelay(20);
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, regMPASP_PCRU1_MPASP_C2PMSG_64), 0x80000000, 0x80000000, false);
    }
    ret
}

unsafe fn psp_v15_0_0_ring_create(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let mut ret: i32 = 0;
    let mut psp_ring_reg: u32 = 0;
    let ring = &mut (*psp).km_ring;
    let adev = (*psp).adev;

    if amdgpu_sriov_vf(adev) {
        ret = psp_v15_0_0_ring_stop(psp, ring_type);
        if ret != 0 { DRM_ERROR!("psp_v14_0_ring_stop_sriov failed!\n"); return ret; }
        psp_ring_reg = lower_32_bits(ring.ring_mem_mc_addr);
        WREG32_SOC15!(MP0, 0, regMPASP_SMN_C2PMSG_102, psp_ring_reg);
        psp_ring_reg = upper_32_bits(ring.ring_mem_mc_addr);
        WREG32_SOC15!(MP0, 0, regMPASP_SMN_C2PMSG_103, psp_ring_reg);
        WREG32_SOC15!(MP0, 0, regMPASP_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_INIT_GPCOM_RING);
        mdelay(20);
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, regMPASP_SMN_C2PMSG_101), 0x80000000, 0x8000FFFF, false);
    } else if amdgpu_ip_version(adev, MP0_HWIP, 0) == IP_VERSION!(15, 0, 5) {
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, REG_MPASP_PCRU0_MPASP_C2PMSG_64), 0x80000000, 0x80000000, false);
        if ret != 0 { DRM_ERROR!("Failed to wait for trust OS ready for ring creation\n"); return ret; }
        psp_ring_reg = lower_32_bits(ring.ring_mem_mc_addr); WREG32_SOC15!(MP0, 0, REG_MPASP_PCRU0_MPASP_C2PMSG_69, psp_ring_reg);
        psp_ring_reg = upper_32_bits(ring.ring_mem_mc_addr); WREG32_SOC15!(MP0, 0, REG_MPASP_PCRU0_MPASP_C2PMSG_70, psp_ring_reg);
        psp_ring_reg = ring.ring_size; WREG32_SOC15!(MP0, 0, REG_MPASP_PCRU0_MPASP_C2PMSG_71, psp_ring_reg);
        psp_ring_reg = (ring_type as u32) << 16; WREG32_SOC15!(MP0, 0, REG_MPASP_PCRU0_MPASP_C2PMSG_64, psp_ring_reg);
        mdelay(20);
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, REG_MPASP_PCRU0_MPASP_C2PMSG_64), 0x80000000, 0x8000FFFF, false);
    } else {
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, regMPASP_PCRU1_MPASP_C2PMSG_64), 0x80000000, 0x80000000, false);
        if ret != 0 { DRM_ERROR!("Failed to wait for trust OS ready for ring creation\n"); return ret; }
        psp_ring_reg = lower_32_bits(ring.ring_mem_mc_addr); WREG32_SOC15!(MP0, 0, regMPASP_PCRU1_MPASP_C2PMSG_69, psp_ring_reg);
        psp_ring_reg = upper_32_bits(ring.ring_mem_mc_addr); WREG32_SOC15!(MP0, 0, regMPASP_PCRU1_MPASP_C2PMSG_70, psp_ring_reg);
        psp_ring_reg = ring.ring_size; WREG32_SOC15!(MP0, 0, regMPASP_PCRU1_MPASP_C2PMSG_71, psp_ring_reg);
        psp_ring_reg = (ring_type as u32) << 16; WREG32_SOC15!(MP0, 0, regMPASP_PCRU1_MPASP_C2PMSG_64, psp_ring_reg);
        mdelay(20);
        ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, regMPASP_PCRU1_MPASP_C2PMSG_64), 0x80000000, 0x8000FFFF, false);
    }
    ret
}

unsafe fn psp_v15_0_0_ring_destroy(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let ring = &mut (*psp).km_ring;
    let adev = (*psp).adev;
    let ret = psp_v15_0_0_ring_stop(psp, ring_type);
    if ret != 0 { DRM_ERROR!("Fail to stop psp ring\n"); }
    amdgpu_bo_free_kernel(&mut (*adev).firmware.rbuf, &mut ring.ring_mem_mc_addr, &mut ring.ring_mem as *mut _ as *mut *mut core::ffi::c_void);
    ret
}

unsafe fn psp_v15_0_0_ring_get_wptr(psp: *mut psp_context) -> u32 {
    let adev = (*psp).adev;
    if amdgpu_sriov_vf(adev) { RREG32_SOC15!(MP0, 0, regMPASP_SMN_C2PMSG_102) }
    else if amdgpu_ip_version(adev, MP0_HWIP, 0) == IP_VERSION!(15, 0, 5) { RREG32_SOC15!(MP0, 0, REG_MPASP_PCRU0_MPASP_C2PMSG_67) }
    else { RREG32_SOC15!(MP0, 0, regMPASP_PCRU1_MPASP_C2PMSG_67) }
}

unsafe fn psp_v15_0_0_ring_set_wptr(psp: *mut psp_context, value: u32) {
    let adev = (*psp).adev;
    if amdgpu_sriov_vf(adev) {
        WREG32_SOC15!(MP0, 0, regMPASP_SMN_C2PMSG_102, value);
        WREG32_SOC15!(MP0, 0, regMPASP_SMN_C2PMSG_101, GFX_CTRL_CMD_ID_CONSUME_CMD);
    } else if amdgpu_ip_version(adev, MP0_HWIP, 0) == IP_VERSION!(15, 0, 5) {
        WREG32_SOC15!(MP0, 0, REG_MPASP_PCRU0_MPASP_C2PMSG_67, value);
    } else {
        WREG32_SOC15!(MP0, 0, regMPASP_PCRU1_MPASP_C2PMSG_67, value);
    }
}

static PSP_V15_0_0_FUNCS: psp_funcs = psp_funcs {
    init_microcode: Some(psp_v15_0_0_init_microcode),
    ring_create: Some(psp_v15_0_0_ring_create),
    ring_stop: Some(psp_v15_0_0_ring_stop),
    ring_destroy: Some(psp_v15_0_0_ring_destroy),
    ring_get_wptr: Some(psp_v15_0_0_ring_get_wptr),
    ring_set_wptr: Some(psp_v15_0_0_ring_set_wptr),
};

pub unsafe fn psp_v15_0_0_set_psp_funcs(psp: *mut psp_context) {
    (*psp).funcs = &PSP_V15_0_0_FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
