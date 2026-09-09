/* Translated from mmhub_v3_0_1.c. */

const REGMMVM_L2_CNTL3_DEFAULT: u32 = 0x80100007;
const REGMMVM_L2_CNTL4_DEFAULT: u32 = 0x000000c1;
const REGMMVM_L2_CNTL5_DEFAULT: u32 = 0x00003fe0;

static MMHUB_CLIENT_IDS_V3_0_1: [[&str; 2]; 30] = [
    ["VMC", ""], ["ISPXT", "ISPXT"], ["ISPIXT", "ISPIXT"], ["", "DCEDWB"],
    ["DCEDMC", "DCEDMC"], ["DCEVGA", "DCEVGA"], ["MP0", "MP0"], ["MP1", "MP1"],
    ["MPM", "MPM"], ["", ""], ["", "ISPMWR0"], ["", "ISPMWR1"],
    ["ISPTNR", "ISPTNR"], ["", "ISPSWR"], ["ISPCRD0", "ISPCWR0"], ["ISPCRD1", "ISPCWR1"],
    ["ISPCRD2", "ISPCWR2"], ["", "ISPCWR3"], ["", "XDP"], ["", ""],
    ["", ""], ["", "OSSSYS"], ["HDP", "HDP"], ["LSDMA", "LSDMA"],
    ["JPEG", "JPEG"], ["", ""], ["", ""], ["VSCH", "VSCH"], ["VCNU", "VCNU"], ["VCN", "VCN"],
];

unsafe fn mmhub_v3_0_1_get_invalidate_req(vmid: u32, flush_type: u32) -> u32 {
    let mut req = 0u32;
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, PER_VMID_INVALIDATE_REQ, 1u32 << vmid);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, FLUSH_TYPE, flush_type);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PTES, 1);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE0, 1);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE1, 1);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE2, 1);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L1_PTES, 1);
    REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, CLEAR_PROTECTION_FAULT_STATUS_ADDR, 0)
}

unsafe fn mmhub_v3_0_1_print_l2_protection_fault_status(adev: *mut amdgpu_device, status: u32) {
    let cid = REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, CID);
    let rw = REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, RW);
    dev_err!((*adev).dev, "MMVM_L2_PROTECTION_FAULT_STATUS:0x{:08X}\n", status);
    let mmhub_cid = amdgpu_mmhub_client_name!(&mut (*adev).mmhub, cid, rw);
    dev_err!((*adev).dev, "\t Faulty UTCL2 client ID: {} (0x{:x})\n", mmhub_cid.unwrap_or("unknown"), cid);
    dev_err!((*adev).dev, "\t MORE_FAULTS: 0x{:lx}\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, MORE_FAULTS));
    dev_err!((*adev).dev, "\t WALKER_ERROR: 0x{:lx}\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, WALKER_ERROR));
    dev_err!((*adev).dev, "\t PERMISSION_FAULTS: 0x{:lx}\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, PERMISSION_FAULTS));
    dev_err!((*adev).dev, "\t MAPPING_ERROR: 0x{:lx}\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, MAPPING_ERROR));
    dev_err!((*adev).dev, "\t RW: 0x{:x}\n", rw);
}

unsafe fn mmhub_v3_0_1_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let hub = &(*adev).vmhub[AMDGPU_MMHUB0!(0)];
    WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32, hub.ctx_addr_distance * vmid, lower_32_bits(page_table_base));
    WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32, hub.ctx_addr_distance * vmid, upper_32_bits(page_table_base));
}

