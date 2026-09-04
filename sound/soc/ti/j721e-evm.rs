// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 Texas Instruments Incorporated - http://www.ti.com
// Author: Peter Ujfalusi <peter.ujfalusi@ti.com>

// Linux kernel headers (included in original C file):
// #include <linux/cleanup.h>
// #include <linux/clk.h>
// #include <linux/module.h>
// #include <linux/of.h>
// #include <linux/platform_device.h>
// #include <sound/core.h>
// #include <sound/pcm.h>
// #include <sound/pcm_params.h>
// #include <sound/soc.h>
// #include "davinci-mcasp.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr::{self, null_mut};

// Maximum number of configuration entries for prefixes:
// CPB: 2 (mcasp10 + codec)
// IVI: 3 (mcasp0 + 2x codec)
const J721E_CODEC_CONF_COUNT: usize = 5;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
enum J721eAudioDomainId {
    J721eAudioDomainCpb = 0,
    J721eAudioDomainIvi = 1,
    J721eAudioDomainLast = 2,
}

const J721E_CLK_PARENT_48000: usize = 0;
const J721E_CLK_PARENT_44100: usize = 1;

const J721E_MAX_CLK_HSDIV: u32 = 128;
const PCM1368A_MAX_SYSCLK: u32 = 36864000;

// DAI format: right-justified, normal clock polarity/phase, codec as master, cpu as master
// J721E_DAI_FMT = (SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC)

#[repr(C)]
#[derive(Clone, Copy, Debug)]
enum J721eBoardType {
    J721eBoardCpb = 1,
    J721eBoardCpbIvi = 2,
}

#[repr(C)]
struct J721eAudioMatchData {
    board_type: J721eBoardType,
    num_links: c_int,
    pll_rates: [u32; 2],
}

static RATIOS_FOR_PCM3168A: [u32; 3] = [256, 512, 768];

#[repr(C)]
struct J721eAudioClocks {
    target: *mut c_void,
    parent: [*mut c_void; 2],
}

#[repr(C)]
struct J721eAudioDomain {
    codec: J721eAudioClocks,
    mcasp: J721eAudioClocks,
    parent_clk_id: c_int,
    active: c_int,
    active_link: c_uint,
    rate: c_uint,
}

#[repr(C)]
struct J721ePriv {
    dev: *mut c_void,
    card: c_void,
    codec_conf: [c_void; J721E_CODEC_CONF_COUNT],
    rate_range: c_void,
    match_data: *const J721eAudioMatchData,
    pll_rates: [u32; 2],
    hsdiv_rates: [u32; 2],
    audio_domains: [J721eAudioDomain; 2],
    mutex: c_void,
    dai_links: [c_void; 0],
}

