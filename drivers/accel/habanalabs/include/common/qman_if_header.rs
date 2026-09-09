/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2018 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

/*
 * PRIMARY QUEUE
 */

#[repr(C)]
pub struct hl_bd {
    pub ptr: u64,
    pub len: u32,
    pub ctl: u32,
}

pub const HL_BD_SIZE: usize = core::mem::size_of::<hl_bd>();

/*
 * S/W CTL FIELDS.
 *
 * BD_CTL_REPEAT_VALID tells the CP whether the repeat field in the BD CTL is
 * valid. 1 means the repeat field is valid, 0 means not-valid,
 * i.e. repeat == 1
 */
pub const BD_CTL_REPEAT_VALID_SHIFT: u32 = 24;
pub const BD_CTL_REPEAT_VALID_MASK: u32 = 0x01000000;

pub const BD_CTL_SHADOW_INDEX_SHIFT: u32 = 0;
pub const BD_CTL_SHADOW_INDEX_MASK: u32 = 0x00000FFF;

/*
 * H/W CTL FIELDS
 */

pub const BD_CTL_COMP_OFFSET_SHIFT: u32 = 16;
pub const BD_CTL_COMP_OFFSET_MASK: u32 = 0x0FFF0000;

pub const BD_CTL_COMP_DATA_SHIFT: u32 = 0;
pub const BD_CTL_COMP_DATA_MASK: u32 = 0x0000FFFF;

/*
 * COMPLETION QUEUE
 */

#[repr(C)]
pub struct hl_cq_entry {
    pub data: u32,
}

pub const HL_CQ_ENTRY_SIZE: usize = core::mem::size_of::<hl_cq_entry>();

pub const CQ_ENTRY_READY_SHIFT: u32 = 31;
pub const CQ_ENTRY_READY_MASK: u32 = 0x80000000;

pub const CQ_ENTRY_SHADOW_INDEX_VALID_SHIFT: u32 = 30;
pub const CQ_ENTRY_SHADOW_INDEX_VALID_MASK: u32 = 0x40000000;

pub const CQ_ENTRY_SHADOW_INDEX_SHIFT: u32 = BD_CTL_SHADOW_INDEX_SHIFT;
pub const CQ_ENTRY_SHADOW_INDEX_MASK: u32 = BD_CTL_SHADOW_INDEX_MASK;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
