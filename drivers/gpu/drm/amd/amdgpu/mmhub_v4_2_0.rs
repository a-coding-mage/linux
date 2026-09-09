/* Rust translation of mmhub_v4_2_0.c. */

const regMMVM_L2_CNTL3_DEFAULT: u32 = 0x80100007;
const regMMVM_L2_CNTL4_DEFAULT: u32 = 0x000000c1;
const regMMVM_L2_CNTL5_DEFAULT: u32 = 0x00003fe0;

static mmhub_client_ids_v4_2_0: [[*const u8; 2]; 47] = [
    [b"VMC\0".as_ptr(), core::ptr::null()], [core::ptr::null(); 2],
    [b"MPNHT\0".as_ptr(), b"MPNHT\0".as_ptr()], [core::ptr::null(), b"DBGU0\0".as_ptr()],
    [core::ptr::null(); 2], [core::ptr::null(); 2], [core::ptr::null(); 2],
    [b"MPIFOE\0".as_ptr(), b"MPIFOE\0".as_ptr()], [b"MPIO\0".as_ptr(), b"MPIO\0".as_ptr()],
    [core::ptr::null(); 2], [core::ptr::null(), b"UTCL2_NHT\0".as_ptr()],
    [b"JPEG0\0".as_ptr(), b"JPEG0\0".as_ptr()], [b"VCN0\0".as_ptr(), b"VCN0\0".as_ptr()],
    [b"VCNU0\0".as_ptr(), b"VCNU0\0".as_ptr()], [b"VSCH0\0".as_ptr(), b"VSCH0\0".as_ptr()],
    [b"LSDMA\0".as_ptr(), b"LSDMA\0".as_ptr()], [core::ptr::null(); 2],
    [core::ptr::null(); 2], [core::ptr::null(); 2], [core::ptr::null(); 2],
    [core::ptr::null(); 2], [core::ptr::null(); 2], [core::ptr::null(); 2],
    [core::ptr::null(); 2], [core::ptr::null(); 2], [core::ptr::null(); 2],
    [core::ptr::null(); 2], [core::ptr::null(); 2], [core::ptr::null(); 2],
    [core::ptr::null(); 2], [core::ptr::null(); 2], [core::ptr::null(); 2],
    [core::ptr::null(); 2], [b"DBGU1\0".as_ptr(), b"DBGU1\0".as_ptr()],
    [b"DBGU2\0".as_ptr(), b"DBGU2\0".as_ptr()], [b"MPRAS\0".as_ptr(), b"MPRAS\0".as_ptr()],
    [b"MP1\0".as_ptr(), b"MP1\0".as_ptr()], [b"MP0\0".as_ptr(), b"MP0\0".as_ptr()],
    [core::ptr::null(), b"IH\0".as_ptr()], [core::ptr::null(); 2],
    [core::ptr::null(); 2], [core::ptr::null(), b"JPEG1\0".as_ptr()],
    [core::ptr::null(), b"VCN1\0".as_ptr()], [core::ptr::null(), b"VCNU1\0".as_ptr()],
    [core::ptr::null(), b"VSCH1\0".as_ptr()], [core::ptr::null(); 2],
    [core::ptr::null(); 2],
];

unsafe fn mmhub_v4_2_0_get_xgmi_info(adev: *mut amdgpu_device) -> i32 {
    if !(*adev).gmc.xgmi.connected_to_cpu { return 0; }
    let xgmi_lfb_cntl = RREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regMMMC_VM_XGMI_LFB_CNTL);
    let seg_size = (REG_GET_FIELD(RREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regMMMC_VM_XGMI_LFB_SIZE), MMMC_VM_XGMI_LFB_SIZE, PF_LFB_SIZE) as u64) << 24;
    let max_region = REG_GET_FIELD(xgmi_lfb_cntl, MMMC_VM_XGMI_LFB_CNTL, PF_MAX_REGION);
    (*adev).gmc.xgmi.num_physical_nodes = max_region + 1;
    if (*adev).gmc.xgmi.num_physical_nodes > 4 { return -EINVAL; }
    (*adev).gmc.xgmi.physical_node_id = REG_GET_FIELD(xgmi_lfb_cntl, MMMC_VM_XGMI_LFB_CNTL, PF_LFB_REGION);
    if (*adev).gmc.xgmi.physical_node_id > 3 { return -EINVAL; }
    (*adev).gmc.xgmi.node_segment_size = seg_size;
    0
}

