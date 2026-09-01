// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependency intent:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, <bpf/usdt.bpf.h>, "bpf_misc.h"

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "kprobe.session"]
// __success
pub unsafe extern "C" fn kprobe_session_return_0(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    0
}

#[no_mangle]
#[link_section = "kprobe.session"]
// __success
pub unsafe extern "C" fn kprobe_session_return_1(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    1
}

#[no_mangle]
#[link_section = "kprobe.session"]
// __failure
// __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
pub unsafe extern "C" fn kprobe_session_return_2(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    2
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
