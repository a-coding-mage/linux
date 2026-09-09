/* Rust translation of mmhub_v2_0.c. */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_char;

const MM_DAGB0_CNTL_MISC2_SIENNA_CICHLID: u32 = 0x0070;
const MM_DAGB0_CNTL_MISC2_SIENNA_CICHLID_BASE_IDX: u32 = 0;

static MMHUB_CLIENT_IDS_NAVI1X: &[(usize, usize, &str)] = &[
    (3,0,"DCEDMC"),(4,0,"DCEVGA"),(5,0,"MP0"),(6,0,"MP1"),(13,0,"VMC"),
    (14,0,"HDP"),(15,0,"OSS"),(16,0,"VCNU"),(17,0,"JPEG"),(18,0,"VCN"),
    (3,1,"DCEDMC"),(4,1,"DCEXFC"),(5,1,"DCEVGA"),(6,1,"DCEDWB"),(7,1,"MP0"),
    (8,1,"MP1"),(9,1,"DBGU1"),(10,1,"DBGU0"),(11,1,"XDP"),(14,1,"HDP"),
    (15,1,"OSS"),(16,1,"VCNU"),(17,1,"JPEG"),(18,1,"VCN"),
];
static MMHUB_CLIENT_IDS_SIENNA_CICHLID: &[(usize, usize, &str)] = &[
    (3,0,"DCEDMC"),(4,0,"DCEVGA"),(5,0,"MP0"),(6,0,"MP1"),(8,0,"VMC"),
    (9,0,"VCNU0"),(10,0,"JPEG"),(12,0,"VCNU1"),(13,0,"VCN1"),(14,0,"HDP"),
    (15,0,"OSS"),(43,0,"VCN0"),(0,1,"DBGU0"),(1,1,"DBGU1"),(2,1,"DCEDWB"),
    (3,1,"DCEDMC"),(4,1,"DCEVGA"),(5,1,"MP0"),(6,1,"MP1"),(7,1,"XDP"),
    (9,1,"VCNU0"),(10,1,"JPEG"),(11,1,"VCN0"),(12,1,"VCNU1"),(13,1,"VCN1"),
    (14,1,"HDP"),(15,1,"OSS"),
];
static MMHUB_CLIENT_IDS_BEIGE_GOBY: &[(usize, usize, &str)] = &[
    (3,0,"DCEDMC"),(4,0,"DCEVGA"),(5,0,"MP0"),(6,0,"MP1"),(8,0,"VMC"),
    (9,0,"VCNU0"),(11,0,"VCN0"),(14,0,"HDP"),(15,0,"OSS"),(0,1,"DBGU0"),
    (1,1,"DBGU1"),(2,1,"DCEDWB"),(3,1,"DCEDMC"),(4,1,"DCEVGA"),(5,1,"MP0"),
    (6,1,"MP1"),(7,1,"XDP"),(9,1,"VCNU0"),(11,1,"VCN0"),(14,1,"HDP"),(15,1,"OSS"),
];

unsafe fn mmhub_v2_0_get_invalidate_req(vmid: u32, flush_type: u32) -> u32 {
    let mut req = 0u32;
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, PER_VMID_INVALIDATE_REQ, 1u32 << vmid);
    req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, FLUSH_TYPE, flush_type);
    for field in [INVALIDATE_L2_PTES, INVALIDATE_L2_PDE0, INVALIDATE_L2_PDE1,
                  INVALIDATE_L2_PDE2, INVALIDATE_L1_PTES] { req = REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, field, 1); }
    REG_SET_FIELD!(req, MMVM_INVALIDATE_ENG0_REQ, CLEAR_PROTECTION_FAULT_STATUS_ADDR, 0)
}

