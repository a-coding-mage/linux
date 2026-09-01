// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) 2004 James Courtier-Dutton <James@superbug.demon.co.uk>
 *  Driver CA0106 chips. e.g. Sound Blaster Audigy LS and Live 24bit
 *  Version: 0.0.25
 *
 *  FEATURES currently supported:
 *    Front, Rear and Center/LFE.
 *    Surround40 and Surround51.
 *    Capture from MIC an LINE IN input.
 *    SPDIF digital playback of PCM stereo and AC3/DTS works.
 *    (One can use a standard mono mini-jack to one RCA plugs cable.
 *     or one can use a standard stereo mini-jack to two RCA plugs cable.
 *     Plug one of the RCA plugs into the Coax input of the external decoder/receiver.)
 *    ( In theory one could output 3 different AC3 streams at once, to 3 different SPDIF outputs. )
 *    Notes on how to capture sound:
 *      The AC97 is used in the PLAYBACK direction.
 *      The output from the AC97 chip, instead of reaching the speakers, is fed into the Philips 1361T ADC.
 *      So, to record from the MIC, set the MIC Playback volume to max,
 *      unmute the MIC and turn up the MASTER Playback volume.
 *      So, to prevent feedback when capturing, minimise the "Capture feedback into Playback" volume.
 *
 *    The only playback controls that currently do anything are: -
 *    Analog Front
 *    Analog Rear
 *    Analog Center/LFE
 *    SPDIF Front
 *    SPDIF Rear
 *    SPDIF Center/LFE
 *
 *    For capture from Mic in or Line in.
 *    Digital/Analog ( switch must be in Analog mode for CAPTURE. )
 *
 *    CAPTURE feedback into PLAYBACK
 *
 *  Changelog:
 *    Support interrupts per period.
 *    Removed noise from Center/LFE channel when in Analog mode.
 *    Rename and remove mixer controls.
 *  0.0.6
 *    Use separate card based DMA buffer for periods table list.
 *  0.0.7
 *    Change remove and rename ctrls into lists.
 *  0.0.8
 *    Try to fix capture sources.
 *  0.0.9
 *    Fix AC3 output.
 *    Enable S32_LE format support.
 *  0.0.10
 *    Enable playback 48000 and 96000 rates. (Rates other that these do not work, even with "plug:front".)
 *  0.0.11
 *    Add Model name recognition.
 *  0.0.12
 *    Correct interrupt timing. interrupt at end of period, instead of in the middle of a playback period.
 *    Remove redundent "voice" handling.
 *  0.0.13
 *    Single trigger call for multi channels.
 *  0.0.14
 *    Set limits based on what the sound card hardware can do.
 *    playback periods_min=2, periods_max=8
 *    capture hw constraints require period_size = n * 64 bytes.
 *    playback hw constraints require period_size = n * 64 bytes.
 *  0.0.15
 *    Minor updates.
 *  0.0.16
 *    Implement 192000 sample rate.
 *  0.0.17
 *    Add support for SB0410 and SB0413.
 *  0.0.18
 *    Modified Copyright message.
 *  0.0.19
 *    Finally fix support for SB Live 24 bit. SB0410 and SB0413.
 *    The output codec needs resetting, otherwise all output is muted.
 *  0.0.20
 *    Merge "pci_disable_device(pci);" fixes.
 *  0.0.21
 *    Add 4 capture channels. (SPDIF only comes in on channel 0. )
 *    Add SPDIF capture using optional digital I/O module for SB Live 24bit. (Analog capture does not yet work.)
 *  0.0.22
 *    Add support for MSI K8N Diamond Motherboard with onboard SB Live 24bit without AC97. From kiksen, bug #901
 *  0.0.23
 *    Implement support for Line-in capture on SB Live 24bit.
 *  0.0.24
 *    Add support for mute control on SB Live 24bit (cards w/ SPI DAC)
 *  0.0.25
 *    Powerdown SPI DAC channels when not in use
 *
 *  BUGS:
 *    Some stability problems when unloading the snd-ca0106 kernel module.
 *    --
 *
 *  TODO:
 *    4 Capture channels, only one implemented so far.
 *    Other capture rates apart from 48khz not implemented.
 *    MIDI
 *    --
 *  GENERAL INFO:
 *    Model: SB0310
 *    P17 Chip: CA0106-DAT
 *    AC97 Codec: STAC 9721
 *    ADC: Philips 1361T (Stereo 24bit)
 *    DAC: WM8746EDS (6-channel, 24bit, 192Khz)
 *
 *  GENERAL INFO:
 *    Model: SB0410
 *    P17 Chip: CA0106-DAT
 *    AC97 Codec: None
 *    ADC: WM8775EDS (4 Channel)
 *    DAC: CS4382 (114 dB, 24-Bit, 192 kHz, 8-Channel D/A Converter with DSD Support)
 *    SPDIF Out control switches between Mic in and SPDIF out.
 *    No sound out or mic input working yet.
 *
 *  GENERAL INFO:
 *    Model: SB0413
 *    P17 Chip: CA0106-DAT
 *    AC97 Codec: None.
 *    ADC: Unknown
 *    DAC: Unknown
 *    Trying to handle it like the SB0410.
 *
 *  This code was initially based on code from ALSA's emu10k1x.c which is:
 *  Copyright (c) by Francisco Moraes <fmoraes@nc.rr.com>
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = c_uint;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;

// Includes translated as external dependencies:
// linux/delay.h, linux/init.h, linux/interrupt.h, linux/pci.h, linux/slab.h,
// linux/module.h, linux/dma-mapping.h, sound/core.h, sound/initval.h,
// sound/pcm.h, sound/ac97_codec.h, sound/info.h, and "ca0106.h".
// Module metadata:
// MODULE_AUTHOR("James Courtier-Dutton <James@superbug.demon.co.uk>");
// MODULE_DESCRIPTION("CA0106");
// MODULE_LICENSE("GPL");

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;
static mut subsystem: [c_uint; SNDRV_CARDS] = [0; SNDRV_CARDS]; /* Force card subsystem model */

// module_param_array / MODULE_PARM_DESC declarations are module metadata in C.

#[repr(C)]
pub struct snd_ca0106_details {
    serial: c_uint,
    name: *const c_char,
    ac97: c_int,
    gpio_type: c_int,
    i2c_adc: c_int,
    spi_dac: c_int,
}

unsafe impl Sync for snd_ca0106_details {}

#[repr(C)]
pub struct snd_pcm_hardware {
    info: c_uint,
    formats: c_ulong,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    buffer_bytes_max: c_ulong,
    period_bytes_min: c_ulong,
    period_bytes_max: c_ulong,
    periods_min: c_uint,
    periods_max: c_uint,
    fifo_size: c_uint,
}

#[repr(C)]
pub struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct snd_pcm_chmap_elem {
    channels: c_uint,
    map: [c_uint; 8],
}

#[repr(C)]
pub struct snd_ca0106_pcm {
    emu: *mut snd_ca0106,
    substream: *mut snd_pcm_substream,
    channel_id: c_int,
    running: c_int,
}

#[repr(C)]
pub struct snd_ca0106_channel {
    emu: *mut snd_ca0106,
    number: c_int,
    use_: c_int,
    epcm: *mut snd_ca0106_pcm,
}

#[repr(C)]
pub struct snd_dma_buffer {
    area: *mut u8,
    addr: u32,
    bytes: u32,
}

#[repr(C)]
pub struct snd_ca_midi {
    dev_id: *mut c_void,
    tx_enable: c_int,
    rx_enable: c_int,
    ipr_tx: c_int,
    ipr_rx: c_int,
    port: c_int,
    reset: c_int,
    enter_uart: c_int,
    ack: c_int,
    input_avail: c_int,
    output_ready: c_int,
    channel: c_uint,
    interrupt_enable: Option<unsafe extern "C" fn(*mut snd_ca_midi, c_int)>,
    interrupt_disable: Option<unsafe extern "C" fn(*mut snd_ca_midi, c_int)>,
    read: Option<unsafe extern "C" fn(*mut snd_ca_midi, c_int) -> u8>,
    write: Option<unsafe extern "C" fn(*mut snd_ca_midi, c_int, c_int)>,
    get_dev_id_card: Option<unsafe extern "C" fn(*mut c_void) -> *mut snd_card>,
    get_dev_id_port: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    interrupt: Option<unsafe extern "C" fn(*mut snd_ca_midi, c_uint)>,
}

#[repr(C)]
pub struct snd_ca0106 {
    card: *mut snd_card,
    pci: *mut pci_dev,
    irq: c_int,
    emu_lock: spinlock_t,
    port: c_ulong,
    buffer: *mut snd_dma_buffer,
    serial: c_uint,
    model: u16,
    details: *const snd_ca0106_details,
    playback_channels: [snd_ca0106_channel; 4],
    capture_channels: [snd_ca0106_channel; 4],
    spdif_str_bits: [c_uint; 4],
    spdif_bits: [c_uint; 4],
    spdif_enable: c_int,
    capture_source: c_int,
    i2c_capture_volume: [[c_uint; 2]; 4],
    i2c_capture_source: c_int,
    spi_dac_reg: [c_uint; 16],
    pcm: [*mut snd_pcm; 4],
    ac97: *mut snd_ac97,
    midi: snd_ca_midi,
    midi2: snd_ca_midi,
}

