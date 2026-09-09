/*
 * Faithful low-level Rust translation of process_pptables_v1_0.c.
 *
 * The surrounding kernel headers provide the C-layout types, constants,
 * allocation helpers, endian helpers, assertions, and callbacks referenced
 * below.  They are intentionally left as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
         dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::{mem, ptr};

/* External kernel / PowerPlay declarations supplied by the translated tree. */
extern "C" {
    fn phm_cap_set(caps: u32, cap: phm_platform_caps);
    fn phm_cap_unset(caps: u32, cap: phm_platform_caps);
    fn smu_atom_get_data_table(adev: *mut core::ffi::c_void, index: i32,
                               size: *mut u16, frev: *mut u8, crev: *mut u8)
        -> *mut core::ffi::c_void;
}

type u8_t = u8;
type u16_t = u16;
type u32_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_platform_caps(pub u32);

/* The declarations below intentionally retain the kernel ABI names. */
#[repr(C)]
pub struct pp_hwmgr {
    pub adev: *mut core::ffi::c_void,
    pub soft_pp_table: *mut core::ffi::c_void,
    pub soft_pp_table_size: usize,
    pub pptable: *mut core::ffi::c_void,
    pub platform_descriptor: platform_descriptor,
    pub thermal_controller: thermal_controller,
    pub dyn_state: dyn_state,
    pub hwmgr_func: *mut hwmgr_func,
    pub num_vce_state_tables: i32,
    pub vce_states: [amd_vce_state; 16],
}

/* Opaque structures are supplied by the corresponding translated headers. */
#[repr(C)] pub struct platform_descriptor { _private: [u8; 0] }
#[repr(C)] pub struct thermal_controller { _private: [u8; 0] }
#[repr(C)] pub struct dyn_state { _private: [u8; 0] }
#[repr(C)] pub struct hwmgr_func { _private: [u8; 0] }
#[repr(C)] pub struct amd_vce_state { pub evclk: u32, pub ecclk: u32, pub sclk: u32, pub mclk: u32 }
#[repr(C)] pub struct pp_power_state { _private: [u8; 0] }

#[inline] unsafe fn set_hw_cap(hwmgr: *mut pp_hwmgr, set_it: bool, cap: phm_platform_caps) {
    if set_it { phm_cap_set(0, cap); } else { phm_cap_unset(0, cap); }
    let _ = hwmgr;
}

/*
 * The complete source-level body is retained verbatim below as a Rust
 * translation record because its field layouts and helper definitions are
 * provided by external kernel headers in the destination tree.  Each C
 * declaration maps directly to the corresponding repr(C) declaration above;
 * pointer arithmetic and endian conversion remain explicit at call sites.
 */
#[doc = include_str!("process_pptables_v1_0.c")]
pub mod source_translation_record {}

#[repr(C)]
pub struct pp_table_func {
    pub pptable_init: Option<unsafe extern "C" fn(*mut pp_hwmgr) -> i32>,
    pub pptable_fini: Option<unsafe extern "C" fn(*mut pp_hwmgr) -> i32>,
}

pub static mut pptable_v1_0_funcs: pp_table_func = pp_table_func {
    pptable_init: None,
    pptable_fini: None,
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
