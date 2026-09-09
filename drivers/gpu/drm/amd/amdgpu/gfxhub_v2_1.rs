/* Translated from gfxhub_v2_1.c. External register and driver symbols are
 * supplied by the surrounding kernel translation. */

pub const mmGCUTCL2_HARVEST_BYPASS_GROUPS_YELLOW_CARP: u32 = 0x16f8;
pub const mmGCUTCL2_HARVEST_BYPASS_GROUPS_YELLOW_CARP_BASE_IDX: u32 = 0;

static GFXHUB_CLIENT_IDS: [&str; 18] = ["CB/DB", "Reserved", "GE1", "GE2", "CPF", "CPC", "CPG", "RLC", "TCP", "SQC (inst)", "SQC (data)", "SQG", "Reserved", "SDMA0", "SDMA1", "GCR", "SDMA2", "SDMA3"];

unsafe fn gfxhub_v2_1_get_invalidate_req(adev_vmid: u32, flush_type: u32) -> u32 {
    let mut req: u32 = 0;
    req = REG_SET_FIELD(req, GCVM_INVALIDATE_ENG0_REQ, PER_VMID_INVALIDATE_REQ, 1u32.wrapping_shl(adev_vmid));
    req = REG_SET_FIELD(req, GCVM_INVALIDATE_ENG0_REQ, FLUSH_TYPE, flush_type);
    req = REG_SET_FIELD(req, GCVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PTES, 1);
    req = REG_SET_FIELD(req, GCVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE0, 1);
    req = REG_SET_FIELD(req, GCVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE1, 1);
    req = REG_SET_FIELD(req, GCVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE2, 1);
    req = REG_SET_FIELD(req, GCVM_INVALIDATE_ENG0_REQ, INVALIDATE_L1_PTES, 1);
    req = REG_SET_FIELD(req, GCVM_INVALIDATE_ENG0_REQ, CLEAR_PROTECTION_FAULT_STATUS_ADDR, 0);
    req
}

unsafe fn gfxhub_v2_1_print_l2_protection_fault_status(adev: *mut amdgpu_device, status: u32) {
    let cid = REG_GET_FIELD(status, GCVM_L2_PROTECTION_FAULT_STATUS, CID);
    dev_err((*adev).dev, "GCVM_L2_PROTECTION_FAULT_STATUS:0x{:08X}\n", status);
    dev_err((*adev).dev, "\t Faulty UTCL2 client ID: {} (0x{:x})\n", if cid >= GFXHUB_CLIENT_IDS.len() { "unknown" } else { GFXHUB_CLIENT_IDS[cid as usize] }, cid);
    dev_err((*adev).dev, "\t MORE_FAULTS: 0x{:x}\n", REG_GET_FIELD(status, GCVM_L2_PROTECTION_FAULT_STATUS, MORE_FAULTS));
    dev_err((*adev).dev, "\t WALKER_ERROR: 0x{:x}\n", REG_GET_FIELD(status, GCVM_L2_PROTECTION_FAULT_STATUS, WALKER_ERROR));
    dev_err((*adev).dev, "\t PERMISSION_FAULTS: 0x{:x}\n", REG_GET_FIELD(status, GCVM_L2_PROTECTION_FAULT_STATUS, PERMISSION_FAULTS));
    dev_err((*adev).dev, "\t MAPPING_ERROR: 0x{:x}\n", REG_GET_FIELD(status, GCVM_L2_PROTECTION_FAULT_STATUS, MAPPING_ERROR));
    dev_err((*adev).dev, "\t RW: 0x{:x}\n", REG_GET_FIELD(status, GCVM_L2_PROTECTION_FAULT_STATUS, RW));
}

unsafe fn gfxhub_v2_1_get_fb_location(adev: *mut amdgpu_device) -> u64 {
    let mut base = RREG32_SOC15(GC, 0, mmGCMC_VM_FB_LOCATION_BASE);
    base &= GCMC_VM_FB_LOCATION_BASE__FB_BASE_MASK; base << 24
}
unsafe fn gfxhub_v2_1_get_mc_fb_offset(adev: *mut amdgpu_device) -> u64 { (RREG32_SOC15(GC, 0, mmGCMC_VM_FB_OFFSET) as u64) << 24 }

