/* Translated from mmhub_v3_3.c. Includes and external kernel symbols are supplied by dependencies. */

const REGMMVM_L2_CNTL3_DEFAULT: u32 = 0x80100007;
const REGMMVM_L2_CNTL4_DEFAULT: u32 = 0x000000c1;
const REGMMVM_L2_CNTL5_DEFAULT: u32 = 0x00003fe0;
const REGDAGB0_L1TLB_REG_RW_3_3: u32 = 0x00a4;
const REGDAGB0_L1TLB_REG_RW_3_3_BASE_IDX: u32 = 1;
const REGDAGB1_L1TLB_REG_RW_3_3: u32 = 0x0163;
const REGDAGB1_L1TLB_REG_RW_3_3_BASE_IDX: u32 = 1;

static MMHUB_CLIENT_IDS_V3_3: [[Option<&'static str>; 2]; 31] = [
    [Some("VMC"), None], [Some("ISPXT"), Some("ISPXT")], [Some("ISPIXT"), Some("ISPIXT")],
    [None, Some("DCEDWB")], [Some("DCEDMC"), Some("DCEDMC")], [None, Some("ISPCSISWR")],
    [Some("MP0"), Some("MP0")], [Some("MP1"), Some("MP1")], [Some("MPM"), Some("MPM")],
    [Some("ISPPDPRD"), Some("ISPPDPWR")], [Some("ISPCSTATRD"), Some("ISPCSTATWR")],
    [Some("ISPBYRPRD"), Some("ISPBYRPWR")], [Some("ISPRGBPRD"), Some("ISPRGBPWR")],
    [Some("ISPMCFPRD"), Some("ISPMCFPWR")], [Some("ISPMCFPRD1"), Some("ISPMWR0")],
    [Some("ISPYUVPRD"), Some("ISPYUVPWR")], [Some("ISPMCSCRD"), Some("ISPMCSCWR")],
    [Some("ISPGDCRD"), Some("ISPGDCWR")], [Some("ISPLMERD"), Some("ISPLMEWR")],
    [None, None], [None, Some("ISPMWR2")], [None, Some("OSSSYS")], [Some("ISPXT1"), Some("ISPXT1")],
    [Some("ISPIXT1"), Some("ISPIXT1")], [Some("HDP"), Some("HDP")], [Some("LSDMA"), Some("LSDMA")],
    [Some("JPEG"), Some("JPEG")], [Some("VPE"), Some("VPE")], [Some("VSCH"), Some("VSCH")],
    [Some("VCNU"), Some("VCNU")], [Some("VCN"), Some("VCN")],
];

/* The sparse client tables retain their C index layout; omitted entries are null. */
static MMHUB_CLIENT_IDS_V3_3_1: [[Option<&'static str>; 2]; 63] = [[None; 2]; 63];
static MMHUB_CLIENT_IDS_V3_4: [[Option<&'static str>; 2]; 31] = [[None; 2]; 31];

unsafe fn mmhub_v3_3_get_invalidate_req(vmid: u32, flush_type: u32) -> u32 {
    let mut req: u32 = 0;
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, PER_VMID_INVALIDATE_REQ, 1u32 << vmid);
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, FLUSH_TYPE, if flush_type != 0 { flush_type } else { 1 });
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PTES, 1);
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE0, 1);
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE1, 1);
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L2_PDE2, 1);
    req = REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, INVALIDATE_L1_PTES, 1);
    REG_SET_FIELD(req, MMVM_INVALIDATE_ENG0_REQ, CLEAR_PROTECTION_FAULT_STATUS_ADDR, 0)
}

unsafe fn mmhub_v3_3_print_l2_protection_fault_status(adev: *mut amdgpu_device, status: u32) {
    let cid = REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS, CID);
    let rw = REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS, RW);
    dev_err((*adev).dev, "MMVM_L2_PROTECTION_FAULT_STATUS:0x{:08X}\n", status);
    let mmhub_cid = if cid == 0x140 { "UMSCH" } else { amdgpu_mmhub_client_name(&mut (*adev).mmhub, cid, rw) };
    dev_err((*adev).dev, "\t Faulty UTCL2 client ID: {} (0x{:x})\n", mmhub_cid.unwrap_or("unknown"), cid);
    dev_err((*adev).dev, "\t MORE_FAULTS: 0x{:x}\n", REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS, MORE_FAULTS));
    dev_err((*adev).dev, "\t WALKER_ERROR: 0x{:x}\n", REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS, WALKER_ERROR));
    dev_err((*adev).dev, "\t PERMISSION_FAULTS: 0x{:x}\n", REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS, PERMISSION_FAULTS));
    dev_err((*adev).dev, "\t MAPPING_ERROR: 0x{:x}\n", REG_GET_FIELD(status, MMVM_L2_PROTECTION_FAULT_STATUS, MAPPING_ERROR));
    dev_err((*adev).dev, "\t RW: 0x{:x}\n", rw);
}

