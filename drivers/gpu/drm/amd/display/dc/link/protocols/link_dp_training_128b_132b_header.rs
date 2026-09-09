/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

// Dependency supplied by the corresponding translated header/module:
// #include "link_dp_training.h"

unsafe extern "C" {
    pub fn dp_perform_128b_132b_link_training(
        link: *mut dc_link,
        link_res: *const link_resource,
        lt_settings: *mut link_training_settings,
    ) -> link_training_result;

    pub fn decide_128b_132b_training_settings(
        link: *mut dc_link,
        link_res: *const link_resource,
        link_settings: *const dc_link_settings,
        lt_settings: *mut link_training_settings,
    );

    pub fn dp_decide_128b_132b_lttpr_mode(link: *mut dc_link) -> lttpr_mode;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
