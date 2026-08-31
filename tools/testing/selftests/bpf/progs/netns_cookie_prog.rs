// SPDX-License-Identifier: GPL-2.0

// Dependencies from "vmlinux.h" and <bpf/bpf_helpers.h> are expected to be
// supplied by the BPF Rust build environment.

const AF_INET6: i32 = 10;

const BPF_MAP_TYPE_SK_STORAGE: u32 = 24;
const BPF_MAP_TYPE_SOCKMAP: u32 = 15;
const BPF_F_NO_PREALLOC: u32 = 1;
const BPF_SK_STORAGE_GET_F_CREATE: u64 = 1;
const BPF_NOEXIST: u64 = 1;
const BPF_SOCK_OPS_TCP_CONNECT_CB: u32 = 2;
const BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB: u32 = 4;
const TCX_PASS: i32 = 0;
const SK_PASS: i32 = 1;

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock_ops {
    pub op: u32,
    pub family: u32,
    pub sk: *mut bpf_sock,
}

#[repr(C)]
pub struct sk_msg_md {
    pub family: u32,
    pub sk: *mut bpf_sock,
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

unsafe extern "C" {
    fn bpf_sk_storage_get(
        map: *mut bpf_map_def,
        sk: *mut bpf_sock,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut i32;
    fn bpf_get_netns_cookie(ctx: *mut core::ffi::c_void) -> i64;
    fn bpf_sock_map_update(
        ctx: *mut bpf_sock_ops,
        map: *mut bpf_map_def,
        key: *mut u32,
        flags: u64,
    ) -> i64;
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sockops_netns_cookies: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
    max_entries: 0,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sk_msg_netns_cookies: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
    max_entries: 0,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sock_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    map_flags: 0,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
    max_entries: 2,
};

#[unsafe(no_mangle)]
pub static mut tcx_init_netns_cookie: i32 = 0;
#[unsafe(no_mangle)]
pub static mut tcx_netns_cookie: i32 = 0;
#[unsafe(no_mangle)]
pub static mut cgroup_skb_init_netns_cookie: i32 = 0;
#[unsafe(no_mangle)]
pub static mut cgroup_skb_netns_cookie: i32 = 0;

#[unsafe(link_section = "sockops")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_netns_cookie_sockops(ctx: *mut bpf_sock_ops) -> i32 {
    let sk: *mut bpf_sock = unsafe { (*ctx).sk };
    let mut cookie: *mut i32;
    let mut key: u32 = 0;

    if unsafe { (*ctx).family } != AF_INET6 as u32 {
        return 1;
    }

    if sk.is_null() {
        return 1;
    }

    match unsafe { (*ctx).op } {
        BPF_SOCK_OPS_TCP_CONNECT_CB => {
            cookie = unsafe {
                bpf_sk_storage_get(
                    &raw mut sockops_netns_cookies,
                    sk,
                    core::ptr::null_mut(),
                    BPF_SK_STORAGE_GET_F_CREATE,
                )
            };
            if cookie.is_null() {
                return 1;
            }

            unsafe {
                *cookie = bpf_get_netns_cookie(ctx as *mut core::ffi::c_void) as i32;
            }
        }
        BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB => {
            unsafe {
                bpf_sock_map_update(ctx, &raw mut sock_map, &mut key, BPF_NOEXIST);
            }
        }
        _ => {}
    }

    1
}

#[unsafe(link_section = "sk_msg")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_netns_cookie_sk_msg(msg: *mut sk_msg_md) -> i32 {
    let sk: *mut bpf_sock = unsafe { (*msg).sk };
    let cookie: *mut i32;

    if unsafe { (*msg).family } != AF_INET6 as u32 {
        return 1;
    }

    if sk.is_null() {
        return 1;
    }

    cookie = unsafe {
        bpf_sk_storage_get(
            &raw mut sk_msg_netns_cookies,
            sk,
            core::ptr::null_mut(),
            BPF_SK_STORAGE_GET_F_CREATE,
        )
    };
    if cookie.is_null() {
        return 1;
    }

    unsafe {
        *cookie = bpf_get_netns_cookie(msg as *mut core::ffi::c_void) as i32;
    }

    1
}

#[unsafe(link_section = "tcx/ingress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_netns_cookie_tcx(skb: *mut __sk_buff) -> i32 {
    unsafe {
        tcx_init_netns_cookie = bpf_get_netns_cookie(core::ptr::null_mut()) as i32;
        tcx_netns_cookie = bpf_get_netns_cookie(skb as *mut core::ffi::c_void) as i32;
    }
    TCX_PASS
}

#[unsafe(link_section = "cgroup_skb/ingress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_netns_cookie_cgroup_skb(skb: *mut __sk_buff) -> i32 {
    unsafe {
        cgroup_skb_init_netns_cookie = bpf_get_netns_cookie(core::ptr::null_mut()) as i32;
        cgroup_skb_netns_cookie = bpf_get_netns_cookie(skb as *mut core::ffi::c_void) as i32;
    }
    SK_PASS
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
