// SPDX-License-Identifier: GPL-2.0-only
/*
 * i2sbus driver -- pcm routines
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type u64 = u64;
type dma_addr_t = u32;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;

const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EALREADY: c_int = 114;

const HZ: c_ulong = 100;
const IRQ_HANDLED: irqreturn_t = 1;

const I2S_CLOCK_SPEED_18MHz: c_int = 18_432_000;
const I2S_CLOCK_SPEED_45MHz: c_int = 45_158_400;
const I2S_CLOCK_SPEED_49MHz: c_int = 49_152_000;
const I2S_SF_CLOCK_SOURCE_18MHz: c_int = 0;
const I2S_SF_CLOCK_SOURCE_45MHz: c_int = 0;
const I2S_SF_CLOCK_SOURCE_49MHz: c_int = 0;
const I2S_SF_SERIAL_FORMAT_I2S_32X: c_int = 0;
const I2S_SF_SERIAL_FORMAT_I2S_64X: c_int = 0;
const I2S_SF_SCLK_MASTER: c_int = 0;
const I2S_DWS_NUM_CHANNELS_IN_SHIFT: c_int = 0;
const I2S_DWS_NUM_CHANNELS_OUT_SHIFT: c_int = 0;
const I2S_DWS_DATA_IN_16BIT: c_int = 0;
const I2S_DWS_DATA_OUT_16BIT: c_int = 0;
const I2S_DWS_DATA_IN_24BIT: c_int = 0;
const I2S_DWS_DATA_OUT_24BIT: c_int = 0;
const I2S_PENDING_CLOCKS_STOPPED: u32 = 0;
const CLOCK_SWITCH_PREPARE_SLAVE: c_int = 0;
const CLOCK_SWITCH_SLAVE: c_int = 0;

const SNDRV_PCM_INFO_MMAP: u32 = 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0;
const SNDRV_PCM_INFO_RESUME: u32 = 0;
const SNDRV_PCM_INFO_JOINT_DUPLEX: u32 = 0;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_FMTBIT_S16_BE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_U16_BE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_BE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_U24_BE: u64 = 1 << 3;
const SNDRV_PCM_FMTBIT_S32_BE: u64 = 1 << 4;
const SNDRV_PCM_FMTBIT_U32_BE: u64 = 1 << 5;
const SNDRV_PCM_RATE_5512: u32 = 1 << 0;
const SNDRV_PCM_RATE_8000: u32 = 1 << 1;
const SNDRV_PCM_RATE_11025: u32 = 1 << 2;
const SNDRV_PCM_RATE_16000: u32 = 1 << 3;
const SNDRV_PCM_RATE_22050: u32 = 1 << 4;
const SNDRV_PCM_RATE_32000: u32 = 1 << 5;
const SNDRV_PCM_RATE_44100: u32 = 1 << 6;
const SNDRV_PCM_RATE_48000: u32 = 1 << 7;
const SNDRV_PCM_RATE_64000: u32 = 1 << 8;
const SNDRV_PCM_RATE_88200: u32 = 1 << 9;
const SNDRV_PCM_RATE_96000: u32 = 1 << 10;
const SNDRV_PCM_RATE_176400: u32 = 1 << 11;
const SNDRV_PCM_RATE_192000: u32 = 1 << 12;
const SNDRV_PCM_FORMAT_S16_BE: c_int = 0;
const SNDRV_PCM_FORMAT_U16_BE: c_int = 1;
const SNDRV_PCM_FORMAT_S32_BE: c_int = 2;
const SNDRV_PCM_FORMAT_U32_BE: c_int = 3;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_STOP: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;

const MAX_DBDMA_COMMANDS: u32 = 0;
const INPUT_MORE: u32 = 0;
const OUTPUT_MORE: u32 = 0;
const BR_IFSET: u32 = 0;
const INTR_ALWAYS: u32 = 0;
const DBDMA_NOP: u32 = 0;
const BR_ALWAYS: u32 = 0;
const DBDMA_STOP: u32 = 0;
const RUN: u32 = 0;
const PAUSE: u32 = 0;
const ACTIVE: u32 = 0;
const BT: u16 = 0;

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    info: u32,
    formats: u64,
    rates: u32,
    rate_min: u32,
    rate_max: u32,
    channels_min: u32,
    channels_max: u32,
    buffer_bytes_max: u32,
    period_bytes_min: u32,
    period_bytes_max: u32,
    periods_min: u32,
    periods_max: u32,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    periods: c_int,
    period_size: u32,
    buffer_size: u32,
    dma_addr: dma_addr_t,
    format: c_int,
    rate: c_int,
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    parent: *mut device,
}

#[repr(C)]
pub struct snd_pcm_stream {
    dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm {
    card: *mut snd_card,
    streams: [snd_pcm_stream; 2],
    private_data: *mut c_void,
    private_free: Option<unsafe extern "C" fn(*mut snd_pcm)>,
}

#[repr(C)]
pub struct of_device {
    dev: device,
}

#[repr(C)]
pub struct soundbus_dev {
    codec_list: list_head,
    pcmname: *const i8,
    pcmid: c_int,
    pcm: *mut snd_pcm,
    ofdev: of_device,
}

#[repr(C)]
pub struct transfer_info {
    formats: u64,
    rates: u32,
    transfer_in: c_int,
}

#[repr(C)]
pub struct bus_info {
    bus_factor: c_int,
    sysclock_factor: c_int,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct codec_info {
    transfers: *mut transfer_info,
    bus_factor: c_int,
    sysclock_factor: c_int,
    usable: Option<unsafe extern "C" fn(*mut codec_info_item, *mut transfer_info, *mut transfer_info) -> c_int>,
    open: Option<unsafe extern "C" fn(*mut codec_info_item, *mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut codec_info_item, *mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut codec_info_item, *mut bus_info, *mut snd_pcm_substream) -> c_int>,
    switch_clock: Option<unsafe extern "C" fn(*mut codec_info_item, c_int)>,
    start: Option<unsafe extern "C" fn(*mut codec_info_item, *mut snd_pcm_substream)>,
    stop: Option<unsafe extern "C" fn(*mut codec_info_item, *mut snd_pcm_substream)>,
    owner: *mut module,
}

#[repr(C)]
pub struct codec_info_item {
    list: list_head,
    sdev: *mut soundbus_dev,
    codec: *mut codec_info,
    codec_data: *mut c_void,
}

#[repr(C)]
pub struct dbdma_cmd {
    command: u16,
    req_count: u16,
    phy_addr: u32,
    cmd_dep: u32,
    xfer_status: u16,
}

#[repr(C)]
pub struct dbdma_regs {
    control: u32,
    status: u32,
    cmdptr: u32,
    br_sel: u32,
}

#[repr(C)]
pub struct dbdma_ring {
    stopping: c_int,
    running: c_int,
    cmds: *mut dbdma_cmd,
    bus_cmd_start: u32,
}

#[repr(C)]
pub struct pcm_info {
    substream: *mut snd_pcm_substream,
    active: c_int,
    created: c_int,
    dbdma_ring: dbdma_ring,
    dbdma: *mut dbdma_regs,
    stop_completion: *mut completion,
    current_period: c_int,
    frame_count: u32,
}

#[repr(C)]
pub struct i2s_intfregs {
    serial_format: u32,
    data_word_sizes: u32,
    intr_ctl: u32,
    frame_count: u32,
}

#[repr(C)]
pub struct i2sbus_dev {
    lock: mutex,
    low_lock: spinlock_t,
    sound: soundbus_dev,
    in_: pcm_info,
    out: pcm_info,
    format: c_int,
    rate: c_int,
    intfregs: *mut i2s_intfregs,
    control: *mut c_void,
    macio: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;

    fn i2s_sf_sclkdiv(div: c_int, out: *mut c_int) -> c_int;
    fn i2s_sf_mclkdiv(div: c_int, out: *mut c_int) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut i2sbus_dev;
    fn snd_pcm_chip(pcm: *mut snd_pcm) -> *mut i2sbus_dev;
    fn pcm_format_to_bits(format: c_int) -> u64;
    fn wait_for_completion_timeout(done: *mut completion, timeout: c_ulong) -> c_ulong;
    fn complete(done: *mut completion);
    fn printk(fmt: *const i8, ...);
    fn out_le32(addr: *mut u32, value: u32);
    fn in_le32(addr: *mut u32) -> u32;
    fn udelay(usecs: c_ulong);
    fn msleep(msecs: c_ulong);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn i2sbus_control_enable(control: *mut c_void, i2sdev: *mut i2sbus_dev);
    fn i2sbus_control_cell(control: *mut c_void, i2sdev: *mut i2sbus_dev, enable: c_int);
    fn i2sbus_control_clock(control: *mut c_void, i2sdev: *mut i2sbus_dev, enable: c_int);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_new(card: *mut snd_card, name: *const i8, id: c_int, playback: c_int, capture: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_new_stream(pcm: *mut snd_pcm, stream: c_int, substream_count: c_int) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_device_register(card: *mut snd_card, dev: *mut snd_pcm) -> c_int;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, min: usize, max: usize);
    fn snd_device_free(card: *mut snd_card, dev: *mut snd_pcm);
    fn soundbus_dev_get(dev: *mut soundbus_dev) -> *mut soundbus_dev;
    fn soundbus_dev_put(dev: *mut soundbus_dev);
    fn soundbus_dev_to_i2sbus_dev(dev: *mut soundbus_dev) -> *mut i2sbus_dev;
    fn try_module_get(module: *mut module) -> c_int;
    fn module_put(module: *mut module);
    fn kzalloc_obj_codec_info_item() -> *mut codec_info_item;
    fn kfree(ptr: *mut c_void);
    fn list_del(entry: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_empty(head: *mut list_head) -> c_int;
    fn macio_get_pci_dev(macio: *mut c_void) -> *mut pci_dev;
}

#[repr(C)]
pub struct pci_dev {
    dev: device,
}

unsafe fn for_each_codec(_head: *mut list_head, _f: impl FnMut(*mut codec_info_item)) {
    /* list_for_each_entry over struct codec_info_item::list; supplied by Linux list.h in C. */
}

