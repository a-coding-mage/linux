/*
 * Copyright 2021 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

// Dependencies are supplied by the surrounding translation unit.

pub const DCN_BASE__INST0_SEG0: u32 = 0x00000012;
pub const DCN_BASE__INST0_SEG1: u32 = 0x000000C0;
pub const DCN_BASE__INST0_SEG2: u32 = 0x000034C0;
pub const DCN_BASE__INST0_SEG3: u32 = 0x00009000;
pub const DCN_BASE__INST0_SEG4: u32 = 0x02403C00;
pub const DCN_BASE__INST0_SEG5: u32 = 0;

// C token-pasting macro equivalents are retained as declarative hooks for
// the register definitions supplied by the dependent headers.
#[macro_export]
macro_rules! BASE_INNER {
    (0) => { DCN_BASE__INST0_SEG0 };
    (1) => { DCN_BASE__INST0_SEG1 };
    (2) => { DCN_BASE__INST0_SEG2 };
    (3) => { DCN_BASE__INST0_SEG3 };
    (4) => { DCN_BASE__INST0_SEG4 };
    (5) => { DCN_BASE__INST0_SEG5 };
}

// CTX dmub
// REGS dmub->regs_dcn31
// REG_OFFSET_EXP(reg_name) = BASE(reg_name_BASE_IDX) + reg_name

/* Registers. */

pub static dmub_srv_dcn316_regs: dmub_srv_dcn31_regs = dmub_srv_dcn31_regs {
    regs: [
        DMUB_DCN31_REGS!()
        DMCUB_INTERNAL_REGS!()
    ],
    fields_mask: [
        DMUB_DCN31_FIELDS_MASK!()
    ],
    fields_shift: [
        DMUB_DCN31_FIELDS_SHIFT!()
    ],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
