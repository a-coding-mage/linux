/* SPDX-License-Identifier: GPL-2.0 */
/* NUMA memory policies for Linux. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub const NO_INTERLEAVE_INDEX: ::core::primitive::c_ulong = !0;

#[cfg(feature = "CONFIG_NUMA")]
#[repr(C)]
pub struct mempolicy {
    pub refcnt: atomic_t,
    pub mode: u16,
    pub flags: u16,
    pub nodes: nodemask_t,
    pub home_node: ::core::ffi::c_int,
    pub w: mempolicy_w,
    pub rcu: rcu_head,
}

#[cfg(feature = "CONFIG_NUMA")]
#[repr(C)]
pub union mempolicy_w {
    pub cpuset_mems_allowed: nodemask_t,
    pub user_nodemask: nodemask_t,
}

#[cfg(feature = "CONFIG_NUMA")]
extern "C" {
    pub fn __mpol_put(pol: *mut mempolicy);
    pub fn __mpol_dup(pol: *mut mempolicy) -> *mut mempolicy;
    pub fn __mpol_equal(a: *mut mempolicy, b: *mut mempolicy) -> bool;
}

#[cfg(feature = "CONFIG_NUMA")]
#[inline]
pub unsafe fn mpol_put(pol: *mut mempolicy) {
    if !pol.is_null() { __mpol_put(pol); }
}

#[cfg(feature = "CONFIG_NUMA")]
#[inline]
pub unsafe fn mpol_needs_cond_ref(pol: *mut mempolicy) -> ::core::ffi::c_int {
    ( (!pol.is_null()) && ((*pol).flags & MPOL_F_SHARED != 0) ) as ::core::ffi::c_int
}

#[cfg(feature = "CONFIG_NUMA")]
#[inline]
pub unsafe fn mpol_cond_put(pol: *mut mempolicy) {
    if mpol_needs_cond_ref(pol) != 0 { __mpol_put(pol); }
}

#[cfg(feature = "CONFIG_NUMA")]
#[inline]
pub unsafe fn mpol_dup(mut pol: *mut mempolicy) -> *mut mempolicy {
    if !pol.is_null() { pol = __mpol_dup(pol); }
    pol
}

#[cfg(feature = "CONFIG_NUMA")]
#[inline]
pub unsafe fn mpol_get(pol: *mut mempolicy) {
    if !pol.is_null() { atomic_inc(&mut (*pol).refcnt); }
}

#[cfg(feature = "CONFIG_NUMA")]
#[inline]
pub unsafe fn mpol_equal(a: *mut mempolicy, b: *mut mempolicy) -> bool {
    if a == b { true } else { __mpol_equal(a, b) }
}

#[cfg(feature = "CONFIG_NUMA")]
#[repr(C)]
pub struct shared_policy { pub root: rb_root, pub lock: rwlock_t }

#[cfg(feature = "CONFIG_NUMA")]
#[repr(C)]
pub struct sp_node {
    pub nd: rb_node,
    pub start: pgoff_t,
    pub end: pgoff_t,
    pub policy: *mut mempolicy,
}

#[cfg(feature = "CONFIG_NUMA")]
extern "C" {
    pub fn vma_dup_policy(src: *mut vm_area_struct, dst: *mut vm_area_struct) -> ::core::ffi::c_int;
    pub fn mpol_shared_policy_init(sp: *mut shared_policy, mpol: *mut mempolicy);
    pub fn mpol_set_shared_policy(sp: *mut shared_policy, vma: *mut vm_area_struct, mpol: *mut mempolicy) -> ::core::ffi::c_int;
    pub fn mpol_free_shared_policy(sp: *mut shared_policy);
    pub fn mpol_shared_policy_lookup(sp: *mut shared_policy, idx: pgoff_t) -> *mut mempolicy;
    pub fn get_task_policy(p: *mut task_struct) -> *mut mempolicy;
    pub fn __get_vma_policy(vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong, ilx: *mut pgoff_t) -> *mut mempolicy;
    pub fn get_vma_policy(vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong, order: ::core::ffi::c_int, ilx: *mut pgoff_t) -> *mut mempolicy;
    pub fn vma_policy_mof(vma: *mut vm_area_struct) -> bool;
    pub fn numa_default_policy();
    pub fn numa_policy_init();
    pub fn mpol_rebind_task(tsk: *mut task_struct, new: *const nodemask_t);
    pub fn mpol_rebind_mm(mm: *mut mm_struct, new: *mut nodemask_t);
    pub fn huge_node(vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong, gfp_flags: gfp_t, mpol: *mut *mut mempolicy, nodemask: *mut *mut nodemask_t) -> ::core::ffi::c_int;
    pub fn init_nodemask_of_mempolicy(mask: *mut nodemask_t) -> bool;
    pub fn mempolicy_in_oom_domain(tsk: *mut task_struct, mask: *const nodemask_t) -> bool;
    pub fn mempolicy_slab_node() -> ::core::ffi::c_uint;
    pub static mut policy_zone: zone_type;
    pub fn do_migrate_pages(mm: *mut mm_struct, from: *const nodemask_t, to: *const nodemask_t, flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn mpol_to_str(buffer: *mut ::core::ffi::c_char, maxlen: ::core::ffi::c_int, pol: *mut mempolicy);
    pub fn vma_migratable(vma: *mut vm_area_struct) -> bool;
    pub fn mpol_misplaced(folio: *mut folio, vmf: *mut vm_fault, addr: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn mpol_put_task_policy(task: *mut task_struct);
    pub fn apply_policy_zone(policy: *mut mempolicy, zone: zone_type) -> bool;
    pub fn mempolicy_set_node_perf(node: ::core::ffi::c_uint, coords: *mut access_coordinate) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_NUMA"))]
#[repr(C)]
pub struct mempolicy {}

// CONFIG_NUMA-disabled inline implementations.
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn get_task_policy(_: *mut task_struct) -> *mut mempolicy { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn mpol_equal(_: *mut mempolicy, _: *mut mempolicy) -> bool { true }
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn mpol_put(_: *mut mempolicy) {}
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn mpol_cond_put(_: *mut mempolicy) {}
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn mpol_get(_: *mut mempolicy) {}
#[cfg(not(feature = "CONFIG_NUMA"))]
#[repr(C)] pub struct shared_policy {}
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn mpol_shared_policy_init(_: *mut shared_policy, _: *mut mempolicy) {}
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn mpol_free_shared_policy(_: *mut shared_policy) {}
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn mpol_shared_policy_lookup(_: *mut shared_policy, _: pgoff_t) -> *mut mempolicy { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn get_vma_policy(_: *mut vm_area_struct, _: ::core::ffi::c_ulong, _: ::core::ffi::c_int, ilx: *mut pgoff_t) -> *mut mempolicy { *ilx = 0; core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn vma_dup_policy(_: *mut vm_area_struct, _: *mut vm_area_struct) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn numa_policy_init() {}
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn numa_default_policy() {}
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn mpol_rebind_task(_: *mut task_struct, _: *const nodemask_t) {}
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn mpol_rebind_mm(_: *mut mm_struct, _: *mut nodemask_t) {}
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn huge_node(_: *mut vm_area_struct, _: ::core::ffi::c_ulong, _: gfp_t, mpol: *mut *mut mempolicy, nodemask: *mut *mut nodemask_t) -> ::core::ffi::c_int { *mpol = core::ptr::null_mut(); *nodemask = core::ptr::null_mut(); 0 }
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn init_nodemask_of_mempolicy(_: *mut nodemask_t) -> bool { false }
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn do_migrate_pages(_: *mut mm_struct, _: *const nodemask_t, _: *const nodemask_t, _: ::core::ffi::c_int) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn check_highest_zone(_: ::core::ffi::c_int) {}
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn mpol_misplaced(_: *mut folio, _: *mut vm_fault, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int { -1 }
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn mpol_put_task_policy(_: *mut task_struct) {}
#[cfg(not(feature = "CONFIG_NUMA"))]
#[inline] pub unsafe fn mpol_is_preferred_many(_: *mut mempolicy) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