#[repr(C)]
pub struct snd_card {
    dev: *mut device,
    private_data: *mut snd_ca0106,
    private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    sync_irq: c_int,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    private_data: *mut c_void,
    private_free: Option<unsafe extern "C" fn(*mut snd_pcm_runtime)>,
    hw: snd_pcm_hardware,
    period_size: snd_pcm_uframes_t,
    buffer_size: snd_pcm_uframes_t,
    periods: c_uint,
    rate: c_uint,
    format: c_uint,
    channels: c_uint,
    dma_addr: u32,
    dma_area: *mut c_void,
    frame_bits: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: c_int,
    next: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_pcm_stream {
    substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_pcm {
    private_data: *mut c_void,
    info_flags: c_uint,
    name: [c_char; 80],
    streams: [snd_pcm_stream; 2],
}

#[repr(C)]
pub struct snd_ac97 {
    private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_ac97_bus {
    no_vra: c_int,
}

#[repr(C)]
pub struct snd_ac97_template {
    private_data: *mut c_void,
    scaps: c_uint,
}

#[repr(C)]
pub struct snd_ac97_bus_ops {
    write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>,
    read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16>,
}

#[repr(C)]
pub struct pci_dev {
    dev: device,
    irq: c_int,
    revision: c_uint,
}

#[repr(C)]
pub struct pci_device_id {
    vendor: c_uint,
    device: c_uint,
}

#[repr(C)]
pub struct pci_driver {
    name: *const c_char,
    id_table: *const pci_device_id,
    probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pm: *const c_void,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    static snd_pcm_std_chmaps: *const snd_pcm_chmap_elem;
    static THIS_MODULE: *mut c_void;
    static KBUILD_MODNAME: *const c_char;

    fn outl(value: c_uint, port: c_ulong);
    fn inl(port: c_ulong) -> c_uint;
    fn outb(value: c_uint, port: c_ulong);
    fn inw(port: c_ulong) -> u16;
    fn outw(value: c_uint, port: c_ulong);
    fn udelay(usecs: c_uint);
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_ca0106;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_hw_constraint_step(runtime: *mut snd_pcm_runtime, cond: c_uint, param: c_int, step: c_ulong) -> c_int;
    fn snd_pcm_set_sync(substream: *mut snd_pcm_substream);
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn snd_pcm_trigger_done(s: *mut snd_pcm_substream, substream: *mut snd_pcm_substream);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, rbus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn snd_devm_alloc_pages(dev: *mut device, ty: c_int, size: usize) -> *mut snd_dma_buffer;
    fn pci_set_master(pci: *mut pci_dev);
    fn pci_read_config_dword(pci: *mut pci_dev, where_: c_int, val: *mut c_uint) -> c_int;
    fn pci_read_config_word(pci: *mut pci_dev, where_: c_int, val: *mut u16) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer(substream: *mut snd_pcm_substream, ty: c_int, dev: *mut device, size_min: usize, size_max: usize);
    fn snd_pcm_add_chmap_ctls(pcm: *mut snd_pcm, stream: c_int, map: *const snd_pcm_chmap_elem, max_channels: c_int, mask: c_uint, private_value: *mut c_void) -> c_int;
    fn ca_midi_init(chip: *mut snd_ca0106, midi: *mut snd_ca_midi, device: c_int, name: *const c_char) -> c_int;
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_ca0106_mixer(chip: *mut snd_ca0106) -> c_int;
    fn snd_ca0106_proc_init(chip: *mut snd_ca0106);
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn snd_ca0106_mixer_suspend(chip: *mut snd_ca0106);
    fn snd_ca0106_mixer_resume(chip: *mut snd_ca0106);
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const GFP_KERNEL: c_uint = 0;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_ulong = 0x80;
const DMA_BIT_MASK_32: u64 = (1u64 << 32) - 1;

// Constants supplied by ca0106.h and ALSA/Linux headers.
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 2;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 3;
const SNDRV_PCM_INFO_SYNC_START: c_uint = 1 << 4;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 1 << 1;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_192000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 3;
const SNDRV_PCM_FORMAT_S16_LE: c_uint = 2;
const SNDRV_PCM_FORMAT_S32_LE: c_uint = 10;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_CHMAP_RL: c_uint = 5;
const SNDRV_CHMAP_RR: c_uint = 6;
const SNDRV_CHMAP_FC: c_uint = 3;
const SNDRV_CHMAP_LFE: c_uint = 4;
const SNDRV_CHMAP_SL: c_uint = 7;
const SNDRV_CHMAP_SR: c_uint = 8;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const AC97_SCAP_NO_SPDIF: c_uint = 1;
const AC97_REC_GAIN: c_uint = 0x1c;
const PCI_SUBSYSTEM_VENDOR_ID: c_int = 0x2c;
const PCI_SUBSYSTEM_ID: c_int = 0x2e;

const CA0106_PTR: c_ulong = 0;
const CA0106_DATA: c_ulong = 4;
const CA0106_INTE: c_ulong = 8;
const CA0106_IPR: c_ulong = 12;
const CA0106_AC97ADDRESS: c_ulong = 16;
const CA0106_AC97DATA: c_ulong = 18;
const CA0106_HCFG: c_ulong = 20;
const CA0106_GPIO: c_ulong = 24;
const SPI: c_uint = 0;
const I2C_D0: c_uint = 0;
const I2C_D1: c_uint = 1;
const I2C_A: c_uint = 2;
const I2C_A_ADC_LAST: c_uint = 1 << 0;
const I2C_A_ADC_START: c_uint = 1 << 1;
const I2C_A_ADC_ADD: c_uint = 1 << 2;
const I2C_A_ADC_ABORT: c_uint = 1 << 3;
const HCFG_PLAYBACK_S32_LE: c_uint = 1 << 0;
const HCFG_CAPTURE_S32_LE: c_uint = 1 << 1;
const HCFG_AC97: c_uint = 1 << 2;
const HCFG_AUDIOENABLE: c_uint = 1 << 3;
const PCM_FRONT_CHANNEL: c_int = 0;
const PCM_REAR_CHANNEL: c_int = 1;
const PCM_CENTER_LFE_CHANNEL: c_int = 2;
const PCM_UNKNOWN_CHANNEL: c_int = 3;
const PLAYBACK_LIST_ADDR: c_uint = 0;
const PLAYBACK_LIST_SIZE: c_uint = 1;
const PLAYBACK_LIST_PTR: c_uint = 2;
const PLAYBACK_DMA_ADDR: c_uint = 3;
const PLAYBACK_PERIOD_SIZE: c_uint = 4;
const PLAYBACK_POINTER: c_uint = 5;
const PLAYBACK_MUTE: c_uint = 6;
const CAPTURE_DMA_ADDR: c_uint = 7;
const CAPTURE_BUFFER_SIZE: c_uint = 8;
const CAPTURE_POINTER: c_uint = 9;
const CAPTURE_MUTE: c_uint = 10;
const CAPTURE_CONTROL: c_uint = 11;
const PLAYBACK_ROUTING1: c_uint = 12;
const PLAYBACK_ROUTING2: c_uint = 13;
const CAPTURE_ROUTING1: c_uint = 14;
const CAPTURE_ROUTING2: c_uint = 15;
const CAPTURE_VOLUME1: c_uint = 16;
const CAPTURE_VOLUME2: c_uint = 17;
const CAPTURE_SOURCE: c_uint = 18;
const SPDIF_SELECT1: c_uint = 19;
const SPDIF_SELECT2: c_uint = 20;
const SPCS0: c_uint = 30;
const SPCS1: c_uint = 31;
const SPCS2: c_uint = 32;
const SPCS3: c_uint = 33;
const SPCS_CLKACCY_1000PPM: c_uint = 0;
const SPCS_SAMPLERATE_48: c_uint = 2 << 24;
const SPCS_CHANNELNUM_LEFT: c_uint = 1 << 20;
const SPCS_SOURCENUM_UNSPEC: c_uint = 0;
const SPCS_GENERATIONSTATUS: c_uint = 1 << 15;
const SPCS_EMPHASIS_NONE: c_uint = 0;
const SPCS_COPYRIGHT: c_uint = 1 << 2;
const EXTENDED_INT_MASK: c_uint = 40;
const BASIC_INTERRUPT: c_uint = 41;
const EXTENDED_INT: c_uint = 42;
const ADC_MASTER: c_uint = 0x0c;
const ADC_MUX_LINEIN: c_uint = 0x15;
const ADC_MUX: c_uint = 0x15;
const SPI_REG_SHIFT: c_uint = 8;
const SPI_DACD0_REG: c_int = 10;
const SPI_DACD1_REG: c_int = 10;
const SPI_DACD2_REG: c_int = 10;
const SPI_DACD4_REG: c_int = 14;
const SPI_DACD0_BIT: c_int = 1 << 0;
const SPI_DACD1_BIT: c_int = 1 << 1;
const SPI_DACD2_BIT: c_int = 1 << 2;
const SPI_DACD4_BIT: c_int = 1 << 4;
const SPI_LDA1_REG: c_uint = 0;
const SPI_RDA1_REG: c_uint = 1;
const SPI_PL_REG: c_uint = 2;
const SPI_FMT_REG: c_uint = 3;
const SPI_LDA2_REG: c_uint = 4;
const SPI_RDA2_REG: c_uint = 5;
const SPI_LDA3_REG: c_uint = 6;
const SPI_RDA3_REG: c_uint = 7;
const SPI_MASTDA_REG: c_uint = 8;
const SPI_MS_REG: c_uint = 10;
const SPI_LDA4_REG: c_uint = 13;
const SPI_RDA4_REG: c_uint = 14;
const SPI_DA_BIT_0dB: c_uint = 0;
const SPI_PL_BIT_L_L: c_uint = 1 << 0;
const SPI_PL_BIT_R_R: c_uint = 1 << 1;
const SPI_IZD_BIT: c_uint = 1 << 2;
const SPI_FMT_BIT_I2S: c_uint = 1 << 3;
const SPI_IWL_BIT_24: c_uint = 1 << 4;
const SPI_DA_BIT_UPDATE: c_uint = 1 << 5;
const CA0106_MIDI_CHAN_A: c_uint = 0;
const CA0106_MIDI_CHAN_B: c_uint = 1;
const INTE_MIDI_TX_A: c_int = 1 << 0;
const INTE_MIDI_RX_A: c_int = 1 << 1;
const INTE_MIDI_TX_B: c_int = 1 << 2;
const INTE_MIDI_RX_B: c_int = 1 << 3;
const IPR_MIDI_TX_A: c_int = 1 << 4;
const IPR_MIDI_RX_A: c_int = 1 << 5;
const IPR_MIDI_TX_B: c_int = 1 << 6;
const IPR_MIDI_RX_B: c_int = 1 << 7;
const MIDI_UART_A_DATA: c_int = 0x100;
const MIDI_UART_B_DATA: c_int = 0x110;
const CA0106_MPU401_RESET: c_int = 0xff;
const CA0106_MPU401_ENTER_UART: c_int = 0x3f;
const CA0106_MPU401_ACK: c_int = 0xfe;
const CA0106_MIDI_INPUT_AVAIL: c_int = 0x80;
const CA0106_MIDI_OUTPUT_READY: c_int = 0x40;

const fn SPI_REG(reg: c_uint, value: c_uint) -> c_uint {
    (reg << SPI_REG_SHIFT) | value
}

static ca0106_chip_details: [snd_ca0106_details; 16] = [
    snd_ca0106_details { serial: 0x10131102, name: b"X-Fi Extreme Audio [SBxxxx]\0".as_ptr() as *const c_char, ac97: 0, gpio_type: 1, i2c_adc: 1, spi_dac: 0 },
    snd_ca0106_details { serial: 0x10121102, name: b"X-Fi Extreme Audio [SB0790]\0".as_ptr() as *const c_char, ac97: 0, gpio_type: 1, i2c_adc: 1, spi_dac: 0 },
    snd_ca0106_details { serial: 0x10021102, name: b"AudigyLS [SB0310]\0".as_ptr() as *const c_char, ac97: 1, gpio_type: 0, i2c_adc: 0, spi_dac: 0 },
    snd_ca0106_details { serial: 0x10051102, name: b"AudigyLS [SB0310b]\0".as_ptr() as *const c_char, ac97: 1, gpio_type: 0, i2c_adc: 0, spi_dac: 0 },
    snd_ca0106_details { serial: 0x10061102, name: b"Live! 7.1 24bit [SB0410]\0".as_ptr() as *const c_char, ac97: 0, gpio_type: 1, i2c_adc: 1, spi_dac: 0 },
    snd_ca0106_details { serial: 0x10071102, name: b"Live! 7.1 24bit [SB0413]\0".as_ptr() as *const c_char, ac97: 0, gpio_type: 1, i2c_adc: 1, spi_dac: 0 },
    snd_ca0106_details { serial: 0x100a1102, name: b"Audigy SE [SB0570]\0".as_ptr() as *const c_char, ac97: 0, gpio_type: 1, i2c_adc: 1, spi_dac: 0x4021 },
    snd_ca0106_details { serial: 0x10111102, name: b"Audigy SE OEM [SB0570a]\0".as_ptr() as *const c_char, ac97: 0, gpio_type: 1, i2c_adc: 1, spi_dac: 0x4021 },
    snd_ca0106_details { serial: 0x10041102, name: b"Sound Blaster 5.1vx [SB1070]\0".as_ptr() as *const c_char, ac97: 0, gpio_type: 1, i2c_adc: 0, spi_dac: 0x0124 },
    snd_ca0106_details { serial: 0x10091462, name: b"MSI K8N Diamond MB [SB0438]\0".as_ptr() as *const c_char, ac97: 0, gpio_type: 2, i2c_adc: 1, spi_dac: 0 },
    snd_ca0106_details { serial: 0x10091102, name: b"MSI K8N Diamond MB\0".as_ptr() as *const c_char, ac97: 0, gpio_type: 2, i2c_adc: 1, spi_dac: 0x4021 },
    snd_ca0106_details { serial: 0x1458a006, name: b"Giga-byte GA-G1975X\0".as_ptr() as *const c_char, ac97: 0, gpio_type: 1, i2c_adc: 1, spi_dac: 0 },
    snd_ca0106_details { serial: 0x30381297, name: b"Shuttle XPC SD31P [SD31P]\0".as_ptr() as *const c_char, ac97: 0, gpio_type: 1, i2c_adc: 1, spi_dac: 0 },
    snd_ca0106_details { serial: 0x30411297, name: b"Shuttle XPC SD11G5 [SD11G5]\0".as_ptr() as *const c_char, ac97: 0, gpio_type: 1, i2c_adc: 1, spi_dac: 0 },
    snd_ca0106_details { serial: 0, name: b"AudigyLS [Unknown]\0".as_ptr() as *const c_char, ac97: 0, gpio_type: 0, i2c_adc: 0, spi_dac: 0 },
    snd_ca0106_details { serial: 0, name: ptr::null(), ac97: 0, gpio_type: 0, i2c_adc: 0, spi_dac: 0 },
];

static snd_ca0106_playback_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_SYNC_START,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
    rates: SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000,
    rate_min: 48000,
    rate_max: 192000,
    channels_min: 2, //1,
    channels_max: 2, //6,
    buffer_bytes_max: ((65536 - 64) * 8) as c_ulong,
    period_bytes_min: 64,
    period_bytes_max: (65536 - 64) as c_ulong,
    periods_min: 2,
    periods_max: 8,
    fifo_size: 0,
};

static snd_ca0106_capture_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
    // #if 0 FIXME: 44.1kHz capture causes noisy output on 48kHz; disabled in the C source.
    rates: SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000,
    rate_min: 48000,
    rate_max: 192000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: (65536 - 128) as c_ulong,
    period_bytes_min: 64,
    period_bytes_max: (32768 - 64) as c_ulong,
    periods_min: 2,
    periods_max: 2,
    fifo_size: 0,
};

unsafe fn with_emu_lock<T>(emu: *mut snd_ca0106, f: impl FnOnce() -> T) -> T {
    let mut flags: c_ulong = 0;
    unsafe { spin_lock_irqsave(&mut (*emu).emu_lock, &mut flags) };
    let ret = f();
    unsafe { spin_unlock_irqrestore(&mut (*emu).emu_lock, flags) };
    ret
}

pub unsafe extern "C" fn snd_ca0106_ptr_read(emu: *mut snd_ca0106, reg: c_uint, chn: c_uint) -> c_uint {
    let regptr = (reg << 16) | chn;
    unsafe {
        with_emu_lock(emu, || {
            outl(regptr, (*emu).port + CA0106_PTR);
            inl((*emu).port + CA0106_DATA)
        })
    }
}

pub unsafe extern "C" fn snd_ca0106_ptr_write(emu: *mut snd_ca0106, reg: c_uint, chn: c_uint, data: c_uint) {
    let regptr = (reg << 16) | chn;
    unsafe {
        with_emu_lock(emu, || {
            outl(regptr, (*emu).port + CA0106_PTR);
            outl(data, (*emu).port + CA0106_DATA);
        });
    }
}

pub unsafe extern "C" fn snd_ca0106_spi_write(emu: *mut snd_ca0106, data: c_uint) -> c_int {
    let reg = SPI;
    if data > 0xffff {
        return 1;
    }
    let mut tmp = unsafe { snd_ca0106_ptr_read(emu, reg, 0) };
    let reset = (tmp & !0x3ffff) | 0x20000;
    let set = reset | 0x10000;
    unsafe { snd_ca0106_ptr_write(emu, reg, 0, reset | data) };
    tmp = unsafe { snd_ca0106_ptr_read(emu, reg, 0) };
    let _ = tmp;
    unsafe { snd_ca0106_ptr_write(emu, reg, 0, set | data) };
    let mut result = 1;
    for _n in 0..100 {
        unsafe { udelay(10) };
        tmp = unsafe { snd_ca0106_ptr_read(emu, reg, 0) };
        if (tmp & 0x10000) == 0 {
            result = 0;
            break;
        }
    }
    if result != 0 {
        return 1;
    }
    unsafe { snd_ca0106_ptr_write(emu, reg, 0, reset | data) };
    tmp = unsafe { snd_ca0106_ptr_read(emu, reg, 0) };
    let _ = tmp;
    0
}

pub unsafe extern "C" fn snd_ca0106_i2c_write(emu: *mut snd_ca0106, reg: u32, value: u32) -> c_int {
    let mut timeout = 0;
    let mut status: c_int = 0;
    if reg > 0x7f || value > 0x1ff {
        unsafe { dev_err((*(*emu).card).dev, b"i2c_write: invalid values.\n\0".as_ptr() as *const c_char) };
        return -EINVAL;
    }
    let mut tmp = (reg << 25) | (value << 16);
    unsafe { snd_ca0106_ptr_write(emu, I2C_D1, 0, tmp) };
    let mut retry = 0;
    while retry < 10 {
        tmp = 0;
        tmp |= I2C_A_ADC_LAST | I2C_A_ADC_START | I2C_A_ADC_ADD;
        unsafe { snd_ca0106_ptr_write(emu, I2C_A, 0, tmp) };
        loop {
            status = unsafe { snd_ca0106_ptr_read(emu, I2C_A, 0) as c_int };
            timeout += 1;
            if (status as c_uint & I2C_A_ADC_START) == 0 {
                break;
            }
            if timeout > 1000 {
                break;
            }
        }
        if (status as c_uint & I2C_A_ADC_ABORT) == 0 {
            break;
        }
        retry += 1;
    }
    if retry == 10 {
        unsafe { dev_err((*(*emu).card).dev, b"Writing to ADC failed!\n\0".as_ptr() as *const c_char) };
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn snd_ca0106_intr_enable(emu: *mut snd_ca0106, intrenb: c_uint) {
    unsafe {
        with_emu_lock(emu, || {
            let intr_enable = inl((*emu).port + CA0106_INTE) | intrenb;
            outl(intr_enable, (*emu).port + CA0106_INTE);
        });
    }
}

unsafe extern "C" fn snd_ca0106_intr_disable(emu: *mut snd_ca0106, intrenb: c_uint) {
    unsafe {
        with_emu_lock(emu, || {
            let intr_enable = inl((*emu).port + CA0106_INTE) & !intrenb;
            outl(intr_enable, (*emu).port + CA0106_INTE);
        });
    }
}

unsafe extern "C" fn snd_ca0106_pcm_free_substream(runtime: *mut snd_pcm_runtime) {
    unsafe { kfree((*runtime).private_data) };
}

static spi_dacd_reg: [c_int; 5] = [SPI_DACD0_REG, SPI_DACD1_REG, SPI_DACD2_REG, 0, SPI_DACD4_REG];
static spi_dacd_bit: [c_int; 5] = [SPI_DACD0_BIT, SPI_DACD1_BIT, SPI_DACD2_BIT, 0, SPI_DACD4_BIT];

unsafe fn restore_spdif_bits(chip: *mut snd_ca0106, idx: c_int) {
    let i = idx as usize;
    unsafe {
        if (*chip).spdif_str_bits[i] != (*chip).spdif_bits[i] {
            (*chip).spdif_str_bits[i] = (*chip).spdif_bits[i];
            snd_ca0106_ptr_write(chip, SPCS0 + idx as c_uint, 0, (*chip).spdif_str_bits[i]);
        }
    }
}

unsafe fn snd_ca0106_channel_dac(chip: *mut snd_ca0106, details: *const snd_ca0106_details, channel_id: c_int) -> c_int {
    unsafe {
        match channel_id {
            PCM_FRONT_CHANNEL => ((*details).spi_dac & 0xf000) >> (4 * 3),
            PCM_REAR_CHANNEL => ((*details).spi_dac & 0x0f00) >> (4 * 2),
            PCM_CENTER_LFE_CHANNEL => ((*details).spi_dac & 0x00f0) >> (4 * 1),
            PCM_UNKNOWN_CHANNEL => ((*details).spi_dac & 0x000f) >> (4 * 0),
            _ => {
                dev_dbg((*(*chip).card).dev, b"ca0106: unknown channel_id %d\n\0".as_ptr() as *const c_char, channel_id);
                0
            }
        }
    }
}

unsafe fn snd_ca0106_pcm_power_dac(chip: *mut snd_ca0106, channel_id: c_int, power: c_int) -> c_int {
    unsafe {
        if (*(*chip).details).spi_dac != 0 {
            let dac = snd_ca0106_channel_dac(chip, (*chip).details, channel_id) as usize;
            let reg = spi_dacd_reg[dac] as usize;
            let bit = spi_dacd_bit[dac] as c_uint;
            if power != 0 {
                (*chip).spi_dac_reg[reg] &= !bit;
            } else {
                (*chip).spi_dac_reg[reg] |= bit;
            }
            if snd_ca0106_spi_write(chip, (*chip).spi_dac_reg[reg]) != 0 {
                return -ENXIO;
            }
        }
    }
    0
}

unsafe fn alloc_epcm() -> *mut snd_ca0106_pcm {
    unsafe { kzalloc(size_of::<snd_ca0106_pcm>(), GFP_KERNEL) as *mut snd_ca0106_pcm }
}

unsafe fn snd_ca0106_pcm_open_playback_channel(substream: *mut snd_pcm_substream, channel_id: c_int) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let channel = &mut (*chip).playback_channels[channel_id as usize] as *mut snd_ca0106_channel;
        let runtime = (*substream).runtime;
        let epcm = alloc_epcm();
        if epcm.is_null() {
            return -ENOMEM;
        }
        (*epcm).emu = chip;
        (*epcm).substream = substream;
        (*epcm).channel_id = channel_id;
        (*runtime).private_data = epcm as *mut c_void;
        (*runtime).private_free = Some(snd_ca0106_pcm_free_substream);
        (*runtime).hw = snd_ca0106_playback_hw;
        (*channel).emu = chip;
        (*channel).number = channel_id;
        (*channel).use_ = 1;
        (*channel).epcm = epcm;
        let mut err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
        if err < 0 {
            return err;
        }
        err = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 64);
        if err < 0 {
            return err;
        }
        snd_pcm_set_sync(substream);
        if channel_id != PCM_FRONT_CHANNEL {
            err = snd_ca0106_pcm_power_dac(chip, channel_id, 1);
            if err < 0 {
                return err;
            }
        }
        restore_spdif_bits(chip, channel_id);
        0
    }
}

unsafe extern "C" fn snd_ca0106_pcm_close_playback(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let epcm = (*runtime).private_data as *mut snd_ca0106_pcm;
        (*chip).playback_channels[(*epcm).channel_id as usize].use_ = 0;
        restore_spdif_bits(chip, (*epcm).channel_id);
        if (*epcm).channel_id != PCM_FRONT_CHANNEL {
            let err = snd_ca0106_pcm_power_dac(chip, (*epcm).channel_id, 0);
            if err < 0 {
                return err;
            }
        }
        0
    }
}