extern "C" {
    // DAPM widget and route structures from sound/soc.h
    static J721E_CPB_DAPM_WIDGETS: [c_void; 7];
    static J721E_CPB_DAPM_ROUTES: [c_void; 14];
    static J721E_IVI_CODEC_A_DAPM_WIDGETS: [c_void; 7];
    static J721E_CODEC_A_DAPM_ROUTES: [c_void; 12];
    static J721E_IVI_CODEC_B_DAPM_WIDGETS: [c_void; 7];
    static J721E_CODEC_B_DAPM_ROUTES: [c_void; 12];

    // External audio/clock/device functions
    fn snd_soc_substream_to_rtd(substream: *mut c_void) -> *mut c_void;
    fn snd_soc_card_get_drvdata(card: *mut c_void) -> *mut c_void;
    fn snd_soc_rtd_to_cpu(rtd: *mut c_void, idx: c_int) -> *mut c_void;
    fn snd_soc_card_to_dapm(card: *mut c_void) -> *mut c_void;

    fn clk_set_parent(clk: *mut c_void, parent: *mut c_void) -> c_int;
    fn clk_set_rate(clk: *mut c_void, rate: c_uint) -> c_int;
    fn clk_get_parent(clk: *mut c_void) -> *mut c_void;
    fn clk_get_rate(clk: *mut c_void) -> c_uint;
    fn clk_put(clk: *mut c_void);

    fn devm_clk_get(dev: *mut c_void, id: *const c_char) -> *mut c_void;
    fn devm_kcalloc(
        dev: *mut c_void,
        n: c_uint,
        size: usize,
        gfp_flags: u32,
    ) -> *mut c_void;
    fn devm_kzalloc(dev: *mut c_void, size: usize, gfp_flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);

    fn kasprintf(gfp_flags: u32, fmt: *const c_char, ...) -> *mut c_char;

    fn snd_interval_refine(t: *mut c_void, constraint: *mut c_void) -> c_int;
    fn snd_pcm_hw_constraint_single(
        runtime: *mut c_void,
        var: c_int,
        val: c_uint,
    ) -> c_int;
    fn snd_pcm_hw_rule_add(
        runtime: *mut c_void,
        flags: c_int,
        var: c_int,
        func: unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int,
        private: *mut c_void,
        dep: c_int,
        ...: i32,
    ) -> c_int;

    fn snd_soc_dai_set_tdm_slot(
        dai: *mut c_void,
        tx_mask: u32,
        rx_mask: u32,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut c_void,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;

    fn snd_soc_dapm_new_controls(
        dapm: *mut c_void,
        widgets: *const c_void,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut c_void,
        routes: *const c_void,
        num: c_int,
    ) -> c_int;

    fn snd_soc_of_parse_card_name(card: *mut c_void, propname: *const c_char) -> c_int;

    fn of_parse_phandle(node: *mut c_void, phandle_name: *const c_char, index: c_int) -> *mut c_void;
    fn of_node_put(node: *mut c_void);

    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut c_void, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);

    fn mutex_init(mutex: *mut c_void);
    fn mutex_lock(mutex: *mut c_void);
    fn mutex_unlock(mutex: *mut c_void);

    fn devm_snd_soc_register_card(dev: *mut c_void, card: *mut c_void) -> c_int;

    fn for_each_rtd_codec_dais(rtd: *mut c_void, i: *mut c_int, codec_dai: *mut *mut c_void);

    fn IS_ERR(ptr: *const c_void) -> bool;
    fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;

    fn snd_interval_any(t: *mut c_void);

    fn of_device_get_match_data(dev: *mut c_void) -> *const c_void;

    static THIS_MODULE: c_void;
}

// Opaque types for ASoC structures
enum HwParamInterval {}
enum SndPcmHwRule {}
enum SndInterval {}
enum DavincimcaspContext {}

const GFP_KERNEL: u32 = 0x10c0;
const ENOTSUPP: c_int = -524;
const EPROBE_DEFER: c_int = -517;
const EINVAL: c_int = -22;
const ENOMEM: c_int = -12;
const ENODEV: c_int = -19;

const MCASP_CLK_HCLK_AUXCLK: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 10;
const SND_SOC_CLOCK_IN: c_int = 0;

