// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Francisco Moraes <fmoraes@nc.rr.com>
 *  Driver EMU10K1X chips
 *
 *  Parts of this code were adapted from audigyls.c driver which is
 *  Copyright (c) by James Courtier-Dutton <James@superbug.demon.co.uk>
 *
 *  BUGS:
 *    --
 *
 *  TODO:
 *
 *  Chips (SB0200 model):
 *    - EMU10K1X-DBQ
 *    - STAC 9708T
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_uint;
type spinlock_t = c_uint;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool_; SNDRV_CARDS] = [true; SNDRV_CARDS];

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut enable: [bool_; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;

/* Module metadata and module parameters:
 * MODULE_AUTHOR("Francisco Moraes <fmoraes@nc.rr.com>");
 * MODULE_DESCRIPTION("EMU10K1X");
 * MODULE_LICENSE("GPL");
 * module_param_array(index, int, NULL, 0444);
 * MODULE_PARM_DESC(index, "Index value for the EMU10K1X soundcard.");
 * module_param_array(id, charp, NULL, 0444);
 * MODULE_PARM_DESC(id, "ID string for the EMU10K1X soundcard.");
 * module_param_array(enable, bool, NULL, 0444);
 * MODULE_PARM_DESC(enable, "Enable the EMU10K1X soundcard.");
 */

// some definitions were borrowed from emu10k1 driver as they seem to be the same
/************************************************************************************************/
/* PCI function 0 registers, address = <val> + PCIBASE0						*/
/************************************************************************************************/

const PTR: c_ulong = 0x00; /* Indexed register set pointer register */
/* NOTE: The CHANNELNUM and ADDRESS words can
 * be modified independently of each other.
 */

const DATA: c_ulong = 0x04; /* Indexed register set data register */

const IPR: c_ulong = 0x08; /* Global interrupt pending register */
/* Clear pending interrupts by writing a 1 to
 * the relevant bits and zero to the other bits
 */
const IPR_MIDITRANSBUFEMPTY: c_uint = 0x00000001; /* MIDI UART transmit buffer empty */
const IPR_MIDIRECVBUFEMPTY: c_uint = 0x00000002; /* MIDI UART receive buffer empty */
const IPR_CH_0_LOOP: c_uint = 0x00000800; /* Channel 0 loop */
const IPR_CH_0_HALF_LOOP: c_uint = 0x00000100; /* Channel 0 half loop */
const IPR_CAP_0_LOOP: c_uint = 0x00080000; /* Channel capture loop */
const IPR_CAP_0_HALF_LOOP: c_uint = 0x00010000; /* Channel capture half loop */

const INTE: c_ulong = 0x0c; /* Interrupt enable register */
const INTE_MIDITXENABLE: c_uint = 0x00000001; /* Enable MIDI transmit-buffer-empty interrupts */
const INTE_MIDIRXENABLE: c_uint = 0x00000002; /* Enable MIDI receive-buffer-empty interrupts */
const INTE_CH_0_LOOP: c_uint = 0x00000800; /* Channel 0 loop */
const INTE_CH_0_HALF_LOOP: c_uint = 0x00000100; /* Channel 0 half loop */
const INTE_CAP_0_LOOP: c_uint = 0x00080000; /* Channel capture loop */
const INTE_CAP_0_HALF_LOOP: c_uint = 0x00010000; /* Channel capture half loop */

const HCFG: c_ulong = 0x14; /* Hardware config register */

const HCFG_LOCKSOUNDCACHE: c_uint = 0x00000008; /* 1 = Cancel bustmaster accesses to soundcache */
/* NOTE: This should generally never be used. */
const HCFG_AUDIOENABLE: c_uint = 0x00000001; /* 0 = CODECs transmit zero-valued samples */
/* Should be set to 1 when the EMU10K1 is
 * completely initialized.
 */
const GPIO: c_ulong = 0x18; /* Defaults: 00001080-Analog, 00001000-SPDIF. */

const AC97DATA: c_ulong = 0x1c; /* AC97 register set data register (16 bit) */

const AC97ADDRESS: c_ulong = 0x1e; /* AC97 register set address register (8 bit) */

/********************************************************************************************************/
/* Emu10k1x pointer-offset register set, accessed through the PTR and DATA registers			*/
/********************************************************************************************************/
const PLAYBACK_LIST_ADDR: c_uint = 0x00; /* Base DMA address of a list of pointers to each period/size */
/* One list entry: 4 bytes for DMA address,
 * 4 bytes for period_size << 16.
 * One list entry is 8 bytes long.
 * One list entry for each period in the buffer.
 */
const PLAYBACK_LIST_SIZE: c_uint = 0x01; /* Size of list in bytes << 16. E.g. 8 periods -> 0x00380000 */
const PLAYBACK_LIST_PTR: c_uint = 0x02; /* Pointer to the current period being played */
const PLAYBACK_DMA_ADDR: c_uint = 0x04; /* Playback DMA address */
const PLAYBACK_PERIOD_SIZE: c_uint = 0x05; /* Playback period size */
const PLAYBACK_POINTER: c_uint = 0x06; /* Playback period pointer. Sample currently in DAC */
const PLAYBACK_UNKNOWN1: c_uint = 0x07;
const PLAYBACK_UNKNOWN2: c_uint = 0x08;

/* Only one capture channel supported */
const CAPTURE_DMA_ADDR: c_uint = 0x10; /* Capture DMA address */
const CAPTURE_BUFFER_SIZE: c_uint = 0x11; /* Capture buffer size */
const CAPTURE_POINTER: c_uint = 0x12; /* Capture buffer pointer. Sample currently in ADC */
const CAPTURE_UNKNOWN: c_uint = 0x13;

/* From 0x20 - 0x3f, last samples played on each channel */

const TRIGGER_CHANNEL: c_uint = 0x40; /* Trigger channel playback */
const TRIGGER_CHANNEL_0: c_uint = 0x00000001; /* Trigger channel 0 */
const TRIGGER_CHANNEL_1: c_uint = 0x00000002; /* Trigger channel 1 */
const TRIGGER_CHANNEL_2: c_uint = 0x00000004; /* Trigger channel 2 */
const TRIGGER_CAPTURE: c_uint = 0x00000100; /* Trigger capture channel */

const ROUTING: c_uint = 0x41; /* Setup sound routing ? */
const ROUTING_FRONT_LEFT: c_uint = 0x00000001;
const ROUTING_FRONT_RIGHT: c_uint = 0x00000002;
const ROUTING_REAR_LEFT: c_uint = 0x00000004;
const ROUTING_REAR_RIGHT: c_uint = 0x00000008;
const ROUTING_CENTER_LFE: c_uint = 0x00010000;

const SPCS0: c_uint = 0x42; /* SPDIF output Channel Status 0 register */

const SPCS1: c_uint = 0x43; /* SPDIF output Channel Status 1 register */

const SPCS2: c_uint = 0x44; /* SPDIF output Channel Status 2 register */

const SPCS_CLKACCYMASK: c_uint = 0x30000000; /* Clock accuracy */
const SPCS_CLKACCY_1000PPM: c_uint = 0x00000000; /* 1000 parts per million */
const SPCS_CLKACCY_50PPM: c_uint = 0x10000000; /* 50 parts per million */
const SPCS_CLKACCY_VARIABLE: c_uint = 0x20000000; /* Variable accuracy */
const SPCS_SAMPLERATEMASK: c_uint = 0x0f000000; /* Sample rate */
const SPCS_SAMPLERATE_44: c_uint = 0x00000000; /* 44.1kHz sample rate */
const SPCS_SAMPLERATE_48: c_uint = 0x02000000; /* 48kHz sample rate */
const SPCS_SAMPLERATE_32: c_uint = 0x03000000; /* 32kHz sample rate */
const SPCS_CHANNELNUMMASK: c_uint = 0x00f00000; /* Channel number */
const SPCS_CHANNELNUM_UNSPEC: c_uint = 0x00000000; /* Unspecified channel number */
const SPCS_CHANNELNUM_LEFT: c_uint = 0x00100000; /* Left channel */
const SPCS_CHANNELNUM_RIGHT: c_uint = 0x00200000; /* Right channel */
const SPCS_SOURCENUMMASK: c_uint = 0x000f0000; /* Source number */
const SPCS_SOURCENUM_UNSPEC: c_uint = 0x00000000; /* Unspecified source number */
const SPCS_GENERATIONSTATUS: c_uint = 0x00008000; /* Originality flag (see IEC-958 spec) */
const SPCS_CATEGORYCODEMASK: c_uint = 0x00007f00; /* Category code (see IEC-958 spec) */
const SPCS_MODEMASK: c_uint = 0x000000c0; /* Mode (see IEC-958 spec) */
const SPCS_EMPHASISMASK: c_uint = 0x00000038; /* Emphasis */
const SPCS_EMPHASIS_NONE: c_uint = 0x00000000; /* No emphasis */
const SPCS_EMPHASIS_50_15: c_uint = 0x00000008; /* 50/15 usec 2 channel */
const SPCS_COPYRIGHT: c_uint = 0x00000004; /* Copyright asserted flag -- do not modify */
const SPCS_NOTAUDIODATA: c_uint = 0x00000002; /* 0 = Digital audio, 1 = not audio */
const SPCS_PROFESSIONAL: c_uint = 0x00000001; /* 0 = Consumer (IEC-958), 1 = pro (AES3-1992) */

const SPDIF_SELECT: c_uint = 0x45; /* Enables SPDIF or Analogue outputs 0-Analogue, 0x700-SPDIF */

/* This is the MPU port on the card */
const MUDATA: c_int = 0x47;
const MUCMD: c_int = 0x48;
const MUSTAT: c_int = MUCMD;

/* From 0x50 - 0x5f, last samples captured */

/*
 * The hardware has 3 channels for playback and 1 for capture.
 *  - channel 0 is the front channel
 *  - channel 1 is the rear channel
 *  - channel 2 is the center/lfe channel
 * Volume is controlled by the AC97 for the front and rear channels by
 * the PCM Playback Volume, Sigmatel Surround Playback Volume and
 * Surround Playback Volume. The Sigmatel 4-Speaker Stereo switch affects
 * the front/rear channel mixing in the REAR OUT jack. When using the
 * 4-Speaker Stereo, both front and rear channels will be mixed in the
 * REAR OUT.
 * The center/lfe channel has no volume control and cannot be muted during
 * playback.
 */

#[repr(C)]
struct emu10k1x_voice {
    emu: *mut emu10k1x,
    number: c_int,
    use_: c_int,
    epcm: *mut emu10k1x_pcm,
}

#[repr(C)]
struct emu10k1x_pcm {
    emu: *mut emu10k1x,
    substream: *mut snd_pcm_substream,
    voice: *mut emu10k1x_voice,
    running: u16,
}

#[repr(C)]
struct emu10k1x_midi {
    emu: *mut emu10k1x,
    rmidi: *mut snd_rawmidi,
    substream_input: *mut snd_rawmidi_substream,
    substream_output: *mut snd_rawmidi_substream,
    midi_mode: c_uint,
    input_lock: spinlock_t,
    output_lock: spinlock_t,
    open_lock: spinlock_t,
    tx_enable: c_int,
    rx_enable: c_int,
    port: c_int,
    ipr_tx: c_int,
    ipr_rx: c_int,
    interrupt: Option<unsafe extern "C" fn(*mut emu10k1x, c_uint)>,
}

// definition of the chip-specific record
#[repr(C)]
struct emu10k1x {
    card: *mut snd_card,
    pci: *mut pci_dev,
    port: c_ulong,
    irq: c_int,
    revision: u8,      /* chip revision */
    serial: c_uint,    /* serial number */
    model: u16,        /* subsystem id */
    emu_lock: spinlock_t,
    voice_lock: spinlock_t,
    ac97: *mut snd_ac97,
    pcm: *mut snd_pcm,
    voices: [emu10k1x_voice; 3],
    capture_voice: emu10k1x_voice,
    spdif_bits: [u32; 3], // SPDIF out setup
    dma_buffer: *mut snd_dma_buffer,
    midi: emu10k1x_midi,
}

#[repr(C)]
struct snd_pcm_hardware {
    info: c_uint,
    formats: c_uint,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    buffer_bytes_max: c_uint,
    period_bytes_min: c_uint,
    period_bytes_max: c_uint,
    periods_min: c_uint,
    periods_max: c_uint,
    fifo_size: c_uint,
}

#[repr(C)]
struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
struct snd_rawmidi_ops {
    open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}

#[repr(C)]
struct snd_kcontrol_new {
    iface: c_uint,
    name: *const c_char,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    access: c_uint,
    count: c_uint,
}

#[repr(C)]
struct snd_ac97_bus_ops {
    write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>,
    read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16>,
}

#[repr(C)]
struct snd_ac97_template {
    private_data: *mut c_void,
    scaps: c_uint,
}

#[repr(C)]
struct snd_pcm_chmap_elem {
    channels: c_uint,
    map: [c_uint; 4],
}

#[repr(C)]
struct snd_pcm_runtime {
    private_data: *mut c_void,
    private_free: Option<unsafe extern "C" fn(*mut snd_pcm_runtime)>,
    hw: snd_pcm_hardware,
    period_size: snd_pcm_uframes_t,
    periods: c_uint,
    dma_addr: c_uint,
    buffer_size: snd_pcm_uframes_t,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    pcm: *mut snd_pcm,
    ops: *mut snd_pcm_ops,
}

#[repr(C)]
struct snd_pcm {
    device: c_int,
    private_data: *mut c_void,
    info_flags: c_uint,
    name: [c_char; 80],
}

#[repr(C)]
struct snd_rawmidi {
    private_data: *mut c_void,
    private_free: Option<unsafe extern "C" fn(*mut snd_rawmidi)>,
    info_flags: c_uint,
    name: [c_char; 80],
}

#[repr(C)]
struct snd_rawmidi_substream {
    rmidi: *mut snd_rawmidi,
}

#[repr(C)]
struct snd_card {
    private_data: *mut c_void,
    private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    sync_irq: c_int,
    dev: *mut device,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

#[repr(C)]
struct pci_dev {
    dev: device,
    irq: c_int,
    revision: u8,
}

#[repr(C)]
struct device;
#[repr(C)]
struct snd_ac97 {
    private_data: *mut c_void,
}
#[repr(C)]
struct snd_ac97_bus {
    no_vra: c_int,
}
#[repr(C)]
struct snd_dma_buffer {
    area: *mut u8,
    addr: c_uint,
}
#[repr(C)]
struct snd_pcm_hw_params;
#[repr(C)]
struct snd_info_entry {
    private_data: *mut c_void,
}
#[repr(C)]
struct snd_info_buffer;
#[repr(C)]
struct snd_kcontrol;
#[repr(C)]
struct snd_ctl_elem_info {
    type_: c_uint,
    count: c_uint,
}
#[repr(C)]
struct snd_ctl_elem_id;
#[repr(C)]
struct snd_ctl_elem_value {
    id: snd_ctl_elem_id,
    value: snd_ctl_elem_value_value,
}
#[repr(C)]
union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
    iec958: snd_ctl_elem_value_iec958,
}
#[repr(C)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}
type c_long = i64;
#[repr(C)]
struct snd_ctl_elem_value_iec958 {
    status: [u8; 24],
}
#[repr(C)]
struct pci_device_id;
#[repr(C)]
struct pci_driver {
    name: *const c_char,
    id_table: *const pci_device_id,
    probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
}

extern "C" {
    static snd_pcm_std_chmaps: *const snd_pcm_chmap_elem;
    static KBUILD_MODNAME: *const c_char;
    static THIS_MODULE: *mut c_void;

    fn outl(value: c_uint, port: c_ulong);
    fn inl(port: c_ulong) -> c_uint;
    fn outb(value: u8, port: c_ulong);
    fn inw(port: c_ulong) -> u16;
    fn outw(value: u16, port: c_ulong);
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut emu10k1x;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_uint) -> c_int;
    fn snd_pcm_hw_constraint_step(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, step: c_ulong) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, type_: c_int, data: *mut device, size: usize, max: usize);
    fn snd_pcm_add_chmap_ctls(pcm: *mut snd_pcm, stream: c_int, map: *const snd_pcm_chmap_elem, max_channels: c_int, private_value: c_ulong, info_ret: *mut c_void) -> c_int;
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, rbus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn snd_devm_alloc_pages(dev: *mut device, type_: c_int, size: usize) -> *mut snd_dma_buffer;
    fn pci_set_master(pci: *mut pci_dev);
    fn pci_read_config_dword(pci: *mut pci_dev, where_: c_int, val: *mut c_uint) -> c_int;
    fn pci_read_config_word(pci: *mut pci_dev, where_: c_int, val: *mut u16) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: c_int) -> c_int;
    fn snd_card_rw_proc_new(card: *mut snd_card, name: *const c_char, private_data: *mut c_void, read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer), write: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer));
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut emu10k1x;
    fn snd_ctl_get_ioffidx(kcontrol: *mut snd_kcontrol, id: *mut snd_ctl_elem_id) -> c_uint;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buffer: *mut u8, count: usize) -> c_int;
    fn snd_rawmidi_transmit(substream: *mut snd_rawmidi_substream, buffer: *mut u8, count: usize) -> c_int;
    fn snd_BUG_ON(condition: bool) -> bool;
    fn snd_rawmidi_new(card: *mut snd_card, id: *const c_char, device: c_int, output_count: c_int, input_count: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_rawmidi_set_ops(rmidi: *mut snd_rawmidi, stream: c_int, ops: *const snd_rawmidi_ops);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, ret: c_int) -> c_int;
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENXIO: c_int = 6;
const EIO: c_int = 5;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: c_uint = 0x80;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 2;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 3;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 0;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 0;
const SNDRV_PCM_HW_PARAM_PERIODS: c_uint = 0;
const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_uint = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_CHMAP_RL: c_uint = 0;
const SNDRV_CHMAP_RR: c_uint = 1;
const SNDRV_CHMAP_FC: c_uint = 2;
const SNDRV_CHMAP_LFE: c_uint = 3;
const AC97_SCAP_NO_SPDIF: c_uint = 1 << 0;
const DMA_BIT_MASK_28: u64 = (1u64 << 28) - 1;
const PCI_SUBSYSTEM_VENDOR_ID: c_int = 0x2c;
const PCI_SUBSYSTEM_ID: c_int = 0x2e;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_PCM: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1 << 0;
const SNDRV_RAWMIDI_STREAM_OUTPUT: c_int = 0;
const SNDRV_RAWMIDI_STREAM_INPUT: c_int = 1;
const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 1 << 0;
const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 1 << 1;
const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 1 << 2;

