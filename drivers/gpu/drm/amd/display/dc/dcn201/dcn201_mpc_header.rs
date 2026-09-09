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

// Dependency declarations formerly supplied by dcn20/dcn20_mpc.h are
// intentionally left to the surrounding translation unit.

#[macro_export]
macro_rules! TO_DCN201_MPC {
    ($mpc_base:expr) => { container_of!($mpc_base, dcn201_mpc, base) };
}

#[macro_export]
macro_rules! MPC_REG_LIST_DCN201 {
    ($inst:tt) => { MPC_REG_LIST_DCN2_0!($inst) };
}

#[macro_export]
macro_rules! MPC_OUT_MUX_REG_LIST_DCN201 {
    ($inst:tt) => { MPC_OUT_MUX_REG_LIST_DCN2_0!($inst) };
}

#[macro_export]
macro_rules! MPC_REG_VARIABLE_LIST_DCN201 {
    () => { MPC_REG_VARIABLE_LIST_DCN2_0!() };
}

#[macro_export]
macro_rules! MPC_COMMON_MASK_SH_LIST_DCN201 {
    ($mask_sh:tt) => {
        MPC_COMMON_MASK_SH_LIST_DCN2_0!($mask_sh),
        SF!(MPC_OUT0_MUX, MPC_OUT_RATE_CONTROL, $mask_sh),
        SF!(MPC_OUT0_MUX, MPC_OUT_RATE_CONTROL_DISABLE, $mask_sh),
        SF!(MPC_OUT0_MUX, MPC_OUT_FLOW_CONTROL_MODE, $mask_sh),
        SF!(MPC_OUT0_MUX, MPC_OUT_FLOW_CONTROL_COUNT0, $mask_sh),
        SF!(MPC_OUT0_MUX, MPC_OUT_FLOW_CONTROL_COUNT1, $mask_sh)
    };
}

// The original macro expands the inherited DCN2.0 register fields followed
// by these five fields.
#[macro_export]
macro_rules! MPC_REG_FIELD_LIST_DCN201 {
    ($ty:ty) => {
        MPC_REG_FIELD_LIST_DCN2_0!($ty);
        MPC_OUT_RATE_CONTROL: $ty;
        MPC_OUT_RATE_CONTROL_DISABLE: $ty;
        MPC_OUT_FLOW_CONTROL_MODE: $ty;
        MPC_OUT_FLOW_CONTROL_COUNT0: $ty;
        MPC_OUT_FLOW_CONTROL_COUNT1: $ty;
    };
}

#[repr(C)]
pub struct dcn201_mpc_registers {
    MPC_REG_VARIABLE_LIST_DCN201!();
}

#[repr(C)]
pub struct dcn201_mpc_shift {
    MPC_REG_FIELD_LIST_DCN201!(u8);
}

#[repr(C)]
pub struct dcn201_mpc_mask {
    MPC_REG_FIELD_LIST_DCN201!(u32);
}

#[repr(C)]
pub struct dcn201_mpc {
    pub base: mpc,
    pub mpcc_in_use_mask: ::core::ffi::c_int,
    pub num_mpcc: ::core::ffi::c_int,
    pub mpc_regs: *const dcn201_mpc_registers,
    pub mpc_shift: *const dcn201_mpc_shift,
    pub mpc_mask: *const dcn201_mpc_mask,
}

extern "C" {
    pub fn dcn201_mpc_construct(
        mpc201: *mut dcn201_mpc,
        ctx: *mut dc_context,
        mpc_regs: *const dcn201_mpc_registers,
        mpc_shift: *const dcn201_mpc_shift,
        mpc_mask: *const dcn201_mpc_mask,
        num_mpcc: ::core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
