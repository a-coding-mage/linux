/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

/* C header dependencies and build-time CONFIG_* conditions are preserved here
 * as Rust feature conditions; the referenced kernel types and helpers are
 * supplied by other translated files. */

#[cfg(feature = "CONFIG_RSEQ")]
extern "C" {
    pub fn __rseq_handle_slowpath(regs: *mut pt_regs);
    pub fn __rseq_signal_deliver(sig: ::core::ffi::c_int, regs: *mut pt_regs);
}

#[cfg(feature = "CONFIG_RSEQ")]
#[inline(always)]
pub unsafe fn rseq_v2(t: *mut task_struct) -> bool {
    cfg!(feature = "CONFIG_GENERIC_IRQ_ENTRY") && likely((*t).rseq.event.has_rseq > 1)
}

#[cfg(feature = "CONFIG_RSEQ")]
#[inline]
pub unsafe fn rseq_handle_slowpath(regs: *mut pt_regs) {
    if cfg!(feature = "CONFIG_GENERIC_ENTRY") {
        if (*current).rseq.event.slowpath {
            __rseq_handle_slowpath(regs);
        }
    } else if (*current).rseq.event.sched_switch && (*current).rseq.event.has_rseq {
        __rseq_handle_slowpath(regs);
    }
}

#[cfg(feature = "CONFIG_RSEQ")]
#[inline]
pub unsafe fn rseq_signal_deliver(ksig: *mut ksignal, regs: *mut pt_regs) {
    if rseq_v2(current) {
        /* has_rseq is implied in rseq_v2() */
        if (*current).rseq.event.user_irq {
            __rseq_signal_deliver((*ksig).sig, regs);
        }
    } else if (*current).rseq.event.has_rseq {
        __rseq_signal_deliver((*ksig).sig, regs);
    }
}

#[cfg(feature = "CONFIG_RSEQ")]
#[inline]
pub unsafe fn rseq_raise_notify_resume(t: *mut task_struct) {
    set_tsk_thread_flag(t, TIF_RSEQ);
}

#[cfg(feature = "CONFIG_RSEQ")]
#[inline(always)]
pub unsafe fn rseq_sched_switch_event(t: *mut task_struct) {
    let ev: *mut rseq_event = &mut (*t).rseq.event;
    if rseq_v2(t) {
        let raise = (*ev).user_irq || (*ev).ids_changed;
        if raise {
            (*ev).sched_switch = true;
            rseq_raise_notify_resume(t);
        }
    } else if (*ev).has_rseq {
        (*t).rseq.event.ids_changed = true;
        (*t).rseq.event.sched_switch = true;
        rseq_raise_notify_resume(t);
    }
}

#[cfg(feature = "CONFIG_RSEQ")]
#[inline(always)]
pub unsafe fn rseq_sched_set_ids_changed(t: *mut task_struct) {
    (*t).rseq.event.ids_changed = true;
}

#[cfg(feature = "CONFIG_RSEQ")]
#[inline]
pub unsafe fn rseq_force_update() {
    if (*current).rseq.event.has_rseq {
        (*current).rseq.event.ids_changed = true;
        (*current).rseq.event.sched_switch = true;
        rseq_raise_notify_resume(current);
    }
}

#[cfg(feature = "CONFIG_RSEQ")]
#[inline]
pub unsafe fn rseq_virt_userspace_exit() {
    if !cfg!(feature = "CONFIG_HAVE_GENERIC_TIF_BITS") && (*current).rseq.event.sched_switch {
        rseq_raise_notify_resume(current);
    }
}

#[cfg(feature = "CONFIG_RSEQ")]
#[inline]
pub unsafe fn rseq_reset(t: *mut task_struct) {
    /* Protect against preemption and membarrier IPI. */
    guard_irqsave();
    core::ptr::write_bytes(&mut (*t).rseq, 0, 1);
    (*t).rseq.ids.cpu_id = RSEQ_CPU_ID_UNINITIALIZED;
}

#[cfg(feature = "CONFIG_RSEQ")]
#[inline]
pub unsafe fn rseq_execve(t: *mut task_struct) {
    rseq_reset(t);
}

#[cfg(feature = "CONFIG_RSEQ")]
#[inline]
pub unsafe fn rseq_fork(t: *mut task_struct, clone_flags: u64) {
    if clone_flags & CLONE_VM != 0 {
        rseq_reset(t);
    } else {
        (*t).rseq = (*current).rseq;
    }
}

#[cfg(feature = "CONFIG_RSEQ")]
#[inline]
pub unsafe fn rseq_alloc_align() -> ::core::ffi::c_uint {
    1u32 << get_count_order(core::mem::offset_of!(rseq, end))
}

#[cfg(not(feature = "CONFIG_RSEQ"))]
#[inline]
pub unsafe fn rseq_v2(_t: *mut task_struct) -> bool { false }
#[cfg(not(feature = "CONFIG_RSEQ"))]
#[inline]
pub unsafe fn rseq_handle_slowpath(_regs: *mut pt_regs) {}
#[cfg(not(feature = "CONFIG_RSEQ"))]
#[inline]
pub unsafe fn rseq_signal_deliver(_ksig: *mut ksignal, _regs: *mut pt_regs) {}
#[cfg(not(feature = "CONFIG_RSEQ"))]
#[inline]
pub unsafe fn rseq_sched_switch_event(_t: *mut task_struct) {}
#[cfg(not(feature = "CONFIG_RSEQ"))]
#[inline]
pub unsafe fn rseq_sched_set_ids_changed(_t: *mut task_struct) {}
#[cfg(not(feature = "CONFIG_RSEQ"))]
#[inline]
pub unsafe fn rseq_force_update() {}
#[cfg(not(feature = "CONFIG_RSEQ"))]
#[inline]
pub unsafe fn rseq_virt_userspace_exit() {}
#[cfg(not(feature = "CONFIG_RSEQ"))]
#[inline]
pub unsafe fn rseq_fork(_t: *mut task_struct, _clone_flags: u64) {}
#[cfg(not(feature = "CONFIG_RSEQ"))]
#[inline]
pub unsafe fn rseq_execve(_t: *mut task_struct) {}

#[cfg(feature = "CONFIG_DEBUG_RSEQ")]
extern "C" { pub fn rseq_syscall(regs: *mut pt_regs); }
#[cfg(not(feature = "CONFIG_DEBUG_RSEQ"))]
#[inline]
pub unsafe fn rseq_syscall(_regs: *mut pt_regs) {}

#[cfg(feature = "CONFIG_RSEQ_SLICE_EXTENSION")]
extern "C" {
    pub fn rseq_syscall_enter_work(syscall: isize);
    pub fn rseq_slice_extension_prctl(arg2: usize, arg3: usize) -> ::core::ffi::c_int;
}
#[cfg(not(feature = "CONFIG_RSEQ_SLICE_EXTENSION"))]
#[inline]
pub unsafe fn rseq_syscall_enter_work(_syscall: isize) {}
#[cfg(not(feature = "CONFIG_RSEQ_SLICE_EXTENSION"))]
#[inline]
pub unsafe fn rseq_slice_extension_prctl(_arg2: usize, _arg3: usize) -> ::core::ffi::c_int { -ENOTSUPP }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
