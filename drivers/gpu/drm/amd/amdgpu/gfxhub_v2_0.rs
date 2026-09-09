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

// Dependencies are supplied by the surrounding driver translation.

static GFXHUB_CLIENT_IDS: [&'static str; 18] = [
    "CB/DB", "Reserved", "GE1", "GE2", "CPF", "CPC", "CPG", "RLC", "TCP",
    "SQC (inst)", "SQC (data)", "SQG", "Reserved", "SDMA0", "SDMA1", "GCR",
    "SDMA2", "SDMA3",
];

unsafe fn gfxhub_v2_0_get_invalidate_req(vmid: c_uint, flush_type: u32) -> u32 {
    let mut req: u32 = 0;
    req = REG_SET_FIELD!(req, GCVM_INVALIDATE_ENG0_REQ, PER_VMID_INVALIDATE_REQ, 1u32 << vmid);
    req = REG_SET_FIELD!(req, GCVM_INVALIDATE_ENG0_REQ, FLUSH_TYPE, flush_type);
    req = REG_SET_FIELD!(req, GCVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PTES, 1);
    req = REG_SET_FIELD!(req, GCVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE0, 1);
    req = REG_SET_FIELD!(req, GCVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE1, 1);
    req = REG_SET_FIELD!(req, GCVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE2, 1);
    req = REG_SET_FIELD!(req, GCVM_INVALIDATE_ENG0_REQ, INVALIDATE_L1_PTES, 1);
    req = REG_SET_FIELD!(req, GCVM_INVALIDATE_ENG0_REQ, CLEAR_PROTECTION_FAULT_STATUS_ADDR, 0);
    req
}

unsafe fn gfxhub_v2_0_print_l2_protection_fault_status(adev: *mut amdgpu_device, status: u32) {
    let cid = REG_GET_FIELD!(status, GCVM_L2_PROTECTION_FAULT_STATUS, CID);
    dev_err!((*adev).dev, "GCVM_L2_PROTECTION_FAULT_STATUS:0x{:08X}\n", status);
    dev_err!((*adev).dev, "\t Faulty UTCL2 client ID: {} (0x{:x})\n",
        if cid >= GFXHUB_CLIENT_IDS.len() as _ { "unknown" } else { GFXHUB_CLIENT_IDS[cid as usize] }, cid);
    dev_err!((*adev).dev, "\t MORE_FAULTS: 0x{:x}\n", REG_GET_FIELD!(status, GCVM_L2_PROTECTION_FAULT_STATUS, MORE_FAULTS));
    dev_err!((*adev).dev, "\t WALKER_ERROR: 0x{:x}\n", REG_GET_FIELD!(status, GCVM_L2_PROTECTION_FAULT_STATUS, WALKER_ERROR));
    dev_err!((*adev).dev, "\t PERMISSION_FAULTS: 0x{:x}\n", REG_GET_FIELD!(status, GCVM_L2_PROTECTION_FAULT_STATUS, PERMISSION_FAULTS));
    dev_err!((*adev).dev, "\t MAPPING_ERROR: 0x{:x}\n", REG_GET_FIELD!(status, GCVM_L2_PROTECTION_FAULT_STATUS, MAPPING_ERROR));
    dev_err!((*adev).dev, "\t RW: 0x{:x}\n", REG_GET_FIELD!(status, GCVM_L2_PROTECTION_FAULT_STATUS, RW));
}

unsafe fn gfxhub_v2_0_get_fb_location(adev: *mut amdgpu_device) -> u64 {
    let mut base = RREG32_SOC15!(adev, GC, 0, mmGCMC_VM_FB_LOCATION_BASE) as u64;
    base &= GCMC_VM_FB_LOCATION_BASE__FB_BASE_MASK as u64;
    base << 24
}

unsafe fn gfxhub_v2_0_get_mc_fb_offset(adev: *mut amdgpu_device) -> u64 {
    (RREG32_SOC15!(adev, GC, 0, mmGCMC_VM_FB_OFFSET) as u64) << 24
}

