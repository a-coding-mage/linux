// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

#[repr(C)]
pub struct S_struct {
    pub a: ::core::ffi::c_int,
    pub b: ::core::ffi::c_int,
    pub c: ::core::ffi::c_int,
}

#[repr(C)]
pub union U_union {
    pub a: ::core::ffi::c_int,
    pub b: ::core::ffi::c_int,
    pub c: ::core::ffi::c_int,
}

#[repr(C)]
pub struct S1_struct {
    pub a: ::core::ffi::c_int,
    pub b: ::core::ffi::c_int,
    pub c: ::core::ffi::c_int,
}

#[repr(C)]
pub union U1_union {
    pub a: ::core::ffi::c_int,
    pub b: ::core::ffi::c_int,
    pub c: ::core::ffi::c_int,
}

pub type T = ::core::ffi::c_int;
pub type S = ::core::ffi::c_int;
pub type U = ::core::ffi::c_int;
pub type T1 = ::core::ffi::c_int;
pub type S1 = ::core::ffi::c_int;
pub type U1 = ::core::ffi::c_int;

#[repr(C)]
pub struct root_struct {
    pub m_1: S,
    pub m_2: T,
    pub m_3: U,
    pub m_4: S1,
    pub m_5: T1,
    pub m_6: U1,
    pub m_7: S_struct,
    pub m_8: S1_struct,
    pub m_9: U_union,
    pub m_10: U1_union,
}

#[no_mangle]
pub unsafe extern "C" fn func(root: *mut root_struct) -> ::core::ffi::c_int {
    let _ = root;
    0
}

#[no_mangle]
pub unsafe extern "C" fn kfunc_a(root: *mut root_struct) -> ::core::ffi::c_int {
    let _ = root;
    0
}

#[no_mangle]
pub unsafe extern "C" fn kfunc_b(root: *mut root_struct) -> ::core::ffi::c_int {
    let _ = root;
    0
}

#[no_mangle]
pub unsafe extern "C" fn kfunc_c(
    a: *mut root_struct,
    b: *mut root_struct,
) -> *mut root_struct {
    let _ = b;
    a
}

#[no_mangle]
pub unsafe extern "C" fn kfunc_d(
    a: *mut root_struct,
    b: *mut root_struct,
) -> ::core::ffi::c_int {
    let _ = a;
    let _ = b;
    0
}

#[no_mangle]
pub unsafe extern "C" fn kfunc_e(
    a__arena: *mut root_struct,
    b__arena__nullable: *mut root_struct,
    c__arena: *mut root_struct,
    d__arena__nullable: *mut root_struct,
    e__arena: *mut root_struct,
) -> ::core::ffi::c_int {
    let _ = a__arena;
    let _ = b__arena__nullable;
    let _ = c__arena;
    let _ = d__arena__nullable;
    let _ = e__arena;
    0
}

#[no_mangle]
pub unsafe extern "C" fn kfunc_f(
    a: *mut root_struct,
    b__arena: *mut root_struct,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let _ = a;
    let _ = b__arena;
    let _ = flags;
    0
}

#[no_mangle]
pub unsafe extern "C" fn kfunc_g(
    a__arena: *mut root_struct,
    b__arena__nullable: *mut root_struct,
) -> *mut root_struct {
    let _ = b__arena__nullable;
    a__arena
}
