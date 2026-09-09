/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: linux/thread_info.h and asm/smp_plat.h.

/*
 * For v7 SMP cores running a preemptible kernel we may be pre-empted
 * during a TLB maintenance operation, so execute an inner-shareable dsb
 * to ensure that the maintenance completes in case we migrate to another
 * CPU.
 */
#[cfg(all(CONFIG_PREEMPTION, CONFIG_SMP, CONFIG_CPU_V7))]
macro_rules! __complete_pending_tlbi {
    () => {
        dsb(ish)
    };
}

#[cfg(not(all(CONFIG_PREEMPTION, CONFIG_SMP, CONFIG_CPU_V7)))]
macro_rules! __complete_pending_tlbi {
    () => {};
}

/*
 * switch_to(prev, next) should switch from task `prev' to `next'
 * `prev' will never be the same as `next'.  schedule() itself
 * contains the memory barrier to tell GCC not to cache `current'.
 */
extern "C" {
    fn __switch_to(
        prev: *mut task_struct,
        prev_thread_info: *mut thread_info,
        next_thread_info: *mut thread_info,
    ) -> *mut task_struct;
}

macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:ident) => {{
        __complete_pending_tlbi!();
        if IS_ENABLED!(CONFIG_CURRENT_POINTER_IN_TPIDRURO) || is_smp() {
            __this_cpu_write!(__entry_task, $next);
        }
        $last = unsafe {
            __switch_to(
                $prev,
                task_thread_info($prev),
                task_thread_info($next),
            )
        };
    }};
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
