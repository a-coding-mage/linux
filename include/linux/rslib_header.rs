// SPDX-License-Identifier: GPL-2.0
/*
 * Generic Reed Solomon encoder / decoder library
 *
 * Copyright (C) 2004 Thomas Gleixner (tglx@kernel.org)
 *
 * RS code lifted from reed solomon library written by Phil Karn
 * Copyright 2002 Phil Karn, KA9Q
 */

// linux/types.h and linux/gfp.h provide gfp_t and GFP_KERNEL.

#[repr(C)]
pub struct rs_codec {
    pub mm: ::core::ffi::c_int,
    pub nn: ::core::ffi::c_int,
    pub alpha_to: *mut u16,
    pub index_of: *mut u16,
    pub genpoly: *mut u16,
    pub nroots: ::core::ffi::c_int,
    pub fcr: ::core::ffi::c_int,
    pub prim: ::core::ffi::c_int,
    pub iprim: ::core::ffi::c_int,
    pub gfpoly: ::core::ffi::c_int,
    pub gffunc: ::core::option::Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub users: ::core::ffi::c_int,
    pub list: list_head,
}

#[repr(C)]
pub struct rs_control {
    pub codec: *mut rs_codec,
    pub buffers: [u16; 0],
}

// General purpose RS codec, 8-bit data width, symbol width 1-15 bit.
#[cfg(feature = "CONFIG_REED_SOLOMON_ENC8")]
unsafe extern "C" {
    pub fn encode_rs8(
        rs: *mut rs_control,
        data: *mut u8,
        len: ::core::ffi::c_int,
        par: *mut u16,
        invmsk: u16,
    ) -> ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_REED_SOLOMON_DEC8")]
unsafe extern "C" {
    pub fn decode_rs8(
        rs: *mut rs_control,
        data: *mut u8,
        par: *mut u16,
        len: ::core::ffi::c_int,
        s: *mut u16,
        no_eras: ::core::ffi::c_int,
        eras_pos: *mut ::core::ffi::c_int,
        invmsk: u16,
        corr: *mut u16,
    ) -> ::core::ffi::c_int;
}

// General purpose RS codec, 16-bit data width, symbol width 1-15 bit.
#[cfg(feature = "CONFIG_REED_SOLOMON_ENC16")]
unsafe extern "C" {
    pub fn encode_rs16(
        rs: *mut rs_control,
        data: *mut u16,
        len: ::core::ffi::c_int,
        par: *mut u16,
        invmsk: u16,
    ) -> ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_REED_SOLOMON_DEC16")]
unsafe extern "C" {
    pub fn decode_rs16(
        rs: *mut rs_control,
        data: *mut u16,
        par: *mut u16,
        len: ::core::ffi::c_int,
        s: *mut u16,
        no_eras: ::core::ffi::c_int,
        eras_pos: *mut ::core::ffi::c_int,
        invmsk: u16,
        corr: *mut u16,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    pub fn init_rs_gfp(
        symsize: ::core::ffi::c_int,
        gfpoly: ::core::ffi::c_int,
        fcr: ::core::ffi::c_int,
        prim: ::core::ffi::c_int,
        nroots: ::core::ffi::c_int,
        gfp: gfp_t,
    ) -> *mut rs_control;

    pub fn init_rs_non_canonical(
        symsize: ::core::ffi::c_int,
        func: ::core::option::Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
        fcr: ::core::ffi::c_int,
        prim: ::core::ffi::c_int,
        nroots: ::core::ffi::c_int,
    ) -> *mut rs_control;

    pub fn free_rs(rs: *mut rs_control);
}

#[inline]
pub unsafe fn init_rs(
    symsize: ::core::ffi::c_int,
    gfpoly: ::core::ffi::c_int,
    fcr: ::core::ffi::c_int,
    prim: ::core::ffi::c_int,
    nroots: ::core::ffi::c_int,
) -> *mut rs_control {
    init_rs_gfp(symsize, gfpoly, fcr, prim, nroots, GFP_KERNEL)
}

#[inline]
pub unsafe fn rs_modnn(rs: *mut rs_codec, mut x: ::core::ffi::c_int) -> ::core::ffi::c_int {
    while x >= (*rs).nn {
        x -= (*rs).nn;
        x = (x >> (*rs).mm) + (x & (*rs).nn);
    }
    x
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
