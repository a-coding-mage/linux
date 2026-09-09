/* Translated from mmhub_v1_0.c. External kernel types, constants, and register
 * accessors are supplied by the surrounding translation unit. */

const MM_DAGB0_CNTL_MISC2_RV: u32 = 0x008f;
const MM_DAGB0_CNTL_MISC2_RV_BASE_IDX: u32 = 0;

unsafe fn mmhub_v1_0_get_fb_location(adev: *mut amdgpu_device) -> u64 {
    let mut base = RREG32_SOC15!(MMHUB, 0, mmMC_VM_FB_LOCATION_BASE) as u64;
    let mut top = RREG32_SOC15!(MMHUB, 0, mmMC_VM_FB_LOCATION_TOP) as u64;
    base = (base & MC_VM_FB_LOCATION_BASE__FB_BASE_MASK as u64) << 24;
    top = (top & MC_VM_FB_LOCATION_TOP__FB_TOP_MASK as u64) << 24;
    (*adev).gmc.fb_start = base;
    (*adev).gmc.fb_end = top;
    base
}

unsafe fn mmhub_v1_0_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0!(0)];
    WREG32_SOC15_OFFSET!(MMHUB, 0, mmVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32,
        hub.ctx_addr_distance * vmid, lower_32_bits(page_table_base));
    WREG32_SOC15_OFFSET!(MMHUB, 0, mmVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32,
        hub.ctx_addr_distance * vmid, upper_32_bits(page_table_base));
}

unsafe fn mmhub_v1_0_init_gart_aperture_regs(adev: *mut amdgpu_device) {
    let pt_base = amdgpu_gmc_pd_addr!((*adev).gart.bo);
    mmhub_v1_0_setup_vm_pt_regs(adev, 0, pt_base);
    WREG32_SOC15!(MMHUB, 0, mmVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, ((*adev).gmc.gart_start >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, mmVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, ((*adev).gmc.gart_start >> 44) as u32);
    WREG32_SOC15!(MMHUB, 0, mmVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, ((*adev).gmc.gart_end >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, mmVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, ((*adev).gmc.gart_end >> 44) as u32);
}

unsafe fn mmhub_v1_0_init_system_aperture_regs(adev: *mut amdgpu_device) {
    WREG32_SOC15!(MMHUB, 0, mmMC_VM_AGP_BASE, 0);
    WREG32_SOC15!(MMHUB, 0, mmMC_VM_AGP_BOT, (*adev).gmc.agp_start >> 24);
    WREG32_SOC15!(MMHUB, 0, mmMC_VM_AGP_TOP, (*adev).gmc.agp_end >> 24);
    WREG32_SOC15!(MMHUB, 0, mmMC_VM_SYSTEM_APERTURE_LOW_ADDR,
        core::cmp::min((*adev).gmc.fb_start, (*adev).gmc.agp_start) >> 18);
    if (*adev).apu_flags & (AMD_APU_IS_RAVEN2 | AMD_APU_IS_RENOIR | AMD_APU_IS_GREEN_SARDINE) != 0 {
        WREG32_SOC15!(MMHUB, 0, mmMC_VM_SYSTEM_APERTURE_HIGH_ADDR,
            core::cmp::max(((*adev).gmc.fb_end >> 18) + 1, (*adev).gmc.agp_end >> 18));
    } else {
        WREG32_SOC15!(MMHUB, 0, mmMC_VM_SYSTEM_APERTURE_HIGH_ADDR,
            core::cmp::max((*adev).gmc.fb_end, (*adev).gmc.agp_end) >> 18);
    }
    if amdgpu_sriov_vf!(adev) { return; }
    let value = amdgpu_gmc_vram_mc2pa!(adev, (*adev).mem_scratch.gpu_addr);
    WREG32_SOC15!(MMHUB, 0, mmMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, (value >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, mmMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, (value >> 44) as u32);
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32, ((*adev).dummy_page_addr >> 12) as u32);
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32, ((*adev).dummy_page_addr >> 44) as u32);
    let mut tmp = RREG32_SOC15!(MMHUB, 0, mmVM_L2_PROTECTION_FAULT_CNTL2);
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL2, ACTIVE_PAGE_MIGRATION_PTE_READ_RETRY, 1);
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_PROTECTION_FAULT_CNTL2, tmp);
}

unsafe fn mmhub_v1_0_init_tlb_regs(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15!(MMHUB, 0, mmMC_VM_MX_L1_TLB_CNTL);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 1);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, SYSTEM_ACCESS_MODE, 3);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 1);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, SYSTEM_APERTURE_UNMAPPED_ACCESS, 0);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, MTYPE, MTYPE_UC);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ATC_EN, 1);
    WREG32_SOC15!(MMHUB, 0, mmMC_VM_MX_L1_TLB_CNTL, tmp);
}

