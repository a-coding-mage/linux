/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2011-2014 Autronica Fire and Security AS */
/* Rust translation of hsr_main.h. */

pub const HSR_LIFE_CHECK_INTERVAL: u32 = 2000;
pub const HSR_NODE_FORGET_TIME: u32 = 60000;
pub const HSR_PROXY_NODE_FORGET_TIME: u32 = 60000;
pub const HSR_ANNOUNCE_INTERVAL: u32 = 100;
pub const HSR_ENTRY_FORGET_TIME: u32 = 400;
pub const MAX_SLAVE_DIFF: u32 = 3000;
pub const HSR_SEQNR_START: u16 = u16::MAX - 1024;
pub const HSR_SUP_SEQNR_START: u16 = HSR_SEQNR_START / 2;
pub const PRUNE_PERIOD: u32 = 3000;
pub const PRUNE_PROXY_PERIOD: u32 = 3000;
pub const HSR_TLV_EOT: u8 = 0;
pub const HSR_TLV_ANNOUNCE: u8 = 22;
pub const HSR_TLV_LIFE_CHECK: u8 = 23;
pub const PRP_TLV_LIFE_CHECK_DD: u8 = 20;
pub const PRP_TLV_LIFE_CHECK_DA: u8 = 21;
pub const PRP_TLV_REDBOX_MAC: u8 = 30;
pub const HSR_V1_SUP_LSDUSIZE: u16 = 52;
pub const PRP_LAN_ID: u8 = 0x5;

/* Types and constants supplied by the surrounding kernel translation. */
pub const ETH_ALEN: usize = 6;

#[repr(C)]
pub struct hsr_ethhdr { pub ethhdr: ethhdr, pub hsr_tag: hsr_tag }
#[repr(C)]
pub struct hsr_vlan_ethhdr { pub vlanhdr: vlan_ethhdr, pub hsr_tag: hsr_tag }
#[repr(C)]
pub struct hsr_sup_tlv { pub HSR_TLV_type: u8, pub HSR_TLV_length: u8 }
#[repr(C)]
pub struct hsr_sup_tag { pub path_and_HSR_ver: u16, pub sequence_nr: u16, pub tlv: hsr_sup_tlv }
#[repr(C)]
pub struct hsr_sup_payload { pub macaddress_A: [u8; ETH_ALEN] }
#[repr(C)]
pub struct hsrv0_ethhdr_sp { pub ethhdr: ethhdr, pub hsr_sup: hsr_sup_tag }
#[repr(C)]
pub struct hsrv1_ethhdr_sp { pub ethhdr: ethhdr, pub hsr: hsr_tag, pub hsr_sup: hsr_sup_tag }
#[repr(C)]
pub struct prp_rct { pub sequence_nr: u16, pub lan_id_and_LSDU_size: u16, pub PRP_suffix: u16 }

#[repr(C)]
pub struct hsr_port {
    pub port_list: list_head,
    pub dev: *mut net_device,
    pub hsr: *mut hsr_priv,
    pub r#type: hsr_port_type,
    pub rcu: rcu_head,
    pub original_macaddress: [u8; ETH_ALEN],
}
pub struct hsr_frame_info;
pub struct hsr_node;

#[repr(C)]
pub struct hsr_proto_ops {
    pub send_sv_frame: Option<unsafe extern "C" fn(*mut hsr_port, *mut c_ulong, *const u8)>,
    pub handle_san_frame: Option<unsafe extern "C" fn(bool, hsr_port_type, *mut hsr_node)>,
    pub drop_frame: Option<unsafe extern "C" fn(*mut hsr_frame_info, *mut hsr_port) -> bool>,
    pub get_untagged_frame: Option<unsafe extern "C" fn(*mut hsr_frame_info, *mut hsr_port) -> *mut sk_buff>,
    pub create_tagged_frame: Option<unsafe extern "C" fn(*mut hsr_frame_info, *mut hsr_port) -> *mut sk_buff>,
    pub fill_frame_info: Option<unsafe extern "C" fn(u16, *mut sk_buff, *mut hsr_frame_info) -> i32>,
    pub invalid_dan_ingress_frame: Option<unsafe extern "C" fn(u16) -> bool>,
    pub update_san_info: Option<unsafe extern "C" fn(*mut hsr_node, bool)>,
    pub register_frame_out: Option<unsafe extern "C" fn(*mut hsr_port, *mut hsr_frame_info) -> i32>,
}

#[repr(C)]
pub struct hsr_self_node {
    pub macaddress_A: [u8; ETH_ALEN],
    pub macaddress_B: [u8; ETH_ALEN],
    pub rcu_head: rcu_head,
}

