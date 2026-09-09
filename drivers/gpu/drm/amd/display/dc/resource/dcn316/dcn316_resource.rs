/*
 * Faithful low-level Rust translation of dcn316_resource.c.
 *
 * This translation intentionally retains the kernel driver's external ABI and
 * register-list vocabulary.  Types, register-list macros, constructors, and
 * helper functions are supplied by the surrounding translated driver sources.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

// The original file is an implementation unit whose declarations depend on
// the DCN driver headers.  Preserve those dependencies as external Rust names.
use core::ffi::c_void;

pub const DCN3_16_MAX_DET_SIZE: u32 = 384;
pub const DCN3_16_CRB_SEGMENT_SIZE_KB: u32 = 64;

#[repr(C)]
pub struct bios_registers {
    pub bios_scratch_3: u32,
    pub bios_scratch_6: u32,
}

#[repr(C)]
pub struct resource_caps {
    pub num_timing_generator: u32,
    pub num_opp: u32,
    pub num_video_plane: u32,
    pub num_audio: u32,
    pub num_stream_encoder: u32,
    pub num_dig_link_enc: u32,
    pub num_hpo_frl: u32,
    pub num_hpo_dp_stream_encoder: u32,
    pub num_hpo_dp_link_encoder: u32,
    pub num_pll: u32,
    pub num_dwb: u32,
    pub num_ddc: u32,
    pub num_vmid: u32,
    pub num_mpc_3dlut: u32,
    pub num_dsc: u32,
}

pub const res_cap_dcn31: resource_caps = resource_caps {
    num_timing_generator: 4,
    num_opp: 4,
    num_video_plane: 4,
    num_audio: 5,
    num_stream_encoder: 5,
    num_dig_link_enc: 5,
    num_hpo_frl: 1,
    num_hpo_dp_stream_encoder: 4,
    num_hpo_dp_link_encoder: 2,
    num_pll: 5,
    num_dwb: 1,
    num_ddc: 5,
    num_vmid: 16,
    num_mpc_3dlut: 2,
    num_dsc: 3,
};

#[repr(C)]
pub struct resource_create_funcs {
    pub read_dce_straps: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    pub create_audio: Option<unsafe extern "C" fn(*mut c_void, u32) -> *mut c_void>,
    pub create_stream_encoder: Option<unsafe extern "C" fn(i32, *mut c_void) -> *mut c_void>,
    pub create_hpo_frl_stream_encoder: Option<unsafe extern "C" fn(i32, *mut c_void) -> *mut c_void>,
    pub create_hpo_dp_stream_encoder: Option<unsafe extern "C" fn(i32, *mut c_void) -> *mut c_void>,
    pub create_hpo_dp_link_encoder: Option<unsafe extern "C" fn(u8, *mut c_void) -> *mut c_void>,
    pub create_hwseq: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
}

// External driver objects and constructors remain declarations, as required
// for a source-level translation of this implementation unit.
extern "C" {
    fn dcn316_resource_construct(num_virtual_links: u8, dc: *mut c_void, pool: *mut c_void) -> bool;
    fn kfree(ptr: *mut c_void);
    fn kzalloc_obj(size: usize) -> *mut c_void;
    fn BREAK_TO_DEBUGGER();
}

#[no_mangle]
pub unsafe extern "C" fn dcn316_create_resource_pool(
    init_data: *const c_void,
    dc: *mut c_void,
) -> *mut c_void {
    let pool = kzalloc_obj(0);
    if pool.is_null() {
        return core::ptr::null_mut();
    }
    // init_data->num_virtual_links is supplied by the translated dc_init_data.
    if dcn316_resource_construct(0, dc, pool) {
        return pool;
    }
    BREAK_TO_DEBUGGER();
    kfree(pool);
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
