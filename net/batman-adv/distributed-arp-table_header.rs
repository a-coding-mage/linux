/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Antonio Quartulli
 */

// C dependencies: main.h, linux/compiler.h, linux/netdevice.h,
// linux/netlink.h, linux/skbuff.h, linux/types.h, uapi/linux/batadv_packet.h,
// and originator.h.

#[cfg(feature = "CONFIG_BATMAN_ADV_DAT")]
pub const BATADV_DAT_ADDR_MAX: batadv_dat_addr_t = !0 as batadv_dat_addr_t;

#[cfg(feature = "CONFIG_BATMAN_ADV_DAT")]
extern "C" {
    pub fn batadv_dat_status_update(net_dev: *mut net_device);
    pub fn batadv_dat_snoop_outgoing_arp_request(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
    ) -> bool;
    pub fn batadv_dat_snoop_incoming_arp_request(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        hdr_size: i32,
    ) -> bool;
    pub fn batadv_dat_snoop_outgoing_arp_reply(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
    );
    pub fn batadv_dat_snoop_incoming_arp_reply(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        hdr_size: i32,
    ) -> bool;
    pub fn batadv_dat_snoop_outgoing_dhcp_ack(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        proto: __be16,
        vid: u16,
    );
    pub fn batadv_dat_snoop_incoming_dhcp_ack(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        hdr_size: i32,
    );
    pub fn batadv_dat_drop_broadcast_packet(
        bat_priv: *mut batadv_priv,
        forw_packet: *mut batadv_forw_packet,
    ) -> bool;

    pub fn batadv_dat_init(bat_priv: *mut batadv_priv) -> i32;
    pub fn batadv_dat_free(bat_priv: *mut batadv_priv);
    pub fn batadv_dat_cache_dump(
        msg: *mut sk_buff,
        cb: *mut netlink_callback,
    ) -> i32;
}

#[cfg(feature = "CONFIG_BATMAN_ADV_DAT")]
pub unsafe fn batadv_dat_init_orig_node_addr(orig_node: *mut batadv_orig_node) {
    let addr: u32 = batadv_choose_orig((*orig_node).orig, BATADV_DAT_ADDR_MAX);
    (*orig_node).dat_addr = addr as batadv_dat_addr_t;
}

#[cfg(feature = "CONFIG_BATMAN_ADV_DAT")]
pub unsafe fn batadv_dat_init_own_addr(
    bat_priv: *mut batadv_priv,
    primary_if: *mut batadv_hard_iface,
) {
    let addr: u32 = batadv_choose_orig(
        (*(*primary_if).net_dev).dev_addr,
        BATADV_DAT_ADDR_MAX,
    );
    (*bat_priv).dat.addr = addr as batadv_dat_addr_t;
}

#[cfg(feature = "CONFIG_BATMAN_ADV_DAT")]
pub unsafe fn batadv_dat_inc_counter(bat_priv: *mut batadv_priv, subtype: u8) {
    match subtype {
        BATADV_P_DAT_DHT_GET => {
            batadv_inc_counter(bat_priv, BATADV_CNT_DAT_GET_RX);
        }
        BATADV_P_DAT_DHT_PUT => {
            batadv_inc_counter(bat_priv, BATADV_CNT_DAT_PUT_RX);
        }
        _ => {}
    }
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_status_update(_net_dev: *mut net_device) {}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_snoop_outgoing_arp_request(
    _bat_priv: *mut batadv_priv,
    _skb: *mut sk_buff,
) -> bool { false }

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_snoop_incoming_arp_request(
    _bat_priv: *mut batadv_priv,
    _skb: *mut sk_buff,
    _hdr_size: i32,
) -> bool { false }

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_snoop_outgoing_arp_reply(
    _bat_priv: *mut batadv_priv,
    _skb: *mut sk_buff,
) -> bool { false }

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_snoop_incoming_arp_reply(
    _bat_priv: *mut batadv_priv,
    _skb: *mut sk_buff,
    _hdr_size: i32,
) -> bool { false }

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_snoop_outgoing_dhcp_ack(
    _bat_priv: *mut batadv_priv,
    _skb: *mut sk_buff,
    _proto: __be16,
    _vid: u16,
) {}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_snoop_incoming_dhcp_ack(
    _bat_priv: *mut batadv_priv,
    _skb: *mut sk_buff,
    _hdr_size: i32,
) {}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_drop_broadcast_packet(
    _bat_priv: *mut batadv_priv,
    _forw_packet: *mut batadv_forw_packet,
) -> bool { false }

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_init_orig_node_addr(_orig_node: *mut batadv_orig_node) {}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_init_own_addr(
    _bat_priv: *mut batadv_priv,
    _iface: *mut batadv_hard_iface,
) {}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_init(_bat_priv: *mut batadv_priv) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_free(_bat_priv: *mut batadv_priv) {}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_cache_dump(
    _msg: *mut sk_buff,
    _cb: *mut netlink_callback,
) -> i32 { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_BATMAN_ADV_DAT"))]
pub unsafe fn batadv_dat_inc_counter(
    _bat_priv: *mut batadv_priv,
    _subtype: u8,
) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