/* hardware definition */
static snd_emu10k1x_playback_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_48000,
    rate_min: 48000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 32 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 16 * 1024,
    periods_min: 2,
    periods_max: 8,
    fifo_size: 0,
};

static snd_emu10k1x_capture_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_48000,
    rate_min: 48000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 32 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 16 * 1024,
    periods_min: 2,
    periods_max: 2,
    fifo_size: 0,
};

unsafe extern "C" fn snd_emu10k1x_ptr_read(emu: *mut emu10k1x, reg: c_uint, chn: c_uint) -> c_uint {
    let regptr: c_uint = (reg << 16) | chn;
    outl(regptr, (*emu).port + PTR);
    inl((*emu).port + DATA)
}

unsafe extern "C" fn snd_emu10k1x_ptr_write(emu: *mut emu10k1x, reg: c_uint, chn: c_uint, data: c_uint) {
    let regptr: c_uint = (reg << 16) | chn;
    outl(regptr, (*emu).port + PTR);
    outl(data, (*emu).port + DATA);
}

unsafe extern "C" fn snd_emu10k1x_intr_enable(emu: *mut emu10k1x, intrenb: c_uint) {
    let intr_enable: c_uint = inl((*emu).port + INTE) | intrenb;
    outl(intr_enable, (*emu).port + INTE);
}

