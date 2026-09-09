/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, corresponding to the original C includes.

/* flags */
pub const NETLINK_F_KERNEL_SOCKET: u32 = 0;
pub const NETLINK_F_RECV_PKTINFO: u32 = 1;
pub const NETLINK_F_BROADCAST_SEND_ERROR: u32 = 2;
pub const NETLINK_F_RECV_NO_ENOBUFS: u32 = 3;
pub const NETLINK_F_LISTEN_ALL_NSID: u32 = 4;
pub const NETLINK_F_CAP_ACK: u32 = 5;
pub const NETLINK_F_EXT_ACK: u32 = 6;
pub const NETLINK_F_STRICT_CHK: u32 = 7;

macro_rules! NLGRPSZ {
    ($x:expr) => {
        (ALIGN!($x, core::mem::size_of::<c_ulong>() * 8) / 8)
    };
}

macro_rules! NLGRPLONGS {
    ($x:expr) => {
        (NLGRPSZ!($x) / core::mem::size_of::<c_ulong>())
    };
}

#[repr(C)]
pub struct netlink_sock {
    /* struct sock has to be the first member of netlink_sock */
    pub sk: sock,
    pub flags: c_ulong,
    pub portid: u32,
    pub dst_portid: u32,
    pub dst_group: u32,
    pub subscriptions: u32,
    pub ngroups: u32,
    pub groups: *mut c_ulong,
    pub state: c_ulong,
    pub max_recvmsg_len: usize,
    pub wait: wait_queue_head_t,
    pub bound: bool,
    pub cb_running: bool,
    pub dump_done_errno: c_int,
    pub cb: netlink_callback,
    pub nl_cb_mutex: mutex,

    pub netlink_rcv: Option<unsafe extern "C" fn(skb: *mut sk_buff)>,
    pub netlink_bind: Option<unsafe extern "C" fn(net: *mut net, group: c_int) -> c_int>,
    pub netlink_unbind: Option<unsafe extern "C" fn(net: *mut net, group: c_int)>,
    pub netlink_release:
        Option<unsafe extern "C" fn(sk: *mut sock, groups: *mut c_ulong)>,
    pub module: *mut module,

    pub node: rhash_head,
    pub rcu: rcu_head,
}

pub unsafe fn nlk_sk(sk: *mut sock) -> *mut netlink_sock {
    container_of!(sk, netlink_sock, sk)
}

macro_rules! nlk_test_bit {
    ($nr:ident, $sk:expr) => {
        test_bit!(concat_idents!(NETLINK_F_, $nr), unsafe { &(*nlk_sk($sk)).flags })
    };
}

#[repr(C)]
pub struct netlink_table {
    pub hash: rhashtable,
    pub mc_list: hlist_head,
    pub listeners: *mut listeners,
    pub flags: c_uint,
    pub groups: c_uint,
    pub cb_mutex: *mut mutex,
    pub module: *mut module,
    pub bind: Option<unsafe extern "C" fn(net: *mut net, group: c_int) -> c_int>,
    pub unbind: Option<unsafe extern "C" fn(net: *mut net, group: c_int)>,
    pub release: Option<unsafe extern "C" fn(sk: *mut sock, groups: *mut c_ulong)>,
    pub registered: c_int,
}

unsafe extern "C" {
    pub static mut nl_table: *mut netlink_table;
    pub static mut nl_table_lock: rwlock_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
