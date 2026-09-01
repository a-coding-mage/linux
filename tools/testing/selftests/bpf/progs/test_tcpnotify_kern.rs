// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependencies:
// <stddef.h>, <string.h>, <netinet/in.h>, <linux/bpf.h>,
// <linux/if_ether.h>, <linux/if_packet.h>, <linux/ip.h>,
// <linux/ipv6.h>, <linux/types.h>, <linux/socket.h>, <linux/tcp.h>,
// <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>, "test_tcpnotify.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type __u32 = u32;
pub type __u64 = u64;

pub const BPF_MAP_TYPE_ARRAY: __u32 = 2;
pub const BPF_MAP_TYPE_PERF_EVENT_ARRAY: __u32 = 4;
pub const BPF_ANY: __u64 = 0;
pub const BPF_F_CURRENT_CPU: __u64 = 0xffffffff;

pub const BPF_SOCK_OPS_TIMEOUT_INIT: i32 = 1;
pub const BPF_SOCK_OPS_RWND_INIT: i32 = 2;
pub const BPF_SOCK_OPS_TCP_CONNECT_CB: i32 = 3;
pub const BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB: i32 = 4;
pub const BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB: i32 = 5;
pub const BPF_SOCK_OPS_NEEDS_ECN: i32 = 6;
pub const BPF_SOCK_OPS_BASE_RTT: i32 = 7;
pub const BPF_SOCK_OPS_RTO_CB: i32 = 8;
pub const BPF_SOCK_OPS_RETRANS_CB: i32 = 9;
pub const BPF_SOCK_OPS_TCP_LISTEN_CB: i32 = 10;

pub const BPF_SOCK_OPS_RETRANS_CB_FLAG: i32 = 1 << 1;
pub const BPF_SOCK_OPS_RTO_CB_FLAG: i32 = 1 << 2;

extern "C" {
    pub static TESTPORT: __u32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sock_ops {
    pub op: __u32,
    pub reply: i32,
    pub remote_port: __u32,
    pub total_retrans: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpnotify_globals {
    pub total_retrans: __u32,
    pub ncalls: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_notifier {
    pub type_: __u32,
    pub subtype: __u32,
    pub source: __u32,
    pub hash: __u32,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

extern "C" {
    pub fn bpf_ntohl(x: __u32) -> __u32;
    pub fn bpf_sock_ops_cb_flags_set(skops: *mut bpf_sock_ops, argval: i32) -> i32;
    pub fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    pub fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i32;
    pub fn bpf_perf_event_output(
        ctx: *mut core::ffi::c_void,
        map: *mut core::ffi::c_void,
        flags: __u64,
        data: *const core::ffi::c_void,
        size: __u64,
    ) -> i32;
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut global_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 4,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<tcpnotify_globals>() as __u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut perf_event_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERF_EVENT_ARRAY,
    max_entries: 0,
    key_size: core::mem::size_of::<i32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[link_section = "sockops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_testcb(skops: *mut bpf_sock_ops) -> i32 {
    let mut rv: i32 = -1;
    let op: i32;

    op = (*skops).op as i32;

    if bpf_ntohl((*skops).remote_port) != TESTPORT {
        (*skops).reply = -1;
        return 0;
    }

    match op {
        BPF_SOCK_OPS_TIMEOUT_INIT
        | BPF_SOCK_OPS_RWND_INIT
        | BPF_SOCK_OPS_NEEDS_ECN
        | BPF_SOCK_OPS_BASE_RTT
        | BPF_SOCK_OPS_RTO_CB => {
            rv = 1;
        }

        BPF_SOCK_OPS_TCP_CONNECT_CB
        | BPF_SOCK_OPS_TCP_LISTEN_CB
        | BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB
        | BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB => {
            bpf_sock_ops_cb_flags_set(
                skops,
                BPF_SOCK_OPS_RETRANS_CB_FLAG | BPF_SOCK_OPS_RTO_CB_FLAG,
            );
            rv = 1;
        }
        BPF_SOCK_OPS_RETRANS_CB => {
            let key: __u32 = 0;
            let mut g: tcpnotify_globals;
            let gp: *mut tcpnotify_globals;
            let msg: tcp_notifier = tcp_notifier {
                type_: 0xde,
                subtype: 0xad,
                source: 0xbe,
                hash: 0xef,
            };

            rv = 1;

            /* Update results */
            gp = bpf_map_lookup_elem(
                &mut global_map as *mut _ as *mut core::ffi::c_void,
                &key as *const _ as *const core::ffi::c_void,
            ) as *mut tcpnotify_globals;
            if gp.is_null() {
                (*skops).reply = rv;
                return 1;
            }
            g = *gp;
            g.total_retrans = (*skops).total_retrans;
            g.ncalls = g.ncalls.wrapping_add(1);
            bpf_map_update_elem(
                &mut global_map as *mut _ as *mut core::ffi::c_void,
                &key as *const _ as *const core::ffi::c_void,
                &g as *const _ as *const core::ffi::c_void,
                BPF_ANY,
            );
            bpf_perf_event_output(
                skops as *mut core::ffi::c_void,
                &mut perf_event_map as *mut _ as *mut core::ffi::c_void,
                BPF_F_CURRENT_CPU,
                &msg as *const _ as *const core::ffi::c_void,
                core::mem::size_of::<tcp_notifier>() as __u64,
            );
        }
        _ => {
            rv = -1;
        }
    }
    (*skops).reply = rv;
    1
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
