// Translated from the PowerPC membarrier header.
// The original CONFIG_SMP condition is a build-time configuration condition.

use core::ffi::c_int;

#[repr(C)]
pub struct mm_struct {
    pub membarrier_state: atomic_t,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}

pub const MEMBARRIER_STATE_PRIVATE_EXPEDITED: c_int = 1 << 0;
pub const MEMBARRIER_STATE_GLOBAL_EXPEDITED: c_int = 1 << 1;

unsafe extern "C" {
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn smp_mb();
}

#[inline]
pub unsafe fn membarrier_arch_switch_mm(
    prev: *mut mm_struct,
    next: *mut mm_struct,
    _tsk: *mut task_struct,
) {
    /*
     * Only need the full barrier when switching between processes.
     * Barrier when switching from kernel to userspace is not
     * required here, given that it is implied by mmdrop(). Barrier
     * when switching from userspace to kernel is not needed after
     * store to rq->curr.
     */
    // IS_ENABLED(CONFIG_SMP) is preserved as build-time conditional intent.
    if ((unsafe { atomic_read(&(*next).membarrier_state) }
        & (MEMBARRIER_STATE_PRIVATE_EXPEDITED | MEMBARRIER_STATE_GLOBAL_EXPEDITED)) == 0
        || prev.is_null())
    {
        return;
    }

    /*
     * The membarrier system call requires a full memory barrier
     * after storing to rq->curr, before going back to user-space.
     */
    unsafe { smp_mb() };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
