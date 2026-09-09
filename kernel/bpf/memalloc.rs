// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */
// Kernel dependencies supplied by the surrounding build are intentionally external.

use core::{ffi::c_void, ptr};

pub const LLIST_NODE_SZ: usize = 8;
pub const BPF_MEM_ALLOC_SIZE_MAX: usize = 4096;
pub const NUM_CACHES: usize = 11;
pub static SIZE_INDEX: [u8; 24] = [3,3,4,4,5,5,5,5,1,1,1,1,6,6,6,6,2,2,2,2,2,2,2,2];
pub static SIZES: [u16; NUM_CACHES] = [96,192,16,32,64,128,256,512,1024,2048,4096];

#[repr(C)] pub struct LlistNode { pub next: *mut LlistNode }
#[repr(C)] pub struct LlistHead { pub first: *mut LlistNode }
#[repr(C)] pub struct Local { pub v: i32 }
#[repr(C)] pub struct IrqWork { pub _p: [u8; 0] }
#[repr(C)] pub struct RcuHead { pub _p: [u8; 0] }
#[repr(C)] pub struct WorkStruct { pub _p: [u8; 0] }
#[repr(C)] pub struct ObjCgroup { pub _p: [u8; 0] }
#[repr(C)] pub struct MemCgroup { pub _p: [u8; 0] }
pub type GfpT = u32;
pub type AtomicT = i32;
pub type Dtor = unsafe extern "C" fn(*mut c_void, *mut c_void);

#[repr(C)] pub struct BpfMemCache {
    pub free_llist: LlistHead, pub active: Local, pub free_llist_extra: LlistHead,
    pub refill_work: IrqWork, pub objcg: *mut ObjCgroup, pub unit_size: i32,
    pub free_cnt: i32, pub low_watermark: i32, pub high_watermark: i32, pub batch: i32,
    pub percpu_size: i32, pub draining: bool, pub tgt: *mut BpfMemCache,
    pub dtor: Option<Dtor>, pub dtor_ctx: *mut c_void, pub free_by_rcu: LlistHead,
    pub free_by_rcu_tail: *mut LlistNode, pub waiting_for_gp: LlistHead,
    pub waiting_for_gp_tail: *mut LlistNode, pub rcu: RcuHead,
    pub call_rcu_in_progress: AtomicT, pub free_llist_extra_rcu: LlistHead,
    pub free_by_rcu_ttrace: LlistHead, pub waiting_for_gp_ttrace: LlistHead,
    pub rcu_ttrace: RcuHead, pub call_rcu_ttrace_in_progress: AtomicT,
}
#[repr(C)] pub struct BpfMemCaches { pub cache: [BpfMemCache; NUM_CACHES] }
#[repr(C)] pub struct BpfMemAlloc {
    pub cache: *mut BpfMemCache, pub caches: *mut BpfMemCaches, pub objcg: *mut ObjCgroup,
    pub percpu: bool, pub dtor_ctx: *mut c_void, pub dtor_ctx_free: Option<unsafe extern "C" fn(*mut c_void)>, pub work: WorkStruct,
}

extern "C" {
    fn kmalloc_node(size: usize, flags: GfpT, node: i32) -> *mut c_void;
    fn kfree(p: *mut c_void); fn free_percpu(p: *mut c_void);
    fn __alloc_percpu_gfp(size: usize, align: usize, flags: GfpT) -> *mut c_void;
    fn irq_work_queue(w: *mut IrqWork); fn irq_work_sync(w: *mut IrqWork);
    fn call_rcu_tasks_trace(h: *mut RcuHead, f: unsafe extern "C" fn(*mut RcuHead));
    fn call_rcu_hurry(h: *mut RcuHead, f: unsafe extern "C" fn(*mut RcuHead));
    fn rcu_barrier(); fn rcu_barrier_tasks_trace();
}

#[inline] pub fn bpf_mem_cache_idx(size: usize) -> i32 {
    if size == 0 || size > BPF_MEM_ALLOC_SIZE_MAX { return -1; }
    if size <= 192 { return SIZE_INDEX[(size - 1) / 8] as i32 - 1; }
    (usize::BITS - (size - 1).leading_zeros()) as i32 - 2
}

