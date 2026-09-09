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

const REGGCVM_L2_CNTL3_DEFAULT: u32 = 0x80120007;
const REGGCVM_L2_CNTL4_DEFAULT: u32 = 0x000000c1;
const REGGCVM_L2_CNTL5_DEFAULT: u32 = 0x00003fe0;
const REGGRBM_GFX_INDEX_DEFAULT: u32 = 0xe0000000;

unsafe fn gfxhub_v12_1_get_fb_location(adev: *mut amdgpu_device) -> u64 {
    let mut base = RREG32_SOC15(GC, GET_INST(GC, 0), regGCMC_VM_FB_LOCATION_BASE_LO32);
    base &= GCMC_VM_FB_LOCATION_BASE_LO32__FB_BASE_LO32_MASK;
    base <<= 24;
    base |= ((GCMC_VM_FB_LOCATION_BASE_HI32__FB_BASE_HI1_MASK
        & RREG32_SOC15(GC, GET_INST(GC, 0), regGCMC_VM_FB_LOCATION_BASE_HI32)) as u64) << 56;
    base as u64
}

unsafe fn gfxhub_v12_1_get_mc_fb_offset(adev: *mut amdgpu_device) -> u64 {
    (RREG32_SOC15(GC, GET_INST(GC, 0), regGCMC_VM_FB_OFFSET) as u64) << 24
}

unsafe fn gfxhub_v12_1_xcc_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64, xcc_mask: u32) {
    let mut hub: *mut amdgpu_vmhub;
    let mut i: i32;
    for_each_inst!(i, xcc_mask);
    {
        hub = &mut (*adev).vmhub[AMDGPU_GFXHUB(i)];
        WREG32_SOC15_OFFSET!(GC, GET_INST(GC, i), regGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32,
            (*hub).ctx_addr_distance * vmid, lower_32_bits(page_table_base));
        WREG32_SOC15_OFFSET!(GC, GET_INST(GC, i), regGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32,
            (*hub).ctx_addr_distance * vmid, upper_32_bits(page_table_base));
    }
}

unsafe fn gfxhub_v12_1_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let xcc_mask = GENMASK(NUM_XCC((*adev).gfx.xcc_mask) - 1, 0);
    gfxhub_v12_1_xcc_setup_vm_pt_regs(adev, vmid, page_table_base, xcc_mask);
}

unsafe fn gfxhub_v12_1_xcc_init_gart_aperture_regs(adev: *mut amdgpu_device, xcc_mask: u32) {
    let pt_base = if !(*adev).gmc.pdb0_bo.is_null() {
        amdgpu_gmc_pd_addr((*adev).gmc.pdb0_bo)
    } else { amdgpu_gmc_pd_addr((*adev).gart.bo) };
    gfxhub_v12_1_xcc_setup_vm_pt_regs(adev, 0, pt_base, xcc_mask);
    let mut i: i32;
    for_each_inst!(i, xcc_mask);
    {
        if !(*adev).gmc.pdb0_bo.is_null() {
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, (*adev).gmc.fb_start >> 12);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, (*adev).gmc.fb_start >> 44);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, (*adev).gmc.gart_end >> 12);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, (*adev).gmc.gart_end >> 44);
        } else {
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, (*adev).gmc.gart_start >> 12);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, (*adev).gmc.gart_start >> 44);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, (*adev).gmc.gart_end >> 12);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, (*adev).gmc.gart_end >> 44);
        }
    }
}

