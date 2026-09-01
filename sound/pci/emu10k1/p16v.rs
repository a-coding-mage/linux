// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by James Courtier-Dutton <James@superbug.demon.co.uk>
 *  Driver p16v chips
 *  Version: 0.25
 *
 *  FEATURES currently supported:
 *    Output fixed at S32_LE, 2 channel to hw:0,0
 *    Rates: 44.1, 48, 96, 192.
 *
 *  Changelog:
 *  0.8
 *    Use separate card based buffer for periods table.
 *  0.9
 *    Use 2 channel output streams instead of 8 channel.
 *       (8 channel output streams might be good for ASIO type output)
 *    Corrected speaker output, so Front -> Front etc.
 *  0.10
 *    Fixed missed interrupts.
 *  0.11
 *    Add Sound card model number and names.
 *    Add Analog volume controls.
 *  0.12
 *    Corrected playback interrupts. Now interrupt per period, instead of half period.
 *  0.13
 *    Use single trigger for multichannel.
 *  0.14
 *    Mic capture now works at fixed: S32_LE, 96000Hz, Stereo.
 *  0.15
 *    Force buffer_size / period_size == INTEGER.
 *  0.16
 *    Update p16v.c to work with changed alsa api.
 *  0.17
 *    Update p16v.c to work with changed alsa api. Removed boot_devs.
 *  0.18
 *    Merging with snd-emu10k1 driver.
 *  0.19
 *    One stereo channel at 24bit now works.
 *  0.20
 *    Added better register defines.
 *  0.21
 *    Integrated with snd-emu10k1 driver.
 *  0.22
 *    Removed #if 0 ... #endif
 *  0.23
 *    Implement different capture rates.
 *  0.24
 *    Implement different capture source channels.
 *    e.g. When HD Capture source is set to SPDIF,
 *    setting HD Capture channel to 0 captures from CDROM digital input.
 *    setting HD Capture channel to 1 captures from SPDIF in.
 *  0.25
 *    Include capture buffer sizes.
 *
 *  BUGS:
 *    Some stability problems when unloading the snd-p16v kernel module.
 *    --
 *
 *  TODO:
 *    SPDIF out.
 *    Find out how to change capture sample rates. E.g. To record SPDIF at 48000Hz.
 *    Currently capture fixed at 48000Hz.
 *
 *    --
 *  GENERAL INFO:
 *    Model: SB0240
 *    P16V Chip: CA0151-DBS
 *    Audigy 2 Chip: CA0102-IAT
 *    AC97 Codec: STAC 9721
 *    ADC: Philips 1361T (Stereo 24bit)
 *    DAC: CS4382-K (8-channel, 24bit, 192Khz)
 *
 *  This code was initially based on code from ALSA's emu10k1x.c which is:
 *  Copyright (c) by Francisco Moraes <fmoraes@nc.rr.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const SET_CHANNEL: c_int = 0; /* Testing channel outputs 0=Front, 1=Center/LFE, 2=Unknown, 3=Rear */
const PCM_FRONT_CHANNEL: c_int = 0;
const PCM_REAR_CHANNEL: c_int = 1;
const PCM_CENTER_LFE_CHANNEL: c_int = 2;
const PCM_SIDE_CHANNEL: c_int = 3;
const CONTROL_FRONT_CHANNEL: c_int = 0;
const CONTROL_REAR_CHANNEL: c_int = 3;
const CONTROL_CENTER_LFE_CHANNEL: c_int = 1;
const CONTROL_SIDE_CHANNEL: c_int = 2;

/* Card IDs:
 * Class 0401: 1102:0004 (rev 04) Subsystem: 1102:2002 -> Audigy2 ZS 7.1 Model:SB0350
 * Class 0401: 1102:0004 (rev 04) Subsystem: 1102:1007 -> Audigy2 6.1    Model:SB0240
 * Class 0401: 1102:0004 (rev 04) Subsystem: 1102:1002 -> Audigy2 Platinum  Model:SB msb0240230009266
 * Class 0401: 1102:0004 (rev 04) Subsystem: 1102:2007 -> Audigy4 Pro Model:SB0380 M1SB0380472001901E
 *
 */

type u32 = u32;
type snd_pcm_uframes_t = usize;