unsafe fn mmhub_v4_2_0_get_fb_location(adev: *mut amdgpu_device) -> u64 {
    let mut base = RREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regMMMC_VM_FB_LOCATION_BASE_LO32) as u64;
    base &= MMMC_VM_FB_LOCATION_BASE_LO32__FB_BASE_LO32_MASK as u64;
    base <<= 24;
    base |= ((MMMC_VM_FB_LOCATION_BASE_HI32__FB_BASE_HI1_MASK & RREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regMMMC_VM_FB_LOCATION_BASE_HI32)) as u64) << 56;
    base
}

unsafe fn mmhub_v4_2_0_get_mc_fb_offset(adev: *mut amdgpu_device) -> u64 {
    (RREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regMMMC_VM_FB_OFFSET) as u64) << 24
}

unsafe fn mmhub_v4_2_0_mid_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64, mid_mask: u32) {
    let mut i: i32 = 0;
    for_each_inst!(i, mid_mask) {
        let hub = &(*adev).vmhub[AMDGPU_MMHUB0(i)];
        WREG32_SOC15_OFFSET(MMHUB, GET_INST(MMHUB, i), regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32, hub.ctx_addr_distance * vmid, lower_32_bits(page_table_base));
        WREG32_SOC15_OFFSET(MMHUB, GET_INST(MMHUB, i), regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32, hub.ctx_addr_distance * vmid, upper_32_bits(page_table_base));
    }
}

unsafe fn mmhub_v4_2_0_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    mmhub_v4_2_0_mid_setup_vm_pt_regs(adev, vmid, page_table_base, (*adev).aid_mask);
}

unsafe fn mmhub_v4_2_0_mid_init_gart_aperture_regs(adev: *mut amdgpu_device, mid_mask: u32) {
    let pt_base = if !(*adev).gmc.pdb0_bo.is_null() { amdgpu_gmc_pd_addr((*adev).gmc.pdb0_bo) } else { amdgpu_gmc_pd_addr((*adev).gart.bo) };
    mmhub_v4_2_0_mid_setup_vm_pt_regs(adev, 0, pt_base, mid_mask);
    let mut i: i32 = 0;
    for_each_inst!(i, mid_mask) {
        let (start, end) = if !(*adev).gmc.pdb0_bo.is_null() { ((*adev).gmc.fb_start, (*adev).gmc.gart_end) } else { ((*adev).gmc.gart_start, (*adev).gmc.gart_end) };
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, (start >> 12) as u32);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, (start >> 44) as u32);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, (end >> 12) as u32);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, (end >> 44) as u32);
    }
}

