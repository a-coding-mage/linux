/* Translated from gfxhub_v1_0.c. External types, constants, and macros are
 * supplied by the surrounding AMDGPU bindings. */

unsafe fn gfxhub_v1_0_get_mc_fb_offset(adev: *mut amdgpu_device) -> u64 {
    (RREG32_SOC15!(GC, 0, mmMC_VM_FB_OFFSET) as u64) << 24
}

unsafe fn gfxhub_v1_0_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let hub = &mut (*adev).vmhub[AMDGPU_GFXHUB!(0) as usize];
    WREG32_SOC15_OFFSET!(GC, 0, mmVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32,
        hub.ctx_addr_distance * vmid, lower_32_bits(page_table_base));
    WREG32_SOC15_OFFSET!(GC, 0, mmVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32,
        hub.ctx_addr_distance * vmid, upper_32_bits(page_table_base));
}

unsafe fn gfxhub_v1_0_init_gart_aperture_regs(adev: *mut amdgpu_device) {
    let pt_base: u64 = if !(*adev).gmc.pdb0_bo.is_null() {
        amdgpu_gmc_pd_addr((*adev).gmc.pdb0_bo)
    } else { amdgpu_gmc_pd_addr((*adev).gart.bo) };
    gfxhub_v1_0_setup_vm_pt_regs(adev, 0, pt_base);
    if !(*adev).gmc.pdb0_bo.is_null() {
        WREG32_SOC15!(GC, 0, mmVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, (*adev).gmc.fb_start >> 12);
        WREG32_SOC15!(GC, 0, mmVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, (*adev).gmc.fb_start >> 44);
        WREG32_SOC15!(GC, 0, mmVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, (*adev).gmc.gart_end >> 12);
        WREG32_SOC15!(GC, 0, mmVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, (*adev).gmc.gart_end >> 44);
    } else {
        WREG32_SOC15!(GC, 0, mmVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, (*adev).gmc.gart_start >> 12);
        WREG32_SOC15!(GC, 0, mmVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, (*adev).gmc.gart_start >> 44);
        WREG32_SOC15!(GC, 0, mmVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, (*adev).gmc.gart_end >> 12);
        WREG32_SOC15!(GC, 0, mmVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, (*adev).gmc.gart_end >> 44);
    }
}

unsafe fn gfxhub_v1_0_init_system_aperture_regs(adev: *mut amdgpu_device) {
    let mut value: u64;
    if !amdgpu_sriov_vf(adev) || (*adev).asic_type <= CHIP_VEGA10 {
        WREG32_SOC15_RLC!(GC, 0, mmMC_VM_AGP_BASE, 0);
        WREG32_SOC15_RLC!(GC, 0, mmMC_VM_AGP_BOT, (*adev).gmc.agp_start >> 24);
        WREG32_SOC15_RLC!(GC, 0, mmMC_VM_AGP_TOP, (*adev).gmc.agp_end >> 24);
        WREG32_SOC15_RLC!(GC, 0, mmMC_VM_SYSTEM_APERTURE_LOW_ADDR,
            core::cmp::min((*adev).gmc.fb_start, (*adev).gmc.agp_start) >> 18);
        if (*adev).apu_flags & (AMD_APU_IS_RAVEN2 | AMD_APU_IS_RENOIR | AMD_APU_IS_GREEN_SARDINE) != 0 {
            WREG32_SOC15_RLC!(GC, 0, mmMC_VM_SYSTEM_APERTURE_HIGH_ADDR,
                core::cmp::max(((*adev).gmc.fb_end >> 18) + 1, (*adev).gmc.agp_end >> 18));
        } else {
            WREG32_SOC15_RLC!(GC, 0, mmMC_VM_SYSTEM_APERTURE_HIGH_ADDR,
                core::cmp::max((*adev).gmc.fb_end, (*adev).gmc.agp_end) >> 18);
        }
        value = amdgpu_gmc_vram_mc2pa(adev, (*adev).mem_scratch.gpu_addr);
        WREG32_SOC15!(GC, 0, mmMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, value >> 12);
        WREG32_SOC15!(GC, 0, mmMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, value >> 44);
        WREG32_SOC15!(GC, 0, mmVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32, (*adev).dummy_page_addr >> 12);
        WREG32_SOC15!(GC, 0, mmVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32, (*adev).dummy_page_addr >> 44);
        WREG32_FIELD15!(GC, 0, VM_L2_PROTECTION_FAULT_CNTL2, ACTIVE_PAGE_MIGRATION_PTE_READ_RETRY, 1);
    }
    if !(*adev).gmc.pdb0_bo.is_null() {
        WREG32_SOC15!(GC, 0, mmMC_VM_FB_LOCATION_TOP, 0);
        WREG32_SOC15!(GC, 0, mmMC_VM_FB_LOCATION_BASE, 0x00ffffff);
        WREG32_SOC15!(GC, 0, mmMC_VM_AGP_TOP, 0);
        WREG32_SOC15!(GC, 0, mmMC_VM_AGP_BOT, 0xffffff);
        WREG32_SOC15!(GC, 0, mmMC_VM_SYSTEM_APERTURE_LOW_ADDR, 0x3fffffff);
        WREG32_SOC15!(GC, 0, mmMC_VM_SYSTEM_APERTURE_HIGH_ADDR, 0);
    }
}

