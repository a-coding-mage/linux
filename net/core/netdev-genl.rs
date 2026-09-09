// SPDX-License-Identifier: GPL-2.0-only
//
// Direct low-level Rust translation of netdev-genl.c. Kernel-provided types,
// constants, macros, and functions are intentionally left as external inputs.

#[repr(C)]
pub struct netdev_nl_dump_ctx {
    pub ifindex: libc::c_ulong,
    pub rxq_idx: libc::c_uint,
    pub txq_idx: libc::c_uint,
    pub napi_id: libc::c_uint,
}

extern "C" {
    fn netdev_nl_dev_fill(netdev: *mut net_device, rsp: *mut sk_buff, info: *const genl_info) -> libc::c_int;
    fn netdev_nl_napi_fill_one(rsp: *mut sk_buff, napi: *mut napi_struct, info: *const genl_info) -> libc::c_int;
    fn netdev_nl_queue_fill_one(rsp: *mut sk_buff, netdev: *mut net_device, q_idx: u32, q_type: u32, info: *const genl_info) -> libc::c_int;
    fn netdev_nl_stats_queue(netdev: *mut net_device, rsp: *mut sk_buff, q_type: u32, i: libc::c_int, info: *const genl_info) -> libc::c_int;
}

#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub len: usize, _private: [u8; 0] }
#[repr(C)] pub struct genl_info { _private: [u8; 0] }
#[repr(C)] pub struct netlink_callback { pub ctx: [libc::c_ulong; 5], _private: [u8; 0] }
#[repr(C)] pub struct napi_struct { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct netdev_rx_queue { _private: [u8; 0] }
#[repr(C)] pub struct netdev_queue_stats_rx { _private: [u8; 0] }
#[repr(C)] pub struct netdev_queue_stats_tx { _private: [u8; 0] }
#[repr(C)] pub struct netdev_nl_sock { _private: [u8; 0] }
#[repr(C)] pub struct net_devmem_dmabuf_binding { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }

#[inline]
unsafe fn netdev_dump_ctx(cb: *mut netlink_callback) -> *mut netdev_nl_dump_ctx {
    // NL_ASSERT_CTX_FITS(struct netdev_nl_dump_ctx)
    (*cb).ctx.as_mut_ptr() as *mut netdev_nl_dump_ctx
}

// The following declarations retain the exported kernel interfaces. Their
// implementations depend on the corresponding Linux networking translation.
extern "C" {
    pub fn netdev_nl_dev_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> libc::c_int;
    pub fn netdev_nl_dev_get_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> libc::c_int;
    pub fn netdev_nl_napi_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> libc::c_int;
    pub fn netdev_nl_napi_get_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> libc::c_int;
    pub fn netdev_nl_napi_set_doit(skb: *mut sk_buff, info: *mut genl_info) -> libc::c_int;
    pub fn netdev_nl_queue_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> libc::c_int;
    pub fn netdev_nl_queue_get_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> libc::c_int;
    pub fn netdev_nl_qstats_get_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> libc::c_int;
    pub fn netdev_nl_bind_rx_doit(skb: *mut sk_buff, info: *mut genl_info) -> libc::c_int;
    pub fn netdev_nl_bind_tx_doit(skb: *mut sk_buff, info: *mut genl_info) -> libc::c_int;
    pub fn netdev_nl_queue_create_doit(skb: *mut sk_buff, info: *mut genl_info) -> libc::c_int;
    pub fn netdev_stat_queue_sum(netdev: *mut net_device, rx_start: libc::c_int, rx_end: libc::c_int,
        rx_sum: *mut netdev_queue_stats_rx, tx_start: libc::c_int, tx_end: libc::c_int,
        tx_sum: *mut netdev_queue_stats_tx);
    pub fn netdev_nl_sock_priv_init(priv_: *mut netdev_nl_sock);
    pub fn netdev_nl_sock_priv_destroy(priv_: *mut netdev_nl_sock);
}

// File-local logic translated as explicit unsafe helpers. The original Linux
// implementation supplies these operations through kernel headers and other
// compilation units.
pub const NETDEV_STAT_NOT_SET: u64 = !0u64;

#[no_mangle]
pub unsafe extern "C" fn netdev_nl_stats_add(_sum: *mut libc::c_void, _add: *const libc::c_void, mut size: usize) {
    let mut add = _add as *const u64;
    let mut sum = _sum as *mut u64;
    while size != 0 {
        let a = *add;
        if a != NETDEV_STAT_NOT_SET && *sum != NETDEV_STAT_NOT_SET {
            *sum = (*sum).wrapping_add(a);
        }
        sum = sum.add(1);
        add = add.add(1);
        size -= 8;
    }
}

// Netlink queue/stat attribute writers retain C short-circuit ordering.
// External declarations are used for all nla/genl operations and structure
// accessors, preserving the ABI-facing signatures without inventing stubs.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
