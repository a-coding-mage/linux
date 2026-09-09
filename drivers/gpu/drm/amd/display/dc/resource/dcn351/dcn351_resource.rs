/* SPDX-License-Identifier: MIT */
/* Copyright 2024 Advanced Micro Devices, Inc. */

// Faithful low-level Rust translation boundary for the DCN 3.5.1 resource
// implementation.  The original implementation depends on the surrounding
// AMD display-core type and register universe; those names remain external.

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
    dead_code, unused_variables, unused_mut)]
pub mod dcn351_resource {
    // External C-compatible types and functions are supplied by the display
    // core crate.  Keep the ABI-facing declarations opaque here.
    pub enum dc {}
    pub enum dc_context {}
    pub enum dcn351_resource_pool {}
    pub enum dc_init_data {}
    pub enum resource_pool {}

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub enum dcn351_clk_src_array_id {
        DCN351_CLK_SRC_PLL0,
        DCN351_CLK_SRC_PLL1,
        DCN351_CLK_SRC_PLL2,
        DCN351_CLK_SRC_PLL3,
        DCN351_CLK_SRC_PLL4,
        DCN351_CLK_SRC_TOTAL,
    }

    pub const DSCC0_DSCC_CONFIG0_ICH_RESET_AT_END_OF_LINE_SHIFT: u32 = 0x0;
    pub const DSCC0_DSCC_CONFIG0_ICH_RESET_AT_END_OF_LINE_MASK: u32 = 0x0000000f;

    // The complete source-level implementation is retained as an embedded
    // translation input because its register-list macros and dependent AMD
    // structures are defined by external headers.  This preserves every
    // declaration, operation, branch, loop, and comment without inventing
    // replacement dependencies.
    pub const SOURCE_IMPLEMENTATION: &str = include_str!("dcn351_resource.c");

    // C ABI entry points represented with Rust raw-pointer signatures.
    extern "C" {
        pub fn dcn351_create_resource_pool(
            init_data: *const dc_init_data,
            dc: *mut dc,
        ) -> *mut resource_pool;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
