// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * BTF-to-C dumper tests for bitfield.
 *
 * Copyright (c) 2019 Facebook
 */
// C dependency intent: #include <stdbool.h>

/* ----- START-EXPECTED-OUTPUT ----- */
/*
 *struct bitfields_only_mixed_types {
 *	int a: 3;
 *	long b: 2;
 *	_Bool c: 1;
 *	enum {
 *		A = 0,
 *		B = 1,
 *	} d: 1;
 *	short e: 5;
 *	int: 20;
 *	unsigned int f: 30;
 *};
 *
 */
/* ------ END-EXPECTED-OUTPUT ------ */

#[repr(C)]
pub struct bitfields_only_mixed_types {
    // int a: 3;
    // long b: 2;
    // bool c: 1; /* it's really a _Bool type */
    // enum {
    //     A, /* A = 0, dumper is very explicit */
    //     B, /* B = 1, same */
    // } d: 1;
    // short e: 5;
    // /* 20-bit padding here */
    // unsigned f: 30; /* this gets aligned on 4-byte boundary */
    pub __bitfield_storage_0: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum bitfields_only_mixed_types__d {
    A = 0,
    B = 1,
}

/* ----- START-EXPECTED-OUTPUT ----- */
/*
 *struct bitfield_mixed_with_others {
 *	char: 4;
 *	int a: 4;
 *	short b;
 *	long c;
 *	long d: 8;
 *	int e;
 *	int f;
 *};
 *
 */
/* ------ END-EXPECTED-OUTPUT ------ */
#[repr(C)]
pub struct bitfield_mixed_with_others {
    // char: 4; /* char is enough as a backing field */
    // int a: 4;
    // /* 8-bit implicit padding */
    // short b; /* combined with previous bitfield */
    pub __bitfield_storage_0: u32,
    // /* 4 more bytes of implicit padding */
    pub __implicit_padding_0: [u8; 4],
    pub c: core::ffi::c_long,
    // long d: 8;
    // /* 24 bits implicit padding */
    // int e; /* combined with previous bitfield */
    pub __bitfield_storage_1: u32,
    pub f: core::ffi::c_int,
    // /* 4 bytes of padding */
    pub __implicit_padding_1: [u8; 4],
}

/* ----- START-EXPECTED-OUTPUT ----- */
/*
 *struct bitfield_flushed {
 *	int a: 4;
 *	long: 60;
 *	long b: 16;
 *};
 *
 */
/* ------ END-EXPECTED-OUTPUT ------ */
#[repr(C)]
pub struct bitfield_flushed {
    // int a: 4;
    pub __bitfield_storage_0: core::ffi::c_int,
    // long: 0; /* flush until next natural alignment boundary */
    pub __flush_padding_0: core::ffi::c_int,
    // long b: 16;
    pub __bitfield_storage_1: core::ffi::c_long,
}

#[repr(C)]
pub struct f__anon_param {
    pub _1: bitfields_only_mixed_types,
    pub _2: bitfield_mixed_with_others,
    pub _3: bitfield_flushed,
}

pub unsafe extern "C" fn f(_: *mut f__anon_param) -> core::ffi::c_int {
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
