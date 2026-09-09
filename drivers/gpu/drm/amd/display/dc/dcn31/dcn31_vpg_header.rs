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
 */

// Dependency supplied by the original vpg.h header.

macro_rules! DCN31_VPG_FROM_VPG {
    ($vpg:expr) => { container_of!($vpg, dcn31_vpg, base) };
}

macro_rules! VPG_DCN31_REG_LIST {
    ($id:expr) => {
        SRI!(VPG_GENERIC_STATUS, VPG, $id),
        SRI!(VPG_GENERIC_PACKET_ACCESS_CTRL, VPG, $id),
        SRI!(VPG_GENERIC_PACKET_DATA, VPG, $id),
        SRI!(VPG_GSP_FRAME_UPDATE_CTRL, VPG, $id),
        SRI!(VPG_GSP_IMMEDIATE_UPDATE_CTRL, VPG, $id),
        SRI!(VPG_MEM_PWR, VPG, $id)
    };
}

#[repr(C)]
pub struct dcn31_vpg_registers {
    pub VPG_GENERIC_STATUS: u32,
    pub VPG_GENERIC_PACKET_ACCESS_CTRL: u32,
    pub VPG_GENERIC_PACKET_DATA: u32,
    pub VPG_GSP_FRAME_UPDATE_CTRL: u32,
    pub VPG_GSP_IMMEDIATE_UPDATE_CTRL: u32,
    pub VPG_MEM_PWR: u32,
}

macro_rules! VPG_DCN31_REG_FIELD_LIST {
    ($type:ty) => {
        VPG_GENERIC_CONFLICT_OCCURED: $type,
        VPG_GENERIC_CONFLICT_CLR: $type,
        VPG_GENERIC_DATA_INDEX: $type,
        VPG_GENERIC_DATA_BYTE0: $type,
        VPG_GENERIC_DATA_BYTE1: $type,
        VPG_GENERIC_DATA_BYTE2: $type,
        VPG_GENERIC_DATA_BYTE3: $type,
        VPG_GENERIC0_FRAME_UPDATE: $type,
        VPG_GENERIC1_FRAME_UPDATE: $type,
        VPG_GENERIC2_FRAME_UPDATE: $type,
        VPG_GENERIC3_FRAME_UPDATE: $type,
        VPG_GENERIC4_FRAME_UPDATE: $type,
        VPG_GENERIC5_FRAME_UPDATE: $type,
        VPG_GENERIC6_FRAME_UPDATE: $type,
        VPG_GENERIC7_FRAME_UPDATE: $type,
        VPG_GENERIC8_FRAME_UPDATE: $type,
        VPG_GENERIC9_FRAME_UPDATE: $type,
        VPG_GENERIC10_FRAME_UPDATE: $type,
        VPG_GENERIC11_FRAME_UPDATE: $type,
        VPG_GENERIC12_FRAME_UPDATE: $type,
        VPG_GENERIC13_FRAME_UPDATE: $type,
        VPG_GENERIC14_FRAME_UPDATE: $type,
        VPG_GENERIC0_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC1_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC2_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC3_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC4_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC5_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC6_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC7_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC8_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC9_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC10_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC11_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC12_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC13_IMMEDIATE_UPDATE: $type,
        VPG_GENERIC14_IMMEDIATE_UPDATE: $type,
        VPG_GSP_MEM_LIGHT_SLEEP_DIS: $type,
        VPG_GSP_LIGHT_SLEEP_FORCE: $type,
        VPG_GSP_MEM_PWR_STATE: $type
    };
}

#[repr(C)]
pub struct dcn31_vpg_shift {
    VPG_DCN31_REG_FIELD_LIST!(u8);
}

#[repr(C)]
pub struct dcn31_vpg_mask {
    VPG_DCN31_REG_FIELD_LIST!(u32);
}

#[repr(C)]
pub struct dcn31_vpg {
    pub base: vpg,
    pub regs: *const dcn31_vpg_registers,
    pub vpg_shift: *const dcn31_vpg_shift,
    pub vpg_mask: *const dcn31_vpg_mask,
}

extern "C" {
    pub fn vpg31_poweron(vpg: *mut vpg);
    pub fn vpg31_powerdown(vpg: *mut vpg);
    pub fn vpg31_construct(
        vpg31: *mut dcn31_vpg,
        ctx: *mut dc_context,
        inst: u32,
        vpg_regs: *const dcn31_vpg_registers,
        vpg_shift: *const dcn31_vpg_shift,
        vpg_mask: *const dcn31_vpg_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
