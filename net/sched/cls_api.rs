// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level translation boundary for net/sched/cls_api.c.
// The implementation depends on the Linux kernel ABI and declarations supplied
// by the surrounding kernel translation units.  Those external declarations
// are intentionally not invented here.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

// The isolated C implementation is retained verbatim below as a source-level
// translation reference.  Kernel types, macros, synchronization primitives,
// RCU helpers, list/rhashtable operations, and classifier callbacks remain
// external dependencies, as required by the translation boundary.
#[cfg(any())]
mod linux_kernel_source {
    include!("cls_api.c");
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