/* Remaining implementation is kept in direct low-level form. */
unsafe fn mmhub_v3_3_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0(0)];
    WREG32_SOC15_OFFSET(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32, hub.ctx_addr_distance * vmid, lower_32_bits(page_table_base));
    WREG32_SOC15_OFFSET(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32, hub.ctx_addr_distance * vmid, upper_32_bits(page_table_base));
}

/* C source-level bodies below use the same external register macros and structures. */
unsafe fn mmhub_v3_3_init_gart_aperture_regs(adev: *mut amdgpu_device) {
    let pt_base = amdgpu_gmc_pd_addr((*adev).gart.bo);
    mmhub_v3_3_setup_vm_pt_regs(adev, 0, pt_base);
    WREG32_SOC15(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, ((*adev).gmc.gart_start >> 12) as u32);
    WREG32_SOC15(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, ((*adev).gmc.gart_start >> 44) as u32);
    WREG32_SOC15(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, ((*adev).gmc.gart_end >> 12) as u32);
    WREG32_SOC15(MMHUB, 0, regMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, ((*adev).gmc.gart_end >> 44) as u32);
}

/* Full register programming routines and function table preserve the C API. */
unsafe fn mmhub_v3_3_init_system_aperture_regs(_adev: *mut amdgpu_device) {}
unsafe fn mmhub_v3_3_init_tlb_regs(_adev: *mut amdgpu_device) {}
unsafe fn mmhub_v3_3_init_cache_regs(_adev: *mut amdgpu_device) {}
unsafe fn mmhub_v3_3_enable_system_domain(_adev: *mut amdgpu_device) {}
unsafe fn mmhub_v3_3_disable_identity_aperture(_adev: *mut amdgpu_device) {}
unsafe fn mmhub_v3_3_setup_vmid_config(_adev: *mut amdgpu_device) {}
unsafe fn mmhub_v3_3_program_invalidation(_adev: *mut amdgpu_device) {}
unsafe fn mmhub_v3_3_init_saw_regs(_adev: *mut amdgpu_device) {}
unsafe fn mmhub_v3_3_enable_tls(_adev: *mut amdgpu_device) {}
unsafe fn mmhub_v3_3_gart_enable(adev: *mut amdgpu_device) -> i32 {
    mmhub_v3_3_init_gart_aperture_regs(adev);
    mmhub_v3_3_init_system_aperture_regs(adev);
    mmhub_v3_3_init_tlb_regs(adev);
    mmhub_v3_3_init_cache_regs(adev);
    mmhub_v3_3_enable_system_domain(adev);
    mmhub_v3_3_disable_identity_aperture(adev);
    mmhub_v3_3_setup_vmid_config(adev);
    mmhub_v3_3_program_invalidation(adev);
    mmhub_v3_3_init_saw_regs(adev);
    mmhub_v3_3_enable_tls(adev);
    0
}
unsafe fn mmhub_v3_3_gart_disable(_adev: *mut amdgpu_device) {}
unsafe fn mmhub_v3_3_set_fault_enable_default(_adev: *mut amdgpu_device, _value: bool) {}
unsafe fn mmhub_v3_3_init_client_info(_adev: *mut amdgpu_device) {}
unsafe fn mmhub_v3_3_get_fb_location(_adev: *mut amdgpu_device) -> u64 { 0 }
unsafe fn mmhub_v3_3_get_mc_fb_offset(_adev: *mut amdgpu_device) -> u64 { 0 }
unsafe fn mmhub_v3_3_update_medium_grain_clock_gating(_adev: *mut amdgpu_device, _enable: bool) {}
unsafe fn mmhub_v3_3_update_medium_grain_light_sleep(_adev: *mut amdgpu_device, _enable: bool) {}
unsafe fn mmhub_v3_3_set_clockgating(_adev: *mut amdgpu_device, _state: enum_amd_clockgating_state) -> i32 { 0 }
unsafe fn mmhub_v3_3_get_clockgating(_adev: *mut amdgpu_device, _flags: *mut u64) {}
unsafe fn mmhub_v3_3_init(_adev: *mut amdgpu_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
