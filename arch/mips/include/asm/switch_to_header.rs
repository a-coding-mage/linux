/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994, 95, 96, 97, 98, 99, 2003, 06 by Ralf Baechle
 * Copyright (C) 1996 by Paul M. Antoine
 * Copyright (C) 1999 Silicon Graphics
 * Kevin D. Kissell, kevink@mips.org and Carsten Langgaard, carstenl@mips.com
 * Copyright (C) 2000 MIPS Technologies, Inc.
 */

// C dependencies: asm/cpu-features.h, asm/watch.h, asm/dsp.h,
// asm/cop2.h, and asm/fpu.h.

pub struct task_struct;
pub struct thread_info;

/// resume - resume execution of a task
/// @prev: The task previously executed.
/// @next: The task to begin executing.
/// @next_ti: task_thread_info(next).
///
/// This function is used whilst scheduling to save the context of prev and
/// load the context of next. Returns prev.
extern "C" {
    pub fn resume(
        prev: *mut task_struct,
        next: *mut task_struct,
        next_ti: *mut thread_info,
    ) -> *mut task_struct;

    pub static mut ll_bit: u32;
    pub static mut ll_task: *mut task_struct;
}

// CONFIG_MIPS_MT_FPAFF controls the conditional definition in the source.
#[cfg(CONFIG_MIPS_MT_FPAFF)]
#[macro_export]
macro_rules! __mips_mt_fpaff_switch_to {
    ($prev:expr, $next:expr) => {{
        let __prev_ti = task_thread_info($prev);
        if cpu_has_fpu
            && test_ti_thread_flag(__prev_ti, TIF_FPUBOUND)
            && (!(KSTK_STATUS($prev) & ST0_CU1))
        {
            clear_ti_thread_flag(__prev_ti, TIF_FPUBOUND);
            (*$prev).cpus_mask = (*$prev).thread.user_cpus_allowed;
        }
        (*$next).thread.emulated_fp = 0;
    }};
}

#[cfg(not(CONFIG_MIPS_MT_FPAFF))]
#[macro_export]
macro_rules! __mips_mt_fpaff_switch_to {
    ($prev:expr, $next:expr) => {{
        let _ = $prev;
        let _ = $next;
    }};
}

#[macro_export]
macro_rules! __clear_r5_hw_ll_bit {
    () => {{
        if cpu_has_mips_r5 || cpu_has_mips_r6 {
            write_c0_lladdr(0);
        }
    }};
}

#[macro_export]
macro_rules! __clear_software_ll_bit {
    () => {{
        if !__builtin_constant_p(cpu_has_llsc) || !cpu_has_llsc {
            ll_bit = 0;
        }
    }};
}

// CONFIG_MIPS_FP_SUPPORT controls whether FCSR exceptions are sanitized.
#[cfg(CONFIG_MIPS_FP_SUPPORT)]
#[macro_export]
macro_rules! __sanitize_fcr31 {
    ($next:expr) => {{
        let fcr31 = mask_fcr31_x((*$next).thread.fpu.fcr31);
        let mut pc: *mut core::ffi::c_void;
        if unlikely(fcr31 != 0) {
            pc = task_pt_regs($next).cp0_epc as *mut core::ffi::c_void;
            (*$next).thread.fpu.fcr31 &= !fcr31;
            force_fcr31_sig(fcr31, pc, $next);
        }
    }};
}

#[cfg(not(CONFIG_MIPS_FP_SUPPORT))]
#[macro_export]
macro_rules! __sanitize_fcr31 {
    ($next:expr) => {{
        let _ = $next;
    }};
}

#[macro_export]
macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        __mips_mt_fpaff_switch_to!($prev, $next);
        lose_fpu_inatomic(1, $prev);
        if tsk_used_math($next) {
            __sanitize_fcr31!($next);
        }
        if cpu_has_dsp {
            __save_dsp($prev);
            __restore_dsp($next);
        }
        if cop2_present {
            let status: u32 = read_c0_status();
            set_c0_status(ST0_CU2);
            if KSTK_STATUS($prev) & ST0_CU2 != 0 {
                if cop2_lazy_restore {
                    KSTK_STATUS($prev) &= !ST0_CU2;
                }
                cop2_save($prev);
            }
            if KSTK_STATUS($next) & ST0_CU2 != 0 && !cop2_lazy_restore {
                cop2_restore($next);
            }
            write_c0_status(status);
        }
        __clear_r5_hw_ll_bit!();
        __clear_software_ll_bit!();
        if cpu_has_userlocal {
            write_c0_userlocal((*task_thread_info($next)).tp_value);
        }
        __restore_watch($next);
        $last = resume($prev, $next, task_thread_info($next));
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
