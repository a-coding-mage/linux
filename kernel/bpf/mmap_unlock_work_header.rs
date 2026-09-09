/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2021 Facebook
 */

// C dependencies:
// linux/atomic.h, linux/err.h, linux/irq_work.h

/* irq_work to run mmap_read_unlock() in irq_work */
#[repr(C)]
pub struct mmap_unlock_irq_work {
    pub irq_work: irq_work,
    pub mm: *mut mm_struct,
    pub active: atomic_t,
}

extern "C" {
    pub static mut mmap_unlock_work: mmap_unlock_irq_work;

    fn irqs_disabled() -> bool;
    fn irq_work_is_busy(work: *const irq_work) -> bool;
    fn atomic_cmpxchg_acquire(v: *mut atomic_t, old: i32, new: i32) -> i32;
    fn atomic_set_release(v: *mut atomic_t, value: i32);
    fn mmap_read_unlock(mm: *mut mm_struct);
    fn rwsem_release(dep_map: *mut lockdep_map, ip: usize);
    fn irq_work_queue(work: *mut irq_work);
}

// The following types and constants are supplied by the translated kernel
// dependencies.
extern "C" {
    type irq_work;
    type mm_struct;
    type atomic_t;
    type lockdep_map;
}

// DECLARE_PER_CPU(struct mmap_unlock_irq_work, mmap_unlock_work)
// is represented by the external per-CPU symbol above.

/*
 * We cannot do mmap_read_unlock() when the irq is disabled, because of
 * risk to deadlock with rq_lock. To look up vma when the irqs are
 * disabled, we need to run mmap_read_unlock() in irq_work. We use a
 * percpu variable to do the irq_work. The active flag reserves the slot
 * before mmap_read_trylock() and until the irq_work callback consumes mm.
 */
#[inline]
pub unsafe fn bpf_mmap_unlock_guard_get() -> *mut mmap_unlock_irq_work {
    let work: *mut mmap_unlock_irq_work;

    if !irqs_disabled() {
        return core::ptr::null_mut();
    }

    /*
     * PREEMPT_RT does not allow to trylock mmap sem in interrupt
     * disabled context. Force the fallback code.
     */
    // if (IS_ENABLED(CONFIG_PREEMPT_RT))
    #[cfg(feature = "CONFIG_PREEMPT_RT")]
    {
        return ERR_PTR(-EBUSY);
    }

    work = this_cpu_ptr(&mmap_unlock_work);
    if irq_work_is_busy(&(*work).irq_work)
        || atomic_cmpxchg_acquire(&mut (*work).active, 0, 1) != 0
    {
        return ERR_PTR(-EBUSY);
    }

    work
}

#[inline]
pub unsafe fn bpf_mmap_unlock_guard_put(work: *mut mmap_unlock_irq_work) {
    if !work.is_null() {
        atomic_set_release(&mut (*work).active, 0);
    }
}

#[inline]
pub unsafe fn bpf_mmap_unlock_mm(
    work: *mut mmap_unlock_irq_work,
    mm: *mut mm_struct,
) {
    if work.is_null() {
        mmap_read_unlock(mm);
    } else {
        (*work).mm = mm;

        /* The lock will be released once we're out of interrupt
         * context. Tell lockdep that we've released it now so
         * it doesn't complain that we forgot to release it.
         */
        rwsem_release(&mut (*mm).mmap_lock.dep_map, _RET_IP_);
        irq_work_queue(&mut (*work).irq_work);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
