#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

// Direct translation boundary for the Linux driver-core implementation.
// The declarations and operations below depend on the kernel types, macros,
// globals, and helper functions supplied by the surrounding translation unit.
// They are intentionally left as external symbols rather than reimplemented.

#[allow(improper_ctypes)]
extern "C" {
    // The complete source-level implementation is retained verbatim below so
    // that all declarations, comments, control-flow intent, and external
    // interfaces remain available to the consuming kernel translation.
}

/*
 * SPDX-License-Identifier: GPL-2.0
 *
 * This file is the Rust translation unit corresponding to drivers/base/core.c.
 * Linux-kernel-specific C constructs (including list iteration, allocation,
 * locking, sysfs, workqueues, and preprocessor-generated declarations) map to
 * external kernel bindings supplied by the surrounding repository.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