unsafe extern "C" fn snd_emu10k1x_intr_disable(emu: *mut emu10k1x, intrenb: c_uint) {
    let intr_enable: c_uint = inl((*emu).port + INTE) & !intrenb;
    outl(intr_enable, (*emu).port + INTE);
}

unsafe extern "C" fn snd_emu10k1x_gpio_write(emu: *mut emu10k1x, value: c_uint) {
    outl(value, (*emu).port + GPIO);
}

unsafe extern "C" fn snd_emu10k1x_pcm_free_substream(runtime: *mut snd_pcm_runtime) {
    kfree((*runtime).private_data);
}

unsafe extern "C" fn snd_emu10k1x_pcm_interrupt(_emu: *mut emu10k1x, voice: *mut emu10k1x_voice) {
    let epcm: *mut emu10k1x_pcm = (*voice).epcm;
    if epcm.is_null() {
        return;
    }
    if (*epcm).substream.is_null() {
        return;
    }
    /*
    dev_info(emu->card->dev,
             "IRQ: position = 0x%x, period = 0x%x, size = 0x%x\n",
               epcm->substream->ops->pointer(epcm->substream),
               snd_pcm_lib_period_bytes(epcm->substream),
               snd_pcm_lib_buffer_bytes(epcm->substream));
    */
    snd_pcm_period_elapsed((*epcm).substream);
}

/* open callback */
unsafe extern "C" fn snd_emu10k1x_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut emu10k1x = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut err: c_int;

    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }
    err = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 64);
    if err < 0 {
        return err;
    }

    let epcm: *mut emu10k1x_pcm = kzalloc(size_of::<emu10k1x_pcm>(), GFP_KERNEL) as *mut emu10k1x_pcm;
    if epcm.is_null() {
        return -ENOMEM;
    }
    (*epcm).emu = chip;
    (*epcm).substream = substream;

    (*runtime).private_data = epcm as *mut c_void;
    (*runtime).private_free = Some(snd_emu10k1x_pcm_free_substream);

    (*runtime).hw = snd_emu10k1x_playback_hw;
    0
}

/* close callback */
unsafe extern "C" fn snd_emu10k1x_playback_close(_substream: *mut snd_pcm_substream) -> c_int {
    0
}

/* hw_params callback */
unsafe extern "C" fn snd_emu10k1x_pcm_hw_params(substream: *mut snd_pcm_substream, _hw_params: *mut snd_pcm_hw_params) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let epcm: *mut emu10k1x_pcm = (*runtime).private_data as *mut emu10k1x_pcm;

    if (*epcm).voice.is_null() {
        (*epcm).voice = &mut (*(*epcm).emu).voices[(*(*substream).pcm).device as usize];
        (*(*epcm).voice).use_ = 1;
        (*(*epcm).voice).epcm = epcm;
    }
    0
}

/* hw_free callback */
unsafe extern "C" fn snd_emu10k1x_pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;

    if (*runtime).private_data.is_null() {
        return 0;
    }

    let epcm: *mut emu10k1x_pcm = (*runtime).private_data as *mut emu10k1x_pcm;
    if !(*epcm).voice.is_null() {
        (*(*epcm).voice).use_ = 0;
        (*(*epcm).voice).epcm = ptr::null_mut();
        (*epcm).voice = ptr::null_mut();
    }
    0
}

/* prepare callback */
unsafe extern "C" fn snd_emu10k1x_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let emu: *mut emu10k1x = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let epcm: *mut emu10k1x_pcm = (*runtime).private_data as *mut emu10k1x_pcm;
    let voice: c_int = (*(*epcm).voice).number;
    let mut table_base: *mut u32 = (*(*emu).dma_buffer).area.add(1024 * voice as usize) as *mut u32;
    let period_size_bytes: u32 = frames_to_bytes(runtime, (*runtime).period_size);
    let mut i: c_uint = 0;

    while i < (*runtime).periods {
        *table_base = (*runtime).dma_addr.wrapping_add(i.wrapping_mul(period_size_bytes));
        table_base = table_base.add(1);
        *table_base = period_size_bytes << 16;
        table_base = table_base.add(1);
        i += 1;
    }

    snd_emu10k1x_ptr_write(emu, PLAYBACK_LIST_ADDR, voice as c_uint, (*(*emu).dma_buffer).addr + 1024 * voice as c_uint);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_LIST_SIZE, voice as c_uint, ((*runtime).periods - 1) << 19);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_LIST_PTR, voice as c_uint, 0);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_POINTER, voice as c_uint, 0);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_UNKNOWN1, voice as c_uint, 0);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_UNKNOWN2, voice as c_uint, 0);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_DMA_ADDR, voice as c_uint, (*runtime).dma_addr);
    snd_emu10k1x_ptr_write(emu, PLAYBACK_PERIOD_SIZE, voice as c_uint, frames_to_bytes(runtime, (*runtime).period_size) << 16);
    0
}

/* trigger callback */
unsafe extern "C" fn snd_emu10k1x_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let emu: *mut emu10k1x = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let epcm: *mut emu10k1x_pcm = (*runtime).private_data as *mut emu10k1x_pcm;
    let channel: c_int = (*(*epcm).voice).number;
    let mut result: c_int = 0;

    /*
    dev_dbg(emu->card->dev,
        "trigger - emu10k1x = 0x%x, cmd = %i, pointer = %d\n",
        (int)emu, cmd, (int)substream->ops->pointer(substream));
    */

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            if (*runtime).periods == 2 {
                snd_emu10k1x_intr_enable(emu, (INTE_CH_0_LOOP | INTE_CH_0_HALF_LOOP) << channel);
            } else {
                snd_emu10k1x_intr_enable(emu, INTE_CH_0_LOOP << channel);
            }
            (*epcm).running = 1;
            snd_emu10k1x_ptr_write(emu, TRIGGER_CHANNEL, 0, snd_emu10k1x_ptr_read(emu, TRIGGER_CHANNEL, 0) | (TRIGGER_CHANNEL_0 << channel));
        }
        SNDRV_PCM_TRIGGER_STOP => {
            (*epcm).running = 0;
            snd_emu10k1x_intr_disable(emu, (INTE_CH_0_LOOP | INTE_CH_0_HALF_LOOP) << channel);
            snd_emu10k1x_ptr_write(emu, TRIGGER_CHANNEL, 0, snd_emu10k1x_ptr_read(emu, TRIGGER_CHANNEL, 0) & !(TRIGGER_CHANNEL_0 << channel));
        }
        _ => {
            result = -EINVAL;
        }
    }
    result
}

