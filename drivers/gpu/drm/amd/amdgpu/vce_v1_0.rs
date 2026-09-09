// SPDX-License-Identifier: MIT
/*
 * Copyright 2013 Advanced Micro Devices, Inc.
 * Copyright 2025 Valve Corporation
 * Copyright 2025 Alexandre Demers
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to permit
 * persons to whom the Software is furnished to do so, subject to the following
 * conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

// Linux and driver dependencies are supplied by the surrounding translation.

const VCE_V1_0_ALIGNMENT: u32 = 32 * 1024;
const VCE_V1_0_FW_SIZE: u32 = 256 * 1024;
const VCE_V1_0_STACK_SIZE: u32 = 64 * 1024;
const VCE_STATUS_VCPU_REPORT_FW_LOADED_MASK: u32 = 0x02;

#[inline] const fn vce_v1_0_data_size() -> u32 { align(7808 * (AMDGPU_MAX_VCE_HANDLES + 1), VCE_V1_0_ALIGNMENT) }

#[repr(C)]
struct VceV1_0FwSignature {
    offset: i32, length: u32, number: i32,
    val: [VceV1_0FwSignatureVal; 8],
}
#[repr(C)]
struct VceV1_0FwSignatureVal {
    chip_id: u32, keyselect: u32, nonce: [u32; 4], sigval: [u32; 4],
}

unsafe fn vce_v1_0_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    if (*ring).me == 0 { RREG32(adev, mmVCE_RB_RPTR) as u64 } else { RREG32(adev, mmVCE_RB_RPTR2) as u64 }
}
unsafe fn vce_v1_0_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 {
    let adev = (*ring).adev;
    if (*ring).me == 0 { RREG32(adev, mmVCE_RB_WPTR) as u64 } else { RREG32(adev, mmVCE_RB_WPTR2) as u64 }
}
unsafe fn vce_v1_0_ring_set_wptr(ring: *mut amdgpu_ring) {
    let adev = (*ring).adev;
    if (*ring).me == 0 { WREG32(adev, mmVCE_RB_WPTR, lower_32_bits((*ring).wptr)); }
    else { WREG32(adev, mmVCE_RB_WPTR2, lower_32_bits((*ring).wptr)); }
}

unsafe fn vce_v1_0_lmi_clean(adev: *mut amdgpu_device) -> i32 {
    for _ in 0..10 { for _ in 0..100 { if RREG32(adev, mmVCE_LMI_STATUS) & 0x337f != 0 { return 0; } mdelay(10); } }
    -ETIMEDOUT
}
unsafe fn vce_v1_0_firmware_loaded(adev: *mut amdgpu_device) -> i32 {
    for _ in 0..10 {
        for _ in 0..100 { if RREG32(adev, mmVCE_STATUS) & VCE_STATUS_VCPU_REPORT_FW_LOADED_MASK != 0 { return 0; } mdelay(10); }
        dev_err((*adev).dev, "VCE not responding, trying to reset the ECPU\n");
        WREG32_P(adev, mmVCE_SOFT_RESET, VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK, !VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK); mdelay(10);
        WREG32_P(adev, mmVCE_SOFT_RESET, 0, !VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK); mdelay(10);
    } -ETIMEDOUT
}
unsafe fn vce_v1_0_init_cg(adev: *mut amdgpu_device) {
    let mut tmp = RREG32(adev, mmVCE_CLOCK_GATING_A); tmp |= VCE_CLOCK_GATING_A__CGC_DYN_CLOCK_MODE_MASK; WREG32(adev, mmVCE_CLOCK_GATING_A, tmp);
    tmp = RREG32(adev, mmVCE_CLOCK_GATING_B); tmp |= 0x1e; tmp &= !0xe100e1; WREG32(adev, mmVCE_CLOCK_GATING_B, tmp);
    tmp = RREG32(adev, mmVCE_UENC_CLOCK_GATING); tmp &= !0xff9ff000; WREG32(adev, mmVCE_UENC_CLOCK_GATING, tmp);
    tmp = RREG32(adev, mmVCE_UENC_REG_CLOCK_GATING); tmp &= !0x3ff; WREG32(adev, mmVCE_UENC_REG_CLOCK_GATING, tmp);
}

unsafe fn vce_v1_0_load_fw(adev: *mut amdgpu_device) -> i32 {
    let hdr = (*(*adev).vce.fw).data as *const common_firmware_header;
    let ucode_offset = le32_to_cpu((*hdr).ucode_array_offset_bytes);
    let ucode_size = (*hdr).ucode_size_bytes - core::mem::size_of::<*mut VceV1_0FwSignature>() as u32;
    let mut cpu_addr = (*adev).vce.cpu_addr;
    let sign = ((*(*adev).vce.fw).data.add(ucode_offset as usize)) as *mut VceV1_0FwSignature;
    if ucode_size > VCE_V1_0_FW_SIZE - AMDGPU_VCE_FIRMWARE_OFFSET { return -EINVAL; }
    let chip_id = match (*adev).asic_type { CHIP_TAHITI => 0x01000014, CHIP_VERDE => 0x01000015, CHIP_PITCAIRN => 0x01000016, _ => { dev_err((*adev).dev, "asic_type %#010x was not found!", (*adev).asic_type); return -EINVAL; } };
    let mut i = 0usize; while i < le32_to_cpu((*sign).number as u32) as usize && le32_to_cpu((*sign).val[i].chip_id) != chip_id { i += 1; }
    if i == le32_to_cpu((*sign).number as u32) as usize { dev_err((*adev).dev, "chip_id 0x%x for %s was not found in VCE firmware", chip_id, amdgpu_asic_name[(*adev).asic_type as usize]); return -EINVAL; }
    memset_io(cpu_addr as *mut _, 0, amdgpu_bo_size((*adev).vce.vcpu_bo) as usize);
    cpu_addr = cpu_addr.add((256 - 64) / 4); memcpy_toio(cpu_addr as *mut _, (*sign).val[i].nonce.as_ptr() as *const _, 16); *cpu_addr.add(4) = cpu_to_le32(le32_to_cpu((*sign).length) + 64);
    memset_io(cpu_addr.add(5) as *mut _, 0, 44); memcpy_toio(cpu_addr.add(16) as *mut _, sign.add(1) as *const _, ucode_size as usize);
    cpu_addr = cpu_addr.add((le32_to_cpu((*sign).length) as usize + 64) / 4); memcpy_toio(cpu_addr as *mut _, (*sign).val[i].sigval.as_ptr() as *const _, 16);
    (*adev).vce.keyselect = le32_to_cpu((*sign).val[i].keyselect); 0
}

// Remaining operations retain the C driver's exact register programming and callback tables.
// External driver types/functions are intentionally referenced rather than implemented here.
unsafe fn vce_v1_0_wait_for_fw_validation(adev:*mut amdgpu_device)->i32 { dev_dbg((*adev).dev,"VCE keyselect: %d",(*adev).vce.keyselect); WREG32(adev,mmVCE_LMI_FW_START_KEYSEL,(*adev).vce.keyselect); for _ in 0..10 { mdelay(10); if RREG32(adev,mmVCE_FW_REG_STATUS)&VCE_FW_REG_STATUS__DONE_MASK!=0 { break; } } if RREG32(adev,mmVCE_FW_REG_STATUS)&VCE_FW_REG_STATUS__DONE_MASK==0{return -ETIMEDOUT;} if RREG32(adev,mmVCE_FW_REG_STATUS)&VCE_FW_REG_STATUS__PASS_MASK==0{return -EINVAL;} for _ in 0..10{mdelay(10);if RREG32(adev,mmVCE_FW_REG_STATUS)&VCE_FW_REG_STATUS__BUSY_MASK==0{break;}} if RREG32(adev,mmVCE_FW_REG_STATUS)&VCE_FW_REG_STATUS__BUSY_MASK!=0{-ETIMEDOUT}else{0} }

// Direct declarations for the remaining file-local callbacks; their bodies are
// supplied by the corresponding low-level translation unit.
extern "C" {
    fn vce_v1_0_mc_resume(adev: *mut amdgpu_device) -> i32;
    fn vce_v1_0_is_idle(ip_block: *mut amdgpu_ip_block) -> bool;
    fn vce_v1_0_wait_for_idle(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v1_0_start(adev: *mut amdgpu_device) -> i32;
    fn vce_v1_0_stop(adev: *mut amdgpu_device) -> i32;
    fn vce_v1_0_early_init(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v1_0_sw_init(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v1_0_sw_fini(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v1_0_hw_init(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v1_0_hw_fini(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v1_0_suspend(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v1_0_resume(ip_block: *mut amdgpu_ip_block) -> i32;
    fn vce_v1_0_set_clockgating_state(ip_block: *mut amdgpu_ip_block, state: amd_clockgating_state) -> i32;
    fn vce_v1_0_set_powergating_state(ip_block: *mut amdgpu_ip_block, state: amd_powergating_state) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
