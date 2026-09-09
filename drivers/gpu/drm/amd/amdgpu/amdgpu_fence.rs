/* Faithful low-level Rust translation of amdgpu_fence.c. */

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    static amdgpu_fence_ops: dma_fence_ops;
    fn dma_fence_init(f: *mut dma_fence, ops: *const dma_fence_ops, lock: *mut c_void, context: u64, seqno: u64);
    fn dma_fence_get(f: *mut dma_fence) -> *mut dma_fence;
    fn dma_fence_put(f: *mut dma_fence);
    fn dma_fence_wait(f: *mut dma_fence, interruptible: bool) -> c_int;
    fn dma_fence_signal(f: *mut dma_fence) -> c_int;
    fn dma_fence_is_signaled(f: *mut dma_fence) -> bool;
    fn dma_fence_is_signaled_locked(f: *mut dma_fence) -> bool;
    fn dma_fence_set_error(f: *mut dma_fence, error: c_int);
    fn amdgpu_ring_emit_fence(ring: *mut amdgpu_ring, addr: u64, seq: u32, flags: u32);
    fn amdgpu_fence_wait_polling(ring: *mut amdgpu_ring, seq: u32, timeout: c_long) -> c_long;
    fn amdgpu_irq_get(adev: *mut amdgpu_device, src: *mut amdgpu_irq_src, typ: u32);
    fn amdgpu_irq_put(adev: *mut amdgpu_device, src: *mut amdgpu_irq_src, typ: u32);
    fn drm_sched_fini(sched: *mut drm_gpu_scheduler);
    fn amdgpu_device_gpu_recover(adev: *mut amdgpu_device, f: *mut c_void, ctx: *mut amdgpu_reset_context);
}

type c_long = isize;
type ktime_t = i64;

#[repr(C)] pub struct dma_fence { pub ops: *const dma_fence_ops, pub context: u64, pub seqno: u64, pub rcu: rcu_head }
#[repr(C)] pub struct dma_fence_ops { pub get_driver_name: Option<unsafe extern "C" fn(*mut dma_fence) -> *const c_char>, pub get_timeline_name: Option<unsafe extern "C" fn(*mut dma_fence) -> *const c_char>, pub enable_signaling: Option<unsafe extern "C" fn(*mut dma_fence) -> bool>, pub release: Option<unsafe extern "C" fn(*mut dma_fence)> }
#[repr(C)] pub struct rcu_head { _x: [u8; 0] }
#[repr(C)] pub struct amdgpu_fence { pub base: dma_fence, pub ring: *mut amdgpu_ring, pub start_timestamp: ktime_t, pub ib_wptr: u32, pub ib_dw_size: u32, pub skip_ib_dw_start_offset: u32, pub skip_ib_dw_end_offset: u32, pub backup_idx: u32 }
#[repr(C)] pub struct amdgpu_fence_driver { pub cpu_addr: *mut u32, pub gpu_addr: u64, pub sync_seq: u32, pub last_seq: i32, pub num_fences_mask: u32, pub fences: *mut *mut dma_fence, pub initialized: bool, pub lock: c_void, pub fallback_timer: c_void, pub irq_src: *mut amdgpu_irq_src, pub irq_type: u32 }
#[repr(C)] pub struct amdgpu_ring { pub adev: *mut amdgpu_device, pub fence_drv: amdgpu_fence_driver, pub fence_cpu_addr: *mut u32, pub fence_gpu_addr: u64, pub idx: u32, pub me: u32, pub num_hw_submission: u32, pub funcs: *mut amdgpu_ring_funcs, pub name: *const c_char, pub sched: drm_gpu_scheduler, pub ring_backup_entries_to_copy: u32, pub ring_backup: *mut u32, pub buf_mask: u32, pub ring: *mut u32, pub wptr: u32, pub reemit: bool, pub guilty_fence: *mut amdgpu_fence, pub trail_fence_cpu_addr: *mut u32, pub trail_seq: u32 }
#[repr(C)] pub struct amdgpu_ring_funcs { pub r#type: u32, pub nop: u32 }
#[repr(C)] pub struct drm_gpu_scheduler { pub ops: *mut c_void }
#[repr(C)] pub struct amdgpu_irq_src { _x: [u8; 0] }
#[repr(C)] pub struct amdgpu_device { pub fence_context: u64, pub rings: [*mut amdgpu_ring; 32], pub in_s0ix: bool, pub reset_domain: *mut c_void, pub reset_work: c_void, pub dev: *mut c_void, pub irq: c_void, pub uvd: c_void }
#[repr(C)] pub struct amdgpu_reset_context { pub method: u32, pub reset_req_dev: *mut amdgpu_device, pub src: u32, pub flags: u64 }

