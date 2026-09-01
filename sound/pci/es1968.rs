// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for ESS Maestro 1/2/2E Sound Card (started 21.8.99)
 *  Copyright (c) by Matze Braun <MatzeBraun@gmx.de>.
 *                   Takashi Iwai <tiwai@suse.de>
 *
 *  Most of the driver code comes from Zach Brown(zab@redhat.com)
 *	Alan Cox OSS Driver
 *  Rewritted from card-es1938.c source.
 *
 *  TODO:
 *   Perhaps Synth
 *
 *  Notes from Zach Brown about the driver code
 *
 *  Hardware Description
 *
 *	A working Maestro setup contains the Maestro chip wired to a
 *	codec or 2.  In the Maestro we have the APUs, the ASSP, and the
 *	Wavecache.  The APUs can be though of as virtual audio routing
 *	channels.  They can take data from a number of sources and perform
 *	basic encodings of the data.  The wavecache is a storehouse for
 *	PCM data.  Typically it deals with PCI and interracts with the
 *	APUs.  The ASSP is a wacky DSP like device that ESS is loth
 *	to release docs on.  Thankfully it isn't required on the Maestro
 *	until you start doing insane things like FM emulation and surround
 *	encoding.  The codecs are almost always AC-97 compliant codecs,
 *	but it appears that early Maestros may have had PT101 (an ESS
 *	part?) wired to them.  The only real difference in the Maestro
 *	families is external goop like docking capability, memory for
 *	the ASSP, and initialization differences.
 *
 *  Driver Operation
 *
 *	We only drive the APU/Wavecache as typical DACs and drive the
 *	mixers in the codecs.  There are 64 APUs.  We assign 6 to each
 *	/dev/dsp? device.  2 channels for output, and 4 channels for
 *	input.
 *
 *	Each APU can do a number of things, but we only really use
 *	3 basic functions.  For playback we use them to convert PCM
 *	data fetched over PCI by the wavecahche into analog data that
 *	is handed to the codec.  One APU for mono, and a pair for stereo.
 *	When in stereo, the combination of smarts in the APU and Wavecache
 *	decide which wavecache gets the left or right channel.
 *
 *	For record we still use the old overly mono system.  For each in
 *	coming channel the data comes in from the codec, through a 'input'
 *	APU, through another rate converter APU, and then into memory via
 *	the wavecache and PCI.  If its stereo, we mash it back into LRLR in
 *	software.  The pass between the 2 APUs is supposedly what requires us
 *	to have a 512 byte buffer sitting around in wavecache/memory.
 *
 *	The wavecache makes our life even more fun.  First off, it can
 *	only address the first 28 bits of PCI address space, making it
 *	useless on quite a few architectures.  Secondly, its insane.
 *	It claims to fetch from 4 regions of PCI space, each 4 meg in length.
 *	But that doesn't really work.  You can only use 1 region.  So all our
 *	allocations have to be in 4meg of each other.  Booo.  Hiss.
 *	So we have a module parameter, dsps_order, that is the order of
 *	the number of dsps to provide.  All their buffer space is allocated
 *	on open time.  The sonicvibes OSS routines we inherited really want
 *	power of 2 buffers, so we have all those next to each other, then
 *	512 byte regions for the recording wavecaches.  This ends up
 *	wasting quite a bit of memory.  The only fixes I can see would be
 *	getting a kernel allocator that could work in zones, or figuring out
 *	just how to coerce the WP into doing what we want.
 *
 *	The indirection of the various registers means we have to spinlock
 *	nearly all register accesses.  We have the main register indirection
 *	like the wave cache, maestro registers, etc.  Then we have beasts
 *	like the APU interface that is indirect registers gotten at through
 *	the main maestro indirection.  Ouch.  We spinlock around the actual
 *	ports on a per card basis.  This means spinlock activity at each IO
 *	operation, but the only IO operation clusters are in non critical
 *	paths and it makes the code far easier to follow.  Interrupts are
 *	blocked while holding the locks because the int handler has to
 *	get at some of them :(.  The mixer interface doesn't, however.
 *	We also have an OSS state lock that is thrown around in a few
 *	places.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, improper_ctypes, unused_assignments, unused_mut, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = ::core::primitive::u8;
type u16 = ::core::primitive::u16;
type u32 = ::core::primitive::u32;
type bool_ = bool;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_uint;
type ktime_t = i64;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];

const CARD_NAME: &[u8] = b"ESS Maestro1/2\0";
const DRIVER_NAME: &[u8] = b"ES1968\0";

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 1-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP; /* Enable this card */
static mut total_bufsize: [c_int; SNDRV_CARDS] = [1024; SNDRV_CARDS];
static mut pcm_substreams_p: [c_int; SNDRV_CARDS] = [4; SNDRV_CARDS];
static mut pcm_substreams_c: [c_int; SNDRV_CARDS] = [1; SNDRV_CARDS];
static mut clock: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
static mut use_pm: [c_int; SNDRV_CARDS] = [2; SNDRV_CARDS];
static mut enable_mpu: [c_int; SNDRV_CARDS] = [2; SNDRV_CARDS];
/* #ifdef SUPPORT_JOYSTICK */
static mut joystick: [bool; SNDRV_CARDS] = [false; SNDRV_CARDS];
/* #endif */
static mut radio_nr: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];

const NR_APUS: usize = 64;
const NR_APU_REGS: usize = 16;

/* NEC Versas ? */
const NEC_VERSA_SUBID1: u32 = 0x80581033;
const NEC_VERSA_SUBID2: u32 = 0x803c1033;

/* Mode Flags */
const ESS_FMT_STEREO: u8 = 0x01;
const ESS_FMT_16BIT: u8 = 0x02;

const DAC_RUNNING: c_int = 1;
const ADC_RUNNING: c_int = 2;

/* Values for the ESM_LEGACY_AUDIO_CONTROL */
const ESS_DISABLE_AUDIO: u16 = 0x8000;
const ESS_ENABLE_SERIAL_IRQ: u16 = 0x4000;
const IO_ADRESS_ALIAS: u16 = 0x0020;
const MPU401_IRQ_ENABLE: u16 = 0x0010;
const MPU401_IO_ENABLE: u16 = 0x0008;
const GAME_IO_ENABLE: u16 = 0x0004;
const FM_IO_ENABLE: u16 = 0x0002;
const SB_IO_ENABLE: u16 = 0x0001;

/* Values for the ESM_CONFIG_A */
const PIC_SNOOP1: u16 = 0x4000;
const PIC_SNOOP2: u16 = 0x2000;
const SAFEGUARD: u16 = 0x0800;
const DMA_CLEAR: u16 = 0x0700;
const DMA_DDMA: u16 = 0x0000;
const DMA_TDMA: u16 = 0x0100;
const DMA_PCPCI: u16 = 0x0200;
const POST_WRITE: u16 = 0x0080;
const PCI_TIMING: u16 = 0x0040;
const SWAP_LR: u16 = 0x0020;
const SUBTR_DECODE: u16 = 0x0002;

/* Values for the ESM_CONFIG_B */
const SPDIF_CONFB: u16 = 0x0100;
const HWV_CONFB: u16 = 0x0080;
const DEBOUNCE: u16 = 0x0040;
const GPIO_CONFB: u16 = 0x0020;
const CHI_CONFB: u16 = 0x0010;
const IDMA_CONFB: u16 = 0x0008; /*undoc */
const MIDI_FIX: u16 = 0x0004; /*undoc */
const IRQ_TO_ISA: u16 = 0x0001; /*undoc */

/* Values for Ring Bus Control B */
const RINGB_2CODEC_ID_MASK: u16 = 0x0003;
const RINGB_DIS_VALIDATION: u16 = 0x0008;
const RINGB_EN_SPDIF: u32 = 0x0010;
const RINGB_EN_2CODEC: u16 = 0x0020;
const RINGB_SING_BIT_DUAL: u16 = 0x0040;

/* ****Port Addresses**** */
const ESM_INDEX: c_ulong = 0x02;
const ESM_DATA: c_ulong = 0x00;
const ESM_AC97_INDEX: c_ulong = 0x30;
const ESM_AC97_DATA: c_ulong = 0x32;
const ESM_RING_BUS_DEST: c_ulong = 0x34;
const ESM_RING_BUS_CONTR_A: c_ulong = 0x36;
const ESM_RING_BUS_CONTR_B: c_ulong = 0x38;
const ESM_RING_BUS_SDO: c_ulong = 0x3A;
const WC_INDEX: c_ulong = 0x10;
const WC_DATA: c_ulong = 0x12;
const WC_CONTROL: c_ulong = 0x14;
const ASSP_INDEX: c_ulong = 0x80;
const ASSP_MEMORY: c_ulong = 0x82;
const ASSP_DATA: c_ulong = 0x84;
const ASSP_CONTROL_A: c_ulong = 0xA2;
const ASSP_CONTROL_B: c_ulong = 0xA4;
const ASSP_CONTROL_C: c_ulong = 0xA6;
const ASSP_HOSTW_INDEX: c_ulong = 0xA8;
const ASSP_HOSTW_DATA: c_ulong = 0xAA;
const ASSP_HOSTW_IRQ: c_ulong = 0xAC;
const ESM_MPU401_PORT: c_ulong = 0x98;
const ESM_PORT_HOST_IRQ: c_ulong = 0x18;

const IDR0_DATA_PORT: u16 = 0x00;
const IDR1_CRAM_POINTER: u16 = 0x01;
const IDR2_CRAM_DATA: u16 = 0x02;
const IDR3_WAVE_DATA: u16 = 0x03;
const IDR4_WAVE_PTR_LOW: u16 = 0x04;
const IDR5_WAVE_PTR_HI: u16 = 0x05;
const IDR6_TIMER_CTRL: u16 = 0x06;
const IDR7_WAVE_ROMRAM: u16 = 0x07;

const WRITEABLE_MAP: u32 = 0xEFFFFF;
const READABLE_MAP: u32 = 0x64003F;

/* PCI Register */
const ESM_LEGACY_AUDIO_CONTROL: c_int = 0x40;
const ESM_ACPI_COMMAND: c_int = 0x54;
const ESM_CONFIG_A: c_int = 0x50;
const ESM_CONFIG_B: c_int = 0x52;
const ESM_DDMA: c_int = 0x60;

/* Bob Bits */
const ESM_BOB_ENABLE: u16 = 0x0001;
const ESM_BOB_START: u16 = 0x0001;

/* Host IRQ Control Bits */
const ESM_RESET_MAESTRO: u16 = 0x8000;
const ESM_RESET_DIRECTSOUND: u16 = 0x4000;
const ESM_HIRQ_ClkRun: u16 = 0x0100;
const ESM_HIRQ_HW_VOLUME: u16 = 0x0040;
const ESM_HIRQ_HARPO: u16 = 0x0030; /* What's that? */
const ESM_HIRQ_ASSP: u16 = 0x0010;
const ESM_HIRQ_DSIE: u16 = 0x0004;
const ESM_HIRQ_MPU401: u16 = 0x0002;
const ESM_HIRQ_SB: u16 = 0x0001;

/* Host IRQ Status Bits */
const ESM_MPU401_IRQ: u32 = 0x02;
const ESM_SB_IRQ: u32 = 0x01;
const ESM_SOUND_IRQ: u32 = 0x04;
const ESM_ASSP_IRQ: u32 = 0x10;
const ESM_HWVOL_IRQ: u32 = 0x40;

const ESS_SYSCLK: c_int = 50000000;
const ESM_BOB_FREQ: c_int = 200;
const ESM_BOB_FREQ_MAX: c_int = 800;
const ESM_FREQ_ESM1: c_long = 49152000 / 1024; /* default rate 48000 */
const ESM_FREQ_ESM2: c_long = 50000000 / 1024;
type c_long = isize;

/* APU Modes: reg 0x00, bit 4-7 */
const ESM_APU_MODE_SHIFT: c_int = 4;
const ESM_APU_MODE_MASK: c_int = 0xf << 4;
const ESM_APU_OFF: c_int = 0x00;
const ESM_APU_16BITLINEAR: c_int = 0x01; /* 16-Bit Linear Sample Player */
const ESM_APU_16BITSTEREO: c_int = 0x02; /* 16-Bit Stereo Sample Player */
const ESM_APU_8BITLINEAR: c_int = 0x03; /* 8-Bit Linear Sample Player */
const ESM_APU_8BITSTEREO: c_int = 0x04; /* 8-Bit Stereo Sample Player */
const ESM_APU_8BITDIFF: c_int = 0x05; /* 8-Bit Differential Sample Playrer */
const ESM_APU_DIGITALDELAY: c_int = 0x06; /* Digital Delay Line */
const ESM_APU_DUALTAP: c_int = 0x07; /* Dual Tap Reader */
const ESM_APU_CORRELATOR: c_int = 0x08; /* Correlator */
const ESM_APU_INPUTMIXER: c_int = 0x09; /* Input Mixer */
const ESM_APU_WAVETABLE: c_int = 0x0A; /* Wave Table Mode */
const ESM_APU_SRCONVERTOR: c_int = 0x0B; /* Sample Rate Convertor */
const ESM_APU_16BITPINGPONG: c_int = 0x0C; /* 16-Bit Ping-Pong Sample Player */
const ESM_APU_RESERVED1: c_int = 0x0D; /* Reserved 1 */
const ESM_APU_RESERVED2: c_int = 0x0E; /* Reserved 2 */
const ESM_APU_RESERVED3: c_int = 0x0F; /* Reserved 3 */

const ESM_APU_FILTER_Q_SHIFT: c_int = 0;
const ESM_APU_FILTER_Q_MASK: c_int = 3 << 0;
/* APU Filtey Q Control */
const ESM_APU_FILTER_LESSQ: c_int = 0x00;
const ESM_APU_FILTER_MOREQ: c_int = 0x03;
const ESM_APU_FILTER_TYPE_SHIFT: c_int = 2;
const ESM_APU_FILTER_TYPE_MASK: c_int = 3 << 2;
const ESM_APU_ENV_TYPE_SHIFT: c_int = 8;
const ESM_APU_ENV_TYPE_MASK: c_int = 3 << 8;
const ESM_APU_ENV_STATE_SHIFT: c_int = 10;
const ESM_APU_ENV_STATE_MASK: c_int = 3 << 10;
const ESM_APU_END_CURVE: c_int = 1 << 12;
const ESM_APU_INT_ON_LOOP: c_int = 1 << 13;
const ESM_APU_DMA_ENABLE: c_int = 1 << 14;