unsafe fn gfxhub_v2_1_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let hub = &mut (*adev).vmhub[AMDGPU_GFXHUB(0)];
    WREG32_SOC15_OFFSET(GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32, hub.ctx_addr_distance * vmid, lower_32_bits(page_table_base));
    WREG32_SOC15_OFFSET(GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32, hub.ctx_addr_distance * vmid, upper_32_bits(page_table_base));
}
unsafe fn gfxhub_v2_1_init_gart_aperture_regs(adev: *mut amdgpu_device) {
    let pt_base = amdgpu_gmc_pd_addr((*adev).gart.bo); gfxhub_v2_1_setup_vm_pt_regs(adev, 0, pt_base);
    WREG32_SOC15(GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, ((*adev).gmc.gart_start >> 12) as u32);
    WREG32_SOC15(GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, ((*adev).gmc.gart_start >> 44) as u32);
    WREG32_SOC15(GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, ((*adev).gmc.gart_end >> 12) as u32);
    WREG32_SOC15(GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, ((*adev).gmc.gart_end >> 44) as u32);
}

unsafe fn gfxhub_v2_1_init_system_aperture_regs(adev: *mut amdgpu_device) {
    if amdgpu_sriov_vf(adev) { return; }
    WREG32_SOC15(GC, 0, mmGCMC_VM_AGP_BASE, 0);
    WREG32_SOC15(GC, 0, mmGCMC_VM_AGP_BOT, (*adev).gmc.agp_start >> 24);
    WREG32_SOC15(GC, 0, mmGCMC_VM_AGP_TOP, (*adev).gmc.agp_end >> 24);
    WREG32_SOC15(GC, 0, mmGCMC_VM_SYSTEM_APERTURE_LOW_ADDR, min((*adev).gmc.fb_start, (*adev).gmc.agp_start) >> 18);
    WREG32_SOC15(GC, 0, mmGCMC_VM_SYSTEM_APERTURE_HIGH_ADDR, max((*adev).gmc.fb_end, (*adev).gmc.agp_end) >> 18);
    let value = amdgpu_gmc_vram_mc2pa(adev, (*adev).mem_scratch.gpu_addr);
    WREG32_SOC15(GC, 0, mmGCMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, (value >> 12) as u32);
    WREG32_SOC15(GC, 0, mmGCMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, (value >> 44) as u32);
    WREG32_SOC15(GC, 0, mmGCVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32, ((*adev).dummy_page_addr >> 12) as u32);
    WREG32_SOC15(GC, 0, mmGCVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32, ((*adev).dummy_page_addr >> 44) as u32);
    WREG32_FIELD15(GC, 0, GCVM_L2_PROTECTION_FAULT_CNTL2, ACTIVE_PAGE_MIGRATION_PTE_READ_RETRY, 1);
}

unsafe fn gfxhub_v2_1_init_tlb_regs(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15(GC, 0, mmGCMC_VM_MX_L1_TLB_CNTL);
    tmp = REG_SET_FIELD(tmp, GCMC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 1);
    tmp = REG_SET_FIELD(tmp, GCMC_VM_MX_L1_TLB_CNTL, SYSTEM_ACCESS_MODE, 3);
    tmp = REG_SET_FIELD(tmp, GCMC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 1);
    tmp = REG_SET_FIELD(tmp, GCMC_VM_MX_L1_TLB_CNTL, SYSTEM_APERTURE_UNMAPPED_ACCESS, 0);
    tmp = REG_SET_FIELD(tmp, GCMC_VM_MX_L1_TLB_CNTL, MTYPE, MTYPE_UC); WREG32_SOC15(GC, 0, mmGCMC_VM_MX_L1_TLB_CNTL, tmp);
}

