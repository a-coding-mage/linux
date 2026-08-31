// SPDX-License-Identifier: GPL-2.0-or-later

/*
 * See more info about struct exception_table_entry
 * in arch/loongarch/include/asm/extable.h
 */
pub const EX_ENTRY_SIZE: usize = 12;
pub const EX_ORIG_OFFSET: usize = 0;
pub const EX_NEW_OFFSET: usize = 4;

/*
 * See more info about struct jump_entry
 * in include/linux/jump_label.h
 */
pub const JUMP_ENTRY_SIZE: usize = 16;
pub const JUMP_ORIG_OFFSET: usize = 0;
pub const JUMP_NEW_OFFSET: usize = 4;
pub const JUMP_KEY_OFFSET: usize = 8;

/*
 * See more info about struct alt_instr
 * in arch/loongarch/include/asm/alternative.h
 */
pub const ALT_ENTRY_SIZE: usize = 12;
pub const ALT_ORIG_OFFSET: usize = 0;
pub const ALT_NEW_OFFSET: usize = 4;
pub const ALT_FEATURE_OFFSET: usize = 8;
pub const ALT_ORIG_LEN_OFFSET: usize = 10;
pub const ALT_NEW_LEN_OFFSET: usize = 11;
