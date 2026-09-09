/* Translated from amdgpu_ring.h. */

// Dependencies supplied by the surrounding kernel translation are intentionally external.

pub const AMDGPU_MAX_RINGS: usize = 149;
pub const AMDGPU_MAX_HWIP_RINGS: usize = 64;
pub const AMDGPU_MAX_GFX_RINGS: usize = 2;
pub const AMDGPU_MAX_SW_GFX_RINGS: usize = 2;
pub const AMDGPU_MAX_COMPUTE_RINGS: usize = 8;
pub const AMDGPU_MAX_VCE_RINGS: usize = 3;
pub const AMDGPU_MAX_UVD_ENC_RINGS: usize = 2;
pub const AMDGPU_MAX_VPE_RINGS: usize = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_ring_priority_level { AMDGPU_RING_PRIO_0, AMDGPU_RING_PRIO_1, AMDGPU_RING_PRIO_DEFAULT = 1, AMDGPU_RING_PRIO_2, AMDGPU_RING_PRIO_MAX }

pub const AMDGPU_FENCE_OWNER_UNDEFINED: usize = 0;
pub const AMDGPU_FENCE_OWNER_VM: usize = 1;
pub const AMDGPU_FENCE_OWNER_KFD: usize = 2;
pub const AMDGPU_FENCE_FLAG_64BIT: u32 = 1 << 0;
pub const AMDGPU_FENCE_FLAG_INT: u32 = 1 << 1;
pub const AMDGPU_FENCE_FLAG_TC_WB_ONLY: u32 = 1 << 2;
pub const AMDGPU_FENCE_FLAG_EXEC: u32 = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_ring_type {
    AMDGPU_RING_TYPE_GFX = AMDGPU_HW_IP_GFX, AMDGPU_RING_TYPE_COMPUTE = AMDGPU_HW_IP_COMPUTE,
    AMDGPU_RING_TYPE_SDMA = AMDGPU_HW_IP_DMA, AMDGPU_RING_TYPE_UVD = AMDGPU_HW_IP_UVD,
    AMDGPU_RING_TYPE_VCE = AMDGPU_HW_IP_VCE, AMDGPU_RING_TYPE_UVD_ENC = AMDGPU_HW_IP_UVD_ENC,
    AMDGPU_RING_TYPE_VCN_DEC = AMDGPU_HW_IP_VCN_DEC, AMDGPU_RING_TYPE_VCN_ENC = AMDGPU_HW_IP_VCN_ENC,
    AMDGPU_RING_TYPE_VCN_JPEG = AMDGPU_HW_IP_VCN_JPEG, AMDGPU_RING_TYPE_VPE = AMDGPU_HW_IP_VPE,
    AMDGPU_RING_TYPE_KIQ, AMDGPU_RING_TYPE_MES, AMDGPU_RING_TYPE_UMSCH_MM,
    AMDGPU_RING_TYPE_CPER, AMDGPU_RING_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum amdgpu_ib_pool_type { AMDGPU_IB_POOL_DELAYED, AMDGPU_IB_POOL_IMMEDIATE, AMDGPU_IB_POOL_DIRECT, AMDGPU_IB_POOL_MAX }

#[repr(C)]
pub struct amdgpu_ib { pub sa_bo: *mut drm_suballoc, pub length_dw: u32, pub gpu_addr: u64, pub ptr: *mut u32, pub flags: u32 }

#[repr(C)]
pub struct amdgpu_sched { pub num_scheds: u32, pub sched: [*mut drm_gpu_scheduler; AMDGPU_MAX_HWIP_RINGS] }

#[repr(C)]
pub struct amdgpu_fence_driver {
    pub gpu_addr: u64, pub cpu_addr: *mut u32, pub sync_seq: u32, pub last_seq: atomic_t,
    pub initialized: bool, pub irq_src: *mut amdgpu_irq_src, pub irq_type: u32,
    pub fallback_timer: timer_list, pub num_fences_mask: u32, pub lock: spinlock_t,
    pub fences: *mut *mut dma_fence,
}

#[repr(C)]
pub struct amdgpu_fence {
    pub base: dma_fence, pub ring: *mut amdgpu_ring, pub start_timestamp: ktime_t,
    pub ib_wptr: u64, pub ib_dw_size: u32, pub skip_ib_dw_start_offset: u32,
    pub skip_ib_dw_end_offset: u32, pub context: u64, pub backup_idx: u32,
}

#[repr(C)]
pub struct amdgpu_ring_funcs {
    pub type_: amdgpu_ring_type, pub align_mask: u32, pub nop: u32,
    pub support_64bit_ptrs: bool, pub no_user_fence: bool, pub secure_submission_supported: bool,
    pub extra_bytes: u32,
    pub get_rptr: Option<unsafe extern "C" fn(*mut amdgpu_ring) -> u64>,
    pub get_wptr: Option<unsafe extern "C" fn(*mut amdgpu_ring) -> u64>,
    pub set_wptr: Option<unsafe extern "C" fn(*mut amdgpu_ring)>,
    pub parse_cs: Option<unsafe extern "C" fn(*mut amdgpu_cs_parser,*mut amdgpu_job,*mut amdgpu_ib)->i32>,
    pub patch_cs_in_place: Option<unsafe extern "C" fn(*mut amdgpu_cs_parser,*mut amdgpu_job,*mut amdgpu_ib)->i32>,
    pub emit_frame_size: u32, pub emit_ib_size: u32,
    pub emit_ib: Option<unsafe extern "C" fn(*mut amdgpu_ring,*mut amdgpu_job,*mut amdgpu_ib,u32)>,
    pub emit_fence: Option<unsafe extern "C" fn(*mut amdgpu_ring,u64,u64,u32)>,
    pub emit_pipeline_sync: Option<unsafe extern "C" fn(*mut amdgpu_ring)>,
    pub emit_vm_flush: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32,u64)>,
    pub emit_hdp_flush: Option<unsafe extern "C" fn(*mut amdgpu_ring)>,
    pub emit_gds_switch: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32,u32,u32,u32,u32,u32,u32)>,
    pub test_ring: Option<unsafe extern "C" fn(*mut amdgpu_ring)->i32>,
    pub test_ib: Option<unsafe extern "C" fn(*mut amdgpu_ring,i64)->i32>,
    pub insert_nop: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32)>,
    pub insert_start: Option<unsafe extern "C" fn(*mut amdgpu_ring)>, pub insert_end: Option<unsafe extern "C" fn(*mut amdgpu_ring)>,
    pub pad_ib: Option<unsafe extern "C" fn(*mut amdgpu_ring,*mut amdgpu_ib)>,
    pub init_cond_exec: Option<unsafe extern "C" fn(*mut amdgpu_ring,u64)->u32>,
    pub begin_use: Option<unsafe extern "C" fn(*mut amdgpu_ring)>, pub end_use: Option<unsafe extern "C" fn(*mut amdgpu_ring)>,
    pub emit_switch_buffer: Option<unsafe extern "C" fn(*mut amdgpu_ring)>, pub emit_cntxcntl: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32)>,
    pub emit_gfx_shadow: Option<unsafe extern "C" fn(*mut amdgpu_ring,u64,u64,u64,bool,i32)>,
    pub emit_rreg: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32,u32)>, pub emit_wreg: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32,u32)>,
    pub emit_reg_wait: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32,u32,u32)>,
    pub emit_reg_write_reg_wait: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32,u32,u32,u32)>,
    pub emit_frame_cntl: Option<unsafe extern "C" fn(*mut amdgpu_ring,bool,bool)>, pub soft_recovery: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32)>,
    pub preempt_ib: Option<unsafe extern "C" fn(*mut amdgpu_ring)->i32>, pub emit_mem_sync: Option<unsafe extern "C" fn(*mut amdgpu_ring)>,
    pub emit_wave_limit: Option<unsafe extern "C" fn(*mut amdgpu_ring,bool)>, pub patch_cntl: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32)>,
    pub patch_ce: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32)>, pub patch_de: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32)>,
    pub reset: Option<unsafe extern "C" fn(*mut amdgpu_ring,u32,*mut amdgpu_fence)->i32>, pub emit_cleaner_shader: Option<unsafe extern "C" fn(*mut amdgpu_ring)>,
}

