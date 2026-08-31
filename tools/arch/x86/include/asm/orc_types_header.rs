/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2017 Josh Poimboeuf <jpoimboe@redhat.com>
 */

// C header dependencies:
// #include <linux/types.h>
// #include <linux/compiler.h>

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

// C header dependency, only when not assembling:
// #include <asm/byteorder.h>

/*
 * This struct is more or less a vastly simplified version of the DWARF Call
 * Frame Information standard.  It contains only the necessary parts of DWARF
 * CFI, simplified for ease of access by the in-kernel unwinder.  It tells the
 * unwinder how to find the previous SP and BP (and sometimes entry regs) on
 * the stack for a given code address.  Each instance of the struct corresponds
 * to one or more code locations.
 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct orc_entry {
    pub sp_offset: i16,
    pub bp_offset: i16,

    /*
     * C bitfield layout:
     *
     * #if defined(__LITTLE_ENDIAN_BITFIELD)
     * unsigned sp_reg:4;
     * unsigned bp_reg:4;
     * unsigned type:3;
     * unsigned signal:1;
     * #elif defined(__BIG_ENDIAN_BITFIELD)
     * unsigned bp_reg:4;
     * unsigned sp_reg:4;
     * unsigned unused:4;
     * unsigned signal:1;
     * unsigned type:3;
     * #endif
     *
     * Rust has no native C-compatible bitfields, so this field preserves the
     * packed storage for those bitfields.
     */
    pub bitfields: u16,
}

#[cfg(target_endian = "little")]
pub const ORC_ENTRY_SP_REG_MASK: u16 = 0x000f;
#[cfg(target_endian = "little")]
pub const ORC_ENTRY_BP_REG_MASK: u16 = 0x00f0;
#[cfg(target_endian = "little")]
pub const ORC_ENTRY_TYPE_MASK: u16 = 0x0700;
#[cfg(target_endian = "little")]
pub const ORC_ENTRY_SIGNAL_MASK: u16 = 0x0800;

#[cfg(target_endian = "little")]
pub const ORC_ENTRY_SP_REG_SHIFT: u32 = 0;
#[cfg(target_endian = "little")]
pub const ORC_ENTRY_BP_REG_SHIFT: u32 = 4;
#[cfg(target_endian = "little")]
pub const ORC_ENTRY_TYPE_SHIFT: u32 = 8;
#[cfg(target_endian = "little")]
pub const ORC_ENTRY_SIGNAL_SHIFT: u32 = 11;

#[cfg(target_endian = "big")]
pub const ORC_ENTRY_BP_REG_MASK: u16 = 0xf000;
#[cfg(target_endian = "big")]
pub const ORC_ENTRY_SP_REG_MASK: u16 = 0x0f00;
#[cfg(target_endian = "big")]
pub const ORC_ENTRY_UNUSED_MASK: u16 = 0x00f0;
#[cfg(target_endian = "big")]
pub const ORC_ENTRY_SIGNAL_MASK: u16 = 0x0008;
#[cfg(target_endian = "big")]
pub const ORC_ENTRY_TYPE_MASK: u16 = 0x0007;

#[cfg(target_endian = "big")]
pub const ORC_ENTRY_BP_REG_SHIFT: u32 = 12;
#[cfg(target_endian = "big")]
pub const ORC_ENTRY_SP_REG_SHIFT: u32 = 8;
#[cfg(target_endian = "big")]
pub const ORC_ENTRY_UNUSED_SHIFT: u32 = 4;
#[cfg(target_endian = "big")]
pub const ORC_ENTRY_SIGNAL_SHIFT: u32 = 3;
#[cfg(target_endian = "big")]
pub const ORC_ENTRY_TYPE_SHIFT: u32 = 0;
