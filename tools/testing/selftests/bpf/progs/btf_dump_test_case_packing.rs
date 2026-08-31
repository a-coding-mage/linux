// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * BTF-to-C dumper tests for struct packing determination.
 *
 * Copyright (c) 2019 Facebook
 */
/* ----- START-EXPECTED-OUTPUT ----- */
#[repr(C, packed)]
pub struct packed_trailing_space {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_short,
}

#[repr(C)]
pub struct non_packed_trailing_space {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_short,
}

#[repr(C, packed)]
pub struct packed_fields {
    pub a: ::std::os::raw::c_short,
    pub b: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct non_packed_fields {
    pub a: ::std::os::raw::c_short,
    pub b: ::std::os::raw::c_int,
}

#[repr(C, packed)]
pub struct nested_packed {
    /* char: 4; int a: 4; */
    pub __bindgen_bitfield_1: ::std::os::raw::c_uchar,
    pub b: ::std::os::raw::c_long,
    pub e: nested_packed__bindgen_ty_1,
}

#[repr(C, packed)]
pub struct nested_packed__bindgen_ty_1 {
    pub c: ::std::os::raw::c_char,
    pub d: ::std::os::raw::c_int,
}

#[repr(C)]
pub union union_is_never_packed {
    /* int a: 4; */
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_char,
    /* char c: 1; */
    pub c: ::std::os::raw::c_char,
}

#[repr(C)]
pub union union_does_not_need_packing {
    pub __bindgen_anon_1: union_does_not_need_packing__bindgen_ty_1,
    pub c: ::std::os::raw::c_int,
}

#[repr(C, packed)]
pub struct union_does_not_need_packing__bindgen_ty_1 {
    pub a: ::std::os::raw::c_long,
    pub b: ::std::os::raw::c_int,
}

#[repr(C)]
pub union jump_code_union {
    pub code: [::std::os::raw::c_char; 5],
    pub __bindgen_anon_1: jump_code_union__bindgen_ty_1,
}

#[repr(C, packed)]
pub struct jump_code_union__bindgen_ty_1 {
    pub jump: ::std::os::raw::c_char,
    pub offset: ::std::os::raw::c_int,
}

/* ----- START-EXPECTED-OUTPUT ----- */
/*
 *struct nested_packed_but_aligned_struct {
 *	int x1;
 *	int x2;
 *};
 *
 *struct outer_implicitly_packed_struct {
 *	char y1;
 *	struct nested_packed_but_aligned_struct y2;
 *} __attribute__((packed));
 *
 */
/* ------ END-EXPECTED-OUTPUT ------ */

#[repr(C, packed)]
pub struct nested_packed_but_aligned_struct {
    pub x1: ::std::os::raw::c_int,
    pub x2: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct outer_implicitly_packed_struct {
    pub y1: ::std::os::raw::c_char,
    pub y2: nested_packed_but_aligned_struct,
}
/* ----- START-EXPECTED-OUTPUT ----- */
/*
 *struct usb_ss_ep_comp_descriptor {
 *	char: 8;
 *	char bDescriptorType;
 *	char bMaxBurst;
 *	short wBytesPerInterval;
 *};
 *
 *struct usb_host_endpoint {
 *	long: 64;
 *	char: 8;
 *	struct usb_ss_ep_comp_descriptor ss_ep_comp;
 *	long: 0;
 *} __attribute__((packed));
 *
 */
/* ------ END-EXPECTED-OUTPUT ------ */

#[repr(C, packed)]
pub struct usb_ss_ep_comp_descriptor {
    /* char: 8; */
    pub __bindgen_bitfield_1: ::std::os::raw::c_uchar,
    pub bDescriptorType: ::std::os::raw::c_char,
    pub bMaxBurst: ::std::os::raw::c_char,
    /* int: 0; */
    pub wBytesPerInterval: ::std::os::raw::c_short,
}

#[repr(C)]
pub struct usb_host_endpoint {
    /* long: 64; */
    pub __bindgen_bitfield_1: ::std::os::raw::c_ulong,
    /* char: 8; */
    pub __bindgen_bitfield_2: ::std::os::raw::c_uchar,
    pub ss_ep_comp: usb_ss_ep_comp_descriptor,
    /* long: 0; */
}

/* ----- START-EXPECTED-OUTPUT ----- */
#[repr(C, packed)]
pub struct nested_packed_struct {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_char,
}

#[repr(C)]
pub struct outer_nonpacked_struct {
    pub a: ::std::os::raw::c_short,
    pub b: nested_packed_struct,
}

#[repr(C, packed)]
pub struct outer_packed_struct {
    pub a: ::std::os::raw::c_short,
    pub b: nested_packed_struct,
}

/* ------ END-EXPECTED-OUTPUT ------ */

#[repr(C)]
pub struct f__bindgen_ty_1 {
    pub _1: packed_trailing_space,
    pub _2: non_packed_trailing_space,
    pub _3: packed_fields,
    pub _4: non_packed_fields,
    pub _5: nested_packed,
    pub _6: union_is_never_packed,
    pub _7: union_does_not_need_packing,
    pub _8: jump_code_union,
    pub _9: outer_implicitly_packed_struct,
    pub _10: usb_host_endpoint,
    pub _11: outer_nonpacked_struct,
    pub _12: outer_packed_struct,
}

#[no_mangle]
pub unsafe extern "C" fn f(_: *mut f__bindgen_ty_1) -> ::std::os::raw::c_int {
    return 0;
}
