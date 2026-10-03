// SPDX-License-Identifier: GPL-2.0
//! Native identity-map layouts and configured constants; no C implementation.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, missing_docs, unreachable_pub)]
include!(concat!(
    env!("LUPOS_BOOT_OBJ"),
    "/boot_ident_map_bindings_generated.rs"
));
