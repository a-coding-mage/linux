// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  card-als4000.c - driver for Avance Logic ALS4000 based soundcards.
 *  Copyright (C) 2000 by Bart Hartgers <bart@etpmod.phys.tue.nl>,
 *			  Jaroslav Kysela <perex@perex.cz>
 *  Copyright (C) 2002, 2008 by Andreas Mohr <hw7oshyuv3001@sneakemail.com>
 *
 *  Framework borrowed from Massimo Piccioni's card-als100.c.
 *
 * NOTES
 *
 *  Since Avance does not provide any meaningful documentation, and I
 *  bought an ALS4000 based soundcard, I was forced to base this driver
 *  on reverse engineering.
 *
 *  Note: this is no longer true (thank you!):
 *  pretty verbose chip docu (ALS4000a.PDF) can be found on the ALSA web site.
 *  Page numbers stated anywhere below with the "SPECS_PAGE:" tag
 *  refer to: ALS4000a.PDF specs Ver 1.0, May 28th, 1998.
 *
 *  The ALS4000 seems to be the PCI-cousin of the ALS100. It contains an
 *  ALS100-like SB DSP/mixer, an OPL3 synth, a MPU401 and a gameport
 *  interface. These subsystems can be mapped into ISA io-port space,
 *  using the PCI-interface. In addition, the PCI-bit provides DMA and IRQ
 *  services to the subsystems.
 *
 * While ALS4000 is very similar to a SoundBlaster, the differences in
 * DMA and capturing require more changes to the SoundBlaster than
 * desirable, so I made this separate driver.
 *
 * The ALS4000 can do real full duplex playback/capture.
 *
 * FMDAC:
 * - 0x4f -> port 0x14
 * - port 0x15 |= 1
 *
 * Enable/disable 3D sound:
 * - 0x50 -> port 0x14
 * - change bit 6 (0x40) of port 0x15
 *
 * Set QSound:
 * - 0xdb -> port 0x14
 * - set port 0x15:
 *   0x3e (mode 3), 0x3c (mode 2), 0x3a (mode 1), 0x38 (mode 0)
 *
 * Set KSound:
 * - value -> some port 0x0c0d
 *
 * ToDo:
 * - by default, don't enable legacy game and use PCI game I/O
 * - power management? (card can do voice wakeup according to datasheet!!)
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u8 = ::core::primitive::u8;
type u16 = ::core::primitive::u16;
type u32 = ::core::primitive::u32;
type bool_ = bool;
type dma_addr_t = c_ulong;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;

const SNDRV_CARDS: usize = 32;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [core::ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool_; SNDRV_CARDS] = [true; SNDRV_CARDS];

const SUPPORT_JOYSTICK: bool = true; /* #if IS_REACHABLE(CONFIG_GAMEPORT) */

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool_; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP; /* Enable this card */
static mut joystick_port: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_int,
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_sb {
    pub alt_port: c_ulong,
    pub mode: c_int,
    pub playback_format: c_int,
    pub capture_format: c_int,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub rmidi: *mut snd_rawmidi,
    pub mpu_port: c_ulong,
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub pcm: *mut snd_pcm,
    pub irq: c_int,
    pub reg_lock: spinlock_t,
    pub mixer_lock: spinlock_t,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub format: c_int,
    pub channels: c_uint,
    pub rate: c_uint,
    pub dma_addr: dma_addr_t,
}
#[repr(C)]
pub struct snd_rawmidi {
    pub private_data: *mut c_void,
}
#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
}
#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gameport {
    pub io: c_int,
}
#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card_als4000 {
    /* most frequent access first */
    pub iobase: c_ulong,
    pub pci: *mut pci_dev,
    pub chip: *mut snd_sb,
    pub gameport: *mut gameport,
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: c_ulong,
}

const fn PCI_DEVICE(vendor: u32, device: u32) -> pci_device_id {
    pci_device_id {
        vendor,
        device,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }
}

static snd_als4000_ids: [pci_device_id; 2] = [
    PCI_DEVICE(0x4005, 0x4000), /* ALS4000 */
    pci_device_id { vendor: 0, device: 0, subvendor: 0, subdevice: 0, class: 0, class_mask: 0, driver_data: 0 },
];

#[repr(u32)]
#[derive(Copy, Clone)]
enum als4k_iobase_t {
    /* IOx: B == Byte, W = Word, D = DWord; SPECS_PAGE: 37 */
    ALS4K_IOD_00_AC97_ACCESS = 0x00,
    ALS4K_IOW_04_AC97_READ = 0x04,
    ALS4K_IOB_06_AC97_STATUS = 0x06,
    ALS4K_IOB_07_IRQSTATUS = 0x07,
    ALS4K_IOD_08_GCR_DATA = 0x08,
    ALS4K_IOB_0C_GCR_INDEX = 0x0c,
    ALS4K_IOB_0E_IRQTYPE_SB_CR1E_MPU = 0x0e,
    ALS4K_IOB_10_ADLIB_ADDR0 = 0x10,
    ALS4K_IOB_11_ADLIB_ADDR1 = 0x11,
    ALS4K_IOB_12_ADLIB_ADDR2 = 0x12,
    ALS4K_IOB_13_ADLIB_ADDR3 = 0x13,
    ALS4K_IOB_14_MIXER_INDEX = 0x14,
    ALS4K_IOB_15_MIXER_DATA = 0x15,
    ALS4K_IOB_16_ESP_RESET = 0x16,
    ALS4K_IOB_16_ACK_FOR_CR1E = 0x16, /* 2nd function */
    ALS4K_IOB_18_OPL_ADDR0 = 0x18,
    ALS4K_IOB_19_OPL_ADDR1 = 0x19,
    ALS4K_IOB_1A_ESP_RD_DATA = 0x1a,
    ALS4K_IOB_1C_ESP_CMD_DATA = 0x1c,
    ALS4K_IOB_1C_ESP_WR_STATUS = 0x1c, /* 2nd function */
    ALS4K_IOB_1E_ESP_RD_STATUS8 = 0x1e,
    ALS4K_IOB_1F_ESP_RD_STATUS16 = 0x1f,
    ALS4K_IOB_20_ESP_GAMEPORT_200 = 0x20,
    ALS4K_IOB_21_ESP_GAMEPORT_201 = 0x21,
    ALS4K_IOB_30_MIDI_DATA = 0x30,
    ALS4K_IOB_31_MIDI_STATUS = 0x31,
    ALS4K_IOB_31_MIDI_COMMAND = 0x31, /* 2nd function */
}

