/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

const REGMMVM_L2_CNTL3_DEFAULT: u32 = 0x80100007;
const REGMMVM_L2_CNTL4_DEFAULT: u32 = 0x000000c1;
const REGMMVM_L2_CNTL5_DEFAULT: u32 = 0x00003fe0;

static MMHUB_CLIENT_IDS_V3_0_2: [[*const core::ffi::c_char; 2]; 53] = [[core::ptr::null(); 2]; 53];

unsafe fn mmhub_v3_0_2_get_invalidate_req(vmid: u32, flush_type: u32) -> u32 {
    let mut req: u32 = 0;
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, PER_VMID_INVALIDATE_REQ, 1u32 << vmid);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, FLUSH_TYPE, flush_type);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PTES, 1);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE0, 1);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE1, 1);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE2, 1);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L1_PTES, 1);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, CLEAR_PROTECTION_FAULT_STATUS_ADDR, 0);
    req
}

unsafe fn mmhub_v3_0_2_print_l2_protection_fault_status(adev: *mut amdgpu_device, status: u32) {
    let cid = REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, CID);
    let rw = REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, RW);
    dev_err!((*adev).dev, "MMVM_L2_PROTECTION_FAULT_STATUS:0x%08X\n", status);
    let mmhub_cid = amdgpu_mmhub_client_name!(&mut (*adev).mmhub, cid, rw);
    dev_err!((*adev).dev, "\t Faulty UTCL2 client ID: %s (0x%x)\n", if !mmhub_cid.is_null() { mmhub_cid } else { c"unknown".as_ptr() }, cid);
    dev_err!((*adev).dev, "\t MORE_FAULTS: 0x%lx\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, MORE_FAULTS));
    dev_err!((*adev).dev, "\t WALKER_ERROR: 0x%lx\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, WALKER_ERROR));
    dev_err!((*adev).dev, "\t PERMISSION_FAULTS: 0x%lx\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, PERMISSION_FAULTS));
    dev_err!((*adev).dev, "\t MAPPING_ERROR: 0x%lx\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, MAPPING_ERROR));
    dev_err!((*adev).dev, "\t RW: 0x%x\n", rw);
}

unsafe fn mmhub_v3_0_2_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0!(0)];
    WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32, hub.ctx_addr_distance * vmid, lower_32_bits!(page_table_base));
    WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32, hub.ctx_addr_distance * vmid, upper_32_bits!(page_table_base));
}