/* pointer callback */
unsafe extern "C" fn snd_emu10k1x_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let emu: *mut emu10k1x = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let epcm: *mut emu10k1x_pcm = (*runtime).private_data as *mut emu10k1x_pcm;
    let channel: c_int = (*(*epcm).voice).number;
    let mut ptr_: snd_pcm_uframes_t;
    let ptr1: snd_pcm_uframes_t;
    let mut ptr2: snd_pcm_uframes_t;
    let ptr3: snd_pcm_uframes_t;
    let ptr4: snd_pcm_uframes_t;

    if (*epcm).running == 0 {
        return 0;
    }

    ptr3 = snd_emu10k1x_ptr_read(emu, PLAYBACK_LIST_PTR, channel as c_uint) as snd_pcm_uframes_t;
    let mut ptr1_mut = snd_emu10k1x_ptr_read(emu, PLAYBACK_POINTER, channel as c_uint) as snd_pcm_uframes_t;
    ptr4 = snd_emu10k1x_ptr_read(emu, PLAYBACK_LIST_PTR, channel as c_uint) as snd_pcm_uframes_t;

    if ptr4 == 0 && ptr1_mut == frames_to_bytes(runtime, (*runtime).buffer_size) as snd_pcm_uframes_t {
        return 0;
    }

    if ptr3 != ptr4 {
        ptr1_mut = snd_emu10k1x_ptr_read(emu, PLAYBACK_POINTER, channel as c_uint) as snd_pcm_uframes_t;
    }
    ptr1 = ptr1_mut;
    ptr2 = bytes_to_frames(runtime, ptr1 as c_uint);
    ptr2 += (ptr4 >> 3) * (*runtime).period_size;
    ptr_ = ptr2;

    if ptr_ >= (*runtime).buffer_size {
        ptr_ -= (*runtime).buffer_size;
    }
    ptr_
}

/* operators */
static snd_emu10k1x_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_emu10k1x_playback_open),
    close: Some(snd_emu10k1x_playback_close),
    hw_params: Some(snd_emu10k1x_pcm_hw_params),
    hw_free: Some(snd_emu10k1x_pcm_hw_free),
    prepare: Some(snd_emu10k1x_pcm_prepare),
    trigger: Some(snd_emu10k1x_pcm_trigger),
    pointer: Some(snd_emu10k1x_pcm_pointer),
};

/* open_capture callback */
unsafe extern "C" fn snd_emu10k1x_pcm_open_capture(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut emu10k1x = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut err: c_int;

    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        return err;
    }
    err = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 64);
    if err < 0 {
        return err;
    }

    let epcm: *mut emu10k1x_pcm = kzalloc(size_of::<emu10k1x_pcm>(), GFP_KERNEL) as *mut emu10k1x_pcm;
    if epcm.is_null() {
        return -ENOMEM;
    }

    (*epcm).emu = chip;
    (*epcm).substream = substream;

    (*runtime).private_data = epcm as *mut c_void;
    (*runtime).private_free = Some(snd_emu10k1x_pcm_free_substream);
    (*runtime).hw = snd_emu10k1x_capture_hw;
    0
}

/* close callback */
unsafe extern "C" fn snd_emu10k1x_pcm_close_capture(_substream: *mut snd_pcm_substream) -> c_int {
    0
}

/* hw_params callback */
unsafe extern "C" fn snd_emu10k1x_pcm_hw_params_capture(substream: *mut snd_pcm_substream, _hw_params: *mut snd_pcm_hw_params) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let epcm: *mut emu10k1x_pcm = (*runtime).private_data as *mut emu10k1x_pcm;

    if (*epcm).voice.is_null() {
        if (*(*epcm).emu).capture_voice.use_ != 0 {
            return -EBUSY;
        }
        (*epcm).voice = &mut (*(*epcm).emu).capture_voice;
        (*(*epcm).voice).epcm = epcm;
        (*(*epcm).voice).use_ = 1;
    }
    0
}

/* hw_free callback */
unsafe extern "C" fn snd_emu10k1x_pcm_hw_free_capture(substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;

    if (*runtime).private_data.is_null() {
        return 0;
    }
    let epcm: *mut emu10k1x_pcm = (*runtime).private_data as *mut emu10k1x_pcm;

    if !(*epcm).voice.is_null() {
        (*(*epcm).voice).use_ = 0;
        (*(*epcm).voice).epcm = ptr::null_mut();
        (*epcm).voice = ptr::null_mut();
    }
    0
}

/* prepare capture callback */
unsafe extern "C" fn snd_emu10k1x_pcm_prepare_capture(substream: *mut snd_pcm_substream) -> c_int {
    let emu: *mut emu10k1x = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;

    snd_emu10k1x_ptr_write(emu, CAPTURE_DMA_ADDR, 0, (*runtime).dma_addr);
    snd_emu10k1x_ptr_write(emu, CAPTURE_BUFFER_SIZE, 0, frames_to_bytes(runtime, (*runtime).buffer_size) << 16); // buffer size in bytes
    snd_emu10k1x_ptr_write(emu, CAPTURE_POINTER, 0, 0);
    snd_emu10k1x_ptr_write(emu, CAPTURE_UNKNOWN, 0, 0);
    0
}

/* trigger_capture callback */
unsafe extern "C" fn snd_emu10k1x_pcm_trigger_capture(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let emu: *mut emu10k1x = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let epcm: *mut emu10k1x_pcm = (*runtime).private_data as *mut emu10k1x_pcm;
    let mut result: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            snd_emu10k1x_intr_enable(emu, INTE_CAP_0_LOOP | INTE_CAP_0_HALF_LOOP);
            snd_emu10k1x_ptr_write(emu, TRIGGER_CHANNEL, 0, snd_emu10k1x_ptr_read(emu, TRIGGER_CHANNEL, 0) | TRIGGER_CAPTURE);
            (*epcm).running = 1;
        }
        SNDRV_PCM_TRIGGER_STOP => {
            (*epcm).running = 0;
            snd_emu10k1x_intr_disable(emu, INTE_CAP_0_LOOP | INTE_CAP_0_HALF_LOOP);
            snd_emu10k1x_ptr_write(emu, TRIGGER_CHANNEL, 0, snd_emu10k1x_ptr_read(emu, TRIGGER_CHANNEL, 0) & !TRIGGER_CAPTURE);
        }
        _ => {
            result = -EINVAL;
        }
    }
    result
}

/* pointer_capture callback */
unsafe extern "C" fn snd_emu10k1x_pcm_pointer_capture(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let emu: *mut emu10k1x = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let epcm: *mut emu10k1x_pcm = (*runtime).private_data as *mut emu10k1x_pcm;
    let mut ptr_: snd_pcm_uframes_t;

    if (*epcm).running == 0 {
        return 0;
    }

    ptr_ = bytes_to_frames(runtime, snd_emu10k1x_ptr_read(emu, CAPTURE_POINTER, 0));
    if ptr_ >= (*runtime).buffer_size {
        ptr_ -= (*runtime).buffer_size;
    }
    ptr_
}

static snd_emu10k1x_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_emu10k1x_pcm_open_capture),
    close: Some(snd_emu10k1x_pcm_close_capture),
    hw_params: Some(snd_emu10k1x_pcm_hw_params_capture),
    hw_free: Some(snd_emu10k1x_pcm_hw_free_capture),
    prepare: Some(snd_emu10k1x_pcm_prepare_capture),
    trigger: Some(snd_emu10k1x_pcm_trigger_capture),
    pointer: Some(snd_emu10k1x_pcm_pointer_capture),
};

unsafe extern "C" fn snd_emu10k1x_ac97_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    let emu: *mut emu10k1x = (*ac97).private_data as *mut emu10k1x;
    outb(reg as u8, (*emu).port + AC97ADDRESS);
    inw((*emu).port + AC97DATA)
}

