// SPDX-License-Identifier: GPL-2.0-only
/*
 * PC-Speaker driver for Linux
 *
 * Copyright (C) 1997-2001  David Woodhouse
 * Copyright (C) 2001-2008  Stas Sergeev
 */

// C dependencies: linux/init.h, linux/module.h, linux/platform_device.h,
// sound/core.h, sound/initval.h, sound/pcm.h, linux/input.h, linux/delay.h,
// linux/bitops.h, linux/mm.h, pcsp_input.h, pcsp.h.

// MODULE_AUTHOR("Stas Sergeev <stsp@users.sourceforge.net>");
// MODULE_DESCRIPTION("PC-Speaker driver");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:pcspkr");

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

extern "C" {
    static mut hrtimer_resolution: c_uint;
    static mut loops_per_jiffy: libc::c_long;
    static THIS_MODULE: *mut c_void;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn fls(x: c_int) -> c_int;
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn hrtimer_setup(
        timer: *mut hrtimer,
        function: hrtimer_restart_fn,
        clock_id: c_int,
        mode: hrtimer_mode,
    );
    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: c_int,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_pcsp_new_pcm(chip: *mut snd_pcsp) -> c_int;
    fn snd_pcsp_new_mixer(chip: *mut snd_pcsp, nopcm: bool) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn debug_pagealloc_enabled() -> bool;
    fn pcspkr_input_init(input_dev: *mut *mut input_dev, dev: *mut device) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn pcsp_sync_stop(chip: *mut snd_pcsp);
    fn pcspkr_stop_sound();
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);

    fn pcsp_do_timer(timer: *mut hrtimer) -> hrtimer_restart;
}

const SNDRV_DEFAULT_IDX1: c_int = -1;
const SNDRV_DEFAULT_STR1: *mut c_char = ptr::null_mut();
const SNDRV_DEFAULT_ENABLE1: bool = true;

const PCSP_MAX_PERIOD_NS: c_uint = 60_000;
const PCSP_MIN_PERIOD_NS: c_uint = 21_000;
const PCSP_MIN_LPJ: libc::c_long = 1_000_000;
const MIN_DIV: c_int = 64;
const MAX_DIV: c_int = 256;
const PCSP_MAX_TREBLE: c_int = 4;
const PCSP_DEFAULT_TREBLE: c_int = 3;
const CLOCK_MONOTONIC: c_int = 1;
const HRTIMER_MODE_REL: hrtimer_mode = 1;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const PCSP_DEBUG: bool = false;

static mut index: c_int = SNDRV_DEFAULT_IDX1; /* Index 0-MAX */
static mut id: *mut c_char = SNDRV_DEFAULT_STR1; /* ID for this card */
static mut enable: bool = SNDRV_DEFAULT_ENABLE1; /* Enable this card */
static mut nopcm: bool = false; /* Disable PCM capability of the driver */

// module_param(index, int, 0444);
// MODULE_PARM_DESC(index, "Index value for pcsp soundcard.");
// module_param(id, charp, 0444);
// MODULE_PARM_DESC(id, "ID string for pcsp soundcard.");
// module_param(enable, bool, 0444);
// MODULE_PARM_DESC(enable, "Enable PC-Speaker sound.");
// module_param(nopcm, bool, 0444);
// MODULE_PARM_DESC(nopcm, "Disable PC-Speaker PCM sound. Only beeps remain.");

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hrtimer {
    _private: [u8; 0],
}

type hrtimer_restart = c_int;
type hrtimer_mode = c_int;
type hrtimer_restart_fn = unsafe extern "C" fn(*mut hrtimer) -> hrtimer_restart;

#[repr(C)]
pub struct input_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub driver: *mut c_char,
    pub shortname: *mut c_char,
    pub longname: *mut c_char,
}

#[repr(C)]
pub struct snd_pcsp {
    pub max_treble: c_int,
    pub treble: c_int,
    pub playback_ptr: c_int,
    pub period_ptr: c_int,
    pub timer_active: atomic_t,
    pub enable: c_int,
    pub pcspkr: c_int,
    pub substream_lock: spinlock_t,
    pub card: *mut snd_card,
    pub port: c_int,
    pub irq: c_int,
    pub dma: c_int,
    pub timer: hrtimer,
    pub input_dev: *mut input_dev,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut platform_device)>,
}

static mut pcsp_chip: snd_pcsp = snd_pcsp {
    max_treble: 0,
    treble: 0,
    playback_ptr: 0,
    period_ptr: 0,
    timer_active: atomic_t { _private: [] },
    enable: 0,
    pcspkr: 0,
    substream_lock: spinlock_t { _private: [] },
    card: ptr::null_mut(),
    port: 0,
    irq: 0,
    dma: 0,
    timer: hrtimer { _private: [] },
    input_dev: ptr::null_mut(),
};

