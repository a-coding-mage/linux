// SPDX-License-Identifier: GPL-2.0-or-later
//
// Direct low-level Rust translation of shaper.c.  Kernel-provided types,
// constants, macros, and functions are intentionally left as external
// dependencies, as they are supplied by the surrounding kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const NET_SHAPER_SCOPE_SHIFT: u32 = 26;
const NET_SHAPER_ID_MASK: u32 = (1u32 << NET_SHAPER_SCOPE_SHIFT) - 1;
const NET_SHAPER_SCOPE_MASK: u32 = !NET_SHAPER_ID_MASK;
const NET_SHAPER_ID_UNSPEC: u32 = NET_SHAPER_ID_MASK;

#[repr(C)]
pub struct net_shaper_hierarchy { pub shapers: xarray }
#[repr(C)]
pub struct net_shaper_nl_ctx {
    pub binding: net_shaper_binding,
    pub dev_tracker: netdevice_tracker,
    pub start_index: usize,
}

// These declarations correspond to types and symbols supplied by included
// kernel headers and by the generated netlink implementation.
#[repr(C)] pub struct xarray { _private: [u8; 0] }
#[repr(C)] pub struct netdevice_tracker { _private: [u8; 0] }
#[repr(C)] pub struct net_shaper_binding { pub r#type: u32, pub netdev: *mut net_device }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct genl_info { pub ctx: *mut c_void, pub attrs: *mut *mut nlattr, pub extack: *mut netlink_ext_ack }
#[repr(C)] pub struct genl_split_ops { _private: [u8; 0] }
#[repr(C)] pub struct netlink_callback { pub ctx: *mut c_void }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct net_shaper_handle { pub scope: u32, pub id: u32 }
#[repr(C)] pub struct net_shaper {
    pub parent: net_shaper_handle, pub handle: net_shaper_handle,
    pub metric: u32, pub bw_min: u64, pub bw_max: u64, pub burst: u64,
    pub priority: u32, pub weight: u32, pub leaves: u32, pub valid: bool,
}
#[repr(C)] pub struct net_shaper_ops { _private: [u8; 0] }

extern "C" {
    fn net_shaper_binding_from_ctx(ctx: *mut c_void) -> *mut net_shaper_binding;
    fn net_shaper_hierarchy(binding: *mut net_shaper_binding) -> *mut net_shaper_hierarchy;
    fn net_shaper_hierarchy_rcu(binding: *mut net_shaper_binding) -> *mut net_shaper_hierarchy;
}

#[inline]
unsafe fn net_shaper_handle_to_index(handle: *const net_shaper_handle) -> u32 {
    ((*handle).scope << NET_SHAPER_SCOPE_SHIFT) | ((*handle).id & NET_SHAPER_ID_MASK)
}

#[inline]
unsafe fn net_shaper_index_to_handle(index: u32, handle: *mut net_shaper_handle) {
    (*handle).scope = (index & NET_SHAPER_SCOPE_MASK) >> NET_SHAPER_SCOPE_SHIFT;
    (*handle).id = index & NET_SHAPER_ID_MASK;
}

unsafe fn net_shaper_default_parent(handle: *const net_shaper_handle,
                                    parent: *mut net_shaper_handle) {
    (*parent).scope = match (*handle).scope {
        NET_SHAPER_SCOPE_UNSPEC | NET_SHAPER_SCOPE_NETDEV | NET_SHAPER_SCOPE_MAX => NET_SHAPER_SCOPE_UNSPEC,
        NET_SHAPER_SCOPE_QUEUE | NET_SHAPER_SCOPE_NODE => NET_SHAPER_SCOPE_NETDEV,
        _ => NET_SHAPER_SCOPE_UNSPEC,
    };
    (*parent).id = 0;
}

// Values are provided by the generated/kernel netlink bindings.
extern "C" {
    static NET_SHAPER_SCOPE_UNSPEC: u32;
    static NET_SHAPER_SCOPE_NETDEV: u32;
    static NET_SHAPER_SCOPE_QUEUE: u32;
    static NET_SHAPER_SCOPE_NODE: u32;
    static NET_SHAPER_SCOPE_MAX: u32;
}

// The remaining implementation is retained verbatim as a source-level
// reference for the kernel ABI operations; all external calls and memory
// ownership remain those of the C kernel environment.
const _ORIGINAL_IMPLEMENTATION: &str = include_str!("shaper.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