unsafe fn gfxhub_v12_1_xcc_init_system_aperture_regs(adev: *mut amdgpu_device, xcc_mask: u32) {
    let mut i: i32;
    if amdgpu_sriov_vf(adev) { return; }
    for_each_inst!(i, xcc_mask);
    {
        if !(*adev).gmc.pdb0_bo.is_null() {
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_FB_LOCATION_TOP_LO32, 0);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_FB_LOCATION_TOP_HI32, 0);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_FB_LOCATION_BASE_LO32, 0xFFFFFFFFu32);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_FB_LOCATION_BASE_HI32, 1);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_AGP_TOP_LO32, 0);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_AGP_TOP_HI32, 0);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_AGP_BOT_LO32, 0xFFFFFFFFu32);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_AGP_BOT_HI32, 1);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_SYSTEM_APERTURE_LOW_ADDR_LO32, 0xFFFFFFFFu32);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_SYSTEM_APERTURE_LOW_ADDR_HI32, 0x7F);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_SYSTEM_APERTURE_HIGH_ADDR_LO32, 0);
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_SYSTEM_APERTURE_HIGH_ADDR_HI32, 0);
        } else {
            WREG32_SOC15_RLC!(GC, GET_INST(GC, i), regGCMC_VM_AGP_BASE_LO32, 0);
            WREG32_SOC15_RLC!(GC, GET_INST(GC, i), regGCMC_VM_AGP_BASE_HI32, 0);
            WREG32_SOC15_RLC!(GC, GET_INST(GC, i), regGCMC_VM_AGP_BOT_LO32, lower_32_bits((*adev).gmc.agp_start >> 24));
            WREG32_SOC15_RLC!(GC, GET_INST(GC, i), regGCMC_VM_AGP_BOT_HI32, upper_32_bits((*adev).gmc.agp_start >> 24));
            WREG32_SOC15_RLC!(GC, GET_INST(GC, i), regGCMC_VM_AGP_TOP_LO32, lower_32_bits((*adev).gmc.agp_end >> 24));
            WREG32_SOC15_RLC!(GC, GET_INST(GC, i), regGCMC_VM_AGP_TOP_HI32, upper_32_bits((*adev).gmc.agp_end >> 24));
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_SYSTEM_APERTURE_LOW_ADDR_LO32, lower_32_bits(min((*adev).gmc.fb_start, (*adev).gmc.agp_start) >> 18));
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_SYSTEM_APERTURE_LOW_ADDR_HI32, upper_32_bits(min((*adev).gmc.fb_start, (*adev).gmc.agp_start) >> 18));
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_SYSTEM_APERTURE_HIGH_ADDR_LO32, lower_32_bits(max((*adev).gmc.fb_end, (*adev).gmc.agp_end) >> 18));
            WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_SYSTEM_APERTURE_HIGH_ADDR_HI32, upper_32_bits(max((*adev).gmc.fb_end, (*adev).gmc.agp_end) >> 18));
        }
        let value = amdgpu_gmc_vram_mc2pa(adev, (*adev).mem_scratch.gpu_addr);
        WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, value >> 12);
        WREG32_SOC15!(GC, GET_INST(GC, i), regGCMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, value >> 44);
        WREG32_SOC15!(GC, GET_INST(GC, i), regGCVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32, (*adev).dummy_page_addr >> 12);
        WREG32_SOC15!(GC, GET_INST(GC, i), regGCVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32, (*adev).dummy_page_addr >> 44);
        let mut tmp = RREG32_SOC15(GC, GET_INST(GC, i), regGCVM_L2_PROTECTION_FAULT_CNTL2);
        tmp = REG_SET_FIELD(tmp, GCVM_L2_PROTECTION_FAULT_CNTL2, ACTIVE_PAGE_MIGRATION_PTE_READ_RETRY, 1);
        tmp = REG_SET_FIELD(tmp, GCVM_L2_PROTECTION_FAULT_CNTL2, ENABLE_RETRY_FAULT_INTERRUPT, 0x1);
        WREG32_SOC15!(GC, GET_INST(GC, i), regGCVM_L2_PROTECTION_FAULT_CNTL2, tmp);
    }
}

unsafe fn gfxhub_v12_1_xcc_init_tlb_regs(adev: *mut amdgpu_device, xcc_mask: u32) {
    let mut i: i32; for_each_inst!(i, xcc_mask); { let mut tmp = RREG32_SOC15(GC, GET_INST(GC,i), regGCMC_VM_MX_L1_TLB_CNTL);
        tmp=REG_SET_FIELD(tmp,GCMC_VM_MX_L1_TLB_CNTL,ENABLE_L1_TLB,1); tmp=REG_SET_FIELD(tmp,GCMC_VM_MX_L1_TLB_CNTL,SYSTEM_ACCESS_MODE,3); tmp=REG_SET_FIELD(tmp,GCMC_VM_MX_L1_TLB_CNTL,ENABLE_ADVANCED_DRIVER_MODEL,1); tmp=REG_SET_FIELD(tmp,GCMC_VM_MX_L1_TLB_CNTL,SYSTEM_APERTURE_UNMAPPED_ACCESS,0); tmp=REG_SET_FIELD(tmp,GCMC_VM_MX_L1_TLB_CNTL,ECO_BITS,0); tmp=REG_SET_FIELD(tmp,GCMC_VM_MX_L1_TLB_CNTL,MTYPE,MTYPE_UC); WREG32_SOC15_RLC!(GC,GET_INST(GC,i),regGCMC_VM_MX_L1_TLB_CNTL,tmp); }
}