const ALS4K_IOB_0E_MPU_IRQ: u32 = 0x10;
const ALS4K_IOB_0E_CR1E_IRQ: u32 = 0x40;
const ALS4K_IOB_0E_SB_DMA_IRQ: u32 = 0x80;

#[repr(u32)]
#[derive(Copy, Clone)]
enum als4k_gcr_t { /* all registers 32bit wide; SPECS_PAGE: 38 to 42 */
    ALS4K_GCR8C_MISC_CTRL = 0x8c,
    ALS4K_GCR90_TEST_MODE_REG = 0x90,
    ALS4K_GCR91_DMA0_ADDR = 0x91,
    ALS4K_GCR92_DMA0_MODE_COUNT = 0x92,
    ALS4K_GCR93_DMA1_ADDR = 0x93,
    ALS4K_GCR94_DMA1_MODE_COUNT = 0x94,
    ALS4K_GCR95_DMA3_ADDR = 0x95,
    ALS4K_GCR96_DMA3_MODE_COUNT = 0x96,
    ALS4K_GCR99_DMA_EMULATION_CTRL = 0x99,
    ALS4K_GCRA0_FIFO1_CURRENT_ADDR = 0xa0,
    ALS4K_GCRA1_FIFO1_STATUS_BYTECOUNT = 0xa1,
    ALS4K_GCRA2_FIFO2_PCIADDR = 0xa2,
    ALS4K_GCRA3_FIFO2_COUNT = 0xa3,
    ALS4K_GCRA4_FIFO2_CURRENT_ADDR = 0xa4,
    ALS4K_GCRA5_FIFO1_STATUS_BYTECOUNT = 0xa5,
    ALS4K_GCRA6_PM_CTRL = 0xa6,
    ALS4K_GCRA7_PCI_ACCESS_STORAGE = 0xa7,
    ALS4K_GCRA8_LEGACY_CFG1 = 0xa8,
    ALS4K_GCRA9_LEGACY_CFG2 = 0xa9,
    ALS4K_GCRFF_DUMMY_SCRATCH = 0xff,
}

const ALS4K_GCR8C_IRQ_MASK_CTRL_ENABLE: u32 = 0x8000;
const ALS4K_GCR8C_CHIP_REV_MASK: u32 = 0xf0000;

unsafe fn snd_als4k_iobase_writeb(iobase: c_ulong, reg: als4k_iobase_t, val: u8) {
    outb(val, iobase.wrapping_add(reg as c_ulong));
}

unsafe fn snd_als4k_iobase_writel(iobase: c_ulong, reg: als4k_iobase_t, val: u32) {
    outl(val, iobase.wrapping_add(reg as c_ulong));
}

unsafe fn snd_als4k_iobase_readb(iobase: c_ulong, reg: als4k_iobase_t) -> u8 {
    inb(iobase.wrapping_add(reg as c_ulong))
}

unsafe fn snd_als4k_iobase_readl(iobase: c_ulong, reg: als4k_iobase_t) -> u32 {
    inl(iobase.wrapping_add(reg as c_ulong))
}

unsafe fn snd_als4k_gcr_write_addr(iobase: c_ulong, reg: als4k_gcr_t, val: u32) {
    snd_als4k_iobase_writeb(iobase, als4k_iobase_t::ALS4K_IOB_0C_GCR_INDEX, reg as u8);
    snd_als4k_iobase_writel(iobase, als4k_iobase_t::ALS4K_IOD_08_GCR_DATA, val);
}

unsafe fn snd_als4k_gcr_write(sb: *mut snd_sb, reg: als4k_gcr_t, val: u32) {
    snd_als4k_gcr_write_addr((*sb).alt_port, reg, val);
}

unsafe fn snd_als4k_gcr_read_addr(iobase: c_ulong, reg: als4k_gcr_t) -> u32 {
    /* SPECS_PAGE: 37/38 */
    snd_als4k_iobase_writeb(iobase, als4k_iobase_t::ALS4K_IOB_0C_GCR_INDEX, reg as u8);
    snd_als4k_iobase_readl(iobase, als4k_iobase_t::ALS4K_IOD_08_GCR_DATA)
}

