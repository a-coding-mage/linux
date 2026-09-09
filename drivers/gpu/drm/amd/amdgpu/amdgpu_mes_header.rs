/* Translated from amdgpu_mes.h. External kernel types and functions are supplied by dependencies. */

pub const AMDGPU_MES_MAX_COMPUTE_PIPES: usize = 8;
pub const AMDGPU_MES_MAX_GFX_PIPES: usize = 2;
pub const AMDGPU_MES_MAX_SDMA_PIPES: usize = 2;
pub const AMDGPU_MES_API_VERSION_SHIFT: u32 = 12;
pub const AMDGPU_MES_FEAT_VERSION_SHIFT: u32 = 24;
pub const AMDGPU_MES_VERSION_MASK: u32 = 0x00000fff;
pub const AMDGPU_MES_API_VERSION_MASK: u32 = 0x00fff000;
pub const AMDGPU_MES_FEAT_VERSION_MASK: u32 = 0xff000000;
pub const AMDGPU_MES_MSCRATCH_SIZE: u32 = 0x40000;
pub const AMDGPU_MES_INVALID_DB_OFFSET: u32 = 0xffff_ffff;
pub const AMDGPU_MES_PROC_CTX_SIZE: usize = 0x1000;
pub const AMDGPU_MES_GANG_CTX_SIZE: usize = 0x1000;
pub const AMDGPU_MES_PROC_CTX_ARRAY_MAX: usize = 128;
pub const AMDGPU_MES_GANG_CTX_ARRAY_MAX: usize = 512;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_mes_priority_level { Low = 0, Normal = 1, Medium = 2, High = 3, Realtime = 4, NumLevels }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_mes_pipe { Pipe0 = 0, Pipe1 = 1, MaxMesPipes = 2 }
pub const AMDGPU_MES_SCHED_PIPE: amdgpu_mes_pipe = amdgpu_mes_pipe::Pipe0;
pub const AMDGPU_MES_KIQ_PIPE: amdgpu_mes_pipe = amdgpu_mes_pipe::Pipe1;
pub const AMDGPU_MAX_MES_INST_PIPES: usize = 2 * AMDGPU_MAX_GC_INSTANCES;

