/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/types.h, atom-types.h, atombios.h, and pppcielanes.h

/** \file
 * Functions related to PCIe lane changes.
 */

/* For converting from number of lanes to lane bits.  */
static const PP_R600_ENCODE_LANES: [u8; 17] = [
    0,          /*  0 Not Supported  */
    1,          /*  1 Lane  */
    2,          /*  2 Lanes  */
    0,          /*  3 Not Supported  */
    3,          /*  4 Lanes  */
    0,          /*  5 Not Supported  */
    0,          /*  6 Not Supported  */
    0,          /*  7 Not Supported  */
    4,          /*  8 Lanes  */
    0,          /*  9 Not Supported  */
    0,          /* 10 Not Supported  */
    0,          /* 11 Not Supported  */
    5,          /* 12 Lanes (Not actually supported)  */
    0,          /* 13 Not Supported  */
    0,          /* 14 Not Supported  */
    0,          /* 15 Not Supported  */
    6           /* 16 Lanes  */
];

static const PP_R600_DECODED_LANES: [u8; 8] = [16, 1, 2, 4, 8, 12, 16, 0];

pub extern "C" fn encode_pcie_lane_width(num_lanes: u32) -> u8 {
    unsafe { *PP_R600_ENCODE_LANES.as_ptr().add(num_lanes as usize) }
}

pub extern "C" fn decode_pcie_lane_width(num_lanes: u32) -> u8 {
    unsafe { *PP_R600_DECODED_LANES.as_ptr().add(num_lanes as usize) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