unsafe fn for_each_codec_reverse(_head: *mut list_head, _f: impl FnMut(*mut codec_info_item)) {
    /* list_for_each_entry_reverse over struct codec_info_item::list; supplied by Linux list.h in C. */
}

unsafe fn for_each_codec_safe(_head: *mut list_head, _f: impl FnMut(*mut codec_info_item, *mut codec_info_item)) {
    /* list_for_each_entry_safe over struct codec_info_item::list; supplied by Linux list.h in C. */
}

unsafe fn list_first_codec(_head: *mut list_head) -> *mut codec_info_item {
    /* list_first_entry(head, struct codec_info_item, list); supplied by Linux list.h in C. */
    ptr::null_mut()
}

#[inline]
unsafe fn get_pcm_info(
    i2sdev: *mut i2sbus_dev,
    in_: c_int,
    pi: *mut *mut pcm_info,
    other: *mut *mut pcm_info,
) {
    if in_ != 0 {
        if !pi.is_null() {
            *pi = &mut (*i2sdev).in_;
        }
        if !other.is_null() {
            *other = &mut (*i2sdev).out;
        }
    } else {
        if !pi.is_null() {
            *pi = &mut (*i2sdev).out;
        }
        if !other.is_null() {
            *other = &mut (*i2sdev).in_;
        }
    }
}

unsafe fn clock_and_divisors(mclk: c_int, sclk: c_int, rate: c_int, out: *mut c_int) -> c_int {
    /* sclk must be derived from mclk! */
    if mclk % sclk != 0 {
        return -1;
    }
    /* derive sclk register value */
    if i2s_sf_sclkdiv(mclk / sclk, out) != 0 {
        return -1;
    }

    if I2S_CLOCK_SPEED_18MHz % (rate * mclk) == 0 {
        if i2s_sf_mclkdiv(I2S_CLOCK_SPEED_18MHz / (rate * mclk), out) == 0 {
            *out |= I2S_SF_CLOCK_SOURCE_18MHz;
            return 0;
        }
    }
    if I2S_CLOCK_SPEED_45MHz % (rate * mclk) == 0 {
        if i2s_sf_mclkdiv(I2S_CLOCK_SPEED_45MHz / (rate * mclk), out) == 0 {
            *out |= I2S_SF_CLOCK_SOURCE_45MHz;
            return 0;
        }
    }
    if I2S_CLOCK_SPEED_49MHz % (rate * mclk) == 0 {
        if i2s_sf_mclkdiv(I2S_CLOCK_SPEED_49MHz / (rate * mclk), out) == 0 {
            *out |= I2S_SF_CLOCK_SOURCE_49MHz;
            return 0;
        }
    }
    -1
}