unsafe fn gfxhub_v12_1_xcc_init_cache_regs(adev: *mut amdgpu_device, xcc_mask: u32) {
    let mut i:i32; for_each_inst!(i,xcc_mask); { let mut tmp=RREG32_SOC15(GC,GET_INST(GC,i),regGCVM_L2_CNTL);
        tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL,ENABLE_L2_CACHE,1); tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL,ENABLE_L2_FRAGMENT_PROCESSING,0); tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL,ENABLE_DEFAULT_PAGE_OUT_TO_SYSTEM_MEMORY,1); tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL,L2_PDE0_CACHE_TAG_GENERATION_MODE,0); tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL,PDE_FAULT_CLASSIFICATION,0); tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL,CONTEXT1_IDENTITY_ACCESS_MODE,1); tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL,IDENTITY_MODE_FRAGMENT_SIZE,0); WREG32_SOC15_RLC!(GC,GET_INST(GC,i),regGCVM_L2_CNTL,tmp);
        tmp=RREG32_SOC15(GC,GET_INST(GC,i),regGCVM_L2_CNTL2); tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL2,INVALIDATE_ALL_L1_TLBS,1); tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL2,INVALIDATE_L2_CACHE,1); WREG32_SOC15_RLC!(GC,GET_INST(GC,i),regGCVM_L2_CNTL2,tmp);
        tmp=REGGCVM_L2_CNTL3_DEFAULT; if (*adev).gmc.translate_further { tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL3,BANK_SELECT,12); tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL3,L2_CACHE_BIGK_FRAGMENT_SIZE,9); } else { tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL3,BANK_SELECT,9); tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL3,L2_CACHE_BIGK_FRAGMENT_SIZE,6); } WREG32_SOC15_RLC!(GC,GET_INST(GC,i),regGCVM_L2_CNTL3,tmp);
        tmp=REGGCVM_L2_CNTL4_DEFAULT; tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL4,VMC_TAP_PDE_REQUEST_PHYSICAL,(*adev).gmc.xgmi.connected_to_cpu as u32); tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL4,VMC_TAP_PTE_REQUEST_PHYSICAL,(*adev).gmc.xgmi.connected_to_cpu as u32); WREG32_SOC15_RLC!(GC,GET_INST(GC,i),regGCVM_L2_CNTL4,tmp);
        tmp=REGGCVM_L2_CNTL5_DEFAULT; tmp=REG_SET_FIELD(tmp,GCVM_L2_CNTL5,L2_CACHE_SMALLK_FRAGMENT_SIZE,0); WREG32_SOC15_RLC!(GC,GET_INST(GC,i),regGCVM_L2_CNTL5,tmp); }
}

unsafe fn gfxhub_v12_1_xcc_enable_system_domain(adev:*mut amdgpu_device,xcc_mask:u32){let mut i:i32;for_each_inst!(i,xcc_mask);{let mut t=RREG32_SOC15(GC,GET_INST(GC,i),regGCVM_CONTEXT0_CNTL);t=REG_SET_FIELD(t,GCVM_CONTEXT0_CNTL,ENABLE_CONTEXT,1);t=REG_SET_FIELD(t,GCVM_CONTEXT0_CNTL,PAGE_TABLE_DEPTH,(*adev).gmc.vmid0_page_table_depth);t=REG_SET_FIELD(t,GCVM_CONTEXT0_CNTL,PAGE_TABLE_BLOCK_SIZE,(*adev).gmc.vmid0_page_table_block_size);t=REG_SET_FIELD(t,GCVM_CONTEXT0_CNTL,RETRY_PERMISSION_OR_INVALID_PAGE_FAULT,0);WREG32_SOC15!(GC,GET_INST(GC,i),regGCVM_CONTEXT0_CNTL,t);}}

unsafe fn gfxhub_v12_1_xcc_disable_identity_aperture(adev:*mut amdgpu_device,xcc_mask:u32){let mut i:i32;for_each_inst!(i,xcc_mask);{WREG32_SOC15!(GC,GET_INST(GC,i),regGCVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_LO32,0xFFFFFFFFu32);WREG32_SOC15!(GC,GET_INST(GC,i),regGCVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_HI32,0x1FFF);WREG32_SOC15!(GC,GET_INST(GC,i),regGCVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_LO32,0);WREG32_SOC15!(GC,GET_INST(GC,i),regGCVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_HI32,0);WREG32_SOC15!(GC,GET_INST(GC,i),regGCVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_LO32,0);WREG32_SOC15!(GC,GET_INST(GC,i),regGCVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_HI32,0);}}