unsafe fn snd_als4k_gcr_read(sb: *mut snd_sb, reg: als4k_gcr_t) -> u32 {
    snd_als4k_gcr_read_addr((*sb).alt_port, reg)
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum als4k_cr_t { /* all registers 8bit wide; SPECS_PAGE: 20 to 23 */
    ALS4K_CR0_SB_CONFIG = 0x00,
    ALS4K_CR2_MISC_CONTROL = 0x02,
    ALS4K_CR3_CONFIGURATION = 0x03,
    ALS4K_CR17_FIFO_STATUS = 0x17,
    ALS4K_CR18_ESP_MAJOR_VERSION = 0x18,
    ALS4K_CR19_ESP_MINOR_VERSION = 0x19,
    ALS4K_CR1A_MPU401_UART_MODE_CONTROL = 0x1a,
    ALS4K_CR1C_FIFO2_BLOCK_LENGTH_LO = 0x1c,
    ALS4K_CR1D_FIFO2_BLOCK_LENGTH_HI = 0x1d,
    ALS4K_CR1E_FIFO2_CONTROL = 0x1e, /* secondary PCM FIFO (recording) */
    ALS4K_CR3A_MISC_CONTROL = 0x3a,
    ALS4K_CR3B_CRC32_BYTE0 = 0x3b, /* for testing, activate via CR3A */
    ALS4K_CR3C_CRC32_BYTE1 = 0x3c,
    ALS4K_CR3D_CRC32_BYTE2 = 0x3d,
    ALS4K_CR3E_CRC32_BYTE3 = 0x3e,
}

const ALS4K_CR0_DMA_CONTIN_MODE_CTRL: u32 = 0x02; /* IRQ/FIFO controlled for 0/1 */
const ALS4K_CR0_DMA_90H_MODE_CTRL: u32 = 0x04; /* IRQ/FIFO controlled for 0/1 */
const ALS4K_CR0_MX80_81_REG_WRITE_ENABLE: u32 = 0x80;

unsafe fn snd_als4_cr_write(chip: *mut snd_sb, reg: als4k_cr_t, data: u8) {
    /* Control Register is reg | 0xc0 (bit 7, 6 set) on sbmixer_index
     * NOTE: assumes chip->mixer_lock to be locked externally already!
     * SPECS_PAGE: 6 */
    snd_sbmixer_write(chip, (reg as u8) | 0xc0, data);
}

unsafe fn snd_als4_cr_read(chip: *mut snd_sb, reg: als4k_cr_t) -> u8 {
    /* NOTE: assumes chip->mixer_lock to be locked externally already! */
    snd_sbmixer_read(chip, (reg as u8) | 0xc0)
}

unsafe fn snd_als4000_set_rate(chip: *mut snd_sb, rate: c_uint) {
    if ((*chip).mode & SB_RATE_LOCK) == 0 {
        snd_sbdsp_command(chip, SB_DSP_SAMPLE_RATE_OUT);
        snd_sbdsp_command(chip, (rate >> 8) as c_int);
        snd_sbdsp_command(chip, rate as c_int);
    }
}

unsafe fn snd_als4000_set_capture_dma(chip: *mut snd_sb, addr: dma_addr_t, size: c_uint) {
    /* SPECS_PAGE: 40 */
    snd_als4k_gcr_write(chip, als4k_gcr_t::ALS4K_GCRA2_FIFO2_PCIADDR, addr as u32);
    snd_als4k_gcr_write(chip, als4k_gcr_t::ALS4K_GCRA3_FIFO2_COUNT, size.wrapping_sub(1));
}

unsafe fn snd_als4000_set_playback_dma(chip: *mut snd_sb, addr: dma_addr_t, size: c_uint) {
    /* SPECS_PAGE: 38 */
    snd_als4k_gcr_write(chip, als4k_gcr_t::ALS4K_GCR91_DMA0_ADDR, addr as u32);
    snd_als4k_gcr_write(chip, als4k_gcr_t::ALS4K_GCR92_DMA0_MODE_COUNT, size.wrapping_sub(1) | 0x180000);
}

const ALS4000_FORMAT_SIGNED: c_int = 1 << 0;
const ALS4000_FORMAT_16BIT: c_int = 1 << 1;
const ALS4000_FORMAT_STEREO: c_int = 1 << 2;

unsafe fn snd_als4000_get_format(runtime: *mut snd_pcm_runtime) -> c_int {
    let mut result: c_int = 0;
    if snd_pcm_format_signed((*runtime).format) != 0 {
        result |= ALS4000_FORMAT_SIGNED;
    }
    if snd_pcm_format_physical_width((*runtime).format) == 16 {
        result |= ALS4000_FORMAT_16BIT;
    }
    if (*runtime).channels > 1 {
        result |= ALS4000_FORMAT_STEREO;
    }
    result
}

/* structure for setting up playback */
#[repr(C)]
#[derive(Copy, Clone)]
struct playback_cmd_val {
    dsp_cmd: u8,
    dma_on: u8,
    dma_off: u8,
    format: u8,
}

static playback_cmd_vals: [playback_cmd_val; 8] = [
    /* ALS4000_FORMAT_U8_MONO */
    playback_cmd_val { dsp_cmd: SB_DSP4_OUT8_AI as u8, dma_on: SB_DSP_DMA8_ON as u8, dma_off: SB_DSP_DMA8_OFF as u8, format: SB_DSP4_MODE_UNS_MONO as u8 },
    /* ALS4000_FORMAT_S8_MONO */
    playback_cmd_val { dsp_cmd: SB_DSP4_OUT8_AI as u8, dma_on: SB_DSP_DMA8_ON as u8, dma_off: SB_DSP_DMA8_OFF as u8, format: SB_DSP4_MODE_SIGN_MONO as u8 },
    /* ALS4000_FORMAT_U16L_MONO */
    playback_cmd_val { dsp_cmd: SB_DSP4_OUT16_AI as u8, dma_on: SB_DSP_DMA16_ON as u8, dma_off: SB_DSP_DMA16_OFF as u8, format: SB_DSP4_MODE_UNS_MONO as u8 },
    /* ALS4000_FORMAT_S16L_MONO */
    playback_cmd_val { dsp_cmd: SB_DSP4_OUT16_AI as u8, dma_on: SB_DSP_DMA16_ON as u8, dma_off: SB_DSP_DMA16_OFF as u8, format: SB_DSP4_MODE_SIGN_MONO as u8 },
    /* ALS4000_FORMAT_U8_STEREO */
    playback_cmd_val { dsp_cmd: SB_DSP4_OUT8_AI as u8, dma_on: SB_DSP_DMA8_ON as u8, dma_off: SB_DSP_DMA8_OFF as u8, format: SB_DSP4_MODE_UNS_STEREO as u8 },
    /* ALS4000_FORMAT_S8_STEREO */
    playback_cmd_val { dsp_cmd: SB_DSP4_OUT8_AI as u8, dma_on: SB_DSP_DMA8_ON as u8, dma_off: SB_DSP_DMA8_OFF as u8, format: SB_DSP4_MODE_SIGN_STEREO as u8 },
    /* ALS4000_FORMAT_U16L_STEREO */
    playback_cmd_val { dsp_cmd: SB_DSP4_OUT16_AI as u8, dma_on: SB_DSP_DMA16_ON as u8, dma_off: SB_DSP_DMA16_OFF as u8, format: SB_DSP4_MODE_UNS_STEREO as u8 },
    /* ALS4000_FORMAT_S16L_STEREO */
    playback_cmd_val { dsp_cmd: SB_DSP4_OUT16_AI as u8, dma_on: SB_DSP_DMA16_ON as u8, dma_off: SB_DSP_DMA16_OFF as u8, format: SB_DSP4_MODE_SIGN_STEREO as u8 },
];

unsafe fn playback_cmd(chip: *mut snd_sb) -> playback_cmd_val {
    playback_cmd_vals[(*chip).playback_format as usize]
}

/* structure for setting up capture */
const CMD_WIDTH8: u8 = 0x04;
const CMD_SIGNED: u8 = 0x10;
const CMD_MONO: u8 = 0x80;
const CMD_STEREO: u8 = 0xA0;
static capture_cmd_vals: [u8; 8] = [
    CMD_WIDTH8 | CMD_MONO,                 /* ALS4000_FORMAT_U8_MONO */
    CMD_WIDTH8 | CMD_SIGNED | CMD_MONO,    /* ALS4000_FORMAT_S8_MONO */
    CMD_MONO,                              /* ALS4000_FORMAT_U16L_MONO */
    CMD_SIGNED | CMD_MONO,                 /* ALS4000_FORMAT_S16L_MONO */
    CMD_WIDTH8 | CMD_STEREO,               /* ALS4000_FORMAT_U8_STEREO */
    CMD_WIDTH8 | CMD_SIGNED | CMD_STEREO,  /* ALS4000_FORMAT_S8_STEREO */
    CMD_STEREO,                            /* ALS4000_FORMAT_U16L_STEREO */
    CMD_SIGNED | CMD_STEREO,               /* ALS4000_FORMAT_S16L_STEREO */
];

unsafe fn capture_cmd(chip: *mut snd_sb) -> u8 {
    capture_cmd_vals[(*chip).capture_format as usize]
}

unsafe extern "C" fn snd_als4000_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let size: c_ulong;
    let mut count: c_uint;

    (*chip).capture_format = snd_als4000_get_format(runtime);

    size = snd_pcm_lib_buffer_bytes(substream);
    count = snd_pcm_lib_period_bytes(substream) as c_uint;

    if ((*chip).capture_format & ALS4000_FORMAT_16BIT) != 0 {
        count >>= 1;
    }
    count = count.wrapping_sub(1);

    spin_lock_irq(&mut (*chip).reg_lock);
    snd_als4000_set_rate(chip, (*runtime).rate);
    snd_als4000_set_capture_dma(chip, (*runtime).dma_addr, size as c_uint);
    spin_unlock_irq(&mut (*chip).reg_lock);

    spin_lock_irq(&mut (*chip).mixer_lock);
    snd_als4_cr_write(chip, als4k_cr_t::ALS4K_CR1C_FIFO2_BLOCK_LENGTH_LO, (count & 0xff) as u8);
    snd_als4_cr_write(chip, als4k_cr_t::ALS4K_CR1D_FIFO2_BLOCK_LENGTH_HI, (count >> 8) as u8);
    spin_unlock_irq(&mut (*chip).mixer_lock);

    0
}

