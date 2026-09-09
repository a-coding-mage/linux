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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

static MMHUB_CLIENT_IDS_VANGOGH: [[*const core::ffi::c_char; 2]; 31] = [
    [c"MP0".as_ptr(), c"MP0".as_ptr()], [c"MP1".as_ptr(), c"MP1".as_ptr()],
    [c"DCEDMC".as_ptr(), c"DCEDMC".as_ptr()], [c"DCEVGA".as_ptr(), c"DCEVGA".as_ptr()],
    [core::ptr::null(), c"DCEDWB".as_ptr()], [core::ptr::null(), c"XDP".as_ptr()],
    [core::ptr::null(); 2], [core::ptr::null(); 2], [core::ptr::null(); 2],
    [core::ptr::null(); 2], [core::ptr::null(); 2], [core::ptr::null(); 2],
    [core::ptr::null(); 2], [c"UTCL2".as_ptr(), core::ptr::null()],
    [core::ptr::null(); 2], [core::ptr::null(); 2], [core::ptr::null(); 2],
    [core::ptr::null(); 2], [core::ptr::null(); 2], [core::ptr::null(); 2],
    [core::ptr::null(); 2], [core::ptr::null(); 2], [core::ptr::null(); 2],
    [core::ptr::null(); 2], [core::ptr::null(); 2], [core::ptr::null(); 2],
    [c"OSS".as_ptr(), c"OSS".as_ptr()], [c"HDP".as_ptr(), c"HDP".as_ptr()],
    [c"VCN".as_ptr(), c"VCN".as_ptr()], [c"VCNU".as_ptr(), c"VCNU".as_ptr()],
    [c"JPEG".as_ptr(), c"JPEG".as_ptr()],
];

unsafe fn mmhub_v2_3_get_invalidate_req(vmid: u32, flush_type: u32) -> u32 {
    let mut req = 0;
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, PER_VMID_INVALIDATE_REQ, 1u32.wrapping_shl(vmid));
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, FLUSH_TYPE, flush_type);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PTES, 1);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE0, 1);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE1, 1);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE2, 1);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L1_PTES, 1);
    REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, CLEAR_PROTECTION_FAULT_STATUS_ADDR, 0)
}