extern "C" {
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint;
    static SNDRV_PCM_INFO_RESUME: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_SYNC_START: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_HW_PARAM_PERIODS: c_int;
    static SNDRV_PCM_IOCTL1_SYNC_ID: c_uint;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: usize;
    static SNDRV_PCM_STREAM_CAPTURE: usize;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static SNDRV_PCM_SUBCLASS_GENERIC_MIX: c_int;
    static SNDRV_CTL_ELEM_TYPE_INTEGER: c_int;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_int;
    static SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint;

    static A_SPDIF_SAMPLERATE: c_int;
    static A_SPDIF_RATE_MASK: u32;
    static A_EHC_SRC48_MASK: u32;
    static A_SPDIF_44100: u32;
    static A_EHC_SRC48_44: u32;
    static A_SPDIF_96000: u32;
    static A_EHC_SRC48_96: u32;
    static A_SPDIF_192000: u32;
    static A_EHC_SRC48_192: u32;
    static A_SPDIF_48000: u32;
    static A_EHC_SRC48_BYPASS: u32;
    static A_I2S_CAPTURE_RATE: c_int;
    static A_I2S_CAPTURE_44100: u32;
    static A_I2S_CAPTURE_96000: u32;
    static A_I2S_CAPTURE_192000: u32;
    static A_I2S_CAPTURE_48000: u32;
    static PLAYBACK_LIST_ADDR: c_int;
    static PLAYBACK_LIST_SIZE: c_int;
    static PLAYBACK_LIST_PTR: c_int;
    static PLAYBACK_DMA_ADDR: c_int;
    static PLAYBACK_PERIOD_SIZE: c_int;
    static PLAYBACK_POINTER: c_int;
    static PLAYBACK_FIFO_END_ADDRESS: c_int;
    static PLAYBACK_FIFO_POINTER: c_int;
    static CAPTURE_FIFO_POINTER: c_int;
    static CAPTURE_DMA_ADDR: c_int;
    static CAPTURE_BUFFER_SIZE: c_int;
    static CAPTURE_POINTER: c_int;
    static CAPTURE_P16V_SOURCE: c_int;
    static BASIC_INTERRUPT: c_int;
    static INTE2: usize;
    static IPR2: usize;
    static INTE2_PLAYBACK_CH_0_LOOP: u32;
    static INTE2_CAPTURE_CH_0_LOOP: u32;
    static INTE2_CAPTURE_CH_0_HALF_LOOP: u32;
    static PLAYBACK_VOLUME_MIXER9: c_int;
    static PLAYBACK_VOLUME_MIXER10: c_int;
    static PLAYBACK_VOLUME_MIXER7: c_int;
    static PLAYBACK_VOLUME_MIXER8: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
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
    pub fifo_size: usize,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub rate: c_uint,
    pub format: c_uint,
    pub channels: c_uint,
    pub buffer_size: snd_pcm_uframes_t,
    pub period_size: snd_pcm_uframes_t,
    pub periods: c_uint,
    pub frame_bits: c_uint,
    pub dma_addr: u32,
    pub dma_area: *mut c_void,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm {
    pub device: c_int,
    pub private_data: *mut c_void,
    pub streams: [snd_pcm_stream; 2],
    pub info_flags: c_uint,
    pub dev_subclass: c_int,
    pub name: [c_char; 80],
}

#[repr(C)]
pub struct snd_pcm_stream {
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub pcm: *mut snd_pcm,
    pub stream: c_int,
    pub next: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub area: *mut c_void,
    pub addr: u32,
    pub bytes: usize,
}

#[repr(C)]
pub struct snd_emu10k1 {
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub p16v_device_offset: c_int,
    pub p16v_buffer: *mut snd_dma_buffer,
    pub pcm_p16v: *mut snd_pcm,
    pub p16v_interrupt: Option<unsafe extern "C" fn(*mut snd_emu10k1)>,
    pub p16v_capture_source: c_uint,
    pub p16v_capture_channel: c_uint,
    pub p16v_saved: *mut c_uint,
    pub port: usize,
    pub emu_lock: spinlock_t,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_integer_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_integer_info {
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub union snd_kcontrol_tlv {
    pub p: *const c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub tlv: snd_kcontrol_tlv,
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub ioctl: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_uint, *mut c_void) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

extern "C" {
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_emu10k1;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_set_sync_per_card(
        substream: *mut snd_pcm_substream,
        arg: *mut c_void,
        id: *const u8,
        len: c_int,
    );
    fn snd_pcm_lib_ioctl(substream: *mut snd_pcm_substream, cmd: c_uint, arg: *mut c_void) -> c_int;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> u32;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: u32) -> snd_pcm_uframes_t;
    fn snd_emu10k1_ptr_read(emu: *mut snd_emu10k1, reg: c_int, ch: c_int) -> u32;
    fn snd_emu10k1_ptr_write(emu: *mut snd_emu10k1, reg: c_int, ch: c_int, data: u32);
    fn snd_emu10k1_ptr20_read(emu: *mut snd_emu10k1, reg: c_int, ch: c_int) -> u32;
    fn snd_emu10k1_ptr20_write(emu: *mut snd_emu10k1, reg: c_int, ch: c_int, data: u32);
    fn inl(port: usize) -> c_uint;
    fn outl(value: c_uint, port: usize);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_trigger_done(s: *mut snd_pcm_substream, master: *mut snd_pcm_substream);
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: usize, ops: *const snd_pcm_ops);
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snd_pcm_set_managed_buffer(
        substream: *mut snd_pcm_substream,
        type_: c_int,
        data: *mut device,
        size: usize,
        max: usize,
    );
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_emu10k1;
    fn snd_ctl_enum_info(
        uinfo: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        texts: *const *const c_char,
    ) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn vmalloc(size: usize) -> *mut c_uint;
    fn vfree(addr: *mut c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
}

unsafe fn array_size(a: usize, b: usize) -> usize {
    a.wrapping_mul(b)
}

/* hardware definition */
static mut snd_p16v_playback_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,
    formats: 0,
    rates: 0,
    rate_min: 44100,
    rate_max: 192000,
    channels_min: 8,
    channels_max: 8,
    buffer_bytes_max: (65536 - 64) * 8,
    period_bytes_min: 64,
    period_bytes_max: 65536 - 64,
    periods_min: 2,
    periods_max: 8,
    fifo_size: 0,
};

static mut snd_p16v_capture_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,
    formats: 0,
    rates: 0,
    rate_min: 44100,
    rate_max: 192000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 65536 - 64,
    period_bytes_min: 64,
    period_bytes_max: (65536 - 128) >> 1, /* size has to be N*64 bytes */
    periods_min: 2,
    periods_max: 2,
    fifo_size: 0,
};

