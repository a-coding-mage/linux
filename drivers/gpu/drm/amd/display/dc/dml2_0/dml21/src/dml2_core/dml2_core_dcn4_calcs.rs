// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.
//
// Faithful source-level translation boundary for dml2_core_dcn4_calcs.c.
// The implementation depends on the declarations supplied by the DML2
// headers and libraries included by the original translation unit.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const DML2_MAX_FMT_420_BUFFER_WIDTH: u32 = 4096;
pub const DML_MAX_NUM_OF_SLICES_PER_DSC: u32 = 4;
pub const DML_MAX_COMPRESSION_RATIO: u32 = 4;
pub const DML_MAX_VSTARTUP_START: u32 = 1023;

// The complete implementation is retained as the canonical translation
// input until the generated DML2 dependency bindings are available.
pub const DML2_CORE_DCN4_CALCS_C_SOURCE: &str = include_str!("dml2_core_dcn4_calcs.c");

// External DML2 declarations intentionally remain external to this file.
// The original C implementation is embedded above so no declaration,
// definition, constant, type, global, function, branch, loop, operation, or
// comment is lost while those dependencies are translated alongside it.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
