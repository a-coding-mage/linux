// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

use core::ffi::{c_char, c_int, c_uint, c_void};

// Dependencies from:
// <linux/bitfield.h>
// <linux/clk.h>
// <sound/pcm_params.h>
// <sound/pcm_iec958.h>
// <sound/soc.h>
// <sound/soc-dai.h>
// "aiu.h"

const fn bit(nr: c_uint) -> c_uint {
    1u32 << nr
}

const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

fn field_prep(mask: c_uint, val: c_uint) -> c_uint {
    (val << mask.trailing_zeros()) & mask
}

fn __ffs(word: c_uint) -> c_uint {
    word.trailing_zeros()
}

const AIU_958_MISC_NON_PCM: c_uint = bit(0);
const AIU_958_MISC_MODE_16BITS: c_uint = bit(1);
const AIU_958_MISC_16BITS_ALIGN: c_uint = genmask(6, 5);
const AIU_958_MISC_MODE_32BITS: c_uint = bit(7);
const AIU_958_MISC_U_FROM_STREAM: c_uint = bit(12);
const AIU_958_MISC_FORCE_LR: c_uint = bit(13);
const AIU_958_CTRL_HOLD_EN: c_uint = bit(0);
const AIU_CLK_CTRL_958_DIV_EN: c_uint = bit(1);
const AIU_CLK_CTRL_958_DIV: c_uint = genmask(5, 4);
const AIU_CLK_CTRL_958_DIV_MORE: c_uint = bit(12);

const AIU_CS_WORD_LEN: usize = 4;
const AIU_958_INTERNAL_DIV: c_uint = 2;

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}

#[repr(C)]
pub struct aiu_clk {
    pub clk: *mut clk,
}

#[repr(C)]
pub struct aiu_platform_clks {
    pub clk_num: c_int,
    pub clks: *mut aiu_clk,
}

#[repr(C)]
pub struct aiu {
    pub spdif: aiu_platform_clks,
    pub spdif_mclk: *mut clk,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub trigger: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            cmd: c_int,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub hw_free: Option<
        unsafe extern "C" fn(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int,
    >,
    pub startup: Option<
        unsafe extern "C" fn(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int,
    >,
    pub shutdown: Option<
        unsafe extern "C" fn(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai),
    >,
}

unsafe extern "C" {
    static MCLK: usize;

    static AIU_CLK_CTRL: c_uint;
    static AIU_958_CTRL: c_uint;
    static AIU_958_MISC: c_uint;
    static AIU_958_CHSTAT_L0: c_uint;
    static AIU_958_CHSTAT_R0: c_uint;
    static AIU_958_CHSTAT_L1: c_uint;
    static AIU_958_CHSTAT_R1: c_uint;

    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static EINVAL: c_int;

    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint)
        -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_pcm_create_iec958_consumer_hw_params(
        params: *mut snd_pcm_hw_params,
        cs: *mut u8,
        len: usize,
    ) -> c_int;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_bulk_prepare_enable(num_clks: c_int, clks: *mut aiu_clk) -> c_int;
    fn clk_bulk_disable_unprepare(num_clks: c_int, clks: *mut aiu_clk);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" fn aiu_encoder_spdif_divider_enable(
    component: *mut snd_soc_component,
    enable: bool,
) {
    unsafe {
        snd_soc_component_update_bits(
            component,
            AIU_CLK_CTRL,
            AIU_CLK_CTRL_958_DIV_EN,
            if enable { AIU_CLK_CTRL_958_DIV_EN } else { 0 },
        );
    }
}

unsafe extern "C" fn aiu_encoder_spdif_hold(component: *mut snd_soc_component, enable: bool) {
    unsafe {
        snd_soc_component_update_bits(
            component,
            AIU_958_CTRL,
            AIU_958_CTRL_HOLD_EN,
            if enable { AIU_958_CTRL_HOLD_EN } else { 0 },
        );
    }
}

unsafe extern "C" fn aiu_encoder_spdif_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let component = (*dai).component;

        if cmd == SNDRV_PCM_TRIGGER_START
            || cmd == SNDRV_PCM_TRIGGER_RESUME
            || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
        {
            aiu_encoder_spdif_hold(component, false);
            return 0;
        }

        if cmd == SNDRV_PCM_TRIGGER_STOP
            || cmd == SNDRV_PCM_TRIGGER_SUSPEND
            || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
        {
            aiu_encoder_spdif_hold(component, true);
            return 0;
        }

        -EINVAL
    }
}

