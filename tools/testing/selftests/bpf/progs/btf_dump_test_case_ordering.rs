// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * BTF-to-C dumper test for topological sorting of dependent structs.
 *
 * Copyright (c) 2019 Facebook
 */
/* ----- START-EXPECTED-OUTPUT ----- */
#[repr(C)]
pub struct s1 {}

/* forward declaration: struct s3; */

/* forward declaration: struct s4; */

#[repr(C)]
pub struct s2 {
    pub s2: *mut s2,
    pub s3: *mut s3,
    pub s4: *mut s4,
}

#[repr(C)]
pub struct s3 {
    pub s1: s1,
    pub s2: s2,
}

#[repr(C)]
pub struct s4 {
    pub s1: s1,
    pub s3: s3,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

#[repr(C)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
pub struct callback_head {
    pub next: *mut callback_head,
    pub func: Option<unsafe extern "C" fn(*mut callback_head)>,
}

#[repr(C)]
pub struct root_struct {
    pub s4: s4,
    pub l: list_head,
    pub n: hlist_node,
    pub h: hlist_head,
    pub cb: callback_head,
}

/*------ END-EXPECTED-OUTPUT ------ */

#[no_mangle]
pub unsafe extern "C" fn f(root: *mut root_struct) -> ::std::os::raw::c_int {
    let _ = root;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