unsafe fn mmhub_v3_0_1_init_gart_aperture_regs(adev: *mut amdgpu_device) {
    let pt_base = amdgpu_gmc_pd_addr!((*adev).gart.bo);
    mmhub_v3_0_1_setup_vm_pt_regs(adev, 0, pt_base);
    WREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, ((*adev).gmc.gart_start >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, ((*adev).gmc.gart_start >> 44) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, ((*adev).gmc.gart_end >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, ((*adev).gmc.gart_end >> 44) as u32);
}

unsafe fn mmhub_v3_0_1_init_system_aperture_regs(adev: *mut amdgpu_device) {
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

unsafe fn mmhub_v3_0_1_init_tlb_regs(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15!(MMHUB, 0, regMMMC_VM_MX_L1_TLB_CNTL);
    tmp = REG_SET_FIELD!(tmp, MMMC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 1);
    tmp = REG_SET_FIELD!(tmp, MMMC_VM_MX_L1_TLB_CNTL, SYSTEM_ACCESS_MODE, 3);
    tmp = REG_SET_FIELD!(tmp, MMMC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 1);
    tmp = REG_SET_FIELD!(tmp, MMMC_VM_MX_L1_TLB_CNTL, SYSTEM_APERTURE_UNMAPPED_ACCESS, 0);
    tmp = REG_SET_FIELD!(tmp, MMMC_VM_MX_L1_TLB_CNTL, ECO_BITS, 0);
    tmp = REG_SET_FIELD!(tmp, MMMC_VM_MX_L1_TLB_CNTL, MTYPE, MTYPE_UC);
    WREG32_SOC15!(MMHUB, 0, regMMMC_VM_MX_L1_TLB_CNTL, tmp);
}

unsafe fn mmhub_v3_0_1_init_cache_regs(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15!(MMHUB, 0, regMMVM_L2_CNTL);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, ENABLE_L2_CACHE, 1);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, ENABLE_L2_FRAGMENT_PROCESSING, 0);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, ENABLE_DEFAULT_PAGE_OUT_TO_SYSTEM_MEMORY, 1);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, L2_PDE0_CACHE_TAG_GENERATION_MODE, 0);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, PDE_FAULT_CLASSIFICATION, 0);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, CONTEXT1_IDENTITY_ACCESS_MODE, 1);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, IDENTITY_MODE_FRAGMENT_SIZE, 0);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_CNTL, tmp);
    tmp = RREG32_SOC15!(MMHUB, 0, regMMVM_L2_CNTL2);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL2, INVALIDATE_ALL_L1_TLBS, 1);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL2, INVALIDATE_L2_CACHE, 1);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_CNTL2, tmp);
    tmp = REGMMVM_L2_CNTL3_DEFAULT;
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL3, BANK_SELECT, if (*adev).gmc.translate_further { 12 } else { 9 });
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, if (*adev).gmc.translate_further { 9 } else { 6 });
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_CNTL3, tmp);
    tmp = REG_SET_FIELD!(REGMMVM_L2_CNTL4_DEFAULT, MMVM_L2_CNTL4, VMC_TAP_PDE_REQUEST_PHYSICAL, 0);
    tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL4, VMC_TAP_PTE_REQUEST_PHYSICAL, 0);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_CNTL4, tmp);
    tmp = REG_SET_FIELD!(REGMMVM_L2_CNTL5_DEFAULT, MMVM_L2_CNTL5, L2_CACHE_SMALLK_FRAGMENT_SIZE, 0);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_CNTL5, tmp);
}

unsafe fn mmhub_v3_0_1_enable_system_domain(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_CNTL);
    tmp = REG_SET_FIELD!(tmp, MMVM_CONTEXT0_CNTL, ENABLE_CONTEXT, 1);
    tmp = REG_SET_FIELD!(tmp, MMVM_CONTEXT0_CNTL, PAGE_TABLE_DEPTH, 0);
    tmp = REG_SET_FIELD!(tmp, MMVM_CONTEXT0_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, 0);
    WREG32_SOC15!(MMHUB, 0, regMMVM_CONTEXT0_CNTL, tmp);
}

unsafe fn mmhub_v3_0_1_disable_identity_aperture(adev: *mut amdgpu_device) {
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_LO32, 0xffffffff);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_HI32, 0x0000000f);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_LO32, 0);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_HI32, 0);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_LO32, 0);
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_HI32, 0);
}

