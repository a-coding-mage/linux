// SPDX-License-Identifier: GPL-2.0-only
/*
 * ams-delta.c  --  SoC audio for Amstrad E3 (Delta) videophone
 *
 * Copyright (C) 2009 Janusz Krzysztofik <jkrzyszt@tis.icnet.pl>
 *
 * Initially based on sound/soc/omap/osk5912.x
 * Copyright (C) 2008 Mistral Solutions
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type bool_ = bool;
type u8 = u8;
type size_t = usize;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    private_value: c_ulong,
}

#[repr(C)]
pub struct snd_soc_card {
    name: *const c_char,
    owner: *mut c_void,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_enum {
    items: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    item: [c_uint; 4],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    pins: list_head,
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct snd_soc_jack_gpio {
    name: *const c_char,
    report: c_int,
    invert: c_int,
    debounce_time: c_int,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_int,
    invert: c_int,
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cx20442_codec {
    ready: bool_,
    component: *mut snd_soc_component,
}

#[repr(C)]
pub struct tty_struct {
    disc_data: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_component {
    card: *mut snd_soc_card,
    dev: *mut device,
}

#[repr(C)]
pub struct tty_ldisc_ops {
    name: *const c_char,
    num: c_int,
    owner: *mut c_void,
    open: Option<unsafe extern "C" fn(*mut tty_struct) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut tty_struct)>,
    hangup: Option<unsafe extern "C" fn(*mut tty_struct)>,
    receive_buf: Option<unsafe extern "C" fn(*mut tty_struct, *const u8, *const u8, size_t)>,
    write_wakeup: Option<unsafe extern "C" fn(*mut tty_struct)>,
}

#[repr(C)]
pub struct snd_soc_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    no_capture_mute: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai {
    driver: *mut snd_soc_dai_driver,
    component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_pcm_substream {
    stream: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    name: *const c_char,
    stream_name: *const c_char,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    ops: *mut snd_soc_ops,
    dai_fmt: c_uint,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
}

const EUNATCH: c_int = 49;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const GPIOD_OUT_HIGH: c_int = 1;
const SND_JACK_HEADSET: c_int = 0x0001 | 0x0002;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const N_V253: c_int = 19;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0x0003;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0x3000;
const DRV_NAME: *const c_char = b"ams-delta-audio\0".as_ptr() as *const c_char;

const AMS_DELTA_MOUTHPIECE: c_int = 0;
const AMS_DELTA_EARPIECE: c_int = 1;
const AMS_DELTA_MICROPHONE: c_int = 2;
const AMS_DELTA_SPEAKER: c_int = 3;
const AMS_DELTA_AGC: c_int = 4;

const AMS_DELTA_MIXED: u16 =
    ((1u16 << AMS_DELTA_EARPIECE) | (1u16 << AMS_DELTA_MICROPHONE));
const AMS_DELTA_HANDSET: u16 =
    ((1u16 << AMS_DELTA_MOUTHPIECE) | (1u16 << AMS_DELTA_EARPIECE));
const AMS_DELTA_HANDSFREE: u16 =
    ((1u16 << AMS_DELTA_MICROPHONE) | (1u16 << AMS_DELTA_SPEAKER));
const AMS_DELTA_SPEAKERPHONE: u16 = AMS_DELTA_HANDSFREE | (1u16 << AMS_DELTA_AGC);

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static mut jiffies: c_ulong;
    static mut v253_ops: tty_ldisc_ops;

    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_card;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_get_pin_status(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_enable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync_unlocked(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_jack_add_gpiods(
        dev: *mut device,
        jack: *mut snd_soc_jack,
        count: c_int,
        gpios: *mut snd_soc_jack_gpio,
    ) -> c_int;
    fn snd_soc_jack_add_pins(
        jack: *mut snd_soc_jack,
        count: c_int,
        pins: *mut snd_soc_jack_pin,
    ) -> c_int;
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn WARN_ON(condition: bool_) -> bool_;
    fn timer_setup(timer: *mut timer_list, callback: unsafe extern "C" fn(*mut timer_list), flags: c_uint);
    fn timer_delete_sync(timer: *mut timer_list) -> c_int;
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_bh(lock: *mut spinlock_t);
    fn spin_unlock_bh(lock: *mut spinlock_t);
    fn tty_register_ldisc(ops: *mut tty_ldisc_ops) -> c_int;
    fn tty_unregister_ldisc(ops: *mut tty_ldisc_ops);
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_register_card(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_unregister_card(card: *mut snd_soc_card);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut snd_soc_card;
}

unsafe fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool_ {
    event != 0
}

static mut handset_mute: *mut gpio_desc = core::ptr::null_mut();
static mut handsfree_mute: *mut gpio_desc = core::ptr::null_mut();

unsafe extern "C" fn ams_delta_event_handset(
    _w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    gpiod_set_value_cansleep(handset_mute, (!SND_SOC_DAPM_EVENT_ON(event)) as c_int);
    0
}

unsafe extern "C" fn ams_delta_event_handsfree(
    _w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    gpiod_set_value_cansleep(handsfree_mute, (!SND_SOC_DAPM_EVENT_ON(event)) as c_int);
    0
}

/* Board specific DAPM widgets */
/* SND_SOC_DAPM_MIC/HP/SPK macro initializers require external ASoC definitions. */
static ams_delta_dapm_widgets: [snd_soc_dapm_widget; 4] = unsafe { core::mem::zeroed() };

