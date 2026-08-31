// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// C dependencies: <linux/bpf.h>, <stdint.h>, <bpf/bpf_helpers.h>,
// and <bpf/bpf_core_read.h>.

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

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
    /*
     * unsigned bitfields
     *
     * C source fields:
     * uint8_t      ub1: 1;
     * uint8_t      ub2: 2;
     * uint32_t     ub7: 7;
     *
     * signed bitfields
     *
     * int8_t       sb4: 4;
     * int32_t      sb20: 20;
     *
     * Rust has no native C-compatible bitfield declarations. The field layout
     * is intentionally left to the external BPF_CORE_READ_BITFIELD_PROBED
     * equivalent used below.
     */

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

#[link_section = "raw_tracepoint/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn test_core_bitfields(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let in_: *mut core_reloc_bitfields = (&raw mut data.in_) as *mut _ as *mut core_reloc_bitfields;
    let out: *mut core_reloc_bitfields_output =
        (&raw mut data.out) as *mut _ as *mut core_reloc_bitfields_output;

    (*out).ub1 = BPF_CORE_READ_BITFIELD_PROBED!(in_, ub1);
    (*out).ub2 = BPF_CORE_READ_BITFIELD_PROBED!(in_, ub2);
    (*out).ub7 = BPF_CORE_READ_BITFIELD_PROBED!(in_, ub7);
    (*out).sb4 = BPF_CORE_READ_BITFIELD_PROBED!(in_, sb4);
    (*out).sb20 = BPF_CORE_READ_BITFIELD_PROBED!(in_, sb20);
    (*out).u32 = BPF_CORE_READ_BITFIELD_PROBED!(in_, u32);
    (*out).s32 = BPF_CORE_READ_BITFIELD_PROBED!(in_, s32);

    let _ = ctx;

    0
}
