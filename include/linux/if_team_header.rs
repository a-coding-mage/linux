/* SPDX-License-Identifier: GPL-2.0-or-later */
/* include/linux/if_team.h - Network team device driver header */

/* C dependencies supplied by other headers are intentionally external. */

use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)]
pub struct team_pcpu_stats {
    pub rx_packets: u64_stats_t,
    pub rx_bytes: u64_stats_t,
    pub rx_multicast: u64_stats_t,
    pub tx_packets: u64_stats_t,
    pub tx_bytes: u64_stats_t,
    pub syncp: u64_stats_sync,
    pub rx_dropped: u32,
    pub tx_dropped: u32,
    pub rx_nohandler: u32,
}

#[repr(C)] pub struct team;
#[repr(C)] pub struct net_device;
#[repr(C)] pub struct netpoll;
#[repr(C)] pub struct sk_buff;
#[repr(C)] pub struct module;
#[repr(C)] pub struct header_ops;
#[repr(C)] pub struct delayed_work;
#[repr(C)] pub struct u64_stats_sync;
#[repr(C)] pub struct u64_stats_t;
#[repr(C)] pub struct hlist_node;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct hlist_head;
#[repr(C)] pub struct rcu_head;
#[repr(C)] pub struct atomic_t;

#[repr(C)]
pub struct team_port {
    pub dev: *mut net_device,
    pub tx_hlist: hlist_node,
    pub list: list_head,
    pub team: *mut team,
    pub tx_index: c_int,
    pub rx_enabled: bool,
    pub linkup: bool,
    pub state: team_port_state,
    pub user: team_port_user,
    pub changed: bool,
    pub removed: bool,
    pub orig: team_port_orig,
    #[cfg(CONFIG_NET_POLL_CONTROLLER)] pub np: *mut netpoll,
    pub priority: i32,
    pub queue_id: u16,
    pub qom_list: list_head,
    pub rcu: rcu_head,
    pub mode_priv: [c_long; 0],
}

#[repr(C)] pub struct team_port_state { pub linkup: bool, pub speed: u32, pub duplex: u8 }
#[repr(C)] pub struct team_port_user { pub linkup: bool, pub linkup_enabled: bool }
#[repr(C)] pub struct team_port_orig { pub dev_addr: [u8; MAX_ADDR_LEN], pub mtu: u32 }

extern "C" {
    pub fn rcu_dereference(p: *const c_void) -> *mut c_void;
    pub fn READ_ONCE<T: Copy>(p: *const T) -> T;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn netpoll_send_skb(np: *mut netpoll, skb: *mut sk_buff);
}

pub unsafe fn team_port_get_rcu(dev: *const net_device) -> *mut team_port {
    rcu_dereference(dev as *const c_void) as *mut team_port
}
pub unsafe fn team_port_rx_enabled(port: *mut team_port) -> bool { READ_ONCE(&(*port).rx_enabled) }
pub unsafe fn team_port_tx_enabled(port: *mut team_port) -> bool { READ_ONCE(&(*port).tx_index) != -1 }
pub unsafe fn team_port_enabled(port: *mut team_port) -> bool { team_port_rx_enabled(port) && team_port_tx_enabled(port) }
pub unsafe fn team_port_txable(port: *mut team_port) -> bool { (*port).linkup && team_port_tx_enabled(port) }
pub unsafe fn team_port_dev_txable(dev: *const net_device) -> bool {
    rcu_read_lock(); let port = team_port_get_rcu(dev);
    let txable = !port.is_null() && team_port_txable(port); rcu_read_unlock(); txable
}

#[repr(C)] pub struct team_mode_ops {
    pub init: Option<unsafe extern "C" fn(*mut team) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut team)>,
    pub receive: Option<unsafe extern "C" fn(*mut team, *mut team_port, *mut sk_buff) -> rx_handler_result_t>,
    pub transmit: Option<unsafe extern "C" fn(*mut team, *mut sk_buff) -> bool>,
    pub port_enter: Option<unsafe extern "C" fn(*mut team, *mut team_port) -> c_int>,
    pub port_leave: Option<unsafe extern "C" fn(*mut team, *mut team_port)>,
    pub port_change_dev_addr: Option<unsafe extern "C" fn(*mut team, *mut team_port)>,
    pub port_tx_disabled: Option<unsafe extern "C" fn(*mut team, *mut team_port)>,
}
pub type rx_handler_result_t = c_int;
extern "C" { pub fn team_modeop_port_enter(*mut team, *mut team_port) -> c_int; pub fn team_modeop_port_change_dev_addr(*mut team, *mut team_port); }

#[repr(C)] pub enum team_option_type { TEAM_OPTION_TYPE_U32, TEAM_OPTION_TYPE_STRING, TEAM_OPTION_TYPE_BINARY, TEAM_OPTION_TYPE_BOOL, TEAM_OPTION_TYPE_S32 }
#[repr(C)] pub struct team_option_inst_info { pub array_index: u32, pub port: *mut team_port }
#[repr(C)] pub union team_gsetter_data { pub u32_val: u32, pub str_val: *const c_char, pub bin_val: team_binary_val, pub bool_val: bool, pub s32_val: i32 }
#[repr(C)] pub struct team_binary_val { pub ptr: *const c_void, pub len: u32 }
#[repr(C)] pub struct team_gsetter_ctx { pub data: team_gsetter_data, pub info: *mut team_option_inst_info }
#[repr(C)] pub struct team_option {
    pub list: list_head, pub name: *const c_char, pub per_port: bool, pub array_size: u32, pub type_: team_option_type,
    pub init: Option<unsafe extern "C" fn(*mut team, *mut team_option_inst_info)>,
    pub getter: Option<unsafe extern "C" fn(*mut team, *mut team_gsetter_ctx)>,
    pub setter: Option<unsafe extern "C" fn(*mut team, *mut team_gsetter_ctx) -> c_int>,
}
extern "C" { pub fn team_option_inst_set_change(*mut team_option_inst_info); pub fn team_options_change_check(*mut team); }