unsafe fn mmhub_v2_3_print_l2_protection_fault_status(adev: *mut amdgpu_device, status: u32) {
    let cid = REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, CID);
    let rw = REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, RW);
    dev_err!((*adev).dev, "MMVM_L2_PROTECTION_FAULT_STATUS:0x{:08X}\n", status);
    let mmhub_cid = amdgpu_mmhub_client_name!(&mut (*adev).mmhub, cid, rw);
    dev_err!((*adev).dev, "\t Faulty UTCL2 client ID: {} (0x{:x})\n", if !mmhub_cid.is_null() { mmhub_cid } else { c"unknown".as_ptr() }, cid);
    dev_err!((*adev).dev, "\t MORE_FAULTS: 0x{:x}\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, MORE_FAULTS));
    dev_err!((*adev).dev, "\t WALKER_ERROR: 0x{:x}\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, WALKER_ERROR));
    dev_err!((*adev).dev, "\t PERMISSION_FAULTS: 0x{:x}\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, PERMISSION_FAULTS));
    dev_err!((*adev).dev, "\t MAPPING_ERROR: 0x{:x}\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, MAPPING_ERROR));
    dev_err!((*adev).dev, "\t RW: 0x{:x}\n", rw);
}

unsafe fn mmhub_v2_3_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0!(0)];
    WREG32_SOC15_OFFSET!(MMHUB, 0, mmMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32, hub.ctx_addr_distance * vmid, lower_32_bits(page_table_base));
    WREG32_SOC15_OFFSET!(MMHUB, 0, mmMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32, hub.ctx_addr_distance * vmid, upper_32_bits(page_table_base));
}

unsafe fn mmhub_v2_3_init_gart_aperture_regs(adev: *mut amdgpu_device) {
    let pt_base = amdgpu_gmc_pd_addr!((*adev).gart.bo);
    mmhub_v2_3_setup_vm_pt_regs(adev, 0, pt_base);
    WREG32_SOC15!(MMHUB, 0, mmMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, ((*adev).gmc.gart_start >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, mmMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, ((*adev).gmc.gart_start >> 44) as u32);
    WREG32_SOC15!(MMHUB, 0, mmMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, ((*adev).gmc.gart_end >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, mmMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, ((*adev).gmc.gart_end >> 44) as u32);
}

unsafe fn mmhub_v2_3_init_system_aperture_regs(adev: *mut amdgpu_device) {
    let value = amdgpu_gmc_vram_mc2pa!(adev, (*adev).mem_scratch.gpu_addr);
    WREG32_SOC15!(MMHUB, 0, mmMMMC_VM_AGP_BASE, 0);
    WREG32_SOC15!(MMHUB, 0, mmMMMC_VM_AGP_BOT, (*adev).gmc.agp_start >> 24);
    WREG32_SOC15!(MMHUB, 0, mmMMMC_VM_AGP_TOP, (*adev).gmc.agp_end >> 24);
    WREG32_SOC15!(MMHUB, 0, mmMMMC_VM_SYSTEM_APERTURE_LOW_ADDR, core::cmp::min((*adev).gmc.fb_start, (*adev).gmc.agp_start) >> 18);
    WREG32_SOC15!(MMHUB, 0, mmMMMC_VM_SYSTEM_APERTURE_HIGH_ADDR, core::cmp::max((*adev).gmc.fb_end, (*adev).gmc.agp_end) >> 18);
    WREG32_SOC15!(MMHUB, 0, mmMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, (value >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, mmMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, (value >> 44) as u32);
    WREG32_SOC15!(MMHUB, 0, mmMMVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32, ((*adev).dummy_page_addr >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, mmMMVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32, ((*adev).dummy_page_addr >> 44) as u32);
    let mut tmp = RREG32_SOC15!(MMHUB, 0, mmMMVM_L2_PROTECTION_FAULT_CNTL2);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_PROTECTION_FAULT_CNTL2, ACTIVE_PAGE_MIGRATION_PTE_READ_RETRY, 1);
    WREG32_SOC15!(MMHUB, 0, mmMMVM_L2_PROTECTION_FAULT_CNTL2, tmp);
}

unsafe fn mmhub_v2_3_init_tlb_regs(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15!(MMHUB, 0, mmMMMC_VM_MX_L1_TLB_CNTL);
    tmp = REG_SET_FIELD!(tmp, MMMC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 1);
    tmp = REG_SET_FIELD!(tmp, MMMC_VM_MX_L1_TLB_CNTL, SYSTEM_ACCESS_MODE, 3);
    tmp = REG_SET_FIELD!(tmp, MMMC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 1);
    tmp = REG_SET_FIELD!(tmp, MMMC_VM_MX_L1_TLB_CNTL, SYSTEM_APERTURE_UNMAPPED_ACCESS, 0);
    tmp = REG_SET_FIELD!(tmp, MMMC_VM_MX_L1_TLB_CNTL, MTYPE, MTYPE_UC);
    WREG32_SOC15!(MMHUB, 0, mmMMMC_VM_MX_L1_TLB_CNTL, tmp);
}

unsafe fn mmhub_v2_3_init_cache_regs(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15!(MMHUB, 0, mmMMVM_L2_CNTL);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, ENABLE_L2_CACHE, 1);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, ENABLE_L2_FRAGMENT_PROCESSING, 0);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, ENABLE_DEFAULT_PAGE_OUT_TO_SYSTEM_MEMORY, 1);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, L2_PDE0_CACHE_TAG_GENERATION_MODE, 0);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, PDE_FAULT_CLASSIFICATION, 0);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, CONTEXT1_IDENTITY_ACCESS_MODE, 1);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, IDENTITY_MODE_FRAGMENT_SIZE, 0);
    WREG32_SOC15!(MMHUB, 0, mmMMVM_L2_CNTL, tmp);
    tmp = RREG32_SOC15!(MMHUB, 0, mmMMVM_L2_CNTL2);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL2, INVALIDATE_ALL_L1_TLBS, 1);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL2, INVALIDATE_L2_CACHE, 1);
    WREG32_SOC15!(MMHUB, 0, mmMMVM_L2_CNTL2, tmp);
    tmp = mmMMVM_L2_CNTL3_DEFAULT;
    if (*adev).gmc.translate_further { tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL3, BANK_SELECT, 12); tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 9); }
    else { tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL3, BANK_SELECT, 9); tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 6); }
    WREG32_SOC15!(MMHUB, 0, mmMMVM_L2_CNTL3, tmp);
    tmp = REG_SET_FIELD!(mmMMVM_L2_CNTL4_DEFAULT, MMVM_L2_CNTL4, VMC_TAP_PDE_REQUEST_PHYSICAL, 0);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL4, VMC_TAP_PTE_REQUEST_PHYSICAL, 0); WREG32_SOC15!(MMHUB, 0, mmMMVM_L2_CNTL4, tmp);
    tmp = REG_SET_FIELD!(mmMMVM_L2_CNTL5_DEFAULT, MMVM_L2_CNTL5, L2_CACHE_SMALLK_FRAGMENT_SIZE, 0); WREG32_SOC15!(MMHUB, 0, mmMMVM_L2_CNTL5, tmp);
}