unsafe fn mmhub_v1_0_enable_system_domain(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15!(MMHUB, 0, mmVM_CONTEXT0_CNTL);
    tmp = REG_SET_FIELD!(tmp, VM_CONTEXT0_CNTL, ENABLE_CONTEXT, 1);
    tmp = REG_SET_FIELD!(tmp, VM_CONTEXT0_CNTL, PAGE_TABLE_DEPTH, 0);
    tmp = REG_SET_FIELD!(tmp, VM_CONTEXT0_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, 0);
    WREG32_SOC15!(MMHUB, 0, mmVM_CONTEXT0_CNTL, tmp);
}

unsafe fn mmhub_v1_0_init_cache_regs(adev: *mut amdgpu_device) {
    if amdgpu_sriov_vf!(adev) { return; }
    let mut tmp = RREG32_SOC15!(MMHUB, 0, mmVM_L2_CNTL);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, ENABLE_L2_CACHE, 1);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, ENABLE_L2_FRAGMENT_PROCESSING, 1);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, L2_PDE0_CACHE_TAG_GENERATION_MODE, 0);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, PDE_FAULT_CLASSIFICATION, 0);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, CONTEXT1_IDENTITY_ACCESS_MODE, 1);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, IDENTITY_MODE_FRAGMENT_SIZE, 0);
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_CNTL, tmp);
    tmp = RREG32_SOC15!(MMHUB, 0, mmVM_L2_CNTL2);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL2, INVALIDATE_ALL_L1_TLBS, 1);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL2, INVALIDATE_L2_CACHE, 1);
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_CNTL2, tmp);
    tmp = mmVM_L2_CNTL3_DEFAULT;
    if (*adev).gmc.translate_further {
        tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL3, BANK_SELECT, 12);
        tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 9);
    } else {
        tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL3, BANK_SELECT, 9);
        tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 6);
    }
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_CNTL3, tmp);
    tmp = mmVM_L2_CNTL4_DEFAULT;
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL4, VMC_TAP_PDE_REQUEST_PHYSICAL, 0);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL4, VMC_TAP_PTE_REQUEST_PHYSICAL, 0);
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_CNTL4, tmp);
}

unsafe fn mmhub_v1_0_gart_enable(adev: *mut amdgpu_device) -> i32 {
    if amdgpu_sriov_vf!(adev) {
        WREG32_SOC15!(MMHUB, 0, mmMC_VM_FB_LOCATION_BASE, (*adev).gmc.vram_start >> 24);
        WREG32_SOC15!(MMHUB, 0, mmMC_VM_FB_LOCATION_TOP, (*adev).gmc.vram_end >> 24);
    }
    mmhub_v1_0_init_gart_aperture_regs(adev);
    mmhub_v1_0_init_system_aperture_regs(adev);
    mmhub_v1_0_init_tlb_regs(adev);
    mmhub_v1_0_init_cache_regs(adev);
    mmhub_v1_0_enable_system_domain(adev);
    mmhub_v1_0_disable_identity_aperture(adev);
    mmhub_v1_0_setup_vmid_config(adev);
    mmhub_v1_0_program_invalidation(adev);
    0
}

unsafe fn mmhub_v1_0_disable_identity_aperture(adev: *mut amdgpu_device) {
    if amdgpu_sriov_vf!(adev) { return; }
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_LO32, 0xffff_ffff);
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_HI32, 0x0000_000f);
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_LO32, 0);
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_HI32, 0);
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_LO32, 0);
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_HI32, 0);
}

unsafe fn mmhub_v1_0_program_invalidation(adev: *mut amdgpu_device) {
    let hub = &(*adev).vmhub[AMDGPU_MMHUB0!(0)];
    for i in 0..18u32 {
        WREG32_SOC15_OFFSET!(MMHUB, 0, mmVM_INVALIDATE_ENG0_ADDR_RANGE_LO32, i * hub.eng_addr_distance, 0xffff_ffff);
        WREG32_SOC15_OFFSET!(MMHUB, 0, mmVM_INVALIDATE_ENG0_ADDR_RANGE_HI32, i * hub.eng_addr_distance, 0x1f);
    }
}

unsafe fn mmhub_v1_0_update_power_gating(adev: *mut amdgpu_device, enable: bool) {
    if amdgpu_sriov_vf!(adev) { return; }
    if (*adev).pg_flags & AMD_PG_SUPPORT_MMHUB != 0 {
        amdgpu_dpm_set_powergating_by_smu!(adev, AMD_IP_BLOCK_TYPE_GMC, enable, 0);
    }
}

