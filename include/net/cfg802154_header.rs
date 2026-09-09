/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2007, 2008, 2009 Siemens AG
 *
 * Written by:
 * Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
 */

// Translated from net/cfg802154.h. Kernel dependencies are supplied externally.

pub struct wpan_phy;
pub struct wpan_phy_cca;
pub struct cfg802154_scan_request;
pub struct cfg802154_beacon_request;
pub struct ieee802154_addr;

#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub struct ieee802154_llsec_device_key;
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub struct ieee802154_llsec_seclevel;
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub struct ieee802154_llsec_params;
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub struct ieee802154_llsec_device;
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub struct ieee802154_llsec_table;
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub struct ieee802154_llsec_key_id;
#[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
pub struct ieee802154_llsec_key;

#[repr(C)]
pub struct cfg802154_ops {
    pub add_virtual_intf_deprecated: Option<unsafe extern "C" fn(*mut wpan_phy, *const core::ffi::c_char, u8, i32) -> *mut net_device>,
    pub del_virtual_intf_deprecated: Option<unsafe extern "C" fn(*mut wpan_phy, *mut net_device)>,
    pub suspend: Option<unsafe extern "C" fn(*mut wpan_phy) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut wpan_phy) -> i32>,
    pub add_virtual_intf: Option<unsafe extern "C" fn(*mut wpan_phy, *const core::ffi::c_char, u8, nl802154_iftype, __le64) -> i32>,
    pub del_virtual_intf: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev) -> i32>,
    pub set_channel: Option<unsafe extern "C" fn(*mut wpan_phy, u8, u8) -> i32>,
    pub set_cca_mode: Option<unsafe extern "C" fn(*mut wpan_phy, *const wpan_phy_cca) -> i32>,
    pub set_cca_ed_level: Option<unsafe extern "C" fn(*mut wpan_phy, i32) -> i32>,
    pub set_tx_power: Option<unsafe extern "C" fn(*mut wpan_phy, i32) -> i32>,
    pub set_pan_id: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, __le16) -> i32>,
    pub set_short_addr: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, __le16) -> i32>,
    pub set_backoff_exponent: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, u8, u8) -> i32>,
    pub set_max_csma_backoffs: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, u8) -> i32>,
    pub set_max_frame_retries: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, i8) -> i32>,
    pub set_lbt_mode: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, bool) -> i32>,
    pub set_ackreq_default: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, bool) -> i32>,
    pub trigger_scan: Option<unsafe extern "C" fn(*mut wpan_phy, *mut cfg802154_scan_request) -> i32>,
    pub abort_scan: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev) -> i32>,
    pub send_beacons: Option<unsafe extern "C" fn(*mut wpan_phy, *mut cfg802154_beacon_request) -> i32>,
    pub stop_beacons: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev) -> i32>,
    pub associate: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, *mut ieee802154_addr) -> i32>,
    pub disassociate: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, *mut ieee802154_addr) -> i32>,
    #[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
    pub get_llsec_table: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, *mut *mut ieee802154_llsec_table)>,
    #[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
    pub lock_llsec_table: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev)>,
    #[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
    pub unlock_llsec_table: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev)>,
    // TODO remove locking/get table callbacks; this is part of the nl802154 interface.
    #[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
    pub get_llsec_params: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, *mut ieee802154_llsec_params) -> i32>,
    #[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
    pub set_llsec_params: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, *const ieee802154_llsec_params, i32) -> i32>,
    #[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
    pub add_llsec_key: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, *const ieee802154_llsec_key_id, *const ieee802154_llsec_key) -> i32>,
    #[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
    pub del_llsec_key: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, *const ieee802154_llsec_key_id) -> i32>,
    #[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
    pub add_seclevel: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, *const ieee802154_llsec_seclevel) -> i32>,
    #[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
    pub del_seclevel: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, *const ieee802154_llsec_seclevel) -> i32>,
    #[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
    pub add_device: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, *const ieee802154_llsec_device) -> i32>,
    #[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
    pub del_device: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, __le64) -> i32>,
    #[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
    pub add_devkey: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, __le64, *const ieee802154_llsec_device_key) -> i32>,
    #[cfg(feature = "CONFIG_IEEE802154_NL802154_EXPERIMENTAL")]
    pub del_devkey: Option<unsafe extern "C" fn(*mut wpan_phy, *mut wpan_dev, __le64, *const ieee802154_llsec_device_key) -> i32>,
}

pub unsafe fn wpan_phy_supported_bool(b: bool, st: nl802154_supported_bool_states) -> bool {
    match st {
        NL802154_SUPPORTED_BOOL_TRUE => b,
        NL802154_SUPPORTED_BOOL_FALSE => !b,
        NL802154_SUPPORTED_BOOL_BOTH => true,
        _ => { /* WARN_ON(1) */ false }
    }
}