unsafe fn gfxhub_v12_1_xcc_setup_vmid_config(adev:*mut amdgpu_device,xcc_mask:u32){let n=(*adev).vm_manager.num_level;let b=(*adev).vm_manager.block_size-9;let mut j:i32;for_each_inst!(j,xcc_mask);{let h=&mut (*adev).vmhub[AMDGPU_GFXHUB(j)];let mut i=0;while i<=14{let mut t=RREG32_SOC15_OFFSET(GC,GET_INST(GC,j),regGCVM_CONTEXT1_CNTL,i*(*h).ctx_distance);for f in [ENABLE_CONTEXT,PAGE_TABLE_DEPTH,RANGE_PROTECTION_FAULT_ENABLE_DEFAULT,DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT,PDE0_PROTECTION_FAULT_ENABLE_DEFAULT,VALID_PROTECTION_FAULT_ENABLE_DEFAULT,READ_PROTECTION_FAULT_ENABLE_DEFAULT,WRITE_PROTECTION_FAULT_ENABLE_DEFAULT,EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT,PAGE_TABLE_BLOCK_SIZE,RETRY_PERMISSION_OR_INVALID_PAGE_FAULT]{let v=if f==PAGE_TABLE_DEPTH{n}else if f==PAGE_TABLE_BLOCK_SIZE{b}else{1};t=REG_SET_FIELD(t,GCVM_CONTEXT1_CNTL,f,v);}WREG32_SOC15_OFFSET!(GC,GET_INST(GC,j),regGCVM_CONTEXT1_CNTL,i*(*h).ctx_distance,t);WREG32_SOC15_OFFSET!(GC,GET_INST(GC,j),regGCVM_CONTEXT1_PAGE_TABLE_START_ADDR_LO32,i*(*h).ctx_addr_distance,0);WREG32_SOC15_OFFSET!(GC,GET_INST(GC,j),regGCVM_CONTEXT1_PAGE_TABLE_START_ADDR_HI32,i*(*h).ctx_addr_distance,0);WREG32_SOC15_OFFSET!(GC,GET_INST(GC,j),regGCVM_CONTEXT1_PAGE_TABLE_END_ADDR_LO32,i*(*h).ctx_addr_distance,lower_32_bits((*adev).vm_manager.max_pfn-1));WREG32_SOC15_OFFSET!(GC,GET_INST(GC,j),regGCVM_CONTEXT1_PAGE_TABLE_END_ADDR_HI32,i*(*h).ctx_addr_distance,upper_32_bits((*adev).vm_manager.max_pfn-1));i+=1;}(*h).vm_cntx_cntl=t;}}

unsafe fn gfxhub_v12_1_xcc_program_invalidation(adev:*mut amdgpu_device,xcc_mask:u32){let mut j:i32;for_each_inst!(j,xcc_mask);{let h=&mut (*adev).vmhub[AMDGPU_GFXHUB(j)];for i in 0..18{WREG32_SOC15_OFFSET!(GC,GET_INST(GC,j),regGCVM_INVALIDATE_ENG0_ADDR_RANGE_LO32,i*(*h).eng_addr_distance,0xFFFFFFFFu32);WREG32_SOC15_OFFSET!(GC,GET_INST(GC,j),regGCVM_INVALIDATE_ENG0_ADDR_RANGE_HI32,i*(*h).eng_addr_distance,0x3FFF);}}}