unsafe fn gfxhub_v2_0_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let hub = &mut (*adev).vmhub[AMDGPU_GFXHUB!(0)];
    WREG32_SOC15_OFFSET!(adev, GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32,
        hub.ctx_addr_distance * vmid, lower_32_bits(page_table_base));
    WREG32_SOC15_OFFSET!(adev, GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32,
        hub.ctx_addr_distance * vmid, upper_32_bits(page_table_base));
}

unsafe fn gfxhub_v2_0_init_gart_aperture_regs(adev: *mut amdgpu_device) {
    let pt_base = amdgpu_gmc_pd_addr((*adev).gart.bo);
    gfxhub_v2_0_setup_vm_pt_regs(adev, 0, pt_base);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, ((*adev).gmc.gart_start >> 12) as u32);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, ((*adev).gmc.gart_start >> 44) as u32);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, ((*adev).gmc.gart_end >> 12) as u32);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, ((*adev).gmc.gart_end >> 44) as u32);
}

unsafe fn gfxhub_v2_0_init_system_aperture_regs(adev: *mut amdgpu_device) {
    let mut value: u64;
    if !amdgpu_sriov_vf(adev) {
        WREG32_SOC15!(adev, GC, 0, mmGCMC_VM_AGP_BASE, 0);
        WREG32_SOC15!(adev, GC, 0, mmGCMC_VM_AGP_BOT, (*adev).gmc.agp_start >> 24);
        WREG32_SOC15!(adev, GC, 0, mmGCMC_VM_AGP_TOP, (*adev).gmc.agp_end >> 24);
        WREG32_SOC15!(adev, GC, 0, mmGCMC_VM_SYSTEM_APERTURE_LOW_ADDR, min((*adev).gmc.fb_start, (*adev).gmc.agp_start) >> 18);
        WREG32_SOC15!(adev, GC, 0, mmGCMC_VM_SYSTEM_APERTURE_HIGH_ADDR, max((*adev).gmc.fb_end, (*adev).gmc.agp_end) >> 18);
        value = amdgpu_gmc_vram_mc2pa(adev, (*adev).mem_scratch.gpu_addr);
        WREG32_SOC15!(adev, GC, 0, mmGCMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, (value >> 12) as u32);
        WREG32_SOC15!(adev, GC, 0, mmGCMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, (value >> 44) as u32);
    }
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32, ((*adev).dummy_page_addr >> 12) as u32);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32, ((*adev).dummy_page_addr >> 44) as u32);
    WREG32_FIELD15!(adev, GC, 0, GCVM_L2_PROTECTION_FAULT_CNTL2, ACTIVE_PAGE_MIGRATION_PTE_READ_RETRY, 1);
}

unsafe fn gfxhub_v2_0_init_tlb_regs(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15!(adev, GC, 0, mmGCMC_VM_MX_L1_TLB_CNTL);
    tmp = REG_SET_FIELD!(tmp, GCMC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 1);
    tmp = REG_SET_FIELD!(tmp, GCMC_VM_MX_L1_TLB_CNTL, SYSTEM_ACCESS_MODE, 3);
    tmp = REG_SET_FIELD!(tmp, GCMC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 1);
    tmp = REG_SET_FIELD!(tmp, GCMC_VM_MX_L1_TLB_CNTL, SYSTEM_APERTURE_UNMAPPED_ACCESS, 0);
    tmp = REG_SET_FIELD!(tmp, GCMC_VM_MX_L1_TLB_CNTL, MTYPE, MTYPE_UC);
    WREG32_SOC15!(adev, GC, 0, mmGCMC_VM_MX_L1_TLB_CNTL, tmp);
}