unsafe fn gfxhub_v2_1_init_cache_regs(adev: *mut amdgpu_device) {
    if amdgpu_sriov_vf(adev) { return; }
    let mut tmp = RREG32_SOC15(GC, 0, mmGCVM_L2_CNTL);
    tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL, ENABLE_L2_CACHE, 1); tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL, ENABLE_L2_FRAGMENT_PROCESSING, 0); tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL, ENABLE_DEFAULT_PAGE_OUT_TO_SYSTEM_MEMORY, 1);
    tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL, L2_PDE0_CACHE_TAG_GENERATION_MODE, 0); tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL, PDE_FAULT_CLASSIFICATION, 0); tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL, CONTEXT1_IDENTITY_ACCESS_MODE, 1); tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL, IDENTITY_MODE_FRAGMENT_SIZE, 0); WREG32_SOC15(GC, 0, mmGCVM_L2_CNTL, tmp);
    tmp = RREG32_SOC15(GC, 0, mmGCVM_L2_CNTL2); tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL2, INVALIDATE_ALL_L1_TLBS, 1); tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL2, INVALIDATE_L2_CACHE, 1); WREG32_SOC15(GC, 0, mmGCVM_L2_CNTL2, tmp);
    tmp = mmGCVM_L2_CNTL3_DEFAULT; if (*adev).gmc.translate_further { tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL3, BANK_SELECT, 12); tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 9); } else { tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL3, BANK_SELECT, 9); tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 6); } WREG32_SOC15(GC, 0, mmGCVM_L2_CNTL3, tmp);
    tmp = mmGCVM_L2_CNTL4_DEFAULT; tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL4, VMC_TAP_PDE_REQUEST_PHYSICAL, 0); tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL4, VMC_TAP_PTE_REQUEST_PHYSICAL, 0); WREG32_SOC15(GC, 0, mmGCVM_L2_CNTL4, tmp);
    tmp = mmGCVM_L2_CNTL5_DEFAULT; tmp = REG_SET_FIELD(tmp, GCVM_L2_CNTL5, L2_CACHE_SMALLK_FRAGMENT_SIZE, 0); WREG32_SOC15(GC, 0, mmGCVM_L2_CNTL5, tmp);
}

unsafe fn gfxhub_v2_1_enable_system_domain(adev: *mut amdgpu_device) { let mut tmp = RREG32_SOC15(GC, 0, mmGCVM_CONTEXT0_CNTL); tmp = REG_SET_FIELD(tmp, GCVM_CONTEXT0_CNTL, ENABLE_CONTEXT, 1); tmp = REG_SET_FIELD(tmp, GCVM_CONTEXT0_CNTL, PAGE_TABLE_DEPTH, 0); tmp = REG_SET_FIELD(tmp, GCVM_CONTEXT0_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, 0); WREG32_SOC15(GC, 0, mmGCVM_CONTEXT0_CNTL, tmp); }
unsafe fn gfxhub_v2_1_disable_identity_aperture(adev: *mut amdgpu_device) { if amdgpu_sriov_vf(adev) { return; } WREG32_SOC15(GC,0,mmGCVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_LO32,0xffffffff); WREG32_SOC15(GC,0,mmGCVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_HI32,0xf); WREG32_SOC15(GC,0,mmGCVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_LO32,0); WREG32_SOC15(GC,0,mmGCVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_HI32,0); WREG32_SOC15(GC,0,mmGCVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_LO32,0); WREG32_SOC15(GC,0,mmGCVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_HI32,0); }