unsafe extern "C" fn snd_als4000_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let size: c_ulong;
    let mut count: c_uint;

    (*chip).playback_format = snd_als4000_get_format(runtime);

    size = snd_pcm_lib_buffer_bytes(substream);
    count = snd_pcm_lib_period_bytes(substream) as c_uint;

    if ((*chip).playback_format & ALS4000_FORMAT_16BIT) != 0 {
        count >>= 1;
    }
    count = count.wrapping_sub(1);

    /* FIXME: from second playback on, there's a lot more clicks and pops
     * involved here than on first playback. Fiddling with
     * tons of different settings didn't help (DMA, speaker on/off,
     * reordering, ...). Something seems to get enabled on playback
     * that I haven't found out how to disable again, which then causes
     * the switching pops to reach the speakers the next time here. */
    spin_lock_irq(&mut (*chip).reg_lock);
    snd_als4000_set_rate(chip, (*runtime).rate);
    snd_als4000_set_playback_dma(chip, (*runtime).dma_addr, size as c_uint);

    /* SPEAKER_ON not needed, since dma_on seems to also enable speaker */
    /* snd_sbdsp_command(chip, SB_DSP_SPEAKER_ON); */
    snd_sbdsp_command(chip, playback_cmd(chip).dsp_cmd as c_int);
    snd_sbdsp_command(chip, playback_cmd(chip).format as c_int);
    snd_sbdsp_command(chip, (count & 0xff) as c_int);
    snd_sbdsp_command(chip, (count >> 8) as c_int);
    snd_sbdsp_command(chip, playback_cmd(chip).dma_off as c_int);
    spin_unlock_irq(&mut (*chip).reg_lock);

    0
}

unsafe extern "C" fn snd_als4000_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut result: c_int = 0;

    spin_lock(&mut (*chip).reg_lock);
    spin_lock(&mut (*chip).mixer_lock);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            (*chip).mode |= SB_RATE_LOCK_CAPTURE;
            snd_als4_cr_write(chip, als4k_cr_t::ALS4K_CR1E_FIFO2_CONTROL, capture_cmd(chip));
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            (*chip).mode &= !SB_RATE_LOCK_CAPTURE;
            snd_als4_cr_write(chip, als4k_cr_t::ALS4K_CR1E_FIFO2_CONTROL, capture_cmd(chip));
        }
        _ => {
            result = -EINVAL;
        }
    }
    spin_unlock(&mut (*chip).mixer_lock);
    spin_unlock(&mut (*chip).reg_lock);
    result
}

unsafe extern "C" fn snd_als4000_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut result: c_int = 0;

    spin_lock(&mut (*chip).reg_lock);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            (*chip).mode |= SB_RATE_LOCK_PLAYBACK;
            snd_sbdsp_command(chip, playback_cmd(chip).dma_on as c_int);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            snd_sbdsp_command(chip, playback_cmd(chip).dma_off as c_int);
            (*chip).mode &= !SB_RATE_LOCK_PLAYBACK;
        }
        _ => {
            result = -EINVAL;
        }
    }
    spin_unlock(&mut (*chip).reg_lock);
    result
}

