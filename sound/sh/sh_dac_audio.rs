// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sh_dac_audio.c - SuperH DAC audio driver for ALSA
 *
 * Copyright (c) 2009 by Rafael Ignacio Zurita <rizurita@yahoo.com>
 *
 * Based on sh_dac_audio.c (Copyright (C) 2004, 2005 by Andriy Skulysh)
 */

// Dependencies from the original C includes:
// linux/hrtimer.h, linux/interrupt.h, linux/io.h, linux/platform_device.h,
// linux/slab.h, linux/module.h, sound/core.h, sound/initval.h, sound/pcm.h,
// sound/sh_dac_audio.h, asm/clock.h, asm/hd64461.h, mach/hp6xx.h, cpu/dac.h.
// MODULE_AUTHOR("Rafael Ignacio Zurita <rizurita@yahoo.com>");
// MODULE_DESCRIPTION("SuperH DAC audio driver");
// MODULE_LICENSE("GPL");

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::offset_of;
use core::ptr;

type ssize_t = isize;
type snd_pcm_uframes_t = c_ulong;
type ktime_t = i64;

const SNDRV_DEFAULT_IDX1: c_int = -1;
static SNDRV_DEFAULT_STR1_VALUE: &[u8] = b"\0";

const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 2;
const SNDRV_PCM_INFO_HALF_DUPLEX: c_uint = 1 << 3;
const SNDRV_PCM_FMTBIT_U8: c_ulong = 1 << 0;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_DMA_TYPE_CONTINUOUS: c_int = 0;
const SNDRV_DEV_LOWLEVEL: c_int = 0;
const HRTIMER_MODE_REL: c_int = 0;
const CLOCK_MONOTONIC: c_int = 1;
const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;

/* Module Parameters */
static mut index: c_int = SNDRV_DEFAULT_IDX1;
static mut id: *mut c_char = SNDRV_DEFAULT_STR1_VALUE.as_ptr() as *mut c_char;
// module_param(index, int, 0444);
// MODULE_PARM_DESC(index, "Index value for SuperH DAC audio.");
// module_param(id, charp, 0444);
// MODULE_PARM_DESC(id, "ID string for SuperH DAC audio.");

#[repr(C)]
pub struct snd_card {
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub buffer_size: c_int,
    pub period_size: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct hrtimer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device {
    pub device_data: *mut c_void,
}

#[repr(C)]
pub struct iov_iter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub name: [c_char; 80],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dac_audio_pdata {
    pub start: Option<unsafe extern "C" fn(*mut dac_audio_pdata)>,
    pub stop: Option<unsafe extern "C" fn(*mut dac_audio_pdata)>,
    pub buffer_size: usize,
    pub channel: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_ulong,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub copy: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            c_int,
            c_ulong,
            *mut iov_iter,
            c_ulong,
        ) -> c_int,
    >,
    pub fill_silence:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_ulong, c_ulong) -> c_int>,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hrtimer_restart {
    HRTIMER_NORESTART = 0,
}

/* main struct */
#[repr(C)]
pub struct snd_sh_dac {
    pub card: *mut snd_card,
    pub substream: *mut snd_pcm_substream,
    pub hrtimer: hrtimer,
    pub wakeups_per_second: ktime_t,

    pub rate: c_int,
    pub empty: c_int,
    pub data_buffer: *mut c_char,
    pub buffer_begin: *mut c_char,
    pub buffer_end: *mut c_char,
    pub processed: c_int, /* bytes proccesed, to compare with period_size */
    pub buffer_size: c_int,
    pub pdata: *mut dac_audio_pdata,
}

unsafe extern "C" {
    static THIS_MODULE: *mut module;

    fn hrtimer_start(timer: *mut hrtimer, tim: ktime_t, mode: c_int);
    fn hrtimer_cancel(timer: *mut hrtimer) -> c_int;
    fn hrtimer_setup(
        timer: *mut hrtimer,
        function: unsafe extern "C" fn(*mut hrtimer) -> hrtimer_restart,
        clock_id: c_int,
        mode: c_int,
    );

    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_sh_dac;
    fn copy_from_iter(addr: *mut c_void, bytes: c_ulong, i: *mut iov_iter) -> usize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> ssize_t;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        data: *mut c_void,
        size: usize,
        max: usize,
    ) -> c_int;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn snd_card_free(card: *mut c_void);
    fn kfree(objp: *mut c_void);
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, size: snd_pcm_uframes_t) -> ssize_t;
    fn sh_dac_output(value: c_char, channel: c_int);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_device_new(
        card: *mut snd_card,
        ty: c_int,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut module,
        extra_size: c_int,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_card_register(card: *mut snd_card) -> c_int;
}