const ESM_APU_SUBMIX_GROUP_SHIRT: c_int = 0;
const ESM_APU_SUBMIX_GROUP_MASK: c_int = 7 << 0;
const ESM_APU_SUBMIX_MODE: c_int = 1 << 3;
const ESM_APU_6dB: c_int = 1 << 4;
const ESM_APU_DUAL_EFFECT: c_int = 1 << 5;
const ESM_APU_EFFECT_CHANNELS_SHIFT: c_int = 6;
const ESM_APU_EFFECT_CHANNELS_MASK: c_int = 3 << 6;
const ESM_APU_STEP_SIZE_MASK: c_int = 0x0fff;
const ESM_APU_PHASE_SHIFT: c_int = 0;
const ESM_APU_PHASE_MASK: c_int = 0xff << 0;
const ESM_APU_WAVE64K_PAGE_SHIFT: c_int = 8; /* most 8bit of wave start offset */
const ESM_APU_WAVE64K_PAGE_MASK: c_int = 0xff << 8;
const ESM_APU_EFFECT_GAIN_SHIFT: c_int = 0;
const ESM_APU_EFFECT_GAIN_MASK: c_int = 0xff << 0;
const ESM_APU_TREMOLO_DEPTH_SHIFT: c_int = 8;
const ESM_APU_TREMOLO_DEPTH_MASK: c_int = 0xf << 8;
const ESM_APU_TREMOLO_RATE_SHIFT: c_int = 12;
const ESM_APU_TREMOLO_RATE_MASK: c_int = 0xf << 12;
const ESM_APU_AMPLITUDE_NOW_SHIFT: c_int = 8;
const ESM_APU_AMPLITUDE_NOW_MASK: c_int = 0xff << 8;
const ESM_APU_POLAR_PAN_SHIFT: c_int = 0;
const ESM_APU_POLAR_PAN_MASK: c_int = 0x3f << 0;
/* Polar Pan Control */
const ESM_APU_PAN_CENTER_CIRCLE: c_int = 0x00;
const ESM_APU_PAN_MIDDLE_RADIUS: c_int = 0x01;
const ESM_APU_PAN_OUTSIDE_RADIUS: c_int = 0x02;
const ESM_APU_FILTER_TUNING_SHIFT: c_int = 8;
const ESM_APU_FILTER_TUNING_MASK: c_int = 0xff << 8;
const ESM_APU_DATA_SRC_A_SHIFT: c_int = 0;
const ESM_APU_DATA_SRC_A_MASK: c_int = 0x7f << 0;
const ESM_APU_INV_POL_A: c_int = 1 << 7;
const ESM_APU_DATA_SRC_B_SHIFT: c_int = 8;
const ESM_APU_DATA_SRC_B_MASK: c_int = 0x7f << 8;
const ESM_APU_INV_POL_B: c_int = 1 << 15;
const ESM_APU_VIBRATO_RATE_SHIFT: c_int = 0;
const ESM_APU_VIBRATO_RATE_MASK: c_int = 0xf << 0;
const ESM_APU_VIBRATO_DEPTH_SHIFT: c_int = 4;
const ESM_APU_VIBRATO_DEPTH_MASK: c_int = 0xf << 4;
const ESM_APU_VIBRATO_PHASE_SHIFT: c_int = 8;
const ESM_APU_VIBRATO_PHASE_MASK: c_int = 0xff << 8;
const ESM_APU_RADIUS_SELECT: c_int = 1 << 6;
/* APU Filter Control */
const ESM_APU_FILTER_2POLE_LOPASS: c_int = 0x00;
const ESM_APU_FILTER_2POLE_BANDPASS: c_int = 0x01;
const ESM_APU_FILTER_2POLE_HIPASS: c_int = 0x02;
const ESM_APU_FILTER_1POLE_LOPASS: c_int = 0x03;
const ESM_APU_FILTER_1POLE_HIPASS: c_int = 0x04;
const ESM_APU_FILTER_OFF: c_int = 0x05;
/* APU ATFP Type */
const ESM_APU_ATFP_AMPLITUDE: c_int = 0x00;
const ESM_APU_ATFP_TREMELO: c_int = 0x01;
const ESM_APU_ATFP_FILTER: c_int = 0x02;
const ESM_APU_ATFP_PAN: c_int = 0x03;
/* APU ATFP Flags */
const ESM_APU_ATFP_FLG_OFF: c_int = 0x00;
const ESM_APU_ATFP_FLG_WAIT: c_int = 0x01;
const ESM_APU_ATFP_FLG_DONE: c_int = 0x02;
const ESM_APU_ATFP_FLG_INPROCESS: c_int = 0x03;

/* capture mixing buffer size */
const ESM_MEM_ALIGN: c_int = 0x1000;
const ESM_MIXBUF_SIZE: c_int = 0x400;
const ESM_MODE_PLAY: c_int = 0;
const ESM_MODE_CAPTURE: c_int = 1;

/* APU use in the driver */
#[repr(C)]
enum snd_enum_apu_type {
    ESM_APU_PCM_PLAY,
    ESM_APU_PCM_CAPTURE,
    ESM_APU_PCM_RATECONV,
    ESM_APU_FREE,
}

/* chip type */
const TYPE_MAESTRO: c_int = 0;
const TYPE_MAESTRO2: c_int = 1;
const TYPE_MAESTRO2E: c_int = 2;

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}
#[repr(C)]
pub struct snd_dma_buffer {
    area: *mut u8,
    addr: u32,
    bytes: c_int,
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_dev {
    dev: device,
    irq: c_int,
    vendor: u16,
    device: u16,
    subsystem_vendor: u16,
}
#[repr(C)]
pub struct snd_card {
    dev: *mut device,
    private_data: *mut c_void,
    sync_irq: c_int,
    private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    driver: [c_char; 16],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}
#[repr(C)]
pub struct snd_pcm {
    private_data: *mut c_void,
    private_free: Option<unsafe extern "C" fn(*mut snd_pcm)>,
    info_flags: c_uint,
    name: [c_char; 80],
}
#[repr(C)]
pub struct snd_pcm_runtime {
    private_data: *mut c_void,
    hw: snd_pcm_hardware,
    rate: c_uint,
    channels: c_uint,
    format: c_int,
    dma_bytes: c_int,
}
#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
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
}
#[repr(C)]
pub struct snd_rawmidi {
    private_data: *mut c_void,
}
#[repr(C)]
pub struct spinlock_t {
    _private: c_int,
}
#[repr(C)]
pub struct mutex {
    _private: c_int,
}
#[repr(C)]
pub struct work_struct {
    _private: c_int,
}
#[repr(C)]
pub struct snd_kcontrol {
    id: snd_ctl_elem_id,
}
#[repr(C)]
pub struct snd_ctl_elem_id {
    _private: c_int,
}
#[repr(C)]
pub struct input_dev {
    name: *mut c_char,
    phys: *mut c_char,
    id: input_id,
    dev: input_dev_dev,
    evbit: [c_ulong; 1],
    keybit: [c_ulong; 8],
}
#[repr(C)]
pub struct input_id {
    bustype: u16,
    vendor: u16,
    product: u16,
}
#[repr(C)]
pub struct input_dev_dev {
    parent: *mut device,
}
#[repr(C)]
pub struct gameport {
    io: c_ulong,
}
#[repr(C)]
pub struct resource {
    _private: c_int,
}
#[repr(C)]
pub struct v4l2_device {
    _private: c_int,
}
#[repr(C)]
pub struct snd_tea575x {
    v4l2_dev: *mut v4l2_device,
    private_data: *mut c_void,
    radio_nr: c_int,
    ops: *const snd_tea575x_ops,
    bus_info: [c_char; 32],
    card: [c_char; 32],
}
#[repr(C)]
pub struct snd_tea575x_ops {
    set_pins: Option<unsafe extern "C" fn(*mut snd_tea575x, u8)>,
    get_pins: Option<unsafe extern "C" fn(*mut snd_tea575x) -> u8>,
    set_direction: Option<unsafe extern "C" fn(*mut snd_tea575x, bool)>,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: c_int,
}
#[repr(C)]
pub struct snd_pcm_hardware {
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
pub struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}
#[repr(C)]
pub struct snd_ac97_bus_ops {
    write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>,
    read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16>,
}
#[repr(C)]
pub struct pci_device_id {
    vendor: u32,
    device: u32,
    subvendor: u32,
    subdevice: u32,
    class: u32,
    class_mask: u32,
    driver_data: c_ulong,
}
#[repr(C)]
pub struct pci_driver_inner {
    pm: *const c_void,
}
#[repr(C)]
pub struct pci_driver {
    name: *const c_char,
    id_table: *const pci_device_id,
    probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    driver: pci_driver_inner,
}

/* DMA Hack! */
#[repr(C)]
pub struct esm_memory {
    buf: snd_dma_buffer,
    empty: c_int, /* status */
    list: list_head,
}

/* Playback Channel */
#[repr(C)]
pub struct esschan {
    running: c_int,
    apu: [u8; 4],
    apu_mode: [u8; 4],
    /* playback/capture pcm buffer */
    memory: *mut esm_memory,
    /* capture mixer buffer */
    mixbuf: *mut esm_memory,
    hwptr: c_uint, /* current hw pointer in bytes */
    count: c_uint, /* sample counter in bytes */
    dma_size: c_uint, /* total buffer size in bytes */
    frag_size: c_uint, /* period size in bytes */
    wav_shift: c_uint,
    base: [u16; 4], /* offset for ptr */
    /* stereo/16bit flag */
    fmt: u8,
    mode: c_int, /* playback / capture */
    bob_freq: c_int, /* required timer frequency */
    substream: *mut snd_pcm_substream,
    /* linked list */
    list: list_head,
    wc_map: [u16; 4],
}

#[repr(C)]
pub struct es1968 {
    /* Module Config */
    total_bufsize: c_int, /* in bytes */
    playback_streams: c_int,
    capture_streams: c_int,
    clock: c_uint, /* clock */
    /* for clock measurement */
    in_measurement: c_uint,
    measure_apu: c_uint,
    measure_lastpos: c_uint,
    measure_count: c_uint,
    /* buffer */
    dma: snd_dma_buffer,
    /* Resources... */
    irq: c_int,
    io_port: c_ulong,
    type_: c_int,
    pci: *mut pci_dev,
    card: *mut snd_card,
    pcm: *mut snd_pcm,
    do_pm: c_int, /* power-management enabled */
    /* DMA memory block */
    buf_list: list_head,
    /* ALSA Stuff */
    ac97: *mut snd_ac97,
    rmidi: *mut snd_rawmidi,
    reg_lock: spinlock_t,
    in_suspend: c_uint,
    /* Maestro Stuff */
    maestro_map: [u16; 32],
    bobclient: c_int, /* active timer instancs */
    bob_freq: c_int, /* timer frequency */
    memory_mutex: mutex, /* memory lock */
    /* APU states */
    apu: [u8; NR_APUS],
    /* active substreams */
    substream_list: list_head,
    substream_lock: spinlock_t,
    apu_map: [[u16; NR_APU_REGS]; NR_APUS],
    /* #ifdef SUPPORT_JOYSTICK */
    gameport: *mut gameport,
    /* #endif */
    /* #ifdef CONFIG_SND_ES1968_INPUT */
    input_dev: *mut input_dev,
    phys: [c_char; 64], /* physical device path */
    /* #else */
    master_switch: *mut snd_kcontrol, /* for h/w volume control */
    master_volume: *mut snd_kcontrol,
    /* #endif */
    hwvol_work: work_struct,
    /* #ifdef CONFIG_SND_ES1968_RADIO */
    v4l2_dev: v4l2_device,
    tea: snd_tea575x,
    tea575x_tuner: c_uint,
    /* #endif */
}

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ENOSYS: c_int = 38;
const ENOENT: c_int = 2;
const ENXIO: c_int = 6;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 2;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 3;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 4;
const SNDRV_PCM_INFO_NONINTERLEAVED: c_uint = 1 << 5;
const SNDRV_PCM_FMTBIT_U8: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 1;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 0;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 1;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int = 0;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1;
const AC97_MASTER: u16 = 0x02;
const PCI_SUBSYSTEM_VENDOR_ID: c_int = 0x2c;
const PCI_CLASS_MULTIMEDIA_AUDIO: u32 = 0x0401;
const IRQF_SHARED: c_uint = 0x80;
const MPU401_HW_MPU401: c_int = 0;
const MPU401_INFO_INTEGRATED: c_uint = 1;
const MPU401_INFO_IRQ_HOOK: c_uint = 2;
const BUS_PCI: u16 = 0x01;
const EV_KEY: c_int = 0x01;
const KEY_MUTE: c_int = 113;
const KEY_VOLUMEDOWN: c_int = 114;
const KEY_VOLUMEUP: c_int = 115;
const TEA575X_DATA: u8 = 1;
const TEA575X_CLK: u8 = 2;
const TEA575X_WREN: u8 = 4;
const TEA575X_MOST: u8 = 8;
const KBUILD_MODNAME: *const c_char = b"snd_es1968\0".as_ptr() as *const c_char;
const THIS_MODULE: *mut c_void = ptr::null_mut();