#[repr(C)]
pub struct amdgpu_mes {
    pub adev: *mut amdgpu_device, pub mutex_hidden: mutex, pub doorbell_ida: ida,
    pub queue_id_lock: spinlock_t, pub sched_version: u32, pub kiq_version: u32,
    pub fw_version: [u32; 2], pub enable_legacy_queue_map: bool, pub total_max_queue: u32,
    pub max_doorbell_slices: u32, pub default_process_quantum: u64, pub default_gang_quantum: u64,
    pub ring: [amdgpu_ring; AMDGPU_MAX_MES_INST_PIPES],
    pub ring_lock: [spinlock_t; AMDGPU_MAX_MES_INST_PIPES], pub fw: [*const firmware; 2],
    pub ucode_fw_obj: [*mut amdgpu_bo; AMDGPU_MAX_MES_INST_PIPES],
    pub ucode_fw_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES], pub ucode_fw_ptr: [*mut u32; AMDGPU_MAX_MES_INST_PIPES], pub uc_start_addr: [u64; 2],
    pub data_fw_obj: [*mut amdgpu_bo; AMDGPU_MAX_MES_INST_PIPES], pub data_fw_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES], pub data_fw_ptr: [*mut u32; AMDGPU_MAX_MES_INST_PIPES], pub data_start_addr: [u64; 2],
    pub eop_gpu_obj: [*mut amdgpu_bo; AMDGPU_MAX_MES_INST_PIPES], pub eop_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES],
    pub mqd_backup: [*mut core::ffi::c_void; AMDGPU_MAX_MES_INST_PIPES], pub irq: [amdgpu_irq_src; AMDGPU_MAX_MES_INST_PIPES],
    pub vmid_mask_gfxhub: u32, pub vmid_mask_mmhub: u32, pub gfx_hqd_mask: [u32; 2], pub compute_hqd_mask: [u32; 8], pub sdma_hqd_mask: [u32; 2], pub aggregated_doorbells: [u32; 5],
    pub sch_ctx_offs: [u32; AMDGPU_MAX_MES_INST_PIPES], pub sch_ctx_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES], pub sch_ctx_ptr: [*mut u64; AMDGPU_MAX_MES_INST_PIPES],
    pub query_status_fence_offs: [u32; AMDGPU_MAX_MES_INST_PIPES], pub query_status_fence_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES], pub query_status_fence_ptr: [*mut u64; AMDGPU_MAX_MES_INST_PIPES],
    pub saved_flags: u32, pub kiq_hw_init: Option<unsafe extern "C" fn(*mut amdgpu_device, u32) -> i32>, pub kiq_hw_fini: Option<unsafe extern "C" fn(*mut amdgpu_device, u32) -> i32>,
    pub db_start_dw_offset: u32, pub num_mes_dbs: u32, pub doorbell_bitmap: *mut core::ffi::c_ulong, pub event_log_size: u32, pub event_log_gpu_obj: *mut amdgpu_bo, pub event_log_gpu_addr: u64, pub event_log_cpu_addr: *mut core::ffi::c_void,
    pub funcs: *const amdgpu_mes_funcs, pub resource_1: [*mut amdgpu_bo; 2], pub resource_1_gpu_addr: [u64; 2], pub resource_1_addr: [*mut core::ffi::c_void; 2],
    pub hung_queue_db_array_size: i32, pub hung_queue_hqd_info_offset: i32, pub hung_queue_db_array_gpu_obj: [*mut amdgpu_bo; AMDGPU_MAX_MES_INST_PIPES], pub hung_queue_db_array_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES], pub hung_queue_db_array_cpu_addr: [*mut core::ffi::c_void; AMDGPU_MAX_MES_INST_PIPES],
    pub enable_coop_mode: bool, pub master_xcc_ids: [i32; AMDGPU_MAX_MES_INST_PIPES], pub shared_cmd_buf_obj: [*mut amdgpu_bo; AMDGPU_MAX_MES_INST_PIPES], pub shared_cmd_buf_gpu_addr: [u64; AMDGPU_MAX_MES_INST_PIPES], pub compute_pipe_reset_enabled: bool, pub gfx_pipe_reset_enabled: bool, pub use_rs64mem: bool, pub ctx_array_size_bo: *mut amdgpu_bo, pub ctx_array_size_gpu_addr: u64, pub ctx_array_size_cpu_ptr: *mut u32, pub proc_ctx_array_size: u32, pub proc_ctx_bitmap: *mut core::ffi::c_ulong, pub gang_ctx_array_size: u32, pub gang_ctx_array_index: u32, pub gang_ctx_bitmap: *mut core::ffi::c_ulong,
}

#[repr(C)] pub struct amdgpu_mes_hung_queue_hqd_info { pub bit0_31: u32 }
#[repr(C)] pub struct amdgpu_mes_gang { pub gang_id:i32, pub priority:i32, pub inprocess_gang_priority:i32, pub global_priority_level:i32, pub list:list_head, pub process:*mut amdgpu_mes_process, pub gang_ctx_bo:*mut amdgpu_bo, pub gang_ctx_gpu_addr:u64, pub gang_ctx_cpu_ptr:*mut core::ffi::c_void, pub gang_quantum:u64, pub queue_list:list_head }
#[repr(C)] pub struct amdgpu_mes_queue { pub list:list_head, pub gang:*mut amdgpu_mes_gang, pub queue_id:i32, pub doorbell_off:u64, pub mqd_obj:*mut amdgpu_bo, pub mqd_cpu_ptr:*mut core::ffi::c_void, pub mqd_gpu_addr:u64, pub wptr_gpu_addr:u64, pub queue_type:i32, pub paging:i32, pub ring:*mut amdgpu_ring }
#[repr(C)] pub struct amdgpu_mes_queue_properties { pub queue_type:i32, pub hqd_base_gpu_addr:u64, pub rptr_gpu_addr:u64, pub wptr_gpu_addr:u64, pub wptr_mc_addr:u64, pub queue_size:u32, pub eop_gpu_addr:u64, pub hqd_pipe_priority:u32, pub hqd_queue_priority:u32, pub paging:bool, pub ring:*mut amdgpu_ring, pub doorbell_off:u64 }
#[repr(C)] pub struct amdgpu_mes_gang_properties { pub priority:u32, pub gang_quantum:u32, pub inprocess_gang_priority:u32, pub priority_level:u32, pub global_priority_level:i32 }