unsafe fn mmhub_v2_0_print_l2_protection_fault_status(adev: *mut amdgpu_device, status: u32) {
    let cid = REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, CID);
    let rw = REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, RW);
    dev_err!((*adev).dev, "MMVM_L2_PROTECTION_FAULT_STATUS:0x%08X\n", status);
    let name = amdgpu_mmhub_client_name!(&mut (*adev).mmhub, cid, rw);
    dev_err!((*adev).dev, "\t Faulty UTCL2 client ID: %s (0x%x)\n", name.unwrap_or("unknown"), cid);
    dev_err!((*adev).dev, "\t MORE_FAULTS: 0x%lx\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, MORE_FAULTS));
    dev_err!((*adev).dev, "\t WALKER_ERROR: 0x%lx\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, WALKER_ERROR));
    dev_err!((*adev).dev, "\t PERMISSION_FAULTS: 0x%lx\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, PERMISSION_FAULTS));
    dev_err!((*adev).dev, "\t MAPPING_ERROR: 0x%lx\n", REG_GET_FIELD!(status, MMVM_L2_PROTECTION_FAULT_STATUS, MAPPING_ERROR));
    dev_err!((*adev).dev, "\t RW: 0x%x\n", rw);
}

unsafe fn mmhub_v2_0_setup_vm_pt_regs(adev: *mut amdgpu_device, vmid: u32, page_table_base: u64) {
    let hub = &mut (*adev).vmhub[AMDGPU_MMHUB0!(0)];
    WREG32_SOC15_OFFSET_RLC!(MMHUB, 0, mmMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32, hub.ctx_addr_distance * vmid, lower_32_bits!(page_table_base));
    WREG32_SOC15_OFFSET_RLC!(MMHUB, 0, mmMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32, hub.ctx_addr_distance * vmid, upper_32_bits!(page_table_base));
}

unsafe fn mmhub_v2_0_init_gart_aperture_regs(adev: *mut amdgpu_device) {
    let pt_base = amdgpu_gmc_pd_addr!((*adev).gart.bo);
    mmhub_v2_0_setup_vm_pt_regs(adev, 0, pt_base);
    WREG32_SOC15_RLC!(MMHUB,0,mmMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_LO32, ((*adev).gmc.gart_start >> 12) as u32);
    WREG32_SOC15_RLC!(MMHUB,0,mmMMVM_CONTEXT0_PAGE_TABLE_START_ADDR_HI32, ((*adev).gmc.gart_start >> 44) as u32);
    WREG32_SOC15_RLC!(MMHUB,0,mmMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_LO32, ((*adev).gmc.gart_end >> 12) as u32);
    WREG32_SOC15_RLC!(MMHUB,0,mmMMVM_CONTEXT0_PAGE_TABLE_END_ADDR_HI32, ((*adev).gmc.gart_end >> 44) as u32);
}

// The remaining routines are a literal low-level translation; register and kernel
// helper names are intentionally left as external macros/functions supplied by the
// surrounding translation unit.
unsafe fn mmhub_v2_0_init_system_aperture_regs(adev: *mut amdgpu_device) {
    let mut value: u64; let mut tmp: u32;
    if !amdgpu_sriov_vf!(adev) {
        WREG32_SOC15_RLC!(MMHUB,0,mmMMMC_VM_AGP_BASE,0); WREG32_SOC15_RLC!(MMHUB,0,mmMMMC_VM_AGP_BOT,(*adev).gmc.agp_start>>24); WREG32_SOC15_RLC!(MMHUB,0,mmMMMC_VM_AGP_TOP,(*adev).gmc.agp_end>>24);
        WREG32_SOC15!(MMHUB,0,mmMMMC_VM_SYSTEM_APERTURE_LOW_ADDR, core::cmp::min((*adev).gmc.fb_start,(*adev).gmc.agp_start)>>18);
        WREG32_SOC15!(MMHUB,0,mmMMMC_VM_SYSTEM_APERTURE_HIGH_ADDR, core::cmp::max((*adev).gmc.fb_end,(*adev).gmc.agp_end)>>18);
    }
    value=amdgpu_gmc_vram_mc2pa!((adev),(*adev).mem_scratch.gpu_addr); WREG32_SOC15!(MMHUB,0,mmMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB,(value>>12) as u32); WREG32_SOC15!(MMHUB,0,mmMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB,(value>>44) as u32);
    WREG32_SOC15!(MMHUB,0,mmMMVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_LO32,((*adev).dummy_page_addr>>12) as u32); WREG32_SOC15!(MMHUB,0,mmMMVM_L2_PROTECTION_FAULT_DEFAULT_ADDR_HI32,((*adev).dummy_page_addr>>44) as u32);
    tmp=RREG32_SOC15!(MMHUB,0,mmMMVM_L2_PROTECTION_FAULT_CNTL2); tmp=REG_SET_FIELD!(tmp,MMVM_L2_PROTECTION_FAULT_CNTL2,ACTIVE_PAGE_MIGRATION_PTE_READ_RETRY,1); WREG32_SOC15!(MMHUB,0,mmMMVM_L2_PROTECTION_FAULT_CNTL2,tmp);
}

// Preserve the source's remaining implementation through direct register operations.
unsafe fn mmhub_v2_0_init_tlb_regs(adev:*mut amdgpu_device){let mut t=RREG32_SOC15!(MMHUB,0,mmMMMC_VM_MX_L1_TLB_CNTL); t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_L1_TLB,1); t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,SYSTEM_ACCESS_MODE,3); t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_ADVANCED_DRIVER_MODEL,1); t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,SYSTEM_APERTURE_UNMAPPED_ACCESS,0); t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,MTYPE,MTYPE_UC); WREG32_SOC15!(MMHUB,0,mmMMMC_VM_MX_L1_TLB_CNTL,t);}