unsafe extern "C" fn snd_emu10k1x_ac97_write(ac97: *mut snd_ac97, reg: u16, val: u16) {
    let emu: *mut emu10k1x = (*ac97).private_data as *mut emu10k1x;
    outb(reg as u8, (*emu).port + AC97ADDRESS);
    outw(val, (*emu).port + AC97DATA);
}

unsafe extern "C" fn snd_emu10k1x_ac97(chip: *mut emu10k1x) -> c_int {
    let mut pbus: *mut snd_ac97_bus = ptr::null_mut();
    let mut ac97: snd_ac97_template = core::mem::zeroed();
    let mut err: c_int;
    static ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
        write: Some(snd_emu10k1x_ac97_write),
        read: Some(snd_emu10k1x_ac97_read),
    };

    err = snd_ac97_bus((*chip).card, 0, &ops, ptr::null_mut(), &mut pbus);
    if err < 0 {
        return err;
    }
    (*pbus).no_vra = 1; /* we don't need VRA */

    memset(&mut ac97 as *mut _ as *mut c_void, 0, size_of::<snd_ac97_template>());
    ac97.private_data = chip as *mut c_void;
    ac97.scaps = AC97_SCAP_NO_SPDIF;
    snd_ac97_mixer(pbus, &mut ac97, &mut (*chip).ac97)
}

unsafe extern "C" fn snd_emu10k1x_free(card: *mut snd_card) {
    let chip: *mut emu10k1x = (*card).private_data as *mut emu10k1x;

    snd_emu10k1x_ptr_write(chip, TRIGGER_CHANNEL, 0, 0);
    // disable interrupts
    outl(0, (*chip).port + INTE);
    // disable audio
    outl(HCFG_LOCKSOUNDCACHE, (*chip).port + HCFG);
}

unsafe extern "C" fn snd_emu10k1x_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let status: c_uint;
    let chip: *mut emu10k1x = dev_id as *mut emu10k1x;
    let mut pvoice: *mut emu10k1x_voice = (*chip).voices.as_mut_ptr();
    let mut i: c_int;
    let mut mask: c_int;

    status = inl((*chip).port + IPR);

    if status == 0 {
        return IRQ_NONE;
    }

    // capture interrupt
    if status & (IPR_CAP_0_LOOP | IPR_CAP_0_HALF_LOOP) != 0 {
        let cap_voice: *mut emu10k1x_voice = &mut (*chip).capture_voice;
        if (*cap_voice).use_ != 0 {
            snd_emu10k1x_pcm_interrupt(chip, cap_voice);
        } else {
            snd_emu10k1x_intr_disable(chip, INTE_CAP_0_LOOP | INTE_CAP_0_HALF_LOOP);
        }
    }

    mask = (IPR_CH_0_LOOP | IPR_CH_0_HALF_LOOP) as c_int;
    i = 0;
    while i < 3 {
        if status & mask as c_uint != 0 {
            if (*pvoice).use_ != 0 {
                snd_emu10k1x_pcm_interrupt(chip, pvoice);
            } else {
                snd_emu10k1x_intr_disable(chip, mask as c_uint);
            }
        }
        pvoice = pvoice.add(1);
        mask <<= 1;
        i += 1;
    }

    if status & (IPR_MIDITRANSBUFEMPTY | IPR_MIDIRECVBUFEMPTY) != 0 {
        if let Some(interrupt) = (*chip).midi.interrupt {
            interrupt(chip, status);
        } else {
            snd_emu10k1x_intr_disable(chip, INTE_MIDITXENABLE | INTE_MIDIRXENABLE);
        }
    }

    // acknowledge the interrupt if necessary
    outl(status, (*chip).port + IPR);

    /* dev_dbg(chip->card->dev, "interrupt %08x\n", status); */
    IRQ_HANDLED
}

static surround_map: [snd_pcm_chmap_elem; 2] = [
    snd_pcm_chmap_elem { channels: 2, map: [SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, 0, 0] },
    snd_pcm_chmap_elem { channels: 0, map: [0; 4] },
];

static clfe_map: [snd_pcm_chmap_elem; 2] = [
    snd_pcm_chmap_elem { channels: 2, map: [SNDRV_CHMAP_FC, SNDRV_CHMAP_LFE, 0, 0] },
    snd_pcm_chmap_elem { channels: 0, map: [0; 4] },
];

unsafe extern "C" fn snd_emu10k1x_pcm(emu: *mut emu10k1x, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut map: *const snd_pcm_chmap_elem = ptr::null();
    let mut err: c_int;
    let mut capture: c_int = 0;

    if device == 0 {
        capture = 1;
    }

    err = snd_pcm_new((*emu).card, c"emu10k1x".as_ptr(), device, 1, capture, &mut pcm);
    if err < 0 {
        return err;
    }

    (*pcm).private_data = emu as *mut c_void;

    match device {
        0 => {
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_emu10k1x_playback_ops);
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_emu10k1x_capture_ops);
        }
        1 | 2 => {
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_emu10k1x_playback_ops);
        }
        _ => {}
    }

    (*pcm).info_flags = 0;
    match device {
        0 => {
            strscpy((*pcm).name.as_mut_ptr(), c"EMU10K1X Front".as_ptr());
            map = snd_pcm_std_chmaps;
        }
        1 => {
            strscpy((*pcm).name.as_mut_ptr(), c"EMU10K1X Rear".as_ptr());
            map = surround_map.as_ptr();
        }
        2 => {
            strscpy((*pcm).name.as_mut_ptr(), c"EMU10K1X Center/LFE".as_ptr());
            map = clfe_map.as_ptr();
        }
        _ => {}
    }
    (*emu).pcm = pcm;

    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*emu).pci).dev, 32 * 1024, 32 * 1024);

    snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK, map, 2, 1 << 2, ptr::null_mut())
}

unsafe extern "C" fn snd_emu10k1x_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let chip: *mut emu10k1x = (*card).private_data as *mut emu10k1x;
    let mut err: c_int;
    let mut ch: c_int;

    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }

    if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK_28) < 0 {
        dev_err((*card).dev, c"error to set 28bit mask DMA\n".as_ptr());
        return -ENXIO;
    }

    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).irq = -1;

    spin_lock_init(&mut (*chip).emu_lock);
    spin_lock_init(&mut (*chip).voice_lock);

    err = pcim_request_all_regions(pci, c"EMU10K1X".as_ptr());
    if err < 0 {
        return err;
    }
    (*chip).port = pci_resource_start(pci, 0);

    if devm_request_irq(&mut (*pci).dev, (*pci).irq, snd_emu10k1x_interrupt, IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void) != 0 {
        dev_err((*card).dev, c"cannot grab irq %d\n".as_ptr(), (*pci).irq);
        return -EBUSY;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    (*card).private_free = Some(snd_emu10k1x_free);

    (*chip).dma_buffer = snd_devm_alloc_pages(&mut (*pci).dev, SNDRV_DMA_TYPE_DEV, 4 * 1024);
    if (*chip).dma_buffer.is_null() {
        return -ENOMEM;
    }

    pci_set_master(pci);
    /* read revision & serial */
    (*chip).revision = (*pci).revision;
    pci_read_config_dword(pci, PCI_SUBSYSTEM_VENDOR_ID, &mut (*chip).serial);
    pci_read_config_word(pci, PCI_SUBSYSTEM_ID, &mut (*chip).model);
    dev_info((*card).dev, c"Model %04x Rev %08x Serial %08x\n".as_ptr(), (*chip).model as c_int, (*chip).revision as c_uint, (*chip).serial);

    outl(0, (*chip).port + INTE);

    ch = 0;
    while ch < 3 {
        (*chip).voices[ch as usize].emu = chip;
        (*chip).voices[ch as usize].number = ch;
        ch += 1;
    }

    /*
     *  Init to 0x02109204 :
     *  Clock accuracy    = 0     (1000ppm)
     *  Sample Rate       = 2     (48kHz)
     *  Audio Channel     = 1     (Left of 2)
     *  Source Number     = 0     (Unspecified)
     *  Generation Status = 1     (Original for Cat Code 12)
     *  Cat Code          = 12    (Digital Signal Mixer)
     *  Mode              = 0     (Mode 0)
     *  Emphasis          = 0     (None)
     *  CP                = 1     (Copyright unasserted)
     *  AN                = 0     (Audio data)
     *  P                 = 0     (Consumer)
     */
    (*chip).spdif_bits[0] = SPCS_CLKACCY_1000PPM | SPCS_SAMPLERATE_48 | SPCS_CHANNELNUM_LEFT | SPCS_SOURCENUM_UNSPEC | SPCS_GENERATIONSTATUS | 0x00001200 | 0x00000000 | SPCS_EMPHASIS_NONE | SPCS_COPYRIGHT;
    snd_emu10k1x_ptr_write(chip, SPCS0, 0, (*chip).spdif_bits[0]);
    (*chip).spdif_bits[1] = SPCS_CLKACCY_1000PPM | SPCS_SAMPLERATE_48 | SPCS_CHANNELNUM_LEFT | SPCS_SOURCENUM_UNSPEC | SPCS_GENERATIONSTATUS | 0x00001200 | 0x00000000 | SPCS_EMPHASIS_NONE | SPCS_COPYRIGHT;
    snd_emu10k1x_ptr_write(chip, SPCS1, 0, (*chip).spdif_bits[1]);
    (*chip).spdif_bits[2] = SPCS_CLKACCY_1000PPM | SPCS_SAMPLERATE_48 | SPCS_CHANNELNUM_LEFT | SPCS_SOURCENUM_UNSPEC | SPCS_GENERATIONSTATUS | 0x00001200 | 0x00000000 | SPCS_EMPHASIS_NONE | SPCS_COPYRIGHT;
    snd_emu10k1x_ptr_write(chip, SPCS2, 0, (*chip).spdif_bits[2]);

    snd_emu10k1x_ptr_write(chip, SPDIF_SELECT, 0, 0x700); // disable SPDIF
    snd_emu10k1x_ptr_write(chip, ROUTING, 0, 0x1003F); // routing
    snd_emu10k1x_gpio_write(chip, 0x1080); // analog mode

    outl(HCFG_LOCKSOUNDCACHE | HCFG_AUDIOENABLE, (*chip).port + HCFG);
    0
}

