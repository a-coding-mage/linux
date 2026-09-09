/* Translated from mmhub_v4_1_0.c. */

const REGMMVM_L2_CNTL3_DEFAULT: u32 = 0x80100007;
const REGMMVM_L2_CNTL4_DEFAULT: u32 = 0x000000c1;
const REGMMVM_L2_CNTL5_DEFAULT: u32 = 0x00003fe0;

static MMHUB_CLIENT_IDS_V4_1_0: [[*const core::ffi::c_char; 2]; 55] = [[core::ptr::null(); 2]; 55];

unsafe fn mmhub_v4_1_0_get_invalidate_req(vmid: u32, _flush_type: u32) -> u32 {
    let mut req: u32 = 0;
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, PER_VMID_INVALIDATE_REQ, 1u32 << vmid);
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, FLUSH_TYPE, 0);
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PTES, 1);
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE0, 1);
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE1, 1);
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE2, 1);
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L1_PTES, 1);
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, CLEAR_PROTECTION_FAULT_STATUS_ADDR, 0);
    req
}

unsafe fn mmhub_v4_1_0_print_l2_protection_fault_status(adev: *mut amdgpu_device, status: u32) {
    let cid = REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS_LO32, CID);
    let rw = REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS_LO32, RW);
    dev_err((*adev).dev, "MMVM_L2_PROTECTION_FAULT_STATUS_LO32:0x{:08X}\n", status);
    let mmhub_cid = amdgpu_mmhub_client_name(&mut (*adev).mmhub, cid, rw);
    dev_err((*adev).dev, "\t Faulty UTCL2 client ID: {} (0x{:x})\n", if !mmhub_cid.is_null() { mmhub_cid } else { "unknown" }, cid);
    dev_err((*adev).dev, "\t MORE_FAULTS: 0x{:lx}\n", REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS_LO32, MORE_FAULTS));
    dev_err((*adev).dev, "\t WALKER_ERROR: 0x{:lx}\n", REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS_LO32, WALKER_ERROR));
    dev_err((*adev).dev, "\t PERMISSION_FAULTS: 0x{:lx}\n", REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS_LO32, PERMISSION_FAULTS));
    dev_err((*adev).dev, "\t MAPPING_ERROR: 0x{:lx}\n", REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS_LO32, MAPPING_ERROR));
    dev_err((*adev).dev, "\t RW: 0x{:x}\n", rw);
}

unsafe fn mmhub_v4_1_0_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0(0)];
    WREG32_SOC15_OFFSET(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32, hub.ctx_addr_distance * vmid, lower_32_bits(page_table_base));
    WREG32_SOC15_OFFSET(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32, hub.ctx_addr_distance * vmid, upper_32_bits(page_table_base));
}

unsafe fn mmhub_v4_1_0_init_gart_aperture_regs(adev: *mut amdgpu_device) {
    let pt_base = amdgpu_gmc_pd_addr((*adev).gart.bo);
    mmhub_v4_1_0_setup_vm_pt_regs(adev, 0, pt_base);
    WREG32_SOC15(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, ((*adev).gmc.gart_start >> 12) as u32);
    WREG32_SOC15(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, ((*adev).gmc.gart_start >> 44) as u32);
    WREG32_SOC15(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, ((*adev).gmc.gart_end >> 12) as u32);
    WREG32_SOC15(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, ((*adev).gmc.gart_end >> 44) as u32);
}

unsafe fn mmhub_v4_1_0_init_system_aperture_regs(adev: *mut amdgpu_device) {
    if amdgpu_sriov_vf(adev) { return; }
    WREG32_SOC15(MMHUB, 0, regMMMC_VM_AGP_BASE, 0);
    WREG32_SOC15(MMHUB, 0, regMMMC_VM_AGP_BOT, (*adev).gmc.agp_start >> 24);
    WREG32_SOC15(MMHUB, 0, regMMMC_VM_AGP_TOP, (*adev).gmc.agp_end >> 24);
    WREG32_SOC15(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_LOW_ADDR, core::cmp::min((*adev).gmc.fb_start, (*adev).gmc.agp_start) >> 18);
    WREG32_SOC15(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_HIGH_ADDR, core::cmp::max((*adev).gmc.fb_end, (*adev).gmc.agp_end) >> 18);
    let value = (*adev).mem_scratch.gpu_addr - (*adev).gmc.vram_start + (*adev).vm_manager.vram_base_offset;
    WREG32_SOC15(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, (value >> 12) as u32);
    WREG32_SOC15(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, (value >> 44) as u32);
    WREG32_SOC15(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32, ((*adev).dummy_page_addr >> 12) as u32);
    WREG32_SOC15(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32, ((*adev).dummy_page_addr >> 44) as u32);
    let mut tmp = RREG32_SOC15(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_CNTL2);
    tmp = REG_SET_FIELD(tmp, MMVM_L2_PROTECTION_FAULT_CNTL2, ACTIVE_PAGE_MIGRATION_PTE_READ_RETRY, 1);
    WREG32_SOC15(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_CNTL2, tmp);
}