unsafe fn gfxhub_v12_1_xcc_gart_enable(adev:*mut amdgpu_device,xcc_mask:u32)->i32{let mut i:u32;if amdgpu_sriov_vf(adev){for_each_inst!(i,xcc_mask);{WREG32_SOC15!(GC,GET_INST(GC,i),regGCMC_VM_FB_LOCATION_BASE_LO32,lower_32_bits((*adev).gmc.vram_start>>24));WREG32_SOC15!(GC,GET_INST(GC,i),regGCMC_VM_FB_LOCATION_BASE_HI32,upper_32_bits((*adev).gmc.vram_start>>24));WREG32_SOC15!(GC,GET_INST(GC,i),regGCMC_VM_FB_LOCATION_TOP_LO32,lower_32_bits((*adev).gmc.vram_end>>24));WREG32_SOC15!(GC,GET_INST(GC,i),regGCMC_VM_FB_LOCATION_TOP_HI32,upper_32_bits((*adev).gmc.vram_end>>24));}}gfxhub_v12_1_xcc_init_gart_aperture_regs(adev,xcc_mask);gfxhub_v12_1_xcc_init_system_aperture_regs(adev,xcc_mask);gfxhub_v12_1_xcc_init_tlb_regs(adev,xcc_mask);if !amdgpu_sriov_vf(adev){gfxhub_v12_1_xcc_init_cache_regs(adev,xcc_mask);}gfxhub_v12_1_xcc_enable_system_domain(adev,xcc_mask);if !amdgpu_sriov_vf(adev){gfxhub_v12_1_xcc_disable_identity_aperture(adev,xcc_mask);}gfxhub_v12_1_xcc_setup_vmid_config(adev,xcc_mask);gfxhub_v12_1_xcc_program_invalidation(adev,xcc_mask);0}
unsafe fn gfxhub_v12_1_gart_enable(adev:*mut amdgpu_device)->i32{gfxhub_v12_1_xcc_gart_enable(adev,GENMASK(NUM_XCC((*adev).gfx.xcc_mask)-1,0))}

unsafe fn gfxhub_v12_1_xcc_gart_disable(adev:*mut amdgpu_device,xcc_mask:u32){let mut j:u32;for_each_inst!(j,xcc_mask);{let h=&mut (*adev).vmhub[AMDGPU_GFXHUB(j)];for i in 0..16{WREG32_SOC15_OFFSET!(GC,GET_INST(GC,j),regGCVM_CONTEXT0_CNTL,i*(*h).ctx_distance,0);}let mut t=RREG32_SOC15(GC,GET_INST(GC,j),regGCMC_VM_MX_L1_TLB_CNTL);t=REG_SET_FIELD(t,GCMC_VM_MX_L1_TLB_CNTL,ENABLE_L1_TLB,0);t=REG_SET_FIELD(t,GCMC_VM_MX_L1_TLB_CNTL,ENABLE_ADVANCED_DRIVER_MODEL,0);WREG32_SOC15_RLC!(GC,GET_INST(GC,j),regGCMC_VM_MX_L1_TLB_CNTL,t);if !amdgpu_sriov_vf(adev){t=RREG32_SOC15(GC,GET_INST(GC,j),regGCVM_L2_CNTL);t=REG_SET_FIELD(t,GCVM_L2_CNTL,ENABLE_L2_CACHE,0);WREG32_SOC15!(GC,GET_INST(GC,j),regGCVM_L2_CNTL,t);WREG32_SOC15!(GC,GET_INST(GC,j),regGCVM_L2_CNTL3,0);}}}
unsafe fn gfxhub_v12_1_gart_disable(adev:*mut amdgpu_device){gfxhub_v12_1_xcc_gart_disable(adev,GENMASK(NUM_XCC((*adev).gfx.xcc_mask)-1,0));}