unsafe fn j721e_configure_refclk(
    priv: *mut J721ePriv,
    audio_domain: c_uint,
    rate: c_uint,
) -> c_int {
    let domain = &mut (*priv).audio_domains[audio_domain as usize];
    let mut scki: c_uint = 0;
    let mut ret: c_int = -EINVAL;
    let mut i: c_int = 0;
    let mut clk_id: usize = 0;

    if (rate % 8000) == 0 && (*priv).pll_rates[J721E_CLK_PARENT_48000] != 0 {
        clk_id = J721E_CLK_PARENT_48000;
    } else if (rate % 11025) == 0 && (*priv).pll_rates[J721E_CLK_PARENT_44100] != 0 {
        clk_id = J721E_CLK_PARENT_44100;
    } else if (rate % 11025) == 0 && (*priv).pll_rates[J721E_CLK_PARENT_48000] != 0 {
        clk_id = J721E_CLK_PARENT_48000;
    } else {
        return ret;
    }

    while (i as usize) < RATIOS_FOR_PCM3168A.len() {
        scki = RATIOS_FOR_PCM3168A[i as usize].wrapping_mul(rate);

        if (*priv).pll_rates[clk_id] / scki <= J721E_MAX_CLK_HSDIV {
            ret = 0;
            break;
        }
        i += 1;
    }

    if ret != 0 {
        dev_err(
            (*priv).dev,
            c"No valid clock configuration for %u Hz\n".as_ptr(),
            rate,
        );
        return ret;
    }

    if domain.parent_clk_id == -1
        || (*priv).hsdiv_rates[domain.parent_clk_id as usize] != scki
    {
        dev_dbg(
            (*priv).dev,
            c"domain%u configuration for %u Hz: %s, %dxFS (SCKI: %u Hz)\n".as_ptr(),
            audio_domain,
            rate,
            if clk_id == J721E_CLK_PARENT_48000 {
                c"PLL4".as_ptr()
            } else {
                c"PLL15".as_ptr()
            },
            RATIOS_FOR_PCM3168A[i as usize] as c_int,
            scki,
        );

        if domain.parent_clk_id != clk_id as c_int {
            ret = clk_set_parent(domain.codec.target, domain.codec.parent[clk_id]);
            if ret != 0 {
                return ret;
            }

            ret = clk_set_parent(domain.mcasp.target, domain.mcasp.parent[clk_id]);
            if ret != 0 {
                return ret;
            }

            domain.parent_clk_id = clk_id as c_int;
        }

        ret = clk_set_rate(domain.codec.target, scki);
        if ret != 0 {
            dev_err(
                (*priv).dev,
                c"codec set rate failed for %u Hz\n".as_ptr(),
                scki,
            );
            return ret;
        }

        ret = clk_set_rate(domain.mcasp.target, scki);
        if ret == 0 {
            (*priv).hsdiv_rates[domain.parent_clk_id as usize] = scki;
        } else {
            dev_err(
                (*priv).dev,
                c"mcasp set rate failed for %u Hz\n".as_ptr(),
                scki,
            );
            return ret;
        }
    }

    ret
}

unsafe fn j721e_rule_rate(
    params: *mut HwParamInterval,
    rule: *mut SndPcmHwRule,
) -> c_int {
    let t = (*rule).private as *mut SndInterval;
    snd_interval_refine(
        params as *mut c_void,
        t as *mut c_void,
    )
}

unsafe fn j721e_audio_startup(substream: *mut c_void) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let priv = snd_soc_card_get_drvdata((*rtd as *mut DavincimcaspContext as *mut c_void)) as *mut J721ePriv;
    let domain_id = (*rtd as *mut DavincimcaspContext as *mut c_void) as c_uint;
    let domain = &mut (*priv).audio_domains[domain_id as usize];
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut codec_dai: *mut c_void = null_mut();
    let mut active_rate: c_uint = 0;
    let mut ret: c_int = 0;
    let mut i: c_int = 0;

    mutex_lock(&mut (*priv).mutex as *mut c_void);

    domain.active += 1;

    i = 0;
    while (i as usize) < 2 {
        active_rate = (*priv).audio_domains[i as usize].rate;
        if active_rate != 0 {
            break;
        }
        i += 1;
    }

    if active_rate != 0 {
        ret = snd_pcm_hw_constraint_single(
            substream as *mut c_void,
            SNDRV_PCM_HW_PARAM_RATE,
            active_rate,
        );
    } else {
        ret = snd_pcm_hw_rule_add(
            substream as *mut c_void,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            j721e_rule_rate,
            &mut (*priv).rate_range as *mut c_void,
            SNDRV_PCM_HW_PARAM_RATE,
            -1,
        );
    }

    if ret != 0 {
        goto_out(domain, priv, ret);
        return ret;
    }

    ret = snd_soc_dai_set_tdm_slot(cpu_dai, 0x3, 0x3, 2, 32);
    if ret != 0 && ret != -ENOTSUPP {
        goto_out(domain, priv, ret);
        return ret;
    }

    for_each_rtd_codec_dais(rtd, &mut i, &mut codec_dai);
    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x3, 0x3, 2, 32);
    if ret != 0 && ret != -ENOTSUPP {
        goto_out(domain, priv, ret);
        return ret;
    }

    if ret == -ENOTSUPP {
        ret = 0;
    }

    mutex_unlock(&mut (*priv).mutex as *mut c_void);
    ret
}

