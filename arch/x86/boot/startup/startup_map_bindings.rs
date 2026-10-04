// SPDX-License-Identifier: GPL-2.0
//! Native startup page-table/CPU layouts, declarations, and configured constants.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, missing_docs, unreachable_pub)]
include!(concat!(
    env!("LUPOS_STARTUP_OBJ"),
    "/startup_map_bindings_generated.rs"
));
