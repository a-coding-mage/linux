// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>

extern "C" {
    // extern int bpf_testmod_ksym_percpu __ksym;
    static mut bpf_testmod_ksym_percpu: ::core::ffi::c_int;

    fn bpf_this_cpu_ptr(percpu_ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn ksym_fail(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    *(bpf_this_cpu_ptr(
        &bpf_testmod_ksym_percpu as *const ::core::ffi::c_int as *const ::core::ffi::c_void,
    ) as *mut ::core::ffi::c_int)
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
