/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file holds Hub protocol constants and data structures that are
 * defined in chapter 11 (Hub Specification) of the USB 2.0 specification.
 *
 * It is used/shared between the USB core, the HCDs and couple of other USB
 * drivers.
 */

/* __u8 etc. and USB_* symbols are supplied by other headers. */

pub const USB_MAXCHILDREN: usize = 31;
pub const USB_SS_MAXPORTS: usize = 15;

pub const USB_RT_HUB: u32 = USB_TYPE_CLASS | USB_RECIP_DEVICE;
pub const USB_RT_PORT: u32 = USB_TYPE_CLASS | USB_RECIP_OTHER;

pub const HUB_PORT_STATUS: u32 = 0;
pub const HUB_PORT_PD_STATUS: u32 = 1;
pub const HUB_EXT_PORT_STATUS: u32 = 2;

pub const HUB_CLEAR_TT_BUFFER: u32 = 8;
pub const HUB_RESET_TT: u32 = 9;
pub const HUB_GET_TT_STATE: u32 = 10;
pub const HUB_STOP_TT: u32 = 11;

pub const HUB_SET_DEPTH: u32 = 12;
pub const HUB_GET_PORT_ERR_COUNT: u32 = 13;

pub const C_HUB_LOCAL_POWER: u32 = 0;
pub const C_HUB_OVER_CURRENT: u32 = 1;

pub const USB_PORT_FEAT_CONNECTION: u32 = 0;
pub const USB_PORT_FEAT_ENABLE: u32 = 1;
pub const USB_PORT_FEAT_SUSPEND: u32 = 2;
pub const USB_PORT_FEAT_OVER_CURRENT: u32 = 3;
pub const USB_PORT_FEAT_RESET: u32 = 4;
pub const USB_PORT_FEAT_L1: u32 = 5;
pub const USB_PORT_FEAT_POWER: u32 = 8;
pub const USB_PORT_FEAT_LOWSPEED: u32 = 9;
pub const USB_PORT_FEAT_C_CONNECTION: u32 = 16;
pub const USB_PORT_FEAT_C_ENABLE: u32 = 17;
pub const USB_PORT_FEAT_C_SUSPEND: u32 = 18;
pub const USB_PORT_FEAT_C_OVER_CURRENT: u32 = 19;
pub const USB_PORT_FEAT_C_RESET: u32 = 20;
pub const USB_PORT_FEAT_TEST: u32 = 21;
pub const USB_PORT_FEAT_INDICATOR: u32 = 22;
pub const USB_PORT_FEAT_C_PORT_L1: u32 = 23;

pub const USB_PORT_FEAT_LINK_STATE: u32 = 5;
pub const USB_PORT_FEAT_U1_TIMEOUT: u32 = 23;
pub const USB_PORT_FEAT_U2_TIMEOUT: u32 = 24;
pub const USB_PORT_FEAT_C_PORT_LINK_STATE: u32 = 25;
pub const USB_PORT_FEAT_C_PORT_CONFIG_ERROR: u32 = 26;
pub const USB_PORT_FEAT_REMOTE_WAKE_MASK: u32 = 27;
pub const USB_PORT_FEAT_BH_PORT_RESET: u32 = 28;
pub const USB_PORT_FEAT_C_BH_PORT_RESET: u32 = 29;
pub const USB_PORT_FEAT_FORCE_LINKPM_ACCEPT: u32 = 30;

#[inline]
pub const fn USB_PORT_LPM_TIMEOUT(p: u32) -> u32 { (p & 0xff) << 8 }

pub const USB_PORT_FEAT_REMOTE_WAKE_CONNECT: u32 = 1 << 8;
pub const USB_PORT_FEAT_REMOTE_WAKE_DISCONNECT: u32 = 1 << 9;
pub const USB_PORT_FEAT_REMOTE_WAKE_OVER_CURRENT: u32 = 1 << 10;

#[repr(C, packed)]
pub struct usb_port_status {
    pub wPortStatus: __le16,
    pub wPortChange: __le16,
    pub dwExtPortStatus: __le32,
}