unsafe extern "C" fn snd_ca0106_pcm_open_playback_front(substream: *mut snd_pcm_substream) -> c_int { unsafe { snd_ca0106_pcm_open_playback_channel(substream, PCM_FRONT_CHANNEL) } }
unsafe extern "C" fn snd_ca0106_pcm_open_playback_center_lfe(substream: *mut snd_pcm_substream) -> c_int { unsafe { snd_ca0106_pcm_open_playback_channel(substream, PCM_CENTER_LFE_CHANNEL) } }
unsafe extern "C" fn snd_ca0106_pcm_open_playback_unknown(substream: *mut snd_pcm_substream) -> c_int { unsafe { snd_ca0106_pcm_open_playback_channel(substream, PCM_UNKNOWN_CHANNEL) } }
unsafe extern "C" fn snd_ca0106_pcm_open_playback_rear(substream: *mut snd_pcm_substream) -> c_int { unsafe { snd_ca0106_pcm_open_playback_channel(substream, PCM_REAR_CHANNEL) } }

unsafe fn snd_ca0106_pcm_open_capture_channel(substream: *mut snd_pcm_substream, channel_id: c_int) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let channel = &mut (*chip).capture_channels[channel_id as usize] as *mut snd_ca0106_channel;
        let runtime = (*substream).runtime;
        let epcm = alloc_epcm();
        if epcm.is_null() {
            return -ENOMEM;
        }
        (*epcm).emu = chip;
        (*epcm).substream = substream;
        (*epcm).channel_id = channel_id;
        (*runtime).private_data = epcm as *mut c_void;
        (*runtime).private_free = Some(snd_ca0106_pcm_free_substream);
        (*runtime).hw = snd_ca0106_capture_hw;
        (*channel).emu = chip;
        (*channel).number = channel_id;
        (*channel).use_ = 1;
        (*channel).epcm = epcm;
        let mut err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
        if err < 0 {
            return err;
        }
        err = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 64);
        if err < 0 {
            return err;
        }
        0
    }
}

