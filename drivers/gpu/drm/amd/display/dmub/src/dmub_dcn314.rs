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
 */

// Dependencies supplied by the surrounding DMUB implementation:
// ../dmub_srv.h, dmub_reg.h, dmub_dcn314.h,
// dcn/dcn_3_1_4_offset.h, and dcn/dcn_3_1_4_sh_mask.h.

const DCN_BASE__INST0_SEG0: u32 = 0x00000012;
const DCN_BASE__INST0_SEG1: u32 = 0x000000c0;
const DCN_BASE__INST0_SEG2: u32 = 0x000034c0;
const DCN_BASE__INST0_SEG3: u32 = 0x00009000;
const DCN_BASE__INST0_SEG4: u32 = 0x02403c00;
const DCN_BASE__INST0_SEG5: u32 = 0;

// C preprocessor definitions retained as Rust-side dependency expressions.
// BASE, register offset/field tables, and dmub_srv_dcn31_regs are defined by
// the corresponding platform headers/bindings.

pub static dmub_srv_dcn314_regs: dmub_srv_dcn31_regs = dmub_srv_dcn31_regs {
    regs: [DMUB_DCN31_REGS!(), DMCUB_INTERNAL_REGS!()],
    field_masks: [DMUB_DCN31_FIELDS_MASKS!()],
    field_shifts: [DMUB_DCN31_FIELDS_SHIFTS!()],
};

pub unsafe fn dmub_dcn314_is_psrsu_supported(dmub: *const dmub_srv) -> bool {
    (*dmub).fw_version >= DMUB_FW_VERSION!(8, 0, 16)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
