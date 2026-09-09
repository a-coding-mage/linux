/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2024 Advanced Micro Devices, Inc.
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

// Dependency equivalent of: #include "hw_sequencer_private.h"

extern "C" {
    pub fn dcn351_calc_blocks_to_gate(
        dc: *mut dc,
        context: *mut dc_state,
        update_state: *mut pg_block_update,
    );
    pub fn dcn351_calc_blocks_to_ungate(
        dc: *mut dc,
        context: *mut dc_state,
        update_state: *mut pg_block_update,
    );
    pub fn dcn351_hw_block_power_up(
        dc: *mut dc,
        update_state: *mut pg_block_update,
    );
    pub fn dcn351_hw_block_power_down(
        dc: *mut dc,
        update_state: *mut pg_block_update,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
