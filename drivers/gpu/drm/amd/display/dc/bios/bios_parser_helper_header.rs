/*
 * Copyright 2012-16 Advanced Micro Devices, Inc.
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

// C header guard: __DAL_BIOS_PARSER_HELPER_H__

// External types supplied by the surrounding translation unit.
pub struct bios_parser;
pub struct dc_bios;

extern "C" {
    pub fn bios_get_image(
        bp: *mut dc_bios,
        offset: u32,
        size: u32,
    ) -> *mut u8;

    pub fn bios_is_accelerated_mode(bios: *mut dc_bios) -> bool;
    pub fn bios_set_scratch_acc_mode_change(bios: *mut dc_bios, state: u32);
    pub fn bios_set_scratch_critical_state(bios: *mut dc_bios, state: bool);
}

#[macro_export]
macro_rules! GET_IMAGE {
    ($type:ty, $offset:expr) => {{
        bios_get_image(
            &mut (*bp).base,
            $offset,
            core::mem::size_of::<$type>() as u32,
        ) as *mut $type
    }};
}

/* Upper bound on the number of records in a VBIOS record chain. Prevents
 * unbounded looping if the VBIOS image is malformed and lacks a terminator.
 */
pub const BIOS_MAX_NUM_RECORD: u32 = 256;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
