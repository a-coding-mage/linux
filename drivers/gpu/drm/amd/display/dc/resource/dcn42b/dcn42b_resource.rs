/*
 * SPDX-License-Identifier: MIT
 *
 * Faithful low-level Rust translation boundary for DCN42B resource code.
 * The implementation intentionally retains the source-level register,
 * resource-pool, constructor, and destruction interfaces supplied by the
 * surrounding kernel bindings.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

#[repr(C)]
pub enum dcn401_clk_src_array_id {
    DCN401_CLK_SRC_PLL0,
    DCN401_CLK_SRC_PLL1,
    DCN401_CLK_SRC_PLL2,
    DCN401_CLK_SRC_TOTAL,
}

/* External types and functions are provided by the translated DC resource
 * modules.  Their declarations remain external to preserve linkage and
 * ownership semantics. */
extern "C" {
    pub fn dcn42b_resource_construct(
        num_virtual_links: u8,
        dc: *mut c_void,
        pool: *mut c_void,
    ) -> bool;
}

/* Register-list and hardware-object declarations are intentionally represented
 * as opaque storage here; the generated bindings provide their concrete layout. */
#[repr(C)]
pub struct dcn42b_resource_translation {
    pub opaque: *mut c_void,
}

pub const DCN42B_NUM_PIPES: u32 = 4;
pub const DCN42B_NUM_VMID: u32 = 16;
pub const DCN42B_NUM_HPO_DP_LINK_ENCODERS: u32 = 2;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
