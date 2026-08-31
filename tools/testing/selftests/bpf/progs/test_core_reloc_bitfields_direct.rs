// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// C dependencies removed from executable Rust:
// <linux/bpf.h>, <stdint.h>, <bpf/bpf_helpers.h>, <bpf/bpf_core_read.h>
// SEC(...) section placement is translated with link_section attributes where
// directly applicable. BPF_CORE_READ_BITFIELD is preserved as an external macro
// dependency expected from the surrounding BPF Rust environment.

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

#[repr(C)]
pub struct Data {
    pub in_: [::core::ffi::c_char; 256],
    pub out: [::core::ffi::c_char; 256],
}

#[no_mangle]
pub static mut data: Data = Data {
    in_: [0; 256],
    out: [0; 256],
};

#[repr(C)]
pub struct core_reloc_bitfields {
    /* unsigned bitfields */
    // uint8_t ub1: 1;
    // uint8_t ub2: 2;
    // uint32_t ub7: 7;
    /* signed bitfields */
    // int8_t sb4: 4;
    // int32_t sb20: 20;
    /* non-bitfields */
    pub u32: u32,
    pub s32: i32,
}

/* bitfield read results, all as plain integers */
#[repr(C)]
pub struct core_reloc_bitfields_output {
    pub ub1: i64,
    pub ub2: i64,
    pub ub7: i64,
    pub sb4: i64,
    pub sb20: i64,
    pub u32: i64,
    pub s32: i64,
}

#[repr(C)]
pub struct pt_regs {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct trace_sys_enter {
    pub regs: *mut pt_regs,
    pub id: ::core::ffi::c_long,
}

#[no_mangle]
#[link_section = "tp_btf/sys_enter"]
pub unsafe extern "C" fn test_core_bitfields_direct(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let in_: *mut core_reloc_bitfields = (&raw mut data.in_).cast::<core_reloc_bitfields>();
    let out: *mut core_reloc_bitfields_output = (&raw mut data.out).cast::<core_reloc_bitfields_output>();

    (*out).ub1 = BPF_CORE_READ_BITFIELD!(in_, ub1);
    (*out).ub2 = BPF_CORE_READ_BITFIELD!(in_, ub2);
    (*out).ub7 = BPF_CORE_READ_BITFIELD!(in_, ub7);
    (*out).sb4 = BPF_CORE_READ_BITFIELD!(in_, sb4);
    (*out).sb20 = BPF_CORE_READ_BITFIELD!(in_, sb20);
    (*out).u32 = BPF_CORE_READ_BITFIELD!(in_, u32);
    (*out).s32 = BPF_CORE_READ_BITFIELD!(in_, s32);

    let _ = ctx;
    0
}
