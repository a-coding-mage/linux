/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Dependency supplied by the translated core types.

#[repr(C)]
pub struct dce110_resource_pool {
    pub base: resource_pool,
}

// Equivalent of:
// container_of(pool, struct dce110_resource_pool, base)
#[macro_export]
macro_rules! TO_DCE110_RES_POOL {
    ($pool:expr) => {
        container_of!($pool, dce110_resource_pool, base)
    };
}

extern "C" {
    pub fn dce110_resource_build_pipe_hw_param(pipe_ctx: *mut pipe_ctx);

    pub fn dce110_create_resource_pool(
        num_virtual_links: u8,
        dc: *mut dc,
        asic_id: hw_asic_id,
    ) -> *mut resource_pool;

    pub fn dce110_find_first_free_match_stream_enc_for_link(
        res_ctx: *mut resource_context,
        pool: *const resource_pool,
        stream: *mut dc_stream_state,
    ) -> *mut stream_encoder;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