unsafe extern "C" fn snd_ca0106_pcm_close_capture(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let epcm = (*runtime).private_data as *mut snd_ca0106_pcm;
        (*chip).capture_channels[(*epcm).channel_id as usize].use_ = 0;
        0
    }
}

unsafe extern "C" fn snd_ca0106_pcm_open_0_capture(substream: *mut snd_pcm_substream) -> c_int { unsafe { snd_ca0106_pcm_open_capture_channel(substream, 0) } }
unsafe extern "C" fn snd_ca0106_pcm_open_1_capture(substream: *mut snd_pcm_substream) -> c_int { unsafe { snd_ca0106_pcm_open_capture_channel(substream, 1) } }
unsafe extern "C" fn snd_ca0106_pcm_open_2_capture(substream: *mut snd_pcm_substream) -> c_int { unsafe { snd_ca0106_pcm_open_capture_channel(substream, 2) } }
unsafe extern "C" fn snd_ca0106_pcm_open_3_capture(substream: *mut snd_pcm_substream) -> c_int { unsafe { snd_ca0106_pcm_open_capture_channel(substream, 3) } }

unsafe extern "C" fn snd_ca0106_pcm_prepare_playback(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let emu = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let epcm = (*runtime).private_data as *mut snd_ca0106_pcm;
        let channel = (*epcm).channel_id;
        let table_base = (*(*emu).buffer).area.add(8 * 16 * channel as usize) as *mut u32;
        let period_size_bytes = frames_to_bytes(runtime, (*runtime).period_size);
        let hcfg_mask = HCFG_PLAYBACK_S32_LE;
        let mut hcfg_set = 0;
        let reg40_mask = 0x30000u32 << (channel << 1);
        let mut reg40_set = 0;
        let reg71_mask = 0x03030000u32;
        let mut reg71_set = 0;
        match (*runtime).rate {
            44100 => { reg40_set = 0x10000u32 << (channel << 1); reg71_set = 0x01010000; }
            48000 => { reg40_set = 0; reg71_set = 0; }
            96000 => { reg40_set = 0x20000u32 << (channel << 1); reg71_set = 0x02020000; }
            192000 => { reg40_set = 0x30000u32 << (channel << 1); reg71_set = 0x03030000; }
            _ => { reg40_set = 0; reg71_set = 0; }
        }
        match (*runtime).format {
            SNDRV_PCM_FORMAT_S16_LE => hcfg_set = 0,
            SNDRV_PCM_FORMAT_S32_LE => hcfg_set = HCFG_PLAYBACK_S32_LE,
            _ => hcfg_set = 0,
        }
        let mut hcfg = inl((*emu).port + CA0106_HCFG);
        hcfg = (hcfg & !hcfg_mask) | hcfg_set;
        outl(hcfg, (*emu).port + CA0106_HCFG);
        let mut reg40 = snd_ca0106_ptr_read(emu, 0x40, 0);
        reg40 = (reg40 & !reg40_mask) | reg40_set;
        snd_ca0106_ptr_write(emu, 0x40, 0, reg40);
        let mut reg71 = snd_ca0106_ptr_read(emu, 0x71, 0);
        reg71 = (reg71 & !reg71_mask) | reg71_set;
        snd_ca0106_ptr_write(emu, 0x71, 0, reg71);
        for i in 0..(*runtime).periods as usize {
            *table_base.add(i * 2) = (*runtime).dma_addr + (i as u32 * period_size_bytes);
            *table_base.add(i * 2 + 1) = period_size_bytes << 16;
        }
        snd_ca0106_ptr_write(emu, PLAYBACK_LIST_ADDR, channel as c_uint, (*(*emu).buffer).addr + (8 * 16 * channel as u32));
        snd_ca0106_ptr_write(emu, PLAYBACK_LIST_SIZE, channel as c_uint, ((*runtime).periods - 1) << 19);
        snd_ca0106_ptr_write(emu, PLAYBACK_LIST_PTR, channel as c_uint, 0);
        snd_ca0106_ptr_write(emu, PLAYBACK_DMA_ADDR, channel as c_uint, (*runtime).dma_addr);
        snd_ca0106_ptr_write(emu, PLAYBACK_PERIOD_SIZE, channel as c_uint, frames_to_bytes(runtime, (*runtime).period_size) << 16);
        snd_ca0106_ptr_write(emu, PLAYBACK_PERIOD_SIZE, channel as c_uint, 0);
        snd_ca0106_ptr_write(emu, PLAYBACK_POINTER, channel as c_uint, 0);
        snd_ca0106_ptr_write(emu, 0x07, channel as c_uint, 0);
        snd_ca0106_ptr_write(emu, 0x08, channel as c_uint, 0);
        snd_ca0106_ptr_write(emu, PLAYBACK_MUTE, 0, 0);
        0
    }
}

