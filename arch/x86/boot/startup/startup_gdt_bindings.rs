// SPDX-License-Identifier: GPL-2.0
//! Native descriptor layouts, selectors and assembly declarations only.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, missing_docs, unreachable_pub)]
include!(concat!(
    env!("LUPOS_STARTUP_OBJ"),
    "/startup_gdt_bindings_generated.rs"
));
