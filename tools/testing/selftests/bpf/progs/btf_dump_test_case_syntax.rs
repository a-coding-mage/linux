// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * BTF-to-C dumper test for majority of C syntax quirks.
 *
 * Copyright (c) 2019 Facebook
 */
/* ----- START-EXPECTED-OUTPUT ----- */
#[repr(C)]
pub enum e1 {
    A = 0,
    B = 1,
}

#[repr(u32)]
pub enum e2 {
    C = 100,
    D = 4294967295,
    E = 0,
}

pub type e2_t = e2;

#[repr(C)]
pub enum e3_t {
    F = 0,
    G = 1,
    H = 2,
}

/* ----- START-EXPECTED-OUTPUT ----- */
/*
 *enum e_byte {
 *	EBYTE_1 = 0,
 *	EBYTE_2 = 1,
 *} __attribute__((mode(byte)));
 *
 */
/* ----- END-EXPECTED-OUTPUT ----- */
#[repr(u8)]
pub enum e_byte {
    EBYTE_1 = 0,
    EBYTE_2 = 1,
}

/* ----- START-EXPECTED-OUTPUT ----- */
/*
 *enum e_word {
 *	EWORD_1 = 0LL,
 *	EWORD_2 = 1LL,
 *} __attribute__((mode(word)));
 *
 */
/* ----- END-EXPECTED-OUTPUT ----- */
#[repr(u64)]
pub enum e_word {
    EWORD_1 = 0,
    EWORD_2 = 1,
} /* force to use 8-byte backing for this enum */

/* ----- START-EXPECTED-OUTPUT ----- */
#[repr(u64)]
pub enum e_big {
    EBIG_1 = 1000000000000,
}

pub type int_t = i32;

pub type crazy_ptr_t = *const int_t;

pub type we_need_to_go_deeper_ptr_t = *mut *mut *mut *mut *mut int_t;

pub type how_about_this_ptr_t =
    *const *const *const *mut *const *mut *const we_need_to_go_deeper_ptr_t;

pub type ptr_arr_t = [*mut int_t; 10];

pub type fn_ptr1_t = Option<unsafe extern "C" fn(i32)>;

pub type printf_fn_t = Option<unsafe extern "C" fn(*const core::ffi::c_char, ...)>;

/* ------ END-EXPECTED-OUTPUT ------ */
/*
 * While previous function pointers are pretty trivial (C-syntax-level
 * trivial), the following are deciphered here for future generations:
 *
 * - `fn_ptr2_t`: function, taking anonymous struct as a first arg and pointer
 *   to a function, that takes int and returns int, as a second arg; returning
 *   a pointer to a const pointer to a char. Equivalent to:
 *	typedef struct { int a; } s_t;
 *	typedef int (*fn_t)(int);
 *	typedef char * const * (*fn_ptr2_t)(s_t, fn_t);
 *
 * - `fn_complex_t`: pointer to a function returning struct and accepting
 *   union and struct. All structs and enum are anonymous and defined inline.
 *
 * - `signal_t: pointer to a function accepting a pointer to a function as an
 *   argument and returning pointer to a function as a result. Sane equivalent:
 *	typedef void (*signal_handler_t)(int);
 *	typedef signal_handler_t (*signal_ptr_t)(int, signal_handler_t);
 *
 * - fn_ptr_arr1_t: array of pointers to a function accepting pointer to
 *   a pointer to an int and returning pointer to a char. Easy.
 *
 * - fn_ptr_arr2_t: array of const pointers to a function taking no arguments
 *   and returning a const pointer to a function, that takes pointer to a
 *   `int -> char *` function and returns pointer to a char. Equivalent:
 *   typedef char * (*fn_input_t)(int);
 *   typedef char * (*fn_output_outer_t)(fn_input_t);
 *   typedef const fn_output_outer_t (* fn_output_inner_t)(void);
 *   typedef const fn_output_inner_t fn_ptr_arr2_t[5];
 */
/* ----- START-EXPECTED-OUTPUT ----- */
#[repr(C)]
pub struct fn_ptr2_t_arg1 {
    pub a: i32,
}

pub type fn_ptr2_t = Option<
    unsafe extern "C" fn(
        fn_ptr2_t_arg1,
        Option<unsafe extern "C" fn(i32) -> i32>,
    ) -> *const *mut core::ffi::c_char,
>;

#[repr(C)]
pub struct fn_complex_t_ret_b_arg2 {
    pub c: i32,
}

#[repr(C)]
pub union fn_complex_t_ret_b_arg3 {
    pub d: core::ffi::c_char,
    pub e: [i32; 5],
}

#[repr(C)]
pub struct fn_complex_t_ret {
    pub a: i32,
    pub b: Option<
        unsafe extern "C" fn(i32, fn_complex_t_ret_b_arg2, fn_complex_t_ret_b_arg3),
    >,
}

#[repr(C)]
pub union fn_complex_t_arg1 {
    pub f: *mut core::ffi::c_void,
    pub g: [core::ffi::c_char; 16],
}

#[repr(C)]
pub struct fn_complex_t_arg2 {
    pub h: i32,
}

pub type fn_complex_t =
    Option<unsafe extern "C" fn(fn_complex_t_arg1, fn_complex_t_arg2) -> fn_complex_t_ret>;

pub type signal_t = Option<
    unsafe extern "C" fn(
        i32,
        Option<unsafe extern "C" fn(i32)>,
    ) -> Option<unsafe extern "C" fn(i32)>,
>;

pub type fn_ptr_arr1_t = [Option<unsafe extern "C" fn(*mut *mut i32) -> *mut core::ffi::c_char>; 10];