unsafe fn gfxhub_v2_1_setup_vmid_config(adev: *mut amdgpu_device) { let hub=&mut (*adev).vmhub[AMDGPU_GFXHUB(0)]; let mut tmp=0; for i in 0..=14 { tmp=RREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT1_CNTL,i*hub.ctx_distance); tmp=REG_SET_FIELD(tmp,GCVM_CONTEXT1_CNTL,ENABLE_CONTEXT,1); tmp=REG_SET_FIELD(tmp,GCVM_CONTEXT1_CNTL,PAGE_TABLE_DEPTH,(*adev).vm_manager.num_level); tmp=REG_SET_FIELD(tmp,GCVM_CONTEXT1_CNTL,RANGE_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,GCVM_CONTEXT1_CNTL,DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,GCVM_CONTEXT1_CNTL,PDE0_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,GCVM_CONTEXT1_CNTL,VALID_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,GCVM_CONTEXT1_CNTL,READ_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,GCVM_CONTEXT1_CNTL,WRITE_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,GCVM_CONTEXT1_CNTL,EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT,1); tmp=REG_SET_FIELD(tmp,GCVM_CONTEXT1_CNTL,PAGE_TABLE_BLOCK_SIZE,(*adev).vm_manager.block_size-9); tmp=REG_SET_FIELD(tmp,GCVM_CONTEXT1_CNTL,RETRY_PERMISSION_OR_INVALID_PAGE_FAULT,!(*adev).gmc.noretry); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT1_CNTL,i*hub.ctx_distance,tmp); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT1_PAGE_TABLE_START_ADDR_LO32,i*hub.ctx_addr_distance,0); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT1_PAGE_TABLE_START_ADDR_HI32,i*hub.ctx_addr_distance,0); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT1_PAGE_TABLE_END_ADDR_LO32,i*hub.ctx_addr_distance,lower_32_bits((*adev).vm_manager.max_pfn-1)); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT1_PAGE_TABLE_END_ADDR_HI32,i*hub.ctx_addr_distance,upper_32_bits((*adev).vm_manager.max_pfn-1)); } hub.vm_cntx_cntl=tmp; }

unsafe fn gfxhub_v2_1_program_invalidation(adev: *mut amdgpu_device) { let hub=&mut (*adev).vmhub[AMDGPU_GFXHUB(0)]; for i in 0..18 { WREG32_SOC15_OFFSET(GC,0,mmGCVM_INVALIDATE_ENG0_ADDR_RANGE_LO32,i*hub.eng_addr_distance,0xffffffff); WREG32_SOC15_OFFSET(GC,0,mmGCVM_INVALIDATE_ENG0_ADDR_RANGE_HI32,i*hub.eng_addr_distance,0x1f); } }
unsafe fn gfxhub_v2_1_gart_enable(adev: *mut amdgpu_device) -> i32 { if amdgpu_sriov_vf(adev) { WREG32_SOC15(GC,0,mmGCMC_VM_FB_LOCATION_BASE,(*adev).gmc.vram_start>>24); WREG32_SOC15(GC,0,mmGCMC_VM_FB_LOCATION_TOP,(*adev).gmc.vram_end>>24); } gfxhub_v2_1_init_gart_aperture_regs(adev); gfxhub_v2_1_init_system_aperture_regs(adev); gfxhub_v2_1_init_tlb_regs(adev); gfxhub_v2_1_init_cache_regs(adev); gfxhub_v2_1_enable_system_domain(adev); gfxhub_v2_1_disable_identity_aperture(adev); gfxhub_v2_1_setup_vmid_config(adev); gfxhub_v2_1_program_invalidation(adev); 0 }

unsafe fn gfxhub_v2_1_gart_disable(adev: *mut amdgpu_device) { let hub=&mut (*adev).vmhub[AMDGPU_GFXHUB(0)]; for i in 0..16 { WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_CNTL,i*hub.ctx_distance,0); } let mut tmp=RREG32_SOC15(GC,0,mmGCMC_VM_MX_L1_TLB_CNTL); tmp=REG_SET_FIELD(tmp,GCMC_VM_MX_L1_TLB_CNTL,ENABLE_L1_TLB,0); tmp=REG_SET_FIELD(tmp,GCMC_VM_MX_L1_TLB_CNTL,ENABLE_ADVANCED_DRIVER_MODEL,0); WREG32_SOC15(GC,0,mmGCMC_VM_MX_L1_TLB_CNTL,tmp); if amdgpu_sriov_vf(adev){return;} WREG32_FIELD15(GC,0,GCVM_L2_CNTL,ENABLE_L2_CACHE,0); WREG32_SOC15(GC,0,mmGCVM_L2_CNTL3,0); }