/* How they are connected to codec pins */
static ams_delta_audio_map: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: b"TELIN\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"Mouthpiece\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Earpiece\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"TELOUT\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"MIC\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"Microphone\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Speaker\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"SPKOUT\0".as_ptr() as *const c_char,
    },
];

/*
 * Controls, functional after the modem line discipline is activated.
 */

/* Virtual switch: audio input/output constellations */
static ams_delta_audio_mode: [*const c_char; 4] = [
    b"Mixed\0".as_ptr() as *const c_char,
    b"Handset\0".as_ptr() as *const c_char,
    b"Handsfree\0".as_ptr() as *const c_char,
    b"Speakerphone\0".as_ptr() as *const c_char,
];

/* Selection <-> pin translation */
static ams_delta_audio_mode_pins: [u16; 4] = [
    AMS_DELTA_MIXED,
    AMS_DELTA_HANDSET,
    AMS_DELTA_HANDSFREE,
    AMS_DELTA_SPEAKERPHONE,
];

static mut ams_delta_audio_agc: u16 = 0;

/*
 * Used for passing a codec structure pointer
 * from the board initialization code to the tty line discipline.
 */
static mut cx20442_codec: cx20442_codec = cx20442_codec {
    ready: false,
    component: core::ptr::null_mut(),
};

