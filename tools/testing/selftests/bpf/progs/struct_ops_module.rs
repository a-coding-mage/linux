// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
// "../test_kmods/bpf_testmod.h"

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_1: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub test_2: Option<
        unsafe extern "C" fn(a: ::core::ffi::c_int, b: ::core::ffi::c_int),
    >,
    pub test_maybe_null: Option<
        unsafe extern "C" fn(
            dummy: ::core::ffi::c_int,
            task: *mut task_struct,
        ) -> ::core::ffi::c_int,
    >,
    pub data: ::core::ffi::c_int,
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

#[unsafe(no_mangle)]
pub static mut test_1_result: ::core::ffi::c_int = 0;

#[unsafe(no_mangle)]
pub static mut test_2_result: ::core::ffi::c_int = 0;

#[unsafe(link_section = "struct_ops/test_1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_1() -> ::core::ffi::c_int {
    unsafe {
        test_1_result = 0xdeadbeefu32 as ::core::ffi::c_int;
    }
    0
}

#[unsafe(link_section = "struct_ops/test_2")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_2(a: ::core::ffi::c_int, b: ::core::ffi::c_int) {
    unsafe {
        test_2_result = a.wrapping_add(b);
    }
}

#[unsafe(link_section = "?struct_ops/test_3")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_3(
    a: ::core::ffi::c_int,
    b: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let result = a.wrapping_add(b).wrapping_add(3);
    unsafe {
        test_2_result = result;
    }
    result
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod_1: bpf_testmod_ops = bpf_testmod_ops {
    test_1: Some(test_1),
    test_2: Some(test_2),
    test_maybe_null: None,
    data: 0x1,
};

#[unsafe(link_section = "struct_ops/test_2")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_2_v2(a: ::core::ffi::c_int, b: ::core::ffi::c_int) {
    unsafe {
        test_2_result = a.wrapping_mul(b);
    }
}

#[repr(C)]
pub struct bpf_testmod_ops___v2 {
    pub test_1: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub test_2: Option<
        unsafe extern "C" fn(a: ::core::ffi::c_int, b: ::core::ffi::c_int),
    >,
    pub test_maybe_null: Option<
        unsafe extern "C" fn(
            dummy: ::core::ffi::c_int,
            task: *mut task_struct,
        ) -> ::core::ffi::c_int,
    >,
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod_2: bpf_testmod_ops___v2 = bpf_testmod_ops___v2 {
    test_1: Some(test_1),
    test_2: Some(test_2_v2),
    test_maybe_null: None,
};

#[repr(C)]
pub struct bpf_testmod_ops___zeroed {
    pub test_1: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub test_2: Option<
        unsafe extern "C" fn(a: ::core::ffi::c_int, b: ::core::ffi::c_int),
    >,
    pub test_maybe_null: Option<
        unsafe extern "C" fn(
            dummy: ::core::ffi::c_int,
            task: *mut task_struct,
        ) -> ::core::ffi::c_int,
    >,
    pub zeroed_op: Option<
        unsafe extern "C" fn(a: ::core::ffi::c_int, b: ::core::ffi::c_int),
    >,
    pub zeroed: ::core::ffi::c_int,
}

#[unsafe(link_section = "struct_ops/test_3")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zeroed_op() -> ::core::ffi::c_int {
    1
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod_zeroed: bpf_testmod_ops___zeroed = bpf_testmod_ops___zeroed {
    test_1: Some(test_1),
    test_2: Some(test_2_v2),
    test_maybe_null: None,
    zeroed_op: Some(unsafe {
        ::core::mem::transmute::<
            unsafe extern "C" fn() -> ::core::ffi::c_int,
            unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int),
        >(zeroed_op)
    }),
    zeroed: 0,
};

#[repr(C)]
pub struct bpf_testmod_ops___incompatible {
    pub test_1: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub test_2: Option<unsafe extern "C" fn(a: *mut ::core::ffi::c_int)>,
    pub data: ::core::ffi::c_int,
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod_incompatible: bpf_testmod_ops___incompatible =
    bpf_testmod_ops___incompatible {
        test_1: Some(test_1),
        test_2: Some(unsafe {
            ::core::mem::transmute::<
                unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int),
                unsafe extern "C" fn(*mut ::core::ffi::c_int),
            >(test_2)
        }),
        data: 3,
    };

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