unsafe fn gfxhub_v2_1_set_fault_enable_default(adev: *mut amdgpu_device, value: bool) { if amdgpu_sriov_vf(adev){return;} let mut tmp=RREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_CNTL); for field in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT,PDE0_PROTECTION_FAULT_ENABLE_DEFAULT,PDE1_PROTECTION_FAULT_ENABLE_DEFAULT,PDE2_PROTECTION_FAULT_ENABLE_DEFAULT,TRANSLATE_FURTHER_PROTECTION_FAULT_ENABLE_DEFAULT,NACK_PROTECTION_FAULT_ENABLE_DEFAULT,DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT,VALID_PROTECTION_FAULT_ENABLE_DEFAULT,READ_PROTECTION_FAULT_ENABLE_DEFAULT,WRITE_PROTECTION_FAULT_ENABLE_DEFAULT,EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT] { tmp=REG_SET_FIELD(tmp,GCVM_L2_PROTECTION_FAULT_CNTL,field,value); } tmp=REG_SET_FIELD(tmp,GCVM_L2_PROTECTION_FAULT_CNTL,CRASH_ON_NO_RETRY_FAULT,!value); tmp=REG_SET_FIELD(tmp,GCVM_L2_PROTECTION_FAULT_CNTL,CRASH_ON_RETRY_FAULT,!value); WREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_CNTL,tmp); }

unsafe fn gfxhub_v2_1_init(adev: *mut amdgpu_device) { let hub=&mut (*adev).vmhub[AMDGPU_GFXHUB(0)]; hub.ctx0_ptb_addr_lo32=SOC15_REG_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32); hub.ctx0_ptb_addr_hi32=SOC15_REG_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32); hub.vm_inv_eng0_sem=SOC15_REG_OFFSET(GC,0,mmGCVM_INVALIDATE_ENG0_SEM); hub.vm_inv_eng0_req=SOC15_REG_OFFSET(GC,0,mmGCVM_INVALIDATE_ENG0_REQ); hub.vm_inv_eng0_ack=SOC15_REG_OFFSET(GC,0,mmGCVM_INVALIDATE_ENG0_ACK); hub.vm_context0_cntl=SOC15_REG_OFFSET(GC,0,mmGCVM_CONTEXT0_CNTL); hub.vm_l2_pro_fault_status=SOC15_REG_OFFSET(GC,0,mmGCVM_L2_PROTECTION_FAULT_STATUS); hub.vm_l2_pro_fault_cntl=SOC15_REG_OFFSET(GC,0,mmGCVM_L2_PROTECTION_FAULT_CNTL); hub.ctx_distance=mmGCVM_CONTEXT1_CNTL-mmGCVM_CONTEXT0_CNTL; hub.ctx_addr_distance=mmGCVM_CONTEXT1_PAGE_TABLE_BASE_ADDR_LO32-mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32; hub.eng_distance=mmGCVM_INVALIDATE_ENG1_REQ-mmGCVM_INVALIDATE_ENG0_REQ; hub.eng_addr_distance=mmGCVM_INVALIDATE_ENG1_ADDR_RANGE_LO32-mmGCVM_INVALIDATE_ENG0_ADDR_RANGE_LO32; hub.vm_cntx_cntl_vm_fault=GCVM_CONTEXT1_CNTL__RANGE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|GCVM_CONTEXT1_CNTL__DUMMY_PAGE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|GCVM_CONTEXT1_CNTL__PDE0_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|GCVM_CONTEXT1_CNTL__VALID_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|GCVM_CONTEXT1_CNTL__READ_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|GCVM_CONTEXT1_CNTL__WRITE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK|GCVM_CONTEXT1_CNTL__EXECUTE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK; hub.vmhub_funcs=&gfxhub_v2_1_vmhub_funcs; }

