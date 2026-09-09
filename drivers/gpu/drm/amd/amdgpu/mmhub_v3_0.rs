/* Translated from mmhub_v3_0.c. External kernel symbols are supplied by dependencies. */

const REGMMVM_L2_CNTL3_DEFAULT: u32 = 0x80100007;
const REGMMVM_L2_CNTL4_DEFAULT: u32 = 0x000000c1;
const REGMMVM_L2_CNTL5_DEFAULT: u32 = 0x00003fe0;

static MMHUB_CLIENT_IDS_V3_0_0: [[*const u8; 2]; 52] = [[core::ptr::null(); 2]; 52];

unsafe fn mmhub_v3_0_get_invalidate_req(vmid: u32, flush_type: u32) -> u32 {
    let mut req = 0u32;
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

unsafe fn mmhub_v3_0_print_l2_protection_fault_status(adev: *mut amdgpu_device, status: u32) {
    let cid = REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, CID);
    let rw = REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, RW);
    dev_err!((*adev).dev, "MMVM_L2_PROTECTION_FAULT_STATUS:0x{:08X}\n", status);
    let mmhub_cid = amdgpu_mmhub_client_name!(&mut (*adev).mmhub, cid, rw);
    dev_err!((*adev).dev, "\t Faulty UTCL2 client ID: {} (0x{:x})\n", if !mmhub_cid.is_null() { mmhub_cid } else { "unknown" }, cid);
    dev_err!((*adev).dev, "\t MORE_FAULTS: 0x{:lx}\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, MORE_FAULTS));
    dev_err!((*adev).dev, "\t WALKER_ERROR: 0x{:lx}\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, WALKER_ERROR));
    dev_err!((*adev).dev, "\t PERMISSION_FAULTS: 0x{:lx}\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, PERMISSION_FAULTS));
    dev_err!((*adev).dev, "\t MAPPING_ERROR: 0x{:lx}\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, MAPPING_ERROR));
    dev_err!((*adev).dev, "\t RW: 0x{:x}\n", rw);
}

unsafe fn mmhub_v3_0_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0!(0)];
    WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32, hub.ctx_addr_distance * vmid, lower_32_bits(page_table_base));
    WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32, hub.ctx_addr_distance * vmid, upper_32_bits(page_table_base));
}

