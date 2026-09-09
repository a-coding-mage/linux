// SPDX-License-Identifier: GPL-2.0

// Dependency provided by trace_dynevent.h.

pub const SYNTH_SYSTEM: &str = "synthetic";
pub const SYNTH_FIELDS_MAX: usize = 64;

// STR_VAR_LEN_MAX = MAX_FILTER_STR_VAL; it must be a multiple of sizeof(u64).
pub const STR_VAR_LEN_MAX: usize = MAX_FILTER_STR_VAL;

#[repr(C)]
pub struct synth_field {
    pub type_: *mut core::ffi::c_char,
    pub name: *mut core::ffi::c_char,
    pub size: usize,
    pub offset: u32,
    pub field_pos: u32,
    pub is_signed: bool,
    pub is_string: bool,
    pub is_dynamic: bool,
    pub is_stack: bool,
}

#[repr(C)]
pub struct synth_event {
    pub devent: dyn_event,
    pub ref_: i32,
    pub name: *mut core::ffi::c_char,
    pub fields: *mut *mut synth_field,
    pub n_fields: u32,
    pub dynamic_fields: *mut *mut synth_field,
    pub n_dynamic_fields: u32,
    pub n_u64: u32,
    pub class: trace_event_class,
    pub call: trace_event_call,
    pub tp: *mut tracepoint,
    pub mod_: *mut module,
}

extern "C" {
    pub fn find_synth_event(name: *const core::ffi::c_char) -> *mut synth_event;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
