/* Translated from mmhub_v1_8.c. */

const REGVM_L2_CNTL3_DEFAULT: u32 = 0x80100007;
const REGVM_L2_CNTL4_DEFAULT: u32 = 0x000000c1;

unsafe fn mmhub_v1_8_get_fb_location(adev: *mut amdgpu_device) -> u64 {
    let mut base = RREG32_SOC15!(MMHUB, 0, regMC_VM_FB_LOCATION_BASE) as u64;
    let mut top = RREG32_SOC15!(MMHUB, 0, regMC_VM_FB_LOCATION_TOP) as u64;
    base &= MC_VM_FB_LOCATION_BASE__FB_BASE_MASK as u64;
    base <<= 24;
    top &= MC_VM_FB_LOCATION_TOP__FB_TOP_MASK as u64;
    top <<= 24;
    (*adev).gmc.fb_start = base;
    (*adev).gmc.fb_end = top;
    base
}

unsafe fn mmhub_v1_8_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let inst_mask = (*adev).aid_mask;
    for_each_inst!(i, inst_mask) {
        let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0!(i)];
        WREG32_SOC15_OFFSET!(MMHUB, i, regVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32,
            hub.ctx_addr_distance * vmid, lower_32_bits(page_table_base));
        WREG32_SOC15_OFFSET!(MMHUB, i, regVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32,
            hub.ctx_addr_distance * vmid, upper_32_bits(page_table_base));
    }
}

unsafe fn mmhub_v1_8_init_gart_aperture_regs(adev: *mut amdgpu_device) {
    let gart_start = if amdgpu_virt_xgmi_migrate_enabled(adev) { (*adev).gmc.vram_start } else { (*adev).gmc.fb_start };
    let pt_base = if !(*adev).gmc.pdb0_bo.is_null() { amdgpu_gmc_pd_addr((*adev).gmc.pdb0_bo) } else { amdgpu_gmc_pd_addr((*adev).gart.bo) };
    mmhub_v1_8_setup_vm_pt_regs(adev, 0, pt_base);
    let inst_mask = (*adev).aid_mask;
    for_each_inst!(i, inst_mask) {
        let (start, end) = if !(*adev).gmc.pdb0_bo.is_null() { (gart_start, (*adev).gmc.gart_end) } else { ((*adev).gmc.gart_start, (*adev).gmc.gart_end) };
        WREG32_SOC15!(MMHUB, i, regVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, (start >> 12) as u32);
        WREG32_SOC15!(MMHUB, i, regVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, (start >> 44) as u32);
        WREG32_SOC15!(MMHUB, i, regVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, (end >> 12) as u32);
        WREG32_SOC15!(MMHUB, i, regVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, (end >> 44) as u32);
    }
}

unsafe fn mmhub_v1_8_init_system_aperture_regs(adev: *mut amdgpu_device) {
    if amdgpu_sriov_vf(adev) { return; }
    let inst_mask = (*adev).aid_mask;
    for_each_inst!(i, inst_mask) {
        WREG32_SOC15!(MMHUB, i, regMC_VM_AGP_BASE, 0);
        WREG32_SOC15!(MMHUB, i, regMC_VM_AGP_BOT, (*adev).gmc.agp_start >> 24);
        WREG32_SOC15!(MMHUB, i, regMC_VM_AGP_TOP, (*adev).gmc.agp_end >> 24);
        WREG32_SOC15!(MMHUB, i, regMC_VM_SYSTEM_APERTURE_LOW_ADDR, min((*adev).gmc.fb_start, (*adev).gmc.agp_start) >> 18);
        WREG32_SOC15!(MMHUB, i, regMC_VM_SYSTEM_APERTURE_HIGH_ADDR, max((*adev).gmc.fb_end, (*adev).gmc.agp_end) >> 18);
        if !(*adev).gmc.pdb0_bo.is_null() {
            WREG32_SOC15!(MMHUB, i, regMC_VM_AGP_BOT, 0xFFFFFF); WREG32_SOC15!(MMHUB, i, regMC_VM_AGP_TOP, 0);
            WREG32_SOC15!(MMHUB, i, regMC_VM_FB_LOCATION_TOP, 0); WREG32_SOC15!(MMHUB, i, regMC_VM_FB_LOCATION_BASE, 0x00FFFFFF);
            WREG32_SOC15!(MMHUB, i, regMC_VM_SYSTEM_APERTURE_LOW_ADDR, 0x3FFFFFFF);
            WREG32_SOC15!(MMHUB, i, regMC_VM_SYSTEM_APERTURE_HIGH_ADDR, 0);
        }
        let value = amdgpu_gmc_vram_mc2pa(adev, (*adev).mem_scratch.gpu_addr);
        WREG32_SOC15!(MMHUB, i, regMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, (value >> 12) as u32);
        WREG32_SOC15!(MMHUB, i, regMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, (value >> 44) as u32);
        WREG32_SOC15!(MMHUB, i, regVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32, ((*adev).dummy_page_addr >> 12) as u32);
        WREG32_SOC15!(MMHUB, i, regVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32, ((*adev).dummy_page_addr >> 44) as u32);
        let mut tmp = RREG32_SOC15!(MMHUB, i, regVM_L2_PROTECTION_FAULT_CNTL2);
        tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL2, ACTIVE_PAGE_MIGRATION_PTE_READ_RETRY, 1);
        WREG32_SOC15!(MMHUB, i, regVM_L2_PROTECTION_FAULT_CNTL2, tmp);
    }
}

