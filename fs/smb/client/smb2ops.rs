// SPDX-License-Identifier: GPL-2.0
//
// Faithful source payload for the isolated translation unit.  The referenced
// C unit is retained as source text because its kernel-provided declarations
// and ABI types are external to this isolated translation pass.
#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const SMB2OPS_C_SOURCE: &str = include_str!("smb2ops.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