unsafe extern "C" fn ams_delta_set_audio_mode(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let card: *mut snd_soc_card = snd_kcontrol_chip(kcontrol);
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let control: *mut soc_enum = (*kcontrol).private_value as *mut soc_enum;
    let pins: u16;
    let mut pin: c_int;
    let mut changed: c_int = 0;

    /* Refuse any mode changes if we are not able to control the codec. */
    if !cx20442_codec.ready {
        return -EUNATCH;
    }

    if (*ucontrol).value.enumerated.item[0] >= (*control).items {
        return -EINVAL;
    }

    snd_soc_dapm_mutex_lock(dapm);

    /* Translate selection to bitmap */
    pins = ams_delta_audio_mode_pins[(*ucontrol).value.enumerated.item[0] as usize];

    /* Setup pins after corresponding bits if changed */
    pin = ((pins & (1u16 << AMS_DELTA_MOUTHPIECE)) != 0) as c_int;

    if pin != snd_soc_dapm_get_pin_status(dapm, b"Mouthpiece\0".as_ptr() as *const c_char) {
        changed = 1;
        if pin != 0 {
            snd_soc_dapm_enable_pin_unlocked(dapm, b"Mouthpiece\0".as_ptr() as *const c_char);
        } else {
            snd_soc_dapm_disable_pin_unlocked(dapm, b"Mouthpiece\0".as_ptr() as *const c_char);
        }
    }
    pin = ((pins & (1u16 << AMS_DELTA_EARPIECE)) != 0) as c_int;
    if pin != snd_soc_dapm_get_pin_status(dapm, b"Earpiece\0".as_ptr() as *const c_char) {
        changed = 1;
        if pin != 0 {
            snd_soc_dapm_enable_pin_unlocked(dapm, b"Earpiece\0".as_ptr() as *const c_char);
        } else {
            snd_soc_dapm_disable_pin_unlocked(dapm, b"Earpiece\0".as_ptr() as *const c_char);
        }
    }
    pin = ((pins & (1u16 << AMS_DELTA_MICROPHONE)) != 0) as c_int;
    if pin != snd_soc_dapm_get_pin_status(dapm, b"Microphone\0".as_ptr() as *const c_char) {
        changed = 1;
        if pin != 0 {
            snd_soc_dapm_enable_pin_unlocked(dapm, b"Microphone\0".as_ptr() as *const c_char);
        } else {
            snd_soc_dapm_disable_pin_unlocked(dapm, b"Microphone\0".as_ptr() as *const c_char);
        }
    }
    pin = ((pins & (1u16 << AMS_DELTA_SPEAKER)) != 0) as c_int;
    if pin != snd_soc_dapm_get_pin_status(dapm, b"Speaker\0".as_ptr() as *const c_char) {
        changed = 1;
        if pin != 0 {
            snd_soc_dapm_enable_pin_unlocked(dapm, b"Speaker\0".as_ptr() as *const c_char);
        } else {
            snd_soc_dapm_disable_pin_unlocked(dapm, b"Speaker\0".as_ptr() as *const c_char);
        }
    }
    pin = ((pins & (1u16 << AMS_DELTA_AGC)) != 0) as c_int;
    if pin != ams_delta_audio_agc as c_int {
        ams_delta_audio_agc = pin as u16;
        changed = 1;
        if pin != 0 {
            snd_soc_dapm_enable_pin_unlocked(dapm, b"AGCIN\0".as_ptr() as *const c_char);
        } else {
            snd_soc_dapm_disable_pin_unlocked(dapm, b"AGCIN\0".as_ptr() as *const c_char);
        }
    }

    if changed != 0 {
        snd_soc_dapm_sync_unlocked(dapm);
    }

    snd_soc_dapm_mutex_unlock(dapm);

    changed
}

unsafe extern "C" fn ams_delta_get_audio_mode(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let card: *mut snd_soc_card = snd_kcontrol_chip(kcontrol);
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let mut pins: u16;
    let mut mode: u16;

    pins = ((snd_soc_dapm_get_pin_status(dapm, b"Mouthpiece\0".as_ptr() as *const c_char)
        << AMS_DELTA_MOUTHPIECE)
        | (snd_soc_dapm_get_pin_status(dapm, b"Earpiece\0".as_ptr() as *const c_char)
            << AMS_DELTA_EARPIECE)) as u16;
    if pins != 0 {
        pins |= (snd_soc_dapm_get_pin_status(dapm, b"Microphone\0".as_ptr() as *const c_char)
            << AMS_DELTA_MICROPHONE) as u16;
    } else {
        pins = ((snd_soc_dapm_get_pin_status(dapm, b"Microphone\0".as_ptr() as *const c_char)
            << AMS_DELTA_MICROPHONE)
            | (snd_soc_dapm_get_pin_status(dapm, b"Speaker\0".as_ptr() as *const c_char)
                << AMS_DELTA_SPEAKER)
            | ((ams_delta_audio_agc as c_int) << AMS_DELTA_AGC)) as u16;
    }

    mode = 0;
    while (mode as usize) < ams_delta_audio_mode.len() {
        if pins == ams_delta_audio_mode_pins[mode as usize] {
            break;
        }
        mode += 1;
    }

    if (mode as usize) >= ams_delta_audio_mode.len() {
        return -EINVAL;
    }

    (*ucontrol).value.enumerated.item[0] = mode as c_uint;

    0
}