#[repr(C)]
pub struct amdgpu_ring {
    pub adev: *mut amdgpu_device, pub funcs: *const amdgpu_ring_funcs, pub fence_drv: amdgpu_fence_driver, pub sched: drm_gpu_scheduler,
    pub ring_obj: *mut amdgpu_bo, pub ring: *mut u32, pub ring_backup: *mut u32, pub ring_backup_entries_to_copy: u32,
    pub reemit: bool, pub guilty_fence: *mut amdgpu_fence, pub rptr_offs: u32, pub rptr_gpu_addr: u64, pub rptr_cpu_addr: *mut u32,
    pub wptr: u64, pub wptr_old: u64, pub ring_size: u32, pub max_dw: u32, pub count_dw: i32, pub gpu_addr: u64, pub ptr_mask: u64, pub buf_mask: u32,
    pub idx: u32, pub xcc_id: u32, pub xcp_id: u32, pub me: u32, pub pipe: u32, pub queue: u32, pub mqd_obj: *mut amdgpu_bo, pub mqd_gpu_addr: u64, pub mqd_ptr: *mut core::ffi::c_void, pub mqd_size: u32, pub eop_gpu_addr: u64, pub doorbell_index: u32, pub use_doorbell: bool, pub use_pollmem: bool, pub wptr_offs: u32, pub wptr_gpu_addr: u64, pub wptr_cpu_addr: *mut u32, pub fence_offs: u32, pub fence_gpu_addr: u64, pub fence_cpu_addr: *mut u32, pub current_ctx: u64, pub name: [i8;16], pub trail_seq: u32, pub trail_fence_offs: u32, pub trail_fence_gpu_addr: u64, pub trail_fence_cpu_addr: *mut u32, pub cond_exe_offs: u32, pub cond_exe_gpu_addr: u64, pub cond_exe_cpu_addr: *mut u32, pub set_q_mode_offs: u32, pub set_q_mode_ptr: *mut u32, pub set_q_mode_token: u64, pub vm_hub: u32, pub vm_inv_eng: u32, pub vmid_wait: *mut dma_fence, pub has_compute_vm_bug: bool, pub no_scheduler: bool, pub no_user_submission: bool, pub hw_prio: i32, pub num_hw_submission: u32, pub sched_score: *mut atomic_t, pub is_sw_ring: bool, pub entry_index: u32, pub cached_rptr: u64,
}