unsafe fn init_hardware_constants() {
    snd_p16v_playback_hw.info = SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_SYNC_START;
    snd_p16v_playback_hw.formats = SNDRV_PCM_FMTBIT_S32_LE; /* Only supports 24-bit samples padded to 32 bits. */
    snd_p16v_playback_hw.rates =
        SNDRV_PCM_RATE_192000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_44100;

    snd_p16v_capture_hw.info = SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_MMAP_VALID;
    snd_p16v_capture_hw.formats = SNDRV_PCM_FMTBIT_S32_LE;
    snd_p16v_capture_hw.rates =
        SNDRV_PCM_RATE_192000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_44100;
}

/* open_playback callback */
unsafe extern "C" fn snd_p16v_pcm_open_playback_channel(
    substream: *mut snd_pcm_substream,
    _channel_id: c_int,
) -> c_int {
    let runtime = (*substream).runtime;
    let err: c_int;

    /*
    dev_dbg(emu->card->dev, "epcm device=%d, channel_id=%d\n",
           substream->pcm->device, channel_id);
    */

    init_hardware_constants();
    (*runtime).hw = snd_p16v_playback_hw;

    /*
    #if 0 debug
    dev_dbg(emu->card->dev,
           "p16v: open channel_id=%d, channel=%p, use=0x%x\n",
           channel_id, channel, channel->use);
    dev_dbg(emu->card->dev, "open:channel_id=%d, chip=%p, channel=%p\n",
           channel_id, chip, channel);
    #endif debug
    */
    /* channel->interrupt = snd_p16v_pcm_channel_interrupt; */
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }

    0
}

/* open_capture callback */
unsafe extern "C" fn snd_p16v_pcm_open_capture_channel(
    substream: *mut snd_pcm_substream,
    _channel_id: c_int,
) -> c_int {
    let runtime = (*substream).runtime;
    let err: c_int;

    /*
    dev_dbg(emu->card->dev, "epcm device=%d, channel_id=%d\n",
           substream->pcm->device, channel_id);
    */

    init_hardware_constants();
    (*runtime).hw = snd_p16v_capture_hw;

    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }

    0
}

/* close callback */
unsafe extern "C" fn snd_p16v_pcm_close_playback(_substream: *mut snd_pcm_substream) -> c_int {
    0
}

/* close callback */
unsafe extern "C" fn snd_p16v_pcm_close_capture(_substream: *mut snd_pcm_substream) -> c_int {
    0
}

unsafe extern "C" fn snd_p16v_pcm_open_playback_front(substream: *mut snd_pcm_substream) -> c_int {
    snd_p16v_pcm_open_playback_channel(substream, PCM_FRONT_CHANNEL)
}

unsafe extern "C" fn snd_p16v_pcm_open_capture(substream: *mut snd_pcm_substream) -> c_int {
    // Only using channel 0 for now, but the card has 2 channels.
    snd_p16v_pcm_open_capture_channel(substream, 0)
}

unsafe extern "C" fn snd_p16v_pcm_ioctl_playback(
    substream: *mut snd_pcm_substream,
    cmd: c_uint,
    arg: *mut c_void,
) -> c_int {
    if cmd == SNDRV_PCM_IOCTL1_SYNC_ID {
        static ID: [u8; 4] = [b'P', b'1', b'6', b'V'];
        snd_pcm_set_sync_per_card(substream, arg, ID.as_ptr(), 4);
        return 0;
    }
    snd_pcm_lib_ioctl(substream, cmd, arg)
}

