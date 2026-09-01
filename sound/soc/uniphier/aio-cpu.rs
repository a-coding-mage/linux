// SPDX-License-Identifier: GPL-2.0
//
// Socionext UniPhier AIO ALSA CPU DAI driver.
//
// Copyright (c) 2016-2018 Socionext Inc.

// Linux kernel headers: linux/clk.h, linux/errno.h, linux/kernel.h,
// linux/mfd/syscon.h, linux/module.h, linux/of.h, linux/of_platform.h,
// linux/platform_device.h, linux/reset.h, sound/core.h, sound/pcm.h,
// sound/pcm_params.h, sound/soc.h
// Module interface: aio.h

use core::ffi::c_int;
use core::ptr::{null, null_mut};

// External type declarations
#[repr(C)]
pub struct uniphier_aio_chip {
    // Implementation details from external module
}

#[repr(C)]
pub struct uniphier_aio {
    // Implementation details from external module
}

#[repr(C)]
pub struct uniphier_aio_sub {
    // Implementation details from external module
}

#[repr(C)]
pub struct uniphier_aio_spec {
    // Implementation details from external module
}

#[repr(C)]
pub struct uniphier_aio_chip_spec {
    // Implementation details from external module
}

#[repr(C)]
pub struct uniphier_aio_pll {
    // Implementation details from external module
}

#[repr(C)]
pub struct snd_soc_dai {
    // Implementation details from external module
}

#[repr(C)]
pub struct snd_pcm_substream {
    // Implementation details from external module
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    // Implementation details from external module
}

#[repr(C)]
pub struct snd_soc_component {
    // Implementation details from external module
}

#[repr(C)]
pub struct snd_kcontrol {
    // Implementation details from external module
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    // Implementation details from external module
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    // Implementation details from external module
}

#[repr(C)]
pub struct snd_kcontrol_new {
    // Implementation details from external module
}

#[repr(C)]
pub struct snd_soc_component_driver {
    // Implementation details from external module
}

#[repr(C)]
pub struct platform_device {
    // Implementation details from external module
}

#[repr(C)]
pub struct device {
    // Implementation details from external module
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, u32, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, u32, u32) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, u32) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub compress_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_dai) -> c_int>,
}