static gfxhub_v2_1_vmhub_funcs: amdgpu_vmhub_funcs = amdgpu_vmhub_funcs { print_l2_protection_fault_status: Some(gfxhub_v2_1_print_l2_protection_fault_status), get_invalidate_req: Some(gfxhub_v2_1_get_invalidate_req) };

// The following register save/restore and halt paths retain the original
// ordering and are expressed through the translated kernel register helpers.
unsafe fn gfxhub_v2_1_utcl2_harvest(adev: *mut amdgpu_device) { let max_sa_mask=amdgpu_gfx_create_bitmask((*adev).gfx.config.max_sh_per_se*(*adev).gfx.config.max_shader_engines); match amdgpu_ip_version(adev,GC_HWIP,0) { IP_VERSION(10,3,1)|IP_VERSION(10,3,3)=>{ let mut e=RREG32_SOC15(GC,0,mmCC_GC_SA_UNIT_DISABLE)&CC_GC_SA_UNIT_DISABLE__SA_DISABLE_MASK; e>>=CC_GC_SA_UNIT_DISABLE__SA_DISABLE__SHIFT; let mut v=RREG32_SOC15(GC,0,mmGC_USER_SA_UNIT_DISABLE)&GC_USER_SA_UNIT_DISABLE__SA_DISABLE_MASK; v>>=GC_USER_SA_UNIT_DISABLE__SA_DISABLE__SHIFT; let mut d=(e|v)&max_sa_mask; let mut tmp=0; let mut i=0; while d>0 { if d&1!=0 {tmp|=0x3<<(i*2);} d>>=1;i+=1;} WREG32_SOC15(GC,0,mmGCUTCL2_HARVEST_BYPASS_GROUPS_YELLOW_CARP,tmp); }, _=>{} } }

unsafe fn gfxhub_v2_1_save_regs(adev: *mut amdgpu_device) { (*adev).gmc.VM_L2_CNTL=RREG32_SOC15(GC,0,mmGCVM_L2_CNTL); (*adev).gmc.VM_L2_CNTL2=RREG32_SOC15(GC,0,mmGCVM_L2_CNTL2); (*adev).gmc.VM_DUMMY_PAGE_FAULT_CNTL=RREG32_SOC15(GC,0,mmGCVM_DUMMY_PAGE_FAULT_CNTL); (*adev).gmc.VM_DUMMY_PAGE_FAULT_ADDR_LO32=RREG32_SOC15(GC,0,mmGCVM_DUMMY_PAGE_FAULT_ADDR_LO32); (*adev).gmc.VM_DUMMY_PAGE_FAULT_ADDR_HI32=RREG32_SOC15(GC,0,mmGCVM_DUMMY_PAGE_FAULT_ADDR_HI32); (*adev).gmc.VM_L2_PROTECTION_FAULT_CNTL=RREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_CNTL); (*adev).gmc.VM_L2_PROTECTION_FAULT_CNTL2=RREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_CNTL2); (*adev).gmc.VM_L2_PROTECTION_FAULT_MM_CNTL3=RREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_MM_CNTL3); (*adev).gmc.VM_L2_PROTECTION_FAULT_MM_CNTL4=RREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_MM_CNTL4); (*adev).gmc.VM_L2_PROTECTION_FAULT_ADDR_LO32=RREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_ADDR_LO32); (*adev).gmc.VM_L2_PROTECTION_FAULT_ADDR_HI32=RREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_ADDR_HI32); (*adev).gmc.VM_DEBUG=RREG32_SOC15(GC,0,mmGCVM_DEBUG); (*adev).gmc.VM_L2_MM_GROUP_RT_CLASSES=RREG32_SOC15(GC,0,mmGCVM_L2_MM_GROUP_RT_CLASSES); (*adev).gmc.VM_L2_BANK_SELECT_RESERVED_CID=RREG32_SOC15(GC,0,mmGCVM_L2_BANK_SELECT_RESERVED_CID); (*adev).gmc.VM_L2_BANK_SELECT_RESERVED_CID2=RREG32_SOC15(GC,0,mmGCVM_L2_BANK_SELECT_RESERVED_CID2); (*adev).gmc.VM_L2_CACHE_PARITY_CNTL=RREG32_SOC15(GC,0,mmGCVM_L2_CACHE_PARITY_CNTL); (*adev).gmc.VM_L2_IH_LOG_CNTL=RREG32_SOC15(GC,0,mmGCVM_L2_IH_LOG_CNTL); for i in 0..=15 { (*adev).gmc.VM_CONTEXT_CNTL[i]=RREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_CNTL,i); (*adev).gmc.VM_CONTEXT_PAGE_TABLE_BASE_ADDR_LO32[i]=RREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32,i*2); (*adev).gmc.VM_CONTEXT_PAGE_TABLE_BASE_ADDR_HI32[i]=RREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32,i*2); (*adev).gmc.VM_CONTEXT_PAGE_TABLE_START_ADDR_LO32[i]=RREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32,i*2); (*adev).gmc.VM_CONTEXT_PAGE_TABLE_START_ADDR_HI32[i]=RREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32,i*2); (*adev).gmc.VM_CONTEXT_PAGE_TABLE_END_ADDR_LO32[i]=RREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32,i*2); (*adev).gmc.VM_CONTEXT_PAGE_TABLE_END_ADDR_HI32[i]=RREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32,i*2); } (*adev).gmc.MC_VM_MX_L1_TLB_CNTL=RREG32_SOC15(GC,0,mmGCMC_VM_MX_L1_TLB_CNTL); }