/* prepare playback callback */
unsafe extern "C" fn snd_p16v_pcm_prepare_playback(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let channel = (*(*substream).pcm).device - (*emu).p16v_device_offset;
    let table_base = ((*(*emu).p16v_buffer).area as *mut u8).add(8 * 16 * channel as usize) as *mut u32;
    let period_size_bytes = frames_to_bytes(runtime, (*runtime).period_size);
    let mut i: c_int;
    let mut tmp: u32;

    /*
    #if 0 debug
    dev_dbg(emu->card->dev,
        "prepare:channel_number=%d, rate=%d, "
           "format=0x%x, channels=%d, buffer_size=%ld, "
           "period_size=%ld, periods=%u, frames_to_bytes=%d\n",
           channel, runtime->rate, runtime->format, runtime->channels,
           runtime->buffer_size, runtime->period_size,
           runtime->periods, frames_to_bytes(runtime, 1));
    dev_dbg(emu->card->dev,
        "dma_addr=%x, dma_area=%p, table_base=%p\n",
           runtime->dma_addr, runtime->dma_area, table_base);
    dev_dbg(emu->card->dev,
        "dma_addr=%x, dma_area=%p, dma_bytes(size)=%x\n",
           emu->p16v_buffer->addr, emu->p16v_buffer->area,
           emu->p16v_buffer->bytes);
    #endif debug
    */
    tmp = snd_emu10k1_ptr_read(emu, A_SPDIF_SAMPLERATE, channel);
    tmp &= !(A_SPDIF_RATE_MASK | A_EHC_SRC48_MASK);
    match (*runtime).rate {
        44100 => snd_emu10k1_ptr_write(
            emu,
            A_SPDIF_SAMPLERATE,
            channel,
            tmp | A_SPDIF_44100 | A_EHC_SRC48_44,
        ),
        96000 => snd_emu10k1_ptr_write(
            emu,
            A_SPDIF_SAMPLERATE,
            channel,
            tmp | A_SPDIF_96000 | A_EHC_SRC48_96,
        ),
        192000 => snd_emu10k1_ptr_write(
            emu,
            A_SPDIF_SAMPLERATE,
            channel,
            tmp | A_SPDIF_192000 | A_EHC_SRC48_192,
        ),
        48000 | _ => snd_emu10k1_ptr_write(
            emu,
            A_SPDIF_SAMPLERATE,
            channel,
            tmp | A_SPDIF_48000 | A_EHC_SRC48_BYPASS,
        ),
    }
    /* FIXME: Check emu->buffer.size before actually writing to it. */
    i = 0;
    while i < (*runtime).periods as c_int {
        *table_base.add((i * 2) as usize) =
            (*runtime).dma_addr.wrapping_add((i as u32).wrapping_mul(period_size_bytes));
        *table_base.add((i * 2 + 1) as usize) = period_size_bytes << 16;
        i += 1;
    }

    snd_emu10k1_ptr20_write(
        emu,
        PLAYBACK_LIST_ADDR,
        channel,
        (*(*emu).p16v_buffer).addr.wrapping_add((8 * 16 * channel) as u32),
    );
    snd_emu10k1_ptr20_write(emu, PLAYBACK_LIST_SIZE, channel, ((*runtime).periods - 1) << 19);
    snd_emu10k1_ptr20_write(emu, PLAYBACK_LIST_PTR, channel, 0);
    snd_emu10k1_ptr20_write(emu, PLAYBACK_DMA_ADDR, channel, (*runtime).dma_addr);
    //snd_emu10k1_ptr20_write(emu, PLAYBACK_PERIOD_SIZE, channel, frames_to_bytes(runtime, runtime->period_size)<<16); // buffer size in bytes
    snd_emu10k1_ptr20_write(emu, PLAYBACK_PERIOD_SIZE, channel, 0); // buffer size in bytes
    snd_emu10k1_ptr20_write(emu, PLAYBACK_POINTER, channel, 0);
    snd_emu10k1_ptr20_write(emu, PLAYBACK_FIFO_END_ADDRESS, channel, 0);
    snd_emu10k1_ptr20_write(emu, PLAYBACK_FIFO_POINTER, channel, 0);

    0
}

/* prepare capture callback */
unsafe extern "C" fn snd_p16v_pcm_prepare_capture(substream: *mut snd_pcm_substream) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let channel = (*(*substream).pcm).device - (*emu).p16v_device_offset;

    /*
    dev_dbg(emu->card->dev, "prepare capture:channel_number=%d, rate=%d, "
           "format=0x%x, channels=%d, buffer_size=%ld, period_size=%ld, "
           "frames_to_bytes=%d\n",
           channel, runtime->rate, runtime->format, runtime->channels,
           runtime->buffer_size, runtime->period_size,
           frames_to_bytes(runtime, 1));
    */
    match (*runtime).rate {
        44100 => snd_emu10k1_ptr_write(emu, A_I2S_CAPTURE_RATE, channel, A_I2S_CAPTURE_44100),
        96000 => snd_emu10k1_ptr_write(emu, A_I2S_CAPTURE_RATE, channel, A_I2S_CAPTURE_96000),
        192000 => snd_emu10k1_ptr_write(emu, A_I2S_CAPTURE_RATE, channel, A_I2S_CAPTURE_192000),
        48000 | _ => snd_emu10k1_ptr_write(emu, A_I2S_CAPTURE_RATE, channel, A_I2S_CAPTURE_48000),
    }
    /* FIXME: Check emu->buffer.size before actually writing to it. */
    snd_emu10k1_ptr20_write(emu, CAPTURE_FIFO_POINTER, channel, 0);
    snd_emu10k1_ptr20_write(emu, CAPTURE_DMA_ADDR, channel, (*runtime).dma_addr);
    snd_emu10k1_ptr20_write(
        emu,
        CAPTURE_BUFFER_SIZE,
        channel,
        frames_to_bytes(runtime, (*runtime).buffer_size) << 16,
    ); // buffer size in bytes
    snd_emu10k1_ptr20_write(emu, CAPTURE_POINTER, channel, 0);
    //snd_emu10k1_ptr20_write(emu, CAPTURE_SOURCE, 0x0, 0x333300e4); /* Select MIC or Line in */
    //snd_emu10k1_ptr20_write(emu, EXTENDED_INT_MASK, 0, snd_emu10k1_ptr20_read(emu, EXTENDED_INT_MASK, 0) | (0x110000<<channel));

    0
}