unsafe fn mmhub_v1_8_init_tlb_regs(adev: *mut amdgpu_device) {
    let mut tmp;
    if amdgpu_sriov_reg_indirect_l1_tlb_cntl(adev) {
        tmp = RREG32_SOC15!(MMHUB, 0, regMC_VM_MX_L1_TLB_CNTL);
        tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 1); tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, SYSTEM_ACCESS_MODE, 3);
        tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 1); tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, SYSTEM_APERTURE_UNMAPPED_ACCESS, 0);
        tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, MTYPE, MTYPE_UC); tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ATC_EN, 1);
        psp_reg_program_no_ring(&mut (*adev).psp, tmp, PSP_REG_MMHUB_L1_TLB_CNTL);
    } else { let inst_mask = (*adev).aid_mask; for_each_inst!(i, inst_mask) {
        tmp = RREG32_SOC15!(MMHUB, i, regMC_VM_MX_L1_TLB_CNTL);
        tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 1); tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, SYSTEM_ACCESS_MODE, 3);
        tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 1); tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, SYSTEM_APERTURE_UNMAPPED_ACCESS, 0);
        tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, MTYPE, MTYPE_UC); tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ATC_EN, 1);
        WREG32_SOC15!(MMHUB, i, regMC_VM_MX_L1_TLB_CNTL, tmp);
    }}
}

unsafe fn mmhub_v1_8_init_snoop_override_regs(adev: *mut amdgpu_device) {
    if amdgpu_sriov_vf(adev) { return; }
    let distance = regDAGB1_WRCLI_GPU_SNOOP_OVERRIDE - regDAGB0_WRCLI_GPU_SNOOP_OVERRIDE;
    let inst_mask = (*adev).aid_mask;
    for_each_inst!(i, inst_mask) { for j in 0..5 { let off = j * distance;
        let mut tmp = RREG32_SOC15_OFFSET!(MMHUB, i, regDAGB0_WRCLI_GPU_SNOOP_OVERRIDE, off); tmp |= 1 << 15; WREG32_SOC15_OFFSET!(MMHUB, i, regDAGB0_WRCLI_GPU_SNOOP_OVERRIDE, off, tmp);
        tmp = RREG32_SOC15_OFFSET!(MMHUB, i, regDAGB0_WRCLI_GPU_SNOOP_OVERRIDE_VALUE, off); tmp |= 1 << 15; WREG32_SOC15_OFFSET!(MMHUB, i, regDAGB0_WRCLI_GPU_SNOOP_OVERRIDE_VALUE, off, tmp);
    }}
}

