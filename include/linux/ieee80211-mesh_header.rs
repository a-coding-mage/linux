/* SPDX-License-Identifier: GPL-2.0-only */
/* IEEE 802.11 mesh definitions */

// Dependencies supplied by the surrounding kernel translation:
// u8, u16, u32, __le16, __le32, ETH_ALEN, le32_to_cpu, and get_unaligned_le16.

pub const IEEE80211_MAX_MESH_ID_LEN: usize = 32;

#[repr(C, packed(2))]
pub struct ieee80211s_hdr {
    pub flags: u8,
    pub ttl: u8,
    pub seqnum: __le32,
    pub eaddr1: [u8; ETH_ALEN],
    pub eaddr2: [u8; ETH_ALEN],
}

#[repr(C, packed)]
pub struct ieee80211_mesh_hwmp_preq_target {
    pub flags: u8,
    pub addr: [u8; ETH_ALEN],
    pub sn: __le32,
}

#[repr(C, packed)]
pub struct ieee80211_mesh_hwmp_preq_top {
    pub flags: u8,
    pub hopcount: u8,
    pub ttl: u8,
    pub preq_id: __le32,
    pub orig_addr: [u8; ETH_ALEN],
    pub orig_sn: __le32,
    // optional AE, lifetime, metric, target
    pub variable: [u8; 0],
}

#[repr(C, packed)]
pub struct ieee80211_mesh_hwmp_preq_bottom {
    pub lifetime: __le32,
    pub metric: __le32,
    pub target_count: u8,
    pub targets: [ieee80211_mesh_hwmp_preq_target; 0],
}

#[repr(C, packed)]
pub struct ieee80211_mesh_hwmp_prep_top {
    pub flags: u8,
    pub hopcount: u8,
    pub ttl: u8,
    pub target_addr: [u8; ETH_ALEN],
    pub target_sn: __le32,
    // optional Target External Address
    pub variable: [u8; 0],
}

#[repr(C, packed)]
pub struct ieee80211_mesh_hwmp_prep_bottom {
    pub lifetime: __le32,
    pub metric: __le32,
    pub orig_addr: [u8; ETH_ALEN],
    pub orig_sn: __le32,
}

#[repr(C, packed)]
pub struct ieee80211_mesh_hwmp_perr_dst {
    pub flags: u8,
    pub addr: [u8; ETH_ALEN],
    pub sn: __le32,
    // optional Destination External Address
    pub variable: [u8; 0],
}

#[repr(C, packed)]
pub struct ieee80211_mesh_hwmp_perr {
    pub ttl: u8,
    pub number_of_dst: u8,
    // Destinations
    pub variable: [u8; 0],
}

pub const MESH_FLAGS_AE_A4: u8 = 0x1;
pub const MESH_FLAGS_AE_A5_A6: u8 = 0x2;
pub const MESH_FLAGS_AE: u8 = 0x3;
pub const MESH_FLAGS_PS_DEEP: u8 = 0x4;
pub const AE_F: u8 = 1 << 6;

pub const IEEE80211_PREQ_PROACTIVE_PREP_FLAG: u8 = 1 << 2;
pub const IEEE80211_PREQ_TO_FLAG: u8 = 1 << 0;
pub const IEEE80211_PREQ_USN_FLAG: u8 = 1 << 2;

#[repr(i32)]
pub enum ieee80211_preq_flags { }

#[repr(C, packed)]
pub struct ieee80211_mesh_chansw_params_ie {
    pub mesh_ttl: u8,
    pub mesh_flags: u8,
    pub mesh_reason: __le16,
    pub mesh_pre_value: __le16,
}

#[repr(C, packed)]
pub struct ieee80211_meshconf_ie {
    pub meshconf_psel: u8,
    pub meshconf_pmetric: u8,
    pub meshconf_congest: u8,
    pub meshconf_synch: u8,
    pub meshconf_auth: u8,
    pub meshconf_form: u8,
    pub meshconf_cap: u8,
}