unsafe fn mmhub_v4_2_0_mid_init_system_aperture_regs(adev: *mut amdgpu_device, mid_mask: u32) {
    if amdgpu_sriov_vf(adev) { return; }
    let mut i: i32 = 0;
    for_each_inst!(i, mid_mask) {
        if !(*adev).gmc.pdb0_bo.is_null() {
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_FB_LOCATION_TOP_LO32, 0);
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_FB_LOCATION_TOP_HI32, 0);
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_FB_LOCATION_BASE_LO32, 0xffffffff);
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_FB_LOCATION_BASE_HI32, 1);
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_AGP_TOP_LO32, 0);
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_AGP_TOP_HI32, 0);
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_AGP_BOT_LO32, 0xffffffff);
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_AGP_BOT_HI32, 1);
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_SYSTEM_APERTURE_LOW_ADDR_LO32, 0xffffffff);
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_SYSTEM_APERTURE_LOW_ADDR_HI32, 0x7f);
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_SYSTEM_APERTURE_HIGH_ADDR_LO32, 0);
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_SYSTEM_APERTURE_HIGH_ADDR_HI32, 0);
        } else {
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_AGP_BASE_LO32, 0);
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_AGP_BASE_HI32, 0);
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_AGP_BOT_LO32, lower_32_bits((*adev).gmc.agp_start >> 24));
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_AGP_BOT_HI32, upper_32_bits((*adev).gmc.agp_start >> 24));
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_AGP_TOP_LO32, lower_32_bits((*adev).gmc.agp_end >> 24));
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_AGP_TOP_HI32, upper_32_bits((*adev).gmc.agp_end >> 24));
            let low = core::cmp::min((*adev).gmc.fb_start, (*adev).gmc.agp_start) >> 18;
            let high = core::cmp::max((*adev).gmc.fb_end, (*adev).gmc.agp_end) >> 18;
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_SYSTEM_APERTURE_LOW_ADDR_LO32, lower_32_bits(low));
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_SYSTEM_APERTURE_LOW_ADDR_HI32, upper_32_bits(low));
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_SYSTEM_APERTURE_HIGH_ADDR_LO32, lower_32_bits(high));
            WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_SYSTEM_APERTURE_HIGH_ADDR_HI32, upper_32_bits(high));
        }
        let value = amdgpu_gmc_vram_mc2pa(adev, (*adev).mem_scratch.gpu_addr);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB, (value >> 12) as u32);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB, (value >> 44) as u32);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32, ((*adev).dummy_page_addr >> 12) as u32);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32, ((*adev).dummy_page_addr >> 44) as u32);
        let mut tmp = RREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_PROTECTION_FAULT_CNTL2);
        tmp = REG_SET_FIELD(tmp, MMVM_L2_PROTECTION_FAULT_CNTL2, ACTIVE_PAGE_MIGRATION_PTE_READ_RETRY, 1);
        tmp = REG_SET_FIELD(tmp, MMVM_L2_PROTECTION_FAULT_CNTL2, ENABLE_RETRY_FAULT_INTERRUPT, 1);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_PROTECTION_FAULT_CNTL2, tmp);
    }
}

unsafe fn mmhub_v4_2_0_mid_init_tlb_regs(adev: *mut amdgpu_device, mid_mask: u32) {
    let mut i: i32 = 0;
    for_each_inst!(i, mid_mask) {
        let mut tmp = RREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_MX_L1_TLB_CNTL);
        tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 1);
        tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, SYSTEM_ACCESS_MODE, 3);
        tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 1);
        tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, SYSTEM_APERTURE_UNMAPPED_ACCESS, 0);
        tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, ECO_BITS, 0);
        tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, MTYPE, MTYPE_UC);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMMC_VM_MX_L1_TLB_CNTL, tmp);
    }
}

unsafe fn mmhub_v4_2_0_mid_init_cache_regs(adev: *mut amdgpu_device, mid_mask: u32) {
    if amdgpu_sriov_vf(adev) { return; }
    let mut i: i32 = 0;
    for_each_inst!(i, mid_mask) {
        let mut tmp = RREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_CNTL);
        tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, ENABLE_L2_CACHE, 1);
        tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, ENABLE_L2_FRAGMENT_PROCESSING, 0);
        tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, ENABLE_DEFAULT_PAGE_OUT_TO_SYSTEM_MEMORY, 1);
        tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, L2_PDE0_CACHE_TAG_GENERATION_MODE, 0);
        tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, PDE_FAULT_CLASSIFICATION, 0);
        tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, CONTEXT1_IDENTITY_ACCESS_MODE, 1);
        tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, IDENTITY_MODE_FRAGMENT_SIZE, 0);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_CNTL, tmp);
        tmp = RREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_CNTL2);
        tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL2, INVALIDATE_ALL_L1_TLBS, 1);
        tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL2, INVALIDATE_L2_CACHE, 1);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_CNTL2, tmp);
        tmp = regMMVM_L2_CNTL3_DEFAULT;
        if (*adev).gmc.translate_further { tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL3, BANK_SELECT, 12); tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 9); }
        else { tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL3, BANK_SELECT, 9); tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL3, L2_CACHE_BIGK_FRAGMENT_SIZE, 6); }
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_CNTL3, tmp);
        tmp = regMMVM_L2_CNTL4_DEFAULT;
        let physical = (*adev).gmc.xgmi.connected_to_cpu || (*adev).gmc.is_app_apu;
        tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL4, VMC_TAP_PDE_REQUEST_PHYSICAL, physical as u32);
        tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL4, VMC_TAP_PTE_REQUEST_PHYSICAL, physical as u32);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_CNTL4, tmp);
        tmp = REG_SET_FIELD(regMMVM_L2_CNTL5_DEFAULT, MMVM_L2_CNTL5, L2_CACHE_SMALLK_FRAGMENT_SIZE, 0);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_CNTL5, tmp);
    }
}