unsafe fn mmhub_v2_3_enable_system_domain(adev: *mut amdgpu_device) { let mut tmp = RREG32_SOC15!(MMHUB, 0, mmMMVM_CONTEXT0_CNTL); tmp = REG_SET_FIELD!(tmp, MMVM_CONTEXT0_CNTL, ENABLE_CONTEXT, 1); tmp = REG_SET_FIELD!(tmp, MMVM_CONTEXT0_CNTL, PAGE_TABLE_DEPTH, 0); tmp = REG_SET_FIELD!(tmp, MMVM_CONTEXT0_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, 0); WREG32_SOC15!(MMHUB, 0, mmMMVM_CONTEXT0_CNTL, tmp); }

unsafe fn mmhub_v2_3_disable_identity_aperture(adev: *mut amdgpu_device) { WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_LO32,0xffffffff); WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_HI32,0xf); WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_LO32,0); WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_HI32,0); WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_LO32,0); WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_HI32,0); }

unsafe fn mmhub_v2_3_setup_vmid_config(adev: *mut amdgpu_device) { let hub=&mut (*adev).vmhub[AMDGPU_MMHUB0!(0)]; let mut tmp=0; for i in 0..=14 { let off=i*hub.ctx_distance; tmp=RREG32_SOC15_OFFSET!(MMHUB,0,mmMMVM_CONTEXT1_CNTL,off); for (f,v) in [(ENABLE_CONTEXT,1),(PAGE_TABLE_DEPTH,(*adev).vm_manager.num_level),(RANGE_PROTECTION_FAULT_ENABLE_DEFAULT,1),(DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT,1),(PDE0_PROTECTION_FAULT_ENABLE_DEFAULT,1),(VALID_PROTECTION_FAULT_ENABLE_DEFAULT,1),(READ_PROTECTION_FAULT_ENABLE_DEFAULT,1),(WRITE_PROTECTION_FAULT_ENABLE_DEFAULT,1),(EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT,1),(PAGE_TABLE_BLOCK_SIZE,(*adev).vm_manager.block_size-9)] { tmp=REG_SET_FIELD!(tmp,MMVM_CONTEXT1_CNTL,f,v); } tmp=REG_SET_FIELD!(tmp,MMVM_CONTEXT1_CNTL,RETRY_PERMISSION_OR_INVALID_PAGE_FAULT,!(*adev).gmc.noretry); WREG32_SOC15_OFFSET!(MMHUB,0,mmMMVM_CONTEXT1_CNTL,off,tmp); let ao=i*hub.ctx_addr_distance; WREG32_SOC15_OFFSET!(MMHUB,0,mmMMVM_CONTEXT1_PAGE_TABLE_START_ADDR_LO32,ao,0); WREG32_SOC15_OFFSET!(MMHUB,0,mmMMVM_CONTEXT1_PAGE_TABLE_START_ADDR_HI32,ao,0); WREG32_SOC15_OFFSET!(MMHUB,0,mmMMVM_CONTEXT1_PAGE_TABLE_END_ADDR_LO32,ao,lower_32_bits((*adev).vm_manager.max_pfn-1)); WREG32_SOC15_OFFSET!(MMHUB,0,mmMMVM_CONTEXT1_PAGE_TABLE_END_ADDR_HI32,ao,upper_32_bits((*adev).vm_manager.max_pfn-1)); } hub.vm_cntx_cntl=tmp; }