extern "C" {
    fn outb(value: u8, port: c_ulong);
    fn outw(value: u16, port: c_ulong);
    fn outl(value: u32, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn inw(port: c_ulong) -> u16;
    fn inl(port: c_ulong) -> u32;
    fn cond_resched();
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_BUG_ON(cond: bool) -> bool;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memset(ptr: *mut c_void, value: c_int, n: usize) -> *mut c_void;
    fn snd_dma_free_pages(buf: *mut snd_dma_buffer);
    fn snd_dma_alloc_pages_fallback(t: c_int, dev: *mut device, size: c_int, buf: *mut snd_dma_buffer) -> c_int;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut es1968;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn snd_pcm_set_runtime_buffer(substream: *mut snd_pcm_substream, buf: *mut snd_dma_buffer);
    fn params_buffer_bytes(hw_params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_hw_constraint_pow2(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback: c_int, capture: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_ac97_read(ac97: *mut snd_ac97, reg: u16) -> c_int;
    fn snd_ac97_update(ac97: *mut snd_ac97, reg: u16, value: c_int) -> c_int;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn input_report_key(dev: *mut input_dev, code: c_int, value: c_int);
    fn input_sync(dev: *mut input_dev);
    fn schedule_work(work: *mut work_struct) -> bool;
    fn snd_mpu401_uart_interrupt(irq: c_int, private_data: *mut c_void);
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, rbus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn snd_ctl_find_id_mixer(card: *mut snd_card, name: *const c_char) -> *mut snd_kcontrol;
    fn pci_read_config_word(pci: *mut pci_dev, where_: c_int, val: *mut u16) -> c_int;
    fn pci_read_config_dword(pci: *mut pci_dev, where_: c_int, val: *mut u32) -> c_int;
    fn pci_write_config_word(pci: *mut pci_dev, where_: c_int, val: u16) -> c_int;
    fn udelay(usecs: c_uint);
    fn msleep(msecs: c_uint);
    fn ktime_get() -> ktime_t;
    fn ktime_sub(lhs: ktime_t, rhs: ktime_t) -> ktime_t;
    fn ktime_to_us(kt: ktime_t) -> c_uint;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn mutex_init(mutex: *mut mutex);
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn snd_power_change_state(card: *mut snd_card, state: c_int) -> c_int;
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_request_region(dev: *mut device, start: c_ulong, n: c_ulong, name: *const c_char) -> *mut resource;
    fn gameport_allocate_port() -> *mut gameport;
    fn gameport_set_name(gp: *mut gameport, name: *const c_char);
    fn gameport_set_phys(gp: *mut gameport, fmt: *const c_char, ...);
    fn gameport_set_dev_parent(gp: *mut gameport, dev: *mut device);
    fn gameport_register_port(gp: *mut gameport);
    fn gameport_unregister_port(gp: *mut gameport);
    fn pci_name(pci: *mut pci_dev) -> *const c_char;
    fn devm_input_allocate_device(dev: *mut device) -> *mut input_dev;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn __set_bit(nr: c_int, addr: *mut c_ulong);
    fn input_register_device(dev: *mut input_dev) -> c_int;
    fn snd_tea575x_exit(tea: *mut snd_tea575x);
    fn v4l2_device_unregister(v4l2_dev: *mut v4l2_device);
    fn v4l2_device_register(dev: *mut device, v4l2_dev: *mut v4l2_device) -> c_int;
    fn snd_tea575x_init(tea: *mut snd_tea575x, module: *mut c_void) -> c_int;
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn pci_set_master(pci: *mut pci_dev);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_mpu401_uart_new(card: *mut snd_card, device: c_int, hardware: c_int, port: c_ulong, info_flags: c_uint, irq: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, ret: c_int) -> c_int;
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn list_add(new_: *mut list_head, head: *mut list_head) {
    (*new_).next = (*head).next;
    (*new_).prev = head;
    (*(*head).next).prev = new_;
    (*head).next = new_;
}

unsafe fn list_del(entry: *mut list_head) {
    (*(*entry).next).prev = (*entry).prev;
    (*(*entry).prev).next = (*entry).next;
}

unsafe fn list_entry_esm_memory(ptr_: *mut list_head) -> *mut esm_memory {
    (ptr_ as *mut u8).offset(-(core::mem::offset_of!(esm_memory, list) as isize)) as *mut esm_memory
}

unsafe fn list_entry_esschan(ptr_: *mut list_head) -> *mut esschan {
    (ptr_ as *mut u8).offset(-(core::mem::offset_of!(esschan, list) as isize)) as *mut esschan
}

fn ALIGN(x: c_int, a: c_int) -> c_int {
    (x + a - 1) & !(a - 1)
}

fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

fn DMA_BIT_MASK(n: u32) -> u64 {
    (1u64 << n) - 1
}

unsafe fn kmalloc_obj<T>() -> *mut T {
    kmalloc(size_of::<T>(), 0) as *mut T
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(size_of::<T>(), 0) as *mut T
}

unsafe extern "C" fn snd_es1968_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;

static snd_es1968_ids: [pci_device_id; 4] = [
    pci_device_id {
        /* Maestro 1 */
        vendor: 0x1285,
        device: 0x0100,
        subvendor: 0,
        subdevice: 0,
        class: PCI_CLASS_MULTIMEDIA_AUDIO << 8,
        class_mask: 0xffff00,
        driver_data: TYPE_MAESTRO as c_ulong,
    },
    pci_device_id {
        /* Maestro 2 */
        vendor: 0x125d,
        device: 0x1968,
        subvendor: 0,
        subdevice: 0,
        class: PCI_CLASS_MULTIMEDIA_AUDIO << 8,
        class_mask: 0xffff00,
        driver_data: TYPE_MAESTRO2 as c_ulong,
    },
    pci_device_id {
        /* Maestro 2E */
        vendor: 0x125d,
        device: 0x1978,
        subvendor: 0,
        subdevice: 0,
        class: PCI_CLASS_MULTIMEDIA_AUDIO << 8,
        class_mask: 0xffff00,
        driver_data: TYPE_MAESTRO2E as c_ulong,
    },
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];

/* *********************
   * Low Level Funcs!  *
   *********************/

/* no spinlock */
unsafe fn __maestro_write(chip: *mut es1968, reg: u16, data: u16) {
    outw(reg, (*chip).io_port + ESM_INDEX);
    outw(data, (*chip).io_port + ESM_DATA);
    (*chip).maestro_map[reg as usize] = data;
}

unsafe fn maestro_write(chip: *mut es1968, reg: u16, data: u16) {
    spin_lock(&mut (*chip).reg_lock);
    __maestro_write(chip, reg, data);
    spin_unlock(&mut (*chip).reg_lock);
}

/* no spinlock */
unsafe fn __maestro_read(chip: *mut es1968, reg: u16) -> u16 {
    if (READABLE_MAP & (1u32 << reg)) != 0 {
        outw(reg, (*chip).io_port + ESM_INDEX);
        (*chip).maestro_map[reg as usize] = inw((*chip).io_port + ESM_DATA);
    }
    (*chip).maestro_map[reg as usize]
}

unsafe fn maestro_read(chip: *mut es1968, reg: u16) -> u16 {
    let ret;
    spin_lock(&mut (*chip).reg_lock);
    ret = __maestro_read(chip, reg);
    spin_unlock(&mut (*chip).reg_lock);
    ret
}

/* Wait for the codec bus to be free */
unsafe fn snd_es1968_ac97_wait(chip: *mut es1968) -> c_int {
    let mut timeout: c_int = 100000;
    while {
        let old = timeout;
        timeout -= 1;
        old > 0
    } {
        if (inb((*chip).io_port + ESM_AC97_INDEX) & 1) == 0 {
            return 0;
        }
        cond_resched();
    }
    dev_dbg((*(*chip).card).dev, b"ac97 timeout\n\0".as_ptr() as *const c_char);
    1 /* timeout */
}

unsafe fn snd_es1968_ac97_wait_poll(chip: *mut es1968) -> c_int {
    let mut timeout: c_int = 100000;
    while {
        let old = timeout;
        timeout -= 1;
        old > 0
    } {
        if (inb((*chip).io_port + ESM_AC97_INDEX) & 1) == 0 {
            return 0;
        }
    }
    dev_dbg((*(*chip).card).dev, b"ac97 timeout\n\0".as_ptr() as *const c_char);
    1 /* timeout */
}

unsafe extern "C" fn snd_es1968_ac97_write(ac97: *mut snd_ac97, reg: u16, val: u16) {
    let chip = (*ac97).private_data as *mut es1968;
    snd_es1968_ac97_wait(chip);
    /* Write the bus */
    outw(val, (*chip).io_port + ESM_AC97_DATA);
    /*msleep(1);*/
    outb(reg as u8, (*chip).io_port + ESM_AC97_INDEX);
    /*msleep(1);*/
}

unsafe extern "C" fn snd_es1968_ac97_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    let mut data: u16 = 0;
    let chip = (*ac97).private_data as *mut es1968;
    snd_es1968_ac97_wait(chip);
    outb((reg | 0x80) as u8, (*chip).io_port + ESM_AC97_INDEX);
    /*msleep(1);*/
    if snd_es1968_ac97_wait_poll(chip) == 0 {
        data = inw((*chip).io_port + ESM_AC97_DATA);
        /*msleep(1);*/
    }
    data
}

/* no spinlock */
unsafe fn apu_index_set(chip: *mut es1968, index: u16) {
    let mut i: c_int = 0;
    __maestro_write(chip, IDR1_CRAM_POINTER, index);
    while i < 1000 {
        if __maestro_read(chip, IDR1_CRAM_POINTER) == index {
            return;
        }
        i += 1;
    }
    dev_dbg((*(*chip).card).dev, b"APU register select failed. (Timeout)\n\0".as_ptr() as *const c_char);
}

/* no spinlock */
unsafe fn apu_data_set(chip: *mut es1968, data: u16) {
    let mut i: c_int = 0;
    while i < 1000 {
        if __maestro_read(chip, IDR0_DATA_PORT) == data {
            return;
        }
        __maestro_write(chip, IDR0_DATA_PORT, data);
        i += 1;
    }
    dev_dbg((*(*chip).card).dev, b"APU register set probably failed (Timeout)!\n\0".as_ptr() as *const c_char);
}

/* no spinlock */
unsafe fn __apu_set_register(chip: *mut es1968, channel: u16, mut reg: u8, data: u16) {
    if snd_BUG_ON(channel as usize >= NR_APUS) {
        return;
    }
    (*chip).apu_map[channel as usize][reg as usize] = data;
    reg |= (channel << 4) as u8;
    apu_index_set(chip, reg as u16);
    apu_data_set(chip, data);
}

unsafe fn apu_set_register(chip: *mut es1968, channel: c_int, reg: c_int, data: u32) {
    spin_lock(&mut (*chip).reg_lock);
    __apu_set_register(chip, channel as u16, reg as u8, data as u16);
    spin_unlock(&mut (*chip).reg_lock);
}

unsafe fn __apu_get_register(chip: *mut es1968, channel: u16, mut reg: u8) -> u16 {
    if snd_BUG_ON(channel as usize >= NR_APUS) {
        return 0;
    }
    reg |= (channel << 4) as u8;
    apu_index_set(chip, reg as u16);
    __maestro_read(chip, IDR0_DATA_PORT)
}

unsafe fn apu_get_register(chip: *mut es1968, channel: u8, reg: c_int) -> u16 {
    let ret;
    spin_lock(&mut (*chip).reg_lock);
    ret = __apu_get_register(chip, channel as u16, reg as u8);
    spin_unlock(&mut (*chip).reg_lock);
    ret
}

/* #if 0 ASSP is not supported: assp_set_register() and assp_get_register() intentionally disabled. */

unsafe fn wave_set_register(chip: *mut es1968, reg: u16, value: u32) {
    spin_lock(&mut (*chip).reg_lock);
    outw(reg, (*chip).io_port + WC_INDEX);
    outw(value as u16, (*chip).io_port + WC_DATA);
    spin_unlock(&mut (*chip).reg_lock);
}

unsafe fn wave_get_register(chip: *mut es1968, reg: u16) -> u16 {
    let ret;
    spin_lock(&mut (*chip).reg_lock);
    outw(reg, (*chip).io_port + WC_INDEX);
    ret = inw((*chip).io_port + WC_DATA);
    spin_unlock(&mut (*chip).reg_lock);
    ret
}

/* *******************
   * Bob the Timer!  *
   *******************/

unsafe fn snd_es1968_bob_stop(chip: *mut es1968) {
    let mut reg: u16;
    reg = __maestro_read(chip, 0x11);
    reg &= !ESM_BOB_ENABLE;
    __maestro_write(chip, 0x11, reg);
    reg = __maestro_read(chip, 0x17);
    reg &= !ESM_BOB_START;
    __maestro_write(chip, 0x17, reg);
}

unsafe fn snd_es1968_bob_start(chip: *mut es1968) {
    let mut prescale: c_int;
    let mut divide: c_int;
    /* compute ideal interrupt frequency for buffer size & play rate */
    /* first, find best prescaler value to match freq */
    prescale = 5;
    while prescale < 12 {
        if (*chip).bob_freq > (ESS_SYSCLK >> (prescale + 9)) {
            break;
        }
        prescale += 1;
    }
    /* next, back off prescaler whilst getting divider into optimum range */
    divide = 1;
    while prescale > 5 && divide < 32 {
        prescale -= 1;
        divide <<= 1;
    }
    divide >>= 1;
    /* now fine-tune the divider for best match */
    while divide < 31 {
        if (*chip).bob_freq > ((ESS_SYSCLK >> (prescale + 9)) / (divide + 1)) {
            break;
        }
        divide += 1;
    }
    /* divide = 0 is illegal, but don't let prescale = 4! */
    if divide == 0 {
        divide += 1;
        if prescale > 5 {
            prescale -= 1;
        }
    } else if divide > 1 {
        divide -= 1;
    }
    __maestro_write(chip, 6, (0x9000 | (prescale << 5) | divide) as u16); /* set reg */
    /* Now set IDR 11/17 */
    __maestro_write(chip, 0x11, __maestro_read(chip, 0x11) | 1);
    __maestro_write(chip, 0x17, __maestro_read(chip, 0x17) | 1);
}

/* call with substream spinlock */
unsafe fn snd_es1968_bob_inc(chip: *mut es1968, freq: c_int) {
    (*chip).bobclient += 1;
    if (*chip).bobclient == 1 {
        (*chip).bob_freq = freq;
        snd_es1968_bob_start(chip);
    } else if (*chip).bob_freq < freq {
        snd_es1968_bob_stop(chip);
        (*chip).bob_freq = freq;
        snd_es1968_bob_start(chip);
    }
}

/* call with substream spinlock */
unsafe fn snd_es1968_bob_dec(chip: *mut es1968) {
    (*chip).bobclient -= 1;
    if (*chip).bobclient <= 0 {
        snd_es1968_bob_stop(chip);
    } else if (*chip).bob_freq > ESM_BOB_FREQ {
        /* check reduction of timer frequency */
        let mut max_freq = ESM_BOB_FREQ;
        let mut p = (*chip).substream_list.next;
        while p != &mut (*chip).substream_list {
            let es = list_entry_esschan(p);
            if max_freq < (*es).bob_freq {
                max_freq = (*es).bob_freq;
            }
            p = (*p).next;
        }
        if max_freq != (*chip).bob_freq {
            snd_es1968_bob_stop(chip);
            (*chip).bob_freq = max_freq;
            snd_es1968_bob_start(chip);
        }
    }
}

unsafe fn snd_es1968_calc_bob_rate(chip: *mut es1968, es: *mut esschan, runtime: *mut snd_pcm_runtime) -> c_int {
    /* we acquire 4 interrupts per period for precise control.. */
    let mut freq: c_int = ((*runtime).rate * 4) as c_int;
    if ((*es).fmt & ESS_FMT_STEREO) != 0 {
        freq <<= 1;
    }
    if ((*es).fmt & ESS_FMT_16BIT) != 0 {
        freq <<= 1;
    }
    freq /= (*es).frag_size as c_int;
    if freq < ESM_BOB_FREQ {
        freq = ESM_BOB_FREQ;
    } else if freq > ESM_BOB_FREQ_MAX {
        freq = ESM_BOB_FREQ_MAX;
    }
    freq
}

/*************
 *  PCM Part *
 *************/

unsafe fn snd_es1968_compute_rate(chip: *mut es1968, freq: u32) -> u32 {
    let rate: u32 = (freq << 16) / (*chip).clock;
    /* #if 0 XXX: do we need this?
     * if rate > 0x10000 { rate = 0x10000; }
     */
    rate
}

/* get current pointer */
unsafe fn snd_es1968_get_dma_ptr(chip: *mut es1968, es: *mut esschan) -> c_uint {
    let mut offset: c_uint;
    offset = apu_get_register(chip, (*es).apu[0], 5) as c_uint;
    offset = offset.wrapping_sub((*es).base[0] as c_uint);
    offset & 0xFFFE /* hardware is in words */
}

unsafe fn snd_es1968_apu_set_freq(chip: *mut es1968, apu: c_int, freq: c_int) {
    apu_set_register(
        chip,
        apu,
        2,
        ((apu_get_register(chip, apu as u8, 2) & 0x00FF) as c_int | ((freq & 0xff) << 8) | 0x10) as u32,
    );
    apu_set_register(chip, apu, 3, (freq >> 8) as u32);
}

/* spin lock held */
unsafe fn snd_es1968_trigger_apu(esm: *mut es1968, apu: c_int, mode: c_int) {
    /* set the APU mode */
    __apu_set_register(
        esm,
        apu as u16,
        0,
        (((__apu_get_register(esm, apu as u16, 0) & 0xff0f) as c_int) | (mode << 4)) as u16,
    );
}

unsafe fn snd_es1968_pcm_start(chip: *mut es1968, es: *mut esschan) {
    spin_lock(&mut (*chip).reg_lock);
    __apu_set_register(chip, (*es).apu[0] as u16, 5, (*es).base[0]);
    snd_es1968_trigger_apu(chip, (*es).apu[0] as c_int, (*es).apu_mode[0] as c_int);
    if (*es).mode == ESM_MODE_CAPTURE {
        __apu_set_register(chip, (*es).apu[2] as u16, 5, (*es).base[2]);
        snd_es1968_trigger_apu(chip, (*es).apu[2] as c_int, (*es).apu_mode[2] as c_int);
    }
    if ((*es).fmt & ESS_FMT_STEREO) != 0 {
        __apu_set_register(chip, (*es).apu[1] as u16, 5, (*es).base[1]);
        snd_es1968_trigger_apu(chip, (*es).apu[1] as c_int, (*es).apu_mode[1] as c_int);
        if (*es).mode == ESM_MODE_CAPTURE {
            __apu_set_register(chip, (*es).apu[3] as u16, 5, (*es).base[3]);
            snd_es1968_trigger_apu(chip, (*es).apu[3] as c_int, (*es).apu_mode[3] as c_int);
        }
    }
    spin_unlock(&mut (*chip).reg_lock);
}

unsafe fn snd_es1968_pcm_stop(chip: *mut es1968, es: *mut esschan) {
    spin_lock(&mut (*chip).reg_lock);
    snd_es1968_trigger_apu(chip, (*es).apu[0] as c_int, 0);
    snd_es1968_trigger_apu(chip, (*es).apu[1] as c_int, 0);
    if (*es).mode == ESM_MODE_CAPTURE {
        snd_es1968_trigger_apu(chip, (*es).apu[2] as c_int, 0);
        snd_es1968_trigger_apu(chip, (*es).apu[3] as c_int, 0);
    }
    spin_unlock(&mut (*chip).reg_lock);
}

/* set the wavecache control reg */
unsafe fn snd_es1968_program_wavecache(chip: *mut es1968, es: *mut esschan, channel: c_int, addr: u32, capture: c_int) {
    let mut tmpval: u32 = (addr.wrapping_sub(0x10)) & 0xFFF8;
    if capture == 0 {
        if ((*es).fmt & ESS_FMT_16BIT) == 0 {
            tmpval |= 4; /* 8bit */
        }
        if ((*es).fmt & ESS_FMT_STEREO) != 0 {
            tmpval |= 2; /* stereo */
        }
    }
    /* set the wavecache control reg */
    wave_set_register(chip, ((*es).apu[channel as usize] as u16) << 3, tmpval);
    (*es).wc_map[channel as usize] = tmpval as u16;
}

unsafe fn snd_es1968_playback_setup(chip: *mut es1968, es: *mut esschan, runtime: *mut snd_pcm_runtime) {
    let mut pa: u32;
    let mut high_apu: c_int = 0;
    let mut channel: c_int;
    let mut apu: c_int;
    let mut i: c_int;
    let size: c_int;
    let mut freq: u32;
    size = ((*es).dma_size >> (*es).wav_shift) as c_int;
    if ((*es).fmt & ESS_FMT_STEREO) != 0 {
        high_apu += 1;
    }
    channel = 0;
    while channel <= high_apu {
        apu = (*es).apu[channel as usize] as c_int;
        snd_es1968_program_wavecache(chip, es, channel, (*(*es).memory).buf.addr, 0);
        /* Offset to PCMBAR */
        pa = (*(*es).memory).buf.addr;
        pa = pa.wrapping_sub((*chip).dma.addr);
        pa >>= 1; /* words */
        pa |= 0x00400000; /* System RAM (Bit 22) */
        if ((*es).fmt & ESS_FMT_STEREO) != 0 {
            /* Enable stereo */
            if channel != 0 {
                pa |= 0x00800000; /* (Bit 23) */
            }
            if ((*es).fmt & ESS_FMT_16BIT) != 0 {
                pa >>= 1;
            }
        }
        /* base offset of dma calcs when reading the pointer on this left one */
        (*es).base[channel as usize] = (pa & 0xFFFF) as u16;
        i = 0;
        while i < 16 {
            apu_set_register(chip, apu, i, 0x0000);
            i += 1;
        }
        /* Load the buffer into the wave engine */
        apu_set_register(chip, apu, 4, ((pa >> 16) & 0xFF) << 8);
        apu_set_register(chip, apu, 5, pa & 0xFFFF);
        apu_set_register(chip, apu, 6, (pa + size as u32) & 0xFFFF);
        /* setting loop == sample len */
        apu_set_register(chip, apu, 7, size as u32);
        /* clear effects/env.. */
        apu_set_register(chip, apu, 8, 0x0000);
        /* set amp now to 0xd0 (?), low byte is 'amplitude dest'? */
        apu_set_register(chip, apu, 9, 0xD000);
        /* clear routing stuff */
        apu_set_register(chip, apu, 11, 0x0000);
        /* dma on, no envelopes, filter to all 1s) */
        apu_set_register(chip, apu, 0, 0x400F);
        if ((*es).fmt & ESS_FMT_16BIT) != 0 {
            (*es).apu_mode[channel as usize] = ESM_APU_16BITLINEAR as u8;
        } else {
            (*es).apu_mode[channel as usize] = ESM_APU_8BITLINEAR as u8;
        }
        if ((*es).fmt & ESS_FMT_STEREO) != 0 {
            /* set panning: left or right */
            /* Check: different panning. On my Canyon 3D Chipset the
               Channels are swapped. I don't know, about the output
               to the SPDif Link. Perhaps you have to change this
               and not the APU Regs 4-5. */
            apu_set_register(chip, apu, 10, 0x8F00 | if channel != 0 { 0 } else { 0x10 });
            (*es).apu_mode[channel as usize] += 1; /* stereo */
        } else {
            apu_set_register(chip, apu, 10, 0x8F08);
        }
        channel += 1;
    }
    spin_lock(&mut (*chip).reg_lock);
    /* clear WP interrupts */
    outw(1, (*chip).io_port + 0x04);
    /* enable WP ints */
    outw(inw((*chip).io_port + ESM_PORT_HOST_IRQ) | ESM_HIRQ_DSIE, (*chip).io_port + ESM_PORT_HOST_IRQ);
    spin_unlock(&mut (*chip).reg_lock);
    freq = (*runtime).rate;
    /* set frequency */
    if freq > 48000 {
        freq = 48000;
    }
    if freq < 4000 {
        freq = 4000;
    }
    /* hmmm.. */
    if ((*es).fmt & ESS_FMT_16BIT) == 0 && ((*es).fmt & ESS_FMT_STEREO) == 0 {
        freq >>= 1;
    }
    freq = snd_es1968_compute_rate(chip, freq);
    /* Load the frequency, turn on 6dB */
    snd_es1968_apu_set_freq(chip, (*es).apu[0] as c_int, freq as c_int);
    snd_es1968_apu_set_freq(chip, (*es).apu[1] as c_int, freq as c_int);
}

unsafe fn init_capture_apu(chip: *mut es1968, es: *mut esschan, channel: c_int, mut pa: c_uint, bsize: c_uint, mode: c_int, route: c_int) {
    let mut i: c_int;
    let apu = (*es).apu[channel as usize] as c_int;
    (*es).apu_mode[channel as usize] = mode as u8;
    /* set the wavecache control reg */
    snd_es1968_program_wavecache(chip, es, channel, pa, 1);
    /* Offset to PCMBAR */
    pa = pa.wrapping_sub((*chip).dma.addr);
    pa >>= 1; /* words */
    /* base offset of dma calcs when reading the pointer on this left one */
    (*es).base[channel as usize] = (pa & 0xFFFF) as u16;
    pa |= 0x00400000; /* bit 22 -> System RAM */
    /* Begin loading the APU */
    i = 0;
    while i < 16 {
        apu_set_register(chip, apu, i, 0x0000);
        i += 1;
    }
    /* need to enable subgroups.. and we should probably have different groups for different /dev/dsps.. */
    apu_set_register(chip, apu, 2, 0x8);
    /* Load the buffer into the wave engine */
    apu_set_register(chip, apu, 4, ((pa >> 16) & 0xFF) << 8);
    apu_set_register(chip, apu, 5, pa & 0xFFFF);
    apu_set_register(chip, apu, 6, (pa + bsize) & 0xFFFF);
    apu_set_register(chip, apu, 7, bsize);
    /* clear effects/env.. */
    apu_set_register(chip, apu, 8, 0x00F0);
    /* amplitude now?  sure.  why not. */
    apu_set_register(chip, apu, 9, 0x0000);
    /* set filter tune, radius, polar pan */
    apu_set_register(chip, apu, 10, 0x8F08);
    /* route input */
    apu_set_register(chip, apu, 11, route as u32);
    /* dma on, no envelopes, filter to all 1s) */
    apu_set_register(chip, apu, 0, 0x400F);
}

unsafe fn snd_es1968_capture_setup(chip: *mut es1968, es: *mut esschan, runtime: *mut snd_pcm_runtime) {
    let size: c_int = ((*es).dma_size >> (*es).wav_shift) as c_int;
    let mut freq: u32;
    /* APU assignments:
       0 = mono/left SRC
       1 = right SRC
       2 = mono/left Input Mixer
       3 = right Input Mixer
    */
    /* data seems to flow from the codec, through an apu into
       the 'mixbuf' bit of page, then through the SRC apu
       and out to the real 'buffer'.  ok.  sure. */
    /* input mixer (left/mono) */
    /* parallel in crap, see maestro reg 0xC [8-11] */
    init_capture_apu(chip, es, 2, (*(*es).mixbuf).buf.addr, (ESM_MIXBUF_SIZE / 4) as c_uint, ESM_APU_INPUTMIXER, 0x14);
    /* SRC (left/mono); get input from inputing apu */
    init_capture_apu(chip, es, 0, (*(*es).memory).buf.addr, size as c_uint, ESM_APU_SRCONVERTOR, (*es).apu[2] as c_int);
    if ((*es).fmt & ESS_FMT_STEREO) != 0 {
        /* input mixer (right) */
        init_capture_apu(chip, es, 3, (*(*es).mixbuf).buf.addr + (ESM_MIXBUF_SIZE / 2) as u32, (ESM_MIXBUF_SIZE / 4) as c_uint, ESM_APU_INPUTMIXER, 0x15);
        /* SRC (right) */
        init_capture_apu(chip, es, 1, (*(*es).memory).buf.addr + (size * 2) as u32, size as c_uint, ESM_APU_SRCONVERTOR, (*es).apu[3] as c_int);
    }
    freq = (*runtime).rate;
    /* Sample Rate conversion APUs don't like 0x10000 for their rate */
    if freq > 47999 {
        freq = 47999;
    }
    if freq < 4000 {
        freq = 4000;
    }
    freq = snd_es1968_compute_rate(chip, freq);
    /* Load the frequency, turn on 6dB */
    snd_es1968_apu_set_freq(chip, (*es).apu[0] as c_int, freq as c_int);
    snd_es1968_apu_set_freq(chip, (*es).apu[1] as c_int, freq as c_int);
    /* fix mixer rate at 48khz.  and its _must_ be 0x10000. */
    freq = 0x10000;
    snd_es1968_apu_set_freq(chip, (*es).apu[2] as c_int, freq as c_int);
    snd_es1968_apu_set_freq(chip, (*es).apu[3] as c_int, freq as c_int);
    spin_lock(&mut (*chip).reg_lock);
    /* clear WP interrupts */
    outw(1, (*chip).io_port + 0x04);
    /* enable WP ints */
    outw(inw((*chip).io_port + ESM_PORT_HOST_IRQ) | ESM_HIRQ_DSIE, (*chip).io_port + ESM_PORT_HOST_IRQ);
    spin_unlock(&mut (*chip).reg_lock);
}

/*******************
 *  ALSA Interface *
 *******************/

unsafe extern "C" fn snd_es1968_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let es = (*runtime).private_data as *mut esschan;
    (*es).dma_size = snd_pcm_lib_buffer_bytes(substream);
    (*es).frag_size = snd_pcm_lib_period_bytes(substream);
    (*es).wav_shift = 1; /* maestro handles always 16bit */
    (*es).fmt = 0;
    if snd_pcm_format_width((*runtime).format) == 16 {
        (*es).fmt |= ESS_FMT_16BIT;
    }
    if (*runtime).channels > 1 {
        (*es).fmt |= ESS_FMT_STEREO;
        if ((*es).fmt & ESS_FMT_16BIT) != 0 {
            /* 8bit is already word shifted */
            (*es).wav_shift += 1;
        }
    }
    (*es).bob_freq = snd_es1968_calc_bob_rate(chip, es, runtime);
    match (*es).mode {
        ESM_MODE_PLAY => snd_es1968_playback_setup(chip, es, runtime),
        ESM_MODE_CAPTURE => snd_es1968_capture_setup(chip, es, runtime),
        _ => {}
    }
    0
}

unsafe extern "C" fn snd_es1968_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let es = (*(*substream).runtime).private_data as *mut esschan;
    spin_lock(&mut (*chip).substream_lock);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            if (*es).running == 0 {
                snd_es1968_bob_inc(chip, (*es).bob_freq);
                (*es).count = 0;
                (*es).hwptr = 0;
                snd_es1968_pcm_start(chip, es);
                (*es).running = 1;
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            if (*es).running != 0 {
                snd_es1968_pcm_stop(chip, es);
                (*es).running = 0;
                snd_es1968_bob_dec(chip);
            }
        }
        _ => {}
    }
    spin_unlock(&mut (*chip).substream_lock);
    0
}

