/* SPDX-License-Identifier: (GPL-2.0 OR MIT)
 *
 * Copyright (c) 2018 Baylibre SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

/* Dependencies from the original C header:
 * <linux/clk.h>
 * <linux/regmap.h>
 * <sound/pcm.h>
 * <sound/soc.h>
 * <sound/soc-dai.h>
 */

pub const AXG_TDM_NUM_LANES: core::ffi::c_uint = 4;
pub const AXG_TDM_CHANNEL_MAX: core::ffi::c_uint = 128;
pub const AXG_TDM_FORMATS: core::ffi::c_ulong = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

#[repr(C)]
pub struct axg_tdm_iface {
    pub sclk: *mut clk,
    pub lrclk: *mut clk,
    pub mclk: *mut clk,
    pub mclk_rate: core::ffi::c_ulong,

    /* format is common to all the DAIs of the iface */
    pub fmt: core::ffi::c_uint,
    pub slots: core::ffi::c_uint,
    pub slot_width: core::ffi::c_uint,

    /* For component wide symmetry */
    pub rate: core::ffi::c_int,
}

#[inline]
pub unsafe fn axg_tdm_lrclk_invert(fmt: core::ffi::c_uint) -> bool {
    ((fmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S)
        ^ ((fmt & (SND_SOC_DAIFMT_IB_IF | SND_SOC_DAIFMT_NB_IF)) != 0)
}

#[inline]
pub unsafe fn axg_tdm_sclk_invert(fmt: core::ffi::c_uint) -> bool {
    (fmt & (SND_SOC_DAIFMT_IB_IF | SND_SOC_DAIFMT_IB_NF)) != 0
}

#[repr(C)]
pub struct axg_tdm_stream {
    pub iface: *mut axg_tdm_iface,
    pub formatter_list: list_head,
    pub lock: mutex,
    pub channels: core::ffi::c_uint,
    pub width: core::ffi::c_uint,
    pub physical_width: core::ffi::c_uint,
    pub mask: *mut u32,
    pub ready: bool,

    /* For continuous clock tracking */
    pub clk_enabled: bool,
}

unsafe extern "C" {
    pub fn axg_tdm_stream_alloc(iface: *mut axg_tdm_iface) -> *mut axg_tdm_stream;
    pub fn axg_tdm_stream_free(ts: *mut axg_tdm_stream);
    pub fn axg_tdm_stream_start(ts: *mut axg_tdm_stream) -> core::ffi::c_int;
    pub fn axg_tdm_stream_stop(ts: *mut axg_tdm_stream);
    pub fn axg_tdm_stream_set_cont_clocks(
        ts: *mut axg_tdm_stream,
        fmt: core::ffi::c_uint,
    ) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn axg_tdm_stream_reset(ts: *mut axg_tdm_stream) -> core::ffi::c_int {
    unsafe {
        axg_tdm_stream_stop(ts);
        axg_tdm_stream_start(ts)
    }
}

unsafe extern "C" {
    pub fn axg_tdm_set_tdm_slots(
        dai: *mut snd_soc_dai,
        tx_mask: *mut u32,
        rx_mask: *mut u32,
        slots: core::ffi::c_uint,
        slot_width: core::ffi::c_uint,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