unsafe extern "C" fn snd_ca0106_pcm_prepare_capture(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let emu = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let epcm = (*runtime).private_data as *mut snd_ca0106_pcm;
        let channel = (*epcm).channel_id;
        let hcfg_mask = HCFG_CAPTURE_S32_LE;
        let mut hcfg_set = 0;
        let mut over_sampling = 0x2;
        let reg71_mask = 0x0000c000u32;
        let mut reg71_set = 0;
        match (*runtime).rate {
            44100 => reg71_set = 0x00004000,
            48000 => reg71_set = 0,
            96000 => { reg71_set = 0x00008000; over_sampling = 0xa; }
            192000 => { reg71_set = 0x0000c000; over_sampling = 0xa; }
            _ => reg71_set = 0,
        }
        match (*runtime).format {
            SNDRV_PCM_FORMAT_S16_LE => hcfg_set = 0,
            SNDRV_PCM_FORMAT_S32_LE => hcfg_set = HCFG_CAPTURE_S32_LE,
            _ => hcfg_set = 0,
        }
        let mut hcfg = inl((*emu).port + CA0106_HCFG);
        hcfg = (hcfg & !hcfg_mask) | hcfg_set;
        outl(hcfg, (*emu).port + CA0106_HCFG);
        let mut reg71 = snd_ca0106_ptr_read(emu, 0x71, 0);
        reg71 = (reg71 & !reg71_mask) | reg71_set;
        snd_ca0106_ptr_write(emu, 0x71, 0, reg71);
        if (*(*emu).details).i2c_adc == 1 {
            snd_ca0106_i2c_write(emu, ADC_MASTER, over_sampling);
        }
        snd_ca0106_ptr_write(emu, 0x13, channel as c_uint, 0);
        snd_ca0106_ptr_write(emu, CAPTURE_DMA_ADDR, channel as c_uint, (*runtime).dma_addr);
        snd_ca0106_ptr_write(emu, CAPTURE_BUFFER_SIZE, channel as c_uint, frames_to_bytes(runtime, (*runtime).buffer_size) << 16);
        snd_ca0106_ptr_write(emu, CAPTURE_POINTER, channel as c_uint, 0);
        0
    }
}

unsafe extern "C" fn snd_ca0106_pcm_trigger_playback(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    unsafe {
        let emu = snd_pcm_substream_chip(substream);
        let mut result = 0;
        let mut basic = 0u32;
        let mut extended = 0u32;
        let running = match cmd {
            SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => 1,
            _ => 0,
        };
        let mut s = substream;
        while !s.is_null() {
            if snd_pcm_substream_chip(s) == emu && (*s).stream == SNDRV_PCM_STREAM_PLAYBACK {
                let runtime = (*s).runtime;
                let epcm = (*runtime).private_data as *mut snd_ca0106_pcm;
                let channel = (*epcm).channel_id;
                (*epcm).running = running;
                basic |= 0x1 << channel;
                extended |= 0x10 << channel;
                snd_pcm_trigger_done(s, substream);
            }
            s = (*s).next;
        }
        match cmd {
            SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
                let mut bits = snd_ca0106_ptr_read(emu, EXTENDED_INT_MASK, 0);
                bits |= extended;
                snd_ca0106_ptr_write(emu, EXTENDED_INT_MASK, 0, bits);
                bits = snd_ca0106_ptr_read(emu, BASIC_INTERRUPT, 0);
                bits |= basic;
                snd_ca0106_ptr_write(emu, BASIC_INTERRUPT, 0, bits);
            }
            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
                let mut bits = snd_ca0106_ptr_read(emu, BASIC_INTERRUPT, 0);
                bits &= !basic;
                snd_ca0106_ptr_write(emu, BASIC_INTERRUPT, 0, bits);
                bits = snd_ca0106_ptr_read(emu, EXTENDED_INT_MASK, 0);
                bits &= !extended;
                snd_ca0106_ptr_write(emu, EXTENDED_INT_MASK, 0, bits);
            }
            _ => result = -EINVAL,
        }
        result
    }
}

unsafe extern "C" fn snd_ca0106_pcm_trigger_capture(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    unsafe {
        let emu = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let epcm = (*runtime).private_data as *mut snd_ca0106_pcm;
        let channel = (*epcm).channel_id;
        let mut result = 0;
        match cmd {
            SNDRV_PCM_TRIGGER_START => {
                snd_ca0106_ptr_write(emu, EXTENDED_INT_MASK, 0, snd_ca0106_ptr_read(emu, EXTENDED_INT_MASK, 0) | (0x110000 << channel));
                snd_ca0106_ptr_write(emu, BASIC_INTERRUPT, 0, snd_ca0106_ptr_read(emu, BASIC_INTERRUPT, 0) | (0x100 << channel));
                (*epcm).running = 1;
            }
            SNDRV_PCM_TRIGGER_STOP => {
                snd_ca0106_ptr_write(emu, BASIC_INTERRUPT, 0, snd_ca0106_ptr_read(emu, BASIC_INTERRUPT, 0) & !(0x100 << channel));
                snd_ca0106_ptr_write(emu, EXTENDED_INT_MASK, 0, snd_ca0106_ptr_read(emu, EXTENDED_INT_MASK, 0) & !(0x110000 << channel));
                (*epcm).running = 0;
            }
            _ => result = -EINVAL,
        }
        result
    }
}

unsafe extern "C" fn snd_ca0106_pcm_pointer_playback(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    unsafe {
        let emu = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let epcm = (*runtime).private_data as *mut snd_ca0106_pcm;
        let channel = (*epcm).channel_id;
        let mut timeout = 10;
        if (*epcm).running == 0 {
            return 0;
        }
        let mut prev_ptr: c_uint = !0;
        loop {
            let mut ptr_ = snd_ca0106_ptr_read(emu, PLAYBACK_LIST_PTR, channel as c_uint);
            ptr_ = (ptr_ >> 3).wrapping_mul((*runtime).period_size as c_uint);
            ptr_ = ptr_.wrapping_add(bytes_to_frames(runtime, snd_ca0106_ptr_read(emu, PLAYBACK_POINTER, channel as c_uint)) as c_uint);
            if ptr_ as snd_pcm_uframes_t >= (*runtime).buffer_size {
                ptr_ = ptr_.wrapping_sub((*runtime).buffer_size as c_uint);
            }
            if prev_ptr == ptr_ {
                return ptr_ as snd_pcm_uframes_t;
            }
            prev_ptr = ptr_;
            timeout -= 1;
            if timeout == 0 {
                break;
            }
        }
        dev_warn((*(*emu).card).dev, b"ca0106: unstable DMA pointer!\n\0".as_ptr() as *const c_char);
        0
    }
}

