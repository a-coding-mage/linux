// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2022 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

// Dependencies from the original C source:
// <kunit/test.h>, <linux/module.h>, <sound/pcm.h>, <sound/pcm_params.h>,
// <sound/soc.h>, <uapi/sound/asound.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type u8 = ::std::os::raw::c_uchar;
type u32 = ::std::os::raw::c_uint;
type snd_pcm_format_t = ::std::os::raw::c_int;

const SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t = 2;
const SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format_t = 6;
const SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t = 10;
const SNDRV_PCM_HW_PARAM_FORMAT: ::std::os::raw::c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: ::std::os::raw::c_int = 10;
const SNDRV_PCM_HW_PARAM_RATE: ::std::os::raw::c_int = 11;

#[repr(C)]
struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_interval {
    min: ::std::os::raw::c_uint,
    max: ::std::os::raw::c_uint,
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct kunit_case {
    run_case: Option<unsafe extern "C" fn(*mut kunit)>,
}

#[repr(C)]
struct kunit_suite {
    name: *const ::std::os::raw::c_char,
    test_cases: *mut kunit_case,
}

unsafe extern "C" {
    fn _snd_pcm_hw_params_any(params: *mut snd_pcm_hw_params);
    fn snd_mask_none(mask: *mut snd_mask);
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: ::std::os::raw::c_int) -> *mut snd_mask;
    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        var: ::std::os::raw::c_int,
    ) -> *mut snd_interval;
    fn params_set_format(params: *mut snd_pcm_hw_params, fmt: snd_pcm_format_t);
    fn params_width(params: *const snd_pcm_hw_params) -> ::std::os::raw::c_int;
    fn snd_soc_tdm_params_to_bclk(
        params: *const snd_pcm_hw_params,
        tdm_width: ::std::os::raw::c_uint,
        tdm_slots: ::std::os::raw::c_uint,
        slot_multiple: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    fn snd_soc_params_to_bclk(params: *const snd_pcm_hw_params) -> ::std::os::raw::c_int;
}

#[repr(C)]
struct tdm_params_to_bclk_case {
    rate: u32,
    fmt: snd_pcm_format_t,
    channels: u8,
    tdm_width: u8,
    tdm_slots: u8,
    slot_multiple: u8,
    bclk: u32,
}

static tdm_params_to_bclk_cases: [tdm_params_to_bclk_case; 86] = [
    /* rate		fmt	   channels tdm_width tdm_slots slot_multiple bclk */
    /* From params only */
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 128000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 256000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 192000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 384000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 256000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 512000 },
    tdm_params_to_bclk_case { rate: 44100, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 705600 },
    tdm_params_to_bclk_case { rate: 44100, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 1411200 },
    tdm_params_to_bclk_case { rate: 44100, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 1058400 },
    tdm_params_to_bclk_case { rate: 44100, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 2116800 },
    tdm_params_to_bclk_case { rate: 44100, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 1411200 },
    tdm_params_to_bclk_case { rate: 44100, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 2822400 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 6144000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 12288000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 9216000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 18432000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 12288000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 0, bclk: 24576000 },
    /* I2S from params */
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 256000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 256000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 384000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 384000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 512000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 512000 },
    tdm_params_to_bclk_case { rate: 44100, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 1411200 },
    tdm_params_to_bclk_case { rate: 44100, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 1411200 },
    tdm_params_to_bclk_case { rate: 44100, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 2116800 },
    tdm_params_to_bclk_case { rate: 44100, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 2116800 },
    tdm_params_to_bclk_case { rate: 44100, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 2822400 },
    tdm_params_to_bclk_case { rate: 44100, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 2822400 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 12288000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 12288000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 18432000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 18432000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 1, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 24576000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 2, tdm_width: 0, tdm_slots: 0, slot_multiple: 2, bclk: 24576000 },
    /* Fixed 8-slot TDM, other values from params */
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 1, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 1024000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 2, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 1024000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 3, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 1024000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 4, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 1024000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 1, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 2048000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 2, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 2048000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 3, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 2048000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 4, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 2048000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 1, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 49152000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 2, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 49152000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 3, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 49152000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 4, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 49152000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 1, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 98304000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 2, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 98304000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 3, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 98304000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 4, tdm_width: 0, tdm_slots: 8, slot_multiple: 0, bclk: 98304000 },
    /* Fixed 32-bit TDM, other values from params */
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 1, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 256000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 2, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 512000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 3, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 768000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 4, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 1024000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 1, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 256000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 2, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 512000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 3, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 768000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 4, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 1024000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 1, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 12288000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 2, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 24576000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 3, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 36864000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 4, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 49152000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 1, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 12288000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 2, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 24576000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 3, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 36864000 },
    tdm_params_to_bclk_case { rate: 384000, fmt: SNDRV_PCM_FORMAT_S32_LE, channels: 4, tdm_width: 32, tdm_slots: 0, slot_multiple: 0, bclk: 49152000 },
    /* Fixed 6-slot 24-bit TDM, other values from params */
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 1, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 1152000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 2, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 1152000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 3, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 1152000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 4, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 1152000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 1, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 1152000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 2, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 1152000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 3, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 1152000 },
    tdm_params_to_bclk_case { rate: 8000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 4, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 1152000 },
    tdm_params_to_bclk_case { rate: 192000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 1, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 27648000 },
    tdm_params_to_bclk_case { rate: 192000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 2, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 27648000 },
    tdm_params_to_bclk_case { rate: 192000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 3, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 27648000 },
    tdm_params_to_bclk_case { rate: 192000, fmt: SNDRV_PCM_FORMAT_S16_LE, channels: 4, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 27648000 },
    tdm_params_to_bclk_case { rate: 192000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 1, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 27648000 },
    tdm_params_to_bclk_case { rate: 192000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 2, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 27648000 },
    tdm_params_to_bclk_case { rate: 192000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 3, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 27648000 },
    tdm_params_to_bclk_case { rate: 192000, fmt: SNDRV_PCM_FORMAT_S24_LE, channels: 4, tdm_width: 24, tdm_slots: 6, slot_multiple: 0, bclk: 27648000 },
];

