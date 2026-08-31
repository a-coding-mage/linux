// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
// C dependencies: "bpf_experimental.h", <bpf/bpf_helpers.h>, "bpf_misc.h", <stdbool.h>

pub type __s8 = i8;
pub type __u8 = u8;
pub type __s16 = i16;
pub type __u16 = u16;
pub type __s32 = i32;
pub type __u32 = u32;
pub type __s64 = i64;
pub type __u64 = u64;

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [b'G' as ::core::ffi::c_char, b'P' as ::core::ffi::c_char, b'L' as ::core::ffi::c_char, 0];

pub type s32 = __s32;
pub type i32_t = s32;
pub type u8_t = __u8;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Enum {
    EA1 = 0,
    EA2 = 11,
    EA3 = 10,
}

pub const EA1: Enum = Enum::EA1;
pub const EA2: Enum = Enum::EA2;
pub const EA3: Enum = Enum::EA3;

pub type Enumu64 = __u64;
pub const EB1: Enumu64 = 0u64;
pub const EB2: Enumu64 = 12u64;

pub type Enums64 = __s64;
pub const EC1: Enums64 = 0i64;
pub const EC2: Enums64 = 13i64;

#[unsafe(no_mangle)]
pub static mut var_s64: __s64 = -1;
#[unsafe(no_mangle)]
pub static mut var_u64: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut var_s32: i32_t = -1;
#[unsafe(no_mangle)]
pub static mut var_u32: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut var_s16: __s16 = -1;
#[unsafe(no_mangle)]
pub static mut var_u16: __u16 = 0;
#[unsafe(no_mangle)]
pub static mut var_s8: __s8 = -1;
#[unsafe(no_mangle)]
pub static mut var_u8: u8_t = 0;
#[unsafe(no_mangle)]
pub static mut var_ea: Enum = Enum::EA1;
#[unsafe(no_mangle)]
pub static mut var_eb: Enumu64 = EB1;
#[unsafe(no_mangle)]
pub static mut var_ec: Enums64 = EC1;
#[unsafe(no_mangle)]
pub static mut var_b: bool = false;
#[unsafe(no_mangle)]
pub static mut arr: [i32_t; 32] = [0; 32];
#[unsafe(no_mangle)]
pub static mut enum_arr: [Enum; 32] = [Enum::EA1; 32];
#[unsafe(no_mangle)]
pub static mut three_d: [[[i32_t; 17]; 19]; 47] = [[[0; 17]; 19]; 47];
#[unsafe(no_mangle)]
pub static mut ptr_arr: [*const volatile_i32; 32] = [::core::ptr::null(); 32];

#[repr(transparent)]
pub struct volatile_i32(pub i32_t);

#[repr(C)]
#[derive(Copy, Clone)]
pub union Struct2AnonUnion {
    pub var_u8: [u8_t; 3],
    pub filler3: __s16,
    // C also has an unnamed `const int:1` bit-field in this union.
    pub mat: [[s32; 5]; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct2Anon {
    // C has an unnamed `const int:1` bit-field before this union.
    pub u: Struct2AnonUnion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct2 {
    pub filler: __u16,
    pub __bindgen_anon_1: Struct2Anon,
}

impl Struct2 {
    #[inline]
    pub fn u(&self) -> &Struct2AnonUnion {
        &self.__bindgen_anon_1.u
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct {
    // C starts with an unnamed `int:16` bit-field.
    pub __bindgen_padding_0: [u8; 2],
    pub filler: __u16,
    pub filler2: __u16,
    pub struct2: [[Struct2; 4]; 2],
}

#[unsafe(no_mangle)]
pub static mut stru: __u32 = 0; /* same prefix as below */
#[unsafe(no_mangle)]
pub static mut struct1: [Struct; 3] = [Struct {
    __bindgen_padding_0: [0; 2],
    filler: 0,
    filler2: 0,
    struct2: [[Struct2 {
        filler: 0,
        __bindgen_anon_1: Struct2Anon {
            u: Struct2AnonUnion { mat: [[0; 5]; 7] },
        },
    }; 4]; 2],
}; 3];
#[unsafe(no_mangle)]
pub static mut struct11: [[Struct; 7]; 11] = [[Struct {
    __bindgen_padding_0: [0; 2],
    filler: 0,
    filler2: 0,
    struct2: [[Struct2 {
        filler: 0,
        __bindgen_anon_1: Struct2Anon {
            u: Struct2AnonUnion { mat: [[0; 5]; 7] },
        },
    }; 4]; 2],
}; 7]; 11];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct3Anon1 {
    pub var_u8_l: u8_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct3Anon3 {
    pub var_u8_h: u8_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct3Anon2 {
    pub __bindgen_anon_1: Struct3Anon3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct3 {
    pub __bindgen_anon_1: Struct3Anon1,
    pub __bindgen_anon_2: Struct3Anon2,
}

pub type Struct3_t = Struct3;

#[repr(C)]
#[derive(Copy, Clone)]
pub union Union {
    pub var_u16: __u16,
    pub struct3: Struct3_t,
}

#[unsafe(no_mangle)]
pub static mut union1: Union = Union {
    var_u16: (-1i32) as __u16,
};

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_set_globals(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut a: __s8;

    let _ = ctx;

    a = ::core::ptr::read_volatile(&raw const var_s64) as __s8;
    a = ::core::ptr::read_volatile(&raw const var_u64) as __s8;
    a = ::core::ptr::read_volatile(&raw const var_s32) as __s8;
    a = ::core::ptr::read_volatile(&raw const var_u32) as __s8;
    a = ::core::ptr::read_volatile(&raw const var_s16) as __s8;
    a = ::core::ptr::read_volatile(&raw const var_u16) as __s8;
    a = ::core::ptr::read_volatile(&raw const var_s8) as __s8;
    a = ::core::ptr::read_volatile(&raw const var_u8) as __s8;
    a = ::core::ptr::read_volatile(&raw const var_ea) as __s8;
    a = ::core::ptr::read_volatile(&raw const var_eb) as __s8;
    a = ::core::ptr::read_volatile(&raw const var_ec) as __s8;
    a = ::core::ptr::read_volatile(&raw const var_b) as __s8;
    a = ::core::ptr::read_volatile(&raw const struct1[2].struct2[1][2].u().var_u8[2]) as __s8;
    a = ::core::ptr::read_volatile(&raw const union1.var_u16) as __s8;
    a = ::core::ptr::read_volatile(&raw const arr[3]) as __s8;
    a = ::core::ptr::read_volatile(&raw const arr[EA2 as usize]) as __s8;
    a = ::core::ptr::read_volatile(&raw const enum_arr[EC2 as usize]) as __s8;
    a = ::core::ptr::read_volatile(&raw const three_d[31][7][EA2 as usize]) as __s8;
    a = ::core::ptr::read_volatile(&raw const struct1[2].struct2[1][2].u().mat[5][3]) as __s8;
    a = ::core::ptr::read_volatile(&raw const struct11[7][5].struct2[0][1].u().mat[3][0]) as __s8;

    a as ::core::ffi::c_int
}