/* SOC_ENUM_SINGLE_EXT_DECL and SOC_ENUM_EXT require external ASoC macro expansion. */
static mut ams_delta_audio_enum: soc_enum = soc_enum { items: 4 };
static ams_delta_audio_controls: [snd_kcontrol_new; 1] = unsafe { core::mem::zeroed() };

/* Hook switch */
static mut ams_delta_hook_switch: snd_soc_jack = snd_soc_jack {
    pins: list_head {
        next: core::ptr::null_mut(),
        prev: core::ptr::null_mut(),
    },
};
static mut ams_delta_hook_switch_gpios: [snd_soc_jack_gpio; 1] = [snd_soc_jack_gpio {
    name: b"hook_switch\0".as_ptr() as *const c_char,
    report: SND_JACK_HEADSET,
    invert: 1,
    debounce_time: 150,
}];

/* After we are able to control the codec over the modem,
 * the hook switch can be used for dynamic DAPM reconfiguration. */
static mut ams_delta_hook_switch_pins: [snd_soc_jack_pin; 4] = [
    /* Handset */
    snd_soc_jack_pin {
        pin: b"Mouthpiece\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
        invert: 0,
    },
    snd_soc_jack_pin {
        pin: b"Earpiece\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
        invert: 0,
    },
    /* Handsfree */
    snd_soc_jack_pin {
        pin: b"Microphone\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
        invert: 1,
    },
    snd_soc_jack_pin {
        pin: b"Speaker\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
        invert: 1,
    },
];

/*
 * Modem line discipline, required for making above controls functional.
 * Activated from userspace with ldattach, possibly invoked from udev rule.
 */

/* To actually apply any modem controlled configuration changes to the codec,
 * we must connect codec DAI pins to the modem for a moment.  Be careful not
 * to interfere with our digital mute function that shares the same hardware. */
static mut cx81801_timer: timer_list = timer_list { _private: [] };
static mut cx81801_cmd_pending: bool_ = false;
static mut ams_delta_muted: bool_ = true;
static mut ams_delta_lock: spinlock_t = spinlock_t { _private: [] };
static mut gpiod_modem_codec: *mut gpio_desc = core::ptr::null_mut();

unsafe extern "C" fn cx81801_timeout(_unused: *mut timer_list) {
    let muted: c_int;

    spin_lock(&mut ams_delta_lock);
    cx81801_cmd_pending = false;
    muted = ams_delta_muted as c_int;
    spin_unlock(&mut ams_delta_lock);

    /* Reconnect the codec DAI back from the modem to the CPU DAI
     * only if digital mute still off */
    if muted == 0 {
        gpiod_set_value(gpiod_modem_codec, 0);
    }
}

/* Line discipline .open() */
unsafe extern "C" fn cx81801_open(tty: *mut tty_struct) -> c_int {
    let ret: c_int;

    if cx20442_codec.component.is_null() {
        return -ENODEV;
    }

    /*
     * Pass the codec structure pointer for use by other ldisc callbacks,
     * both the card and the codec specific parts.
     */
    (*tty).disc_data = &mut cx20442_codec as *mut cx20442_codec as *mut c_void;

    ret = v253_ops.open.unwrap()(tty);

    if ret < 0 {
        (*tty).disc_data = core::ptr::null_mut();
    }

    ret
}