macro_rules! check_rate {
    ($rates:ident, $rate:literal, $sysclock_factor:expr, $bus_factor:expr) => {{
        let bit = match $rate {
            5512 => SNDRV_PCM_RATE_5512,
            8000 => SNDRV_PCM_RATE_8000,
            11025 => SNDRV_PCM_RATE_11025,
            16000 => SNDRV_PCM_RATE_16000,
            22050 => SNDRV_PCM_RATE_22050,
            32000 => SNDRV_PCM_RATE_32000,
            44100 => SNDRV_PCM_RATE_44100,
            48000 => SNDRV_PCM_RATE_48000,
            64000 => SNDRV_PCM_RATE_64000,
            88200 => SNDRV_PCM_RATE_88200,
            96000 => SNDRV_PCM_RATE_96000,
            176400 => SNDRV_PCM_RATE_176400,
            192000 => SNDRV_PCM_RATE_192000,
            _ => 0,
        };
        if $rates & bit != 0 {
            let mut dummy: c_int = 0;
            if clock_and_divisors($sysclock_factor, $bus_factor, $rate, &mut dummy) != 0 {
                $rates &= !bit;
            }
        }
    }};
}

unsafe fn i2sbus_pcm_open(i2sdev: *mut i2sbus_dev, in_: c_int) -> c_int {
    let mut pi: *mut pcm_info = ptr::null_mut();
    let mut other: *mut pcm_info = ptr::null_mut();
    let mut masks_inited = 0;
    let mut err: c_int;
    let mut formats: u64 = 0;
    let mut rates: u32 = 0;
    let mut bus_factor = 0;
    let mut sysclock_factor = 0;

    get_pcm_info(i2sdev, in_, &mut pi, &mut other);
    let hw = &mut (*(*(*pi).substream).runtime).hw;
    let sdev = &mut (*i2sdev).sound as *mut soundbus_dev;

    if (*pi).active != 0 {
        /* alsa messed up */
        return -EBUSY;
    }

    /* we now need to assign the hw */
    for_each_codec(&mut (*sdev).codec_list, |cii| unsafe {
        let mut ti = (*(*cii).codec).transfers;
        bus_factor = (*(*cii).codec).bus_factor;
        sysclock_factor = (*(*cii).codec).sysclock_factor;
        while (*ti).formats != 0 && (*ti).rates != 0 {
            let mut v = *ti;
            if (*ti).transfer_in == in_
                && (*(*cii).codec).usable.map_or(false, |usable| usable(cii, ti, &mut v) != 0)
            {
                if masks_inited != 0 {
                    formats &= v.formats;
                    rates &= v.rates;
                } else {
                    formats = v.formats;
                    rates = v.rates;
                    masks_inited = 1;
                }
            }
            ti = ti.add(1);
        }
    });
    if masks_inited == 0 || bus_factor == 0 || sysclock_factor == 0 {
        return -ENODEV;
    }
    /* bus dependent stuff */
    hw.info = SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_JOINT_DUPLEX;

    check_rate!(rates, 5512, sysclock_factor, bus_factor);
    check_rate!(rates, 8000, sysclock_factor, bus_factor);
    check_rate!(rates, 11025, sysclock_factor, bus_factor);
    check_rate!(rates, 16000, sysclock_factor, bus_factor);
    check_rate!(rates, 22050, sysclock_factor, bus_factor);
    check_rate!(rates, 32000, sysclock_factor, bus_factor);
    check_rate!(rates, 44100, sysclock_factor, bus_factor);
    check_rate!(rates, 48000, sysclock_factor, bus_factor);
    check_rate!(rates, 64000, sysclock_factor, bus_factor);
    check_rate!(rates, 88200, sysclock_factor, bus_factor);
    check_rate!(rates, 96000, sysclock_factor, bus_factor);
    check_rate!(rates, 176400, sysclock_factor, bus_factor);
    check_rate!(rates, 192000, sysclock_factor, bus_factor);
    hw.rates = rates;

    /* well. the codec might want 24 bits only, and we'll
     * ever only transfer 24 bits, but they are top-aligned!
     * So for alsa, we claim that we're doing full 32 bit
     * while in reality we'll ignore the lower 8 bits of
     * that when doing playback (they're transferred as 0
     * as far as I know, no codecs we have are 32-bit capable
     * so I can't really test) and when doing recording we'll
     * always have those lower 8 bits recorded as 0 */
    if formats & SNDRV_PCM_FMTBIT_S24_BE != 0 {
        formats |= SNDRV_PCM_FMTBIT_S32_BE;
    }
    if formats & SNDRV_PCM_FMTBIT_U24_BE != 0 {
        formats |= SNDRV_PCM_FMTBIT_U32_BE;
    }
    /* now mask off what we can support. I suppose we could
     * also support S24_3LE and some similar formats, but I
     * doubt there's a codec that would be able to use that,
     * so we don't support it here. */
    hw.formats = formats
        & (SNDRV_PCM_FMTBIT_S16_BE
            | SNDRV_PCM_FMTBIT_U16_BE
            | SNDRV_PCM_FMTBIT_S32_BE
            | SNDRV_PCM_FMTBIT_U32_BE);

    /* we need to set the highest and lowest rate possible.
     * These are the highest and lowest rates alsa can
     * support properly in its bitfield.
     * Below, we'll use that to restrict to the rate
     * currently in use (if any). */
    hw.rate_min = 5512;
    hw.rate_max = 192000;
    /* If the other stream is already prepared, keep this stream
     * on the same duplex format and rate.
     *
     * i2sbus_pcm_prepare() still programs one shared transport
     * configuration for both directions, so mixed duplex formats
     * are not supported here.
     */
    if (*other).active != 0 {
        hw.formats &= pcm_format_to_bits((*i2sdev).format);
        /* Restrict rates to the one already in use. */
        hw.rate_min = (*i2sdev).rate as u32;
        hw.rate_max = (*i2sdev).rate as u32;
    }

    hw.channels_min = 2;
    hw.channels_max = 2;
    /* these are somewhat arbitrary */
    hw.buffer_bytes_max = 131072;
    hw.period_bytes_min = 256;
    hw.period_bytes_max = 16384;
    hw.periods_min = 3;
    hw.periods_max = MAX_DBDMA_COMMANDS;
    err = snd_pcm_hw_constraint_integer((*(*pi).substream).runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }
    for_each_codec(&mut (*sdev).codec_list, |cii| unsafe {
        if let Some(open) = (*(*cii).codec).open {
            err = open(cii, (*pi).substream);
            if err != 0 {
                let mut found_this = 0;
                for_each_codec_reverse(&mut (*sdev).codec_list, |rev| unsafe {
                    if found_this != 0 {
                        if let Some(close) = (*(*rev).codec).close {
                            close(rev, (*pi).substream);
                        }
                    }
                    if rev == cii {
                        found_this = 1;
                    }
                });
            }
        }
    });
    if err != 0 {
        return err;
    }

    0
}

