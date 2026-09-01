// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook

// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

#[repr(C)]
pub struct ipv_counts {
    pub v4: ::core::ffi::c_uint,
    pub v6: ::core::ffi::c_uint,
}

// Original C declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(key_size, sizeof(int));
//     __uint(value_size, sizeof(struct ipv_counts));
//     __uint(max_entries, 4);
// } btf_map SEC(".maps");
//
// The BPF map-definition macros and section attribute are supplied by external
// BPF support code in the original build.
unsafe extern "C" {
    static mut btf_map: ::core::ffi::c_void;

    fn bpf_map_lookup_elem(
        map: *mut ::core::ffi::c_void,
        key: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn test_long_fname_2() -> ::core::ffi::c_int {
    let counts: *mut ipv_counts;
    let key: ::core::ffi::c_int = 0;

    counts = unsafe {
        bpf_map_lookup_elem(
            ::core::ptr::addr_of_mut!(btf_map),
            ::core::ptr::addr_of!(key) as *const ::core::ffi::c_void,
        ) as *mut ipv_counts
    };
    if counts.is_null() {
        return 0;
    }

    unsafe {
        (*counts).v6 = (*counts).v6.wrapping_add(1);
    }

    0
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn test_long_fname_1() -> ::core::ffi::c_int {
    unsafe { test_long_fname_2() }
}

// Original C section attribute: SEC("dummy_tracepoint")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _dummy_tracepoint(arg: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let _ = arg;
    unsafe { test_long_fname_1() }
}

// Original C declaration: char _license[] SEC("license") = "GPL";
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [b'G' as ::core::ffi::c_char, b'P' as ::core::ffi::c_char, b'L' as ::core::ffi::c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