#[repr(C)] pub struct team_mode { pub kind: *const c_char, pub owner: *mut module, pub priv_size: usize, pub port_priv_size: usize, pub ops: *const team_mode_ops, pub lag_tx_type: c_int }
pub const TEAM_PORT_HASHBITS: usize = 4;
pub const TEAM_PORT_HASHENTRIES: usize = 1 << TEAM_PORT_HASHBITS;
pub const TEAM_MODE_PRIV_LONGS: usize = 4;
pub const TEAM_DEFAULT_NUM_TX_QUEUES: usize = 16;
pub const TEAM_DEFAULT_NUM_RX_QUEUES: usize = 16;

#[repr(C)] pub struct team {
    pub pcpu_stats: *mut team_pcpu_stats, pub header_ops_cache: *const header_ops,
    pub tx_en_port_count: c_int, pub rx_en_port_count: c_int,
    pub tx_en_port_hlist: [hlist_head; TEAM_PORT_HASHENTRIES], pub port_list: list_head,
    pub option_list: list_head, pub option_inst_list: list_head, pub mode: *const team_mode,
    pub ops: team_mode_ops, pub user_carrier_enabled: bool, pub queue_override_enabled: bool,
    pub qom_lists: *mut list_head, pub port_mtu_change_allowed: bool, pub notifier_ctx: bool,
    pub notify_peers: team_timer, pub mcast_rejoin: team_timer, pub mode_priv: [c_long; TEAM_MODE_PRIV_LONGS],
}
#[repr(C)] pub struct team_timer { pub count: u32, pub interval: u32, pub count_pending: atomic_t, pub dw: delayed_work }

extern "C" { pub fn team_options_register(*mut team, *const team_option, usize) -> c_int; pub fn team_options_unregister(*mut team, *const team_option, usize); pub fn team_mode_register(*const team_mode) -> c_int; pub fn team_mode_unregister(*const team_mode); }
extern "C" { pub fn dev_queue_xmit(*mut sk_buff) -> c_int; }
// Preprocessor configuration, kernel list-iteration helpers, and skb/netpoll helpers remain external.
pub const TEAM_MODE_PRIV_SIZE: usize = core::mem::size_of::<c_long>() * TEAM_MODE_PRIV_LONGS;

pub unsafe fn team_netpoll_send_skb(port: *mut team_port, skb: *mut sk_buff) {
    #[cfg(CONFIG_NET_POLL_CONTROLLER)] { netpoll_send_skb((*port).np, skb); }
    #[cfg(not(CONFIG_NET_POLL_CONTROLLER))] { let _ = (port, skb); }
}

// The following operations use kernel-provided skb, qdisc, RCU, and hlist primitives.
extern "C" {
    pub fn skb_set_queue_mapping(skb: *mut sk_buff, mapping: u16);
    pub fn qdisc_skb_cb(skb: *mut sk_buff) -> *mut qdisc_skb_cb_t;
    pub fn netpoll_tx_running(dev: *mut net_device) -> bool;
    pub fn netdev_from_priv(team: *mut team) -> *mut net_device;
}
#[repr(C)] pub struct qdisc_skb_cb_t { pub slave_dev_queue_mapping: u16 }

pub unsafe fn team_dev_queue_xmit(team: *mut team, port: *mut team_port, skb: *mut sk_buff) -> c_int {
    let mapping = (*qdisc_skb_cb(skb)).slave_dev_queue_mapping;
    skb_set_queue_mapping(skb, mapping);
    // skb->dev = port->dev; field access is supplied by the kernel skb definition.
    if netpoll_tx_running(netdev_from_priv(team)) { team_netpoll_send_skb(port, skb); return 0; }
    dev_queue_xmit(skb)
}

pub unsafe fn team_tx_port_index_hash(team: *mut team, tx_port_index: c_int) -> *mut hlist_head {
    &mut (*team).tx_en_port_hlist[(tx_port_index as usize) & (TEAM_PORT_HASHENTRIES - 1)]
}

// hlist_for_each_entry[_rcu] and list_for_each_entry[_continue]_rcu are kernel macros;
// their declaration-level control flow is preserved here through external helpers.
extern "C" {
    pub fn team_get_port_by_tx_index_impl(*mut team, c_int) -> *mut team_port;
    pub fn team_get_port_by_tx_index_rcu_impl(*mut team, c_int) -> *mut team_port;
    pub fn team_get_first_port_txable_rcu_impl(*mut team, *mut team_port) -> *mut team_port;
}
pub unsafe fn team_get_port_by_tx_index(team: *mut team, index: c_int) -> *mut team_port { team_get_port_by_tx_index_impl(team, index) }
pub unsafe fn team_get_port_by_tx_index_rcu(team: *mut team, index: c_int) -> *mut team_port { team_get_port_by_tx_index_rcu_impl(team, index) }
pub unsafe fn team_get_first_port_txable_rcu(team: *mut team, port: *mut team_port) -> *mut team_port { team_get_first_port_txable_rcu_impl(team, port) }

pub unsafe fn team_num_to_port_index(team: *mut team, num: u32) -> c_int {
    let count = READ_ONCE(&(*team).tx_en_port_count);
    if count == 0 { 0 } else { (num % count as u32) as c_int }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
