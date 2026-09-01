// SPDX-License-Identifier: GPL-2.0
/*
 * Arnaldo Carvalho de Melo <acme@redhat.com>
 *
 * Copyright (C) 2024, Red Hat, Inc
 */

use std::os::raw::{c_char, c_int};
use std::ptr;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_member {
    pub name_off: u32,
}

unsafe extern "C" {
    fn btf__type_by_id(btf: *mut btf, type_id: c_int) -> *const btf_type;
    fn btf_members(t: *const btf_type) -> *const btf_member;
    fn btf_vlen(t: *const btf_type) -> u32;
    fn btf__name_by_offset(btf: *mut btf, offset: u32) -> *const c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

pub unsafe extern "C" fn __btf_type__find_member_by_name(
    btf: *mut btf,
    type_id: c_int,
    member_name: *const c_char,
) -> *const btf_member {
    let t: *const btf_type = unsafe { btf__type_by_id(btf, type_id) };
    let mut m: *const btf_member;
    let mut i: u32;

    i = 0;
    m = unsafe { btf_members(t) };
    while i < unsafe { btf_vlen(t) } {
        let current_member_name: *const c_char =
            unsafe { btf__name_by_offset(btf, (*m).name_off) };

        if unsafe { strcmp(current_member_name, member_name) } == 0 {
            return m;
        }

        i = i.wrapping_add(1);
        m = unsafe { m.add(1) };
    }

    ptr::null()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