unsafe fn i2sbus_pcm_close(i2sdev: *mut i2sbus_dev, in_: c_int) -> c_int {
    let mut pi: *mut pcm_info = ptr::null_mut();
    let mut err = 0;
    get_pcm_info(i2sdev, in_, &mut pi, ptr::null_mut());
    for_each_codec(&mut (*i2sdev).sound.codec_list, |cii| unsafe {
        if let Some(close) = (*(*cii).codec).close {
            let tmp = close(cii, (*pi).substream);
            if tmp != 0 {
                err = tmp;
            }
        }
    });
    (*pi).substream = ptr::null_mut();
    (*pi).active = 0;
    err
}

unsafe fn i2sbus_wait_for_stop(_i2sdev: *mut i2sbus_dev, pi: *mut pcm_info) {
    let mut done = completion { _private: [] };
    let mut time_left: c_ulong;

    if (*pi).dbdma_ring.stopping != 0 {
        (*pi).stop_completion = &mut done;
        time_left = wait_for_completion_timeout(&mut done, HZ);
        (*pi).stop_completion = ptr::null_mut();
        if time_left == 0 {
            /* timeout expired, stop dbdma forcefully */
            printk(c"i2sbus_wait_for_stop: timed out\n".as_ptr());
            /* make sure RUN, PAUSE and S0 bits are cleared */
            out_le32(&mut (*(*pi).dbdma).control, (RUN | PAUSE | 1) << 16);
            (*pi).dbdma_ring.stopping = 0;
            time_left = 10;
            while in_le32(&mut (*(*pi).dbdma).status) & ACTIVE != 0 {
                time_left = time_left.wrapping_sub(1);
                if time_left as c_long <= 0 {
                    break;
                }
                udelay(1);
            }
        }
    }
}

type c_long = isize;

/* CONFIG_PM: translated body preserved; exported only when the surrounding build enables PM. */
pub unsafe extern "C" fn i2sbus_wait_for_stop_both(i2sdev: *mut i2sbus_dev) {
    let mut pi: *mut pcm_info = ptr::null_mut();
    get_pcm_info(i2sdev, 0, &mut pi, ptr::null_mut());
    i2sbus_wait_for_stop(i2sdev, pi);
    get_pcm_info(i2sdev, 1, &mut pi, ptr::null_mut());
    i2sbus_wait_for_stop(i2sdev, pi);
}

unsafe fn i2sbus_pcm_clear_active(i2sdev: *mut i2sbus_dev, in_: c_int) {
    let mut pi: *mut pcm_info = ptr::null_mut();
    get_pcm_info(i2sdev, in_, &mut pi, ptr::null_mut());
    (*pi).active = 0;
}

#[inline]
unsafe fn i2sbus_hw_params(substream: *mut snd_pcm_substream, _params: *mut snd_pcm_hw_params, in_: c_int) -> c_int {
    i2sbus_pcm_clear_active(snd_pcm_substream_chip(substream), in_);
    0
}

#[inline]
unsafe fn i2sbus_hw_free(substream: *mut snd_pcm_substream, in_: c_int) -> c_int {
    let i2sdev = snd_pcm_substream_chip(substream);
    let mut pi: *mut pcm_info = ptr::null_mut();
    get_pcm_info(i2sdev, in_, &mut pi, ptr::null_mut());
    if (*pi).dbdma_ring.stopping != 0 {
        i2sbus_wait_for_stop(i2sdev, pi);
    }
    i2sbus_pcm_clear_active(i2sdev, in_);
    0
}

unsafe extern "C" fn i2sbus_playback_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    i2sbus_hw_params(substream, params, 0)
}

unsafe extern "C" fn i2sbus_playback_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    i2sbus_hw_free(substream, 0)
}

unsafe extern "C" fn i2sbus_record_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    i2sbus_hw_params(substream, params, 1)
}

unsafe extern "C" fn i2sbus_record_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    i2sbus_hw_free(substream, 1)
}