pub const IEEE80211_MESHCONF_CAPAB_ACCEPT_PLINKS: u8 = 0x01;
pub const IEEE80211_MESHCONF_CAPAB_FORWARDING: u8 = 0x08;
pub const IEEE80211_MESHCONF_CAPAB_TBTT_ADJUSTING: u8 = 0x20;
pub const IEEE80211_MESHCONF_CAPAB_POWER_SAVE_LEVEL: u8 = 0x40;
pub const IEEE80211_MESHCONF_FORM_CONNECTED_TO_GATE: u8 = 0x1;
pub const WLAN_EID_CHAN_SWITCH_PARAM_TX_RESTRICT: u8 = 1 << 0;
pub const WLAN_EID_CHAN_SWITCH_PARAM_INITIATOR: u8 = 1 << 1;
pub const WLAN_EID_CHAN_SWITCH_PARAM_REASON: u8 = 1 << 2;

#[repr(C, packed)]
pub struct ieee80211_rann_ie {
    pub rann_flags: u8,
    pub rann_hopcount: u8,
    pub rann_ttl: u8,
    pub rann_addr: [u8; ETH_ALEN],
    pub rann_seq: __le32,
    pub rann_interval: __le32,
    pub rann_metric: __le32,
}

pub const RANN_FLAG_IS_GATE: u8 = 1 << 0;

pub const WLAN_MESH_ACTION_LINK_METRIC_REPORT: i32 = 0;
pub const WLAN_MESH_ACTION_HWMP_PATH_SELECTION: i32 = 1;
pub const WLAN_MESH_ACTION_GATE_ANNOUNCEMENT: i32 = 2;
pub const WLAN_MESH_ACTION_CONGESTION_CONTROL_NOTIFICATION: i32 = 3;
pub const WLAN_MESH_ACTION_MCCA_SETUP_REQUEST: i32 = 4;
pub const WLAN_MESH_ACTION_MCCA_SETUP_REPLY: i32 = 5;
pub const WLAN_MESH_ACTION_MCCA_ADVERTISEMENT_REQUEST: i32 = 6;
pub const WLAN_MESH_ACTION_MCCA_ADVERTISEMENT: i32 = 7;
pub const WLAN_MESH_ACTION_MCCA_TEARDOWN: i32 = 8;
pub const WLAN_MESH_ACTION_TBTT_ADJUSTMENT_REQUEST: i32 = 9;
pub const WLAN_MESH_ACTION_TBTT_ADJUSTMENT_RESPONSE: i32 = 10;

pub const IEEE80211_SYNC_METHOD_NEIGHBOR_OFFSET: i32 = 1;
pub const IEEE80211_SYNC_METHOD_VENDOR: i32 = 255;
pub const IEEE80211_PATH_PROTOCOL_HWMP: i32 = 1;
pub const IEEE80211_PATH_PROTOCOL_VENDOR: i32 = 255;
pub const IEEE80211_PATH_METRIC_AIRTIME: i32 = 1;
pub const IEEE80211_PATH_METRIC_VENDOR: i32 = 255;
pub const IEEE80211_ROOTMODE_NO_ROOT: i32 = 0;
pub const IEEE80211_ROOTMODE_ROOT: i32 = 1;
pub const IEEE80211_PROACTIVE_PREQ_NO_PREP: i32 = 2;
pub const IEEE80211_PROACTIVE_PREQ_WITH_PREP: i32 = 3;
pub const IEEE80211_PROACTIVE_RANN: i32 = 4;

#[inline]
pub unsafe fn ieee80211_mesh_preq_prep_ae_enabled(ie: *const u8) -> bool {
    *ie & AE_F != 0
}

#[inline]
pub unsafe fn ieee80211_mesh_hwmp_preq_get_bottom(ie: *const u8) -> *mut ieee80211_mesh_hwmp_preq_bottom {
    ie.add(core::mem::size_of::<ieee80211_mesh_hwmp_preq_top>()
        + if ieee80211_mesh_preq_prep_ae_enabled(ie) { ETH_ALEN } else { 0 })
        as *mut ieee80211_mesh_hwmp_preq_bottom
}

#[inline]
pub unsafe fn ieee80211_mesh_hwmp_prep_get_bottom(ie: *const u8) -> *mut ieee80211_mesh_hwmp_prep_bottom {
    ie.add(core::mem::size_of::<ieee80211_mesh_hwmp_prep_top>()
        + if ieee80211_mesh_preq_prep_ae_enabled(ie) { ETH_ALEN } else { 0 })
        as *mut ieee80211_mesh_hwmp_prep_bottom
}

