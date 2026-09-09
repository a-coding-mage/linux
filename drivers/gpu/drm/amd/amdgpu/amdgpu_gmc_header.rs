/* Translated from amdgpu_gmc.h. */

// Dependencies supplied by the surrounding kernel translation.

pub const AMDGPU_GMC_FAULT_RING_ORDER: usize = 8;
pub const AMDGPU_GMC_FAULT_RING_SIZE: usize = 1 << AMDGPU_GMC_FAULT_RING_ORDER;
pub const AMDGPU_GMC_FAULT_HASH_ORDER: usize = 8;
pub const AMDGPU_GMC_FAULT_HASH_SIZE: usize = 1 << AMDGPU_GMC_FAULT_HASH_ORDER;
pub const AMDGPU_GMC_FAULT_TIMEOUT: u64 = 5000;
pub const AMDGPU_GMC_XNACK_FLAG_CHAIN: u32 = 1 << 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_memory_partition {
    UNKNOWN_MEMORY_PARTITION_MODE = 0,
    AMDGPU_NPS1_PARTITION_MODE = 1,
    AMDGPU_NPS2_PARTITION_MODE = 2,
    AMDGPU_NPS3_PARTITION_MODE = 3,
    AMDGPU_NPS4_PARTITION_MODE = 4,
    AMDGPU_NPS6_PARTITION_MODE = 6,
    AMDGPU_NPS8_PARTITION_MODE = 8,
}

pub const AMDGPU_ALL_NPS_MASK: u32 = (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 6) | (1 << 8);
pub const AMDGPU_GMC_INIT_RESET_NPS: u32 = 1 << 0;
pub const AMDGPU_MAX_MEM_RANGES: usize = 8;
pub const AMDGPU_GMC9_FAULT_SOURCE_DATA_RETRY: u32 = 0x80;
pub const AMDGPU_GMC9_FAULT_SOURCE_DATA_READ: u32 = 0x40;
pub const AMDGPU_GMC9_FAULT_SOURCE_DATA_WRITE: u32 = 0x20;
pub const AMDGPU_GMC9_FAULT_SOURCE_DATA_EXE: u32 = 0x10;
pub const AMDGPU_GMC121_FAULT_SOURCE_DATA_READ: u32 = 0x400000;
pub const AMDGPU_GMC121_FAULT_SOURCE_DATA_WRITE: u32 = 0x200000;
pub const AMDGPU_GMC121_FAULT_SOURCE_DATA_EXE: u32 = 0x100000;

