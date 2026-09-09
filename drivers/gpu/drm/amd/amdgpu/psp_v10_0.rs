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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Author: Huang Rui
 */

// Linux and AMDGPU headers provide the types, constants, register helpers,
// macros, and external functions referenced by this implementation.

extern "C" {
    fn amdgpu_ucode_ip_version_decode(adev: *mut amdgpu_device, hwip: u32, prefix: *mut i8, size: usize);
    fn psp_init_asd_microcode(psp: *mut psp_context, prefix: *const i8) -> i32;
    fn psp_init_ta_microcode(psp: *mut psp_context, prefix: *const i8) -> i32;
    fn psp_wait_for(psp: *mut psp_context, reg: u32, mask: u32, field: u32, val: u32) -> i32;
    fn amdgpu_bo_free_kernel(bo: *mut *mut core::ffi::c_void, mc_addr: *mut u64, cpu_addr: *mut *mut core::ffi::c_void);
}

unsafe fn psp_v10_0_init_microcode(psp: *mut psp_context) -> i32 {
    let adev = (*psp).adev;
    let mut ucode_prefix = [0i8; 30];
    let mut err: i32 = 0;

    DRM_DEBUG!("\n");
    amdgpu_ucode_ip_version_decode(adev, MP0_HWIP, ucode_prefix.as_mut_ptr(), ucode_prefix.len());

    err = psp_init_asd_microcode(psp, ucode_prefix.as_ptr());
    if err != 0 {
        return err;
    }

    err = psp_init_ta_microcode(psp, ucode_prefix.as_ptr());
    if amdgpu_ip_version(adev, GC_HWIP, 0) == IP_VERSION(9, 1, 0)
        && (*(*adev).pdev).revision == 0xa1
        && (*psp).securedisplay_context.context.bin_desc.fw_version >= 0x27000008
    {
        (*adev).psp.securedisplay_context.context.bin_desc.size_bytes = 0;
    }
    err
}

unsafe fn psp_v10_0_ring_create(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let mut ret: i32 = 0;
    let mut psp_ring_reg: u32 = 0;
    let ring = &mut (*psp).km_ring;
    let adev = (*psp).adev;

    // Write low address of the ring to C2PMSG_69
    psp_ring_reg = lower_32_bits(ring.ring_mem_mc_addr);
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_69, psp_ring_reg);
    // Write high address of the ring to C2PMSG_70
    psp_ring_reg = upper_32_bits(ring.ring_mem_mc_addr);
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_70, psp_ring_reg);
    // Write size of ring to C2PMSG_71
    psp_ring_reg = ring.ring_size;
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_71, psp_ring_reg);
    // Write the ring initialization command to C2PMSG_64
    psp_ring_reg = ring_type as u32;
    psp_ring_reg <<= 16;
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_64, psp_ring_reg);

    // There might be handshake issue with hardware which needs delay
    mdelay(20);
    // Wait for response flag (bit 31) in C2PMSG_64
    ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0);
    ret
}

unsafe fn psp_v10_0_ring_stop(psp: *mut psp_context, _ring_type: psp_ring_type) -> i32 {
    let mut ret: i32 = 0;
    let _adev = (*psp).adev;

    // Write the ring destroy command to C2PMSG_64
    let psp_ring_reg: u32 = 3 << 16;
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_64, psp_ring_reg);
    // There might be handshake issue with hardware which needs delay
    mdelay(20);
    // Wait for response flag (bit 31) in C2PMSG_64
    ret = psp_wait_for(psp, SOC15_REG_OFFSET!(MP0, 0, mmMP0_SMN_C2PMSG_64), MBOX_TOS_RESP_FLAG, MBOX_TOS_RESP_MASK, 0);
    ret
}

unsafe fn psp_v10_0_ring_destroy(psp: *mut psp_context, ring_type: psp_ring_type) -> i32 {
    let ring = &mut (*psp).km_ring;
    let adev = (*psp).adev;
    let ret = psp_v10_0_ring_stop(psp, ring_type);
    if ret != 0 {
        DRM_ERROR!("Fail to stop psp ring\n");
    }
    amdgpu_bo_free_kernel(&mut (*adev).firmware.rbuf, &mut ring.ring_mem_mc_addr, &mut ring.ring_mem as *mut _ as *mut *mut core::ffi::c_void);
    ret
}

unsafe fn psp_v10_0_mode1_reset(psp: *mut psp_context) -> i32 {
    drm_info!(adev_to_drm((*psp).adev), "psp mode 1 reset not supported now!\n");
    -EINVAL
}

unsafe fn psp_v10_0_ring_get_wptr(psp: *mut psp_context) -> u32 {
    let _adev = (*psp).adev;
    RREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_67)
}

unsafe fn psp_v10_0_ring_set_wptr(psp: *mut psp_context, value: u32) {
    let _adev = (*psp).adev;
    WREG32_SOC15!(MP0, 0, mmMP0_SMN_C2PMSG_67, value);
}

static PSP_V10_0_FUNCS: psp_funcs = psp_funcs {
    init_microcode: Some(psp_v10_0_init_microcode),
    ring_create: Some(psp_v10_0_ring_create),
    ring_stop: Some(psp_v10_0_ring_stop),
    ring_destroy: Some(psp_v10_0_ring_destroy),
    mode1_reset: Some(psp_v10_0_mode1_reset),
    ring_get_wptr: Some(psp_v10_0_ring_get_wptr),
    ring_set_wptr: Some(psp_v10_0_ring_set_wptr),
};

pub unsafe fn psp_v10_0_set_psp_funcs(psp: *mut psp_context) {
    (*psp).funcs = &PSP_V10_0_FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