unsafe fn gfxhub_v1_0_init_tlb_regs(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15!(GC, 0, mmMC_VM_MX_L1_TLB_CNTL);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 1);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, SYSTEM_ACCESS_MODE, 3);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 1);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, SYSTEM_APERTURE_UNMAPPED_ACCESS, 0);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, MTYPE, MTYPE_UC);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ATC_EN, 1);
    WREG32_SOC15_RLC!(GC, 0, mmMC_VM_MX_L1_TLB_CNTL, tmp);
}

unsafe fn gfxhub_v1_0_init_cache_regs(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15!(GC, 0, mmVM_L2_CNTL);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, ENABLE_L2_CACHE, 1);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, ENABLE_L2_FRAGMENT_PROCESSING, 1);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, L2_PDE0_CACHE_TAG_GENERATION_MODE, 0);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, PDE_FAULT_CLASSIFICATION, 0);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, CONTEXT1_IDENTITY_ACCESS_MODE, 1);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, IDENTITY_MODE_FRAGMENT_SIZE, 0);
    WREG32_SOC15_RLC!(GC, 0, mmVM_L2_CNTL, tmp);
    tmp = RREG32_SOC15!(GC, 0, mmVM_L2_CNTL2);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL2, INVALIDATE_ALL_L1_TLBS, 1);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL2, INVALIDATE_L2_CACHE, 1);
    WREG32_SOC15_RLC!(GC, 0, mmVM_L2_CNTL2, tmp);
    tmp = mmVM_L2_CNTL3_DEFAULT;
    if (*adev).gmc.translate_further {
        tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL3, BANK_SELECT, 12);
        tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 9);
    } else {
        tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL3, BANK_SELECT, 9);
        tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 6);
    }
    WREG32_SOC15_RLC!(GC, 0, mmVM_L2_CNTL3, tmp);
    tmp = mmVM_L2_CNTL4_DEFAULT;
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL4, VMC_TAP_PDE_REQUEST_PHYSICAL, (*adev).gmc.xgmi.connected_to_cpu as u32);
    tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL4, VMC_TAP_PTE_REQUEST_PHYSICAL, (*adev).gmc.xgmi.connected_to_cpu as u32);
    WREG32_SOC15_RLC!(GC, 0, mmVM_L2_CNTL4, tmp);
}

unsafe fn gfxhub_v1_0_enable_system_domain(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15!(GC, 0, mmVM_CONTEXT0_CNTL);
    tmp = REG_SET_FIELD!(tmp, VM_CONTEXT0_CNTL, ENABLE_CONTEXT, 1);
    tmp = REG_SET_FIELD!(tmp, VM_CONTEXT0_CNTL, PAGE_TABLE_DEPTH, (*adev).gmc.vmid0_page_table_depth);
    tmp = REG_SET_FIELD!(tmp, VM_CONTEXT0_CNTL, PAGE_TABLE_BLOCK_SIZE, (*adev).gmc.vmid0_page_table_block_size);
    tmp = REG_SET_FIELD!(tmp, VM_CONTEXT0_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, 0);
    WREG32_SOC15!(GC, 0, mmVM_CONTEXT0_CNTL, tmp);
}