/* Line discipline .close() */
unsafe extern "C" fn cx81801_close(tty: *mut tty_struct) {
    let component: *mut snd_soc_component = cx20442_codec.component;
    let dapm: *mut snd_soc_dapm_context;

    if WARN_ON((*tty).disc_data != (&mut cx20442_codec as *mut cx20442_codec as *mut c_void)) {
        return;
    }

    timer_delete_sync(&mut cx81801_timer);

    /* Prevent the hook switch from further changing the DAPM pins */
    INIT_LIST_HEAD(&mut ams_delta_hook_switch.pins);

    if component.is_null() {
        return;
    }

    v253_ops.close.unwrap()(tty);

    dapm = snd_soc_card_to_dapm((*component).card);

    /* Revert back to default audio input/output constellation */
    snd_soc_dapm_mutex_lock(dapm);

    snd_soc_dapm_disable_pin_unlocked(dapm, b"Mouthpiece\0".as_ptr() as *const c_char);
    snd_soc_dapm_enable_pin_unlocked(dapm, b"Earpiece\0".as_ptr() as *const c_char);
    snd_soc_dapm_enable_pin_unlocked(dapm, b"Microphone\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin_unlocked(dapm, b"Speaker\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin_unlocked(dapm, b"AGCIN\0".as_ptr() as *const c_char);

    snd_soc_dapm_sync_unlocked(dapm);

    snd_soc_dapm_mutex_unlock(dapm);
}

/* Line discipline .hangup() */
unsafe extern "C" fn cx81801_hangup(tty: *mut tty_struct) {
    cx81801_close(tty);
}

/* Line discipline .receive_buf() */
unsafe extern "C" fn cx81801_receive(
    tty: *mut tty_struct,
    cp: *const u8,
    fp: *const u8,
    count: size_t,
) {
    let component: *mut snd_soc_component = cx20442_codec.component;
    let mut c: *const c_uchar = core::ptr::null();
    let apply: c_int;
    let mut ret: c_int;

    if WARN_ON((*tty).disc_data != (&mut cx20442_codec as *mut cx20442_codec as *mut c_void)) {
        return;
    }

    if !cx20442_codec.ready {
        /* First modem response, complete setup procedure */

        /* Initialize timer used for config pulse generation */
        timer_setup(&mut cx81801_timer, cx81801_timeout, 0);

        v253_ops.receive_buf.unwrap()(tty, cp, fp, count);

        /* Link hook switch to DAPM pins */
        ret = snd_soc_jack_add_pins(
            &mut ams_delta_hook_switch,
            ams_delta_hook_switch_pins.len() as c_int,
            ams_delta_hook_switch_pins.as_mut_ptr(),
        );
        if ret != 0 {
            dev_warn(
                (*component).dev,
                b"Failed to link hook switch to DAPM pins, will continue with hook switch unlinked.\n\0"
                    .as_ptr() as *const c_char,
            );
        }

        return;
    }

    v253_ops.receive_buf.unwrap()(tty, cp, fp, count);

    c = cp.add(count - 1) as *const c_uchar;
    while c >= cp as *const c_uchar {
        if *c != b'\r' {
            c = c.sub(1);
            continue;
        }
        /* Complete modem response received, apply config to codec */

        spin_lock_bh(&mut ams_delta_lock);
        mod_timer(&mut cx81801_timer, jiffies.wrapping_add(msecs_to_jiffies(150)));
        apply = (!ams_delta_muted && !cx81801_cmd_pending) as c_int;
        cx81801_cmd_pending = true;
        spin_unlock_bh(&mut ams_delta_lock);

        /* Apply config pulse by connecting the codec to the modem
         * if not already done */
        if apply != 0 {
            gpiod_set_value(gpiod_modem_codec, 1);
        }
        break;
    }
}

type c_uchar = u8;

/* Line discipline .write_wakeup() */
unsafe extern "C" fn cx81801_wakeup(tty: *mut tty_struct) {
    v253_ops.write_wakeup.unwrap()(tty);
}

static mut cx81801_ops: tty_ldisc_ops = tty_ldisc_ops {
    name: b"cx81801\0".as_ptr() as *const c_char,
    num: N_V253,
    owner: unsafe { THIS_MODULE },
    open: Some(cx81801_open),
    close: Some(cx81801_close),
    hangup: Some(cx81801_hangup),
    receive_buf: Some(cx81801_receive),
    write_wakeup: Some(cx81801_wakeup),
};

/*
 * Even if not very useful, the sound card can still work without any of the
 * above functionality activated.  You can still control its audio input/output
 * constellation and speakerphone gain from userspace by issuing AT commands
 * over the modem port.
 */

static mut ams_delta_ops: snd_soc_ops = snd_soc_ops {
    startup: None,
    shutdown: None,
};

unsafe extern "C" fn ams_delta_mute(
    _dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let apply: c_int;

    if ams_delta_muted as c_int == mute {
        return 0;
    }

    spin_lock_bh(&mut ams_delta_lock);
    ams_delta_muted = mute != 0;
    apply = (!cx81801_cmd_pending) as c_int;
    spin_unlock_bh(&mut ams_delta_lock);

    if apply != 0 {
        gpiod_set_value(gpiod_modem_codec, (mute != 0) as c_int);
    }
    0
}

/* Our codec DAI probably doesn't have its own .ops structure */
static ams_delta_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    mute_stream: Some(ams_delta_mute),
    no_capture_mute: 1,
};