unsafe extern "C" fn snd_es1968_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let es = (*(*substream).runtime).private_data as *mut esschan;
    let ptr_ = snd_es1968_get_dma_ptr(chip, es) << (*es).wav_shift;
    bytes_to_frames((*substream).runtime, ptr_ % (*es).dma_size)
}

static snd_es1968_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | /*SNDRV_PCM_INFO_PAUSE |*/ SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 65536,
    period_bytes_min: 256,
    period_bytes_max: 65536,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

static snd_es1968_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_NONINTERLEAVED | SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_BLOCK_TRANSFER | /*SNDRV_PCM_INFO_PAUSE |*/ SNDRV_PCM_INFO_RESUME,
    formats: /*SNDRV_PCM_FMTBIT_U8 |*/ SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 65536,
    period_bytes_min: 256,
    period_bytes_max: 65536,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

/* *************************
   * DMA memory management *
   *************************/

/* Because the Maestro can only take addresses relative to the PCM base address
   register :( */

unsafe fn calc_available_memory_size(chip: *mut es1968) -> c_int {
    let mut max_size: c_int = 0;
    let mut p: *mut list_head;
    p = (*chip).buf_list.next;
    while p != &mut (*chip).buf_list {
        let buf = list_entry_esm_memory(p);
        if (*buf).empty != 0 && (*buf).buf.bytes > max_size {
            max_size = (*buf).buf.bytes;
        }
        p = (*p).next;
    }
    if max_size >= 128 * 1024 {
        max_size = 127 * 1024;
    }
    max_size
}

