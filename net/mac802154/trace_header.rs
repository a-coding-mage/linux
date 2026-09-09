/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of mac802154/trace.h. Linux tracepoint registration is
 * supplied by the surrounding trace infrastructure. */

use core::ffi::c_char;

/* External C types and helpers supplied by the kernel headers. */
#[repr(C)]
pub struct ieee802154_local {
    pub hw: ieee802154_hw,
}
#[repr(C)]
pub struct ieee802154_hw {
    pub phy: *mut wpan_phy,
}
#[repr(C)]
pub struct wpan_phy;
#[repr(C)]
pub struct wpan_phy_cca {
    pub mode: nl802154_cca_modes,
    pub opt: nl802154_cca_opts,
}
#[repr(C)]
pub struct ieee802154_coord_desc {
    pub page: u8,
    pub channel: u8,
    pub addr: ieee802154_addr,
}
#[repr(C)]
pub struct ieee802154_addr {
    pub pan_id: __le16,
    pub extended_addr: __le64,
}

pub type __le16 = u16;
pub type __le64 = u64;
pub type s8 = i8;
pub type s32 = i32;
pub type nl802154_cca_modes = i32;
pub type nl802154_cca_opts = i32;

pub const MAXNAME: usize = 32;

/* The following declarations correspond to TRACE_EVENT/DEFINE_EVENT entries.
 * The original LOCAL_ASSIGN operation copies wpan_phy_name(local->hw.phy)
 * into the fixed-size trace entry name; CCA_ASSIGN copies mode and opt. */

#[repr(C)]
pub struct LocalOnlyEvt4 {
    pub wpan_phy_name: [c_char; MAXNAME],
}
#[repr(C)]
pub struct DrvReturnInt {
    pub wpan_phy_name: [c_char; MAXNAME],
    pub ret: i32,
}
#[repr(C)]
pub struct DrvSetChannel {
    pub wpan_phy_name: [c_char; MAXNAME],
    pub page: u8,
    pub channel: u8,
}
#[repr(C)]
pub struct DrvSetCcaMode {
    pub wpan_phy_name: [c_char; MAXNAME],
    pub cca_mode: nl802154_cca_modes,
    pub cca_opt: nl802154_cca_opts,
}
#[repr(C)]
pub struct DrvSetCcaEdLevel { pub wpan_phy_name: [c_char; MAXNAME], pub mbm: s32 }
#[repr(C)]
pub struct DrvSetTxPower { pub wpan_phy_name: [c_char; MAXNAME], pub power: s32 }
#[repr(C)]
pub struct DrvSetLbtMode { pub wpan_phy_name: [c_char; MAXNAME], pub mode: bool }
#[repr(C)]
pub struct DrvSetShortAddr { pub wpan_phy_name: [c_char; MAXNAME], pub short_addr: __le16 }
#[repr(C)]
pub struct DrvSetPanId { pub wpan_phy_name: [c_char; MAXNAME], pub pan_id: __le16 }
#[repr(C)]
pub struct DrvSetExtendedAddr { pub wpan_phy_name: [c_char; MAXNAME], pub extended_addr: __le64 }
#[repr(C)]
pub struct DrvSetPanCoord { pub wpan_phy_name: [c_char; MAXNAME], pub is_coord: bool }
#[repr(C)]
pub struct DrvSetCsmaParams {
    pub wpan_phy_name: [c_char; MAXNAME],
    pub min_be: u8,
    pub max_be: u8,
    pub max_csma_backoffs: u8,
}
#[repr(C)]
pub struct DrvSetMaxFrameRetries { pub wpan_phy_name: [c_char; MAXNAME], pub max_frame_retries: s8 }
#[repr(C)]
pub struct DrvSetPromiscuousMode { pub wpan_phy_name: [c_char; MAXNAME], pub on: bool }
#[repr(C)]
pub struct NewScanEvent {
    pub pan_id: __le16,
    pub addr: __le64,
    pub channel: u8,
    pub page: u8,
}

/* Tracepoint callbacks retain the original TP_PROTO argument lists. */
unsafe extern "C" {
    pub fn trace_802154_drv_return_void(local: *mut ieee802154_local);
    pub fn trace_802154_drv_return_int(local: *mut ieee802154_local, ret: i32);
    pub fn trace_802154_drv_start(local: *mut ieee802154_local);
    pub fn trace_802154_drv_stop(local: *mut ieee802154_local);
    pub fn trace_802154_drv_set_channel(local: *mut ieee802154_local, page: u8, channel: u8);
    pub fn trace_802154_drv_set_cca_mode(local: *mut ieee802154_local, cca: *const wpan_phy_cca);
    pub fn trace_802154_drv_set_cca_ed_level(local: *mut ieee802154_local, mbm: s32);
    pub fn trace_802154_drv_set_tx_power(local: *mut ieee802154_local, power: s32);
    pub fn trace_802154_drv_set_lbt_mode(local: *mut ieee802154_local, mode: bool);
    pub fn trace_802154_drv_set_short_addr(local: *mut ieee802154_local, short_addr: __le16);
    pub fn trace_802154_drv_set_pan_id(local: *mut ieee802154_local, pan_id: __le16);
    pub fn trace_802154_drv_set_extended_addr(local: *mut ieee802154_local, extended_addr: __le64);
    pub fn trace_802154_drv_set_pan_coord(local: *mut ieee802154_local, is_coord: bool);
    pub fn trace_802154_drv_set_csma_params(local: *mut ieee802154_local, min_be: u8, max_be: u8, max_csma_backoffs: u8);
    pub fn trace_802154_drv_set_max_frame_retries(local: *mut ieee802154_local, max_frame_retries: s8);
    pub fn trace_802154_drv_set_promiscuous_mode(local: *mut ieee802154_local, on: bool);
    pub fn trace_802154_new_scan_event(desc: *mut ieee802154_coord_desc);
    pub fn trace_802154_scan_event(desc: *mut ieee802154_coord_desc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