/* Will be used if the codec ever has its own digital_mute function */
unsafe extern "C" fn ams_delta_startup(substream: *mut snd_pcm_substream) -> c_int {
    ams_delta_mute(core::ptr::null_mut(), 0, (*substream).stream)
}

unsafe extern "C" fn ams_delta_shutdown(substream: *mut snd_pcm_substream) {
    ams_delta_mute(core::ptr::null_mut(), 1, (*substream).stream);
}

/*
 * Card initialization
 */

unsafe extern "C" fn ams_delta_cx20442_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let card: *mut snd_soc_card = (*rtd).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let mut ret: c_int;
    /* Codec is ready, now add/activate board specific controls */

    /* Store a pointer to the codec structure for tty ldisc use */
    cx20442_codec.component = (*snd_soc_rtd_to_codec(rtd, 0)).component;

    /* Add hook switch - can be used to control the codec from userspace
     * even if line discipline fails */
    ret = snd_soc_card_jack_new_pins(
        card,
        b"hook_switch\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET,
        &mut ams_delta_hook_switch,
        core::ptr::null_mut(),
        0,
    );
    if ret != 0 {
        dev_warn(
            (*card).dev,
            b"Failed to allocate resources for hook switch, will continue without one.\n\0".as_ptr()
                as *const c_char,
        );
    } else {
        ret = snd_soc_jack_add_gpiods(
            (*card).dev,
            &mut ams_delta_hook_switch,
            ams_delta_hook_switch_gpios.len() as c_int,
            ams_delta_hook_switch_gpios.as_mut_ptr(),
        );
        if ret != 0 {
            dev_warn(
                (*card).dev,
                b"Failed to set up hook switch GPIO line, will continue with hook switch inactive.\n\0"
                    .as_ptr() as *const c_char,
            );
        }
    }

    gpiod_modem_codec = devm_gpiod_get(
        (*card).dev,
        b"modem_codec\0".as_ptr() as *const c_char,
        GPIOD_OUT_HIGH,
    );
    if IS_ERR(gpiod_modem_codec as *const c_void) {
        dev_warn(
            (*card).dev,
            b"Failed to obtain modem_codec GPIO\n\0".as_ptr() as *const c_char,
        );
        return 0;
    }

    /* Set up digital mute if not provided by the codec */
    if (*(*codec_dai).driver).ops.is_null() {
        (*(*codec_dai).driver).ops = &ams_delta_dai_ops;
    } else {
        ams_delta_ops.startup = Some(ams_delta_startup);
        ams_delta_ops.shutdown = Some(ams_delta_shutdown);
    }

    /* Register optional line discipline for over the modem control */
    ret = tty_register_ldisc(&mut cx81801_ops);
    if ret != 0 {
        dev_warn(
            (*card).dev,
            b"Failed to register line discipline, will continue without any controls.\n\0".as_ptr()
                as *const c_char,
        );
        return 0;
    }

    /* Set up initial pin constellation */
    snd_soc_dapm_disable_pin(dapm, b"Mouthpiece\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"Speaker\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"AGCIN\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"AGCOUT\0".as_ptr() as *const c_char);

    0
}