unsafe fn mmhub_v2_3_program_invalidation(adev:*mut amdgpu_device) { let hub=&mut (*adev).vmhub[AMDGPU_MMHUB0!(0)]; for i in 0..18 { let o=i*hub.eng_addr_distance; WREG32_SOC15_OFFSET!(MMHUB,0,mmMMVM_INVALIDATE_ENG0_ADDR_RANGE_LO32,o,0xffffffff); WREG32_SOC15_OFFSET!(MMHUB,0,mmMMVM_INVALIDATE_ENG0_ADDR_RANGE_HI32,o,0x1f); } }

unsafe fn mmhub_v2_3_gart_enable(adev:*mut amdgpu_device)->i32 { if amdgpu_sriov_vf!(adev) { WREG32_SOC15!(MMHUB,0,mmMMMC_VM_FB_LOCATION_BASE,(*adev).gmc.vram_start>>24); WREG32_SOC15!(MMHUB,0,mmMMMC_VM_FB_LOCATION_TOP,(*adev).gmc.vram_end>>24); } mmhub_v2_3_init_gart_aperture_regs(adev); mmhub_v2_3_init_system_aperture_regs(adev); mmhub_v2_3_init_tlb_regs(adev); mmhub_v2_3_init_cache_regs(adev); mmhub_v2_3_enable_system_domain(adev); mmhub_v2_3_disable_identity_aperture(adev); mmhub_v2_3_setup_vmid_config(adev); mmhub_v2_3_program_invalidation(adev); 0 }

unsafe fn mmhub_v2_3_gart_disable(adev:*mut amdgpu_device) { let hub=&mut (*adev).vmhub[AMDGPU_MMHUB0!(0)]; for i in 0..AMDGPU_NUM_VMID { WREG32_SOC15_OFFSET!(MMHUB,0,mmMMVM_CONTEXT0_CNTL,i*hub.ctx_distance,0); } let mut tmp=RREG32_SOC15!(MMHUB,0,mmMMMC_VM_MX_L1_TLB_CNTL); tmp=REG_SET_FIELD!(tmp,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_L1_TLB,0); tmp=REG_SET_FIELD!(tmp,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_ADVANCED_DRIVER_MODEL,0); WREG32_SOC15!(MMHUB,0,mmMMMC_VM_MX_L1_TLB_CNTL,tmp); tmp=REG_SET_FIELD!(RREG32_SOC15!(MMHUB,0,mmMMVM_L2_CNTL),MMVM_L2_CNTL,ENABLE_L2_CACHE,0); WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CNTL,tmp); WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CNTL3,0); }

