/* SPDX-License-Identifier: GPL-2.0-only OR MIT */

// Translated from drm_gpuvm.h. Kernel/DRM types and functions are external dependencies.

use core::ffi::{c_char, c_void};

#[repr(C)] pub struct drm_gpuvm { pub name: *const c_char, pub flags: drm_gpuvm_flags, pub drm: *mut drm_device, pub mm_start: u64, pub mm_range: u64, pub rb: drm_gpuvm_rb, pub kref: kref, pub kernel_alloc_node: drm_gpuva, pub ops: *const drm_gpuvm_ops, pub r_obj: *mut drm_gem_object, pub extobj: drm_gpuvm_obj_list, pub evict: drm_gpuvm_obj_list, pub bo_defer: llist_head }
#[repr(C)] pub struct drm_gpuvm_bo { pub vm: *mut drm_gpuvm, pub obj: *mut drm_gem_object, pub evicted: bool, pub kref: kref, pub list: drm_gpuvm_bo_lists }
#[repr(C)] pub struct drm_gpuvm_ops;
#[repr(C)] pub struct drm_device;
#[repr(C)] pub struct drm_gem_object { pub resv: *mut dma_resv }
#[repr(C)] pub struct drm_exec;
#[repr(C)] pub struct dma_fence;
#[repr(C)] pub struct dma_resv;
#[repr(C)] pub struct kref;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct llist_head;
#[repr(C)] pub struct llist_node;
#[repr(C)] pub struct rb_node;
#[repr(C)] pub struct rb_root_cached;
#[repr(C)] pub struct spinlock_t;

#[repr(C)] #[derive(Copy, Clone)] pub struct drm_gpuva { pub vm: *mut drm_gpuvm, pub vm_bo: *mut drm_gpuvm_bo, pub flags: drm_gpuva_flags, pub va: drm_gpuva_va, pub gem: drm_gpuva_gem, pub rb: drm_gpuva_rb }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_gpuva_va { pub addr: u64, pub range: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_gpuva_gem { pub offset: u64, pub obj: *mut drm_gem_object, pub entry: list_head }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_gpuva_rb { pub node: rb_node, pub entry: list_head, pub __subtree_last: u64 }
#[repr(C)] pub struct drm_gpuvm_rb { pub tree: rb_root_cached, pub list: list_head }
#[repr(C)] pub struct drm_gpuvm_obj_list { pub list: list_head, pub local_list: *mut list_head, pub lock: spinlock_t }
#[repr(C)] pub struct drm_gpuvm_bo_lists { pub gpuva: list_head, pub entry: drm_gpuvm_bo_entries }
#[repr(C)] pub struct drm_gpuvm_bo_entries { pub gem: list_head, pub extobj: list_head, pub evict: list_head, pub bo_defer: llist_node }

#[repr(C)] #[derive(Copy, Clone)] pub struct drm_gpuvm_map_req { pub map: drm_gpuva_op_map }
#[repr(C)] pub struct drm_gpuvm_exec { pub exec: drm_exec, pub flags: u32, pub vm: *mut drm_gpuvm, pub num_fences: u32, pub extra: drm_gpuvm_exec_extra }
#[repr(C)] pub struct drm_gpuvm_exec_extra { pub fn_: Option<unsafe extern "C" fn(*mut drm_gpuvm_exec) -> i32>, pub priv_: *mut c_void }

#[repr(C)] #[derive(Copy, Clone)] pub struct drm_gpuva_op_map { pub va: drm_gpuva_va, pub gem: drm_gpuva_op_gem }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_gpuva_op_gem { pub offset: u64, pub obj: *mut drm_gem_object }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_gpuva_op_unmap { pub va: *mut drm_gpuva, pub keep: bool }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_gpuva_op_remap { pub prev: *mut drm_gpuva_op_map, pub next: *mut drm_gpuva_op_map, pub unmap: *mut drm_gpuva_op_unmap }
#[repr(C)] #[derive(Copy, Clone)] pub struct drm_gpuva_op_prefetch { pub va: *mut drm_gpuva }
#[repr(C)] pub union drm_gpuva_op_data { pub map: drm_gpuva_op_map, pub remap: drm_gpuva_op_remap, pub unmap: drm_gpuva_op_unmap, pub prefetch: drm_gpuva_op_prefetch }
#[repr(C)] pub struct drm_gpuva_op { pub entry: list_head, pub op: drm_gpuva_op_type, pub data: drm_gpuva_op_data }
#[repr(C)] pub struct drm_gpuva_ops { pub list: list_head }

