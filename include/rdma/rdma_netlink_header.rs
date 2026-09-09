/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// <linux/netlink.h>, <uapi/rdma/rdma_netlink.h>, and <rdma/ib_verbs.h>

use core::ffi::c_void;

pub struct ib_device;
pub struct sk_buff;
pub struct nlmsghdr;
pub struct netlink_ext_ack;
pub struct netlink_callback;
pub struct net;
pub struct net_device;
pub struct list_head;
pub type gfp_t = u32;

#[repr(u32)]
pub enum RdmANlDevAttr {
    RDMA_NLDEV_ATTR_EMPTY_STRING = 1,
    RDMA_NLDEV_ATTR_ENTRY_STRLEN = 16,
    RDMA_NLDEV_ATTR_CHARDEV_TYPE_SIZE = 32,
}

#[repr(C)]
pub struct rdma_nl_cbs {
    pub doit: Option<unsafe extern "C" fn(
        skb: *mut sk_buff,
        nlh: *mut nlmsghdr,
        extack: *mut netlink_ext_ack,
    ) -> i32>,
    pub dump: Option<unsafe extern "C" fn(
        skb: *mut sk_buff,
        nlcb: *mut netlink_callback,
    ) -> i32>,
    pub flags: u8,
}

#[repr(u32)]
pub enum rdma_nl_flags {
    // Require CAP_NET_ADMIN
    RDMA_NL_ADMIN_PERM = 1 << 0,
}

// Define this module as providing netlink services for NETLINK_RDMA, with
// index _index. The original macro also performs a build-time equality check
// and emits a kernel module alias.
#[macro_export]
macro_rules! MODULE_ALIAS_RDMA_NETLINK {
    ($index:ident, $val:expr) => {
        const _: () = {
            assert!($index == $val);
        };
    };
}

// Register client in RDMA netlink.
pub unsafe extern "C" fn rdma_nl_register(
    index: u32,
    cb_table: *const rdma_nl_cbs,
);

// Remove a client from IB netlink.
pub unsafe extern "C" fn rdma_nl_unregister(index: u32);

// Put a new message in a supplied skb.
pub unsafe extern "C" fn ibnl_put_msg(
    skb: *mut sk_buff,
    nlh: *mut *mut nlmsghdr,
    seq: i32,
    len: i32,
    client: i32,
    op: i32,
    flags: i32,
) -> *mut c_void;

// Put a new attribute in a supplied skb.
pub unsafe extern "C" fn ibnl_put_attr(
    skb: *mut sk_buff,
    nlh: *mut nlmsghdr,
    len: i32,
    data: *mut c_void,
    type_: i32,
) -> i32;

// Send the supplied skb to a specific userspace PID.
pub unsafe extern "C" fn rdma_nl_unicast(
    net: *mut net,
    skb: *mut sk_buff,
    pid: u32,
) -> i32;

// Send, with wait/1 retry, the supplied skb to a specific userspace PID.
pub unsafe extern "C" fn rdma_nl_unicast_wait(
    net: *mut net,
    skb: *mut sk_buff,
    pid: u32,
) -> i32;

// Send the supplied skb to a netlink group.
pub unsafe extern "C" fn rdma_nl_multicast(
    net: *mut net,
    skb: *mut sk_buff,
    group: u32,
    flags: gfp_t,
) -> i32;

// Check if there are any listeners to the netlink group.
pub unsafe extern "C" fn rdma_nl_chk_listeners(group: u32) -> bool;

// The event type is supplied by <uapi/rdma/rdma_netlink.h>.
pub type rdma_nl_notify_event_type = u32;

// Prepare and send an event message.
pub unsafe extern "C" fn rdma_nl_notify_event(
    ib: *mut ib_device,
    port_num: u32,
    type_: rdma_nl_notify_event_type,
) -> i32;

#[repr(C)]
pub struct rdma_link_ops {
    pub list: list_head,
    pub type_: *const core::ffi::c_char,
    pub newlink: Option<unsafe extern "C" fn(
        ibdev_name: *const core::ffi::c_char,
        ndev: *mut net_device,
    ) -> i32>,
    pub dellink: Option<unsafe extern "C" fn(dev: *mut ib_device) -> i32>,
}

pub unsafe extern "C" fn rdma_link_register(ops: *mut rdma_link_ops);
pub unsafe extern "C" fn rdma_link_unregister(ops: *mut rdma_link_ops);

// MODULE_ALIAS_RDMA_LINK(type) expands to MODULE_ALIAS("rdma-link-" type).
#[macro_export]
macro_rules! MODULE_ALIAS_RDMA_LINK {
    ($type_:expr) => {};
}

// MODULE_ALIAS_RDMA_CLIENT(type) expands to MODULE_ALIAS("rdma-client-" type).
#[macro_export]
macro_rules! MODULE_ALIAS_RDMA_CLIENT {
    ($type_:expr) => {};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