#[repr(C)] pub struct mes_add_queue_input { pub xcc_id:u32, pub process_id:u32, pub page_table_base_addr:u64, pub process_va_start:u64, pub process_va_end:u64, pub process_quantum:u64, pub process_context_addr:u64, pub gang_quantum:u64, pub gang_context_addr:u64, pub inprocess_gang_priority:u32, pub gang_global_priority_level:u32, pub doorbell_offset:u32, pub mqd_addr:u64, pub wptr_addr:u64, pub wptr_mc_addr:u64, pub queue_type:u32, pub paging:u32, pub gws_base:u32, pub gws_size:u32, pub tba_addr:u64, pub tma_addr:u64, pub trap_en:u32, pub skip_process_ctx_clear:u32, pub is_kfd_process:u32, pub is_aql_queue:u32, pub queue_size:u32, pub exclusively_scheduled:u32, pub sh_mem_config_data:u32, pub vm_cntx_cntl:u32, pub process_context_array_index:u32, pub gang_context_array_index:u32 }
#[repr(C)] pub struct mes_remove_queue_input { pub xcc_id:u32, pub doorbell_offset:u32, pub gang_context_addr:u64, pub queue_type:u32, pub remove_queue_after_reset:bool, pub gang_context_array_index:u32 }
#[repr(C)] pub struct mes_map_legacy_queue_input { pub xcc_id:u32, pub queue_type:u32, pub doorbell_offset:u32, pub pipe_id:u32, pub queue_id:u32, pub mqd_addr:u64, pub wptr_addr:u64 }
#[repr(C)] pub struct mes_unmap_legacy_queue_input { pub xcc_id:u32, pub action:amdgpu_unmap_queues_action, pub queue_type:u32, pub doorbell_offset:u32, pub pipe_id:u32, pub queue_id:u32, pub trail_fence_addr:u64, pub trail_fence_data:u64 }
#[repr(C)] pub struct mes_suspend_gang_input { pub xcc_id:u32, pub suspend_all_gangs:bool, pub suspend_all_sdma_gangs:bool, pub gang_context_addr:u64, pub suspend_fence_addr:u64, pub suspend_fence_value:u32, pub doorbell_offset:u32 }
#[repr(C)] pub struct mes_resume_gang_input { pub xcc_id:u32, pub resume_all_gangs:bool, pub gang_context_addr:u64, pub doorbell_offset:u32 }
#[repr(C)] pub struct mes_reset_queue_input { pub xcc_id:u32, pub queue_type:u32, pub doorbell_offset:u32, pub use_mmio:bool, pub me_id:u32, pub pipe_id:u32, pub queue_id:u32, pub mqd_addr:u64, pub wptr_addr:u64, pub vmid:u32, pub legacy_gfx:bool, pub is_kq:bool }
#[repr(C)] pub struct mes_detect_and_reset_queue_input { pub queue_type:u32, pub detect_only:bool, pub xcc_id:u32 }
#[repr(C)] pub struct mes_inv_tlbs_pasid_input { pub xcc_id:u32, pub pasid:u16, pub hub_id:u8, pub flush_type:u8 }

