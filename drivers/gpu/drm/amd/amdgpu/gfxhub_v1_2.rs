/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding kernel/amdgpu translation.

const REG_VM_L2_CNTL3_DEFAULT: u32 = 0x80100007;
const REG_VM_L2_CNTL4_DEFAULT: u32 = 0x000000c1;

unsafe fn gfxhub_v1_2_get_mc_fb_offset(adev: *mut amdgpu_device) -> u64 {
    (RREG32_SOC15(GC, GET_INST(GC, 0), regMC_VM_FB_OFFSET) as u64) << 24
}

unsafe fn gfxhub_v1_2_xcc_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64, xcc_mask: u32) {
    let hub: *mut amdgpu_vmhub;
    for_each_inst!(i, xcc_mask, {
        hub = &mut (*adev).vmhub[AMDGPU_GFXHUB(i)];
        WREG32_SOC15_OFFSET(GC, GET_INST(GC, i), regVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32,
            (*hub).ctx_addr_distance * vmid, lower_32_bits(page_table_base));
        WREG32_SOC15_OFFSET(GC, GET_INST(GC, i), regVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32,
            (*hub).ctx_addr_distance * vmid, upper_32_bits(page_table_base));
    });
}

unsafe fn gfxhub_v1_2_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let xcc_mask = GENMASK(NUM_XCC((*adev).gfx.xcc_mask) - 1, 0);
    gfxhub_v1_2_xcc_setup_vm_pt_regs(adev, vmid, page_table_base, xcc_mask);
}

unsafe fn gfxhub_v1_2_xcc_init_gart_aperture_regs(adev: *mut amdgpu_device, xcc_mask: u32) {
    let gart_start = if amdgpu_virt_xgmi_migrate_enabled(adev) { (*adev).gmc.vram_start } else { (*adev).gmc.fb_start };
    let pt_base = if !(*adev).gmc.pdb0_bo.is_null() { amdgpu_gmc_pd_addr((*adev).gmc.pdb0_bo) } else { amdgpu_gmc_pd_addr((*adev).gart.bo) };
    gfxhub_v1_2_xcc_setup_vm_pt_regs(adev, 0, pt_base, xcc_mask);
    for_each_inst!(i, xcc_mask, {
        let start = if !(*adev).gmc.pdb0_bo.is_null() { gart_start } else { (*adev).gmc.gart_start };
        WREG32_SOC15(GC, GET_INST(GC, i), regVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, (start >> 12) as u32);
        WREG32_SOC15(GC, GET_INST(GC, i), regVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, (start >> 44) as u32);
        WREG32_SOC15(GC, GET_INST(GC, i), regVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, ((*adev).gmc.gart_end >> 12) as u32);
        WREG32_SOC15(GC, GET_INST(GC, i), regVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, ((*adev).gmc.gart_end >> 44) as u32);
    });
}

