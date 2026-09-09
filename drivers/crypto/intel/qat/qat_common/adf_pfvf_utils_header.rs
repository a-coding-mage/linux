/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2021 Intel Corporation */

// Dependency intent from the C header:
// #include <linux/types.h>
// #include "adf_pfvf_msg.h"

/* How long to wait for far side to acknowledge receipt */
pub const ADF_PFVF_MSG_ACK_DELAY_US: u32 = 4;
pub const ADF_PFVF_MSG_ACK_MAX_DELAY_US: u32 = 1 * USEC_PER_SEC;

unsafe extern "C" {
    pub fn adf_pfvf_calc_blkmsg_crc(buf: *const u8, buf_len: u8) -> u8;
    pub fn adf_pfvf_crc_init();

    pub fn adf_pfvf_csr_msg_of(
        accel_dev: *mut adf_accel_dev,
        msg: pfvf_message,
        fmt: *const pfvf_csr_format,
    ) -> u32;
    pub fn adf_pfvf_message_of(
        accel_dev: *mut adf_accel_dev,
        raw_msg: u32,
        fmt: *const pfvf_csr_format,
    ) -> pfvf_message;
}

#[repr(C)]
pub struct pfvf_field_format {
    pub offset: u8,
    pub mask: u32,
}

#[repr(C)]
pub struct pfvf_csr_format {
    pub r#type: pfvf_field_format,
    pub data: pfvf_field_format,
}

pub unsafe fn adf_vf_compat_checker(vf_compat_ver: u8) -> u8 {
    if vf_compat_ver == 0 {
        return ADF_PF2VF_VF_INCOMPATIBLE;
    }

    if vf_compat_ver <= ADF_PFVF_COMPAT_THIS_VERSION {
        return ADF_PF2VF_VF_COMPATIBLE;
    }

    ADF_PF2VF_VF_COMPAT_UNKNOWN
}

// External types and constants supplied by adf_pfvf_msg.h and other dependencies.
#[allow(non_camel_case_types)]
pub enum adf_accel_dev {}

// These names are intentionally left as external dependencies, matching the C header.
// USEC_PER_SEC
// pfvf_message
// ADF_PF2VF_VF_INCOMPATIBLE
// ADF_PFVF_COMPAT_THIS_VERSION
// ADF_PF2VF_VF_COMPATIBLE
// ADF_PF2VF_VF_COMPAT_UNKNOWN

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
