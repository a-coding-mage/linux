// SPDX-License-Identifier: GPL-2.0-or-later
/*  azt3328.c - driver for Aztech AZF3328 based soundcards (e.g. PCI168).
 *  Copyright (C) 2002, 2005 - 2011 by Andreas Mohr <andi AT lisas.de>
 *
 *  Framework borrowed from Bart Hartgers's als4000.c.
 *  Driver developed on PCI168 AP(W) version (PCI rev. 10, subsystem ID 1801),
 *  found in a Fujitsu-Siemens PC ("Cordant", aluminum case).
 *
 *  This is a source-level Rust translation of the isolated C implementation.
 *  Linux/ALSA declarations, constants and helper macros that come from headers
 *  remain external dependencies.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type u8 = ::core::primitive::u8;
type u16 = ::core::primitive::u16;
type u32 = ::core::primitive::u32;
type bool_ = bool;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;
type spinlock_t = c_void;

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct pci_dev { pub dev: device, pub irq: c_int }
#[repr(C)]
pub struct pci_device_id { pub driver_data: c_ulong }
#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub number: c_int,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub sync_irq: c_int,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
    pub mixername: [c_char; 80],
}
#[repr(C)]
pub struct snd_timer {
    pub sticks: c_ulong,
    pub name: [c_char; 32],
    pub private_data: *mut c_void,
    pub hw: snd_timer_hardware,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_hardware {
    pub flags: c_uint,
    pub resolution: c_uint,
    pub ticks: c_uint,
    pub start: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>,
    pub precise_resolution: Option<unsafe extern "C" fn(*mut snd_timer, *mut c_ulong, *mut c_ulong) -> c_int>,
}
#[repr(C)]
pub struct snd_timer_id {
    pub dev_class: c_int,
    pub dev_sclass: c_int,
    pub card: c_int,
    pub device: c_int,
    pub subdevice: c_int,
}
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
#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub private_data: *mut c_void,
    pub dma_addr: c_ulong,
    pub rate: c_uint,
    pub format: c_int,
    pub channels: c_uint,
}
#[repr(C)]
pub struct snd_pcm { pub card: *mut snd_card, pub private_data: *mut c_void, pub info_flags: c_uint, pub name: [c_char; 32] }
#[repr(C)]
pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime, pub pcm: *mut snd_pcm }
#[repr(C)]
pub struct snd_rawmidi { pub private_data: *mut c_void }
#[repr(C)]
pub struct snd_opl3 { pub private_data: *mut c_void }
#[repr(C)]
pub struct snd_ac97 { pub private_data: *mut snd_azf3328, pub pci: *mut pci_dev }
#[repr(C)]
pub struct snd_ac97_bus { _private: [u8; 0] }
#[repr(C)]
pub struct snd_ac97_template { pub scaps: c_uint, pub private_data: *mut c_void, pub pci: *mut pci_dev }
#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub write: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort, c_ushort)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort) -> c_ushort>,
}
type c_ushort = u16;
#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}
#[repr(C)]
pub struct gameport {
    pub io: c_ulong,
    pub open: Option<unsafe extern "C" fn(*mut gameport, c_int) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut gameport)>,
    pub fuzz: c_int,
    pub cooked_read: Option<unsafe extern "C" fn(*mut gameport, *mut c_int, *mut c_int) -> c_int>,
}

unsafe extern "C" {
    fn inb(port: c_ulong) -> u8;
    fn inw(port: c_ulong) -> u16;
    fn inl(port: c_ulong) -> u32;
    fn outb(value: u8, port: c_ulong);
    fn outw(value: u16, port: c_ulong);
    fn outl(value: u32, port: c_ulong);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn mdelay(msecs: c_uint);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn WARN(condition: c_int, fmt: *const c_char, ...) -> c_int;
    fn WARN_ONCE(condition: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_BUG_ON(condition: bool) -> c_int;
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, private_data: *mut c_void, rbus: *mut *mut snd_ac97_bus) -> c_int;
    fn snd_ac97_mixer(bus: *mut snd_ac97_bus, template: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int;
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, min: usize, max: usize);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_azf3328;
    fn snd_pcm_format_width(format: c_int) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_ulong) -> snd_pcm_uframes_t;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_timer_chip(timer: *mut snd_timer) -> *mut snd_azf3328;
    fn snd_timer_new(card: *mut snd_card, id: *const c_char, tid: *mut snd_timer_id, rtimer: *mut *mut snd_timer) -> c_int;
    fn snd_timer_interrupt(timer: *mut snd_timer, ticks: c_ulong);
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *mut c_void, extra: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_card_free_on_error(dev: *mut device, ret: c_int) -> c_int;
    fn snd_component_add(card: *mut snd_card, component: *const c_char) -> c_int;
    fn snd_mpu401_uart_new(card: *mut snd_card, device: c_int, hardware: c_int, port: c_ulong, info: c_uint, irq: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_mpu401_uart_interrupt(irq: c_int, private_data: *mut c_void);
    fn snd_opl3_create(card: *mut snd_card, l_port: c_ulong, r_port: c_ulong, hardware: c_int, integrated: c_int, ropl3: *mut *mut snd_opl3) -> c_int;
    fn snd_opl3_timer_new(opl3: *mut snd_opl3, timer1_dev: c_int, timer2_dev: c_int) -> c_int;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, ops: *mut c_void) -> c_int;
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const c_char) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut c_void) -> c_int;
    fn pci_set_master(pci: *mut pci_dev);
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn pci_name(pci: *mut pci_dev) -> *const c_char;
    fn dev_get_drvdata(dev: *mut device) -> *mut snd_card;
    fn gameport_allocate_port() -> *mut gameport;
    fn gameport_set_name(gp: *mut gameport, name: *const c_char);
    fn gameport_set_phys(gp: *mut gameport, fmt: *const c_char, ...);
    fn gameport_set_dev_parent(gp: *mut gameport, dev: *mut device);
    fn gameport_set_port_data(gp: *mut gameport, data: *mut c_void);
    fn gameport_get_port_data(gp: *mut gameport) -> *mut snd_azf3328;
    fn gameport_register_port(gp: *mut gameport);
    fn gameport_unregister_port(gp: *mut gameport);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
}

unsafe extern "C" {
    static mut index: [c_int; SNDRV_CARDS as usize];
    static mut id: [*mut c_char; SNDRV_CARDS as usize];
    static mut enable: [bool; SNDRV_CARDS as usize];
}

const AZF_USE_AC97_LAYER: c_int = 1;
const SUPPORT_GAMEPORT: c_int = 1;
static mut seqtimer_scaling: c_int = 128;

unsafe extern "C" {
    static SNDRV_CARDS: c_int;
    static AC97_3D_CONTROL: u16;
    static AC97_POWERDOWN: u16;
    static AC97_EXTENDED_ID: u16;
    static AC97_EXTENDED_STATUS: u16;
    static AC97_VENDOR_ID1: u16;
    static AC97_VENDOR_ID2: u16;
    static AC97_RESET: u16;
    static AC97_REC_GAIN_MIC: u16;
}

const AZF_CODEC_PLAYBACK: snd_azf3328_codec_type = 0;
const AZF_CODEC_CAPTURE: snd_azf3328_codec_type = 1;
const AZF_CODEC_I2S_OUT: snd_azf3328_codec_type = 2;
type snd_azf3328_codec_type = c_int;

#[repr(C)]
pub struct snd_azf3328_codec_data {
    pub io_base: c_ulong,
    pub dma_base: c_uint,
    pub lock: *mut spinlock_t,
    pub substream: *mut snd_pcm_substream,
    pub running: bool,
    pub type_: snd_azf3328_codec_type,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_azf3328 {
    pub ctrl_io: c_ulong,
    pub game_io: c_ulong,
    pub mpu_io: c_ulong,
    pub opl3_io: c_ulong,
    pub mixer_io: c_ulong,
    pub reg_lock: spinlock_t,
    pub timer: *mut snd_timer,
    pub pcm: [*mut snd_pcm; 3],
    pub codecs: [snd_azf3328_codec_data; 3],
    pub ac97: *mut snd_ac97,
    pub card: *mut snd_card,
    pub rmidi: *mut snd_rawmidi,
    pub gameport: *mut gameport,
    pub axes: [u16; 4],
    pub pci: *mut pci_dev,
    pub irq: c_int,
    pub shadow_reg_ctrl_6AH: u16,
    pub saved_regs_ctrl: [u32; AZF_ALIGN(AZF_IO_SIZE_CTRL_PM) / 4],
    pub saved_regs_game: [u32; AZF_ALIGN(AZF_IO_SIZE_GAME_PM) / 4],
    pub saved_regs_mpu: [u32; AZF_ALIGN(AZF_IO_SIZE_MPU_PM) / 4],
    pub saved_regs_opl3: [u32; AZF_ALIGN(AZF_IO_SIZE_OPL3_PM) / 4],
    pub saved_regs_mixer: [u32; AZF_ALIGN(AZF_IO_SIZE_MIXER_PM) / 4],
}

unsafe extern "C" {
    static IDX_MIXER_PLAY_MASTER: c_uint;
    static IDX_MIXER_RESET: c_uint;
    static IDX_MIXER_WAVEOUT: c_uint;
    static IDX_MIXER_FMSYNTH: c_uint;
    static IDX_MIXER_MODEMOUT: c_uint;
    static IDX_MIXER_BASSTREBLE: c_uint;
    static IDX_MIXER_PCBEEP: c_uint;
    static IDX_MIXER_MODEMIN: c_uint;
    static IDX_MIXER_MIC: c_uint;
    static IDX_MIXER_LINEIN: c_uint;
    static IDX_MIXER_CDAUDIO: c_uint;
    static IDX_MIXER_VIDEO: c_uint;
    static IDX_MIXER_AUX: c_uint;
    static IDX_MIXER_REC_SELECT: c_uint;
    static IDX_MIXER_REC_VOLUME: c_uint;
    static IDX_MIXER_ADVCTL1: c_uint;
    static IDX_MIXER_ADVCTL2: c_uint;
    static IDX_MIXER_SOMETHING30H: c_uint;
    static IDX_IO_CODEC_SOUNDFORMAT: c_uint;
    static IDX_IO_CODEC_DMA_FLAGS: c_uint;
    static IDX_IO_CODEC_DMA_START_1: c_uint;
    static IDX_IO_CODEC_DMA_CURRPOS: c_uint;
    static IDX_IO_CODEC_IRQTYPE: c_uint;
    static IDX_IO_IRQSTATUS: c_uint;
    static IDX_IO_TIMER_VALUE: c_uint;
    static IDX_IO_6AH: c_uint;
    static IDX_GAME_HWCONFIG: c_uint;
    static IDX_GAME_AXIS_VALUE: c_uint;
    static IDX_GAME_LEGACY_COMPATIBLE: c_uint;
    static IDX_GAME_AXES_CONFIG: c_uint;
}

const fn AZF_ALIGN(x: usize) -> usize { (x + 3) & !3 }
const AZF_IO_SIZE_CTRL_PM: usize = 0;
const AZF_IO_SIZE_GAME_PM: usize = 0;
const AZF_IO_SIZE_MPU_PM: usize = 0;
const AZF_IO_SIZE_OPL3_PM: usize = 0;
const AZF_IO_SIZE_MIXER_PM: usize = 0;

unsafe extern "C" {
    static AZF_IO_SIZE_CTRL: c_uint;
    static AZF_IO_SIZE_MIXER: c_uint;
    static AZF_IO_OFFS_CODEC_PLAYBACK: c_ulong;
    static AZF_IO_OFFS_CODEC_CAPTURE: c_ulong;
    static AZF_IO_OFFS_CODEC_I2S_OUT: c_ulong;
    static DMA_RUN_SOMETHING1: u16;
    static DMA_RUN_SOMETHING2: u16;
    static DMA_RESUME: u16;
    static SOMETHING_ALMOST_ALWAYS_SET: u16;
    static DMA_EPILOGUE_SOMETHING: u16;
    static DMA_SOMETHING_ELSE: u16;
    static IRQ_SOMETHING: u8;
    static IRQ_PLAYBACK: u8;
    static IRQ_RECORDING: u8;
    static IRQ_I2S_OUT: u8;
    static IRQ_GAMEPORT: u8;
    static IRQ_MPU401: u8;
    static IRQ_TIMER: u8;
    static TIMER_VALUE_MASK: c_uint;
    static TIMER_COUNTDOWN_ENABLE: c_uint;
    static TIMER_IRQ_ENABLE: c_uint;
    static IO_6A_PAUSE_PLAYBACK_BIT8: c_uint;
    static IO_6A_SOMETHING2_GAMEPORT: c_uint;
    static GAME_HWCFG_IRQ_ENABLE: u8;
    static GAME_HWCFG_LEGACY_ADDRESS_ENABLE: u8;
    static GAME_HWCFG_ADC_COUNTER_FREQ_STD: c_uint;
    static GAME_HWCFG_ADC_COUNTER_FREQ_1_200: c_uint;
    static GAME_AXES_SAMPLING_READY: u8;
}

const AZF_MUTE_BIT: u8 = 0x80;
const AZF_REG_MASK: u16 = 0x3f;
const AZF_AC97_REG_UNSUPPORTED: u16 = 0x8000;
const AZF_AC97_REG_REAL_IO_READ: u16 = 0x4000;
const AZF_AC97_REG_REAL_IO_WRITE: u16 = 0x2000;
const AZF_AC97_REG_REAL_IO_RW: u16 = AZF_AC97_REG_REAL_IO_READ | AZF_AC97_REG_REAL_IO_WRITE;
const AZF_AC97_REG_EMU_IO_READ: u16 = 0x0400;
const AZF_AC97_REG_EMU_IO_WRITE: u16 = 0x0200;
const AZF_AC97_REG_EMU_IO_RW: u16 = AZF_AC97_REG_EMU_IO_READ | AZF_AC97_REG_EMU_IO_WRITE;

unsafe fn snd_azf3328_io_reg_setb(reg: c_uint, mask: u8, do_set: bool) -> c_int {
    let prev = inb(reg as c_ulong);
    let new = if do_set { prev | mask } else { prev & !mask };
    outb(new, reg as c_ulong);
    if new != prev { 1 } else { 0 }
}

unsafe fn snd_azf3328_codec_outb(codec: *const snd_azf3328_codec_data, reg: c_uint, value: u8) {
    outb(value, (*codec).io_base + reg as c_ulong);
}
unsafe fn snd_azf3328_codec_inb(codec: *const snd_azf3328_codec_data, reg: c_uint) -> u8 {
    inb((*codec).io_base + reg as c_ulong)
}
unsafe fn snd_azf3328_codec_outw(codec: *const snd_azf3328_codec_data, reg: c_uint, value: u16) {
    outw(value, (*codec).io_base + reg as c_ulong);
}
unsafe fn snd_azf3328_codec_inw(codec: *const snd_azf3328_codec_data, reg: c_uint) -> u16 {
    inw((*codec).io_base + reg as c_ulong)
}
unsafe fn snd_azf3328_codec_outl_multi(codec: *const snd_azf3328_codec_data, reg: c_uint, buffer: *const c_void, mut count: c_int) {
    let mut addr = (*codec).io_base + reg as c_ulong;
    let mut buf = buffer as *const u32;
    if count != 0 {
        loop {
            outl(*buf, addr);
            buf = buf.add(1);
            addr += 4;
            count -= 1;
            if count == 0 { break; }
        }
    }
}
unsafe fn snd_azf3328_codec_inl(codec: *const snd_azf3328_codec_data, reg: c_uint) -> u32 {
    inl((*codec).io_base + reg as c_ulong)
}
unsafe fn snd_azf3328_ctrl_outb(chip: *const snd_azf3328, reg: c_uint, value: u8) { outb(value, (*chip).ctrl_io + reg as c_ulong); }
unsafe fn snd_azf3328_ctrl_inb(chip: *const snd_azf3328, reg: c_uint) -> u8 { inb((*chip).ctrl_io + reg as c_ulong) }
unsafe fn snd_azf3328_ctrl_inw(chip: *const snd_azf3328, reg: c_uint) -> u16 { inw((*chip).ctrl_io + reg as c_ulong) }
unsafe fn snd_azf3328_ctrl_outw(chip: *const snd_azf3328, reg: c_uint, value: u16) { outw(value, (*chip).ctrl_io + reg as c_ulong); }
unsafe fn snd_azf3328_ctrl_outl(chip: *const snd_azf3328, reg: c_uint, value: u32) { outl(value, (*chip).ctrl_io + reg as c_ulong); }
unsafe fn snd_azf3328_game_outb(chip: *const snd_azf3328, reg: c_uint, value: u8) { outb(value, (*chip).game_io + reg as c_ulong); }
unsafe fn snd_azf3328_game_outw(chip: *const snd_azf3328, reg: c_uint, value: u16) { outw(value, (*chip).game_io + reg as c_ulong); }
unsafe fn snd_azf3328_game_inb(chip: *const snd_azf3328, reg: c_uint) -> u8 { inb((*chip).game_io + reg as c_ulong) }
unsafe fn snd_azf3328_game_inw(chip: *const snd_azf3328, reg: c_uint) -> u16 { inw((*chip).game_io + reg as c_ulong) }
unsafe fn snd_azf3328_mixer_outw(chip: *const snd_azf3328, reg: c_uint, value: u16) { outw(value, (*chip).mixer_io + reg as c_ulong); }
unsafe fn snd_azf3328_mixer_inw(chip: *const snd_azf3328, reg: c_uint) -> u16 { inw((*chip).mixer_io + reg as c_ulong) }

unsafe fn snd_azf3328_mixer_mute_control(chip: *const snd_azf3328, reg: c_uint, do_mute: bool) -> bool {
    let portbase = (*chip).mixer_io + reg as c_ulong + 1;
    let updated = snd_azf3328_io_reg_setb(portbase as c_uint, AZF_MUTE_BIT, do_mute) != 0;
    if do_mute { !updated } else { updated }
}
unsafe fn snd_azf3328_mixer_mute_control_master(chip: *const snd_azf3328, do_mute: bool) -> bool {
    snd_azf3328_mixer_mute_control(chip, IDX_MIXER_PLAY_MASTER, do_mute)
}
unsafe fn snd_azf3328_mixer_mute_control_pcm(chip: *const snd_azf3328, do_mute: bool) -> bool {
    snd_azf3328_mixer_mute_control(chip, IDX_MIXER_WAVEOUT, do_mute)
}
unsafe fn snd_azf3328_mixer_reset(chip: *const snd_azf3328) {
    snd_azf3328_mixer_mute_control_master(chip, true);
    snd_azf3328_mixer_outw(chip, IDX_MIXER_RESET, 0);
}

unsafe fn snd_azf3328_mixer_ac97_map_unsupported(chip: *const snd_azf3328, reg: u16, mode: *const c_char) {
    dev_warn((*(*chip).card).dev, c"missing %s emulation for AC97 register 0x%02x!\n".as_ptr(), mode, reg as c_int);
}

unsafe fn snd_azf3328_mixer_ac97_map_reg_idx(reg: u16) -> u16 {
    #[repr(C)]
    struct mapper { azf_reg: u16 }
    let azf_reg_mapper: [mapper; 18] = [
        mapper { azf_reg: IDX_MIXER_RESET as u16 | AZF_AC97_REG_REAL_IO_WRITE | AZF_AC97_REG_EMU_IO_READ },
        mapper { azf_reg: IDX_MIXER_PLAY_MASTER as u16 },
        mapper { azf_reg: IDX_MIXER_FMSYNTH as u16 },
        mapper { azf_reg: IDX_MIXER_MODEMOUT as u16 },
        mapper { azf_reg: IDX_MIXER_BASSTREBLE as u16 },
        mapper { azf_reg: IDX_MIXER_PCBEEP as u16 },
        mapper { azf_reg: IDX_MIXER_MODEMIN as u16 },
        mapper { azf_reg: IDX_MIXER_MIC as u16 },
        mapper { azf_reg: IDX_MIXER_LINEIN as u16 },
        mapper { azf_reg: IDX_MIXER_CDAUDIO as u16 },
        mapper { azf_reg: IDX_MIXER_VIDEO as u16 },
        mapper { azf_reg: IDX_MIXER_AUX as u16 },
        mapper { azf_reg: IDX_MIXER_WAVEOUT as u16 },
        mapper { azf_reg: IDX_MIXER_REC_SELECT as u16 },
        mapper { azf_reg: IDX_MIXER_REC_VOLUME as u16 },
        mapper { azf_reg: AZF_AC97_REG_EMU_IO_RW },
        mapper { azf_reg: IDX_MIXER_ADVCTL2 as u16 },
        mapper { azf_reg: IDX_MIXER_ADVCTL1 as u16 },
    ];
    let mut reg_azf = AZF_AC97_REG_UNSUPPORTED;
    if reg <= AC97_3D_CONTROL {
        let reg_idx = (reg / 2) as usize;
        reg_azf = azf_reg_mapper[reg_idx].azf_reg;
        if (reg_azf & !AZF_REG_MASK) == 0 {
            reg_azf |= AZF_AC97_REG_REAL_IO_RW;
        }
    } else if reg == AC97_POWERDOWN {
        reg_azf = AZF_AC97_REG_EMU_IO_RW;
    } else if reg == AC97_EXTENDED_ID {
        reg_azf = AZF_AC97_REG_EMU_IO_READ;
    } else if reg == AC97_EXTENDED_STATUS {
        reg_azf = AZF_AC97_REG_EMU_IO_RW;
    } else if reg == AC97_VENDOR_ID1 || reg == AC97_VENDOR_ID2 {
        reg_azf = AZF_AC97_REG_EMU_IO_READ;
    }
    reg_azf
}

unsafe extern "C" {
    static AC97_BC_DEDICATED_MIC: u16;
    static AC97_BC_BASS_TREBLE: u16;
    static AC97_BC_HEADPHONE: u16;
    static AC97_PD_ADC_STATUS: u16;
    static AC97_PD_DAC_STATUS: u16;
    static AC97_PD_MIXER_STATUS: u16;
    static AC97_PD_VREF_STATUS: u16;
}
unsafe fn azf_emulated_ac97_caps() -> u16 {
    AC97_BC_DEDICATED_MIC | AC97_BC_BASS_TREBLE | AC97_BC_HEADPHONE | (13 << 10)
}
unsafe fn azf_emulated_ac97_powerdown() -> u16 {
    AC97_PD_ADC_STATUS | AC97_PD_DAC_STATUS | AC97_PD_MIXER_STATUS | AC97_PD_VREF_STATUS
}
const azf_emulated_ac97_vendor_id: c_uint = 0x415a5401;

unsafe extern "C" fn snd_azf3328_mixer_ac97_read(ac97: *mut snd_ac97, reg_ac97: u16) -> u16 {
    let chip = (*ac97).private_data;
    let reg_azf = snd_azf3328_mixer_ac97_map_reg_idx(reg_ac97);
    let mut reg_val: u16 = 0;
    let mut unsupported = false;
    dev_dbg((*(*chip).card).dev, c"snd_azf3328_mixer_ac97_read reg_ac97 %u\n".as_ptr(), reg_ac97 as c_uint);
    if (reg_azf & AZF_AC97_REG_UNSUPPORTED) != 0 {
        unsupported = true;
    } else {
        if (reg_azf & AZF_AC97_REG_REAL_IO_READ) != 0 {
            reg_val = snd_azf3328_mixer_inw(chip, (reg_azf & AZF_REG_MASK) as c_uint);
        } else {
            snd_azf3328_mixer_inw(chip, IDX_MIXER_SOMETHING30H);
        }
        if (reg_azf & AZF_AC97_REG_EMU_IO_READ) != 0 {
            if reg_ac97 == AC97_RESET {
                reg_val |= azf_emulated_ac97_caps();
            } else if reg_ac97 == AC97_POWERDOWN {
                reg_val |= azf_emulated_ac97_powerdown();
            } else if reg_ac97 == AC97_EXTENDED_ID || reg_ac97 == AC97_EXTENDED_STATUS {
                reg_val |= 0;
            } else if reg_ac97 == AC97_VENDOR_ID1 {
                reg_val = (azf_emulated_ac97_vendor_id >> 16) as u16;
            } else if reg_ac97 == AC97_VENDOR_ID2 {
                reg_val = (azf_emulated_ac97_vendor_id & 0xffff) as u16;
            } else {
                unsupported = true;
            }
        }
    }
    if unsupported {
        snd_azf3328_mixer_ac97_map_unsupported(chip, reg_ac97, c"read".as_ptr());
    }
    reg_val
}

unsafe extern "C" fn snd_azf3328_mixer_ac97_write(ac97: *mut snd_ac97, reg_ac97: u16, val: u16) {
    let chip = (*ac97).private_data;
    let reg_azf = snd_azf3328_mixer_ac97_map_reg_idx(reg_ac97);
    let mut unsupported = false;
    dev_dbg((*(*chip).card).dev, c"snd_azf3328_mixer_ac97_write reg_ac97 %u val %u\n".as_ptr(), reg_ac97 as c_uint, val as c_uint);
    if (reg_azf & AZF_AC97_REG_UNSUPPORTED) != 0 {
        unsupported = true;
    } else if (reg_azf & AZF_AC97_REG_REAL_IO_WRITE) != 0 {
        snd_azf3328_mixer_outw(chip, (reg_azf & AZF_REG_MASK) as c_uint, val);
    } else if (reg_azf & AZF_AC97_REG_EMU_IO_WRITE) != 0 {
        if !(reg_ac97 == AC97_REC_GAIN_MIC || reg_ac97 == AC97_POWERDOWN || reg_ac97 == AC97_EXTENDED_STATUS) {
            unsupported = true;
        }
    }
    if unsupported {
        snd_azf3328_mixer_ac97_map_unsupported(chip, reg_ac97, c"write".as_ptr());
    }
}

unsafe fn snd_azf3328_mixer_new(chip: *mut snd_azf3328) -> c_int {
    let mut bus: *mut snd_ac97_bus = null_mut();
    let mut ac97: snd_ac97_template = zeroed();
    let ops = snd_ac97_bus_ops { write: Some(snd_azf3328_mixer_ac97_write), read: Some(snd_azf3328_mixer_ac97_read) };
    ac97.scaps = AC97_SCAP_SKIP_MODEM | AC97_SCAP_AUDIO | AC97_SCAP_NO_SPDIF;
    ac97.private_data = chip as *mut c_void;
    ac97.pci = (*chip).pci;
    let mut rc = snd_ac97_bus((*chip).card, 0, &ops, null_mut(), &mut bus);
    if rc == 0 {
        rc = snd_ac97_mixer(bus, &mut ac97, &mut (*chip).ac97);
    }
    if rc != 0 {
        dev_err((*(*chip).card).dev, c"AC97 init failed, err %d!\n".as_ptr(), rc);
    }
    rc
}

unsafe extern "C" {
    static AC97_SCAP_SKIP_MODEM: c_uint;
    static AC97_SCAP_AUDIO: c_uint;
    static AC97_SCAP_NO_SPDIF: c_uint;
    static AZF_FREQ_4000: c_uint;
    static AZF_FREQ_4800: c_uint;
    static AZF_FREQ_5512: c_uint;
    static AZF_FREQ_6620: c_uint;
    static AZF_FREQ_8000: c_uint;
    static AZF_FREQ_9600: c_uint;
    static AZF_FREQ_11025: c_uint;
    static AZF_FREQ_13240: c_uint;
    static AZF_FREQ_16000: c_uint;
    static AZF_FREQ_22050: c_uint;
    static AZF_FREQ_32000: c_uint;
    static AZF_FREQ_44100: c_uint;
    static AZF_FREQ_48000: c_uint;
    static AZF_FREQ_66200: c_uint;
    static SOUNDFORMAT_FREQ_SUSPECTED_4000: u8;
    static SOUNDFORMAT_FREQ_SUSPECTED_4800: u8;
    static SOUNDFORMAT_FREQ_5510: u8;
    static SOUNDFORMAT_FREQ_6620: u8;
    static SOUNDFORMAT_FREQ_8000: u8;
    static SOUNDFORMAT_FREQ_9600: u8;
    static SOUNDFORMAT_FREQ_11025: u8;
    static SOUNDFORMAT_FREQ_SUSPECTED_13240: u8;
    static SOUNDFORMAT_FREQ_16000: u8;
    static SOUNDFORMAT_FREQ_22050: u8;
    static SOUNDFORMAT_FREQ_32000: u8;
    static SOUNDFORMAT_FREQ_44100: u8;
    static SOUNDFORMAT_FREQ_48000: u8;
    static SOUNDFORMAT_FREQ_SUSPECTED_66200: u8;
    static SOUNDFORMAT_FLAG_2CHANNELS: u16;
    static SOUNDFORMAT_FLAG_16BIT: u16;
}

unsafe fn snd_azf3328_codec_setfmt(codec: *mut snd_azf3328_codec_data, bitrate: c_uint, format_width: c_uint, channels: c_uint) {
    let mut val: u16 = 0xff00;
    let freq: u8;
    if bitrate == AZF_FREQ_4000 { freq = SOUNDFORMAT_FREQ_SUSPECTED_4000; }
    else if bitrate == AZF_FREQ_4800 { freq = SOUNDFORMAT_FREQ_SUSPECTED_4800; }
    else if bitrate == AZF_FREQ_5512 { freq = SOUNDFORMAT_FREQ_5510; }
    else if bitrate == AZF_FREQ_6620 { freq = SOUNDFORMAT_FREQ_6620; }
    else if bitrate == AZF_FREQ_8000 { freq = SOUNDFORMAT_FREQ_8000; }
    else if bitrate == AZF_FREQ_9600 { freq = SOUNDFORMAT_FREQ_9600; }
    else if bitrate == AZF_FREQ_11025 { freq = SOUNDFORMAT_FREQ_11025; }
    else if bitrate == AZF_FREQ_13240 { freq = SOUNDFORMAT_FREQ_SUSPECTED_13240; }
    else if bitrate == AZF_FREQ_16000 { freq = SOUNDFORMAT_FREQ_16000; }
    else if bitrate == AZF_FREQ_22050 { freq = SOUNDFORMAT_FREQ_22050; }
    else if bitrate == AZF_FREQ_32000 { freq = SOUNDFORMAT_FREQ_32000; }
    else if bitrate == AZF_FREQ_48000 { freq = SOUNDFORMAT_FREQ_48000; }
    else if bitrate == AZF_FREQ_66200 { freq = SOUNDFORMAT_FREQ_SUSPECTED_66200; }
    else {
        if bitrate != AZF_FREQ_44100 {
            pr_warn(c"azf3328: unknown bitrate %d, assuming 44.1kHz!\n".as_ptr(), bitrate);
        }
        freq = SOUNDFORMAT_FREQ_44100;
    }
    val |= freq as u16;
    if channels == 2 { val |= SOUNDFORMAT_FLAG_2CHANNELS; }
    if format_width == 16 { val |= SOUNDFORMAT_FLAG_16BIT; }
    snd_azf3328_codec_outw(codec, IDX_IO_CODEC_SOUNDFORMAT, val);
    if (*codec).type_ != AZF_CODEC_CAPTURE {
        snd_azf3328_codec_outw(codec, IDX_IO_CODEC_DMA_FLAGS,
            snd_azf3328_codec_inw(codec, IDX_IO_CODEC_DMA_FLAGS)
                | DMA_RUN_SOMETHING1 | DMA_RUN_SOMETHING2 | SOMETHING_ALMOST_ALWAYS_SET
                | DMA_EPILOGUE_SOMETHING | DMA_SOMETHING_ELSE);
    }
}

unsafe fn snd_azf3328_codec_setfmt_lowpower(codec: *mut snd_azf3328_codec_data) {
    snd_azf3328_codec_setfmt(codec, AZF_FREQ_4000, 8, 1);
}

unsafe fn snd_azf3328_ctrl_reg_6AH_update(chip: *mut snd_azf3328, bitmask: c_uint, enable_: bool) {
    let do_mask = !enable_;
    if do_mask { (*chip).shadow_reg_ctrl_6AH |= bitmask as u16; }
    else { (*chip).shadow_reg_ctrl_6AH &= !(bitmask as u16); }
    dev_dbg((*(*chip).card).dev, c"6AH_update mask 0x%04x do_mask %d: val 0x%04x\n".as_ptr(), bitmask, do_mask as c_int, (*chip).shadow_reg_ctrl_6AH as c_uint);
    snd_azf3328_ctrl_outw(chip, IDX_IO_6AH, (*chip).shadow_reg_ctrl_6AH);
}
unsafe fn snd_azf3328_ctrl_enable_codecs(chip: *mut snd_azf3328, enable_: bool) {
    dev_dbg((*(*chip).card).dev, c"codec_enable %d\n".as_ptr(), enable_ as c_int);
    snd_azf3328_ctrl_reg_6AH_update(chip, IO_6A_PAUSE_PLAYBACK_BIT8, enable_);
}
unsafe fn snd_azf3328_ctrl_codec_activity(chip: *mut snd_azf3328, codec_type: snd_azf3328_codec_type, enable_: bool) {
    let codec = &mut (*chip).codecs[codec_type as usize] as *mut snd_azf3328_codec_data;
    let need_change = (*codec).running != enable_;
    dev_dbg((*(*chip).card).dev, c"codec_activity: %s codec, enable %d, need_change %d\n".as_ptr(), (*codec).name, enable_ as c_int, need_change as c_int);
    if need_change {
        let peer_codecs: [(snd_azf3328_codec_type, snd_azf3328_codec_type); 3] = [
            (AZF_CODEC_CAPTURE, AZF_CODEC_I2S_OUT),
            (AZF_CODEC_PLAYBACK, AZF_CODEC_I2S_OUT),
            (AZF_CODEC_PLAYBACK, AZF_CODEC_CAPTURE),
        ];
        let call_function = if enable_ {
            true
        } else {
            !(*chip).codecs[peer_codecs[codec_type as usize].0 as usize].running
                && !(*chip).codecs[peer_codecs[codec_type as usize].1 as usize].running
        };
        if call_function { snd_azf3328_ctrl_enable_codecs(chip, enable_); }
        if !enable_ { snd_azf3328_codec_setfmt_lowpower(codec); }
        (*codec).running = enable_;
    }
}

#[repr(C, packed)]
struct codec_setup_io { dma_start_1: u32, dma_start_2: u32, dma_lengths: u32 }

unsafe fn snd_azf3328_codec_setdmaa(chip: *mut snd_azf3328, codec: *mut snd_azf3328_codec_data, addr: c_ulong, period_bytes: c_uint, buffer_bytes: c_uint) {
    WARN_ONCE((period_bytes & 1) as c_int, c"odd period length!?\n".as_ptr());
    WARN_ONCE((buffer_bytes != 2 * period_bytes) as c_int, c"missed our input expectations! %u vs. %u\n".as_ptr(), buffer_bytes, period_bytes);
    if !(*codec).running {
        let area_length = buffer_bytes / 2;
        let setup_io = codec_setup_io {
            dma_start_1: addr as u32,
            dma_start_2: (addr + area_length as c_ulong) as u32,
            dma_lengths: (area_length << 16) | area_length,
        };
        dev_dbg((*(*chip).card).dev, c"setdma: buffers %08x[%u] / %08x[%u], %u, %u\n".as_ptr(), setup_io.dma_start_1, area_length, setup_io.dma_start_2, area_length, period_bytes, buffer_bytes);
        snd_azf3328_codec_outl_multi(codec, IDX_IO_CODEC_DMA_START_1, &setup_io as *const _ as *const c_void, 3);
    }
}

unsafe extern "C" fn snd_azf3328_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let codec = (*runtime).private_data as *mut snd_azf3328_codec_data;
    (*codec).dma_base = (*runtime).dma_addr as c_uint;
    0
}

unsafe extern "C" fn snd_azf3328_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let codec = (*runtime).private_data as *mut snd_azf3328_codec_data;
    let result = 0;
    let mut flags1: u16;
    let mut previously_muted = false;
    let is_main_mixer_playback_codec = AZF_CODEC_PLAYBACK == (*codec).type_;
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            dev_dbg((*(*chip).card).dev, c"START PCM %s\n".as_ptr(), (*codec).name);
            if is_main_mixer_playback_codec {
                previously_muted = snd_azf3328_mixer_mute_control_pcm(chip, true);
            }
            snd_azf3328_codec_setfmt(codec, (*runtime).rate, snd_pcm_format_width((*runtime).format), (*runtime).channels);
            flags1 = snd_azf3328_codec_inw(codec, IDX_IO_CODEC_DMA_FLAGS);
            flags1 &= !DMA_RESUME;
            snd_azf3328_codec_outw(codec, IDX_IO_CODEC_DMA_FLAGS, flags1);
            snd_azf3328_codec_outw(codec, IDX_IO_CODEC_IRQTYPE, 0xffff);
            snd_azf3328_codec_setdmaa(chip, codec, (*runtime).dma_addr, snd_pcm_lib_period_bytes(substream), snd_pcm_lib_buffer_bytes(substream));
            snd_azf3328_codec_outw(codec, IDX_IO_CODEC_DMA_FLAGS, 0x0000);
            snd_azf3328_codec_outw(codec, IDX_IO_CODEC_DMA_FLAGS, DMA_RUN_SOMETHING1);
            snd_azf3328_codec_outw(codec, IDX_IO_CODEC_DMA_FLAGS, DMA_RUN_SOMETHING1 | DMA_RUN_SOMETHING2);
            snd_azf3328_codec_outw(codec, IDX_IO_CODEC_DMA_FLAGS, DMA_RESUME | SOMETHING_ALMOST_ALWAYS_SET | DMA_EPILOGUE_SOMETHING | DMA_SOMETHING_ELSE);
            snd_azf3328_ctrl_codec_activity(chip, (*codec).type_, true);
            if is_main_mixer_playback_codec && !previously_muted {
                snd_azf3328_mixer_mute_control_pcm(chip, false);
            }
            dev_dbg((*(*chip).card).dev, c"PCM STARTED %s\n".as_ptr(), (*codec).name);
        }
        SNDRV_PCM_TRIGGER_RESUME => {
            dev_dbg((*(*chip).card).dev, c"PCM RESUME %s\n".as_ptr(), (*codec).name);
            if (*codec).running {
                let f = snd_azf3328_codec_inw(codec, IDX_IO_CODEC_DMA_FLAGS) | DMA_RESUME;
                snd_azf3328_codec_outw(codec, IDX_IO_CODEC_DMA_FLAGS, f);
            }
        }
        SNDRV_PCM_TRIGGER_STOP => {
            dev_dbg((*(*chip).card).dev, c"PCM STOP %s\n".as_ptr(), (*codec).name);
            if is_main_mixer_playback_codec {
                previously_muted = snd_azf3328_mixer_mute_control_pcm(chip, true);
            }
            flags1 = snd_azf3328_codec_inw(codec, IDX_IO_CODEC_DMA_FLAGS);
            flags1 &= !DMA_RESUME;
            snd_azf3328_codec_outw(codec, IDX_IO_CODEC_DMA_FLAGS, flags1);
            flags1 |= DMA_RUN_SOMETHING1;
            snd_azf3328_codec_outw(codec, IDX_IO_CODEC_DMA_FLAGS, flags1);
            flags1 &= !DMA_RUN_SOMETHING1;
            snd_azf3328_codec_outw(codec, IDX_IO_CODEC_DMA_FLAGS, flags1);
            snd_azf3328_ctrl_codec_activity(chip, (*codec).type_, false);
            if is_main_mixer_playback_codec && !previously_muted {
                snd_azf3328_mixer_mute_control_pcm(chip, false);
            }
            dev_dbg((*(*chip).card).dev, c"PCM STOPPED %s\n".as_ptr(), (*codec).name);
        }
        SNDRV_PCM_TRIGGER_SUSPEND => {
            dev_dbg((*(*chip).card).dev, c"PCM SUSPEND %s\n".as_ptr(), (*codec).name);
            let f = snd_azf3328_codec_inw(codec, IDX_IO_CODEC_DMA_FLAGS) & !DMA_RESUME;
            snd_azf3328_codec_outw(codec, IDX_IO_CODEC_DMA_FLAGS, f);
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => { WARN(1, c"FIXME: SNDRV_PCM_TRIGGER_PAUSE_PUSH NIY!\n".as_ptr()); }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => { WARN(1, c"FIXME: SNDRV_PCM_TRIGGER_PAUSE_RELEASE NIY!\n".as_ptr()); }
        _ => {
            WARN(1, c"FIXME: unknown trigger mode!\n".as_ptr());
            return -EINVAL;
        }
    }
    result
}

unsafe extern "C" {
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static EINVAL: c_int;
}

unsafe extern "C" fn snd_azf3328_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let codec = (*(*substream).runtime).private_data as *const snd_azf3328_codec_data;
    let mut result = snd_azf3328_codec_inl(codec, IDX_IO_CODEC_DMA_CURRPOS) as c_ulong;
    result = result.wrapping_sub((*codec).dma_base as c_ulong);
    let frmres = bytes_to_frames((*substream).runtime, result);
    dev_dbg((*(*(*substream).pcm).card).dev, c"%08li %s @ 0x%8lx, frames %8ld\n".as_ptr(), jiffies, (*codec).name, result, frmres);
    frmres
}

unsafe extern "C" { static jiffies: c_ulong; }

unsafe fn snd_azf3328_gameport_irq_enable(chip: *mut snd_azf3328, enable_: bool) {
    snd_azf3328_io_reg_setb(((*chip).game_io + IDX_GAME_HWCONFIG as c_ulong) as c_uint, GAME_HWCFG_IRQ_ENABLE, enable_);
}
unsafe fn snd_azf3328_gameport_legacy_address_enable(chip: *mut snd_azf3328, enable_: bool) {
    snd_azf3328_io_reg_setb(((*chip).game_io + IDX_GAME_HWCONFIG as c_ulong) as c_uint, GAME_HWCFG_LEGACY_ADDRESS_ENABLE, enable_);
}
unsafe fn snd_azf3328_gameport_set_counter_frequency(chip: *mut snd_azf3328, freq_cfg: c_uint) {
    snd_azf3328_io_reg_setb(((*chip).game_io + IDX_GAME_HWCONFIG as c_ulong) as c_uint, 0x02, (freq_cfg & 1) != 0);
    snd_azf3328_io_reg_setb(((*chip).game_io + IDX_GAME_HWCONFIG as c_ulong) as c_uint, 0x04, (freq_cfg & 2) != 0);
}
unsafe fn snd_azf3328_gameport_axis_circuit_enable(chip: *mut snd_azf3328, enable_: bool) {
    snd_azf3328_ctrl_reg_6AH_update(chip, IO_6A_SOMETHING2_GAMEPORT, enable_);
}
unsafe fn snd_azf3328_gameport_interrupt(chip: *mut snd_azf3328) {
    dev_dbg((*(*chip).card).dev, c"gameport irq\n".as_ptr());
    snd_azf3328_game_inw(chip, IDX_GAME_AXIS_VALUE);
}
unsafe extern "C" fn snd_azf3328_gameport_open(gameport: *mut gameport, mode: c_int) -> c_int {
    let chip = gameport_get_port_data(gameport);
    dev_dbg((*(*chip).card).dev, c"gameport_open, mode %d\n".as_ptr(), mode);
    let res = if mode == GAMEPORT_MODE_COOKED || mode == GAMEPORT_MODE_RAW { 0 } else { -1 };
    snd_azf3328_gameport_set_counter_frequency(chip, GAME_HWCFG_ADC_COUNTER_FREQ_STD);
    snd_azf3328_gameport_axis_circuit_enable(chip, res == 0);
    res
}
unsafe extern "C" fn snd_azf3328_gameport_close(gameport: *mut gameport) {
    let chip = gameport_get_port_data(gameport);
    dev_dbg((*(*chip).card).dev, c"gameport_close\n".as_ptr());
    snd_azf3328_gameport_set_counter_frequency(chip, GAME_HWCFG_ADC_COUNTER_FREQ_1_200);
    snd_azf3328_gameport_axis_circuit_enable(chip, false);
}
unsafe extern "C" fn snd_azf3328_gameport_cooked_read(gameport: *mut gameport, axes: *mut c_int, buttons: *mut c_int) -> c_int {
    let chip = gameport_get_port_data(gameport);
    if snd_BUG_ON(chip.is_null()) != 0 { return 0; }
    let mut val = snd_azf3328_game_inb(chip, IDX_GAME_LEGACY_COMPATIBLE);
    *buttons = ((!val >> 4) & 0xf) as c_int;
    val = snd_azf3328_game_inb(chip, IDX_GAME_AXES_CONFIG);
    if (val & GAME_AXES_SAMPLING_READY) != 0 {
        for i in 0..4 {
            val = ((i << 4) | 0x0f) as u8;
            snd_azf3328_game_outb(chip, IDX_GAME_AXES_CONFIG, val);
            (*chip).axes[i] = snd_azf3328_game_inw(chip, IDX_GAME_AXIS_VALUE);
        }
    }
    val = 0x03;
    snd_azf3328_game_outb(chip, IDX_GAME_AXES_CONFIG, val);
    snd_azf3328_game_outw(chip, IDX_GAME_AXIS_VALUE, 0xffff);
    for i in 0..4 {
        *axes.add(i) = (*chip).axes[i] as c_int;
        if *axes.add(i) == 0xffff { *axes.add(i) = -1; }
    }
    dev_dbg((*(*chip).card).dev, c"cooked_read: axes %d %d %d %d buttons %d\n".as_ptr(), *axes.add(0), *axes.add(1), *axes.add(2), *axes.add(3), *buttons);
    0
}
unsafe fn snd_azf3328_gameport(chip: *mut snd_azf3328, _dev: c_int) -> c_int {
    let gp = gameport_allocate_port();
    (*chip).gameport = gp;
    if gp.is_null() {
        dev_err((*(*chip).card).dev, c"cannot alloc memory for gameport\n".as_ptr());
        return -ENOMEM;
    }
    gameport_set_name(gp, c"AZF3328 Gameport".as_ptr());
    gameport_set_phys(gp, c"pci%s/gameport0".as_ptr(), pci_name((*chip).pci));
    gameport_set_dev_parent(gp, &mut (*(*chip).pci).dev);
    (*gp).io = (*chip).game_io;
    gameport_set_port_data(gp, chip as *mut c_void);
    (*gp).open = Some(snd_azf3328_gameport_open);
    (*gp).close = Some(snd_azf3328_gameport_close);
    (*gp).fuzz = 16;
    (*gp).cooked_read = Some(snd_azf3328_gameport_cooked_read);
    snd_azf3328_gameport_legacy_address_enable(chip, false);
    snd_azf3328_gameport_set_counter_frequency(chip, GAME_HWCFG_ADC_COUNTER_FREQ_1_200);
    snd_azf3328_gameport_axis_circuit_enable(chip, false);
    gameport_register_port((*chip).gameport);
    0
}
unsafe fn snd_azf3328_gameport_free(chip: *mut snd_azf3328) {
    if !(*chip).gameport.is_null() {
        gameport_unregister_port((*chip).gameport);
        (*chip).gameport = null_mut();
    }
    snd_azf3328_gameport_irq_enable(chip, false);
}
unsafe extern "C" { static GAMEPORT_MODE_COOKED: c_int; static GAMEPORT_MODE_RAW: c_int; static ENOMEM: c_int; }

unsafe fn snd_azf3328_irq_log_unknown_type(chip: *mut snd_azf3328, which: u8) {
    dev_dbg((*(*chip).card).dev, c"unknown IRQ type (%x) occurred, please report!\n".as_ptr(), which as c_uint);
}
unsafe fn snd_azf3328_pcm_interrupt(chip: *mut snd_azf3328, first_codec: *const snd_azf3328_codec_data, status: u8) {
    let mut codec_type = AZF_CODEC_PLAYBACK;
    let mut codec = first_codec;
    while codec_type <= AZF_CODEC_I2S_OUT {
        if (status & (1u8 << codec_type)) != 0 {
            let which = snd_azf3328_codec_inb(codec, IDX_IO_CODEC_IRQTYPE);
            snd_azf3328_codec_outb(codec, IDX_IO_CODEC_IRQTYPE, which);
            if !(*codec).substream.is_null() {
                snd_pcm_period_elapsed((*codec).substream);
                dev_dbg((*(*chip).card).dev, c"%s period done (#%x), @ %x\n".as_ptr(), (*codec).name, which as c_uint, snd_azf3328_codec_inl(codec, IDX_IO_CODEC_DMA_CURRPOS));
            } else {
                dev_warn((*(*chip).card).dev, c"irq handler problem!\n".as_ptr());
            }
            if (which & IRQ_SOMETHING) != 0 { snd_azf3328_irq_log_unknown_type(chip, which); }
        }
        codec_type += 1;
        codec = codec.add(1);
    }
}
unsafe extern "C" fn snd_azf3328_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut snd_azf3328;
    static mut irq_count: c_ulong = 0;
    let status = snd_azf3328_ctrl_inb(chip, IDX_IO_IRQSTATUS);
    if (status & (IRQ_PLAYBACK | IRQ_RECORDING | IRQ_I2S_OUT | IRQ_GAMEPORT | IRQ_MPU401 | IRQ_TIMER)) == 0 {
        return IRQ_NONE;
    }
    dev_dbg((*(*chip).card).dev, c"irq_count %ld! IDX_IO_IRQSTATUS %04x\n".as_ptr(), irq_count, status as c_uint);
    irq_count = irq_count.wrapping_add(1);
    if (status & IRQ_TIMER) != 0 {
        if !(*chip).timer.is_null() { snd_timer_interrupt((*chip).timer, (*(*chip).timer).sticks); }
        snd_azf3328_ctrl_outb(chip, IDX_IO_TIMER_VALUE + 3, 0x07);
        dev_dbg((*(*chip).card).dev, c"timer IRQ\n".as_ptr());
    }
    if (status & (IRQ_PLAYBACK | IRQ_RECORDING | IRQ_I2S_OUT)) != 0 {
        snd_azf3328_pcm_interrupt(chip, (*chip).codecs.as_ptr(), status);
    }
    if (status & IRQ_GAMEPORT) != 0 { snd_azf3328_gameport_interrupt(chip); }
    if (status & IRQ_MPU401) != 0 {
        snd_mpu401_uart_interrupt(irq, (*(*chip).rmidi).private_data);
        dev_dbg((*(*chip).card).dev, c"MPU401 IRQ\n".as_ptr());
    }
    IRQ_HANDLED
}
unsafe extern "C" { static IRQ_NONE: irqreturn_t; static IRQ_HANDLED: irqreturn_t; }

unsafe extern "C" {
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_FMTBIT_S8: c_uint;
    static SNDRV_PCM_FMTBIT_U8: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_U16_LE: c_uint;
    static SNDRV_PCM_RATE_5512: c_uint;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_RATE_KNOT: c_uint;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
}

unsafe fn snd_azf3328_hardware() -> snd_pcm_hardware {
    snd_pcm_hardware {
        info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
        formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE,
        rates: SNDRV_PCM_RATE_5512 | SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_KNOT,
        rate_min: AZF_FREQ_4000,
        rate_max: AZF_FREQ_66200,
        channels_min: 1,
        channels_max: 2,
        buffer_bytes_max: 64 * 1024,
        period_bytes_min: 1024,
        period_bytes_max: 32 * 1024,
        periods_min: 2,
        periods_max: 2,
        fifo_size: 0,
    }
}

static mut snd_azf3328_fixed_rates: [c_uint; 14] = [0; 14];

unsafe fn snd_azf3328_pcm_open(substream: *mut snd_pcm_substream, codec_type: snd_azf3328_codec_type) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let codec = &mut (*chip).codecs[codec_type as usize] as *mut snd_azf3328_codec_data;
    (*codec).substream = substream;
    (*runtime).hw = snd_azf3328_hardware();
    snd_azf3328_fixed_rates = [AZF_FREQ_4000, AZF_FREQ_4800, AZF_FREQ_5512, AZF_FREQ_6620, AZF_FREQ_8000, AZF_FREQ_9600, AZF_FREQ_11025, AZF_FREQ_13240, AZF_FREQ_16000, AZF_FREQ_22050, AZF_FREQ_32000, AZF_FREQ_44100, AZF_FREQ_48000, AZF_FREQ_66200];
    let constraints = snd_pcm_hw_constraint_list { count: 14, list: snd_azf3328_fixed_rates.as_ptr(), mask: 0 };
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints);
    (*runtime).private_data = codec as *mut c_void;
    0
}
unsafe extern "C" fn snd_azf3328_pcm_playback_open(substream: *mut snd_pcm_substream) -> c_int { snd_azf3328_pcm_open(substream, AZF_CODEC_PLAYBACK) }
unsafe extern "C" fn snd_azf3328_pcm_capture_open(substream: *mut snd_pcm_substream) -> c_int { snd_azf3328_pcm_open(substream, AZF_CODEC_CAPTURE) }
unsafe extern "C" fn snd_azf3328_pcm_i2s_out_open(substream: *mut snd_pcm_substream) -> c_int { snd_azf3328_pcm_open(substream, AZF_CODEC_I2S_OUT) }
unsafe extern "C" fn snd_azf3328_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let codec = (*(*substream).runtime).private_data as *mut snd_azf3328_codec_data;
    (*codec).substream = null_mut();
    0
}

static snd_azf3328_playback_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_azf3328_pcm_playback_open), close: Some(snd_azf3328_pcm_close), prepare: Some(snd_azf3328_pcm_prepare), trigger: Some(snd_azf3328_pcm_trigger), pointer: Some(snd_azf3328_pcm_pointer) };
static snd_azf3328_capture_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_azf3328_pcm_capture_open), close: Some(snd_azf3328_pcm_close), prepare: Some(snd_azf3328_pcm_prepare), trigger: Some(snd_azf3328_pcm_trigger), pointer: Some(snd_azf3328_pcm_pointer) };
static snd_azf3328_i2s_out_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_azf3328_pcm_i2s_out_open), close: Some(snd_azf3328_pcm_close), prepare: Some(snd_azf3328_pcm_prepare), trigger: Some(snd_azf3328_pcm_trigger), pointer: Some(snd_azf3328_pcm_pointer) };

unsafe fn snd_azf3328_pcm(chip: *mut snd_azf3328) -> c_int {
    const AZF_PCMDEV_STD: c_int = 0;
    const AZF_PCMDEV_I2S_OUT: c_int = 1;
    let mut pcm: *mut snd_pcm = null_mut();
    let mut err = snd_pcm_new((*chip).card, c"AZF3328 DSP".as_ptr(), AZF_PCMDEV_STD, 1, 1, &mut pcm);
    if err < 0 { return err; }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_azf3328_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_azf3328_capture_ops);
    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = 0;
    strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
    (*chip).pcm[AZF_CODEC_PLAYBACK as usize] = pcm;
    (*chip).pcm[AZF_CODEC_CAPTURE as usize] = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, 64 * 1024, 64 * 1024);
    err = snd_pcm_new((*chip).card, c"AZF3328 I2S OUT".as_ptr(), AZF_PCMDEV_I2S_OUT, 1, 0, &mut pcm);
    if err < 0 { return err; }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_azf3328_i2s_out_ops);
    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = 0;
    strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
    (*chip).pcm[AZF_CODEC_I2S_OUT as usize] = pcm;
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, 64 * 1024, 64 * 1024);
    0
}
unsafe extern "C" { static SNDRV_PCM_STREAM_PLAYBACK: c_int; static SNDRV_PCM_STREAM_CAPTURE: c_int; static SNDRV_DMA_TYPE_DEV: c_int; }

unsafe extern "C" fn snd_azf3328_timer_start(timer: *mut snd_timer) -> c_int {
    let chip = snd_timer_chip(timer);
    let mut delay = (((*timer).sticks as c_uint * seqtimer_scaling as c_uint) - 1) & TIMER_VALUE_MASK;
    if delay < 49 {
        dev_dbg((*(*chip).card).dev, c"delay was too low (%d)!\n".as_ptr(), delay);
        delay = 49;
    }
    dev_dbg((*(*chip).card).dev, c"setting timer countdown value %d\n".as_ptr(), delay);
    delay |= TIMER_COUNTDOWN_ENABLE | TIMER_IRQ_ENABLE;
    snd_azf3328_ctrl_outl(chip, IDX_IO_TIMER_VALUE, delay);
    0
}
unsafe extern "C" fn snd_azf3328_timer_stop(timer: *mut snd_timer) -> c_int {
    let chip = snd_timer_chip(timer);
    snd_azf3328_ctrl_outb(chip, IDX_IO_TIMER_VALUE + 3, 0x04);
    0
}
unsafe extern "C" fn snd_azf3328_timer_precise_resolution(_timer: *mut snd_timer, num: *mut c_ulong, den: *mut c_ulong) -> c_int {
    *num = 1;
    *den = (1024000 / seqtimer_scaling) as c_ulong;
    0
}
static mut snd_azf3328_timer_hw: snd_timer_hardware = snd_timer_hardware {
    flags: 0,
    resolution: 977,
    ticks: 1024000,
    start: Some(snd_azf3328_timer_start),
    stop: Some(snd_azf3328_timer_stop),
    precise_resolution: Some(snd_azf3328_timer_precise_resolution),
};
unsafe fn snd_azf3328_timer(chip: *mut snd_azf3328, device: c_int) -> c_int {
    let mut timer: *mut snd_timer = null_mut();
    let mut tid: snd_timer_id = zeroed();
    tid.dev_class = SNDRV_TIMER_CLASS_CARD;
    tid.dev_sclass = SNDRV_TIMER_SCLASS_NONE;
    tid.card = (*(*chip).card).number;
    tid.device = device;
    tid.subdevice = 0;
    snd_azf3328_timer_hw.flags = SNDRV_TIMER_HW_AUTO;
    snd_azf3328_timer_hw.resolution *= seqtimer_scaling as c_uint;
    snd_azf3328_timer_hw.ticks /= seqtimer_scaling as c_uint;
    let err = snd_timer_new((*chip).card, c"AZF3328".as_ptr(), &mut tid, &mut timer);
    if err < 0 { return err; }
    strscpy((*timer).name.as_mut_ptr(), c"AZF3328 timer".as_ptr());
    (*timer).private_data = chip as *mut c_void;
    (*timer).hw = snd_azf3328_timer_hw;
    (*chip).timer = timer;
    snd_azf3328_timer_stop(timer);
    0
}
unsafe extern "C" { static SNDRV_TIMER_CLASS_CARD: c_int; static SNDRV_TIMER_SCLASS_NONE: c_int; static SNDRV_TIMER_HW_AUTO: c_uint; }

unsafe extern "C" fn snd_azf3328_free(card: *mut snd_card) {
    let chip = (*card).private_data as *mut snd_azf3328;
    snd_azf3328_mixer_reset(chip);
    snd_azf3328_timer_stop((*chip).timer);
    snd_azf3328_gameport_free(chip);
}

unsafe fn snd_azf3328_debug_show_ports(chip: *const snd_azf3328) {
    dev_dbg((*(*chip).card).dev, c"ctrl_io 0x%lx, game_io 0x%lx, mpu_io 0x%lx, opl3_io 0x%lx, mixer_io 0x%lx, irq %d\n".as_ptr(), (*chip).ctrl_io, (*chip).game_io, (*chip).mpu_io, (*chip).opl3_io, (*chip).mixer_io, (*chip).irq);
    let mut tmp: u16 = 0;
    while (tmp as c_uint) < AZF_IO_SIZE_CTRL {
        dev_dbg((*(*chip).card).dev, c"ctrl 0x%02x: 0x%04x\n".as_ptr(), tmp as c_uint, snd_azf3328_ctrl_inw(chip, tmp as c_uint) as c_uint);
        tmp += 2;
    }
    tmp = 0;
    while (tmp as c_uint) < AZF_IO_SIZE_MIXER {
        dev_dbg((*(*chip).card).dev, c"mixer 0x%02x: 0x%04x\n".as_ptr(), tmp as c_uint, snd_azf3328_mixer_inw(chip, tmp as c_uint) as c_uint);
        tmp += 2;
    }
}

unsafe fn DMA_BIT_MASK(n: c_uint) -> u64 { if n == 64 { !0 } else { (1u64 << n) - 1 } }

unsafe fn snd_azf3328_create(card: *mut snd_card, pci: *mut pci_dev, _device_type: c_ulong) -> c_int {
    let chip = (*card).private_data as *mut snd_azf3328;
    let mut err = pcim_enable_device(pci);
    if err < 0 { return err; }
    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).irq = -1;
    if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(24)) != 0 {
        dev_err((*card).dev, c"architecture does not support 24bit PCI busmaster DMA\n".as_ptr());
        return -ENXIO;
    }
    err = pcim_request_all_regions(pci, c"Aztech AZF3328".as_ptr());
    if err < 0 { return err; }
    (*chip).ctrl_io = pci_resource_start(pci, 0);
    (*chip).game_io = pci_resource_start(pci, 1);
    (*chip).mpu_io = pci_resource_start(pci, 2);
    (*chip).opl3_io = pci_resource_start(pci, 3);
    (*chip).mixer_io = pci_resource_start(pci, 4);
    let names = [c"PLAYBACK".as_ptr(), c"CAPTURE".as_ptr(), c"I2S_OUT".as_ptr()];
    let offs = [AZF_IO_OFFS_CODEC_PLAYBACK, AZF_IO_OFFS_CODEC_CAPTURE, AZF_IO_OFFS_CODEC_I2S_OUT];
    for i in 0..3 {
        (*chip).codecs[i].io_base = (*chip).ctrl_io + offs[i];
        (*chip).codecs[i].lock = &mut (*chip).reg_lock;
        (*chip).codecs[i].type_ = i as c_int;
        (*chip).codecs[i].name = names[i];
    }
    if devm_request_irq(&mut (*pci).dev, (*pci).irq, snd_azf3328_interrupt, IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void) != 0 {
        dev_err((*card).dev, c"unable to grab IRQ %d\n".as_ptr(), (*pci).irq);
        return -EBUSY;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    (*card).private_free = Some(snd_azf3328_free);
    pci_set_master(pci);
    snd_azf3328_debug_show_ports(chip);
    err = snd_azf3328_mixer_new(chip);
    if err < 0 { return err; }
    let dma_init = DMA_RUN_SOMETHING2 | DMA_EPILOGUE_SOMETHING | DMA_SOMETHING_ELSE;
    for codec_type in 0..3 {
        let codec = &mut (*chip).codecs[codec_type] as *mut snd_azf3328_codec_data;
        (*codec).running = true;
        snd_azf3328_ctrl_codec_activity(chip, codec_type as c_int, false);
        snd_azf3328_codec_outb(codec, IDX_IO_CODEC_DMA_FLAGS, dma_init as u8);
    }
    0
}
unsafe extern "C" { static ENXIO: c_int; static EBUSY: c_int; static IRQF_SHARED: c_ulong; static KBUILD_MODNAME: *const c_char; }

unsafe fn __snd_azf3328_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = null_mut();
    let mut opl3: *mut snd_opl3 = null_mut();
    if dev >= SNDRV_CARDS { return -ENODEV; }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }
    let mut err = snd_devm_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], THIS_MODULE, size_of::<snd_azf3328>(), &mut card);
    if err < 0 { return err; }
    let chip = (*card).private_data as *mut snd_azf3328;
    strscpy((*card).driver.as_mut_ptr(), c"AZF3328".as_ptr());
    strscpy((*card).shortname.as_mut_ptr(), c"Aztech AZF3328 (PCI168)".as_ptr());
    err = snd_azf3328_create(card, pci, (*pci_id).driver_data);
    if err < 0 { return err; }
    err = snd_mpu401_uart_new(card, 0, MPU401_HW_AZT2320, (*chip).mpu_io, MPU401_INFO_INTEGRATED | MPU401_INFO_IRQ_HOOK, -1, &mut (*chip).rmidi);
    if err < 0 {
        dev_err((*card).dev, c"no MPU-401 device at 0x%lx?\n".as_ptr(), (*chip).mpu_io);
        return err;
    }
    err = snd_azf3328_timer(chip, 0);
    if err < 0 { return err; }
    err = snd_azf3328_pcm(chip);
    if err < 0 { return err; }
    if snd_opl3_create(card, (*chip).opl3_io, (*chip).opl3_io + 2, OPL3_HW_AUTO, 1, &mut opl3) < 0 {
        dev_err((*card).dev, c"no OPL3 device at 0x%lx-0x%lx?\n".as_ptr(), (*chip).opl3_io, (*chip).opl3_io + 2);
    } else {
        err = snd_opl3_timer_new(opl3, 1, 2);
        if err < 0 { return err; }
        err = snd_opl3_hwdep_new(opl3, 0, 1, null_mut());
        if err < 0 { return err; }
        (*opl3).private_data = chip as *mut c_void;
    }
    sprintf((*card).longname.as_mut_ptr(), c"%s at 0x%lx, irq %i".as_ptr(), (*card).shortname.as_ptr(), (*chip).ctrl_io, (*chip).irq);
    err = snd_card_register(card);
    if err < 0 { return err; }
    dev_info((*card).dev, c"Sound driver for Aztech AZF3328-based soundcards such as PCI168.\n".as_ptr());
    dev_info((*card).dev, c"Hardware was completely undocumented, unfortunately.\n".as_ptr());
    dev_info((*card).dev, c"Feel free to contact andi AT lisas.de for bug reports etc.!\n".as_ptr());
    dev_info((*card).dev, c"User-scalable sequencer timer set to %dHz (1024000Hz / %d).\n".as_ptr(), 1024000 / seqtimer_scaling, seqtimer_scaling);
    snd_azf3328_gameport(chip, dev);
    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}
unsafe extern "C" {
    static ENODEV: c_int; static ENOENT: c_int; static THIS_MODULE: *mut c_void;
    static MPU401_HW_AZT2320: c_int; static MPU401_INFO_INTEGRATED: c_uint; static MPU401_INFO_IRQ_HOOK: c_uint; static OPL3_HW_AUTO: c_int;
}

unsafe extern "C" fn snd_azf3328_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_azf3328_probe(pci, pci_id))
}

unsafe fn snd_azf3328_suspend_regs(chip: *const snd_azf3328, mut io_addr: c_ulong, count: c_uint, mut saved_regs: *mut u32) {
    for _ in 0..count {
        *saved_regs = inl(io_addr);
        dev_dbg((*(*chip).card).dev, c"suspend: io 0x%04lx: 0x%08x\n".as_ptr(), io_addr, *saved_regs);
        saved_regs = saved_regs.add(1);
        io_addr += size_of::<u32>() as c_ulong;
    }
}
unsafe fn snd_azf3328_resume_regs(chip: *const snd_azf3328, mut saved_regs: *const u32, mut io_addr: c_ulong, count: c_uint) {
    for _ in 0..count {
        outl(*saved_regs, io_addr);
        dev_dbg((*(*chip).card).dev, c"resume: io 0x%04lx: 0x%08x --> 0x%08x\n".as_ptr(), io_addr, *saved_regs, inl(io_addr));
        saved_regs = saved_regs.add(1);
        io_addr += size_of::<u32>() as c_ulong;
    }
}
unsafe fn snd_azf3328_suspend_ac97(chip: *mut snd_azf3328) { snd_ac97_suspend((*chip).ac97); }
unsafe fn snd_azf3328_resume_ac97(chip: *const snd_azf3328) { snd_ac97_resume((*chip).ac97); }

unsafe extern "C" fn snd_azf3328_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev);
    let chip = (*card).private_data as *mut snd_azf3328;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    snd_azf3328_suspend_ac97(chip);
    snd_azf3328_suspend_regs(chip, (*chip).ctrl_io, (*chip).saved_regs_ctrl.len() as c_uint, (*chip).saved_regs_ctrl.as_mut_ptr());
    let saved_regs_ctrl_u16 = (*chip).saved_regs_ctrl.as_mut_ptr() as *mut u16;
    *saved_regs_ctrl_u16.add((IDX_IO_6AH / 2) as usize) = (*chip).shadow_reg_ctrl_6AH;
    snd_azf3328_suspend_regs(chip, (*chip).game_io, (*chip).saved_regs_game.len() as c_uint, (*chip).saved_regs_game.as_mut_ptr());
    snd_azf3328_suspend_regs(chip, (*chip).mpu_io, (*chip).saved_regs_mpu.len() as c_uint, (*chip).saved_regs_mpu.as_mut_ptr());
    snd_azf3328_suspend_regs(chip, (*chip).opl3_io, (*chip).saved_regs_opl3.len() as c_uint, (*chip).saved_regs_opl3.as_mut_ptr());
    0
}
unsafe extern "C" fn snd_azf3328_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev);
    let chip = (*card).private_data as *const snd_azf3328;
    snd_azf3328_resume_regs(chip, (*chip).saved_regs_game.as_ptr(), (*chip).game_io, (*chip).saved_regs_game.len() as c_uint);
    snd_azf3328_resume_regs(chip, (*chip).saved_regs_mpu.as_ptr(), (*chip).mpu_io, (*chip).saved_regs_mpu.len() as c_uint);
    snd_azf3328_resume_regs(chip, (*chip).saved_regs_opl3.as_ptr(), (*chip).opl3_io, (*chip).saved_regs_opl3.len() as c_uint);
    snd_azf3328_resume_ac97(chip);
    snd_azf3328_resume_regs(chip, (*chip).saved_regs_ctrl.as_ptr(), (*chip).ctrl_io, (*chip).saved_regs_ctrl.len() as c_uint);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}
unsafe extern "C" { static SNDRV_CTL_POWER_D3hot: c_int; static SNDRV_CTL_POWER_D0: c_int; }

/* DEFINE_SIMPLE_DEV_PM_OPS(snd_azf3328_pm, snd_azf3328_suspend, snd_azf3328_resume);
 * module_pci_driver(azf3328_driver);
 * These module/driver registration macros are provided by the kernel build.
 */


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
