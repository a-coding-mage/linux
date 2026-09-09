/* SPDX-License-Identifier: GPL-2.0-or-later */

// The ORC_REG_* registers are base registers which are used to find other
// registers on the stack.
//
// ORC_REG_PREV_SP, also known as DWARF Call Frame Address (CFA), is the
// address of the previous frame: the caller's SP before it called the current
// function.
//
// ORC_REG_UNDEFINED means the corresponding register's value didn't change in
// the current frame.
//
// The most commonly used base registers are SP and FP -- which the previous SP
// is usually based on -- and PREV_SP and UNDEFINED -- which the previous FP is
// usually based on.
//
// The rest of the base registers are needed for special cases like entry code
// and GCC realigned stacks.
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
 * This struct is more or less a vastly simplified version of the DWARF Call
 * Frame Information standard.  It contains only the necessary parts of DWARF
 * CFI, simplified for ease of access by the in-kernel unwinder.  It tells the
 * unwinder how to find the previous SP and FP (and sometimes entry regs) on
 * the stack for a given code address.  Each instance of the struct corresponds
 * to one or more code locations.
 *
 * The final five C bit-fields occupy one u32 in declaration order. Rust has no
 * native bit-field syntax, so their storage and bit operations are represented
 * explicitly below.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OrcEntry {
    pub sp_offset: i16,
    pub fp_offset: i16,
    pub ra_offset: i16,
    pub regs: u32,
}

impl OrcEntry {
    pub const fn new(sp_offset: i16, fp_offset: i16, ra_offset: i16, regs: u32) -> Self {
        Self { sp_offset, fp_offset, ra_offset, regs }
    }

    pub const fn sp_reg(&self) -> u32 { self.regs & 0x0f }
    pub const fn fp_reg(&self) -> u32 { (self.regs >> 4) & 0x0f }
    pub const fn ra_reg(&self) -> u32 { (self.regs >> 8) & 0x0f }
    pub const fn entry_type(&self) -> u32 { (self.regs >> 12) & 0x07 }
    pub const fn signal(&self) -> u32 { (self.regs >> 15) & 0x01 }

    pub fn set_sp_reg(&mut self, value: u32) { self.regs = (self.regs & !0x0f) | (value & 0x0f); }
    pub fn set_fp_reg(&mut self, value: u32) { self.regs = (self.regs & !(0x0f << 4)) | ((value & 0x0f) << 4); }
    pub fn set_ra_reg(&mut self, value: u32) { self.regs = (self.regs & !(0x0f << 8)) | ((value & 0x0f) << 8); }
    pub fn set_type(&mut self, value: u32) { self.regs = (self.regs & !(0x07 << 12)) | ((value & 0x07) << 12); }
    pub fn set_signal(&mut self, value: u32) { self.regs = (self.regs & !(0x01 << 15)) | ((value & 0x01) << 15); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