unsafe extern "C" fn snd_ca0106_pcm_pointer_capture(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    unsafe {
        let emu = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let epcm = (*runtime).private_data as *mut snd_ca0106_pcm;
        let channel = (*epcm).channel_id;
        if (*epcm).running == 0 {
            return 0;
        }
        let ptr1 = snd_ca0106_ptr_read(emu, CAPTURE_POINTER, channel as c_uint);
        let ptr2 = bytes_to_frames(runtime, ptr1);
        let mut ptr_ = ptr2;
        if ptr_ >= (*runtime).buffer_size {
            ptr_ -= (*runtime).buffer_size;
        }
        ptr_
    }
}

static snd_ca0106_playback_front_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ca0106_pcm_open_playback_front), close: Some(snd_ca0106_pcm_close_playback), prepare: Some(snd_ca0106_pcm_prepare_playback), trigger: Some(snd_ca0106_pcm_trigger_playback), pointer: Some(snd_ca0106_pcm_pointer_playback) };
static snd_ca0106_capture_0_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ca0106_pcm_open_0_capture), close: Some(snd_ca0106_pcm_close_capture), prepare: Some(snd_ca0106_pcm_prepare_capture), trigger: Some(snd_ca0106_pcm_trigger_capture), pointer: Some(snd_ca0106_pcm_pointer_capture) };
static snd_ca0106_capture_1_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ca0106_pcm_open_1_capture), close: Some(snd_ca0106_pcm_close_capture), prepare: Some(snd_ca0106_pcm_prepare_capture), trigger: Some(snd_ca0106_pcm_trigger_capture), pointer: Some(snd_ca0106_pcm_pointer_capture) };
static snd_ca0106_capture_2_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ca0106_pcm_open_2_capture), close: Some(snd_ca0106_pcm_close_capture), prepare: Some(snd_ca0106_pcm_prepare_capture), trigger: Some(snd_ca0106_pcm_trigger_capture), pointer: Some(snd_ca0106_pcm_pointer_capture) };
static snd_ca0106_capture_3_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ca0106_pcm_open_3_capture), close: Some(snd_ca0106_pcm_close_capture), prepare: Some(snd_ca0106_pcm_prepare_capture), trigger: Some(snd_ca0106_pcm_trigger_capture), pointer: Some(snd_ca0106_pcm_pointer_capture) };
static snd_ca0106_playback_center_lfe_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ca0106_pcm_open_playback_center_lfe), close: Some(snd_ca0106_pcm_close_playback), prepare: Some(snd_ca0106_pcm_prepare_playback), trigger: Some(snd_ca0106_pcm_trigger_playback), pointer: Some(snd_ca0106_pcm_pointer_playback) };
static snd_ca0106_playback_unknown_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ca0106_pcm_open_playback_unknown), close: Some(snd_ca0106_pcm_close_playback), prepare: Some(snd_ca0106_pcm_prepare_playback), trigger: Some(snd_ca0106_pcm_trigger_playback), pointer: Some(snd_ca0106_pcm_pointer_playback) };
static snd_ca0106_playback_rear_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ca0106_pcm_open_playback_rear), close: Some(snd_ca0106_pcm_close_playback), prepare: Some(snd_ca0106_pcm_prepare_playback), trigger: Some(snd_ca0106_pcm_trigger_playback), pointer: Some(snd_ca0106_pcm_pointer_playback) };

unsafe extern "C" fn snd_ca0106_ac97_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    unsafe {
        let emu = (*ac97).private_data as *mut snd_ca0106;
        with_emu_lock(emu, || {
            outb(reg as c_uint, (*emu).port + CA0106_AC97ADDRESS);
            inw((*emu).port + CA0106_AC97DATA)
        })
    }
}

unsafe extern "C" fn snd_ca0106_ac97_write(ac97: *mut snd_ac97, reg: u16, val: u16) {
    unsafe {
        let emu = (*ac97).private_data as *mut snd_ca0106;
        with_emu_lock(emu, || {
            outb(reg as c_uint, (*emu).port + CA0106_AC97ADDRESS);
            outw(val as c_uint, (*emu).port + CA0106_AC97DATA);
        });
    }
}

unsafe fn snd_ca0106_ac97(chip: *mut snd_ca0106) -> c_int {
    unsafe {
        let mut pbus: *mut snd_ac97_bus = ptr::null_mut();
        let mut ac97 = snd_ac97_template { private_data: ptr::null_mut(), scaps: 0 };
        static ops: snd_ac97_bus_ops = snd_ac97_bus_ops { write: Some(snd_ca0106_ac97_write), read: Some(snd_ca0106_ac97_read) };
        let err = snd_ac97_bus((*chip).card, 0, &ops, ptr::null_mut(), &mut pbus);
        if err < 0 {
            return err;
        }
        (*pbus).no_vra = 1;
        memset(&mut ac97 as *mut _ as *mut c_void, 0, size_of::<snd_ac97_template>());
        ac97.private_data = chip as *mut c_void;
        ac97.scaps = AC97_SCAP_NO_SPDIF;
        snd_ac97_mixer(pbus, &mut ac97, &mut (*chip).ac97)
    }
}

unsafe extern "C" fn snd_ca0106_free(card: *mut snd_card) {
    unsafe {
        let chip = (*card).private_data;
        ca0106_stop_chip(chip);
    }
}

unsafe extern "C" fn snd_ca0106_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    unsafe {
        let chip = dev_id as *mut snd_ca0106;
        let status = inl((*chip).port + CA0106_IPR);
        if status == 0 {
            return IRQ_NONE;
        }
        let stat76 = snd_ca0106_ptr_read(chip, EXTENDED_INT, 0);
        let mut mask = 0x11u32;
        for i in 0..4 {
            let pchannel = &mut (*chip).playback_channels[i] as *mut snd_ca0106_channel;
            if (stat76 & mask) != 0 && (*pchannel).use_ != 0 {
                snd_pcm_period_elapsed((*(*pchannel).epcm).substream);
            }
            mask <<= 1;
        }
        mask = 0x110000;
        for i in 0..4 {
            let pchannel = &mut (*chip).capture_channels[i] as *mut snd_ca0106_channel;
            if (stat76 & mask) != 0 && (*pchannel).use_ != 0 {
                snd_pcm_period_elapsed((*(*pchannel).epcm).substream);
            }
            mask <<= 1;
        }
        snd_ca0106_ptr_write(chip, EXTENDED_INT, 0, stat76);
        if !(*chip).midi.dev_id.is_null() && (status & ((*chip).midi.ipr_tx | (*chip).midi.ipr_rx) as c_uint) != 0 {
            if let Some(interrupt) = (*chip).midi.interrupt {
                interrupt(&mut (*chip).midi, status);
            } else if let Some(disable) = (*chip).midi.interrupt_disable {
                disable(&mut (*chip).midi, (*chip).midi.tx_enable | (*chip).midi.rx_enable);
            }
        }
        outl(status, (*chip).port + CA0106_IPR);
        IRQ_HANDLED
    }
}

static surround_map: [snd_pcm_chmap_elem; 2] = [
    snd_pcm_chmap_elem { channels: 2, map: [SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, 0, 0, 0, 0, 0, 0] },
    snd_pcm_chmap_elem { channels: 0, map: [0; 8] },
];
static clfe_map: [snd_pcm_chmap_elem; 2] = [
    snd_pcm_chmap_elem { channels: 2, map: [SNDRV_CHMAP_FC, SNDRV_CHMAP_LFE, 0, 0, 0, 0, 0, 0] },
    snd_pcm_chmap_elem { channels: 0, map: [0; 8] },
];
static side_map: [snd_pcm_chmap_elem; 2] = [
    snd_pcm_chmap_elem { channels: 2, map: [SNDRV_CHMAP_SL, SNDRV_CHMAP_SR, 0, 0, 0, 0, 0, 0] },
    snd_pcm_chmap_elem { channels: 0, map: [0; 8] },
];

unsafe fn snd_ca0106_pcm(emu: *mut snd_ca0106, device: c_int) -> c_int {
    unsafe {
        let mut pcm: *mut snd_pcm = ptr::null_mut();
        let mut map: *const snd_pcm_chmap_elem = ptr::null();
        let mut err = snd_pcm_new((*emu).card, b"ca0106\0".as_ptr() as *const c_char, device, 1, 1, &mut pcm);
        if err < 0 {
            return err;
        }
        (*pcm).private_data = emu as *mut c_void;
        match device {
            0 => {
                snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ca0106_playback_front_ops);
                snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_ca0106_capture_0_ops);
                map = snd_pcm_std_chmaps;
            }
            1 => {
                snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ca0106_playback_rear_ops);
                snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_ca0106_capture_1_ops);
                map = surround_map.as_ptr();
            }
            2 => {
                snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ca0106_playback_center_lfe_ops);
                snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_ca0106_capture_2_ops);
                map = clfe_map.as_ptr();
            }
            3 => {
                snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ca0106_playback_unknown_ops);
                snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_ca0106_capture_3_ops);
                map = side_map.as_ptr();
            }
            _ => {}
        }
        (*pcm).info_flags = 0;
        strscpy((*pcm).name.as_mut_ptr(), b"CA0106\0".as_ptr() as *const c_char);
        let mut substream = (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream;
        while !substream.is_null() {
            snd_pcm_set_managed_buffer(substream, SNDRV_DMA_TYPE_DEV, &mut (*(*emu).pci).dev, 64 * 1024, 64 * 1024);
            substream = (*substream).next;
        }
        substream = (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream;
        while !substream.is_null() {
            snd_pcm_set_managed_buffer(substream, SNDRV_DMA_TYPE_DEV, &mut (*(*emu).pci).dev, 64 * 1024, 64 * 1024);
            substream = (*substream).next;
        }
        err = snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK, map, 2, 1 << 2, ptr::null_mut());
        if err < 0 {
            return err;
        }
        (*emu).pcm[device as usize] = pcm;
        0
    }
}