#[repr(C)] pub struct amdgpu_gmc_fault { pub timestamp: u64, pub next: u64, pub key: atomic64_t, pub timestamp_expiry: u64 }
#[repr(C)] pub struct amdgpu_vmhub_funcs {
    pub print_l2_protection_fault_status: Option<unsafe extern "C" fn(*mut amdgpu_device, u32)>,
    pub get_invalidate_req: Option<unsafe extern "C" fn(u32, u32) -> u32>,
}
#[repr(C)] pub struct amdgpu_vmhub {
    pub ctx0_ptb_addr_lo32:u32, pub ctx0_ptb_addr_hi32:u32, pub vm_inv_eng0_sem:u32, pub vm_inv_eng0_req:u32, pub vm_inv_eng0_ack:u32, pub vm_context0_cntl:u32, pub vm_l2_pro_fault_status:u32, pub vm_l2_pro_fault_cntl:u32,
    pub ctx_distance:u32, pub ctx_addr_distance:u32, pub eng_distance:u32, pub eng_addr_distance:u32,
    pub vm_cntx_cntl:u32, pub vm_cntx_cntl_vm_fault:u32, pub vm_l2_bank_select_reserved_cid2:u32, pub vm_contexts_disable:u32,
    pub sdma_invalidation_workaround: bool, pub vmhub_funcs:*const amdgpu_vmhub_funcs,
}
#[repr(C)] pub struct amdgpu_gmc_funcs {
    pub flush_gpu_tlb: Option<unsafe extern "C" fn(*mut amdgpu_device,u32,u32,u32)>,
    pub flush_gpu_tlb_pasid: Option<unsafe extern "C" fn(*mut amdgpu_device,u16,u32,bool,u32)>,
    pub emit_flush_gpu_tlb: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32,u64)->u64>,
    pub emit_pasid_mapping: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32,u32)>,
    pub set_prt: Option<unsafe extern "C" fn(*mut amdgpu_device,bool)>,
    pub get_vm_pde: Option<unsafe extern "C" fn(*mut amdgpu_device,i32,*mut u64,*mut u64)>,
    pub get_vm_pte: Option<unsafe extern "C" fn(*mut amdgpu_device,*mut amdgpu_vm,*mut amdgpu_bo,u32,*mut u64)>,
    pub override_vm_pte_flags: Option<unsafe extern "C" fn(*mut amdgpu_device,*mut amdgpu_vm,u64,*mut u64)>,
    pub get_vbios_fb_size: Option<unsafe extern "C" fn(*mut amdgpu_device)->u32>,
    pub get_dcc_alignment: Option<unsafe extern "C" fn(*mut amdgpu_device)->u32>,
    pub query_mem_partition_mode: Option<unsafe extern "C" fn(*mut amdgpu_device)->amdgpu_memory_partition>,
    pub request_mem_partition_mode: Option<unsafe extern "C" fn(*mut amdgpu_device,i32)->i32>,
    pub need_reset_on_init: Option<unsafe extern "C" fn(*mut amdgpu_device)->bool>,
}
#[repr(C)] pub union amdgpu_mem_partition_info_data { pub range: amdgpu_mem_partition_range, pub numa: amdgpu_mem_partition_numa }
#[repr(C)] #[derive(Copy,Clone)] pub struct amdgpu_mem_partition_range { pub fpfn:u32, pub lpfn:u32 }
#[repr(C)] #[derive(Copy,Clone)] pub struct amdgpu_mem_partition_numa { pub node:i32 }
#[repr(C)] pub struct amdgpu_mem_partition_info { pub data: amdgpu_mem_partition_info_data, pub size:u64 }
pub const INVALID_PFN: i32 = -1;
#[repr(C)] pub struct amdgpu_gmc_memrange { pub base_address:u64, pub limit_address:u64, pub flags:u32, pub nid_mask:i32 }
#[repr(C)] #[derive(Copy,Clone)] pub enum amdgpu_gart_placement { AMDGPU_GART_PLACEMENT_BEST_FIT=0, AMDGPU_GART_PLACEMENT_HIGH, AMDGPU_GART_PLACEMENT_LOW }

