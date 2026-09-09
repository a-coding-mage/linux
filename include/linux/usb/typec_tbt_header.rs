/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <linux/usb/typec_altmode.h> and <linux/bitfield.h>.

pub const USB_TYPEC_VENDOR_INTEL: u32 = 0x8087;
/* Alias for convenience */
pub const USB_TYPEC_TBT_SID: u32 = USB_TYPEC_VENDOR_INTEL;

/* Connector state for Thunderbolt3 */
pub const TYPEC_TBT_MODE: u32 = TYPEC_STATE_MODAL;

/**
 * struct typec_thunderbolt_data - Thundebolt3 Alt Mode specific data
 * @device_mode: Device Discover Mode VDO
 * @cable_mode: Cable Discover Mode VDO
 * @enter_vdo: Enter Mode VDO
 */
#[repr(C)]
pub struct typec_thunderbolt_data {
    pub device_mode: u32,
    pub cable_mode: u32,
    pub enter_vdo: u32,
}

/* TBT3 Device Discover Mode VDO bits */
pub const TBT_MODE: u32 = 1u32 << 0;
#[inline]
pub const fn TBT_ADAPTER(_vdo_: u32) -> u32 {
    (_vdo_ & (1u32 << 16)) >> 16
}
pub const TBT_ADAPTER_LEGACY: u32 = 0;
pub const TBT_ADAPTER_TBT3: u32 = 1;
pub const TBT_INTEL_SPECIFIC_B0: u32 = 1u32 << 26;
pub const TBT_VENDOR_SPECIFIC_B0: u32 = 1u32 << 30;
pub const TBT_VENDOR_SPECIFIC_B1: u32 = 1u32 << 31;

#[inline]
pub const fn TBT_SET_ADAPTER(a: u32) -> u32 {
    (a & 1) << 16
}

/* TBT3 Cable Discover Mode VDO bits */
#[inline]
pub const fn TBT_CABLE_SPEED(_vdo_: u32) -> u32 {
    (_vdo_ & (0x7u32 << 16)) >> 16
}
pub const TBT_CABLE_USB3_GEN1: u32 = 1;
pub const TBT_CABLE_USB3_PASSIVE: u32 = 2;
pub const TBT_CABLE_10_AND_20GBPS: u32 = 3;

#[inline]
pub const fn TBT_CABLE_ROUNDED_SUPPORT(_vdo_: u32) -> u32 {
    (_vdo_ & (0x3u32 << 19)) >> 19
}

pub const TBT_GEN3_NON_ROUNDED: u32 = 0;
pub const TBT_GEN3_GEN4_ROUNDED_NON_ROUNDED: u32 = 1;
pub const TBT_CABLE_ROUNDED: u32 = 1u32 << 19;
pub const TBT_CABLE_OPTICAL: u32 = 1u32 << 21;
pub const TBT_CABLE_RETIMER: u32 = 1u32 << 22;
pub const TBT_CABLE_LINK_TRAINING: u32 = 1u32 << 23;
pub const TBT_CABLE_ACTIVE_PASSIVE: u32 = 1u32 << 25;

#[inline]
pub const fn TBT_SET_CABLE_SPEED(_s_: u32) -> u32 {
    (_s_ & 0x7) << 16
}

#[inline]
pub const fn TBT_SET_CABLE_ROUNDED(_g_: u32) -> u32 {
    (_g_ & 0x3) << 19
}

/* TBT3 Device Enter Mode VDO bits */
#[inline]
pub const fn TBT_ENTER_MODE_CABLE_SPEED(s: u32) -> u32 {
    TBT_SET_CABLE_SPEED(s)
}
pub const TBT_ENTER_MODE_UNI_DIR_LSRX: u32 = 1u32 << 23;
pub const TBT_ENTER_MODE_ACTIVE_CABLE: u32 = 1u32 << 24;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