unsafe fn gfxhub_v1_0_disable_identity_aperture(adev: *mut amdgpu_device) {
    WREG32_SOC15!(GC, 0, mmVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_LO32, 0xffffffff);
    WREG32_SOC15!(GC, 0, mmVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_HI32, 0x0000000f);
    WREG32_SOC15!(GC, 0, mmVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_LO32, 0);
    WREG32_SOC15!(GC, 0, mmVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_HI32, 0);
    WREG32_SOC15!(GC, 0, mmVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_LO32, 0);
    WREG32_SOC15!(GC, 0, mmVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_HI32, 0);
}

unsafe fn gfxhub_v1_0_setup_vmid_config(adev: *mut amdgpu_device) {
    let hub = &mut (*adev).vmhub[AMDGPU_GFXHUB!(0) as usize];
    let mut num_level = (*adev).vm_manager.num_level;
    let mut block_size = (*adev).vm_manager.block_size;
    if (*adev).gmc.translate_further { num_level -= 1; } else { block_size -= 9; }
    for i in 0..=14 {
        let off = i * hub.ctx_distance;
        let mut tmp = RREG32_SOC15_OFFSET!(GC, 0, mmVM_CONTEXT1_CNTL, off);
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
        tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT,
            (!(*adev).gmc.noretry || (*adev).asic_type == CHIP_ALDEBARAN) as u32);
        WREG32_SOC15_OFFSET!(GC, 0, mmVM_CONTEXT1_CNTL, off, tmp);
        let aoff = i * hub.ctx_addr_distance;
        WREG32_SOC15_OFFSET!(GC, 0, mmVM_CONTEXT1_PAGE_TABLE_START_ADDR_LO32, aoff, 0);
        WREG32_SOC15_OFFSET!(GC, 0, mmVM_CONTEXT1_PAGE_TABLE_START_ADDR_HI32, aoff, 0);
        WREG32_SOC15_OFFSET!(GC, 0, mmVM_CONTEXT1_PAGE_TABLE_END_ADDR_LO32, aoff, lower_32_bits((*adev).vm_manager.max_pfn - 1));
        WREG32_SOC15_OFFSET!(GC, 0, mmVM_CONTEXT1_PAGE_TABLE_END_ADDR_HI32, aoff, upper_32_bits((*adev).vm_manager.max_pfn - 1));
    }
}

unsafe fn gfxhub_v1_0_program_invalidation(adev: *mut amdgpu_device) {
    let hub = &mut (*adev).vmhub[AMDGPU_GFXHUB!(0) as usize];
    for i in 0..18 {
        let off = i * hub.eng_addr_distance;
        WREG32_SOC15_OFFSET!(GC, 0, mmVM_INVALIDATE_ENG0_ADDR_RANGE_LO32, off, 0xffffffff);
        WREG32_SOC15_OFFSET!(GC, 0, mmVM_INVALIDATE_ENG0_ADDR_RANGE_HI32, off, 0x1f);
    }
}

unsafe fn gfxhub_v1_0_gart_enable(adev: *mut amdgpu_device) -> i32 {
    gfxhub_v1_0_init_gart_aperture_regs(adev); gfxhub_v1_0_init_system_aperture_regs(adev);
    gfxhub_v1_0_init_tlb_regs(adev); if !amdgpu_sriov_vf(adev) { gfxhub_v1_0_init_cache_regs(adev); }
    gfxhub_v1_0_enable_system_domain(adev); if !amdgpu_sriov_vf(adev) { gfxhub_v1_0_disable_identity_aperture(adev); }
    gfxhub_v1_0_setup_vmid_config(adev); gfxhub_v1_0_program_invalidation(adev); 0
}