unsafe fn mmhub_v1_8_init_cache_regs(adev: *mut amdgpu_device) {
    if amdgpu_sriov_vf(adev) { return; }
    let inst_mask = (*adev).aid_mask;
    for_each_inst!(i, inst_mask) {
        let mut tmp = RREG32_SOC15!(MMHUB, i, regVM_L2_CNTL);
        tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, ENABLE_L2_CACHE, 1); tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, ENABLE_L2_FRAGMENT_PROCESSING, 1);
        tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, L2_PDE0_CACHE_TAG_GENERATION_MODE, 0); tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, PDE_FAULT_CLASSIFICATION, 0);
        tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, CONTEXT1_IDENTITY_ACCESS_MODE, 1); tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, IDENTITY_MODE_FRAGMENT_SIZE, 0); WREG32_SOC15!(MMHUB, i, regVM_L2_CNTL, tmp);
        tmp = RREG32_SOC15!(MMHUB, i, regVM_L2_CNTL2); tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL2, INVALIDATE_ALL_L1_TLBS, 1); tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL2, INVALIDATE_L2_CACHE, 1); WREG32_SOC15!(MMHUB, i, regVM_L2_CNTL2, tmp);
        tmp = REGVM_L2_CNTL3_DEFAULT; if (*adev).gmc.translate_further { tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL3, BANK_SELECT, 12); tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 9); } else { tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL3, BANK_SELECT, 9); tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 6); } WREG32_SOC15!(MMHUB, i, regVM_L2_CNTL3, tmp);
        tmp = REGVM_L2_CNTL4_DEFAULT; let physical = (*adev).gmc.xgmi.connected_to_cpu || (*adev).gmc.is_app_apu; tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL4, VMC_TAP_PDE_REQUEST_PHYSICAL, physical as u32); tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL4, VMC_TAP_PTE_REQUEST_PHYSICAL, physical as u32); WREG32_SOC15!(MMHUB, i, regVM_L2_CNTL4, tmp);
    }
}

unsafe fn mmhub_v1_8_enable_system_domain(adev: *mut amdgpu_device) { let inst_mask = (*adev).aid_mask; for_each_inst!(i, inst_mask) { let mut tmp = RREG32_SOC15!(MMHUB, i, regVM_CONTEXT0_CNTL); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT0_CNTL, ENABLE_CONTEXT, 1); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT0_CNTL, PAGE_TABLE_DEPTH, (*adev).gmc.vmid0_page_table_depth); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT0_CNTL, PAGE_TABLE_BLOCK_SIZE, (*adev).gmc.vmid0_page_table_block_size); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT0_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, 0); WREG32_SOC15!(MMHUB, i, regVM_CONTEXT0_CNTL, tmp); }}

unsafe fn mmhub_v1_8_disable_identity_aperture(adev: *mut amdgpu_device) { if amdgpu_sriov_vf(adev) { return; } let inst_mask = (*adev).aid_mask; for_each_inst!(i, inst_mask) { WREG32_SOC15!(MMHUB, i, regVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_LO32, 0xFFFFFFFF); WREG32_SOC15!(MMHUB, i, regVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_HI32, 0x0000000F); WREG32_SOC15!(MMHUB, i, regVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_LO32, 0); WREG32_SOC15!(MMHUB, i, regVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_HI32, 0); WREG32_SOC15!(MMHUB, i, regVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_LO32, 0); WREG32_SOC15!(MMHUB, i, regVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_HI32, 0); }}

unsafe fn mmhub_v1_8_setup_vmid_config(adev: *mut amdgpu_device) { let mut num_level = (*adev).vm_manager.num_level; let mut block_size = (*adev).vm_manager.block_size; if (*adev).gmc.translate_further { num_level -= 1; } else { block_size -= 9; } let inst_mask = (*adev).aid_mask; for_each_inst!(j, inst_mask) { let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0!(j)]; for i in 0..=14 { let off = i * hub.ctx_distance; let mut tmp = RREG32_SOC15_OFFSET!(MMHUB, j, regVM_CONTEXT1_CNTL, off); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, ENABLE_CONTEXT, 1); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, PAGE_TABLE_DEPTH, num_level); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, RANGE_PROTECTION_FAULT_ENABLE_DEFAULT, 1); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT, 1); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, PDE0_PROTECTION_FAULT_ENABLE_DEFAULT, 1); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, VALID_PROTECTION_FAULT_ENABLE_DEFAULT, 1); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, READ_PROTECTION_FAULT_ENABLE_DEFAULT, 1); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, WRITE_PROTECTION_FAULT_ENABLE_DEFAULT, 1); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT, 1); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, PAGE_TABLE_BLOCK_SIZE, block_size); tmp = REG_SET_FIELD!(tmp, VM_CONTEXT1_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, 1); WREG32_SOC15_OFFSET!(MMHUB, j, regVM_CONTEXT1_CNTL, off, tmp); let aoff = i * hub.ctx_addr_distance; WREG32_SOC15_OFFSET!(MMHUB, j, regVM_CONTEXT1_PAGE_TABLE_START_ADDR_LO32, aoff, 0); WREG32_SOC15_OFFSET!(MMHUB, j, regVM_CONTEXT1_PAGE_TABLE_START_ADDR_HI32, aoff, 0); WREG32_SOC15_OFFSET!(MMHUB, j, regVM_CONTEXT1_PAGE_TABLE_END_ADDR_LO32, aoff, lower_32_bits((*adev).vm_manager.max_pfn - 1)); WREG32_SOC15_OFFSET!(MMHUB, j, regVM_CONTEXT1_PAGE_TABLE_END_ADDR_HI32, aoff, upper_32_bits((*adev).vm_manager.max_pfn - 1)); } }}