#[repr(C)] #[derive(Copy, Clone)] pub struct drm_gpuva_ops_callbacks { pub vm_free: Option<unsafe extern "C" fn(*mut drm_gpuvm)>, pub op_alloc: Option<unsafe extern "C" fn() -> *mut drm_gpuva_op>, pub op_free: Option<unsafe extern "C" fn(*mut drm_gpuva_op)>, pub vm_bo_alloc: Option<unsafe extern "C" fn() -> *mut drm_gpuvm_bo>, pub vm_bo_free: Option<unsafe extern "C" fn(*mut drm_gpuvm_bo)>, pub vm_bo_validate: Option<unsafe extern "C" fn(*mut drm_gpuvm_bo, *mut drm_exec) -> i32>, pub sm_step_map: Option<unsafe extern "C" fn(*mut drm_gpuva_op, *mut c_void) -> i32>, pub sm_step_remap: Option<unsafe extern "C" fn(*mut drm_gpuva_op, *mut c_void) -> i32>, pub sm_step_unmap: Option<unsafe extern "C" fn(*mut drm_gpuva_op, *mut c_void) -> i32> }
pub type drm_gpuvm_ops = drm_gpuvm_ops_callbacks;

#[repr(C)] #[derive(Copy, Clone)] pub enum drm_gpuva_flags { DRM_GPUVA_INVALIDATED = 1, DRM_GPUVA_SPARSE = 2, DRM_GPUVA_USERBITS = 4 }
#[repr(C)] #[derive(Copy, Clone)] pub enum drm_gpuvm_flags { DRM_GPUVM_RESV_PROTECTED = 1, DRM_GPUVM_IMMEDIATE_MODE = 2, DRM_GPUVM_USERBITS = 4 }
#[repr(C)] #[derive(Copy, Clone)] pub enum drm_gpuva_op_type { DRM_GPUVA_OP_MAP, DRM_GPUVA_OP_REMAP, DRM_GPUVA_OP_UNMAP, DRM_GPUVA_OP_PREFETCH, DRM_GPUVA_OP_DRIVER }

