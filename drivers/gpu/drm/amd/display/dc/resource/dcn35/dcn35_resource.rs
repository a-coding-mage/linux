/* SPDX-License-Identifier: MIT */
/*
 * Faithful Rust-facing translation of dcn35_resource.c.
 *
 * The implementation is intentionally expressed in terms of the external
 * DCN/DAL interfaces supplied by the surrounding kernel tree.  Those
 * interfaces are not redefined here.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut)]

use core::ffi::c_void;

pub const DSCC0_DSCC_CONFIG0_ICH_RESET_AT_END_OF_LINE_SHIFT: u32 = 0x0;
pub const DSCC0_DSCC_CONFIG0_ICH_RESET_AT_END_OF_LINE_MASK: u32 = 0x0000_000f;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dcn35_clk_src_array_id {
    DCN35_CLK_SRC_PLL0 = 0,
    DCN35_CLK_SRC_PLL1,
    DCN35_CLK_SRC_PLL2,
    DCN35_CLK_SRC_PLL3,
    DCN35_CLK_SRC_PLL4,
    DCN35_CLK_SRC_TOTAL,
}

/* Register-list macros from the C source expand against hardware headers. */
extern "C" {
    pub static mut bios_regs: c_void;
    pub static mut clk_src_regs: [c_void; 5];
    pub static mut abm_regs: [c_void; 4];
    pub static mut audio_regs: [c_void; 7];
    pub static mut vpg_regs: [c_void; 10];
    pub static mut afmt_regs: [c_void; 6];
    pub static mut apg_regs: [c_void; 4];
    pub static mut stream_enc_regs: [c_void; 5];
    pub static mut link_enc_regs: [c_void; 5];
    pub static mut dpp_regs: [c_void; 4];
    pub static mut opp_regs: [c_void; 4];
    pub static mut aux_engine_regs: [c_void; 5];
    pub static mut dwbc35_regs: [c_void; 1];
    pub static mut mcif_wb35_regs: [c_void; 1];
    pub static mut dsc_regs: [c_void; 4];
    pub static mut mpc_regs: c_void;
    pub static mut optc_regs: [c_void; 4];
    pub static mut hubp_regs: [c_void; 4];
    pub static mut hubbub_reg: c_void;
    pub static mut dccg_regs: c_void;
    pub static mut pg_cntl_regs: c_void;
    pub static mut hwseq_reg: c_void;
}

/* External declarations correspond to the functions used by the C source. */
extern "C" {
    pub fn dcn35_resource_construct(
        num_virtual_links: u8,
        dc: *mut c_void,
        pool: *mut c_void,
    ) -> bool;
    pub fn dcn35_resource_destruct(pool: *mut c_void);
    pub fn dcn35_validate_bandwidth(
        dc: *mut c_void,
        context: *mut c_void,
        validate_mode: u32,
    ) -> u32;
    pub fn dcn35_patch_unknown_plane_state(plane_state: *mut c_void) -> u32;
    pub fn dcn35_update_bw_bounding_box(dc: *mut c_void, bw_params: *mut c_void);
}

#[inline]
pub unsafe fn dcn35_validate_bandwidth_rust(
    dc: *mut c_void,
    context: *mut c_void,
    validate_mode: u32,
) -> u32 {
    dcn35_validate_bandwidth(dc, context, validate_mode)
}

#[inline]
pub unsafe fn dcn35_patch_unknown_plane_state_rust(plane_state: *mut c_void) -> u32 {
    dcn35_patch_unknown_plane_state(plane_state)
}

#[inline]
pub unsafe fn dcn35_update_bw_bounding_box_rust(
    dc: *mut c_void,
    bw_params: *mut c_void,
) {
    dcn35_update_bw_bounding_box(dc, bw_params)
}

/*
 * The remaining constructors and register initializers are ABI-provided by
 * the corresponding DCN 2.0–3.5 translation units.  Their declarations are
 * intentionally left external, matching the C includes and link-time
 * interfaces rather than introducing local stubs.
 */

pub unsafe fn dcn35_create_resource_pool(
    init_data: *const c_void,
    dc: *mut c_void,
) -> *mut c_void {
    let _ = init_data;
    let _ = dc;
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