/* allocate a new memory chunk with the specified size */
unsafe fn snd_es1968_new_memory(chip: *mut es1968, mut size: c_int) -> *mut esm_memory {
    let mut p: *mut list_head;
    let mut buf: *mut esm_memory;
    size = ALIGN(size, ESM_MEM_ALIGN);
    p = (*chip).buf_list.next;
    while p != &mut (*chip).buf_list {
        buf = list_entry_esm_memory(p);
        if (*buf).empty != 0 && (*buf).buf.bytes >= size {
            if (*buf).buf.bytes > size {
                let chunk: *mut esm_memory = kmalloc_obj();
                if chunk.is_null() {
                    return ptr::null_mut();
                }
                (*chunk).buf = (*buf).buf;
                (*chunk).buf.bytes -= size;
                (*chunk).buf.area = (*chunk).buf.area.add(size as usize);
                (*chunk).buf.addr += size as u32;
                (*chunk).empty = 1;
                (*buf).buf.bytes = size;
                list_add(&mut (*chunk).list, &mut (*buf).list);
            }
            (*buf).empty = 0;
            return buf;
        }
        p = (*p).next;
    }
    ptr::null_mut()
}

/* free a memory chunk */
unsafe fn snd_es1968_free_memory(chip: *mut es1968, mut buf: *mut esm_memory) {
    let mut chunk: *mut esm_memory;
    (*buf).empty = 1;
    if (*buf).list.prev != &mut (*chip).buf_list {
        chunk = list_entry_esm_memory((*buf).list.prev);
        if (*chunk).empty != 0 {
            (*chunk).buf.bytes += (*buf).buf.bytes;
            list_del(&mut (*buf).list);
            kfree(buf as *mut c_void);
            buf = chunk;
        }
    }
    if (*buf).list.next != &mut (*chip).buf_list {
        chunk = list_entry_esm_memory((*buf).list.next);
        if (*chunk).empty != 0 {
            (*buf).buf.bytes += (*chunk).buf.bytes;
            list_del(&mut (*chunk).list);
            kfree(chunk as *mut c_void);
        }
    }
}

unsafe fn snd_es1968_free_dmabuf(chip: *mut es1968) {
    let mut p: *mut list_head;
    if (*chip).dma.area.is_null() {
        return;
    }
    snd_dma_free_pages(&mut (*chip).dma);
    while {
        p = (*chip).buf_list.next;
        p != &mut (*chip).buf_list
    } {
        let chunk = list_entry_esm_memory(p);
        list_del(p);
        kfree(chunk as *mut c_void);
    }
}

unsafe fn snd_es1968_init_dmabuf(chip: *mut es1968) -> c_int {
    let mut err: c_int;
    let chunk: *mut esm_memory;
    err = snd_dma_alloc_pages_fallback(SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, (*chip).total_bufsize, &mut (*chip).dma);
    if err < 0 || (*chip).dma.area.is_null() {
        dev_err((*(*chip).card).dev, b"can't allocate dma pages for size %d\n\0".as_ptr() as *const c_char, (*chip).total_bufsize);
        return -ENOMEM;
    }
    if (((*chip).dma.addr + (*chip).dma.bytes as u32 - 1) & !((1u32 << 28) - 1)) != 0 {
        snd_dma_free_pages(&mut (*chip).dma);
        dev_err((*(*chip).card).dev, b"DMA buffer beyond 256MB.\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }
    INIT_LIST_HEAD(&mut (*chip).buf_list);
    /* allocate an empty chunk */
    chunk = kmalloc_obj();
    if chunk.is_null() {
        snd_es1968_free_dmabuf(chip);
        return -ENOMEM;
    }
    memset((*chip).dma.area as *mut c_void, 0, ESM_MEM_ALIGN as usize);
    (*chunk).buf = (*chip).dma;
    (*chunk).buf.area = (*chunk).buf.area.add(ESM_MEM_ALIGN as usize);
    (*chunk).buf.addr += ESM_MEM_ALIGN as u32;
    (*chunk).buf.bytes -= ESM_MEM_ALIGN;
    (*chunk).empty = 1;
    list_add(&mut (*chunk).list, &mut (*chip).buf_list);
    0
}

/* setup the dma_areas */
/* buffer is extracted from the pre-allocated memory chunk */
unsafe extern "C" fn snd_es1968_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let chan = (*runtime).private_data as *mut esschan;
    let size = params_buffer_bytes(hw_params);
    if !(*chan).memory.is_null() {
        if (*(*chan).memory).buf.bytes >= size {
            (*runtime).dma_bytes = size;
            return 0;
        }
        snd_es1968_free_memory(chip, (*chan).memory);
    }
    (*chan).memory = snd_es1968_new_memory(chip, size);
    if (*chan).memory.is_null() {
        dev_dbg((*(*chip).card).dev, b"cannot allocate dma buffer: size = %d\n\0".as_ptr() as *const c_char, size);
        return -ENOMEM;
    }
    snd_pcm_set_runtime_buffer(substream, &mut (*(*chan).memory).buf);
    (*runtime).dma_bytes = size;
    1 /* area was changed */
}

/* remove dma areas if allocated */
unsafe extern "C" fn snd_es1968_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let chan: *mut esschan;
    if (*runtime).private_data.is_null() {
        return 0;
    }
    chan = (*runtime).private_data as *mut esschan;
    if !(*chan).memory.is_null() {
        snd_es1968_free_memory(chip, (*chan).memory);
        (*chan).memory = ptr::null_mut();
    }
    0
}

/*
 * allocate APU pair
 */
unsafe fn snd_es1968_alloc_apu_pair(chip: *mut es1968, type_: c_int) -> c_int {
    let mut apu: c_int = 0;
    while apu < NR_APUS as c_int {
        if (*chip).apu[apu as usize] == snd_enum_apu_type::ESM_APU_FREE as u8
            && (*chip).apu[(apu + 1) as usize] == snd_enum_apu_type::ESM_APU_FREE as u8
        {
            (*chip).apu[apu as usize] = type_ as u8;
            (*chip).apu[(apu + 1) as usize] = type_ as u8;
            return apu;
        }
        apu += 2;
    }
    -EBUSY
}

/*
 * release APU pair
 */
unsafe fn snd_es1968_free_apu_pair(chip: *mut es1968, apu: c_int) {
    (*chip).apu[apu as usize] = snd_enum_apu_type::ESM_APU_FREE as u8;
    (*chip).apu[(apu + 1) as usize] = snd_enum_apu_type::ESM_APU_FREE as u8;
}

/******************
 * PCM open/close *
 ******************/

unsafe extern "C" fn snd_es1968_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let es: *mut esschan;
    let apu1: c_int;
    /* search 2 APUs */
    apu1 = snd_es1968_alloc_apu_pair(chip, snd_enum_apu_type::ESM_APU_PCM_PLAY as c_int);
    if apu1 < 0 {
        return apu1;
    }
    es = kzalloc_obj();
    if es.is_null() {
        snd_es1968_free_apu_pair(chip, apu1);
        return -ENOMEM;
    }
    (*es).apu[0] = apu1 as u8;
    (*es).apu[1] = (apu1 + 1) as u8;
    (*es).apu_mode[0] = 0;
    (*es).apu_mode[1] = 0;
    (*es).running = 0;
    (*es).substream = substream;
    (*es).mode = ESM_MODE_PLAY;
    (*runtime).private_data = es as *mut c_void;
    (*runtime).hw = snd_es1968_playback;
    (*runtime).hw.buffer_bytes_max = calc_available_memory_size(chip) as c_uint;
    (*runtime).hw.period_bytes_max = (*runtime).hw.buffer_bytes_max;
    spin_lock(&mut (*chip).substream_lock);
    list_add(&mut (*es).list, &mut (*chip).substream_list);
    spin_unlock(&mut (*chip).substream_lock);
    0
}

unsafe extern "C" fn snd_es1968_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let chip = snd_pcm_substream_chip(substream);
    let es: *mut esschan;
    let mut err: c_int;
    let apu1: c_int;
    let apu2: c_int;
    apu1 = snd_es1968_alloc_apu_pair(chip, snd_enum_apu_type::ESM_APU_PCM_CAPTURE as c_int);
    if apu1 < 0 {
        return apu1;
    }
    apu2 = snd_es1968_alloc_apu_pair(chip, snd_enum_apu_type::ESM_APU_PCM_RATECONV as c_int);
    if apu2 < 0 {
        snd_es1968_free_apu_pair(chip, apu1);
        return apu2;
    }
    es = kzalloc_obj();
    if es.is_null() {
        snd_es1968_free_apu_pair(chip, apu1);
        snd_es1968_free_apu_pair(chip, apu2);
        return -ENOMEM;
    }
    (*es).apu[0] = apu1 as u8;
    (*es).apu[1] = (apu1 + 1) as u8;
    (*es).apu[2] = apu2 as u8;
    (*es).apu[3] = (apu2 + 1) as u8;
    (*es).apu_mode[0] = 0;
    (*es).apu_mode[1] = 0;
    (*es).apu_mode[2] = 0;
    (*es).apu_mode[3] = 0;
    (*es).running = 0;
    (*es).substream = substream;
    (*es).mode = ESM_MODE_CAPTURE;
    /* get mixbuffer */
    (*es).mixbuf = snd_es1968_new_memory(chip, ESM_MIXBUF_SIZE);
    if (*es).mixbuf.is_null() {
        snd_es1968_free_apu_pair(chip, apu1);
        snd_es1968_free_apu_pair(chip, apu2);
        kfree(es as *mut c_void);
        return -ENOMEM;
    }
    memset((*(*es).mixbuf).buf.area as *mut c_void, 0, ESM_MIXBUF_SIZE as usize);
    (*runtime).private_data = es as *mut c_void;
    (*runtime).hw = snd_es1968_capture;
    (*runtime).hw.buffer_bytes_max = (calc_available_memory_size(chip) - 1024) as c_uint; /* keep MIXBUF size */
    (*runtime).hw.period_bytes_max = (*runtime).hw.buffer_bytes_max;
    err = snd_pcm_hw_constraint_pow2(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES);
    if err < 0 {
        return err;
    }
    spin_lock(&mut (*chip).substream_lock);
    list_add(&mut (*es).list, &mut (*chip).substream_list);
    spin_unlock(&mut (*chip).substream_lock);
    0
}

unsafe extern "C" fn snd_es1968_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let es: *mut esschan;
    if (*(*substream).runtime).private_data.is_null() {
        return 0;
    }
    es = (*(*substream).runtime).private_data as *mut esschan;
    spin_lock(&mut (*chip).substream_lock);
    list_del(&mut (*es).list);
    spin_unlock(&mut (*chip).substream_lock);
    snd_es1968_free_apu_pair(chip, (*es).apu[0] as c_int);
    kfree(es as *mut c_void);
    0
}

unsafe extern "C" fn snd_es1968_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let es: *mut esschan;
    if (*(*substream).runtime).private_data.is_null() {
        return 0;
    }
    es = (*(*substream).runtime).private_data as *mut esschan;
    spin_lock(&mut (*chip).substream_lock);
    list_del(&mut (*es).list);
    spin_unlock(&mut (*chip).substream_lock);
    snd_es1968_free_memory(chip, (*es).mixbuf);
    snd_es1968_free_apu_pair(chip, (*es).apu[0] as c_int);
    snd_es1968_free_apu_pair(chip, (*es).apu[2] as c_int);
    kfree(es as *mut c_void);
    0
}

static snd_es1968_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_es1968_playback_open),
    close: Some(snd_es1968_playback_close),
    hw_params: Some(snd_es1968_hw_params),
    hw_free: Some(snd_es1968_hw_free),
    prepare: Some(snd_es1968_pcm_prepare),
    trigger: Some(snd_es1968_pcm_trigger),
    pointer: Some(snd_es1968_pcm_pointer),
};

static snd_es1968_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_es1968_capture_open),
    close: Some(snd_es1968_capture_close),
    hw_params: Some(snd_es1968_hw_params),
    hw_free: Some(snd_es1968_hw_free),
    prepare: Some(snd_es1968_pcm_prepare),
    trigger: Some(snd_es1968_pcm_trigger),
    pointer: Some(snd_es1968_pcm_pointer),
};

/*
 * measure clock
 */
const CLOCK_MEASURE_BUFSIZE: c_int = 16768; /* enough large for a single shot */