unsafe extern "C" fn snd_pcsp_create(card: *mut snd_card) -> c_int {
    let resolution: c_uint = hrtimer_resolution;
    let div: c_int;
    let min_div: c_int;
    let order: c_int;

    if !nopcm {
        if resolution > PCSP_MAX_PERIOD_NS {
            dev_err(
                (*card).dev,
                c"PCSP: Timer resolution is not sufficient (%unS)\n".as_ptr(),
                resolution,
            );
            dev_err(
                (*card).dev,
                c"PCSP: Make sure you have HPET and ACPI enabled.\n".as_ptr(),
            );
            dev_err((*card).dev, c"PCSP: Turned into nopcm mode.\n".as_ptr());
            nopcm = true;
        }
    }

    if loops_per_jiffy >= PCSP_MIN_LPJ && resolution <= PCSP_MIN_PERIOD_NS {
        min_div = MIN_DIV;
    } else {
        min_div = MAX_DIV;
    }
    if PCSP_DEBUG {
        dev_dbg(
            (*card).dev,
            c"PCSP: lpj=%li, min_div=%i, res=%u\n".as_ptr(),
            loops_per_jiffy,
            min_div,
            resolution,
        );
    }

    div = MAX_DIV / min_div;
    order = fls(div) - 1;

    pcsp_chip.max_treble = core::cmp::min(order, PCSP_MAX_TREBLE);
    pcsp_chip.treble = core::cmp::min(pcsp_chip.max_treble, PCSP_DEFAULT_TREBLE);
    pcsp_chip.playback_ptr = 0;
    pcsp_chip.period_ptr = 0;
    atomic_set(&mut pcsp_chip.timer_active, 0);
    pcsp_chip.enable = 1;
    pcsp_chip.pcspkr = 1;

    spin_lock_init(&mut pcsp_chip.substream_lock);

    pcsp_chip.card = card;
    pcsp_chip.port = 0x61;
    pcsp_chip.irq = -1;
    pcsp_chip.dma = -1;
    (*card).private_data = &mut pcsp_chip as *mut snd_pcsp as *mut c_void;

    0
}

unsafe extern "C" fn pcsp_stop_beep(chip: *mut snd_pcsp) {
    pcsp_sync_stop(chip);
    pcspkr_stop_sound();
}

unsafe extern "C" fn alsa_card_pcsp_free(card: *mut snd_card) {
    pcsp_stop_beep((*card).private_data as *mut snd_pcsp);
}

unsafe extern "C" fn snd_card_pcsp_probe(devnum: c_int, dev: *mut device) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut err: c_int;

    if devnum != 0 {
        return -EINVAL;
    }

    hrtimer_setup(
        &mut pcsp_chip.timer,
        pcsp_do_timer,
        CLOCK_MONOTONIC,
        HRTIMER_MODE_REL,
    );

    err = snd_devm_card_new(dev, index, id, THIS_MODULE, 0, &mut card);
    if err < 0 {
        return err;
    }

    err = snd_pcsp_create(card);
    if err < 0 {
        return err;
    }

    if !nopcm {
        err = snd_pcsp_new_pcm(&mut pcsp_chip);
        if err < 0 {
            return err;
        }
    }
    err = snd_pcsp_new_mixer(&mut pcsp_chip, nopcm);
    if err < 0 {
        return err;
    }

    strscpy((*card).driver, c"PC-Speaker".as_ptr());
    strscpy((*card).shortname, c"pcsp".as_ptr());
    sprintf(
        (*card).longname,
        c"Internal PC-Speaker at port 0x%x".as_ptr(),
        pcsp_chip.port,
    );

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }
    (*card).private_free = Some(alsa_card_pcsp_free);

    0
}

unsafe extern "C" fn alsa_card_pcsp_init(dev: *mut device) -> c_int {
    let err: c_int;

    err = snd_card_pcsp_probe(0, dev);
    if err != 0 {
        dev_err(dev, c"PC-Speaker initialization failed.\n".as_ptr());
        return err;
    }

    /* Well, CONFIG_DEBUG_PAGEALLOC makes the sound horrible. Lets alert */
    if debug_pagealloc_enabled() {
        dev_warn(
            dev,
            c"PCSP: CONFIG_DEBUG_PAGEALLOC is enabled, which may make the sound noisy.\n".as_ptr(),
        );
    }

    0
}

unsafe extern "C" fn pcsp_probe(dev: *mut platform_device) -> c_int {
    let mut err: c_int;

    err = pcspkr_input_init(&mut pcsp_chip.input_dev, &mut (*dev).dev);
    if err < 0 {
        return err;
    }

    err = alsa_card_pcsp_init(&mut (*dev).dev);
    if err < 0 {
        return err;
    }

    platform_set_drvdata(dev, &mut pcsp_chip as *mut snd_pcsp as *mut c_void);
    0
}

unsafe extern "C" fn pcsp_suspend(dev: *mut device) -> c_int {
    let chip: *mut snd_pcsp = dev_get_drvdata(dev) as *mut snd_pcsp;
    pcsp_stop_beep(chip);
    0
}

// DEFINE_SIMPLE_DEV_PM_OPS(pcsp_pm, pcsp_suspend, NULL);
static pcsp_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(pcsp_suspend),
    resume: None,
};

unsafe extern "C" fn pcsp_shutdown(dev: *mut platform_device) {
    let chip: *mut snd_pcsp = platform_get_drvdata(dev) as *mut snd_pcsp;
    pcsp_stop_beep(chip);
}

static mut pcsp_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"pcspkr".as_ptr(),
        pm: &pcsp_pm,
    },
    probe: Some(pcsp_probe),
    shutdown: Some(pcsp_shutdown),
};

unsafe extern "C" fn pcsp_init() -> c_int {
    if !enable {
        return -ENODEV;
    }
    platform_driver_register(&mut pcsp_platform_driver)
}

unsafe extern "C" fn pcsp_exit() {
    platform_driver_unregister(&mut pcsp_platform_driver);
}

// module_init(pcsp_init);
// module_exit(pcsp_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