unsafe extern "C" fn dac_audio_start_timer(chip: *mut snd_sh_dac) {
    unsafe {
        hrtimer_start(
            &mut (*chip).hrtimer,
            (*chip).wakeups_per_second,
            HRTIMER_MODE_REL,
        );
    }
}

unsafe extern "C" fn dac_audio_stop_timer(chip: *mut snd_sh_dac) {
    unsafe {
        hrtimer_cancel(&mut (*chip).hrtimer);
    }
}

unsafe extern "C" fn dac_audio_reset(chip: *mut snd_sh_dac) {
    unsafe {
        dac_audio_stop_timer(chip);
        (*chip).buffer_end = (*chip).data_buffer;
        (*chip).buffer_begin = (*chip).buffer_end;
        (*chip).processed = 0;
        (*chip).empty = 1;
    }
}

unsafe extern "C" fn dac_audio_set_rate(chip: *mut snd_sh_dac) {
    unsafe {
        (*chip).wakeups_per_second = 1000000000 / ((*chip).rate as ktime_t);
    }
}

/* PCM INTERFACE */

static snd_sh_dac_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_HALF_DUPLEX,
    formats: SNDRV_PCM_FMTBIT_U8,
    rates: SNDRV_PCM_RATE_8000,
    rate_min: 8000,
    rate_max: 8000,
    channels_min: 1,
    channels_max: 1,
    buffer_bytes_max: 48 * 1024,
    period_bytes_min: 1,
    period_bytes_max: 48 * 1024,
    periods_min: 1,
    periods_max: 1024,
};

unsafe extern "C" fn snd_sh_dac_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;

        (*runtime).hw = snd_sh_dac_pcm_hw;

        (*chip).substream = substream;
        (*chip).buffer_end = (*chip).data_buffer;
        (*chip).buffer_begin = (*chip).buffer_end;
        (*chip).processed = 0;
        (*chip).empty = 1;

        ((*(*chip).pdata).start.unwrap())((*chip).pdata);

        0
    }
}

unsafe extern "C" fn snd_sh_dac_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);

        (*chip).substream = ptr::null_mut();

        dac_audio_stop_timer(chip);
        ((*(*chip).pdata).stop.unwrap())((*chip).pdata);

        0
    }
}

unsafe extern "C" fn snd_sh_dac_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let runtime = (*(*chip).substream).runtime;

        (*chip).buffer_size = (*runtime).buffer_size;
        memset(
            (*chip).data_buffer as *mut c_void,
            0,
            (*(*chip).pdata).buffer_size,
        );

        0
    }
}

unsafe extern "C" fn snd_sh_dac_pcm_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);

        match cmd {
            SNDRV_PCM_TRIGGER_START => {
                dac_audio_start_timer(chip);
            }
            SNDRV_PCM_TRIGGER_STOP => {
                (*chip).buffer_end = (*chip).data_buffer;
                (*chip).buffer_begin = (*chip).buffer_end;
                (*chip).processed = 0;
                (*chip).empty = 1;
                dac_audio_stop_timer(chip);
            }
            _ => {
                return -EINVAL;
            }
        }

        0
    }
}

unsafe extern "C" fn snd_sh_dac_pcm_copy(
    substream: *mut snd_pcm_substream,
    channel: c_int,
    pos: c_ulong,
    src: *mut iov_iter,
    count: c_ulong,
) -> c_int {
    unsafe {
        /* channel is not used (interleaved data) */
        let _ = channel;
        let chip = snd_pcm_substream_chip(substream);

        if copy_from_iter((*chip).data_buffer.add(pos as usize) as *mut c_void, count, src) != count as usize {
            return -EFAULT;
        }
        (*chip).buffer_end = (*chip).data_buffer.add((pos + count) as usize);

        if (*chip).empty != 0 {
            (*chip).empty = 0;
            dac_audio_start_timer(chip);
        }

        0
    }
}

