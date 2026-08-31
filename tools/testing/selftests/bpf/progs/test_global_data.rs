// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Isovalent, Inc.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_ARRAY: __u32 = 2;
const TC_ACT_OK: i32 = 0;

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

unsafe extern "C" {
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i64;
}

#[repr(C)]
struct result_number_map_def {
    type_: __u32,
    max_entries: __u32,
    key_size: __u32,
    value_size: __u32,
}

#[unsafe(link_section = ".maps")]
static mut result_number: result_number_map_def = result_number_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 11,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[repr(C)]
struct result_string_map_def {
    type_: __u32,
    max_entries: __u32,
    key_size: __u32,
    value_size: __u32,
}

#[unsafe(link_section = ".maps")]
static mut result_string: result_string_map_def = result_string_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 5,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<[i8; 32]>() as __u32,
};

#[repr(C)]
struct foo {
    a: __u8,
    b: __u32,
    c: __u64,
}

#[repr(C)]
struct result_struct_map_def {
    type_: __u32,
    max_entries: __u32,
    key_size: __u32,
    value_size: __u32,
}

#[unsafe(link_section = ".maps")]
static mut result_struct: result_struct_map_def = result_struct_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 5,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<foo>() as __u32,
};

/* Relocation tests for __u64s. */
static mut num0: __u64 = 0;
static mut num1: __u64 = 42;
static num2: __u64 = 24;
static mut num3: __u64 = 0;
static mut num4: __u64 = 0xffeeff;
static num5: __u64 = 0xabab;
static num6: __u64 = 0xab;

/* Relocation tests for strings. */
static str0: [i8; 32] = [
    b'a' as i8, b'b' as i8, b'c' as i8, b'd' as i8, b'e' as i8, b'f' as i8, b'g' as i8,
    b'h' as i8, b'i' as i8, b'j' as i8, b'k' as i8, b'l' as i8, b'm' as i8, b'n' as i8,
    b'o' as i8, b'p' as i8, b'q' as i8, b'r' as i8, b's' as i8, b't' as i8, b'u' as i8,
    b'v' as i8, b'w' as i8, b'x' as i8, b'y' as i8, b'z' as i8, 0, 0, 0, 0, 0, 0,
];
static mut str1: [i8; 32] = [
    b'a' as i8, b'b' as i8, b'c' as i8, b'd' as i8, b'e' as i8, b'f' as i8, b'g' as i8,
    b'h' as i8, b'i' as i8, b'j' as i8, b'k' as i8, b'l' as i8, b'm' as i8, b'n' as i8,
    b'o' as i8, b'p' as i8, b'q' as i8, b'r' as i8, b's' as i8, b't' as i8, b'u' as i8,
    b'v' as i8, b'w' as i8, b'x' as i8, b'y' as i8, b'z' as i8, 0, 0, 0, 0, 0, 0,
];
static mut str2: [i8; 32] = [0; 32];

/* Relocation tests for structs. */
static struct0: foo = foo {
    a: 42,
    b: 0xfefeefef,
    c: 0x1111111111111111u64,
};
static mut struct1: foo = foo { a: 0, b: 0, c: 0 };
static struct2: foo = foo { a: 0, b: 0, c: 0 };
static mut struct3: foo = foo {
    a: 41,
    b: 0xeeeeefef,
    c: 0x2111111111111111u64,
};

