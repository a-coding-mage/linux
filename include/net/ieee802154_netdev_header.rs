/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * An interface between IEEE802.15.4 device and rest of the kernel.
 * Rust translation of ieee802154_netdev.h.
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C, packed)]
pub struct ieee802154_beacon_hdr {
    /* C bit-fields; endian-specific layout is supplied by the target ABI. */
    pub control: u16,
    pub gts: u8,
    pub pending: u8,
}

#[repr(C, packed)]
pub struct ieee802154_mac_cmd_pl { pub cmd_id: u8 }

#[repr(C)]
pub union ieee802154_sechdr_source { pub short_src: __le32, pub extended_src: __le64 }
#[repr(C, packed)]
pub struct ieee802154_sechdr {
    pub control: u8,
    pub key_id: u8,
    pub frame_counter: __le32,
    pub source: ieee802154_sechdr_source,
}

#[repr(C, packed)]
pub struct ieee802154_hdr_fc { pub control: u16 }

#[repr(C, packed)]
pub struct ieee802154_assoc_req_pl { pub control: u8 }
#[repr(C, packed)]
pub struct ieee802154_assoc_resp_pl { pub short_addr: __le16, pub status: u8 }

pub enum ieee802154_frame_version {
    IEEE802154_2003_STD,
    IEEE802154_2006_STD,
    IEEE802154_STD,
    IEEE802154_RESERVED_STD,
}
pub const IEEE802154_MULTIPURPOSE_STD: ieee802154_frame_version = ieee802154_frame_version::IEEE802154_2003_STD;

pub enum ieee802154_addressing_mode {
    IEEE802154_NO_ADDRESSING,
    IEEE802154_RESERVED,
    IEEE802154_SHORT_ADDRESSING,
    IEEE802154_EXTENDED_ADDRESSING,
}
pub enum ieee802154_association_status {
    IEEE802154_ASSOCIATION_SUCCESSFUL = 0x00,
    IEEE802154_PAN_AT_CAPACITY = 0x01,
    IEEE802154_PAN_ACCESS_DENIED = 0x02,
    IEEE802154_HOPPING_SEQUENCE_OFFSET_DUP = 0x03,
    IEEE802154_FAST_ASSOCIATION_SUCCESSFUL = 0x80,
}
pub enum ieee802154_disassociation_reason {
    IEEE802154_COORD_WISHES_DEVICE_TO_LEAVE = 0x1,
    IEEE802154_DEVICE_WISHES_TO_LEAVE = 0x2,
}

#[repr(C)]
pub struct ieee802154_hdr { pub fc: ieee802154_hdr_fc, pub seq: u8, pub source: ieee802154_addr, pub dest: ieee802154_addr, pub sec: ieee802154_sechdr }
#[repr(C)]
pub struct ieee802154_beacon_frame { pub mhr: ieee802154_hdr, pub mac_pl: ieee802154_beacon_hdr }
#[repr(C)]
pub struct ieee802154_mac_cmd_frame { pub mhr: ieee802154_hdr, pub mac_pl: ieee802154_mac_cmd_pl }
#[repr(C)]
pub struct ieee802154_beacon_req_frame { pub mhr: ieee802154_hdr, pub mac_pl: ieee802154_mac_cmd_pl }
#[repr(C)]
pub struct ieee802154_association_req_frame { pub mhr: ieee802154_hdr, pub mac_pl: ieee802154_mac_cmd_pl, pub assoc_req_pl: ieee802154_assoc_req_pl }
#[repr(C)]
pub struct ieee802154_association_resp_frame { pub mhr: ieee802154_hdr, pub mac_pl: ieee802154_mac_cmd_pl, pub assoc_resp_pl: ieee802154_assoc_resp_pl }
#[repr(C)]
pub struct ieee802154_disassociation_notif_frame { pub mhr: ieee802154_hdr, pub mac_pl: ieee802154_mac_cmd_pl, pub disassoc_pl: u8 }

