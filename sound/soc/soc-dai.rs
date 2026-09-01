// SPDX-License-Identifier: GPL-2.0
//
// soc-dai.c
//
// Copyright (C) 2019 Renesas Electronics Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//

// C dependencies:
// #include <sound/soc.h>
// #include <sound/soc-dai.h>
// #include <sound/soc-link.h>

use core::ffi::{c_char, c_int, c_uint, c_void};

type u64 = u64;
type size_t = usize;
type snd_pcm_sframes_t = isize;

extern "C" {
    fn snd_soc_ret(dev: *mut c_void, ret: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_component_set_sysclk(
        component: *mut snd_soc_component,
        clk_id: c_int,
        source: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_component_set_pll(
        component: *mut snd_soc_component,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_dai_tdm_mask_set(dai: *mut snd_soc_dai, stream: c_int, mask: c_uint);
    fn snd_soc_dai_get_pcm_stream(dai: *const snd_soc_dai, dir: c_int) -> *const snd_soc_pcm_stream;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_codec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_tstamp64 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub active: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_stream {
    pub active: c_int,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut c_void,
    pub name: *const c_char,
    pub driver: *mut snd_soc_dai_driver,
    pub component: *mut snd_soc_component,
    pub bclk_ratio: c_uint,
    pub bclk: *mut clk,
    pub mark_hw_params: *mut snd_pcm_substream,
    pub mark_startup: *mut snd_pcm_substream,
    pub mark_trigger: *mut snd_pcm_substream,
    pub mark_compr_startup: *mut snd_compr_stream,
    pub stream: [snd_soc_dai_stream; 2],
    pub probed: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub ops: *const snd_soc_dai_ops,
    pub cops: *const snd_soc_cdai_ops,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_clkdiv: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub num_auto_selectable_formats: c_int,
    pub auto_selectable_formats: *const u64,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub xlate_tdm_slot_mask: Option<unsafe extern "C" fn(c_uint, *mut c_uint, *mut c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub set_tdm_idle: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub set_channel_map: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, *const c_uint, c_uint, *const c_uint) -> c_int>,
    pub get_channel_map: Option<unsafe extern "C" fn(*const snd_soc_dai, *mut c_uint, *mut c_uint, *mut c_uint, *mut c_uint) -> c_int>,
    pub set_tristate: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub mute_unmute_on_trigger: c_int,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_int,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub compress_new: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub probe_order: c_int,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub remove_order: c_int,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub delay: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> snd_pcm_sframes_t>,
}

#[repr(C)]
pub struct snd_soc_cdai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_soc_dai)>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_compr_stream, c_int, *mut snd_soc_dai) -> c_int>,
    pub set_params: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_params, *mut snd_soc_dai) -> c_int>,
    pub get_params: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_codec, *mut snd_soc_dai) -> c_int>,
    pub ack: Option<unsafe extern "C" fn(*mut snd_compr_stream, size_t, *mut snd_soc_dai) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_tstamp64, *mut snd_soc_dai) -> c_int>,
    pub set_metadata: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_metadata, *mut snd_soc_dai) -> c_int>,
    pub get_metadata: Option<unsafe extern "C" fn(*mut snd_compr_stream, *mut snd_compr_metadata, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub num_cpus: c_int,
    pub num_codecs: c_int,
    pub dai_fmt: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
    pub dais: *mut *mut snd_soc_dai,
    pub num_dais: c_int,
    pub num_cpus: c_int,
    pub num_codecs: c_int,
}

unsafe fn _soc_dai_ret(dai: *const snd_soc_dai, func: *const c_char, ret: c_int) -> c_int {
    snd_soc_ret((*dai).dev, ret, c"at %s() on %s\n".as_ptr(), func, (*dai).name)
}

unsafe fn soc_dai_ret(dai: *const snd_soc_dai, ret: c_int) -> c_int {
    _soc_dai_ret(dai, c"unknown".as_ptr(), ret)
}

