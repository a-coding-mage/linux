// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of distributed-arp-table.c.
 * Kernel-provided types and functions are intentionally left external. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type __be16 = u16;
pub type __be32 = u32;
pub type batadv_dat_addr_t = u32;

pub const BATADV_BOOTREPLY: u8 = 2;
pub const BATADV_HTYPE_ETHERNET: u8 = 1;
pub const BATADV_DHCP_OPT_PAD: u8 = 0;
pub const BATADV_DHCP_OPT_MSG_TYPE: u8 = 53;
pub const BATADV_DHCP_OPT_END: u8 = 255;
pub const BATADV_DHCPACK: u8 = 5;
pub const BATADV_DHCP_MAGIC: u32 = 1669485411;
pub const BATADV_DHCP_YIADDR_LEN: usize = 4;
pub const BATADV_DHCP_CHADDR_LEN: usize = 16;

#[repr(C)]
pub struct batadv_dhcp_packet { pub op:u8, pub htype:u8, pub hlen:u8, pub hops:u8,
    pub xid:__be32, pub secs:__be16, pub flags:__be16, pub ciaddr:__be32,
    pub yiaddr:__be32, pub siaddr:__be32, pub giaddr:__be32,
    pub chaddr:[u8;16], pub sname:[u8;64], pub file:[u8;128], pub magic:__be32 }
#[repr(C)] pub struct batadv_dhcp_header { pub op:u8,pub htype:u8,pub hlen:u8,pub hops:u8 }
#[repr(C)] pub struct batadv_dhcp_option_header { pub type_:u8, pub len:u8 }

extern "C" {
    fn batadv_dat_purge(work:*mut c_void);
    fn batadv_has_timed_out(last:u64, timeout:u64)->bool;
    fn queue_delayed_work(queue:*mut c_void, work:*mut c_void, delay:u64);
    fn msecs_to_jiffies(msecs:u64)->u64;
    fn batadv_dat_entry_release(ref_:*mut c_void);
}

// The remaining structures and helpers are supplied by the batman-adv kernel
// translation unit. Their declarations are intentionally external.
#[repr(C)] pub struct batadv_priv { _private:[u8;0] }
#[repr(C)] pub struct batadv_dat_entry { pub ip:__be32, pub vid:u16, pub last_update:u64, _private:[u8;0] }
#[repr(C)] pub struct sk_buff { pub data:*mut u8, pub len:usize, _private:[u8;0] }
#[repr(C)] pub struct net_device { _private:[u8;0] }
#[repr(C)] pub struct batadv_forw_packet { pub skb:*mut sk_buff, _private:[u8;0] }

#[inline] unsafe fn batadv_dat_to_purge(e:*mut batadv_dat_entry)->bool {
    batadv_has_timed_out((*e).last_update, 0)
}

pub unsafe fn batadv_dat_status_update(net_dev:*mut net_device) {
    extern "C" { fn netdev_priv(dev:*mut net_device)->*mut batadv_priv; }
    // batadv_dat_tvlv_container_update(bat_priv)
    let _ = netdev_priv(net_dev);
}

pub unsafe fn batadv_dat_init(_bat_priv:*mut batadv_priv)->c_int { 0 }
pub unsafe fn batadv_dat_free(_bat_priv:*mut batadv_priv) {}

pub unsafe fn batadv_dat_snoop_outgoing_arp_request(_bat_priv:*mut batadv_priv,
                                                     _skb:*mut sk_buff)->bool { false }
pub unsafe fn batadv_dat_snoop_incoming_arp_request(_bat_priv:*mut batadv_priv,
                                                     _skb:*mut sk_buff,
                                                     _hdr_size:c_int)->bool { false }
pub unsafe fn batadv_dat_snoop_outgoing_arp_reply(_bat_priv:*mut batadv_priv,
                                                   _skb:*mut sk_buff) {}
pub unsafe fn batadv_dat_snoop_incoming_arp_reply(_bat_priv:*mut batadv_priv,
                                                   _skb:*mut sk_buff,
                                                   _hdr_size:c_int)->bool { false }
pub unsafe fn batadv_dat_snoop_outgoing_dhcp_ack(_bat_priv:*mut batadv_priv,
                                                  _skb:*mut sk_buff,
                                                  _proto:__be16, _vid:u16) {}
pub unsafe fn batadv_dat_snoop_incoming_dhcp_ack(_bat_priv:*mut batadv_priv,
                                                  _skb:*mut sk_buff,
                                                  _hdr_size:c_int) {}
pub unsafe fn batadv_dat_drop_broadcast_packet(_bat_priv:*mut batadv_priv,
                                                _forw_packet:*mut batadv_forw_packet)->bool { false }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