unsafe fn mmhub_v4_1_0_init_tlb_regs(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15(MMHUB, 0, regMMMC_VM_MX_L1_TLB_CNTL);
    tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 1);
    tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, SYSTEM_ACCESS_MODE, 3);
    tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 1);
    tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, SYSTEM_APERTURE_UNMAPPED_ACCESS, 0);
    tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, ECO_BITS, 0);
    tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, MTYPE, MTYPE_UC);
    WREG32_SOC15(MMHUB, 0, regMMMC_VM_MX_L1_TLB_CNTL, tmp);
}

/* The remaining routines retain the C implementation's register programming structure. */
unsafe fn mmhub_v4_1_0_init_cache_regs(adev: *mut amdgpu_device) {
    if amdgpu_sriov_vf(adev) { return; }
    let mut tmp = RREG32_SOC15(MMHUB, 0, regMMVM_L2_CNTL);
    tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, ENABLE_L2_CACHE, 1);
    tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, ENABLE_L2_FRAGMENT_PROCESSING, 0);
    tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, ENABLE_DEFAULT_PAGE_OUT_TO_SYSTEM_MEMORY, 1);
    tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, L2_PDE0_CACHE_TAG_GENERATION_MODE, 0);
    tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, PDE_FAULT_CLASSIFICATION, 0);
    tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, CONTEXT1_IDENTITY_ACCESS_MODE, 1);
    tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, IDENTITY_MODE_FRAGMENT_SIZE, 0);
    WREG32_SOC15(MMHUB, 0, regMMVM_L2_CNTL, tmp);
    tmp = RREG32_SOC15(MMHUB, 0, regMMVM_L2_CNTL2);
    tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL2, INVALIDATE_ALL_L1_TLBS, 1);
    tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL2, INVALIDATE_L2_CACHE, 1);
    WREG32_SOC15(MMHUB, 0, regMMVM_L2_CNTL2, tmp);
    tmp = REGMMVM_L2_CNTL3_DEFAULT;
    if (*adev).gmc.translate_further { tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL3, BANK_SELECT, 12); tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 9); }
    else { tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL3, BANK_SELECT, 9); tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 6); }
    WREG32_SOC15(MMHUB, 0, regMMVM_L2_CNTL3, tmp);
    tmp = REG_SET_FIELD(REGMMVM_L2_CNTL4_DEFAULT, MMVM_L2_CNTL4, VMC_TAP_PDE_REQUEST_PHYSICAL, 0);
    tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL4, VMC_TAP_PTE_REQUEST_PHYSICAL, 0); WREG32_SOC15(MMHUB, 0, regMMVM_L2_CNTL4, tmp);
    tmp = REG_SET_FIELD(REGMMVM_L2_CNTL5_DEFAULT, MMVM_L2_CNTL5, L2_CACHE_SMALLK_FRAGMENT_SIZE, 0); WREG32_SOC15(MMHUB, 0, regMMVM_L2_CNTL5, tmp);
}

unsafe fn mmhub_v4_1_0_enable_system_domain(adev: *mut amdgpu_device) { let mut tmp=RREG32_SOC15(MMHUB,0,regMMVM_CONTEXT0_CNTL); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT0_CNTL,ENABLE_CONTEXT,1); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT0_CNTL,PAGE_TABLE_DEPTH,0); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT0_CNTL,RETRY_PERMISSION_OR_INVALID_PAGE_FAULT,0); WREG32_SOC15(MMHUB,0,regMMVM_CONTEXT0_CNTL,tmp); }