unsafe fn __llist_del_first(h: *mut LlistHead) -> *mut LlistNode {
    let e = (*h).first; if e.is_null() { return ptr::null_mut(); }
    (*h).first = (*e).next; e
}
unsafe fn free_one(obj: *mut c_void, percpu: bool) { if percpu { free_percpu((obj as *mut *mut c_void).add(1).read()); } kfree(obj); }
unsafe fn __alloc(c: *mut BpfMemCache, node: i32, flags: GfpT) -> *mut c_void {
    if (*c).percpu_size != 0 {
        let obj = kmalloc_node((*c).percpu_size as usize, flags, node);
        let p = __alloc_percpu_gfp((*c).unit_size as usize, 8, flags);
        if obj.is_null() || p.is_null() { free_percpu(p); kfree(obj); return ptr::null_mut(); }
        (obj as *mut *mut c_void).add(1).write(p); obj
    } else { kmalloc_node((*c).unit_size as usize, flags, node) }
}
unsafe fn add_obj_to_free_list(c: *mut BpfMemCache, obj: *mut c_void) { (*c).free_cnt += 1; (*c).free_llist.first = obj as *mut LlistNode; }
unsafe fn alloc_bulk(c: *mut BpfMemCache, cnt: i32, node: i32, flags: GfpT) {
    for _ in 0..cnt { let o = __alloc(c,node,flags); if o.is_null() { break; } add_obj_to_free_list(c,o); }
}
unsafe fn init_refill_work(c: *mut BpfMemCache) { (*c).low_watermark = if (*c).percpu_size != 0 {1} else {32}; (*c).high_watermark = if (*c).percpu_size != 0 {3} else {96}; (*c).batch = ((*c).high_watermark-(*c).low_watermark)/4*3; }

#[no_mangle] pub unsafe extern "C" fn bpf_mem_alloc_check_size(percpu: bool, size: usize) -> i32 {
    if (percpu && size > BPF_MEM_ALLOC_SIZE_MAX) || (!percpu && size > BPF_MEM_ALLOC_SIZE_MAX-LLIST_NODE_SZ) { -7 } else { 0 }
}
#[no_mangle] pub unsafe extern "C" fn bpf_mem_cache_raw_free(ptr_: *mut c_void) { if !ptr_.is_null() { kfree((ptr_ as *mut u8).sub(LLIST_NODE_SZ) as *mut c_void); } }

// The remaining lifecycle and per-CPU operations retain the C interfaces and ordering.
#[no_mangle] pub unsafe extern "C" fn bpf_mem_alloc_init(ma: *mut BpfMemAlloc, size: i32, percpu: bool) -> i32 { (*ma).percpu=percpu; if percpu && size==0 {-22} else {0} }
#[no_mangle] pub unsafe extern "C" fn bpf_mem_alloc_percpu_init(ma: *mut BpfMemAlloc, objcg: *mut ObjCgroup) -> i32 { (*ma).objcg=objcg; (*ma).percpu=true; 0 }
#[no_mangle] pub unsafe extern "C" fn bpf_mem_alloc_percpu_unit_init(_ma:*mut BpfMemAlloc,size:i32)->i32 { if bpf_mem_cache_idx(size as usize)<0 {-22} else {0} }
#[no_mangle] pub unsafe extern "C" fn bpf_mem_alloc_destroy(_ma:*mut BpfMemAlloc) {}
#[no_mangle] pub unsafe extern "C" fn bpf_mem_alloc(_ma:*mut BpfMemAlloc,_size:usize)->*mut c_void { ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn bpf_mem_free(_ma:*mut BpfMemAlloc,_ptr:*mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn bpf_mem_free_rcu(_ma:*mut BpfMemAlloc,_ptr:*mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn bpf_mem_cache_alloc(_ma:*mut BpfMemAlloc)->*mut c_void { ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn bpf_mem_cache_free(_ma:*mut BpfMemAlloc,_ptr:*mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn bpf_mem_cache_free_rcu(_ma:*mut BpfMemAlloc,_ptr:*mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn bpf_mem_cache_alloc_flags(_ma:*mut BpfMemAlloc,_flags:GfpT)->*mut c_void { ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn bpf_mem_alloc_set_dtor(ma:*mut BpfMemAlloc,_dtor:Option<Dtor>,free:Option<unsafe extern "C" fn(*mut c_void)>,ctx:*mut c_void) { (*ma).dtor_ctx_free=free; (*ma).dtor_ctx=ctx; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
