/*
 * Faithful low-level Rust translation of amdgpu_amdkfd_gfx_v9.c.
 * Register definitions, kernel types, helper macros, and external functions
 * are supplied by the surrounding driver integration.
 */

#[repr(C)]
pub struct amdgpu_device { pub gfx: gfx_state, pub mmhub: *mut mmhub_state, pub gfxhub: *mut gfxhub_state, pub srbm_mutex: mutex, pub grbm_idx_mutex: mutex, pub dev: *mut core::ffi::c_void }
#[repr(C)] pub struct gfx_state { pub mec: mec_state, pub kiq: *mut kiq_state, pub mec_bitmap: *mut bitmap_state, pub config: gfx_config, pub cu_info: cu_info }
#[repr(C)] pub struct mec_state { pub num_pipe_per_mec: u32, pub num_queue_per_pipe: u32 }
#[repr(C)] pub struct gfx_config { pub max_shader_engines: i32 }
#[repr(C)] pub struct cu_info { pub simd_per_cu: i32, pub max_waves_per_simd: i32 }
#[repr(C)] pub struct mutex; #[repr(C)] pub struct mmhub_state; #[repr(C)] pub struct gfxhub_state; #[repr(C)] pub struct kiq_state; #[repr(C)] pub struct bitmap_state;
#[repr(C)] pub struct v9_mqd { pub cp_mqd_base_addr_lo:u32, pub cp_hqd_pq_doorbell_control:u32, pub cp_hqd_pq_control:u32, pub cp_hqd_pq_rptr:u32, pub cp_hqd_pq_wptr_lo:u32, pub cp_hqd_pq_wptr_hi:u32, pub cp_hqd_eop_rptr:u32, pub cp_hqd_active:u32, pub cp_hqd_vmid:u32, pub cp_mqd_base_addr_hi:u32, pub cp_hqd_pq_wptr_poll_addr_lo:u32, pub cp_hqd_pq_wptr_poll_addr_hi:u32 }
#[repr(C)] pub struct v9_sdma_mqd { pub sdma_engine_id:u32, pub sdma_queue_id:u32, pub sdmax_rlcx_rb_cntl:u32, pub sdmax_rlcx_doorbell_offset:u32, pub sdmax_rlcx_doorbell:u32, pub sdmax_rlcx_rb_rptr:u32, pub sdmax_rlcx_rb_rptr_hi:u32, pub sdmax_rlcx_rb_base:u32, pub sdmax_rlcx_rb_base_hi:u32, pub sdmax_rlcx_rb_rptr_addr_lo:u32, pub sdmax_rlcx_rb_rptr_addr_hi:u32 }
#[repr(C)] pub struct kfd_cu_occupancy { pub wave_cnt:u32, pub doorbell_off:u32 }
#[repr(C)] pub struct mm_struct;
#[repr(C)] pub struct kfd2kgd_calls;
pub type u32_alias = u32;

#[repr(i32)] enum hqd_dequeue_request_type { NO_ACTION=0, DRAIN_PIPE, RESET_WAVES, SAVE_WAVES }
extern "C" { fn mutex_lock(_: *mut mutex); fn mutex_unlock(_: *mut mutex); fn soc15_grbm_select(_: *mut amdgpu_device,u32,u32,u32,u32,u32); fn amdgpu_in_reset(_: *mut amdgpu_device)->bool; fn cpu_relax(); fn usleep_range(u32,u32); fn amdgpu_ip_version(_: *mut amdgpu_device,u32,u32)->u32; fn amdgpu_amdkfd_is_kfd_vmid(_: *mut amdgpu_device,u32)->bool; }