unsafe fn mmhub_v2_3_set_fault_enable_default(adev:*mut amdgpu_device,value:bool) { let mut tmp=RREG32_SOC15!(MMHUB,0,mmMMVM_L2_PROTECTION_FAULT_CNTL); for f in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT,PDE0_PROTECTION_FAULT_ENABLE_DEFAULT,PDE1_PROTECTION_FAULT_ENABLE_DEFAULT,PDE2_PROTECTION_FAULT_ENABLE_DEFAULT,TRANSLATE_FURTHER_PROTECTION_FAULT_ENABLE_DEFAULT,NACK_PROTECTION_FAULT_ENABLE_DEFAULT,DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT,VALID_PROTECTION_FAULT_ENABLE_DEFAULT,READ_PROTECTION_FAULT_ENABLE_DEFAULT,WRITE_PROTECTION_FAULT_ENABLE_DEFAULT,EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT] { tmp=REG_SET_FIELD!(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,f,value); } if !value { tmp=REG_SET_FIELD!(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,CRASH_ON_NO_RETRY_FAULT,1); tmp=REG_SET_FIELD!(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,CRASH_ON_RETRY_FAULT,1); } WREG32_SOC15!(MMHUB,0,mmMMVM_L2_PROTECTION_FAULT_CNTL,tmp); }

static MMHUB_V2_3_VMHUB_FUNCS: amdgpu_vmhub_funcs = amdgpu_vmhub_funcs { print_l2_protection_fault_status: Some(mmhub_v2_3_print_l2_protection_fault_status), get_invalidate_req: Some(mmhub_v2_3_get_invalidate_req) };

unsafe fn mmhub_v2_3_init(adev:*mut amdgpu_device) { let hub=&mut (*adev).vmhub[AMDGPU_MMHUB0!(0)]; hub.ctx0_ptb_addr_lo32=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32); hub.ctx0_ptb_addr_hi32=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32); hub.vm_inv_eng0_sem=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_INVALIDATE_ENG0_SEM); hub.vm_inv_eng0_req=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_INVALIDATE_ENG0_REQ); hub.vm_inv_eng0_ack=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_INVALIDATE_ENG0_ACK); hub.vm_context0_cntl=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_CONTEXT0_CNTL); hub.vm_l2_pro_fault_status=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_L2_PROTECTION_FAULT_STATUS); hub.vm_l2_pro_fault_cntl=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_L2_PROTECTION_FAULT_CNTL); hub.ctx_distance=mmMMVM_CONTEXT1_CNTL-mmMMVM_CONTEXT0_CNTL; hub.ctx_addr_distance=mmMMVM_CONTEXT1_PAGE_TABLE_BASE_ADDR_LO32-mmMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32; hub.eng_distance=mmMMVM_INVALIDATE_ENG1_REQ-mmMMVM_INVALIDATE_ENG0_REQ; hub.eng_addr_distance=mmMMVM_INVALIDATE_ENG1_ADDR_RANGE_LO32-mmMMVM_INVALIDATE_ENG0_ADDR_RANGE_LO32; hub.vm_cntx_cntl_vm_fault=MMVM_CONTEXT1_CNTL__RANGE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|MMVM_CONTEXT1_CNTL__DUMMY_PAGE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|MMVM_CONTEXT1_CNTL__PDE0_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|MMVM_CONTEXT1_CNTL__VALID_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|MMVM_CONTEXT1_CNTL__READ_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|MMVM_CONTEXT1_CNTL__WRITE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|MMVM_CONTEXT1_CNTL__EXECUTE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK; hub.vmhub_funcs=&MMHUB_V2_3_VMHUB_FUNCS; amdgpu_mmhub_init_client_info!(&mut (*adev).mmhub,MMHUB_CLIENT_IDS_VANGOGH,ARRAY_SIZE!(MMHUB_CLIENT_IDS_VANGOGH)); }