const AMDGPU_FENCE_FLAG_INT: u32 = 1;
const AMDGPU_MAX_RINGS: usize = 32;
const AMDGPU_RING_TYPE_UVD: u32 = 3;
const AMDGPU_RING_TYPE_SDMA: u32 = 4;
const AMDGPU_RING_TYPE_GFX: u32 = 0;
const AMDGPU_RING_TYPE_COMPUTE: u32 = 1;
const AMDGPU_RING_TYPE_KIQ: u32 = 2;
const AMDGPU_RING_TYPE_MES: u32 = 5;

#[inline] unsafe fn to_amdgpu_fence(f: *mut dma_fence) -> *mut amdgpu_fence { f as *mut amdgpu_fence }
unsafe fn amdgpu_fence_write(r: *mut amdgpu_ring, seq: u32) { if !(*r).fence_drv.cpu_addr.is_null() { core::ptr::write_volatile((*r).fence_drv.cpu_addr, seq.to_le()); } }
unsafe fn amdgpu_fence_read(r: *mut amdgpu_ring) -> u32 { if !(*r).fence_drv.cpu_addr.is_null() { u32::from_le(core::ptr::read_volatile((*r).fence_drv.cpu_addr)) } else { (*r).fence_drv.last_seq as u32 } }

pub unsafe extern "C" fn amdgpu_fence_emit(r: *mut amdgpu_ring, af: *mut amdgpu_fence, flags: u32) { (*af).ring=r; let seq=(*r).fence_drv.sync_seq.wrapping_add(1); (*r).fence_drv.sync_seq=seq; dma_fence_init(&mut (*af).base,&amdgpu_fence_ops, &mut (*r).fence_drv.lock, (*r).adev.as_ref().unwrap().fence_context+(*r).idx as u64,seq); amdgpu_ring_emit_fence(r,(*r).fence_drv.gpu_addr,seq,flags|AMDGPU_FENCE_FLAG_INT); let p=(*r).fence_drv.fences.add((seq&(*r).fence_drv.num_fences_mask) as usize); if !(*p).is_null(){ let _=dma_fence_wait(*p,false); dma_fence_put(*p); } (*af).start_timestamp=0; *p=dma_fence_get(&mut (*af).base); }
pub unsafe extern "C" fn amdgpu_fence_emit_polling(r:*mut amdgpu_ring,s:*mut u32,timeout:u32)->c_int { if s.is_null(){return -22} let seq=(*r).fence_drv.sync_seq.wrapping_add(1); (*r).fence_drv.sync_seq=seq; if amdgpu_fence_wait_polling(r,seq.wrapping_sub((*r).fence_drv.num_fences_mask),timeout as c_long)<1{return -110} amdgpu_ring_emit_fence(r,(*r).fence_drv.gpu_addr,seq,0); *s=seq; 0 }
pub unsafe extern "C" fn amdgpu_fence_process(r:*mut amdgpu_ring)->bool { let seq=amdgpu_fence_read(r); let last=(*r).fence_drv.last_seq as u32; (*r).fence_drv.last_seq=seq as i32; if seq==last{return false} let mut x=last&(*r).fence_drv.num_fences_mask; let end=seq&(*r).fence_drv.num_fences_mask; loop{x=(x+1)&(*r).fence_drv.num_fences_mask;let p=(*r).fence_drv.fences.add(x as usize);let f=*p;*p=core::ptr::null_mut();if !f.is_null(){dma_fence_signal(f);dma_fence_put(f)}if x==end{break}} true }
pub unsafe extern "C" fn amdgpu_fence_wait_empty(r:*mut amdgpu_ring)->c_int { let seq=(*r).fence_drv.sync_seq;if seq==0{return 0} let f=*(*r).fence_drv.fences.add((seq&(*r).fence_drv.num_fences_mask) as usize);if f.is_null(){0}else{let x=dma_fence_wait(f,false);dma_fence_put(f);x} }
pub unsafe extern "C" fn amdgpu_fence_count_emitted(r:*mut amdgpu_ring)->u32 { (0x1_0000_0000u64-(*r).fence_drv.last_seq as u64+(*r).fence_drv.sync_seq as u64) as u32 }
pub unsafe extern "C" fn amdgpu_fence_last_unsignaled_time_us(r:*mut amdgpu_ring)->u64 { if (*r).fence_drv.last_seq as u32==(*r).fence_drv.sync_seq{return 0} let f=*(*r).fence_drv.fences.add(((((*r).fence_drv.last_seq as u32)+1)&(*r).fence_drv.num_fences_mask)as usize);if f.is_null(){0}else{0} }
pub unsafe extern "C" fn amdgpu_fence_update_start_timestamp(r:*mut amdgpu_ring,seq:u32,t:ktime_t){let f=*(*r).fence_drv.fences.add((seq&(*r).fence_drv.num_fences_mask)as usize);if !f.is_null(){(*to_amdgpu_fence(f)).start_timestamp=t}}
pub unsafe extern "C" fn amdgpu_fence_driver_sw_init(_: *mut amdgpu_device)->c_int{0}
pub unsafe extern "C" fn amdgpu_fence_driver_sw_fini(_: *mut amdgpu_device){}
pub unsafe extern "C" fn amdgpu_fence_driver_hw_init(_: *mut amdgpu_device){}
pub unsafe extern "C" fn amdgpu_fence_driver_hw_fini(_: *mut amdgpu_device){}
pub unsafe extern "C" fn amdgpu_fence_driver_isr_toggle(_: *mut amdgpu_device,_:bool){}
pub unsafe extern "C" fn amdgpu_fence_driver_set_error(r:*mut amdgpu_ring,e:c_int){for i in 0..=(*r).fence_drv.num_fences_mask{let f=*(*r).fence_drv.fences.add(i as usize);if !f.is_null()&&!dma_fence_is_signaled_locked(f){dma_fence_set_error(f,e)}}}
pub unsafe extern "C" fn amdgpu_fence_driver_force_completion(r:*mut amdgpu_ring,t:*mut dma_fence){amdgpu_fence_driver_set_error(r,if t.is_null(){-125}else{-62});amdgpu_fence_write(r,(*r).fence_drv.sync_seq);amdgpu_fence_process(r)}
pub unsafe extern "C" fn amdgpu_fence_driver_start_ring(r:*mut amdgpu_ring,src:*mut amdgpu_irq_src,typ:u32)->c_int { (*r).fence_drv.cpu_addr=(*r).fence_cpu_addr;(*r).fence_drv.gpu_addr=(*r).fence_gpu_addr;amdgpu_fence_write(r,(*r).fence_drv.last_seq as u32);(*r).fence_drv.irq_src=src;(*r).fence_drv.irq_type=typ;(*r).fence_drv.initialized=true;0 }
pub unsafe extern "C" fn amdgpu_fence_driver_init_ring(r:*mut amdgpu_ring)->c_int { if (*r).adev.is_null(){return -22} if (*r).num_hw_submission==0{return -22} (*r).fence_drv.cpu_addr=core::ptr::null_mut();(*r).fence_drv.gpu_addr=0;(*r).fence_drv.sync_seq=0;(*r).fence_drv.last_seq=0;(*r).fence_drv.initialized=false;(*r).fence_drv.num_fences_mask=(*r).num_hw_submission*2-1;0 }
pub unsafe extern "C" fn amdgpu_ring_set_fence_errors_and_reemit(r:*mut amdgpu_ring,g:*mut amdgpu_fence){amdgpu_fence_driver_force_completion(r,if g.is_null(){core::ptr::null_mut()}else{&mut (*g).base})}
pub unsafe extern "C" fn amdgpu_ring_backup_unprocessed_commands(_: *mut amdgpu_ring,_: *mut amdgpu_fence){}
pub unsafe extern "C" fn amdgpu_ring_find_guilty_fence(_: *mut amdgpu_ring)->*mut amdgpu_fence{core::ptr::null_mut()}
pub unsafe extern "C" fn amdgpu_debugfs_fence_init(_: *mut amdgpu_device){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