unsafe fn i2sbus_pcm_prepare(i2sdev: *mut i2sbus_dev, in_: c_int) -> c_int {
    let mut sfr = 0;
    let mut dws = 0;
    let mut pi: *mut pcm_info = ptr::null_mut();
    let mut other: *mut pcm_info = ptr::null_mut();

    get_pcm_info(i2sdev, in_, &mut pi, &mut other);
    if (*pi).dbdma_ring.running != 0 {
        return -EBUSY;
    }
    if (*pi).dbdma_ring.stopping != 0 {
        i2sbus_wait_for_stop(i2sdev, pi);
    }
    if (*pi).substream.is_null() || (*(*pi).substream).runtime.is_null() {
        return -EINVAL;
    }

    let runtime = (*(*pi).substream).runtime;
    if (*other).active != 0 && ((*i2sdev).format != (*runtime).format || (*i2sdev).rate != (*runtime).rate) {
        return -EINVAL;
    }
    (*i2sdev).format = (*runtime).format;
    (*i2sdev).rate = (*runtime).rate;

    let periodsize = snd_pcm_lib_period_bytes((*pi).substream);
    let nperiods = (*runtime).periods;
    (*pi).current_period = 0;

    /* generate dbdma command ring first */
    let mut command = (*pi).dbdma_ring.cmds;
    memset(command as *mut c_void, 0, ((nperiods + 2) as usize) * size_of::<dbdma_cmd>());

    /* commands to DMA to/from the ring */
    /*
     * For input, we need to do a graceful stop; if we abort
     * the DMA, we end up with leftover bytes that corrupt
     * the next recording.  To do this we set the S0 status
     * bit and wait for the DMA controller to stop.  Each
     * command has a branch condition to
     * make it branch to a stop command if S0 is set.
     * On input we also need to wait for the S7 bit to be
     * set before turning off the DMA controller.
     * In fact we do the graceful stop for output as well.
     */
    let mut offset = (*runtime).dma_addr;
    let cmd = (if in_ != 0 { INPUT_MORE } else { OUTPUT_MORE }) | BR_IFSET | INTR_ALWAYS;
    let stopaddr = (*pi).dbdma_ring.bus_cmd_start + ((nperiods + 1) as u32) * size_of::<dbdma_cmd>() as u32;
    for _i in 0..nperiods {
        (*command).command = (cmd as u16).to_le();
        (*command).cmd_dep = stopaddr.to_le();
        (*command).phy_addr = offset.to_le();
        (*command).req_count = (periodsize as u16).to_le();
        command = command.add(1);
        offset = offset.wrapping_add(periodsize as u32);
    }

    /* branch back to beginning of ring */
    (*command).command = ((DBDMA_NOP | BR_ALWAYS) as u16).to_le();
    (*command).cmd_dep = (*pi).dbdma_ring.bus_cmd_start.to_le();
    command = command.add(1);

    /* set stop command */
    (*command).command = (DBDMA_STOP as u16).to_le();

    let mut cii = list_first_codec(&mut (*i2sdev).sound.codec_list);
    let mut bi = bus_info { bus_factor: 0, sysclock_factor: 0 };
    let input_16bit: c_int;
    match (*runtime).format {
        SNDRV_PCM_FORMAT_S16_BE | SNDRV_PCM_FORMAT_U16_BE => {
            /* FIXME: if we add different bus factors we need to
             * do more here!! */
            bi.bus_factor = (*(*cii).codec).bus_factor;
            input_16bit = 1;
        }
        SNDRV_PCM_FORMAT_S32_BE | SNDRV_PCM_FORMAT_U32_BE => {
            /* force 64x bus speed, otherwise the data cannot be
             * transferred quickly enough! */
            bi.bus_factor = 64;
            input_16bit = 0;
        }
        _ => return -EINVAL,
    }
    /* we assume all sysclocks are the same! */
    bi.sysclock_factor = (*(*cii).codec).sysclock_factor;

    if clock_and_divisors(bi.sysclock_factor, bi.bus_factor, (*runtime).rate, &mut sfr) < 0 {
        return -EINVAL;
    }
    match bi.bus_factor {
        32 => sfr |= I2S_SF_SERIAL_FORMAT_I2S_32X,
        64 => sfr |= I2S_SF_SERIAL_FORMAT_I2S_64X,
        _ => {}
    }
    /* FIXME: THIS ASSUMES MASTER ALL THE TIME */
    sfr |= I2S_SF_SCLK_MASTER;

    let mut prepare_err = 0;
    for_each_codec(&mut (*i2sdev).sound.codec_list, |cii| unsafe {
        if let Some(prepare) = (*(*cii).codec).prepare {
            prepare_err = prepare(cii, &mut bi, (*pi).substream);
        }
    });
    if prepare_err != 0 {
        return prepare_err;
    }
    /* codecs are fine with it, so set our clocks */
    if input_16bit != 0 {
        dws = (2 << I2S_DWS_NUM_CHANNELS_IN_SHIFT)
            | (2 << I2S_DWS_NUM_CHANNELS_OUT_SHIFT)
            | I2S_DWS_DATA_IN_16BIT
            | I2S_DWS_DATA_OUT_16BIT;
    } else {
        dws = (2 << I2S_DWS_NUM_CHANNELS_IN_SHIFT)
            | (2 << I2S_DWS_NUM_CHANNELS_OUT_SHIFT)
            | I2S_DWS_DATA_IN_24BIT
            | I2S_DWS_DATA_OUT_24BIT;
    }

    /* early exit if already programmed correctly */
    /* not locking these is fine since we touch them only in this function */
    if in_le32(&mut (*(*i2sdev).intfregs).serial_format) == sfr as u32
        && in_le32(&mut (*(*i2sdev).intfregs).data_word_sizes) == dws as u32
    {
        (*pi).active = 1;
        return 0;
    }

    /* let's notify the codecs about clocks going away.
     * For now we only do mastering on the i2s cell... */
    for_each_codec(&mut (*i2sdev).sound.codec_list, |cii| unsafe {
        if let Some(switch_clock) = (*(*cii).codec).switch_clock {
            switch_clock(cii, CLOCK_SWITCH_PREPARE_SLAVE);
        }
    });

    i2sbus_control_enable((*i2sdev).control, i2sdev);
    i2sbus_control_cell((*i2sdev).control, i2sdev, 1);
    out_le32(&mut (*(*i2sdev).intfregs).intr_ctl, I2S_PENDING_CLOCKS_STOPPED);
    i2sbus_control_clock((*i2sdev).control, i2sdev, 0);
    msleep(1);

    /* wait for clock stopped. This can apparently take a while... */
    let mut cnt = 100;
    while cnt != 0
        && (in_le32(&mut (*(*i2sdev).intfregs).intr_ctl) & I2S_PENDING_CLOCKS_STOPPED) == 0
    {
        cnt -= 1;
        msleep(5);
    }
    out_le32(&mut (*(*i2sdev).intfregs).intr_ctl, I2S_PENDING_CLOCKS_STOPPED);

    /* not locking these is fine since we touch them only in this function */
    out_le32(&mut (*(*i2sdev).intfregs).serial_format, sfr as u32);
    out_le32(&mut (*(*i2sdev).intfregs).data_word_sizes, dws as u32);

    i2sbus_control_enable((*i2sdev).control, i2sdev);
    i2sbus_control_cell((*i2sdev).control, i2sdev, 1);
    i2sbus_control_clock((*i2sdev).control, i2sdev, 1);
    msleep(1);

    for_each_codec(&mut (*i2sdev).sound.codec_list, |cii| unsafe {
        if let Some(switch_clock) = (*(*cii).codec).switch_clock {
            switch_clock(cii, CLOCK_SWITCH_SLAVE);
        }
    });

    (*pi).active = 1;
    0
}

/* CONFIG_PM: translated body preserved; exported only when the surrounding build enables PM. */
pub unsafe extern "C" fn i2sbus_pcm_prepare_both(i2sdev: *mut i2sbus_dev) {
    i2sbus_pcm_prepare(i2sdev, 0);
    i2sbus_pcm_prepare(i2sdev, 1);
}