unsafe fn gfxhub_v2_1_restore_regs(adev: *mut amdgpu_device) { WREG32_SOC15(GC,0,mmGCVM_L2_CNTL,(*adev).gmc.VM_L2_CNTL); WREG32_SOC15(GC,0,mmGCVM_L2_CNTL2,(*adev).gmc.VM_L2_CNTL2); WREG32_SOC15(GC,0,mmGCVM_DUMMY_PAGE_FAULT_CNTL,(*adev).gmc.VM_DUMMY_PAGE_FAULT_CNTL); WREG32_SOC15(GC,0,mmGCVM_DUMMY_PAGE_FAULT_ADDR_LO32,(*adev).gmc.VM_DUMMY_PAGE_FAULT_ADDR_LO32); WREG32_SOC15(GC,0,mmGCVM_DUMMY_PAGE_FAULT_ADDR_HI32,(*adev).gmc.VM_DUMMY_PAGE_FAULT_ADDR_HI32); WREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_CNTL,(*adev).gmc.VM_L2_PROTECTION_FAULT_CNTL); WREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_CNTL2,(*adev).gmc.VM_L2_PROTECTION_FAULT_CNTL2); WREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_MM_CNTL3,(*adev).gmc.VM_L2_PROTECTION_FAULT_MM_CNTL3); WREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_MM_CNTL4,(*adev).gmc.VM_L2_PROTECTION_FAULT_MM_CNTL4); WREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_ADDR_LO32,(*adev).gmc.VM_L2_PROTECTION_FAULT_ADDR_LO32); WREG32_SOC15(GC,0,mmGCVM_L2_PROTECTION_FAULT_ADDR_HI32,(*adev).gmc.VM_L2_PROTECTION_FAULT_ADDR_HI32); WREG32_SOC15(GC,0,mmGCVM_DEBUG,(*adev).gmc.VM_DEBUG); WREG32_SOC15(GC,0,mmGCVM_L2_MM_GROUP_RT_CLASSES,(*adev).gmc.VM_L2_MM_GROUP_RT_CLASSES); WREG32_SOC15(GC,0,mmGCVM_L2_BANK_SELECT_RESERVED_CID,(*adev).gmc.VM_L2_BANK_SELECT_RESERVED_CID); WREG32_SOC15(GC,0,mmGCVM_L2_BANK_SELECT_RESERVED_CID2,(*adev).gmc.VM_L2_BANK_SELECT_RESERVED_CID2); WREG32_SOC15(GC,0,mmGCVM_L2_CACHE_PARITY_CNTL,(*adev).gmc.VM_L2_CACHE_PARITY_CNTL); WREG32_SOC15(GC,0,mmGCVM_L2_IH_LOG_CNTL,(*adev).gmc.VM_L2_IH_LOG_CNTL); for i in 0..=15 { WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_CNTL,i,(*adev).gmc.VM_CONTEXT_CNTL[i]); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32,i*2,(*adev).gmc.VM_CONTEXT_PAGE_TABLE_BASE_ADDR_LO32[i]); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32,i*2,(*adev).gmc.VM_CONTEXT_PAGE_TABLE_BASE_ADDR_HI32[i]); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32,i*2,(*adev).gmc.VM_CONTEXT_PAGE_TABLE_START_ADDR_LO32[i]); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32,i*2,(*adev).gmc.VM_CONTEXT_PAGE_TABLE_START_ADDR_HI32[i]); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32,i*2,(*adev).gmc.VM_CONTEXT_PAGE_TABLE_END_ADDR_LO32[i]); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32,i*2,(*adev).gmc.VM_CONTEXT_PAGE_TABLE_END_ADDR_HI32[i]); } WREG32_SOC15(GC,0,mmGCMC_VM_FB_LOCATION_BASE,(*adev).gmc.vram_start>>24); WREG32_SOC15(GC,0,mmGCMC_VM_FB_LOCATION_TOP,(*adev).gmc.vram_end>>24); WREG32_SOC15(GC,0,mmGCMC_VM_MX_L1_TLB_CNTL,(*adev).gmc.MC_VM_MX_L1_TLB_CNTL); }

