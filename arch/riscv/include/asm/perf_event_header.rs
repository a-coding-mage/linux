/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 SiFive
 * Copyright (C) 2018 Andes Technology Corporation
 *
 */

// C header guard: _ASM_RISCV_PERF_EVENT_H

// CONFIG_PERF_EVENTS conditionally includes the following declarations.
// The corresponding build-time condition should be supplied by the consumer.

/// Equivalent to `perf_arch_bpf_user_pt_regs(regs)`.
#[macro_export]
macro_rules! perf_arch_bpf_user_pt_regs {
    ($regs:expr) => {
        $regs as *mut user_regs_struct
    };
}

/// Declaration corresponding to the compiler builtin used by the C macro.
unsafe extern "C" {
    fn __builtin_frame_address(level: i32) -> *mut core::ffi::c_void;
}

/// Equivalent to `perf_arch_fetch_caller_regs(regs, __ip)`.
#[macro_export]
macro_rules! perf_arch_fetch_caller_regs {
    ($regs:expr, $__ip:expr) => {{
        unsafe {
            (*$regs).epc = $__ip;
            (*$regs).s0 = __builtin_frame_address(0) as usize;
            (*$regs).sp = current_stack_pointer;
            (*$regs).status = SR_PP;
        }
    }};
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