unsafe fn mmhub_v4_2_0_mid_enable_system_domain(adev: *mut amdgpu_device, mid_mask: u32) {
    let mut i: i32 = 0;
    for_each_inst!(i, mid_mask) {
        let mut tmp = RREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_CONTEXT0_CNTL);
        tmp = REG_SET_FIELD(tmp, MMVM_CONTEXT0_CNTL, ENABLE_CONTEXT, 1);
        tmp = REG_SET_FIELD(tmp, MMVM_CONTEXT0_CNTL, PAGE_TABLE_DEPTH, (*adev).gmc.vmid0_page_table_depth);
        tmp = REG_SET_FIELD(tmp, MMVM_CONTEXT0_CNTL, PAGE_TABLE_BLOCK_SIZE, (*adev).gmc.vmid0_page_table_block_size);
        tmp = REG_SET_FIELD(tmp, MMVM_CONTEXT0_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, 0);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_CONTEXT0_CNTL, tmp);
    }
}

unsafe fn mmhub_v4_2_0_mid_disable_identity_aperture(adev: *mut amdgpu_device, mid_mask: u32) {
    if amdgpu_sriov_vf(adev) { return; }
    let mut i: i32 = 0;
    for_each_inst!(i, mid_mask) {
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_LO32, 0xffffffff);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_HI32, 0x1fff);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_LO32, 0);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_HI32, 0);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_LO32, 0);
        WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_HI32, 0);
    }
}

unsafe fn mmhub_v4_2_0_mid_setup_vmid_config(adev: *mut amdgpu_device, mid_mask: u32) {
    let mut j: i32 = 0;
    for_each_inst!(j, mid_mask) {
        let hub = &(*adev).vmhub[AMDGPU_MMHUB0(j)];
        for i in 0..=14 {
            let off = i * hub.ctx_distance;
            let mut tmp = RREG32_SOC15_OFFSET(MMHUB, GET_INST(MMHUB, j), regMMVM_CONTEXT1_CNTL, off);
            tmp = REG_SET_FIELD(tmp, MMVM_CONTEXT1_CNTL, ENABLE_CONTEXT, 1);
            tmp = REG_SET_FIELD(tmp, MMVM_CONTEXT1_CNTL, PAGE_TABLE_DEPTH, (*adev).vm_manager.num_level);
            for field in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT, DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT, PDE0_PROTECTION_FAULT_ENABLE_DEFAULT, VALID_PROTECTION_FAULT_ENABLE_DEFAULT, READ_PROTECTION_FAULT_ENABLE_DEFAULT, WRITE_PROTECTION_FAULT_ENABLE_DEFAULT, EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT] { tmp = REG_SET_FIELD(tmp, MMVM_CONTEXT1_CNTL, field, 1); }
            tmp = REG_SET_FIELD(tmp, MMVM_CONTEXT1_CNTL, PAGE_TABLE_BLOCK_SIZE, (*adev).vm_manager.block_size - 9);
            tmp = REG_SET_FIELD(tmp, MMVM_CONTEXT1_CNTL, RETRY_PERMISSION_OR_INVALID_PAGE_FAULT, (!(*adev).gmc.noretry) as u32);
            WREG32_SOC15_OFFSET(MMHUB, GET_INST(MMHUB, j), regMMVM_CONTEXT1_CNTL, off, tmp);
            let aoff = i * hub.ctx_addr_distance;
            WREG32_SOC15_OFFSET(MMHUB, GET_INST(MMHUB, j), regMMVM_CONTEXT1_PAGE_TABLE_START_ADDR_LO32, aoff, 0);
            WREG32_SOC15_OFFSET(MMHUB, GET_INST(MMHUB, j), regMMVM_CONTEXT1_PAGE_TABLE_START_ADDR_HI32, aoff, 0);
            WREG32_SOC15_OFFSET(MMHUB, GET_INST(MMHUB, j), regMMVM_CONTEXT1_PAGE_TABLE_END_ADDR_LO32, aoff, lower_32_bits((*adev).vm_manager.max_pfn - 1));
            WREG32_SOC15_OFFSET(MMHUB, GET_INST(MMHUB, j), regMMVM_CONTEXT1_PAGE_TABLE_END_ADDR_HI32, aoff, upper_32_bits((*adev).vm_manager.max_pfn - 1));
        }
    }
}