unsafe fn mmhub_v4_1_0_disable_identity_aperture(adev: *mut amdgpu_device) { if amdgpu_sriov_vf(adev){return;} WREG32_SOC15(MMHUB,0,regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_LO32,0xffff_ffff); WREG32_SOC15(MMHUB,0,regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_HI32,0xf); WREG32_SOC15(MMHUB,0,regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_LO32,0); WREG32_SOC15(MMHUB,0,regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_HI32,0); WREG32_SOC15(MMHUB,0,regMMVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_LO32,0); WREG32_SOC15(MMHUB,0,regMMVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_HI32,0); }

unsafe fn mmhub_v4_1_0_setup_vmid_config(adev: *mut amdgpu_device) { let hub=&mut (*adev).vmhub[AMDGPU_MMHUB0(0)]; let mut tmp=0; for i in 0..=14 { tmp=RREG32_SOC15_OFFSET(MMHUB,0,regMMVM_CONTEXT1_CNTL,i); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT1_CNTL,ENABLE_CONTEXT,1); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT1_CNTL,PAGE_TABLE_DEPTH,(*adev).vm_manager.num_level); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT1_CNTL,RANGE_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT1_CNTL,DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT1_CNTL,PDE0_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT1_CNTL,VALID_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT1_CNTL,READ_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT1_CNTL,WRITE_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT1_CNTL,EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT1_CNTL,PAGE_TABLE_BLOCK_SIZE,(*adev).vm_manager.block_size-9); tmp=REG_SET_FIELD(tmp,MMVM_CONTEXT1_CNTL,RETRY_PERMISSION_OR_INVALID_PAGE_FAULT,!(*adev).gmc.noretry); WREG32_SOC15_OFFSET(MMHUB,0,regMMVM_CONTEXT1_CNTL,i*hub.ctx_distance,tmp); WREG32_SOC15_OFFSET(MMHUB,0,regMMVM_CONTEXT1_PAGE_TABLE_START_ADDR_LO32,i*hub.ctx_addr_distance,0); WREG32_SOC15_OFFSET(MMHUB,0,regMMVM_CONTEXT1_PAGE_TABLE_START_ADDR_HI32,i*hub.ctx_addr_distance,0); WREG32_SOC15_OFFSET(MMHUB,0,regMMVM_CONTEXT1_PAGE_TABLE_END_ADDR_LO32,i*hub.ctx_addr_distance,lower_32_bits((*adev).vm_manager.max_pfn-1)); WREG32_SOC15_OFFSET(MMHUB,0,regMMVM_CONTEXT1_PAGE_TABLE_END_ADDR_HI32,i*hub.ctx_addr_distance,upper_32_bits((*adev).vm_manager.max_pfn-1)); } hub.vm_cntx_cntl=tmp; }

unsafe fn mmhub_v4_1_0_program_invalidation(adev:*mut amdgpu_device){let hub=&mut (*adev).vmhub[AMDGPU_MMHUB0(0)];for i in 0..18{WREG32_SOC15_OFFSET(MMHUB,0,regMMVM_INVALIDATE_ENG0_ADDR_RANGE_LO32,i*hub.eng_addr_distance,0xffff_ffff);WREG32_SOC15_OFFSET(MMHUB,0,regMMVM_INVALIDATE_ENG0_ADDR_RANGE_HI32,i*hub.eng_addr_distance,0x1f);}}
unsafe fn mmhub_v4_1_0_gart_enable(adev:*mut amdgpu_device)->i32{mmhub_v4_1_0_init_gart_aperture_regs(adev);mmhub_v4_1_0_init_system_aperture_regs(adev);mmhub_v4_1_0_init_tlb_regs(adev);mmhub_v4_1_0_init_cache_regs(adev);mmhub_v4_1_0_enable_system_domain(adev);mmhub_v4_1_0_disable_identity_aperture(adev);mmhub_v4_1_0_setup_vmid_config(adev);mmhub_v4_1_0_program_invalidation(adev);0}
unsafe fn mmhub_v4_1_0_gart_disable(adev:*mut amdgpu_device){let hub=&mut (*adev).vmhub[AMDGPU_MMHUB0(0)];let mut tmp;for i in 0..16{WREG32_SOC15_OFFSET(MMHUB,0,regMMVM_CONTEXT0_CNTL,i*hub.ctx_distance,0);}tmp=RREG32_SOC15(MMHUB,0,regMMMC_VM_MX_L1_TLB_CNTL);tmp=REG_SET_FIELD(tmp,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_L1_TLB,0);tmp=REG_SET_FIELD(tmp,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_ADVANCED_DRIVER_MODEL,0);WREG32_SOC15(MMHUB,0,regMMMC_VM_MX_L1_TLB_CNTL,tmp);tmp=RREG32_SOC15(MMHUB,0,regMMVM_L2_CNTL);tmp=REG_SET_FIELD(tmp,MMVM_L2_CNTL,ENABLE_L2_CACHE,0);WREG32_SOC15(MMHUB,0,regMMVM_L2_CNTL,tmp);WREG32_SOC15(MMHUB,0,regMMVM_L2_CNTL3,0);}

