/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * Copyright(c) 2021 Mediatek Corporation. All rights reserved.
 *
 * Author: Bo Pan <bo.pan@mediatek.com>
 */

// Dependency corresponding to: #include <sound/sof/header.h>

#[repr(C, packed)]
pub struct sof_ipc_dai_mtk_afe_params {
    pub hdr: sof_ipc_hdr,
    pub channels: u32,
    pub rate: u32,
    pub format: u32,
    pub stream_id: u32,
    pub reserved: [u32; 4], /* reserve for future */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