// The full amdgpu_gmc object is represented with its source fields; external kernel types remain dependencies.
#[repr(C)] pub struct amdgpu_gmc {
    pub aper_size: resource_size_t, pub aper_base: resource_size_t, pub mc_vram_size:u64, pub visible_vram_size:u64,
    pub agp_size:u64, pub agp_start:u64, pub agp_end:u64, pub gart_size:u64, pub gart_start:u64, pub gart_end:u64,
    pub vram_start:u64, pub vram_end:u64, pub fb_start:u64, pub fb_end:u64, pub vram_width:u32, pub real_vram_size:u64, pub vram_mtrr:i32, pub mc_mask:u64, pub pte_addr_mask:u64,
    pub fw:*const firmware, pub fw_version:u32, pub vm_fault:amdgpu_irq_src, pub vram_type:u32, pub vram_vendor:u8, pub prt_warning:bool, pub sdpif_register:u32,
    pub shared_aperture_start:u64, pub shared_aperture_end:u64, pub private_aperture_start:u64, pub private_aperture_end:u64, pub invalidate_lock:spinlock_t, pub translate_further:bool, pub vm_fault_info:*mut kfd_vm_fault_info, pub vm_fault_info_updated:atomic_t,
    pub fault_ring:[amdgpu_gmc_fault; AMDGPU_GMC_FAULT_RING_SIZE], pub fault_hash:[u64; AMDGPU_GMC_FAULT_HASH_SIZE], pub last_fault:u64, pub tmz_enabled:bool, pub is_app_apu:bool,
    pub mem_partitions:*mut amdgpu_mem_partition_info, pub num_mem_partitions:u8, pub gmc_funcs:*const amdgpu_gmc_funcs, pub requested_nps_mode:amdgpu_memory_partition, pub supported_nps_modes:u32, pub reset_flags:u32,
    pub xgmi:amdgpu_xgmi, pub ecc_irq:amdgpu_irq_src, pub noretry:i32, pub xnack_flags:u32, pub vmid0_page_table_block_size:u32, pub vmid0_page_table_depth:u32, pub pdb0_bo:*mut amdgpu_bo, pub ptr_pdb0:*mut core::ffi::c_void,
    pub mall_size:u64, pub m_half_use:u32, pub num_umc:i32, pub VM_L2_CNTL:u64, pub VM_L2_CNTL2:u64, pub VM_DUMMY_PAGE_FAULT_CNTL:u64, pub VM_DUMMY_PAGE_FAULT_ADDR_LO32:u64, pub VM_DUMMY_PAGE_FAULT_ADDR_HI32:u64,
    pub VM_L2_PROTECTION_FAULT_CNTL:u64, pub VM_L2_PROTECTION_FAULT_CNTL2:u64, pub VM_L2_PROTECTION_FAULT_MM_CNTL3:u64, pub VM_L2_PROTECTION_FAULT_MM_CNTL4:u64, pub VM_L2_PROTECTION_FAULT_ADDR_LO32:u64, pub VM_L2_PROTECTION_FAULT_ADDR_HI32:u64, pub VM_DEBUG:u64, pub VM_L2_MM_GROUP_RT_CLASSES:u64, pub VM_L2_BANK_SELECT_RESERVED_CID:u64, pub VM_L2_BANK_SELECT_RESERVED_CID2:u64, pub VM_L2_CACHE_PARITY_CNTL:u64, pub VM_L2_IH_LOG_CNTL:u64,
    pub VM_CONTEXT_CNTL:[u64;16], pub VM_CONTEXT_PAGE_TABLE_BASE_ADDR_LO32:[u64;16], pub VM_CONTEXT_PAGE_TABLE_BASE_ADDR_HI32:[u64;16], pub VM_CONTEXT_PAGE_TABLE_START_ADDR_LO32:[u64;16], pub VM_CONTEXT_PAGE_TABLE_START_ADDR_HI32:[u64;16], pub VM_CONTEXT_PAGE_TABLE_END_ADDR_LO32:[u64;16], pub VM_CONTEXT_PAGE_TABLE_END_ADDR_HI32:[u64;16], pub MC_VM_MX_L1_TLB_CNTL:u64, pub noretry_flags:u64, pub init_pte_flags:u64, pub flush_tlb_needs_extra_type_0:bool, pub flush_tlb_needs_extra_type_2:bool, pub flush_pasid_uses_kiq:bool, pub override_pte:bool,
}

pub unsafe fn amdgpu_gmc_vram_full_visible(gmc:*const amdgpu_gmc)->bool { WARN_ON((*gmc).real_vram_size < (*gmc).visible_vram_size); (*gmc).real_vram_size == (*gmc).visible_vram_size }
pub unsafe fn amdgpu_gmc_sign_extend(addr:u64, max_level:u32)->u64 { let start=if max_level==4 {0x0100000000000000} else {0x0000800000000000}; if addr>=start { addr | if max_level==4 {0xff00000000000000} else {0xffff800000000000} } else {addr} }