#[repr(C)] #[derive(Copy,Clone)] pub enum mes_misc_opcode { WriteReg, ReadReg, WrmRegWait, WrmRegWrWait, SetShaderDebugger, ChangeConfig }
#[repr(C)] pub struct mes_misc_op_input { pub xcc_id:u32, pub op:mes_misc_opcode, pub data:[u8; 64] }
#[repr(C)] pub struct amdgpu_mes_funcs { pub add_hw_queue:Option<unsafe extern "C" fn(*mut amdgpu_mes,*mut mes_add_queue_input)->i32>, pub remove_hw_queue:Option<unsafe extern "C" fn(*mut amdgpu_mes,*mut mes_remove_queue_input)->i32>, pub map_legacy_queue:Option<unsafe extern "C" fn(*mut amdgpu_mes,*mut mes_map_legacy_queue_input)->i32>, pub unmap_legacy_queue:Option<unsafe extern "C" fn(*mut amdgpu_mes,*mut mes_unmap_legacy_queue_input)->i32>, pub suspend_gang:Option<unsafe extern "C" fn(*mut amdgpu_mes,*mut mes_suspend_gang_input)->i32>, pub resume_gang:Option<unsafe extern "C" fn(*mut amdgpu_mes,*mut mes_resume_gang_input)->i32>, pub misc_op:Option<unsafe extern "C" fn(*mut amdgpu_mes,*mut mes_misc_op_input)->i32>, pub reset_hw_queue:Option<unsafe extern "C" fn(*mut amdgpu_mes,*mut mes_reset_queue_input)->i32>, pub detect_and_reset_hung_queues:Option<unsafe extern "C" fn(*mut amdgpu_mes,*mut mes_detect_and_reset_queue_input)->i32>, pub invalidate_tlbs_pasid:Option<unsafe extern "C" fn(*mut amdgpu_mes,*mut mes_inv_tlbs_pasid_input)->i32> }
#[repr(C)] pub struct amdgpu_mqd_prop { pub mqd_gpu_addr:u64,pub hqd_base_gpu_addr:u64,pub rptr_gpu_addr:u64,pub wptr_gpu_addr:u64,pub queue_size:u32,pub use_doorbell:bool,pub doorbell_index:u32,pub eop_gpu_addr:u64,pub hqd_pipe_priority:u32,pub hqd_queue_priority:u32,pub mqd_stride_size:u32,pub allow_tunneling:bool,pub hqd_active:bool,pub shadow_addr:u64,pub gds_bkup_addr:u64,pub csa_addr:u64,pub fence_address:u64,pub tmz_queue:bool,pub kernel_queue:bool,pub cu_mask:*mut u32,pub cu_mask_count:u32,pub cu_flags:u32,pub is_user_cu_masked:bool }
#[repr(C)] pub struct amdgpu_mqd { pub mqd_size:core::ffi::c_uint, pub init_mqd:Option<unsafe extern "C" fn(*mut amdgpu_device,*mut core::ffi::c_void,*mut amdgpu_mqd_prop)->i32> }
pub const AMDGPU_UPDATE_FLAG_DBG_WA_ENABLE:u32=1; pub const AMDGPU_UPDATE_FLAG_DBG_WA_DISABLE:u32=2; pub const AMDGPU_UPDATE_FLAG_IS_GWS:u32=4;
pub const fn amdgpu_mqd_size_align(size:usize)->usize { AMDGPU_GPU_PAGE_ALIGN(size+32) }
extern "C" { fn mutex_lock(*mut mutex); fn mutex_unlock(*mut mutex); fn memalloc_noreclaim_save()->u32; fn memalloc_noreclaim_restore(u32); }
#[inline] pub unsafe fn amdgpu_mes_lock(mes:*mut amdgpu_mes) { mutex_lock(core::ptr::addr_of_mut!((*mes).mutex_hidden)); (*mes).saved_flags=memalloc_noreclaim_save(); }
#[inline] pub unsafe fn amdgpu_mes_unlock(mes:*mut amdgpu_mes) { memalloc_noreclaim_restore((*mes).saved_flags); mutex_unlock(core::ptr::addr_of_mut!((*mes).mutex_hidden)); }

