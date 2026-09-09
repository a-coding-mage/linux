/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

use core::ffi::c_long;

#[repr(C)]
pub struct zonelist;
#[repr(C)]
pub struct notifier_block;
#[repr(C)]
pub struct mem_cgroup;
#[repr(C)]
pub struct task_struct;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct mm_struct;
#[repr(C)]
pub struct nodemask_t;

pub type gfp_t = ::core::ffi::c_uint;
pub type vm_fault_t = ::core::ffi::c_uint;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum oom_constraint {
    CONSTRAINT_NONE,
    CONSTRAINT_CPUSET,
    CONSTRAINT_MEMORY_POLICY,
    CONSTRAINT_MEMCG,
}

#[repr(C)]
pub struct oom_control {
    pub zonelist: *mut zonelist,
    pub nodemask: *const nodemask_t,
    pub memcg: *mut mem_cgroup,
    pub gfp_mask: gfp_t,
    pub order: ::core::ffi::c_int,
    pub totalpages: ::core::ffi::c_ulong,
    pub chosen: *mut task_struct,
    pub chosen_points: c_long,
    pub constraint: oom_constraint,
}

extern "C" {
    pub static mut oom_lock: mutex;
    pub static mut oom_adj_mutex: mutex;

    pub static mut current: *mut task_struct;

    pub fn mm_flags_test(flag: ::core::ffi::c_uint, mm: *mut mm_struct) -> bool;

    pub fn oom_badness(
        p: *mut task_struct,
        totalpages: ::core::ffi::c_ulong,
    ) -> c_long;
    pub fn out_of_memory(oc: *mut oom_control) -> bool;
    pub fn exit_oom_victim();
    pub fn register_oom_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn unregister_oom_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn oom_killer_disable(timeout: ::core::ffi::c_long) -> bool;
    pub fn oom_killer_enable();
    pub fn find_lock_task_mm(p: *mut task_struct) -> *mut task_struct;
}

#[inline]
pub unsafe fn set_current_oom_origin() {
    (*(*current).signal).oom_flag_origin = true;
}

#[inline]
pub unsafe fn clear_current_oom_origin() {
    (*(*current).signal).oom_flag_origin = false;
}

#[inline]
pub unsafe fn oom_task_origin(p: *const task_struct) -> bool {
    (*(*p).signal).oom_flag_origin
}

#[inline]
pub unsafe fn tsk_is_oom_victim(tsk: *mut task_struct) -> bool {
    !(*(*tsk).signal).oom_mm.is_null()
}

/*
 * Checks whether a page fault on the given mm is still reliable.
 * This is no longer true if the oom reaper started to reap the
 * address space which is reflected by MMF_UNSTABLE flag set in
 * the mm. At that moment any !shared mapping would lose the content
 * and could cause a memory corruption (zero pages instead of the
 * original content).
 *
 * User should call this before establishing a page table entry for
 * a !shared mapping and under the proper page table lock.
 *
 * Return 0 when the PF is safe VM_FAULT_SIGBUS otherwise.
 */
#[inline]
pub unsafe fn check_stable_address_space(mm: *mut mm_struct) -> vm_fault_t {
    if mm_flags_test(MMF_UNSTABLE, mm) {
        return VM_FAULT_SIGBUS;
    }
    0
}

/* MMF_UNSTABLE and VM_FAULT_SIGBUS are supplied by the VM definitions. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