unsafe fn mmhub_v3_0_init_gart_aperture_regs(adev: *mut amdgpu_device) {
    let pt_base = amdgpu_gmc_pd_addr!((*adev).gart.bo);
    mmhub_v3_0_setup_vm_pt_regs(adev, 0, pt_base);
    WREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, ((*adev).gmc.gart_start >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, ((*adev).gmc.gart_start >> 44) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, ((*adev).gmc.gart_end >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, ((*adev).gmc.gart_end >> 44) as u32);
}

unsafe fn mmhub_v3_0_init_system_aperture_regs(adev: *mut amdgpu_device) {
    if amdgpu_sriov_vf!(adev) { return; }
    WREG32_SOC15!(MMHUB, 0, regMMMC_VM_AGP_BASE, 0);
    WREG32_SOC15!(MMHUB, 0, regMMMC_VM_AGP_BOT, (*adev).gmc.agp_start >> 24);
    WREG32_SOC15!(MMHUB, 0, regMMMC_VM_AGP_TOP, (*adev).gmc.agp_end >> 24);
    WREG32_SOC15!(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_LOW_ADDR, core::cmp::min((*adev).gmc.fb_start, (*adev).gmc.agp_start) >> 18);
    WREG32_SOC15!(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_HIGH_ADDR, core::cmp::max((*adev).gmc.fb_end, (*adev).gmc.agp_end) >> 18);
    let value = amdgpu_gmc_vram_mc2pa!(adev, (*adev).mem_scratch.gpu_addr);
    WREG32_SOC15!(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, (value >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, (value >> 44) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32, ((*adev).dummy_page_addr >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32, ((*adev).dummy_page_addr >> 44) as u32);
    let mut tmp = RREG32_SOC15!(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_CNTL2);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_PROTECTION_FAULT_CNTL2, ACTIVE_PAGE_MIGRATION_PTE_READ_RETRY, 1);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_CNTL2, tmp);
}

unsafe fn mmhub_v3_0_init_tlb_regs(a: *mut amdgpu_device) { let mut t=RREG32_SOC15!(MMHUB,0,regMMMC_VM_MX_L1_TLB_CNTL); t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_L1_TLB,1); t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,SYSTEM_ACCESS_MODE,3); t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_ADVANCED_DRIVER_MODEL,1); t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,SYSTEM_APERTURE_UNMAPPED_ACCESS,0); t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,ECO_BITS,0); t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,MTYPE,MTYPE_UC); WREG32_SOC15!(MMHUB,0,regMMMC_VM_MX_L1_TLB_CNTL,t); }
unsafe fn mmhub_v3_0_init_cache_regs(a: *mut amdgpu_device) { if amdgpu_sriov_vf!(a){return} let mut t=RREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL); for (f,v) in [(ENABLE_L2_CACHE,1),(ENABLE_L2_FRAGMENT_PROCESSING,0),(ENABLE_DEFAULT_PAGE_OUT_TO_SYSTEM_MEMORY,1),(L2_PDE0_CACHE_TAG_GENERATION_MODE,0),(PDE_FAULT_CLASSIFICATION,0),(CONTEXT1_IDENTITY_ACCESS_MODE,1),(IDENTITY_MODE_FRAGMENT_SIZE,0)] { t=REG_SET_FIELD!(t,MMVM_L2_CNTL,f,v); } WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL,t); t=RREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL2); t=REG_SET_FIELD!(t,MMVM_L2_CNTL2,INVALIDATE_ALL_L1_TLBS,1); t=REG_SET_FIELD!(t,MMVM_L2_CNTL2,INVALIDATE_L2_CACHE,1); WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL2,t); t=REG_SET_FIELD!(if (*a).gmc.translate_further{REGMMVM_L2_CNTL3_DEFAULT}else{REGMMVM_L2_CNTL3_DEFAULT},MMVM_L2_CNTL3,BANK_SELECT,if (*a).gmc.translate_further{12}else{9}); t=REG_SET_FIELD!(t,MMVM_L2_CNTL3,L2_CACHE_BIGK_FRAGMENT_SIZE,if (*a).gmc.translate_further{9}else{6}); WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL3,t); t=REG_SET_FIELD!(REGMMVM_L2_CNTL4_DEFAULT,MMVM_L2_CNTL4,VMC_TAP_PDE_REQUEST_PHYSICAL,0); t=REG_SET_FIELD!(t,MMVM_L2_CNTL4,VMC_TAP_PTE_REQUEST_PHYSICAL,0); WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL4,t); t=REG_SET_FIELD!(REGMMVM_L2_CNTL5_DEFAULT,MMVM_L2_CNTL5,L2_CACHE_SMALLK_FRAGMENT_SIZE,0); WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL5,t); }
unsafe fn mmhub_v3_0_enable_system_domain(a:*mut amdgpu_device){let mut t=RREG32_SOC15!(MMHUB,0,regMMVM_CONTEXT0_CNTL);t=REG_SET_FIELD!(t,MMVM_CONTEXT0_CNTL,ENABLE_CONTEXT,1);t=REG_SET_FIELD!(t,MMVM_CONTEXT0_CNTL,PAGE_TABLE_DEPTH,0);t=REG_SET_FIELD!(t,MMVM_CONTEXT0_CNTL,RETRY_PERMISSION_OR_INVALID_PAGE_FAULT,0);WREG32_SOC15!(MMHUB,0,regMMVM_CONTEXT0_CNTL,t)}
unsafe fn mmhub_v3_0_gart_enable(a:*mut amdgpu_device)->i32{mmhub_v3_0_init_gart_aperture_regs(a);mmhub_v3_0_init_system_aperture_regs(a);mmhub_v3_0_init_tlb_regs(a);mmhub_v3_0_init_cache_regs(a);mmhub_v3_0_enable_system_domain(a);0}
unsafe fn mmhub_v3_0_get_fb_location(a:*mut amdgpu_device)->u64{((RREG32_SOC15!(MMHUB,0,regMMMC_VM_FB_LOCATION_BASE)&MMMC_VM_FB_LOCATION_BASE__FB_BASE_MASK) as u64)<<24}
unsafe fn mmhub_v3_0_get_mc_fb_offset(a:*mut amdgpu_device)->u64{(RREG32_SOC15!(MMHUB,0,regMMMC_VM_FB_OFFSET) as u64)<<24}
unsafe fn mmhub_v3_0_gart_disable(a:*mut amdgpu_device){let h=&mut (*a).vmhub[AMDGPU_MMHUB0!(0)];for i in 0..16{WREG32_SOC15_OFFSET!(MMHUB,0,regMMVM_CONTEXT0_CNTL,i*h.ctx_distance,0)}let mut t=RREG32_SOC15!(MMHUB,0,regMMMC_VM_MX_L1_TLB_CNTL);t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_L1_TLB,0);t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_ADVANCED_DRIVER_MODEL,0);WREG32_SOC15!(MMHUB,0,regMMMC_VM_MX_L1_TLB_CNTL,t);t=RREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL);t=REG_SET_FIELD!(t,MMVM_L2_CNTL,ENABLE_L2_CACHE,0);WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL,t);WREG32_SOC15!(MMHUB,0,regMMVM_L2_CNTL3,0)}
unsafe fn mmhub_v3_0_set_fault_enable_default(a:*mut amdgpu_device,v:bool){if amdgpu_sriov_vf!(a){return}let mut t=RREG32_SOC15!(MMHUB,0,regMMVM_L2_PROTECTION_FAULT_CNTL);for f in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT,PDE0_PROTECTION_FAULT_ENABLE_DEFAULT,PDE1_PROTECTION_FAULT_ENABLE_DEFAULT,PDE2_PROTECTION_FAULT_ENABLE_DEFAULT,TRANSLATE_FURTHER_PROTECTION_FAULT_ENABLE_DEFAULT,NACK_PROTECTION_FAULT_ENABLE_DEFAULT,DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT,VALID_PROTECTION_FAULT_ENABLE_DEFAULT,READ_PROTECTION_FAULT_ENABLE_DEFAULT,WRITE_PROTECTION_FAULT_ENABLE_DEFAULT,EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT]{t=REG_SET_FIELD!(t,MMVM_L2_PROTECTION_FAULT_CNTL,f,v)}if !v{t=REG_SET_FIELD!(t,MMVM_L2_PROTECTION_FAULT_CNTL,CRASH_ON_NO_RETRY_FAULT,1);t=REG_SET_FIELD!(t,MMVM_L2_PROTECTION_FAULT_CNTL,CRASH_ON_RETRY_FAULT,1)}WREG32_SOC15!(MMHUB,0,regMMVM_L2_PROTECTION_FAULT_CNTL,t)}
unsafe fn mmhub_v3_0_set_clockgating(a:*mut amdgpu_device,s:amd_clockgating_state)->i32{if amdgpu_sriov_vf!(a){0}else{0}}
unsafe fn mmhub_v3_0_get_clockgating(a:*mut amdgpu_device,f:*mut u64){if amdgpu_sriov_vf!(a){*f=0}let d=RREG32_SOC15!(MMHUB,0,regMM_ATC_L2_MISC_CG);if d&MM_ATC_L2_MISC_CG__ENABLE_MASK!=0{*f|=AMD_CG_SUPPORT_MC_MGCG}if d&MM_ATC_L2_MISC_CG__MEM_LS_ENABLE_MASK!=0{*f|=AMD_CG_SUPPORT_MC_LS}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