#[repr(C)]
pub struct hsr_priv {
    pub rcu_head: rcu_head,
    pub ports: list_head,
    pub node_db: list_head,
    pub proxy_node_db: list_head,
    pub self_node: *mut hsr_self_node,
    pub announce_timer: timer_list,
    pub announce_proxy_timer: timer_list,
    pub prune_timer: timer_list,
    pub prune_proxy_timer: timer_list,
    pub announce_count: i32,
    pub sequence_nr: u16,
    pub sup_sequence_nr: u16,
    pub prot_version: hsr_version,
    pub seqnr_lock: spinlock_t,
    pub list_lock: spinlock_t,
    pub proto_ops: *const hsr_proto_ops,
    pub net_id: u8,
    pub fwd_offloaded: bool,
    pub redbox: bool,
    pub macaddress_redbox: [u8; ETH_ALEN],
    pub sup_multicast_addr: [u8; ETH_ALEN],
}

pub unsafe fn set_hsr_tag_path(ht: *mut hsr_tag, path: u16) {
    (*ht).path_and_LSDU_size = htons((ntohs((*ht).path_and_LSDU_size) & 0x0fff) | (path << 12));
}
pub unsafe fn set_hsr_tag_LSDU_size(ht: *mut hsr_tag, LSDU_size: u16) {
    (*ht).path_and_LSDU_size = htons((ntohs((*ht).path_and_LSDU_size) & 0xf000) | (LSDU_size & 0x0fff));
}
pub unsafe fn set_hsr_stag_path(hst: *mut hsr_sup_tag, path: u16) { set_hsr_tag_path(hst as *mut hsr_tag, path); }
pub unsafe fn set_hsr_stag_HSR_ver(hst: *mut hsr_sup_tag, HSR_ver: u16) { set_hsr_tag_LSDU_size(hst as *mut hsr_tag, HSR_ver); }
pub unsafe fn get_prp_LSDU_size(rct: *mut prp_rct) -> u16 { ntohs((*rct).lan_id_and_LSDU_size) & 0x0fff }
pub unsafe fn set_prp_lan_id(rct: *mut prp_rct, lan_id: u16) { (*rct).lan_id_and_LSDU_size = htons((ntohs((*rct).lan_id_and_LSDU_size) & 0x0fff) | (lan_id << 12)); }
pub unsafe fn set_prp_LSDU_size(rct: *mut prp_rct, LSDU_size: u16) { (*rct).lan_id_and_LSDU_size = htons((ntohs((*rct).lan_id_and_LSDU_size) & 0xf000) | (LSDU_size & 0x0fff)); }

pub unsafe extern "C" { pub fn hsr_port_get_hsr(hsr: *mut hsr_priv, pt: hsr_port_type) -> *mut hsr_port; }

pub unsafe fn hsr_get_skb_sequence_nr(skb: *mut sk_buff) -> u16 {
    let h = skb_mac_header(skb) as *mut hsr_ethhdr;
    ntohs((*h).hsr_tag.sequence_nr)
}
pub unsafe fn skb_get_PRP_rct(skb: *mut sk_buff) -> *mut prp_rct {
    let tail = skb_tail_pointer(skb).offset(-(HSR_HLEN as isize)) as *mut prp_rct;
    if (*tail).PRP_suffix == htons(ETH_P_PRP) { tail } else { core::ptr::null_mut() }
}
pub unsafe fn prp_get_skb_sequence_nr(rct: *mut prp_rct) -> u16 { ntohs((*rct).sequence_nr) }
pub unsafe fn prp_check_lsdu_size(skb: *mut sk_buff, rct: *mut prp_rct, is_sup: bool) -> bool {
    let expected_lsdu_size: i32;
    if is_sup {
        expected_lsdu_size = HSR_V1_SUP_LSDUSIZE as i32;
    } else {
        let eth = skb_mac_header(skb) as *mut ethhdr;
        expected_lsdu_size = (*skb).len as i32 - 14 - if (*eth).h_proto == htons(ETH_P_8021Q) { 4 } else { 0 };
    }
    expected_lsdu_size == get_prp_LSDU_size(rct) as i32
}

/* CONFIG_DEBUG_FS controls whether these are external functions or empty inline functions. */
#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe extern "C" {
    pub fn hsr_debugfs_rename(dev: *mut net_device);
    pub fn hsr_debugfs_init(priv_: *mut hsr_priv, hsr_dev: *mut net_device);
    pub fn hsr_debugfs_term(priv_: *mut hsr_priv);
    pub fn hsr_debugfs_create_root();
    pub fn hsr_debugfs_remove_root();
}
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn hsr_debugfs_rename(_dev: *mut net_device) {}
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn hsr_debugfs_init(_priv_: *mut hsr_priv, _hsr_dev: *mut net_device) {}
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn hsr_debugfs_term(_priv_: *mut hsr_priv) {}
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn hsr_debugfs_create_root() {}
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn hsr_debugfs_remove_root() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
