/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Filesystem parameter description and parser
 *
 * Copyright (C) 2018 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::{c_char, c_void};

/* Supplied by the corresponding filesystem headers. */
pub struct path;
pub struct p_log;
pub struct fs_context;
pub struct fs_parameter;
pub struct kuid_t;
pub struct kgid_t;

#[repr(C)]
pub struct constant_table {
    pub name: *const c_char,
    pub value: i32,
}

pub struct fs_parameter_spec;
pub struct fs_parse_result;

pub type fs_param_type = unsafe extern "C" fn(
    log: *mut p_log,
    desc: *const fs_parameter_spec,
    param: *mut fs_parameter,
    result: *mut fs_parse_result,
) -> i32;

/* The type of parameter expected. */
unsafe extern "C" {
    pub fn fs_param_is_bool(
        log: *mut p_log,
        desc: *const fs_parameter_spec,
        param: *mut fs_parameter,
        result: *mut fs_parse_result,
    ) -> i32;
    pub fn fs_param_is_u32(
        log: *mut p_log,
        desc: *const fs_parameter_spec,
        param: *mut fs_parameter,
        result: *mut fs_parse_result,
    ) -> i32;
    pub fn fs_param_is_s32(
        log: *mut p_log,
        desc: *const fs_parameter_spec,
        param: *mut fs_parameter,
        result: *mut fs_parse_result,
    ) -> i32;
    pub fn fs_param_is_u64(
        log: *mut p_log,
        desc: *const fs_parameter_spec,
        param: *mut fs_parameter,
        result: *mut fs_parse_result,
    ) -> i32;
    pub fn fs_param_is_enum(
        log: *mut p_log,
        desc: *const fs_parameter_spec,
        param: *mut fs_parameter,
        result: *mut fs_parse_result,
    ) -> i32;
    pub fn fs_param_is_string(
        log: *mut p_log,
        desc: *const fs_parameter_spec,
        param: *mut fs_parameter,
        result: *mut fs_parse_result,
    ) -> i32;
    pub fn fs_param_is_blockdev(
        log: *mut p_log,
        desc: *const fs_parameter_spec,
        param: *mut fs_parameter,
        result: *mut fs_parse_result,
    ) -> i32;
    pub fn fs_param_is_fd(
        log: *mut p_log,
        desc: *const fs_parameter_spec,
        param: *mut fs_parameter,
        result: *mut fs_parse_result,
    ) -> i32;
    pub fn fs_param_is_uid(
        log: *mut p_log,
        desc: *const fs_parameter_spec,
        param: *mut fs_parameter,
        result: *mut fs_parse_result,
    ) -> i32;
    pub fn fs_param_is_gid(
        log: *mut p_log,
        desc: *const fs_parameter_spec,
        param: *mut fs_parameter,
        result: *mut fs_parse_result,
    ) -> i32;
    pub fn fs_param_is_file_or_string(
        log: *mut p_log,
        desc: *const fs_parameter_spec,
        param: *mut fs_parameter,
        result: *mut fs_parse_result,
    ) -> i32;
}

/* Specification of the type of value a parameter wants. */
#[repr(C)]
pub struct fs_parameter_spec {
    pub name: *const c_char,
    pub type_: Option<fs_param_type>,
    pub opt: u8,
    pub flags: u16,
    pub data: *const c_void,
}

pub const fs_param_neg_with_no: u16 = 0x0002;
pub const fs_param_can_be_empty: u16 = 0x0004;
pub const fs_param_deprecated: u16 = 0x0008;

/* Result of parse. */
#[repr(C)]
pub union fs_parse_result_value {
    pub boolean: bool,
    pub int_32: i32,
    pub uint_32: u32,
    pub uint_64: u64,
    pub uid: kuid_t,
    pub gid: kgid_t,
}

#[repr(C)]
pub struct fs_parse_result {
    pub negated: bool,
    pub value: fs_parse_result_value,
}

