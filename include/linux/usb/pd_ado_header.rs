// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (c) 2017 Dialog Semiconductor
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 */

/* ADO : Alert Data Object */
pub const USB_PD_ADO_TYPE_SHIFT: u32 = 24;
pub const USB_PD_ADO_TYPE_MASK: u32 = 0xff;
pub const USB_PD_ADO_FIXED_BATT_SHIFT: u32 = 20;
pub const USB_PD_ADO_FIXED_BATT_MASK: u32 = 0xf;
pub const USB_PD_ADO_HOT_SWAP_BATT_SHIFT: u32 = 16;
pub const USB_PD_ADO_HOT_SWAP_BATT_MASK: u32 = 0xf;

pub const USB_PD_ADO_TYPE_BATT_STATUS_CHANGE: u32 = 1u32 << 1;
pub const USB_PD_ADO_TYPE_OCP: u32 = 1u32 << 2;
pub const USB_PD_ADO_TYPE_OTP: u32 = 1u32 << 3;
pub const USB_PD_ADO_TYPE_OP_COND_CHANGE: u32 = 1u32 << 4;
pub const USB_PD_ADO_TYPE_SRC_INPUT_CHANGE: u32 = 1u32 << 5;
pub const USB_PD_ADO_TYPE_OVP: u32 = 1u32 << 6;

#[inline]
pub fn usb_pd_ado_type(ado: u32) -> u32 {
    (ado >> USB_PD_ADO_TYPE_SHIFT) & USB_PD_ADO_TYPE_MASK
}

#[inline]
pub fn usb_pd_ado_fixed_batt(ado: u32) -> u32 {
    (ado >> USB_PD_ADO_FIXED_BATT_SHIFT) & USB_PD_ADO_FIXED_BATT_MASK
}

#[inline]
pub fn usb_pd_ado_hot_swap_batt(ado: u32) -> u32 {
    (ado >> USB_PD_ADO_HOT_SWAP_BATT_SHIFT) & USB_PD_ADO_HOT_SWAP_BATT_MASK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
