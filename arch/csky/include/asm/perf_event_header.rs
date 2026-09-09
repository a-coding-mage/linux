/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding ABI definitions.

/// Fetch the caller's register state.
///
/// This is the Rust equivalent of the C macro and intentionally retains its
/// raw-pointer and inline-assembly behavior.
#[macro_export]
macro_rules! perf_arch_fetch_caller_regs {
    ($regs:expr, $ip:expr) => {{
        unsafe {
            (*$regs).pc = $ip;
            // `__builtin_frame_address(0)` denotes the current frame address.
            let frame_address: usize;
            core::arch::asm!("mov {0}, fp", out(reg) frame_address);
            regs_fp($regs) = frame_address as ::core::ffi::c_ulong;

            let user_stack_pointer: usize;
            core::arch::asm!("mov {0}, sp", out(reg) user_stack_pointer);
            (*$regs).usp = user_stack_pointer as ::core::ffi::c_ulong;
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
