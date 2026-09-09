/* SPDX-License-Identifier: GPL-2.0
 *
 * linux/sound/sdw.h -- SoundWire helpers for ALSA/ASoC
 *
 * Copyright (c) 2022 Cirrus Logic Inc.
 *
 * Author: Charles Keepax <ckeepax@opensource.cirrus.com>
 */

// Dependencies supplied by the corresponding Linux SoundWire and ALSA headers.

/**
 * snd_sdw_params_to_config() - Conversion from hw_params to SoundWire config
 *
 * @substream: Pointer to the PCM substream structure
 * @params: Pointer to the hardware params structure
 * @stream_config: Stream configuration for the SoundWire audio stream
 * @port_config: Port configuration for the SoundWire audio stream
 *
 * This function provides a basic conversion from the hw_params structure to
 * SoundWire configuration structures. The user will at a minimum need to also
 * set the port number in the port config, but may also override more of the
 * setup, or in the case of a complex user, not use this helper at all and
 * open-code everything.
 */
#[inline]
pub unsafe fn snd_sdw_params_to_config(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    stream_config: *mut sdw_stream_config,
    port_config: *mut sdw_port_config,
) {
    (*stream_config).frame_rate = params_rate(params);
    (*stream_config).ch_count = params_channels(params);
    (*stream_config).bps = snd_pcm_format_width(params_format(params));

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*stream_config).direction = SDW_DATA_DIR_RX;
    } else {
        (*stream_config).direction = SDW_DATA_DIR_TX;
    }

    (*port_config).ch_mask = genmask((*stream_config).ch_count.wrapping_sub(1), 0);
}

#[inline]
const fn genmask(high: u32, low: u32) -> u32 {
    if high >= 31 {
        u32::MAX & (u32::MAX << low)
    } else {
        ((1u32 << high.wrapping_add(1)) - 1) & (u32::MAX << low)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