pub type fn_ptr_arr2_t = [Option<
    unsafe extern "C" fn() -> Option<
        unsafe extern "C" fn(
            Option<unsafe extern "C" fn(i32) -> *mut core::ffi::c_char>,
        ) -> *mut core::ffi::c_char,
    >,
>; 5];

#[repr(C)]
pub struct struct_w_typedefs {
    pub a: int_t,
    pub b: crazy_ptr_t,
    pub c: we_need_to_go_deeper_ptr_t,
    pub d: how_about_this_ptr_t,
    pub e: ptr_arr_t,
    pub f: fn_ptr1_t,
    pub g: printf_fn_t,
    pub h: fn_ptr2_t,
    pub i: fn_complex_t,
    pub j: signal_t,
    pub k: fn_ptr_arr1_t,
    pub l: fn_ptr_arr2_t,
}

#[repr(C)]
pub struct anon_struct_t {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub enum struct_fwd {}

pub type struct_fwd_t = struct_fwd;

pub type struct_fwd_ptr_t = *mut struct_fwd;

pub enum union_fwd {}

pub type union_fwd_t = union_fwd;

pub type union_fwd_ptr_t = *mut union_fwd;

#[repr(C)]
pub struct struct_empty {}

#[repr(C)]
pub enum struct_simple_f {
    ANON_VAL1 = 1,
    ANON_VAL2 = 2,
}

#[repr(C)]
pub struct struct_simple {
    pub a: i32,
    pub b: core::ffi::c_char,
    pub p: *const int_t,
    pub s: struct_empty,
    pub e: e2,
    pub f: struct_simple_f,
    pub arr1: [i32; 13],
    pub arr2: [e2; 5],
}

#[repr(C)]
pub union union_empty {}

#[repr(C)]
pub union union_simple {
    pub ptr: *mut core::ffi::c_void,
    pub num: i32,
    pub num2: int_t,
    pub u: union_empty,
}

#[repr(C)]
pub struct struct_in_struct_not_so_hard_as_well {
    pub a: i32,
}

#[repr(C)]
pub union struct_in_struct_anon_union_is_good {
    pub b: i32,
    pub c: i32,
}

#[repr(C)]
pub struct struct_in_struct_anon_struct {
    pub d: i32,
    pub e: i32,
}

#[repr(C)]
pub union struct_in_struct_anon_union {
    pub f: i32,
    pub g: i32,
}

#[repr(C)]
pub struct struct_in_struct {
    pub simple: struct_simple,
    pub also_simple: union_simple,
    pub not_so_hard_as_well: struct_in_struct_not_so_hard_as_well,
    pub anon_union_is_good: struct_in_struct_anon_union_is_good,
    pub anonymous_struct: struct_in_struct_anon_struct,
    pub anonymous_union: struct_in_struct_anon_union,
}

#[repr(C)]
pub struct struct_in_array {}

#[repr(C)]
pub struct struct_in_array_typed {}

pub type struct_in_array_t = [struct_in_array_typed; 2];

#[repr(C)]
pub struct struct_with_embedded_stuff_anon_struct_e {
    pub c: *mut struct_with_embedded_stuff,
    pub d: *const core::ffi::c_char,
}

#[repr(C)]
pub union struct_with_embedded_stuff_anon_struct_anon_union {
    pub f: core::ffi::c_long,
    pub g: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct struct_with_embedded_stuff_anon_struct {
    pub b: i32,
    pub e: struct_with_embedded_stuff_anon_struct_e,
    pub anonymous_union: struct_with_embedded_stuff_anon_struct_anon_union,
}

#[repr(C)]
pub union struct_with_embedded_stuff_j {
    pub h: *const int_t,
    pub i: Option<unsafe extern "C" fn(core::ffi::c_char, i32, *mut core::ffi::c_void)>,
}

#[repr(C)]
pub enum struct_with_embedded_stuff_m {
    K = 100,
    L = 200,
}

#[repr(C)]
pub struct struct_with_embedded_stuff_r {
    pub o: core::ffi::c_char,
    pub p: i32,
    pub q: Option<unsafe extern "C" fn(i32)>,
}

#[repr(C)]
pub struct struct_with_embedded_stuff {
    pub a: i32,
    pub anonymous_struct: struct_with_embedded_stuff_anon_struct,
    pub j: struct_with_embedded_stuff_j,
    pub m: struct_with_embedded_stuff_m,
    pub n: [core::ffi::c_char; 16],
    pub r: [struct_with_embedded_stuff_r; 5],
    pub s: [struct_in_struct; 10],
    pub t: [i32; 11],
    pub u: *mut [struct_in_array; 2],
    pub v: *mut struct_in_array_t,
}

#[repr(C)]
pub struct float_struct {
    pub f: f32,
    pub d: *const f64,
    pub ld: *mut core::ffi::c_longdouble,
}

#[repr(C)]
pub struct root_struct {
    pub _1: e1,
    pub _2: e2,
    pub _2_1: e2_t,
    pub _2_2: e3_t,
    pub _100: e_byte,
    pub _101: e_word,
    pub _102: e_big,
    pub _3: struct_w_typedefs,
    pub _7: anon_struct_t,
    pub _8: *mut struct_fwd,
    pub _9: *mut struct_fwd_t,
    pub _10: struct_fwd_ptr_t,
    pub _11: *mut union_fwd,
    pub _12: *mut union_fwd_t,
    pub _13: union_fwd_ptr_t,
    pub _14: struct_with_embedded_stuff,
    pub _15: float_struct,
}

/* ------ END-EXPECTED-OUTPUT ------ */

#[no_mangle]
pub unsafe extern "C" fn f(_s: *mut root_struct) -> i32 {
    return 0;
}
