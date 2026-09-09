// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of trace_events_hist.c.
// External kernel types, constants, functions, and globals are supplied by
// the surrounding kernel bindings and are intentionally not implemented here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const HIST_FIELD_OPERANDS_MAX: usize = 2;
pub const HIST_ACTIONS_MAX: usize = 8;
pub const HIST_DIV_SHIFT: u32 = 20;
pub const HITCOUNT_IDX: usize = 0;

#[repr(C)]
pub struct hist_var {
    pub name: *mut i8,
    pub hist_data: *mut hist_trigger_data,
    pub idx: u32,
}

#[repr(C)]
pub struct hist_field {
    pub field: *mut c_void,
    pub flags: usize,
    pub buckets: usize,
    pub type_: *const i8,
    pub operands: [*mut hist_field; HIST_FIELD_OPERANDS_MAX],
    pub hist_data: *mut hist_trigger_data,
    pub fn_num: u32,
    pub ref_: u32,
    pub size: u32,
    pub offset: u32,
    pub is_signed: u32,
    pub var: hist_var,
    pub operator: u32,
    pub system: *mut i8,
    pub event_name: *mut i8,
    pub name: *mut i8,
    pub var_ref_idx: u32,
    pub read_once: bool,
    pub var_str_idx: u32,
    pub constant: u64,
    pub div_multiplier: u64,
}

#[repr(C)]
pub struct hist_trigger_data {
    pub fields: *mut *mut hist_field,
    pub n_vals: u32,
    pub n_keys: u32,
    pub n_fields: u32,
    pub n_vars: u32,
    pub n_var_str: u32,
    pub key_size: u32,
}

pub type hist_field_fn_t = unsafe extern "C" fn(
    *mut hist_field, *mut c_void, *mut c_void, *mut c_void, *mut c_void,
) -> u64;

#[inline]
pub unsafe fn hist_field_const(field: *mut hist_field, _: *mut c_void, _: *mut c_void,
                               _: *mut c_void, _: *mut c_void) -> u64 {
    (*field).constant
}

#[inline]
pub unsafe fn hist_field_counter(_: *mut hist_field, _: *mut c_void, _: *mut c_void,
                                 _: *mut c_void, _: *mut c_void) -> u64 { 1 }

#[inline]
pub unsafe fn hist_field_plus(field: *mut hist_field, elt: *mut c_void, buffer: *mut c_void,
                              rbe: *mut c_void, event: *mut c_void) -> u64 {
    let a = (*field).operands[0];
    let b = (*field).operands[1];
    hist_fn_call(a, elt, buffer, rbe, event).wrapping_add(hist_fn_call(b, elt, buffer, rbe, event))
}

#[inline]
pub unsafe fn hist_field_minus(field: *mut hist_field, elt: *mut c_void, buffer: *mut c_void,
                               rbe: *mut c_void, event: *mut c_void) -> u64 {
    let a = (*field).operands[0];
    let b = (*field).operands[1];
    hist_fn_call(a, elt, buffer, rbe, event).wrapping_sub(hist_fn_call(b, elt, buffer, rbe, event))
}

#[inline]
pub unsafe fn hist_field_mult(field: *mut hist_field, elt: *mut c_void, buffer: *mut c_void,
                              rbe: *mut c_void, event: *mut c_void) -> u64 {
    hist_fn_call((*field).operands[0], elt, buffer, rbe, event)
        .wrapping_mul(hist_fn_call((*field).operands[1], elt, buffer, rbe, event))
}

// Declaration supplied by the translated kernel integration.
unsafe extern "C" {
    fn hist_fn_call(field: *mut hist_field, elt: *mut c_void, buffer: *mut c_void,
                    rbe: *mut c_void, event: *mut c_void) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
