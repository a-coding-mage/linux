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
 *
 * Authors: AMD
 */

// Dependency supplied by the surrounding translation unit: dc_types.h.

pub const NUM_PTS_IN_REGION: i32 = 16;
pub const NUM_REGIONS: i32 = 32;
pub const MAX_HW_POINTS: i32 = NUM_PTS_IN_REGION * NUM_REGIONS;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum table_type {
    type_pq_table,
    type_de_pq_table,
}

// Opaque type supplied by dc_types.h.
#[repr(C)]
pub struct fixed31_32 {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mod_color_is_table_init(type_: table_type) -> bool;

    pub fn mod_color_get_table(type_: table_type) -> *mut fixed31_32;

    pub fn mod_color_set_table_init_state(type_: table_type, state: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
