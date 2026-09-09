/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2019 Intel Corporation
 */

// Dependencies supplied by <uapi/sound/sof/header.h> and <sound/sof/header.h>

/**
 * \brief Channel map, specifies transformation of one-to-many or many-to-one.
 *
 * In case of one-to-many specifies how the output channels are computed out of
 * a single source channel,
 * in case of many-to-one specifies how a single target channel is computed
 * from a multichannel input stream.
 *
 * Channel index specifies position of the channel in the stream on the 'one'
 * side.
 *
 * Ext ID is the identifier of external part of the transformation. Depending
 * on the context, it may be pipeline ID, dai ID, ...
 *
 * Channel mask describes which channels are taken into account on the "many"
 * side. Bit[i] set to 1 means that i-th channel is used for computation
 * (either as source or as a target).
 *
 * Channel mask is followed by array of coefficients in Q2.30 format,
 * one per each channel set in the mask (left to right, LS bit set in the
 * mask corresponds to ch_coeffs[0]).
 */
#[repr(C, packed)]
pub struct sof_ipc_channel_map {
    pub ch_index: u32,
    pub ext_id: u32,
    pub ch_mask: u32,
    pub reserved: u32,
    pub ch_coeffs: [i32; 0],
}

/**
 * \brief Complete map for each channel of a multichannel stream.
 *
 * num_ch_map Specifies number of items in the ch_map.
 * More than one transformation per a single channel is allowed (in case
 * multiple external entities are transformed).
 * A channel may be skipped in the transformation list, then it is filled
 * with 0's by the transformation function.
 */
#[repr(C, packed)]
pub struct sof_ipc_stream_map {
    pub hdr: sof_ipc_cmd_hdr,
    pub num_ch_map: u32,
    pub reserved: [u32; 3],
    pub ch_map: [sof_ipc_channel_map; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
