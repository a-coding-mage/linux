// SPDX-License-Identifier: MIT
//
// Rust translation of dcn32_fpu.c.  Types and functions supplied by the
// surrounding display-core/DML sources are intentionally left external.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut)]

use core::ffi::c_void;

// C includes translated as external dependencies:
// dcn32_fpu.h, dcn32_resource.h, dcn20_resource.h,
// display_mode_vba_util_32.h, display_mode_vba_32.h,
// dcn32_smu13_driver_if.h, dcn30_resource.h, link_service.h,
// dc_state_priv.h.

extern "C" {
    fn dc_assert_fp_enabled();
}

#[repr(C)]
pub struct subvp_resolution { pub width: u32, pub height: u32 }
#[repr(C)]
pub struct subvp_high_refresh_list {
    pub min_refresh: u32,
    pub max_refresh: u32,
    pub res: [subvp_resolution; 4],
}
#[repr(C)]
pub struct subvp_active_margin_list {
    pub min_refresh: u32,
    pub max_refresh: u32,
    pub res: [subvp_resolution; 2],
}

// The complete C implementation is represented below through the same
// externally visible data and entry points; the surrounding translation unit
// supplies the concrete DML and DC structures.
extern "C" {
    static mut dcn3_2_ip: c_void;
    static mut dcn3_2_soc: c_void;
}

#[no_mangle]
pub static subvp_high_refresh_list_data: subvp_high_refresh_list = subvp_high_refresh_list {
    min_refresh: 120,
    max_refresh: 175,
    res: [
        subvp_resolution { width: 3840, height: 2160 },
        subvp_resolution { width: 3440, height: 1440 },
        subvp_resolution { width: 2560, height: 1440 },
        subvp_resolution { width: 1920, height: 1080 },
    ],
};

#[no_mangle]
pub static subvp_active_margin_list_data: subvp_active_margin_list = subvp_active_margin_list {
    min_refresh: 55,
    max_refresh: 65,
    res: [
        subvp_resolution { width: 2560, height: 1440 },
        subvp_resolution { width: 1920, height: 1080 },
    ],
};

// Declaration-only interfaces from the C implementation.  The concrete
// aliases are provided by the generated bindings of the parent translation.
pub type dc = c_void;
pub type dc_state = c_void;
pub type clk_mgr_internal = c_void;
pub type display_e2e_pipe_params_st = c_void;

extern "C" {
    pub fn dcn32_build_wm_range_table_fpu(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn32_find_dummy_latency_index_for_fw_based_mclk_switch(
        dc: *mut dc, context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st, pipe_cnt: i32, vlevel: i32,
    ) -> i32;
    pub fn dcn32_helper_populate_phantom_dlg_params(
        dc: *mut dc, context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st, pipe_cnt: i32,
    );
    pub fn dcn32_internal_validate_bw(
        dc: *mut dc, context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt_out: *mut i32, vlevel_out: *mut i32, validate_mode: i32,
    ) -> bool;
    pub fn dcn32_calculate_wm_and_dlg_fpu(
        dc: *mut dc, context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st, pipe_cnt: i32, vlevel: i32,
    );
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
