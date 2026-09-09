/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding DMUB implementation:
// `dmub_srv_common_regs`, `REG_OFFSET`, `FD_MASK`, `FD_SHIFT`,
// `DMUB_COMMON_REGS`, `DMCUB_INTERNAL_REGS`, and `DMUB_COMMON_FIELDS`.

// C preprocessor aliases retained as Rust-side dependency names.
// BASE_INNER(seg) expands to DMU_BASE__INST0_SEG##seg.
// CTX expands to dmub; REGS expands to dmub->regs.

/* Registers. */

pub static dmub_srv_dcn21_regs: dmub_srv_common_regs = dmub_srv_common_regs {
    regs: [
        DMUB_COMMON_REGS!(),
        DMCUB_INTERNAL_REGS!(),
    ],
    masks: [
        DMUB_COMMON_FIELDS!().map(|(reg, field)| FD_MASK(reg, field)),
    ],
    shifts: [
        DMUB_COMMON_FIELDS!().map(|(reg, field)| FD_SHIFT(reg, field)),
    ],
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