unsafe extern "C" fn snd_als4000_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let mut result: c_uint;

    spin_lock(&mut (*chip).reg_lock);
    result = snd_als4k_gcr_read(chip, als4k_gcr_t::ALS4K_GCRA4_FIFO2_CURRENT_ADDR);
    spin_unlock(&mut (*chip).reg_lock);
    result &= 0xffff;
    bytes_to_frames((*substream).runtime, result as c_ulong)
}

unsafe extern "C" fn snd_als4000_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let mut result: c_uint;

    spin_lock(&mut (*chip).reg_lock);
    result = snd_als4k_gcr_read(chip, als4k_gcr_t::ALS4K_GCRA0_FIFO1_CURRENT_ADDR);
    spin_unlock(&mut (*chip).reg_lock);
    result &= 0xffff;
    bytes_to_frames((*substream).runtime, result as c_ulong)
}

/* FIXME: this IRQ routine doesn't really support IRQ sharing (we always
 * return IRQ_HANDLED no matter whether we actually had an IRQ flag or not).
 * ALS4000a.PDF writes that while ACKing IRQ in PCI block will *not* ACK
 * the IRQ in the SB core, ACKing IRQ in SB block *will* ACK the PCI IRQ
 * register (alt_port + ALS4K_IOB_0E_IRQTYPE_SB_CR1E_MPU). Probably something
 * could be optimized here to query/write one register only...
 * And even if both registers need to be queried, then there's still the
 * question of whether it's actually correct to ACK PCI IRQ before reading
 * SB IRQ like we do now, since ALS4000a.PDF mentions that PCI IRQ will *clear*
 * SB IRQ status.
 * (hmm, SPECS_PAGE: 38 mentions it the other way around!)
 * And do we *really* need the lock here for *reading* SB_DSP4_IRQSTATUS??
 * */
unsafe extern "C" fn snd_als4000_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut snd_sb;
    let pci_irqstatus: c_uint;
    let sb_irqstatus: c_uint;

    /* find out which bit of the ALS4000 PCI block produced the interrupt,
       SPECS_PAGE: 38, 5 */
    pci_irqstatus = snd_als4k_iobase_readb((*chip).alt_port, als4k_iobase_t::ALS4K_IOB_0E_IRQTYPE_SB_CR1E_MPU) as c_uint;
    if (pci_irqstatus & ALS4K_IOB_0E_SB_DMA_IRQ) != 0 && !(*chip).playback_substream.is_null() {
        /* playback */
        snd_pcm_period_elapsed((*chip).playback_substream);
    }
    if (pci_irqstatus & ALS4K_IOB_0E_CR1E_IRQ) != 0 && !(*chip).capture_substream.is_null() {
        /* capturing */
        snd_pcm_period_elapsed((*chip).capture_substream);
    }
    if (pci_irqstatus & ALS4K_IOB_0E_MPU_IRQ) != 0 && !(*chip).rmidi.is_null() {
        /* MPU401 interrupt */
        snd_mpu401_uart_interrupt(irq, (*(*chip).rmidi).private_data);
    }
    /* ACK the PCI block IRQ */
    snd_als4k_iobase_writeb((*chip).alt_port, als4k_iobase_t::ALS4K_IOB_0E_IRQTYPE_SB_CR1E_MPU, pci_irqstatus as u8);

    spin_lock(&mut (*chip).mixer_lock);
    /* SPECS_PAGE: 20 */
    sb_irqstatus = snd_sbmixer_read(chip, SB_DSP4_IRQSTATUS) as c_uint;
    spin_unlock(&mut (*chip).mixer_lock);

    if (sb_irqstatus & SB_IRQTYPE_8BIT) != 0 {
        snd_sb_ack_8bit(chip);
    }
    if (sb_irqstatus & SB_IRQTYPE_16BIT) != 0 {
        snd_sb_ack_16bit(chip);
    }
    if (sb_irqstatus & SB_IRQTYPE_MPUIN) != 0 {
        inb((*chip).mpu_port);
    }
    if (sb_irqstatus & ALS4K_IRQTYPE_CR1E_DMA) != 0 {
        snd_als4k_iobase_readb((*chip).alt_port, als4k_iobase_t::ALS4K_IOB_16_ACK_FOR_CR1E);
    }

    /* dev_dbg(chip->card->dev, "als4000: irq 0x%04x 0x%04x\n",
                     pci_irqstatus, sb_irqstatus); */

    /* only ack the things we actually handled above */
    IRQ_RETVAL(((pci_irqstatus & (ALS4K_IOB_0E_SB_DMA_IRQ | ALS4K_IOB_0E_CR1E_IRQ | ALS4K_IOB_0E_MPU_IRQ)) != 0
        || (sb_irqstatus & (SB_IRQTYPE_8BIT | SB_IRQTYPE_16BIT | SB_IRQTYPE_MPUIN | ALS4K_IRQTYPE_CR1E_DMA)) != 0) as c_int)
}

/*****************************************************************/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: c_ulong,
    pub period_bytes_min: c_ulong,
    pub period_bytes_max: c_ulong,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: c_uint,
}

static snd_als4000_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE, /* formats */
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 65536,
    period_bytes_min: 64,
    period_bytes_max: 65536,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

static snd_als4000_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE, /* formats */
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 65536,
    period_bytes_min: 64,
    period_bytes_max: 65536,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

/*****************************************************************/

unsafe extern "C" fn snd_als4000_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;

    (*chip).playback_substream = substream;
    (*runtime).hw = snd_als4000_playback;
    0
}

unsafe extern "C" fn snd_als4000_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);

    (*chip).playback_substream = core::ptr::null_mut();
    0
}

unsafe extern "C" fn snd_als4000_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;

    (*chip).capture_substream = substream;
    (*runtime).hw = snd_als4000_capture;
    0
}

unsafe extern "C" fn snd_als4000_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);

    (*chip).capture_substream = core::ptr::null_mut();
    0
}

/******************************************************************/

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

static snd_als4000_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_als4000_playback_open),
    close: Some(snd_als4000_playback_close),
    prepare: Some(snd_als4000_playback_prepare),
    trigger: Some(snd_als4000_playback_trigger),
    pointer: Some(snd_als4000_playback_pointer),
};

