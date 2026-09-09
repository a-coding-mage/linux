/*
 * Copyright 2012-17 Advanced Micro Devices, Inc.
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

// C dependencies: ../dcn10/dcn10_hubp.h and ../dcn20/dcn20_hubp.h

#[macro_export]
macro_rules! TO_DCN201_HUBP {
    ($hubp:expr) => { container_of!($hubp, dcn201_hubp, base) };
}

#[macro_export]
macro_rules! HUBP_REG_LIST_DCN201 {
    ($id:expr) => {
        HUBP_REG_LIST_DCN!($id),
        SRI!(PREFETCH_SETTINGS, HUBPREQ, $id),
        SRI!(PREFETCH_SETTINGS_C, HUBPREQ, $id),
        SRI!(DCSURF_FLIP_CONTROL2, HUBPREQ, $id),
        SRI!(CURSOR_SETTINGS, HUBPREQ, $id),
        SRI!(CURSOR_SURFACE_ADDRESS_HIGH, CURSOR0_, $id),
        SRI!(CURSOR_SURFACE_ADDRESS, CURSOR0_, $id),
        SRI!(CURSOR_SIZE, CURSOR0_, $id),
        SRI!(CURSOR_CONTROL, CURSOR0_, $id),
        SRI!(CURSOR_POSITION, CURSOR0_, $id),
        SRI!(CURSOR_HOT_SPOT, CURSOR0_, $id),
        SRI!(CURSOR_DST_OFFSET, CURSOR0_, $id),
        SRI!(DMDATA_ADDRESS_HIGH, CURSOR0_, $id),
        SRI!(DMDATA_ADDRESS_LOW, CURSOR0_, $id),
        SRI!(DMDATA_CNTL, CURSOR0_, $id),
        SRI!(DMDATA_SW_CNTL, CURSOR0_, $id),
        SRI!(DMDATA_QOS_CNTL, CURSOR0_, $id),
        SRI!(DMDATA_SW_DATA, CURSOR0_, $id),
        SRI!(DMDATA_STATUS, CURSOR0_, $id),
        SRI!(FLIP_PARAMETERS_0, HUBPREQ, $id),
        SRI!(FLIP_PARAMETERS_2, HUBPREQ, $id)
    };
}

#[macro_export]
macro_rules! HUBP_MASK_SH_LIST_DCN201 {
    ($mask_sh:expr) => {
        HUBP_MASK_SH_LIST_DCN!($mask_sh),
        HUBP_SF!(HUBPREQ0_PREFETCH_SETTINGS, DST_Y_PREFETCH, $mask_sh),
        HUBP_SF!(HUBPREQ0_PREFETCH_SETTINGS, VRATIO_PREFETCH, $mask_sh),
        HUBP_SF!(HUBPREQ0_PREFETCH_SETTINGS_C, VRATIO_PREFETCH_C, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCSURF_FLIP_CONTROL2, SURFACE_TRIPLE_BUFFER_ENABLE, $mask_sh),
        HUBP_SF!(HUBPREQ0_CURSOR_SETTINGS, CURSOR0_DST_Y_OFFSET, $mask_sh),
        HUBP_SF!(HUBPREQ0_CURSOR_SETTINGS, CURSOR0_CHUNK_HDL_ADJUST, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_SURFACE_ADDRESS_HIGH, CURSOR_SURFACE_ADDRESS_HIGH, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_SURFACE_ADDRESS, CURSOR_SURFACE_ADDRESS, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_SIZE, CURSOR_WIDTH, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_SIZE, CURSOR_HEIGHT, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_CONTROL, CURSOR_MODE, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_CONTROL, CURSOR_2X_MAGNIFY, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_CONTROL, CURSOR_PITCH, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_CONTROL, CURSOR_LINES_PER_CHUNK, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_CONTROL, CURSOR_ENABLE, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_POSITION, CURSOR_X_POSITION, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_POSITION, CURSOR_Y_POSITION, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_HOT_SPOT, CURSOR_HOT_SPOT_X, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_HOT_SPOT, CURSOR_HOT_SPOT_Y, $mask_sh),
        HUBP_SF!(CURSOR0_0_CURSOR_DST_OFFSET, CURSOR_DST_X_OFFSET, $mask_sh),
        HUBP_SF!(CURSOR0_0_DMDATA_ADDRESS_HIGH, DMDATA_ADDRESS_HIGH, $mask_sh),
        HUBP_SF!(CURSOR0_0_DMDATA_CNTL, DMDATA_MODE, $mask_sh),
        HUBP_SF!(CURSOR0_0_DMDATA_CNTL, DMDATA_UPDATED, $mask_sh),
        HUBP_SF!(CURSOR0_0_DMDATA_CNTL, DMDATA_REPEAT, $mask_sh),
        HUBP_SF!(CURSOR0_0_DMDATA_CNTL, DMDATA_SIZE, $mask_sh),
        HUBP_SF!(CURSOR0_0_DMDATA_SW_CNTL, DMDATA_SW_UPDATED, $mask_sh),
        HUBP_SF!(CURSOR0_0_DMDATA_SW_CNTL, DMDATA_SW_REPEAT, $mask_sh),
        HUBP_SF!(CURSOR0_0_DMDATA_SW_CNTL, DMDATA_SW_SIZE, $mask_sh),
        HUBP_SF!(CURSOR0_0_DMDATA_QOS_CNTL, DMDATA_QOS_MODE, $mask_sh),
        HUBP_SF!(CURSOR0_0_DMDATA_QOS_CNTL, DMDATA_QOS_LEVEL, $mask_sh),
        HUBP_SF!(CURSOR0_0_DMDATA_QOS_CNTL, DMDATA_DL_DELTA, $mask_sh),
        HUBP_SF!(HUBPREQ0_FLIP_PARAMETERS_0, DST_Y_PER_VM_FLIP, $mask_sh),
        HUBP_SF!(HUBPREQ0_FLIP_PARAMETERS_0, DST_Y_PER_ROW_FLIP, $mask_sh),
        HUBP_SF!(HUBPREQ0_FLIP_PARAMETERS_2, REFCYC_PER_META_CHUNK_FLIP_L, $mask_sh),
        HUBP_SF!(HUBP0_DCHUBP_CNTL, HUBP_VREADY_AT_OR_AFTER_VSYNC, $mask_sh),
        HUBP_SF!(HUBP0_DCHUBP_CNTL, HUBP_DISABLE_STOP_DATA_DURING_VM, $mask_sh),
        HUBP_SF!(HUBPREQ0_DCSURF_FLIP_CONTROL, HUBPREQ_MASTER_UPDATE_LOCK_STATUS, $mask_sh)
    };
}

#[macro_export]
macro_rules! DCN201_HUBP_REG_VARIABLE_LIST { () => { DCN2_HUBP_REG_COMMON_VARIABLE_LIST!() }; }
#[macro_export]
macro_rules! DCN201_HUBP_REG_FIELD_VARIABLE_LIST { ($type:ty) => { DCN2_HUBP_REG_FIELD_VARIABLE_LIST!($type) }; }

#[repr(C)]
pub struct dcn201_hubp_registers {
    DCN201_HUBP_REG_VARIABLE_LIST!();
}

#[repr(C)]
pub struct dcn201_hubp_shift {
    DCN201_HUBP_REG_FIELD_VARIABLE_LIST!(u8);
}

#[repr(C)]
pub struct dcn201_hubp_mask {
    DCN201_HUBP_REG_FIELD_VARIABLE_LIST!(u32);
}

#[repr(C)]
pub struct dcn201_hubp {
    pub base: hubp,
    pub state: dcn_hubp_state,
    pub hubp_regs: *const dcn201_hubp_registers,
    pub hubp_shift: *const dcn201_hubp_shift,
    pub hubp_mask: *const dcn201_hubp_mask,
}

extern "C" {
    pub fn dcn201_hubp_construct(
        hubp201: *mut dcn201_hubp,
        ctx: *mut dc_context,
        inst: u32,
        hubp_regs: *const dcn201_hubp_registers,
        hubp_shift: *const dcn201_hubp_shift,
        hubp_mask: *const dcn201_hubp_mask,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