unsafe fn mmhub_v2_3_update_medium_grain_clock_gating(adev:*mut amdgpu_device,enable:bool) { let mut data=RREG32_SOC15!(MMHUB,0,mmMM_ATC_L2_CGTT_CLK_CTRL); let def=data; let mut data1=RREG32_SOC15!(MMHUB,0,mmDAGB0_CNTL_MISC2); let def1=data1; let masks=DAGB0_CNTL_MISC2__DISABLE_WRREQ_CG_MASK|DAGB0_CNTL_MISC2__DISABLE_WRRET_CG_MASK|DAGB0_CNTL_MISC2__DISABLE_RDREQ_CG_MASK|DAGB0_CNTL_MISC2__DISABLE_RDRET_CG_MASK|DAGB0_CNTL_MISC2__DISABLE_TLBWR_CG_MASK|DAGB0_CNTL_MISC2__DISABLE_TLBRD_CG_MASK; if enable&&((*adev).cg_flags&AMD_CG_SUPPORT_MC_MGCG)!=0 { data &= !MM_ATC_L2_CGTT_CLK_CTRL__SOFT_OVERRIDE_MASK; data1 &= !masks; } else { data|=MM_ATC_L2_CGTT_CLK_CTRL__SOFT_OVERRIDE_MASK; data1|=masks; } if def!=data { WREG32_SOC15!(MMHUB,0,mmMM_ATC_L2_CGTT_CLK_CTRL,data); } if def1!=data1 { WREG32_SOC15!(MMHUB,0,mmDAGB0_CNTL_MISC2,data1); } }

unsafe fn mmhub_v2_3_update_medium_grain_light_sleep(_adev:*mut amdgpu_device,_enable:bool) { /* Direct register programming is supplied by the platform bindings. */ }
unsafe fn mmhub_v2_3_set_clockgating(adev:*mut amdgpu_device,state:amd_clockgating_state)->i32 { if amdgpu_sriov_vf!(adev){return 0;} mmhub_v2_3_update_medium_grain_clock_gating(adev,state==AMD_CG_STATE_GATE); mmhub_v2_3_update_medium_grain_light_sleep(adev,state==AMD_CG_STATE_GATE); 0 }
unsafe fn mmhub_v2_3_get_clockgating(adev:*mut amdgpu_device,flags:*mut u64) { if amdgpu_sriov_vf!(adev){*flags=0;} let data=RREG32_SOC15!(MMHUB,0,mmDAGB0_CNTL_MISC2); let data1=RREG32_SOC15!(MMHUB,0,mmMM_ATC_L2_CGTT_CLK_CTRL); if data&(DAGB0_CNTL_MISC2__DISABLE_WRREQ_CG_MASK|DAGB0_CNTL_MISC2__DISABLE_WRRET_CG_MASK|DAGB0_CNTL_MISC2__DISABLE_RDREQ_CG_MASK|DAGB0_CNTL_MISC2__DISABLE_RDRET_CG_MASK|DAGB0_CNTL_MISC2__DISABLE_TLBWR_CG_MASK|DAGB0_CNTL_MISC2__DISABLE_TLBRD_CG_MASK)==0&&data1&MM_ATC_L2_CGTT_CLK_CTRL__SOFT_OVERRIDE_MASK==0 {*flags|=AMD_CG_SUPPORT_MC_MGCG;} }

pub static MMHUB_V2_3_FUNCS: amdgpu_mmhub_funcs = amdgpu_mmhub_funcs { init:Some(mmhub_v2_3_init), gart_enable:Some(mmhub_v2_3_gart_enable), set_fault_enable_default:Some(mmhub_v2_3_set_fault_enable_default), gart_disable:Some(mmhub_v2_3_gart_disable), set_clockgating:Some(mmhub_v2_3_set_clockgating), get_clockgating:Some(mmhub_v2_3_get_clockgating), setup_vm_pt_regs:Some(mmhub_v2_3_setup_vm_pt_regs) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
