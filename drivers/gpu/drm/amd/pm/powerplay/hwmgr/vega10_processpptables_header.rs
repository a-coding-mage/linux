/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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
 */

// Dependency supplied by the surrounding translation unit: hwmgr.h

#[repr(i32)]
pub enum Vega10_I2CLineID {
    Vega10_I2CLineID_DDC1 = 0x90,
    Vega10_I2CLineID_DDC2 = 0x91,
    Vega10_I2CLineID_DDC3 = 0x92,
    Vega10_I2CLineID_DDC4 = 0x93,
    Vega10_I2CLineID_DDC5 = 0x94,
    Vega10_I2CLineID_DDC6 = 0x95,
    Vega10_I2CLineID_SCLSDA = 0x96,
    Vega10_I2CLineID_DDCVGA = 0x97,
}

pub const Vega10_I2C_DDC1DATA: i32 = 0;
pub const Vega10_I2C_DDC1CLK: i32 = 1;
pub const Vega10_I2C_DDC2DATA: i32 = 2;
pub const Vega10_I2C_DDC2CLK: i32 = 3;
pub const Vega10_I2C_DDC3DATA: i32 = 4;
pub const Vega10_I2C_DDC3CLK: i32 = 5;
pub const Vega10_I2C_SDA: i32 = 40;
pub const Vega10_I2C_SCL: i32 = 41;
pub const Vega10_I2C_DDC4DATA: i32 = 65;
pub const Vega10_I2C_DDC4CLK: i32 = 66;
pub const Vega10_I2C_DDC5DATA: i32 = 0x48;
pub const Vega10_I2C_DDC5CLK: i32 = 0x49;
pub const Vega10_I2C_DDC6DATA: i32 = 0x4a;
pub const Vega10_I2C_DDC6CLK: i32 = 0x4b;
pub const Vega10_I2C_DDCVGADATA: i32 = 0x4c;
pub const Vega10_I2C_DDCVGACLK: i32 = 0x4d;

extern "C" {
    pub static vega10_pptable_funcs: pp_table_func;

    pub fn vega10_get_number_of_powerplay_table_entries(
        hwmgr: *mut pp_hwmgr,
    ) -> ::core::ffi::c_int;

    pub fn vega10_get_powerplay_table_entry(
        hwmgr: *mut pp_hwmgr,
        entry_index: u32,
        power_state: *mut pp_power_state,
        call_back_func: Option<
            unsafe extern "C" fn(
                *mut pp_hwmgr,
                *mut ::core::ffi::c_void,
                *mut pp_power_state,
                *mut ::core::ffi::c_void,
                u32,
            ) -> ::core::ffi::c_int,
        >,
    ) -> ::core::ffi::c_int;

    pub fn vega10_baco_set_cap(hwmgr: *mut pp_hwmgr) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
