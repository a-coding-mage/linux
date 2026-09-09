/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_CSKY_SWITCH_TO_H

// Dependency supplied by the surrounding kernel translation:
// #include <linux/thread_info.h>

#[cfg(CONFIG_CPU_HAS_FPU)]
// Dependency supplied by the surrounding kernel translation:
// #include <abi/fpu.h>

#[cfg(CONFIG_CPU_HAS_FPU)]
#[inline(always)]
pub unsafe fn __switch_to_fpu(prev: *mut task_struct, next: *mut task_struct) {
    save_to_user_fp(&mut (*prev).thread.user_fp);
    restore_from_user_fp(&mut (*next).thread.user_fp);
}

#[cfg(not(CONFIG_CPU_HAS_FPU))]
#[inline(always)]
pub unsafe fn __switch_to_fpu(_prev: *mut task_struct, _next: *mut task_struct) {}

/*
 * Context switching is now performed out-of-line in switch_to.S
 */
extern "C" {
    pub fn __switch_to(prev: *mut task_struct, next: *mut task_struct) -> *mut task_struct;
}

macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        let __prev: *mut task_struct = $prev;
        let __next: *mut task_struct = $next;
        unsafe {
            __switch_to_fpu(__prev, __next);
            $last = __switch_to($prev, $next);
        }
    }};
}

// External declarations supplied by the surrounding kernel translation.
extern "C" {
    fn save_to_user_fp(fp: *mut user_fp_state);
    fn restore_from_user_fp(fp: *mut user_fp_state);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