unsafe fn mmhub_v1_0_set_fault_enable_default(adev: *mut amdgpu_device, value: bool) {
    if amdgpu_sriov_vf!(adev) { return; }
    let mut tmp = RREG32_SOC15!(MMHUB, 0, mmVM_L2_PROTECTION_FAULT_CNTL);
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, RANGE_PROTECTION_FAULT_ENABLE_DEFAULT, value);
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, PDE0_PROTECTION_FAULT_ENABLE_DEFAULT, value);
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, PDE1_PROTECTION_FAULT_ENABLE_DEFAULT, value);
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, PDE2_PROTECTION_FAULT_ENABLE_DEFAULT, value);
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, TRANSLATE_FURTHER_PROTECTION_FAULT_ENABLE_DEFAULT, value);
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, NACK_PROTECTION_FAULT_ENABLE_DEFAULT, value);
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT, value);
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, VALID_PROTECTION_FAULT_ENABLE_DEFAULT, value);
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, READ_PROTECTION_FAULT_ENABLE_DEFAULT, value);
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, WRITE_PROTECTION_FAULT_ENABLE_DEFAULT, value);
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT, value);
    if !value {
        tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, CRASH_ON_NO_RETRY_FAULT, 1);
        tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, CRASH_ON_RETRY_FAULT, 1);
    }
    WREG32_SOC15!(MMHUB, 0, mmVM_L2_PROTECTION_FAULT_CNTL, tmp);
}

unsafe fn mmhub_v1_0_setup_vmid_config(adev: *mut amdgpu_device) {
    let hub = &(*adev).vmhub[AMDGPU_MMHUB0!(0)];
    let mut num_level = (*adev).vm_manager.num_level;
    let mut block_size = (*adev).vm_manager.block_size;
    if (*adev).gmc.translate_further { num_level -= 1; } else { block_size -= 9; }
    for i in 0..15u32 {
        let off = i * hub.ctx_distance;
        let mut tmp = RREG32_SOC15_OFFSET!(MMHUB, 0, mmVM_CONTEXT1_CNTL, off);
        tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, ENABLE_CONTEXT, 1);
        tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, PAGE_TABLE_DEPTH, num_level);
        tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, RANGE_PROTECTION_FAULT_ENABLE_DEFAULT, 1);
        tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT, 1);
        tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, PDE0_PROTECTION_FAULT_ENABLE_DEFAULT, 1);
        tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, VALID_PROTECTION_FAULT_ENABLE_DEFAULT, 1);
        tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, READ_PROTECTION_FAULT_ENABLE_DEFAULT, 1);
        tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, WRITE_PROTECTION_FAULT_ENABLE_DEFAULT, 1);
        tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT, 1);
        tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, PAGE_TABLE_BLOCK_SIZE, block_size);
        tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, !(*adev).gmc.noretry);
        WREG32_SOC15_OFFSET!(MMHUB, 0, mmVM_CONTEXT1_CNTL, off, tmp);
        let aoff = i * hub.ctx_addr_distance;
        WREG32_SOC15_OFFSET!(MMHUB, 0, mmVM_CONTEXT1_PAGE_TABLE_START_ADDR_LO32, aoff, 0);
        WREG32_SOC15_OFFSET!(MMHUB, 0, mmVM_CONTEXT1_PAGE_TABLE_START_ADDR_HI32, aoff, 0);
        WREG32_SOC15_OFFSET!(MMHUB, 0, mmVM_CONTEXT1_PAGE_TABLE_END_ADDR_LO32, aoff, lower_32_bits((*adev).vm_manager.max_pfn - 1));
        WREG32_SOC15_OFFSET!(MMHUB, 0, mmVM_CONTEXT1_PAGE_TABLE_END_ADDR_HI32, aoff, upper_32_bits((*adev).vm_manager.max_pfn - 1));
    }
    if amdgpu_ip_version!(adev, ISP_HWIP, 0) != 0 { /* SAW initialization is provided by the MMHUB implementation. */ }
}

unsafe fn mmhub_v1_0_gart_disable(adev: *mut amdgpu_device) {
    let hub = &(*adev).vmhub[AMDGPU_MMHUB0!(0)];
    for i in 0..AMDGPU_NUM_VMID { WREG32_SOC15_OFFSET!(MMHUB, 0, mmVM_CONTEXT0_CNTL, i * hub.ctx_distance, 0); }
    let mut tmp = RREG32_SOC15!(MMHUB, 0, mmMC_VM_MX_L1_TLB_CNTL);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 0);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 0);
    WREG32_SOC15!(MMHUB, 0, mmMC_VM_MX_L1_TLB_CNTL, tmp);
    if !amdgpu_sriov_vf!(adev) {
        tmp = RREG32_SOC15!(MMHUB, 0, mmVM_L2_CNTL);
        tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, ENABLE_L2_CACHE, 0);
        WREG32_SOC15!(MMHUB, 0, mmVM_L2_CNTL, tmp);
        WREG32_SOC15!(MMHUB, 0, mmVM_L2_CNTL3, 0);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