unsafe fn mmhub_v3_0_1_setup_vmid_config(adev: *mut amdgpu_device) {
    let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0!(0)];
    let mut tmp = 0u32;
    for i in 0..=14 {
        let off = i * hub.ctx_distance;
        tmp = RREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_CONTEXT1_CNTL, off);
        tmp = REG_SET_FIELD!(tmp, MMVM_CONTEXT1_CNTL, ENABLE_CONTEXT, 1);
        tmp = REG_SET_FIELD!(tmp, MMVM_CONTEXT1_CNTL, PAGE_TABLE_DEPTH, (*adev).vm_manager.num_level);
        for field in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT, DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT, PDE0_PROTECTION_FAULT_ENABLE_DEFAULT, VALID_PROTECTION_FAULT_ENABLE_DEFAULT, READ_PROTECTION_FAULT_ENABLE_DEFAULT, WRITE_PROTECTION_FAULT_ENABLE_DEFAULT, EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT] { tmp = REG_SET_FIELD!(tmp, MMVM_CONTEXT1_CNTL, field, 1); }
        tmp = REG_SET_FIELD!(tmp, MMVM_CONTEXT1_CNTL, PAGE_TABLE_BLOCK_SIZE, (*adev).vm_manager.block_size - 9);
        tmp = REG_SET_FIELD!(tmp, MMVM_CONTEXT1_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, !(*adev).gmc.noretry);
        WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_CONTEXT1_CNTL, off, tmp);
        WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_CONTEXT1_PAGE_TABLE_START_ADDR_LO32, i * hub.ctx_addr_distance, 0);
        WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_CONTEXT1_PAGE_TABLE_START_ADDR_HI32, i * hub.ctx_addr_distance, 0);
        WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_CONTEXT1_PAGE_TABLE_END_ADDR_LO32, i * hub.ctx_addr_distance, lower_32_bits((*adev).vm_manager.max_pfn - 1));
        WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_CONTEXT1_PAGE_TABLE_END_ADDR_HI32, i * hub.ctx_addr_distance, upper_32_bits((*adev).vm_manager.max_pfn - 1));
    }
    hub.vm_cntx_cntl = tmp;
}

unsafe fn mmhub_v3_0_1_program_invalidation(adev: *mut amdgpu_device) {
    let hub = &(*adev).vmhub[AMDGPU_MMHUB0!(0)];
    for i in 0..18 { WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_INVALIDATE_ENG0_ADDR_RANGE_LO32, i * hub.eng_addr_distance, 0xffffffff); WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_INVALIDATE_ENG0_ADDR_RANGE_HI32, i * hub.eng_addr_distance, 0x1f); }
}

unsafe fn mmhub_v3_0_1_gart_enable(adev: *mut amdgpu_device) -> i32 { mmhub_v3_0_1_init_gart_aperture_regs(adev); mmhub_v3_0_1_init_system_aperture_regs(adev); mmhub_v3_0_1_init_tlb_regs(adev); mmhub_v3_0_1_init_cache_regs(adev); mmhub_v3_0_1_enable_system_domain(adev); mmhub_v3_0_1_disable_identity_aperture(adev); mmhub_v3_0_1_setup_vmid_config(adev); mmhub_v3_0_1_program_invalidation(adev); 0 }

unsafe fn mmhub_v3_0_1_gart_disable(adev: *mut amdgpu_device) {
    let hub = &(*adev).vmhub[AMDGPU_MMHUB0!(0)];
    for i in 0..16 { WREG32_SOC15_OFFSET!(MMHUB, 0, regMMVM_CONTEXT0_CNTL, i * hub.ctx_distance, 0); }
    let mut tmp = RREG32_SOC15!(MMHUB, 0, regMMMC_VM_MX_L1_TLB_CNTL);
    tmp = REG_SET_FIELD!(tmp, MMMC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 0); tmp = REG_SET_FIELD!(tmp, MMMC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 0); WREG32_SOC15!(MMHUB, 0, regMMMC_VM_MX_L1_TLB_CNTL, tmp);
    tmp = RREG32_SOC15!(MMHUB, 0, regMMVM_L2_CNTL); tmp = REG_SET_FIELD!(tmp, MMVM_L2_CNTL, ENABLE_L2_CACHE, 0); WREG32_SOC15!(MMHUB, 0, regMMVM_L2_CNTL, tmp); WREG32_SOC15!(MMHUB, 0, regMMVM_L2_CNTL3, 0);
}