extern "C" {
    pub fn ieee802154_hdr_push(skb: *mut sk_buff, hdr: *mut ieee802154_hdr) -> i32;
    pub fn ieee802154_hdr_pull(skb: *mut sk_buff, hdr: *mut ieee802154_hdr) -> i32;
    pub fn ieee802154_hdr_peek_addrs(skb: *const sk_buff, hdr: *mut ieee802154_hdr) -> i32;
    pub fn ieee802154_hdr_peek(skb: *const sk_buff, hdr: *mut ieee802154_hdr) -> i32;
    pub fn ieee802154_beacon_push(skb: *mut sk_buff, beacon: *mut ieee802154_beacon_frame) -> i32;
    pub fn ieee802154_mac_cmd_push(skb: *mut sk_buff, frame: *mut core::ffi::c_void, pl: *const core::ffi::c_void, pl_len: u32) -> i32;
    pub fn ieee802154_mac_cmd_pl_pull(skb: *mut sk_buff, mac_pl: *mut ieee802154_mac_cmd_pl) -> i32;
    pub fn ieee802154_max_payload(hdr: *const ieee802154_hdr) -> i32;
}

#[inline]
pub unsafe fn ieee802154_hdr_length(skb: *mut sk_buff) -> i32 {
    let mut hdr = core::mem::MaybeUninit::<ieee802154_hdr>::uninit();
    let len = ieee802154_hdr_pull(skb, hdr.as_mut_ptr());
    if len > 0 { skb_push(skb, len); }
    len
}

#[inline]
pub unsafe fn ieee802154_addr_equal(a1: *const ieee802154_addr, a2: *const ieee802154_addr) -> bool {
    (*a1).pan_id == (*a2).pan_id && (*a1).mode == (*a2).mode &&
        ((*a1).mode != IEEE802154_ADDR_LONG || (*a1).extended_addr == (*a2).extended_addr) &&
        ((*a1).mode != IEEE802154_ADDR_SHORT || (*a1).short_addr == (*a2).short_addr)
}

#[inline]
pub unsafe fn ieee802154_devaddr_from_raw(raw: *const core::ffi::c_void) -> __le64 {
    let mut temp: u64 = 0;
    core::ptr::copy_nonoverlapping(raw as *const u8, &mut temp as *mut u64 as *mut u8, IEEE802154_ADDR_LEN);
    swab64(temp) as __le64
}
#[inline]
pub unsafe fn ieee802154_devaddr_to_raw(raw: *mut core::ffi::c_void, addr: __le64) {
    let temp = swab64(addr as u64);
    core::ptr::copy_nonoverlapping(&temp as *const u64 as *const u8, raw as *mut u8, IEEE802154_ADDR_LEN);
}

#[repr(C)]
pub struct ieee802154_mac_cb { pub lqi: u8, pub type_: u8, pub ackreq: bool, pub secen: bool, pub secen_override: bool, pub seclevel: u8, pub seclevel_override: bool, pub source: ieee802154_addr, pub dest: ieee802154_addr }

#[inline] pub unsafe fn mac_cb(skb: *mut sk_buff) -> *mut ieee802154_mac_cb { (*skb).cb.as_mut_ptr() as *mut ieee802154_mac_cb }
#[inline] pub unsafe fn mac_cb_init(skb: *mut sk_buff) -> *mut ieee802154_mac_cb {
    core::ptr::write_bytes((*skb).cb.as_mut_ptr(), 0, core::mem::size_of::<ieee802154_mac_cb>());
    mac_cb(skb)
}

pub const IEEE802154_MAC_SCAN_ED: u32 = 0;
pub const IEEE802154_MAC_SCAN_ACTIVE: u32 = 1;
pub const IEEE802154_MAC_SCAN_PASSIVE: u32 = 2;
pub const IEEE802154_MAC_SCAN_ORPHAN: u32 = 3;
pub const IEEE802154_LLSEC_DEVKEY_IGNORE: u32 = 0;
pub const IEEE802154_LLSEC_DEVKEY_RESTRICT: u32 = 1;
pub const IEEE802154_LLSEC_DEVKEY_RECORD: u32 = 2;

#[repr(C)]
pub struct ieee802154_mac_params { pub transmit_power: i8, pub min_be: u8, pub max_be: u8, pub csma_retries: u8, pub frame_retries: i8, pub lbt: bool, pub cca: wpan_phy_cca, pub cca_ed_level: i32 }
pub struct wpan_phy;