unsafe extern "C" fn snd_sh_dac_pcm_silence(
    substream: *mut snd_pcm_substream,
    channel: c_int,
    pos: c_ulong,
    count: c_ulong,
) -> c_int {
    unsafe {
        /* channel is not used (interleaved data) */
        let _ = channel;
        let chip = snd_pcm_substream_chip(substream);

        memset(
            (*chip).data_buffer.add(pos as usize) as *mut c_void,
            0,
            count as usize,
        );
        (*chip).buffer_end = (*chip).data_buffer.add((pos + count) as usize);

        if (*chip).empty != 0 {
            (*chip).empty = 0;
            dac_audio_start_timer(chip);
        }

        0
    }
}

unsafe extern "C" fn snd_sh_dac_pcm_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let pointer = (*chip).buffer_begin.offset_from((*chip).data_buffer) as c_int;

        pointer as snd_pcm_uframes_t
    }
}

/* pcm ops */
static snd_sh_dac_pcm_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_sh_dac_pcm_open),
    close: Some(snd_sh_dac_pcm_close),
    prepare: Some(snd_sh_dac_pcm_prepare),
    trigger: Some(snd_sh_dac_pcm_trigger),
    pointer: Some(snd_sh_dac_pcm_pointer),
    copy: Some(snd_sh_dac_pcm_copy),
    fill_silence: Some(snd_sh_dac_pcm_silence),
};

unsafe extern "C" fn snd_sh_dac_pcm(chip: *mut snd_sh_dac, device: c_int) -> c_int {
    unsafe {
        let mut err: c_int;
        let mut pcm: *mut snd_pcm = ptr::null_mut();

        /* device should be always 0 for us */
        err = snd_pcm_new(
            (*chip).card,
            c"SH_DAC PCM".as_ptr(),
            device,
            1,
            0,
            &mut pcm,
        );
        if err < 0 {
            return err;
        }

        (*pcm).private_data = chip as *mut c_void;
        strscpy((*pcm).name.as_mut_ptr(), c"SH_DAC PCM".as_ptr());
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_sh_dac_pcm_ops);

        /* buffer size=48K */
        snd_pcm_set_managed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_CONTINUOUS,
            ptr::null_mut(),
            48 * 1024,
            48 * 1024,
        );

        0
    }
}
/* END OF PCM INTERFACE */

/* driver .remove  --  destructor */
unsafe extern "C" fn snd_sh_dac_remove(devptr: *mut platform_device) {
    unsafe {
        snd_card_free(platform_get_drvdata(devptr));
    }
}

/* free -- it has been defined by create */
unsafe extern "C" fn snd_sh_dac_free(chip: *mut snd_sh_dac) -> c_int {
    unsafe {
        /* release the data */
        kfree((*chip).data_buffer as *mut c_void);
        kfree(chip as *mut c_void);

        0
    }
}

unsafe extern "C" fn snd_sh_dac_dev_free(device: *mut snd_device) -> c_int {
    unsafe {
        let chip = (*device).device_data as *mut snd_sh_dac;

        snd_sh_dac_free(chip)
    }
}

unsafe extern "C" fn sh_dac_audio_timer(handle: *mut hrtimer) -> hrtimer_restart {
    unsafe {
        let chip = (handle as *mut u8).sub(offset_of!(snd_sh_dac, hrtimer)) as *mut snd_sh_dac;
        let runtime = (*(*chip).substream).runtime;
        let b_ps: ssize_t = frames_to_bytes(runtime, (*runtime).period_size);

        if (*chip).empty == 0 {
            sh_dac_output(*(*chip).buffer_begin, (*(*chip).pdata).channel);
            (*chip).buffer_begin = (*chip).buffer_begin.add(1);

            (*chip).processed += 1;
            if (*chip).processed >= b_ps as c_int {
                (*chip).processed -= b_ps as c_int;
                snd_pcm_period_elapsed((*chip).substream);
            }

            if (*chip).buffer_begin
                == (*chip)
                    .data_buffer
                    .add(((*chip).buffer_size - 1) as usize)
            {
                (*chip).buffer_begin = (*chip).data_buffer;
            }

            if (*chip).buffer_begin == (*chip).buffer_end {
                (*chip).empty = 1;
            }
        }

        if (*chip).empty == 0 {
            hrtimer_start(
                &mut (*chip).hrtimer,
                (*chip).wakeups_per_second,
                HRTIMER_MODE_REL,
            );
        }

        hrtimer_restart::HRTIMER_NORESTART
    }
}