static spi_dac_init: [c_uint; 15] = [
    SPI_REG(SPI_LDA1_REG, SPI_DA_BIT_0dB), /* 0dB dig. attenuation */
    SPI_REG(SPI_RDA1_REG, SPI_DA_BIT_0dB),
    SPI_REG(SPI_PL_REG, SPI_PL_BIT_L_L | SPI_PL_BIT_R_R | SPI_IZD_BIT),
    SPI_REG(SPI_FMT_REG, SPI_FMT_BIT_I2S | SPI_IWL_BIT_24),
    SPI_REG(SPI_LDA2_REG, SPI_DA_BIT_0dB),
    SPI_REG(SPI_RDA2_REG, SPI_DA_BIT_0dB),
    SPI_REG(SPI_LDA3_REG, SPI_DA_BIT_0dB),
    SPI_REG(SPI_RDA3_REG, SPI_DA_BIT_0dB),
    SPI_REG(SPI_MASTDA_REG, SPI_DA_BIT_0dB),
    SPI_REG(9, 0x00),
    SPI_REG(SPI_MS_REG, SPI_DACD0_BIT as c_uint | SPI_DACD1_BIT as c_uint | SPI_DACD2_BIT as c_uint),
    SPI_REG(12, 0x00),
    SPI_REG(SPI_LDA4_REG, SPI_DA_BIT_0dB),
    SPI_REG(SPI_RDA4_REG, SPI_DA_BIT_0dB | SPI_DA_BIT_UPDATE),
    SPI_REG(SPI_DACD4_REG as c_uint, SPI_DACD4_BIT as c_uint),
];

static i2c_adc_init: [[c_uint; 2]; 13] = [
    [0x17, 0x00], /* Reset */
    [0x07, 0x00], /* Timeout */
    [0x0b, 0x22], /* Interface control */
    [0x0c, 0x22], /* Master mode control */
    [0x0d, 0x08], /* Powerdown control */
    [0x0e, 0xcf], /* Attenuation Left  0x01 = -103dB, 0xff = 24dB */
    [0x0f, 0xcf], /* Attenuation Right 0.5dB steps */
    [0x10, 0x7b], /* ALC Control 1 */
    [0x11, 0x00], /* ALC Control 2 */
    [0x12, 0x32], /* ALC Control 3 */
    [0x13, 0x00], /* Noise gate control */
    [0x14, 0xa6], /* Limiter control */
    [0x15, ADC_MUX_LINEIN], /* ADC Mixer control */
];

unsafe fn ca0106_init_chip(chip: *mut snd_ca0106, resume: c_int) {
    unsafe {
        outl(0, (*chip).port + CA0106_INTE);
        let def_bits = SPCS_CLKACCY_1000PPM | SPCS_SAMPLERATE_48 | SPCS_CHANNELNUM_LEFT | SPCS_SOURCENUM_UNSPEC | SPCS_GENERATIONSTATUS | 0x00001200 | 0x00000000 | SPCS_EMPHASIS_NONE | SPCS_COPYRIGHT;
        if resume == 0 {
            for i in 0..4 {
                (*chip).spdif_bits[i] = def_bits;
                (*chip).spdif_str_bits[i] = def_bits;
            }
        }
        snd_ca0106_ptr_write(chip, SPCS1, 0, (*chip).spdif_str_bits[1]);
        snd_ca0106_ptr_write(chip, SPCS0, 0, (*chip).spdif_str_bits[0]);
        snd_ca0106_ptr_write(chip, SPCS2, 0, (*chip).spdif_str_bits[2]);
        snd_ca0106_ptr_write(chip, SPCS3, 0, (*chip).spdif_str_bits[3]);
        snd_ca0106_ptr_write(chip, PLAYBACK_MUTE, 0, 0x00fc0000);
        snd_ca0106_ptr_write(chip, CAPTURE_MUTE, 0, 0x00fc0000);
        outb(AC97_REC_GAIN, (*chip).port + CA0106_AC97ADDRESS);
        outw(0x8000, (*chip).port + CA0106_AC97DATA);
        snd_ca0106_ptr_write(chip, SPDIF_SELECT1, 0, 0xf);
        snd_ca0106_ptr_write(chip, SPDIF_SELECT2, 0, 0x000f0000);
        (*chip).spdif_enable = 0;
        snd_ca0106_ptr_write(chip, CAPTURE_CONTROL, 0, 0x40c81000);
        snd_ca0106_ptr_write(chip, CAPTURE_CONTROL, 1, 0xffffffff);
        snd_ca0106_ptr_write(chip, CAPTURE_CONTROL, 2, 0x30300000);
        snd_ca0106_ptr_write(chip, CAPTURE_CONTROL, 3, 0x00700000);
        snd_ca0106_ptr_write(chip, PLAYBACK_ROUTING1, 0, 0x32765410);
        snd_ca0106_ptr_write(chip, PLAYBACK_ROUTING2, 0, 0x76767676);
        snd_ca0106_ptr_write(chip, CAPTURE_ROUTING1, 0, 0x32765410);
        snd_ca0106_ptr_write(chip, CAPTURE_ROUTING2, 0, 0x76767676);
        for ch in 0..4 {
            snd_ca0106_ptr_write(chip, CAPTURE_VOLUME1, ch, 0x30303030);
            snd_ca0106_ptr_write(chip, CAPTURE_VOLUME2, ch, 0x30303030);
        }
        if (*(*chip).details).i2c_adc == 1 {
            snd_ca0106_ptr_write(chip, CAPTURE_SOURCE, 0, 0x333300e4);
            if resume == 0 {
                (*chip).capture_source = 3;
            }
        } else if (*(*chip).details).ac97 == 1 {
            snd_ca0106_ptr_write(chip, CAPTURE_SOURCE, 0, 0x444400e4);
            if resume == 0 {
                (*chip).capture_source = 4;
            }
        } else {
            snd_ca0106_ptr_write(chip, CAPTURE_SOURCE, 0, 0x333300e4);
            if resume == 0 {
                (*chip).capture_source = 3;
            }
        }
        if (*(*chip).details).gpio_type == 2 {
            outl(0, (*chip).port + CA0106_GPIO);
            outl(0x005f5301, (*chip).port + CA0106_GPIO);
        } else if (*(*chip).details).gpio_type == 1 {
            outl(0, (*chip).port + CA0106_GPIO);
            outl(0x005f5301, (*chip).port + CA0106_GPIO);
        } else {
            outl(0, (*chip).port + CA0106_GPIO);
            outl(0x005f03a3, (*chip).port + CA0106_GPIO);
        }
        snd_ca0106_intr_enable(chip, 0x105);
        outl(HCFG_AC97 | HCFG_AUDIOENABLE, (*chip).port + CA0106_HCFG);
        if (*(*chip).details).i2c_adc == 1 {
            for n in 0..i2c_adc_init.len() {
                snd_ca0106_i2c_write(chip, i2c_adc_init[n][0], i2c_adc_init[n][1]);
            }
            for n in 0..4 {
                (*chip).i2c_capture_volume[n][0] = 0xcf;
                (*chip).i2c_capture_volume[n][1] = 0xcf;
            }
            (*chip).i2c_capture_source = 2;
        }
        if (*(*chip).details).spi_dac != 0 {
            for n in 0..spi_dac_init.len() {
                let reg = spi_dac_init[n] >> SPI_REG_SHIFT;
                snd_ca0106_spi_write(chip, spi_dac_init[n]);
                if (reg as usize) < (*chip).spi_dac_reg.len() {
                    (*chip).spi_dac_reg[reg as usize] = spi_dac_init[n];
                }
            }
            snd_ca0106_pcm_power_dac(chip, PCM_FRONT_CHANNEL, 1);
        }
    }
}

unsafe fn ca0106_stop_chip(chip: *mut snd_ca0106) {
    unsafe {
        snd_ca0106_ptr_write(chip, BASIC_INTERRUPT, 0, 0);
        outl(0, (*chip).port + CA0106_INTE);
        snd_ca0106_ptr_write(chip, EXTENDED_INT_MASK, 0, 0);
        udelay(1000);
        outl(0, (*chip).port + CA0106_HCFG);
    }
}