unsafe fn mmhub_v4_2_0_mid_program_invalidation(adev: *mut amdgpu_device, mid_mask: u32) {
    let mut j: i32 = 0;
    for_each_inst!(j, mid_mask) { let hub = &(*adev).vmhub[AMDGPU_MMHUB0(j)]; for i in 0..18 { let off = i * hub.eng_addr_distance; WREG32_SOC15_OFFSET(MMHUB, GET_INST(MMHUB, j), regMMVM_INVALIDATE_ENG0_ADDR_RANGE_LO32, off, 0xffffffff); WREG32_SOC15_OFFSET(MMHUB, GET_INST(MMHUB, j), regMMVM_INVALIDATE_ENG0_ADDR_RANGE_HI32, off, 0x3fff); } }
}

unsafe fn mmhub_v4_2_0_mid_gart_enable(adev: *mut amdgpu_device, mid_mask: u32) -> i32 { mmhub_v4_2_0_mid_init_gart_aperture_regs(adev, mid_mask); mmhub_v4_2_0_mid_init_system_aperture_regs(adev, mid_mask); mmhub_v4_2_0_mid_init_tlb_regs(adev, mid_mask); mmhub_v4_2_0_mid_init_cache_regs(adev, mid_mask); mmhub_v4_2_0_mid_enable_system_domain(adev, mid_mask); mmhub_v4_2_0_mid_disable_identity_aperture(adev, mid_mask); mmhub_v4_2_0_mid_setup_vmid_config(adev, mid_mask); mmhub_v4_2_0_mid_program_invalidation(adev, mid_mask); 0 }
unsafe fn mmhub_v4_2_0_gart_enable(adev: *mut amdgpu_device) -> i32 { mmhub_v4_2_0_mid_gart_enable(adev, (*adev).aid_mask) }

unsafe fn mmhub_v4_2_0_mid_gart_disable(adev: *mut amdgpu_device, mid_mask: u32) {
    let mut j: i32 = 0;
    for_each_inst!(j, mid_mask) { let hub = &(*adev).vmhub[AMDGPU_MMHUB0(j)]; for i in 0..16 { WREG32_SOC15_OFFSET(MMHUB, GET_INST(MMHUB, j), regMMVM_CONTEXT0_CNTL, i * hub.ctx_distance, 0); } let mut tmp = RREG32_SOC15(MMHUB, GET_INST(MMHUB, j), regMMMC_VM_MX_L1_TLB_CNTL); tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, ENABLE_L1_TLB, 0); tmp = REG_SET_FIELD(tmp, MMMC_VM_MX_L1_TLB_CNTL, ENABLE_ADVANCED_DRIVER_MODEL, 0); WREG32_SOC15(MMHUB, GET_INST(MMHUB, j), regMMMC_VM_MX_L1_TLB_CNTL, tmp); tmp = RREG32_SOC15(MMHUB, GET_INST(MMHUB, j), regMMVM_L2_CNTL); tmp = REG_SET_FIELD(tmp, MMVM_L2_CNTL, ENABLE_L2_CACHE, 0); WREG32_SOC15(MMHUB, GET_INST(MMHUB, j), regMMVM_L2_CNTL, tmp); WREG32_SOC15(MMHUB, GET_INST(MMHUB, j), regMMVM_L2_CNTL3, 0); }
}
unsafe fn mmhub_v4_2_0_gart_disable(adev: *mut amdgpu_device) { mmhub_v4_2_0_mid_gart_disable(adev, (*adev).aid_mask); }

