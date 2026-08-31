// SPDX-License-Identifier: GPL-2.0

// C dependencies: <string.h>, <linux/bpf.h>, <linux/in.h>, <linux/in6.h>,
// <sys/socket.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>,
// <bpf_sockopt_helpers.h>

type __u16 = u16;
type __u32 = u32;
type __be16 = __u16;
type __be32 = __u32;

const AF_INET6: u16 = 10;
const BPF_MAP_TYPE_SK_STORAGE: u32 = 24;
const BPF_F_NO_PREALLOC: u32 = 1;
const BPF_SK_STORAGE_GET_F_CREATE: u64 = 1;

#[repr(C)]
pub struct in6_addr {
    pub s6_addr32: [__be32; 4],
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
}

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: __be16,
    pub sin6_flowinfo: __be32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: __u32,
}

#[repr(C)]
pub struct bpf_sock_addr {
    pub user_family: __u32,
    pub user_ip4: __u32,
    pub user_ip6: [__u32; 4],
    pub user_port: __u32,
    pub family: __u32,
    pub type_: __u32,
    pub protocol: __u32,
    pub msg_src_ip4: __u32,
    pub msg_src_ip6: [__u32; 4],
    pub sk: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct svc_addr {
    pub addr: [__be32; 4],
    pub port: __be16,
}

#[repr(C)]
pub struct service_mapping_def {
    pub type_: __u32,
    pub map_flags: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut port: __u16 = 0;

#[no_mangle]
#[link_section = ".maps"]
pub static mut service_mapping: service_mapping_def = service_mapping_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as __u32,
    value_size: core::mem::size_of::<svc_addr>() as __u32,
};

extern "C" {
    fn bpf_bind(ctx: *mut bpf_sock_addr, addr: *mut sockaddr, addr_len: i32) -> i64;
    fn bpf_sk_storage_get(
        map: *mut service_mapping_def,
        sk: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut svc_addr;
    fn get_set_sk_priority(ctx: *mut bpf_sock_addr) -> i32;
}

#[inline]
fn bpf_htons(x: __u16) -> __be16 {
    x.to_be()
}

#[inline]
fn bpf_htonl(x: __u32) -> __be32 {
    x.to_be()
}

#[no_mangle]
#[link_section = "cgroup/connect6"]
pub unsafe extern "C" fn connect6(ctx: *mut bpf_sock_addr) -> i32 {
    let mut sa: sockaddr_in6 = core::mem::zeroed();
    let mut orig: *mut svc_addr;

    /* Force local address to [::1]:22223. */
    sa.sin6_family = AF_INET6;
    sa.sin6_port = bpf_htons(22223);
    sa.sin6_addr.s6_addr32[3] = bpf_htonl(1);

    if bpf_bind(
        ctx,
        (&mut sa as *mut sockaddr_in6).cast::<sockaddr>(),
        core::mem::size_of::<sockaddr_in6>() as i32,
    ) != 0
    {
        return 0;
    }

    /* Rewire service [fc00::1]:60000 to backend [::1]:port. */
    if (*ctx).user_port == bpf_htons(60000) as __u32 {
        orig = bpf_sk_storage_get(
            &mut service_mapping as *mut service_mapping_def,
            (*ctx).sk,
            core::ptr::null_mut(),
            BPF_SK_STORAGE_GET_F_CREATE,
        );
        if orig.is_null() {
            return 0;
        }

        (*orig).addr[0] = (*ctx).user_ip6[0];
        (*orig).addr[1] = (*ctx).user_ip6[1];
        (*orig).addr[2] = (*ctx).user_ip6[2];
        (*orig).addr[3] = (*ctx).user_ip6[3];
        (*orig).port = (*ctx).user_port as __be16;

        (*ctx).user_ip6[0] = 0;
        (*ctx).user_ip6[1] = 0;
        (*ctx).user_ip6[2] = 0;
        (*ctx).user_ip6[3] = bpf_htonl(1);
        (*ctx).user_port = bpf_htons(port) as __u32;
    }
    1
}

#[no_mangle]
#[link_section = "cgroup/getsockname6"]
pub unsafe extern "C" fn getsockname6(ctx: *mut bpf_sock_addr) -> i32 {
    if get_set_sk_priority(ctx) == 0 {
        return 1;
    }

    /* Expose local server as [fc00::1]:60000 to client. */
    if (*ctx).user_port == bpf_htons(port) as __u32 {
        (*ctx).user_ip6[0] = bpf_htonl(0xfc000000);
        (*ctx).user_ip6[1] = 0;
        (*ctx).user_ip6[2] = 0;
        (*ctx).user_ip6[3] = bpf_htonl(1);
        (*ctx).user_port = bpf_htons(60000) as __u32;
    }
    1
}

#[no_mangle]
#[link_section = "cgroup/getpeername6"]
pub unsafe extern "C" fn getpeername6(ctx: *mut bpf_sock_addr) -> i32 {
    let mut orig: *mut svc_addr;

    if get_set_sk_priority(ctx) == 0 {
        return 1;
    }

    /* Expose service [fc00::1]:60000 as peer instead of backend. */
    if (*ctx).user_port == bpf_htons(port) as __u32 {
        orig = bpf_sk_storage_get(
            &mut service_mapping as *mut service_mapping_def,
            (*ctx).sk,
            core::ptr::null_mut(),
            0,
        );
        if !orig.is_null() {
            (*ctx).user_ip6[0] = (*orig).addr[0];
            (*ctx).user_ip6[1] = (*orig).addr[1];
            (*ctx).user_ip6[2] = (*orig).addr[2];
            (*ctx).user_ip6[3] = (*orig).addr[3];
            (*ctx).user_port = (*orig).port as __u32;
        }
    }
    1
}