unsafe extern "C" fn snd_emu10k1x_proc_reg_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let emu: *mut emu10k1x = (*entry).private_data as *mut emu10k1x;
    let mut value: c_ulong;
    let mut value1: c_ulong;
    let mut value2: c_ulong;
    let mut i: c_int;

    snd_iprintf(buffer, c"Registers:\n\n".as_ptr());
    i = 0;
    while i < 0x20 {
        value = inl((*emu).port + i as c_ulong) as c_ulong;
        snd_iprintf(buffer, c"Register %02X: %08lX\n".as_ptr(), i, value);
        i += 4;
    }
    snd_iprintf(buffer, c"\nRegisters\n\n".as_ptr());
    i = 0;
    while i <= 0x48 {
        value = snd_emu10k1x_ptr_read(emu, i as c_uint, 0) as c_ulong;
        if i < 0x10 || (i >= 0x20 && i < 0x40) {
            value1 = snd_emu10k1x_ptr_read(emu, i as c_uint, 1) as c_ulong;
            value2 = snd_emu10k1x_ptr_read(emu, i as c_uint, 2) as c_ulong;
            snd_iprintf(buffer, c"%02X: %08lX %08lX %08lX\n".as_ptr(), i, value, value1, value2);
        } else {
            snd_iprintf(buffer, c"%02X: %08lX\n".as_ptr(), i, value);
        }
        i += 1;
    }
}

unsafe extern "C" fn snd_emu10k1x_proc_reg_write(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let emu: *mut emu10k1x = (*entry).private_data as *mut emu10k1x;
    let mut line: [c_char; 64] = [0; 64];
    let mut reg: c_uint = 0;
    let mut channel_id: c_uint = 0;
    let mut val: c_uint = 0;

    while snd_info_get_line(buffer, line.as_mut_ptr(), size_of::<[c_char; 64]>() as c_int) == 0 {
        if sscanf(line.as_ptr(), c"%x %x %x".as_ptr(), &mut reg, &mut channel_id, &mut val) != 3 {
            continue;
        }

        if reg < 0x49 && channel_id <= 2 {
            snd_emu10k1x_ptr_write(emu, reg, channel_id, val);
        }
    }
}

unsafe extern "C" fn snd_emu10k1x_proc_init(emu: *mut emu10k1x) -> c_int {
    snd_card_rw_proc_new((*emu).card, c"emu10k1x_regs".as_ptr(), emu as *mut c_void, snd_emu10k1x_proc_reg_read, snd_emu10k1x_proc_reg_write);
    0
}

/* #define snd_emu10k1x_shared_spdif_info snd_ctl_boolean_mono_info */

unsafe extern "C" fn snd_emu10k1x_shared_spdif_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let emu: *mut emu10k1x = snd_kcontrol_chip(kcontrol);

    (*ucontrol).value.integer.value[0] = if snd_emu10k1x_ptr_read(emu, SPDIF_SELECT, 0) == 0x700 { 0 } else { 1 };
    0
}

unsafe extern "C" fn snd_emu10k1x_shared_spdif_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let emu: *mut emu10k1x = snd_kcontrol_chip(kcontrol);
    let val: c_uint;

    val = (*ucontrol).value.integer.value[0] as c_uint;

    if val != 0 {
        // enable spdif output
        snd_emu10k1x_ptr_write(emu, SPDIF_SELECT, 0, 0x000);
        snd_emu10k1x_ptr_write(emu, ROUTING, 0, 0x700);
        snd_emu10k1x_gpio_write(emu, 0x1000);
    } else {
        // disable spdif output
        snd_emu10k1x_ptr_write(emu, SPDIF_SELECT, 0, 0x700);
        snd_emu10k1x_ptr_write(emu, ROUTING, 0, 0x1003F);
        snd_emu10k1x_gpio_write(emu, 0x1080);
    }
    0
}

static snd_emu10k1x_shared_spdif: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"Analog/Digital Output Jack".as_ptr(),
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(snd_emu10k1x_shared_spdif_get),
    put: Some(snd_emu10k1x_shared_spdif_put),
    access: 0,
    count: 0,
};

unsafe extern "C" fn snd_emu10k1x_spdif_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn snd_emu10k1x_spdif_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let emu: *mut emu10k1x = snd_kcontrol_chip(kcontrol);
    let idx: c_uint = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id);

    (*ucontrol).value.iec958.status[0] = ((*emu).spdif_bits[idx as usize] >> 0) as u8 & 0xff;
    (*ucontrol).value.iec958.status[1] = ((*emu).spdif_bits[idx as usize] >> 8) as u8 & 0xff;
    (*ucontrol).value.iec958.status[2] = ((*emu).spdif_bits[idx as usize] >> 16) as u8 & 0xff;
    (*ucontrol).value.iec958.status[3] = ((*emu).spdif_bits[idx as usize] >> 24) as u8 & 0xff;
    0
}

unsafe extern "C" fn snd_emu10k1x_spdif_get_mask(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    (*ucontrol).value.iec958.status[0] = 0xff;
    (*ucontrol).value.iec958.status[1] = 0xff;
    (*ucontrol).value.iec958.status[2] = 0xff;
    (*ucontrol).value.iec958.status[3] = 0xff;
    0
}

unsafe extern "C" fn snd_emu10k1x_spdif_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let emu: *mut emu10k1x = snd_kcontrol_chip(kcontrol);
    let idx: c_uint = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id);
    let change: c_int;
    let val: c_uint;

    val = (((*ucontrol).value.iec958.status[0] as c_uint) << 0)
        | (((*ucontrol).value.iec958.status[1] as c_uint) << 8)
        | (((*ucontrol).value.iec958.status[2] as c_uint) << 16)
        | (((*ucontrol).value.iec958.status[3] as c_uint) << 24);
    change = (val != (*emu).spdif_bits[idx as usize]) as c_int;
    if change != 0 {
        snd_emu10k1x_ptr_write(emu, SPCS0 + idx, 0, val);
        (*emu).spdif_bits[idx as usize] = val;
    }
    change
}

static snd_emu10k1x_spdif_mask_control: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: c"IEC958 Playback Mask".as_ptr(),
    count: 3,
    info: Some(snd_emu10k1x_spdif_info),
    get: Some(snd_emu10k1x_spdif_get_mask),
    put: None,
};

static snd_emu10k1x_spdif_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: c"IEC958 Playback Default".as_ptr(),
    count: 3,
    info: Some(snd_emu10k1x_spdif_info),
    get: Some(snd_emu10k1x_spdif_get),
    put: Some(snd_emu10k1x_spdif_put),
    access: 0,
};

