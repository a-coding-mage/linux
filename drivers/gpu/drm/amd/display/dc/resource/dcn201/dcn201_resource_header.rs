/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// C header guard: __DC_RESOURCE_DCN201_H__
// Dependency originally supplied by: core_types.h

pub const RRDPCS_PHY_DP_TX_PSTATE_POWER_UP: u32 = 0x0000_0000;
pub const RRDPCS_PHY_DP_TX_PSTATE_HOLD: u32 = 0x0000_0001;
pub const RRDPCS_PHY_DP_TX_PSTATE_HOLD_OFF: u32 = 0x0000_0002;
pub const RRDPCS_PHY_DP_TX_PSTATE_POWER_DOWN: u32 = 0x0000_0003;

pub struct dc;
pub struct resource_pool;
pub struct _vcs_dpi_display_pipe_params_st;
pub struct dc_init_data;

#[repr(C)]
pub struct dcn201_resource_pool {
    pub base: resource_pool,
}

/// C equivalent of `container_of(pool, struct dcn201_resource_pool, base)`.
#[inline]
pub unsafe fn TO_DCN201_RES_POOL(pool: *mut resource_pool) -> *mut dcn201_resource_pool {
    pool as *mut dcn201_resource_pool
}

extern "C" {
    pub fn dcn201_create_resource_pool(
        init_data: *const dc_init_data,
        dc: *mut dc,
    ) -> *mut resource_pool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