extern "C" { fn WARN_ON(condition: bool) -> bool; }
extern "C" { pub fn amdgpu_gmc_is_pdb0_enabled(*mut amdgpu_device)->bool; pub fn amdgpu_gmc_pdb0_alloc(*mut amdgpu_device)->i32; pub fn amdgpu_gmc_get_pde_for_bo(*mut amdgpu_bo,i32,*mut u64,*mut u64); pub fn amdgpu_gmc_set_pte_pde(*mut amdgpu_device,*mut core::ffi::c_void,u32,u64,u64)->i32; pub fn amdgpu_gmc_pd_addr(*mut amdgpu_bo)->u64; pub fn amdgpu_gmc_agp_addr(*mut ttm_buffer_object)->u64; }
extern "C" {
    pub fn amdgpu_gmc_sysvm_location(*mut amdgpu_device,*mut amdgpu_gmc); pub fn amdgpu_gmc_vram_location(*mut amdgpu_device,*mut amdgpu_gmc,u64); pub fn amdgpu_gmc_gart_location(*mut amdgpu_device,*mut amdgpu_gmc,amdgpu_gart_placement); pub fn amdgpu_gmc_agp_location(*mut amdgpu_device,*mut amdgpu_gmc); pub fn amdgpu_gmc_set_agp_default(*mut amdgpu_device,*mut amdgpu_gmc);
    pub fn amdgpu_gmc_filter_faults(*mut amdgpu_device,*mut amdgpu_ih_ring,u64,u16,u64)->bool; pub fn amdgpu_gmc_filter_faults_remove(*mut amdgpu_device,u64,u16);
    pub fn amdgpu_gmc_handle_retry_fault(*mut amdgpu_device,*mut amdgpu_iv_entry,u64,u32,u32,bool)->i32; pub fn amdgpu_gmc_ras_sw_init(*mut amdgpu_device)->i32; pub fn amdgpu_gmc_allocate_vm_inv_eng(*mut amdgpu_device)->i32;
    pub fn amdgpu_gmc_flush_gpu_tlb(*mut amdgpu_device,u32,u32,u32); pub fn amdgpu_gmc_flush_gpu_tlb_pasid(*mut amdgpu_device,u16,u32,bool,u32)->i32; pub fn amdgpu_gmc_fw_reg_write_reg_wait(*mut amdgpu_device,u32,u32,u32,u32,u32);
    pub fn amdgpu_gmc_tmz_set(*mut amdgpu_device); pub fn amdgpu_gmc_noretry_set(*mut amdgpu_device); pub fn amdgpu_gmc_set_vm_fault_masks(*mut amdgpu_device,i32,bool); pub fn amdgpu_gmc_init_vga_resv_regions(*mut amdgpu_device); pub fn amdgpu_gmc_init_pdb0(*mut amdgpu_device); pub fn amdgpu_gmc_vram_mc2pa(*mut amdgpu_device,u64)->u64; pub fn amdgpu_gmc_vram_pa(*mut amdgpu_device,*mut amdgpu_bo)->u64; pub fn amdgpu_gmc_vram_checking(*mut amdgpu_device)->i32; pub fn amdgpu_gmc_sysfs_init(*mut amdgpu_device)->i32; pub fn amdgpu_gmc_sysfs_fini(*mut amdgpu_device);
    pub fn amdgpu_gmc_get_nps_memranges(*mut amdgpu_device,*mut amdgpu_mem_partition_info,*mut u8)->i32; pub fn amdgpu_gmc_request_memory_partition(*mut amdgpu_device,i32)->i32; pub fn amdgpu_gmc_prepare_nps_mode_change(*mut amdgpu_device); pub fn amdgpu_gmc_need_reset_on_init(*mut amdgpu_device)->bool;
    pub fn amdgpu_gmc_get_vf_memory_partition(*mut amdgpu_device)->amdgpu_memory_partition; pub fn amdgpu_gmc_get_memory_partition(*mut amdgpu_device,*mut u32)->amdgpu_memory_partition; pub fn amdgpu_gmc_query_memory_partition(*mut amdgpu_device)->amdgpu_memory_partition; pub fn amdgpu_gmc_init_mem_ranges(*mut amdgpu_device)->i32; pub fn amdgpu_gmc_init_sw_mem_ranges(*mut amdgpu_device,*mut amdgpu_mem_partition_info); pub fn amdgpu_gmc_get_vram_info(*mut amdgpu_device,*mut i32,*mut i32,*mut i32)->i32; pub fn amdgpu_gmc_set_gart_size(*mut amdgpu_device,u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
