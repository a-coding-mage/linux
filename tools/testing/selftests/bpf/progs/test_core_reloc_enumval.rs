// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies:
// #include <linux/bpf.h>
// #include <stdint.h>
// #include <stdbool.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_core_read.h>

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct Data {
    pub in_: [i8; 256],
    pub out: [i8; 256],
    pub skip: bool,
}

#[no_mangle]
pub static mut data: Data = Data {
    in_: [0; 256],
    out: [0; 256],
    skip: false,
};

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum named_enum {
    NAMED_ENUM_VAL1 = 1,
    NAMED_ENUM_VAL2 = 2,
    NAMED_ENUM_VAL3 = 3,
}

pub type anon_enum = i32;

pub const ANON_ENUM_VAL1: anon_enum = 0x10;
pub const ANON_ENUM_VAL2: anon_enum = 0x20;
pub const ANON_ENUM_VAL3: anon_enum = 0x30;

#[repr(C)]
pub struct core_reloc_enumval_output {
    pub named_val1_exists: bool,
    pub named_val2_exists: bool,
    pub named_val3_exists: bool,
    pub anon_val1_exists: bool,
    pub anon_val2_exists: bool,
    pub anon_val3_exists: bool,

    pub named_val1: i32,
    pub named_val2: i32,
    pub anon_val1: i32,
    pub anon_val2: i32,
}

// Original C section annotation: SEC("raw_tracepoint/sys_enter")
#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn test_core_enumval(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    // Original C condition:
    // #if __has_builtin(__builtin_preserve_enum_value)
    #[cfg(has_builtin_preserve_enum_value)]
    {
        let out: *mut core_reloc_enumval_output =
            (&mut data.out as *mut [i8; 256]).cast::<core_reloc_enumval_output>();
        let named: named_enum = core::mem::transmute(0i32);
        let anon: anon_enum = 0;

        (*out).named_val1_exists = bpf_core_enum_value_exists!(named, named_enum::NAMED_ENUM_VAL1);
        (*out).named_val2_exists =
            bpf_core_enum_value_exists!(named_enum, named_enum::NAMED_ENUM_VAL2);
        (*out).named_val3_exists =
            bpf_core_enum_value_exists!(named_enum, named_enum::NAMED_ENUM_VAL3);

        (*out).anon_val1_exists = bpf_core_enum_value_exists!(anon, ANON_ENUM_VAL1);
        (*out).anon_val2_exists = bpf_core_enum_value_exists!(anon_enum, ANON_ENUM_VAL2);
        (*out).anon_val3_exists = bpf_core_enum_value_exists!(anon_enum, ANON_ENUM_VAL3);

        (*out).named_val1 = bpf_core_enum_value!(named, named_enum::NAMED_ENUM_VAL1);
        (*out).named_val2 = bpf_core_enum_value!(named, named_enum::NAMED_ENUM_VAL2);
        /* NAMED_ENUM_VAL3 value is optional */

        (*out).anon_val1 = bpf_core_enum_value!(anon, ANON_ENUM_VAL1);
        (*out).anon_val2 = bpf_core_enum_value!(anon, ANON_ENUM_VAL2);
        /* ANON_ENUM_VAL3 value is optional */
    }

    // Original C fallback:
    // #else
    #[cfg(not(has_builtin_preserve_enum_value))]
    {
        data.skip = true;
    }
    // #endif

    0
}