// External function declarations
extern "C" {
    fn uniphier_priv(dai: *const snd_soc_dai) -> *mut uniphier_aio;
    fn dev_err(dev: *const device, fmt: *const i8, ...);
    fn strcmp(s1: *const i8, s2: *const i8) -> c_int;
    fn aio_chip_set_pll(chip: *mut uniphier_aio_chip, pll_id: c_int, freq_out: u32) -> c_int;
    fn aio_init(sub: *mut uniphier_aio_sub) -> c_int;
    fn aio_port_reset(sub: *mut uniphier_aio_sub);
    fn aio_port_set_volume(sub: *mut uniphier_aio_sub, vol: c_int);
    fn aio_src_reset(sub: *mut uniphier_aio_sub);
    fn aio_port_set_param(sub: *mut uniphier_aio_sub, pass_through: c_int, params: *mut snd_pcm_hw_params) -> c_int;
    fn aio_src_set_param(sub: *mut uniphier_aio_sub, params: *mut snd_pcm_hw_params) -> c_int;
    fn aio_port_set_enable(sub: *mut uniphier_aio_sub, enable: c_int);
    fn aio_if_set_param(sub: *mut uniphier_aio_sub, pass_through: c_int) -> c_int;
    fn aio_srcif_set_param(sub: *mut uniphier_aio_sub) -> c_int;
    fn aio_srcch_set_param(sub: *mut uniphier_aio_sub) -> c_int;
    fn aio_srcch_set_enable(sub: *mut uniphier_aio_sub, enable: c_int);
    fn aio_iecout_set_enable(chip: *mut uniphier_aio_chip, enable: bool);
    fn aio_chip_init(chip: *mut uniphier_aio_chip);
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: u32, dir: c_int) -> c_int;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: u32, freq_out: u32) -> c_int;
    fn snd_soc_dai_active(dai: *const snd_soc_dai) -> c_int;
    fn reset_control_assert(rstc: *mut core::ffi::c_void);
    fn clk_disable_unprepare(clk: *mut core::ffi::c_void);
    fn clk_prepare_enable(clk: *mut core::ffi::c_void) -> c_int;
    fn reset_control_deassert(rstc: *mut core::ffi::c_void) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *const snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(comp: *const snd_soc_component) -> *mut core::ffi::c_void;
    fn devm_kzalloc(dev: *const device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn of_device_get_match_data(dev: *const device) -> *const core::ffi::c_void;
    fn syscon_regmap_lookup_by_phandle(np: *const core::ffi::c_void, property: *const i8) -> *mut core::ffi::c_void;
    fn devm_clk_get(dev: *const device, id: *const i8) -> *mut core::ffi::c_void;
    fn devm_reset_control_get_shared(dev: *const device, id: *const i8) -> *mut core::ffi::c_void;
    fn devm_kcalloc(dev: *const device, n: usize, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_kmemdup_array(dev: *const device, src: *const core::ffi::c_void, n: usize, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn spin_lock_init(lock: *mut core::ffi::c_void);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn devm_snd_soc_register_component(dev: *const device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *const core::ffi::c_void, num_dai: c_int) -> c_int;
    fn uniphier_aiodma_soc_register_platform(pdev: *mut platform_device) -> c_int;
    fn platform_get_drvdata(pdev: *const platform_device) -> *mut core::ffi::c_void;
    fn params_rate(p: *const snd_pcm_hw_params) -> c_int;
    fn snd_soc_new_compress(component: *mut snd_soc_component, dai: *mut snd_soc_dai) -> c_int;
    fn for_each_component_dais(component: *mut snd_soc_component, dai: *mut *mut snd_soc_dai);
}

// Constants from Linux kernel
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const PORT_DIR_OUTPUT: c_int = 0;
const PORT_DIR_INPUT: c_int = 1;
const PORT_TYPE_CONV: c_int = 1;
const SND_SOC_DAIFMT_FORMAT_MASK: u32 = 0x0f;
const SND_SOC_DAIFMT_LEFT_J: u32 = 0x00;
const SND_SOC_DAIFMT_RIGHT_J: u32 = 0x01;
const SND_SOC_DAIFMT_I2S: u32 = 0x02;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: u32 = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: u32 = 0x00000003;
const SNDRV_CTL_ELEM_TYPE_INTEGER: u32 = 1;

// DAI clock IDs
const AUD_CLK_IO: c_int = 0;
const AUD_CLK_A1: c_int = 1;
const AUD_CLK_F1: c_int = 2;
const AUD_CLK_A2: c_int = 3;
const AUD_CLK_F2: c_int = 4;
const AUD_CLK_A: c_int = 5;
const AUD_CLK_F: c_int = 6;
const AUD_CLK_APLL: c_int = 7;
const AUD_CLK_RX0: c_int = 8;
const AUD_CLK_USB0: c_int = 9;
const AUD_CLK_HSC0: c_int = 10;

// PLL IDs
const AUD_PLL_A1: c_int = 0;
const AUD_PLL_F1: c_int = 1;
const AUD_PLL_A2: c_int = 2;
const AUD_PLL_F2: c_int = 3;
const AUD_PLL_APLL: c_int = 4;
const AUD_PLL_RX0: c_int = 5;
const AUD_PLL_USB0: c_int = 6;
const AUD_PLL_HSC0: c_int = 7;

// Hardware port IDs
const AUD_HW_HPCMOUT1: c_int = 0;
const AUD_HW_PCMOUT1: c_int = 1;
const AUD_HW_PCMOUT2: c_int = 2;
const AUD_HW_PCMOUT3: c_int = 3;
const AUD_HW_HIECOUT1: c_int = 4;
const AUD_HW_IECOUT1: c_int = 5;

// Volume constants
const AUD_VOL_INIT: c_int = 0;
const AUD_VOL_MAX: c_int = 0x7fff;

// Error codes
const EINVAL: c_int = 22;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = -517;
const GFP_KERNEL: u32 = 0;

fn is_valid_pll(chip: *mut uniphier_aio_chip, pll_id: c_int) -> bool {
    unsafe {
        if pll_id < 0 || (*chip).num_plls <= pll_id {
            let dev = &(*(*chip).pdev).dev;
            dev_err(dev, "PLL(%d) is not supported\n" as *const i8, pll_id);
            return false;
        }

        (*(*chip).plls.add(pll_id as usize)).enable != 0
    }
}

// find_volume - find volume supported HW port by HW port number
// @chip: the AIO chip pointer
// @oport_hw: HW port number, one of AUD_HW_XXXX
//
// Find AIO device from device list by HW port number. Volume feature is
// available only in Output and PCM ports, this limitation comes from HW
// specifications.
//
// Return: The pointer of AIO substream if successful, otherwise NULL on error.
fn find_volume(chip: *mut uniphier_aio_chip, oport_hw: c_int) -> *mut uniphier_aio_sub {
    unsafe {
        for i in 0..(*chip).num_aios as usize {
            let sub = &mut (*(*chip).aios.add(i)).sub[0];

            if sub.swm.is_null() {
                continue;
            }

            if (*sub.swm).oport.hw == oport_hw {
                return sub;
            }
        }

        null_mut()
    }
}

fn match_spec(spec: *const uniphier_aio_spec, name: *const i8, dir: c_int) -> bool {
    unsafe {
        if dir == SNDRV_PCM_STREAM_PLAYBACK && (*spec).swm.dir != PORT_DIR_OUTPUT {
            return false;
        }

        if dir == SNDRV_PCM_STREAM_CAPTURE && (*spec).swm.dir != PORT_DIR_INPUT {
            return false;
        }

        if !(*spec).name.is_null() && strcmp((*spec).name, name) == 0 {
            return true;
        }

        if !(*spec).gname.is_null() && strcmp((*spec).gname, name) == 0 {
            return true;
        }

        false
    }
}

// find_spec - find HW specification info by name
// @aio: the AIO device pointer
// @name: name of device
// @direction: the direction of substream, SNDRV_PCM_STREAM_*
//
// Find hardware specification information from list by device name. This
// information is used for telling the difference of SoCs to driver.
//
// Specification list is array of 'struct uniphier_aio_spec' which is defined
// in each drivers (see: aio-i2s.c).
//
// Return: The pointer of hardware specification of AIO if successful,
// otherwise NULL on error.
fn find_spec(aio: *const uniphier_aio, name: *const i8, direction: c_int) -> *const uniphier_aio_spec {
    unsafe {
        let chip_spec = (*aio).chip as *const uniphier_aio_chip_spec;

        for i in 0..(*chip_spec).num_specs as usize {
            let spec = &(*chip_spec).specs.add(i);

            if match_spec(spec, name, direction) {
                return spec;
            }
        }

        null()
    }
}

// find_divider - find clock divider by frequency
// @aio: the AIO device pointer
// @pll_id: PLL ID, should be AUD_PLL_XX
// @freq: required frequency
//
// Find suitable clock divider by frequency.
//
// Return: The ID of PLL if successful, otherwise negative error value.
fn find_divider(aio: *mut uniphier_aio, pll_id: c_int, freq: u32) -> c_int {
    const MUL: [i32; 4] = [1, 1, 1, 2];
    const DIV: [i32; 4] = [2, 3, 1, 3];

    if !is_valid_pll(unsafe { (*aio).chip }, pll_id) {
        return -EINVAL;
    }

    unsafe {
        let pll = &(*(*aio).chip).plls.add(pll_id as usize);
        for i in 0..MUL.len() {
            if (*pll).freq * MUL[i] as u32 / DIV[i] as u32 == freq {
                return i as c_int;
            }
        }
    }

    -ENOTSUPP
}

unsafe extern "C" fn uniphier_aio_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int,
                                             freq: u32, dir: c_int) -> c_int {
    let aio = uniphier_priv(dai);
    let dev = &(*(*aio).chip).pdev.as_ref().unwrap().dev;
    let mut pll_auto = false;
    let mut pll_id: c_int;
    let mut div_id: c_int;

    pll_id = match clk_id {
        AUD_CLK_IO => return -ENOTSUPP,
        AUD_CLK_A1 => AUD_PLL_A1,
        AUD_CLK_F1 => AUD_PLL_F1,
        AUD_CLK_A2 => AUD_PLL_A2,
        AUD_CLK_F2 => AUD_PLL_F2,
        AUD_CLK_A => {
            pll_auto = true;
            AUD_PLL_A1
        }
        AUD_CLK_F => {
            pll_auto = true;
            AUD_PLL_F1
        }
        AUD_CLK_APLL => AUD_PLL_APLL,
        AUD_CLK_RX0 => AUD_PLL_RX0,
        AUD_CLK_USB0 => AUD_PLL_USB0,
        AUD_CLK_HSC0 => AUD_PLL_HSC0,
        _ => {
            dev_err(dev, "Sysclk(%d) is not supported\n" as *const i8, clk_id);
            return -EINVAL;
        }
    };

    if pll_auto {
        pll_id = 0;
        while pll_id < (*(*aio).chip).num_plls {
            div_id = find_divider(aio, pll_id, freq);
            if div_id >= 0 {
                (*aio).plldiv = div_id;
                break;
            }
            pll_id += 1;
        }
        if pll_id == (*(*aio).chip).num_plls {
            dev_err(dev, "Sysclk frequency is not supported(%d)\n" as *const i8, freq);
            return -EINVAL;
        }
    }

    if dir == SND_SOC_CLOCK_OUT {
        (*aio).pll_out = pll_id;
    } else {
        (*aio).pll_in = pll_id;
    }

    0
}

unsafe extern "C" fn uniphier_aio_set_pll(dai: *mut snd_soc_dai, pll_id: c_int,
                                          source: c_int, freq_in: u32,
                                          freq_out: u32) -> c_int {
    let aio = uniphier_priv(dai);

    if !is_valid_pll((*aio).chip, pll_id) {
        return -EINVAL;
    }

    let ret = aio_chip_set_pll((*aio).chip, pll_id, freq_out);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn uniphier_aio_set_fmt(dai: *mut snd_soc_dai, fmt: u32) -> c_int {
    let aio = uniphier_priv(dai);
    let dev = &(*(*aio).chip).pdev.as_ref().unwrap().dev;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_I2S => {
            (*aio).fmt = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
        }
        _ => {
            dev_err(dev, "Format is not supported(%d)\n" as *const i8, fmt & SND_SOC_DAIFMT_FORMAT_MASK);
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn uniphier_aio_startup(substream: *mut snd_pcm_substream,
                                          dai: *mut snd_soc_dai) -> c_int {
    let aio = uniphier_priv(dai);
    let sub = &mut (*aio).sub[(*substream).stream as usize];

    (*sub).substream = substream;
    (*sub).pass_through = 0;
    (*sub).use_mmap = true;

    aio_init(sub)
}

unsafe extern "C" fn uniphier_aio_shutdown(substream: *mut snd_pcm_substream,
                                           dai: *mut snd_soc_dai) {
    let aio = uniphier_priv(dai);
    let sub = &mut (*aio).sub[(*substream).stream as usize];

    (*sub).substream = null_mut();
}

unsafe extern "C" fn uniphier_aio_hw_params(substream: *mut snd_pcm_substream,
                                            params: *mut snd_pcm_hw_params,
                                            dai: *mut snd_soc_dai) -> c_int {
    let aio = uniphier_priv(dai);
    let sub = &mut (*aio).sub[(*substream).stream as usize];
    let dev = &(*(*aio).chip).pdev.as_ref().unwrap().dev;
    let mut freq: u32;

    freq = match params_rate(params) {
        48000 | 32000 | 24000 => 12288000,
        44100 | 22050 => 11289600,
        _ => {
            dev_err(dev, "Rate is not supported(%d)\n" as *const i8, params_rate(params));
            return -EINVAL;
        }
    };

    let ret = snd_soc_dai_set_sysclk(dai, AUD_CLK_A, freq, SND_SOC_CLOCK_OUT);
    if ret != 0 {
        return ret;
    }

    (*sub).params = *params;
    (*sub).setting = 1;

    aio_port_reset(sub);
    aio_port_set_volume(sub, (*sub).vol);
    aio_src_reset(sub);

    0
}

unsafe extern "C" fn uniphier_aio_hw_free(substream: *mut snd_pcm_substream,
                                          dai: *mut snd_soc_dai) -> c_int {
    let aio = uniphier_priv(dai);
    let sub = &mut (*aio).sub[(*substream).stream as usize];

    (*sub).setting = 0;

    0
}

unsafe extern "C" fn uniphier_aio_prepare(substream: *mut snd_pcm_substream,
                                          dai: *mut snd_soc_dai) -> c_int {
    let aio = uniphier_priv(dai);
    let sub = &mut (*aio).sub[(*substream).stream as usize];
    let mut ret: c_int;

    ret = aio_port_set_param(sub, (*sub).pass_through, &(*sub).params);
    if ret != 0 {
        return ret;
    }
    ret = aio_src_set_param(sub, &(*sub).params);
    if ret != 0 {
        return ret;
    }
    aio_port_set_enable(sub, 1);

    ret = aio_if_set_param(sub, (*sub).pass_through);
    if ret != 0 {
        return ret;
    }

    if (*(*sub).swm).port_type == PORT_TYPE_CONV {
        ret = aio_srcif_set_param(sub);
        if ret != 0 {
            return ret;
        }
        ret = aio_srcch_set_param(sub);
        if ret != 0 {
            return ret;
        }
        aio_srcch_set_enable(sub, 1);
    }

    0
}

unsafe extern "C" fn uniphier_aio_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let aio = uniphier_priv(dai);

    for i in 0..2 {
        let sub = &mut (*aio).sub[i];
        let spec = find_spec(aio, (*dai).name, i as c_int);
        if spec.is_null() {
            continue;
        }

        (*sub).swm = (*spec).swm as *mut _;
        (*sub).spec = spec as *mut _;

        (*sub).vol = AUD_VOL_INIT;
    }

    aio_iecout_set_enable((*aio).chip, true);
    aio_chip_init((*aio).chip);
    (*(*aio).chip).active = 1;

    0
}

unsafe extern "C" fn uniphier_aio_dai_remove(dai: *mut snd_soc_dai) -> c_int {
    let aio = uniphier_priv(dai);

    (*(*aio).chip).active = 0;

    0
}

unsafe extern "C" fn uniphier_aio_ld11_probe(dai: *mut snd_soc_dai) -> c_int {
    let mut ret: c_int;

    ret = uniphier_aio_dai_probe(dai);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_dai_set_pll(dai, AUD_PLL_A1, 0, 0, 36864000);
    if ret < 0 {
        return ret;
    }
    ret = snd_soc_dai_set_pll(dai, AUD_PLL_F1, 0, 0, 36864000);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_dai_set_pll(dai, AUD_PLL_A2, 0, 0, 33868800);
    if ret < 0 {
        return ret;
    }
    ret = snd_soc_dai_set_pll(dai, AUD_PLL_F2, 0, 0, 33868800);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn uniphier_aio_pxs2_probe(dai: *mut snd_soc_dai) -> c_int {
    let mut ret: c_int;

    ret = uniphier_aio_dai_probe(dai);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_dai_set_pll(dai, AUD_PLL_A1, 0, 0, 36864000);
    if ret < 0 {
        return ret;
    }
    ret = snd_soc_dai_set_pll(dai, AUD_PLL_F1, 0, 0, 36864000);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_dai_set_pll(dai, AUD_PLL_A2, 0, 0, 33868800);
    if ret < 0 {
        return ret;
    }
    ret = snd_soc_dai_set_pll(dai, AUD_PLL_F2, 0, 0, 33868800);
    if ret < 0 {
        return ret;
    }

    0
}

pub const UNIPHIER_AIO_I2S_LD11_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(uniphier_aio_ld11_probe),
    remove: Some(uniphier_aio_dai_remove),
    set_sysclk: Some(uniphier_aio_set_sysclk),
    set_pll: Some(uniphier_aio_set_pll),
    set_fmt: Some(uniphier_aio_set_fmt),
    startup: Some(uniphier_aio_startup),
    shutdown: Some(uniphier_aio_shutdown),
    hw_params: Some(uniphier_aio_hw_params),
    hw_free: Some(uniphier_aio_hw_free),
    prepare: Some(uniphier_aio_prepare),
    compress_new: None,
};
// EXPORT_SYMBOL_GPL(uniphier_aio_i2s_ld11_ops)

pub const UNIPHIER_AIO_SPDIF_LD11_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(uniphier_aio_ld11_probe),
    remove: Some(uniphier_aio_dai_remove),
    set_sysclk: Some(uniphier_aio_set_sysclk),
    set_pll: Some(uniphier_aio_set_pll),
    set_fmt: None,
    startup: Some(uniphier_aio_startup),
    shutdown: Some(uniphier_aio_shutdown),
    hw_params: Some(uniphier_aio_hw_params),
    hw_free: Some(uniphier_aio_hw_free),
    prepare: Some(uniphier_aio_prepare),
    compress_new: None,
};
// EXPORT_SYMBOL_GPL(uniphier_aio_spdif_ld11_ops)

pub const UNIPHIER_AIO_SPDIF_LD11_OPS2: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(uniphier_aio_ld11_probe),
    remove: Some(uniphier_aio_dai_remove),
    set_sysclk: Some(uniphier_aio_set_sysclk),
    set_pll: Some(uniphier_aio_set_pll),
    set_fmt: None,
    startup: Some(uniphier_aio_startup),
    shutdown: Some(uniphier_aio_shutdown),
    hw_params: Some(uniphier_aio_hw_params),
    hw_free: Some(uniphier_aio_hw_free),
    prepare: Some(uniphier_aio_prepare),
    compress_new: Some(snd_soc_new_compress),
};
// EXPORT_SYMBOL_GPL(uniphier_aio_spdif_ld11_ops2)

pub const UNIPHIER_AIO_I2S_PXS2_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(uniphier_aio_pxs2_probe),
    remove: Some(uniphier_aio_dai_remove),
    set_sysclk: Some(uniphier_aio_set_sysclk),
    set_pll: Some(uniphier_aio_set_pll),
    set_fmt: Some(uniphier_aio_set_fmt),
    startup: Some(uniphier_aio_startup),
    shutdown: Some(uniphier_aio_shutdown),
    hw_params: Some(uniphier_aio_hw_params),
    hw_free: Some(uniphier_aio_hw_free),
    prepare: Some(uniphier_aio_prepare),
    compress_new: None,
};
// EXPORT_SYMBOL_GPL(uniphier_aio_i2s_pxs2_ops)

