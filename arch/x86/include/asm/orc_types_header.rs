/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2017 Josh Poimboeuf <jpoimboe@redhat.com>
 */

/*
 * The ORC_REG_* registers are base registers which are used to find other
 * registers on the stack.
 *
 * ORC_REG_PREV_SP, also known as DWARF Call Frame Address (CFA), is the
 * address of the previous frame: the caller's SP before it called the current
 * function.
 *
 * ORC_REG_UNDEFINED means the corresponding register's value didn't change in
 * the current frame.
 *
 * The most commonly used base registers are SP and BP -- which the previous SP
 * is usually based on -- and PREV_SP and UNDEFINED -- which the previous BP is
 * usually based on.
 *
 * The rest of the base registers are needed for special cases like entry code
 * and GCC realigned stacks.
 */
pub const ORC_REG_UNDEFINED: u32 = 0;
pub const ORC_REG_AX: u32 = 1;
pub const ORC_REG_DX: u32 = 2;
pub const ORC_REG_SP: u32 = 3;
pub const ORC_REG_BP: u32 = 4;
pub const ORC_REG_DI: u32 = 5;
pub const ORC_REG_R10: u32 = 6;
pub const ORC_REG_R13: u32 = 7;
pub const ORC_REG_PREV_SP: u32 = 8;
pub const ORC_REG_SP_INDIRECT: u32 = 9;
pub const ORC_REG_BP_INDIRECT: u32 = 10;
pub const ORC_REG_MAX: u32 = 15;

pub const ORC_TYPE_UNDEFINED: u32 = 0;
pub const ORC_TYPE_END_OF_STACK: u32 = 1;
pub const ORC_TYPE_CALL: u32 = 2;
pub const ORC_TYPE_REGS: u32 = 3;
pub const ORC_TYPE_REGS_PARTIAL: u32 = 4;

/*
 * This struct is more or less a vastly simplified version of the DWARF Call
 * Frame Information standard.  It contains only the necessary parts of DWARF
 * CFI, simplified for ease of access by the in-kernel unwinder.  It tells the
 * unwinder how to find the previous SP and BP (and sometimes entry regs) on
 * the stack for a given code address.  Each instance of the struct corresponds
 * to one or more code locations.
 *
 * The final field stores the C bit-fields.  Its interpretation follows the
 * target's __LITTLE_ENDIAN_BITFIELD or __BIG_ENDIAN_BITFIELD layout.
 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct orc_entry {
    pub sp_offset: i16,
    pub bp_offset: i16,
    pub bits: u16,
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
