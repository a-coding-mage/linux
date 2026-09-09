/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Helper functions for H264 codecs.
 *
 * Copyright (c) 2019 Collabora, Ltd.
 *
 * Author: Boris Brezillon <boris.brezillon@collabora.com>
 */

// Dependency: <media/v4l2-ctrls.h>

/**
 * struct v4l2_h264_reflist_builder - Reference list builder object
 *
 * This object stores the context of the P/B0/B1 reference list builder.
 * This procedure is described in section '8.2.4 Decoding process for reference
 * picture lists construction' of the H264 spec.
 */
#[repr(C)]
pub struct v4l2_h264_reflist_builder_refs {
    pub top_field_order_cnt: i32,
    pub bottom_field_order_cnt: i32,
    pub frame_num: i32,
    // C bit-field: u16 longterm : 1;
    pub longterm: u16,
}

#[repr(C)]
pub struct v4l2_h264_reflist_builder {
    pub refs: [v4l2_h264_reflist_builder_refs; V4L2_H264_NUM_DPB_ENTRIES],
    pub cur_pic_order_count: i32,
    pub cur_pic_fields: u8,
    pub unordered_reflist: [v4l2_h264_reference; V4L2_H264_REF_LIST_LEN],
    pub num_valid: u8,
}

unsafe extern "C" {
    pub fn v4l2_h264_init_reflist_builder(
        b: *mut v4l2_h264_reflist_builder,
        dec_params: *const v4l2_ctrl_h264_decode_params,
        sps: *const v4l2_ctrl_h264_sps,
        dpb: *const v4l2_h264_dpb_entry,
    );

    /**
     * Build the B0/B1 reference lists.
     *
     * `b0_reflist` and `b1_reflist` are 32-sized arrays used to store the
     * corresponding reference lists.
     */
    pub fn v4l2_h264_build_b_ref_lists(
        builder: *const v4l2_h264_reflist_builder,
        b0_reflist: *mut v4l2_h264_reference,
        b1_reflist: *mut v4l2_h264_reference,
    );

    /**
     * Build the P reference list.
     *
     * `reflist` is a 32-sized array used to store the P reference list.
     */
    pub fn v4l2_h264_build_p_ref_list(
        builder: *const v4l2_h264_reflist_builder,
        reflist: *mut v4l2_h264_reference,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
