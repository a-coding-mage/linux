/* SPDX-License-Identifier: GPL-2.0 */

// External dependencies supplied by the surrounding USB Type-C definitions:
// TYPEC_STATE_MODAL, VDO_CMD_VENDOR, BIT, GENMASK, and FIELD_GET.

pub const USB_TYPEC_DP_SID: u32 = 0xff01;
pub const USB_TYPEC_NVIDIA_VLINK_SID: u32 = 0x955;
pub const USB_TYPEC_DP_MODE: u32 = 1;

pub const TYPEC_DP_STATE_A: u32 = TYPEC_STATE_MODAL;
pub const TYPEC_DP_STATE_B: u32 = TYPEC_DP_STATE_A + 1;
pub const TYPEC_DP_STATE_C: u32 = TYPEC_DP_STATE_B + 1;
pub const TYPEC_DP_STATE_D: u32 = TYPEC_DP_STATE_C + 1;
pub const TYPEC_DP_STATE_E: u32 = TYPEC_DP_STATE_D + 1;
pub const TYPEC_DP_STATE_F: u32 = TYPEC_DP_STATE_E + 1;

#[repr(C)]
pub struct typec_displayport_data {
    pub status: u32,
    pub conf: u32,
}

pub const DP_PIN_ASSIGN_A: u32 = 0;
pub const DP_PIN_ASSIGN_B: u32 = DP_PIN_ASSIGN_A + 1;
pub const DP_PIN_ASSIGN_C: u32 = DP_PIN_ASSIGN_B + 1;
pub const DP_PIN_ASSIGN_D: u32 = DP_PIN_ASSIGN_C + 1;
pub const DP_PIN_ASSIGN_E: u32 = DP_PIN_ASSIGN_D + 1;
pub const DP_PIN_ASSIGN_F: u32 = DP_PIN_ASSIGN_E + 1;
pub const DP_PIN_ASSIGN_MAX: u32 = DP_PIN_ASSIGN_F + 1;

pub const DP_CMD_STATUS_UPDATE: u32 = VDO_CMD_VENDOR(0);
pub const DP_CMD_CONFIGURE: u32 = VDO_CMD_VENDOR(1);

pub const DP_CAP_UFP_D: u32 = 1;
pub const DP_CAP_DFP_D: u32 = 2;
pub const DP_CAP_DFP_D_AND_UFP_D: u32 = 3;
pub const DP_CAP_RECEPTACLE: u32 = 1 << 6;
pub const DP_CAP_USB: u32 = 1 << 7;
pub const DP_CAP_UHBR_13_5_SUPPORT: u32 = 1 << 26;
pub const DP_CAP_DPAM_VERSION: u32 = 1 << 30;
pub const DP_CAP_SIGNALLING_HBR3: u32 = 1;
pub const DP_CAP_SIGNALLING_UHBR10: u32 = 2;
pub const DP_CAP_SIGNALLING_UHBR20: u32 = 3;
pub const DP_CAP_CABLE_TYPE_PASSIVE: u32 = 0;
pub const DP_CAP_CABLE_TYPE_RE_TIMER: u32 = 1;
pub const DP_CAP_CABLE_TYPE_RE_DRIVER: u32 = 2;
pub const DP_CAP_CABLE_TYPE_OPTICAL: u32 = 3;

#[inline]
pub const fn DP_CAP_CAPABILITY(cap: u32) -> u32 { cap & 3 }
#[inline]
pub const fn DP_CAP_DP_SIGNALLING(cap: u32) -> u32 { (cap >> 2) & 0xf }
#[inline]
pub const fn DP_CAP_DFP_D_PIN_ASSIGN(cap: u32) -> u32 { (cap >> 8) & 0xff }
#[inline]
pub const fn DP_CAP_UFP_D_PIN_ASSIGN(cap: u32) -> u32 { (cap >> 16) & 0xff }
#[inline]
pub const fn DP_CAP_PIN_ASSIGN_UFP_D(cap: u32) -> u32 {
    if cap & DP_CAP_RECEPTACLE != 0 { DP_CAP_UFP_D_PIN_ASSIGN(cap) } else { DP_CAP_DFP_D_PIN_ASSIGN(cap) }
}
#[inline]
pub const fn DP_CAP_PIN_ASSIGN_DFP_D(cap: u32) -> u32 {
    if cap & DP_CAP_RECEPTACLE != 0 { DP_CAP_DFP_D_PIN_ASSIGN(cap) } else { DP_CAP_UFP_D_PIN_ASSIGN(cap) }
}
#[inline]
pub const fn DP_CAP_CABLE_TYPE(cap: u32) -> u32 { (cap >> 28) & 3 }

pub const DP_STATUS_CON_DISABLED: u32 = 0;
pub const DP_STATUS_CON_DFP_D: u32 = 1;
pub const DP_STATUS_CON_UFP_D: u32 = 2;
pub const DP_STATUS_CON_BOTH: u32 = 3;
pub const DP_STATUS_POWER_LOW: u32 = 1 << 2;
pub const DP_STATUS_ENABLED: u32 = 1 << 3;
pub const DP_STATUS_PREFER_MULTI_FUNC: u32 = 1 << 4;
pub const DP_STATUS_SWITCH_TO_USB: u32 = 1 << 5;
pub const DP_STATUS_EXIT_DP_MODE: u32 = 1 << 6;
pub const DP_STATUS_HPD_STATE: u32 = 1 << 7;
pub const DP_STATUS_IRQ_HPD: u32 = 1 << 8;

#[inline]
pub const fn DP_STATUS_CONNECTION(status: u32) -> u32 { status & 3 }

pub const DP_CONF_UFP_U_AS_DFP_D: u32 = 1 << 0;
pub const DP_CONF_UFP_U_AS_UFP_D: u32 = 1 << 1;
pub const DP_CONF_SIGNALLING_MASK: u32 = 0x3c;
pub const DP_CONF_SIGNALLING_SHIFT: u32 = 2;
pub const DP_CONF_SIGNALLING_HBR3: u32 = 1;
pub const DP_CONF_SIGNALLING_UHBR10: u32 = 2;
pub const DP_CONF_SIGNALLING_UHBR20: u32 = 3;
pub const DP_CONF_PIN_ASSIGNEMENT_SHIFT: u32 = 8;
pub const DP_CONF_PIN_ASSIGNEMENT_MASK: u32 = 0xff00;
pub const DP_CONF_UHBR13_5_SUPPORT: u32 = 1 << 26;
pub const DP_CONF_CABLE_TYPE_MASK: u32 = 0x30000000;
pub const DP_CONF_CABLE_TYPE_SHIFT: u32 = 28;
pub const DP_CONF_CABLE_TYPE_PASSIVE: u32 = 0;
pub const DP_CONF_CABLE_TYPE_RE_TIMER: u32 = 1;
pub const DP_CONF_CABLE_TYPE_RE_DRIVER: u32 = 2;
pub const DP_CONF_CABLE_TYPE_OPTICAL: u32 = 3;
pub const DP_CONF_DPAM_VERSION: u32 = 1 << 30;

#[inline]
pub const fn DP_CONF_CURRENTLY(conf: u32) -> u32 { conf & 3 }
#[inline]
pub const fn DP_CONF_SET_PIN_ASSIGN(a: u32) -> u32 { a << 8 }
#[inline]
pub const fn DP_CONF_GET_PIN_ASSIGN(conf: u32) -> u32 { (conf >> 8) & 0xff }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
