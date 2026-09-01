// SPDX-License-Identifier: GPL-2.0
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

// Dependencies from the original C source:
// <linux/module.h>, <linux/pm_runtime.h>, <sound/soc.h>,
// <sound/hdaudio_ext.h>, <sound/hda_i915.h>, <sound/hda_codec.h>, "hda.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

extern "C" {
    static snd_soc_hda_codec_dai_ops: snd_soc_dai_ops;

    fn devm_kcalloc(
        dev: *mut device,
        n: usize,
        size: usize,
        flags: c_uint,
    ) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn snd_pcm_direction_name(direction: c_int) -> *const c_char;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_register_dai(
        component: *mut snd_soc_component,
        dai_drv: *mut snd_soc_dai_driver,
        legacy_dai_naming: bool,
    ) -> *mut snd_soc_dai;
    fn snd_soc_unregister_dai(dai: *mut snd_soc_dai);
    fn snd_soc_dapm_new_dai_widgets(dapm: *mut snd_soc_dapm_context, dai: *mut snd_soc_dai) -> c_int;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut hda_pcm_stream,
        capture: *mut hda_pcm_stream,
    );
    fn snd_soc_dapm_free_widget(w: *mut snd_soc_dapm_widget);
    fn snd_soc_dai_get_widget(dai: *mut snd_soc_dai, stream: c_int) -> *mut snd_soc_dapm_widget;

    fn snd_hda_codec_build_controls(codec: *mut hda_codec) -> c_int;
    fn pm_runtime_set_active(dev: *mut device);
    fn snd_hda_codec_set_power_save(codec: *mut hda_codec, delay: c_int);
    fn snd_hda_codec_register(codec: *mut hda_codec);
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn dev_to_hda_codec(dev: *mut device) -> *mut hda_codec;
    fn hda_codec_to_driver(codec: *mut hda_codec) -> *mut hda_codec_driver;
    fn atomic_read(v: *mut atomic_t) -> c_int;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn WARN_ON(condition: bool) -> bool;
    fn snd_hdac_ext_bus_get_hlink_by_addr(bus: *mut hdac_bus, addr: c_uint) -> *mut hdac_ext_link;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn hda_codec_is_display(codec: *mut hda_codec) -> bool;
    fn snd_hdac_display_power(bus: *mut hdac_bus, addr: c_uint, enable: bool);
    fn snd_hdac_ext_bus_link_get(bus: *mut hdac_bus, hlink: *mut hdac_ext_link);
    fn snd_hda_codec_device_new(
        bus: *mut hda_bus,
        card: *mut snd_card,
        addr: c_uint,
        codec: *mut hda_codec,
        snddev_managed: bool,
    ) -> c_int;
    fn snd_hda_codec_set_name(codec: *mut hda_codec, name: *const c_char) -> c_int;
    fn snd_hdac_regmap_init(hdev: *mut hdac_device) -> c_int;
    fn snd_hda_codec_parse_pcms(codec: *mut hda_codec) -> c_int;
    fn snd_hda_codec_cleanup_for_unbind(codec: *mut hda_codec);
    fn snd_hdac_ext_bus_link_put(bus: *mut hdac_bus, hlink: *mut hdac_ext_link);
    fn pm_runtime_forbid(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_set_suspended(dev: *mut device);
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *mut snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);

    fn list_empty(head: *mut list_head) -> bool;
    fn list_first_entry_hda_pcm(head: *mut list_head) -> *mut hda_pcm;
    fn list_next_entry_hda_pcm(pos: *mut hda_pcm) -> *mut hda_pcm;
    fn list_for_each_entry_hda_pcm_count(head: *mut list_head) -> c_int;
    fn list_for_each_entry_hda_pcm_first(head: *mut list_head) -> *mut hda_pcm;
    fn list_for_each_entry_hda_pcm_next(pos: *mut hda_pcm, head: *mut list_head) -> *mut hda_pcm;
    fn for_each_component_dais_safe_first(
        component: *mut snd_soc_component,
        save: *mut *mut snd_soc_dai,
    ) -> *mut snd_soc_dai;
    fn for_each_component_dais_safe_next(
        component: *mut snd_soc_component,
        dai: *mut snd_soc_dai,
        save: *mut *mut snd_soc_dai,
    ) -> *mut snd_soc_dai;
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENODEV: c_int = 19;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SND_SOC_NOPM: c_int = 0;

