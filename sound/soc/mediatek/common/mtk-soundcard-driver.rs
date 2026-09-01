// SPDX-License-Identifier: GPL-2.0
/*
 * mtk-soundcard-driver.c  --  MediaTek soundcard driver common
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Trevor Wu <trevor.wu@mediatek.com>
 */

// Dependencies from the original C includes:
// linux/module.h, linux/of.h, linux/of_platform.h, sound/soc.h,
// mtk-dsp-sof-common.h, mtk-soc-card.h, mtk-soundcard-driver.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const MTK_CONSTRAINT_PLAYBACK: mtk_pcm_constraint_type = 0;
const MTK_CONSTRAINT_CAPTURE: mtk_pcm_constraint_type = 1;

type mtk_pcm_constraint_type = c_uint;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub dynamic: c_uint,
    pub dai_fmt: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_pcm_constraints_data {
    pub rates: *const snd_pcm_hw_constraint_list,
    pub channels: *const snd_pcm_hw_constraint_list,
}

#[repr(C)]
pub struct mtk_card_data {
    pub card: *mut snd_soc_card,
    pub pcm_constraints: *const mtk_pcm_constraints_data,
    pub num_jacks: c_uint,
    pub jacks: *mut snd_soc_jack,
}

#[repr(C)]
pub struct mtk_soc_card_data {
    pub card_data: *mut mtk_card_data,
    pub accdet: *mut snd_soc_component,
    pub sof_priv: *const c_void,
}

#[repr(C)]
pub struct mtk_soundcard_pdata {
    pub card_data: *mut mtk_card_data,
    pub card_name: *const c_char,
    pub sof_priv: *const c_void,
    pub soc_probe: Option<unsafe extern "C" fn(*mut mtk_soc_card_data, bool) -> c_int>,
}