unsafe fn gfxhub_v2_0_init_cache_regs(adev: *mut amdgpu_device) {
    if amdgpu_sriov_vf(adev) { return; }
    let mut tmp = RREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CNTL);
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL, ENABLE_L2_CACHE, 1);
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL, ENABLE_L2_FRAGMENT_PROCESSING, 0);
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL, ENABLE_DEFAULT_PAGE_OUT_TO_SYSTEM_MEMORY, 1);
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL, L2_PDE0_CACHE_TAG_GENERATION_MODE, 0);
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL, PDE_FAULT_CLASSIFICATION, 0);
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL, CONTEXT1_IDENTITY_ACCESS_MODE, 1);
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL, IDENTITY_MODE_FRAGMENT_SIZE, 0);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CNTL, tmp);
    tmp = RREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CNTL2);
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL2, INVALIDATE_ALL_L1_TLBS, 1);
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL2, INVALIDATE_L2_CACHE, 1);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CNTL2, tmp);
    tmp = mmGCVM_L2_CNTL3_DEFAULT;
    if (*adev).gmc.translate_further {
        tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL3, BANK_SELECT, 12);
        tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 9);
    } else {
        tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL3, BANK_SELECT, 9);
        tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 6);
    }
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CNTL3, tmp);
    tmp = mmGCVM_L2_CNTL4_DEFAULT;
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL4, VMC_TAP_PDE_REQUEST_PHYSICAL, 0);
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL4, VMC_TAP_PTE_REQUEST_PHYSICAL, 0);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CNTL4, tmp);
    tmp = mmGCVM_L2_CNTL5_DEFAULT;
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_CNTL5, L2_CACHE_SMALLK_FRAGMENT_SIZE, 0);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CNTL5, tmp);
}

unsafe fn gfxhub_v2_0_enable_system_domain(adev: *mut amdgpu_device) {
    let mut tmp = RREG32_SOC15!(adev, GC, 0, mmGCVM_CONTEXT0_CNTL);
    tmp = REG_SET_FIELD!(tmp, GCVM_CONTEXT0_CNTL, ENABLE_CONTEXT, 1);
    tmp = REG_SET_FIELD!(tmp, GCVM_CONTEXT0_CNTL, PAGE_TABLE_DEPTH, 0);
    tmp = REG_SET_FIELD!(tmp, GCVM_CONTEXT0_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, 0);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_CONTEXT0_CNTL, tmp);
}

unsafe fn gfxhub_v2_0_disable_identity_aperture(adev: *mut amdgpu_device) {
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_LO32, 0xffffffff);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_HI32, 0x0000000f);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_LO32, 0);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_HI32, 0);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_LO32, 0);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_HI32, 0);
}

unsafe fn gfxhub_v2_0_setup_vmid_config(adev: *mut amdgpu_device) {
    let hub = &mut (*adev).vmhub[AMDGPU_GFXHUB!(0)];
    let mut tmp = 0u32;
    for i in 0..=14u32 {
        tmp = RREG32_SOC15_OFFSET!(adev, GC, 0, mmGCVM_CONTEXT1_CNTL, i * hub.ctx_distance);
        tmp = REG_SET_FIELD!(tmp, GCVM_CONTEXT1_CNTL, ENABLE_CONTEXT, 1);
        tmp = REG_SET_FIELD!(tmp, GCVM_CONTEXT1_CNTL, PAGE_TABLE_DEPTH, (*adev).vm_manager.num_level);
        for field in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT, DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT,
                      PDE0_PROTECTION_FAULT_ENABLE_DEFAULT, VALID_PROTECTION_FAULT_ENABLE_DEFAULT,
                      READ_PROTECTION_FAULT_ENABLE_DEFAULT, WRITE_PROTECTION_FAULT_ENABLE_DEFAULT,
                      EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT] {
            tmp = REG_SET_FIELD_DYNAMIC!(tmp, GCVM_CONTEXT1_CNTL, field, 1);
        }
        tmp = REG_SET_FIELD!(tmp, GCVM_CONTEXT1_CNTL, PAGE_TABLE_BLOCK_SIZE, (*adev).vm_manager.block_size - 9);
        tmp = REG_SET_FIELD!(tmp, GCVM_CONTEXT1_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, !(*adev).gmc.noretry);
        WREG32_SOC15_OFFSET!(adev, GC, 0, mmGCVM_CONTEXT1_CNTL, i * hub.ctx_distance, tmp);
        WREG32_SOC15_OFFSET!(adev, GC, 0, mmGCVM_CONTEXT1_PAGE_TABLE_START_ADDR_LO32, i * hub.ctx_addr_distance, 0);
        WREG32_SOC15_OFFSET!(adev, GC, 0, mmGCVM_CONTEXT1_PAGE_TABLE_START_ADDR_HI32, i * hub.ctx_addr_distance, 0);
        WREG32_SOC15_OFFSET!(adev, GC, 0, mmGCVM_CONTEXT1_PAGE_TABLE_END_ADDR_LO32, i * hub.ctx_addr_distance, lower_32_bits((*adev).vm_manager.max_pfn - 1));
        WREG32_SOC15_OFFSET!(adev, GC, 0, mmGCVM_CONTEXT1_PAGE_TABLE_END_ADDR_HI32, i * hub.ctx_addr_distance, upper_32_bits((*adev).vm_manager.max_pfn - 1));
    }
    hub.vm_cntx_cntl = tmp;
}