pub const USB_PORT_STAT_CONNECTION: u32 = 0x0001;
pub const USB_PORT_STAT_ENABLE: u32 = 0x0002;
pub const USB_PORT_STAT_SUSPEND: u32 = 0x0004;
pub const USB_PORT_STAT_OVERCURRENT: u32 = 0x0008;
pub const USB_PORT_STAT_RESET: u32 = 0x0010;
pub const USB_PORT_STAT_L1: u32 = 0x0020;
pub const USB_PORT_STAT_POWER: u32 = 0x0100;
pub const USB_PORT_STAT_LOW_SPEED: u32 = 0x0200;
pub const USB_PORT_STAT_HIGH_SPEED: u32 = 0x0400;
pub const USB_PORT_STAT_TEST: u32 = 0x0800;
pub const USB_PORT_STAT_INDICATOR: u32 = 0x1000;

pub const USB_PORT_STAT_LINK_STATE: u32 = 0x01e0;
pub const USB_SS_PORT_STAT_POWER: u32 = 0x0200;
pub const USB_SS_PORT_STAT_SPEED: u32 = 0x1c00;
pub const USB_PORT_STAT_SPEED_5GBPS: u32 = 0x0000;
pub const USB_SS_PORT_STAT_MASK: u32 = USB_PORT_STAT_CONNECTION | USB_PORT_STAT_ENABLE | USB_PORT_STAT_OVERCURRENT | USB_PORT_STAT_RESET;

pub const USB_SS_PORT_LS_U0: u32 = 0x0000;
pub const USB_SS_PORT_LS_U1: u32 = 0x0020;
pub const USB_SS_PORT_LS_U2: u32 = 0x0040;
pub const USB_SS_PORT_LS_U3: u32 = 0x0060;
pub const USB_SS_PORT_LS_SS_DISABLED: u32 = 0x0080;
pub const USB_SS_PORT_LS_RX_DETECT: u32 = 0x00a0;
pub const USB_SS_PORT_LS_SS_INACTIVE: u32 = 0x00c0;
pub const USB_SS_PORT_LS_POLLING: u32 = 0x00e0;
pub const USB_SS_PORT_LS_RECOVERY: u32 = 0x0100;
pub const USB_SS_PORT_LS_HOT_RESET: u32 = 0x0120;
pub const USB_SS_PORT_LS_COMP_MOD: u32 = 0x0140;
pub const USB_SS_PORT_LS_LOOPBACK: u32 = 0x0160;

pub const USB_PORT_STAT_C_CONNECTION: u32 = 0x0001;
pub const USB_PORT_STAT_C_ENABLE: u32 = 0x0002;
pub const USB_PORT_STAT_C_SUSPEND: u32 = 0x0004;
pub const USB_PORT_STAT_C_OVERCURRENT: u32 = 0x0008;
pub const USB_PORT_STAT_C_RESET: u32 = 0x0010;
pub const USB_PORT_STAT_C_L1: u32 = 0x0020;
pub const USB_PORT_STAT_C_BH_RESET: u32 = 0x0020;
pub const USB_PORT_STAT_C_LINK_STATE: u32 = 0x0040;
pub const USB_PORT_STAT_C_CONFIG_ERROR: u32 = 0x0080;

pub const USB_EXT_PORT_STAT_RX_SPEED_ID: u32 = 0x0000000f;
pub const USB_EXT_PORT_STAT_TX_SPEED_ID: u32 = 0x000000f0;
pub const USB_EXT_PORT_STAT_RX_LANES: u32 = 0x00000f00;
pub const USB_EXT_PORT_STAT_TX_LANES: u32 = 0x0000f000;

#[inline]
pub const fn USB_EXT_PORT_RX_LANES(p: u32) -> u32 { (p & USB_EXT_PORT_STAT_RX_LANES) >> 8 }
#[inline]
pub const fn USB_EXT_PORT_TX_LANES(p: u32) -> u32 { (p & USB_EXT_PORT_STAT_TX_LANES) >> 12 }