static snd_als4000_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_als4000_capture_open),
    close: Some(snd_als4000_capture_close),
    prepare: Some(snd_als4000_capture_prepare),
    trigger: Some(snd_als4000_capture_trigger),
    pointer: Some(snd_als4000_capture_pointer),
};

unsafe fn snd_als4000_pcm(chip: *mut snd_sb, device: c_int) -> c_int {
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut err: c_int;

    err = snd_pcm_new((*chip).card, c"ALS4000 DSP".as_ptr(), device, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }
    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = SNDRV_PCM_INFO_JOINT_DUPLEX;
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_als4000_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_als4000_capture_ops);

    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, 64 * 1024, 64 * 1024);

    (*chip).pcm = pcm;

    0
}

/******************************************************************/

unsafe fn snd_als4000_set_addr(iobase: c_ulong, sb_io: c_uint, mpu_io: c_uint, opl_io: c_uint, game_io: c_uint) {
    let mut cfg1: u32 = 0;
    let mut cfg2: u32 = 0;

    if mpu_io > 0 {
        cfg2 |= (mpu_io | 1) << 16;
    }
    if sb_io > 0 {
        cfg2 |= sb_io | 1;
    }
    if game_io > 0 {
        cfg1 |= (game_io | 1) << 16;
    }
    if opl_io > 0 {
        cfg1 |= opl_io | 1;
    }
    snd_als4k_gcr_write_addr(iobase, als4k_gcr_t::ALS4K_GCRA8_LEGACY_CFG1, cfg1);
    snd_als4k_gcr_write_addr(iobase, als4k_gcr_t::ALS4K_GCRA9_LEGACY_CFG2, cfg2);
}

unsafe fn snd_als4000_configure(chip: *mut snd_sb) {
    let tmp: u8;
    let mut i: u32;

    /* do some more configuration */
    spin_lock_irq(&mut (*chip).mixer_lock);
    tmp = snd_als4_cr_read(chip, als4k_cr_t::ALS4K_CR0_SB_CONFIG);
    snd_als4_cr_write(chip, als4k_cr_t::ALS4K_CR0_SB_CONFIG, tmp | ALS4K_CR0_MX80_81_REG_WRITE_ENABLE as u8);
    /* always select DMA channel 0, since we do not actually use DMA
     * SPECS_PAGE: 19/20 */
    snd_sbmixer_write(chip, SB_DSP4_DMASETUP, SB_DMASETUP_DMA0);
    snd_als4_cr_write(chip, als4k_cr_t::ALS4K_CR0_SB_CONFIG, tmp & !(ALS4K_CR0_MX80_81_REG_WRITE_ENABLE as u8));
    spin_unlock_irq(&mut (*chip).mixer_lock);

    spin_lock_irq(&mut (*chip).reg_lock);
    /* enable interrupts */
    snd_als4k_gcr_write(chip, als4k_gcr_t::ALS4K_GCR8C_MISC_CTRL, ALS4K_GCR8C_IRQ_MASK_CTRL_ENABLE);

    /* SPECS_PAGE: 39 */
    i = als4k_gcr_t::ALS4K_GCR91_DMA0_ADDR as u32;
    while i <= als4k_gcr_t::ALS4K_GCR96_DMA3_MODE_COUNT as u32 {
        snd_als4k_gcr_write(chip, core::mem::transmute::<u32, als4k_gcr_t>(i), 0);
        i += 1;
    }
    /* enable burst mode to prevent dropouts during high PCI bus usage */
    snd_als4k_gcr_write(
        chip,
        als4k_gcr_t::ALS4K_GCR99_DMA_EMULATION_CTRL,
        (snd_als4k_gcr_read(chip, als4k_gcr_t::ALS4K_GCR99_DMA_EMULATION_CTRL) & !0x07) | 0x04,
    );
    spin_unlock_irq(&mut (*chip).reg_lock);
}

/* #ifdef SUPPORT_JOYSTICK */
unsafe fn snd_als4000_create_gameport(acard: *mut snd_card_als4000, dev: c_int) -> c_int {
    let mut gp: *mut gameport;
    let mut r: *mut resource = core::ptr::null_mut();
    let mut io_port: c_int;

    if joystick_port[dev as usize] == 0 {
        return -ENODEV;
    }

    if joystick_port[dev as usize] == 1 {
        /* auto-detect */
        io_port = 0x200;
        while io_port <= 0x218 {
            r = devm_request_region(&mut (*(*acard).pci).dev, io_port as c_ulong, 8, c"ALS4000 gameport".as_ptr());
            if !r.is_null() {
                break;
            }
            io_port += 8;
        }
    } else {
        io_port = joystick_port[dev as usize];
        r = devm_request_region(&mut (*(*acard).pci).dev, io_port as c_ulong, 8, c"ALS4000 gameport".as_ptr());
    }

    if r.is_null() {
        dev_warn(&mut (*(*acard).pci).dev, c"cannot reserve joystick ports\n".as_ptr());
        return -EBUSY;
    }

    gp = gameport_allocate_port();
    (*acard).gameport = gp;
    if gp.is_null() {
        dev_err(&mut (*(*acard).pci).dev, c"cannot allocate memory for gameport\n".as_ptr());
        return -ENOMEM;
    }

    gameport_set_name(gp, c"ALS4000 Gameport".as_ptr());
    gameport_set_phys(gp, c"pci%s/gameport0".as_ptr(), pci_name((*acard).pci));
    gameport_set_dev_parent(gp, &mut (*(*acard).pci).dev);
    (*gp).io = io_port;

    /* Enable legacy joystick port */
    snd_als4000_set_addr((*acard).iobase, 0, 0, 0, 1);

    gameport_register_port((*acard).gameport);

    0
}

unsafe fn snd_als4000_free_gameport(acard: *mut snd_card_als4000) {
    if !(*acard).gameport.is_null() {
        gameport_unregister_port((*acard).gameport);
        (*acard).gameport = core::ptr::null_mut();

        /* disable joystick */
        snd_als4000_set_addr((*acard).iobase, 0, 0, 0, 0);
    }
}
/* #else
 * static inline int snd_als4000_create_gameport(struct snd_card_als4000 *acard, int dev) { return -ENOSYS; }
 * static inline void snd_als4000_free_gameport(struct snd_card_als4000 *acard) { }
 * #endif
 */

