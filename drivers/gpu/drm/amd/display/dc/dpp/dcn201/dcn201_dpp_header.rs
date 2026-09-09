/* Copyright 2016 Advanced Micro Devices, Inc.
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

// Dependency supplied by the DCN20 header.

macro_rules! TO_DCN201_DPP {
    ($dpp:expr) => {
        container_of!($dpp, dcn201_dpp, base)
    };
}

macro_rules! TF_REG_LIST_DCN201 {
    ($id:expr) => {
        TF_REG_LIST_DCN20!($id)
    };
}

macro_rules! TF_REG_LIST_SH_MASK_DCN201 {
    ($mask_sh:expr) => {
        TF_REG_LIST_SH_MASK_DCN20!($mask_sh)
    };
}

macro_rules! TF_REG_FIELD_LIST_DCN201 {
    ($type:ty) => {
        TF_REG_FIELD_LIST_DCN2_0!($type)
    };
}

#[repr(C)]
pub struct dcn201_dpp_shift {
    TF_REG_FIELD_LIST_DCN201!(u8);
}

#[repr(C)]
pub struct dcn201_dpp_mask {
    TF_REG_FIELD_LIST_DCN201!(u32);
}

macro_rules! DPP_DCN201_REG_VARIABLE_LIST {
    () => {
        DPP_DCN2_REG_VARIABLE_LIST!()
    };
}

#[repr(C)]
pub struct dcn201_dpp_registers {
    DPP_DCN201_REG_VARIABLE_LIST!();
}

#[repr(C)]
pub struct dcn201_dpp {
    pub base: dpp,

    pub tf_regs: *const dcn201_dpp_registers,
    pub tf_shift: *const dcn201_dpp_shift,
    pub tf_mask: *const dcn201_dpp_mask,

    pub filter_v: *const u16,
    pub filter_h: *const u16,
    pub filter_v_c: *const u16,
    pub filter_h_c: *const u16,
    pub lb_pixel_depth_supported: i32,
    pub lb_memory_size: i32,
    pub lb_bits_per_entry: i32,
    pub is_write_to_ram_a_safe: bool,
    pub scl_data: scaler_data,
    pub pwl_data: pwl_params,
}

extern "C" {
    pub fn dpp201_construct(
        dpp2: *mut dcn201_dpp,
        ctx: *mut dc_context,
        inst: u32,
        tf_regs: *const dcn201_dpp_registers,
        tf_shift: *const dcn201_dpp_shift,
        tf_mask: *const dcn201_dpp_mask,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