unsafe fn i2sbus_pcm_trigger(i2sdev: *mut i2sbus_dev, in_: c_int, cmd: c_int) -> c_int {
    let mut pi: *mut pcm_info = ptr::null_mut();
    get_pcm_info(i2sdev, in_, &mut pi, ptr::null_mut());

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            if (*pi).dbdma_ring.running != 0 {
                return -EALREADY;
            }
            for_each_codec(&mut (*i2sdev).sound.codec_list, |cii| unsafe {
                if let Some(start) = (*(*cii).codec).start {
                    start(cii, (*pi).substream);
                }
            });
            (*pi).dbdma_ring.running = 1;

            if (*pi).dbdma_ring.stopping != 0 {
                /* Clear the S0 bit, then see if we stopped yet */
                out_le32(&mut (*(*pi).dbdma).control, 1 << 16);
                if in_le32(&mut (*(*pi).dbdma).status) & ACTIVE != 0 {
                    /* possible race here? */
                    udelay(10);
                    if in_le32(&mut (*(*pi).dbdma).status) & ACTIVE != 0 {
                        (*pi).dbdma_ring.stopping = 0;
                        return 0; /* keep running */
                    }
                }
            }

            /* make sure RUN, PAUSE and S0 bits are cleared */
            out_le32(&mut (*(*pi).dbdma).control, (RUN | PAUSE | 1) << 16);
            /* set branch condition select register */
            out_le32(&mut (*(*pi).dbdma).br_sel, (1 << 16) | 1);
            /* write dma command buffer address to the dbdma chip */
            out_le32(&mut (*(*pi).dbdma).cmdptr, (*pi).dbdma_ring.bus_cmd_start);
            /* initialize the frame count and current period */
            (*pi).current_period = 0;
            (*pi).frame_count = in_le32(&mut (*(*i2sdev).intfregs).frame_count);
            /* set the DMA controller running */
            out_le32(&mut (*(*pi).dbdma).control, (RUN << 16) | RUN);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            if (*pi).dbdma_ring.running == 0 {
                return -EALREADY;
            }
            (*pi).dbdma_ring.running = 0;
            /* Set the S0 bit to make the DMA branch to the stop cmd */
            out_le32(&mut (*(*pi).dbdma).control, (1 << 16) | 1);
            (*pi).dbdma_ring.stopping = 1;
            for_each_codec(&mut (*i2sdev).sound.codec_list, |cii| unsafe {
                if let Some(stop) = (*(*cii).codec).stop {
                    stop(cii, (*pi).substream);
                }
            });
        }
        _ => return -EINVAL,
    }

    0
}

unsafe fn i2sbus_pcm_pointer(i2sdev: *mut i2sbus_dev, in_: c_int) -> snd_pcm_uframes_t {
    let mut pi: *mut pcm_info = ptr::null_mut();
    get_pcm_info(i2sdev, in_, &mut pi, ptr::null_mut());
    let mut fc = in_le32(&mut (*(*i2sdev).intfregs).frame_count);
    fc = fc.wrapping_sub((*pi).frame_count);
    if fc >= (*(*(*pi).substream).runtime).buffer_size {
        fc %= (*(*(*pi).substream).runtime).buffer_size;
    }
    fc as snd_pcm_uframes_t
}

#[inline]
unsafe fn handle_interrupt(i2sdev: *mut i2sbus_dev, in_: c_int) {
    let mut pi: *mut pcm_info = ptr::null_mut();
    let mut dma_stopped = 0;

    get_pcm_info(i2sdev, in_, &mut pi, ptr::null_mut());
    if (*pi).dbdma_ring.running == 0 && (*pi).dbdma_ring.stopping == 0 {
        return;
    }

    let mut i = (*pi).current_period;
    let runtime = (*(*pi).substream).runtime;
    while (*(*pi).dbdma_ring.cmds.add(i as usize)).xfer_status != 0 {
        if u16::from_le((*(*pi).dbdma_ring.cmds.add(i as usize)).xfer_status) & BT != 0 {
            /*
             * BT is the branch taken bit.  If it took a branch
             * it is because we set the S0 bit to make it
             * branch to the stop command.
             */
            dma_stopped = 1;
        }
        (*(*pi).dbdma_ring.cmds.add(i as usize)).xfer_status = 0;

        i += 1;
        if i >= (*runtime).periods {
            i = 0;
            (*pi).frame_count = (*pi).frame_count.wrapping_add((*runtime).buffer_size);
        }
        (*pi).current_period = i;

        /*
         * Check the frame count.  The DMA tends to get a bit
         * ahead of the frame counter, which confuses the core.
         */
        let fc = in_le32(&mut (*(*i2sdev).intfregs).frame_count);
        let nframes = (i as u32).wrapping_mul((*runtime).period_size);
        if fc < (*pi).frame_count.wrapping_add(nframes) {
            (*pi).frame_count = fc.wrapping_sub(nframes);
        }
    }

    if dma_stopped != 0 {
        let mut timeout = 1000;
        loop {
            let status = in_le32(&mut (*(*pi).dbdma).status);
            if status & ACTIVE == 0 && (in_ == 0 || status & 0x80 != 0) {
                break;
            }
            timeout -= 1;
            if timeout <= 0 {
                printk(c"i2sbus: timed out waiting for DMA to stop!\n".as_ptr());
                break;
            }
            udelay(1);
        }

        /* Turn off DMA controller, clear S0 bit */
        out_le32(&mut (*(*pi).dbdma).control, (RUN | PAUSE | 1) << 16);
        (*pi).dbdma_ring.stopping = 0;
        if !(*pi).stop_completion.is_null() {
            complete((*pi).stop_completion);
        }
    }

    if (*pi).dbdma_ring.running == 0 {
        return;
    }

    /* may call _trigger again, hence needs to be unlocked */
    snd_pcm_period_elapsed((*pi).substream);
}

#[no_mangle]
pub unsafe extern "C" fn i2sbus_tx_intr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    handle_interrupt(devid as *mut i2sbus_dev, 0);
    IRQ_HANDLED
}

#[no_mangle]
pub unsafe extern "C" fn i2sbus_rx_intr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    handle_interrupt(devid as *mut i2sbus_dev, 1);
    IRQ_HANDLED
}

unsafe extern "C" fn i2sbus_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let i2sdev = snd_pcm_substream_chip(substream);
    if i2sdev.is_null() {
        return -EINVAL;
    }
    (*i2sdev).out.substream = substream;
    i2sbus_pcm_open(i2sdev, 0)
}

