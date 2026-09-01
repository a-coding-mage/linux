// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

unsafe extern "C" {
    fn PT_REGS_RC(ctx: *mut pt_regs) -> __u64;
}

#[allow(non_camel_case_types)]
pub type __u64 = u64;

#[allow(non_camel_case_types)]
pub enum pt_regs {}

#[link_section = "raw_tracepoint/consume_skb"]
#[no_mangle]
pub unsafe extern "C" fn while_true(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    let mut i: __u64 = 0;
    let mut sum: __u64 = 0;

    loop {
        ::core::ptr::write_volatile(&mut i, ::core::ptr::read_volatile(&i).wrapping_add(1));
        ::core::ptr::write_volatile(
            &mut sum,
            ::core::ptr::read_volatile(&sum).wrapping_add(PT_REGS_RC(ctx)),
        );
        if !(::core::ptr::read_volatile(&i) < 0x100000000u64) {
            break;
        }
    }

    ::core::ptr::read_volatile(&sum) as ::core::ffi::c_int
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
