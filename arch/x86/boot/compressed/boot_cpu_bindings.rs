// SPDX-License-Identifier: GPL-2.0
//! Native boot CPU declarations, separate from the kernel CPUID API.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, missing_docs, unreachable_pub)]
include!(concat!(
    env!("LUPOS_BOOT_OBJ"),
    "/boot_cpu_bindings_generated.rs"
));