unsafe fn snd_ca0106_create(dev: c_int, card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    unsafe {
        let chip = (*card).private_data;
        let mut err = pcim_enable_device(pci);
        if err < 0 {
            return err;
        }
        if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK_32) != 0 {
            dev_err((*card).dev, b"error to set 32bit mask DMA\n\0".as_ptr() as *const c_char);
            return -ENXIO;
        }
        (*chip).card = card;
        (*chip).pci = pci;
        (*chip).irq = -1;
        spin_lock_init(&mut (*chip).emu_lock);
        err = pcim_request_all_regions(pci, b"snd_ca0106\0".as_ptr() as *const c_char);
        if err < 0 {
            return err;
        }
        (*chip).port = pci_resource_start(pci, 0);
        if devm_request_irq(&mut (*pci).dev, (*pci).irq, snd_ca0106_interrupt, IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void) != 0 {
            dev_err((*card).dev, b"cannot grab irq\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
        (*chip).irq = (*pci).irq;
        (*card).sync_irq = (*chip).irq;
        (*chip).buffer = snd_devm_alloc_pages(&mut (*pci).dev, SNDRV_DMA_TYPE_DEV, 1024);
        if (*chip).buffer.is_null() {
            return -ENOMEM;
        }
        pci_set_master(pci);
        pci_read_config_dword(pci, PCI_SUBSYSTEM_VENDOR_ID, &mut (*chip).serial);
        pci_read_config_word(pci, PCI_SUBSYSTEM_ID, &mut (*chip).model);
        dev_info((*card).dev, b"Model %04x Rev %08x Serial %08x\n\0".as_ptr() as *const c_char, (*chip).model as c_uint, (*pci).revision, (*chip).serial);
        strscpy((*card).driver.as_mut_ptr(), b"CA0106\0".as_ptr() as *const c_char);
        strscpy((*card).shortname.as_mut_ptr(), b"CA0106\0".as_ptr() as *const c_char);
        let mut c = ca0106_chip_details.as_ptr();
        while (*c).serial != 0 {
            if subsystem[dev as usize] != 0 {
                if (*c).serial == subsystem[dev as usize] {
                    break;
                }
            } else if (*c).serial == (*chip).serial {
                break;
            }
            c = c.add(1);
        }
        (*chip).details = c;
        if subsystem[dev as usize] != 0 {
            dev_info((*card).dev, b"Sound card name=%s, subsystem=0x%x. Forced to subsystem=0x%x\n\0".as_ptr() as *const c_char, (*c).name, (*chip).serial, subsystem[dev as usize]);
        }
        sprintf((*card).longname.as_mut_ptr(), b"%s at 0x%lx irq %i\0".as_ptr() as *const c_char, (*c).name, (*chip).port, (*chip).irq);
        ca0106_init_chip(chip, 0);
        0
    }
}

unsafe extern "C" fn ca0106_midi_interrupt_enable(midi: *mut snd_ca_midi, intr: c_int) {
    unsafe { snd_ca0106_intr_enable((*midi).dev_id as *mut snd_ca0106, intr as c_uint) };
}

unsafe extern "C" fn ca0106_midi_interrupt_disable(midi: *mut snd_ca_midi, intr: c_int) {
    unsafe { snd_ca0106_intr_disable((*midi).dev_id as *mut snd_ca0106, intr as c_uint) };
}

unsafe extern "C" fn ca0106_midi_read(midi: *mut snd_ca_midi, idx: c_int) -> u8 {
    unsafe { snd_ca0106_ptr_read((*midi).dev_id as *mut snd_ca0106, ((*midi).port + idx) as c_uint, 0) as u8 }
}

unsafe extern "C" fn ca0106_midi_write(midi: *mut snd_ca_midi, data: c_int, idx: c_int) {
    unsafe { snd_ca0106_ptr_write((*midi).dev_id as *mut snd_ca0106, ((*midi).port + idx) as c_uint, 0, data as c_uint) };
}

unsafe extern "C" fn ca0106_dev_id_card(dev_id: *mut c_void) -> *mut snd_card {
    unsafe { (*(dev_id as *mut snd_ca0106)).card }
}

unsafe extern "C" fn ca0106_dev_id_port(dev_id: *mut c_void) -> c_int {
    unsafe { (*(dev_id as *mut snd_ca0106)).port as c_int }
}

unsafe fn snd_ca0106_midi(chip: *mut snd_ca0106, channel: c_uint) -> c_int {
    unsafe {
        let (name, midi): (*const c_char, *mut snd_ca_midi) = if channel == CA0106_MIDI_CHAN_B {
            let midi = &mut (*chip).midi2 as *mut snd_ca_midi;
            (*midi).tx_enable = INTE_MIDI_TX_B;
            (*midi).rx_enable = INTE_MIDI_RX_B;
            (*midi).ipr_tx = IPR_MIDI_TX_B;
            (*midi).ipr_rx = IPR_MIDI_RX_B;
            (*midi).port = MIDI_UART_B_DATA;
            (b"CA0106 MPU-401 (UART) B\0".as_ptr() as *const c_char, midi)
        } else {
            let midi = &mut (*chip).midi as *mut snd_ca_midi;
            (*midi).tx_enable = INTE_MIDI_TX_A;
            (*midi).rx_enable = INTE_MIDI_TX_B;
            (*midi).ipr_tx = IPR_MIDI_TX_A;
            (*midi).ipr_rx = IPR_MIDI_RX_A;
            (*midi).port = MIDI_UART_A_DATA;
            (b"CA0106 MPU-401 (UART)\0".as_ptr() as *const c_char, midi)
        };
        (*midi).reset = CA0106_MPU401_RESET;
        (*midi).enter_uart = CA0106_MPU401_ENTER_UART;
        (*midi).ack = CA0106_MPU401_ACK;
        (*midi).input_avail = CA0106_MIDI_INPUT_AVAIL;
        (*midi).output_ready = CA0106_MIDI_OUTPUT_READY;
        (*midi).channel = channel;
        (*midi).interrupt_enable = Some(ca0106_midi_interrupt_enable);
        (*midi).interrupt_disable = Some(ca0106_midi_interrupt_disable);
        (*midi).read = Some(ca0106_midi_read);
        (*midi).write = Some(ca0106_midi_write);
        (*midi).get_dev_id_card = Some(ca0106_dev_id_card);
        (*midi).get_dev_id_port = Some(ca0106_dev_id_port);
        (*midi).dev_id = chip as *mut c_void;
        let err = ca_midi_init(chip, midi, 0, name);
        if err < 0 {
            return err;
        }
        0
    }
}

unsafe fn __snd_ca0106_probe(pci: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    unsafe {
        let mut card: *mut snd_card = ptr::null_mut();
        if dev >= SNDRV_CARDS as c_int {
            return -ENODEV;
        }
        if !enable[dev as usize] {
            dev += 1;
            return -ENOENT;
        }
        let mut err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], THIS_MODULE, size_of::<snd_ca0106>(), &mut card);
        if err < 0 {
            return err;
        }
        let chip = (*card).private_data;
        err = snd_ca0106_create(dev, card, pci);
        if err < 0 {
            return err;
        }
        (*card).private_free = Some(snd_ca0106_free);
        for i in 0..4 {
            err = snd_ca0106_pcm(chip, i);
            if err < 0 {
                return err;
            }
        }
        if (*(*chip).details).ac97 == 1 {
            err = snd_ca0106_ac97(chip);
            if err < 0 {
                return err;
            }
        }
        err = snd_ca0106_mixer(chip);
        if err < 0 {
            return err;
        }
        dev_dbg((*card).dev, b"probe for MIDI channel A ...\0".as_ptr() as *const c_char);
        err = snd_ca0106_midi(chip, CA0106_MIDI_CHAN_A);
        if err < 0 {
            return err;
        }
        dev_dbg((*card).dev, b" done.\n\0".as_ptr() as *const c_char);
        // #ifdef CONFIG_SND_PROC_FS
        snd_ca0106_proc_init(chip);
        // #endif
        err = snd_card_register(card);
        if err < 0 {
            return err;
        }
        pci_set_drvdata(pci, card as *mut c_void);
        dev += 1;
        0
    }
}

unsafe extern "C" fn snd_ca0106_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    unsafe { snd_card_free_on_error(&mut (*pci).dev, __snd_ca0106_probe(pci, pci_id)) }
}

// #ifdef CONFIG_PM_SLEEP
unsafe extern "C" fn snd_ca0106_suspend(dev: *mut device) -> c_int {
    unsafe {
        let card = dev_get_drvdata(dev) as *mut snd_card;
        let chip = (*card).private_data;
        snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
        if (*(*chip).details).ac97 != 0 {
            snd_ac97_suspend((*chip).ac97);
        }
        snd_ca0106_mixer_suspend(chip);
        ca0106_stop_chip(chip);
        0
    }
}

unsafe extern "C" fn snd_ca0106_resume(dev: *mut device) -> c_int {
    unsafe {
        let card = dev_get_drvdata(dev) as *mut snd_card;
        let chip = (*card).private_data;
        ca0106_init_chip(chip, 1);
        if (*(*chip).details).ac97 != 0 {
            snd_ac97_resume((*chip).ac97);
        }
        snd_ca0106_mixer_resume(chip);
        if (*(*chip).details).spi_dac != 0 {
            for i in 0..(*chip).spi_dac_reg.len() {
                snd_ca0106_spi_write(chip, (*chip).spi_dac_reg[i]);
            }
        }
        snd_power_change_state(card, SNDRV_CTL_POWER_D0);
        0
    }
}
// static SIMPLE_DEV_PM_OPS(snd_ca0106_pm, snd_ca0106_suspend, snd_ca0106_resume);
// #define SND_CA0106_PM_OPS &snd_ca0106_pm

static snd_ca0106_ids: [pci_device_id; 2] = [
    pci_device_id { vendor: 0x1102, device: 0x0007 }, /* Audigy LS or Live 24bit */
    pci_device_id { vendor: 0, device: 0 },
];
// MODULE_DEVICE_TABLE(pci, snd_ca0106_ids);

static mut ca0106_driver: pci_driver = pci_driver {
    name: b"snd_ca0106\0".as_ptr() as *const c_char,
    id_table: snd_ca0106_ids.as_ptr(),
    probe: Some(snd_ca0106_probe),
    driver: device_driver {
        pm: ptr::null(),
    },
};
// module_pci_driver(ca0106_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