unsafe fn snd_p16v_intr_enable(emu: *mut snd_emu10k1, intrenb: c_uint) {
    let mut enable: c_uint;
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut (*emu).emu_lock, &mut flags);
    enable = inl((*emu).port + INTE2) | intrenb;
    outl(enable, (*emu).port + INTE2);
    spin_unlock_irqrestore(&mut (*emu).emu_lock, flags);
}

unsafe fn snd_p16v_intr_disable(emu: *mut snd_emu10k1, intrenb: c_uint) {
    let mut disable: c_uint;
    let mut flags: usize = 0;

    spin_lock_irqsave(&mut (*emu).emu_lock, &mut flags);
    disable = inl((*emu).port + INTE2) & !intrenb;
    outl(disable, (*emu).port + INTE2);
    spin_unlock_irqrestore(&mut (*emu).emu_lock, flags);
}

unsafe extern "C" fn snd_p16v_interrupt(emu: *mut snd_emu10k1) {
    let mut status: c_uint;

    status = inl((*emu).port + IPR2);
    while status != 0 {
        let mask: u32 = INTE2_PLAYBACK_CH_0_LOOP; /* Full Loop */

        /* dev_dbg(emu->card->dev, "p16v status=0x%x\n", status); */
        if status & mask != 0 {
            let substream = (*(*emu).pcm_p16v).streams[SNDRV_PCM_STREAM_PLAYBACK].substream;
            let runtime = (*substream).runtime;

            if !runtime.is_null() && !(*runtime).private_data.is_null() {
                snd_pcm_period_elapsed(substream);
            } else {
                dev_err(
                    (*(*emu).card).dev,
                    b"p16v: status: 0x%08x, mask=0x%08x\n\0".as_ptr() as *const c_char,
                    status,
                    mask,
                );
            }
        }
        if status & 0x110000 != 0 {
            let substream = (*(*emu).pcm_p16v).streams[SNDRV_PCM_STREAM_CAPTURE].substream;
            let runtime = (*substream).runtime;

            /* dev_info(emu->card->dev, "capture int found\n"); */
            if !runtime.is_null() && !(*runtime).private_data.is_null() {
                /* dev_info(emu->card->dev, "capture period_elapsed\n"); */
                snd_pcm_period_elapsed(substream);
            }
        }
        outl(status, (*emu).port + IPR2); /* ack all */
        status = inl((*emu).port + IPR2);
    }
}

/* trigger_playback callback */
unsafe extern "C" fn snd_p16v_pcm_trigger_playback(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let mut runtime: *mut snd_pcm_runtime;
    let mut channel: c_int;
    let mut result: c_int = 0;
    let mut s: *mut snd_pcm_substream;
    let mut basic: u32 = 0;
    let mut inte: u32 = 0;
    let mut running: c_int = 0;

    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START => running = 1,
        x if x == SNDRV_PCM_TRIGGER_STOP => running = 0,
        _ => running = 0,
    }
    s = substream;
    while !s.is_null() {
        if snd_pcm_substream_chip(s) != emu || (*s).stream != SNDRV_PCM_STREAM_PLAYBACK as c_int {
            s = (*s).next;
            continue;
        }
        runtime = (*s).runtime;
        channel = (*(*substream).pcm).device - (*emu).p16v_device_offset;
        /* dev_dbg(emu->card->dev, "p16v channel=%d\n", channel); */
        (*runtime).private_data = running as isize as *mut c_void;
        basic |= 0x1 << channel;
        inte |= INTE2_PLAYBACK_CH_0_LOOP << channel;
        snd_pcm_trigger_done(s, substream);
        s = (*s).next;
    }
    /* dev_dbg(emu->card->dev, "basic=0x%x, inte=0x%x\n", basic, inte); */

    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START => {
            snd_p16v_intr_enable(emu, inte);
            snd_emu10k1_ptr20_write(
                emu,
                BASIC_INTERRUPT,
                0,
                snd_emu10k1_ptr20_read(emu, BASIC_INTERRUPT, 0) | basic,
            );
        }
        x if x == SNDRV_PCM_TRIGGER_STOP => {
            snd_emu10k1_ptr20_write(
                emu,
                BASIC_INTERRUPT,
                0,
                snd_emu10k1_ptr20_read(emu, BASIC_INTERRUPT, 0) & !basic,
            );
            snd_p16v_intr_disable(emu, inte);
        }
        _ => {
            result = -EINVAL;
        }
    }
    result
}