extern "C" {
    pub fn drm_gpuva_insert(*mut drm_gpuvm, *mut drm_gpuva) -> i32; pub fn drm_gpuva_remove(*mut drm_gpuva);
    pub fn drm_gpuva_link(*mut drm_gpuva, *mut drm_gpuvm_bo); pub fn drm_gpuva_unlink(*mut drm_gpuva); pub fn drm_gpuva_unlink_defer(*mut drm_gpuva);
    pub fn drm_gpuva_find(*mut drm_gpuvm, u64, u64) -> *mut drm_gpuva; pub fn drm_gpuva_find_first(*mut drm_gpuvm, u64, u64) -> *mut drm_gpuva;
    pub fn drm_gpuvm_init(*mut drm_gpuvm, *const c_char, drm_gpuvm_flags, *mut drm_device, *mut drm_gem_object, u64, u64, u64, u64, *const drm_gpuvm_ops);
    pub fn drm_gpuvm_put(*mut drm_gpuvm); pub fn drm_gpuvm_range_valid(*mut drm_gpuvm, u64, u64) -> bool; pub fn drm_gpuvm_interval_empty(*mut drm_gpuvm, u64, u64) -> bool;
    pub fn drm_gpuvm_resv_object_alloc(*mut drm_device) -> *mut drm_gem_object;
    pub fn drm_gpuvm_prepare_vm(*mut drm_gpuvm, *mut drm_exec, u32) -> i32; pub fn drm_gpuvm_prepare_objects(*mut drm_gpuvm, *mut drm_exec, u32) -> i32; pub fn drm_gpuvm_prepare_range(*mut drm_gpuvm, *mut drm_exec, u64, u64, u32) -> i32;
    pub fn drm_gpuvm_exec_lock(*mut drm_gpuvm_exec) -> i32; pub fn drm_gpuvm_exec_lock_array(*mut drm_gpuvm_exec, *mut *mut drm_gem_object, u32) -> i32; pub fn drm_gpuvm_exec_lock_range(*mut drm_gpuvm_exec, u64, u64) -> i32;
    pub fn drm_gpuvm_validate(*mut drm_gpuvm, *mut drm_exec) -> i32; pub fn drm_gpuvm_resv_add_fence(*mut drm_gpuvm, *mut drm_exec, *mut dma_fence, dma_resv_usage, dma_resv_usage);
    pub fn drm_gpuvm_bo_create(*mut drm_gpuvm, *mut drm_gem_object) -> *mut drm_gpuvm_bo; pub fn drm_gpuvm_bo_obtain_locked(*mut drm_gpuvm, *mut drm_gem_object) -> *mut drm_gpuvm_bo; pub fn drm_gpuvm_bo_obtain_prealloc(*mut drm_gpuvm_bo) -> *mut drm_gpuvm_bo;
    pub fn drm_gpuvm_bo_put(*mut drm_gpuvm_bo) -> bool; pub fn drm_gpuvm_bo_put_deferred(*mut drm_gpuvm_bo) -> bool; pub fn drm_gpuvm_bo_deferred_cleanup(*mut drm_gpuvm);
    pub fn drm_gpuvm_bo_find(*mut drm_gpuvm, *mut drm_gem_object) -> *mut drm_gpuvm_bo; pub fn drm_gpuvm_bo_evict(*mut drm_gpuvm_bo, bool); pub fn drm_gpuvm_bo_extobj_add(*mut drm_gpuvm_bo);
    pub fn drm_gpuvm_sm_map_ops_create(*mut drm_gpuvm, *const drm_gpuvm_map_req) -> *mut drm_gpuva_ops; pub fn drm_gpuvm_madvise_ops_create(*mut drm_gpuvm, *const drm_gpuvm_map_req) -> *mut drm_gpuva_ops; pub fn drm_gpuvm_sm_unmap_ops_create(*mut drm_gpuvm, u64, u64) -> *mut drm_gpuva_ops; pub fn drm_gpuvm_prefetch_ops_create(*mut drm_gpuvm, u64, u64) -> *mut drm_gpuva_ops; pub fn drm_gpuvm_bo_unmap_ops_create(*mut drm_gpuvm_bo) -> *mut drm_gpuva_ops; pub fn drm_gpuva_ops_free(*mut drm_gpuvm, *mut drm_gpuva_ops);
    pub fn drm_gpuvm_sm_map(*mut drm_gpuvm, *mut c_void, *const drm_gpuvm_map_req) -> i32; pub fn drm_gpuvm_sm_unmap(*mut drm_gpuvm, *mut c_void, u64, u64) -> i32; pub fn drm_gpuvm_sm_map_exec_lock(*mut drm_gpuvm, *mut drm_exec, u32, *mut drm_gpuvm_map_req) -> i32; pub fn drm_gpuvm_sm_unmap_exec_lock(*mut drm_gpuvm, *mut drm_exec, u64, u64) -> i32;
    pub fn drm_gpuva_map(*mut drm_gpuvm, *mut drm_gpuva, *const drm_gpuva_op_map); pub fn drm_gpuva_remap(*mut drm_gpuva, *mut drm_gpuva, *const drm_gpuva_op_remap); pub fn drm_gpuva_unmap(*const drm_gpuva_op_unmap);
}

#[repr(C)] #[derive(Copy, Clone)] pub enum dma_resv_usage { DMA_RESV_USAGE_BOOKKEEP = 0, DMA_RESV_USAGE_READ, DMA_RESV_USAGE_WRITE, DMA_RESV_USAGE_KERNEL }

