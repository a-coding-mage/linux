/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependency: dcn20/dcn20_opp.h supplies the DCN2.0 register and field lists
// used by the original C macros below.

// OPP_REG_VARIABLE_LIST_DCN3_5 expands the DCN2.0 register list followed by
// these DCN3.5-specific registers.
macro_rules! OPP_REG_VARIABLE_LIST_DCN3_5 {
    () => {
        OPP_REG_VARIABLE_LIST_DCN2_0!();
        pub OPP_TOP_CLK_CONTROL: u32;
        pub OPP_ABM_CONTROL: u32;
    };
}

// OPP_MASK_SH_LIST_DCN35(mask_sh) extends the DCN2.0 list with the DCN3.5
// field definition.
macro_rules! OPP_MASK_SH_LIST_DCN35 {
    ($mask_sh:ident) => {
        OPP_MASK_SH_LIST_DCN20!($mask_sh),
        OPP_SF!(OPP_TOP_CLK_CONTROL, OPP_FGCG_REP_DIS, $mask_sh)
    };
}

// OPP_DCN35_REG_FIELD_LIST(type) extends the DCN2.0 register-field list.
macro_rules! OPP_DCN35_REG_FIELD_LIST {
    ($type:ty) => {
        OPP_DCN20_REG_FIELD_LIST!($type);
        OPP_FGCG_REP_DIS: $type;
    };
}

#[repr(C)]
pub struct dcn35_opp_registers {
    OPP_REG_VARIABLE_LIST_DCN3_5!();
}

#[repr(C)]
pub struct dcn35_opp_shift {
    OPP_DCN35_REG_FIELD_LIST!(u8);
}

#[repr(C)]
pub struct dcn35_opp_mask {
    OPP_DCN35_REG_FIELD_LIST!(u32);
}

extern "C" {
    pub fn dcn35_opp_construct(
        oppn20: *mut dcn20_opp,
        ctx: *mut dc_context,
        inst: u32,
        regs: *const dcn35_opp_registers,
        opp_shift: *const dcn35_opp_shift,
        opp_mask: *const dcn35_opp_mask,
    );

    pub fn dcn35_opp_set_fgcg(oppn20: *mut dcn20_opp, enable: bool);

    pub fn dcn35_opp_read_reg_state(
        opp: *mut output_pixel_processor,
        opp_reg_state: *mut dcn_opp_reg_state,
    );
}

// External types supplied by the included DCN2.0 and display headers.
pub enum dcn20_opp {}
pub enum dc_context {}
pub enum output_pixel_processor {}
pub enum dcn_opp_reg_state {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