#[repr(C)]
pub struct wpan_phy_supported {
    pub channels: [u32; IEEE802154_MAX_PAGE as usize + 1],
    pub cca_modes: u32,
    pub cca_opts: u32,
    pub iftypes: u32,
    pub lbt: nl802154_supported_bool_states,
    pub min_minbe: u8,
    pub max_minbe: u8,
    pub min_maxbe: u8,
    pub max_maxbe: u8,
    pub min_csma_backoffs: u8,
    pub max_csma_backoffs: u8,
    pub min_frame_retries: i8,
    pub max_frame_retries: i8,
    pub tx_powers_size: usize,
    pub cca_ed_levels_size: usize,
    pub tx_powers: *const i32,
    pub cca_ed_levels: *const i32,
}

#[repr(C)]
pub struct wpan_phy_cca { pub mode: nl802154_cca_modes, pub opt: nl802154_cca_opts }

pub unsafe fn wpan_phy_cca_cmp(a: *const wpan_phy_cca, b: *const wpan_phy_cca) -> bool {
    if (*a).mode != (*b).mode { return false; }
    if (*a).mode == NL802154_CCA_ENERGY_CARRIER { return (*a).opt == (*b).opt; }
    true
}

#[repr(u32)]
pub enum wpan_phy_flags {
    WPAN_PHY_FLAG_TXPOWER = BIT(1),
    WPAN_PHY_FLAG_CCA_ED_LEVEL = BIT(2),
    WPAN_PHY_FLAG_CCA_MODE = BIT(3),
    WPAN_PHY_FLAG_STATE_QUEUE_STOPPED = BIT(4),
    WPAN_PHY_FLAG_DATAGRAMS_ONLY = BIT(5),
}

#[repr(C)]
pub struct wpan_phy {
    pub privid: *const core::ffi::c_void,
    pub flags: c_ulong,
    pub current_channel: u8,
    pub current_page: u8,
    pub supported: wpan_phy_supported,
    pub transmit_power: i32,
    pub cca: wpan_phy_cca,
    pub perm_extended_addr: __le64,
    pub cca_ed_level: i32,
    pub symbol_duration: u32,
    pub lifs_period: u16,
    pub sifs_period: u16,
    pub dev: device,
    pub _net: possible_net_t,
    pub queue_lock: spinlock_t,
    pub ongoing_txs: atomic_t,
    pub hold_txs: atomic_t,
    pub sync_txq: wait_queue_head_t,
    pub filtering: ieee802154_filtering_level,
    pub priv_: [u8; 0],
}

pub unsafe fn wpan_phy_net(wpan_phy: *mut wpan_phy) -> *mut net { read_pnet(&(*wpan_phy)._net) }
pub unsafe fn wpan_phy_net_set(wpan_phy: *mut wpan_phy, net: *mut net) { write_pnet(&mut (*wpan_phy)._net, net); }

pub unsafe fn ieee802154_chan_is_valid(phy: *const wpan_phy, page: u8, channel: u8) -> bool {
    if page as u32 > IEEE802154_MAX_PAGE || channel as u32 > IEEE802154_MAX_CHANNEL ||
       ((*phy).supported.channels[page as usize] & BIT(channel)) == 0 { return false; }
    true
}

pub unsafe fn wpan_phy_set_dev(phy: *mut wpan_phy, dev: *mut device) { (*phy).dev.parent = dev; }

pub unsafe fn wpan_phy_priv(phy: *mut wpan_phy) -> *mut core::ffi::c_void {
    // BUG_ON(!phy)
    &mut (*phy).priv_ as *mut [u8; 0] as *mut core::ffi::c_void
}

pub unsafe fn wpan_phy_put(phy: *mut wpan_phy) { put_device(&mut (*phy).dev); }
pub unsafe fn wpan_phy_name(phy: *mut wpan_phy) -> *const core::ffi::c_char { dev_name(&(*phy).dev) }

#[repr(C)]
pub union ieee802154_addr_union { pub short_addr: __le16, pub extended_addr: __le64 }
#[repr(C)]
pub struct ieee802154_addr { pub mode: u8, pub pan_id: __le16, pub addr: ieee802154_addr_union }

#[repr(C)]
pub struct ieee802154_coord_desc { pub addr: ieee802154_addr, pub page: u8, pub channel: u8, pub superframe_spec: u16, pub link_quality: u8, pub gts_permit: bool }
#[repr(C)]
pub struct ieee802154_pan_device { pub pan_id: __le16, pub mode: u8, pub short_addr: __le16, pub extended_addr: __le64, pub node: list_head }
#[repr(C)]
pub struct cfg802154_scan_request { pub type_: nl802154_scan_types, pub page: u8, pub channels: u32, pub duration: u8, pub wpan_dev: *mut wpan_dev, pub wpan_phy: *mut wpan_phy }
#[repr(C)]
pub struct cfg802154_beacon_request { pub interval: u8, pub wpan_dev: *mut wpan_dev, pub wpan_phy: *mut wpan_phy }
#[repr(C)]
pub struct cfg802154_mac_pkt { pub node: list_head, pub skb: *mut sk_buff, pub sdata: *mut ieee802154_sub_if_data, pub page: u8, pub channel: u8 }