unsafe fn snd_soc_dai_ops(dai: *const snd_soc_dai) -> *const snd_soc_dai_ops {
    if (*(*dai).driver).ops.is_null() {
        core::ptr::null()
    } else {
        (*(*dai).driver).ops
    }
}

unsafe fn snd_soc_dai_cops(dai: *const snd_soc_dai) -> *const snd_soc_cdai_ops {
    if (*(*dai).driver).cops.is_null() {
        core::ptr::null()
    } else {
        (*(*dai).driver).cops
    }
}

/*
 * We might want to check substream by using list.
 * In such case, we can update these macros.
 */

pub unsafe extern "C" fn snd_soc_dai_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let ret: c_int;
    let ops = snd_soc_dai_ops(dai);

    if !ops.is_null() && (*ops).set_sysclk.is_some() {
        ret = ((*ops).set_sysclk.unwrap())(dai, clk_id, freq, dir);
    } else {
        ret = snd_soc_component_set_sysclk((*dai).component, clk_id, 0, freq, dir);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_set_clkdiv(
    dai: *mut snd_soc_dai,
    div_id: c_int,
    div: c_int,
) -> c_int {
    let mut ret: c_int = -EINVAL;
    let ops = snd_soc_dai_ops(dai);

    if !ops.is_null() && (*ops).set_clkdiv.is_some() {
        ret = ((*ops).set_clkdiv.unwrap())(dai, div_id, div);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_set_pll(
    dai: *mut snd_soc_dai,
    pll_id: c_int,
    source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let ret: c_int;
    let ops = snd_soc_dai_ops(dai);

    if !ops.is_null() && (*ops).set_pll.is_some() {
        ret = ((*ops).set_pll.unwrap())(dai, pll_id, source, freq_in, freq_out);
    } else {
        ret = snd_soc_component_set_pll((*dai).component, pll_id, source, freq_in, freq_out);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_set_bclk_ratio(
    dai: *mut snd_soc_dai,
    ratio: c_uint,
) -> c_int {
    let mut ret: c_int = -ENOTSUPP;
    let ops = snd_soc_dai_ops(dai);

    if !ops.is_null() && (*ops).set_bclk_ratio.is_some() {
        ret = ((*ops).set_bclk_ratio.unwrap())(dai, ratio);
    }

    if ret == 0 {
        (*dai).bclk_ratio = ratio;
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_set_bclk_clk(dai: *mut snd_soc_dai, bclk: *mut clk) {
    (*dai).bclk = bclk;
}

unsafe fn soc_dai_fmt_match_cnt(fmt: u64) -> c_int {
    let mut cnt: c_int = 0;

    if fmt & SND_SOC_POSSIBLE_DAIFMT_FORMAT_MASK != 0 {
        cnt += 1;
    }
    if fmt & SND_SOC_POSSIBLE_DAIFMT_CLOCK_MASK != 0 {
        cnt += 1;
    }
    if fmt & SND_SOC_POSSIBLE_DAIFMT_INV_MASK != 0 {
        cnt += 1;
    }

    cnt
}

unsafe fn soc_dai_auto_select_format(
    fmt: u64,
    rtd: *const snd_soc_pcm_runtime,
    idx: c_int,
    best_fmt: *mut u64,
) {
    let max_idx = (*(*rtd).dai_link).num_cpus + (*(*rtd).dai_link).num_codecs;

    /*
     * NOTE
     * It doesn't support Multi CPU/Codec for now
     */
    if (*(*rtd).dai_link).num_cpus != 1 || (*(*rtd).dai_link).num_codecs != 1 {
        return;
    }

    if idx >= max_idx {
        return;
    }

    let dai = *(*rtd).dais.offset(idx as isize);
    let ops = (*(*dai).driver).ops;

    /* zero chance of auto select format */
    if ops.is_null() || (*ops).num_auto_selectable_formats == 0 {
        return;
    }

    /*
     ****************************
     *            NOTE
     ****************************
     * Using .auto_selectable_formats is not mandatory,
     * It try to find best formats as much as possible, but automatically selecting the
     * perfect format is impossible. So you can select full or missing format manually
     * from Sound Card.
     *
     * ex)
     * CPU					Codec
     * (A)[0] I2S/LEFT_J : IB_NF/IB_IF	(X)[0] I2S/DSP_A: NB_NF : GATED
     * (B)[1] DSP_A/DSP_B: NB_NF/IB_NF	(Y)[1] LEFT_J:    NB_NF : GATED
     * (C)[2] ...
     *
     * 1. (A) -> (X) : I2S		:update best format
     * 2. (A) -> (Y) : LEFT_J
     * 3. (B) -> (X) : DSP_A/NB_NF	:update best format
     * 4. (B) -> (Y) : NB_NF
     * 5. (C) -> (X) ...
     * 6. (C) -> (Y) ...
     * ...
     *
     * In above case GATED will not be selected
     */

    /* find best formats */
    let mut i: c_int = 0;
    while i < (*ops).num_auto_selectable_formats {
        let available_fmt = fmt & *(*ops).auto_selectable_formats.offset(i as isize);

        /* In case of last DAI */
        if idx + 1 >= max_idx {
            let cnt1 = soc_dai_fmt_match_cnt(*best_fmt);
            let cnt2 = soc_dai_fmt_match_cnt(available_fmt);

            if cnt1 < cnt2 {
                *best_fmt = available_fmt;
            }
        } else {
            /* parse with next DAI */
            soc_dai_auto_select_format(available_fmt, rtd, idx + 1, best_fmt);
        }

        i += 1;
    }
}

unsafe fn soc_dai_convert_possiblefmt_to_daifmt(
    possible_fmt: u64,
    configured_fmt: c_uint,
) -> c_uint {
    let mut fmt: c_uint = 0;
    let mut mask: c_uint = 0;

    /*
     * convert POSSIBLE_DAIFMT to DAIFMT
     *
     * Some basic/default settings on each is defined as 0.
     * see
     *	SND_SOC_DAIFMT_NB_NF
     *	SND_SOC_DAIFMT_GATED
     *
     * SND_SOC_DAIFMT_xxx_MASK can't notice it if Sound Card specify
     * these value, and will be overwrite to auto selected value.
     *
     * To avoid such issue, loop from 63 to 0 here.
     * Small number of SND_SOC_POSSIBLE_xxx will be Hi priority.
     * Basic/Default settings of each part and above are defined
     * as Hi priority (= small number) of SND_SOC_POSSIBLE_xxx.
     */
    let mut i: c_int = 63;
    while i >= 0 {
        let pos: u64 = 1u64 << i;

        match possible_fmt & pos {
            /*
             * for format
             */
            SND_SOC_POSSIBLE_DAIFMT_I2S
            | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J
            | SND_SOC_POSSIBLE_DAIFMT_LEFT_J
            | SND_SOC_POSSIBLE_DAIFMT_DSP_A
            | SND_SOC_POSSIBLE_DAIFMT_DSP_B
            | SND_SOC_POSSIBLE_DAIFMT_AC97
            | SND_SOC_POSSIBLE_DAIFMT_PDM => {
                fmt = (fmt & !SND_SOC_DAIFMT_FORMAT_MASK) | i as c_uint;
            }
            /*
             * for clock
             */
            SND_SOC_POSSIBLE_DAIFMT_CONT => {
                fmt = (fmt & !SND_SOC_DAIFMT_CLOCK_MASK) | SND_SOC_DAIFMT_CONT;
            }
            SND_SOC_POSSIBLE_DAIFMT_GATED => {
                fmt = (fmt & !SND_SOC_DAIFMT_CLOCK_MASK) | SND_SOC_DAIFMT_GATED;
            }
            /*
             * for clock invert
             */
            SND_SOC_POSSIBLE_DAIFMT_NB_NF => {
                fmt = (fmt & !SND_SOC_DAIFMT_INV_MASK) | SND_SOC_DAIFMT_NB_NF;
            }
            SND_SOC_POSSIBLE_DAIFMT_NB_IF => {
                fmt = (fmt & !SND_SOC_DAIFMT_INV_MASK) | SND_SOC_DAIFMT_NB_IF;
            }
            SND_SOC_POSSIBLE_DAIFMT_IB_NF => {
                fmt = (fmt & !SND_SOC_DAIFMT_INV_MASK) | SND_SOC_DAIFMT_IB_NF;
            }
            SND_SOC_POSSIBLE_DAIFMT_IB_IF => {
                fmt = (fmt & !SND_SOC_DAIFMT_INV_MASK) | SND_SOC_DAIFMT_IB_IF;
            }
            _ => {}
        }

        i -= 1;
    }

    /*
     * Some driver might have very complex limitation.
     * In such case, user want to auto-select non-limitation part,
     * and want to manually specify complex part.
     *
     * Or for example, if both CPU and Codec can be clock provider,
     * but because of its quality, user want to specify it manually.
     *
     * Ignore already configured format if exist
     */
    if configured_fmt & SND_SOC_DAIFMT_FORMAT_MASK == 0 {
        mask |= SND_SOC_DAIFMT_FORMAT_MASK;
    }
    if configured_fmt & SND_SOC_DAIFMT_CLOCK_MASK == 0 {
        mask |= SND_SOC_DAIFMT_CLOCK_MASK;
    }
    if configured_fmt & SND_SOC_DAIFMT_INV_MASK == 0 {
        mask |= SND_SOC_DAIFMT_INV_MASK;
    }
    if configured_fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK == 0 {
        mask |= SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;
    }

    configured_fmt | (fmt & mask)
}

pub unsafe extern "C" fn snd_soc_dai_auto_select_format(rtd: *const snd_soc_pcm_runtime) -> c_uint {
    let dai_link = (*rtd).dai_link;
    let mut possible_fmt: u64 = 0;

    soc_dai_auto_select_format(!0u64, rtd, 0, &mut possible_fmt);

    soc_dai_convert_possiblefmt_to_daifmt(possible_fmt, (*dai_link).dai_fmt)
}

pub unsafe extern "C" fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let mut ret: c_int = -ENOTSUPP;
    let ops = snd_soc_dai_ops(dai);

    if !ops.is_null() && (*ops).set_fmt.is_some() {
        ret = ((*ops).set_fmt.unwrap())(dai, fmt);
    }

    soc_dai_ret(dai, ret)
}

unsafe fn snd_soc_xlate_tdm_slot_mask(
    slots: c_uint,
    tx_mask: *mut c_uint,
    rx_mask: *mut c_uint,
) -> c_int {
    if *tx_mask != 0 || *rx_mask != 0 {
        return 0;
    }

    if slots == 0 {
        return -EINVAL;
    }

    *tx_mask = (1u32 << slots) - 1;
    *rx_mask = (1u32 << slots) - 1;

    0
}

pub unsafe extern "C" fn snd_soc_dai_set_tdm_slot(
    dai: *mut snd_soc_dai,
    mut tx_mask: c_uint,
    mut rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let mut ret: c_int = -ENOTSUPP;
    let mut stream: c_int;
    let mut tdm_mask: [*mut c_uint; 2] = [&mut tx_mask, &mut rx_mask];
    let ops = snd_soc_dai_ops(dai);

    if slots != 0 {
        if !ops.is_null() && (*ops).xlate_tdm_slot_mask.is_some() {
            ret = ((*ops).xlate_tdm_slot_mask.unwrap())(slots as c_uint, &mut tx_mask, &mut rx_mask);
        } else {
            ret = snd_soc_xlate_tdm_slot_mask(slots as c_uint, &mut tx_mask, &mut rx_mask);
        }
        if ret != 0 {
            return soc_dai_ret(dai, ret);
        }
    }

    stream = 0;
    while stream < 2 {
        snd_soc_dai_tdm_mask_set(dai, stream, *tdm_mask[stream as usize]);
        stream += 1;
    }

    if !ops.is_null() && (*ops).set_tdm_slot.is_some() {
        ret = ((*ops).set_tdm_slot.unwrap())(dai, tx_mask, rx_mask, slots, slot_width);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_set_tdm_idle(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    tx_mode: c_int,
    rx_mode: c_int,
) -> c_int {
    let mut ret: c_int = -EOPNOTSUPP;
    let ops = snd_soc_dai_ops(dai);

    /* You can't write to the RX line */
    if rx_mode == SND_SOC_DAI_TDM_IDLE_ZERO {
        return soc_dai_ret(dai, -EINVAL);
    }

    if !ops.is_null() && (*ops).set_tdm_idle.is_some() {
        ret = ((*ops).set_tdm_idle.unwrap())(dai, tx_mask, rx_mask, tx_mode, rx_mode);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_set_channel_map(
    dai: *mut snd_soc_dai,
    tx_num: c_uint,
    tx_slot: *const c_uint,
    rx_num: c_uint,
    rx_slot: *const c_uint,
) -> c_int {
    let mut ret: c_int = -ENOTSUPP;
    let ops = snd_soc_dai_ops(dai);

    if !ops.is_null() && (*ops).set_channel_map.is_some() {
        ret = ((*ops).set_channel_map.unwrap())(dai, tx_num, tx_slot, rx_num, rx_slot);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_get_channel_map(
    dai: *const snd_soc_dai,
    tx_num: *mut c_uint,
    tx_slot: *mut c_uint,
    rx_num: *mut c_uint,
    rx_slot: *mut c_uint,
) -> c_int {
    let mut ret: c_int = -ENOTSUPP;
    let ops = snd_soc_dai_ops(dai);

    if !ops.is_null() && (*ops).get_channel_map.is_some() {
        ret = ((*ops).get_channel_map.unwrap())(dai, tx_num, tx_slot, rx_num, rx_slot);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_set_tristate(
    dai: *mut snd_soc_dai,
    tristate: c_int,
) -> c_int {
    let mut ret: c_int = -EINVAL;
    let ops = snd_soc_dai_ops(dai);

    if !ops.is_null() && (*ops).set_tristate.is_some() {
        ret = ((*ops).set_tristate.unwrap())(dai, tristate);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_prepare(
    dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let mut ret: c_int = 0;
    let ops = snd_soc_dai_ops(dai);

    if !snd_soc_dai_stream_valid(dai, (*substream).stream) {
        return 0;
    }

    if !ops.is_null() && (*ops).prepare.is_some() {
        ret = ((*ops).prepare.unwrap())(substream, dai);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_mute_is_ctrled_at_trigger(dai: *mut snd_soc_dai) -> c_int {
    let ops = snd_soc_dai_ops(dai);

    if !ops.is_null() {
        return (*ops).mute_unmute_on_trigger;
    }

    0
}

pub unsafe extern "C" fn snd_soc_dai_digital_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    direction: c_int,
) -> c_int {
    let mut ret: c_int = -ENOTSUPP;
    let ops = snd_soc_dai_ops(dai);

    /*
     * ignore if direction was CAPTURE
     * and it had .no_capture_mute flag
     */
    if !ops.is_null()
        && (*ops).mute_stream.is_some()
        && (direction == SNDRV_PCM_STREAM_PLAYBACK || (*ops).no_capture_mute == 0)
    {
        ret = ((*ops).mute_stream.unwrap())(dai, mute, direction);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_hw_params(
    dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut ret: c_int = 0;
    let ops = snd_soc_dai_ops(dai);

    if !ops.is_null() && (*ops).hw_params.is_some() {
        ret = ((*ops).hw_params.unwrap())(substream, params, dai);
    }

    /* mark substream if succeeded */
    if ret == 0 {
        (*dai).mark_hw_params = substream;
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_hw_free(
    dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
    rollback: c_int,
) {
    let ops = snd_soc_dai_ops(dai);

    if rollback != 0 && (*dai).mark_hw_params != substream {
        return;
    }

    if !ops.is_null() && (*ops).hw_free.is_some() {
        ((*ops).hw_free.unwrap())(substream, dai);
    }

    /* remove marked substream */
    (*dai).mark_hw_params = core::ptr::null_mut();
}

pub unsafe extern "C" fn snd_soc_dai_startup(
    dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let mut ret: c_int = 0;
    let ops = snd_soc_dai_ops(dai);

    if !snd_soc_dai_stream_valid(dai, (*substream).stream) {
        return 0;
    }

    if !ops.is_null() && (*ops).startup.is_some() {
        ret = ((*ops).startup.unwrap())(substream, dai);
    }

    /* mark substream if succeeded */
    if ret == 0 {
        (*dai).mark_startup = substream;
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_shutdown(
    dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
    rollback: c_int,
) {
    let ops = snd_soc_dai_ops(dai);

    if !snd_soc_dai_stream_valid(dai, (*substream).stream) {
        return;
    }

    if rollback != 0 && (*dai).mark_startup != substream {
        return;
    }

    if !ops.is_null() && (*ops).shutdown.is_some() {
        ((*ops).shutdown.unwrap())(substream, dai);
    }

    /* remove marked substream */
    (*dai).mark_startup = core::ptr::null_mut();
}

pub unsafe extern "C" fn snd_soc_dai_compress_new(
    dai: *mut snd_soc_dai,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let mut ret: c_int = -ENOTSUPP;
    let ops = snd_soc_dai_ops(dai);

    if !ops.is_null() && (*ops).compress_new.is_some() {
        ret = ((*ops).compress_new.unwrap())(rtd);
    }

    soc_dai_ret(dai, ret)
}

/*
 * snd_soc_dai_stream_valid() - check if a DAI supports the given stream
 *
 * Returns true if the DAI supports the indicated stream type.
 */
pub unsafe extern "C" fn snd_soc_dai_stream_valid(dai: *const snd_soc_dai, dir: c_int) -> bool {
    let stream = snd_soc_dai_get_pcm_stream(dai, dir);

    /* If the codec specifies any channels at all, it supports the stream */
    (*stream).channels_min != 0
}

pub unsafe extern "C" fn snd_soc_dai_action(dai: *mut snd_soc_dai, stream: c_int, action: c_int) {
    /* see snd_soc_dai_stream_active() */
    (*dai).stream[stream as usize].active += action;

    /* see snd_soc_component_active() */
    (*(*dai).component).active += action;
}

pub unsafe extern "C" fn snd_soc_dai_active(dai: *const snd_soc_dai) -> c_int {
    let mut stream: c_int;
    let mut active: c_int;

    active = 0;
    stream = 0;
    while stream < 2 {
        active += (*dai).stream[stream as usize].active;
        stream += 1;
    }

    active
}

pub unsafe extern "C" fn snd_soc_pcm_dai_probe(
    rtd: *mut snd_soc_pcm_runtime,
    order: c_int,
) -> c_int {
    let mut i: c_int = 0;

    while i < (*rtd).num_dais {
        let dai = *(*rtd).dais.offset(i as isize);
        let ops = snd_soc_dai_ops(dai);

        if (*dai).probed != 0 {
            i += 1;
            continue;
        }

        if !ops.is_null() {
            if (*ops).probe_order != order {
                i += 1;
                continue;
            }

            if (*ops).probe.is_some() {
                let ret = ((*ops).probe.unwrap())(dai);

                if ret < 0 {
                    return soc_dai_ret(dai, ret);
                }
            }
        }
        (*dai).probed = 1;
        i += 1;
    }

    0
}

pub unsafe extern "C" fn snd_soc_pcm_dai_remove(
    rtd: *mut snd_soc_pcm_runtime,
    order: c_int,
) -> c_int {
    let mut ret: c_int = 0;
    let mut i: c_int = 0;

    while i < (*rtd).num_dais {
        let dai = *(*rtd).dais.offset(i as isize);
        let ops = snd_soc_dai_ops(dai);

        if (*dai).probed == 0 {
            i += 1;
            continue;
        }

        if !ops.is_null() {
            if (*ops).remove_order != order {
                i += 1;
                continue;
            }

            if (*ops).remove.is_some() {
                let r = ((*ops).remove.unwrap())(dai);
                if r < 0 {
                    ret = r; /* use last error */
                }
            }
        }
        (*dai).probed = 0;
        i += 1;
    }

    ret
}

pub unsafe extern "C" fn snd_soc_pcm_dai_new(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let mut i: c_int = 0;

    while i < (*rtd).num_dais {
        let dai = *(*rtd).dais.offset(i as isize);
        let ops = snd_soc_dai_ops(dai);

        if !ops.is_null() && (*ops).pcm_new.is_some() {
            let ret = ((*ops).pcm_new.unwrap())(rtd, dai);
            if ret < 0 {
                return soc_dai_ret(dai, ret);
            }
        }

        i += 1;
    }

    0
}

pub unsafe extern "C" fn snd_soc_pcm_dai_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut i: c_int = 0;

    while i < (*rtd).num_dais {
        let dai = *(*rtd).dais.offset(i as isize);
        let ret = snd_soc_dai_prepare(dai, substream);
        if ret < 0 {
            return ret;
        }
        i += 1;
    }

    0
}

unsafe fn soc_dai_trigger(
    dai: *mut snd_soc_dai,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let mut ret: c_int = 0;
    let ops = snd_soc_dai_ops(dai);

    if !snd_soc_dai_stream_valid(dai, (*substream).stream) {
        return 0;
    }

    if !ops.is_null() && (*ops).trigger.is_some() {
        ret = ((*ops).trigger.unwrap())(substream, cmd, dai);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_pcm_dai_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    rollback: c_int,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            let mut i: c_int = 0;
            while i < (*rtd).num_dais {
                let dai = *(*rtd).dais.offset(i as isize);
                ret = soc_dai_trigger(dai, substream, cmd);
                if ret < 0 {
                    break;
                }

                if snd_soc_dai_mute_is_ctrled_at_trigger(dai) != 0 {
                    snd_soc_dai_digital_mute(dai, 0, (*substream).stream);
                }

                (*dai).mark_trigger = substream;
                i += 1;
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            let mut i: c_int = 0;
            while i < (*rtd).num_dais {
                let dai = *(*rtd).dais.offset(i as isize);

                if rollback != 0 && (*dai).mark_trigger != substream {
                    i += 1;
                    continue;
                }

                if snd_soc_dai_mute_is_ctrled_at_trigger(dai) != 0 {
                    snd_soc_dai_digital_mute(dai, 1, (*substream).stream);
                }

                let r = soc_dai_trigger(dai, substream, cmd);
                if r < 0 {
                    ret = r; /* use last ret */
                }
                (*dai).mark_trigger = core::ptr::null_mut();
                i += 1;
            }
        }
        _ => {}
    }

    ret
}

pub unsafe extern "C" fn snd_soc_pcm_dai_delay(
    substream: *mut snd_pcm_substream,
    cpu_delay: *mut snd_pcm_sframes_t,
    codec_delay: *mut snd_pcm_sframes_t,
) {
    let rtd = snd_soc_substream_to_rtd(substream);

    /*
     * We're looking for the delay through the full audio path so it needs to
     * be the maximum of the DAIs doing transmit and the maximum of the DAIs
     * doing receive (ie, all CPUs and all CODECs) rather than just the maximum
     * of all DAIs.
     */

    /* for CPU */
    let mut i: c_int = 0;
    while i < (*rtd).num_cpus {
        let dai = *(*rtd).dais.offset(i as isize);
        let ops = snd_soc_dai_ops(dai);
        if !ops.is_null() && (*ops).delay.is_some() {
            let delay = ((*ops).delay.unwrap())(substream, dai);
            if *cpu_delay < delay {
                *cpu_delay = delay;
            }
        }
        i += 1;
    }

    /* for Codec */
    i = 0;
    while i < (*rtd).num_codecs {
        let dai = *(*rtd).dais.offset(((*rtd).num_cpus + i) as isize);
        let ops = snd_soc_dai_ops(dai);
        if !ops.is_null() && (*ops).delay.is_some() {
            let delay = ((*ops).delay.unwrap())(substream, dai);
            if *codec_delay < delay {
                *codec_delay = delay;
            }
        }
        i += 1;
    }
}

pub unsafe extern "C" fn snd_soc_dai_compr_startup(
    dai: *mut snd_soc_dai,
    cstream: *mut snd_compr_stream,
) -> c_int {
    let mut ret: c_int = 0;
    let cops = snd_soc_dai_cops(dai);

    if !cops.is_null() && (*cops).startup.is_some() {
        ret = ((*cops).startup.unwrap())(cstream, dai);
    }

    /* mark cstream if succeeded */
    if ret == 0 {
        (*dai).mark_compr_startup = cstream;
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_compr_shutdown(
    dai: *mut snd_soc_dai,
    cstream: *mut snd_compr_stream,
    rollback: c_int,
) {
    let cops = snd_soc_dai_cops(dai);

    if rollback != 0 && (*dai).mark_compr_startup != cstream {
        return;
    }

    if !cops.is_null() && (*cops).shutdown.is_some() {
        ((*cops).shutdown.unwrap())(cstream, dai);
    }

    /* remove marked cstream */
    (*dai).mark_compr_startup = core::ptr::null_mut();
}

pub unsafe extern "C" fn snd_soc_dai_compr_trigger(
    dai: *mut snd_soc_dai,
    cstream: *mut snd_compr_stream,
    cmd: c_int,
) -> c_int {
    let mut ret: c_int = 0;
    let cops = snd_soc_dai_cops(dai);

    if !cops.is_null() && (*cops).trigger.is_some() {
        ret = ((*cops).trigger.unwrap())(cstream, cmd, dai);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_compr_set_params(
    dai: *mut snd_soc_dai,
    cstream: *mut snd_compr_stream,
    params: *mut snd_compr_params,
) -> c_int {
    let mut ret: c_int = 0;
    let cops = snd_soc_dai_cops(dai);

    if !cops.is_null() && (*cops).set_params.is_some() {
        ret = ((*cops).set_params.unwrap())(cstream, params, dai);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_compr_get_params(
    dai: *mut snd_soc_dai,
    cstream: *mut snd_compr_stream,
    params: *mut snd_codec,
) -> c_int {
    let mut ret: c_int = 0;
    let cops = snd_soc_dai_cops(dai);

    if !cops.is_null() && (*cops).get_params.is_some() {
        ret = ((*cops).get_params.unwrap())(cstream, params, dai);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_compr_ack(
    dai: *mut snd_soc_dai,
    cstream: *mut snd_compr_stream,
    bytes: size_t,
) -> c_int {
    let mut ret: c_int = 0;
    let cops = snd_soc_dai_cops(dai);

    if !cops.is_null() && (*cops).ack.is_some() {
        ret = ((*cops).ack.unwrap())(cstream, bytes, dai);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_compr_pointer(
    dai: *mut snd_soc_dai,
    cstream: *mut snd_compr_stream,
    tstamp: *mut snd_compr_tstamp64,
) -> c_int {
    let mut ret: c_int = 0;
    let cops = snd_soc_dai_cops(dai);

    if !cops.is_null() && (*cops).pointer.is_some() {
        ret = ((*cops).pointer.unwrap())(cstream, tstamp, dai);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_compr_set_metadata(
    dai: *mut snd_soc_dai,
    cstream: *mut snd_compr_stream,
    metadata: *mut snd_compr_metadata,
) -> c_int {
    let mut ret: c_int = 0;
    let cops = snd_soc_dai_cops(dai);

    if !cops.is_null() && (*cops).set_metadata.is_some() {
        ret = ((*cops).set_metadata.unwrap())(cstream, metadata, dai);
    }

    soc_dai_ret(dai, ret)
}

pub unsafe extern "C" fn snd_soc_dai_compr_get_metadata(
    dai: *mut snd_soc_dai,
    cstream: *mut snd_compr_stream,
    metadata: *mut snd_compr_metadata,
) -> c_int {
    let mut ret: c_int = 0;
    let cops = snd_soc_dai_cops(dai);

    if !cops.is_null() && (*cops).get_metadata.is_some() {
        ret = ((*cops).get_metadata.unwrap())(cstream, metadata, dai);
    }

    soc_dai_ret(dai, ret)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
