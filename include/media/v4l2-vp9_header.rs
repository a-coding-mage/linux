/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Helper functions for vp9 codecs.
 *
 * Copyright (c) 2021 Collabora, Ltd.
 *
 * Author: Andrzej Pietrasiewicz <andrzej.p@collabora.com>
 */

// Dependency supplied by the surrounding media controls bindings.

/**
 * struct v4l2_vp9_frame_mv_context - motion vector-related probabilities
 *
 * @joint: motion vector joint probabilities.
 * @sign: motion vector sign probabilities.
 * @classes: motion vector class probabilities.
 * @class0_bit: motion vector class0 bit probabilities.
 * @bits: motion vector bits probabilities.
 * @class0_fr: motion vector class0 fractional bit probabilities.
 * @fr: motion vector fractional bit probabilities.
 * @class0_hp: motion vector class0 high precision fractional bit probabilities.
 * @hp: motion vector high precision fractional bit probabilities.
 *
 * A member of v4l2_vp9_frame_context.
 */
#[repr(C)]
pub struct v4l2_vp9_frame_mv_context {
    pub joint: [u8; 3],
    pub sign: [u8; 2],
    pub classes: [[u8; 10]; 2],
    pub class0_bit: [u8; 2],
    pub bits: [[u8; 10]; 2],
    pub class0_fr: [[[u8; 3]; 2]; 2],
    pub fr: [[u8; 3]; 2],
    pub class0_hp: [u8; 2],
    pub hp: [u8; 2],
}

/** Frame probabilities, including motion-vector related probabilities. */
#[repr(C)]
pub struct v4l2_vp9_frame_context {
    pub tx8: [[u8; 1]; 2],
    pub tx16: [[u8; 2]; 2],
    pub tx32: [[u8; 3]; 2],
    pub coef: [[[[[u8; 3]; 6]; 6]; 2]; 4],
    pub skip: [u8; 3],
    pub inter_mode: [[u8; 3]; 7],
    pub interp_filter: [[u8; 2]; 4],
    pub is_inter: [u8; 4],
    pub comp_mode: [u8; 5],
    pub single_ref: [[u8; 2]; 5],
    pub comp_ref: [u8; 5],
    pub y_mode: [[u8; 9]; 4],
    pub uv_mode: [[u8; 9]; 10],
    pub partition: [[u8; 3]; 16],
    pub mv: v4l2_vp9_frame_mv_context,
}

/** Pointers to arrays of VP9 symbol counts. */
#[repr(C)]
pub struct v4l2_vp9_frame_symbol_counts {
    pub partition: *mut [[u32; 4]; 16],
    pub skip: *mut [[u32; 2]; 3],
    pub intra_inter: *mut [[u32; 2]; 4],
    pub tx32p: *mut [[u32; 4]; 2],
    pub tx16p: *mut [[u32; 4]; 2],
    pub tx8p: *mut [[u32; 2]; 2],
    pub y_mode: *mut [[u32; 10]; 4],
    pub uv_mode: *mut [[u32; 10]; 10],
    pub comp: *mut [[u32; 2]; 5],
    pub comp_ref: *mut [[u32; 2]; 5],
    pub single_ref: *mut [[[u32; 2]; 2]; 5],
    pub mv_mode: *mut [[u32; 4]; 7],
    pub filter: *mut [[u32; 3]; 4],
    pub mv_joint: *mut [u32; 4],
    pub sign: *mut [u32; 2],
    pub classes: *mut [[u32; 11]; 2],
    pub class0: *mut [[u32; 2]; 2],
    pub bits: *mut [[[u32; 2]; 10]; 2],
    pub class0_fp: *mut [[[u32; 4]; 2]; 2],
    pub fp: *mut [[u32; 4]; 2],
    pub class0_hp: *mut [[u32; 2]; 2],
    pub hp: *mut [[u32; 2]; 2],
    pub coeff: [[[[[*mut [u32; 3]; 6]; 6]; 2]; 2]; 4],
    pub eob: [[[[[*mut u32; 2]; 6]; 6]; 2]; 4],
}

extern "C" {
    pub static v4l2_vp9_kf_y_mode_prob: [[[u8; 9]; 10]; 10]; // Section 10.4 of the spec
    pub static v4l2_vp9_kf_partition_probs: [[u8; 3]; 16]; // Section 10.4 of the spec
    pub static v4l2_vp9_kf_uv_mode_prob: [[u8; 9]; 10]; // Section 10.4 of the spec
    pub static v4l2_vp9_default_probs: v4l2_vp9_frame_context; // Section 10.5 of the spec

    pub fn v4l2_vp9_fw_update_probs(
        probs: *mut v4l2_vp9_frame_context,
        deltas: *const v4l2_ctrl_vp9_compressed_hdr,
        dec_params: *const v4l2_ctrl_vp9_frame,
    );

    pub fn v4l2_vp9_reset_frame_ctx(
        dec_params: *const v4l2_ctrl_vp9_frame,
        frame_context: *mut v4l2_vp9_frame_context,
    ) -> u8;

    pub fn v4l2_vp9_adapt_coef_probs(
        probs: *mut v4l2_vp9_frame_context,
        counts: *mut v4l2_vp9_frame_symbol_counts,
        use_128: bool,
        frame_is_intra: bool,
    );

    pub fn v4l2_vp9_adapt_noncoef_probs(
        probs: *mut v4l2_vp9_frame_context,
        counts: *mut v4l2_vp9_frame_symbol_counts,
        reference_mode: u8,
        interpolation_filter: u8,
        tx_mode: u8,
        flags: u32,
    );

    pub fn v4l2_vp9_seg_feat_enabled(
        feature_enabled: *const u8,
        feature: libc::c_uint,
        segid: libc::c_uint,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
