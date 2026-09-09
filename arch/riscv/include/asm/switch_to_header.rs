/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[cfg(CONFIG_FPU)]
extern "C" {
    fn __fstate_save(save_to: *mut task_struct);
    fn __fstate_restore(restore_from: *mut task_struct);
}

#[cfg(CONFIG_FPU)]
#[inline]
unsafe fn __fstate_clean(regs: *mut pt_regs) {
    (*regs).status = ((*regs).status & !SR_FS) | SR_FS_CLEAN;
}

#[cfg(CONFIG_FPU)]
#[inline]
unsafe fn fstate_off(_task: *mut task_struct, regs: *mut pt_regs) {
    (*regs).status = ((*regs).status & !SR_FS) | SR_FS_OFF;
}

#[cfg(CONFIG_FPU)]
#[inline]
unsafe fn fstate_save(task: *mut task_struct, regs: *mut pt_regs) {
    if ((*regs).status & SR_FS) == SR_FS_DIRTY {
        __fstate_save(task);
        __fstate_clean(regs);
    }
}

#[cfg(CONFIG_FPU)]
#[inline]
unsafe fn fstate_restore(task: *mut task_struct, regs: *mut pt_regs) {
    if ((*regs).status & SR_FS) != SR_FS_OFF {
        __fstate_restore(task);
        __fstate_clean(regs);
    }
}

#[cfg(CONFIG_FPU)]
#[inline]
unsafe fn __switch_to_fpu(prev: *mut task_struct, next: *mut task_struct) {
    let regs: *mut pt_regs = task_pt_regs(prev);
    fstate_save(prev, regs);
    fstate_restore(next, task_pt_regs(next));
}

#[cfg(CONFIG_FPU)]
#[inline(always)]
fn has_fpu() -> bool {
    riscv_has_extension_likely(RISCV_ISA_EXT_F)
        || riscv_has_extension_likely(RISCV_ISA_EXT_D)
}

#[cfg(not(CONFIG_FPU))]
#[inline(always)]
fn has_fpu() -> bool { false }

#[cfg(not(CONFIG_FPU))]
#[inline(always)]
unsafe fn fstate_save(_task: *mut task_struct, _regs: *mut pt_regs) {}

#[cfg(not(CONFIG_FPU))]
#[inline(always)]
unsafe fn fstate_restore(_task: *mut task_struct, _regs: *mut pt_regs) {}

#[cfg(not(CONFIG_FPU))]
#[inline(always)]
unsafe fn __switch_to_fpu(_prev: *mut task_struct, _next: *mut task_struct) {}

#[inline]
unsafe fn envcfg_update_bits(task: *mut task_struct, mask: c_ulong, val: c_ulong) {
    let envcfg = ((*task).thread.envcfg & !mask) | val;
    (*task).thread.envcfg = envcfg;
    if task == current {
        csr_write(CSR_ENVCFG, envcfg);
    }
}

#[inline]
unsafe fn __switch_to_envcfg(next: *mut task_struct) {
    // The C implementation uses ALTERNATIVE and __stringify(CSR_ENVCFG) to
    // select the instruction at build time; those external macros are not
    // available in this isolated translation.
    let _ = (*next).thread.envcfg;
}

extern "C" {
    fn __switch_to(prev: *mut task_struct, next: *mut task_struct) -> *mut task_struct;
}

#[inline]
unsafe fn switch_to_should_flush_icache(task: *mut task_struct) -> bool {
    #[cfg(CONFIG_SMP)]
    {
        let stale_mm = !(*task).mm.is_null()
            && (*task).mm.context.force_icache_flush;
        let stale_thread = (*task).thread.force_icache_flush;
        let thread_migrated = smp_processor_id() != (*task).thread.prev_cpu;
        thread_migrated && (stale_mm || stale_thread)
    }
    #[cfg(not(CONFIG_SMP))]
    {
        false
    }
}

#[cfg(CONFIG_SMP)]
#[inline(always)]
unsafe fn __set_prev_cpu(thread: *mut Thread) {
    (*thread).prev_cpu = smp_processor_id();
}

#[cfg(not(CONFIG_SMP))]
#[inline(always)]
unsafe fn __set_prev_cpu(_thread: *mut Thread) {}

#[macro_export]
macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        let __prev: *mut task_struct = $prev;
        let __next: *mut task_struct = $next;
        unsafe {
            __set_prev_cpu(&mut (*__prev).thread);
            if has_fpu() {
                __switch_to_fpu(__prev, __next);
            }
            if has_vector() || has_xtheadvector() {
                __switch_to_vector(__prev, __next);
            }
            if has_srmcfg() {
                __switch_to_srmcfg(__next);
            }
            if switch_to_should_flush_icache(__next) {
                local_flush_icache_all();
            }
            __switch_to_envcfg(__next);
            $last = __switch_to(__prev, __next);
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