unsafe fn gfxhub_v1_2_xcc_init_system_aperture_regs(adev: *mut amdgpu_device, xcc_mask: u32) {
    for_each_inst!(i, xcc_mask, {
        WREG32_SOC15_RLC(GC, GET_INST(GC, i), regMC_VM_AGP_BASE, 0);
        WREG32_SOC15_RLC(GC, GET_INST(GC, i), regMC_VM_AGP_BOT, (*adev).gmc.agp_start >> 24);
        WREG32_SOC15_RLC(GC, GET_INST(GC, i), regMC_VM_AGP_TOP, (*adev).gmc.agp_end >> 24);
        if !amdgpu_sriov_vf(adev) || (*adev).asic_type <= CHIP_VEGA10 {
            WREG32_SOC15_RLC(GC, GET_INST(GC, i), regMC_VM_SYSTEM_APERTURE_LOW_ADDR,
                min((*adev).gmc.fb_start, (*adev).gmc.agp_start) >> 18);
            let high = if (*adev).apu_flags & (AMD_APU_IS_RAVEN2 | AMD_APU_IS_RENOIR | AMD_APU_IS_GREEN_SARDINE) != 0 {
                max(((*adev).gmc.fb_end >> 18) + 1, (*adev).gmc.agp_end >> 18)
            } else { max((*adev).gmc.fb_end, (*adev).gmc.agp_end) >> 18 };
            WREG32_SOC15_RLC(GC, GET_INST(GC, i), regMC_VM_SYSTEM_APERTURE_HIGH_ADDR, high);
            let value = amdgpu_gmc_vram_mc2pa(adev, (*adev).mem_scratch.gpu_addr);
            WREG32_SOC15(GC, GET_INST(GC, i), regMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, (value >> 12) as u32);
            WREG32_SOC15(GC, GET_INST(GC, i), regMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, (value >> 44) as u32);
            WREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32, ((*adev).dummy_page_addr >> 12) as u32);
            WREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32, ((*adev).dummy_page_addr >> 44) as u32);
            let mut tmp = RREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_PROTECTION_FAULT_CNTL2);
            tmp = REG_SET_FIELD(tmp, VM_L2_PROTECTION_FAULT_CNTL2, ACTIVE_PAGE_MIGRATION_PTE_READ_RETRY, 1);
            WREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_PROTECTION_FAULT_CNTL2, tmp);
        }
        if !(*adev).gmc.pdb0_bo.is_null() && (*adev).gmc.xgmi.connected_to_cpu {
            WREG32_SOC15(GC, GET_INST(GC, i), regMC_VM_FB_LOCATION_TOP, 0);
            WREG32_SOC15(GC, GET_INST(GC, i), regMC_VM_FB_LOCATION_BASE, 0x00ffffff);
            WREG32_SOC15(GC, GET_INST(GC, i), regMC_VM_AGP_TOP, 0);
            WREG32_SOC15(GC, GET_INST(GC, i), regMC_VM_AGP_BOT, 0xffffff);
            WREG32_SOC15(GC, GET_INST(GC, i), regMC_VM_SYSTEM_APERTURE_LOW_ADDR, 0x3fffffff);
            WREG32_SOC15(GC, GET_INST(GC, i), regMC_VM_SYSTEM_APERTURE_HIGH_ADDR, 0);
        }
    });
}

unsafe fn gfxhub_v1_2_xcc_init_tlb_regs(adev: *mut amdgpu_device, xcc_mask: u32) {
    for_each_inst!(i, xcc_mask, {
        let mut tmp = RREG32_SOC15(GC, GET_INST(GC, i), regMC_VM_MX_L1_TLB_CNTL);
        tmp = REG_SET_FIELD(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 1);
        tmp = REG_SET_FIELD(tmp, MC_VM_MX_L1_TLB_CNTL, SYSTEM_ACCESS_MODE, 3);
        tmp = REG_SET_FIELD(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 1);
        tmp = REG_SET_FIELD(tmp, MC_VM_MX_L1_TLB_CNTL, SYSTEM_APERTURE_UNMAPPED_ACCESS, 0);
        tmp = REG_SET_FIELD(tmp, MC_VM_MX_L1_TLB_CNTL, MTYPE, MTYPE_UC);
        tmp = REG_SET_FIELD(tmp, MC_VM_MX_L1_TLB_CNTL, ATC_EN, 1);
        WREG32_SOC15_RLC(GC, GET_INST(GC, i), regMC_VM_MX_L1_TLB_CNTL, tmp);
    });
}