extern "C" {
    pub fn amdgpu_fence_driver_set_error(ring:*mut amdgpu_ring,error:i32); pub fn amdgpu_fence_driver_force_completion(ring:*mut amdgpu_ring,fence:*mut dma_fence);
    pub fn amdgpu_ring_set_fence_errors_and_reemit(ring:*mut amdgpu_ring,fence:*mut amdgpu_fence);
    pub fn amdgpu_ring_max_ibs(ty:amdgpu_ring_type)->u32; pub fn amdgpu_ring_alloc(ring:*mut amdgpu_ring,ndw:u32)->i32;
    pub fn amdgpu_ring_init(adev:*mut amdgpu_device,ring:*mut amdgpu_ring,max_dw:u32,irq:*mut amdgpu_irq_src,irq_type:u32,hw_prio:u32,score:*mut atomic_t)->i32;
    pub fn amdgpu_ring_fini(ring:*mut amdgpu_ring); pub fn amdgpu_ring_test_helper(ring:*mut amdgpu_ring)->i32;
    pub fn amdgpu_ib_get_value(ib:*mut amdgpu_ib,idx:u32)->u32; pub fn amdgpu_ib_get(adev:*mut amdgpu_device,vm:*mut amdgpu_vm,size:u32,pool:amdgpu_ib_pool_type,ib:*mut amdgpu_ib)->i32;
    pub fn amdgpu_fence_driver_hw_init(adev:*mut amdgpu_device); pub fn amdgpu_fence_driver_hw_fini(adev:*mut amdgpu_device);
    pub fn amdgpu_fence_driver_sw_init(adev:*mut amdgpu_device)->i32; pub fn amdgpu_fence_driver_sw_fini(adev:*mut amdgpu_device);
    pub fn amdgpu_fence_emit(ring:*mut amdgpu_ring,af:*mut amdgpu_fence,flags:u32); pub fn amdgpu_fence_emit_polling(ring:*mut amdgpu_ring,s:*mut u32,timeout:u32)->i32;
    pub fn amdgpu_fence_process(ring:*mut amdgpu_ring)->bool; pub fn amdgpu_fence_wait_empty(ring:*mut amdgpu_ring)->i32;
    pub fn amdgpu_fence_count_emitted(ring:*mut amdgpu_ring)->u32; pub fn amdgpu_fence_driver_isr_toggle(adev:*mut amdgpu_device,stop:bool);
    pub fn amdgpu_ring_ib_begin(ring:*mut amdgpu_ring); pub fn amdgpu_ring_ib_end(ring:*mut amdgpu_ring); pub fn amdgpu_ring_commit(ring:*mut amdgpu_ring); pub fn amdgpu_ring_undo(ring:*mut amdgpu_ring);
    pub fn amdgpu_ring_insert_nop(ring:*mut amdgpu_ring,count:u32); pub fn amdgpu_ring_generic_pad_ib(ring:*mut amdgpu_ring,ib:*mut amdgpu_ib);
    pub fn amdgpu_ring_test_ib(ring:*mut amdgpu_ring,timeout:i64)->i32; pub fn amdgpu_ring_sched_ready(ring:*mut amdgpu_ring)->bool;
    pub fn amdgpu_ib_free(ib:*mut amdgpu_ib,f:*mut dma_fence); pub fn amdgpu_ib_schedule(ring:*mut amdgpu_ring,num:u32,ibs:*mut amdgpu_ib,job:*mut amdgpu_job,f:*mut *mut dma_fence)->i32;
    pub fn amdgpu_ib_pool_init(adev:*mut amdgpu_device)->i32; pub fn amdgpu_ib_pool_fini(adev:*mut amdgpu_device);
    pub fn amdgpu_ib_ring_tests(adev:*mut amdgpu_device)->i32; pub fn amdgpu_ring_init_mqd(ring:*mut amdgpu_ring)->i32;
}