unsafe extern "C" fn snd_card_als4000_free(card: *mut snd_card) {
    let acard = (*card).private_data as *mut snd_card_als4000;

    /* make sure that interrupts are disabled */
    snd_als4k_gcr_write_addr((*acard).iobase, als4k_gcr_t::ALS4K_GCR8C_MISC_CTRL, 0);
    /* free resources */
    snd_als4000_free_gameport(acard);
}

unsafe fn __snd_card_als4000_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = core::ptr::null_mut();
    let acard: *mut snd_card_als4000;
    let iobase: c_ulong;
    let mut chip: *mut snd_sb = core::ptr::null_mut();
    let mut opl3: *mut snd_opl3 = core::ptr::null_mut();
    let mut word: u16 = 0;
    let mut err: c_int;

    let _ = pci_id;

    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }

    /* enable PCI device */
    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }

    /* check, if we can restrict PCI DMA transfers to 24 bits */
    if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(24)) != 0 {
        dev_err(&mut (*pci).dev, c"architecture does not support 24bit PCI busmaster DMA\n".as_ptr());
        return -ENXIO;
    }

    err = pcim_request_all_regions(pci, c"ALS4000".as_ptr());
    if err < 0 {
        return err;
    }
    iobase = pci_resource_start(pci, 0);

    pci_read_config_word(pci, PCI_COMMAND, &mut word);
    pci_write_config_word(pci, PCI_COMMAND, word | PCI_COMMAND_IO as u16);
    pci_set_master(pci);

    err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], THIS_MODULE, core::mem::size_of::<snd_card_als4000>(), &mut card);
    if err < 0 {
        return err;
    }

    acard = (*card).private_data as *mut snd_card_als4000;
    (*acard).pci = pci;
    (*acard).iobase = iobase;
    (*card).private_free = Some(snd_card_als4000_free);

    /* disable all legacy ISA stuff */
    snd_als4000_set_addr((*acard).iobase, 0, 0, 0, 0);

    err = snd_sbdsp_create(
        card,
        iobase + als4k_iobase_t::ALS4K_IOB_10_ADLIB_ADDR0 as c_ulong,
        (*pci).irq,
        /* internally registered as IRQF_SHARED in case of ALS4000 SB */
        Some(snd_als4000_interrupt),
        -1,
        -1,
        SB_HW_ALS4000,
        &mut chip,
    );
    if err < 0 {
        return err;
    }
    (*acard).chip = chip;

    (*chip).pci = pci;
    (*chip).alt_port = iobase;

    snd_als4000_configure(chip);

    strscpy((*card).driver.as_mut_ptr(), c"ALS4000".as_ptr());
    strscpy((*card).shortname.as_mut_ptr(), c"Avance Logic ALS4000".as_ptr());
    sprintf((*card).longname.as_mut_ptr(), c"%s at 0x%lx, irq %i".as_ptr(), (*card).shortname.as_ptr(), (*chip).alt_port, (*chip).irq);

    err = snd_mpu401_uart_new(
        card,
        0,
        MPU401_HW_ALS4000,
        iobase + als4k_iobase_t::ALS4K_IOB_30_MIDI_DATA as c_ulong,
        MPU401_INFO_INTEGRATED | MPU401_INFO_IRQ_HOOK,
        -1,
        &mut (*chip).rmidi,
    );
    if err < 0 {
        dev_err(&mut (*pci).dev, c"no MPU-401 device at 0x%lx?\n".as_ptr(), iobase + als4k_iobase_t::ALS4K_IOB_30_MIDI_DATA as c_ulong);
        return err;
    }
    /* FIXME: ALS4000 has interesting MPU401 configuration features
     * at ALS4K_CR1A_MPU401_UART_MODE_CONTROL
     * (pass-thru / UART switching, fast MIDI clock, etc.),
     * however there doesn't seem to be an ALSA API for this...
     * SPECS_PAGE: 21 */

    err = snd_als4000_pcm(chip, 0);
    if err < 0 {
        return err;
    }

    err = snd_sbmixer_new(chip);
    if err < 0 {
        return err;
    }

    if snd_opl3_create(
        card,
        iobase + als4k_iobase_t::ALS4K_IOB_10_ADLIB_ADDR0 as c_ulong,
        iobase + als4k_iobase_t::ALS4K_IOB_12_ADLIB_ADDR2 as c_ulong,
        OPL3_HW_AUTO,
        1,
        &mut opl3,
    ) < 0 {
        dev_err(
            &mut (*pci).dev,
            c"no OPL device at 0x%lx-0x%lx?\n".as_ptr(),
            iobase + als4k_iobase_t::ALS4K_IOB_10_ADLIB_ADDR0 as c_ulong,
            iobase + als4k_iobase_t::ALS4K_IOB_12_ADLIB_ADDR2 as c_ulong,
        );
    } else {
        err = snd_opl3_hwdep_new(opl3, 0, 1, core::ptr::null_mut());
        if err < 0 {
            return err;
        }
    }

    snd_als4000_create_gameport(acard, dev);

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}

unsafe extern "C" fn snd_card_als4000_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_card_als4000_probe(pci, pci_id))
}

unsafe extern "C" fn snd_als4000_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let acard = (*card).private_data as *mut snd_card_als4000;
    let chip = (*acard).chip;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);

    snd_sbmixer_suspend(chip);
    0
}

unsafe extern "C" fn snd_als4000_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let acard = (*card).private_data as *mut snd_card_als4000;
    let chip = (*acard).chip;

    snd_als4000_configure(chip);
    snd_sbdsp_reset(chip);
    snd_sbmixer_resume(chip);

    /* #ifdef SUPPORT_JOYSTICK */
    if !(*acard).gameport.is_null() {
        snd_als4000_set_addr((*acard).iobase, 0, 0, 0, 1);
    }
    /* #endif */

    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

static snd_als4000_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(snd_als4000_suspend),
    resume: Some(snd_als4000_resume),
};