unsafe extern "C" fn aiu_encoder_spdif_setup_cs_word(
    component: *mut snd_soc_component,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    unsafe {
        let mut cs = [0u8; AIU_CS_WORD_LEN];
        let mut val: c_uint;
        let ret: c_int;

        ret = snd_pcm_create_iec958_consumer_hw_params(params, cs.as_mut_ptr(), AIU_CS_WORD_LEN);
        if ret < 0 {
            return ret;
        }

        /* Write the 1st half word */
        val = cs[1] as c_uint | ((cs[0] as c_uint) << 8);
        snd_soc_component_write(component, AIU_958_CHSTAT_L0, val);
        snd_soc_component_write(component, AIU_958_CHSTAT_R0, val);

        /* Write the 2nd half word */
        val = cs[3] as c_uint | ((cs[2] as c_uint) << 8);
        snd_soc_component_write(component, AIU_958_CHSTAT_L1, val);
        snd_soc_component_write(component, AIU_958_CHSTAT_R1, val);

        0
    }
}

unsafe extern "C" fn aiu_encoder_spdif_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let component = (*dai).component;
        let aiu = snd_soc_component_get_drvdata(component) as *mut aiu;
        let mut val: c_uint = 0;
        let mrate: c_uint;
        let mut ret: c_int;

        /* Disable the clock while changing the settings */
        aiu_encoder_spdif_divider_enable(component, false);

        match params_physical_width(params) {
            16 => {
                val |= AIU_958_MISC_MODE_16BITS;
                val |= field_prep(AIU_958_MISC_16BITS_ALIGN, 2);
            }
            32 => {
                val |= AIU_958_MISC_MODE_32BITS;
            }
            _ => {
                dev_err((*dai).dev, c"Unsupported physical width\n".as_ptr());
                return -EINVAL;
            }
        }

        snd_soc_component_update_bits(
            component,
            AIU_958_MISC,
            AIU_958_MISC_NON_PCM
                | AIU_958_MISC_MODE_16BITS
                | AIU_958_MISC_16BITS_ALIGN
                | AIU_958_MISC_MODE_32BITS
                | AIU_958_MISC_FORCE_LR
                | AIU_958_MISC_U_FROM_STREAM,
            val,
        );

        /* Set the stream channel status word */
        ret = aiu_encoder_spdif_setup_cs_word(component, params);
        if ret != 0 {
            dev_err((*dai).dev, c"failed to set channel status word\n".as_ptr());
            return ret;
        }

        snd_soc_component_update_bits(
            component,
            AIU_CLK_CTRL,
            AIU_CLK_CTRL_958_DIV | AIU_CLK_CTRL_958_DIV_MORE,
            field_prep(AIU_CLK_CTRL_958_DIV, __ffs(AIU_958_INTERNAL_DIV)),
        );

        /* 2 * 32bits per subframe * 2 channels = 128 */
        mrate = params_rate(params)
            .wrapping_mul(128)
            .wrapping_mul(AIU_958_INTERNAL_DIV);
        ret = clk_set_rate((*(*aiu).spdif.clks.add(MCLK)).clk, mrate);
        if ret != 0 {
            dev_err((*dai).dev, c"failed to set mclk rate\n".as_ptr());
            return ret;
        }

        aiu_encoder_spdif_divider_enable(component, true);

        0
    }
}

unsafe extern "C" fn aiu_encoder_spdif_hw_free(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let component = (*dai).component;

        aiu_encoder_spdif_divider_enable(component, false);

        0
    }
}

unsafe extern "C" fn aiu_encoder_spdif_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let aiu = snd_soc_component_get_drvdata((*dai).component) as *mut aiu;
        let mut ret: c_int;

        /*
         * NOTE: Make sure the spdif block is on its own divider.
         *
         * The spdif can be clocked by the i2s master clock or its own
         * clock. We should (in theory) change the source depending on the
         * origin of the data.
         *
         * However, considering the clocking scheme used on these platforms,
         * the master clocks will pick the same PLL source when they are
         * playing from the same FIFO. The clock should be in sync so, it
         * should not be necessary to reparent the spdif master clock.
         */
        ret = clk_set_parent((*(*aiu).spdif.clks.add(MCLK)).clk, (*aiu).spdif_mclk);
        if ret != 0 {
            return ret;
        }

        ret = clk_bulk_prepare_enable((*aiu).spdif.clk_num, (*aiu).spdif.clks);
        if ret != 0 {
            dev_err((*dai).dev, c"failed to enable spdif clocks\n".as_ptr());
        }

        ret
    }
}

unsafe extern "C" fn aiu_encoder_spdif_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    unsafe {
        let aiu = snd_soc_component_get_drvdata((*dai).component) as *mut aiu;

        clk_bulk_disable_unprepare((*aiu).spdif.clk_num, (*aiu).spdif.clks);
    }
}

#[no_mangle]
pub static aiu_encoder_spdif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    trigger: Some(aiu_encoder_spdif_trigger),
    hw_params: Some(aiu_encoder_spdif_hw_params),
    hw_free: Some(aiu_encoder_spdif_hw_free),
    startup: Some(aiu_encoder_spdif_startup),
    shutdown: Some(aiu_encoder_spdif_shutdown),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
