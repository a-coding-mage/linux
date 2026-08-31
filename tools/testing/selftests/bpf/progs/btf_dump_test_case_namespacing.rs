// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * BTF-to-C dumper test validating no name versioning happens between
 * independent C namespaces (struct/union/enum vs typedef/enum values).
 *
 * Copyright (c) 2019 Facebook
 */
/* ----- START-EXPECTED-OUTPUT ----- */
#[repr(C)]
pub struct S {
    pub S: ::std::os::raw::c_int,
    pub U: ::std::os::raw::c_int,
}

/* typedef struct S S; -- Rust has one type namespace, so S already names this type. */

#[repr(C)]
pub union U {
    pub S: ::std::os::raw::c_int,
    pub U: ::std::os::raw::c_int,
}

/* typedef union U U; -- Rust has one type namespace, so U already names this type. */

#[repr(C)]
pub enum E {
    V = 0,
}

/* typedef enum E E; -- Rust has one type namespace, so E already names this type. */

#[repr(C)]
pub struct A {}

#[repr(C)]
pub union B {}

#[repr(C)]
pub enum C {
    A = 1,
    B = 2,
    C = 3,
}

#[repr(C)]
pub struct X {}

#[repr(C)]
pub union Y {}

/* enum Z; -- forward declaration only in C. */

/* typedef int X; -- not expressible alongside struct X in Rust's single type namespace. */
/* typedef int Y; -- not expressible alongside union Y in Rust's single type namespace. */
pub type Z = ::std::os::raw::c_int;

/*------ END-EXPECTED-OUTPUT ------ */

#[repr(C)]
pub struct f_param {
    pub _1: S,
    pub _2: S,
    pub _3: U,
    pub _4: U,
    pub _5: E,
    pub _6: E,
    pub a: A,
    pub b: B,
    pub c: C,
    pub x: X,
    pub y: Y,
    pub z: *mut Z,
    /* X and Y below are C typedef names for int, distinct from struct X/union Y. */
    pub xx: ::std::os::raw::c_int,
    pub yy: ::std::os::raw::c_int,
    pub zz: Z,
}

#[no_mangle]
pub unsafe extern "C" fn f(_: *mut f_param) -> ::std::os::raw::c_int {
    return 0;
}
