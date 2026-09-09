/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2008-2009 Texas Instruments Inc
 */

// The declarations below are enabled by the original C header only when
// __KERNEL__ is defined.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vpfe_pin_pol {
    VPFE_PINPOL_POSITIVE,
    VPFE_PINPOL_NEGATIVE,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum vpfe_hw_if_type {
    /* BT656 - 8 bit */
    VPFE_BT656,
    /* BT1120 - 16 bit */
    VPFE_BT1120,
    /* Raw Bayer */
    VPFE_RAW_BAYER,
    /* YCbCr - 8 bit with external sync */
    VPFE_YCBCR_SYNC_8,
    /* YCbCr - 16 bit with external sync */
    VPFE_YCBCR_SYNC_16,
    /* BT656 - 10 bit */
    VPFE_BT656_10BIT,
}

/* interface description */
#[repr(C)]
pub struct vpfe_hw_if_param {
    pub if_type: vpfe_hw_if_type,
    pub hdpol: vpfe_pin_pol,
    pub vdpol: vpfe_pin_pol,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