unsafe fn goto_out(domain: &mut J721eAudioDomain, priv: *mut J721ePriv, ret: c_int) {
    if ret != 0 {
        domain.active -= 1;
    }
}

unsafe fn j721e_audio_hw_params(
    substream: *mut c_void,
    params: *mut HwParamInterval,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd as *mut DavincimcaspContext as *mut c_void);
    let priv = snd_soc_card_get_drvdata(card) as *mut J721ePriv;
    let domain_id = (*rtd as *mut DavincimcaspContext as *mut c_void) as c_uint;
    let domain = &mut (*priv).audio_domains[domain_id as usize];
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut codec_dai: *mut c_void = null_mut();
    let mut sysclk_rate: c_uint = 0;
    let mut slot_width: c_int = 32;
    let mut ret: c_int = 0;
    let mut i: c_int = 0;

    mutex_lock(&mut (*priv).mutex as *mut c_void);

    if domain.rate != 0 && domain.rate != (*rtd as *mut DavincimcaspContext as *mut c_void) as u32 {
        mutex_unlock(&mut (*priv).mutex as *mut c_void);
        return -EINVAL;
    }

    if (*rtd as *mut DavincimcaspContext as *mut c_void) as c_int == 16 {
        slot_width = 16;
    }

    ret = snd_soc_dai_set_tdm_slot(cpu_dai, 0x3, 0x3, 2, slot_width);
    if ret != 0 && ret != -ENOTSUPP {
        mutex_unlock(&mut (*priv).mutex as *mut c_void);
        return ret;
    }

    for_each_rtd_codec_dais(rtd, &mut i, &mut codec_dai);
    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x3, 0x3, 2, slot_width);
    if ret != 0 && ret != -ENOTSUPP {
        mutex_unlock(&mut (*priv).mutex as *mut c_void);
        return ret;
    }

    ret = j721e_configure_refclk(priv, domain_id, (*rtd as *mut DavincimcaspContext as *mut c_void) as u32);
    if ret != 0 {
        mutex_unlock(&mut (*priv).mutex as *mut c_void);
        return ret;
    }

    sysclk_rate = (*priv).hsdiv_rates[domain.parent_clk_id as usize];
    for_each_rtd_codec_dais(rtd, &mut i, &mut codec_dai);
    ret = snd_soc_dai_set_sysclk(codec_dai, 0, sysclk_rate, SND_SOC_CLOCK_IN);
    if ret != 0 && ret != -ENOTSUPP {
        dev_err(
            (*priv).dev,
            c"codec set_sysclk failed for %u Hz\n".as_ptr(),
            sysclk_rate,
        );
        mutex_unlock(&mut (*priv).mutex as *mut c_void);
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(cpu_dai, MCASP_CLK_HCLK_AUXCLK, sysclk_rate, SND_SOC_CLOCK_IN);

    if ret != 0 && ret != -ENOTSUPP {
        dev_err(
            (*priv).dev,
            c"mcasp set_sysclk failed for %u Hz\n".as_ptr(),
            sysclk_rate,
        );
    } else {
        domain.rate = (*rtd as *mut DavincimcaspContext as *mut c_void) as u32;
        ret = 0;
    }

    mutex_unlock(&mut (*priv).mutex as *mut c_void);
    ret
}

unsafe fn j721e_audio_shutdown(substream: *mut c_void) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let priv = snd_soc_card_get_drvdata((*rtd as *mut DavincimcaspContext as *mut c_void)) as *mut J721ePriv;
    let domain_id = (*rtd as *mut DavincimcaspContext as *mut c_void) as c_uint;
    let domain = &mut (*priv).audio_domains[domain_id as usize];

    mutex_lock(&mut (*priv).mutex as *mut c_void);

    domain.active -= 1;
    if domain.active == 0 {
        domain.rate = 0;
        domain.active_link = 0;
    }

    mutex_unlock(&mut (*priv).mutex as *mut c_void);
}

#[repr(C)]
struct SndSocOps {
    startup: unsafe extern "C" fn(*mut c_void) -> c_int,
    hw_params: unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int,
    shutdown: unsafe extern "C" fn(*mut c_void),
}

