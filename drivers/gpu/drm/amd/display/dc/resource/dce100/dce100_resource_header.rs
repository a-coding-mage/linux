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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 *
 */
/*
 * dce100_resource.h
 *
 *  Created on: 2016-01-20
 *      Author: qyang
 */

// Forward declarations supplied by dependent translation units.
pub struct dc;
pub struct resource_pool;
pub struct dc_plane_state;
pub struct dc_caps;
pub struct dc_state;
pub struct dc_stream_state;
pub struct resource_context;
pub struct stream_encoder;

// External enum types supplied by dependent translation units.
pub enum dc_status {}
pub enum dc_validate_mode {}

extern "C" {
    pub fn dce100_create_resource_pool(
        num_virtual_links: u8,
        dc: *mut dc,
    ) -> *mut resource_pool;

    pub fn dce100_validate_plane(
        plane_state: *const dc_plane_state,
        caps: *mut dc_caps,
    ) -> dc_status;

    pub fn dce100_validate_global(
        dc: *mut dc,
        context: *mut dc_state,
    ) -> dc_status;

    pub fn dce100_validate_bandwidth(
        dc: *mut dc,
        context: *mut dc_state,
        validate_mode: dc_validate_mode,
    ) -> dc_status;

    pub fn dce100_add_stream_to_ctx(
        dc: *mut dc,
        new_ctx: *mut dc_state,
        dc_stream: *mut dc_stream_state,
    ) -> dc_status;

    pub fn dce100_find_first_free_match_stream_enc_for_link(
        res_ctx: *mut resource_context,
        pool: *const resource_pool,
        stream: *mut dc_stream_state,
    ) -> *mut stream_encoder;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