#[repr(C)]
pub struct pci_driver_inner {
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub driver: pci_driver_inner,
}

static mut als4000_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_als4000_ids.as_ptr(),
    probe: Some(snd_card_als4000_probe),
    driver: pci_driver_inner {
        pm: &snd_als4000_pm,
    },
};

/* module_pci_driver(als4000_driver); */

unsafe extern "C" {
    static THIS_MODULE: *mut c_void;
    static KBUILD_MODNAME: *const c_char;

    fn outb(value: u8, port: c_ulong);
    fn outl(value: u32, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn inl(port: c_ulong) -> u32;
    fn snd_sbmixer_write(chip: *mut snd_sb, reg: u8, data: u8);
    fn snd_sbmixer_read(chip: *mut snd_sb, reg: u8) -> u8;
    fn snd_sbdsp_command(chip: *mut snd_sb, val: c_int) -> c_int;
    fn snd_pcm_format_signed(format: c_int) -> c_int;
    fn snd_pcm_format_physical_width(format: c_int) -> c_int;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_sb;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_ulong;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_ulong;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_ulong) -> snd_pcm_uframes_t;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_mpu401_uart_interrupt(irq: c_int, private_data: *mut c_void);
    fn snd_sb_ack_8bit(chip: *mut snd_sb);
    fn snd_sb_ack_16bit(chip: *mut snd_sb);
    fn IRQ_RETVAL(x: c_int) -> irqreturn_t;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, min: usize, max: usize);
    fn devm_request_region(dev: *mut device, start: c_ulong, n: c_ulong, name: *const c_char) -> *mut resource;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn gameport_allocate_port() -> *mut gameport;
    fn gameport_set_name(gameport: *mut gameport, name: *const c_char);
    fn gameport_set_phys(gameport: *mut gameport, fmt: *const c_char, ...);
    fn pci_name(pci: *mut pci_dev) -> *const c_char;
    fn gameport_set_dev_parent(gameport: *mut gameport, dev: *mut device);
    fn gameport_register_port(gameport: *mut gameport);
    fn gameport_unregister_port(gameport: *mut gameport);
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pci_read_config_word(pci: *mut pci_dev, where_: c_int, val: *mut u16) -> c_int;
    fn pci_write_config_word(pci: *mut pci_dev, where_: c_int, val: u16) -> c_int;
    fn pci_set_master(pci: *mut pci_dev);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, xid: *mut c_char, module: *mut c_void, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...);
    fn snd_sbdsp_create(card: *mut snd_card, port: c_ulong, irq: c_int, irq_handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, dma8: c_int, dma16: c_int, hardware: c_int, rchip: *mut *mut snd_sb) -> c_int;
    fn snd_mpu401_uart_new(card: *mut snd_card, device: c_int, hardware: c_int, port: c_ulong, info_flags: c_uint, irq: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_als4000_pcm_external(chip: *mut snd_sb, device: c_int) -> c_int;
    fn snd_sbmixer_new(chip: *mut snd_sb) -> c_int;
    fn snd_opl3_create(card: *mut snd_card, l_port: c_ulong, r_port: c_ulong, hardware: c_int, integrated: c_int, ropl3: *mut *mut snd_opl3) -> c_int;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, ops: *mut c_void) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_sbmixer_suspend(chip: *mut snd_sb);
    fn snd_sbmixer_resume(chip: *mut snd_sb);
    fn snd_sbdsp_reset(chip: *mut snd_sb) -> c_int;
}

const fn DMA_BIT_MASK(n: u32) -> u64 {
    if n == 64 { !0u64 } else { (1u64 << n) - 1 }
}

const SB_RATE_LOCK: c_int = 0x0001;
const SB_RATE_LOCK_CAPTURE: c_int = 0x0002;
const SB_RATE_LOCK_PLAYBACK: c_int = 0x0004;
const SB_DSP_SAMPLE_RATE_OUT: c_int = 0x41;
const SB_DSP4_OUT8_AI: c_int = 0xc6;
const SB_DSP4_OUT16_AI: c_int = 0xb6;
const SB_DSP_DMA8_ON: c_int = 0xd0;
const SB_DSP_DMA8_OFF: c_int = 0xd4;
const SB_DSP_DMA16_ON: c_int = 0xd6;
const SB_DSP_DMA16_OFF: c_int = 0xd5;
const SB_DSP4_MODE_UNS_MONO: c_int = 0x00;
const SB_DSP4_MODE_SIGN_MONO: c_int = 0x10;
const SB_DSP4_MODE_UNS_STEREO: c_int = 0x20;
const SB_DSP4_MODE_SIGN_STEREO: c_int = 0x30;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const ENXIO: c_int = 6;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const ENOSYS: c_int = 38;
const SB_DSP4_IRQSTATUS: u8 = 0x82;
const SB_IRQTYPE_8BIT: c_uint = 0x01;
const SB_IRQTYPE_16BIT: c_uint = 0x02;
const SB_IRQTYPE_MPUIN: c_uint = 0x04;
const ALS4K_IRQTYPE_CR1E_DMA: c_uint = 0x40;
const SNDRV_PCM_INFO_MMAP: c_uint = 0x00000001;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 0x00000100;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 0x00000002;
const SNDRV_PCM_FMTBIT_S8: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_U8: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 2;
const SNDRV_PCM_FMTBIT_U16_LE: c_uint = 1 << 3;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 30;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 6;
const SNDRV_PCM_INFO_JOINT_DUPLEX: c_uint = 0x00200000;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 1;
const SB_DSP4_DMASETUP: u8 = 0x81;
const SB_DMASETUP_DMA0: u8 = 0x01;
const PCI_COMMAND: c_int = 0x04;
const PCI_COMMAND_IO: c_int = 0x1;
const SB_HW_ALS4000: c_int = 0x4000;
const MPU401_HW_ALS4000: c_int = 0x4000;
const MPU401_INFO_INTEGRATED: c_uint = 0x0001;
const MPU401_INFO_IRQ_HOOK: c_uint = 0x0002;
const OPL3_HW_AUTO: c_int = 0;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