/* trigger_capture callback */
unsafe extern "C" fn snd_p16v_pcm_trigger_capture(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let channel: c_int = 0;
    let mut result: c_int = 0;
    let inte: u32 = INTE2_CAPTURE_CH_0_LOOP | INTE2_CAPTURE_CH_0_HALF_LOOP;

    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START => {
            snd_p16v_intr_enable(emu, inte);
            snd_emu10k1_ptr20_write(
                emu,
                BASIC_INTERRUPT,
                0,
                snd_emu10k1_ptr20_read(emu, BASIC_INTERRUPT, 0) | (0x100 << channel),
            );
            (*runtime).private_data = 1usize as *mut c_void;
        }
        x if x == SNDRV_PCM_TRIGGER_STOP => {
            snd_emu10k1_ptr20_write(
                emu,
                BASIC_INTERRUPT,
                0,
                snd_emu10k1_ptr20_read(emu, BASIC_INTERRUPT, 0) & !(0x100 << channel),
            );
            snd_p16v_intr_disable(emu, inte);
            //snd_emu10k1_ptr20_write(emu, EXTENDED_INT_MASK, 0, snd_emu10k1_ptr20_read(emu, EXTENDED_INT_MASK, 0) & ~(0x110000<<channel));
            (*runtime).private_data = ptr::null_mut();
        }
        _ => {
            result = -EINVAL;
        }
    }
    result
}

/* pointer_playback callback */
unsafe extern "C" fn snd_p16v_pcm_pointer_playback(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut ptr_: snd_pcm_uframes_t;
    let mut ptr1: snd_pcm_uframes_t;
    let mut ptr2: snd_pcm_uframes_t;
    let ptr3: snd_pcm_uframes_t;
    let ptr4: snd_pcm_uframes_t;
    let channel = (*(*substream).pcm).device - (*emu).p16v_device_offset;

    if (*runtime).private_data.is_null() {
        return 0;
    }

    ptr3 = snd_emu10k1_ptr20_read(emu, PLAYBACK_LIST_PTR, channel) as snd_pcm_uframes_t;
    ptr1 = snd_emu10k1_ptr20_read(emu, PLAYBACK_POINTER, channel) as snd_pcm_uframes_t;
    ptr4 = snd_emu10k1_ptr20_read(emu, PLAYBACK_LIST_PTR, channel) as snd_pcm_uframes_t;
    if ptr3 != ptr4 {
        ptr1 = snd_emu10k1_ptr20_read(emu, PLAYBACK_POINTER, channel) as snd_pcm_uframes_t;
    }
    ptr2 = bytes_to_frames(runtime, ptr1 as u32);
    ptr2 += (ptr4 >> 3) * (*runtime).period_size;
    ptr_ = ptr2;
    if ptr_ >= (*runtime).buffer_size {
        ptr_ -= (*runtime).buffer_size;
    }

    ptr_
}

/* pointer_capture callback */
unsafe extern "C" fn snd_p16v_pcm_pointer_capture(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let emu = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut ptr_: snd_pcm_uframes_t;
    let ptr1: snd_pcm_uframes_t;
    let mut ptr2: snd_pcm_uframes_t = 0;
    let channel: c_int = 0;

    if (*runtime).private_data.is_null() {
        return 0;
    }

    ptr1 = snd_emu10k1_ptr20_read(emu, CAPTURE_POINTER, channel) as snd_pcm_uframes_t;
    ptr2 = bytes_to_frames(runtime, ptr1 as u32);
    ptr_ = ptr2;
    if ptr_ >= (*runtime).buffer_size {
        ptr_ -= (*runtime).buffer_size;
        dev_warn((*(*emu).card).dev, b"buffer capture limited!\n\0".as_ptr() as *const c_char);
    }
    /*
    dev_dbg(emu->card->dev, "ptr1 = 0x%lx, ptr2=0x%lx, ptr=0x%lx, "
           "buffer_size = 0x%x, period_size = 0x%x, bits=%d, rate=%d\n",
           ptr1, ptr2, ptr, (int)runtime->buffer_size,
           (int)runtime->period_size, (int)runtime->frame_bits,
           (int)runtime->rate);
    */
    ptr_
}

/* operators */
static snd_p16v_playback_front_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_p16v_pcm_open_playback_front),
    close: Some(snd_p16v_pcm_close_playback),
    ioctl: Some(snd_p16v_pcm_ioctl_playback),
    prepare: Some(snd_p16v_pcm_prepare_playback),
    trigger: Some(snd_p16v_pcm_trigger_playback),
    pointer: Some(snd_p16v_pcm_pointer_playback),
};

static snd_p16v_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_p16v_pcm_open_capture),
    close: Some(snd_p16v_pcm_close_capture),
    ioctl: None,
    prepare: Some(snd_p16v_pcm_prepare_capture),
    trigger: Some(snd_p16v_pcm_trigger_capture),
    pointer: Some(snd_p16v_pcm_pointer_capture),
};

