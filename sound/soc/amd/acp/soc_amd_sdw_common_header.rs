// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2024 Advanced Micro Devices, Inc. All rights reserved

/*
 * soc_amd_sdw_common.h - prototypes for common helpers
 */

// C dependencies:
// #include <linux/bits.h>
// #include <linux/types.h>
// #include <sound/soc.h>
// #include <sound/soc_sdw_utils.h>

use core::ffi::{c_int, c_uint};

pub const ACP63_SDW_MAX_CPU_DAIS: c_uint = 8;
pub const ACP63_SDW_MAX_LINKS: c_uint = 2;

pub const AMD_SDW_MAX_GROUPS: c_uint = 9;
pub const ACP63_PCI_REV: c_uint = 0x63;
pub const ACP70_PCI_REV: c_uint = 0x70;
pub const ACP71_PCI_REV: c_uint = 0x71;
pub const ACP72_PCI_REV: c_uint = 0x72;

pub const fn bit(nr: c_uint) -> c_uint {
    1u32.wrapping_shl(nr) as c_uint
}

pub const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    let all = !0u32;
    (all.wrapping_shl(l) & all.wrapping_shr(31u32.wrapping_sub(h))) as c_uint
}

pub const fn SOC_JACK_JDSRC(quirk: c_uint) -> c_uint {
    quirk & genmask(3, 0)
}

pub const ASOC_SDW_FOUR_SPK: c_uint = bit(4);
pub const ASOC_SDW_ACP_DMIC: c_uint = bit(5);
pub const ASOC_SDW_CODEC_SPKR: c_uint = bit(15);

pub const AMD_SDW0: c_uint = 0;
pub const AMD_SDW1: c_uint = 1;
pub const ACP63_SW0_AUDIO0_TX: c_uint = 0;
pub const ACP63_SW0_AUDIO1_TX: c_uint = 1;
pub const ACP63_SW0_AUDIO2_TX: c_uint = 2;

pub const ACP63_SW0_AUDIO0_RX: c_uint = 3;
pub const ACP63_SW0_AUDIO1_RX: c_uint = 4;
pub const ACP63_SW0_AUDIO2_RX: c_uint = 5;

pub const ACP63_SW1_AUDIO0_TX: c_uint = 0;
pub const ACP63_SW1_AUDIO0_RX: c_uint = 1;

pub const ACP_DMIC_BE_ID: c_uint = 4;

pub const ACP70_SW_AUDIO0_TX: c_uint = 0;
pub const ACP70_SW_AUDIO1_TX: c_uint = 1;
pub const ACP70_SW_AUDIO2_TX: c_uint = 2;

pub const ACP70_SW_AUDIO0_RX: c_uint = 3;
pub const ACP70_SW_AUDIO1_RX: c_uint = 4;
pub const ACP70_SW_AUDIO2_RX: c_uint = 5;

#[repr(C)]
pub struct amd_mc_ctx {
    pub acp_rev: c_uint,
    pub max_sdw_links: c_uint,
}

// Provided by external Linux kernel headers in the original C translation unit.
pub type device = crate::device;

unsafe extern "C" {
    pub fn get_acp63_cpu_pin_id(
        sdw_link_id: u32,
        be_id: c_int,
        cpu_pin_id: *mut c_int,
        dev: *mut device,
    ) -> c_int;

    pub fn get_acp70_cpu_pin_id(
        sdw_link_id: u32,
        be_id: c_int,
        cpu_pin_id: *mut c_int,
        dev: *mut device,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