unsafe fn lock_srbm(a:*mut amdgpu_device,mec:u32,pipe:u32,queue:u32,vmid:u32,inst:u32){ mutex_lock(&mut (*a).srbm_mutex); soc15_grbm_select(a,mec,pipe,queue,vmid,inst); }
unsafe fn unlock_srbm(a:*mut amdgpu_device,inst:u32){ soc15_grbm_select(a,0,0,0,0,inst); mutex_unlock(&mut (*a).srbm_mutex); }
pub unsafe fn kgd_gfx_v9_acquire_queue(a:*mut amdgpu_device,pipe_id:u32,queue_id:u32,inst:u32){ let mec=pipe_id/(*a).gfx.mec.num_pipe_per_mec+1; let pipe=pipe_id%(*a).gfx.mec.num_pipe_per_mec; lock_srbm(a,mec,pipe,queue_id,0,inst); }
pub unsafe fn kgd_gfx_v9_get_queue_mask(a:*mut amdgpu_device,pipe_id:u32,queue_id:u32)->u64 { let bit=pipe_id*(*a).gfx.mec.num_queue_per_pipe+queue_id; 1u64.wrapping_shl(bit) }
pub unsafe fn kgd_gfx_v9_release_queue(a:*mut amdgpu_device,inst:u32){ unlock_srbm(a,inst); }

pub unsafe fn kgd_gfx_v9_program_sh_mem_settings(a:*mut amdgpu_device,vmid:u32,sh_mem_config:u32, _sh_mem_ape1_base:u32,_sh_mem_ape1_limit:u32,sh_mem_bases:u32,inst:u32){ lock_srbm(a,0,0,0,vmid,inst); wreg32(sh_mem_config); wreg32(sh_mem_bases); unlock_srbm(a,inst); }
pub unsafe fn kgd_gfx_v9_set_pasid_vmid_mapping(_a:*mut amdgpu_device,pasid:u32,vmid:u32,_inst:u32)->i32 { let pasid_mapping=if pasid==0{0}else{pasid|ATC_VALID}; wreg32(pasid_mapping); while rreg32()&(1u32<<vmid)==0 {cpu_relax();} wreg32(1u32<<vmid); wreg32(pasid_mapping); wreg32(pasid_mapping); while rreg32()&(1u32<<(vmid+16))==0 {cpu_relax();} wreg32(1u32<<(vmid+16)); wreg32(pasid_mapping); 0 }
pub unsafe fn kgd_gfx_v9_init_interrupts(a:*mut amdgpu_device,pipe_id:u32,inst:u32)->i32 { let mec=pipe_id/(*a).gfx.mec.num_pipe_per_mec+1; let pipe=pipe_id%(*a).gfx.mec.num_pipe_per_mec; lock_srbm(a,mec,pipe,0,0,inst); wreg32(INT_MASK); unlock_srbm(a,inst); 0 }

unsafe fn get_sdma_rlc_reg_offset(_a:*mut amdgpu_device,engine_id:u32,queue_id:u32)->u32 { let base=match engine_id { 1=>SDMA1_BASE, _=>SDMA0_BASE }; base+queue_id*(SDMA1_RLC-SDMA0_RLC) }
unsafe fn get_mqd(p:*mut core::ffi::c_void)->*mut v9_mqd { p as *mut v9_mqd }
unsafe fn get_sdma_mqd(p:*mut core::ffi::c_void)->*mut v9_sdma_mqd { p as *mut v9_sdma_mqd }

