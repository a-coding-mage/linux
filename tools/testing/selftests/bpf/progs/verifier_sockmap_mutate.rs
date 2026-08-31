// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"

pub const BPF_MAP_TYPE_SOCKHASH: u32 = 18;
pub const BPF_MAP_TYPE_SOCKMAP: u32 = 15;
pub const BPF_ANY: u64 = 0;
pub const XDP_PASS: i32 = 2;

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct sock {}

#[repr(C)]
pub union bpf_iter__sockmap_union {
    pub sk: *mut sock,
}

#[repr(C)]
pub struct bpf_iter__sockmap {
    pub anonymous: bpf_iter__sockmap_union,
}

#[repr(C)]
pub struct bpf_sock {}

#[repr(C)]
pub struct __sk_buff {
    pub sk: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct bpf_sk_lookup {
    pub sk: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct sk_reuseport_md {
    pub sk: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct bpf_sock_ops {
    pub sk: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct xdp_md {}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sockhash: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKHASH,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sockmap: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

pub const CG_OK: i32 = 1;

#[unsafe(no_mangle)]
pub static mut zero: i32 = 0;

unsafe extern "C" {
    fn bpf_map_delete_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> i64;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_sk_release(sk: *mut core::ffi::c_void) -> i64;
    fn bpf_sock_map_update(
        skops: *mut bpf_sock_ops,
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_sock_hash_update(
        skops: *mut bpf_sock_ops,
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

#[inline(always)]
unsafe fn test_sockmap_delete() {
    unsafe {
        bpf_map_delete_elem(&raw mut sockmap as *mut core::ffi::c_void, &raw const zero as *const core::ffi::c_void);
        bpf_map_delete_elem(&raw mut sockhash as *mut core::ffi::c_void, &raw const zero as *const core::ffi::c_void);
    }
}

#[inline(always)]
unsafe fn test_sockmap_update(sk: *mut core::ffi::c_void) {
    if !sk.is_null() {
        unsafe {
            bpf_map_update_elem(
                &raw mut sockmap as *mut core::ffi::c_void,
                &raw const zero as *const core::ffi::c_void,
                sk as *const core::ffi::c_void,
                BPF_ANY,
            );
            bpf_map_update_elem(
                &raw mut sockhash as *mut core::ffi::c_void,
                &raw const zero as *const core::ffi::c_void,
                sk as *const core::ffi::c_void,
                BPF_ANY,
            );
        }
    }
}

#[inline(always)]
unsafe fn test_sockmap_lookup_and_update() {
    let sk: *mut bpf_sock = unsafe {
        bpf_map_lookup_elem(
            &raw mut sockmap as *mut core::ffi::c_void,
            &raw const zero as *const core::ffi::c_void,
        ) as *mut bpf_sock
    };

    if !sk.is_null() {
        unsafe {
            test_sockmap_update(sk as *mut core::ffi::c_void);
            bpf_sk_release(sk as *mut core::ffi::c_void);
        }
    }
}

#[inline(always)]
unsafe fn test_sockmap_mutate(sk: *mut core::ffi::c_void) {
    unsafe {
        test_sockmap_delete();
        test_sockmap_update(sk);
    }
}

#[inline(always)]
unsafe fn test_sockmap_lookup_and_mutate() {
    unsafe {
        test_sockmap_delete();
        test_sockmap_lookup_and_update();
    }
}

// SEC("action")
// __failure __msg("cannot update sockmap in this context")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_sched_act(skb: *mut __sk_buff) -> i32 {
    unsafe {
        test_sockmap_mutate((*skb).sk);
    }
    0
}

// SEC("classifier")
// __failure __msg("cannot update sockmap in this context")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_sched_cls(skb: *mut __sk_buff) -> i32 {
    unsafe {
        test_sockmap_mutate((*skb).sk);
    }
    0
}

// SEC("flow_dissector")
// __failure __msg("cannot update sockmap in this context")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_flow_dissector_delete(_skb: *mut __sk_buff) -> i32 {
    unsafe {
        test_sockmap_delete();
    }
    0
}

// SEC("flow_dissector")
// __failure __msg("cannot update sockmap in this context")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_flow_dissector_update(_skb: *mut __sk_buff) -> i32 {
    unsafe {
        test_sockmap_lookup_and_update(); /* no access to skb->sk */
    }
    0
}

// SEC("iter/sockmap")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_trace_iter(ctx: *mut bpf_iter__sockmap) -> i32 {
    unsafe {
        test_sockmap_mutate((*ctx).anonymous.sk as *mut core::ffi::c_void);
    }
    0
}

// SEC("raw_tp/kfree")
// __failure __msg("cannot update sockmap in this context")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_raw_tp_delete(_ctx: *const core::ffi::c_void) -> i32 {
    unsafe {
        test_sockmap_delete();
    }
    0
}

// SEC("raw_tp/kfree")
// __failure __msg("cannot update sockmap in this context")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_raw_tp_update(_ctx: *const core::ffi::c_void) -> i32 {
    unsafe {
        test_sockmap_lookup_and_update();
    }
    0
}

// SEC("sk_lookup")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_sk_lookup(ctx: *mut bpf_sk_lookup) -> i32 {
    unsafe {
        test_sockmap_mutate((*ctx).sk);
    }
    0
}

// SEC("sk_reuseport")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_sk_reuseport(ctx: *mut sk_reuseport_md) -> i32 {
    unsafe {
        test_sockmap_mutate((*ctx).sk);
    }
    0
}

// SEC("socket")
// __failure __msg("cannot update sockmap in this context")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_socket_filter(skb: *mut __sk_buff) -> i32 {
    unsafe {
        test_sockmap_mutate((*skb).sk);
    }
    0
}

// SEC("sockops")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_sockops_delete(_ctx: *mut bpf_sock_ops) -> i32 {
    unsafe {
        test_sockmap_delete();
    }
    CG_OK
}

// SEC("sockops")
// __failure __msg("cannot update sockmap in this context")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_sockops_update(ctx: *mut bpf_sock_ops) -> i32 {
    unsafe {
        test_sockmap_update((*ctx).sk);
    }
    CG_OK
}

// SEC("sockops")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_sockops_update_dedicated(ctx: *mut bpf_sock_ops) -> i32 {
    unsafe {
        bpf_sock_map_update(
            ctx,
            &raw mut sockmap as *mut core::ffi::c_void,
            &raw const zero as *const core::ffi::c_void,
            BPF_ANY,
        );
        bpf_sock_hash_update(
            ctx,
            &raw mut sockhash as *mut core::ffi::c_void,
            &raw const zero as *const core::ffi::c_void,
            BPF_ANY,
        );
    }
    CG_OK
}

// SEC("xdp")
// __failure __msg("cannot update sockmap in this context")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_xdp(_ctx: *mut xdp_md) -> i32 {
    unsafe {
        test_sockmap_lookup_and_mutate();
    }
    XDP_PASS
}
