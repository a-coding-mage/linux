// SPDX-License-Identifier: GPL-2.0

// C dependencies translated from:
//   #include "vmlinux.h"
//   #include <bpf/bpf_helpers.h>
//   #include <bpf/bpf_endian.h>
//   #include "bpf_tracing_net.h"

pub static mut serv_port: __be16 = 0;

extern "C" {
    pub fn bpf_sock_destroy(sk: *mut sock_common) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut tcp_conn_sockets: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: ::core::mem::size_of::<__u32>() as __u32,
    value_size: ::core::mem::size_of::<__u64>() as __u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut udp_conn_sockets: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: ::core::mem::size_of::<__u32>() as __u32,
    value_size: ::core::mem::size_of::<__u64>() as __u32,
};

#[link_section = "cgroup/connect6"]
#[no_mangle]
pub unsafe extern "C" fn sock_connect(ctx: *mut bpf_sock_addr) -> ::core::ffi::c_int {
    let mut sock_cookie: __u64 = 0;
    let mut key: ::core::ffi::c_int = 0;
    let mut keyc: __u32 = 0;

    if (*ctx).family != AF_INET6 || (*ctx).user_family != AF_INET6 {
        return 1;
    }

    sock_cookie = bpf_get_socket_cookie(ctx as *mut _);
    if (*ctx).protocol == IPPROTO_TCP {
        bpf_map_update_elem(
            &mut tcp_conn_sockets as *mut _ as *mut _,
            &mut key as *mut _ as *mut _,
            &mut sock_cookie as *mut _ as *mut _,
            0,
        );
    } else if (*ctx).protocol == IPPROTO_UDP {
        bpf_map_update_elem(
            &mut udp_conn_sockets as *mut _ as *mut _,
            &mut keyc as *mut _ as *mut _,
            &mut sock_cookie as *mut _ as *mut _,
            0,
        );
    } else {
        return 1;
    }

    return 1;
}

#[link_section = "iter/tcp"]
#[no_mangle]
pub unsafe extern "C" fn iter_tcp6_client(ctx: *mut bpf_iter__tcp) -> ::core::ffi::c_int {
    let sk_common: *mut sock_common = (*ctx).sk_common;
    let mut sock_cookie: __u64 = 0;
    let mut val: *mut __u64;
    let mut key: ::core::ffi::c_int = 0;

    if sk_common.is_null() {
        return 0;
    }

    if (*sk_common).skc_family != AF_INET6 {
        return 0;
    }

    sock_cookie = bpf_get_socket_cookie(sk_common as *mut _);
    val = bpf_map_lookup_elem(
        &mut tcp_conn_sockets as *mut _ as *mut _,
        &mut key as *mut _ as *mut _,
    ) as *mut __u64;
    if val.is_null() {
        return 0;
    }
    /* Destroy connected client sockets. */
    if sock_cookie == *val {
        bpf_sock_destroy(sk_common);
    }

    return 0;
}

#[link_section = "iter/tcp"]
#[no_mangle]
pub unsafe extern "C" fn iter_tcp6_server(ctx: *mut bpf_iter__tcp) -> ::core::ffi::c_int {
    let sk_common: *mut sock_common = (*ctx).sk_common;
    let icsk: *const inet_connection_sock;
    let inet: *const inet_sock;
    let mut tcp_sk: *mut tcp6_sock;
    let srcp: __be16;

    if sk_common.is_null() {
        return 0;
    }

    if (*sk_common).skc_family != AF_INET6 {
        return 0;
    }

    tcp_sk = bpf_skc_to_tcp6_sock(sk_common);
    if tcp_sk.is_null() {
        return 0;
    }

    icsk = &(*tcp_sk).tcp.inet_conn;
    inet = &(*icsk).icsk_inet;
    srcp = (*inet).inet_sport;

    /* Destroy server sockets. */
    if srcp == serv_port {
        bpf_sock_destroy(sk_common);
    }

    return 0;
}

#[link_section = "iter/udp"]
#[no_mangle]
pub unsafe extern "C" fn iter_udp6_client(ctx: *mut bpf_iter__udp) -> ::core::ffi::c_int {
    let udp_sk: *mut udp_sock = (*ctx).udp_sk;
    let sk: *mut sock = udp_sk as *mut sock;
    let mut sock_cookie: __u64 = 0;
    let mut val: *mut __u64;
    let mut key: ::core::ffi::c_int = 0;

    if sk.is_null() {
        return 0;
    }

    sock_cookie = bpf_get_socket_cookie(sk as *mut _);
    val = bpf_map_lookup_elem(
        &mut udp_conn_sockets as *mut _ as *mut _,
        &mut key as *mut _ as *mut _,
    ) as *mut __u64;
    if val.is_null() {
        return 0;
    }
    /* Destroy connected client sockets. */
    if sock_cookie == *val {
        bpf_sock_destroy(sk as *mut sock_common);
    }

    return 0;
}

#[link_section = "iter/udp"]
#[no_mangle]
pub unsafe extern "C" fn iter_udp6_server(ctx: *mut bpf_iter__udp) -> ::core::ffi::c_int {
    let udp_sk: *mut udp_sock = (*ctx).udp_sk;
    let sk: *mut sock = udp_sk as *mut sock;
    let inet: *mut inet_sock;
    let srcp: __be16;

    if sk.is_null() {
        return 0;
    }

    inet = &mut (*udp_sk).inet;
    srcp = (*inet).inet_sport;
    if srcp == serv_port {
        bpf_sock_destroy(sk as *mut sock_common);
    }

    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];
