// SPDX-License-Identifier: GPL-2.0
//! Native compressed-boot descriptor types, bitfields and assembly declarations.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, missing_docs, unreachable_pub)]
include!(concat!(
    env!("LUPOS_BOOT_OBJ"),
    "/boot_idt_bindings_generated.rs"
));