#[no_mangle]
pub unsafe extern "C" fn snd_p16v_pcm(emu: *mut snd_emu10k1, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut substream: *mut snd_pcm_substream;
    let mut err: c_int;
    let capture: c_int = 1;

    /* dev_dbg(emu->card->dev, "snd_p16v_pcm called. device=%d\n", device); */
    (*emu).p16v_device_offset = device;

    err = snd_pcm_new((*emu).card, b"p16v\0".as_ptr() as *const c_char, device, 1, capture, &mut pcm);
    if err < 0 {
        return err;
    }

    (*pcm).private_data = emu as *mut c_void;
    // Single playback 8 channel device.
    // Single capture 2 channel device.
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_p16v_playback_front_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_p16v_capture_ops);

    (*pcm).info_flags = 0;
    (*pcm).dev_subclass = SNDRV_PCM_SUBCLASS_GENERIC_MIX;
    strscpy((*pcm).name.as_mut_ptr(), b"p16v\0".as_ptr() as *const c_char);
    (*emu).pcm_p16v = pcm;
    (*emu).p16v_interrupt = Some(snd_p16v_interrupt);

    substream = (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK].substream;
    while !substream.is_null() {
        snd_pcm_set_managed_buffer(
            substream,
            SNDRV_DMA_TYPE_DEV,
            &mut (*(*emu).pci).dev,
            (65536 - 64) * 8,
            (65536 - 64) * 8,
        );
        /*
        dev_dbg(emu->card->dev,
               "preallocate playback substream: err=%d\n", err);
        */
        substream = (*substream).next;
    }

    substream = (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE].substream;
    while !substream.is_null() {
        snd_pcm_set_managed_buffer(
            substream,
            SNDRV_DMA_TYPE_DEV,
            &mut (*(*emu).pci).dev,
            65536 - 64,
            65536 - 64,
        );
        /*
        dev_dbg(emu->card->dev,
               "preallocate capture substream: err=%d\n", err);
        */
        substream = (*substream).next;
    }

    0
}

unsafe extern "C" fn snd_p16v_volume_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 255;
    0
}

unsafe extern "C" fn snd_p16v_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu = snd_kcontrol_chip(kcontrol);
    let high_low = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let reg = ((*kcontrol).private_value & 0xff) as c_int;
    let value: u32;

    value = snd_emu10k1_ptr20_read(emu, reg, high_low);
    if high_low != 0 {
        (*ucontrol).value.integer.value[0] = (0xff - ((value >> 24) & 0xff)) as i64; /* Left */
        (*ucontrol).value.integer.value[1] = (0xff - ((value >> 16) & 0xff)) as i64; /* Right */
    } else {
        (*ucontrol).value.integer.value[0] = (0xff - ((value >> 8) & 0xff)) as i64; /* Left */
        (*ucontrol).value.integer.value[1] = (0xff - ((value >> 0) & 0xff)) as i64; /* Right */
    }
    0
}

unsafe extern "C" fn snd_p16v_volume_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu = snd_kcontrol_chip(kcontrol);
    let high_low = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let reg = ((*kcontrol).private_value & 0xff) as c_int;
    let mut value: u32;
    let oval: u32;

    value = snd_emu10k1_ptr20_read(emu, reg, 0);
    oval = value;
    if high_low == 1 {
        value &= 0xffff;
        value |= ((0xff - (*ucontrol).value.integer.value[0] as u32) << 24)
            | ((0xff - (*ucontrol).value.integer.value[1] as u32) << 16);
    } else {
        value &= 0xffff0000;
        value |= ((0xff - (*ucontrol).value.integer.value[0] as u32) << 8)
            | (0xff - (*ucontrol).value.integer.value[1] as u32);
    }
    if value != oval {
        snd_emu10k1_ptr20_write(emu, reg, 0, value);
        return 1;
    }
    0
}

unsafe extern "C" fn snd_p16v_capture_source_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXTS: [*const c_char; 8] = [
        b"SPDIF\0".as_ptr() as *const c_char,
        b"I2S\0".as_ptr() as *const c_char,
        b"SRC48\0".as_ptr() as *const c_char,
        b"SRCMulti_SPDIF\0".as_ptr() as *const c_char,
        b"SRCMulti_I2S\0".as_ptr() as *const c_char,
        b"CDIF\0".as_ptr() as *const c_char,
        b"FX\0".as_ptr() as *const c_char,
        b"AC97\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(uinfo, 1, 8, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_p16v_capture_source_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu = snd_kcontrol_chip(kcontrol);

    (*ucontrol).value.enumerated.item[0] = (*emu).p16v_capture_source;
    0
}

unsafe extern "C" fn snd_p16v_capture_source_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu = snd_kcontrol_chip(kcontrol);
    let val: c_uint;
    let mut change: c_int = 0;
    let mask: u32;
    let source: u32;

    val = (*ucontrol).value.enumerated.item[0];
    if val > 7 {
        return -EINVAL;
    }
    change = ((*emu).p16v_capture_source != val) as c_int;
    if change != 0 {
        (*emu).p16v_capture_source = val;
        source = (val << 28) | (val << 24) | (val << 20) | (val << 16);
        mask = snd_emu10k1_ptr20_read(emu, BASIC_INTERRUPT, 0) & 0xffff;
        snd_emu10k1_ptr20_write(emu, BASIC_INTERRUPT, 0, source | mask);
    }
    change
}

unsafe extern "C" fn snd_p16v_capture_channel_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXTS: [*const c_char; 4] = [
        b"0\0".as_ptr() as *const c_char,
        b"1\0".as_ptr() as *const c_char,
        b"2\0".as_ptr() as *const c_char,
        b"3\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(uinfo, 1, 4, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_p16v_capture_channel_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu = snd_kcontrol_chip(kcontrol);

    (*ucontrol).value.enumerated.item[0] = (*emu).p16v_capture_channel;
    0
}

