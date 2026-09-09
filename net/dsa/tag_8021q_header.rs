/* SPDX-License-Identifier: GPL-2.0-or-later */

// Declarations translated from dsa/tag_8021q.h.
// External types are supplied by the corresponding kernel/D S A interfaces.

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsa_switch {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsa_notifier_tag_8021q_vlan_info {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dsa_8021q_xmit(
        skb: *mut sk_buff,
        netdev: *mut net_device,
        tpid: u16,
        tci: u16,
    ) -> *mut sk_buff;

    pub fn dsa_8021q_rcv(
        skb: *mut sk_buff,
        source_port: *mut i32,
        switch_id: *mut i32,
        vbid: *mut i32,
        vid: *mut i32,
    );

    pub fn dsa_tag_8021q_find_user(
        conduit: *mut net_device,
        source_port: i32,
        switch_id: i32,
        vid: i32,
        vbid: i32,
    ) -> *mut net_device;

    pub fn dsa_switch_tag_8021q_vlan_add(
        ds: *mut dsa_switch,
        info: *mut dsa_notifier_tag_8021q_vlan_info,
    ) -> i32;

    pub fn dsa_switch_tag_8021q_vlan_del(
        ds: *mut dsa_switch,
        info: *mut dsa_notifier_tag_8021q_vlan_info,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