unsafe fn es1968_measure_clock(chip: *mut es1968) {
    let mut i: c_int;
    let apu: c_int;
    let mut pa: c_uint;
    let mut offset: c_uint;
    let t: c_uint;
    let memory: *mut esm_memory;
    let start_time: ktime_t;
    let stop_time: ktime_t;
    let diff: ktime_t;
    if (*chip).clock == 0 {
        (*chip).clock = 48000; /* default clock value */
    }
    /* search 2 APUs (although one apu is enough) */
    apu = snd_es1968_alloc_apu_pair(chip, snd_enum_apu_type::ESM_APU_PCM_PLAY as c_int);
    if apu < 0 {
        dev_err((*(*chip).card).dev, b"Hmm, cannot find empty APU pair!?\n\0".as_ptr() as *const c_char);
        return;
    }
    memory = snd_es1968_new_memory(chip, CLOCK_MEASURE_BUFSIZE);
    if memory.is_null() {
        dev_warn((*(*chip).card).dev, b"cannot allocate dma buffer - using default clock %d\n\0".as_ptr() as *const c_char, (*chip).clock);
        snd_es1968_free_apu_pair(chip, apu);
        return;
    }
    memset((*memory).buf.area as *mut c_void, 0, CLOCK_MEASURE_BUFSIZE as usize);
    wave_set_register(chip, (apu << 3) as u16, ((*memory).buf.addr - 0x10) & 0xfff8);
    pa = ((*memory).buf.addr - (*chip).dma.addr) >> 1;
    pa |= 0x00400000; /* System RAM (Bit 22) */
    /* initialize apu */
    i = 0;
    while i < 16 {
        apu_set_register(chip, apu, i, 0x0000);
        i += 1;
    }
    apu_set_register(chip, apu, 0, 0x400f);
    apu_set_register(chip, apu, 4, ((pa >> 16) & 0xff) << 8);
    apu_set_register(chip, apu, 5, pa & 0xffff);
    apu_set_register(chip, apu, 6, (pa + (CLOCK_MEASURE_BUFSIZE / 2) as u32) & 0xffff);
    apu_set_register(chip, apu, 7, (CLOCK_MEASURE_BUFSIZE / 2) as u32);
    apu_set_register(chip, apu, 8, 0x0000);
    apu_set_register(chip, apu, 9, 0xD000);
    apu_set_register(chip, apu, 10, 0x8F08);
    apu_set_register(chip, apu, 11, 0x0000);
    spin_lock(&mut (*chip).reg_lock);
    outw(1, (*chip).io_port + 0x04); /* clear WP interrupts */
    outw(inw((*chip).io_port + ESM_PORT_HOST_IRQ) | ESM_HIRQ_DSIE, (*chip).io_port + ESM_PORT_HOST_IRQ); /* enable WP ints */
    spin_unlock(&mut (*chip).reg_lock);
    snd_es1968_apu_set_freq(chip, apu, (((48000u32) << 16) / (*chip).clock) as c_int); /* 48000 Hz */
    (*chip).in_measurement = 1;
    (*chip).measure_apu = apu as c_uint;
    spin_lock(&mut (*chip).reg_lock);
    snd_es1968_bob_inc(chip, ESM_BOB_FREQ);
    __apu_set_register(chip, apu as u16, 5, (pa & 0xffff) as u16);
    snd_es1968_trigger_apu(chip, apu, ESM_APU_16BITLINEAR);
    start_time = ktime_get();
    spin_unlock(&mut (*chip).reg_lock);
    msleep(50);
    spin_lock(&mut (*chip).reg_lock);
    offset = __apu_get_register(chip, apu as u16, 5) as c_uint;
    stop_time = ktime_get();
    snd_es1968_trigger_apu(chip, apu, 0); /* stop */
    snd_es1968_bob_dec(chip);
    (*chip).in_measurement = 0;
    spin_unlock(&mut (*chip).reg_lock);
    /* check the current position */
    offset = offset.wrapping_sub(pa & 0xffff);
    offset &= 0xfffe;
    offset += (*chip).measure_count * (CLOCK_MEASURE_BUFSIZE / 2) as u32;
    diff = ktime_sub(stop_time, start_time);
    t = ktime_to_us(diff);
    if t == 0 {
        dev_err((*(*chip).card).dev, b"?? calculation error..\n\0".as_ptr() as *const c_char);
    } else {
        offset *= 1000;
        offset = (offset / t) * 1000 + ((offset % t) * 1000) / t;
        if offset < 47500 || offset > 48500 {
            if offset >= 40000 && offset <= 50000 {
                (*chip).clock = ((*chip).clock * offset) / 48000;
            }
        }
        dev_info((*(*chip).card).dev, b"clocking to %d\n\0".as_ptr() as *const c_char, (*chip).clock);
    }
    snd_es1968_free_memory(chip, memory);
    snd_es1968_free_apu_pair(chip, apu);
}

unsafe extern "C" fn snd_es1968_pcm_free(pcm: *mut snd_pcm) {
    let esm = (*pcm).private_data as *mut es1968;
    snd_es1968_free_dmabuf(esm);
    (*esm).pcm = ptr::null_mut();
}

unsafe fn snd_es1968_pcm(chip: *mut es1968, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;
    /* get DMA buffer */
    err = snd_es1968_init_dmabuf(chip);
    if err < 0 {
        return err;
    }
    /* set PCMBAR */
    wave_set_register(chip, 0x01FC, (*chip).dma.addr >> 12);
    wave_set_register(chip, 0x01FD, (*chip).dma.addr >> 12);
    wave_set_register(chip, 0x01FE, (*chip).dma.addr >> 12);
    wave_set_register(chip, 0x01FF, (*chip).dma.addr >> 12);
    err = snd_pcm_new((*chip).card, b"ESS Maestro\0".as_ptr() as *const c_char, device, (*chip).playback_streams, (*chip).capture_streams, &mut pcm);
    if err < 0 {
        return err;
    }
    (*pcm).private_data = chip as *mut c_void;
    (*pcm).private_free = Some(snd_es1968_pcm_free);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_es1968_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_es1968_capture_ops);
    (*pcm).info_flags = 0;
    strscpy((*pcm).name.as_mut_ptr(), b"ESS Maestro\0".as_ptr() as *const c_char, (*pcm).name.len());
    (*chip).pcm = pcm;
    0
}

/*
 * suppress jitter on some maestros when playing stereo
 */
unsafe fn snd_es1968_suppress_jitter(chip: *mut es1968, es: *mut esschan) {
    let cp1: c_uint;
    let cp2: c_uint;
    let diff: c_uint;
    cp1 = __apu_get_register(chip, 0, 5) as c_uint;
    cp2 = __apu_get_register(chip, 1, 5) as c_uint;
    diff = if cp1 > cp2 { cp1 - cp2 } else { cp2 - cp1 };
    if diff > 1 {
        __maestro_write(chip, IDR0_DATA_PORT, cp1 as u16);
    }
}

/*
 * update pointer
 */
unsafe fn snd_es1968_update_pcm(chip: *mut es1968, es: *mut esschan) {
    let mut hwptr: c_uint;
    let diff: c_uint;
    let subs = (*es).substream;
    if subs.is_null() || (*es).running == 0 {
        return;
    }
    hwptr = snd_es1968_get_dma_ptr(chip, es) << (*es).wav_shift;
    hwptr %= (*es).dma_size;
    diff = ((*es).dma_size + hwptr - (*es).hwptr) % (*es).dma_size;
    (*es).hwptr = hwptr;
    (*es).count += diff;
    if (*es).count > (*es).frag_size {
        spin_unlock(&mut (*chip).substream_lock);
        snd_pcm_period_elapsed(subs);
        spin_lock(&mut (*chip).substream_lock);
        (*es).count %= (*es).frag_size;
    }
}

/* The hardware volume works by incrementing / decrementing 2 counters
   (without wrap around) in response to volume button presses and then
   generating an interrupt. The pair of counters is stored in bits 1-3 and 5-7
   of a byte wide register. The meaning of bits 0 and 4 is unknown. */
unsafe extern "C" fn es1968_update_hw_volume(work: *mut work_struct) {
    let chip = (work as *mut u8).offset(-(core::mem::offset_of!(es1968, hwvol_work) as isize)) as *mut es1968;
    let x: c_int;
    let mut val: c_int;
    /* Figure out which volume control button was pushed,
       based on differences from the default register values. */
    x = (inb((*chip).io_port + 0x1c) & 0xee) as c_int;
    /* Reset the volume control registers. */
    outb(0x88, (*chip).io_port + 0x1c);
    outb(0x88, (*chip).io_port + 0x1d);
    outb(0x88, (*chip).io_port + 0x1e);
    outb(0x88, (*chip).io_port + 0x1f);
    if (*chip).in_suspend != 0 {
        return;
    }
    /* #ifndef CONFIG_SND_ES1968_INPUT */
    if (*chip).master_switch.is_null() || (*chip).master_volume.is_null() {
        return;
    }
    val = snd_ac97_read((*chip).ac97, AC97_MASTER);
    match x {
        0x88 => {
            /* mute */
            val ^= 0x8000;
        }
        0xaa => {
            /* volume up */
            if (val & 0x7f) > 0 {
                val -= 1;
            }
            if (val & 0x7f00) > 0 {
                val -= 0x0100;
            }
        }
        0x66 => {
            /* volume down */
            if (val & 0x7f) < 0x1f {
                val += 1;
            }
            if (val & 0x7f00) < 0x1f00 {
                val += 0x0100;
            }
        }
        _ => {}
    }
    if snd_ac97_update((*chip).ac97, AC97_MASTER, val) != 0 {
        snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).master_volume).id);
    }
    /* #else CONFIG_SND_ES1968_INPUT branch:
     * report KEY_MUTE, KEY_VOLUMEUP, or KEY_VOLUMEDOWN through input_report_key/input_sync.
     */
}

/*
 * interrupt handler
 */
unsafe extern "C" fn snd_es1968_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut es1968;
    let event: u32;
    event = inb((*chip).io_port + 0x1A) as u32;
    if event == 0 {
        return IRQ_NONE;
    }
    outw(inw((*chip).io_port + 4) & 1, (*chip).io_port + 4);
    if (event & ESM_HWVOL_IRQ) != 0 {
        schedule_work(&mut (*chip).hwvol_work);
    }
    /* else ack 'em all, i imagine */
    outb(0xFF, (*chip).io_port + 0x1A);
    if (event & ESM_MPU401_IRQ) != 0 && !(*chip).rmidi.is_null() {
        snd_mpu401_uart_interrupt(irq, (*(*chip).rmidi).private_data);
    }
    if (event & ESM_SOUND_IRQ) != 0 {
        let mut p: *mut list_head;
        spin_lock(&mut (*chip).substream_lock);
        p = (*chip).substream_list.next;
        while p != &mut (*chip).substream_list {
            let es = list_entry_esschan(p);
            if (*es).running != 0 {
                snd_es1968_update_pcm(chip, es);
                if ((*es).fmt & ESS_FMT_STEREO) != 0 {
                    snd_es1968_suppress_jitter(chip, es);
                }
            }
            p = (*p).next;
        }
        spin_unlock(&mut (*chip).substream_lock);
        if (*chip).in_measurement != 0 {
            let curp = __apu_get_register(chip, (*chip).measure_apu as u16, 5) as c_uint;
            if curp < (*chip).measure_lastpos {
                (*chip).measure_count += 1;
            }
            (*chip).measure_lastpos = curp;
        }
    }
    IRQ_HANDLED
}

/*
 *  Mixer stuff
 */

unsafe fn snd_es1968_mixer(chip: *mut es1968) -> c_int {
    let mut pbus: *mut snd_ac97_bus = ptr::null_mut();
    let mut ac97: snd_ac97_template = core::mem::zeroed();
    let mut err: c_int;
    static ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
        write: Some(snd_es1968_ac97_write),
        read: Some(snd_es1968_ac97_read),
    };
    err = snd_ac97_bus((*chip).card, 0, &ops, ptr::null_mut(), &mut pbus);
    if err < 0 {
        return err;
    }
    (*pbus).no_vra = 1; /* ES1968 doesn't need VRA */
    memset(&mut ac97 as *mut _ as *mut c_void, 0, size_of::<snd_ac97_template>());
    ac97.private_data = chip as *mut c_void;
    err = snd_ac97_mixer(pbus, &mut ac97, &mut (*chip).ac97);
    if err < 0 {
        return err;
    }
    /* #ifndef CONFIG_SND_ES1968_INPUT */
    /* attach master switch / volumes for h/w volume control */
    (*chip).master_switch = snd_ctl_find_id_mixer((*chip).card, b"Master Playback Switch\0".as_ptr() as *const c_char);
    (*chip).master_volume = snd_ctl_find_id_mixer((*chip).card, b"Master Playback Volume\0".as_ptr() as *const c_char);
    /* #endif */
    0
}

/*
 * reset ac97 codec
 */

unsafe fn snd_es1968_ac97_reset(chip: *mut es1968) {
    let ioaddr = (*chip).io_port;
    let save_ringbus_a: u16;
    let mut save_68: u16;
    let mut w: u16 = 0;
    let mut vend: u32 = 0;
    /* save configuration */
    save_ringbus_a = inw(ioaddr + 0x36);
    //outw(inw(ioaddr + 0x38) & 0xfffc, ioaddr + 0x38); /* clear second codec id? */
    /* set command/status address i/o to 1st codec */
    outw(inw(ioaddr + 0x3a) & 0xfffc, ioaddr + 0x3a);
    outw(inw(ioaddr + 0x3c) & 0xfffc, ioaddr + 0x3c);
    /* disable ac link */
    outw(0x0000, ioaddr + 0x36);
    save_68 = inw(ioaddr + 0x68);
    pci_read_config_word((*chip).pci, 0x58, &mut w); /* something magical with gpio and bus arb. */
    pci_read_config_dword((*chip).pci, PCI_SUBSYSTEM_VENDOR_ID, &mut vend);
    if (w & 1) != 0 {
        save_68 |= 0x10;
    }
    outw(0xfffe, ioaddr + 0x64); /* unmask gpio 0 */
    outw(0x0001, ioaddr + 0x68); /* gpio write */
    outw(0x0000, ioaddr + 0x60); /* write 0 to gpio 0 */
    udelay(20);
    outw(0x0001, ioaddr + 0x60); /* write 1 to gpio 1 */
    msleep(20);
    outw(save_68 | 0x1, ioaddr + 0x68); /* now restore .. */
    outw((inw(ioaddr + 0x38) & 0xfffc) | 0x1, ioaddr + 0x38);
    outw((inw(ioaddr + 0x3a) & 0xfffc) | 0x1, ioaddr + 0x3a);
    outw((inw(ioaddr + 0x3c) & 0xfffc) | 0x1, ioaddr + 0x3c);
    /* now the second codec */
    /* disable ac link */
    outw(0x0000, ioaddr + 0x36);
    outw(0xfff7, ioaddr + 0x64); /* unmask gpio 3 */
    save_68 = inw(ioaddr + 0x68);
    outw(0x0009, ioaddr + 0x68); /* gpio write 0 & 3 ?? */
    outw(0x0001, ioaddr + 0x60); /* write 1 to gpio */
    udelay(20);
    outw(0x0009, ioaddr + 0x60); /* write 9 to gpio */
    msleep(500);
    //outw(inw(ioaddr + 0x38) & 0xfffc, ioaddr + 0x38);
    outw(inw(ioaddr + 0x3a) & 0xfffc, ioaddr + 0x3a);
    outw(inw(ioaddr + 0x3c) & 0xfffc, ioaddr + 0x3c);
    /* #if 0 software reset loop omitted as disabled in the C source. */
    if vend == NEC_VERSA_SUBID1 || vend == NEC_VERSA_SUBID2 {
        /* turn on external amp? */
        outw(0xf9ff, ioaddr + 0x64);
        outw(inw(ioaddr + 0x68) | 0x600, ioaddr + 0x68);
        outw(0x0209, ioaddr + 0x60);
    }
    /* restore.. */
    outw(save_ringbus_a, ioaddr + 0x36);
    /* Turn on the 978 docking chip.
       First frob the "master output enable" bit,
       then set most of the playback volume control registers to max. */
    outb(inb(ioaddr + 0xc0) | (1 << 5), ioaddr + 0xc0);
    outb(0xff, ioaddr + 0xc3);
    outb(0xff, ioaddr + 0xc4);
    outb(0xff, ioaddr + 0xc6);
    outb(0xff, ioaddr + 0xc8);
    outb(0x3f, ioaddr + 0xcf);
    outb(0x3f, ioaddr + 0xd0);
}

