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
/* FILE POLICY AND INTENDED USAGE:
 * This file implements accessors to link resource.
 */

// Dependencies supplied by the surrounding translation unit:
// link_resource.h, protocols/link_dp_capability.h

pub unsafe fn link_get_cur_link_res(
    link: *const dc_link,
    link_res: *mut link_resource,
) {
    let mut i: i32;
    let mut pipe: *mut pipe_ctx = core::ptr::null_mut();

    core::ptr::write_bytes(link_res, 0, 1);

    i = 0;
    while i < MAX_PIPES {
        pipe = &mut (*(*(*link).dc).current_state).res_ctx.pipe_ctx[i as usize];
        if !(*pipe).stream.is_null()
            && !(*(*pipe).stream).link.is_null()
            && (*pipe).top_pipe.is_null()
        {
            if (*(*pipe).stream).link == link {
                *link_res = (*pipe).link_res;
                break;
            }
        }
        i += 1;
    }
}

pub unsafe fn link_get_cur_res_map(dc: *const dc, map: *mut u32) {
    let mut link: *mut dc_link;
    let mut i: u32;
    let mut hpo_dp_recycle_map: u32 = 0;

    *map = 0;

    if (*dc).caps.dp_hpo {
        i = 0;
        while i < (*dc).caps.max_links {
            link = (*dc).links[i as usize];
            if (*link).link_status.link_active
                && link_dp_get_encoding_format(&(*link).reported_link_cap)
                    == DP_128b_132b_ENCODING
                && link_dp_get_encoding_format(&(*link).cur_link_settings)
                    != DP_128b_132b_ENCODING
            {
                /* hpo dp link encoder is considered as recycled, when RX reports 128b/132b encoding capability
                 * but current link doesn't use it.
                 */
                hpo_dp_recycle_map |= 1u32.wrapping_shl(i);
            }
            i += 1;
        }
        *map |= hpo_dp_recycle_map << LINK_RES_HPO_DP_REC_MAP__SHIFT;
    }
}

pub unsafe fn link_restore_res_map(dc: *const dc, map: *mut u32) {
    let mut link: *mut dc_link;
    let mut i: u32;
    let mut available_hpo_dp_count: u32;
    let hpo_dp_recycle_map: u32 = (*map & LINK_RES_HPO_DP_REC_MAP__MASK)
        >> LINK_RES_HPO_DP_REC_MAP__SHIFT;

    if (*dc).caps.dp_hpo {
        available_hpo_dp_count = (*(*dc).res_pool).hpo_dp_link_enc_count;
        /* remove excess 128b/132b encoding support for not recycled links */
        i = 0;
        while i < (*dc).caps.max_links {
            if (hpo_dp_recycle_map & 1u32.wrapping_shl(i)) == 0 {
                link = (*dc).links[i as usize];
                if (*link).type_ != dc_connection_none
                    && link_dp_get_encoding_format(&(*link).verified_link_cap)
                        == DP_128b_132b_ENCODING
                {
                    if available_hpo_dp_count > 0 {
                        available_hpo_dp_count -= 1;
                    } else {
                        /* remove 128b/132b encoding capability by limiting verified link rate to HBR3 */
                        (*link).verified_link_cap.link_rate = LINK_RATE_HIGH3;
                    }
                }
            }
            i += 1;
        }
        /* remove excess 128b/132b encoding support for recycled links */
        i = 0;
        while i < (*dc).caps.max_links {
            if (hpo_dp_recycle_map & 1u32.wrapping_shl(i)) != 0 {
                link = (*dc).links[i as usize];
                if (*link).type_ != dc_connection_none
                    && link_dp_get_encoding_format(&(*link).verified_link_cap)
                        == DP_128b_132b_ENCODING
                {
                    if available_hpo_dp_count > 0 {
                        available_hpo_dp_count -= 1;
                    } else {
                        /* remove 128b/132b encoding capability by limiting verified link rate to HBR3 */
                        (*link).verified_link_cap.link_rate = LINK_RATE_HIGH3;
                    }
                }
            }
            i += 1;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
