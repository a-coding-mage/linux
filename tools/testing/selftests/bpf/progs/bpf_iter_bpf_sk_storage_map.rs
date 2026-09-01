// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// C dependencies: <vmlinux.h>, "bpf_tracing_net.h",
// <bpf/bpf_helpers.h>, and <bpf/bpf_tracing.h>.

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SEC(".maps")
// Original C declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_SK_STORAGE);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __type(key, int);
//     __type(value, int);
// } sk_stg_map SEC(".maps");
#[repr(C)]
pub struct sk_stg_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key: i32,
    pub value: i32,
}

#[no_mangle]
pub static mut sk_stg_map: sk_stg_map_def = sk_stg_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key: 0,
    value: 0,
};

#[no_mangle]
pub static mut val_sum: __u32 = 0;
#[no_mangle]
pub static mut ipv6_sk_count: __u32 = 0;
#[no_mangle]
pub static mut to_add_val: __u32 = 0;

#[repr(C)]
pub struct sock {
    pub sk_family: __u16,
}

#[repr(C)]
pub struct bpf_iter__bpf_sk_storage_map {
    pub sk: *mut sock,
    pub value: *mut __u32,
}

// SEC("iter/bpf_sk_storage_map")
#[no_mangle]
pub unsafe extern "C" fn rw_bpf_sk_storage_map(ctx: *mut bpf_iter__bpf_sk_storage_map) -> i32 {
    let sk: *mut sock = (*ctx).sk;
    let val: *mut __u32 = (*ctx).value;

    if sk.is_null() || val.is_null() {
        return 0;
    }

    if (*sk).sk_family as i32 == AF_INET6 {
        ipv6_sk_count = ipv6_sk_count.wrapping_add(1);
    }

    val_sum = val_sum.wrapping_add(*val);

    *val = (*val).wrapping_add(to_add_val);

    0
}

// SEC("iter/bpf_sk_storage_map")
#[no_mangle]
pub unsafe extern "C" fn oob_write_bpf_sk_storage_map(
    ctx: *mut bpf_iter__bpf_sk_storage_map,
) -> i32 {
    let sk: *mut sock = (*ctx).sk;
    let val: *mut __u32 = (*ctx).value;

    if sk.is_null() || val.is_null() {
        return 0;
    }

    *val.add(1) = 0xdeadbeef;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