#[inline]
pub unsafe fn ieee80211_mesh_hwmp_perr_get_dst(ie: *const u8, dst_idx: u8) -> *mut ieee80211_mesh_hwmp_perr_dst {
    let perr = ie as *const ieee80211_mesh_hwmp_perr;
    let mut pos = ie.add(core::mem::size_of::<ieee80211_mesh_hwmp_perr>());
    let mut dst = core::ptr::null_mut();
    for _ in 0..=dst_idx {
        dst = pos as *mut ieee80211_mesh_hwmp_perr_dst;
        pos = pos.add(core::mem::size_of::<ieee80211_mesh_hwmp_perr_dst>()
            + if (*dst).flags & AE_F != 0 { ETH_ALEN } else { 0 } + 2);
    }
    let _ = perr;
    dst
}

#[inline]
pub unsafe fn ieee80211_mesh_hwmp_perr_get_addr(ie: *const u8, dst_idx: u8) -> *mut u8 {
    (*ieee80211_mesh_hwmp_perr_get_dst(ie, dst_idx)).addr.as_mut_ptr()
}

#[inline]
pub unsafe fn ieee80211_mesh_hwmp_perr_get_sn(ie: *const u8, dst_idx: u8) -> u32 {
    le32_to_cpu((*ieee80211_mesh_hwmp_perr_get_dst(ie, dst_idx)).sn)
}

#[inline]
pub unsafe fn ieee80211_mesh_hwmp_perr_get_rcode(ie: *const u8, dst_idx: u8) -> u16 {
    let dst = ieee80211_mesh_hwmp_perr_get_dst(ie, dst_idx);
    get_unaligned_le16((*dst).variable.as_ptr().add(if (*dst).flags & AE_F != 0 { ETH_ALEN } else { 0 }))
}

#[inline]
pub unsafe fn ieee80211_mesh_preq_size_ok(pos: *const u8, elen: u8) -> bool {
    let mut needed = core::mem::size_of::<ieee80211_mesh_hwmp_preq_top>();
    if (elen as usize) < needed { return false; }
    needed += if ieee80211_mesh_preq_prep_ae_enabled(pos) { ETH_ALEN } else { 0 }
        + core::mem::size_of::<ieee80211_mesh_hwmp_preq_bottom>();
    if (elen as usize) < needed { return false; }
    let count = (*ieee80211_mesh_hwmp_preq_get_bottom(pos)).target_count;
    if count < 1 { return false; }
    needed += count as usize * core::mem::size_of::<ieee80211_mesh_hwmp_preq_target>();
    elen as usize == needed
}

#[inline]
pub unsafe fn ieee80211_mesh_prep_size_ok(pos: *const u8, elen: u8) -> bool {
    let mut needed = core::mem::size_of::<ieee80211_mesh_hwmp_prep_top>();
    if (elen as usize) < needed { return false; }
    needed += if ieee80211_mesh_preq_prep_ae_enabled(pos) { ETH_ALEN } else { 0 }
        + core::mem::size_of::<ieee80211_mesh_hwmp_prep_bottom>();
    elen as usize == needed
}

#[inline]
pub unsafe fn ieee80211_mesh_perr_size_ok(mut pos: *const u8, elen: u8) -> bool {
    let start = pos;
    let perr = pos as *const ieee80211_mesh_hwmp_perr;
    let mut needed = core::mem::size_of::<ieee80211_mesh_hwmp_perr>();
    if (elen as usize) < needed { return false; }
    pos = pos.add(core::mem::size_of::<ieee80211_mesh_hwmp_perr>());
    for _ in 0..(*perr).number_of_dst {
        let dst = pos as *const ieee80211_mesh_hwmp_perr_dst;
        let mut dst_len = core::mem::size_of::<ieee80211_mesh_hwmp_perr_dst>();
        if (elen as usize) < pos.offset_from(start) as usize + dst_len { return false; }
        dst_len += if (*dst).flags & AE_F != 0 { ETH_ALEN } else { 0 } + 2;
        needed += dst_len;
        pos = pos.add(dst_len);
    }
    elen as usize == needed
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
