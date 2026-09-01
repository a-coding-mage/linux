// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2025 Texas Instruments Inc.

/*
 *  soc_sdw_ti_amp - Helpers to handle TI's soundwire based codecs
 */

// Dependencies from the original C includes:
// linux/device.h, linux/errno.h, linux/input.h, sound/jack.h,
// sound/soc-acpi.h, sound/soc-dai.h, sound/soc.h, sound/soc_sdw_utils.h

pub const TIAMP_SPK_VOLUME_0DB: i32 = 200;
pub const TAC5XX2_WIDGET_NAME_MAX: usize = 32;

unsafe extern "C" {
    static GFP_KERNEL: gfp_t;

    fn kasprintf(gfp: gfp_t, fmt: *const c_char, ...) -> *mut c_char;
    fn devm_kasprintf(dev: *mut device, gfp: gfp_t, fmt: *const c_char, ...) -> *mut c_char;
    fn kfree(ptr: *const c_void);
    fn strlen(s: *const c_char) -> usize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;

    fn snd_soc_limit_volume(
        card: *mut snd_soc_card,
        name: *const c_char,
        max: c_int,
    ) -> c_int;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_int,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

pub type c_char = i8;
pub type c_int = i32;
pub type c_void = core::ffi::c_void;
pub type bool = core::ffi::c_bool;
pub type gfp_t = u32;

pub const ENOMEM: c_int = 12;
pub const EINVAL: c_int = 22;
pub const ENAMETOOLONG: c_int = 36;

unsafe extern "C" {
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_MICROPHONE: c_int;
    static SND_JACK_HEADSET: c_int;
    static SND_JACK_BTN_0: c_int;
    static SND_JACK_BTN_1: c_int;
    static SND_JACK_BTN_2: c_int;
    static SND_JACK_BTN_3: c_int;
    static SND_JACK_BTN_4: c_int;
    static KEY_PLAYPAUSE: c_int;
    static KEY_VOICECOMMAND: c_int;
    static KEY_VOLUMEUP: c_int;
    static KEY_VOLUMEDOWN: c_int;
    static KEY_NEXTSONG: c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub components: *mut c_char,
}

#[repr(C)]
pub struct snd_soc_component {
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub name: *const c_char,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct asoc_sdw_codec_info {
    pub amp_num: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct asoc_sdw_mc_private {
    pub sdw_headset: snd_soc_jack,
}

// for_each_rtd_codec_dais(rtd, i, codec_dai) is supplied by sound/soc.h.
unsafe extern "C" {
    fn for_each_rtd_codec_dais_next(
        rtd: *mut snd_soc_pcm_runtime,
        i: *mut c_int,
        codec_dai: *mut *mut snd_soc_dai,
    ) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_ti_amp_initial_settings(
    card: *mut snd_soc_card,
    name_prefix: *const c_char,
) -> c_int {
    let volume_ctl_name: *mut c_char;
    let ret: c_int;

    volume_ctl_name = kasprintf(GFP_KERNEL, c"%s Speaker Volume".as_ptr(), name_prefix);
    if volume_ctl_name.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_limit_volume(card, volume_ctl_name, TIAMP_SPK_VOLUME_0DB);
    if ret != 0 {
        dev_err(
            (*card).dev,
            c"%s update failed %d\n".as_ptr(),
            volume_ctl_name,
            ret,
        );
    }

    kfree(volume_ctl_name as *const c_void);
    0
}

// EXPORT_SYMBOL_NS(asoc_sdw_ti_amp_initial_settings, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_ti_spk_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let mut widget_name = [0 as c_char; 16];
    let mut speaker = [0 as c_char; 16];
    let route = snd_soc_dapm_route {
        sink: speaker.as_ptr(),
        control: core::ptr::null(),
        source: widget_name.as_ptr(),
    };
    let mut codec_dai: *mut snd_soc_dai = core::ptr::null_mut();
    let mut prefix: *const c_char;
    let mut i: c_int = 0;
    let mut ret: c_int = 0;

    while for_each_rtd_codec_dais_next(rtd, &mut i, &mut codec_dai) {
        if strstr((*codec_dai).name, c"tas2783".as_ptr()).is_null() {
            continue;
        }

        prefix = (*(*codec_dai).component).name_prefix;
        if strncmp(prefix, c"tas2783-1".as_ptr(), strlen(c"tas2783-1".as_ptr())) == 0 {
            strscpy(speaker.as_mut_ptr(), c"Left Spk".as_ptr(), speaker.len());
        } else if strncmp(prefix, c"tas2783-2".as_ptr(), strlen(c"tas2783-2".as_ptr())) == 0 {
            strscpy(speaker.as_mut_ptr(), c"Right Spk".as_ptr(), speaker.len());
        } else if strncmp(prefix, c"tas2783-3".as_ptr(), strlen(c"tas2783-3".as_ptr())) == 0 {
            strscpy(speaker.as_mut_ptr(), c"Left Spk2".as_ptr(), speaker.len());
        } else if strncmp(prefix, c"tas2783-4".as_ptr(), strlen(c"tas2783-4".as_ptr())) == 0 {
            strscpy(speaker.as_mut_ptr(), c"Right Spk2".as_ptr(), speaker.len());
        } else {
            ret = -EINVAL;
            dev_err((*card).dev, c"unhandled prefix %s".as_ptr(), prefix);
            break;
        }

        snprintf(
            widget_name.as_mut_ptr(),
            widget_name.len(),
            c"%s SPK".as_ptr(),
            prefix,
        );
        ret = asoc_sdw_ti_amp_initial_settings(card, prefix);
        if ret != 0 {
            return ret;
        }

        ret = snd_soc_dapm_add_routes(dapm, &route, 1);
        if ret != 0 {
            return ret;
        }
    }

    ret
}

// EXPORT_SYMBOL_NS(asoc_sdw_ti_spk_rtd_init, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_ti_amp_init(
    _card: *mut snd_soc_card,
    _dai_links: *mut snd_soc_dai_link,
    info: *mut asoc_sdw_codec_info,
    playback: bool,
) -> c_int {
    if !playback {
        return 0;
    }

    (*info).amp_num += 1;

    0
}

// EXPORT_SYMBOL_NS(asoc_sdw_ti_amp_init, "SND_SOC_SDW_UTILS");

unsafe extern "C" fn asoc_sdw_ti_add_tac5xx2_routes(
    dapm: *mut snd_soc_dapm_context,
    name_prefix: *const c_char,
) -> c_int {
    let mut routes = [
        snd_soc_dapm_route {
            sink: core::ptr::null(),
            control: core::ptr::null(),
            source: core::ptr::null(),
        },
        snd_soc_dapm_route {
            sink: core::ptr::null(),
            control: core::ptr::null(),
            source: core::ptr::null(),
        },
    ];
    let mut left_widget = [0 as c_char; TAC5XX2_WIDGET_NAME_MAX];
    let mut right_widget = [0 as c_char; TAC5XX2_WIDGET_NAME_MAX];

    if strlen(name_prefix) > (TAC5XX2_WIDGET_NAME_MAX - 7) {
        return -ENAMETOOLONG;
    }

    scnprintf(
        left_widget.as_mut_ptr(),
        left_widget.len(),
        c"%s SPK_L".as_ptr(),
        name_prefix,
    );
    scnprintf(
        right_widget.as_mut_ptr(),
        right_widget.len(),
        c"%s SPK_R".as_ptr(),
        name_prefix,
    );

    routes[0] = snd_soc_dapm_route {
        sink: c"Left Spk".as_ptr(),
        control: core::ptr::null(),
        source: left_widget.as_ptr(),
    };
    routes[1] = snd_soc_dapm_route {
        sink: c"Right Spk".as_ptr(),
        control: core::ptr::null(),
        source: right_widget.as_ptr(),
    };

    snd_soc_dapm_add_routes(dapm, routes.as_ptr(), routes.len() as c_int)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_ti_tac5xx2_spk_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let mut ret: c_int;
    let mut i: c_int = 0;
    let mut codec_dai: *mut snd_soc_dai = core::ptr::null_mut();
    let prefix: *const c_char;

    while for_each_rtd_codec_dais_next(rtd, &mut i, &mut codec_dai) {
        if strstr((*codec_dai).name, c"tac5".as_ptr()).is_null()
            && strstr((*codec_dai).name, c"tas2883".as_ptr()).is_null()
        {
            continue;
        }

        prefix = (*(*codec_dai).component).name_prefix;
        if prefix.is_null() {
            dev_warn(
                (*card).dev,
                c"No name prefix found for codec DAI: %s\n".as_ptr(),
                (*codec_dai).name,
            );
            continue;
        }
        ret = asoc_sdw_ti_add_tac5xx2_routes(dapm, prefix);
        if ret != 0 {
            dev_err(
                (*card).dev,
                c"Failed to add routes for %s: %d\n".as_ptr(),
                prefix,
                ret,
            );
            return ret;
        }
    }

    dev_dbg((*card).dev, c"Added TAC5XX2 speaker routes\n".as_ptr());

    0
}

// EXPORT_SYMBOL_NS(asoc_sdw_ti_tac5xx2_spk_rtd_init, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_ti_dmic_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let component: *mut snd_soc_component;

    component = (*dai).component;

    (*card).components = devm_kasprintf(
        (*card).dev,
        GFP_KERNEL,
        c"%s mic:%s".as_ptr(),
        (*card).components,
        (*component).name_prefix,
    );
    if (*card).components.is_null() {
        return -ENOMEM;
    }

    dev_dbg((*card).dev, c"card->components: %s\n".as_ptr(), (*card).components);

    0
}

// EXPORT_SYMBOL_NS(asoc_sdw_ti_dmic_rtd_init, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub static mut ti_sdca_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: unsafe { SND_JACK_HEADPHONE },
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: unsafe { SND_JACK_MICROPHONE },
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_ti_sdca_jack_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let ctx: *mut asoc_sdw_mc_private = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let component: *mut snd_soc_component;
    let jack: *mut snd_soc_jack;
    let mut ret: c_int;

    component = (*dai).component;

    (*card).components = devm_kasprintf(
        (*card).dev,
        GFP_KERNEL,
        c"%s hs:%s".as_ptr(),
        (*card).components,
        (*component).name_prefix,
    );
    if (*card).components.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        c"Headset Jack".as_ptr(),
        SND_JACK_HEADSET
            | SND_JACK_BTN_0
            | SND_JACK_BTN_1
            | SND_JACK_BTN_2
            | SND_JACK_BTN_3
            | SND_JACK_BTN_4,
        &mut (*ctx).sdw_headset,
        ti_sdca_jack_pins.as_mut_ptr(),
        ti_sdca_jack_pins.len() as c_int,
    );
    if ret != 0 {
        dev_err((*(*rtd).card).dev, c"Jack create failed%d\n".as_ptr(), ret);
        return ret;
    }

    jack = &mut (*ctx).sdw_headset;

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_4, KEY_NEXTSONG);

    ret = snd_soc_component_set_jack(component, jack, core::ptr::null_mut());
    if ret != 0 {
        dev_err(
            (*(*rtd).card).dev,
            c"Headset Jack call-back failed: %d\n".as_ptr(),
            ret,
        );
    }

    ret
}

// EXPORT_SYMBOL_NS(asoc_sdw_ti_sdca_jack_rtd_init, "SND_SOC_SDW_UTILS");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
