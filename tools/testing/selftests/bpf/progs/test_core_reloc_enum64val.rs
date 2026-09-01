// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// Dependencies from the original C file:
// <linux/bpf.h>, <stdint.h>, <stdbool.h>, <bpf/bpf_helpers.h>,
// and <bpf/bpf_core_read.h>.

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct Data {
    pub in_: [::core::ffi::c_char; 256],
    pub out: [::core::ffi::c_char; 256],
    pub skip: bool,
}

#[unsafe(no_mangle)]
pub static mut data: Data = Data {
    in_: [0; 256],
    out: [0; 256],
    skip: false,
};

#[repr(u64)]
pub enum named_unsigned_enum64 {
    UNSIGNED_ENUM64_VAL1 = 0x1ffffffffu64,
    UNSIGNED_ENUM64_VAL2 = 0x2ffffffffu64,
    UNSIGNED_ENUM64_VAL3 = 0x3ffffffffu64,
}

#[repr(i64)]
pub enum named_signed_enum64 {
    SIGNED_ENUM64_VAL1 = 0x1ffffffffi64,
    SIGNED_ENUM64_VAL2 = -2,
    SIGNED_ENUM64_VAL3 = 0x3ffffffffi64,
}

#[repr(C)]
pub struct core_reloc_enum64val_output {
    pub unsigned_val1_exists: bool,
    pub unsigned_val2_exists: bool,
    pub unsigned_val3_exists: bool,
    pub signed_val1_exists: bool,
    pub signed_val2_exists: bool,
    pub signed_val3_exists: bool,

    pub unsigned_val1: ::core::ffi::c_long,
    pub unsigned_val2: ::core::ffi::c_long,
    pub signed_val1: ::core::ffi::c_long,
    pub signed_val2: ::core::ffi::c_long,
}

unsafe extern "C" {
    // Rust has no direct file-local equivalent for the BPF CO-RE enum macros'
    // ability to accept either an enum value expression or an enum type name.
    fn bpf_core_enum_value_exists_named_unsigned_enum64(
        value: named_unsigned_enum64,
    ) -> bool;
    fn bpf_core_enum_value_exists_named_signed_enum64(value: named_signed_enum64) -> bool;
    fn bpf_core_enum_value_named_unsigned_enum64(
        value: named_unsigned_enum64,
    ) -> ::core::ffi::c_long;
    fn bpf_core_enum_value_named_signed_enum64(value: named_signed_enum64) -> ::core::ffi::c_long;
}

#[unsafe(link_section = "raw_tracepoint/sys_enter")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_core_enum64val(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let _ = ctx;

    // Original C condition: #if __clang_major__ >= 15
    #[cfg(__clang_major_greater_or_equal_15)]
    {
        let out: *mut core_reloc_enum64val_output =
            (&raw mut data.out).cast::<core_reloc_enum64val_output>();
        let named_unsigned: named_unsigned_enum64 =
            ::core::mem::transmute::<u64, named_unsigned_enum64>(0);
        let named_signed: named_signed_enum64 =
            ::core::mem::transmute::<i64, named_signed_enum64>(0);

        (*out).unsigned_val1_exists =
            bpf_core_enum_value_exists_named_unsigned_enum64(named_unsigned_enum64::UNSIGNED_ENUM64_VAL1);
        (*out).unsigned_val2_exists =
            bpf_core_enum_value_exists_named_unsigned_enum64(named_unsigned_enum64::UNSIGNED_ENUM64_VAL2);
        (*out).unsigned_val3_exists =
            bpf_core_enum_value_exists_named_unsigned_enum64(named_unsigned_enum64::UNSIGNED_ENUM64_VAL3);
        (*out).signed_val1_exists =
            bpf_core_enum_value_exists_named_signed_enum64(named_signed_enum64::SIGNED_ENUM64_VAL1);
        (*out).signed_val2_exists =
            bpf_core_enum_value_exists_named_signed_enum64(named_signed_enum64::SIGNED_ENUM64_VAL2);
        (*out).signed_val3_exists =
            bpf_core_enum_value_exists_named_signed_enum64(named_signed_enum64::SIGNED_ENUM64_VAL3);

        (*out).unsigned_val1 =
            bpf_core_enum_value_named_unsigned_enum64(named_unsigned_enum64::UNSIGNED_ENUM64_VAL1);
        (*out).unsigned_val2 =
            bpf_core_enum_value_named_unsigned_enum64(named_unsigned_enum64::UNSIGNED_ENUM64_VAL2);
        (*out).signed_val1 =
            bpf_core_enum_value_named_signed_enum64(named_signed_enum64::SIGNED_ENUM64_VAL1);
        (*out).signed_val2 =
            bpf_core_enum_value_named_signed_enum64(named_signed_enum64::SIGNED_ENUM64_VAL2);
        /* NAMED_ENUM64_VAL3 value is optional */

        let _ = named_unsigned;
        let _ = named_signed;
    }

    // Original C condition: #else for __clang_major__ < 15
    #[cfg(not(__clang_major_greater_or_equal_15))]
    {
        data.skip = true;
    }

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