unsafe extern "C" fn snd_emu10k1x_mixer(emu: *mut emu10k1x) -> c_int {
    let mut err: c_int;
    let mut kctl: *mut snd_kcontrol;
    let card: *mut snd_card = (*emu).card;

    kctl = snd_ctl_new1(&snd_emu10k1x_spdif_mask_control, emu as *mut c_void);
    if kctl.is_null() {
        return -ENOMEM;
    }
    err = snd_ctl_add(card, kctl);
    if err != 0 {
        return err;
    }
    kctl = snd_ctl_new1(&snd_emu10k1x_shared_spdif, emu as *mut c_void);
    if kctl.is_null() {
        return -ENOMEM;
    }
    err = snd_ctl_add(card, kctl);
    if err != 0 {
        return err;
    }
    kctl = snd_ctl_new1(&snd_emu10k1x_spdif_control, emu as *mut c_void);
    if kctl.is_null() {
        return -ENOMEM;
    }
    err = snd_ctl_add(card, kctl);
    if err != 0 {
        return err;
    }
    0
}

const EMU10K1X_MIDI_MODE_INPUT: c_uint = 1 << 0;
const EMU10K1X_MIDI_MODE_OUTPUT: c_uint = 1 << 1;

unsafe fn mpu401_read(emu: *mut emu10k1x, mpu: *mut emu10k1x_midi, idx: c_int) -> u8 {
    snd_emu10k1x_ptr_read(emu, ((*mpu).port + idx) as c_uint, 0) as u8
}

unsafe fn mpu401_write(emu: *mut emu10k1x, mpu: *mut emu10k1x_midi, data: c_int, idx: c_int) {
    snd_emu10k1x_ptr_write(emu, ((*mpu).port + idx) as c_uint, 0, data as c_uint);
}

unsafe fn mpu401_write_data(emu: *mut emu10k1x, mpu: *mut emu10k1x_midi, data: c_int) {
    mpu401_write(emu, mpu, data, 0);
}

unsafe fn mpu401_write_cmd(emu: *mut emu10k1x, mpu: *mut emu10k1x_midi, data: c_int) {
    mpu401_write(emu, mpu, data, 1);
}

unsafe fn mpu401_read_data(emu: *mut emu10k1x, mpu: *mut emu10k1x_midi) -> u8 {
    mpu401_read(emu, mpu, 0)
}

unsafe fn mpu401_read_stat(emu: *mut emu10k1x, mpu: *mut emu10k1x_midi) -> u8 {
    mpu401_read(emu, mpu, 1)
}

unsafe fn mpu401_input_avail(emu: *mut emu10k1x, mpu: *mut emu10k1x_midi) -> bool {
    (mpu401_read_stat(emu, mpu) & 0x80) == 0
}

unsafe fn mpu401_output_ready(emu: *mut emu10k1x, mpu: *mut emu10k1x_midi) -> bool {
    (mpu401_read_stat(emu, mpu) & 0x40) == 0
}

const MPU401_RESET: c_int = 0xff;
const MPU401_ENTER_UART: c_int = 0x3f;
const MPU401_ACK: u8 = 0xfe;

unsafe extern "C" fn mpu401_clear_rx(emu: *mut emu10k1x, mpu: *mut emu10k1x_midi) {
    let mut timeout: c_int = 100000;
    while timeout > 0 && mpu401_input_avail(emu, mpu) {
        timeout -= 1;
        mpu401_read_data(emu, mpu);
    }
    /* CONFIG_SND_DEBUG:
    if (timeout <= 0)
        dev_err(emu->card->dev,
            "cmd: clear rx timeout (status = 0x%x)\n",
            mpu401_read_stat(emu, mpu));
    */
}

/*

 */

unsafe extern "C" fn do_emu10k1x_midi_interrupt(emu: *mut emu10k1x, midi: *mut emu10k1x_midi, status: c_uint) {
    let mut byte: u8 = 0;

    if (*midi).rmidi.is_null() {
        snd_emu10k1x_intr_disable(emu, ((*midi).tx_enable | (*midi).rx_enable) as c_uint);
        return;
    }

    spin_lock(&mut (*midi).input_lock);
    if (status & (*midi).ipr_rx as c_uint) != 0 && mpu401_input_avail(emu, midi) {
        if ((*midi).midi_mode & EMU10K1X_MIDI_MODE_INPUT) == 0 {
            mpu401_clear_rx(emu, midi);
        } else {
            byte = mpu401_read_data(emu, midi);
            if !(*midi).substream_input.is_null() {
                snd_rawmidi_receive((*midi).substream_input, &mut byte, 1);
            }
        }
    }
    spin_unlock(&mut (*midi).input_lock);

    spin_lock(&mut (*midi).output_lock);
    if (status & (*midi).ipr_tx as c_uint) != 0 && mpu401_output_ready(emu, midi) {
        if !(*midi).substream_output.is_null() && snd_rawmidi_transmit((*midi).substream_output, &mut byte, 1) == 1 {
            mpu401_write_data(emu, midi, byte as c_int);
        } else {
            snd_emu10k1x_intr_disable(emu, (*midi).tx_enable as c_uint);
        }
    }
    spin_unlock(&mut (*midi).output_lock);
}

unsafe extern "C" fn snd_emu10k1x_midi_interrupt(emu: *mut emu10k1x, status: c_uint) {
    do_emu10k1x_midi_interrupt(emu, &mut (*emu).midi, status);
}

unsafe extern "C" fn snd_emu10k1x_midi_cmd(emu: *mut emu10k1x, midi: *mut emu10k1x_midi, cmd: u8, ack: c_int) -> c_int {
    let mut timeout: c_int;
    let mut ok: c_int;

    spin_lock(&mut (*midi).input_lock);
    mpu401_write_data(emu, midi, 0x00);
    /* mpu401_clear_rx(emu, midi); */

    mpu401_write_cmd(emu, midi, cmd as c_int);
    if ack != 0 {
        ok = 0;
        timeout = 10000;
        while ok == 0 && {
            let old = timeout;
            timeout -= 1;
            old > 0
        } {
            if mpu401_input_avail(emu, midi) {
                if mpu401_read_data(emu, midi) == MPU401_ACK {
                    ok = 1;
                }
            }
        }
        if ok == 0 && mpu401_read_data(emu, midi) == MPU401_ACK {
            ok = 1;
        }
    } else {
        ok = 1;
    }
    spin_unlock(&mut (*midi).input_lock);

    if ok == 0 {
        dev_err((*emu).card.as_ref().unwrap().dev, c"midi_cmd: 0x%x failed at 0x%lx (status = 0x%x, data = 0x%x)!!!\n".as_ptr(), cmd as c_int, (*emu).port, mpu401_read_stat(emu, midi) as c_int, mpu401_read_data(emu, midi) as c_int);
        return 1;
    }
    0
}

unsafe extern "C" fn snd_emu10k1x_midi_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let emu: *mut emu10k1x;
    let midi: *mut emu10k1x_midi = (*(*substream).rmidi).private_data as *mut emu10k1x_midi;

    emu = (*midi).emu;
    if snd_BUG_ON(emu.is_null()) {
        return -ENXIO;
    }
    spin_lock(&mut (*midi).open_lock);
    (*midi).midi_mode |= EMU10K1X_MIDI_MODE_INPUT;
    (*midi).substream_input = substream;
    if ((*midi).midi_mode & EMU10K1X_MIDI_MODE_OUTPUT) != 0 {
        spin_unlock(&mut (*midi).open_lock);
        return 0;
    }
    spin_unlock(&mut (*midi).open_lock);
    if snd_emu10k1x_midi_cmd(emu, midi, MPU401_RESET as u8, 1) != 0 {
        return -EIO;
    }
    if snd_emu10k1x_midi_cmd(emu, midi, MPU401_ENTER_UART as u8, 1) != 0 {
        return -EIO;
    }
    0
}

unsafe extern "C" fn snd_emu10k1x_midi_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    let emu: *mut emu10k1x;
    let midi: *mut emu10k1x_midi = (*(*substream).rmidi).private_data as *mut emu10k1x_midi;

    emu = (*midi).emu;
    if snd_BUG_ON(emu.is_null()) {
        return -ENXIO;
    }
    spin_lock(&mut (*midi).open_lock);
    (*midi).midi_mode |= EMU10K1X_MIDI_MODE_OUTPUT;
    (*midi).substream_output = substream;
    if ((*midi).midi_mode & EMU10K1X_MIDI_MODE_INPUT) != 0 {
        spin_unlock(&mut (*midi).open_lock);
        return 0;
    }
    spin_unlock(&mut (*midi).open_lock);
    if snd_emu10k1x_midi_cmd(emu, midi, MPU401_RESET as u8, 1) != 0 {
        return -EIO;
    }
    if snd_emu10k1x_midi_cmd(emu, midi, MPU401_ENTER_UART as u8, 1) != 0 {
        return -EIO;
    }
    0
}

