// SPDX-License-Identifier: MIT
//
// Direct Rust/FFI translation of dcn303_resource.c.  The register-list and
// object definitions are supplied by the surrounding DCN dependency headers.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// C headers intentionally remain external dependencies of this translation.
extern "C" {
    static mut res_cap_dcn303: resource_caps;
    static mut plane_cap: dc_plane_cap;
    static mut config_defaults: dc_check_config;
    static mut panel_config_defaults: dc_panel_config;
    static mut debug_defaults_drv: dc_debug_options;
}

#[repr(C)] pub struct dc_debug_options { _private: [u8; 0] }
#[repr(C)] pub struct dc_check_config { _private: [u8; 0] }
#[repr(C)] pub struct dc_panel_config { _private: [u8; 0] }
#[repr(C)] pub struct resource_caps { pub num_timing_generator: u32, pub num_opp: u32, pub num_video_plane: u32, pub num_audio: u32, pub num_stream_encoder: u32, pub num_hpo_frl: u32, pub num_dwb: u32, pub num_ddc: u32, pub num_vmid: u32, pub num_mpc_3dlut: u32, pub num_dsc: u32 }
#[repr(C)] pub struct dc_plane_cap { _private: [u8; 0] }
#[repr(C)] pub struct dc { _private: [u8; 0] }
#[repr(C)] pub struct dc_context { _private: [u8; 0] }
#[repr(C)] pub struct resource_pool { _private: [u8; 0] }
#[repr(C)] pub struct dc_init_data { pub num_virtual_links: u32 }
#[repr(C)] pub struct clk_bw_params { _private: [u8; 0] }
#[repr(C)] pub struct dc_panel_config_opaque { _private: [u8; 0] }

#[repr(C)] pub enum dcn303_clk_src_array_id { DCN303_CLK_SRC_PLL0, DCN303_CLK_SRC_PLL1, DCN303_CLK_SRC_TOTAL }

extern "C" {
    fn dcn303_resource_construct(num_virtual_links: u8, dc: *mut dc, pool: *mut resource_pool) -> bool;
    fn dcn303_resource_destruct(pool: *mut resource_pool);
    fn resource_construct(num_virtual_links: u8, dc: *mut dc, pool: *mut resource_pool, funcs: *const c_void) -> bool;
    fn dcn303_fpu_update_bw_bounding_box(dc: *mut dc, params: *mut clk_bw_params);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn BREAK_TO_DEBUGGER();
}

#[no_mangle]
pub unsafe extern "C" fn dcn303_update_bw_bounding_box(dc: *mut dc, bw_params: *mut clk_bw_params) {
    dcn303_fpu_update_bw_bounding_box(dc, bw_params);
}

#[no_mangle]
pub unsafe extern "C" fn dcn303_get_panel_config_defaults(panel_config: *mut dc_panel_config) {
    *panel_config = core::ptr::read(&panel_config_defaults);
}

#[no_mangle]
pub unsafe extern "C" fn dcn303_destroy_resource_pool(pool: *mut *mut resource_pool) {
    dcn303_resource_destruct(*pool);
    kfree(*pool as *mut c_void);
    *pool = core::ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn dcn303_create_resource_pool(init_data: *const dc_init_data, dc: *mut dc) -> *mut resource_pool {
    let pool = kzalloc(core::mem::size_of::<resource_pool>(), 0) as *mut resource_pool;
    if pool.is_null() { return core::ptr::null_mut(); }
    if dcn303_resource_construct((*init_data).num_virtual_links as u8, dc, pool) { pool } else {
        BREAK_TO_DEBUGGER();
        kfree(pool as *mut c_void);
        core::ptr::null_mut()
    }
}

// The following declarations preserve the source file's externally visible
// implementation entry points; register structures and constructors are
// defined by the corresponding DCN30/DCN20 dependencies.
extern "C" {
    fn dcn303_resource_construct(num_virtual_links: u8, dc: *mut dc, pool: *mut resource_pool) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
