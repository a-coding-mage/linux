/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependency equivalent of: #include "dc_stream.h"

#[repr(C)]
pub struct dc_stream_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_sink {
    _private: [u8; 0],
}

extern "C" {
    pub fn dc_stream_construct(
        stream: *mut dc_stream_state,
        dc_sink_data: *mut dc_sink,
    ) -> bool;

    pub fn dc_stream_destruct(stream: *mut dc_stream_state);

    pub fn dc_stream_assign_stream_id(stream: *mut dc_stream_state);

    /*
     * Finds the highest refresh rate that can be achieved
     * from starting_freq while staying within flicker criteria
     */
    pub fn dc_stream_calculate_max_flickerless_refresh_rate(
        stream: *mut dc_stream_state,
        starting_refresh_hz: i32,
        is_gaming: bool,
    ) -> i32;

    /*
     * Finds the lowest refresh rate that can be achieved
     * from starting_freq while staying within flicker criteria
     */
    pub fn dc_stream_calculate_min_flickerless_refresh_rate(
        stream: *mut dc_stream_state,
        starting_refresh_hz: i32,
        is_gaming: bool,
    ) -> i32;

    /*
     * Determines if there will be a flicker when moving between 2 refresh rates
     */
    pub fn dc_stream_is_refresh_rate_range_flickerless(
        stream: *mut dc_stream_state,
        hz1: i32,
        hz2: i32,
        is_gaming: bool,
    ) -> bool;

    /*
     * Determines the max instant vtotal delta increase that can be applied without
     * flickering for a given stream
     */
    pub fn dc_stream_get_max_flickerless_instant_vtotal_decrease(
        stream: *mut dc_stream_state,
        is_gaming: bool,
    ) -> u32;

    /*
     * Determines the max instant vtotal delta decrease that can be applied without
     * flickering for a given stream
     */
    pub fn dc_stream_get_max_flickerless_instant_vtotal_increase(
        stream: *mut dc_stream_state,
        is_gaming: bool,
    ) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