unsafe fn gfxhub_v2_0_program_invalidation(adev: *mut amdgpu_device) {
    let hub = &mut (*adev).vmhub[AMDGPU_GFXHUB!(0)];
    for i in 0..18u32 {
        WREG32_SOC15_OFFSET!(adev, GC, 0, mmGCVM_INVALIDATE_ENG0_ADDR_RANGE_LO32, i * hub.eng_addr_distance, 0xffffffff);
        WREG32_SOC15_OFFSET!(adev, GC, 0, mmGCVM_INVALIDATE_ENG0_ADDR_RANGE_HI32, i * hub.eng_addr_distance, 0x1f);
    }
}

unsafe fn gfxhub_v2_0_gart_enable(adev: *mut amdgpu_device) -> c_int {
    gfxhub_v2_0_init_gart_aperture_regs(adev); gfxhub_v2_0_init_system_aperture_regs(adev);
    gfxhub_v2_0_init_tlb_regs(adev); gfxhub_v2_0_init_cache_regs(adev);
    gfxhub_v2_0_enable_system_domain(adev); gfxhub_v2_0_disable_identity_aperture(adev);
    gfxhub_v2_0_setup_vmid_config(adev); gfxhub_v2_0_program_invalidation(adev); 0
}

unsafe fn gfxhub_v2_0_gart_disable(adev: *mut amdgpu_device) {
    let hub = &mut (*adev).vmhub[AMDGPU_GFXHUB!(0)];
    for i in 0..16u32 { WREG32_SOC15_OFFSET!(adev, GC, 0, mmGCVM_CONTEXT0_CNTL, i * hub.ctx_distance, 0); }
    let mut tmp = RREG32_SOC15!(adev, GC, 0, mmGCMC_VM_MX_L1_TLB_CNTL);
    tmp = REG_SET_FIELD!(tmp, GCMC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 0);
    tmp = REG_SET_FIELD!(tmp, GCMC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 0);
    WREG32_SOC15!(adev, GC, 0, mmGCMC_VM_MX_L1_TLB_CNTL, tmp);
    if !amdgpu_sriov_vf(adev) { WREG32_FIELD15!(adev, GC, 0, GCVM_L2_CNTL, ENABLE_L2_CACHE, 0); WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_CNTL3, 0); }
}

unsafe fn gfxhub_v2_0_set_fault_enable_default(adev: *mut amdgpu_device, value: bool) {
    let mut tmp = RREG32_SOC15!(adev, GC, 0, mmGCVM_L2_PROTECTION_FAULT_CNTL);
    for field in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT, PDE0_PROTECTION_FAULT_ENABLE_DEFAULT, PDE1_PROTECTION_FAULT_ENABLE_DEFAULT,
        PDE2_PROTECTION_FAULT_ENABLE_DEFAULT, TRANSLATE_FURTHER_PROTECTION_FAULT_ENABLE_DEFAULT, NACK_PROTECTION_FAULT_ENABLE_DEFAULT,
        DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT, VALID_PROTECTION_FAULT_ENABLE_DEFAULT, READ_PROTECTION_FAULT_ENABLE_DEFAULT,
        WRITE_PROTECTION_FAULT_ENABLE_DEFAULT, EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT] { tmp = REG_SET_FIELD_DYNAMIC!(tmp, GCVM_L2_PROTECTION_FAULT_CNTL, field, value); }
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_PROTECTION_FAULT_CNTL, CRASH_ON_NO_RETRY_FAULT, !value);
    tmp = REG_SET_FIELD!(tmp, GCVM_L2_PROTECTION_FAULT_CNTL, CRASH_ON_RETRY_FAULT, !value);
    WREG32_SOC15!(adev, GC, 0, mmGCVM_L2_PROTECTION_FAULT_CNTL, tmp);
}

