// SPDX-License-Identifier: GPL-2.0-only

// C dependencies:
// #include <sys/socket.h>
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type c_int = i32;
type __u32 = u32;
type __u64 = u64;

const SOCK_DGRAM: c_int = 2;
const BPF_MAP_TYPE_SK_STORAGE: __u32 = 24;
const BPF_F_NO_PREALLOC: __u32 = 1;
const BPF_SK_STORAGE_GET_F_CREATE: __u64 = 1;

#[repr(C)]
pub struct bpf_sock {
    pub r#type: __u32,
}

#[repr(C)]
pub struct bpf_map_def {
    pub r#type: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32,
}

#[no_mangle]
pub static mut invocations: c_int = 0;
#[no_mangle]
pub static mut in_use: c_int = 0;

// Original C uses libbpf BTF map declaration macros:
// struct {
//     __uint(type, BPF_MAP_TYPE_SK_STORAGE);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __type(key, int);
//     __type(value, int);
// } sk_map SEC(".maps");
#[no_mangle]
#[link_section = ".maps"]
pub static mut sk_map: bpf_map_def = bpf_map_def {
    r#type: BPF_MAP_TYPE_SK_STORAGE,
    key_size: core::mem::size_of::<c_int>() as __u32,
    value_size: core::mem::size_of::<c_int>() as __u32,
    max_entries: 0,
    map_flags: BPF_F_NO_PREALLOC,
};

extern "C" {
    fn bpf_sk_storage_get(
        map: *mut bpf_map_def,
        sk: *mut bpf_sock,
        value: *mut core::ffi::c_void,
        flags: __u64,
    ) -> *mut c_int;
}

#[inline(always)]
unsafe fn __sync_fetch_and_add(ptr: *mut c_int, val: c_int) -> c_int {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old.wrapping_add(val));
    old
}

#[no_mangle]
#[link_section = "cgroup/sock_create"]
pub unsafe extern "C" fn sock(ctx: *mut bpf_sock) -> c_int {
    let mut sk_storage: *mut c_int;

    if (*ctx).r#type != SOCK_DGRAM as __u32 {
        return 1;
    }

    sk_storage = bpf_sk_storage_get(
        &mut sk_map,
        ctx,
        core::ptr::null_mut(),
        BPF_SK_STORAGE_GET_F_CREATE,
    );
    if sk_storage.is_null() {
        return 0;
    }
    *sk_storage = 0xdeadbeefu32 as c_int;

    __sync_fetch_and_add(&mut invocations, 1);

    if in_use > 0 {
        /* BPF_CGROUP_INET_SOCK_RELEASE is _not_ called
         * when we return an error from the BPF
         * program!
         */
        return 0;
    }

    __sync_fetch_and_add(&mut in_use, 1);
    1
}

#[no_mangle]
#[link_section = "cgroup/sock_release"]
pub unsafe extern "C" fn sock_release(ctx: *mut bpf_sock) -> c_int {
    let mut sk_storage: *mut c_int;

    if (*ctx).r#type != SOCK_DGRAM as __u32 {
        return 1;
    }

    sk_storage = bpf_sk_storage_get(&mut sk_map, ctx, core::ptr::null_mut(), 0);
    if sk_storage.is_null() || *sk_storage != 0xdeadbeefu32 as c_int {
        return 0;
    }

    __sync_fetch_and_add(&mut invocations, 1);
    __sync_fetch_and_add(&mut in_use, -1);
    1
}
