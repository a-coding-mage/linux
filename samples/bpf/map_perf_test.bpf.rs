/* Copyright (c) 2016 Facebook
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// Dependencies supplied by vmlinux.h, bpf_helpers.h, bpf_tracing.h,
// bpf_core_read.h, errno.h, and linux/version.h are intentionally external.

const MAX_ENTRIES: u32 = 1000;
const MAX_NR_CPUS: u32 = 1024;

// BPF map declarations; __uint/__type/__array and SEC(".maps") are supplied
// by the BPF build environment.
#[repr(C)]
pub struct hash_map_t;
#[repr(C)]
pub struct lru_hash_map_t;
#[repr(C)]
pub struct nocommon_lru_hash_map_t;
#[repr(C)]
pub struct inner_lru;
#[repr(C)]
pub struct array_of_lru_hashs_t;
#[repr(C)]
pub struct percpu_hash_map_t;
#[repr(C)]
pub struct hash_map_alloc_t;
#[repr(C)]
pub struct percpu_hash_map_alloc_t;
#[repr(C)]
pub struct lpm_trie_map_alloc_t;
#[repr(C)]
pub struct array_map_t;
#[repr(C)]
pub struct lru_hash_lookup_map_t;

extern "C" {
    static mut hash_map: hash_map_t;
    static mut lru_hash_map: lru_hash_map_t;
    static mut nocommon_lru_hash_map: nocommon_lru_hash_map_t;
    static mut inner_lru_hash_map: inner_lru;
    static mut array_of_lru_hashs: array_of_lru_hashs_t;
    static mut percpu_hash_map: percpu_hash_map_t;
    static mut hash_map_alloc: hash_map_alloc_t;
    static mut percpu_hash_map_alloc: percpu_hash_map_alloc_t;
    static mut lpm_trie_map_alloc: lpm_trie_map_alloc_t;
    static mut array_map: array_map_t;
    static mut lru_hash_lookup_map: lru_hash_lookup_map_t;

    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_map_update_elem(map: *const core::ffi::c_void, key: *const core::ffi::c_void, value: *const core::ffi::c_void, flags: u64) -> i64;
    fn bpf_map_lookup_elem(map: *const core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_map_delete_elem(map: *const core::ffi::c_void, key: *const core::ffi::c_void) -> i64;
    fn bpf_probe_read_user(dst: *mut core::ffi::c_void, size: u32, src: *const core::ffi::c_void) -> i64;
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_get_smp_processor_id() -> u32;
    fn bpf_trace_printk(fmt: *const core::ffi::c_void, size: u32, arg: i32) -> i64;
}

const BPF_ANY: u64 = 0;
const BPF_F_NO_COMMON_LRU: u64 = 0;
const BPF_F_NUMA_NODE: u64 = 0;
const BPF_F_NO_PREALLOC: u64 = 0;
const ENOENT: i32 = 2;
const EINVAL: i32 = 22;

#[repr(C)]
pub union test_params_t {
    pub dst6: [u16; 8],
    pub fields: test_params_fields,
}
#[repr(C)]
pub struct test_params_fields {
    pub magic0: u16,
    pub magic1: u16,
    pub tcase: u16,
    pub unused16: u16,
    pub unused32: u32,
    pub key: u32,
}

#[repr(C)]
pub struct sockaddr_in;
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_addr: [u8; 16],
}

#[no_mangle]
pub unsafe extern "C" fn stress_hmap() -> i32 {
    let key: u32 = bpf_get_current_pid_tgid() as u32;
    let init_val: i64 = 1;
    for _i in 0..10 {
        bpf_map_update_elem((&raw const hash_map).cast(), (&raw const key).cast(), (&raw const init_val).cast(), BPF_ANY);
        let value = bpf_map_lookup_elem((&raw const hash_map).cast(), (&raw const key).cast());
        if !value.is_null() { bpf_map_delete_elem((&raw const hash_map).cast(), (&raw const key).cast()); }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn stress_percpu_hmap() -> i32 {
    let key: u32 = bpf_get_current_pid_tgid() as u32;
    let init_val: i64 = 1;
    for _i in 0..10 {
        bpf_map_update_elem((&raw const percpu_hash_map).cast(), (&raw const key).cast(), (&raw const init_val).cast(), BPF_ANY);
        let value = bpf_map_lookup_elem((&raw const percpu_hash_map).cast(), (&raw const key).cast());
        if !value.is_null() { bpf_map_delete_elem((&raw const percpu_hash_map).cast(), (&raw const key).cast()); }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn stress_hmap_alloc() -> i32 {
    let key: u32 = bpf_get_current_pid_tgid() as u32;
    let init_val: i64 = 1;
    for _i in 0..10 {
        bpf_map_update_elem((&raw const hash_map_alloc).cast(), (&raw const key).cast(), (&raw const init_val).cast(), BPF_ANY);
        let value = bpf_map_lookup_elem((&raw const hash_map_alloc).cast(), (&raw const key).cast());
        if !value.is_null() { bpf_map_delete_elem((&raw const hash_map_alloc).cast(), (&raw const key).cast()); }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn stress_percpu_hmap_alloc() -> i32 {
    let key: u32 = bpf_get_current_pid_tgid() as u32;
    let init_val: i64 = 1;
    for _i in 0..10 {
        bpf_map_update_elem((&raw const percpu_hash_map_alloc).cast(), (&raw const key).cast(), (&raw const init_val).cast(), BPF_ANY);
        let value = bpf_map_lookup_elem((&raw const percpu_hash_map_alloc).cast(), (&raw const key).cast());
        if !value.is_null() { bpf_map_delete_elem((&raw const percpu_hash_map_alloc).cast(), (&raw const key).cast()); }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn stress_lru_hmap_alloc(fd: i32, uservaddr: *mut sockaddr_in, addrlen: i32) -> i32 {
    let fmt = *b"Failed at stress_lru_hmap_alloc. ret:%dn\0";
    let mut test_params = test_params_t { dst6: [0; 8] };
    let in6 = uservaddr as *mut sockaddr_in6;
    let mut test_case: u16;
    let val: i64 = 1;
    let mut key: u32 = 0;
    let mut ret: i32 = 0;
    if addrlen != core::mem::size_of::<sockaddr_in6>() as i32 { return 0; }
    ret = bpf_probe_read_user((&mut test_params as *mut test_params_t).cast(), 16, (&(*in6).sin6_addr as *const [u8; 16]).cast());
    if ret != 0 { return done_lru(&fmt, ret); }
    if test_params.fields.magic0 != 0xdead || test_params.fields.magic1 != 0xbeef { return 0; }
    test_case = test_params.fields.tcase;
    if test_case != 3 { key = bpf_get_prandom_u32(); }
    if test_case == 0 {
        ret = bpf_map_update_elem((&raw const lru_hash_map).cast(), (&raw const key).cast(), (&raw const val).cast(), BPF_ANY) as i32;
    } else if test_case == 1 {
        ret = bpf_map_update_elem((&raw const nocommon_lru_hash_map).cast(), (&raw const key).cast(), (&raw const val).cast(), BPF_ANY) as i32;
    } else if test_case == 2 {
        let cpu = bpf_get_smp_processor_id();
        let nolocal_lru_map = bpf_map_lookup_elem((&raw const array_of_lru_hashs).cast(), (&raw const cpu).cast());
        if nolocal_lru_map.is_null() { ret = -ENOENT; return done_lru(&fmt, ret); }
        ret = bpf_map_update_elem(nolocal_lru_map, (&raw const key).cast(), (&raw const val).cast(), BPF_ANY) as i32;
    } else if test_case == 3 {
        key = test_params.fields.key;
        for _i in 0..32 { bpf_map_lookup_elem((&raw const lru_hash_lookup_map).cast(), (&raw const key).cast()); key = key.wrapping_add(1); }
    } else { ret = -EINVAL; }
    done_lru(&fmt, ret)
}

unsafe fn done_lru(fmt: &[u8], ret: i32) -> i32 {
    if ret != 0 { bpf_trace_printk(fmt.as_ptr().cast(), fmt.len() as u32, ret); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn stress_lpm_trie_map_alloc() -> i32 {
    let mut key = [0u8; 8];
    key[0..4].copy_from_slice(&32u32.to_ne_bytes());
    key[4] = 192; key[5] = 168; key[6] = 0; key[7] = 1;
    for _i in 0..32 { bpf_map_lookup_elem((&raw const lpm_trie_map_alloc).cast(), key.as_ptr().cast()); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn stress_hash_map_lookup() -> i32 {
    let key: u32 = 1;
    for _i in 0..64 { bpf_map_lookup_elem((&raw const hash_map).cast(), (&raw const key).cast()); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn stress_array_map_lookup() -> i32 {
    let key: u32 = 1;
    for _i in 0..64 { bpf_map_lookup_elem((&raw const array_map).cast(), (&raw const key).cast()); }
    0
}

#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
#[no_mangle]
pub static mut _version: u32 = 0; // LINUX_VERSION_CODE, supplied at build time.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