const CONFIG_PM: bool = cfg!(CONFIG_PM);

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}

#[repr(C)]
pub struct dev_pm_info {
    pub usage_count: atomic_t,
}

#[repr(C)]
pub struct device {
    pub power: dev_pm_info,
}

#[repr(C)]
pub struct hda_pcm_stream {
    pub substreams: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
    pub subformats: u64,
    pub maxbps: c_uint,
}

#[repr(C)]
pub struct hda_pcm {
    pub name: *const c_char,
    pub stream: [hda_pcm_stream; 2],
    pub list: list_head,
}

#[repr(C)]
pub struct hdac_device {
    pub dev: device,
    pub bus: *mut hdac_bus,
    pub addr: c_uint,
    pub registered: bool,
    pub lazy_cache: bool,
}

#[repr(C)]
pub struct hda_preset {
    pub name: *const c_char,
}

#[repr(C)]
pub struct hda_codec {
    pub core: hdac_device,
    pub bus: *mut hda_bus,
    pub preset: *mut hda_preset,
    pub pcm_list_head: list_head,
    pub jackpoll_work: delayed_work,
}

#[repr(C)]
pub struct hda_codec_ops {
    pub probe: Option<unsafe extern "C" fn(*mut hda_codec, *mut hda_preset) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut hda_codec)>,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub ops: *mut hda_codec_ops,
}

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut device,
    pub audio_component: *mut c_void,
}

#[repr(C)]
pub struct hda_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_ext_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *mut c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
    pub subformats: u64,
    pub sig_bits: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub id: c_int,
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub driver: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub idle_bias_on: bool,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

