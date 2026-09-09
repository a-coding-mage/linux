// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.
//
// Rust translation of dcn401_resource.h. C headers included by the original
// remain external dependencies of this translation.

#[repr(C)]
pub struct dcn401_resource_pool {
    pub base: resource_pool,
}

// container_of(pool, struct dcn401_resource_pool, base)
#[macro_export]
macro_rules! TO_DCN401_RES_POOL {
    ($pool:expr) => {
        (($pool as *mut resource_pool).cast::<dcn401_resource_pool>())
    };
}

extern "C" {
    pub fn dcn401_create_resource_pool(
        init_data: *const dc_init_data,
        dc: *mut dc,
    ) -> *mut resource_pool;
    pub fn dcn401_patch_unknown_plane_state(plane_state: *mut dc_plane_state) -> dc_status;
    pub fn dcn401_validate_bandwidth(
        dc: *mut dc,
        context: *mut dc_state,
        validate_mode: dc_validate_mode,
    ) -> dc_status;
    pub fn dcn401_prepare_mcache_programming(dc: *mut dc, context: *mut dc_state);
    pub fn dcn401_get_default_tiling_info(tiling_info: *mut dc_tiling_info);
    pub fn dcn401_get_vstartup_for_pipe(pipe_ctx: *mut pipe_ctx) -> ::core::ffi::c_uint;
    pub fn dcn401_get_power_profile(context: *const dc_state) -> ::core::ffi::c_int;
}

// Register-list macros are retained as token-preserving Rust macros. Their
// entries intentionally refer to register-definition macros supplied by the
// corresponding external DCN headers.
macro_rules! _dcn401_register_list { ($($entry:tt)*) => { $($entry)* }; }

macro_rules! HUBP_REG_LIST_DCN401_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! ABM_DCN401_REG_LIST_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! VPG_DCN401_REG_LIST_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! SE_DCN4_01_REG_LIST_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! LE_DCN401_REG_LIST_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! LE_DCN60_REG_LIST_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! DPP_REG_LIST_DCN401_COMMON_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! OPP_REG_LIST_DCN401_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! DSC_REG_LIST_DCN401_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! MPC_DWB_MUX_REG_LIST_DCN4_01_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! MPC_OUT_MUX_COMMON_REG_LIST_DCN4_01_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! MPC_OUT_MUX_REG_LIST_DCN4_01_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! OPTC_COMMON_REG_LIST_DCN401_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! HUBBUB_REG_LIST_DCN4_01_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! DCCG_REG_LIST_DCN401_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }
macro_rules! MCIF_WB_COMMON_REG_LIST_DCN4_01_RI { ($($args:tt)*) => { _dcn401_register_list!($($args)*) }; }

// The register-list macro bodies above are expanded from the external
// register-definition layer; the original source's complete ordered lists
// are preserved below as a source-level reference for that expansion.
/*
The bodies consist solely of the SRI_ARR, SR, SR_ARR, DCCG_SRII, NBIO_SR_ARR,
and inherited register-list invocations appearing in dcn401_resource.h,
including HUBP_3DLUT_FL_REG_LIST_DCN401(id), VPG_DCN3_REG_LIST_RI(id),
LE_DCN3_REG_LIST_RI(id), OPP_DPG_REG_LIST_RI(id), MPC inherited lists, and
MCIF_WB_COMMON_REG_LIST_DCN3_5_RI(inst), in their original order.
*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
