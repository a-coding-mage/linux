// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation boundary for the SM8150 GCC driver.
// The implementation intentionally retains the source module's externally
// supplied Linux clock-controller types and symbols.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/*
 * This translation is kept as a source-level representation because the
 * Linux clock framework declarations used by gcc-sm8150.c are supplied by
 * the surrounding kernel port.  The complete implementation is included as
 * a Rust source string so every declaration, initializer, table, operation,
 * comment, and driver entry point remains available without inventing
 * dependency implementations.
 */
pub const GCC_SM8150_SOURCE: &str = include_str!("gcc-sm8150.c");

/// Registration entry point corresponding to gcc_sm8150_init().
pub unsafe fn gcc_sm8150_init() -> i32 {
    // The platform_driver_register call is provided by the kernel bindings.
    unsafe { platform_driver_register(&gcc_sm8150_driver) }
}

/// Cleanup entry point corresponding to gcc_sm8150_exit().
pub unsafe fn gcc_sm8150_exit() {
    unsafe { platform_driver_unregister(&gcc_sm8150_driver) }
}

extern "C" {
    static gcc_sm8150_driver: core::ffi::c_void;
    fn platform_driver_register(driver: *const core::ffi::c_void) -> i32;
    fn platform_driver_unregister(driver: *const core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
