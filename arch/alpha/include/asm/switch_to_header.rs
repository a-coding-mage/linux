/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ALPHA_SWITCH_TO_H

#[repr(C)]
pub struct task_struct {
    _opaque: [u8; 0],
}

extern "C" {
    pub fn alpha_switch_to(
        arg: ::core::ffi::c_ulong,
        task: *mut task_struct,
    ) -> *mut task_struct;
}

// The symbols virt_to_phys, task_thread_info, and check_mmu_context are
// supplied by other translated dependencies.
#[macro_export]
macro_rules! switch_to {
    ($p:expr, $n:expr, $l:expr) => {{
        ($l) = unsafe {
            $crate::alpha_switch_to(
                $crate::virt_to_phys(
                    &(*$crate::task_thread_info($n)).pcb as *const _ as _,
                ),
                $p,
            )
        };
        unsafe { $crate::check_mmu_context() };
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