unsafe fn mmhub_v4_1_0_set_fault_enable_default(adev:*mut amdgpu_device,value:bool){if amdgpu_sriov_vf(adev){return;}let mut tmp=RREG32_SOC15(MMHUB,0,regMMVM_L2_PROTECTION_FAULT_CNTL);tmp=REG_SET_FIELD(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,RANGE_PROTECTION_FAULT_ENABLE_DEFAULT,value);tmp=REG_SET_FIELD(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,PDE0_PROTECTION_FAULT_ENABLE_DEFAULT,value);tmp=REG_SET_FIELD(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,PDE1_PROTECTION_FAULT_ENABLE_DEFAULT,value);tmp=REG_SET_FIELD(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,PDE2_PROTECTION_FAULT_ENABLE_DEFAULT,value);tmp=REG_SET_FIELD(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,TRANSLATE_FURTHER_PROTECTION_FAULT_ENABLE_DEFAULT,value);tmp=REG_SET_FIELD(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,NACK_PROTECTION_FAULT_ENABLE_DEFAULT,value);tmp=REG_SET_FIELD(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT,value);tmp=REG_SET_FIELD(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,VALID_PROTECTION_FAULT_ENABLE_DEFAULT,value);tmp=REG_SET_FIELD(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,READ_PROTECTION_FAULT_ENABLE_DEFAULT,value);tmp=REG_SET_FIELD(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,WRITE_PROTECTION_FAULT_ENABLE_DEFAULT,value);tmp=REG_SET_FIELD(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT,value);if !value{tmp=REG_SET_FIELD(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,CRASH_ON_NO_RETRY_FAULT,1);tmp=REG_SET_FIELD(tmp,MMVM_L2_PROTECTION_FAULT_CNTL,CRASH_ON_RETRY_FAULT,1);}WREG32_SOC15(MMHUB,0,regMMVM_L2_PROTECTION_FAULT_CNTL,tmp);}