static J721E_AUDIO_OPS: SndSocOps = SndSocOps {
    startup: j721e_audio_startup,
    hw_params: j721e_audio_hw_params,
    shutdown: j721e_audio_shutdown,
};

unsafe fn j721e_audio_init(rtd: *mut c_void) -> c_int {
    let priv = snd_soc_card_get_drvdata((*rtd as *mut DavincimcaspContext as *mut c_void)) as *mut J721ePriv;
    let domain_id = (*rtd as *mut DavincimcaspContext as *mut c_void) as c_uint;
    let domain = &mut (*priv).audio_domains[domain_id as usize];
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut codec_dai: *mut c_void = null_mut();
    let mut sysclk_rate: c_uint = 0;
    let mut i: c_int = 0;
    let mut ret: c_int = 0;

    ret = j721e_configure_refclk(priv, domain_id, 48000);
    if ret != 0 {
        return ret;
    }

    sysclk_rate = (*priv).hsdiv_rates[domain.parent_clk_id as usize];
    for_each_rtd_codec_dais(rtd, &mut i, &mut codec_dai);
    ret = snd_soc_dai_set_sysclk(codec_dai, 0, sysclk_rate, SND_SOC_CLOCK_IN);
    if ret != 0 && ret != -ENOTSUPP {
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(cpu_dai, MCASP_CLK_HCLK_AUXCLK, sysclk_rate, SND_SOC_CLOCK_IN);
    if ret != 0 && ret != -ENOTSUPP {
        return ret;
    }

    ret = snd_soc_dai_set_tdm_slot(cpu_dai, 0x3, 0x3, 2, 32);
    if ret != 0 && ret != -ENOTSUPP {
        return ret;
    }

    for_each_rtd_codec_dais(rtd, &mut i, &mut codec_dai);
    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x3, 0x3, 2, 32);
    if ret != 0 && ret != -ENOTSUPP {
        return ret;
    }

    0
}

unsafe fn j721e_audio_init_ivi(rtd: *mut c_void) -> c_int {
    let dapm = snd_soc_card_to_dapm((*rtd as *mut DavincimcaspContext as *mut c_void));

    snd_soc_dapm_new_controls(
        dapm,
        &J721E_IVI_CODEC_A_DAPM_WIDGETS[0] as *const c_void,
        7,
    );
    snd_soc_dapm_add_routes(
        dapm,
        &J721E_CODEC_A_DAPM_ROUTES[0] as *const c_void,
        12,
    );
    snd_soc_dapm_new_controls(
        dapm,
        &J721E_IVI_CODEC_B_DAPM_WIDGETS[0] as *const c_void,
        7,
    );
    snd_soc_dapm_add_routes(
        dapm,
        &J721E_CODEC_B_DAPM_ROUTES[0] as *const c_void,
        12,
    );

    j721e_audio_init(rtd)
}

unsafe fn j721e_get_clocks(
    dev: *mut c_void,
    clocks: *mut J721eAudioClocks,
    prefix: *const c_char,
) -> c_int {
    let mut parent: *mut c_void;
    let mut clk_name: *mut c_char;
    let mut ret: c_int;

    (*clocks).target = devm_clk_get(dev, prefix);
    if IS_ERR((*clocks).target as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*clocks).target as *const c_void),
            c"failed to acquire %s\n".as_ptr(),
            prefix,
        );
    }

    clk_name = kasprintf(GFP_KERNEL, c"%s-48000".as_ptr(), prefix);
    if !clk_name.is_null() {
        parent = devm_clk_get(dev, clk_name);
        kfree(clk_name as *mut c_void);
        if IS_ERR(parent as *const c_void) {
            ret = PTR_ERR(parent as *const c_void);
            if ret == -EPROBE_DEFER {
                return ret;
            }

            dev_dbg(dev, c"no 48KHz parent for %s: %d\n".as_ptr(), prefix, ret);
            parent = null_mut();
        }
        (*clocks).parent[J721E_CLK_PARENT_48000] = parent;
    } else {
        return -ENOMEM;
    }

    clk_name = kasprintf(GFP_KERNEL, c"%s-44100".as_ptr(), prefix);
    if !clk_name.is_null() {
        parent = devm_clk_get(dev, clk_name);
        kfree(clk_name as *mut c_void);
        if IS_ERR(parent as *const c_void) {
            ret = PTR_ERR(parent as *const c_void);
            if ret == -EPROBE_DEFER {
                return ret;
            }

            dev_dbg(dev, c"no 44.1KHz parent for %s: %d\n".as_ptr(), prefix, ret);
            parent = null_mut();
        }
        (*clocks).parent[J721E_CLK_PARENT_44100] = parent;
    } else {
        return -ENOMEM;
    }

    if (*clocks).parent[J721E_CLK_PARENT_44100].is_null()
        && (*clocks).parent[J721E_CLK_PARENT_48000].is_null()
    {
        dev_err(
            dev,
            c"At least one parent clock is needed for %s\n".as_ptr(),
            prefix,
        );
        return -EINVAL;
    }

    0
}