unsafe fn mmhub_v4_2_0_mid_set_fault_enable_default(adev: *mut amdgpu_device, value: bool, mid_mask: u32) {
    if amdgpu_sriov_vf(adev) { return; } let mut i: i32 = 0;
    for_each_inst!(i, mid_mask) { let mut tmp = RREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_PROTECTION_FAULT_CNTL_LO32); for field in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT, PDE0_PROTECTION_FAULT_ENABLE_DEFAULT, PDE1_PROTECTION_FAULT_ENABLE_DEFAULT, PDE2_PROTECTION_FAULT_ENABLE_DEFAULT, TRANSLATE_FURTHER_PROTECTION_FAULT_ENABLE_DEFAULT, NACK_PROTECTION_FAULT_ENABLE_DEFAULT, DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT, VALID_PROTECTION_FAULT_ENABLE_DEFAULT, READ_PROTECTION_FAULT_ENABLE_DEFAULT, WRITE_PROTECTION_FAULT_ENABLE_DEFAULT, EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT] { tmp = REG_SET_FIELD(tmp, MMVM_L2_PROTECTION_FAULT_CNTL_LO32, field, value as u32); } if !value { tmp = REG_SET_FIELD(tmp, MMVM_L2_PROTECTION_FAULT_CNTL_LO32, CRASH_ON_NO_RETRY_FAULT, 1); } WREG32_SOC15(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_PROTECTION_FAULT_CNTL_LO32, tmp); }
}
unsafe fn mmhub_v4_2_0_set_fault_enable_default(adev: *mut amdgpu_device, value: bool) { mmhub_v4_2_0_mid_set_fault_enable_default(adev, value, (*adev).aid_mask); }

unsafe fn mmhub_v4_2_0_get_invalidate_req(vmid: u32, _flush_type: u32) -> u32 { let mut req = 0; req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, PER_VMID_INVALIDATE_REQ, 1 << vmid); req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, FLUSH_TYPE, 0); for field in [INVALIDATE_L2_PTES, INVALIDATE_L2_PDE0, INVALIDATE_L2_PDE1, INVALIDATE_L2_PDE2, INVALIDATE_L2_PDE3, INVALIDATE_L1_PTES] { req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, field, 1); } REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, CLEAR_PROTECTION_FAULT_STATUS_ADDR, 0) }

static mmhub_v4_2_0_vmhub_funcs: amdgpu_vmhub_funcs = amdgpu_vmhub_funcs { print_l2_protection_fault_status: Some(mmhub_v4_2_0_print_l2_protection_fault_status), get_invalidate_req: Some(mmhub_v4_2_0_get_invalidate_req) };

unsafe fn mmhub_v4_2_0_print_l2_protection_fault_status(adev: *mut amdgpu_device, status: u32) { let cid = REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS_LO32, CID); let rw = REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS_LO32, RW); dev_err!((*adev).dev, "MMVM_L2_PROTECTION_FAULT_STATUS_LO32:0x%08X\n", status); let name = amdgpu_mmhub_client_name(&(*adev).mmhub, cid, rw); dev_err!((*adev).dev, "\t Faulty UTCL2 client ID: %s (0x%x)\n", if !name.is_null() { name } else { b"unknown\0".as_ptr() }, cid); dev_err!((*adev).dev, "\t MORE_FAULTS: 0x%lx\n", REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS_LO32, MORE_FAULTS)); dev_err!((*adev).dev, "\t WALKER_ERROR: 0x%lx\n", REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS_LO32, WALKER_ERROR)); dev_err!((*adev).dev, "\t PERMISSION_FAULTS: 0x%lx\n", REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS_LO32, PERMISSION_FAULTS)); dev_err!((*adev).dev, "\t MAPPING_ERROR: 0x%lx\n", REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS_LO32, MAPPING_ERROR)); dev_err!((*adev).dev, "\t RW: 0x%x\n", rw); }

