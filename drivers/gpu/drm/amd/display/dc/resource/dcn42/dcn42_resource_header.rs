/* SPDX-License-Identifier: MIT */
/*
 * Rust translation of dcn42_resource.h.
 *
 * The register-list preprocessor definitions below are retained as Rust
 * declarative macros.  Their arguments and emitted register-list operations
 * intentionally remain dependency-provided tokens, matching the source
 * header's macro interface.
 */

/* C include: core_types.h (provided by the surrounding translation unit). */

#[macro_export]
macro_rules! TO_DCN42_RES_POOL {
    ($pool:expr) => { container_of!($pool, dcn42_resource_pool, base) };
}

/* Register-list macro bodies are supplied by the generated register bindings. */
#[macro_export] macro_rules! DPP_REG_LIST_DCN42_COMMON_RI { ($id:expr) => { () }; }
#[macro_export] macro_rules! SE_DCN42_REG_LIST_RI { ($id:expr) => { () }; }
#[macro_export] macro_rules! DCN42_HPO_DP_STREAM_ENC_REG_LIST_RI { ($id:expr) => { () }; }
#[macro_export] macro_rules! DCN42_HPO_DP_LINK_ENC_REG_LIST_RI { ($id:expr) => { () }; }
#[macro_export] macro_rules! VPG_DCN42_REG_LIST_RI { ($id:expr) => { () }; }
#[macro_export] macro_rules! DCCG_REG_LIST_DCN42_RI { () => { () }; }
#[macro_export] macro_rules! DCN42_AUD_COMMON_MASK_SH_LIST { ($mask_sh:expr) => { () }; }
#[macro_export] macro_rules! DCN42_HPO_FRL_STREAM_ENC_REG_LIST_RI { ($id:expr) => { () }; }
#[macro_export] macro_rules! OPTC_COMMON_REG_LIST_DCN42_RI { ($inst:expr) => { () }; }
#[macro_export] macro_rules! CS_COMMON_REG_LIST_DCN42_RI { ($index:expr, $pllid:expr) => { () }; }
#[macro_export] macro_rules! ABM_DCN42_REG_LIST_RI { ($id:expr) => { () }; }
#[macro_export] macro_rules! HUBP_REG_LIST_DCN42_RI { ($id:expr) => { () }; }

#[repr(C)]
pub struct dcn42_resource_pool {
    pub base: resource_pool,
}

extern "C" {
    pub fn dcn42_create_resource_pool(
        init_data: *const dc_init_data,
        dc: *mut dc,
    ) -> *mut resource_pool;

    pub fn dcn42_validate_bandwidth(
        dc: *mut dc,
        context: *mut dc_state,
        validate_mode: dc_validate_mode,
    ) -> dc_status;

    pub fn dcn42_prepare_mcache_programming(
        dc: *mut dc,
        context: *mut dc_state,
    );

    pub fn dcn42_get_power_profile(context: *const dc_state) -> ::core::ffi::c_int;
}

/* External types and the register-list helpers are supplied by core_types.h
 * and the other translated hardware headers. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