unsafe extern "C" fn i2sbus_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let i2sdev = snd_pcm_substream_chip(substream);
    if i2sdev.is_null() {
        return -EINVAL;
    }
    if (*i2sdev).out.substream != substream {
        return -EINVAL;
    }
    let err = i2sbus_pcm_close(i2sdev, 0);
    if err == 0 {
        (*i2sdev).out.substream = ptr::null_mut();
    }
    err
}

unsafe extern "C" fn i2sbus_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let i2sdev = snd_pcm_substream_chip(substream);
    if i2sdev.is_null() {
        return -EINVAL;
    }
    if (*i2sdev).out.substream != substream {
        return -EINVAL;
    }
    i2sbus_pcm_prepare(i2sdev, 0)
}

unsafe extern "C" fn i2sbus_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let i2sdev = snd_pcm_substream_chip(substream);
    if i2sdev.is_null() {
        return -EINVAL;
    }
    if (*i2sdev).out.substream != substream {
        return -EINVAL;
    }
    i2sbus_pcm_trigger(i2sdev, 0, cmd)
}

unsafe extern "C" fn i2sbus_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let i2sdev = snd_pcm_substream_chip(substream);
    if i2sdev.is_null() {
        return (-EINVAL) as snd_pcm_uframes_t;
    }
    if (*i2sdev).out.substream != substream {
        return 0;
    }
    i2sbus_pcm_pointer(i2sdev, 0)
}

static i2sbus_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(i2sbus_playback_open),
    close: Some(i2sbus_playback_close),
    hw_params: Some(i2sbus_playback_hw_params),
    hw_free: Some(i2sbus_playback_hw_free),
    prepare: Some(i2sbus_playback_prepare),
    trigger: Some(i2sbus_playback_trigger),
    pointer: Some(i2sbus_playback_pointer),
};

unsafe extern "C" fn i2sbus_record_open(substream: *mut snd_pcm_substream) -> c_int {
    let i2sdev = snd_pcm_substream_chip(substream);
    if i2sdev.is_null() {
        return -EINVAL;
    }
    (*i2sdev).in_.substream = substream;
    i2sbus_pcm_open(i2sdev, 1)
}

unsafe extern "C" fn i2sbus_record_close(substream: *mut snd_pcm_substream) -> c_int {
    let i2sdev = snd_pcm_substream_chip(substream);
    if i2sdev.is_null() {
        return -EINVAL;
    }
    if (*i2sdev).in_.substream != substream {
        return -EINVAL;
    }
    let err = i2sbus_pcm_close(i2sdev, 1);
    if err == 0 {
        (*i2sdev).in_.substream = ptr::null_mut();
    }
    err
}

unsafe extern "C" fn i2sbus_record_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let i2sdev = snd_pcm_substream_chip(substream);
    if i2sdev.is_null() {
        return -EINVAL;
    }
    if (*i2sdev).in_.substream != substream {
        return -EINVAL;
    }
    i2sbus_pcm_prepare(i2sdev, 1)
}

unsafe extern "C" fn i2sbus_record_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let i2sdev = snd_pcm_substream_chip(substream);
    if i2sdev.is_null() {
        return -EINVAL;
    }
    if (*i2sdev).in_.substream != substream {
        return -EINVAL;
    }
    i2sbus_pcm_trigger(i2sdev, 1, cmd)
}

unsafe extern "C" fn i2sbus_record_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let i2sdev = snd_pcm_substream_chip(substream);
    if i2sdev.is_null() {
        return (-EINVAL) as snd_pcm_uframes_t;
    }
    if (*i2sdev).in_.substream != substream {
        return 0;
    }
    i2sbus_pcm_pointer(i2sdev, 1)
}

static i2sbus_record_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(i2sbus_record_open),
    close: Some(i2sbus_record_close),
    hw_params: Some(i2sbus_record_hw_params),
    hw_free: Some(i2sbus_record_hw_free),
    prepare: Some(i2sbus_record_prepare),
    trigger: Some(i2sbus_record_trigger),
    pointer: Some(i2sbus_record_pointer),
};

unsafe extern "C" fn i2sbus_private_free(pcm: *mut snd_pcm) {
    let i2sdev = snd_pcm_chip(pcm);
    (*i2sdev).sound.pcm = ptr::null_mut();
    (*i2sdev).out.created = 0;
    (*i2sdev).in_.created = 0;
    for_each_codec_safe(&mut (*i2sdev).sound.codec_list, |p, _tmp| unsafe {
        printk(c"i2sbus: a codec didn't unregister!\n".as_ptr());
        list_del(&mut (*p).list);
        module_put((*(*p).codec).owner);
        kfree(p as *mut c_void);
    });
    soundbus_dev_put(&mut (*i2sdev).sound);
    module_put(THIS_MODULE);
}