unsafe extern "C" {
    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;

    fn of_get_child_by_name(node: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn of_property_read_string(
        node: *mut device_node,
        propname: *const c_char,
        out_string: *mut *const c_char,
    ) -> c_int;
    fn of_property_present(node: *mut device_node, propname: *const c_char) -> bool;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_find_device_by_node(np: *mut device_node) -> *mut platform_device;

    fn snd_soc_of_get_dai_link_codecs(
        dev: *mut device,
        codec_node: *mut device_node,
        dai_link: *mut snd_soc_dai_link,
    ) -> c_int;
    fn snd_soc_of_put_dai_link_codecs(dai_link: *mut snd_soc_dai_link);
    fn snd_soc_daifmt_parse_format(
        node: *mut device_node,
        prefix: *const c_char,
    ) -> c_uint;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char)
        -> c_int;
    fn snd_soc_lookup_component(
        dev: *mut device,
        driver_name: *const c_char,
    ) -> *mut snd_soc_component;
    fn snd_soc_card_set_topology_name(card: *mut snd_soc_card, topology_name: *const c_char);
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;

    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, gfp: c_uint) -> *mut c_void;
    fn put_device(dev: *mut device);

    fn mtk_sof_dailink_parse_of(
        dev: *mut device,
        card: *mut snd_soc_card,
        propname: *const c_char,
    ) -> c_int;
    fn mtk_sof_card_probe(card: *mut snd_soc_card) -> c_int;
    fn mtk_sof_card_late_probe(card: *mut snd_soc_card) -> c_int;

    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn strncmp(cs: *const c_char, ct: *const c_char, count: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;

    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info_once(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

#[repr(C)]
struct of_clk_entry {
    name: *const c_char,
    val: c_uint,
}

unsafe fn for_each_available_child_of_node_scoped<F>(parent: *mut device_node, mut f: F) -> c_int
where
    F: FnMut(*mut device_node) -> c_int,
{
    unsafe extern "C" {
        fn of_get_next_available_child(
            node: *mut device_node,
            prev: *mut device_node,
        ) -> *mut device_node;
    }

    let mut sub_node = of_get_next_available_child(parent, ptr::null_mut());
    while !sub_node.is_null() {
        let ret = f(sub_node);
        if ret != 0 {
            of_node_put(sub_node);
            return ret;
        }
        sub_node = of_get_next_available_child(parent, sub_node);
    }

    0
}

unsafe fn set_card_codec_info(
    dev: *mut device,
    sub_node: *mut device_node,
    dai_link: *mut snd_soc_dai_link,
) -> c_int {
    let codec_node: *mut device_node;
    let ret: c_int;

    codec_node = of_get_child_by_name(sub_node, c"codec".as_ptr());
    if codec_node.is_null() {
        dev_dbg(
            dev,
            c"%s no specified codec: setting dummy.\n".as_ptr(),
            (*dai_link).name,
        );

        (*dai_link).codecs = &raw mut snd_soc_dummy_dlc;
        (*dai_link).num_codecs = 1;
        (*dai_link).dynamic = 1;
        return 0;
    }

    /* set card codec info */
    ret = snd_soc_of_get_dai_link_codecs(dev, codec_node, dai_link);

    of_node_put(codec_node);

    if ret < 0 {
        return dev_err_probe(
            dev,
            ret,
            c"%s: codec dai not found\n".as_ptr(),
            (*dai_link).name,
        );
    }

    0
}

unsafe fn set_dailink_daifmt(
    sub_node: *mut device_node,
    dai_link: *mut snd_soc_dai_link,
) -> c_int {
    let mut daifmt: c_uint;
    let mut str_: *const c_char = ptr::null();
    let ret: c_int;
    let of_clk_table = [
        of_clk_entry {
            name: c"cpu".as_ptr(),
            val: SND_SOC_DAIFMT_CBC_CFC,
        },
        of_clk_entry {
            name: c"codec".as_ptr(),
            val: SND_SOC_DAIFMT_CBP_CFP,
        },
    ];

    daifmt = snd_soc_daifmt_parse_format(sub_node, ptr::null());
    if daifmt != 0 {
        (*dai_link).dai_fmt &= SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;
        (*dai_link).dai_fmt |= daifmt;
    }

    /*
     * check "mediatek,clk-provider = xxx"
     * SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK area
     */
    ret = of_property_read_string(
        sub_node,
        c"mediatek,clk-provider".as_ptr(),
        &mut str_,
    );
    if ret == 0 {
        let mut i: usize = 0;

        while i < of_clk_table.len() {
            if strcmp(str_, of_clk_table[i].name) == 0 {
                (*dai_link).dai_fmt &= !SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;
                (*dai_link).dai_fmt |= of_clk_table[i].val;
                break;
            }
            i += 1;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_dai_link_info(card: *mut snd_soc_card) -> c_int {
    let dev: *mut device = (*card).dev;
    let mut dai_link: *mut snd_soc_dai_link;
    let mut dai_link_name: *const c_char = ptr::null();
    let mut ret: c_int;

    /* Loop over all the dai link sub nodes */
    ret = for_each_available_child_of_node_scoped((*dev).of_node, |sub_node| {
        if of_property_read_string(sub_node, c"link-name".as_ptr(), &mut dai_link_name) != 0 {
            return -EINVAL;
        }

        let mut i: c_int = 0;
        dai_link = (*card).dai_link;
        while i < (*card).num_links {
            if strcmp(dai_link_name, (*dai_link).name) == 0 {
                break;
            }
            i += 1;
            dai_link = dai_link.add(1);
        }

        if i >= (*card).num_links {
            return -EINVAL;
        }

        ret = set_card_codec_info(dev, sub_node, dai_link);
        if ret < 0 {
            return ret;
        }

        ret = set_dailink_daifmt(sub_node, dai_link);
        if ret < 0 {
            return ret;
        }

        0
    });

    ret
}
// EXPORT_SYMBOL_GPL(parse_dai_link_info);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clean_card_reference(card: *mut snd_soc_card) {
    let mut dai_link: *mut snd_soc_dai_link;
    let mut i: c_int = 0;

    /* release codec reference gotten by set_card_codec_info */
    dai_link = (*card).dai_link;
    while i < (*card).num_links {
        snd_soc_of_put_dai_link_codecs(dai_link);
        i += 1;
        dai_link = dai_link.add(1);
    }
}
// EXPORT_SYMBOL_GPL(clean_card_reference);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_soundcard_startup(
    substream: *mut snd_pcm_substream,
    ctype: mtk_pcm_constraint_type,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let soc_card: *mut mtk_soc_card_data =
        snd_soc_card_get_drvdata((*rtd).card) as *mut mtk_soc_card_data;
    let mpc: *const mtk_pcm_constraints_data =
        (*(*soc_card).card_data).pcm_constraints.add(ctype as usize);
    let mut ret: c_int;

    if mpc.is_null() {
        return -EINVAL;
    }

    ret = snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        (*mpc).rates,
    );
    if ret < 0 {
        dev_err((*rtd).dev, c"hw_constraint_list rate failed\n".as_ptr());
        return ret;
    }

    ret = snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        (*mpc).channels,
    );
    if ret < 0 {
        dev_err((*rtd).dev, c"hw_constraint_list channel failed\n".as_ptr());
        return ret;
    }

    0
}
// EXPORT_SYMBOL_GPL(mtk_soundcard_startup);

unsafe extern "C" fn mtk_soundcard_playback_startup(
    substream: *mut snd_pcm_substream,
) -> c_int {
    mtk_soundcard_startup(substream, MTK_CONSTRAINT_PLAYBACK)
}

#[unsafe(no_mangle)]
pub static mtk_soundcard_common_playback_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mtk_soundcard_playback_startup),
};
// EXPORT_SYMBOL_GPL(mtk_soundcard_common_playback_ops);

