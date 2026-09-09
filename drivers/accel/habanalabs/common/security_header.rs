/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2022 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

// Translated from security.h. Linux-specific includes and header guards are
// intentionally omitted; their symbols are supplied by other dependencies.

use core::ffi::c_void;

// Opaque external types referenced by this header.
pub enum hl_device {}
pub enum range {}

/* special blocks */
pub const HL_GLBL_ERR_ADDRESS_MASK: u32 = (1u32 << 12) - 1;
/* GLBL_ERR_ADDR register offset from the start of the block */
pub const HL_GLBL_ERR_ADDR_OFFSET: u32 = 0xF44;
/* GLBL_ERR_CAUSE register offset from the start of the block */
pub const HL_GLBL_ERR_CAUSE_OFFSET: u32 = 0xF48;

/*
 * struct hl_special_block_info - stores address details of a particular type of
 * IP block which has a SPECIAL part.
 */
#[repr(C)]
pub struct hl_special_block_info {
    pub block_type: i32,
    pub base_addr: u32,
    pub major: u32,
    pub minor: u32,
    pub sub_minor: u32,
    pub major_offset: u32,
    pub minor_offset: u32,
    pub sub_minor_offset: u32,
}

/*
 * struct hl_automated_pb_cfg - represents configurations of a particular type
 * of IP block which has protection bits.
 */
#[repr(C)]
pub struct hl_automated_pb_cfg {
    pub addr: hl_special_block_info,
    pub prot_map: u32,
    pub data_map: u32,
    pub data: *const u32,
    pub data_size: u8,
}

/* struct hl_special_blocks_cfg - holds special blocks cfg data. */
#[repr(C)]
pub struct hl_special_blocks_cfg {
    pub priv_automated_pb_cfg: *mut hl_automated_pb_cfg,
    pub sec_automated_pb_cfg: *mut hl_automated_pb_cfg,
    pub skip_blocks_cfg: *mut hl_skip_blocks_cfg,
    pub priv_cfg_size: u32,
    pub sec_cfg_size: u32,
    pub prot_lvl_priv: u8,
}

/* Automated security */

/* struct hl_skip_blocks_cfg - holds arrays of block types & block ranges to be
 * excluded from special blocks configurations.
 */
#[repr(C)]
pub struct hl_skip_blocks_cfg {
    pub block_types: *mut i32,
    pub block_types_len: usize,
    pub block_ranges: *mut range,
    pub block_ranges_len: usize,
    pub skip_block_hook: Option<unsafe extern "C" fn(
        hdev: *mut hl_device,
        special_blocks_cfg: *mut hl_special_blocks_cfg,
        blk_idx: u32,
        major: u32,
        minor: u32,
        sub_minor: u32,
    ) -> bool>,
}

/**
 * struct iterate_special_ctx - HW module special block iterator
 */
#[repr(C)]
pub struct iterate_special_ctx {
    /* callback for the HW module special block iterator */
    pub fn_: Option<unsafe extern "C" fn(
        hdev: *mut hl_device,
        block_id: u32,
        major: u32,
        minor: u32,
        sub_minor: u32,
        data: *mut c_void,
    ) -> i32>,
    pub data: *mut c_void,
}

unsafe extern "C" {
    pub fn hl_iterate_special_blocks(
        hdev: *mut hl_device,
        ctx: *mut iterate_special_ctx,
    ) -> i32;
    pub fn hl_check_for_glbl_errors(hdev: *mut hl_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
