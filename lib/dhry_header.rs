/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * "DHRYSTONE" Benchmark Program, C version 2.1.
 * This header contains global definitions and declarations.
 * The original program consists of dhry.h, dhry_1.c, and dhry_2.c.
 *
 * The benchmark intentionally preserves its original data types, layout,
 * declarations, and externally visible interfaces.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Enumeration {
    Ident_1,
    Ident_2,
    Ident_3,
    Ident_4,
    Ident_5,
}

/* General definitions. */
pub type One_Thirty = ::core::ffi::c_int;
pub type One_Fifty = ::core::ffi::c_int;
pub type Capital_Letter = ::core::ffi::c_char;
pub type Boolean = ::core::ffi::c_int;
pub type Str_30 = [::core::ffi::c_char; 31];
pub type Arr_1_Dim = [::core::ffi::c_int; 50];
pub type Arr_2_Dim = [[::core::ffi::c_int; 50]; 50];

#[repr(C)]
pub struct record {
    pub Ptr_Comp: *mut record,
    pub Discr: Enumeration,
    pub variant: record_variant,
}

#[repr(C)]
pub union record_variant {
    pub var_1: record_var_1,
    pub var_2: record_var_2,
    pub var_3: record_var_3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct record_var_1 {
    pub Enum_Comp: Enumeration,
    pub Int_Comp: ::core::ffi::c_int,
    pub Str_Comp: [::core::ffi::c_char; 31],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct record_var_2 {
    pub E_Comp_2: Enumeration,
    pub Str_2_Comp: [::core::ffi::c_char; 31],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct record_var_3 {
    pub Ch_1_Comp: ::core::ffi::c_char,
    pub Ch_2_Comp: ::core::ffi::c_char,
}

pub type Rec_Type = record;
pub type Rec_Pointer = *mut Rec_Type;

unsafe extern "C" {
    pub static mut Int_Glob: ::core::ffi::c_int;
    pub static mut Ch_1_Glob: ::core::ffi::c_char;

    pub fn Proc_6(Enum_Val_Par: Enumeration, Enum_Ref_Par: *mut Enumeration);
    pub fn Proc_7(
        Int_1_Par_Val: One_Fifty,
        Int_2_Par_Val: One_Fifty,
        Int_Par_Ref: *mut One_Fifty,
    );
    pub fn Proc_8(
        Arr_1_Par_Ref: *mut ::core::ffi::c_int,
        Arr_2_Par_Ref: *mut [[::core::ffi::c_int; 50]; 50],
        Int_1_Par_Val: ::core::ffi::c_int,
        Int_2_Par_Val: ::core::ffi::c_int,
    );
    pub fn Func_1(
        Ch_1_Par_Val: Capital_Letter,
        Ch_2_Par_Val: Capital_Letter,
    ) -> Enumeration;
    pub fn Func_2(Str_1_Par_Ref: *mut ::core::ffi::c_char, Str_2_Par_Ref: *mut ::core::ffi::c_char) -> Boolean;

    pub fn dhry(n: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
