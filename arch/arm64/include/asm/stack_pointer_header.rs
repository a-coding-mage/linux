/* SPDX-License-Identifier: GPL-2.0 */

/*
 * How to get the current stack pointer from C.
 *
 * The C declaration binds this register variable to the architecture's
 * stack-pointer register ("sp"). Rust has no direct equivalent for a
 * register-bound external declaration, so the externally supplied symbol is
 * declared here while preserving that binding intent.
 */
unsafe extern "C" {
    pub static mut current_stack_pointer: core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