static J721E_CPB_DATA: J721eAudioMatchData = J721eAudioMatchData {
    board_type: J721eBoardType::J721eBoardCpb,
    num_links: 2,
    pll_rates: [1179648000, 1083801600],
};

static J721E_CPB_IVI_DATA: J721eAudioMatchData = J721eAudioMatchData {
    board_type: J721eBoardType::J721eBoardCpbIvi,
    num_links: 4,
    pll_rates: [1179648000, 1083801600],
};

static J7200_CPB_DATA: J721eAudioMatchData = J721eAudioMatchData {
    board_type: J721eBoardType::J721eBoardCpb,
    num_links: 2,
    pll_rates: [2359296000, 0],
};

#[repr(C)]
struct OfDeviceIdEntry {
    compatible: *const c_char,
    data: *const c_void,
}

static J721E_AUDIO_OF_MATCH: [OfDeviceIdEntry; 4] = [
    OfDeviceIdEntry {
        compatible: c"ti,j721e-cpb-audio".as_ptr(),
        data: &J721E_CPB_DATA as *const _ as *const c_void,
    },
    OfDeviceIdEntry {
        compatible: c"ti,j721e-cpb-ivi-audio".as_ptr(),
        data: &J721E_CPB_IVI_DATA as *const _ as *const c_void,
    },
    OfDeviceIdEntry {
        compatible: c"ti,j7200-cpb-audio".as_ptr(),
        data: &J7200_CPB_DATA as *const _ as *const c_void,
    },
    OfDeviceIdEntry {
        compatible: null_mut(),
        data: null_mut(),
    },
];

unsafe fn j721e_calculate_rate_range(priv: *mut J721ePriv) -> c_int {
    let match_data = (*priv).match_data;
    let domain_clocks = &(*priv).audio_domains[0].mcasp;
    let mut min_rate: c_uint;
    let mut max_rate: c_uint;
    let mut pll_rate: c_uint;
    let mut pll: *mut c_void;

    pll = clk_get_parent(domain_clocks.parent[J721E_CLK_PARENT_44100]);
    if IS_ERR_OR_NULL(pll as *const c_void) {
        (*priv).pll_rates[J721E_CLK_PARENT_44100] =
            (*match_data).pll_rates[J721E_CLK_PARENT_44100];
    } else {
        (*priv).pll_rates[J721E_CLK_PARENT_44100] = clk_get_rate(pll);
        clk_put(pll);
    }

    pll = clk_get_parent(domain_clocks.parent[J721E_CLK_PARENT_48000]);
    if IS_ERR_OR_NULL(pll as *const c_void) {
        (*priv).pll_rates[J721E_CLK_PARENT_48000] =
            (*match_data).pll_rates[J721E_CLK_PARENT_48000];
    } else {
        (*priv).pll_rates[J721E_CLK_PARENT_48000] = clk_get_rate(pll);
        clk_put(pll);
    }

    if (*priv).pll_rates[J721E_CLK_PARENT_44100] == 0
        && (*priv).pll_rates[J721E_CLK_PARENT_48000] == 0
    {
        dev_err((*priv).dev, c"At least one PLL is needed\n".as_ptr());
        return -EINVAL;
    }

    if (*priv).pll_rates[J721E_CLK_PARENT_44100] != 0 {
        pll_rate = (*priv).pll_rates[J721E_CLK_PARENT_44100];
    } else {
        pll_rate = (*priv).pll_rates[J721E_CLK_PARENT_48000];
    }

    min_rate = pll_rate / J721E_MAX_CLK_HSDIV;
    min_rate /= RATIOS_FOR_PCM3168A[RATIOS_FOR_PCM3168A.len() - 1];

    if (*priv).pll_rates[J721E_CLK_PARENT_48000] != 0 {
        pll_rate = (*priv).pll_rates[J721E_CLK_PARENT_48000];
    } else {
        pll_rate = (*priv).pll_rates[J721E_CLK_PARENT_44100];
    }

    if pll_rate > PCM1368A_MAX_SYSCLK {
        pll_rate = PCM1368A_MAX_SYSCLK;
    }

    max_rate = pll_rate / RATIOS_FOR_PCM3168A[0];

    snd_interval_any(&mut (*priv).rate_range as *mut c_void);
    *((&mut (*priv).rate_range as *mut c_void) as *mut c_uint) = min_rate;
    *(((&mut (*priv).rate_range as *mut c_void) as *mut u8).add(4) as *mut c_uint) = max_rate;

    0
}