unsafe extern "C" fn mtk_soundcard_capture_startup(
    substream: *mut snd_pcm_substream,
) -> c_int {
    mtk_soundcard_startup(substream, MTK_CONSTRAINT_CAPTURE)
}

#[unsafe(no_mangle)]
pub static mtk_soundcard_common_capture_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mtk_soundcard_capture_startup),
};
// EXPORT_SYMBOL_GPL(mtk_soundcard_common_capture_ops);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_soundcard_common_probe(pdev: *mut platform_device) -> c_int {
    let mut platform_node: *mut device_node;
    let mut adsp_node: *mut device_node;
    let mut accdet_node: *mut device_node;
    let mut accdet_comp: *mut snd_soc_component;
    let mut accdet_pdev: *mut platform_device;
    let pdata: *const mtk_soundcard_pdata;
    let soc_card_data: *mut mtk_soc_card_data;
    let orig_dai_link: *mut snd_soc_dai_link;
    let mut dai_link: *mut snd_soc_dai_link;
    let jacks: *mut snd_soc_jack;
    let card: *mut snd_soc_card;
    let mut ret: c_int;
    let orig_num_links: c_int;
    let needs_legacy_probe: bool;

    pdata = device_get_match_data(&mut (*pdev).dev) as *const mtk_soundcard_pdata;
    if pdata.is_null() {
        return -EINVAL;
    }

    card = (*(*pdata).card_data).card;
    (*card).dev = &mut (*pdev).dev;
    orig_dai_link = (*card).dai_link;
    orig_num_links = (*card).num_links;

    ret = snd_soc_of_parse_card_name(card, c"model".as_ptr());
    if ret != 0 {
        return ret;
    }

    if (*card).name.is_null() {
        if (*pdata).card_name.is_null() {
            return -EINVAL;
        }

        (*card).name = (*pdata).card_name;
    }

    needs_legacy_probe = !of_property_present((*pdev).dev.of_node, c"audio-routing".as_ptr());
    if needs_legacy_probe {
        /*
         * If we have no .soc_probe() callback there's no way of using
         * any legacy probe mechanism, as that cannot not be generic.
         */
        if (*pdata).soc_probe.is_none() {
            return -EINVAL;
        }

        dev_info_once(
            &mut (*pdev).dev,
            c"audio-routing not found: using legacy probe\n".as_ptr(),
        );
    } else {
        ret = snd_soc_of_parse_audio_routing(card, c"audio-routing".as_ptr());
        if ret != 0 {
            return ret;
        }
    }

    soc_card_data = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<mtk_soc_card_data>(),
        GFP_KERNEL,
    ) as *mut mtk_soc_card_data;
    if soc_card_data.is_null() {
        return -ENOMEM;
    }

    (*soc_card_data).card_data = (*pdata).card_data;

    jacks = devm_kcalloc(
        (*card).dev,
        (*(*soc_card_data).card_data).num_jacks as usize,
        size_of::<snd_soc_jack>(),
        GFP_KERNEL,
    ) as *mut snd_soc_jack;
    if jacks.is_null() {
        return -ENOMEM;
    }

    (*(*soc_card_data).card_data).jacks = jacks;

    accdet_node = of_parse_phandle((*pdev).dev.of_node, c"mediatek,accdet".as_ptr(), 0);
    if !accdet_node.is_null() {
        accdet_pdev = of_find_device_by_node(accdet_node);
        if !accdet_pdev.is_null() {
            accdet_comp = snd_soc_lookup_component(&mut (*accdet_pdev).dev, ptr::null());
            if !accdet_comp.is_null() {
                (*soc_card_data).accdet = accdet_comp;
            } else {
                dev_err(
                    &mut (*pdev).dev,
                    c"No sound component found from mediatek,accdet property\n".as_ptr(),
                );
            }

            put_device(&mut (*accdet_pdev).dev);
        } else {
            dev_err(
                &mut (*pdev).dev,
                c"No device found from mediatek,accdet property\n".as_ptr(),
            );
        }

        of_node_put(accdet_node);
    }

    platform_node = of_parse_phandle((*pdev).dev.of_node, c"mediatek,platform".as_ptr(), 0);
    if platform_node.is_null() {
        return dev_err_probe(
            &mut (*pdev).dev,
            -EINVAL,
            c"Property mediatek,platform missing or invalid\n".as_ptr(),
        );
    }

    /* Check if this SoC has an Audio DSP */
    if !(*pdata).sof_priv.is_null() {
        adsp_node = of_parse_phandle((*pdev).dev.of_node, c"mediatek,adsp".as_ptr(), 0);
    } else {
        adsp_node = ptr::null_mut();
    }

    if !adsp_node.is_null() {
        if of_property_present((*pdev).dev.of_node, c"mediatek,dai-link".as_ptr()) {
            ret = mtk_sof_dailink_parse_of(
                &mut (*pdev).dev,
                card,
                c"mediatek,dai-link".as_ptr(),
            );
            if ret != 0 {
                of_node_put(adsp_node);
                of_node_put(platform_node);
                return dev_err_probe(
                    &mut (*pdev).dev,
                    ret,
                    c"Cannot parse mediatek,dai-link\n".as_ptr(),
                );
            }
        }

        (*soc_card_data).sof_priv = (*pdata).sof_priv;
        (*card).probe = Some(mtk_sof_card_probe);
        (*card).late_probe = Some(mtk_sof_card_late_probe);

        snd_soc_card_set_topology_name(card, c"sof".as_ptr());
    }

    /*
     * Regardless of whether the ADSP is wanted and/or present in a machine
     * specific device tree or not and regardless of whether any AFE_SOF
     * link is present, we have to make sure that the platforms->of_node
     * is not NULL, and set to either ADSP (adsp_node) or AFE (platform_node).
     */
    let mut i: c_int = 0;
    dai_link = (*card).dai_link;
    while i < (*card).num_links {
        if !adsp_node.is_null()
            && strncmp((*dai_link).name, c"AFE_SOF".as_ptr(), strlen(c"AFE_SOF".as_ptr())) == 0
        {
            (*(*dai_link).platforms).of_node = adsp_node;
        } else if (*(*dai_link).platforms).name.is_null()
            && (*(*dai_link).platforms).of_node.is_null()
        {
            (*(*dai_link).platforms).of_node = platform_node;
        }

        i += 1;
        dai_link = dai_link.add(1);
    }

    if !needs_legacy_probe {
        ret = parse_dai_link_info(card);
        if ret != 0 {
            (*card).dai_link = orig_dai_link;
            (*card).num_links = orig_num_links;
            return ret;
        }
    } else {
        if !adsp_node.is_null() {
            of_node_put(adsp_node);
        }
        of_node_put(platform_node);
    }

    if let Some(soc_probe) = (*pdata).soc_probe {
        ret = soc_probe(soc_card_data, needs_legacy_probe);
        if ret != 0 {
            if !needs_legacy_probe {
                clean_card_reference(card);
            }
            (*card).dai_link = orig_dai_link;
            (*card).num_links = orig_num_links;
            return ret;
        }
    }
    snd_soc_card_set_drvdata(card, soc_card_data as *mut c_void);

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);

    if !needs_legacy_probe {
        clean_card_reference(card);
    }

    if ret != 0 {
        dev_err_probe(&mut (*pdev).dev, ret, c"Cannot register card\n".as_ptr());
        (*card).dai_link = orig_dai_link;
        (*card).num_links = orig_num_links;
        return ret;
    }

    0
}
// EXPORT_SYMBOL_GPL(mtk_soundcard_common_probe);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
