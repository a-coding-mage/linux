// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependencies:
// string.h, stdbool.h, linux/bpf.h, linux/in.h, linux/in6.h, sys/socket.h,
// bpf/bpf_helpers.h, bpf/bpf_endian.h, bpf_sockopt_helpers.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u16 = u16;
pub type __be16 = u16;
pub type __be32 = u32;

pub const AF_INET: i32 = 2;
pub const BPF_MAP_TYPE_SK_STORAGE: u32 = 24;
pub const BPF_F_NO_PREALLOC: u32 = 1;
pub const BPF_SK_STORAGE_GET_F_CREATE: u64 = 1;

#[repr(C)]
pub struct bpf_sock_addr {
    pub user_family: u32,
    pub user_ip4: u32,
    pub user_ip6: [u32; 4],
    pub user_port: u32,
    pub family: u32,
    pub type_: u32,
    pub protocol: u32,
    pub msg_src_ip4: u32,
    pub msg_src_ip6: [u32; 4],
    pub sk: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: __be32,
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: __be16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut port: __u16 = 0;

#[repr(C)]
pub struct svc_addr {
    pub addr: __be32,
    pub port: __be16,
}

// Original C declaration used BPF helper macros:
// struct {
//     __uint(type, BPF_MAP_TYPE_SK_STORAGE);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __type(key, int);
//     __type(value, struct svc_addr);
// } service_mapping SEC(".maps");
#[repr(C)]
pub struct service_mapping_def {
    pub type_: u32,
    pub map_flags: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut service_mapping: service_mapping_def = service_mapping_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
};

extern "C" {
    pub fn bpf_bind(
        ctx: *mut bpf_sock_addr,
        addr: *mut sockaddr,
        addr_len: i32,
    ) -> i64;
    pub fn bpf_sk_storage_get(
        map: *mut service_mapping_def,
        sk: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut svc_addr;
    pub fn bpf_htons(x: __u16) -> __be16;
    pub fn bpf_htonl(x: u32) -> __be32;
    pub fn get_set_sk_priority(ctx: *mut bpf_sock_addr) -> bool;
}

#[no_mangle]
#[link_section = "cgroup/connect4"]
pub unsafe extern "C" fn connect4(ctx: *mut bpf_sock_addr) -> i32 {
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut orig: *mut svc_addr;

    /* Force local address to 127.0.0.1:22222. */
    sa.sin_family = AF_INET as u16;
    sa.sin_port = bpf_htons(22222);
    sa.sin_addr.s_addr = bpf_htonl(0x7f000001);

    if bpf_bind(
        ctx,
        &mut sa as *mut sockaddr_in as *mut sockaddr,
        core::mem::size_of::<sockaddr_in>() as i32,
    ) != 0
    {
        return 0;
    }

    /* Rewire service 1.2.3.4:60000 to backend 127.0.0.1:port. */
    if (*ctx).user_port == bpf_htons(60000) as u32 {
        orig = bpf_sk_storage_get(
            &mut service_mapping,
            (*ctx).sk,
            core::ptr::null_mut(),
            BPF_SK_STORAGE_GET_F_CREATE,
        );
        if orig.is_null() {
            return 0;
        }

        (*orig).addr = (*ctx).user_ip4;
        (*orig).port = (*ctx).user_port as __be16;

        (*ctx).user_ip4 = bpf_htonl(0x7f000001);
        (*ctx).user_port = bpf_htons(port) as u32;
    }
    return 1;
}

#[no_mangle]
#[link_section = "cgroup/getsockname4"]
pub unsafe extern "C" fn getsockname4(ctx: *mut bpf_sock_addr) -> i32 {
    if !get_set_sk_priority(ctx) {
        return 1;
    }

    /* Expose local server as 1.2.3.4:60000 to client. */
    if (*ctx).user_port == bpf_htons(port) as u32 {
        (*ctx).user_ip4 = bpf_htonl(0x01020304);
        (*ctx).user_port = bpf_htons(60000) as u32;
    }
    return 1;
}

#[no_mangle]
#[link_section = "cgroup/getpeername4"]
pub unsafe extern "C" fn getpeername4(ctx: *mut bpf_sock_addr) -> i32 {
    let mut orig: *mut svc_addr;

    if !get_set_sk_priority(ctx) {
        return 1;
    }

    /* Expose service 1.2.3.4:60000 as peer instead of backend. */
    if (*ctx).user_port == bpf_htons(port) as u32 {
        orig = bpf_sk_storage_get(
            &mut service_mapping,
            (*ctx).sk,
            core::ptr::null_mut(),
            0,
        );
        if !orig.is_null() {
            (*ctx).user_ip4 = (*orig).addr;
            (*ctx).user_port = (*orig).port as u32;
        }
    }
    return 1;
}