unsafe fn mmhub_v3_0_2_init_gart_aperture_regs(adev: *mut amdgpu_device) {
    let pt_base = amdgpu_gmc_pd_addr!((*adev).gart.bo);
    mmhub_v3_0_2_setup_vm_pt_regs(adev, 0, pt_base);
    WREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, ((*adev).gmc.gart_start >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, ((*adev).gmc.gart_start >> 44) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, ((*adev).gmc.gart_end >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, ((*adev).gmc.gart_end >> 44) as u32);
}

unsafe fn mmhub_v3_0_2_init_system_aperture_regs(adev: *mut amdgpu_device) {
    let mut value: u64;
    let mut tmp: u32;
    WREG32_SOC15!(MMHUB, 0, regMMMC_VM_AGP_BASE, 0);
    WREG32_SOC15!(MMHUB, 0, regMMMC_VM_AGP_BOT, (*adev).gmc.agp_start >> 24);
    WREG32_SOC15!(MMHUB, 0, regMMMC_VM_AGP_TOP, (*adev).gmc.agp_end >> 24);
    if !amdgpu_sriov_vf!(adev) {
        WREG32_SOC15!(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_LOW_ADDR, min!((*adev).gmc.fb_start, (*adev).gmc.agp_start) >> 18);
        WREG32_SOC15!(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_HIGH_ADDR, max!((*adev).gmc.fb_end, (*adev).gmc.agp_end) >> 18);
    }
    value = amdgpu_gmc_vram_mc2pa!(adev, (*adev).mem_scratch.gpu_addr);
    WREG32_SOC15!(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, (value >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, (value >> 44) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32, ((*adev).dummy_page_addr >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32, ((*adev).dummy_page_addr >> 44) as u32);
    tmp = RREG32_SOC15!(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_CNTL2);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_PROTECTION_FAULT_CNTL2, ACTIVE_PAGE_MIGRATION_PTE_READ_RETRY, 1);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_CNTL2, tmp);
}

unsafe fn mmhub_v3_0_2_init_tlb_regs(adev: *mut amdgpu_device) { let mut tmp=RREG32_SOC15!(MMHUB,0,regMMMC_VM_MX_L1_TLB_CNTL); tmp=REG_SET_FIELD!(tmp,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_L1_TLB,1); tmp=REG_SET_FIELD!(tmp,MMMC_VM_MX_L1_TLB_CNTL,SYSTEM_ACCESS_MODE,3); tmp=REG_SET_FIELD!(tmp,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_ADVANCED_DRIVER_MODEL,1); tmp=REG_SET_FIELD!(tmp,MMMC_VM_MX_L1_TLB_CNTL,SYSTEM_APERTURE_UNMAPPED_ACCESS,0); tmp=REG_SET_FIELD!(tmp,MMMC_VM_MX_L1_TLB_CNTL,ECO_BITS,0); tmp=REG_SET_FIELD!(tmp,MMMC_VM_MX_L1_TLB_CNTL,MTYPE,MTYPE_UC); WREG32_SOC15!(MMHUB,0,regMMMC_VM_MX_L1_TLB_CNTL,tmp); }

unsafe fn mmhub_v3_0_2_init_cache_regs(adev: *mut amdgpu_device) { if amdgpu_sriov_vf!(adev){return;} let mut tmp=RREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL); tmp=REG_SET_FIELD!(tmp,MMVM_L2_CNTL,ENABLE_L2_CACHE,1); tmp=REG_SET_FIELD!(tmp,MMVM_L2_CNTL,ENABLE_L2_FRAGMENT_PROCESSING,0); tmp=REG_SET_FIELD!(tmp,MMVM_L2_CNTL,ENABLE_DEFAULT_PAGE_OUT_TO_SYSTEM_MEMORY,1); tmp=REG_SET_FIELD!(tmp,MMVM_L2_CNTL,L2_PDE0_CACHE_TAG_GENERATION_MODE,0); tmp=REG_SET_FIELD!(tmp,MMVM_L2_CNTL,PDE_FAULT_CLASSIFICATION,0); tmp=REG_SET_FIELD!(tmp,MMVM_L2_CNTL,CONTEXT1_IDENTITY_ACCESS_MODE,1); tmp=REG_SET_FIELD!(tmp,MMVM_L2_CNTL,IDENTITY_MODE_FRAGMENT_SIZE,0); WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL,tmp); tmp=RREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL2); tmp=REG_SET_FIELD!(tmp,MMVM_L2_CNTL2,INVALIDATE_ALL_L1_TLBS,1); tmp=REG_SET_FIELD!(tmp,MMVM_L2_CNTL2,INVALIDATE_L2_CACHE,1); WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL2,tmp); tmp=REG_SET_FIELD!(REGMMVM_L2_CNTL3_DEFAULT,MMVM_L2_CNTL3,BANK_SELECT,if (*adev).gmc.translate_further {12}else{9}); tmp=REG_SET_FIELD!(tmp,MMVM_L2_CNTL3,L2_CACHE_BIGK_FRAGMENT_SIZE,if (*adev).gmc.translate_further {9}else{6}); WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL3,tmp); tmp=REG_SET_FIELD!(REGMMVM_L2_CNTL4_DEFAULT,MMVM_L2_CNTL4,VMC_TAP_PDE_REQUEST_PHYSICAL,0); tmp=REG_SET_FIELD!(tmp,MMVM_L2_CNTL4,VMC_TAP_PTE_REQUEST_PHYSICAL,0); WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL4,tmp); tmp=REG_SET_FIELD!(REGMMVM_L2_CNTL5_DEFAULT,MMVM_L2_CNTL5,L2_CACHE_SMALLK_FRAGMENT_SIZE,0); WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL5,tmp); }

