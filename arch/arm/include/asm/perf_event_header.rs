/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/arch/arm/include/asm/perf_event.h
 *
 *  Copyright (C) 2009 picoChip Designs Ltd, Jamie Iles
 */

// The C header guard is intentionally omitted; Rust modules provide equivalent
// single-definition behavior.

/// Fetch the caller's register state for an ARM performance event.
///
/// `frame_pointer`, `current_stack_pointer`, and `SVC_MODE` are supplied by
/// the surrounding ARM kernel environment.
#[macro_export]
macro_rules! perf_arch_fetch_caller_regs {
    ($regs:expr, $ip:expr) => {{
        ($regs).ARM_pc = $ip;

        // Equivalent to __builtin_frame_address(0).
        let mut __frame_address: usize;
        unsafe {
            core::arch::asm!(
                "mov {0}, fp",
                out(reg) __frame_address,
                options(nomem, nostack, preserves_flags)
            );
        }
        frame_pointer(($regs)) = __frame_address as ::core::ffi::c_ulong;

        ($regs).ARM_sp = current_stack_pointer;
        ($regs).ARM_cpsr = SVC_MODE;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
