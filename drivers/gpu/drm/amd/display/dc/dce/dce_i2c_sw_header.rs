/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

pub const DCE_I2C_DEFAULT_I2C_SW_SPEED: i32 = 50;
pub const I2C_SW_RETRIES: i32 = 10;
pub const I2C_SW_TIMEOUT_DELAY: i32 = 3000;

#[repr(C)]
pub struct dce_i2c_sw {
    pub ddc: *mut ddc,
    pub ctx: *mut dc_context,
    pub clock_delay: u32,
    pub speed: u32,
}

extern "C" {
    pub fn dce_i2c_sw_construct(
        dce_i2c_sw: *mut dce_i2c_sw,
        ctx: *mut dc_context,
    );

    pub fn dce_i2c_submit_command_sw(
        pool: *mut resource_pool,
        ddc: *mut ddc,
        cmd: *mut i2c_command,
        dce_i2c_sw: *mut dce_i2c_sw,
    ) -> bool;

    pub fn dce_i2c_engine_acquire_sw(
        dce_i2c_sw: *mut dce_i2c_sw,
        ddc_handle: *mut ddc,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