unsafe fn gfxhub_v1_2_xcc_init_cache_regs(adev: *mut amdgpu_device, xcc_mask: u32) {
    for_each_inst!(i, xcc_mask, {
        let mut tmp = RREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_CNTL);
        tmp = REG_SET_FIELD(tmp, VM_L2_CNTL, ENABLE_L2_CACHE, 1);
        tmp = REG_SET_FIELD(tmp, VM_L2_CNTL, ENABLE_L2_FRAGMENT_PROCESSING, 1);
        tmp = REG_SET_FIELD(tmp, VM_L2_CNTL, L2_PDE0_CACHE_TAG_GENERATION_MODE, 0);
        tmp = REG_SET_FIELD(tmp, VM_L2_CNTL, PDE_FAULT_CLASSIFICATION, 0);
        tmp = REG_SET_FIELD(tmp, VM_L2_CNTL, CONTEXT1_IDENTITY_ACCESS_MODE, 1);
        tmp = REG_SET_FIELD(tmp, VM_L2_CNTL, IDENTITY_MODE_FRAGMENT_SIZE, 0);
        WREG32_SOC15_RLC(GC, GET_INST(GC, i), regVM_L2_CNTL, tmp);
        tmp = RREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_CNTL2);
        tmp = REG_SET_FIELD(tmp, VM_L2_CNTL2, INVALIDATE_ALL_L1_TLBS, 1);
        tmp = REG_SET_FIELD(tmp, VM_L2_CNTL2, INVALIDATE_L2_CACHE, 1);
        WREG32_SOC15_RLC(GC, GET_INST(GC, i), regVM_L2_CNTL2, tmp);
        tmp = REG_VM_L2_CNTL3_DEFAULT;
        let (bank, frag) = if (*adev).gmc.translate_further { (12, 9) } else { (9, 6) };
        tmp = REG_SET_FIELD(tmp, VM_L2_CNTL3, BANK_SELECT, bank);
        tmp = REG_SET_FIELD(tmp, VM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, frag);
        WREG32_SOC15_RLC(GC, GET_INST(GC, i), regVM_L2_CNTL3, tmp);
        tmp = REG_VM_L2_CNTL4_DEFAULT;
        let physical = (*adev).gmc.xgmi.connected_to_cpu || (*adev).gmc.is_app_apu;
        tmp = REG_SET_FIELD(tmp, VM_L2_CNTL4, VMC_TAP_PDE_REQUEST_PHYSICAL, physical as u32);
        tmp = REG_SET_FIELD(tmp, VM_L2_CNTL4, VMC_TAP_PTE_REQUEST_PHYSICAL, physical as u32);
        WREG32_SOC15_RLC(GC, GET_INST(GC, i), regVM_L2_CNTL4, tmp);
    });
}

unsafe fn gfxhub_v1_2_xcc_enable_system_domain(adev: *mut amdgpu_device, xcc_mask: u32) {
    for_each_inst!(i, xcc_mask, {
        let mut tmp = RREG32_SOC15(GC, GET_INST(GC, i), regVM_CONTEXT0_CNTL);
        tmp = REG_SET_FIELD(tmp, VM_CONTEXT0_CNTL, ENABLE_CONTEXT, 1);
        tmp = REG_SET_FIELD(tmp, VM_CONTEXT0_CNTL, PAGE_TABLE_DEPTH, (*adev).gmc.vmid0_page_table_depth);
        tmp = REG_SET_FIELD(tmp, VM_CONTEXT0_CNTL, PAGE_TABLE_BLOCK_SIZE, (*adev).gmc.vmid0_page_table_block_size);
        tmp = REG_SET_FIELD(tmp, VM_CONTEXT0_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, 0);
        WREG32_SOC15(GC, GET_INST(GC, i), regVM_CONTEXT0_CNTL, tmp);
    });
}

unsafe fn gfxhub_v1_2_xcc_disable_identity_aperture(adev: *mut amdgpu_device, xcc_mask: u32) {
    for_each_inst!(i, xcc_mask, {
        WREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_LO32, 0xffffffff);
        WREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_HI32, 0x0000000f);
        WREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_LO32, 0);
        WREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_HI32, 0);
        WREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_LO32, 0);
        WREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_HI32, 0);
    });
}

#[inline]
unsafe fn gfxhub_v1_2_per_process_xnack_support(adev: *mut amdgpu_device) -> bool {
    // TODO: Check if this function is really needed; only 9.4.3 variants use GFXHUB 1.2.
    (*adev).aid_mask != 0
}

