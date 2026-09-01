// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Rust translation of dependencies originally included from:
 * "vmlinux.h"
 * <bpf/bpf_helpers.h>
 * <bpf/bpf_tracing.h>
 * <bpf/bpf_core_read.h>
 * "test_jhash.h" with ATTR defined as __always_inline
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type u32 = ::core::ffi::c_uint;
type size_t = usize;

const BPF_MAP_TYPE_ARRAY: u32 = 2;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    pub ifindex: ::core::ffi::c_int,
    pub mtu: ::core::ffi::c_int,
}

#[repr(C)]
pub struct sk_buff {
    pub len: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
    pub pkt_type: u32,
    pub mark: u32,
    pub queue_mapping: u32,
    pub protocol: u32,
    pub vlan_present: u32,
    pub vlan_tci: u32,
    pub vlan_proto: u32,
    pub priority: u32,
    pub ingress_ifindex: u32,
    pub ifindex: u32,
    pub tc_index: u32,
    pub cb: [u32; 5],
    pub hash: u32,
    pub tc_classid: u32,
    pub data: u32,
    pub data_end: u32,
    pub napi_id: u32,
    pub family: u32,
    pub remote_ip4: u32,
    pub local_ip4: u32,
    pub remote_ip6: [u32; 4],
    pub local_ip6: [u32; 4],
    pub remote_port: u32,
    pub local_port: u32,
    pub data_meta: u32,
}

#[repr(C)]
pub struct bpf_map_def_array_u32_u32_256 {
    pub type_: u32,
    pub max_entries: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut array1: bpf_map_def_array_u32_u32_256 = bpf_map_def_array_u32_u32_256 {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 256,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut array2: bpf_map_def_array_u32_u32_256 = bpf_map_def_array_u32_u32_256 {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 256,
};

extern "C" {
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_map_lookup_elem(map: *mut bpf_map, key: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn jhash(key: *const ::core::ffi::c_void, length: u32, initval: u32) -> u32;
    fn bpf_core_type_exists<T>() -> ::core::ffi::c_int;
}

#[inline(never)]
unsafe fn randmap(v: ::core::ffi::c_int, dev: *const net_device) -> ::core::ffi::c_int {
    let mut map: *mut bpf_map = &mut array1 as *mut _ as *mut bpf_map;
    let key: ::core::ffi::c_int = (bpf_get_prandom_u32() & 0xff) as ::core::ffi::c_int;
    let val: *mut ::core::ffi::c_int;

    if (bpf_get_prandom_u32() & 1) != 0 {
        map = &mut array2 as *mut _ as *mut bpf_map;
    }

    val = bpf_map_lookup_elem(
        map,
        &key as *const _ as *const ::core::ffi::c_void,
    ) as *mut ::core::ffi::c_int;
    if !val.is_null() {
        *val = (bpf_get_prandom_u32() as ::core::ffi::c_int)
            .wrapping_add(v)
            .wrapping_add((*dev).mtu);
    }

    0
}

#[link_section = "tp_btf/xdp_devmap_xmit"]
#[no_mangle]
pub unsafe extern "C" fn tp_xdp_devmap_xmit_multi(
    from_dev: *const net_device,
    to_dev: *const net_device,
    sent: ::core::ffi::c_int,
    drops: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let _ = to_dev;
    let _ = sent;
    let _ = drops;
    let _ = err;
    randmap((*from_dev).ifindex, from_dev)
}

#[link_section = "fentry/eth_type_trans"]
#[no_mangle]
pub unsafe extern "C" fn fentry_eth_type_trans(
    skb: *mut sk_buff,
    dev: *mut net_device,
    protocol: ::core::ffi::c_ushort,
) -> ::core::ffi::c_int {
    let _ = protocol;
    randmap(
        (*dev).ifindex.wrapping_add((*skb).len as ::core::ffi::c_int),
        dev,
    )
}

#[link_section = "fexit/eth_type_trans"]
#[no_mangle]
pub unsafe extern "C" fn fexit_eth_type_trans(
    skb: *mut sk_buff,
    dev: *mut net_device,
    protocol: ::core::ffi::c_ushort,
) -> ::core::ffi::c_int {
    let _ = protocol;
    randmap(
        (*dev).ifindex.wrapping_add((*skb).len as ::core::ffi::c_int),
        dev,
    )
}

#[no_mangle]
pub static never: ::core::ffi::c_int = 0;

#[repr(C)]
pub struct __sk_bUfF {
    /* it will not exist in vmlinux */
    pub len: ::core::ffi::c_int,
}
/* C attribute preserved intent: preserve_access_index */

#[repr(C)]
pub struct bpf_testmod_test_read_ctx {
    /* it exists in bpf_testmod */
    pub len: size_t,
}
/* C attribute preserved intent: preserve_access_index */

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn balancer_ingress(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let data_end: *mut ::core::ffi::c_void = (*ctx).data_end as usize as *mut ::core::ffi::c_void;
    let data: *mut ::core::ffi::c_void = (*ctx).data as usize as *mut ::core::ffi::c_void;
    let mut ptr: *mut ::core::ffi::c_void;
    let nh_off: ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;

    nh_off = 14;

    /* pragma unroll doesn't work on large loops */
    macro_rules! C {
        () => {{
            ptr = (data as *mut u8).offset(i as isize) as *mut ::core::ffi::c_void;
            if (ptr as *mut u8).offset(nh_off as isize) > data_end as *mut u8 {
                break;
            }
            (*ctx).tc_index = jhash(
                ptr as *const ::core::ffi::c_void,
                nh_off as u32,
                (*ctx).cb[0].wrapping_add({
                    let old = i;
                    i = i.wrapping_add(1);
                    old as u32
                }),
            );
            if never != 0 {
                /* below is a dead code with unresolvable CO-RE relo */
                i = i.wrapping_add((*(ctx as *mut __sk_bUfF)).len);
                /* this CO-RE relo may or may not resolve
                 * depending on whether bpf_testmod is loaded.
                 */
                i = i.wrapping_add((*(ctx as *mut bpf_testmod_test_read_ctx)).len as ::core::ffi::c_int);
            }
        }};
    }
    macro_rules! C30 {
        () => {{
            C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!();
            C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!();
            C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!();
        }};
    }
    loop {
        C30!(); C30!(); C30!(); /* 90 calls */
        break;
    }
    0
}

type func_proto_typedef___match = Option<unsafe extern "C" fn(::core::ffi::c_long) -> ::core::ffi::c_int>;
type func_proto_typedef___doesnt_match = Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int>;
type func_proto_typedef_nested1 = Option<unsafe extern "C" fn(func_proto_typedef___match) -> ::core::ffi::c_int>;

#[no_mangle]
pub static mut proto_out: [::core::ffi::c_int; 3] = [0; 3];

#[link_section = "raw_tracepoint/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn core_relo_proto(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let _ = ctx;
    proto_out[0] = bpf_core_type_exists::<func_proto_typedef___match>();
    proto_out[1] = bpf_core_type_exists::<func_proto_typedef___doesnt_match>();
    proto_out[2] = bpf_core_type_exists::<func_proto_typedef_nested1>();

    0
}

#[link_section = "license"]
#[no_mangle]
pub static LICENSE: [::core::ffi::c_char; 4] = [b'G' as ::core::ffi::c_char, b'P' as ::core::ffi::c_char, b'L' as ::core::ffi::c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
