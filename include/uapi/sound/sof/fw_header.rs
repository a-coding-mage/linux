/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 */

/*
 * Firmware file format .
 */

// #include <linux/types.h>

pub const SND_SOF_FW_SIG_SIZE: usize = 4;
pub const SND_SOF_FW_ABI: u32 = 1;
pub const SND_SOF_FW_SIG: &[u8; SND_SOF_FW_SIG_SIZE] = b"Reef";

/*
 * Firmware module is made up of 1 . N blocks of different types. The
 * Block header is used to determine where and how block is to be copied in the
 * DSP/host memory space.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum snd_sof_fw_blk_type {
    SOF_FW_BLK_TYPE_INVALID = -1,
    SOF_FW_BLK_TYPE_START = 0,
    SOF_FW_BLK_TYPE_RSRVD0 = Self::SOF_FW_BLK_TYPE_START as isize,
    SOF_FW_BLK_TYPE_IRAM = 1, /* local instruction RAM */
    SOF_FW_BLK_TYPE_DRAM = 2, /* local data RAM */
    SOF_FW_BLK_TYPE_SRAM = 3, /* system RAM */
    SOF_FW_BLK_TYPE_ROM = 4,
    SOF_FW_BLK_TYPE_IMR = 5,
    SOF_FW_BLK_TYPE_RSRVD6 = 6,
    SOF_FW_BLK_TYPE_RSRVD7 = 7,
    SOF_FW_BLK_TYPE_RSRVD8 = 8,
    SOF_FW_BLK_TYPE_RSRVD9 = 9,
    SOF_FW_BLK_TYPE_RSRVD10 = 10,
    SOF_FW_BLK_TYPE_RSRVD11 = 11,
    SOF_FW_BLK_TYPE_RSRVD12 = 12,
    SOF_FW_BLK_TYPE_RSRVD13 = 13,
    SOF_FW_BLK_TYPE_RSRVD14 = 14,
    /* use SOF_FW_BLK_TYPE_RSVRDX for new block types */
    SOF_FW_BLK_TYPE_NUM,
}

#[repr(C, packed)]
pub struct snd_sof_blk_hdr {
    pub type_: snd_sof_fw_blk_type,
    pub size: u32,   /* bytes minus this header */
    pub offset: u32, /* offset from base */
}

/*
 * Firmware file is made up of 1 .. N different modules types. The module
 * type is used to determine how to load and parse the module.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum snd_sof_fw_mod_type {
    SOF_FW_BASE = 0,   /* base firmware image */
    SOF_FW_MODULE = 1, /* firmware module */
}

#[repr(C, packed)]
pub struct snd_sof_mod_hdr {
    pub type_: snd_sof_fw_mod_type,
    pub size: u32,       /* bytes minus this header */
    pub num_blocks: u32, /* number of blocks */
}

/*
 * Firmware file header.
 */
#[repr(C, packed)]
pub struct snd_sof_fw_header {
    pub sig: [u8; SND_SOF_FW_SIG_SIZE], /* "Reef" */
    pub file_size: u32,                /* size of file minus this header */
    pub num_modules: u32,              /* number of modules */
    pub abi: u32,                      /* version of header format */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