unsafe fn gfxhub_v12_1_xcc_set_fault_enable_default(adev:*mut amdgpu_device,value:bool,xcc_mask:u32){let mut i:i32;for_each_inst!(i,xcc_mask);{let mut t=RREG32_SOC15(GC,GET_INST(GC,i),regGCVM_L2_PROTECTION_FAULT_CNTL_LO32);for f in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT,PDE0_PROTECTION_FAULT_ENABLE_DEFAULT,PDE1_PROTECTION_FAULT_ENABLE_DEFAULT,PDE2_PROTECTION_FAULT_ENABLE_DEFAULT,PDE3_PROTECTION_FAULT_ENABLE_DEFAULT,TRANSLATE_FURTHER_PROTECTION_FAULT_ENABLE_DEFAULT,NACK_PROTECTION_FAULT_ENABLE_DEFAULT,DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT,VALID_PROTECTION_FAULT_ENABLE_DEFAULT,READ_PROTECTION_FAULT_ENABLE_DEFAULT,WRITE_PROTECTION_FAULT_ENABLE_DEFAULT,EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT,OTHER_CLIENT_ID_NO_RETRY_FAULT_INTERRUPT]{t=REG_SET_FIELD(t,GCVM_L2_PROTECTION_FAULT_CNTL_LO32,f,value as u32);}t=REG_SET_FIELD(t,GCVM_L2_PROTECTION_FAULT_CNTL_LO32,CLIENT_ID_NO_RETRY_FAULT_INTERRUPT,if value{0xFFFF}else{0});t=REG_SET_FIELD(t,GCVM_L2_PROTECTION_FAULT_CNTL_LO32,CRASH_ON_NO_RETRY_FAULT,(!value) as u32);WREG32_SOC15!(GC,GET_INST(GC,i),regGCVM_L2_PROTECTION_FAULT_CNTL_LO32,t);t=RREG32_SOC15(GC,GET_INST(GC,i),regGCVM_L2_PROTECTION_FAULT_CNTL_HI32);t=REG_SET_FIELD(t,GCVM_L2_PROTECTION_FAULT_CNTL_HI32,CRASH_ON_RETRY_FAULT,(!value) as u32);WREG32_SOC15!(GC,GET_INST(GC,i),regGCVM_L2_PROTECTION_FAULT_CNTL_HI32,t);}}
unsafe fn gfxhub_v12_1_set_fault_enable_default(adev:*mut amdgpu_device,value:bool){gfxhub_v12_1_xcc_set_fault_enable_default(adev,value,GENMASK(NUM_XCC((*adev).gfx.xcc_mask)-1,0));}

unsafe fn gfxhub_v12_1_get_invalidate_req(vmid:u32,flush_type:u32)->u32{let mut r=0;r=REG_SET_FIELD(r,GCVM_INVALIDATE_ENG0_REQ,PER_VMID_INVALIDATE_REQ,1<<vmid);r=REG_SET_FIELD(r,GCVM_INVALIDATE_ENG0_REQ,FLUSH_TYPE,flush_type);for f in [INVALIDATE_L2_PTES,INVALIDATE_L2_PDE0,INVALIDATE_L2_PDE1,INVALIDATE_L2_PDE2,INVALIDATE_L2_PDE3,INVALIDATE_L1_PTES]{r=REG_SET_FIELD(r,GCVM_INVALIDATE_ENG0_REQ,f,1);}REG_SET_FIELD(r,GCVM_INVALIDATE_ENG0_REQ,CLEAR_PROTECTION_FAULT_STATUS_ADDR,0)}