unsafe fn mmhub_v1_8_program_invalidation(adev: *mut amdgpu_device) { let inst_mask = (*adev).aid_mask; for_each_inst!(j, inst_mask) { let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0!(j)]; for i in 0..18 { let off = i * hub.eng_addr_distance; WREG32_SOC15_OFFSET!(MMHUB, j, regVM_INVALIDATE_ENG0_ADDR_RANGE_LO32, off, 0xffffffff); WREG32_SOC15_OFFSET!(MMHUB, j, regVM_INVALIDATE_ENG0_ADDR_RANGE_HI32, off, 0x1f); } }}

unsafe fn mmhub_v1_8_gart_enable(adev: *mut amdgpu_device) -> i32 { mmhub_v1_8_init_gart_aperture_regs(adev); mmhub_v1_8_init_system_aperture_regs(adev); mmhub_v1_8_init_tlb_regs(adev); mmhub_v1_8_init_cache_regs(adev); mmhub_v1_8_init_snoop_override_regs(adev); mmhub_v1_8_enable_system_domain(adev); mmhub_v1_8_disable_identity_aperture(adev); mmhub_v1_8_setup_vmid_config(adev); mmhub_v1_8_program_invalidation(adev); 0 }

unsafe fn mmhub_v1_8_disable_l1_tlb(adev: *mut amdgpu_device) { let mut tmp; if amdgpu_sriov_reg_indirect_l1_tlb_cntl(adev) { tmp = RREG32_SOC15!(MMHUB, 0, regMC_VM_MX_L1_TLB_CNTL); tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 0); tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 0); psp_reg_program_no_ring(&mut (*adev).psp, tmp, PSP_REG_MMHUB_L1_TLB_CNTL); } else { let inst_mask = (*adev).aid_mask; for_each_inst!(i, inst_mask) { tmp = RREG32_SOC15!(MMHUB, i, regMC_VM_MX_L1_TLB_CNTL); tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 0); tmp = REG_SET_FIELD!(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 0); WREG32_SOC15!(MMHUB, i, regMC_VM_MX_L1_TLB_CNTL, tmp); } }}

unsafe fn mmhub_v1_8_gart_disable(adev: *mut amdgpu_device) { let inst_mask = (*adev).aid_mask; for_each_inst!(j, inst_mask) { let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0!(j)]; for i in 0..16 { WREG32_SOC15_OFFSET!(MMHUB, j, regVM_CONTEXT0_CNTL, i * hub.ctx_distance, 0); } if !amdgpu_sriov_vf(adev) { let mut tmp = RREG32_SOC15!(MMHUB, j, regVM_L2_CNTL); tmp = REG_SET_FIELD!(tmp, VM_L2_CNTL, ENABLE_L2_CACHE, 0); WREG32_SOC15!(MMHUB, j, regVM_L2_CNTL, tmp); WREG32_SOC15!(MMHUB, j, regVM_L2_CNTL3, 0); }} mmhub_v1_8_disable_l1_tlb(adev); }