unsafe extern "C" fn test_tdm_params_to_bclk_one(
    test: *mut kunit,
    rate: ::std::os::raw::c_uint,
    fmt: snd_pcm_format_t,
    channels: ::std::os::raw::c_uint,
    tdm_width: ::std::os::raw::c_uint,
    tdm_slots: ::std::os::raw::c_uint,
    slot_multiple: ::std::os::raw::c_uint,
    expected_bclk: ::std::os::raw::c_uint,
) {
    let mut params = ::std::mem::MaybeUninit::<snd_pcm_hw_params>::uninit();
    let got_bclk: ::std::os::raw::c_int;

    _snd_pcm_hw_params_any(params.as_mut_ptr());
    snd_mask_none(hw_param_mask(params.as_mut_ptr(), SNDRV_PCM_HW_PARAM_FORMAT));
    (*hw_param_interval(params.as_mut_ptr(), SNDRV_PCM_HW_PARAM_RATE)).min = rate;
    (*hw_param_interval(params.as_mut_ptr(), SNDRV_PCM_HW_PARAM_RATE)).max = rate;
    (*hw_param_interval(params.as_mut_ptr(), SNDRV_PCM_HW_PARAM_CHANNELS)).min = channels;
    (*hw_param_interval(params.as_mut_ptr(), SNDRV_PCM_HW_PARAM_CHANNELS)).max = channels;
    params_set_format(params.as_mut_ptr(), fmt);

    got_bclk = snd_soc_tdm_params_to_bclk(
        params.as_ptr(),
        tdm_width,
        tdm_slots,
        slot_multiple,
    );
    /* pr_debug("%s: r=%u sb=%u ch=%u tw=%u ts=%u sm=%u expected=%u got=%d\n",
     *          __func__,
     *          rate, params_width(&params), channels, tdm_width, tdm_slots, slot_multiple,
     *          expected_bclk, got_bclk);
     */
    let _ = params_width(params.as_ptr());
    assert_eq!(expected_bclk, got_bclk as ::std::os::raw::c_uint);
    let _ = test;
}

