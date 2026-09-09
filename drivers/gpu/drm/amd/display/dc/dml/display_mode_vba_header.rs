/*
 * Copyright 2017 Advanced Micro Devices, Inc.
 *
 * Rust translation of display_mode_vba.h.  The declarations below retain the
 * C ABI and the source header's external dependencies.
 */

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/* External types supplied by the DML implementation. */
#[repr(C)]
pub struct display_mode_lib { _private: [u8; 0] }
#[repr(C)]
pub struct display_e2e_pipe_params_st { _private: [u8; 0] }
#[repr(C)]
pub struct ip_params_st { _private: [u8; 0] }
#[repr(C)]
pub struct soc_bounding_box_st { _private: [u8; 0] }
#[repr(C)]
pub struct DmlPipe { _private: [u8; 0] }
#[repr(C)]
pub struct SOCParametersList { _private: [u8; 0] }
#[repr(C)]
pub struct Watermarks { _private: [u8; 0] }

extern "C" {
    pub fn ModeSupportAndSystemConfiguration(mode_lib: *mut display_mode_lib);

    pub fn get_clk_dcf_deepsleep(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn get_wm_urgent(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn get_wm_memory_trip(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn get_wm_writeback_urgent(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn get_wm_stutter_exit(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn get_wm_stutter_enter_exit(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn get_wm_z8_stutter_exit(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn get_wm_z8_stutter_enter_exit(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn get_stutter_efficiency_z8(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn get_stutter_num_bursts_z8(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn get_wm_dram_clock_change(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn get_wm_writeback_dram_clock_change(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;

    pub fn get_total_immediate_flip_bytes(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn get_total_immediate_flip_bw(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn get_total_prefetch_bw(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> f64;
    pub fn dml_get_voltage_level(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn get_total_surface_size_in_mall_bytes(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn get_is_phantom_pipe(mode_lib: *mut display_mode_lib, pipes: *const display_e2e_pipe_params_st, num_pipes: ::core::ffi::c_uint, pipe_idx: ::core::ffi::c_uint) -> bool;
    pub fn PixelClockAdjustmentForProgressiveToInterlaceUnit(mode_lib: *mut display_mode_lib);
}

/* The large VBA working-state aggregate is intentionally opaque here: its
 * complete field layout is defined by the generated DML dependency. */
#[repr(C)]
pub struct vba_vars_st { _private: [u8; 0] }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
