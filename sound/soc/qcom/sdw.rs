// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018-2023, Linaro Limited.
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

// C dependencies:
// <dt-bindings/sound/qcom,lpass.h>
// <dt-bindings/sound/qcom,q6afe.h>
// <linux/module.h>
// <sound/soc.h>
// "sdw.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ptr;

pub type bool_ = bool;
pub type u32 = core::ffi::c_uint;
pub type c_int = core::ffi::c_int;
pub type c_char = core::ffi::c_char;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct sdw_stream_runtime {
    _private: [u8; 0],
}

unsafe extern "C" {
    static WSA_CODEC_DMA_RX_0: c_int;
    static WSA_CODEC_DMA_TX_0: c_int;
    static WSA_CODEC_DMA_RX_1: c_int;
    static WSA_CODEC_DMA_TX_1: c_int;
    static WSA_CODEC_DMA_TX_2: c_int;
    static RX_CODEC_DMA_RX_0: c_int;
    static TX_CODEC_DMA_TX_0: c_int;
    static RX_CODEC_DMA_RX_1: c_int;
    static TX_CODEC_DMA_TX_1: c_int;
    static RX_CODEC_DMA_RX_2: c_int;
    static TX_CODEC_DMA_TX_2: c_int;
    static RX_CODEC_DMA_RX_3: c_int;
    static TX_CODEC_DMA_TX_3: c_int;
    static RX_CODEC_DMA_RX_4: c_int;
    static TX_CODEC_DMA_TX_4: c_int;
    static RX_CODEC_DMA_RX_5: c_int;
    static TX_CODEC_DMA_TX_5: c_int;
    static RX_CODEC_DMA_RX_6: c_int;
    static RX_CODEC_DMA_RX_7: c_int;
    static SLIMBUS_0_RX: c_int;
    static SLIMBUS_6_TX: c_int;
    static LPASS_CDC_DMA_TX3: c_int;
    static LPASS_CDC_DMA_RX0: c_int;

    static SDW_MAX_PORTS: usize;
    static SDW_STREAM_PCM: c_int;
    static ENOMEM: c_int;
    static ENOTSUPP: c_int;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn sdw_alloc_stream(
        name: *const c_char,
        stream_type: c_int,
    ) -> *mut sdw_stream_runtime;
    fn sdw_release_stream(sruntime: *mut sdw_stream_runtime);
    fn snd_soc_dai_set_stream(
        dai: *mut snd_soc_dai,
        stream: *mut sdw_stream_runtime,
        direction: c_int,
    ) -> c_int;
    fn snd_soc_dai_get_channel_map(
        dai: *mut snd_soc_dai,
        tx_num: *mut u32,
        tx_slot: *mut u32,
        rx_num: *mut u32,
        rx_slot: *mut u32,
    ) -> c_int;
    fn snd_soc_dai_set_channel_map(
        dai: *mut snd_soc_dai,
        tx_num: u32,
        tx_slot: *mut u32,
        rx_num: u32,
        rx_slot: *mut u32,
    ) -> c_int;
    fn snd_soc_dai_get_stream(
        dai: *mut snd_soc_dai,
        direction: c_int,
    ) -> *mut sdw_stream_runtime;
    fn sdw_prepare_stream(sruntime: *mut sdw_stream_runtime) -> c_int;
    fn sdw_enable_stream(sruntime: *mut sdw_stream_runtime) -> c_int;
    fn sdw_deprepare_stream(sruntime: *mut sdw_stream_runtime);
    fn sdw_disable_stream(sruntime: *mut sdw_stream_runtime);
    fn ERR_PTR(error: c_int) -> *mut sdw_stream_runtime;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

macro_rules! for_each_rtd_codec_dais {
    ($rtd:expr, $i:ident, $codec_dai:ident, $body:block) => {
        compile_error!("for_each_rtd_codec_dais is supplied by external ASoC dependencies")
    };
}

unsafe fn qcom_snd_is_sdw_dai(id: c_int) -> bool {
    if id == WSA_CODEC_DMA_RX_0
        || id == WSA_CODEC_DMA_TX_0
        || id == WSA_CODEC_DMA_RX_1
        || id == WSA_CODEC_DMA_TX_1
        || id == WSA_CODEC_DMA_TX_2
        || id == RX_CODEC_DMA_RX_0
        || id == TX_CODEC_DMA_TX_0
        || id == RX_CODEC_DMA_RX_1
        || id == TX_CODEC_DMA_TX_1
        || id == RX_CODEC_DMA_RX_2
        || id == TX_CODEC_DMA_TX_2
        || id == RX_CODEC_DMA_RX_3
        || id == TX_CODEC_DMA_TX_3
        || id == RX_CODEC_DMA_RX_4
        || id == TX_CODEC_DMA_TX_4
        || id == RX_CODEC_DMA_RX_5
        || id == TX_CODEC_DMA_TX_5
        || id == RX_CODEC_DMA_RX_6
        || id == RX_CODEC_DMA_RX_7
        || (id >= SLIMBUS_0_RX && id <= SLIMBUS_6_TX)
    {
        return true;
    }

    /*
     * DSP Bypass usecase, cpu dai index overlaps with DSP dai ids,
     * DO NOT MERGE into top switch case
     */
    if id == LPASS_CDC_DMA_TX3 || id == LPASS_CDC_DMA_RX0 {
        return true;
    }

    false
}

/**
 * qcom_snd_sdw_startup() - Helper to start Soundwire stream for SoC audio card
 * @substream: The PCM substream from audio, as passed to snd_soc_ops->startup()
 *
 * Helper for the SoC audio card (snd_soc_ops->startup()) to allocate and set
 * Soundwire stream runtime to each codec DAI.
 *
 * The shutdown() callback should call sdw_release_stream() on the same
 * sdw_stream_runtime.
 *
 * Return: 0 or errno
 */
#[no_mangle]
pub unsafe extern "C" fn qcom_snd_sdw_startup(substream: *mut snd_pcm_substream) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut rx_ch: [u32; SDW_MAX_PORTS] = core::mem::zeroed();
    let mut tx_ch: [u32; SDW_MAX_PORTS] = core::mem::zeroed();
    let sruntime: *mut sdw_stream_runtime;
    let mut codec_dai: *mut snd_soc_dai;
    let mut rx_ch_cnt: u32 = 0;
    let mut tx_ch_cnt: u32 = 0;
    let mut ret: c_int;
    let mut i: c_int;
    let mut j: c_int;

    if !qcom_snd_is_sdw_dai((*cpu_dai).id) {
        return 0;
    }

    sruntime = sdw_alloc_stream((*cpu_dai).name, SDW_STREAM_PCM);
    if sruntime.is_null() {
        return -ENOMEM;
    }

    for_each_rtd_codec_dais!(rtd, i, codec_dai, {
        ret = snd_soc_dai_set_stream(codec_dai, sruntime, (*substream).stream);
        if ret < 0 && ret != -ENOTSUPP {
            dev_err(
                (*rtd).dev,
                b"Failed to set sdw stream on %s\n\0".as_ptr() as *const c_char,
                (*codec_dai).name,
            );
            sdw_release_stream(sruntime);
            return ret;
        } else if ret == -ENOTSUPP {
            /* Ignore unsupported */
            continue;
        }

        ret = snd_soc_dai_get_channel_map(
            codec_dai,
            &mut tx_ch_cnt,
            tx_ch.as_mut_ptr(),
            &mut rx_ch_cnt,
            rx_ch.as_mut_ptr(),
        );
        if ret != 0 && ret != -ENOTSUPP {
            dev_err(
                (*rtd).dev,
                b"Failed to get codec chan map %s\n\0".as_ptr() as *const c_char,
                (*codec_dai).name,
            );
            sdw_release_stream(sruntime);
            return ret;
        } else if ret == -ENOTSUPP {
            /* Ignore unsupported */
            continue;
        }
    });

    if (*cpu_dai).id == RX_CODEC_DMA_RX_0 || (*cpu_dai).id == TX_CODEC_DMA_TX_3 {
        if tx_ch_cnt != 0 || rx_ch_cnt != 0 {
            for_each_rtd_codec_dais!(rtd, j, codec_dai, {
                ret = snd_soc_dai_set_channel_map(
                    codec_dai,
                    tx_ch_cnt,
                    tx_ch.as_mut_ptr(),
                    rx_ch_cnt,
                    rx_ch.as_mut_ptr(),
                );
                if ret != 0 && ret != -ENOTSUPP {
                    sdw_release_stream(sruntime);
                    return ret;
                }
            });
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn qcom_snd_sdw_prepare(
    substream: *mut snd_pcm_substream,
    stream_prepared: *mut bool_,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let sruntime: *mut sdw_stream_runtime;
    let mut ret: c_int;

    if !qcom_snd_is_sdw_dai((*cpu_dai).id) {
        return 0;
    }

    sruntime = qcom_snd_sdw_get_stream(substream);
    if sruntime.is_null() {
        return 0;
    }

    if *stream_prepared {
        return 0;
    }

    ret = sdw_prepare_stream(sruntime);
    if ret != 0 {
        return ret;
    }

    /**
     * NOTE: there is a strict hw requirement about the ordering of port
     * enables and actual WSA881x PA enable. PA enable should only happen
     * after soundwire ports are enabled if not DC on the line is
     * accumulated resulting in Click/Pop Noise
     * PA enable/mute are handled as part of codec DAPM and digital mute.
     */

    ret = sdw_enable_stream(sruntime);
    if ret != 0 {
        sdw_deprepare_stream(sruntime);
        return ret;
    }
    *stream_prepared = true;

    ret
}

#[no_mangle]
pub unsafe extern "C" fn qcom_snd_sdw_get_stream(
    substream: *mut snd_pcm_substream,
) -> *mut sdw_stream_runtime {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let mut codec_dai: *mut snd_soc_dai;
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut sruntime: *mut sdw_stream_runtime;
    let mut i: c_int;

    if !qcom_snd_is_sdw_dai((*cpu_dai).id) {
        return ptr::null_mut();
    }

    for_each_rtd_codec_dais!(rtd, i, codec_dai, {
        sruntime = snd_soc_dai_get_stream(codec_dai, (*substream).stream);
        if sruntime != ERR_PTR(-ENOTSUPP) {
            return sruntime;
        }
    });
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn qcom_snd_sdw_shutdown(substream: *mut snd_pcm_substream) {
    let sruntime: *mut sdw_stream_runtime = qcom_snd_sdw_get_stream(substream);

    sdw_release_stream(sruntime);
}

#[no_mangle]
pub unsafe extern "C" fn qcom_snd_sdw_hw_free(
    substream: *mut snd_pcm_substream,
    stream_prepared: *mut bool_,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let sruntime: *mut sdw_stream_runtime;

    if !qcom_snd_is_sdw_dai((*cpu_dai).id) {
        return 0;
    }

    sruntime = qcom_snd_sdw_get_stream(substream);
    if !sruntime.is_null() && *stream_prepared {
        sdw_disable_stream(sruntime);
        sdw_deprepare_stream(sruntime);
        *stream_prepared = false;
    }

    0
}

// EXPORT_SYMBOL_GPL(qcom_snd_sdw_startup);
// EXPORT_SYMBOL_GPL(qcom_snd_sdw_prepare);
// EXPORT_SYMBOL_GPL(qcom_snd_sdw_get_stream);
// EXPORT_SYMBOL_GPL(qcom_snd_sdw_shutdown);
// EXPORT_SYMBOL_GPL(qcom_snd_sdw_hw_free);
// MODULE_DESCRIPTION("Qualcomm ASoC SoundWire helper functions");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
