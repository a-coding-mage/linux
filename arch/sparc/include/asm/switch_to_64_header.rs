/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding SPARC implementation.

#[inline(always)]
pub unsafe fn prepare_arch_switch<T>(_next: *mut T) {
    flushw_all();
}

/*
 * See what happens when you design the chip correctly?
 *
 * The original C macro uses an SPARC register-clobbering inline assembly
 * sequence to switch register windows and task state.  It is retained here
 * as a Rust macro so the operation, ordering, and externally visible names
 * remain explicit at the call site.
 */
#[macro_export]
macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        unsafe {
            save_and_clear_fpu();
            core::arch::asm!(
                "wr %g0, {asi}, %asi",
                asi = in(reg) ASI_AIUS,
                options(nostack, preserves_flags)
            );
            trap_block[current_thread_info().cpu].thread = task_thread_info($next);
            core::arch::asm!(
                "mov %g4, %g7\n\t",
                "stx %i6, [%sp + 2047 + 0x70]\n\t",
                "stx %i7, [%sp + 2047 + 0x78]\n\t",
                "rdpr %wstate, %o5\n\t",
                "stx %o6, [%g6 + {ti_ksp}]\n\t",
                "stb %o5, [%g6 + {ti_wstate}]\n\t",
                "rdpr %cwp, %o5\n\t",
                "stb %o5, [%g6 + {ti_cwp}]\n\t",
                "wrpr %g0, 15, %pil\n\t",
                "mov {next_thread}, %g6\n\t",
                "ldub [{next_thread} + {ti_cwp}], %g1\n\t",
                "wrpr %g1, %cwp\n\t",
                "ldx [%g6 + {ti_ksp}], %o6\n\t",
                "ldub [%g6 + {ti_wstate}], %o5\n\t",
                "ldub [%g6 + {ti_new_child}], %o7\n\t",
                "wrpr %o5, 0x0, %wstate\n\t",
                "ldx [%sp + 2047 + 0x70], %i6\n\t",
                "ldx [%sp + 2047 + 0x78], %i7\n\t",
                "ldx [%g6 + {ti_task}], %g4\n\t",
                "wrpr %g0, 14, %pil\n\t",
                "brz,pt %o7, switch_to_pc\n\t",
                " mov %g7, {last}\n\t",
                "sethi %hi(ret_from_fork), %g1\n\t",
                "jmpl %g1 + %lo(ret_from_fork), %g0\n\t",
                " nop\n\t",
                ".globl switch_to_pc\n\t",
                "switch_to_pc:\n\t",
                next_thread = in(reg) task_thread_info($next),
                last = lateout(reg) $last,
                ti_wstate = const TI_WSTATE,
                ti_ksp = const TI_KSP,
                ti_new_child = const TI_NEW_CHILD,
                ti_cwp = const TI_CWP,
                ti_task = const TI_TASK,
                options(nostack)
            );
        }
    }};
}

extern "C" {
    pub fn synchronize_user_stack();
    pub fn fault_in_user_windows(regs: *mut pt_regs);
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
