// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <linux/bpf.h>
// #include <stdint.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_core_read.h>
// #include "bpf_misc.h"

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct core_reloc_bitfields {
    /*
     * Original C layout uses preserve_access_index bitfields:
     *
     * unsigned bitfields:
     * uint8_t  ub1: 1;
     * uint8_t  ub2: 2;
     * uint32_t ub7: 7;
     * signed bitfields:
     * int8_t   sb4: 4;
     * int32_t  sb20: 20;
     * non-bitfields:
     * uint32_t u32;
     * int32_t  s32;
     *
     * Rust has no native C-compatible bitfield syntax. The CO-RE bitfield
     * names are intentionally preserved at macro call sites below for the
     * future BPF support dependency that supplies BPF_CORE_* equivalents.
     */
    pub u32: u32,
    pub s32: i32,
}

// SEC("tc")
// __description("single CO-RE bitfield roundtrip")
// __btf_path("btf__core_reloc_bitfields.bpf.o")
// __success
// __retval(3)
#[no_mangle]
pub unsafe extern "C" fn single_field_roundtrip(ctx: *mut __sk_buff) -> i32 {
    let mut bitfields: core_reloc_bitfields = core::mem::zeroed();

    let _ = ctx;
    BPF_CORE_WRITE_BITFIELD!(&mut bitfields, ub2, 3);
    return BPF_CORE_READ_BITFIELD!(&bitfields, ub2) as i32;
}

// SEC("tc")
// __description("multiple CO-RE bitfield roundtrip")
// __btf_path("btf__core_reloc_bitfields.bpf.o")
// __success
// __retval(0x3FD)
#[no_mangle]
pub unsafe extern "C" fn multiple_field_roundtrip(ctx: *mut __sk_buff) -> i32 {
    let mut bitfields: core_reloc_bitfields = core::mem::zeroed();
    let mut ub2: u8;
    let mut sb4: i8;

    let _ = ctx;
    BPF_CORE_WRITE_BITFIELD!(&mut bitfields, ub2, 1);
    BPF_CORE_WRITE_BITFIELD!(&mut bitfields, sb4, -1);

    ub2 = BPF_CORE_READ_BITFIELD!(&bitfields, ub2) as u8;
    sb4 = BPF_CORE_READ_BITFIELD!(&bitfields, sb4) as i8;

    return (((sb4 as u8) as i32) << 2) | (ub2 as i32);
}

// SEC("tc")
// __description("adjacent CO-RE bitfield roundtrip")
// __btf_path("btf__core_reloc_bitfields.bpf.o")
// __success
// __retval(7)
#[no_mangle]
pub unsafe extern "C" fn adjacent_field_roundtrip(ctx: *mut __sk_buff) -> i32 {
    let mut bitfields: core_reloc_bitfields = core::mem::zeroed();
    let mut ub1: u8;
    let mut ub2: u8;

    let _ = ctx;
    BPF_CORE_WRITE_BITFIELD!(&mut bitfields, ub1, 1);
    BPF_CORE_WRITE_BITFIELD!(&mut bitfields, ub2, 3);

    ub1 = BPF_CORE_READ_BITFIELD!(&bitfields, ub1) as u8;
    ub2 = BPF_CORE_READ_BITFIELD!(&bitfields, ub2) as u8;

    return ((ub2 as i32) << 1) | (ub1 as i32);
}

// SEC("tc")
// __description("multibyte CO-RE bitfield roundtrip")
// __btf_path("btf__core_reloc_bitfields.bpf.o")
// __success
// __retval(0x21)
#[no_mangle]
pub unsafe extern "C" fn multibyte_field_roundtrip(ctx: *mut __sk_buff) -> i32 {
    let mut bitfields: core_reloc_bitfields = core::mem::zeroed();
    let mut ub7: u32;
    let mut ub1: u8;

    let _ = ctx;
    BPF_CORE_WRITE_BITFIELD!(&mut bitfields, ub1, 1);
    BPF_CORE_WRITE_BITFIELD!(&mut bitfields, ub7, 16);

    ub1 = BPF_CORE_READ_BITFIELD!(&bitfields, ub1) as u8;
    ub7 = BPF_CORE_READ_BITFIELD!(&bitfields, ub7) as u32;

    return ((ub7 as i32) << 1) | (ub1 as i32);
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