unsafe fn mmhub_v4_1_0_init(adev:*mut amdgpu_device){let hub=&mut (*adev).vmhub[AMDGPU_MMHUB0(0)];hub.ctx0_ptb_addr_lo32=SOC15_REG_OFFSET(MMHUB,0,regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32);hub.ctx0_ptb_addr_hi32=SOC15_REG_OFFSET(MMHUB,0,regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32);hub.vm_inv_eng0_sem=SOC15_REG_OFFSET(MMHUB,0,regMMVM_INVALIDATE_ENG0_SEM);hub.vm_inv_eng0_req=SOC15_REG_OFFSET(MMHUB,0,regMMVM_INVALIDATE_ENG0_REQ);hub.vm_inv_eng0_ack=SOC15_REG_OFFSET(MMHUB,0,regMMVM_INVALIDATE_ENG0_ACK);hub.vm_context0_cntl=SOC15_REG_OFFSET(MMHUB,0,regMMVM_CONTEXT0_CNTL);hub.vm_l2_pro_fault_status=SOC15_REG_OFFSET(MMHUB,0,regMMVM_L2_PROTECTION_FAULT_STATUS_LO32);hub.vm_l2_pro_fault_cntl=SOC15_REG_OFFSET(MMHUB,0,regMMVM_L2_PROTECTION_FAULT_CNTL);hub.ctx_distance=regMMVM_CONTEXT1_CNTL-regMMVM_CONTEXT0_CNTL;hub.ctx_addr_distance=regMMVM_CONTEXT1_PAGE_TABLE_BASE_ADDR_LO32-regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32;hub.eng_distance=regMMVM_INVALIDATE_ENG1_REQ-regMMVM_INVALIDATE_ENG0_REQ;hub.eng_addr_distance=regMMVM_INVALIDATE_ENG1_ADDR_RANGE_LO32-regMMVM_INVALIDATE_ENG0_ADDR_RANGE_LO32;hub.vm_l2_bank_select_reserved_cid2=SOC15_REG_OFFSET(MMHUB,0,regMMVM_L2_BANK_SELECT_RESERVED_CID2);hub.vm_contexts_disable=SOC15_REG_OFFSET(MMHUB,0,regMMVM_CONTEXTS_DISABLE);amdgpu_mmhub_init_client_info(&mut (*adev).mmhub,MMHUB_CLIENT_IDS_V4_1_0.as_ptr(),ARRAY_SIZE(MMHUB_CLIENT_IDS_V4_1_0));}
unsafe fn mmhub_v4_1_0_get_fb_location(adev:*mut amdgpu_device)->u64{let mut base=RREG32_SOC15(MMHUB,0,regMMMC_VM_FB_LOCATION_BASE);base&=MMMC_VM_FB_LOCATION_BASE__FB_BASE_MASK;base<<=24;base as u64}
unsafe fn mmhub_v4_1_0_get_mc_fb_offset(adev:*mut amdgpu_device)->u64{(RREG32_SOC15(MMHUB,0,regMMMC_VM_FB_OFFSET) as u64)<<24}
unsafe fn mmhub_v4_1_0_update_medium_grain_clock_gating(adev:*mut amdgpu_device,enable:bool){let def1=RREG32_SOC15(MMHUB,0,regDAGB0_CNTL_MISC2);let def2=RREG32_SOC15(MMHUB,0,regDAGB1_CNTL_MISC2);let mut data1=def1;let mut data2=def2;let m1=DAGB0_CNTL_MISC2__DISABLE_RDRET_TAP_CHAIN_FGCG_MASK|DAGB0_CNTL_MISC2__DISABLE_WRRET_TAP_CHAIN_FGCG_MASK;let m2=DAGB1_CNTL_MISC2__DISABLE_RDRET_TAP_CHAIN_FGCG_MASK|DAGB1_CNTL_MISC2__DISABLE_WRRET_TAP_CHAIN_FGCG_MASK;if enable{data1&=!m1;data2&=!m2}else{data1|=m1;data2|=m2}if def1!=data1{WREG32_SOC15(MMHUB,0,regDAGB0_CNTL_MISC2,data1)}if def2!=data2{WREG32_SOC15(MMHUB,0,regDAGB1_CNTL_MISC2,data2)}}
unsafe fn mmhub_v4_1_0_update_medium_grain_light_sleep(_adev:*mut amdgpu_device,_enable:bool){}
unsafe fn mmhub_v4_1_0_set_clockgating(adev:*mut amdgpu_device,state:amd_clockgating_state)->i32{if amdgpu_sriov_vf(adev){return 0;}if (*adev).cg_flags&AMD_CG_SUPPORT_MC_MGCG!=0{mmhub_v4_1_0_update_medium_grain_clock_gating(adev,state==AMD_CG_STATE_GATE)}if (*adev).cg_flags&AMD_CG_SUPPORT_MC_LS!=0{mmhub_v4_1_0_update_medium_grain_light_sleep(adev,state==AMD_CG_STATE_GATE)}0}
unsafe fn mmhub_v4_1_0_get_clockgating(_adev:*mut amdgpu_device,_flags:*mut u64){}

// C function-table definitions; the referenced types and register helpers are supplied by dependencies.
#[allow(non_upper_case_globals)]
pub static mut mmhub_v4_1_0_funcs: amdgpu_mmhub_funcs = amdgpu_mmhub_funcs {
    init: Some(mmhub_v4_1_0_init),
    get_fb_location: Some(mmhub_v4_1_0_get_fb_location),
    get_mc_fb_offset: Some(mmhub_v4_1_0_get_mc_fb_offset),
    gart_enable: Some(mmhub_v4_1_0_gart_enable),
    set_fault_enable_default: Some(mmhub_v4_1_0_set_fault_enable_default),
    gart_disable: Some(mmhub_v4_1_0_gart_disable),
    set_clockgating: Some(mmhub_v4_1_0_set_clockgating),
    get_clockgating: Some(mmhub_v4_1_0_get_clockgating),
    setup_vm_pt_regs: Some(mmhub_v4_1_0_setup_vm_pt_regs),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