pub const HUB_CHAR_LPSM: u32 = 0x0003;
pub const HUB_CHAR_COMMON_LPSM: u32 = 0x0000;
pub const HUB_CHAR_INDV_PORT_LPSM: u32 = 0x0001;
pub const HUB_CHAR_NO_LPSM: u32 = 0x0002;
pub const HUB_CHAR_COMPOUND: u32 = 0x0004;
pub const HUB_CHAR_OCPM: u32 = 0x0018;
pub const HUB_CHAR_COMMON_OCPM: u32 = 0x0000;
pub const HUB_CHAR_INDV_PORT_OCPM: u32 = 0x0008;
pub const HUB_CHAR_NO_OCPM: u32 = 0x0010;
pub const HUB_CHAR_TTTT: u32 = 0x0060;
pub const HUB_CHAR_PORTIND: u32 = 0x0080;

#[repr(C, packed)]
pub struct usb_hub_status { pub wHubStatus: __le16, pub wHubChange: __le16 }

pub const HUB_STATUS_LOCAL_POWER: u32 = 0x0001;
pub const HUB_STATUS_OVERCURRENT: u32 = 0x0002;
pub const HUB_CHANGE_LOCAL_POWER: u32 = 0x0001;
pub const HUB_CHANGE_OVERCURRENT: u32 = 0x0002;

pub const USB_DT_HUB: u32 = USB_TYPE_CLASS | 0x09;
pub const USB_DT_SS_HUB: u32 = USB_TYPE_CLASS | 0x0a;
pub const USB_DT_HUB_NONVAR_SIZE: usize = 7;
pub const USB_DT_SS_HUB_SIZE: usize = 12;

pub const USB_HUB_PR_FS: u32 = 0;
pub const USB_HUB_PR_HS_NO_TT: u32 = 0;
pub const USB_HUB_PR_HS_SINGLE_TT: u32 = 1;
pub const USB_HUB_PR_HS_MULTI_TT: u32 = 2;
pub const USB_HUB_PR_SS: u32 = 3;

#[repr(C, packed)]
pub struct usb_hub_descriptor {
    pub bDescLength: __u8,
    pub bDescriptorType: __u8,
    pub bNbrPorts: __u8,
    pub wHubCharacteristics: __le16,
    pub bPwrOn2PwrGood: __u8,
    pub bHubContrCurrent: __u8,
    pub u: usb_hub_descriptor__u,
}

#[repr(C)]
pub union usb_hub_descriptor__u {
    pub hs: usb_hub_descriptor__u__hs,
    pub ss: usb_hub_descriptor__u__ss,
}

#[repr(C, packed)]
pub struct usb_hub_descriptor__u__hs {
    pub DeviceRemovable: [__u8; (USB_MAXCHILDREN + 1 + 7) / 8],
    pub PortPwrCtrlMask: [__u8; (USB_MAXCHILDREN + 1 + 7) / 8],
}

#[repr(C, packed)]
pub struct usb_hub_descriptor__u__ss {
    pub bHubHdrDecLat: __u8,
    pub wHubDelay: __le16,
    pub DeviceRemovable: __le16,
}

pub const HUB_LED_AUTO: u32 = 0;
pub const HUB_LED_AMBER: u32 = 1;
pub const HUB_LED_GREEN: u32 = 2;
pub const HUB_LED_OFF: u32 = 3;

#[repr(u8)]
pub enum hub_led_mode {
    INDICATOR_AUTO = 0,
    INDICATOR_CYCLE,
    INDICATOR_GREEN_BLINK,
    INDICATOR_GREEN_BLINK_OFF,
    INDICATOR_AMBER_BLINK,
    INDICATOR_AMBER_BLINK_OFF,
    INDICATOR_ALT_BLINK,
    INDICATOR_ALT_BLINK_OFF,
}

pub const HUB_TTTT_8_BITS: u32 = 0x00;
pub const HUB_TTTT_16_BITS: u32 = 0x20;
pub const HUB_TTTT_24_BITS: u32 = 0x40;
pub const HUB_TTTT_32_BITS: u32 = 0x60;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