unsafe fn gfxhub_v2_1_halt(adev: *mut amdgpu_device) { let hub=&mut (*adev).vmhub[AMDGPU_GFXHUB(0)]; gfxhub_v2_1_set_fault_enable_default(adev,false); for i in 0..=14 { WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT1_PAGE_TABLE_START_ADDR_LO32,i*hub.ctx_addr_distance,!0); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT1_PAGE_TABLE_START_ADDR_HI32,i*hub.ctx_addr_distance,!0); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT1_PAGE_TABLE_END_ADDR_LO32,i*hub.ctx_addr_distance,0); WREG32_SOC15_OFFSET(GC,0,mmGCVM_CONTEXT1_PAGE_TABLE_END_ADDR_HI32,i*hub.ctx_addr_distance,0); } let mut tmp=RREG32_SOC15(GC,0,mmGRBM_STATUS2); let mut time=1000; while (tmp&(GRBM_STATUS2__EA_BUSY_MASK|GRBM_STATUS2__EA_LINK_BUSY_MASK))!=0&&time!=0 { udelay(100); time-=1; tmp=RREG32_SOC15(GC,0,mmGRBM_STATUS2); } if time==0 { drm_warn(adev_to_drm(adev),"failed to wait for GRBM(EA) idle\n"); } }

// Callback table corresponding to the C amdgpu_gfxhub_funcs definition.
pub static gfxhub_v2_1_funcs: amdgpu_gfxhub_funcs = amdgpu_gfxhub_funcs { get_fb_location: Some(gfxhub_v2_1_get_fb_location), get_mc_fb_offset: Some(gfxhub_v2_1_get_mc_fb_offset), setup_vm_pt_regs: Some(gfxhub_v2_1_setup_vm_pt_regs), gart_enable: Some(gfxhub_v2_1_gart_enable), gart_disable: Some(gfxhub_v2_1_gart_disable), set_fault_enable_default: Some(gfxhub_v2_1_set_fault_enable_default), init: Some(gfxhub_v2_1_init), utcl2_harvest: Some(gfxhub_v2_1_utcl2_harvest), mode2_save_regs: Some(gfxhub_v2_1_save_regs), mode2_restore_regs: Some(gfxhub_v2_1_restore_regs), halt: Some(gfxhub_v2_1_halt) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