unsafe fn gfxhub_v1_2_xcc_setup_vmid_config(adev: *mut amdgpu_device, xcc_mask: u32) {
    let mut num_level = (*adev).vm_manager.num_level;
    let mut block_size = (*adev).vm_manager.block_size;
    if (*adev).gmc.translate_further { num_level -= 1; } else { block_size -= 9; }
    for_each_inst!(j, xcc_mask, {
        let hub = &mut (*adev).vmhub[AMDGPU_GFXHUB(j)];
        for i in 0..=14 {
            let off = i * hub.ctx_distance;
            let mut tmp = RREG32_SOC15_OFFSET(GC, GET_INST(GC, j), regVM_CONTEXT1_CNTL, off);
            tmp = REG_SET_FIELD(tmp, VM_CONTEXT1_CNTL, ENABLE_CONTEXT, 1);
            tmp = REG_SET_FIELD(tmp, VM_CONTEXT1_CNTL, PAGE_TABLE_DEPTH, num_level);
            for field in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT, DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT, PDE0_PROTECTION_FAULT_ENABLE_DEFAULT, VALID_PROTECTION_FAULT_ENABLE_DEFAULT, READ_PROTECTION_FAULT_ENABLE_DEFAULT, WRITE_PROTECTION_FAULT_ENABLE_DEFAULT, EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT] { tmp = REG_SET_FIELD(tmp, VM_CONTEXT1_CNTL, field, 1); }
            tmp = REG_SET_FIELD(tmp, VM_CONTEXT1_CNTL, PAGE_TABLE_BLOCK_SIZE, block_size);
            let retry = !(*adev).gmc.noretry || gfxhub_v1_2_per_process_xnack_support(adev);
            tmp = REG_SET_FIELD(tmp, VM_CONTEXT1_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, retry as u32);
            WREG32_SOC15_OFFSET(GC, GET_INST(GC, j), regVM_CONTEXT1_CNTL, off, tmp);
            let aoff = i * hub.ctx_addr_distance;
            WREG32_SOC15_OFFSET(GC, GET_INST(GC, j), regVM_CONTEXT1_PAGE_TABLE_START_ADDR_LO32, aoff, 0);
            WREG32_SOC15_OFFSET(GC, GET_INST(GC, j), regVM_CONTEXT1_PAGE_TABLE_START_ADDR_HI32, aoff, 0);
            WREG32_SOC15_OFFSET(GC, GET_INST(GC, j), regVM_CONTEXT1_PAGE_TABLE_END_ADDR_LO32, aoff, lower_32_bits((*adev).vm_manager.max_pfn - 1));
            WREG32_SOC15_OFFSET(GC, GET_INST(GC, j), regVM_CONTEXT1_PAGE_TABLE_END_ADDR_HI32, aoff, upper_32_bits((*adev).vm_manager.max_pfn - 1));
        }
    });
}

unsafe fn gfxhub_v1_2_xcc_program_invalidation(adev: *mut amdgpu_device, xcc_mask: u32) {
    for_each_inst!(j, xcc_mask, {
        let hub = &(*adev).vmhub[AMDGPU_GFXHUB(j)];
        for i in 0..18 { let off = i * hub.eng_addr_distance;
            WREG32_SOC15_OFFSET(GC, GET_INST(GC, j), regVM_INVALIDATE_ENG0_ADDR_RANGE_LO32, off, 0xffffffff);
            WREG32_SOC15_OFFSET(GC, GET_INST(GC, j), regVM_INVALIDATE_ENG0_ADDR_RANGE_HI32, off, 0x1f);
        }
    });
}

unsafe fn gfxhub_v1_2_xcc_gart_enable(adev: *mut amdgpu_device, xcc_mask: u32) -> i32 {
    gfxhub_v1_2_xcc_init_gart_aperture_regs(adev, xcc_mask);
    gfxhub_v1_2_xcc_init_system_aperture_regs(adev, xcc_mask);
    gfxhub_v1_2_xcc_init_tlb_regs(adev, xcc_mask);
    if !amdgpu_sriov_vf(adev) { gfxhub_v1_2_xcc_init_cache_regs(adev, xcc_mask); }
    gfxhub_v1_2_xcc_enable_system_domain(adev, xcc_mask);
    if !amdgpu_sriov_vf(adev) { gfxhub_v1_2_xcc_disable_identity_aperture(adev, xcc_mask); }
    gfxhub_v1_2_xcc_setup_vmid_config(adev, xcc_mask);
    gfxhub_v1_2_xcc_program_invalidation(adev, xcc_mask);
    0
}

unsafe fn gfxhub_v1_2_gart_enable(adev: *mut amdgpu_device) -> i32 {
    gfxhub_v1_2_xcc_gart_enable(adev, GENMASK(NUM_XCC((*adev).gfx.xcc_mask) - 1, 0))
}

