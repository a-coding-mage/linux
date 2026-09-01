// SPDX-License-Identifier: GPL-2.0+
//
// smdk_spdif.c - S/PDIF audio for SMDK
//
// Copyright (C) 2010 Samsung Electronics Co., Ltd.

// C dependencies removed from executable Rust:
// <linux/clk.h>, <linux/module.h>, <sound/soc.h>, and "spdif.h".

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;

const SND_SOC_SPDIF_INT_MCLK: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;

static KERN_WARNING: &[u8] = b"";
static KERN_ERR: &[u8] = b"";

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct clk {
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
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub ops: *const snd_soc_ops,
    // SND_SOC_DAILINK_REG(spdif) expands to additional ASoC link fields.
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;

    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;

    fn IS_ERR(ptr: *const c_void) -> bool;
    fn printk(fmt: *const c_char, ...) -> c_int;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint_compat,
        dir: c_int,
    ) -> c_int;

    fn platform_device_alloc(name: *const c_char, id: c_int) -> *mut platform_device;
    fn platform_device_add(pdev: *mut platform_device) -> c_int;
    fn platform_device_del(pdev: *mut platform_device);
    fn platform_device_put(pdev: *mut platform_device);
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
}

type c_uint_compat = c_ulong;

/* Audio clock settings are belonged to board specific part. Every
 * board can set audio source clock setting which is matched with H/W
 * like this function-'set_audio_clock_heirachy'.
 */
unsafe extern "C" fn set_audio_clock_heirachy(pdev: *mut platform_device) -> c_int {
    let mut fout_epll: *mut clk;
    let mut mout_epll: *mut clk;
    let mut sclk_audio0: *mut clk;
    let mut sclk_spdif: *mut clk;
    let mut ret: c_int = 0;

    fout_epll = clk_get(core::ptr::null_mut(), c"fout_epll".as_ptr());
    if IS_ERR(fout_epll.cast::<c_void>()) {
        printk(
            c"%s: Cannot find fout_epll.\n".as_ptr(),
            c"set_audio_clock_heirachy".as_ptr(),
        );
        return -EINVAL;
    }

    mout_epll = clk_get(core::ptr::null_mut(), c"mout_epll".as_ptr());
    if IS_ERR(mout_epll.cast::<c_void>()) {
        printk(
            c"%s: Cannot find mout_epll.\n".as_ptr(),
            c"set_audio_clock_heirachy".as_ptr(),
        );
        ret = -EINVAL;
        goto_out1(fout_epll);
        return ret;
    }

    sclk_audio0 = clk_get(&mut (*pdev).dev, c"sclk_audio".as_ptr());
    if IS_ERR(sclk_audio0.cast::<c_void>()) {
        printk(
            c"%s: Cannot find sclk_audio.\n".as_ptr(),
            c"set_audio_clock_heirachy".as_ptr(),
        );
        ret = -EINVAL;
        goto_out2(mout_epll, fout_epll);
        return ret;
    }

    sclk_spdif = clk_get(core::ptr::null_mut(), c"sclk_spdif".as_ptr());
    if IS_ERR(sclk_spdif.cast::<c_void>()) {
        printk(
            c"%s: Cannot find sclk_spdif.\n".as_ptr(),
            c"set_audio_clock_heirachy".as_ptr(),
        );
        ret = -EINVAL;
        goto_out3(sclk_audio0, mout_epll, fout_epll);
        return ret;
    }

    /* Set audio clock hierarchy for S/PDIF */
    clk_set_parent(mout_epll, fout_epll);
    clk_set_parent(sclk_audio0, mout_epll);
    clk_set_parent(sclk_spdif, sclk_audio0);

    clk_put(sclk_spdif);
    goto_out3(sclk_audio0, mout_epll, fout_epll);

    ret
}

unsafe fn goto_out3(sclk_audio0: *mut clk, mout_epll: *mut clk, fout_epll: *mut clk) {
    clk_put(sclk_audio0);
    goto_out2(mout_epll, fout_epll);
}

unsafe fn goto_out2(mout_epll: *mut clk, fout_epll: *mut clk) {
    clk_put(mout_epll);
    goto_out1(fout_epll);
}

unsafe fn goto_out1(fout_epll: *mut clk) {
    clk_put(fout_epll);
}

/* We should haved to set clock directly on this part because of clock
 * scheme of Samsudng SoCs did not support to set rates from abstrct
 * clock of it's hierarchy.
 */
