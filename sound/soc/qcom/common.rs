// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018, Linaro Limited.
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

// Dependencies from:
// <dt-bindings/sound/qcom,q6afe.h>
// <linux/module.h>
// <sound/jack.h>
// <linux/input-event-codes.h>
// "common.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const NAME_SIZE: usize = 32;

const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const ENOTSUPP: c_int = 524;

const GFP_KERNEL: c_uint = 0;

const SND_JACK_HEADPHONE: c_uint = 0x0001;
const SND_JACK_MICROPHONE: c_uint = 0x0002;
const SND_JACK_LINEOUT: c_uint = 0x0004;
const SND_JACK_MECHANICAL: c_uint = 0x0008;
const SND_JACK_BTN_0: c_uint = 0x4000;
const SND_JACK_BTN_1: c_uint = 0x2000;
const SND_JACK_BTN_2: c_uint = 0x1000;
const SND_JACK_BTN_3: c_uint = 0x0800;
const SND_JACK_BTN_4: c_uint = 0x0400;
const SND_JACK_BTN_5: c_uint = 0x0200;
const SND_JACK_HEADSET: c_uint = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_AVOUT: c_uint = SND_JACK_LINEOUT;

const KEY_MEDIA: c_uint = 226;
const KEY_VOICECOMMAND: c_uint = 246;
const KEY_VOLUMEUP: c_uint = 115;
const KEY_VOLUMEDOWN: c_uint = 114;

