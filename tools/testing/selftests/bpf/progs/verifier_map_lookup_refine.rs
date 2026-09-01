// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// <vmlinux.h>, <bpf/bpf_helpers.h>, "bpf_misc.h", "bpf_kfuncs.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u32 = u32;

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcphdr {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn bpf_dynptr_from_skb(skb: *mut __sk_buff, flags: __u32, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_slice(
        ptr: *const bpf_dynptr,
        offset: __u32,
        buffer: *mut core::ffi::c_void,
        buffer__sz: __u32,
    ) -> *mut i8;
    fn bpf_csum_diff(
        from: *mut core::ffi::c_void,
        from_size: __u32,
        to: *mut core::ffi::c_void,
        to_size: __u32,
        seed: __u32,
    ) -> i64;
    fn bpf_tcp_raw_check_syncookie_ipv4(
        iph: *mut core::ffi::c_void,
        th: *mut core::ffi::c_void,
    ) -> i64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct inner_map {
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __uint(max_entries, 1);
    // __type(key, int);
    // __type(value, int);
    _private: [u8; 0],
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut inner_map: inner_map = inner_map { _private: [] };

#[repr(C)]
pub struct outer_map {
    // __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
    // __uint(max_entries, 1);
    // __type(key, int);
    // __array(values, struct inner_map);
    pub values: [*mut inner_map; 1],
}

unsafe impl Sync for outer_map {}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut outer_map: outer_map = outer_map {
    values: [core::ptr::addr_of_mut!(inner_map)],
};

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
// __failure __msg("type=map_ptr_or_null expected=fp")
pub unsafe extern "C" fn mapofmaps_value_as_kfunc_mem_buf(skb: *mut __sk_buff) -> i32 {
    let mut dptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let key: __u32 = 0;
    let inner: *mut core::ffi::c_void;
    let p: *mut i8;

    inner = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(outer_map).cast::<core::ffi::c_void>(),
            (&key as *const __u32).cast::<core::ffi::c_void>(),
        )
    };
    /* intentionally NOT NULL-checked: type is map_ptr_or_null */

    unsafe {
        bpf_dynptr_from_skb(skb, 0, dptr.as_mut_ptr());
    }
    /* arg3 is mem+size */
    p = unsafe { bpf_dynptr_slice(dptr.as_ptr(), 0, inner, 4) };
    if !p.is_null() {
        return unsafe { *p.offset(0) as i32 };
    }
    0
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
// __failure __msg("type=map_ptr_or_null expected=fp")
pub unsafe extern "C" fn mapofmaps_value_as_helper_mem_buf(skb: *mut __sk_buff) -> i32 {
    let key: __u32 = 0;
    let inner: *mut core::ffi::c_void;

    inner = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(outer_map).cast::<core::ffi::c_void>(),
            (&key as *const __u32).cast::<core::ffi::c_void>(),
        )
    };
    /* intentionally NOT NULL-checked: type is map_ptr_or_null */

    /* arg1 is mem+size */
    unsafe { (bpf_csum_diff(inner, 4, core::ptr::null_mut(), 0, 0) + (*skb).len as i64) as i32 }
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
// __failure __msg("type=map_ptr_or_null expected=fp")
pub unsafe extern "C" fn mapofmaps_value_as_helper_fixed_mem(skb: *mut __sk_buff) -> i32 {
    let mut th: [u8; core::mem::size_of::<tcphdr>()] = [0; core::mem::size_of::<tcphdr>()];
    let key: __u32 = 0;
    let inner: *mut core::ffi::c_void;

    inner = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(outer_map).cast::<core::ffi::c_void>(),
            (&key as *const __u32).cast::<core::ffi::c_void>(),
        )
    };
    /* intentionally NOT NULL-checked: type is map_ptr_or_null */

    /* arg1 is fixed-sized mem */
    let _ = skb;
    unsafe { bpf_tcp_raw_check_syncookie_ipv4(inner, th.as_mut_ptr().cast::<core::ffi::c_void>()) as i32 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