unsafe fn gfxhub_v1_0_gart_disable(adev: *mut amdgpu_device) {
    let hub = &mut (*adev).vmhub[AMDGPU_GFXHUB!(0) as usize];
    for i in 0..16 { WREG32_SOC15_OFFSET!(GC, 0, mmVM_CONTEXT0_CNTL, i * hub.ctx_distance, 0); }
    if amdgpu_sriov_vf(adev) { return; }
    let mut tmp = RREG32_SOC15!(GC, 0, mmMC_VM_MX_L1_TLB_CNTL);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 0);
    tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 0);
    WREG32_SOC15_RLC!(GC, 0, mmMC_VM_MX_L1_TLB_CNTL, tmp);
    WREG32_FIELD15!(GC, 0, VM_L2_CNTL, ENABLE_L2_CACHE, 0);
    WREG32_SOC15!(GC, 0, mmVM_L2_CNTL3, 0);
}

unsafe fn gfxhub_v1_0_set_fault_enable_default(adev: *mut amdgpu_device, value: bool) {
    let mut tmp = RREG32_SOC15!(GC, 0, mmVM_L2_PROTECTION_FAULT_CNTL);
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
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, CRASH_ON_NO_RETRY_FAULT, !value);
    tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, CRASH_ON_RETRY_FAULT, !value);
    WREG32_SOC15!(GC, 0, mmVM_L2_PROTECTION_FAULT_CNTL, tmp);
}

unsafe fn gfxhub_v1_0_init(adev: *mut amdgpu_device) {
    let hub = &mut (*adev).vmhub[AMDGPU_GFXHUB!(0) as usize];
    hub.ctx0_ptb_addr_lo32 = SOC15_REG_OFFSET!(GC, 0, mmVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32);
    hub.ctx0_ptb_addr_hi32 = SOC15_REG_OFFSET!(GC, 0, mmVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32);
    hub.vm_inv_eng0_sem = SOC15_REG_OFFSET!(GC, 0, mmVM_INVALIDATE_ENG0_SEM);
    hub.vm_inv_eng0_req = SOC15_REG_OFFSET!(GC, 0, mmVM_INVALIDATE_ENG0_REQ);
    hub.vm_inv_eng0_ack = SOC15_REG_OFFSET!(GC, 0, mmVM_INVALIDATE_ENG0_ACK);
    hub.vm_context0_cntl = SOC15_REG_OFFSET!(GC, 0, mmVM_CONTEXT0_CNTL);
    hub.vm_l2_pro_fault_status = SOC15_REG_OFFSET!(GC, 0, mmVM_L2_PROTECTION_FAULT_STATUS);
    hub.vm_l2_pro_fault_cntl = SOC15_REG_OFFSET!(GC, 0, mmVM_L2_PROTECTION_FAULT_CNTL);
    hub.ctx_distance = mmVM_CONTEXT1_CNTL - mmVM_CONTEXT0_CNTL;
    hub.ctx_addr_distance = mmVM_CONTEXT1_PAGE_TABLE_BASE_ADDR_LO32 - mmVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32;
    hub.eng_distance = mmVM_INVALIDATE_ENG1_REQ - mmVM_INVALIDATE_ENG0_REQ;
    hub.eng_addr_distance = mmVM_INVALIDATE_ENG1_ADDR_RANGE_LO32 - mmVM_INVALIDATE_ENG0_ADDR_RANGE_LO32;
}

pub static gfxhub_v1_0_funcs: amdgpu_gfxhub_funcs = amdgpu_gfxhub_funcs {
    get_mc_fb_offset: Some(gfxhub_v1_0_get_mc_fb_offset), setup_vm_pt_regs: Some(gfxhub_v1_0_setup_vm_pt_regs),
    gart_enable: Some(gfxhub_v1_0_gart_enable), gart_disable: Some(gfxhub_v1_0_gart_disable),
    set_fault_enable_default: Some(gfxhub_v1_0_set_fault_enable_default), init: Some(gfxhub_v1_0_init),
    get_xgmi_info: Some(gfxhub_v1_1_get_xgmi_info),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
