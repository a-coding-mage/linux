/* SPDX-License-Identifier: GPL-2.0+ */
/*
 *  IPv6 IOAM implementation
 *
 *  Author:
 *  Justin Iurman <justin.iurman@uliege.be>
 */

// Dependencies supplied by the corresponding kernel headers:
// linux/net.h, linux/ipv6.h, linux/ioam6.h, linux/ioam6_genl.h,
// linux/rhashtable-types.h

#[repr(C)]
pub struct ioam6_namespace {
    pub head: rhash_head,
    pub rcu: rcu_head,

    pub schema: *mut ioam6_schema,

    pub id: u16,
    pub data: u32,
    pub data_wide: u64,
}

#[repr(C)]
pub struct ioam6_schema {
    pub head: rhash_head,
    pub rcu: rcu_head,

    pub ns: *mut ioam6_namespace,

    pub id: u32,
    pub len: i32,
    pub hdr: u32,

    pub data: [u8; 0],
}

#[repr(C)]
pub struct ioam6_pernet_data {
    pub lock: mutex,
    pub namespaces: rhashtable,
    pub schemas: rhashtable,
}

// CONFIG_IPV6 conditional: when enabled, return net->ipv6.ioam6_data;
// otherwise return NULL.
#[inline]
pub unsafe fn ioam6_pernet(net: *mut net) -> *mut ioam6_pernet_data {
    #[cfg(CONFIG_IPV6)]
    {
        return (*net).ipv6.ioam6_data;
    }
    #[cfg(not(CONFIG_IPV6))]
    {
        return core::ptr::null_mut();
    }
}

unsafe extern "C" {
    pub fn ioam6_namespace(net: *mut net, id: u16) -> *mut ioam6_namespace;
    pub fn ioam6_fill_trace_data(
        skb: *mut sk_buff,
        ns: *mut ioam6_namespace,
        trace: *mut ioam6_trace_hdr,
        is_input: bool,
    );

    pub fn ioam6_trace_compute_nodelen(trace_type: u32) -> u8;

    pub fn ioam6_init() -> i32;
    pub fn ioam6_exit();

    pub fn ioam6_iptunnel_init() -> i32;
    pub fn ioam6_iptunnel_exit();

    pub fn ioam6_event(
        type_: ioam6_event_type,
        net: *mut net,
        gfp: gfp_t,
        opt: *mut core::ffi::c_void,
        opt_len: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