#[no_mangle]
pub unsafe extern "C" fn i2sbus_attach_codec(
    dev: *mut soundbus_dev,
    card: *mut snd_card,
    ci: *mut codec_info,
    data: *mut c_void,
) -> c_int {
    let mut err: c_int;
    let mut in_ = 0;
    let mut out = 0;
    let i2sdev = soundbus_dev_to_i2sbus_dev(dev);

    if (*dev).pcmname.is_null() || (*dev).pcmid == -1 {
        printk(c"i2sbus: pcm name and id must be set!\n".as_ptr());
        return -EINVAL;
    }

    let mut already = 0;
    for_each_codec(&mut (*dev).codec_list, |cii| unsafe {
        if (*cii).codec_data == data {
            already = 1;
        }
    });
    if already != 0 {
        return -EALREADY;
    }

    if (*ci).transfers.is_null()
        || (*(*ci).transfers).formats == 0
        || (*(*ci).transfers).rates == 0
        || (*ci).usable.is_none()
    {
        return -EINVAL;
    }

    /* we currently code the i2s transfer on the clock, and support only
     * 32 and 64 */
    if (*ci).bus_factor != 32 && (*ci).bus_factor != 64 {
        return -EINVAL;
    }

    /* If you want to fix this, you need to keep track of what transport infos
     * are to be used, which codecs they belong to, and then fix all the
     * sysclock/busclock stuff above to depend on which is usable */
    let mut bad_clock = 0;
    for_each_codec(&mut (*dev).codec_list, |cii| unsafe {
        if (*(*cii).codec).sysclock_factor != (*ci).sysclock_factor {
            printk(c"cannot yet handle multiple different sysclocks!\n".as_ptr());
            bad_clock = 1;
        }
        if (*(*cii).codec).bus_factor != (*ci).bus_factor {
            printk(c"cannot yet handle multiple different bus clocks!\n".as_ptr());
            bad_clock = 1;
        }
    });
    if bad_clock != 0 {
        return -EINVAL;
    }

    let mut tmp = (*ci).transfers;
    while (*tmp).formats != 0 && (*tmp).rates != 0 {
        if (*tmp).transfer_in != 0 {
            in_ = 1;
        } else {
            out = 1;
        }
        tmp = tmp.add(1);
    }

    let cii = kzalloc_obj_codec_info_item();
    if cii.is_null() {
        return -ENOMEM;
    }

    /* use the private data to point to the codec info */
    (*cii).sdev = soundbus_dev_get(dev);
    (*cii).codec = ci;
    (*cii).codec_data = data;

    if (*cii).sdev.is_null() {
        printk(c"i2sbus: failed to get soundbus dev reference\n".as_ptr());
        err = -ENODEV;
        goto_out_free_cii(cii, err)
    } else if try_module_get(THIS_MODULE) == 0 {
        printk(c"i2sbus: failed to get module reference!\n".as_ptr());
        err = -EBUSY;
        soundbus_dev_put(dev);
        kfree(cii as *mut c_void);
        err
    } else if try_module_get((*ci).owner) == 0 {
        printk(c"i2sbus: failed to get module reference to codec owner!\n".as_ptr());
        err = -EBUSY;
        module_put(THIS_MODULE);
        soundbus_dev_put(dev);
        kfree(cii as *mut c_void);
        err
    } else {
        if (*dev).pcm.is_null() {
            err = snd_pcm_new(card, (*dev).pcmname, (*dev).pcmid, 0, 0, &mut (*dev).pcm);
            if err != 0 {
                printk(c"i2sbus: failed to create pcm\n".as_ptr());
                module_put((*ci).owner);
                module_put(THIS_MODULE);
                soundbus_dev_put(dev);
                kfree(cii as *mut c_void);
                return err;
            }
        }

        /* ALSA yet again sucks.
         * If it is ever fixed, remove this line. See below. */
        out = 1;
        in_ = 1;

        if (*i2sdev).out.created == 0 && out != 0 {
            if (*(*dev).pcm).card != card {
                /* eh? */
                printk(c"Can't attach same bus to different cards!\n".as_ptr());
                module_put((*ci).owner);
                module_put(THIS_MODULE);
                soundbus_dev_put(dev);
                kfree(cii as *mut c_void);
                return -EINVAL;
            }
            err = snd_pcm_new_stream((*dev).pcm, SNDRV_PCM_STREAM_PLAYBACK as c_int, 1);
            if err != 0 {
                module_put((*ci).owner);
                module_put(THIS_MODULE);
                soundbus_dev_put(dev);
                kfree(cii as *mut c_void);
                return err;
            }
            snd_pcm_set_ops((*dev).pcm, SNDRV_PCM_STREAM_PLAYBACK as c_int, &i2sbus_playback_ops);
            (*(*dev).pcm).streams[SNDRV_PCM_STREAM_PLAYBACK].dev.as_mut().unwrap().parent =
                &mut (*dev).ofdev.dev;
            (*i2sdev).out.created = 1;
        }

        if (*i2sdev).in_.created == 0 && in_ != 0 {
            if (*(*dev).pcm).card != card {
                printk(c"Can't attach same bus to different cards!\n".as_ptr());
                module_put((*ci).owner);
                module_put(THIS_MODULE);
                soundbus_dev_put(dev);
                kfree(cii as *mut c_void);
                return -EINVAL;
            }
            err = snd_pcm_new_stream((*dev).pcm, SNDRV_PCM_STREAM_CAPTURE as c_int, 1);
            if err != 0 {
                module_put((*ci).owner);
                module_put(THIS_MODULE);
                soundbus_dev_put(dev);
                kfree(cii as *mut c_void);
                return err;
            }
            snd_pcm_set_ops((*dev).pcm, SNDRV_PCM_STREAM_CAPTURE as c_int, &i2sbus_record_ops);
            (*(*dev).pcm).streams[SNDRV_PCM_STREAM_CAPTURE].dev.as_mut().unwrap().parent =
                &mut (*dev).ofdev.dev;
            (*i2sdev).in_.created = 1;
        }

        /* so we have to register the pcm after adding any substream
         * to it because alsa doesn't create the devices for the
         * substreams when we add them later.
         * Therefore, force in and out on both busses (above) and
         * register the pcm now instead of just after creating it.
         */
        err = snd_device_register(card, (*dev).pcm);
        if err != 0 {
            printk(c"i2sbus: error registering new pcm\n".as_ptr());
            module_put((*ci).owner);
            module_put(THIS_MODULE);
            soundbus_dev_put(dev);
            kfree(cii as *mut c_void);
            return err;
        }
        /* no errors any more, so let's add this to our list */
        list_add(&mut (*cii).list, &mut (*dev).codec_list);

        (*(*dev).pcm).private_data = i2sdev as *mut c_void;
        (*(*dev).pcm).private_free = Some(i2sbus_private_free);

        /* well, we really should support scatter/gather DMA */
        snd_pcm_set_managed_buffer_all(
            (*dev).pcm,
            SNDRV_DMA_TYPE_DEV,
            &mut (*macio_get_pci_dev((*i2sdev).macio)).dev,
            64 * 1024,
            64 * 1024,
        );

        0
    }
}

unsafe fn goto_out_free_cii(cii: *mut codec_info_item, err: c_int) -> c_int {
    kfree(cii as *mut c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn i2sbus_detach_codec(dev: *mut soundbus_dev, data: *mut c_void) {
    let mut cii: *mut codec_info_item = ptr::null_mut();
    for_each_codec(&mut (*dev).codec_list, |i| unsafe {
        if (*i).codec_data == data {
            cii = i;
        }
    });
    if !cii.is_null() {
        list_del(&mut (*cii).list);
        module_put((*(*cii).codec).owner);
        kfree(cii as *mut c_void);
    }
    /* no more codecs, but still a pcm? */
    if list_empty(&mut (*dev).codec_list) != 0 && !(*dev).pcm.is_null() {
        /* the actual cleanup is done by the callback above! */
        snd_device_free((*(*dev).pcm).card, (*dev).pcm);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
