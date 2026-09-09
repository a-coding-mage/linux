/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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
 */

// C dependencies:
// ../dmub_srv.h, dmub_reg.h, dmub_dcn301.h
// dcn/dcn_3_0_1_offset.h, dcn/dcn_3_0_1_sh_mask.h, vangogh_ip_offset.h

// #define BASE_INNER(seg) DCN_BASE__INST0_SEG##seg
// #define CTX dmub
// #define REGS dmub->regs

/* Registers. */

pub static dmub_srv_dcn301_regs: dmub_srv_common_regs = dmub_srv_common_regs {
    regs: [
        // C macro expansion: DMUB_COMMON_REGS(), with DMUB_SR(reg) => REG_OFFSET(reg)
        DMUB_COMMON_REGS!()
        // C macro expansion: DMCUB_INTERNAL_REGS(), with DMUB_SR(reg) => REG_OFFSET(reg)
        DMCUB_INTERNAL_REGS!()
    ],
    fields: [
        // C macro expansion: DMUB_COMMON_FIELDS(), with DMUB_SF(reg, field) => FD_MASK(reg, field)
        DMUB_COMMON_FIELDS_MASK!()
    ],
    shifts: [
        // C macro expansion: DMUB_COMMON_FIELDS(), with DMUB_SF(reg, field) => FD_SHIFT(reg, field)
        DMUB_COMMON_FIELDS_SHIFT!()
    ],
};

/* Shared functions. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
