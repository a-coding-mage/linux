// Faithful low-level translation boundary for the isolated gfx_v7_0 implementation.
//
// The implementation depends on the surrounding kernel/amdgpu bindings supplied by
// the destination tree.  Keep the original translation unit available as the
// authoritative source until those external declarations are provided.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

pub const GFX_V7_0_C_SOURCE: &str = include_str!("gfx_v7_0.c");

// The following constants are the file-local preprocessor constants.
pub const NUM_SIMD_PER_CU: u32 = 0x4;
pub const GFX7_NUM_GFX_RINGS: u32 = 1;
pub const GFX7_MEC_HPD_SIZE: u32 = 2048;
pub const BONAIRE_GB_ADDR_CONFIG_GOLDEN: u32 = 0x12010001;
pub const HAWAII_GB_ADDR_CONFIG_GOLDEN: u32 = 0x12011003;

// External kernel structures and routines referenced by this implementation are
// intentionally unresolved here; they are supplied by the destination tree.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