unsafe fn mmhub_v3_0_1_set_fault_enable_default(adev: *mut amdgpu_device, value: bool) {
    let mut tmp = RREG32_SOC15!(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_CNTL);
    for field in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT, PDE0_PROTECTION_FAULT_ENABLE_DEFAULT, PDE1_PROTECTION_FAULT_ENABLE_DEFAULT, PDE2_PROTECTION_FAULT_ENABLE_DEFAULT, TRANSLATE_FURTHER_PROTECTION_FAULT_ENABLE_DEFAULT, NACK_PROTECTION_FAULT_ENABLE_DEFAULT, DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT, VALID_PROTECTION_FAULT_ENABLE_DEFAULT, READ_PROTECTION_FAULT_ENABLE_DEFAULT, WRITE_PROTECTION_FAULT_ENABLE_DEFAULT, EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT] { tmp = REG_SET_FIELD!(tmp, MMVM_L2_PROTECTION_FAULT_CNTL, field, value); }
    if !value { tmp = REG_SET_FIELD!(tmp, MMVM_L2_PROTECTION_FAULT_CNTL, CRASH_ON_NO_RETRY_FAULT, 1); tmp = REG_SET_FIELD!(tmp, MMVM_L2_PROTECTION_FAULT_CNTL, CRASH_ON_RETRY_FAULT, 1); }
    WREG32_SOC15!(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_CNTL, tmp);
}

static MMHUB_V3_0_1_VMHUB_FUNCS: amdgpu_vmhub_funcs = amdgpu_vmhub_funcs { print_l2_protection_fault_status: Some(mmhub_v3_0_1_print_l2_protection_fault_status), get_invalidate_req: Some(mmhub_v3_0_1_get_invalidate_req) };

unsafe fn mmhub_v3_0_1_init(adev: *mut amdgpu_device) {
    let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0!(0)];
    hub.ctx0_ptb_addr_lo32 = SOC15_REG_OFFSET!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32); hub.ctx0_ptb_addr_hi32 = SOC15_REG_OFFSET!(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32); hub.vm_inv_eng0_sem = SOC15_REG_OFFSET!(MMHUB, 0, regMMVM_INVALIDATE_ENG0_SEM); hub.vm_inv_eng0_req = SOC15_REG_OFFSET!(MMHUB, 0, regMMVM_INVALIDATE_ENG0_REQ); hub.vm_inv_eng0_ack = SOC15_REG_OFFSET!(MMHUB, 0, regMMVM_INVALIDATE_ENG0_ACK); hub.vm_context0_cntl = SOC15_REG_OFFSET!(MMHUB, 0, regMMVM_CONTEXT0_CNTL); hub.vm_l2_pro_fault_status = SOC15_REG_OFFSET!(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_STATUS); hub.vm_l2_pro_fault_cntl = SOC15_REG_OFFSET!(MMHUB, 0, regMMVM_L2_PROTECTION_FAULT_CNTL);
    hub.ctx_distance = regMMVM_CONTEXT1_CNTL - regMMVM_CONTEXT0_CNTL; hub.ctx_addr_distance = regMMVM_CONTEXT1_PAGE_TABLE_BASE_ADDR_LO32 - regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32; hub.eng_distance = regMMVM_INVALIDATE_ENG1_REQ - regMMVM_INVALIDATE_ENG0_REQ; hub.eng_addr_distance = regMMVM_INVALIDATE_ENG1_ADDR_RANGE_LO32 - regMMVM_INVALIDATE_ENG0_ADDR_RANGE_LO32;
    hub.vm_cntx_cntl_vm_fault = MMVM_CONTEXT1_CNTL__RANGE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK | MMVM_CONTEXT1_CNTL__DUMMY_PAGE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK | MMVM_CONTEXT1_CNTL__PDE0_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK | MMVM_CONTEXT1_CNTL__VALID_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK | MMVM_CONTEXT1_CNTL__READ_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK | MMVM_CONTEXT1_CNTL__WRITE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK | MMVM_CONTEXT1_CNTL__EXECUTE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK;
    hub.vmhub_funcs = &MMHUB_V3_0_1_VMHUB_FUNCS; amdgpu_mmhub_init_client_info!(&mut (*adev).mmhub, MMHUB_CLIENT_IDS_V3_0_1.as_ptr(), MMHUB_CLIENT_IDS_V3_0_1.len());
}