unsafe fn j721e_soc_probe_cpb(
    priv: *mut J721ePriv,
    link_idx: *mut c_int,
    conf_idx: *mut c_int,
) -> c_int {
    let node = (*(*priv).dev as *mut c_void) as *mut c_void;
    let mut compnent: *mut c_void;
    let mut dai_node: *mut c_void;
    let mut codec_node: *mut c_void;
    let domain: *mut J721eAudioDomain;
    let mut comp_count: c_int;
    let mut comp_idx: c_int;
    let mut ret: c_int;

    dai_node = of_parse_phandle(node, c"ti,cpb-mcasp".as_ptr(), 0);
    if dai_node.is_null() {
        dev_err((*priv).dev, c"CPB McASP node is not provided\n".as_ptr());
        return -EINVAL;
    }

    codec_node = of_parse_phandle(node, c"ti,cpb-codec".as_ptr(), 0);
    if codec_node.is_null() {
        dev_err((*priv).dev, c"CPB codec node is not provided\n".as_ptr());
        ret = -EINVAL;
        of_node_put(dai_node);
        return ret;
    }

    domain = &mut (*priv).audio_domains[0];
    ret = j721e_get_clocks((*priv).dev, &mut (*domain).codec, c"cpb-codec-scki".as_ptr());
    if ret != 0 {
        of_node_put(codec_node);
        of_node_put(dai_node);
        return ret;
    }

    ret = j721e_get_clocks((*priv).dev, &mut (*domain).mcasp, c"cpb-mcasp-auxclk".as_ptr());
    if ret != 0 {
        of_node_put(codec_node);
        of_node_put(dai_node);
        return ret;
    }

    comp_count = 6;
    compnent = devm_kcalloc((*priv).dev, comp_count as c_uint, core::mem::size_of::<c_void>(), GFP_KERNEL);
    if compnent.is_null() {
        ret = -ENOMEM;
        of_node_put(codec_node);
        of_node_put(dai_node);
        return ret;
    }

    comp_idx = 0;

    of_node_put(codec_node);
    of_node_put(dai_node);
    0
}