unsafe extern "C" fn set_audio_clock_rate(epll_rate: c_ulong, audio_rate: c_ulong) -> c_int {
    let mut fout_epll: *mut clk;
    let mut sclk_spdif: *mut clk;

    fout_epll = clk_get(core::ptr::null_mut(), c"fout_epll".as_ptr());
    if IS_ERR(fout_epll.cast::<c_void>()) {
        printk(
            c"%s: failed to get fout_epll\n".as_ptr(),
            c"set_audio_clock_rate".as_ptr(),
        );
        return -ENOENT;
    }

    clk_set_rate(fout_epll, epll_rate);
    clk_put(fout_epll);

    sclk_spdif = clk_get(core::ptr::null_mut(), c"sclk_spdif".as_ptr());
    if IS_ERR(sclk_spdif.cast::<c_void>()) {
        printk(
            c"%s: failed to get sclk_spdif\n".as_ptr(),
            c"set_audio_clock_rate".as_ptr(),
        );
        return -ENOENT;
    }

    clk_set_rate(sclk_spdif, audio_rate);
    clk_put(sclk_spdif);

    0
}

unsafe extern "C" fn smdk_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut pll_out: c_ulong;
    let rclk_rate: c_ulong;
    let mut ret: c_int;
    let ratio: c_int;

    match params_rate(params) {
        44100 => {
            pll_out = 45158400;
        }
        32000 | 48000 | 96000 => {
            pll_out = 49152000;
        }
        _ => {
            return -EINVAL;
        }
    }

    /* Setting ratio to 512fs helps to use S/PDIF with HDMI without
     * modify S/PDIF ASoC machine driver.
     */
    ratio = 512;
    rclk_rate = (params_rate(params) * ratio) as c_ulong;

    /* Set audio source clock rates */
    ret = set_audio_clock_rate(pll_out, rclk_rate);
    if ret < 0 {
        return ret;
    }

    /* Set S/PDIF uses internal source clock */
    snd_soc_dai_set_sysclk(
        cpu_dai,
        SND_SOC_SPDIF_INT_MCLK,
        rclk_rate as c_uint_compat,
        SND_SOC_CLOCK_IN,
    )
}

static smdk_spdif_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(smdk_hw_params),
};

// SND_SOC_DAILINK_DEFS(spdif,
//     DAILINK_COMP_ARRAY(COMP_CPU("samsung-spdif")),
//     DAILINK_COMP_ARRAY(COMP_CODEC("spdif-dit", "dit-hifi")),
//     DAILINK_COMP_ARRAY(COMP_PLATFORM("samsung-spdif")));

static mut smdk_dai: snd_soc_dai_link = snd_soc_dai_link {
    name: c"S/PDIF".as_ptr(),
    stream_name: c"S/PDIF PCM Playback".as_ptr(),
    ops: &smdk_spdif_ops,
};

static mut smdk: snd_soc_card = snd_soc_card {
    name: c"SMDK-S/PDIF".as_ptr(),
    owner: core::ptr::null_mut(),
    dai_link: core::ptr::addr_of_mut!(smdk_dai),
    num_links: 1,
};

static mut smdk_snd_spdif_dit_device: *mut platform_device = core::ptr::null_mut();
static mut smdk_snd_spdif_device: *mut platform_device = core::ptr::null_mut();

unsafe extern "C" fn smdk_init() -> c_int {
    let mut ret: c_int;

    smdk.owner = THIS_MODULE;

    smdk_snd_spdif_dit_device = platform_device_alloc(c"spdif-dit".as_ptr(), -1);
    if smdk_snd_spdif_dit_device.is_null() {
        return -ENOMEM;
    }

    ret = platform_device_add(smdk_snd_spdif_dit_device);
    if ret != 0 {
        platform_device_put(smdk_snd_spdif_dit_device);
        return ret;
    }

    smdk_snd_spdif_device = platform_device_alloc(c"soc-audio".as_ptr(), -1);
    if smdk_snd_spdif_device.is_null() {
        ret = -ENOMEM;
        platform_device_del(smdk_snd_spdif_dit_device);
        platform_device_put(smdk_snd_spdif_dit_device);
        return ret;
    }

    platform_set_drvdata(
        smdk_snd_spdif_device,
        core::ptr::addr_of_mut!(smdk).cast::<c_void>(),
    );

    ret = platform_device_add(smdk_snd_spdif_device);
    if ret != 0 {
        platform_device_put(smdk_snd_spdif_device);
        platform_device_del(smdk_snd_spdif_dit_device);
        platform_device_put(smdk_snd_spdif_dit_device);
        return ret;
    }

    /* Set audio clock hierarchy manually */
    ret = set_audio_clock_heirachy(smdk_snd_spdif_device);
    if ret != 0 {
        platform_device_del(smdk_snd_spdif_device);
        platform_device_put(smdk_snd_spdif_device);
        platform_device_del(smdk_snd_spdif_dit_device);
        platform_device_put(smdk_snd_spdif_dit_device);
        return ret;
    }

    0
}

unsafe extern "C" fn smdk_exit() {
    platform_device_unregister(smdk_snd_spdif_device);
    platform_device_unregister(smdk_snd_spdif_dit_device);
}

// module_init(smdk_init);
// module_exit(smdk_exit);

// MODULE_AUTHOR("Seungwhan Youn, <sw.youn@samsung.com>");
// MODULE_DESCRIPTION("ALSA SoC SMDK+S/PDIF");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
