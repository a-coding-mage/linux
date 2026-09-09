/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

/* This module implements functionality for dynamically assigning DIG link
 * encoder resources to display endpoints (links).
 */

/* Dependency: core_types.h */

/*
 * Initialise link encoder resource tracking.
 */
extern "C" {
    pub fn link_enc_cfg_init(dc: *const dc, state: *mut dc_state);

    /*
     * Copies a link encoder assignment from another state.
     */
    pub fn link_enc_cfg_copy(src_ctx: *const dc_state, dst_ctx: *mut dc_state);

    /*
     * Algorithm for assigning available DIG link encoders to streams.
     *
     * Update link_enc_assignments table and link_enc_avail list accordingly in
     * struct resource_context.
     *
     * Loop over all streams twice:
     * a) First assign encoders to unmappable endpoints.
     * b) Then assign encoders to mappable endpoints.
     */
    pub fn link_enc_cfg_link_encs_assign(
        dc: *mut dc,
        state: *mut dc_state,
        streams: *mut *mut dc_stream_state,
        stream_count: u8,
    );

    /*
     * Unassign a link encoder from a stream.
     *
     * Update link_enc_assignments table and link_enc_avail list accordingly in
     * struct resource_context.
     */
    pub fn link_enc_cfg_link_enc_unassign(
        state: *mut dc_state,
        stream: *mut dc_stream_state,
    );

    /*
     * Check whether the transmitter driven by a link encoder is a mappable
     * endpoint.
     */
    pub fn link_enc_cfg_is_transmitter_mappable(
        dc: *mut dc,
        link_enc: *mut link_encoder,
    ) -> bool;

    /* Return stream using DIG link encoder resource. NULL if unused. */
    pub fn link_enc_cfg_get_stream_using_link_enc(
        dc: *mut dc,
        eng_id: engine_id,
    ) -> *mut dc_stream_state;

    /* Return link using DIG link encoder resource. NULL if unused. */
    pub fn link_enc_cfg_get_link_using_link_enc(
        dc: *mut dc,
        eng_id: engine_id,
    ) -> *mut dc_link;

    /* Return DIG link encoder used by link. NULL if unused. */
    pub fn link_enc_cfg_get_link_enc_used_by_link(
        dc: *mut dc,
        link: *const dc_link,
    ) -> *mut link_encoder;

    /* Return next available DIG link encoder. NULL if none available. */
    pub fn link_enc_cfg_get_next_avail_link_enc(dc: *mut dc) -> *mut link_encoder;

    /* Return DIG link encoder. NULL if unused. */
    pub fn link_enc_cfg_get_link_enc(link: *const dc_link) -> *mut link_encoder;

    /* Return DIG link encoder used by stream in current/previous state. NULL if unused. */
    pub fn link_enc_cfg_get_link_enc_used_by_stream_current(
        dc: *mut dc,
        stream: *const dc_stream_state,
    ) -> *mut link_encoder;

    /* Return true if encoder available to use. */
    pub fn link_enc_cfg_is_link_enc_avail(
        dc: *mut dc,
        eng_id: engine_id,
        link: *mut dc_link,
    ) -> bool;

    /* Returns true if encoder assignments in supplied state pass validity checks. */
    pub fn link_enc_cfg_validate(dc: *mut dc, state: *mut dc_state) -> bool;

    /* Set the link encoder assignment mode for the current_state to LINK_ENC_CFG_TRANSIENT mode.
     * This indicates that a new_state is in the process of being applied to hardware.
     * During this transition, old and new encoder assignments should be accessible from the old_state.
     * Only allow transition into transient mode if new encoder assignments are valid.
     */
    pub fn link_enc_cfg_set_transient_mode(
        dc: *mut dc,
        current_state: *mut dc_state,
        new_state: *mut dc_state,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