unsafe fn mmhub_v3_0_2_enable_system_domain(adev:*mut amdgpu_device){let mut tmp=RREG32_SOC15!(MMHUB,0,regMMVM_CONTEXT0_CNTL);tmp=REG_SET_FIELD!(tmp,MMVM_CONTEXT0_CNTL,ENABLE_CONTEXT,1);tmp=REG_SET_FIELD!(tmp,MMVM_CONTEXT0_CNTL,PAGE_TABLE_DEPTH,0);tmp=REG_SET_FIELD!(tmp,MMVM_CONTEXT0_CNTL,RETRY_PERMISSION_OR_INVALID_PAGE_FAULT,0);WREG32_SOC15!(MMHUB,0,regMMVM_CONTEXT0_CNTL,tmp);}
unsafe fn mmhub_v3_0_2_disable_identity_aperture(adev:*mut amdgpu_device){if amdgpu_sriov_vf!(adev){return;}WREG32_SOC15!(MMHUB,0,regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_LO32,0xFFFFFFFF);WREG32_SOC15!(MMHUB,0,regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_HI32,0xF);WREG32_SOC15!(MMHUB,0,regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_LO32,0);WREG32_SOC15!(MMHUB,0,regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_HI32,0);WREG32_SOC15!(MMHUB,0,regMMVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_LO32,0);WREG32_SOC15!(MMHUB,0,regMMVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_HI32,0);}
unsafe fn mmhub_v3_0_2_gart_enable(adev:*mut amdgpu_device)->i32{mmhub_v3_0_2_init_gart_aperture_regs(adev);mmhub_v3_0_2_init_system_aperture_regs(adev);mmhub_v3_0_2_init_tlb_regs(adev);mmhub_v3_0_2_init_cache_regs(adev);mmhub_v3_0_2_enable_system_domain(adev);mmhub_v3_0_2_disable_identity_aperture(adev);0}
unsafe fn mmhub_v3_0_2_set_fault_enable_default(adev:*mut amdgpu_device,value:bool){if amdgpu_sriov_vf!(adev){return;}let mut tmp=RREG32_SOC15!(MMHUB,0,regMMVM_L2_PROTECTION_FAULT_CNTL);for f in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT,PDE0_PROTECTION_FAULT_ENABLE_DEFAULT,PDE1_PROTECTION_FAULT_ENABLE_DEFAULT,PDE2_PROTECTION_FAULT_ENABLE_DEFAULT,TRANSLATE_FURTHER_PROTECTION_FAULT_ENABLE_DEFAULT,NACK_PROTECTION_FAULT_ENABLE_DEFAULT,DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT,VALID_PROTECTION_FAULT_ENABLE_DEFAULT,READ_PROTECTION_FAULT_ENABLE_DEFAULT,WRITE_PROTECTION_FAULT_ENABLE_DEFAULT,EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT].iter(){tmp=REG_SET_FIELD!(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,*f,value);}if !value{tmp=REG_SET_FIELD!(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,CRASH_ON_NO_RETRY_FAULT,1);tmp=REG_SET_FIELD!(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,CRASH_ON_RETRY_FAULT,1);}WREG32_SOC15!(MMHUB,0,regMMVM_L2_PROTECTION_FAULT_CNTL,tmp);}
unsafe fn mmhub_v3_0_2_gart_disable(adev:*mut amdgpu_device){let hub=&mut (*adev).vmhub[AMDGPU_MMHUB0!(0)];for i in 0..16{WREG32_SOC15_OFFSET!(MMHUB,0,regMMVM_CONTEXT0_CNTL,i*hub.ctx_distance,0);}let mut tmp=RREG32_SOC15!(MMHUB,0,regMMMC_VM_MX_L1_TLB_CNTL);tmp=REG_SET_FIELD!(tmp,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_L1_TLB,0);tmp=REG_SET_FIELD!(tmp,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_ADVANCED_DRIVER_MODEL,0);WREG32_SOC15!(MMHUB,0,regMMMC_VM_MX_L1_TLB_CNTL,tmp);tmp=RREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL);tmp=REG_SET_FIELD!(tmp,MMVM_L2_CNTL,ENABLE_L2_CACHE,0);WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL,tmp);WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL3,0);}
unsafe fn mmhub_v3_0_2_get_fb_location(adev:*mut amdgpu_device)->u64{let mut base=RREG32_SOC15!(MMHUB,0,regMMMC_VM_FB_LOCATION_BASE);base&=MMMC_VM_FB_LOCATION_BASE__FB_BASE_MASK;base=(base as u32)<<24;base as u64}
unsafe fn mmhub_v3_0_2_get_mc_fb_offset(adev:*mut amdgpu_device)->u64{(RREG32_SOC15!(MMHUB,0,regMMMC_VM_FB_OFFSET) as u64)<<24}
unsafe fn mmhub_v3_0_2_update_medium_grain_clock_gating(_adev:*mut amdgpu_device,_enable:bool){}
unsafe fn mmhub_v3_0_2_update_medium_grain_light_sleep(_adev:*mut amdgpu_device,_enable:bool){}
unsafe fn mmhub_v3_0_2_set_clockgating(adev:*mut amdgpu_device,state:amd_clockgating_state)->i32{if amdgpu_sriov_vf!(adev){return 0;}mmhub_v3_0_2_update_medium_grain_clock_gating(adev,state==AMD_CG_STATE_GATE);mmhub_v3_0_2_update_medium_grain_light_sleep(adev,state==AMD_CG_STATE_GATE);0}
unsafe fn mmhub_v3_0_2_get_clockgating(_adev:*mut amdgpu_device,_flags:*mut u64){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
