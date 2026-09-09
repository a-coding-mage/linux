/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Copyright (c) 2018 Mellanox Technologies. All rights reserved */

// C dependencies represented by this header: `size_t`, `bool`, and `UINT_MAX`.

#[repr(C)]
pub struct objagg_ops {
    pub obj_size: size_t,
    pub delta_check: Option<unsafe extern "C" fn(
        priv_: *mut ::core::ffi::c_void,
        parent_obj: *const ::core::ffi::c_void,
        obj: *const ::core::ffi::c_void,
    ) -> bool>,
    pub delta_create: Option<unsafe extern "C" fn(
        priv_: *mut ::core::ffi::c_void,
        parent_obj: *mut ::core::ffi::c_void,
        obj: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void>,
    pub delta_destroy: Option<unsafe extern "C" fn(
        priv_: *mut ::core::ffi::c_void,
        delta_priv: *mut ::core::ffi::c_void,
    )>,
    pub root_create: Option<unsafe extern "C" fn(
        priv_: *mut ::core::ffi::c_void,
        obj: *mut ::core::ffi::c_void,
        root_id: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_void>,
    pub root_destroy: Option<unsafe extern "C" fn(
        priv_: *mut ::core::ffi::c_void,
        root_priv: *mut ::core::ffi::c_void,
    )>,
}

pub const OBJAGG_OBJ_ROOT_ID_INVALID: ::core::ffi::c_uint = ::core::ffi::c_uint::MAX;

#[repr(C)]
pub struct objagg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objagg_obj {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objagg_hints {
    _private: [u8; 0],
}

extern "C" {
    pub fn objagg_obj_root_priv(objagg_obj: *const objagg_obj) -> *const ::core::ffi::c_void;
    pub fn objagg_obj_delta_priv(objagg_obj: *const objagg_obj) -> *const ::core::ffi::c_void;
    pub fn objagg_obj_raw(objagg_obj: *const objagg_obj) -> *const ::core::ffi::c_void;

    pub fn objagg_obj_get(
        objagg: *mut objagg,
        obj: *mut ::core::ffi::c_void,
    ) -> *mut objagg_obj;
    pub fn objagg_obj_put(objagg: *mut objagg, objagg_obj: *mut objagg_obj);
    pub fn objagg_create(
        ops: *const objagg_ops,
        hints: *mut objagg_hints,
        priv_: *mut ::core::ffi::c_void,
    ) -> *mut objagg;
    pub fn objagg_destroy(objagg: *mut objagg);
}

#[repr(C)]
pub struct objagg_obj_stats {
    pub user_count: ::core::ffi::c_uint,
    pub delta_user_count: ::core::ffi::c_uint, // includes delta object users
}

#[repr(C)]
pub struct objagg_obj_stats_info {
    pub stats: objagg_obj_stats,
    pub objagg_obj: *mut objagg_obj, // associated object
    pub is_root: bool,
}

#[repr(C)]
pub struct objagg_stats {
    pub root_count: ::core::ffi::c_uint,
    pub stats_info_count: ::core::ffi::c_uint,
    pub stats_info: [objagg_obj_stats_info; 0],
}

extern "C" {
    pub fn objagg_stats_get(objagg: *mut objagg) -> *const objagg_stats;
    pub fn objagg_stats_put(objagg_stats: *const objagg_stats);
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum objagg_opt_algo_type {
    OBJAGG_OPT_ALGO_SIMPLE_GREEDY = 0,
}

extern "C" {
    pub fn objagg_hints_get(
        objagg: *mut objagg,
        opt_algo_type: objagg_opt_algo_type,
    ) -> *mut objagg_hints;
    pub fn objagg_hints_put(objagg_hints: *mut objagg_hints);
    pub fn objagg_hints_stats_get(
        objagg_hints: *mut objagg_hints,
    ) -> *const objagg_stats;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