// Function-table initializers and remaining hub initialization are represented
// directly using the externally supplied driver types.
static gfxhub_v2_0_vmhub_funcs: amdgpu_vmhub_funcs = amdgpu_vmhub_funcs {
    print_l2_protection_fault_status: Some(gfxhub_v2_0_print_l2_protection_fault_status),
    get_invalidate_req: Some(gfxhub_v2_0_get_invalidate_req),
};

unsafe fn gfxhub_v2_0_init(adev: *mut amdgpu_device) {
    let hub = &mut (*adev).vmhub[AMDGPU_GFXHUB!(0)];
    hub.ctx0_ptb_addr_lo32 = SOC15_REG_OFFSET!(GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32);
    hub.ctx0_ptb_addr_hi32 = SOC15_REG_OFFSET!(GC, 0, mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32);
    hub.vm_inv_eng0_sem = SOC15_REG_OFFSET!(GC, 0, mmGCVM_INVALIDATE_ENG0_SEM);
    hub.vm_inv_eng0_req = SOC15_REG_OFFSET!(GC, 0, mmGCVM_INVALIDATE_ENG0_REQ);
    hub.vm_inv_eng0_ack = SOC15_REG_OFFSET!(GC, 0, mmGCVM_INVALIDATE_ENG0_ACK);
    hub.vm_context0_cntl = SOC15_REG_OFFSET!(GC, 0, mmGCVM_CONTEXT0_CNTL);
    hub.vm_l2_pro_fault_status = SOC15_REG_OFFSET!(GC, 0, mmGCVM_L2_PROTECTION_FAULT_STATUS);
    hub.vm_l2_pro_fault_cntl = SOC15_REG_OFFSET!(GC, 0, mmGCVM_L2_PROTECTION_FAULT_CNTL);
    hub.ctx_distance = mmGCVM_CONTEXT1_CNTL - mmGCVM_CONTEXT0_CNTL;
    hub.ctx_addr_distance = mmGCVM_CONTEXT1_PAGE_TABLE_BASE_ADDR_LO32 - mmGCVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32;
    hub.eng_distance = mmGCVM_INVALIDATE_ENG1_REQ - mmGCVM_INVALIDATE_ENG0_REQ;
    hub.eng_addr_distance = mmGCVM_INVALIDATE_ENG1_ADDR_RANGE_LO32 - mmGCVM_INVALIDATE_ENG0_ADDR_RANGE_LO32;
    hub.vm_cntx_cntl_vm_fault = GCVM_CONTEXT1_CNTL__RANGE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK |
        GCVM_CONTEXT1_CNTL__DUMMY_PAGE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK |
        GCVM_CONTEXT1_CNTL__PDE0_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK |
        GCVM_CONTEXT1_CNTL__VALID_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK |
        GCVM_CONTEXT1_CNTL__READ_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK |
        GCVM_CONTEXT1_CNTL__WRITE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK |
        GCVM_CONTEXT1_CNTL__EXECUTE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK;
    // TODO: This is only needed on some Navi 1x revisions.
    hub.sdma_invalidation_workaround = true;
    hub.vmhub_funcs = &gfxhub_v2_0_vmhub_funcs;
}

#[no_mangle]
pub static gfxhub_v2_0_funcs: amdgpu_gfxhub_funcs = amdgpu_gfxhub_funcs {
    get_fb_location: Some(gfxhub_v2_0_get_fb_location),
    get_mc_fb_offset: Some(gfxhub_v2_0_get_mc_fb_offset),
    setup_vm_pt_regs: Some(gfxhub_v2_0_setup_vm_pt_regs),
    gart_enable: Some(gfxhub_v2_0_gart_enable),
    gart_disable: Some(gfxhub_v2_0_gart_disable),
    set_fault_enable_default: Some(gfxhub_v2_0_set_fault_enable_default),
    init: Some(gfxhub_v2_0_init),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