#[repr(C)]
pub struct hdac_ext_bus_ops {
    pub hdev_attach: Option<unsafe extern "C" fn(*mut hdac_device) -> c_int>,
    pub hdev_detach: Option<unsafe extern "C" fn(*mut hdac_device) -> c_int>,
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

unsafe fn hda_codec_create_dais(
    codec: *mut hda_codec,
    pcm_count: c_int,
    drivers: *mut *mut snd_soc_dai_driver,
) -> c_int {
    let dev = &mut (*codec).core.dev as *mut device;
    let mut drvs: *mut snd_soc_dai_driver;
    let mut pcm: *mut hda_pcm;
    let mut i: c_int;

    drvs = devm_kcalloc(
        dev,
        pcm_count as usize,
        core::mem::size_of::<snd_soc_dai_driver>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_driver;
    if drvs.is_null() {
        return -ENOMEM;
    }

    pcm = list_first_entry_hda_pcm(&mut (*codec).pcm_list_head);

    i = 0;
    while i < pcm_count {
        let mut stream: *mut snd_soc_pcm_stream;
        let mut dir: c_int;

        dev_info(dev, c"creating for %s %d\n".as_ptr(), (*pcm).name, i);
        (*drvs.add(i as usize)).id = i;
        (*drvs.add(i as usize)).name = (*pcm).name;
        (*drvs.add(i as usize)).ops = &snd_soc_hda_codec_dai_ops;

        dir = SNDRV_PCM_STREAM_PLAYBACK;
        stream = &mut (*drvs.add(i as usize)).playback;
        if (*pcm).stream[dir as usize].substreams == 0 {
            dev_info(dev, c"skipping playback dai for %s\n".as_ptr(), (*pcm).name);
        } else {
            (*stream).stream_name = devm_kasprintf(
                dev,
                GFP_KERNEL,
                c"%s %s".as_ptr(),
                (*pcm).name,
                snd_pcm_direction_name(dir),
            );
            if (*stream).stream_name.is_null() {
                return -ENOMEM;
            }
            (*stream).channels_min = (*pcm).stream[dir as usize].channels_min;
            (*stream).channels_max = (*pcm).stream[dir as usize].channels_max;
            (*stream).rates = (*pcm).stream[dir as usize].rates;
            (*stream).formats = (*pcm).stream[dir as usize].formats;
            (*stream).subformats = (*pcm).stream[dir as usize].subformats;
            (*stream).sig_bits = (*pcm).stream[dir as usize].maxbps;
        }

        dir = SNDRV_PCM_STREAM_CAPTURE;
        stream = &mut (*drvs.add(i as usize)).capture;
        if (*pcm).stream[dir as usize].substreams == 0 {
            dev_info(dev, c"skipping capture dai for %s\n".as_ptr(), (*pcm).name);
            pcm = list_next_entry_hda_pcm(pcm);
            i += 1;
            continue;
        }

        (*stream).stream_name = devm_kasprintf(
            dev,
            GFP_KERNEL,
            c"%s %s".as_ptr(),
            (*pcm).name,
            snd_pcm_direction_name(dir),
        );
        if (*stream).stream_name.is_null() {
            return -ENOMEM;
        }
        (*stream).channels_min = (*pcm).stream[dir as usize].channels_min;
        (*stream).channels_max = (*pcm).stream[dir as usize].channels_max;
        (*stream).rates = (*pcm).stream[dir as usize].rates;
        (*stream).formats = (*pcm).stream[dir as usize].formats;
        (*stream).subformats = (*pcm).stream[dir as usize].subformats;
        (*stream).sig_bits = (*pcm).stream[dir as usize].maxbps;

        pcm = list_next_entry_hda_pcm(pcm);
        i += 1;
    }

    *drivers = drvs;
    0
}

unsafe fn hda_codec_register_dais(
    codec: *mut hda_codec,
    component: *mut snd_soc_component,
) -> c_int {
    let mut drvs: *mut snd_soc_dai_driver = ptr::null_mut();
    let dapm: *mut snd_soc_dapm_context;
    let mut pcm: *mut hda_pcm;
    let mut ret: c_int;
    let mut pcm_count: c_int = 0;

    if list_empty(&mut (*codec).pcm_list_head) {
        return -EINVAL;
    }
    pcm_count = list_for_each_entry_hda_pcm_count(&mut (*codec).pcm_list_head);

    ret = hda_codec_create_dais(codec, pcm_count, &mut drvs);
    if ret < 0 {
        return ret;
    }

    dapm = snd_soc_component_to_dapm(component);

    pcm = list_for_each_entry_hda_pcm_first(&mut (*codec).pcm_list_head);
    while !pcm.is_null() {
        let dai: *mut snd_soc_dai;

        dai = snd_soc_register_dai(component, drvs, false);
        if dai.is_null() {
            dev_err((*component).dev, c"register dai for %s failed\n".as_ptr(), (*pcm).name);
            return -EINVAL;
        }

        ret = snd_soc_dapm_new_dai_widgets(dapm, dai);
        if ret < 0 {
            dev_err((*component).dev, c"create widgets failed: %d\n".as_ptr(), ret);
            snd_soc_unregister_dai(dai);
            return ret;
        }

        snd_soc_dai_init_dma_data(dai, &mut (*pcm).stream[0], &mut (*pcm).stream[1]);
        drvs = drvs.add(1);
        pcm = list_for_each_entry_hda_pcm_next(pcm, &mut (*codec).pcm_list_head);
    }

    0
}

unsafe fn hda_codec_unregister_dais(
    codec: *mut hda_codec,
    component: *mut snd_soc_component,
) {
    let mut save: *mut snd_soc_dai = ptr::null_mut();
    let mut dai: *mut snd_soc_dai;

    dai = for_each_component_dais_safe_first(component, &mut save);
    while !dai.is_null() {
        let mut pcm: *mut hda_pcm;

        pcm = list_for_each_entry_hda_pcm_first(&mut (*codec).pcm_list_head);
        while !pcm.is_null() {
            if strcmp((*(*dai).driver).name, (*pcm).name) != 0 {
                pcm = list_for_each_entry_hda_pcm_next(pcm, &mut (*codec).pcm_list_head);
                continue;
            }

            let mut stream: c_int = 0;
            while stream < 2 {
                snd_soc_dapm_free_widget(snd_soc_dai_get_widget(dai, stream));
                stream += 1;
            }

            snd_soc_unregister_dai(dai);
            break;
        }
        dai = for_each_component_dais_safe_next(component, dai, &mut save);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hda_codec_probe_complete(codec: *mut hda_codec) -> c_int {
    let hdev = &mut (*codec).core as *mut hdac_device;
    let bus = (*hdev).bus;
    let mut ret: c_int;

    ret = snd_hda_codec_build_controls(codec);
    if ret < 0 {
        dev_err(&mut (*hdev).dev, c"unable to create controls %d\n".as_ptr(), ret);
        return ret;
    }

    /* Bus suspended codecs as it does not manage their pm */
    pm_runtime_set_active(&mut (*hdev).dev);
    /* rpm was forbidden in snd_hda_codec_device_new() */
    snd_hda_codec_set_power_save(codec, 2000);
    snd_hda_codec_register(codec);

    /* Complement pm_runtime_get_sync(bus) in probe */
    pm_runtime_put_autosuspend((*bus).dev);

    ret
}
// EXPORT_SYMBOL_GPL(hda_codec_probe_complete);

/* Expects codec with usage_count=1 and status=suspended */
unsafe extern "C" fn hda_codec_probe(component: *mut snd_soc_component) -> c_int {
    let codec = dev_to_hda_codec((*component).dev);
    let driver = hda_codec_to_driver(codec);
    let hdev = &mut (*codec).core as *mut hdac_device;
    let bus = (*hdev).bus;
    let mut hlink: *mut hdac_ext_link;
    let mut ret: c_int;

    if CONFIG_PM {
        WARN_ON(
            atomic_read(&mut (*hdev).dev.power.usage_count) != 1
                || !pm_runtime_status_suspended(&mut (*hdev).dev),
        );
    }

    hlink = snd_hdac_ext_bus_get_hlink_by_addr(bus, (*hdev).addr);
    if hlink.is_null() {
        dev_err(&mut (*hdev).dev, c"hdac link not found\n".as_ptr());
        return -EIO;
    }

    pm_runtime_get_sync((*bus).dev);
    if hda_codec_is_display(codec) {
        snd_hdac_display_power(bus, (*hdev).addr, true);
    }
    snd_hdac_ext_bus_link_get(bus, hlink);

    ret = snd_hda_codec_device_new(
        (*codec).bus,
        (*(*component).card).snd_card,
        (*hdev).addr,
        codec,
        false,
    );
    if ret < 0 {
        dev_err(&mut (*hdev).dev, c"codec create failed: %d\n".as_ptr(), ret);
        goto_device_new_err(codec, hdev, bus, hlink, ret);
        return ret;
    }

    ret = snd_hda_codec_set_name(codec, (*(*codec).preset).name);
    if ret < 0 {
        dev_err(
            &mut (*hdev).dev,
            c"set name: %s failed: %d\n".as_ptr(),
            (*(*codec).preset).name,
            ret,
        );
        goto_err(codec, hdev, bus, hlink, ret);
        return ret;
    }

    ret = snd_hdac_regmap_init(&mut (*codec).core);
    if ret < 0 {
        dev_err(&mut (*hdev).dev, c"regmap init failed: %d\n".as_ptr(), ret);
        goto_err(codec, hdev, bus, hlink, ret);
        return ret;
    }

    if WARN_ON((*driver).ops.is_null() || (*(*driver).ops).probe.is_none()) {
        ret = -EINVAL;
        goto_err(codec, hdev, bus, hlink, ret);
        return ret;
    }

    ret = ((*(*driver).ops).probe.unwrap())(codec, (*codec).preset);
    if ret < 0 {
        dev_err(&mut (*hdev).dev, c"codec init failed: %d\n".as_ptr(), ret);
        goto_err(codec, hdev, bus, hlink, ret);
        return ret;
    }

    ret = snd_hda_codec_parse_pcms(codec);
    if ret < 0 {
        dev_err(&mut (*hdev).dev, c"unable to map pcms to dai: %d\n".as_ptr(), ret);
        goto_parse_pcms_err(codec, driver, hdev, bus, hlink, ret);
        return ret;
    }

    ret = hda_codec_register_dais(codec, component);
    if ret < 0 {
        dev_err(&mut (*hdev).dev, c"update dais failed: %d\n".as_ptr(), ret);
        goto_parse_pcms_err(codec, driver, hdev, bus, hlink, ret);
        return ret;
    }

    if !hda_codec_is_display(codec) {
        ret = hda_codec_probe_complete(codec);
        if ret < 0 {
            hda_codec_unregister_dais(codec, component);
            goto_parse_pcms_err(codec, driver, hdev, bus, hlink, ret);
            return ret;
        }
    }

    (*codec).core.lazy_cache = true;

    0
}

unsafe fn goto_parse_pcms_err(
    codec: *mut hda_codec,
    driver: *mut hda_codec_driver,
    hdev: *mut hdac_device,
    bus: *mut hdac_bus,
    hlink: *mut hdac_ext_link,
    ret: c_int,
) {
    if !(*driver).ops.is_null() {
        if let Some(remove) = (*(*driver).ops).remove {
            remove(codec);
        }
    }
    goto_err(codec, hdev, bus, hlink, ret);
}

unsafe fn goto_err(
    codec: *mut hda_codec,
    hdev: *mut hdac_device,
    bus: *mut hdac_bus,
    hlink: *mut hdac_ext_link,
    ret: c_int,
) {
    snd_hda_codec_cleanup_for_unbind(codec);
    goto_device_new_err(codec, hdev, bus, hlink, ret);
}

unsafe fn goto_device_new_err(
    codec: *mut hda_codec,
    hdev: *mut hdac_device,
    bus: *mut hdac_bus,
    hlink: *mut hdac_ext_link,
    _ret: c_int,
) {
    if hda_codec_is_display(codec) {
        snd_hdac_display_power(bus, (*hdev).addr, false);
    }

    snd_hdac_ext_bus_link_put(bus, hlink);

    pm_runtime_put_autosuspend((*bus).dev);
}

/* Leaves codec with usage_count=1 and status=suspended */
unsafe extern "C" fn hda_codec_remove(component: *mut snd_soc_component) {
    let codec = dev_to_hda_codec((*component).dev);
    let driver = hda_codec_to_driver(codec);
    let hdev = &mut (*codec).core as *mut hdac_device;
    let bus = (*hdev).bus;
    let mut hlink: *mut hdac_ext_link;
    let was_registered = (*codec).core.registered;

    /* Don't allow any more runtime suspends */
    pm_runtime_forbid(&mut (*hdev).dev);

    hda_codec_unregister_dais(codec, component);

    if !(*driver).ops.is_null() {
        if let Some(remove) = (*(*driver).ops).remove {
            remove(codec);
        }
    }

    snd_hda_codec_cleanup_for_unbind(codec);
    pm_runtime_put_noidle(&mut (*hdev).dev);
    /* snd_hdac_device_exit() is only called on bus remove */
    pm_runtime_set_suspended(&mut (*hdev).dev);

    if hda_codec_is_display(codec) {
        snd_hdac_display_power(bus, (*hdev).addr, false);
    }

    hlink = snd_hdac_ext_bus_get_hlink_by_addr(bus, (*hdev).addr);
    if !hlink.is_null() {
        snd_hdac_ext_bus_link_put(bus, hlink);
    }
    /*
     * HDMI card's hda_codec_probe_complete() (see late_probe()) may
     * not be called due to early error, leaving bus uc unbalanced
     */
    if !was_registered {
        pm_runtime_put_autosuspend((*bus).dev);
    }

    if CONFIG_PM {
        WARN_ON(
            atomic_read(&mut (*hdev).dev.power.usage_count) != 1
                || !pm_runtime_status_suspended(&mut (*hdev).dev),
        );
    }
}

static_strs! {
    AIF1TX = "AIF1TX";
    AIF2TX = "AIF2TX";
    AIF3TX = "AIF3TX";
    AIF1RX = "AIF1RX";
    AIF2RX = "AIF2RX";
    AIF3RX = "AIF3RX";
    ANALOG_CODEC_PLAYBACK = "Analog Codec Playback";
    DIGITAL_CODEC_PLAYBACK = "Digital Codec Playback";
    ALT_ANALOG_CODEC_PLAYBACK = "Alt Analog Codec Playback";
    ANALOG_CODEC_CAPTURE = "Analog Codec Capture";
    DIGITAL_CODEC_CAPTURE = "Digital Codec Capture";
    ALT_ANALOG_CODEC_CAPTURE = "Alt Analog Codec Capture";
    CODEC_INPUT_PIN1 = "Codec Input Pin1";
    CODEC_INPUT_PIN2 = "Codec Input Pin2";
    CODEC_INPUT_PIN3 = "Codec Input Pin3";
    CODEC_OUTPUT_PIN1 = "Codec Output Pin1";
    CODEC_OUTPUT_PIN2 = "Codec Output Pin2";
    CODEC_OUTPUT_PIN3 = "Codec Output Pin3";
    CODEC_PROBING_DAI = "codec-probing-DAI";
    HDA_DESCRIPTION = "HD-Audio codec driver";
    HDA_AUTHOR = "Cezary Rojewski <cezary.rojewski@intel.com>";
    HDA_LICENSE = "GPL";
}

#[link_section = ".rodata"]
static hda_dapm_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route {
        sink: AIF1TX.as_ptr(),
        control: ptr::null(),
        source: CODEC_INPUT_PIN1.as_ptr(),
    },
    snd_soc_dapm_route {
        sink: AIF2TX.as_ptr(),
        control: ptr::null(),
        source: CODEC_INPUT_PIN2.as_ptr(),
    },
    snd_soc_dapm_route {
        sink: AIF3TX.as_ptr(),
        control: ptr::null(),
        source: CODEC_INPUT_PIN3.as_ptr(),
    },
    snd_soc_dapm_route {
        sink: CODEC_OUTPUT_PIN1.as_ptr(),
        control: ptr::null(),
        source: AIF1RX.as_ptr(),
    },
    snd_soc_dapm_route {
        sink: CODEC_OUTPUT_PIN2.as_ptr(),
        control: ptr::null(),
        source: AIF2RX.as_ptr(),
    },
    snd_soc_dapm_route {
        sink: CODEC_OUTPUT_PIN3.as_ptr(),
        control: ptr::null(),
        source: AIF3RX.as_ptr(),
    },
];

static hda_dapm_widgets: [snd_soc_dapm_widget; 12] = [
    /* Audio Interface */
    SND_SOC_DAPM_AIF_IN!(AIF1RX, ANALOG_CODEC_PLAYBACK, 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(AIF2RX, DIGITAL_CODEC_PLAYBACK, 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(AIF3RX, ALT_ANALOG_CODEC_PLAYBACK, 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!(AIF1TX, ANALOG_CODEC_CAPTURE, 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!(AIF2TX, DIGITAL_CODEC_CAPTURE, 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!(AIF3TX, ALT_ANALOG_CODEC_CAPTURE, 0, SND_SOC_NOPM, 0, 0),

    /* Input Pins */
    SND_SOC_DAPM_INPUT!(CODEC_INPUT_PIN1),
    SND_SOC_DAPM_INPUT!(CODEC_INPUT_PIN2),
    SND_SOC_DAPM_INPUT!(CODEC_INPUT_PIN3),

    /* Output Pins */
    SND_SOC_DAPM_OUTPUT!(CODEC_OUTPUT_PIN1),
    SND_SOC_DAPM_OUTPUT!(CODEC_OUTPUT_PIN2),
    SND_SOC_DAPM_OUTPUT!(CODEC_OUTPUT_PIN3),
];

static mut card_binder_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    id: -1,
    name: CODEC_PROBING_DAI.as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: ptr::null_mut(),
        channels_min: 0,
        channels_max: 0,
        rates: 0,
        formats: 0,
        subformats: 0,
        sig_bits: 0,
    },
    capture: snd_soc_pcm_stream {
        stream_name: ptr::null_mut(),
        channels_min: 0,
        channels_max: 0,
        rates: 0,
        formats: 0,
        subformats: 0,
        sig_bits: 0,
    },
    ops: ptr::null(),
};

unsafe extern "C" fn hda_hdev_attach(hdev: *mut hdac_device) -> c_int {
    let codec = dev_to_hda_codec(&mut (*hdev).dev);
    let mut comp_drv: *mut snd_soc_component_driver;

    if hda_codec_is_display(codec) && (*(*hdev).bus).audio_component.is_null() {
        dev_dbg(
            &mut (*hdev).dev,
            c"no i915, skip registration for 0x%08x\n".as_ptr(),
            (*hdev).addr,
        );
        return -ENODEV;
    }

    comp_drv = devm_kzalloc(
        &mut (*hdev).dev,
        core::mem::size_of::<snd_soc_component_driver>(),
        GFP_KERNEL,
    ) as *mut snd_soc_component_driver;
    if comp_drv.is_null() {
        return -ENOMEM;
    }

    /*
     * It's save to rely on dev_name() rather than a copy as component
     * driver's lifetime is directly tied to hda codec one
     */
    (*comp_drv).name = dev_name(&mut (*hdev).dev);
    (*comp_drv).probe = Some(hda_codec_probe);
    (*comp_drv).remove = Some(hda_codec_remove);
    (*comp_drv).idle_bias_on = false;
    if !hda_codec_is_display(codec) {
        (*comp_drv).dapm_widgets = hda_dapm_widgets.as_ptr();
        (*comp_drv).num_dapm_widgets = hda_dapm_widgets.len() as c_uint;
        (*comp_drv).dapm_routes = hda_dapm_routes.as_ptr();
        (*comp_drv).num_dapm_routes = hda_dapm_routes.len() as c_uint;
    }

    snd_soc_register_component(&mut (*hdev).dev, comp_drv, &mut card_binder_dai, 1)
}

unsafe extern "C" fn hda_hdev_detach(hdev: *mut hdac_device) -> c_int {
    let codec = dev_to_hda_codec(&mut (*hdev).dev);

    if (*codec).core.registered {
        cancel_delayed_work_sync(&mut (*codec).jackpoll_work);
    }

    snd_soc_unregister_component(&mut (*hdev).dev);

    0
}

#[no_mangle]
pub static soc_hda_ext_bus_ops: hdac_ext_bus_ops = hdac_ext_bus_ops {
    hdev_attach: Some(hda_hdev_attach),
    hdev_detach: Some(hda_hdev_detach),
};
// EXPORT_SYMBOL_GPL(soc_hda_ext_bus_ops);

// MODULE_DESCRIPTION("HD-Audio codec driver");
// MODULE_AUTHOR("Cezary Rojewski <cezary.rojewski@intel.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
