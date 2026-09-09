/* Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Dependency declarations supplied by dcn20/dcn20_opp.h are intentionally
// left external to this translation unit.

// C macro: container_of(opp, struct dcn201_opp, base)
// C macro: .field_name = reg_name ## __ ## field_name ## post_fix
// C macro: OPP_REG_LIST_DCN10(id), OPP_DPG_REG_LIST(id),
//          SRI(FMT_422_CONTROL, FMT, id)
// C macro: OPP_MASK_SH_LIST_DCN20(mask_sh)
// C macro: OPP_DCN20_REG_FIELD_LIST(type)

#[repr(C)]
pub struct dcn201_opp_shift {
    // OPP_DCN20_REG_FIELD_LIST(u8);
}

#[repr(C)]
pub struct dcn201_opp_mask {
    // OPP_DCN20_REG_FIELD_LIST(u32);
}

#[repr(C)]
pub struct dcn201_opp_registers {
    // OPP_REG_VARIABLE_LIST_DCN2_0;
}

#[repr(C)]
pub struct dcn201_opp {
    pub base: output_pixel_processor,
    pub regs: *const dcn201_opp_registers,
    pub opp_shift: *const dcn201_opp_shift,
    pub opp_mask: *const dcn201_opp_mask,
    pub is_write_to_ram_a_safe: bool,
}

extern "C" {
    pub fn dcn201_opp_construct(
        oppn201: *mut dcn201_opp,
        ctx: *mut dc_context,
        inst: u32,
        regs: *const dcn201_opp_registers,
        opp_shift: *const dcn201_opp_shift,
        opp_mask: *const dcn201_opp_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