pub const UNIPHIER_AIO_SPDIF_PXS2_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(uniphier_aio_pxs2_probe),
    remove: Some(uniphier_aio_dai_remove),
    set_sysclk: Some(uniphier_aio_set_sysclk),
    set_pll: Some(uniphier_aio_set_pll),
    set_fmt: None,
    startup: Some(uniphier_aio_startup),
    shutdown: Some(uniphier_aio_shutdown),
    hw_params: Some(uniphier_aio_hw_params),
    hw_free: Some(uniphier_aio_hw_free),
    prepare: Some(uniphier_aio_prepare),
    compress_new: None,
};
// EXPORT_SYMBOL_GPL(uniphier_aio_spdif_pxs2_ops)

pub const UNIPHIER_AIO_SPDIF_PXS2_OPS2: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(uniphier_aio_pxs2_probe),
    remove: Some(uniphier_aio_dai_remove),
    set_sysclk: Some(uniphier_aio_set_sysclk),
    set_pll: Some(uniphier_aio_set_pll),
    set_fmt: None,
    startup: Some(uniphier_aio_startup),
    shutdown: Some(uniphier_aio_shutdown),
    hw_params: Some(uniphier_aio_hw_params),
    hw_free: Some(uniphier_aio_hw_free),
    prepare: Some(uniphier_aio_prepare),
    compress_new: Some(snd_soc_new_compress),
};
// EXPORT_SYMBOL_GPL(uniphier_aio_spdif_pxs2_ops2)