unsafe fn j721e_soc_probe_ivi(
    priv: *mut J721ePriv,
    link_idx: *mut c_int,
    conf_idx: *mut c_int,
) -> c_int {
    let node = (*(*priv).dev as *mut c_void) as *mut c_void;
    let mut compnent: *mut c_void;
    let mut dai_node: *mut c_void;
    let mut codeca_node: *mut c_void;
    let mut codecb_node: *mut c_void;
    let domain: *mut J721eAudioDomain;
    let mut comp_count: c_int;
    let mut comp_idx: c_int;
    let mut ret: c_int;

    if (*(*priv).match_data).board_type as c_int != J721eBoardType::J721eBoardCpbIvi as c_int {
        return 0;
    }

    dai_node = of_parse_phandle(node, c"ti,ivi-mcasp".as_ptr(), 0);
    if dai_node.is_null() {
        dev_err((*priv).dev, c"IVI McASP node is not provided\n".as_ptr());
        return -EINVAL;
    }

    codeca_node = of_parse_phandle(node, c"ti,ivi-codec-a".as_ptr(), 0);
    if codeca_node.is_null() {
        dev_err((*priv).dev, c"IVI codec-a node is not provided\n".as_ptr());
        ret = -EINVAL;
        of_node_put(dai_node);
        return ret;
    }

    codecb_node = of_parse_phandle(node, c"ti,ivi-codec-b".as_ptr(), 0);
    if codecb_node.is_null() {
        dev_warn((*priv).dev, c"IVI codec-b node is not provided\n".as_ptr());
        ret = 0;
        of_node_put(codeca_node);
        of_node_put(dai_node);
        return ret;
    }

    domain = &mut (*priv).audio_domains[1];
    ret = j721e_get_clocks((*priv).dev, &mut (*domain).codec, c"ivi-codec-scki".as_ptr());
    if ret != 0 {
        of_node_put(codecb_node);
        of_node_put(codeca_node);
        of_node_put(dai_node);
        return ret;
    }

    ret = j721e_get_clocks((*priv).dev, &mut (*domain).mcasp, c"ivi-mcasp-auxclk".as_ptr());
    if ret != 0 {
        of_node_put(codecb_node);
        of_node_put(codeca_node);
        of_node_put(dai_node);
        return ret;
    }

    comp_count = 8;
    compnent = devm_kcalloc((*priv).dev, comp_count as c_uint, core::mem::size_of::<c_void>(), GFP_KERNEL);
    if compnent.is_null() {
        ret = -ENOMEM;
        of_node_put(codecb_node);
        of_node_put(codeca_node);
        of_node_put(dai_node);
        return ret;
    }

    of_node_put(codecb_node);
    of_node_put(codeca_node);
    of_node_put(dai_node);
    0
}

unsafe fn j721e_soc_probe(pdev: *mut c_void) -> c_int {
    let match_data: *const J721eAudioMatchData;
    let mut priv: *mut J721ePriv;
    let mut link_cnt: c_int = 0;
    let mut conf_cnt: c_int = 0;
    let mut ret: c_int;
    let mut i: c_int = 0;

    match_data = of_device_get_match_data(pdev) as *const J721eAudioMatchData;
    if match_data.is_null() {
        dev_err(pdev, c"No compatible match found\n".as_ptr());
        return -ENODEV;
    }

    priv = devm_kzalloc(
        pdev,
        core::mem::size_of::<J721ePriv>() + ((*match_data).num_links as usize * core::mem::size_of::<c_void>()),
        GFP_KERNEL,
    ) as *mut J721ePriv;
    if priv.is_null() {
        return -ENOMEM;
    }

    (*priv).match_data = match_data;

    i = 0;
    while i < 2 {
        (*priv).audio_domains[i as usize].parent_clk_id = -1;
        i += 1;
    }

    (*priv).dev = pdev;

    ret = j721e_soc_probe_cpb(priv, &mut link_cnt, &mut conf_cnt);
    if ret != 0 {
        return ret;
    }

    ret = j721e_soc_probe_ivi(priv, &mut link_cnt, &mut conf_cnt);
    if ret != 0 {
        return ret;
    }

    ret = j721e_calculate_rate_range(priv);
    if ret != 0 {
        return ret;
    }

    mutex_init(&mut (*priv).mutex as *mut c_void);
    ret = devm_snd_soc_register_card((*priv).dev, &mut (*priv).card as *mut c_void);
    if ret != 0 {
        dev_err_probe(
            (*priv).dev,
            ret,
            c"devm_snd_soc_register_card() failed: %d\n".as_ptr(),
            ret,
        );
    }

    ret
}

#[repr(C)]
struct PlatformDriver {
    probe: unsafe extern "C" fn(*mut c_void) -> c_int,
    driver_name: *const c_char,
}

// Kernel module entry points - typically handled via module_platform_driver macro
// This module is registered as a platform driver for device tree matches

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