unsafe extern "C" {
    pub fn __fs_parse(
        log: *mut p_log,
        desc: *const fs_parameter_spec,
        value: *mut fs_parameter,
        result: *mut fs_parse_result,
    ) -> i32;

    pub fn fs_lookup_param(
        fc: *mut fs_context,
        param: *mut fs_parameter,
        want_bdev: bool,
        flags: u32,
        path: *mut path,
    ) -> i32;

    pub fn lookup_constant(
        tbl: *const constant_table,
        name: *const c_char,
        not_found: i32,
    ) -> i32;
}

pub unsafe fn fs_parse(
    fc: *mut fs_context,
    desc: *const fs_parameter_spec,
    param: *mut fs_parameter,
    result: *mut fs_parse_result,
) -> i32 {
    __fs_parse(&mut (*fc).log, desc, param, result)
}

/* CONFIG_VALIDATE_FS_PARSER selects the external validator. */
#[cfg(feature = "CONFIG_VALIDATE_FS_PARSER")]
unsafe extern "C" {
    pub fn fs_validate_description(
        name: *const c_char,
        desc: *const fs_parameter_spec,
    ) -> bool;
}

#[cfg(not(feature = "CONFIG_VALIDATE_FS_PARSER"))]
pub unsafe fn fs_validate_description(
    _name: *const c_char,
    _desc: *const fs_parameter_spec,
) -> bool {
    true
}

/* Parameter type, name, index and flags element constructors. */
#[macro_export]
macro_rules! __fsparam {
    ($type_:expr, $name:expr, $opt:expr, $flags:expr, $data:expr) => {
        $crate::fs_parameter_spec {
            name: $name,
            opt: $opt,
            type_: $type_,
            flags: $flags,
            data: $data,
        }
    };
}

#[macro_export]
macro_rules! fsparam_flag { ($name:expr, $opt:expr) => { $crate::__fsparam!(None, $name, $opt, 0, core::ptr::null()) }; }
#[macro_export]
macro_rules! fsparam_flag_no { ($name:expr, $opt:expr) => { $crate::__fsparam!(None, $name, $opt, $crate::fs_param_neg_with_no, core::ptr::null()) }; }
#[macro_export]
macro_rules! fsparam_bool { ($name:expr, $opt:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_bool), $name, $opt, 0, core::ptr::null()) }; }
#[macro_export]
macro_rules! fsparam_u32 { ($name:expr, $opt:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_u32), $name, $opt, 0, core::ptr::null()) }; }
#[macro_export]
macro_rules! fsparam_u32oct { ($name:expr, $opt:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_u32), $name, $opt, 0, 8usize as *const c_void) }; }
#[macro_export]
macro_rules! fsparam_u32hex { ($name:expr, $opt:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_u32), $name, $opt, 0, 16usize as *const c_void) }; }
#[macro_export]
macro_rules! fsparam_s32 { ($name:expr, $opt:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_s32), $name, $opt, 0, core::ptr::null()) }; }
#[macro_export]
macro_rules! fsparam_u64 { ($name:expr, $opt:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_u64), $name, $opt, 0, core::ptr::null()) }; }
#[macro_export]
macro_rules! fsparam_enum { ($name:expr, $opt:expr, $array:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_enum), $name, $opt, 0, $array) }; }
#[macro_export]
macro_rules! fsparam_string { ($name:expr, $opt:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_string), $name, $opt, 0, core::ptr::null()) }; }
#[macro_export]
macro_rules! fsparam_bdev { ($name:expr, $opt:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_blockdev), $name, $opt, 0, core::ptr::null()) }; }
#[macro_export]
macro_rules! fsparam_fd { ($name:expr, $opt:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_fd), $name, $opt, 0, core::ptr::null()) }; }
#[macro_export]
macro_rules! fsparam_file_or_string { ($name:expr, $opt:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_file_or_string), $name, $opt, 0, core::ptr::null()) }; }
#[macro_export]
macro_rules! fsparam_uid { ($name:expr, $opt:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_uid), $name, $opt, 0, core::ptr::null()) }; }
#[macro_export]
macro_rules! fsparam_gid { ($name:expr, $opt:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_gid), $name, $opt, 0, core::ptr::null()) }; }
#[macro_export]
macro_rules! fsparam_string_empty { ($name:expr, $opt:expr) => { $crate::__fsparam!(Some($crate::fs_param_is_string), $name, $opt, $crate::fs_param_can_be_empty, core::ptr::null()) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