unsafe fn uniphier_aio_dai_suspend(dai: *mut snd_soc_dai) {
    let aio = uniphier_priv(dai);

    if snd_soc_dai_active(dai) == 0 {
        return;
    }

    (*(*aio).chip).num_wup_aios -= 1;
    if (*(*aio).chip).num_wup_aios == 0 {
        reset_control_assert((*(*aio).chip).rst as *mut _);
        clk_disable_unprepare((*(*aio).chip).clk as *mut _);
    }
}

unsafe extern "C" fn uniphier_aio_suspend(component: *mut snd_soc_component) -> c_int {
    let mut dai: *mut snd_soc_dai = null_mut();
    for_each_component_dais(component, &mut dai);
    if !dai.is_null() {
        uniphier_aio_dai_suspend(dai);
    }
    0
}

unsafe extern "C" fn uniphier_aio_dai_resume(dai: *mut snd_soc_dai) -> c_int {
    let aio = uniphier_priv(dai);
    let mut ret: c_int;
    let mut i: c_int;

    if snd_soc_dai_active(dai) == 0 {
        return 0;
    }

    if (*(*aio).chip).active == 0 {
        return 0;
    }

    if (*(*aio).chip).num_wup_aios == 0 {
        ret = clk_prepare_enable((*(*aio).chip).clk as *mut _);
        if ret != 0 {
            return ret;
        }

        ret = reset_control_deassert((*(*aio).chip).rst as *mut _);
        if ret != 0 {
            // goto err_out_clock
            clk_disable_unprepare((*(*aio).chip).clk as *mut _);
            return ret;
        }
    }

    aio_iecout_set_enable((*aio).chip, true);
    aio_chip_init((*aio).chip);

    i = 0;
    while i < 2 {
        let sub = &mut (*aio).sub[i as usize];

        if sub.spec.is_null() || sub.substream.is_null() {
            i += 1;
            continue;
        }

        ret = aio_init(sub);
        if ret != 0 {
            // goto err_out_reset
            if (*(*aio).chip).num_wup_aios == 0 {
                reset_control_assert((*(*aio).chip).rst as *mut _);
            }
            if (*(*aio).chip).num_wup_aios == 0 {
                clk_disable_unprepare((*(*aio).chip).clk as *mut _);
            }
            return ret;
        }

        if (*sub).setting == 0 {
            i += 1;
            continue;
        }

        aio_port_reset(sub);
        aio_src_reset(sub);
        i += 1;
    }
    (*(*aio).chip).num_wup_aios += 1;

    0
}