unsafe extern "C" fn snd_emu10k1x_midi_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let emu: *mut emu10k1x;
    let midi: *mut emu10k1x_midi = (*(*substream).rmidi).private_data as *mut emu10k1x_midi;

    emu = (*midi).emu;
    if snd_BUG_ON(emu.is_null()) {
        return -ENXIO;
    }
    spin_lock(&mut (*midi).open_lock);
    snd_emu10k1x_intr_disable(emu, (*midi).rx_enable as c_uint);
    (*midi).midi_mode &= !EMU10K1X_MIDI_MODE_INPUT;
    (*midi).substream_input = ptr::null_mut();
    if ((*midi).midi_mode & EMU10K1X_MIDI_MODE_OUTPUT) != 0 {
        spin_unlock(&mut (*midi).open_lock);
        return 0;
    }
    spin_unlock(&mut (*midi).open_lock);
    snd_emu10k1x_midi_cmd(emu, midi, MPU401_RESET as u8, 0)
}

unsafe extern "C" fn snd_emu10k1x_midi_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    let emu: *mut emu10k1x;
    let midi: *mut emu10k1x_midi = (*(*substream).rmidi).private_data as *mut emu10k1x_midi;

    emu = (*midi).emu;
    if snd_BUG_ON(emu.is_null()) {
        return -ENXIO;
    }
    spin_lock(&mut (*midi).open_lock);
    snd_emu10k1x_intr_disable(emu, (*midi).tx_enable as c_uint);
    (*midi).midi_mode &= !EMU10K1X_MIDI_MODE_OUTPUT;
    (*midi).substream_output = ptr::null_mut();
    if ((*midi).midi_mode & EMU10K1X_MIDI_MODE_INPUT) != 0 {
        spin_unlock(&mut (*midi).open_lock);
        return 0;
    }
    spin_unlock(&mut (*midi).open_lock);
    snd_emu10k1x_midi_cmd(emu, midi, MPU401_RESET as u8, 0)
}

unsafe extern "C" fn snd_emu10k1x_midi_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let emu: *mut emu10k1x;
    let midi: *mut emu10k1x_midi = (*(*substream).rmidi).private_data as *mut emu10k1x_midi;
    emu = (*midi).emu;
    if snd_BUG_ON(emu.is_null()) {
        return;
    }

    if up != 0 {
        snd_emu10k1x_intr_enable(emu, (*midi).rx_enable as c_uint);
    } else {
        snd_emu10k1x_intr_disable(emu, (*midi).rx_enable as c_uint);
    }
}

unsafe extern "C" fn snd_emu10k1x_midi_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    let emu: *mut emu10k1x;
    let midi: *mut emu10k1x_midi = (*(*substream).rmidi).private_data as *mut emu10k1x_midi;

    emu = (*midi).emu;
    if snd_BUG_ON(emu.is_null()) {
        return;
    }

    if up != 0 {
        let mut max: c_int = 4;
        let mut byte: u8 = 0;

        /* try to send some amount of bytes here before interrupts */
        spin_lock(&mut (*midi).output_lock);
        while max > 0 {
            if mpu401_output_ready(emu, midi) {
                if ((*midi).midi_mode & EMU10K1X_MIDI_MODE_OUTPUT) == 0 || snd_rawmidi_transmit(substream, &mut byte, 1) != 1 {
                    /* no more data */
                    spin_unlock(&mut (*midi).output_lock);
                    return;
                }
                mpu401_write_data(emu, midi, byte as c_int);
                max -= 1;
            } else {
                break;
            }
        }
        spin_unlock(&mut (*midi).output_lock);
        snd_emu10k1x_intr_enable(emu, (*midi).tx_enable as c_uint);
    } else {
        snd_emu10k1x_intr_disable(emu, (*midi).tx_enable as c_uint);
    }
}

/*

 */

static snd_emu10k1x_midi_output: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_emu10k1x_midi_output_open),
    close: Some(snd_emu10k1x_midi_output_close),
    trigger: Some(snd_emu10k1x_midi_output_trigger),
};

static snd_emu10k1x_midi_input: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_emu10k1x_midi_input_open),
    close: Some(snd_emu10k1x_midi_input_close),
    trigger: Some(snd_emu10k1x_midi_input_trigger),
};

unsafe extern "C" fn snd_emu10k1x_midi_free(rmidi: *mut snd_rawmidi) {
    let midi: *mut emu10k1x_midi = (*rmidi).private_data as *mut emu10k1x_midi;
    (*midi).interrupt = None;
    (*midi).rmidi = ptr::null_mut();
}

unsafe extern "C" fn emu10k1x_midi_init(emu: *mut emu10k1x, midi: *mut emu10k1x_midi, device: c_int, name: *mut c_char) -> c_int {
    let mut rmidi: *mut snd_rawmidi = ptr::null_mut();
    let mut err: c_int;

    err = snd_rawmidi_new((*emu).card, name, device, 1, 1, &mut rmidi);
    if err < 0 {
        return err;
    }
    (*midi).emu = emu;
    spin_lock_init(&mut (*midi).open_lock);
    spin_lock_init(&mut (*midi).input_lock);
    spin_lock_init(&mut (*midi).output_lock);
    strscpy((*rmidi).name.as_mut_ptr(), name);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT, &snd_emu10k1x_midi_output);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT, &snd_emu10k1x_midi_input);
    (*rmidi).info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX;
    (*rmidi).private_data = midi as *mut c_void;
    (*rmidi).private_free = Some(snd_emu10k1x_midi_free);
    (*midi).rmidi = rmidi;
    0
}

unsafe extern "C" fn snd_emu10k1x_midi(emu: *mut emu10k1x) -> c_int {
    let midi: *mut emu10k1x_midi = &mut (*emu).midi;
    let mut err: c_int;

    err = emu10k1x_midi_init(emu, midi, 0, c"EMU10K1X MPU-401 (UART)".as_ptr() as *mut c_char);
    if err < 0 {
        return err;
    }

    (*midi).tx_enable = INTE_MIDITXENABLE as c_int;
    (*midi).rx_enable = INTE_MIDIRXENABLE as c_int;
    (*midi).port = MUDATA;
    (*midi).ipr_tx = IPR_MIDITRANSBUFEMPTY as c_int;
    (*midi).ipr_rx = IPR_MIDIRECVBUFEMPTY as c_int;
    (*midi).interrupt = Some(snd_emu10k1x_midi_interrupt);
    0
}

unsafe extern "C" fn __snd_emu10k1x_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut emu10k1x;
    let mut err: c_int;

    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }

    err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], THIS_MODULE, size_of::<emu10k1x>(), &mut card);
    if err < 0 {
        return err;
    }
    chip = (*card).private_data as *mut emu10k1x;

    err = snd_emu10k1x_create(card, pci);
    if err < 0 {
        return err;
    }

    err = snd_emu10k1x_pcm(chip, 0);
    if err < 0 {
        return err;
    }
    err = snd_emu10k1x_pcm(chip, 1);
    if err < 0 {
        return err;
    }
    err = snd_emu10k1x_pcm(chip, 2);
    if err < 0 {
        return err;
    }

    err = snd_emu10k1x_ac97(chip);
    if err < 0 {
        return err;
    }

    err = snd_emu10k1x_mixer(chip);
    if err < 0 {
        return err;
    }

    err = snd_emu10k1x_midi(chip);
    if err < 0 {
        return err;
    }

    snd_emu10k1x_proc_init(chip);

    strscpy((*card).driver.as_mut_ptr(), c"EMU10K1X".as_ptr());
    strscpy((*card).shortname.as_mut_ptr(), c"Dell Sound Blaster Live!".as_ptr());
    sprintf((*card).longname.as_mut_ptr(), c"%s at 0x%lx irq %i".as_ptr(), (*card).shortname.as_ptr(), (*chip).port, (*chip).irq);

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    let _ = pci_id;
    0
}

unsafe extern "C" fn snd_emu10k1x_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_emu10k1x_probe(pci, pci_id))
}

// PCI IDs
/* static const struct pci_device_id snd_emu10k1x_ids[] = {
 *     { PCI_VDEVICE(CREATIVE, 0x0006) },      // Dell OEM version (EMU10K1)
 *     { }
 * };
 * MODULE_DEVICE_TABLE(pci, snd_emu10k1x_ids);
 */
static snd_emu10k1x_ids: [pci_device_id; 2] = [
    pci_device_id,
    pci_device_id,
];

// pci_driver definition
static mut emu10k1x_driver: pci_driver = pci_driver {
    name: unsafe { KBUILD_MODNAME },
    id_table: snd_emu10k1x_ids.as_ptr(),
    probe: Some(snd_emu10k1x_probe),
};

/* module_pci_driver(emu10k1x_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
