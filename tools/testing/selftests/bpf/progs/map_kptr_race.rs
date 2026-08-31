// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies translated as external Rust dependencies:
// <vmlinux.h>
// <bpf/bpf_helpers.h>
// <bpf/bpf_tracing.h>
// "../test_kmods/bpf_testmod_kfunc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type c_int = i32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type c_void = core::ffi::c_void;
pub type u32 = u32;

pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_MAP_TYPE_PERCPU_HASH: u32 = 5;
pub const BPF_MAP_TYPE_SK_STORAGE: u32 = 24;
pub const BPF_F_NO_PREALLOC: u32 = 1;
pub const BPF_ANY: u64 = 0;
pub const BPF_SK_STORAGE_GET_F_CREATE: u64 = 1;
pub const BPF_TCP_SYN_SENT: c_int = 2;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    pub id: u32,
}

#[repr(C)]
pub struct counter_t {
    pub counter: c_int,
}

#[repr(C)]
pub struct refs_t {
    pub refs: counter_t,
}

#[repr(C)]
pub struct prog_test_ref_kfunc {
    pub cnt: refs_t,
}

#[repr(C)]
pub struct map_value {
    pub ref_ptr: *mut prog_test_ref_kfunc,
}

#[repr(C)]
pub struct bpf_map_def_map_value {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

#[repr(C)]
pub struct bpf_map_def_map_value_no_max_entries {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

// Original C declaration used libbpf map-definition macros and SEC(".maps").
#[no_mangle]
#[link_section = ".maps"]
pub static mut race_hash_map: bpf_map_def_map_value = bpf_map_def_map_value {
    type_: BPF_MAP_TYPE_HASH,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    max_entries: 1,
};

// Original C declaration used libbpf map-definition macros and SEC(".maps").
#[no_mangle]
#[link_section = ".maps"]
pub static mut race_percpu_hash_map: bpf_map_def_map_value = bpf_map_def_map_value {
    type_: BPF_MAP_TYPE_PERCPU_HASH,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<map_value>() as u32,
    max_entries: 1,
};

// Original C declaration used libbpf map-definition macros and SEC(".maps").
#[no_mangle]
#[link_section = ".maps"]
pub static mut race_sk_ls_map: bpf_map_def_map_value_no_max_entries =
    bpf_map_def_map_value_no_max_entries {
        type_: BPF_MAP_TYPE_SK_STORAGE,
        map_flags: BPF_F_NO_PREALLOC,
        key_size: core::mem::size_of::<c_int>() as u32,
        value_size: core::mem::size_of::<map_value>() as u32,
    };

#[no_mangle]
pub static mut num_of_refs: c_int = 0;
#[no_mangle]
pub static mut sk_ls_leak_done: c_int = 0;
#[no_mangle]
pub static mut target_map_id: c_int = 0;
#[no_mangle]
pub static mut map_freed: c_int = 0;
#[no_mangle]
pub static mut nr_cpus: c_int = 0;

extern "C" {
    pub fn bpf_map_update_elem(
        map: *mut c_void,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_long;
    pub fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    pub fn bpf_map_lookup_percpu_elem(
        map: *mut c_void,
        key: *const c_void,
        cpu: u32,
    ) -> *mut c_void;
    pub fn bpf_map_delete_elem(map: *mut c_void, key: *const c_void) -> c_long;
    pub fn bpf_sk_storage_get(
        map: *mut c_void,
        sk: *mut sock,
        value: *mut c_void,
        flags: u64,
    ) -> *mut c_void;
    pub fn bpf_sk_storage_delete(map: *mut c_void, sk: *mut sock) -> c_long;
    pub fn bpf_kfunc_call_test_acquire(arg: *mut c_ulong) -> *mut prog_test_ref_kfunc;
    pub fn bpf_kfunc_call_test_release(p: *mut prog_test_ref_kfunc);
    pub fn bpf_kptr_xchg(
        kptr: *mut *mut prog_test_ref_kfunc,
        ptr: *mut prog_test_ref_kfunc,
    ) -> *mut prog_test_ref_kfunc;
}

// SEC("tc")
#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn test_htab_leak(_skb: *mut __sk_buff) -> c_int {
    let mut p: *mut prog_test_ref_kfunc;
    let mut old: *mut prog_test_ref_kfunc;
    let val: map_value = map_value {
        ref_ptr: core::ptr::null_mut(),
    };
    let mut v: *mut map_value;
    let key: c_int = 0;

    if bpf_map_update_elem(
        &raw mut race_hash_map as *mut c_void,
        &key as *const _ as *const c_void,
        &val as *const _ as *const c_void,
        BPF_ANY,
    ) != 0
    {
        return 1;
    }

    v = bpf_map_lookup_elem(
        &raw mut race_hash_map as *mut c_void,
        &key as *const _ as *const c_void,
    ) as *mut map_value;
    if v.is_null() {
        return 2;
    }

    p = bpf_kfunc_call_test_acquire(&mut 0 as *mut c_ulong);
    if p.is_null() {
        return 3;
    }
    old = bpf_kptr_xchg(&raw mut (*v).ref_ptr, p);
    if !old.is_null() {
        bpf_kfunc_call_test_release(old);
    }

    bpf_map_delete_elem(
        &raw mut race_hash_map as *mut c_void,
        &key as *const _ as *const c_void,
    );

    p = bpf_kfunc_call_test_acquire(&mut 0 as *mut c_ulong);
    if p.is_null() {
        return 4;
    }
    old = bpf_kptr_xchg(&raw mut (*v).ref_ptr, p);
    if !old.is_null() {
        bpf_kfunc_call_test_release(old);
    }

    0
}

unsafe fn fill_percpu_kptr(v: *mut map_value) -> c_int {
    let p: *mut prog_test_ref_kfunc;
    let old: *mut prog_test_ref_kfunc;

    p = bpf_kfunc_call_test_acquire(&mut 0 as *mut c_ulong);
    if p.is_null() {
        return 1;
    }
    old = bpf_kptr_xchg(&raw mut (*v).ref_ptr, p);
    if !old.is_null() {
        bpf_kfunc_call_test_release(old);
    }
    0
}

// SEC("tc")
#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn test_percpu_htab_leak(_skb: *mut __sk_buff) -> c_int {
    let mut v: *mut map_value;
    let mut arr: [*mut map_value; 16] = [core::ptr::null_mut(); 16];
    let val: map_value = map_value {
        ref_ptr: core::ptr::null_mut(),
    };
    let key: c_int = 0;
    let mut err: c_int = 0;

