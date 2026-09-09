/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  MTK SPI bus driver definitions
 *
 * Copyright (c) 2015 MediaTek Inc.
 * Author: Leilk Liu <leilk.liu@mediatek.com>
 */

/* Board specific platform_data */
#[repr(C)]
pub struct mtk_chip_config {
    pub sample_sel: u32,
    pub tick_delay: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