unsafe extern "C" fn snd_p16v_capture_channel_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let emu = snd_kcontrol_chip(kcontrol);
    let val: c_uint;
    let mut change: c_int = 0;
    let tmp: u32;

    val = (*ucontrol).value.enumerated.item[0];
    if val > 3 {
        return -EINVAL;
    }
    change = ((*emu).p16v_capture_channel != val) as c_int;
    if change != 0 {
        (*emu).p16v_capture_channel = val;
        tmp = snd_emu10k1_ptr20_read(emu, CAPTURE_P16V_SOURCE, 0) & 0xfffc;
        snd_emu10k1_ptr20_write(emu, CAPTURE_P16V_SOURCE, 0, tmp | val);
    }
    change
}

static snd_p16v_db_scale1: [c_uint; 4] = [0, (-5175i32) as c_uint, 25, 1];

macro_rules! P16V_VOL {
    ($xname:expr, $xreg:expr, $xhl:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname.as_ptr() as *const c_char,
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
            info: Some(snd_p16v_volume_info),
            get: Some(snd_p16v_volume_get),
            put: Some(snd_p16v_volume_put),
            tlv: snd_kcontrol_tlv {
                p: snd_p16v_db_scale1.as_ptr(),
            },
            private_value: (($xreg as usize) | (($xhl as usize) << 8)),
        }
    };
}

static p16v_mixer_controls: [snd_kcontrol_new; 10] = [
    P16V_VOL!(b"HD Analog Front Playback Volume\0", PLAYBACK_VOLUME_MIXER9, 0),
    P16V_VOL!(b"HD Analog Rear Playback Volume\0", PLAYBACK_VOLUME_MIXER10, 1),
    P16V_VOL!(b"HD Analog Center/LFE Playback Volume\0", PLAYBACK_VOLUME_MIXER9, 1),
    P16V_VOL!(b"HD Analog Side Playback Volume\0", PLAYBACK_VOLUME_MIXER10, 0),
    P16V_VOL!(b"HD SPDIF Front Playback Volume\0", PLAYBACK_VOLUME_MIXER7, 0),
    P16V_VOL!(b"HD SPDIF Rear Playback Volume\0", PLAYBACK_VOLUME_MIXER8, 1),
    P16V_VOL!(b"HD SPDIF Center/LFE Playback Volume\0", PLAYBACK_VOLUME_MIXER7, 1),
    P16V_VOL!(b"HD SPDIF Side Playback Volume\0", PLAYBACK_VOLUME_MIXER8, 0),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"HD source Capture\0".as_ptr() as *const c_char,
        access: 0,
        info: Some(snd_p16v_capture_source_info),
        get: Some(snd_p16v_capture_source_get),
        put: Some(snd_p16v_capture_source_put),
        tlv: snd_kcontrol_tlv { p: ptr::null() },
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"HD channel Capture\0".as_ptr() as *const c_char,
        access: 0,
        info: Some(snd_p16v_capture_channel_info),
        get: Some(snd_p16v_capture_channel_get),
        put: Some(snd_p16v_capture_channel_put),
        tlv: snd_kcontrol_tlv { p: ptr::null() },
        private_value: 0,
    },
];

#[no_mangle]
pub unsafe extern "C" fn snd_p16v_mixer(emu: *mut snd_emu10k1) -> c_int {
    let mut i: usize;
    let mut err: c_int;
    let card = (*emu).card;

    i = 0;
    while i < p16v_mixer_controls.len() {
        err = snd_ctl_add(card, snd_ctl_new1(&p16v_mixer_controls[i], emu as *mut c_void));
        if err < 0 {
            return err;
        }
        i += 1;
    }
    0
}

/* CONFIG_PM_SLEEP */

const NUM_CHS: c_int = 1; /* up to 4, but only first channel is used */

#[no_mangle]
pub unsafe extern "C" fn snd_p16v_alloc_pm_buffer(emu: *mut snd_emu10k1) -> c_int {
    (*emu).p16v_saved = vmalloc(array_size((NUM_CHS * 4) as usize, 0x80));
    if (*emu).p16v_saved.is_null() {
        return -ENOMEM;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_p16v_free_pm_buffer(emu: *mut snd_emu10k1) {
    vfree((*emu).p16v_saved);
}

#[no_mangle]
pub unsafe extern "C" fn snd_p16v_suspend(emu: *mut snd_emu10k1) {
    let mut i: c_int;
    let mut ch: c_int;
    let mut val: *mut c_uint;

    val = (*emu).p16v_saved;
    ch = 0;
    while ch < NUM_CHS {
        i = 0;
        while i < 0x80 {
            *val = snd_emu10k1_ptr20_read(emu, i, ch);
            val = val.add(1);
            i += 1;
        }
        ch += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_p16v_resume(emu: *mut snd_emu10k1) {
    let mut i: c_int;
    let mut ch: c_int;
    let mut val: *mut c_uint;

    val = (*emu).p16v_saved;
    ch = 0;
    while ch < NUM_CHS {
        i = 0;
        while i < 0x80 {
            snd_emu10k1_ptr20_write(emu, i, ch, *val);
            val = val.add(1);
            i += 1;
        }
        ch += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