    if bpf_map_update_elem(
        &raw mut race_percpu_hash_map as *mut c_void,
        &key as *const _ as *const c_void,
        &val as *const _ as *const c_void,
        BPF_ANY,
    ) != 0
    {
        return 1;
    }

    let mut i: c_int = 0;
    while i < nr_cpus {
        v = bpf_map_lookup_percpu_elem(
            &raw mut race_percpu_hash_map as *mut c_void,
            &key as *const _ as *const c_void,
            i as u32,
        ) as *mut map_value;
        if v.is_null() {
            return 2;
        }
        arr[i as usize] = v;
        i += 1;
    }

    bpf_map_delete_elem(
        &raw mut race_percpu_hash_map as *mut c_void,
        &key as *const _ as *const c_void,
    );

    i = 0;
    while i < nr_cpus {
        v = arr[i as usize];
        err = fill_percpu_kptr(v);
        if err != 0 {
            return 3;
        }
        i += 1;
    }

    0
}

// SEC("tp_btf/inet_sock_set_state")
// Original C used BPF_PROG(test_sk_ls_leak, struct sock *sk, int oldstate, int newstate).
#[no_mangle]
#[link_section = "tp_btf/inet_sock_set_state"]
pub unsafe extern "C" fn test_sk_ls_leak(
    sk: *mut sock,
    _oldstate: c_int,
    newstate: c_int,
) -> c_int {
    let mut p: *mut prog_test_ref_kfunc;
    let mut old: *mut prog_test_ref_kfunc;
    let v: *mut map_value;

    if newstate != BPF_TCP_SYN_SENT {
        return 0;
    }

    if sk_ls_leak_done != 0 {
        return 0;
    }

    v = bpf_sk_storage_get(
        &raw mut race_sk_ls_map as *mut c_void,
        sk,
        core::ptr::null_mut(),
        BPF_SK_STORAGE_GET_F_CREATE,
    ) as *mut map_value;
    if v.is_null() {
        return 0;
    }

    p = bpf_kfunc_call_test_acquire(&mut 0 as *mut c_ulong);
    if p.is_null() {
        return 0;
    }
    old = bpf_kptr_xchg(&raw mut (*v).ref_ptr, p);
    if !old.is_null() {
        bpf_kfunc_call_test_release(old);
    }

    bpf_sk_storage_delete(&raw mut race_sk_ls_map as *mut c_void, sk);

    p = bpf_kfunc_call_test_acquire(&mut 0 as *mut c_ulong);
    if p.is_null() {
        return 0;
    }
    old = bpf_kptr_xchg(&raw mut (*v).ref_ptr, p);
    if !old.is_null() {
        bpf_kfunc_call_test_release(old);
    }

    sk_ls_leak_done = 1;
    0
}

#[no_mangle]
pub static mut target_map_ptr: c_long = 0;

// SEC("fentry/bpf_map_put")
// Original C used BPF_PROG(map_put, struct bpf_map *map).
#[no_mangle]
#[link_section = "fentry/bpf_map_put"]
pub unsafe extern "C" fn map_put(map: *mut bpf_map) -> c_int {
    if target_map_id != 0 && (*map).id == target_map_id as u32 {
        target_map_ptr = map as c_long;
    }
    0
}

// SEC("fexit/htab_map_free")
// Original C used BPF_PROG(htab_map_free, struct bpf_map *map).
#[no_mangle]
#[link_section = "fexit/htab_map_free"]
pub unsafe extern "C" fn htab_map_free(map: *mut bpf_map) -> c_int {
    if target_map_ptr != 0 && map as c_long == target_map_ptr {
        map_freed = 1;
    }
    0
}

// SEC("fexit/bpf_sk_storage_map_free")
// Original C used BPF_PROG(sk_map_free, struct bpf_map *map).
#[no_mangle]
#[link_section = "fexit/bpf_sk_storage_map_free"]
pub unsafe extern "C" fn sk_map_free(map: *mut bpf_map) -> c_int {
    if target_map_ptr != 0 && map as c_long == target_map_ptr {
        map_freed = 1;
    }
    0
}

// SEC("syscall")
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn count_ref(_ctx: *mut c_void) -> c_int {
    let p: *mut prog_test_ref_kfunc;
    let mut arg: c_ulong = 0;

    p = bpf_kfunc_call_test_acquire(&mut arg as *mut c_ulong);
    if p.is_null() {
        return 1;
    }

    num_of_refs = (*p).cnt.refs.counter;

    bpf_kfunc_call_test_release(p);
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