unsafe extern "C" fn uniphier_aio_resume(component: *mut snd_soc_component) -> c_int {
    let mut dai: *mut snd_soc_dai = null_mut();
    let mut ret: c_int = 0;

    for_each_component_dais(component, &mut dai);
    if !dai.is_null() {
        ret |= uniphier_aio_dai_resume(dai);
    }
    ret
}

unsafe extern "C" fn uniphier_aio_vol_info(kcontrol: *const snd_kcontrol,
                                           uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).info_type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).info_count = 1;
    (*uinfo).info_value_integer_min = 0;
    (*uinfo).info_value_integer_max = AUD_VOL_MAX;

    0
}

unsafe extern "C" fn uniphier_aio_vol_get(kcontrol: *const snd_kcontrol,
                                          ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let chip = snd_soc_component_get_drvdata(comp) as *mut uniphier_aio_chip;
    let sub: *mut uniphier_aio_sub;
    let oport_hw = (*kcontrol).private_value as c_int;

    sub = find_volume(chip, oport_hw);
    if sub.is_null() {
        return 0;
    }

    (*ucontrol).ucontrol_value_integer_value[0] = (*sub).vol;

    0
}

unsafe extern "C" fn uniphier_aio_vol_put(kcontrol: *const snd_kcontrol,
                                          ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let chip = snd_soc_component_get_drvdata(comp) as *mut uniphier_aio_chip;
    let sub: *mut uniphier_aio_sub;
    let oport_hw = (*kcontrol).private_value as c_int;

    sub = find_volume(chip, oport_hw);
    if sub.is_null() {
        return 0;
    }

    if (*sub).vol == (*ucontrol).ucontrol_value_integer_value[0] {
        return 0;
    }
    (*sub).vol = (*ucontrol).ucontrol_value_integer_value[0];

    aio_port_set_volume(sub, (*sub).vol);

    0
}

