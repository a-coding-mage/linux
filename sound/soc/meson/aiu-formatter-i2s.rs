// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2026 BayLibre, SAS.
// Author: Valerio Setti <vsetti@baylibre.com>

// Dependencies translated from:
// #include <sound/pcm_params.h>
// #include <sound/soc.h>
// #include <sound/soc-dai.h>
// #include "aiu.h"
// #include "gx-formatter.h"

const AIU_I2S_SOURCE_DESC_MODE_8CH: u32 = BIT(0);
const AIU_I2S_SOURCE_DESC_MODE_24BIT: u32 = BIT(5);
const AIU_I2S_SOURCE_DESC_MODE_32BIT: u32 = BIT(9);

const AIU_I2S_DAC_CFG_MSB_FIRST: u32 = BIT(2);

unsafe fn aiu_formatter_i2s_get_be(
    w: *mut snd_soc_dapm_widget,
) -> *mut snd_soc_dai {
    let mut p: *mut snd_soc_dapm_path;
    let mut be: *mut snd_soc_dai;

    snd_soc_dapm_widget_for_each_sink_path!(w, p, {
        if !(*p).connect {
            continue;
        }

        if (*(*p).sink).id == snd_soc_dapm_dai_in {
            return (*(*p).sink).priv as *mut snd_soc_dai;
        }

        be = aiu_formatter_i2s_get_be((*p).sink);
        if !be.is_null() {
            return be;
        }
    });

    core::ptr::null_mut()
}

unsafe fn aiu_formatter_i2s_get_stream(
    w: *mut snd_soc_dapm_widget,
) -> *mut gx_stream {
    let be: *mut snd_soc_dai = aiu_formatter_i2s_get_be(w);

    if be.is_null() {
        return core::ptr::null_mut();
    }

    snd_soc_dai_dma_data_get_playback(be)
}

unsafe fn aiu_formatter_i2s_prepare(
    map: *mut regmap,
    quirks: *const gx_formatter_hw,
    ts: *mut gx_stream,
) -> core::ffi::c_int {
    /* Always operate in split (classic interleaved) mode */
    let mut desc: u32 = 0;

    /*
     * Pipeline reset is already implemented in aiu_fifo_i2s_trigger() at
     * trigger time.
     */

    match (*ts).physical_width {
        16 => {
            /* Nothing to do */
        }

        32 => {
            desc |= AIU_I2S_SOURCE_DESC_MODE_24BIT |
                AIU_I2S_SOURCE_DESC_MODE_32BIT;
        }

        _ => {
            return -EINVAL;
        }
    }

    match (*ts).channels {
        2 => {
            /* Nothing to do */
        }
        8 => {
            desc |= AIU_I2S_SOURCE_DESC_MODE_8CH;
        }
        _ => {
            return -EINVAL;
        }
    }

    regmap_update_bits(
        map,
        AIU_I2S_SOURCE_DESC,
        AIU_I2S_SOURCE_DESC_MODE_8CH |
            AIU_I2S_SOURCE_DESC_MODE_24BIT |
            AIU_I2S_SOURCE_DESC_MODE_32BIT,
        desc,
    );

    /* Send data MSB first */
    regmap_update_bits(
        map,
        AIU_I2S_DAC_CFG,
        AIU_I2S_DAC_CFG_MSB_FIRST,
        AIU_I2S_DAC_CFG_MSB_FIRST,
    );

    0
}

pub static aiu_formatter_i2s_ops: gx_formatter_ops = gx_formatter_ops {
    get_stream: Some(aiu_formatter_i2s_get_stream),
    prepare: Some(aiu_formatter_i2s_prepare),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