unsafe fn mmhub_v4_2_0_mid_init(adev: *mut amdgpu_device, mid_mask: u32) {
    let mut i: i32 = 0;
    for_each_inst!(i, mid_mask) {
        let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0(i)];
        hub.ctx0_ptb_addr_lo32 = SOC15_REG_OFFSET(MMHUB, GET_INST(MMHUB, i), regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32);
        hub.ctx0_ptb_addr_hi32 = SOC15_REG_OFFSET(MMHUB, GET_INST(MMHUB, i), regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32);
        hub.vm_inv_eng0_sem = SOC15_REG_OFFSET(MMHUB, GET_INST(MMHUB, i), regMMVM_INVALIDATE_ENG0_SEM);
        hub.vm_inv_eng0_req = SOC15_REG_OFFSET(MMHUB, GET_INST(MMHUB, i), regMMVM_INVALIDATE_ENG0_REQ);
        hub.vm_inv_eng0_ack = SOC15_REG_OFFSET(MMHUB, GET_INST(MMHUB, i), regMMVM_INVALIDATE_ENG0_ACK);
        hub.vm_context0_cntl = SOC15_REG_OFFSET(MMHUB, GET_INST(MMHUB, i), regMMVM_CONTEXT0_CNTL);
        hub.vm_l2_pro_fault_status = SOC15_REG_OFFSET(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_PROTECTION_FAULT_STATUS_LO32);
        hub.vm_l2_pro_fault_cntl = SOC15_REG_OFFSET(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_PROTECTION_FAULT_CNTL_LO32);
        hub.ctx_distance = regMMVM_CONTEXT1_CNTL - regMMVM_CONTEXT0_CNTL;
        hub.ctx_addr_distance = regMMVM_CONTEXT1_PAGE_TABLE_BASE_ADDR_LO32 - regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32;
        hub.eng_distance = regMMVM_INVALIDATE_ENG1_REQ - regMMVM_INVALIDATE_ENG0_REQ;
        hub.eng_addr_distance = regMMVM_INVALIDATE_ENG1_ADDR_RANGE_LO32 - regMMVM_INVALIDATE_ENG0_ADDR_RANGE_LO32;
        hub.vm_cntx_cntl_vm_fault = MMVM_CONTEXT1_CNTL__RANGE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK | MMVM_CONTEXT1_CNTL__DUMMY_PAGE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK | MMVM_CONTEXT1_CNTL__PDE0_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK | MMVM_CONTEXT1_CNTL__VALID_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK | MMVM_CONTEXT1_CNTL__READ_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK | MMVM_CONTEXT1_CNTL__WRITE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK | MMVM_CONTEXT1_CNTL__EXECUTE_PROTECTION_FAULT_ENABLE_INTERRUPT_MASK;
        hub.vm_l2_bank_select_reserved_cid2 = SOC15_REG_OFFSET(MMHUB, GET_INST(MMHUB, i), regMMVM_L2_BANK_SELECT_RESERVED_CID2);
        hub.vm_contexts_disable = SOC15_REG_OFFSET(MMHUB, GET_INST(MMHUB, i), regMMVM_CONTEXTS_DISABLE);
        hub.vmhub_funcs = &mmhub_v4_2_0_vmhub_funcs;
    }
}

unsafe fn mmhub_v4_2_0_init(adev: *mut amdgpu_device) { mmhub_v4_2_0_mid_init(adev, (*adev).aid_mask); amdgpu_mmhub_init_client_info(&mut (*adev).mmhub, mmhub_client_ids_v4_2_0.as_ptr(), ARRAY_SIZE!(mmhub_client_ids_v4_2_0)); }