unsafe fn snd_es1968_reset(chip: *mut es1968) {
    /* Reset */
    outw(ESM_RESET_MAESTRO | ESM_RESET_DIRECTSOUND, (*chip).io_port + ESM_PORT_HOST_IRQ);
    udelay(10);
    outw(0x0000, (*chip).io_port + ESM_PORT_HOST_IRQ);
    udelay(10);
}

/*
 * initialize maestro chip
 */
unsafe fn snd_es1968_chip_init(chip: *mut es1968) {
    let pci = (*chip).pci;
    let mut i: c_int;
    let iobase = (*chip).io_port;
    let mut w: u16 = 0;
    let mut n: u32;
    /* We used to muck around with pci config space that
     * we had no business messing with.  We don't know enough
     * about the machine to know which DMA mode is appropriate,
     * etc.  We were guessing wrong on some machines and making
     * them unhappy.  We now trust in the BIOS to do things right,
     * which almost certainly means a new host of problems will
     * arise with broken BIOS implementations.  screw 'em.
     * We're already intolerant of machines that don't assign
     * IRQs.
     */
    /* Config Reg A */
    pci_read_config_word(pci, ESM_CONFIG_A, &mut w);
    w &= !DMA_CLEAR; /* Clear DMA bits */
    w &= !(PIC_SNOOP1 | PIC_SNOOP2); /* Clear Pic Snoop Mode Bits */
    w &= !SAFEGUARD; /* Safeguard off */
    w |= POST_WRITE; /* Posted write */
    w |= PCI_TIMING; /* PCI timing on */
    /* XXX huh?  claims to be reserved.. */
    w &= !SWAP_LR; /* swap left/right seems to only have effect on SB Emulation */
    w &= !SUBTR_DECODE; /* Subtractive decode off */
    pci_write_config_word(pci, ESM_CONFIG_A, w);
    /* Config Reg B */
    pci_read_config_word(pci, ESM_CONFIG_B, &mut w);
    w &= !(1 << 15); /* Turn off internal clock multiplier */
    /* XXX how do we know which to use? */
    w &= !(1 << 14); /* External clock */
    w &= !SPDIF_CONFB; /* disable S/PDIF output */
    w |= HWV_CONFB; /* HWV on */
    w |= DEBOUNCE; /* Debounce off: easier to push the HW buttons */
    w &= !GPIO_CONFB; /* GPIO 4:5 */
    w |= CHI_CONFB; /* Disconnect from the CHI.  Enabling this made a dell 7500 work. */
    w &= !IDMA_CONFB; /* IDMA off (undocumented) */
    w &= !MIDI_FIX; /* MIDI fix off (undoc) */
    w &= !(1 << 1); /* reserved, always write 0 */
    w &= !IRQ_TO_ISA; /* IRQ to ISA off (undoc) */
    pci_write_config_word(pci, ESM_CONFIG_B, w);
    /* DDMA off */
    pci_read_config_word(pci, ESM_DDMA, &mut w);
    w &= !(1 << 0);
    pci_write_config_word(pci, ESM_DDMA, w);
    /*
     *	Legacy mode
     */
    pci_read_config_word(pci, ESM_LEGACY_AUDIO_CONTROL, &mut w);
    w |= ESS_DISABLE_AUDIO; /* Disable Legacy Audio */
    w &= !ESS_ENABLE_SERIAL_IRQ; /* Disable SIRQ */
    w &= !(0x1f); /* disable mpu irq/io, game port, fm, SB */
    pci_write_config_word(pci, ESM_LEGACY_AUDIO_CONTROL, w);
    /* Set up 978 docking control chip. */
    pci_read_config_word(pci, 0x58, &mut w);
    w |= 1 << 2; /* Enable 978. */
    w |= 1 << 3; /* Turn on 978 hardware volume control. */
    w &= !(1 << 11); /* Turn on 978 mixer volume control. */
    pci_write_config_word(pci, 0x58, w);
    /* Sound Reset */
    snd_es1968_reset(chip);
    /*
     *	Ring Bus Setup
     */
    /* setup usual 0x34 stuff.. 0x36 may be chip specific */
    outw(0xC090, iobase + ESM_RING_BUS_DEST); /* direct sound, stereo */
    udelay(20);
    outw(0x3000, iobase + ESM_RING_BUS_CONTR_A); /* enable ringbus/serial */
    udelay(20);
    /*
     *	Reset the CODEC
     */
    snd_es1968_ac97_reset(chip);
    /* Ring Bus Control B */
    n = inl(iobase + ESM_RING_BUS_CONTR_B);
    n &= !RINGB_EN_SPDIF; /* SPDIF off */
    //w |= RINGB_EN_2CODEC;	/* enable 2nd codec */
    outl(n, iobase + ESM_RING_BUS_CONTR_B);
    /* Set hardware volume control registers to midpoints.
       We can tell which button was pushed based on how they change. */
    outb(0x88, iobase + 0x1c);
    outb(0x88, iobase + 0x1d);
    outb(0x88, iobase + 0x1e);
    outb(0x88, iobase + 0x1f);
    /* it appears some maestros (dell 7500) only work if these are set,
       regardless of whether we use the assp or not. */
    outb(0, iobase + ASSP_CONTROL_B);
    outb(3, iobase + ASSP_CONTROL_A); /* M: Reserved bits... */
    outb(0, iobase + ASSP_CONTROL_C); /* M: Disable ASSP, ASSP IRQ's and FM Port */
    /*
     * set up wavecache
     */
    i = 0;
    while i < 16 {
        /* Write 0 into the buffer area 0x1E0->1EF */
        outw((0x01E0 + i) as u16, iobase + WC_INDEX);
        outw(0x0000, iobase + WC_DATA);
        /* The 1.10 test program seem to write 0 into the buffer area
         * 0x1D0-0x1DF too.*/
        outw((0x01D0 + i) as u16, iobase + WC_INDEX);
        outw(0x0000, iobase + WC_DATA);
        i += 1;
    }
    wave_set_register(chip, IDR7_WAVE_ROMRAM, (wave_get_register(chip, IDR7_WAVE_ROMRAM) & 0xFF00) as u32);
    wave_set_register(chip, IDR7_WAVE_ROMRAM, (wave_get_register(chip, IDR7_WAVE_ROMRAM) | 0x100) as u32);
    wave_set_register(chip, IDR7_WAVE_ROMRAM, (wave_get_register(chip, IDR7_WAVE_ROMRAM) & !0x200) as u32);
    wave_set_register(chip, IDR7_WAVE_ROMRAM, (wave_get_register(chip, IDR7_WAVE_ROMRAM) as u32) | !0x400u32);
    maestro_write(chip, IDR2_CRAM_DATA, 0x0000);
    /* Now back to the DirectSound stuff */
    /* audio serial configuration.. ? */
    maestro_write(chip, 0x08, 0xB004);
    maestro_write(chip, 0x09, 0x001B);
    maestro_write(chip, 0x0A, 0x8000);
    maestro_write(chip, 0x0B, 0x3F37);
    maestro_write(chip, 0x0C, 0x0098);
    /* parallel in, has something to do with recording :) */
    maestro_write(chip, 0x0C, (maestro_read(chip, 0x0C) & !0xF000) | 0x8000);
    /* parallel out */
    maestro_write(chip, 0x0C, (maestro_read(chip, 0x0C) & !0x0F00) | 0x0500);
    maestro_write(chip, 0x0D, 0x7632);
    /* Wave cache control on - test off, sg off,
       enable, enable extra chans 1Mb */
    w = inw(iobase + WC_CONTROL);
    w &= !0xFA00; /* Seems to be reserved? I don't know */
    w |= 0xA000; /* reserved... I don't know */
    w &= !0x0200; /* Channels 56,57,58,59 as Extra Play,Rec Channel enable Seems to crash the Computer if enabled... */
    w |= 0x0100; /* Wave Cache Operation Enabled */
    w |= 0x0080; /* Channels 60/61 as Placback/Record enabled */
    w &= !0x0060; /* Clear Wavtable Size */
    w |= 0x0020; /* Wavetable Size : 1MB */
    /* Bit 4 is reserved */
    w &= !0x000C; /* DMA Stuff? I don't understand what the datasheet means */
    /* Bit 1 is reserved */
    w &= !0x0001; /* Test Mode off */
    outw(w, iobase + WC_CONTROL);
    /* Now clear the APU control ram */
    i = 0;
    while i < NR_APUS as c_int {
        w = 0;
        while w < NR_APU_REGS as u16 {
            apu_set_register(chip, i, w as c_int, 0);
            w += 1;
        }
        i += 1;
    }
}

/* Enable IRQ's */
unsafe fn snd_es1968_start_irq(chip: *mut es1968) {
    let mut w: u16;
    w = ESM_HIRQ_DSIE | ESM_HIRQ_HW_VOLUME;
    if !(*chip).rmidi.is_null() {
        w |= ESM_HIRQ_MPU401;
    }
    outb(w as u8, (*chip).io_port + 0x1A);
    outw(w, (*chip).io_port + ESM_PORT_HOST_IRQ);
}

/*
 * PM support
 */
unsafe extern "C" fn es1968_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let chip = (*card).private_data as *mut es1968;
    if (*chip).do_pm == 0 {
        return 0;
    }
    (*chip).in_suspend = 1;
    cancel_work_sync(&mut (*chip).hwvol_work);
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    snd_ac97_suspend((*chip).ac97);
    snd_es1968_bob_stop(chip);
    0
}

unsafe extern "C" fn es1968_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let chip = (*card).private_data as *mut es1968;
    let mut p: *mut list_head;
    if (*chip).do_pm == 0 {
        return 0;
    }
    snd_es1968_chip_init(chip);
    /* need to restore the base pointers.. */
    if (*chip).dma.addr != 0 {
        /* set PCMBAR */
        wave_set_register(chip, 0x01FC, (*chip).dma.addr >> 12);
    }
    snd_es1968_start_irq(chip);
    /* restore ac97 state */
    snd_ac97_resume((*chip).ac97);
    p = (*chip).substream_list.next;
    while p != &mut (*chip).substream_list {
        let es = list_entry_esschan(p);
        match (*es).mode {
            ESM_MODE_PLAY => snd_es1968_playback_setup(chip, es, (*(*es).substream).runtime),
            ESM_MODE_CAPTURE => snd_es1968_capture_setup(chip, es, (*(*es).substream).runtime),
            _ => {}
        }
        p = (*p).next;
    }
    /* start timer again */
    if (*chip).bobclient != 0 {
        snd_es1968_bob_start(chip);
    }
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    (*chip).in_suspend = 0;
    0
}

/* static DEFINE_SIMPLE_DEV_PM_OPS(es1968_pm, es1968_suspend, es1968_resume); */
static es1968_pm: c_int = 0;