#[repr(C)]
pub union ieee802154_llsec_key_id_union { pub device_addr: ieee802154_addr, pub short_source: __le32, pub extended_source: __le64 }
#[repr(C)]
pub struct ieee802154_llsec_key_id { pub mode: u8, pub id: u8, pub key: ieee802154_llsec_key_id_union }
pub const IEEE802154_LLSEC_KEY_SIZE: usize = 16;
#[repr(C)]
pub struct ieee802154_llsec_key { pub frame_types: u8, pub cmd_frame_ids: u32, pub key: [u8; IEEE802154_LLSEC_KEY_SIZE] }
#[repr(C)]
pub struct ieee802154_llsec_key_entry { pub list: list_head, pub rcu: rcu_head, pub id: ieee802154_llsec_key_id, pub key: *mut ieee802154_llsec_key }
#[repr(C)]
pub struct ieee802154_llsec_params { pub enabled: bool, pub frame_counter: __be32, pub out_level: u8, pub out_key: ieee802154_llsec_key_id, pub default_key_source: __le64, pub pan_id: __le16, pub hwaddr: __le64, pub coord_hwaddr: __le64, pub coord_shortaddr: __le16 }
#[repr(C)]
pub struct ieee802154_llsec_table { pub keys: list_head, pub devices: list_head, pub security_levels: list_head }
#[repr(C)]
pub struct ieee802154_llsec_seclevel { pub list: list_head, pub frame_type: u8, pub cmd_frame_id: u8, pub device_override: bool, pub sec_levels: u32 }
#[repr(C)]
pub struct ieee802154_llsec_device { pub list: list_head, pub pan_id: __le16, pub short_addr: __le16, pub hwaddr: __le64, pub frame_counter: u32, pub seclevel_exempt: bool, pub key_mode: u8, pub keys: list_head }
#[repr(C)]
pub struct ieee802154_llsec_device_key { pub list: list_head, pub key_id: ieee802154_llsec_key_id, pub frame_counter: u32 }

#[repr(C)]
pub struct wpan_dev_header_ops { pub create: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device, *const ieee802154_addr, *const ieee802154_addr, c_uint) -> i32> }
#[repr(C)]
pub struct wpan_dev {
    pub wpan_phy: *mut wpan_phy, pub iftype: i32, pub list: list_head, pub netdev: *mut net_device,
    pub header_ops: *const wpan_dev_header_ops, pub lowpan_dev: *mut net_device, pub identifier: u32,
    pub pan_id: __le16, pub short_addr: __le16, pub extended_addr: __le64, pub bsn: atomic_t, pub dsn: atomic_t,
    pub min_be: u8, pub max_be: u8, pub csma_retries: u8, pub frame_retries: i8, pub lbt: bool, pub ackreq: bool,
    pub association_lock: mutex, pub parent: *mut ieee802154_pan_device, pub children: list_head,
    pub max_associations: c_uint, pub nchildren: c_uint,
}

// C condition: IS_ENABLED(CONFIG_IEEE802154) || IS_ENABLED(CONFIG_6LOWPAN)
pub unsafe fn wpan_dev_hard_header(skb: *mut sk_buff, dev: *mut net_device, daddr: *const ieee802154_addr, saddr: *const ieee802154_addr, len: c_uint) -> i32 {
    let wpan_dev = (*dev).ieee802154_ptr;
    ((*wpan_dev).header_ops).as_ref().unwrap().create.unwrap()(skb, dev, daddr, saddr, len)
}

extern "C" {
    pub fn wpan_phy_new(ops: *const cfg802154_ops, priv_size: usize) -> *mut wpan_phy;
    pub fn wpan_phy_register(phy: *mut wpan_phy) -> i32;
    pub fn wpan_phy_unregister(phy: *mut wpan_phy);
    pub fn wpan_phy_free(phy: *mut wpan_phy);
    pub fn wpan_phy_for_each(fn_: Option<unsafe extern "C" fn(*mut wpan_phy, *mut core::ffi::c_void) -> i32>, data: *mut core::ffi::c_void) -> i32;
    pub fn wpan_phy_find(str_: *const core::ffi::c_char) -> *mut wpan_phy;
    pub fn ieee802154_configure_durations(phy: *mut wpan_phy, page: c_uint, channel: c_uint);
    pub fn cfg802154_device_is_associated(wpan_dev: *mut wpan_dev) -> bool;
    pub fn cfg802154_device_is_parent(wpan_dev: *mut wpan_dev, target: *mut ieee802154_addr) -> bool;
    pub fn cfg802154_device_is_child(wpan_dev: *mut wpan_dev, target: *mut ieee802154_addr) -> *mut ieee802154_pan_device;
    pub fn cfg802154_set_max_associations(wpan_dev: *mut wpan_dev, max: c_uint) -> c_uint;
    pub fn cfg802154_get_free_short_addr(wpan_dev: *mut wpan_dev) -> __le16;
    pub fn put_device(dev: *mut device);
    pub fn dev_name(dev: *const device) -> *const core::ffi::c_char;
    pub fn read_pnet(net: *const possible_net_t) -> *mut net;
    pub fn write_pnet(net: *mut possible_net_t, value: *mut net);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
