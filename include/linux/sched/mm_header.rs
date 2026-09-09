/* SPDX-License-Identifier: GPL-2.0 */

// Routines for handling mm_structs. C header dependencies are supplied by
// other translated units.

extern "C" {
    pub fn mm_alloc() -> *mut mm_struct;
    pub fn __mmdrop(mm: *mut mm_struct);
    pub fn mmput(mm: *mut mm_struct);
    pub fn get_task_mm(task: *mut task_struct) -> *mut mm_struct;
    pub fn mm_access(task: *mut task_struct, mode: u32) -> *mut mm_struct;
    pub fn mm_exit_exec_release(task: *mut task_struct, mm: *mut mm_struct);
}

#[inline]
pub unsafe fn mmgrab(mm: *mut mm_struct) {
    atomic_inc(&mut (*mm).mm_count);
}

#[inline]
pub unsafe fn smp_mb__after_mmgrab() {
    smp_mb__after_atomic();
}

#[inline]
pub unsafe fn mmdrop(mm: *mut mm_struct) {
    // atomic_dec_and_test() supplies the implicit full barrier required by
    // membarrier before returning to user space, after storing to rq->curr.
    if unlikely(atomic_dec_and_test(&mut (*mm).mm_count)) {
        __mmdrop(mm);
    }
}

// CONFIG_PREEMPT_RT: RCU callback for delayed mm drop.
#[cfg(feature = "CONFIG_PREEMPT_RT")]
#[inline]
pub unsafe fn __mmdrop_delayed(rhp: *mut rcu_head) {
    let mm = container_of!(rhp, mm_struct, delayed_drop);
    __mmdrop(mm);
}

#[cfg(feature = "CONFIG_PREEMPT_RT")]
#[inline]
pub unsafe fn mmdrop_sched(mm: *mut mm_struct) {
    // Provides a full memory barrier.
    if atomic_dec_and_test(&mut (*mm).mm_count) {
        call_rcu(&mut (*mm).delayed_drop, __mmdrop_delayed);
    }
}

#[cfg(not(feature = "CONFIG_PREEMPT_RT"))]
#[inline]
pub unsafe fn mmdrop_sched(mm: *mut mm_struct) {
    mmdrop(mm);
}

// Helpers for lazy TLB mm refcounting.
#[inline]
pub unsafe fn mmgrab_lazy_tlb(mm: *mut mm_struct) {
    if IS_ENABLED!(CONFIG_MMU_LAZY_TLB_REFCOUNT) {
        mmgrab(mm);
    }
}

#[inline]
pub unsafe fn mmdrop_lazy_tlb(mm: *mut mm_struct) {
    if IS_ENABLED!(CONFIG_MMU_LAZY_TLB_REFCOUNT) {
        mmdrop(mm);
    } else {
        // mmdrop_lazy_tlb must provide a full memory barrier.
        smp_mb();
    }
}

#[inline]
pub unsafe fn mmdrop_lazy_tlb_sched(mm: *mut mm_struct) {
    if IS_ENABLED!(CONFIG_MMU_LAZY_TLB_REFCOUNT) {
        mmdrop_sched(mm);
    } else {
        smp_mb();
    }
}

#[inline]
pub unsafe fn mmget(mm: *mut mm_struct) {
    atomic_inc(&mut (*mm).mm_users);
}

#[inline]
pub unsafe fn mmget_not_zero(mm: *mut mm_struct) -> bool {
    atomic_inc_not_zero(&mut (*mm).mm_users)
}

// CONFIG_MMU || CONFIG_FUTEX_PRIVATE_HASH
#[cfg(any(feature = "CONFIG_MMU", feature = "CONFIG_FUTEX_PRIVATE_HASH"))]
extern "C" {
    pub fn mmput_async(mm: *mut mm_struct);
}

#[cfg(feature = "CONFIG_MEMCG")]
extern "C" {
    pub fn mm_update_next_owner(mm: *mut mm_struct);
}

#[cfg(not(feature = "CONFIG_MEMCG"))]
#[inline]
pub unsafe fn mm_update_next_owner(_mm: *mut mm_struct) {}

// CONFIG_MMU
#[cfg(feature = "CONFIG_MMU")]
extern "C" {
    pub fn arch_pick_mmap_layout(mm: *mut mm_struct, rlim_stack: *const rlimit);
    pub fn arch_get_unmapped_area(
        filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong,
        flags: c_ulong, vm_flags: vm_flags_t,
    ) -> c_ulong;
    pub fn arch_get_unmapped_area_topdown(
        filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong,
        flags: c_ulong, vm_flags: vm_flags_t,
    ) -> c_ulong;
    pub fn mm_get_unmapped_area(
        filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong,
        flags: c_ulong,
    ) -> c_ulong;
    pub fn mm_get_unmapped_area_vmaflags(
        filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong,
        flags: c_ulong, vma_flags: vma_flags_t,
    ) -> c_ulong;
    pub fn generic_get_unmapped_area(
        filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong,
        flags: c_ulong, vma_flags: vma_flags_t,
    ) -> c_ulong;
    pub fn generic_get_unmapped_area_topdown(
        filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong,
        flags: c_ulong, vma_flags: vma_flags_t,
    ) -> c_ulong;
}

// Fallbacks for architectures that do not define these C macros.
#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn arch_get_mmap_end(_addr: c_ulong, _len: c_ulong, _flags: c_ulong) -> c_ulong {
    TASK_SIZE
}

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn arch_get_mmap_base(_addr: c_ulong, base: c_ulong) -> c_ulong {
    base
}

#[cfg(not(feature = "CONFIG_MMU"))]
#[inline]
pub unsafe fn arch_pick_mmap_layout(_mm: *mut mm_struct, _rlim_stack: *const rlimit) {}

#[inline]
pub unsafe fn in_vfork(tsk: *mut task_struct) -> bool {
    // RCU is required to access real_parent when CLONE_VM and CLONE_PARENT
    // are used together. CLONE_VFORK does not imply CLONE_VM.
    rcu_read_lock();
    let ret = !(*tsk).vfork_done.is_null()
        && (*rcu_dereference((*tsk).real_parent)).mm == (*tsk).mm;
    rcu_read_unlock();
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
