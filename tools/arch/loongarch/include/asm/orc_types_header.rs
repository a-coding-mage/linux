/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * Dependency intent from C header:
 * #include <linux/types.h>
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
 * The most commonly used base registers are SP and FP -- which the previous SP
 * is usually based on -- and PREV_SP and UNDEFINED -- which the previous FP is
 * usually based on.
 *
 * The rest of the base registers are needed for special cases like entry code
 * and GCC realigned stacks.
 */
pub const ORC_REG_UNDEFINED: u32 = 0;
pub const ORC_REG_PREV_SP: u32 = 1;
pub const ORC_REG_SP: u32 = 2;
pub const ORC_REG_FP: u32 = 3;
pub const ORC_REG_MAX: u32 = 4;

pub const ORC_TYPE_UNDEFINED: u32 = 0;
pub const ORC_TYPE_END_OF_STACK: u32 = 1;
pub const ORC_TYPE_CALL: u32 = 2;
pub const ORC_TYPE_REGS: u32 = 3;
pub const ORC_TYPE_REGS_PARTIAL: u32 = 4;

/*
 * C conditional intent:
 * The following type is excluded when __ASSEMBLER__ is defined.
 */

/*
 * This struct is more or less a vastly simplified version of the DWARF Call
 * Frame Information standard.  It contains only the necessary parts of DWARF
 * CFI, simplified for ease of access by the in-kernel unwinder.  It tells the
 * unwinder how to find the previous SP and FP (and sometimes entry regs) on
 * the stack for a given code address.  Each instance of the struct corresponds
 * to one or more code locations.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orc_entry {
    pub sp_offset: i16,
    pub fp_offset: i16,
    pub ra_offset: i16,
    pub bitfield_1: u16,
}

impl orc_entry {
    #[inline]
    pub const fn sp_reg(&self) -> u32 {
        (self.bitfield_1 & 0x000f) as u32
    }

    #[inline]
    pub fn set_sp_reg(&mut self, value: u32) {
        self.bitfield_1 = (self.bitfield_1 & !0x000f) | ((value as u16) & 0x000f);
    }

    #[inline]
    pub const fn fp_reg(&self) -> u32 {
        ((self.bitfield_1 >> 4) & 0x000f) as u32
    }

    #[inline]
    pub fn set_fp_reg(&mut self, value: u32) {
        self.bitfield_1 = (self.bitfield_1 & !0x00f0) | (((value as u16) & 0x000f) << 4);
    }

    #[inline]
    pub const fn ra_reg(&self) -> u32 {
        ((self.bitfield_1 >> 8) & 0x000f) as u32
    }

    #[inline]
    pub fn set_ra_reg(&mut self, value: u32) {
        self.bitfield_1 = (self.bitfield_1 & !0x0f00) | (((value as u16) & 0x000f) << 8);
    }

    #[inline]
    pub const fn r#type(&self) -> u32 {
        ((self.bitfield_1 >> 12) & 0x0007) as u32
    }

    #[inline]
    pub fn set_type(&mut self, value: u32) {
        self.bitfield_1 = (self.bitfield_1 & !0x7000) | (((value as u16) & 0x0007) << 12);
    }

    #[inline]
    pub const fn signal(&self) -> u32 {
        ((self.bitfield_1 >> 15) & 0x0001) as u32
    }

    #[inline]
    pub fn set_signal(&mut self, value: u32) {
        self.bitfield_1 = (self.bitfield_1 & !0x8000) | (((value as u16) & 0x0001) << 15);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