/* create  --  chip-specific constructor for the cards components */
unsafe extern "C" fn snd_sh_dac_create(
    card: *mut snd_card,
    devptr: *mut platform_device,
    rchip: *mut *mut snd_sh_dac,
) -> c_int {
    unsafe {
        let mut chip: *mut snd_sh_dac;
        let err: c_int;

        static ops: snd_device_ops = snd_device_ops {
            dev_free: Some(snd_sh_dac_dev_free),
        };

        *rchip = ptr::null_mut();

        chip = kzalloc(core::mem::size_of::<snd_sh_dac>(), GFP_KERNEL) as *mut snd_sh_dac;
        if chip.is_null() {
            return -ENOMEM;
        }

        (*chip).card = card;

        hrtimer_setup(
            &mut (*chip).hrtimer,
            sh_dac_audio_timer,
            CLOCK_MONOTONIC,
            HRTIMER_MODE_REL,
        );

        dac_audio_reset(chip);
        (*chip).rate = 8000;
        dac_audio_set_rate(chip);

        (*chip).pdata = (*devptr).dev.platform_data as *mut dac_audio_pdata;

        (*chip).data_buffer = kmalloc((*(*chip).pdata).buffer_size, GFP_KERNEL) as *mut c_char;
        if (*chip).data_buffer.is_null() {
            kfree(chip as *mut c_void);
            return -ENOMEM;
        }

        err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip as *mut c_void, &ops);
        if err < 0 {
            snd_sh_dac_free(chip);
            return err;
        }

        *rchip = chip;

        0
    }
}

/* driver .probe  --  constructor */
unsafe extern "C" fn snd_sh_dac_probe(devptr: *mut platform_device) -> c_int {
    unsafe {
        let mut chip: *mut snd_sh_dac = ptr::null_mut();
        let mut card: *mut snd_card = ptr::null_mut();
        let mut err: c_int;

        err = snd_card_new(
            &mut (*devptr).dev,
            index,
            id,
            THIS_MODULE,
            0,
            &mut card,
        );
        if err < 0 {
            dev_err(&mut (*devptr).dev, c"cannot allocate the card\n".as_ptr());
            return err;
        }

        err = snd_sh_dac_create(card, devptr, &mut chip);
        if err < 0 {
            snd_card_free(card as *mut c_void);
            return err;
        }

        err = snd_sh_dac_pcm(chip, 0);
        if err < 0 {
            snd_card_free(card as *mut c_void);
            return err;
        }

        strscpy((*card).driver.as_mut_ptr(), c"snd_sh_dac".as_ptr());
        strscpy(
            (*card).shortname.as_mut_ptr(),
            c"SuperH DAC audio driver".as_ptr(),
        );
        dev_info(
            &mut (*devptr).dev,
            c"%s %s\n".as_ptr(),
            (*card).longname.as_ptr(),
            (*card).shortname.as_ptr(),
        );

        err = snd_card_register(card);
        if err < 0 {
            snd_card_free(card as *mut c_void);
            return err;
        }

        dev_info(
            &mut (*devptr).dev,
            c"ALSA driver for SuperH DAC audio\n".as_ptr(),
        );

        platform_set_drvdata(devptr, card as *mut c_void);
        0
    }
}

/*
 * "driver" definition
 */
static mut sh_dac_driver: platform_driver = platform_driver {
    probe: Some(snd_sh_dac_probe),
    remove: Some(snd_sh_dac_remove),
    driver: device_driver {
        name: c"dac_audio".as_ptr(),
    },
};

// module_platform_driver(sh_dac_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
