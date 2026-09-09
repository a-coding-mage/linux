/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard __ASM_SH_SWITCH_TO_32_H */

#[cfg(feature = "CONFIG_SH_DSP")]
#[inline]
pub unsafe fn is_dsp_enabled(tsk: *const task_struct) -> bool {
    ((*tsk).thread.dsp_status.status & SR_DSP) != 0
}

#[cfg(feature = "CONFIG_SH_DSP")]
#[inline]
pub unsafe fn __restore_dsp(tsk: *mut task_struct) {
    let mut ts2 = &mut (*tsk).thread.dsp_status as *mut _ as *mut u32;
    /* The following is the SH DSP register restore sequence from the C macro. */
    core::arch::asm!(
        ".balign 4", "movs.l @r2+, a0", "movs.l @r2+, a1",
        "movs.l @r2+, a0g", "movs.l @r2+, a1g", "movs.l @r2+, m0",
        "movs.l @r2+, m1", "movs.l @r2+, x0", "movs.l @r2+, x1",
        "movs.l @r2+, y0", "movs.l @r2+, y1", "lds.l @r2+, dsr",
        "ldc.l @r2+, rs", "ldc.l @r2+, re", "ldc.l @r2+, mod",
        in("r2") ts2, options(nostack, preserves_flags)
    );
}

#[cfg(feature = "CONFIG_SH_DSP")]
#[inline]
pub unsafe fn __save_dsp(tsk: *mut task_struct) {
    let mut ts2 = (&mut (*tsk).thread.dsp_status as *mut _ as *mut u32).add(14);
    core::arch::asm!(
        ".balign 4", "stc.l mod, @-r2", "stc.l re, @-r2",
        "stc.l rs, @-r2", "sts.l dsr, @-r2", "movs.l y1, @-r2",
        "movs.l y0, @-r2", "movs.l x1, @-r2", "movs.l x0, @-r2",
        "movs.l m1, @-r2", "movs.l m0, @-r2", "movs.l a1g, @-r2",
        "movs.l a0g, @-r2", "movs.l a1, @-r2", "movs.l a0, @-r2",
        in("r2") ts2, options(nostack, preserves_flags)
    );
}

#[cfg(not(feature = "CONFIG_SH_DSP"))]
#[inline]
pub unsafe fn is_dsp_enabled(_tsk: *const task_struct) -> bool { false }

#[cfg(not(feature = "CONFIG_SH_DSP"))]
#[inline]
pub unsafe fn __save_dsp(_tsk: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_SH_DSP"))]
#[inline]
pub unsafe fn __restore_dsp(_tsk: *mut task_struct) {}

extern "C" {
    pub fn __switch_to(prev: *mut task_struct, next: *mut task_struct) -> *mut task_struct;
}

/* switch_to() should switch tasks to task nr n, first. */
#[macro_export]
macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        unsafe {
            if $crate::is_dsp_enabled($prev) { $crate::__save_dsp($prev); }
            if $crate::is_dsp_enabled($next) { $crate::__restore_dsp($next); }
            let __last = $crate::__switch_to($prev, $next);
            $last = __last;
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