pub const UNIPHIER_AIO_CONTROLS: [snd_kcontrol_new; 6] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        name: "HPCMOUT1 Volume" as *const i8,
        info: Some(uniphier_aio_vol_info),
        get: Some(uniphier_aio_vol_get),
        put: Some(uniphier_aio_vol_put),
        private_value: AUD_HW_HPCMOUT1 as usize,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        name: "PCMOUT1 Volume" as *const i8,
        info: Some(uniphier_aio_vol_info),
        get: Some(uniphier_aio_vol_get),
        put: Some(uniphier_aio_vol_put),
        private_value: AUD_HW_PCMOUT1 as usize,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        name: "PCMOUT2 Volume" as *const i8,
        info: Some(uniphier_aio_vol_info),
        get: Some(uniphier_aio_vol_get),
        put: Some(uniphier_aio_vol_put),
        private_value: AUD_HW_PCMOUT2 as usize,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        name: "PCMOUT3 Volume" as *const i8,
        info: Some(uniphier_aio_vol_info),
        get: Some(uniphier_aio_vol_get),
        put: Some(uniphier_aio_vol_put),
        private_value: AUD_HW_PCMOUT3 as usize,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        name: "HIECOUT1 Volume" as *const i8,
        info: Some(uniphier_aio_vol_info),
        get: Some(uniphier_aio_vol_get),
        put: Some(uniphier_aio_vol_put),
        private_value: AUD_HW_HIECOUT1 as usize,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
        name: "IECOUT1 Volume" as *const i8,
        info: Some(uniphier_aio_vol_info),
        get: Some(uniphier_aio_vol_get),
        put: Some(uniphier_aio_vol_put),
        private_value: AUD_HW_IECOUT1 as usize,
    },
];