/* #ifdef SUPPORT_JOYSTICK */
const JOYSTICK_ADDR: c_ulong = 0x200;
unsafe fn snd_es1968_create_gameport(chip: *mut es1968, dev: c_int) -> c_int {
    let gp: *mut gameport;
    let r: *mut resource;
    let mut val: u16 = 0;
    if !joystick[dev as usize] {
        return -ENODEV;
    }
    r = devm_request_region(&mut (*(*chip).pci).dev, JOYSTICK_ADDR, 8, b"ES1968 gameport\0".as_ptr() as *const c_char);
    if r.is_null() {
        return -EBUSY;
    }
    gp = gameport_allocate_port();
    (*chip).gameport = gp;
    if gp.is_null() {
        dev_err((*(*chip).card).dev, b"cannot allocate memory for gameport\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }
    pci_read_config_word((*chip).pci, ESM_LEGACY_AUDIO_CONTROL, &mut val);
    pci_write_config_word((*chip).pci, ESM_LEGACY_AUDIO_CONTROL, val | 0x04);
    gameport_set_name(gp, b"ES1968 Gameport\0".as_ptr() as *const c_char);
    gameport_set_phys(gp, b"pci%s/gameport0\0".as_ptr() as *const c_char, pci_name((*chip).pci));
    gameport_set_dev_parent(gp, &mut (*(*chip).pci).dev);
    (*gp).io = JOYSTICK_ADDR;
    gameport_register_port(gp);
    0
}

unsafe fn snd_es1968_free_gameport(chip: *mut es1968) {
    if !(*chip).gameport.is_null() {
        gameport_unregister_port((*chip).gameport);
        (*chip).gameport = ptr::null_mut();
    }
}
/* #else
 * static inline int snd_es1968_create_gameport(struct es1968 *chip, int dev) { return -ENOSYS; }
 * static inline void snd_es1968_free_gameport(struct es1968 *chip) { }
 * #endif
 */

/* #ifdef CONFIG_SND_ES1968_INPUT */
unsafe fn snd_es1968_input_register(chip: *mut es1968) -> c_int {
    let input_dev_: *mut input_dev;
    let err: c_int;
    input_dev_ = devm_input_allocate_device(&mut (*(*chip).pci).dev);
    if input_dev_.is_null() {
        return -ENOMEM;
    }
    snprintf((*chip).phys.as_mut_ptr(), (*chip).phys.len(), b"pci-%s/input0\0".as_ptr() as *const c_char, pci_name((*chip).pci));
    (*input_dev_).name = (*(*chip).card).driver.as_mut_ptr();
    (*input_dev_).phys = (*chip).phys.as_mut_ptr();
    (*input_dev_).id.bustype = BUS_PCI;
    (*input_dev_).id.vendor = (*(*chip).pci).vendor;
    (*input_dev_).id.product = (*(*chip).pci).device;
    (*input_dev_).dev.parent = &mut (*(*chip).pci).dev;
    __set_bit(EV_KEY, (*input_dev_).evbit.as_mut_ptr());
    __set_bit(KEY_MUTE, (*input_dev_).keybit.as_mut_ptr());
    __set_bit(KEY_VOLUMEDOWN, (*input_dev_).keybit.as_mut_ptr());
    __set_bit(KEY_VOLUMEUP, (*input_dev_).keybit.as_mut_ptr());
    err = input_register_device(input_dev_);
    if err != 0 {
        return err;
    }
    (*chip).input_dev = input_dev_;
    0
}
/* #endif CONFIG_SND_ES1968_INPUT */

/* #ifdef CONFIG_SND_ES1968_RADIO */
const GPIO_DATA: c_ulong = 0x60;
const IO_MASK: c_ulong = 4; /* mask register offset from GPIO_DATA bits 1=unmask write to given bit */
const IO_DIR: c_ulong = 8; /* direction register offset from GPIO_DATA bits 0/1=read/write direction */

/* GPIO to TEA575x maps */
#[repr(C)]
struct snd_es1968_tea575x_gpio {
    data: u8,
    clk: u8,
    wren: u8,
    most: u8,
    name: *mut c_char,
}

static snd_es1968_tea575x_gpios: [snd_es1968_tea575x_gpio; 2] = [
    snd_es1968_tea575x_gpio { data: 6, clk: 7, wren: 8, most: 9, name: b"SF64-PCE2\0".as_ptr() as *mut c_char },
    snd_es1968_tea575x_gpio { data: 7, clk: 8, wren: 6, most: 10, name: b"M56VAP\0".as_ptr() as *mut c_char },
];

unsafe fn get_tea575x_gpio(chip: *mut es1968) -> *const snd_es1968_tea575x_gpio {
    &snd_es1968_tea575x_gpios[(*chip).tea575x_tuner as usize]
}

unsafe extern "C" fn snd_es1968_tea575x_set_pins(tea: *mut snd_tea575x, pins: u8) {
    let chip = (*tea).private_data as *mut es1968;
    let gpio = *get_tea575x_gpio(chip);
    let mut val: u16 = 0;
    val |= if (pins & TEA575X_DATA) != 0 { 1 << gpio.data } else { 0 };
    val |= if (pins & TEA575X_CLK) != 0 { 1 << gpio.clk } else { 0 };
    val |= if (pins & TEA575X_WREN) != 0 { 1 << gpio.wren } else { 0 };
    outw(val, (*chip).io_port + GPIO_DATA);
}

unsafe extern "C" fn snd_es1968_tea575x_get_pins(tea: *mut snd_tea575x) -> u8 {
    let chip = (*tea).private_data as *mut es1968;
    let gpio = *get_tea575x_gpio(chip);
    let val: u16 = inw((*chip).io_port + GPIO_DATA);
    let mut ret: u8 = 0;
    if (val & (1 << gpio.data)) != 0 {
        ret |= TEA575X_DATA;
    }
    if (val & (1 << gpio.most)) != 0 {
        ret |= TEA575X_MOST;
    }
    ret
}

unsafe extern "C" fn snd_es1968_tea575x_set_direction(tea: *mut snd_tea575x, output: bool) {
    let chip = (*tea).private_data as *mut es1968;
    let io = (*chip).io_port + GPIO_DATA;
    let odir = inw(io + IO_DIR);
    let gpio = *get_tea575x_gpio(chip);
    if output {
        outw(!((1 << gpio.data) | (1 << gpio.clk) | (1 << gpio.wren)), io + IO_MASK);
        outw(odir | (1 << gpio.data) | (1 << gpio.clk) | (1 << gpio.wren), io + IO_DIR);
    } else {
        outw(!((1 << gpio.clk) | (1 << gpio.wren) | (1 << gpio.data) | (1 << gpio.most)), io + IO_MASK);
        outw((odir & !((1 << gpio.data) | (1 << gpio.most))) | (1 << gpio.clk) | (1 << gpio.wren), io + IO_DIR);
    }
}

static snd_es1968_tea_ops: snd_tea575x_ops = snd_tea575x_ops {
    set_pins: Some(snd_es1968_tea575x_set_pins),
    get_pins: Some(snd_es1968_tea575x_get_pins),
    set_direction: Some(snd_es1968_tea575x_set_direction),
};
/* #endif */

unsafe extern "C" fn snd_es1968_free(card: *mut snd_card) {
    let chip = (*card).private_data as *mut es1968;
    cancel_work_sync(&mut (*chip).hwvol_work);
    if (*chip).io_port != 0 {
        outw(1, (*chip).io_port + 0x04); /* clear WP interrupts */
        outw(0, (*chip).io_port + ESM_PORT_HOST_IRQ); /* disable IRQ */
    }
    /* #ifdef CONFIG_SND_ES1968_RADIO */
    snd_tea575x_exit(&mut (*chip).tea);
    v4l2_device_unregister(&mut (*chip).v4l2_dev);
    /* #endif */
    snd_es1968_free_gameport(chip);
}

#[repr(C)]
struct ess_device_list {
    type_: u16, /* chip type */
    vendor: u16, /* subsystem vendor id */
}

static pm_allowlist: [ess_device_list; 8] = [
    ess_device_list { type_: TYPE_MAESTRO2E as u16, vendor: 0x0e11 }, /* Compaq Armada */
    ess_device_list { type_: TYPE_MAESTRO2E as u16, vendor: 0x1028 },
    ess_device_list { type_: TYPE_MAESTRO2E as u16, vendor: 0x103c },
    ess_device_list { type_: TYPE_MAESTRO2E as u16, vendor: 0x1179 },
    ess_device_list { type_: TYPE_MAESTRO2E as u16, vendor: 0x14c0 }, /* HP omnibook 4150 */
    ess_device_list { type_: TYPE_MAESTRO2E as u16, vendor: 0x1558 },
    ess_device_list { type_: TYPE_MAESTRO2E as u16, vendor: 0x125d }, /* a PCI card, e.g. Terratec DMX */
    ess_device_list { type_: TYPE_MAESTRO2 as u16, vendor: 0x125d }, /* a PCI card, e.g. SF64-PCE2 */
];

static mpu_denylist: [ess_device_list; 1] = [
    ess_device_list { type_: TYPE_MAESTRO2 as u16, vendor: 0x125d },
];

unsafe fn snd_es1968_create(
    card: *mut snd_card,
    pci: *mut pci_dev,
    total_bufsize_: c_int,
    play_streams: c_int,
    capt_streams: c_int,
    chip_type: c_int,
    mut do_pm: c_int,
    radio_nr_: c_int,
) -> c_int {
    let chip = (*card).private_data as *mut es1968;
    let mut i: c_int;
    let mut err: c_int;
    /* enable PCI device */
    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }
    /* check, if we can restrict PCI DMA transfers to 28 bits */
    if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(28)) != 0 {
        dev_err((*card).dev, b"architecture does not support 28bit PCI busmaster DMA\n\0".as_ptr() as *const c_char);
        return -ENXIO;
    }
    /* Set Vars */
    (*chip).type_ = chip_type;
    spin_lock_init(&mut (*chip).reg_lock);
    spin_lock_init(&mut (*chip).substream_lock);
    INIT_LIST_HEAD(&mut (*chip).buf_list);
    INIT_LIST_HEAD(&mut (*chip).substream_list);
    mutex_init(&mut (*chip).memory_mutex);
    /* INIT_WORK(&chip->hwvol_work, es1968_update_hw_volume); */
    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).irq = -1;
    (*chip).total_bufsize = total_bufsize_; /* in bytes */
    (*chip).playback_streams = play_streams;
    (*chip).capture_streams = capt_streams;
    err = pcim_request_all_regions(pci, b"ESS Maestro\0".as_ptr() as *const c_char);
    if err < 0 {
        return err;
    }
    (*chip).io_port = pci_resource_start(pci, 0);
    if devm_request_irq(&mut (*pci).dev, (*pci).irq, snd_es1968_interrupt, IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void) != 0 {
        dev_err((*card).dev, b"unable to grab IRQ %d\n\0".as_ptr() as *const c_char, (*pci).irq);
        return -EBUSY;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    (*card).private_free = Some(snd_es1968_free);
    /* Clear Maestro_map */
    i = 0;
    while i < 32 {
        (*chip).maestro_map[i as usize] = 0;
        i += 1;
    }
    /* Clear Apu Map */
    i = 0;
    while i < NR_APUS as c_int {
        (*chip).apu[i as usize] = snd_enum_apu_type::ESM_APU_FREE as u8;
        i += 1;
    }
    /* just to be sure */
    pci_set_master(pci);
    if do_pm > 1 {
        /* disable power-management if not on the allowlist */
        let mut vend: u16 = 0;
        pci_read_config_word((*chip).pci, PCI_SUBSYSTEM_VENDOR_ID, &mut vend);
        i = 0;
        while i < ARRAY_SIZE(&pm_allowlist) as c_int {
            if (*chip).type_ == pm_allowlist[i as usize].type_ as c_int && vend == pm_allowlist[i as usize].vendor {
                do_pm = 1;
                break;
            }
            i += 1;
        }
        if do_pm > 1 {
            /* not matched; disabling pm */
            dev_info((*card).dev, b"not attempting power management.\n\0".as_ptr() as *const c_char);
            do_pm = 0;
        }
    }
    (*chip).do_pm = do_pm;
    snd_es1968_chip_init(chip);
    /* #ifdef CONFIG_SND_ES1968_RADIO */
    /* don't play with GPIOs on laptops */
    if (*(*chip).pci).subsystem_vendor == 0x125d {
        err = v4l2_device_register(&mut (*pci).dev, &mut (*chip).v4l2_dev);
        if err < 0 {
            return err;
        }
        (*chip).tea.v4l2_dev = &mut (*chip).v4l2_dev;
        (*chip).tea.private_data = chip as *mut c_void;
        (*chip).tea.radio_nr = radio_nr_;
        (*chip).tea.ops = &snd_es1968_tea_ops;
        sprintf((*chip).tea.bus_info.as_mut_ptr(), b"PCI:%s\0".as_ptr() as *const c_char, pci_name(pci));
        i = 0;
        while i < ARRAY_SIZE(&snd_es1968_tea575x_gpios) as c_int {
            (*chip).tea575x_tuner = i as c_uint;
            if snd_tea575x_init(&mut (*chip).tea, THIS_MODULE) == 0 {
                dev_info((*card).dev, b"detected TEA575x radio type %s\n\0".as_ptr() as *const c_char, (*get_tea575x_gpio(chip)).name);
                strscpy((*chip).tea.card.as_mut_ptr(), (*get_tea575x_gpio(chip)).name, (*chip).tea.card.len());
                break;
            }
            i += 1;
        }
    }
    /* #endif */
    0
}

unsafe fn __snd_es1968_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut es1968;
    let mut i: c_uint;
    let mut err: c_int;
    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }
    err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], THIS_MODULE, size_of::<es1968>(), &mut card);
    if err < 0 {
        return err;
    }
    chip = (*card).private_data as *mut es1968;
    if total_bufsize[dev as usize] < 128 {
        total_bufsize[dev as usize] = 128;
    }
    if total_bufsize[dev as usize] > 4096 {
        total_bufsize[dev as usize] = 4096;
    }
    err = snd_es1968_create(
        card,
        pci,
        total_bufsize[dev as usize] * 1024, /* in bytes */
        pcm_substreams_p[dev as usize],
        pcm_substreams_c[dev as usize],
        (*pci_id).driver_data as c_int,
        use_pm[dev as usize],
        radio_nr[dev as usize],
    );
    if err < 0 {
        return err;
    }
    match (*chip).type_ {
        TYPE_MAESTRO2E => {
            strscpy((*card).driver.as_mut_ptr(), b"ES1978\0".as_ptr() as *const c_char, (*card).driver.len());
            strscpy((*card).shortname.as_mut_ptr(), b"ESS ES1978 (Maestro 2E)\0".as_ptr() as *const c_char, (*card).shortname.len());
        }
        TYPE_MAESTRO2 => {
            strscpy((*card).driver.as_mut_ptr(), b"ES1968\0".as_ptr() as *const c_char, (*card).driver.len());
            strscpy((*card).shortname.as_mut_ptr(), b"ESS ES1968 (Maestro 2)\0".as_ptr() as *const c_char, (*card).shortname.len());
        }
        TYPE_MAESTRO => {
            strscpy((*card).driver.as_mut_ptr(), b"ESM1\0".as_ptr() as *const c_char, (*card).driver.len());
            strscpy((*card).shortname.as_mut_ptr(), b"ESS Maestro 1\0".as_ptr() as *const c_char, (*card).shortname.len());
        }
        _ => {}
    }
    err = snd_es1968_pcm(chip, 0);
    if err < 0 {
        return err;
    }
    err = snd_es1968_mixer(chip);
    if err < 0 {
        return err;
    }
    if enable_mpu[dev as usize] == 2 {
        /* check the deny list */
        let mut vend: u16 = 0;
        pci_read_config_word((*chip).pci, PCI_SUBSYSTEM_VENDOR_ID, &mut vend);
        i = 0;
        while i < ARRAY_SIZE(&mpu_denylist) as c_uint {
            if (*chip).type_ == mpu_denylist[i as usize].type_ as c_int && vend == mpu_denylist[i as usize].vendor {
                enable_mpu[dev as usize] = 0;
                break;
            }
            i += 1;
        }
    }
    if enable_mpu[dev as usize] != 0 {
        err = snd_mpu401_uart_new(
            card,
            0,
            MPU401_HW_MPU401,
            (*chip).io_port + ESM_MPU401_PORT,
            MPU401_INFO_INTEGRATED | MPU401_INFO_IRQ_HOOK,
            -1,
            &mut (*chip).rmidi,
        );
        if err < 0 {
            dev_warn((*card).dev, b"skipping MPU-401 MIDI support..\n\0".as_ptr() as *const c_char);
        }
    }
    snd_es1968_create_gameport(chip, dev);
    /* #ifdef CONFIG_SND_ES1968_INPUT */
    err = snd_es1968_input_register(chip);
    if err != 0 {
        dev_warn((*card).dev, b"Input device registration failed with error %i\0".as_ptr() as *const c_char, err);
    }
    /* #endif */
    snd_es1968_start_irq(chip);
    (*chip).clock = clock[dev as usize] as c_uint;
    if (*chip).clock == 0 {
        es1968_measure_clock(chip);
    }
    sprintf((*card).longname.as_mut_ptr(), b"%s at 0x%lx, irq %i\0".as_ptr() as *const c_char, (*card).shortname.as_ptr(), (*chip).io_port, (*chip).irq);
    err = snd_card_register(card);
    if err < 0 {
        return err;
    }
    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}

unsafe extern "C" fn snd_es1968_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_es1968_probe(pci, pci_id))
}

static mut es1968_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_es1968_ids.as_ptr(),
    probe: Some(snd_es1968_probe),
    driver: pci_driver_inner {
        pm: &es1968_pm as *const _ as *const c_void,
    },
};

/* module_pci_driver(es1968_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
