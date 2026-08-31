// SPDX-License-Identifier: GPL-2.0

// C dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut test_1_result: ::core::ffi::c_int = 0;

// SEC("?struct_ops/test_1")
// int BPF_PROG(foo)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn foo() -> ::core::ffi::c_int {
    unsafe {
        test_1_result = 42;
    }
    0
}

// SEC("?struct_ops/test_1")
// int BPF_PROG(bar)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bar() -> ::core::ffi::c_int {
    unsafe {
        test_1_result = 24;
    }
    0
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_1: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod_1: bpf_testmod_ops = bpf_testmod_ops {
    test_1: Some(bar),
};
