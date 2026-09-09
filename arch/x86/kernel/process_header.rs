/* SPDX-License-Identifier: GPL-2.0 */
//
// Code shared between 32 and 64 bit

// Dependency intent: <asm/spec-ctrl.h>

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn __switch_to_xtra(prev_p: *mut task_struct, next_p: *mut task_struct);

    pub fn read_task_thread_flags(task: *mut task_struct) -> libc::c_ulong;
    pub fn static_branch_likely(key: *const libc::c_void) -> bool;
}

// External symbols and constants supplied by other translation units.
unsafe extern "C" {
    pub static switch_to_cond_stibp: libc::c_void;
}

/*
 * This needs to be inline to optimize for the common case where no extra
 * work needs to be done.
 */
#[inline]
pub unsafe fn switch_to_extra(prev: *mut task_struct, next: *mut task_struct) {
    let mut next_tif: libc::c_ulong = read_task_thread_flags(next);
    let mut prev_tif: libc::c_ulong = read_task_thread_flags(prev);

    // Build-time condition corresponding to IS_ENABLED(CONFIG_SMP).
    if cfg!(feature = "CONFIG_SMP") {
        /*
         * Avoid __switch_to_xtra() invocation when conditional
         * STIBP is disabled and the only different bit is
         * TIF_SPEC_IB. For CONFIG_SMP=n TIF_SPEC_IB is not
         * in the TIF_WORK_CTXSW masks.
         */
        if !static_branch_likely(&switch_to_cond_stibp) {
            prev_tif &= !_TIF_SPEC_IB;
            next_tif &= !_TIF_SPEC_IB;
        }
    }

    /*
     * __switch_to_xtra() handles debug registers, i/o bitmaps,
     * speculation mitigations etc.
     */
    if (next_tif & _TIF_WORK_CTXSW_NEXT != 0)
        || (prev_tif & _TIF_WORK_CTXSW_PREV != 0)
    {
        __switch_to_xtra(prev, next);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