unsafe fn mmhub_v3_0_1_get_fb_location(adev: *mut amdgpu_device) -> u64 { let mut base = RREG32_SOC15!(MMHUB, 0, regMMMC_VM_FB_LOCATION_BASE); base &= MMMC_VM_FB_LOCATION_BASE__FB_BASE_MASK; (base as u64) << 24 }
unsafe fn mmhub_v3_0_1_get_mc_fb_offset(adev: *mut amdgpu_device) -> u64 { (RREG32_SOC15!(MMHUB, 0, regMMMC_VM_FB_OFFSET) as u64) << 24 }
unsafe fn mmhub_v3_0_1_update_medium_grain_clock_gating(adev: *mut amdgpu_device, enable: bool) { let def = RREG32_SOC15!(MMHUB, 0, regMM_ATC_L2_MISC_CG); let data = if enable { def | MM_ATC_L2_MISC_CG__ENABLE_MASK } else { def & !MM_ATC_L2_MISC_CG__ENABLE_MASK }; if def != data { WREG32_SOC15!(MMHUB, 0, regMM_ATC_L2_MISC_CG, data); } }
unsafe fn mmhub_v3_0_1_update_medium_grain_light_sleep(adev: *mut amdgpu_device, enable: bool) { let def = RREG32_SOC15!(MMHUB, 0, regMM_ATC_L2_MISC_CG); let data = if enable { def | MM_ATC_L2_MISC_CG__MEM_LS_ENABLE_MASK } else { def & !MM_ATC_L2_MISC_CG__MEM_LS_ENABLE_MASK }; if def != data { WREG32_SOC15!(MMHUB, 0, regMM_ATC_L2_MISC_CG, data); } }
unsafe fn mmhub_v3_0_1_set_clockgating(adev: *mut amdgpu_device, state: amd_clockgating_state) -> i32 { if amdgpu_sriov_vf!(adev) { return 0; } let gate = state == AMD_CG_STATE_GATE; mmhub_v3_0_1_update_medium_grain_clock_gating(adev, gate); mmhub_v3_0_1_update_medium_grain_light_sleep(adev, gate); 0 }
unsafe fn mmhub_v3_0_1_get_clockgating(adev: *mut amdgpu_device, flags: *mut u64) { if amdgpu_sriov_vf!(adev) { *flags = 0; } let data = RREG32_SOC15!(MMHUB, 0, regMM_ATC_L2_MISC_CG); if data & MM_ATC_L2_MISC_CG__ENABLE_MASK != 0 { *flags |= AMD_CG_SUPPORT_MC_MGCG; } if data & MM_ATC_L2_MISC_CG__MEM_LS_ENABLE_MASK != 0 { *flags |= AMD_CG_SUPPORT_MC_LS; } }

pub static MMHUB_V3_0_1_FUNCS: amdgpu_mmhub_funcs = amdgpu_mmhub_funcs { init: Some(mmhub_v3_0_1_init), get_fb_location: Some(mmhub_v3_0_1_get_fb_location), get_mc_fb_offset: Some(mmhub_v3_0_1_get_mc_fb_offset), gart_enable: Some(mmhub_v3_0_1_gart_enable), set_fault_enable_default: Some(mmhub_v3_0_1_set_fault_enable_default), gart_disable: Some(mmhub_v3_0_1_gart_disable), set_clockgating: Some(mmhub_v3_0_1_set_clockgating), get_clockgating: Some(mmhub_v3_0_1_get_clockgating), setup_vm_pt_regs: Some(mmhub_v3_0_1_setup_vm_pt_regs) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