extern "C" { fn kref_get(*mut kref); fn drm_exec_fini(*mut drm_exec); }
pub unsafe fn drm_gpuva_invalidate(va: *mut drm_gpuva, invalidate: bool) { if invalidate { (*va).flags = core::mem::transmute(((*va).flags as u32) | 1); } else { (*va).flags = core::mem::transmute(((*va).flags as u32) & !1); } }
pub unsafe fn drm_gpuva_invalidated(va: *mut drm_gpuva) -> bool { ((*va).flags as u32 & 1) != 0 }
pub unsafe fn drm_gpuvm_get(gpuvm: *mut drm_gpuvm) -> *mut drm_gpuvm { kref_get(&mut (*gpuvm).kref); gpuvm }
pub unsafe fn drm_gpuvm_bo_get(vm_bo: *mut drm_gpuvm_bo) -> *mut drm_gpuvm_bo { kref_get(&mut (*vm_bo).kref); vm_bo }
pub unsafe fn drm_gpuvm_resv_protected(gpuvm: *mut drm_gpuvm) -> bool { ((*gpuvm).flags as u32 & 1) != 0 }
pub unsafe fn drm_gpuvm_immediate_mode(gpuvm: *mut drm_gpuvm) -> bool { ((*gpuvm).flags as u32 & 2) != 0 }
pub unsafe fn drm_gpuvm_is_extobj(gpuvm: *mut drm_gpuvm, obj: *mut drm_gem_object) -> bool { !obj.is_null() && (*obj).resv != (*(*gpuvm).r_obj).resv }
pub unsafe fn drm_gpuvm_exec_unlock(vm_exec: *mut drm_gpuvm_exec) { drm_exec_fini(&mut (*vm_exec).exec) }
pub unsafe fn drm_gpuva_init_from_op(va: *mut drm_gpuva, op: *const drm_gpuva_op_map) { (*va).va = (*op).va; (*va).gem.obj = (*op).gem.obj; (*va).gem.offset = (*op).gem.offset; }
pub unsafe fn drm_gpuvm_exec_validate(vm_exec: *mut drm_gpuvm_exec) -> i32 { drm_gpuvm_validate((*vm_exec).vm, &mut (*vm_exec).exec) }
pub unsafe fn drm_gpuvm_exec_resv_add_fence(vm_exec: *mut drm_gpuvm_exec, fence: *mut dma_fence, private_usage: dma_resv_usage, extobj_usage: dma_resv_usage) { drm_gpuvm_resv_add_fence((*vm_exec).vm, &mut (*vm_exec).exec, fence, private_usage, extobj_usage) }
pub unsafe fn __drm_gpuva_next(va: *mut drm_gpuva) -> *mut drm_gpuva { if va.is_null() { core::ptr::null_mut() } else { core::ptr::null_mut() /* list traversal is supplied by the kernel list dependency */ } }
pub unsafe fn drm_gpuva_op_remap_to_unmap_range(op: *const drm_gpuva_op_remap, start_addr: *mut u64, range: *mut u64) { let s = if !(*op).prev.is_null() { (*(*op).prev).va.addr.wrapping_add((*(*op).prev).va.range) } else { (*(*(*op).unmap).va).va.addr }; let e = if !(*op).next.is_null() { (*(*op).next).va.addr } else { (*(*(*op).unmap).va).va.addr.wrapping_add((*(*(*op).unmap).va).va.range) }; if !start_addr.is_null() { *start_addr = s; } if !range.is_null() { *range = e.wrapping_sub(s); } }

// The C list iteration and access macros retain their names and intent here;
// concrete expansion is provided by the kernel list dependency.
macro_rules! drm_gpuvm_resv { ($gpuvm:expr) => { unsafe { (*(*$gpuvm).r_obj).resv } }; }
macro_rules! drm_gpuvm_resv_obj { ($gpuvm:expr) => { unsafe { (*$gpuvm).r_obj } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
