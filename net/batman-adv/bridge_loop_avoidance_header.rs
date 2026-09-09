/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Simon Wunderlich
 */

// Translated from bridge_loop_avoidance.h.  The declarations below depend on
// types and constants supplied by the surrounding batman-adv code.

/// batadv_bla_is_loopdetect_mac() - check if the mac address is from a loop
/// detect frame sent by bridge loop avoidance
/// @mac: mac address to check
///
/// Return: true if it looks like a loop detect frame
/// (mac starts with BA:BE), false otherwise
#[inline]
pub unsafe fn batadv_bla_is_loopdetect_mac(mac: *const u8) -> bool {
    if *mac.add(0) == 0xba && *mac.add(1) == 0xbe {
        return true;
    }

    false
}

#[cfg(feature = "CONFIG_BATMAN_ADV_BLA")]
extern "C" {
    pub fn batadv_bla_rx(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
        vid: u16,
        packet_type: i32,
    ) -> bool;
    pub fn batadv_bla_tx(bat_priv: *mut batadv_priv, skb: *mut sk_buff, vid: u16) -> bool;
    pub fn batadv_bla_is_backbone_gw(
        skb: *mut sk_buff,
        orig_node: *mut batadv_orig_node,
        hdr_size: i32,
    ) -> bool;
    pub fn batadv_bla_claim_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn batadv_bla_backbone_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn batadv_bla_is_backbone_gw_orig(
        bat_priv: *mut batadv_priv,
        orig: *mut u8,
        vid: u16,
    ) -> bool;
    pub fn batadv_bla_check_bcast_duplist(
        bat_priv: *mut batadv_priv,
        skb: *mut sk_buff,
    ) -> bool;
    pub fn batadv_bla_update_orig_address(
        bat_priv: *mut batadv_priv,
        primary_if: *mut batadv_hard_iface,
        oldif: *mut batadv_hard_iface,
    );
    pub fn batadv_bla_status_update(net_dev: *mut net_device);
    pub fn batadv_bla_init(bat_priv: *mut batadv_priv) -> i32;
    pub fn batadv_bla_free(bat_priv: *mut batadv_priv);

    #[cfg(feature = "CONFIG_BATMAN_ADV_DAT")]
    pub fn batadv_bla_check_claim(
        bat_priv: *mut batadv_priv,
        addr: *mut u8,
        vid: u16,
    ) -> bool;
}

#[cfg(feature = "CONFIG_BATMAN_ADV_BLA")]
pub const BATADV_BLA_CRC_INIT: i32 = 0;

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BLA"))]
#[inline]
pub unsafe fn batadv_bla_rx(
    _bat_priv: *mut batadv_priv,
    _skb: *mut sk_buff,
    _vid: u16,
    _packet_type: i32,
) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BLA"))]
#[inline]
pub unsafe fn batadv_bla_tx(
    _bat_priv: *mut batadv_priv,
    _skb: *mut sk_buff,
    _vid: u16,
) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BLA"))]
#[inline]
pub unsafe fn batadv_bla_is_backbone_gw(
    _skb: *mut sk_buff,
    _orig_node: *mut batadv_orig_node,
    _hdr_size: i32,
) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BLA"))]
#[inline]
pub unsafe fn batadv_bla_is_backbone_gw_orig(
    _bat_priv: *mut batadv_priv,
    _orig: *mut u8,
    _vid: u16,
) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BLA"))]
#[inline]
pub unsafe fn batadv_bla_check_bcast_duplist(
    _bat_priv: *mut batadv_priv,
    _skb: *mut sk_buff,
) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BLA"))]
#[inline]
pub unsafe fn batadv_bla_update_orig_address(
    _bat_priv: *mut batadv_priv,
    _primary_if: *mut batadv_hard_iface,
    _oldif: *mut batadv_hard_iface,
) {
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BLA"))]
#[inline]
pub unsafe fn batadv_bla_init(_bat_priv: *mut batadv_priv) -> i32 {
    1
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BLA"))]
#[inline]
pub unsafe fn batadv_bla_free(_bat_priv: *mut batadv_priv) {
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BLA"))]
#[inline]
pub unsafe fn batadv_bla_claim_dump(
    _msg: *mut sk_buff,
    _cb: *mut netlink_callback,
) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BLA"))]
#[inline]
pub unsafe fn batadv_bla_backbone_dump(
    _msg: *mut sk_buff,
    _cb: *mut netlink_callback,
) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_BATMAN_ADV_BLA"))]
#[inline]
pub unsafe fn batadv_bla_check_claim(
    _bat_priv: *mut batadv_priv,
    _addr: *mut u8,
    _vid: u16,
) -> bool {
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