unsafe fn gfxhub_v1_2_xcc_gart_disable(adev: *mut amdgpu_device, xcc_mask: u32) {
    for_each_inst!(j, xcc_mask, {
        let hub = &(*adev).vmhub[AMDGPU_GFXHUB(j)];
        for i in 0..16 { WREG32_SOC15_OFFSET(GC, GET_INST(GC, j), regVM_CONTEXT0_CNTL, i * hub.ctx_distance, 0); }
        let mut tmp = RREG32_SOC15(GC, GET_INST(GC, j), regMC_VM_MX_L1_TLB_CNTL);
        tmp = REG_SET_FIELD(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 0);
        tmp = REG_SET_FIELD(tmp, MC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 0);
        WREG32_SOC15_RLC(GC, GET_INST(GC, j), regMC_VM_MX_L1_TLB_CNTL, tmp);
        if !amdgpu_sriov_vf(adev) {
            tmp = RREG32_SOC15(GC, GET_INST(GC, j), regVM_L2_CNTL);
            tmp = REG_SET_FIELD(tmp, VM_L2_CNTL, ENABLE_L2_CACHE, 0);
            WREG32_SOC15(GC, GET_INST(GC, j), regVM_L2_CNTL, tmp);
            WREG32_SOC15(GC, GET_INST(GC, j), regVM_L2_CNTL3, 0);
        }
    });
}

unsafe fn gfxhub_v1_2_gart_disable(adev: *mut amdgpu_device) {
    gfxhub_v1_2_xcc_gart_disable(adev, GENMASK(NUM_XCC((*adev).gfx.xcc_mask) - 1, 0));
}

unsafe fn gfxhub_v1_2_xcc_set_fault_enable_default(adev: *mut amdgpu_device, value: bool, xcc_mask: u32) {
    for_each_inst!(i, xcc_mask, {
        let mut tmp = RREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_PROTECTION_FAULT_CNTL);
        for field in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT, PDE0_PROTECTION_FAULT_ENABLE_DEFAULT, PDE1_PROTECTION_FAULT_ENABLE_DEFAULT, PDE2_PROTECTION_FAULT_ENABLE_DEFAULT, TRANSLATE_FURTHER_PROTECTION_FAULT_ENABLE_DEFAULT, NACK_PROTECTION_FAULT_ENABLE_DEFAULT, DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT, VALID_PROTECTION_FAULT_ENABLE_DEFAULT, READ_PROTECTION_FAULT_ENABLE_DEFAULT, WRITE_PROTECTION_FAULT_ENABLE_DEFAULT, EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT] { tmp = REG_SET_FIELD(tmp, VM_L2_PROTECTION_FAULT_CNTL, field, value as u32); }
        tmp = REG_SET_FIELD(tmp, VM_L2_PROTECTION_FAULT_CNTL, CRASH_ON_NO_RETRY_FAULT, (!value) as u32);
        tmp = REG_SET_FIELD(tmp, VM_L2_PROTECTION_FAULT_CNTL, CRASH_ON_RETRY_FAULT, (!value) as u32);
        WREG32_SOC15(GC, GET_INST(GC, i), regVM_L2_PROTECTION_FAULT_CNTL, tmp);
    });
}

unsafe fn gfxhub_v1_2_set_fault_enable_default(adev: *mut amdgpu_device, value: bool) {
    gfxhub_v1_2_xcc_set_fault_enable_default(adev, value, GENMASK(NUM_XCC((*adev).gfx.xcc_mask) - 1, 0));
}

unsafe fn gfxhub_v1_2_xcc_init(adev: *mut amdgpu_device, xcc_mask: u32) {
    for_each_inst!(i, xcc_mask, {
        let hub = &mut (*adev).vmhub[AMDGPU_GFXHUB(i)];
        hub.ctx0_ptb_addr_lo32 = SOC15_REG_OFFSET(GC, GET_INST(GC, i), regVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32);
        hub.ctx0_ptb_addr_hi32 = SOC15_REG_OFFSET(GC, GET_INST(GC, i), regVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32);
        hub.vm_inv_eng0_sem = SOC15_REG_OFFSET(GC, GET_INST(GC, i), regVM_INVALIDATE_ENG0_SEM);
        hub.vm_inv_eng0_req = SOC15_REG_OFFSET(GC, GET_INST(GC, i), regVM_INVALIDATE_ENG0_REQ);
        hub.vm_inv_eng0_ack = SOC15_REG_OFFSET(GC, GET_INST(GC, i), regVM_INVALIDATE_ENG0_ACK);
        hub.vm_context0_cntl = SOC15_REG_OFFSET(GC, GET_INST(GC, i), regVM_CONTEXT0_CNTL);
        hub.vm_l2_pro_fault_status = SOC15_REG_OFFSET(GC, GET_INST(GC, i), regVM_L2_PROTECTION_FAULT_STATUS);
        hub.vm_l2_pro_fault_cntl = SOC15_REG_OFFSET(GC, GET_INST(GC, i), regVM_L2_PROTECTION_FAULT_CNTL);
        hub.ctx_distance = regVM_CONTEXT1_CNTL - regVM_CONTEXT0_CNTL;
        hub.ctx_addr_distance = regVM_CONTEXT1_PAGE_TABLE_BASE_ADDR_LO32 - regVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32;
        hub.eng_distance = regVM_INVALIDATE_ENG1_REQ - regVM_INVALIDATE_ENG0_REQ;
        hub.eng_addr_distance = regVM_INVALIDATE_ENG1_ADDR_RANGE_LO32 - regVM_INVALIDATE_ENG0_ADDR_RANGE_LO32;
    });
}