pub const UNIPHIER_AIO_COMPONENT: snd_soc_component_driver = snd_soc_component_driver {
    name: "uniphier-aio" as *const i8,
    controls: UNIPHIER_AIO_CONTROLS.as_ptr(),
    num_controls: 6,
    suspend: Some(uniphier_aio_suspend),
    resume: Some(uniphier_aio_resume),
};

pub unsafe extern "C" fn uniphier_aio_probe(pdev: *mut platform_device) -> c_int {
    let dev = &(*pdev).dev;
    let chip: *mut uniphier_aio_chip;
    let mut ret: c_int;
    let mut i: c_int;
    let mut j: c_int;

    chip = devm_kzalloc(dev, core::mem::size_of::<uniphier_aio_chip>(), GFP_KERNEL) as *mut uniphier_aio_chip;
    if chip.is_null() {
        return -ENOMEM;
    }

    (*chip).chip_spec = of_device_get_match_data(dev) as *const uniphier_aio_chip_spec;
    if (*chip).chip_spec.is_null() {
        return -EINVAL;
    }

    (*chip).regmap_sg = syscon_regmap_lookup_by_phandle((*pdev).dev.of_node, "socionext,syscon" as *const i8);
    if ((*chip).regmap_sg as isize) < 0 {
        let err = ((*chip).regmap_sg as isize) as c_int;
        if err == -EPROBE_DEFER {
            return -EPROBE_DEFER;
        }
        (*chip).regmap_sg = null_mut();
    }

    (*chip).clk = devm_clk_get(dev, "aio" as *const i8);
    if ((*chip).clk as isize) < 0 {
        return ((*chip).clk as isize) as c_int;
    }

    (*chip).rst = devm_reset_control_get_shared(dev, "aio" as *const i8);
    if ((*chip).rst as isize) < 0 {
        return ((*chip).rst as isize) as c_int;
    }

    (*chip).num_aios = (*(*chip).chip_spec).num_dais;
    (*chip).num_wup_aios = (*chip).num_aios;
    (*chip).aios = devm_kcalloc(dev,
                                (*chip).num_aios as usize, core::mem::size_of::<uniphier_aio>(),
                                GFP_KERNEL) as *mut uniphier_aio;
    if (*chip).aios.is_null() {
        return -ENOMEM;
    }

    (*chip).num_plls = (*(*chip).chip_spec).num_plls;
    (*chip).plls = devm_kmemdup_array(dev, (*(*chip).chip_spec).plls as *const core::ffi::c_void, (*chip).num_plls as usize,
                                      core::mem::size_of::<uniphier_aio_pll>(), GFP_KERNEL) as *mut uniphier_aio_pll;
    if (*chip).plls.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*chip).num_aios {
        let aio = &mut (*(*chip).aios.add(i as usize));

        (*aio).chip = chip;
        (*aio).fmt = SND_SOC_DAIFMT_I2S;

        j = 0;
        while j < 2 {
            let sub = &mut (*aio).sub[j as usize];

            (*sub).aio = aio;
            spin_lock_init(&mut (*sub).lock as *mut _);
            j += 1;
        }
        i += 1;
    }

    (*chip).pdev = pdev;
    platform_set_drvdata(pdev, chip as *mut core::ffi::c_void);

    ret = clk_prepare_enable((*chip).clk as *mut _);
    if ret != 0 {
        return ret;
    }

    ret = reset_control_deassert((*chip).rst as *mut _);
    if ret != 0 {
        clk_disable_unprepare((*chip).clk as *mut _);
        return ret;
    }

    ret = devm_snd_soc_register_component(dev, &UNIPHIER_AIO_COMPONENT,
                                          (*(*chip).chip_spec).dais, (*(*chip).chip_spec).num_dais);
    if ret != 0 {
        reset_control_assert((*chip).rst as *mut _);
        clk_disable_unprepare((*chip).clk as *mut _);
        return ret;
    }

    ret = uniphier_aiodma_soc_register_platform(pdev);
    if ret != 0 {
        reset_control_assert((*chip).rst as *mut _);
        clk_disable_unprepare((*chip).clk as *mut _);
        return ret;
    }

    0
}
// EXPORT_SYMBOL_GPL(uniphier_aio_probe)

pub unsafe extern "C" fn uniphier_aio_remove(pdev: *mut platform_device) {
    let chip = platform_get_drvdata(pdev) as *mut uniphier_aio_chip;

    reset_control_assert((*chip).rst as *mut _);
    clk_disable_unprepare((*chip).clk as *mut _);
}
// EXPORT_SYMBOL_GPL(uniphier_aio_remove)

// MODULE_AUTHOR("Katsuhiro Suzuki <suzuki.katsuhiro@socionext.com>")
// MODULE_DESCRIPTION("UniPhier AIO CPU DAI driver.")
// MODULE_LICENSE("GPL v2")

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