extern "C" { pub fn amdgpu_mes_init_microcode(*mut amdgpu_device,i32)->i32; pub fn amdgpu_mes_validate_fw_version(*mut amdgpu_device); pub fn amdgpu_mes_init(*mut amdgpu_device)->i32; pub fn amdgpu_mes_fini(*mut amdgpu_device); pub fn amdgpu_mes_suspend(*mut amdgpu_device,u32)->i32; pub fn amdgpu_mes_resume(*mut amdgpu_device,u32)->i32; }
extern "C" {
    pub fn amdgpu_mes_map_legacy_queue(*mut amdgpu_device,*mut amdgpu_ring,u32)->i32;
    pub fn amdgpu_mes_unmap_legacy_queue(*mut amdgpu_device,*mut amdgpu_ring,amdgpu_unmap_queues_action,u64,u64,u32)->i32;
    pub fn amdgpu_mes_reset_legacy_queue(*mut amdgpu_device,*mut amdgpu_ring,core::ffi::c_uint,bool,u32)->i32;
    pub fn amdgpu_mes_reset_queue_mmio(*mut amdgpu_device,i32,core::ffi::c_uint,core::ffi::c_uint,core::ffi::c_uint,core::ffi::c_uint,u32)->i32;
    pub fn amdgpu_mes_reset_user_queue(*mut amdgpu_device,i32,core::ffi::c_uint,u32)->i32;
    pub fn amdgpu_mes_get_hung_queue_db_array_size(*mut amdgpu_device)->i32;
    pub fn amdgpu_mes_detect_and_reset_hung_queues(*mut amdgpu_device,i32,bool,*mut core::ffi::c_uint,*mut u32,u32)->i32;
    pub fn amdgpu_mes_rreg(*mut amdgpu_device,u32,u32)->u32;
    pub fn amdgpu_mes_wreg(*mut amdgpu_device,u32,u32,u32)->i32;
    pub fn amdgpu_mes_reg_write_reg_wait(*mut amdgpu_device,u32,u32,u32,u32,u32)->i32;
    pub fn amdgpu_mes_hdp_flush(*mut amdgpu_device)->i32;
    pub fn amdgpu_mes_set_shader_debugger(*mut amdgpu_device,u64,u32,*const u32,u32,bool,u32)->i32;
    pub fn amdgpu_mes_flush_shader_debugger(*mut amdgpu_device,u64,u32)->i32;
    pub fn amdgpu_mes_get_aggregated_doorbell_index(*mut amdgpu_device,amdgpu_mes_priority_level)->u32;
    pub fn amdgpu_mes_doorbell_process_slice(*mut amdgpu_device)->i32;
    pub fn amdgpu_mes_suspend_resume_all_supported(*mut amdgpu_device)->bool;
    pub fn amdgpu_mes_queue_reset_by_mes_supported(*mut amdgpu_device)->bool;
    pub fn amdgpu_mes_update_enforce_isolation(*mut amdgpu_device)->i32;
    pub fn amdgpu_mes_rs64mem_init(*mut amdgpu_mes)->i32;
    pub fn amdgpu_mes_rs64mem_fini(*mut amdgpu_mes);
    pub fn amdgpu_mes_rs64mem_setup_bitmaps(*mut amdgpu_mes)->i32;
    pub fn amdgpu_mes_alloc_proc_ctx_index(*mut amdgpu_mes,*mut u32)->i32;
    pub fn amdgpu_mes_free_proc_ctx_index(*mut amdgpu_mes,u32);
    pub fn amdgpu_mes_alloc_gang_ctx_index(*mut amdgpu_mes,*mut u32)->i32;
    pub fn amdgpu_mes_free_gang_ctx_index(*mut amdgpu_mes,u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