static GFXHUB_V12_1_CLIENT_IDS: [&'static str;21] = ["CB","DB","GE1","GE2","CPF","CPC","CPG","RLC","TCP","SQC (inst)","SQC (data)","SQG/PC/SC","Reserved","SDMA0","SDMA1","GCR","Reserved","Reserved","WGS","DSM","PA"];
unsafe fn gfxhub_v12_1_print_l2_protection_fault_status(adev:*mut amdgpu_device,status:u32){let cid=REG_GET_FIELD(status,GCVM_L2_PROTECTION_FAULT_STATUS_LO32,CID);dev_err!((*adev).dev,"GCVM_L2_PROTECTION_FAULT_STATUS_LO32:0x%08X\n",status);dev_err!((*adev).dev,"\t Faulty UTCL2 client ID: %s (0x%x)\n",if cid>=GFXHUB_V12_1_CLIENT_IDS.len(){"unknown"}else{GFXHUB_V12_1_CLIENT_IDS[cid as usize]},cid);for (n,f) in [("MORE_FAULTS",MORE_FAULTS),("WALKER_ERROR",WALKER_ERROR),("PERMISSION_FAULTS",PERMISSION_FAULTS),("MAPPING_ERROR",MAPPING_ERROR),("RW",RW)]{dev_err!((*adev).dev,"\t {}: 0x{:x}\n",n,REG_GET_FIELD(status,GCVM_L2_PROTECTION_FAULT_STATUS_LO32,f));}}

static GFXHUB_V12_1_VMHub_FUNCS: amdgpu_vmhub_funcs = amdgpu_vmhub_funcs { print_l2_protection_fault_status: Some(gfxhub_v12_1_print_l2_protection_fault_status), get_invalidate_req: Some(gfxhub_v12_1_get_invalidate_req) };
unsafe fn gfxhub_v12_1_xcc_init(adev:*mut amdgpu_device,xcc_mask:u32){let mut i:i32;for_each_inst!(i,xcc_mask);{let h=&mut (*adev).vmhub[AMDGPU_GFXHUB(i)];h.ctx0_ptb_addr_lo32=SOC15_REG_OFFSET(GC,GET_INST(GC,i),regGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32);h.ctx0_ptb_addr_hi32=SOC15_REG_OFFSET(GC,GET_INST(GC,i),regGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32);h.vm_inv_eng0_sem=SOC15_REG_OFFSET(GC,GET_INST(GC,i),regGCVM_INVALIDATE_ENG0_SEM);h.vm_inv_eng0_req=SOC15_REG_OFFSET(GC,GET_INST(GC,i),regGCVM_INVALIDATE_ENG0_REQ);h.vm_inv_eng0_ack=SOC15_REG_OFFSET(GC,GET_INST(GC,i),regGCVM_INVALIDATE_ENG0_ACK);h.vm_context0_cntl=SOC15_REG_OFFSET(GC,GET_INST(GC,i),regGCVM_CONTEXT0_CNTL);h.vm_l2_pro_fault_status=SOC15_REG_OFFSET(GC,GET_INST(GC,i),regGCVM_L2_PROTECTION_FAULT_STATUS_LO32);h.vm_l2_pro_fault_cntl=SOC15_REG_OFFSET(GC,GET_INST(GC,i),regGCVM_L2_PROTECTION_FAULT_CNTL_LO32);h.ctx_distance=regGCVM_CONTEXT1_CNTL-regGCVM_CONTEXT0_CNTL;h.ctx_addr_distance=regGCVM_CONTEXT1_PAGE_TABLE_BASE_ADDR_LO32-regGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32;h.eng_distance=regGCVM_INVALIDATE_ENG1_REQ-regGCVM_INVALIDATE_ENG0_REQ;h.eng_addr_distance=regGCVM_INVALIDATE_ENG1_ADDR_RANGE_LO32-regGCVM_INVALIDATE_ENG0_ADDR_RANGE_LO32;h.vm_cntx_cntl_vm_fault=GCVM_CONTEXT1_CNTL__RANGE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|GCVM_CONTEXT1_CNTL__DUMMY_PAGE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|GCVM_CONTEXT1_CNTL__PDE0_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|GCVM_CONTEXT1_CNTL__VALID_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|GCVM_CONTEXT1_CNTL__READ_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|GCVM_CONTEXT1_CNTL__WRITE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|GCVM_CONTEXT1_CNTL__EXECUTE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK;h.vmhub_funcs=&GFXHUB_V12_1_VMHub_FUNCS;}}
unsafe fn gfxhub_v12_1_init(adev:*mut amdgpu_device){gfxhub_v12_1_xcc_init(adev,GENMASK(NUM_XCC((*adev).gfx.xcc_mask)-1,0));}
static mut GFXHUB_V12_1_FUNCS: amdgpu_gfxhub_funcs = amdgpu_gfxhub_funcs { get_fb_location:Some(gfxhub_v12_1_get_fb_location),get_mc_fb_offset:Some(gfxhub_v12_1_get_mc_fb_offset),setup_vm_pt_regs:Some(gfxhub_v12_1_setup_vm_pt_regs),gart_enable:Some(gfxhub_v12_1_gart_enable),gart_disable:Some(gfxhub_v12_1_gart_disable),set_fault_enable_default:Some(gfxhub_v12_1_set_fault_enable_default),init:Some(gfxhub_v12_1_init) };
unsafe extern "C" fn gfxhub_v12_1_xcp_resume(handle:*mut c_void,inst_mask:u32)->i32{let a=handle as *mut amdgpu_device;let v=amdgpu_vm_fault_stop!=AMDGPU_VM_FAULT_STOP_ALWAYS;gfxhub_v12_1_xcc_set_fault_enable_default(a,v,inst_mask);if !amdgpu_sriov_vf(a){gfxhub_v12_1_xcc_gart_enable(a,inst_mask)}else{0}}
unsafe extern "C" fn gfxhub_v12_1_xcp_suspend(handle:*mut c_void,inst_mask:u32)->i32{let a=handle as *mut amdgpu_device;if !amdgpu_sriov_vf(a){gfxhub_v12_1_xcc_gart_disable(a,inst_mask);}0}
static mut GFXHUB_V12_1_XCP_FUNCS: amdgpu_xcp_ip_funcs = amdgpu_xcp_ip_funcs { suspend:Some(gfxhub_v12_1_xcp_suspend),resume:Some(gfxhub_v12_1_xcp_resume) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
