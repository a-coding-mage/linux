// Translated from amdgpu_gfx.h. Included C headers and their symbols are
// supplied by other translation units.

pub const AMDGPU_GFX_NORMAL_MODE: u32 = 0x00000000;
pub const AMDGPU_GFX_SAFE_MODE: u32 = 0x00000001;
pub const AMDGPU_GFX_PG_DISABLED_MODE: u32 = 0x00000002;
pub const AMDGPU_GFX_CG_DISABLED_MODE: u32 = 0x00000004;
pub const AMDGPU_GFX_LBPW_DISABLED_MODE: u32 = 0x00000008;
pub const AMDGPU_MAX_GC_INSTANCES: usize = 8;
pub const AMDGPU_MAX_QUEUES: usize = 128;
pub const AMDGPU_MAX_GFX_QUEUES: usize = AMDGPU_MAX_QUEUES;
pub const AMDGPU_MAX_COMPUTE_QUEUES: usize = AMDGPU_MAX_QUEUES;
pub const AMDGPU_GFX_QUEUE_PRIORITY_MINIMUM: i32 = 0;
pub const AMDGPU_GFX_QUEUE_PRIORITY_MAXIMUM: i32 = 15;
pub const AMDGPU_GFX_MAX_SE: usize = 4;
pub const AMDGPU_GFX_MAX_SH_PER_SE: usize = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_gfx_pipe_priority { AMDGPU_GFX_PIPE_PRIO_NORMAL = AMDGPU_RING_PRIO_1 as isize, AMDGPU_GFX_PIPE_PRIO_HIGH = AMDGPU_RING_PRIO_2 as isize }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_gfx_partition { AMDGPU_SPX_PARTITION_MODE=0, AMDGPU_DPX_PARTITION_MODE=1, AMDGPU_TPX_PARTITION_MODE=2, AMDGPU_QPX_PARTITION_MODE=3, AMDGPU_CPX_PARTITION_MODE=4, AMDGPU_UNKNOWN_COMPUTE_PARTITION_MODE=-1, AMDGPU_AUTO_COMPUTE_PARTITION_MODE=-2 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_gfx_partition_mem_alloc_mode { AMDGPU_PARTITION_MEM_CAPPING_EVEN=0, AMDGPU_PARTITION_MEM_ALLOC_ALL=1 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_gfx_ras_mem_id_type { AMDGPU_GFX_CP_MEM=0, AMDGPU_GFX_GCEA_MEM, AMDGPU_GFX_GC_CANE_MEM, AMDGPU_GFX_GCUTCL2_MEM, AMDGPU_GFX_GDS_MEM, AMDGPU_GFX_LDS_MEM, AMDGPU_GFX_RLC_MEM, AMDGPU_GFX_SP_MEM, AMDGPU_GFX_SPI_MEM, AMDGPU_GFX_SQC_MEM, AMDGPU_GFX_SQ_MEM, AMDGPU_GFX_TA_MEM, AMDGPU_GFX_TCC_MEM, AMDGPU_GFX_TCA_MEM, AMDGPU_GFX_TCI_MEM, AMDGPU_GFX_TCP_MEM, AMDGPU_GFX_TD_MEM, AMDGPU_GFX_TCX_MEM, AMDGPU_GFX_ATC_L2_MEM, AMDGPU_GFX_UTCL2_MEM, AMDGPU_GFX_VML2_MEM, AMDGPU_GFX_VML2_WALKER_MEM, AMDGPU_GFX_MEM_TYPE_NUM }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_unmap_queues_action { PREEMPT_QUEUES=0, RESET_QUEUES, DISABLE_PROCESS_QUEUES, PREEMPT_QUEUES_NO_UNMAP }

#[repr(C)]
pub struct amdgpu_mec { pub hpd_eop_obj:*mut amdgpu_bo, pub hpd_eop_gpu_addr:u64, pub mec_fw_obj:*mut amdgpu_bo, pub mec_fw_gpu_addr:u64, pub mec_fw_data_obj:*mut amdgpu_bo, pub mec_fw_data_gpu_addr:u64, pub num_mec:u32, pub num_pipe_per_mec:u32, pub num_queue_per_pipe:u32, pub mqd_backup:[*mut core::ffi::c_void; AMDGPU_MAX_COMPUTE_RINGS * AMDGPU_MAX_GC_INSTANCES], pub use_mmio_for_reset:bool, pub mes_hung_db_array:*mut u32, pub reset_mutex:mutex }
#[repr(C)] pub struct amdgpu_mec_bitmap { pub queue_bitmap:[u64;2] }
#[repr(C)] pub struct kiq_pm4_funcs { pub kiq_set_resources:Option<unsafe extern "C" fn(*mut amdgpu_ring,u64)>, pub kiq_map_queues:Option<unsafe extern "C" fn(*mut amdgpu_ring,*mut amdgpu_ring)>, pub kiq_unmap_queues:Option<unsafe extern "C" fn(*mut amdgpu_ring,*mut amdgpu_ring,amdgpu_unmap_queues_action,u64,u64)>, pub kiq_query_status:Option<unsafe extern "C" fn(*mut amdgpu_ring,*mut amdgpu_ring,u64,u64)>, pub kiq_invalidate_tlbs:Option<unsafe extern "C" fn(*mut amdgpu_ring,u16,u32,bool)>, pub kiq_reset_hw_queue:Option<unsafe extern "C" fn(*mut amdgpu_ring,u32,u32,u32,u32,u32,u32)>, pub set_resources_size:i32, pub map_queues_size:i32, pub unmap_queues_size:i32, pub query_status_size:i32, pub invalidate_tlbs_size:i32 }
#[repr(C)] pub struct amdgpu_kiq { pub eop_gpu_addr:u64, pub eop_obj:*mut amdgpu_bo, pub ring_lock:spinlock_t, pub ring:amdgpu_ring, pub irq:amdgpu_irq_src, pub pmf:*const kiq_pm4_funcs, pub mqd_backup:*mut core::ffi::c_void }

#[repr(C)] pub struct amdgpu_rb_config { pub rb_backend_disable:u32, pub user_rb_backend_disable:u32, pub raster_config:u32, pub raster_config_1:u32 }
#[repr(C)] pub struct gb_addr_config { pub pipe_interleave_size:u16, pub num_pipes:u8, pub max_compress_frags:u8, pub num_banks:u8, pub num_se:u8, pub num_rb_per_se:u8, pub num_pkrs:u8 }
#[repr(C)] pub struct amdgpu_gfx_config { pub max_shader_engines:u32, pub max_tile_pipes:u32, pub max_cu_per_sh:u32, pub max_sh_per_se:u32, pub max_backends_per_se:u32, pub max_texture_channel_caches:u32, pub max_gprs:u32, pub max_gs_threads:u32, pub max_hw_contexts:u32, pub sc_prim_fifo_size_frontend:u32, pub sc_prim_fifo_size_backend:u32, pub sc_hiz_tile_fifo_size:u32, pub sc_earlyz_tile_fifo_size:u32, pub num_tile_pipes:u32, pub backend_enable_mask:u32, pub mem_max_burst_length_bytes:u32, pub mem_row_size_in_kb:u32, pub shader_engine_tile_size:u32, pub num_gpus:u32, pub multi_gpu_tile_size:u32, pub mc_arb_ramcfg:u32, pub num_banks:u32, pub num_ranks:u32, pub gb_addr_config:u32, pub num_rbs:u32, pub gs_vgt_table_depth:u32, pub gs_prim_buffer_depth:u32, pub tile_mode_array:[u32;32], pub macrotile_mode_array:[u32;16], pub gb_addr_config_fields:gb_addr_config, pub rb_config:[[amdgpu_rb_config;AMDGPU_GFX_MAX_SH_PER_SE];AMDGPU_GFX_MAX_SE], pub double_offchip_lds_buf:u32, pub db_debug2:u32, pub num_sc_per_sh:u32, pub num_packer_per_sc:u32, pub pa_sc_tile_steering_override:u32, pub ta_cntl2_truncate_coord_mode:bool, pub tcc_disabled_mask:u64, pub gc_num_tcp_per_sa:u32, pub gc_num_sdp_interface:u32, pub gc_num_tcps:u32, pub gc_num_tcp_per_wpg:u32, pub gc_tcp_l1_size:u32, pub gc_num_sqc_per_wgp:u32, pub gc_l1_instruction_cache_size_per_sqc:u32, pub gc_l1_data_cache_size_per_sqc:u32, pub gc_gl1c_per_sa:u32, pub gc_gl1c_size_per_instance:u32, pub gc_gl2c_per_gpu:u32, pub gc_tcp_size_per_cu:u32, pub gc_num_cu_per_sqc:u32, pub gc_tcc_size:u32, pub gc_tcp_cache_line_size:u32, pub gc_instruction_cache_size_per_sqc:u32, pub gc_instruction_cache_line_size:u32, pub gc_scalar_data_cache_size_per_sqc:u32, pub gc_scalar_data_cache_line_size:u32, pub gc_tcc_cache_line_size:u32 }
#[repr(C)] pub struct amdgpu_cu_info { pub simd_per_cu:u32, pub max_waves_per_simd:u32, pub wave_front_size:u32, pub max_scratch_slots_per_cu:u32, pub lds_size:u32, pub number:u32, pub ao_cu_mask:u32, pub ao_cu_bitmap:[[u32;4];4], pub bitmap:[[[u32;4];4];AMDGPU_MAX_GC_INSTANCES] }
#[repr(C)] pub struct amdgpu_gfx_shadow_info { pub shadow_size:u32, pub shadow_alignment:u32, pub csa_size:u32, pub csa_alignment:u32, pub eop_size:u32, pub eop_alignment:u32 }
#[repr(C)] pub struct amdgpu_gfx_ras { pub ras_block:amdgpu_ras_block_object, pub enable_watchdog_timer:Option<unsafe extern "C" fn(*mut amdgpu_device)>, pub rlc_gc_fed_irq:Option<unsafe extern "C" fn(*mut amdgpu_device,*mut amdgpu_irq_src,*mut amdgpu_iv_entry)->i32>, pub poison_consumption_handler:Option<unsafe extern "C" fn(*mut amdgpu_device,*mut amdgpu_iv_entry)->i32> }

#[repr(C)] pub struct amdgpu_gfx_funcs { pub get_gpu_clock_counter:Option<unsafe extern "C" fn(*mut amdgpu_device)->u64>, pub select_se_sh:Option<unsafe extern "C" fn(*mut amdgpu_device,u32,u32,u32,i32)>, pub read_wave_data:Option<unsafe extern "C" fn(*mut amdgpu_device,u32,u32,u32,*mut u32,*mut i32)>, pub read_wave_vgprs:Option<unsafe extern "C" fn(*mut amdgpu_device,u32,u32,u32,u32,u32,u32,*mut u32)>, pub read_wave_sgprs:Option<unsafe extern "C" fn(*mut amdgpu_device,u32,u32,u32,u32,u32,*mut u32)>, pub select_me_pipe_q:Option<unsafe extern "C" fn(*mut amdgpu_device,u32,u32,u32,u32,u32)>, pub init_spm_golden:Option<unsafe extern "C" fn(*mut amdgpu_device)>, pub update_perfmon_mgcg:Option<unsafe extern "C" fn(*mut amdgpu_device,bool)>, pub get_gfx_shadow_info:Option<unsafe extern "C" fn(*mut amdgpu_device,*mut amdgpu_gfx_shadow_info,bool)->i32>, pub query_partition_mode:Option<unsafe extern "C" fn(*mut amdgpu_device)->amdgpu_gfx_partition>, pub switch_partition_mode:Option<unsafe extern "C" fn(*mut amdgpu_device,i32)->i32>, pub ih_node_to_logical_xcc:Option<unsafe extern "C" fn(*mut amdgpu_device,i32)->i32>, pub get_xccs_per_xcp:Option<unsafe extern "C" fn(*mut amdgpu_device)->i32>, pub get_hdp_flush_mask:Option<unsafe extern "C" fn(*mut amdgpu_ring,*mut u32,*mut u32)> }
#[repr(C)] pub struct sq_work { pub work:work_struct, pub ih_data:u32 }
#[repr(C)] pub struct amdgpu_pfp { pub pfp_fw_obj:*mut amdgpu_bo, pub pfp_fw_gpu_addr:u64, pub pfp_fw_ptr:*mut u32, pub pfp_fw_data_obj:*mut amdgpu_bo, pub pfp_fw_data_gpu_addr:u64, pub pfp_fw_data_ptr:*mut u32 }
#[repr(C)] pub struct amdgpu_ce { pub ce_fw_obj:*mut amdgpu_bo, pub ce_fw_gpu_addr:u64, pub ce_fw_ptr:*mut u32 }
#[repr(C)] pub struct amdgpu_me { pub me_fw_obj:*mut amdgpu_bo, pub me_fw_gpu_addr:u64, pub me_fw_ptr:*mut u32, pub me_fw_data_obj:*mut amdgpu_bo, pub me_fw_data_gpu_addr:u64, pub me_fw_data_ptr:*mut u32, pub num_me:u32, pub num_pipe_per_me:u32, pub num_queue_per_pipe:u32, pub mqd_backup:[*mut core::ffi::c_void;AMDGPU_MAX_GFX_RINGS], pub use_mmio_for_reset:bool, pub queue_bitmap:[u64;2] }
#[repr(C)] pub struct amdgpu_isolation_work { pub adev:*mut amdgpu_device, pub xcp_id:u32, pub work:delayed_work }
#[repr(C)] pub struct amdgpu_gfx_deferred_entry { pub ring:*mut amdgpu_ring, pub fence:*mut amdgpu_fence }
#[repr(C)] pub struct amdgpu_gfx_ras_reg_entry { pub reg_entry:amdgpu_ras_err_status_reg_entry, pub mem_id_type:amdgpu_gfx_ras_mem_id_type, pub se_num:u32 }
#[repr(C)] pub struct amdgpu_gfx_ras_mem_id_entry { pub mem_id_ent:*const amdgpu_ras_memory_id_entry, pub size:u32 }

#[inline] pub unsafe fn amdgpu_gfx_compute_mode_desc(mode:i32)->&'static str { match mode { 0=>"SPX", 1=>"DPX", 2=>"TPX", 3=>"QPX", 4=>"CPX", _=>"UNKNOWN" } }
#[inline] pub unsafe fn amdgpu_gfx_compute_mem_alloc_mode_desc(mode:i32)->&'static str { match mode { 0=>"CAPPING", 1=>"ALL", _=>"UNKNOWN" } }

#[inline] pub fn amdgpu_gfx_create_bitmask(bit_width:u32)->u32 { ((1u64.wrapping_shl(bit_width))-1) as u32 }
pub const AMDGPU_GFX_MAX_SE_COUNT: usize = AMDGPU_GFX_MAX_SE;

extern "C" {
    pub fn amdgpu_gfx_parse_disable_cu(adev:*mut amdgpu_device, mask:*mut u32, max_se:u32, max_sh:u32);
    pub fn amdgpu_gfx_kiq_init_ring(adev:*mut amdgpu_device, xcc_id:i32)->i32;
    pub fn amdgpu_gfx_kiq_free_ring(ring:*mut amdgpu_ring);
    pub fn amdgpu_gfx_kiq_fini(adev:*mut amdgpu_device, xcc_id:i32);
    pub fn amdgpu_gfx_kiq_init(adev:*mut amdgpu_device, hpd_size:u32, xcc_id:i32)->i32;
    pub fn amdgpu_gfx_mqd_sw_init(adev:*mut amdgpu_device, mqd_size:u32, xcc_id:i32)->i32;
    pub fn amdgpu_gfx_mqd_sw_fini(adev:*mut amdgpu_device, xcc_id:i32);
    pub fn amdgpu_gfx_mqd_symmetrically_map_cu_mask(adev:*mut amdgpu_device, cu_mask:*const u32, cu_mask_count:u32, se_mask:*mut u32);
    pub fn amdgpu_gfx_disable_kcq(adev:*mut amdgpu_device,xcc_id:i32)->i32; pub fn amdgpu_gfx_enable_kcq(adev:*mut amdgpu_device,xcc_id:i32)->i32; pub fn amdgpu_gfx_disable_kgq(adev:*mut amdgpu_device,xcc_id:i32)->i32; pub fn amdgpu_gfx_enable_kgq(adev:*mut amdgpu_device,xcc_id:i32)->i32;
    pub fn amdgpu_gfx_compute_queue_acquire(adev:*mut amdgpu_device); pub fn amdgpu_gfx_graphics_queue_acquire(adev:*mut amdgpu_device);
    pub fn amdgpu_gfx_mec_queue_to_bit(adev:*mut amdgpu_device,mec:i32,pipe:i32,queue:i32)->i32;
    pub fn amdgpu_queue_mask_bit_to_mec_queue(adev:*mut amdgpu_device,bit:i32,mec:*mut i32,pipe:*mut i32,queue:*mut i32);
    pub fn amdgpu_gfx_is_mec_queue_enabled(adev:*mut amdgpu_device,xcc_id:i32,mec:i32,pipe:i32,queue:i32)->bool;
    pub fn amdgpu_gfx_is_high_priority_compute_queue(adev:*mut amdgpu_device,ring:*mut amdgpu_ring)->bool; pub fn amdgpu_gfx_is_high_priority_graphics_queue(adev:*mut amdgpu_device,ring:*mut amdgpu_ring)->bool;
    pub fn amdgpu_gfx_is_me_queue_enabled(adev:*mut amdgpu_device,me:i32,pipe:i32,queue:i32)->bool;
    pub fn amdgpu_gfx_off_ctrl(adev:*mut amdgpu_device,enable:bool); pub fn amdgpu_gfx_off_ctrl_immediate(adev:*mut amdgpu_device,enable:bool);
    pub fn amdgpu_get_gfx_off_status(adev:*mut amdgpu_device,value:*mut u32)->i32; pub fn amdgpu_gfx_ras_late_init(adev:*mut amdgpu_device,ras_block:*mut ras_common_if)->i32; pub fn amdgpu_gfx_ras_suspend(adev:*mut amdgpu_device,ras_block:*mut ras_common_if); pub fn amdgpu_gfx_ras_fini(adev:*mut amdgpu_device,ras_block:*mut ras_common_if);
    pub fn amdgpu_get_gfx_off_entrycount(adev:*mut amdgpu_device,value:*mut u64)->i32; pub fn amdgpu_get_gfx_off_residency(adev:*mut amdgpu_device,residency:*mut u32)->i32; pub fn amdgpu_set_gfx_off_residency(adev:*mut amdgpu_device,value:bool)->i32;
    pub fn amdgpu_kiq_rreg(adev:*mut amdgpu_device,reg:u32,xcc_id:u32)->u32; pub fn amdgpu_kiq_wreg(adev:*mut amdgpu_device,reg:u32,v:u32,xcc_id:u32);
    pub fn amdgpu_gfx_get_num_kcq(adev:*mut amdgpu_device)->i32; pub fn amdgpu_gfx_cp_init_microcode(adev:*mut amdgpu_device,ucode_id:u32);
    pub fn amdgpu_gfx_ras_sw_init(adev:*mut amdgpu_device)->i32; pub fn amdgpu_gfx_poison_consumption_handler(adev:*mut amdgpu_device,entry:*mut amdgpu_iv_entry)->i32;
    pub fn amdgpu_gfx_is_master_xcc(adev:*mut amdgpu_device,xcc_id:i32)->bool; pub fn amdgpu_gfx_sysfs_init(adev:*mut amdgpu_device)->i32; pub fn amdgpu_gfx_sysfs_fini(adev:*mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