unsafe fn gfxhub_v1_2_init(adev: *mut amdgpu_device) {
    gfxhub_v1_2_xcc_init(adev, GENMASK(NUM_XCC((*adev).gfx.xcc_mask) - 1, 0));
}

unsafe fn gfxhub_v1_2_get_xgmi_info(adev: *mut amdgpu_device) -> i32 {
    let xgmi_lfb_cntl = RREG32_SOC15(GC, GET_INST(GC, 0), regMC_VM_XGMI_LFB_CNTL);
    let seg_size = (REG_GET_FIELD(RREG32_SOC15(GC, GET_INST(GC, 0), regMC_VM_XGMI_LFB_SIZE), MC_VM_XGMI_LFB_SIZE, PF_LFB_SIZE) as u64) << 24;
    let max_region = REG_GET_FIELD(xgmi_lfb_cntl, MC_VM_XGMI_LFB_CNTL, PF_MAX_REGION);
    if max_region != 0 || (*adev).gmc.xgmi.connected_to_cpu {
        (*adev).gmc.xgmi.num_physical_nodes = max_region + 1;
        if (*adev).gmc.xgmi.num_physical_nodes > 8 { return -EINVAL; }
        (*adev).gmc.xgmi.physical_node_id = REG_GET_FIELD(xgmi_lfb_cntl, MC_VM_XGMI_LFB_CNTL, PF_LFB_REGION);
        if (*adev).gmc.xgmi.physical_node_id > 7 { return -EINVAL; }
        (*adev).gmc.xgmi.node_segment_size = seg_size;
    }
    0
}

// Function-table initializer and XCP callbacks retain the C ABI shape.
pub static gfxhub_v1_2_funcs: amdgpu_gfxhub_funcs = amdgpu_gfxhub_funcs {
    get_mc_fb_offset: Some(gfxhub_v1_2_get_mc_fb_offset), setup_vm_pt_regs: Some(gfxhub_v1_2_setup_vm_pt_regs),
    gart_enable: Some(gfxhub_v1_2_gart_enable), gart_disable: Some(gfxhub_v1_2_gart_disable),
    set_fault_enable_default: Some(gfxhub_v1_2_set_fault_enable_default), init: Some(gfxhub_v1_2_init),
    get_xgmi_info: Some(gfxhub_v1_2_get_xgmi_info),
};

unsafe fn gfxhub_v1_2_xcp_resume(handle: *mut core::ffi::c_void, inst_mask: u32) -> i32 {
    let adev = handle as *mut amdgpu_device;
    let value = if amdgpu_vm_fault_stop == AMDGPU_VM_FAULT_STOP_ALWAYS { false } else { true };
    gfxhub_v1_2_xcc_set_fault_enable_default(adev, value, inst_mask);
    if !amdgpu_sriov_vf(adev) { return gfxhub_v1_2_xcc_gart_enable(adev, inst_mask); }
    0
}

unsafe fn gfxhub_v1_2_xcp_suspend(handle: *mut core::ffi::c_void, inst_mask: u32) -> i32 {
    let adev = handle as *mut amdgpu_device;
    if !amdgpu_sriov_vf(adev) { gfxhub_v1_2_xcc_gart_disable(adev, inst_mask); }
    0
}

pub static mut gfxhub_v1_2_xcp_funcs: amdgpu_xcp_ip_funcs = amdgpu_xcp_ip_funcs {
    suspend: Some(gfxhub_v1_2_xcp_suspend), resume: Some(gfxhub_v1_2_xcp_resume),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