unsafe fn mmhub_v4_2_0_update_medium_grain_clock_gating(adev: *mut amdgpu_device, enable: bool) {
    let mut def = RREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regMM_ATC_L2_MISC_CG); let mut data = def;
    let def1 = RREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regDAGB0_CNTL_MISC2); let mut data1 = def1;
    let def2 = RREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regDAGB1_CNTL_MISC2); let mut data2 = def2;
    if enable { data |= MM_ATC_L2_MISC_CG__ENABLE_MASK; data1 &= !(DAGB0_CNTL_MISC2__DISABLE_RDRET_TAP_CHAIN_FGCG_MASK | DAGB0_CNTL_MISC2__DISABLE_WRRET_TAP_CHAIN_FGCG_MASK); data2 &= !(DAGB1_CNTL_MISC2__DISABLE_RDRET_TAP_CHAIN_FGCG_MASK | DAGB1_CNTL_MISC2__DISABLE_WRRET_TAP_CHAIN_FGCG_MASK); } else { data &= !MM_ATC_L2_MISC_CG__ENABLE_MASK; data1 |= DAGB0_CNTL_MISC2__DISABLE_RDRET_TAP_CHAIN_FGCG_MASK | DAGB0_CNTL_MISC2__DISABLE_WRRET_TAP_CHAIN_FGCG_MASK; data2 |= DAGB1_CNTL_MISC2__DISABLE_RDRET_TAP_CHAIN_FGCG_MASK | DAGB1_CNTL_MISC2__DISABLE_WRRET_TAP_CHAIN_FGCG_MASK; }
    if def != data { WREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regMM_ATC_L2_MISC_CG, data); } if def1 != data1 { WREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regDAGB0_CNTL_MISC2, data1); } if def2 != data2 { WREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regDAGB1_CNTL_MISC2, data2); }
}
unsafe fn mmhub_v4_2_0_update_medium_grain_light_sleep(adev: *mut amdgpu_device, enable: bool) { let def = RREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regMM_ATC_L2_MISC_CG); let mut data = def; if enable { data |= MM_ATC_L2_MISC_CG__MEM_LS_ENABLE_MASK; } else { data &= !MM_ATC_L2_MISC_CG__MEM_LS_ENABLE_MASK; } if def != data { WREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regMM_ATC_L2_MISC_CG, data); } }
unsafe fn mmhub_v4_2_0_set_clockgating(adev: *mut amdgpu_device, state: amd_clockgating_state) -> i32 { if amdgpu_sriov_vf(adev) { return 0; } if (*adev).cg_flags & AMD_CG_SUPPORT_MC_MGCG != 0 { mmhub_v4_2_0_update_medium_grain_clock_gating(adev, state == AMD_CG_STATE_GATE); } if (*adev).cg_flags & AMD_CG_SUPPORT_MC_LS != 0 { mmhub_v4_2_0_update_medium_grain_light_sleep(adev, state == AMD_CG_STATE_GATE); } 0 }
unsafe fn mmhub_v4_2_0_get_clockgating(adev: *mut amdgpu_device, flags: *mut u64) { if amdgpu_sriov_vf(adev) { *flags = 0; } let data = RREG32_SOC15(MMHUB, GET_INST(MMHUB, 0), regMM_ATC_L2_MISC_CG); if data & MM_ATC_L2_MISC_CG__ENABLE_MASK != 0 { *flags |= AMD_CG_SUPPORT_MC_MGCG as u64; } if data & MM_ATC_L2_MISC_CG__MEM_LS_ENABLE_MASK != 0 { *flags |= AMD_CG_SUPPORT_MC_LS as u64; } }

pub static mmhub_v4_2_0_funcs: amdgpu_mmhub_funcs = amdgpu_mmhub_funcs { init: Some(mmhub_v4_2_0_init), get_fb_location: Some(mmhub_v4_2_0_get_fb_location), get_mc_fb_offset: Some(mmhub_v4_2_0_get_mc_fb_offset), setup_vm_pt_regs: Some(mmhub_v4_2_0_setup_vm_pt_regs), gart_enable: Some(mmhub_v4_2_0_gart_enable), gart_disable: Some(mmhub_v4_2_0_gart_disable), set_fault_enable_default: Some(mmhub_v4_2_0_set_fault_enable_default), set_clockgating: Some(mmhub_v4_2_0_set_clockgating), get_clockgating: Some(mmhub_v4_2_0_get_clockgating), get_xgmi_info: Some(mmhub_v4_2_0_get_xgmi_info) };

unsafe fn mmhub_v4_2_0_xcp_resume(handle: *mut core::ffi::c_void, inst_mask: u32) -> i32 { let adev = handle as *mut amdgpu_device; let value = amdgpu_vm_fault_stop != AMDGPU_VM_FAULT_STOP_ALWAYS; mmhub_v4_2_0_mid_set_fault_enable_default(adev, value, inst_mask); if !amdgpu_sriov_vf(adev) { return mmhub_v4_2_0_mid_gart_enable(adev, inst_mask); } 0 }
unsafe fn mmhub_v4_2_0_xcp_suspend(handle: *mut core::ffi::c_void, inst_mask: u32) -> i32 { let adev = handle as *mut amdgpu_device; if !amdgpu_sriov_vf(adev) { mmhub_v4_2_0_mid_gart_disable(adev, inst_mask); } 0 }
pub static mut mmhub_v4_2_0_xcp_funcs: amdgpu_xcp_ip_funcs = amdgpu_xcp_ip_funcs { suspend: Some(mmhub_v4_2_0_xcp_suspend), resume: Some(mmhub_v4_2_0_xcp_resume) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