pub const IEEE802154_LLSEC_PARAM_ENABLED: u32 = 1 << 0;
pub const IEEE802154_LLSEC_PARAM_FRAME_COUNTER: u32 = 1 << 1;
pub const IEEE802154_LLSEC_PARAM_OUT_LEVEL: u32 = 1 << 2;
pub const IEEE802154_LLSEC_PARAM_OUT_KEY: u32 = 1 << 3;
pub const IEEE802154_LLSEC_PARAM_KEY_SOURCE: u32 = 1 << 4;
pub const IEEE802154_LLSEC_PARAM_PAN_ID: u32 = 1 << 5;
pub const IEEE802154_LLSEC_PARAM_HWADDR: u32 = 1 << 6;
pub const IEEE802154_LLSEC_PARAM_COORD_HWADDR: u32 = 1 << 7;
pub const IEEE802154_LLSEC_PARAM_COORD_SHORTADDR: u32 = 1 << 8;

#[repr(C)]
pub struct ieee802154_llsec_ops {
    pub get_params: Option<unsafe extern "C" fn(*mut net_device, *mut ieee802154_llsec_params) -> i32>,
    pub set_params: Option<unsafe extern "C" fn(*mut net_device, *const ieee802154_llsec_params, i32) -> i32>,
    pub add_key: Option<unsafe extern "C" fn(*mut net_device, *const ieee802154_llsec_key_id, *const ieee802154_llsec_key) -> i32>,
    pub del_key: Option<unsafe extern "C" fn(*mut net_device, *const ieee802154_llsec_key_id) -> i32>,
    pub add_dev: Option<unsafe extern "C" fn(*mut net_device, *const ieee802154_llsec_device) -> i32>,
    pub del_dev: Option<unsafe extern "C" fn(*mut net_device, __le64) -> i32>,
    pub add_devkey: Option<unsafe extern "C" fn(*mut net_device, __le64, *const ieee802154_llsec_device_key) -> i32>,
    pub del_devkey: Option<unsafe extern "C" fn(*mut net_device, __le64, *const ieee802154_llsec_device_key) -> i32>,
    pub add_seclevel: Option<unsafe extern "C" fn(*mut net_device, *const ieee802154_llsec_seclevel) -> i32>,
    pub del_seclevel: Option<unsafe extern "C" fn(*mut net_device, *const ieee802154_llsec_seclevel) -> i32>,
    pub lock_table: Option<unsafe extern "C" fn(*mut net_device)>,
    pub get_table: Option<unsafe extern "C" fn(*mut net_device, *mut *mut ieee802154_llsec_table)>,
    pub unlock_table: Option<unsafe extern "C" fn(*mut net_device)>,
}

#[repr(C)]
pub struct ieee802154_mlme_ops {
    pub assoc_req: Option<unsafe extern "C" fn(*mut net_device, *mut ieee802154_addr, u8, u8, u8) -> i32>,
    pub assoc_resp: Option<unsafe extern "C" fn(*mut net_device, *mut ieee802154_addr, __le16, u8) -> i32>,
    pub disassoc_req: Option<unsafe extern "C" fn(*mut net_device, *mut ieee802154_addr, u8) -> i32>,
    pub start_req: Option<unsafe extern "C" fn(*mut net_device, *mut ieee802154_addr, u8, u8, u8, u8, u8, u8, u8) -> i32>,
    pub scan_req: Option<unsafe extern "C" fn(*mut net_device, u8, u32, u8, u8) -> i32>,
    pub set_mac_params: Option<unsafe extern "C" fn(*mut net_device, *const ieee802154_mac_params) -> i32>,
    pub get_mac_params: Option<unsafe extern "C" fn(*mut net_device, *mut ieee802154_mac_params)>,
    pub llsec: *const ieee802154_llsec_ops,
}

#[inline]
pub unsafe fn ieee802154_mlme_ops(dev: *const net_device) -> *mut ieee802154_mlme_ops { (*dev).ml_priv as *mut ieee802154_mlme_ops }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