unsafe fn mmhub_v1_8_set_fault_enable_default(adev: *mut amdgpu_device, value: bool) { if amdgpu_sriov_vf(adev) { return; } let inst_mask = (*adev).aid_mask; for_each_inst!(i, inst_mask) { let mut tmp = RREG32_SOC15!(MMHUB, i, regVM_L2_PROTECTION_FAULT_CNTL); for field in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT, PDE0_PROTECTION_FAULT_ENABLE_DEFAULT, PDE1_PROTECTION_FAULT_ENABLE_DEFAULT, PDE2_PROTECTION_FAULT_ENABLE_DEFAULT, TRANSLATE_FURTHER_PROTECTION_FAULT_ENABLE_DEFAULT, NACK_PROTECTION_FAULT_ENABLE_DEFAULT, DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT, VALID_PROTECTION_FAULT_ENABLE_DEFAULT, READ_PROTECTION_FAULT_ENABLE_DEFAULT, WRITE_PROTECTION_FAULT_ENABLE_DEFAULT, EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT] { tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, field, value); } if !value { tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, CRASH_ON_NO_RETRY_FAULT, 1); tmp = REG_SET_FIELD!(tmp, VM_L2_PROTECTION_FAULT_CNTL, CRASH_ON_RETRY_FAULT, 1); } WREG32_SOC15!(MMHUB, i, regVM_L2_PROTECTION_FAULT_CNTL, tmp); }}

unsafe fn mmhub_v1_8_init(adev: *mut amdgpu_device) { let inst_mask = (*adev).aid_mask; for_each_inst!(i, inst_mask) { let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0!(i)]; hub.ctx0_ptb_addr_lo32 = SOC15_REG_OFFSET!(MMHUB, i, regVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32); hub.ctx0_ptb_addr_hi32 = SOC15_REG_OFFSET!(MMHUB, i, regVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32); hub.vm_inv_eng0_req = SOC15_REG_OFFSET!(MMHUB, i, regVM_INVALIDATE_ENG0_REQ); hub.vm_inv_eng0_ack = SOC15_REG_OFFSET!(MMHUB, i, regVM_INVALIDATE_ENG0_ACK); hub.vm_context0_cntl = SOC15_REG_OFFSET!(MMHUB, i, regVM_CONTEXT0_CNTL); hub.vm_l2_pro_fault_status = SOC15_REG_OFFSET!(MMHUB, i, regVM_L2_PROTECTION_FAULT_STATUS); hub.vm_l2_pro_fault_cntl = SOC15_REG_OFFSET!(MMHUB, i, regVM_L2_PROTECTION_FAULT_CNTL); hub.ctx_distance = regVM_CONTEXT1_CNTL - regVM_CONTEXT0_CNTL; hub.ctx_addr_distance = regVM_CONTEXT1_PAGE_TABLE_BASE_ADDR_LO32 - regVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32; hub.eng_distance = regVM_INVALIDATE_ENG1_REQ - regVM_INVALIDATE_ENG0_REQ; hub.eng_addr_distance = regVM_INVALIDATE_ENG1_ADDR_RANGE_LO32 - regVM_INVALIDATE_ENG0_ADDR_RANGE_LO32; }}

unsafe fn mmhub_v1_8_set_clockgating(_adev: *mut amdgpu_device, _state: amd_clockgating_state) -> i32 { 0 }
unsafe fn mmhub_v1_8_get_clockgating(_adev: *mut amdgpu_device, _flags: *mut u64) {}

const mmhub_v1_8_funcs: amdgpu_mmhub_funcs = amdgpu_mmhub_funcs { get_fb_location: Some(mmhub_v1_8_get_fb_location), init: Some(mmhub_v1_8_init), gart_enable: Some(mmhub_v1_8_gart_enable), set_fault_enable_default: Some(mmhub_v1_8_set_fault_enable_default), gart_disable: Some(mmhub_v1_8_gart_disable), setup_vm_pt_regs: Some(mmhub_v1_8_setup_vm_pt_regs), set_clockgating: Some(mmhub_v1_8_set_clockgating), get_clockgating: Some(mmhub_v1_8_get_clockgating) };
static mut mmhub_v1_8_ras: amdgpu_mmhub_ras = amdgpu_mmhub_ras { ras_block: amdgpu_ras_block { hw_ops: core::ptr::null() } };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