unsafe extern "C" fn test_tdm_params_to_bclk(test: *mut kunit) {
    let mut i: ::std::os::raw::c_int;

    i = 0;
    while (i as usize) < tdm_params_to_bclk_cases.len() {
        test_tdm_params_to_bclk_one(
            test,
            tdm_params_to_bclk_cases[i as usize].rate,
            tdm_params_to_bclk_cases[i as usize].fmt,
            tdm_params_to_bclk_cases[i as usize].channels as ::std::os::raw::c_uint,
            tdm_params_to_bclk_cases[i as usize].tdm_width as ::std::os::raw::c_uint,
            tdm_params_to_bclk_cases[i as usize].tdm_slots as ::std::os::raw::c_uint,
            tdm_params_to_bclk_cases[i as usize].slot_multiple as ::std::os::raw::c_uint,
            tdm_params_to_bclk_cases[i as usize].bclk,
        );

        if tdm_params_to_bclk_cases[i as usize].slot_multiple > 0 {
            i += 1;
            continue;
        }

        /* Slot multiple 1 should have the same effect as multiple 0 */
        test_tdm_params_to_bclk_one(
            test,
            tdm_params_to_bclk_cases[i as usize].rate,
            tdm_params_to_bclk_cases[i as usize].fmt,
            tdm_params_to_bclk_cases[i as usize].channels as ::std::os::raw::c_uint,
            tdm_params_to_bclk_cases[i as usize].tdm_width as ::std::os::raw::c_uint,
            tdm_params_to_bclk_cases[i as usize].tdm_slots as ::std::os::raw::c_uint,
            1,
            tdm_params_to_bclk_cases[i as usize].bclk,
        );
        i += 1;
    }
}

unsafe extern "C" fn test_snd_soc_params_to_bclk_one(
    test: *mut kunit,
    rate: ::std::os::raw::c_uint,
    fmt: snd_pcm_format_t,
    channels: ::std::os::raw::c_uint,
    expected_bclk: ::std::os::raw::c_uint,
) {
    let mut params = ::std::mem::MaybeUninit::<snd_pcm_hw_params>::uninit();
    let got_bclk: ::std::os::raw::c_int;

    _snd_pcm_hw_params_any(params.as_mut_ptr());
    snd_mask_none(hw_param_mask(params.as_mut_ptr(), SNDRV_PCM_HW_PARAM_FORMAT));
    (*hw_param_interval(params.as_mut_ptr(), SNDRV_PCM_HW_PARAM_RATE)).min = rate;
    (*hw_param_interval(params.as_mut_ptr(), SNDRV_PCM_HW_PARAM_RATE)).max = rate;
    (*hw_param_interval(params.as_mut_ptr(), SNDRV_PCM_HW_PARAM_CHANNELS)).min = channels;
    (*hw_param_interval(params.as_mut_ptr(), SNDRV_PCM_HW_PARAM_CHANNELS)).max = channels;
    params_set_format(params.as_mut_ptr(), fmt);

    got_bclk = snd_soc_params_to_bclk(params.as_ptr());
    /* pr_debug("%s: r=%u sb=%u ch=%u expected=%u got=%d\n",
     *          __func__,
     *          rate, params_width(&params), channels, expected_bclk, got_bclk);
     */
    let _ = params_width(params.as_ptr());
    assert_eq!(expected_bclk, got_bclk as ::std::os::raw::c_uint);
    let _ = test;
}

unsafe extern "C" fn test_snd_soc_params_to_bclk(test: *mut kunit) {
    let mut i: ::std::os::raw::c_int;

    i = 0;
    while (i as usize) < tdm_params_to_bclk_cases.len() {
        /*
         * snd_soc_params_to_bclk() is all the test cases where
         * snd_pcm_hw_params values are not overridden.
         */
        if ((tdm_params_to_bclk_cases[i as usize].tdm_width as ::std::os::raw::c_int)
            | (tdm_params_to_bclk_cases[i as usize].tdm_slots as ::std::os::raw::c_int)
            | (tdm_params_to_bclk_cases[i as usize].slot_multiple as ::std::os::raw::c_int))
            != 0
        {
            i += 1;
            continue;
        }

        test_snd_soc_params_to_bclk_one(
            test,
            tdm_params_to_bclk_cases[i as usize].rate,
            tdm_params_to_bclk_cases[i as usize].fmt,
            tdm_params_to_bclk_cases[i as usize].channels as ::std::os::raw::c_uint,
            tdm_params_to_bclk_cases[i as usize].bclk,
        );
        i += 1;
    }
}

static mut soc_utils_test_cases: [kunit_case; 3] = [
    kunit_case {
        run_case: Some(test_tdm_params_to_bclk),
    },
    kunit_case {
        run_case: Some(test_snd_soc_params_to_bclk),
    },
    kunit_case { run_case: None },
];

static mut soc_utils_test_suite: kunit_suite = kunit_suite {
    name: b"soc-utils\0".as_ptr() as *const ::std::os::raw::c_char,
    test_cases: unsafe { soc_utils_test_cases.as_mut_ptr() },
};

// Original C registration and metadata:
// kunit_test_suites(&soc_utils_test_suite);
// MODULE_DESCRIPTION("ASoC soc-utils kunit test");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