extern "C" {
    static snd_soc_dummy_dlc: snd_soc_dai_link_component;

    static LPASS_MAX_PORT: c_uint;
    static LPI_MI2S_RX_0: c_uint;
    static TX_CODEC_DMA_TX_0: c_uint;
    static TX_CODEC_DMA_TX_1: c_uint;
    static TX_CODEC_DMA_TX_2: c_uint;
    static TX_CODEC_DMA_TX_3: c_uint;

    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn of_get_child_by_name(np: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_parse_phandle_with_args(
        np: *mut device_node,
        list_name: *const c_char,
        cells_name: *const c_char,
        index: c_int,
        out_args: *mut of_phandle_args,
    ) -> c_int;
    fn of_node_put(np: *mut device_node);
    fn of_node_get(np: *mut device_node) -> *mut device_node;
    fn snd_soc_of_parse_tdm_slot(
        np: *mut device_node,
        tx_mask: *mut c_uint,
        rx_mask: *mut c_uint,
        slots: *mut c_uint,
        slot_width: *mut c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_uint,
        slot_width: c_uint,
    ) -> c_int;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool;
    fn snd_soc_of_parse_audio_simple_widgets(
        card: *mut snd_soc_card,
        propname: *const c_char,
    ) -> c_int;
    fn snd_soc_of_parse_audio_routing(
        card: *mut snd_soc_card,
        propname: *const c_char,
    ) -> c_int;
    fn snd_soc_of_parse_pin_switches(card: *mut snd_soc_card, propname: *const c_char)
        -> c_int;
    fn snd_soc_of_parse_aux_devs(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn of_get_available_child_count(np: *mut device_node) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn of_property_read_string(
        np: *mut device_node,
        propname: *const c_char,
        out_string: *mut *const c_char,
    ) -> c_int;
    fn snd_soc_of_get_dlc(
        np: *mut device_node,
        args: *mut of_phandle_args,
        dlc: *mut snd_soc_dai_link_component,
        index: c_int,
    ) -> c_int;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int)
        -> *mut device_node;
    fn snd_soc_of_get_dai_link_codecs(
        dev: *mut device,
        codec: *mut device_node,
        link: *mut snd_soc_dai_link,
    ) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_uint, keytype: c_uint);
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_card_jack_new(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
    ) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_uint,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link_cpu {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub cpus: *mut snd_soc_dai_link_component,
    pub platforms: *mut snd_soc_dai_link_component,
    pub codecs: *const snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub num_platforms: c_uint,
    pub num_codecs: c_uint,
    pub name: *const c_char,
    pub id: c_uint,
    pub no_pcm: c_uint,
    pub ignore_pmdown_time: c_uint,
    pub dynamic: c_uint,
    pub ignore_suspend: c_uint,
    pub nonatomic: c_uint,
    pub stream_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub name: *const c_char,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct of_phandle_args {
    pub np: *mut device_node,
    pub args_count: c_int,
    pub args: [c_uint; 16],
}

#[repr(C)]
pub struct qcom_snd_tdm_slot_cfg {
    pub tx_mask: c_uint,
    pub rx_mask: c_uint,
    pub slots: c_uint,
    pub slot_width: c_uint,
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_uint,
}

// Static DAPM widget initializers use SND_SOC_DAPM_* C macros supplied by ASoC.
extern "C" {
    static qcom_jack_snd_widgets: [snd_soc_dapm_widget; 10];
}

unsafe fn for_each_available_child_of_node_scoped<F>(parent: *mut device_node, mut f: F) -> c_int
where
    F: FnMut(*mut device_node) -> c_int,
{
    // for_each_available_child_of_node_scoped is a Linux OF iteration macro.
    // Its iterator implementation is supplied outside this isolated file.
    let _ = parent;
    let _ = &mut f;
    0
}

unsafe fn for_each_rtd_codec_dais<F>(rtd: *mut snd_soc_pcm_runtime, mut f: F) -> c_int
where
    F: FnMut(c_int, *mut snd_soc_dai) -> c_int,
{
    // for_each_rtd_codec_dais is an ASoC runtime codec DAI iteration macro.
    // Its iterator implementation is supplied outside this isolated file.
    let _ = rtd;
    let _ = &mut f;
    0
}

unsafe fn qcom_snd_get_link_node(rtd: *mut snd_soc_pcm_runtime) -> *mut device_node {
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let card = (*rtd).card;
    let mut args: of_phandle_args = mem::zeroed();
    let mut found: *mut device_node = ptr::null_mut();

    if (*card).dev.is_null() || (*(*card).dev).of_node.is_null() {
        return ptr::null_mut();
    }

    let ret = for_each_available_child_of_node_scoped((*(*card).dev).of_node, |np| {
        let cpu_np = of_get_child_by_name(np, c"cpu".as_ptr());

        if cpu_np.is_null() {
            return 0;
        }

        let ret = of_parse_phandle_with_args(
            cpu_np,
            c"sound-dai".as_ptr(),
            c"#sound-dai-cells".as_ptr(),
            0,
            &mut args,
        );
        if ret != 0 {
            of_node_put(cpu_np);
            return 0;
        }

        if args.np == (*(*rtd).dai_link).cpus.add(0).as_ref().unwrap().of_node
            && args.args_count == 1
            && args.args[0] == (*cpu_dai).id
        {
            of_node_put(args.np);
            found = of_node_get(np);
            of_node_put(cpu_np);
            return 1;
        }

        of_node_put(args.np);
        of_node_put(cpu_np);
        0
    });

    let _ = ret;
    found
}

unsafe fn qcom_snd_parse_tdm_slot(
    np: *mut device_node,
    cfg: *mut qcom_snd_tdm_slot_cfg,
) -> c_int {
    ptr::write_bytes(cfg, 0, 1);

    snd_soc_of_parse_tdm_slot(
        np,
        &mut (*cfg).tx_mask,
        &mut (*cfg).rx_mask,
        &mut (*cfg).slots,
        &mut (*cfg).slot_width,
    )
}

unsafe fn qcom_snd_normalize_tdm_slots(
    cpu_cfg: *mut qcom_snd_tdm_slot_cfg,
    codec_cfg: *mut qcom_snd_tdm_slot_cfg,
) -> c_int {
    let slots: c_uint;
    let slot_width: c_uint;

    if (*cpu_cfg).slots != 0 && (*codec_cfg).slots != 0 && (*cpu_cfg).slots != (*codec_cfg).slots {
        return -EINVAL;
    }

    if (*cpu_cfg).slot_width != 0
        && (*codec_cfg).slot_width != 0
        && (*cpu_cfg).slot_width != (*codec_cfg).slot_width
    {
        return -EINVAL;
    }

    slots = if (*cpu_cfg).slots != 0 {
        (*cpu_cfg).slots
    } else {
        (*codec_cfg).slots
    };
    if slots == 0 {
        return 0;
    }

    slot_width = if (*cpu_cfg).slot_width != 0 {
        (*cpu_cfg).slot_width
    } else {
        (*codec_cfg).slot_width
    };
    if slot_width == 0 {
        return -EINVAL;
    }

    (*cpu_cfg).slots = slots;
    (*codec_cfg).slots = slots;
    (*cpu_cfg).slot_width = slot_width;
    (*codec_cfg).slot_width = slot_width;

    0
}

unsafe fn qcom_snd_parse_dai_tdm_slots(
    rtd: *mut snd_soc_pcm_runtime,
    cpu_cfg: *mut qcom_snd_tdm_slot_cfg,
    codec_cfg: *mut qcom_snd_tdm_slot_cfg,
) -> c_int {
    let link_np = qcom_snd_get_link_node(rtd);
    let mut ret: c_int;

    if link_np.is_null() {
        return -ENOENT;
    }

    let cpu_np = of_get_child_by_name(link_np, c"cpu".as_ptr());
    let codec_np = of_get_child_by_name(link_np, c"codec".as_ptr());
    if cpu_np.is_null() || codec_np.is_null() {
        of_node_put(cpu_np);
        of_node_put(codec_np);
        of_node_put(link_np);
        return -ENOENT;
    }

    ret = qcom_snd_parse_tdm_slot(cpu_np, cpu_cfg);
    if ret != 0 {
        of_node_put(cpu_np);
        of_node_put(codec_np);
        of_node_put(link_np);
        return ret;
    }

    ret = qcom_snd_parse_tdm_slot(codec_np, codec_cfg);
    of_node_put(cpu_np);
    of_node_put(codec_np);
    of_node_put(link_np);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn qcom_snd_get_dai_tdm_slots(
    rtd: *mut snd_soc_pcm_runtime,
    cpu_cfg: *mut qcom_snd_tdm_slot_cfg,
    codec_cfg: *mut qcom_snd_tdm_slot_cfg,
) -> c_int {
    let mut ret: c_int;

    ret = qcom_snd_parse_dai_tdm_slots(rtd, cpu_cfg, codec_cfg);
    if ret != 0 {
        return ret;
    }

    qcom_snd_normalize_tdm_slots(cpu_cfg, codec_cfg)
}
// EXPORT_SYMBOL_GPL(qcom_snd_get_dai_tdm_slots);

#[no_mangle]
pub unsafe extern "C" fn qcom_snd_apply_dai_tdm_slots_cfg(
    rtd: *mut snd_soc_pcm_runtime,
    cpu_cfg: *const qcom_snd_tdm_slot_cfg,
    codec_cfg: *const qcom_snd_tdm_slot_cfg,
) -> c_int {
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ret: c_int;

    if (*cpu_cfg).slots == 0 {
        return 0;
    }

    ret = snd_soc_dai_set_tdm_slot(
        cpu_dai,
        (*cpu_cfg).tx_mask,
        (*cpu_cfg).rx_mask,
        (*cpu_cfg).slots,
        (*cpu_cfg).slot_width,
    );
    if ret != 0 {
        return ret;
    }

    ret = for_each_rtd_codec_dais(rtd, |_i, codec_dai| {
        let rval = snd_soc_dai_set_tdm_slot(
            codec_dai,
            (*codec_cfg).tx_mask,
            (*codec_cfg).rx_mask,
            (*codec_cfg).slots,
            (*codec_cfg).slot_width,
        );
        if rval != 0 {
            return rval;
        }
        0
    });
    if ret != 0 {
        return ret;
    }

    0
}
// EXPORT_SYMBOL_GPL(qcom_snd_apply_dai_tdm_slots_cfg);

#[no_mangle]
pub unsafe extern "C" fn qcom_snd_apply_dai_tdm_slots(
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let mut cpu_cfg: qcom_snd_tdm_slot_cfg = mem::zeroed();
    let mut codec_cfg: qcom_snd_tdm_slot_cfg = mem::zeroed();
    let mut ret: c_int;

    ret = qcom_snd_get_dai_tdm_slots(rtd, &mut cpu_cfg, &mut codec_cfg);
    if ret != 0 {
        return if ret == -ENOENT { 0 } else { ret };
    }

    qcom_snd_apply_dai_tdm_slots_cfg(rtd, &cpu_cfg, &codec_cfg)
}
// EXPORT_SYMBOL_GPL(qcom_snd_apply_dai_tdm_slots);

#[no_mangle]
pub unsafe extern "C" fn qcom_snd_parse_of(card: *mut snd_soc_card) -> c_int {
    let dev = (*card).dev;
    let mut link: *mut snd_soc_dai_link;
    let mut args: of_phandle_args = mem::zeroed();
    let mut dlc: *mut snd_soc_dai_link_component;
    let mut ret: c_int;
    let num_links: c_int;

    ret = snd_soc_of_parse_card_name(card, c"model".as_ptr());
    if ret == 0 && (*card).name.is_null() {
        /* Deprecated, only for compatibility with old device trees */
        ret = snd_soc_of_parse_card_name(card, c"qcom,model".as_ptr());
    }
    if ret != 0 {
        dev_err(dev, c"Error parsing card name: %d\n".as_ptr(), ret);
        return ret;
    }

    if of_property_present((*dev).of_node, c"widgets".as_ptr()) {
        ret = snd_soc_of_parse_audio_simple_widgets(card, c"widgets".as_ptr());
        if ret != 0 {
            return ret;
        }
    }

    /* DAPM routes */
    if of_property_present((*dev).of_node, c"audio-routing".as_ptr()) {
        ret = snd_soc_of_parse_audio_routing(card, c"audio-routing".as_ptr());
        if ret != 0 {
            return ret;
        }
    }
    /* Deprecated, only for compatibility with old device trees */
    if of_property_present((*dev).of_node, c"qcom,audio-routing".as_ptr()) {
        ret = snd_soc_of_parse_audio_routing(card, c"qcom,audio-routing".as_ptr());
        if ret != 0 {
            return ret;
        }
    }

    ret = snd_soc_of_parse_pin_switches(card, c"pin-switches".as_ptr());
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_of_parse_aux_devs(card, c"aux-devs".as_ptr());
    if ret != 0 {
        return ret;
    }

    /* Populate links */
    num_links = of_get_available_child_count((*dev).of_node);

    /* Allocate the DAI link array */
    (*card).dai_link = devm_kcalloc(
        dev,
        num_links as usize,
        mem::size_of::<snd_soc_dai_link>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link;
    if (*card).dai_link.is_null() {
        return -ENOMEM;
    }

    (*card).num_links = num_links;
    link = (*card).dai_link;

    ret = for_each_available_child_of_node_scoped((*dev).of_node, |np| {
        dlc = devm_kcalloc(
            dev,
            2,
            mem::size_of::<snd_soc_dai_link_component>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_component;
        if dlc.is_null() {
            return -ENOMEM;
        }

        (*link).cpus = dlc.add(0);
        (*link).platforms = dlc.add(1);

        (*link).num_cpus = 1;
        (*link).num_platforms = 1;

        let rval = of_property_read_string(np, c"link-name".as_ptr(), &mut (*link).name);
        if rval != 0 {
            dev_err(dev, c"error getting codec dai_link name\n".as_ptr());
            return rval;
        }

        let cpu = of_get_child_by_name(np, c"cpu".as_ptr());
        let platform = of_get_child_by_name(np, c"platform".as_ptr());
        let codec = of_get_child_by_name(np, c"codec".as_ptr());

        if cpu.is_null() {
            dev_err(dev, c"%s: Can't find cpu DT node\n".as_ptr(), (*link).name);
            of_node_put(platform);
            of_node_put(codec);
            return -EINVAL;
        }

        let rval = snd_soc_of_get_dlc(cpu, &mut args, (*link).cpus, 0);
        if rval != 0 {
            dev_err_probe(
                dev,
                rval,
                c"%s: error getting cpu dai name\n".as_ptr(),
                (*link).name,
            );
            of_node_put(cpu);
            of_node_put(platform);
            of_node_put(codec);
            return rval;
        }

        (*link).id = args.args[0];

        if (*link).id >= LPASS_MAX_PORT {
            dev_err(
                dev,
                c"%s: Invalid cpu dai id %d\n".as_ptr(),
                (*link).name,
                (*link).id,
            );
            of_node_put(cpu);
            of_node_put(platform);
            of_node_put(codec);
            return -EINVAL;
        }

        if !platform.is_null() {
            (*(*link).platforms).of_node = of_parse_phandle(platform, c"sound-dai".as_ptr(), 0);
            if (*(*link).platforms).of_node.is_null() {
                dev_err(dev, c"%s: platform dai not found\n".as_ptr(), (*link).name);
                of_node_put(cpu);
                of_node_put(platform);
                of_node_put(codec);
                return -EINVAL;
            }
        } else {
            (*(*link).platforms).of_node = (*(*link).cpus).of_node;
        }

        if !codec.is_null() {
            let rval = snd_soc_of_get_dai_link_codecs(dev, codec, link);
            if rval < 0 {
                dev_err_probe(
                    dev,
                    rval,
                    c"%s: codec dai not found\n".as_ptr(),
                    (*link).name,
                );
                of_node_put(cpu);
                of_node_put(platform);
                of_node_put(codec);
                return rval;
            }

            if !platform.is_null() {
                /* DPCM backend */
                (*link).no_pcm = 1;
                (*link).ignore_pmdown_time = 1;
            }
        } else {
            /* DPCM frontend */
            (*link).codecs = &snd_soc_dummy_dlc;
            (*link).num_codecs = 1;
            (*link).dynamic = 1;
        }

        if !platform.is_null() || codec.is_null() {
            /* DPCM */
            (*link).ignore_suspend = 1;
            (*link).nonatomic = 1;
        }

        (*link).stream_name = (*link).name;
        link = link.add(1);

        of_node_put(cpu);
        of_node_put(platform);
        of_node_put(codec);
        0
    });
    if ret != 0 {
        return ret;
    }

    if (*card).dapm_widgets.is_null() {
        (*card).dapm_widgets = qcom_jack_snd_widgets.as_ptr();
        (*card).num_dapm_widgets = qcom_jack_snd_widgets.len() as c_int;
    }

    0
}
// EXPORT_SYMBOL_GPL(qcom_snd_parse_of);

static mut qcom_headset_jack_pins: [snd_soc_jack_pin; 2] = [
    /* Headset */
    snd_soc_jack_pin {
        pin: c"Mic Jack".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headphone Jack".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
];

#[no_mangle]
pub unsafe extern "C" fn qcom_snd_wcd_jack_setup(
    rtd: *mut snd_soc_pcm_runtime,
    jack: *mut snd_soc_jack,
    jack_setup: *mut bool,
) -> c_int {
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let card = (*rtd).card;
    let mut rval: c_int;

    if !*jack_setup {
        rval = snd_soc_card_jack_new_pins(
            card,
            c"Headset Jack".as_ptr(),
            SND_JACK_HEADSET
                | SND_JACK_LINEOUT
                | SND_JACK_MECHANICAL
                | SND_JACK_BTN_0
                | SND_JACK_BTN_1
                | SND_JACK_BTN_2
                | SND_JACK_BTN_3
                | SND_JACK_BTN_4
                | SND_JACK_BTN_5,
            jack,
            qcom_headset_jack_pins.as_mut_ptr(),
            qcom_headset_jack_pins.len() as c_uint,
        );

        if rval < 0 {
            dev_err((*card).dev, c"Unable to add Headphone Jack\n".as_ptr());
            return rval;
        }

        snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_MEDIA);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
        *jack_setup = true;
    }

    match (*cpu_dai).id {
        id if id == LPI_MI2S_RX_0
            || id == TX_CODEC_DMA_TX_0
            || id == TX_CODEC_DMA_TX_1
            || id == TX_CODEC_DMA_TX_2
            || id == TX_CODEC_DMA_TX_3 =>
        {
            rval = for_each_rtd_codec_dais(rtd, |_i, codec_dai| {
                let set_rval =
                    snd_soc_component_set_jack((*codec_dai).component, jack, ptr::null_mut());
                if set_rval != 0 && set_rval != -ENOTSUPP {
                    dev_warn((*card).dev, c"Failed to set jack: %d\n".as_ptr(), set_rval);
                    return set_rval;
                }
                0
            });
            if rval != 0 {
                return rval;
            }
        }
        _ => {}
    }

    0
}
// EXPORT_SYMBOL_GPL(qcom_snd_wcd_jack_setup);

#[no_mangle]
pub unsafe extern "C" fn qcom_snd_dp_jack_setup(
    rtd: *mut snd_soc_pcm_runtime,
    dp_jack: *mut snd_soc_jack,
    dp_pcm_id: c_int,
) -> c_int {
    let card = (*rtd).card;
    let mut jack_name: [c_char; NAME_SIZE] = [0; NAME_SIZE];
    let mut rval: c_int;

    snprintf(
        jack_name.as_mut_ptr(),
        mem::size_of_val(&jack_name),
        c"DP%d Jack".as_ptr(),
        dp_pcm_id,
    );
    rval = snd_soc_card_jack_new(card, jack_name.as_ptr(), SND_JACK_AVOUT, dp_jack);
    if rval != 0 {
        return rval;
    }

    rval = for_each_rtd_codec_dais(rtd, |_i, codec_dai| {
        let set_rval = snd_soc_component_set_jack((*codec_dai).component, dp_jack, ptr::null_mut());
        if set_rval != 0 && set_rval != -ENOTSUPP {
            dev_warn((*card).dev, c"Failed to set jack: %d\n".as_ptr(), set_rval);
            return set_rval;
        }
        0
    });
    if rval != 0 {
        return rval;
    }

    0
}
// EXPORT_SYMBOL_GPL(qcom_snd_dp_jack_setup);

// MODULE_DESCRIPTION("ASoC Qualcomm helper functions");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
