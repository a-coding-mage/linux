/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * hdmi-codec.h - HDMI Codec driver API
 *
 * Copyright (C) 2014 Texas Instruments Incorporated - https://www.ti.com
 *
 * Author: Jyri Sarha <jsarha@ti.com>
 */

// Dependencies supplied by the surrounding translation unit:
// linux/of_graph.h, linux/hdmi.h, sound/asoundef.h, sound/soc.h,
// and uapi/sound/asound.h.

/*
 * Protocol between ASoC cpu-dai and HDMI-encoder
 */
#[repr(C)]
pub struct hdmi_codec_daifmt {
    pub fmt: hdmi_codec_daifmt_fmt,
    pub bit_clk_inv: u32,
    pub frame_clk_inv: u32,
    pub bit_clk_provider: u32,
    pub frame_clk_provider: u32,
    /* bit_fmt could be standard PCM format or
     * IEC958 encoded format. ALSA IEC958 plugin will pass
     * IEC958_SUBFRAME format to the underneath driver.
     */
    pub bit_fmt: snd_pcm_format_t,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum hdmi_codec_daifmt_fmt {
    HDMI_I2S,
    HDMI_RIGHT_J,
    HDMI_LEFT_J,
    HDMI_DSP_A,
    HDMI_DSP_B,
    HDMI_AC97,
    HDMI_SPDIF,
}

/*
 * HDMI audio parameters
 */
#[repr(C)]
pub struct hdmi_codec_params {
    pub cea: hdmi_audio_infoframe,
    pub iec: snd_aes_iec958,
    pub sample_rate: i32,
    pub sample_width: i32,
    pub channels: i32,
}

pub type hdmi_codec_plugged_cb = unsafe extern "C" fn(dev: *mut device, plugged: bool);

pub struct hdmi_codec_pdata;

#[repr(C)]
pub struct hdmi_codec_ops {
    /*
     * Called when ASoC starts an audio stream setup.
     * Optional
     */
    pub audio_startup: Option<unsafe extern "C" fn(dev: *mut device, data: *mut core::ffi::c_void) -> i32>,

    /*
     * Configures HDMI-encoder for audio stream.
     * Having either prepare or hw_params is mandatory.
     */
    pub hw_params: Option<unsafe extern "C" fn(
        dev: *mut device,
        data: *mut core::ffi::c_void,
        fmt: *mut hdmi_codec_daifmt,
        hparms: *mut hdmi_codec_params,
    ) -> i32>,

    /*
     * Configures HDMI-encoder for audio stream. Can be called
     * multiple times for each setup.
     *
     * Having either prepare or hw_params is mandatory.
     */
    pub prepare: Option<unsafe extern "C" fn(
        dev: *mut device,
        data: *mut core::ffi::c_void,
        fmt: *mut hdmi_codec_daifmt,
        hparms: *mut hdmi_codec_params,
    ) -> i32>,

    /*
     * Shuts down the audio stream.
     * Mandatory
     */
    pub audio_shutdown: Option<unsafe extern "C" fn(dev: *mut device, data: *mut core::ffi::c_void)>,

    /*
     * Mute/unmute HDMI audio stream.
     * Optional
     */
    pub mute_stream: Option<unsafe extern "C" fn(
        dev: *mut device,
        data: *mut core::ffi::c_void,
        enable: bool,
        direction: i32,
    ) -> i32>,

    /*
     * Provides EDID-Like-Data from connected HDMI device.
     * Optional
     */
    pub get_eld: Option<unsafe extern "C" fn(
        dev: *mut device,
        data: *mut core::ffi::c_void,
        buf: *mut u8,
        len: usize,
    ) -> i32>,

    /*
     * Getting DAI ID
     * Optional
     */
    pub get_dai_id: Option<unsafe extern "C" fn(
        comment: *mut snd_soc_component,
        endpoint: *mut device_node,
        data: *mut core::ffi::c_void,
    ) -> i32>,

    /*
     * Hook callback function to handle connector plug event.
     * Optional
     */
    pub hook_plugged_cb: Option<unsafe extern "C" fn(
        dev: *mut device,
        data: *mut core::ffi::c_void,
        func: hdmi_codec_plugged_cb,
        codec_dev: *mut device,
    ) -> i32>,
}

/* HDMI codec initalization data */
#[repr(C)]
pub struct hdmi_codec_pdata {
    pub ops: *const hdmi_codec_ops,
    pub i2s_formats: u64,
    pub i2s: u32,
    pub no_i2s_playback: u32,
    pub no_i2s_capture: u32,
    pub spdif: u32,
    pub no_spdif_playback: u32,
    pub no_spdif_capture: u32,
    pub no_capture_mute: u32,
    pub max_i2s_channels: i32,
    pub data: *mut core::ffi::c_void,
}

pub struct snd_soc_component;
pub struct snd_soc_jack;

pub const HDMI_CODEC_DRV_NAME: &[u8] = b"hdmi-audio-codec\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