/* DAI glue - connects codec <--> CPU */
/* SND_SOC_DAILINK_DEFS(cx20442, ...)
 *   DAILINK_COMP_ARRAY(COMP_CPU("omap-mcbsp.1")),
 *   DAILINK_COMP_ARRAY(COMP_CODEC("cx20442-codec", "cx20442-voice")),
 *   DAILINK_COMP_ARRAY(COMP_PLATFORM("omap-mcbsp.1"))
 * and SND_SOC_DAILINK_REG(cx20442) are external ASoC macro-generated fields. */
static mut ams_delta_dai_link: snd_soc_dai_link = snd_soc_dai_link {
    name: b"CX20442\0".as_ptr() as *const c_char,
    stream_name: b"CX20442\0".as_ptr() as *const c_char,
    init: Some(ams_delta_cx20442_init),
    ops: unsafe { &mut ams_delta_ops },
    dai_fmt: SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
};

/* Audio card driver */
static mut ams_delta_audio_card: snd_soc_card = snd_soc_card {
    name: b"AMS_DELTA\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &mut ams_delta_dai_link },
    num_links: 1,
    controls: ams_delta_audio_controls.as_ptr(),
    num_controls: ams_delta_audio_controls.len() as c_uint,
    dapm_widgets: ams_delta_dapm_widgets.as_ptr(),
    num_dapm_widgets: ams_delta_dapm_widgets.len() as c_uint,
    dapm_routes: ams_delta_audio_map.as_ptr(),
    num_dapm_routes: ams_delta_audio_map.len() as c_uint,
    dev: core::ptr::null_mut(),
};

/* Module init/exit */
unsafe extern "C" fn ams_delta_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &mut ams_delta_audio_card;
    let ret: c_int;

    (*card).dev = &mut (*pdev).dev;

    handset_mute = devm_gpiod_get(
        &mut (*pdev).dev,
        b"handset_mute\0".as_ptr() as *const c_char,
        GPIOD_OUT_HIGH,
    );
    if IS_ERR(handset_mute as *const c_void) {
        return PTR_ERR(handset_mute as *const c_void);
    }

    handsfree_mute = devm_gpiod_get(
        &mut (*pdev).dev,
        b"handsfree_mute\0".as_ptr() as *const c_char,
        GPIOD_OUT_HIGH,
    );
    if IS_ERR(handsfree_mute as *const c_void) {
        return PTR_ERR(handsfree_mute as *const c_void);
    }

    ret = snd_soc_register_card(card);
    if ret != 0 {
        (*card).dev = core::ptr::null_mut();
        return dev_err_probe(
            &mut (*pdev).dev,
            ret,
            b"snd_soc_register_card() failed\n\0".as_ptr() as *const c_char,
        );
    }
    0
}

unsafe extern "C" fn ams_delta_remove(pdev: *mut platform_device) {
    let card: *mut snd_soc_card = platform_get_drvdata(pdev);

    tty_unregister_ldisc(&mut cx81801_ops);

    snd_soc_unregister_card(card);
    (*card).dev = core::ptr::null_mut();
}

static mut ams_delta_driver: platform_driver = platform_driver {
    driver: device_driver { name: DRV_NAME },
    probe: Some(ams_delta_probe),
    remove: Some(ams_delta_remove),
};

/* module_platform_driver(ams_delta_driver); */

/* MODULE_AUTHOR("Janusz Krzysztofik <jkrzyszt@tis.icnet.pl>"); */
/* MODULE_DESCRIPTION("ALSA SoC driver for Amstrad E3 (Delta) videophone"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:" DRV_NAME); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
