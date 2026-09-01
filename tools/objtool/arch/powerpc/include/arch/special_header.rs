/* SPDX-License-Identifier: GPL-2.0-or-later */

pub const EX_ENTRY_SIZE: i32 = 8;
pub const EX_ORIG_OFFSET: i32 = 0;
pub const EX_NEW_OFFSET: i32 = 4;

pub const JUMP_ENTRY_SIZE: i32 = 16;
pub const JUMP_ORIG_OFFSET: i32 = 0;
pub const JUMP_NEW_OFFSET: i32 = 4;
pub const JUMP_KEY_OFFSET: i32 = 8;

pub const ALT_ENTRY_SIZE: i32 = 12;
pub const ALT_ORIG_OFFSET: i32 = 0;
pub const ALT_NEW_OFFSET: i32 = 4;
pub const ALT_FEATURE_OFFSET: i32 = 8;
pub const ALT_ORIG_LEN_OFFSET: i32 = 10;
pub const ALT_NEW_LEN_OFFSET: i32 = 11;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