#[inline] pub unsafe fn amdgpu_ib_value(ib:*mut amdgpu_ib, idx:u32)->u32 { if idx < (*ib).length_dw { *(*ib).ptr.add(idx as usize) } else { 0 } }
#[inline] pub unsafe fn amdgpu_ib_set_value(ib:*mut amdgpu_ib,idx:u32,value:u32) { if idx < (*ib).length_dw { *(*ib).ptr.add(idx as usize)=value; } }

#[inline] pub unsafe fn amdgpu_ring_get_dw_distance(r:*mut amdgpu_ring,start:u64,end:u64)->u32 { let s=(start as u32)&(*r).buf_mask; let mut e=(end as u32)&(*r).buf_mask; if e<s { e+=(*r).ring_size>>2; } e-s }
#[inline] pub unsafe fn amdgpu_ring_set_preempt_cond_exec(r:*mut amdgpu_ring,cond:bool) { *(*r).cond_exe_cpu_addr=cond as u32; }
#[inline] pub unsafe fn amdgpu_ring_write(r:*mut amdgpu_ring,v:u32) { let p=((*r).wptr as u32 & (*r).buf_mask) as usize; *(*r).ring.add(p)=v; (*r).wptr=(*r).wptr.wrapping_add(1); (*r).wptr&=(*r).ptr_mask; (*r).count_dw-=1; }
#[inline] pub unsafe fn amdgpu_ring_patch_cond_exec(r:*mut amdgpu_ring,offset:u32) { if (*r).funcs.is_null() || (*r).funcs.as_ref().unwrap().init_cond_exec.is_none() { return; } if offset<=(*r).buf_mask && *(*r).ring.add(offset as usize)==0 { *(*r).ring.add(offset as usize)=amdgpu_ring_get_dw_distance(r,offset as u64,(*r).wptr.wrapping_sub(1)); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