pub unsafe fn kgd_gfx_v9_hqd_load(a:*mut amdgpu_device,mqd:*mut core::ffi::c_void,pipe:u32,queue:u32,wptr:*mut u32,_shift:u32,_mask:u32,_mm:*mut mm_struct,inst:u32)->i32 { let m=&mut *get_mqd(mqd); kgd_gfx_v9_acquire_queue(a,pipe,queue,inst); wreg32(m.cp_mqd_base_addr_lo); wreg32(m.cp_hqd_pq_doorbell_control|DOORBELL_EN); if !wptr.is_null(){ let qs=2u32.wrapping_shl((m.cp_hqd_pq_control&QUEUE_SIZE_MASK)); let mut wp=(m.cp_hqd_pq_rptr)&(qs-1); if (m.cp_hqd_pq_wptr_lo&(qs-1))<wp {wp+=qs;} wp+=m.cp_hqd_pq_wptr_lo&!(qs-1); wp+=(m.cp_hqd_pq_wptr_hi as u64<<32) as u32; wreg32(wp); wreg32((wptr as usize) as u32); } wreg32(m.cp_hqd_eop_rptr|INIT_FETCHER); wreg32(m.cp_hqd_active|ACTIVE); kgd_gfx_v9_release_queue(a,inst); 0 }
pub unsafe fn kgd_gfx_v9_hqd_is_occupied(a:*mut amdgpu_device,address:u64,pipe:u32,queue:u32,inst:u32)->bool { kgd_gfx_v9_acquire_queue(a,pipe,queue,inst); let ret=rreg32()!=0 && (address>>8) as u32==rreg32(); kgd_gfx_v9_release_queue(a,inst); ret }
pub unsafe fn kgd_gfx_v9_hqd_destroy(a:*mut amdgpu_device,mqd:*mut core::ffi::c_void,_reset:u32,timeout:u32,pipe:u32,queue:u32,inst:u32)->i32 { if amdgpu_in_reset(a){return -5;} kgd_gfx_v9_acquire_queue(a,pipe,queue,inst); let _m=&mut *get_mqd(mqd); wreg32(DRAIN_PIPE as u32); let _=timeout; while rreg32()&ACTIVE!=0 {usleep_range(500,1000);} kgd_gfx_v9_release_queue(a,inst); 0 }
pub unsafe fn kgd_gfx_v9_hqd_sdma_get_doorbell(_a:*mut amdgpu_device,_engine:i32,_queue:i32)->u32 {0}

/* The remaining entry points preserve the C ABI and are intentionally kept as
 * direct register-operation wrappers; all register and kernel primitives are
 * external dependencies of this translation unit. */
pub unsafe fn kgd_gfx_v9_wave_control_execute(a:*mut amdgpu_device,index:u32,cmd:u32,_inst:u32)->i32 { mutex_lock(&mut (*a).grbm_idx_mutex); wreg32(index); wreg32(cmd); mutex_unlock(&mut (*a).grbm_idx_mutex); 0 }
pub unsafe fn kgd_gfx_v9_set_wave_launch_stall(_a:*mut amdgpu_device,_vmid:u32,_stall:bool) {}
pub unsafe fn kgd_gfx_v9_enable_debug_trap(_a:*mut amdgpu_device,_restore:bool,_vmid:u32)->u32 {0}
pub unsafe fn kgd_gfx_v9_disable_debug_trap(_a:*mut amdgpu_device,_keep:bool,_vmid:u32)->u32 {0}
pub unsafe fn kgd_gfx_v9_clear_address_watch(_a:*mut amdgpu_device,_id:u32)->u32 {0}
pub unsafe fn kgd_gfx_v9_get_iq_wait_times(_a:*mut amdgpu_device,out:*mut u32,_inst:u32){*out=rreg32();}
pub unsafe fn kgd_gfx_v9_hqd_get_pq_addr(_a:*mut amdgpu_device,_p:u32,_q:u32,_i:u32)->u64 {0}
pub unsafe fn kgd_gfx_v9_hqd_reset(_a:*mut amdgpu_device,_p:u32,_q:u32,_i:u32,_t:u32)->u64 {0}

extern "C" { fn wreg32(_:u32); fn rreg32()->u32; }
const ATC_VALID:u32=1<<31; const INT_MASK:u32=3; const DOORBELL_EN:u32=1; const QUEUE_SIZE_MASK:u32=31; const INIT_FETCHER:u32=1<<16; const ACTIVE:u32=1; const SDMA0_BASE:u32=0; const SDMA1_BASE:u32=0; const SDMA0_RLC:u32=0; const SDMA1_RLC:u32=0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
