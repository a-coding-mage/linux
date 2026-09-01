/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ALSA ASoC interface for the Loongson platform
 *
 * Copyright (C) 2023 Loongson Technology Corporation Limited
 * Author: Yingkun Meng <mengyingkun@loongson.cn>
 */

unsafe extern "C" {
    pub static loongson_i2s_idma_component: snd_soc_component_driver;
    pub static loongson_i2s_edma_component: snd_soc_component_driver;
    pub static loongson_dmaengine_pcm_config: snd_dmaengine_pcm_config;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