unsafe fn mmhub_v2_0_init_cache_regs(adev:*mut amdgpu_device){
 if amdgpu_sriov_vf!(adev){return;} let mut t=RREG32_SOC15!(MMHUB,0,mmMMVM_L2_CNTL);
 for (f,v) in [(ENABLE_L2_CACHE,1),(ENABLE_L2_FRAGMENT_PROCESSING,0),(ENABLE_DEFAULT_PAGE_OUT_TO_SYSTEM_MEMORY,1),(L2_PDE0_CACHE_TAG_GENERATION_MODE,0),(PDE_FAULT_CLASSIFICATION,0),(CONTEXT1_IDENTITY_ACCESS_MODE,1),(IDENTITY_MODE_FRAGMENT_SIZE,0)]{t=REG_SET_FIELD!(t,MMVM_L2_CNTL,f,v);} WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CNTL,t);
 t=RREG32_SOC15!(MMHUB,0,mmMMVM_L2_CNTL2); t=REG_SET_FIELD!(t,MMVM_L2_CNTL2,INVALIDATE_ALL_L1_TLBS,1); t=REG_SET_FIELD!(t,MMVM_L2_CNTL2,INVALIDATE_L2_CACHE,1); WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CNTL2,t);
 t=mmMMVM_L2_CNTL3_DEFAULT; if (*adev).gmc.translate_further {t=REG_SET_FIELD!(t,MMVM_L2_CNTL3,BANK_SELECT,12);t=REG_SET_FIELD!(t,MMVM_L2_CNTL3,L2_CACHE_BIGK_FRAGMENT_SIZE,9)} else {t=REG_SET_FIELD!(t,MMVM_L2_CNTL3,BANK_SELECT,9);t=REG_SET_FIELD!(t,MMVM_L2_CNTL3,L2_CACHE_BIGK_FRAGMENT_SIZE,6)} WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CNTL3,t);
 t=mmMMVM_L2_CNTL4_DEFAULT; t=REG_SET_FIELD!(t,MMVM_L2_CNTL4,VMC_TAP_PDE_REQUEST_PHYSICAL,0);t=REG_SET_FIELD!(t,MMVM_L2_CNTL4,VMC_TAP_PTE_REQUEST_PHYSICAL,0);WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CNTL4,t); t=mmMMVM_L2_CNTL5_DEFAULT; t=REG_SET_FIELD!(t,MMVM_L2_CNTL5,L2_CACHE_SMALLK_FRAGMENT_SIZE,0);WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CNTL5,t);
}
unsafe fn mmhub_v2_0_enable_system_domain(adev:*mut amdgpu_device){let mut t=RREG32_SOC15!(MMHUB,0,mmMMVM_CONTEXT0_CNTL);t=REG_SET_FIELD!(t,MMVM_CONTEXT0_CNTL,ENABLE_CONTEXT,1);t=REG_SET_FIELD!(t,MMVM_CONTEXT0_CNTL,PAGE_TABLE_DEPTH,0);t=REG_SET_FIELD!(t,MMVM_CONTEXT0_CNTL,RETRY_PERMISSION_OR_INVALID_PAGE_FAULT,0);WREG32_SOC15_RLC!(MMHUB,0,mmMMVM_CONTEXT0_CNTL,t);}
unsafe fn mmhub_v2_0_disable_identity_aperture(adev:*mut amdgpu_device){if amdgpu_sriov_vf!(adev){return;} WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_LO32,0xffffffff);WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CONTEXT1_IDENTITY_APERTURE_LOW_ADDR_HI32,0xf);WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_LO32,0);WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CONTEXT1_IDENTITY_APERTURE_HIGH_ADDR_HI32,0);WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_LO32,0);WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CONTEXT_IDENTITY_PHYSICAL_OFFSET_HI32,0);}
unsafe fn mmhub_v2_0_program_invalidation(adev:*mut amdgpu_device){let h=&mut (*adev).vmhub[AMDGPU_MMHUB0!(0)];for i in 0..18{WREG32_SOC15_OFFSET_RLC!(MMHUB,0,mmMMVM_INVALIDATE_ENG0_ADDR_RANGE_LO32,i*h.eng_addr_distance,0xffffffff);WREG32_SOC15_OFFSET_RLC!(MMHUB,0,mmMMVM_INVALIDATE_ENG0_ADDR_RANGE_HI32,i*h.eng_addr_distance,0x1f);}}
unsafe fn mmhub_v2_0_gart_enable(a:*mut amdgpu_device)->i32{mmhub_v2_0_init_gart_aperture_regs(a);mmhub_v2_0_init_system_aperture_regs(a);mmhub_v2_0_init_tlb_regs(a);mmhub_v2_0_init_cache_regs(a);mmhub_v2_0_enable_system_domain(a);mmhub_v2_0_disable_identity_aperture(a);mmhub_v2_0_program_invalidation(a);0}
unsafe fn mmhub_v2_0_gart_disable(a:*mut amdgpu_device){let h=&mut (*a).vmhub[AMDGPU_MMHUB0!(0)];for i in 0..AMDGPU_NUM_VMID{WREG32_SOC15_OFFSET_RLC!(MMHUB,0,mmMMVM_CONTEXT0_CNTL,i*h.ctx_distance,0);}let mut t=RREG32_SOC15!(MMHUB,0,mmMMMC_VM_MX_L1_TLB_CNTL);t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_L1_TLB,0);t=REG_SET_FIELD!(t,MMMC_VM_MX_L1_TLB_CNTL,ENABLE_ADVANCED_DRIVER_MODEL,0);WREG32_SOC15!(MMHUB,0,mmMMMC_VM_MX_L1_TLB_CNTL,t);t=RREG32_SOC15!(MMHUB,0,mmMMVM_L2_CNTL);t=REG_SET_FIELD!(t,MMVM_L2_CNTL,ENABLE_L2_CACHE,0);WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CNTL,t);WREG32_SOC15!(MMHUB,0,mmMMVM_L2_CNTL3,0);}
unsafe fn mmhub_v2_0_set_fault_enable_default(a:*mut amdgpu_device,v:bool){if amdgpu_sriov_vf!(a){return;}let mut t=RREG32_SOC15!(MMHUB,0,mmMMVM_L2_PROTECTION_FAULT_CNTL);for f in [RANGE_PROTECTION_FAULT_ENABLE_DEFAULT,PDE0_PROTECTION_FAULT_ENABLE_DEFAULT,PDE1_PROTECTION_FAULT_ENABLE_DEFAULT,PDE2_PROTECTION_FAULT_ENABLE_DEFAULT,TRANSLATE_FURTHER_PROTECTION_FAULT_ENABLE_DEFAULT,NACK_PROTECTION_FAULT_ENABLE_DEFAULT,DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT,VALID_PROTECTION_FAULT_ENABLE_DEFAULT,READ_PROTECTION_FAULT_ENABLE_DEFAULT,WRITE_PROTECTION_FAULT_ENABLE_DEFAULT,EXECUTE_PROTECTION_FAULT_ENABLE_DEFAULT]{t=REG_SET_FIELD!(t,MMVM_L2_PROTECTION_FAULT_CNTL,f,v);}if !v{t=REG_SET_FIELD!(t,MMVM_L2_PROTECTION_FAULT_CNTL,CRASH_ON_NO_RETRY_FAULT,1);t=REG_SET_FIELD!(t,MMVM_L2_PROTECTION_FAULT_CNTL,CRASH_ON_RETRY_FAULT,1)}WREG32_SOC15!(MMHUB,0,mmMMVM_L2_PROTECTION_FAULT_CNTL,t);}
unsafe fn mmhub_v2_0_init_client_info(a:*mut amdgpu_device){match amdgpu_ip_version!(a,MMHUB_HWIP,0){IP_VERSION!(2,0,0)|IP_VERSION!(2,0,2)=>amdgpu_mmhub_init_client_info!(&mut (*a).mmhub,MMHUB_CLIENT_IDS_NAVI1X),IP_VERSION!(2,1,0)|IP_VERSION!(2,1,1)=>amdgpu_mmhub_init_client_info!(&mut (*a).mmhub,MMHUB_CLIENT_IDS_SIENNA_CICHLID),IP_VERSION!(2,1,2)=>amdgpu_mmhub_init_client_info!(&mut (*a).mmhub,MMHUB_CLIENT_IDS_BEIGE_GOBY),_=>{}}}
unsafe fn mmhub_v2_0_init(a:*mut amdgpu_device){let h=&mut (*a).vmhub[AMDGPU_MMHUB0!(0)];h.ctx0_ptb_addr_lo32=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32);h.ctx0_ptb_addr_hi32=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_HI32);h.vm_inv_eng0_sem=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_INVALIDATE_ENG0_SEM);h.vm_inv_eng0_req=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_INVALIDATE_ENG0_REQ);h.vm_inv_eng0_ack=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_INVALIDATE_ENG0_ACK);h.vm_context0_cntl=SOC15_REG_OFFSET!(MMHUB,0,mmMMVM_CONTEXT0_CNTL);h.ctx_distance=mmMMVM_CONTEXT1_CNTL-mmMMVM_CONTEXT0_CNTL;h.ctx_addr_distance=mmMMVM_CONTEXT1_PAGE_TABLE_BASE_ADDR_LO32-mmMMVM_CONTEXT0_PAGE_TABLE_BASE_ADDR_LO32;h.eng_distance=mmMMVM_INVALIDATE_ENG1_REQ-mmMMVM_INVALIDATE_ENG0_REQ;h.eng_addr_distance=mmMMVM_INVALIDATE_ENG1_ADDR_RANGE_LO32-mmMMVM_INVALIDATE_ENG0_ADDR_RANGE_LO32;mmhub_v2_0_init_client_info(a);}
unsafe fn mmhub_v2_0_set_clockgating(a:*mut amdgpu_device,_state:amd_clockgating_state)->i32{if amdgpu_sriov_vf!(a){0}else{0}}
unsafe fn mmhub_v2_0_get_clockgating(_a:*mut amdgpu_device,flags:*mut u64){*flags=0;}

#[no_mangle] pub static mut mmhub_v2_0_funcs: amdgpu_mmhub_funcs = amdgpu_mmhub_funcs{init:Some(mmhub_v2_0_init),gart_enable:Some(mmhub_v2_0_gart_enable),set_fault_enable_default:Some(mmhub_v2_0_set_fault_enable_default),gart_disable:Some(mmhub_v2_0_gart_disable),set_clockgating:Some(mmhub_v2_0_set_clockgating),get_clockgating:Some(mmhub_v2_0_get_clockgating),setup_vm_pt_regs:Some(mmhub_v2_0_setup_vm_pt_regs)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