unsafe fn test_reloc(
    map: *mut core::ffi::c_void,
    num: __u32,
    var: *const core::ffi::c_void,
) {
    let key: __u32 = num;
    unsafe {
        bpf_map_update_elem(
            map,
            (&key as *const __u32).cast::<core::ffi::c_void>(),
            var,
            0,
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub unsafe extern "C" fn load_static_data(skb: *mut __sk_buff) -> i32 {
    static bar: __u64 = !0u64;

    let _ = skb;

    unsafe {
        test_reloc(
            (&raw mut result_number).cast::<core::ffi::c_void>(),
            0,
            (&raw const num0).cast::<core::ffi::c_void>(),
        );
        test_reloc(
            (&raw mut result_number).cast::<core::ffi::c_void>(),
            1,
            (&raw const num1).cast::<core::ffi::c_void>(),
        );
        test_reloc(
            (&raw mut result_number).cast::<core::ffi::c_void>(),
            2,
            (&raw const num2).cast::<core::ffi::c_void>(),
        );
        test_reloc(
            (&raw mut result_number).cast::<core::ffi::c_void>(),
            3,
            (&raw const num3).cast::<core::ffi::c_void>(),
        );
        test_reloc(
            (&raw mut result_number).cast::<core::ffi::c_void>(),
            4,
            (&raw const num4).cast::<core::ffi::c_void>(),
        );
        test_reloc(
            (&raw mut result_number).cast::<core::ffi::c_void>(),
            5,
            (&raw const num5).cast::<core::ffi::c_void>(),
        );
        num4 = 1234;
        test_reloc(
            (&raw mut result_number).cast::<core::ffi::c_void>(),
            6,
            (&raw const num4).cast::<core::ffi::c_void>(),
        );
        test_reloc(
            (&raw mut result_number).cast::<core::ffi::c_void>(),
            7,
            (&raw const num0).cast::<core::ffi::c_void>(),
        );
        test_reloc(
            (&raw mut result_number).cast::<core::ffi::c_void>(),
            8,
            (&raw const num6).cast::<core::ffi::c_void>(),
        );

        test_reloc(
            (&raw mut result_string).cast::<core::ffi::c_void>(),
            0,
            (&raw const str0).cast::<core::ffi::c_void>(),
        );
        test_reloc(
            (&raw mut result_string).cast::<core::ffi::c_void>(),
            1,
            (&raw const str1).cast::<core::ffi::c_void>(),
        );
        test_reloc(
            (&raw mut result_string).cast::<core::ffi::c_void>(),
            2,
            (&raw const str2).cast::<core::ffi::c_void>(),
        );
        str1[5] = b'x' as i8;
        test_reloc(
            (&raw mut result_string).cast::<core::ffi::c_void>(),
            3,
            (&raw const str1).cast::<core::ffi::c_void>(),
        );
        core::ptr::copy_nonoverlapping(
            b"hello\0".as_ptr().cast::<i8>(),
            (&raw mut str2).cast::<i8>().add(2),
            core::mem::size_of_val(b"hello\0"),
        );
        test_reloc(
            (&raw mut result_string).cast::<core::ffi::c_void>(),
            4,
            (&raw const str2).cast::<core::ffi::c_void>(),
        );

        test_reloc(
            (&raw mut result_struct).cast::<core::ffi::c_void>(),
            0,
            (&raw const struct0).cast::<core::ffi::c_void>(),
        );
        test_reloc(
            (&raw mut result_struct).cast::<core::ffi::c_void>(),
            1,
            (&raw const struct1).cast::<core::ffi::c_void>(),
        );
        test_reloc(
            (&raw mut result_struct).cast::<core::ffi::c_void>(),
            2,
            (&raw const struct2).cast::<core::ffi::c_void>(),
        );
        test_reloc(
            (&raw mut result_struct).cast::<core::ffi::c_void>(),
            3,
            (&raw const struct3).cast::<core::ffi::c_void>(),
        );

        test_reloc(
            (&raw mut result_number).cast::<core::ffi::c_void>(),
            9,
            (&raw const struct0.c).cast::<core::ffi::c_void>(),
        );
        test_reloc(
            (&raw mut result_number).cast::<core::ffi::c_void>(),
            10,
            (&raw const bar).cast::<core::ffi::c_void>(),
        );
    }

    TC_ACT_OK
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [i8; 4] = [b'G' as i8, b'P' as i8, b'L' as i8, 0];
