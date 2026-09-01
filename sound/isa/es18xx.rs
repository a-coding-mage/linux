// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for generic ESS AudioDrive ES18xx soundcards
 *  Copyright (c) by Christian Fischbach <fishbach@pool.informatik.rwth-aachen.de>
 *  Copyright (c) by Abramo Bagnara <abramo@alsa-project.org>
 */
/* GENERAL NOTES:
 *
 * BUGS:
 * - There are pops (we can't delay in trigger function, cause midlevel
 *   often need to trigger down and then up very quickly).
 *   Any ideas?
 * - Support for 16 bit DMA seems to be broken. I've no hardware to tune it.
 */

/*
 * ES1868  NOTES:
 * - The chip has one half duplex pcm (with very limited full duplex support).
 *
 * - Duplex stereophonic sound is impossible.
 * - Record and playback must share the same frequency rate.
 *
 * - The driver use dma2 for playback and dma1 for capture.
 */

/*
 * ES1869 NOTES:
 *
 * - there are a first full duplex pcm and a second playback only pcm
 *   (incompatible with first pcm capture)
 *
 * - there is support for the capture volume and ESS Spatializer 3D effect.
 *
 * - contrarily to some pages in DS_1869.PDF the rates can be set
 *   independently.
 *
 * - Zoom Video is implemented by sharing the FM DAC, thus the user can
 *   have either FM playback or Video playback but not both simultaneously.
 *   The Video Playback Switch mixer control toggles this choice.
 *
 * BUGS:
 *
 * - There is a major trouble I noted:
 *
 *   using both channel for playback stereo 16 bit samples at 44100 Hz
 *   the second pcm (Audio1) DMA slows down irregularly and sound is garbled.
 *
 *   The same happens using Audio1 for captureing.
 *
 *   The Windows driver does not suffer of this (although it use Audio1
 *   only for captureing). I'm unable to discover why.
 *
 */

/*
 * ES1879 NOTES:
 * - When Zoom Video is enabled (reg 0x71 bit 6 toggled on) the PCM playback
 *   seems to be effected (speaker_test plays a lower frequency). Can't find
 *   anything in the datasheet to account for this, so a Video Playback Switch
 *   control has been included to allow ZV to be enabled only when necessary.
 *   Then again on at least one test system the 0x71 bit 6 enable bit is not
 *   needed for ZV, so maybe the datasheet is entirely wrong here.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type bool_ = bool;
type irqreturn_t = c_int;
type snd_pcm_uframes_t = c_ulong;
type pm_message_t = c_int;
type spinlock_t = c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub sync_irq: c_int,
    pub mixername: [c_char; 80],
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
    pub name: [c_char; 80],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate_num: c_uint,
    pub rate_den: c_uint,
    pub rate: c_uint,
    pub channels: c_uint,
    pub format: c_int,
    pub dma_addr: c_ulong,
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub number: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_rawmidi {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_kcontrol_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_kcontrol)>,
    pub id: snd_kcontrol_id,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ratnum {
    pub num: c_uint,
    pub den_min: c_uint,
    pub den_max: c_uint,
    pub den_step: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_ratnums {
    pub nrats: c_uint,
    pub rats: *const snd_ratnum,
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
    pub buffer_bytes_max: c_ulong,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: c_uint,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub index: c_uint,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pnp_dev {
    pub dev: device,
}

#[repr(C)]
pub struct pnp_device_id {
    pub id: [c_char; 8],
}

#[repr(C)]
pub struct pnp_card_device {
    pub id: [c_char; 8],
}

#[repr(C)]
pub struct pnp_card_device_id {
    pub id: [c_char; 8],
    pub devs: [pnp_card_device; 2],
}

#[repr(C)]
pub struct pnp_card {
    pub dev: device,
}

#[repr(C)]
pub struct pnp_card_link {
    pub card: *mut pnp_card,
}

#[repr(C)]
pub struct isa_driver_inner {
    pub name: *const c_char,
}

#[repr(C)]
pub struct isa_driver {
    pub match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub driver: isa_driver_inner,
}

#[repr(C)]
pub struct pnp_driver {
    pub name: *const c_char,
    pub id_table: *const pnp_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pnp_dev, *const pnp_device_id) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut pnp_dev, pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut pnp_dev) -> c_int>,
}

#[repr(C)]
pub struct pnp_card_driver {
    pub flags: c_uint,
    pub name: *const c_char,
    pub id_table: *const pnp_card_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pnp_card_link, *const pnp_card_device_id) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut pnp_card_link, pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut pnp_card_link) -> c_int>,
}

#[repr(C)]
pub struct snd_es18xx {
    pub card: *mut snd_card,
    pub port: c_ulong,
    pub ctrl_port: c_ulong,
    pub irq: c_int,
    pub dma1: c_int,
    pub dma2: c_int,
    pub version: u16,
    pub caps: c_int,
    pub audio2_vol: u16,
    pub active: u16,
    pub dma1_shift: c_uint,
    pub dma2_shift: c_uint,
    pub pcm: *mut snd_pcm,
    pub playback_a_substream: *mut snd_pcm_substream,
    pub capture_a_substream: *mut snd_pcm_substream,
    pub playback_b_substream: *mut snd_pcm_substream,
    pub rmidi: *mut snd_rawmidi,
    pub hw_volume: *mut snd_kcontrol,
    pub hw_switch: *mut snd_kcontrol,
    pub master_volume: *mut snd_kcontrol,
    pub master_switch: *mut snd_kcontrol,
    pub reg_lock: spinlock_t,
    pub mixer_lock: spinlock_t,
    /* CONFIG_PM */
    pub pm_reg: u8,
    /* CONFIG_PNP */
    pub dev: *mut pnp_dev,
    pub devc: *mut pnp_dev,
}

const AUDIO1_IRQ: u8 = 0x01;
const AUDIO2_IRQ: u8 = 0x02;
const HWV_IRQ: u8 = 0x04;
const MPU_IRQ: u8 = 0x08;

const ES18XX_PCM2: c_int = 0x0001;
const ES18XX_SPATIALIZER: c_int = 0x0002;
const ES18XX_RECMIX: c_int = 0x0004;
const ES18XX_DUPLEX_MONO: c_int = 0x0008;
const ES18XX_DUPLEX_SAME: c_int = 0x0010;
const ES18XX_NEW_RATE: c_int = 0x0020;
const ES18XX_AUXB: c_int = 0x0040;
const ES18XX_HWV: c_int = 0x0080;
const ES18XX_MONO: c_int = 0x0100;
const ES18XX_I2S: c_int = 0x0200;
const ES18XX_MUTEREC: c_int = 0x0400;
const ES18XX_CONTROL: c_int = 0x0800;
const ES18XX_GPO_2BIT: c_int = 0x1000;

/* Power Management */
const ES18XX_PM: u8 = 0x07;
const ES18XX_PM_GPO0: u8 = 0x01;
const ES18XX_PM_GPO1: u8 = 0x02;
const ES18XX_PM_PDR: u8 = 0x04;
const ES18XX_PM_ANA: u8 = 0x08;
const ES18XX_PM_FM: u8 = 0x020;
const ES18XX_PM_SUS: u8 = 0x080;

/* Lowlevel */
const DAC1: u16 = 0x01;
const ADC1: u16 = 0x02;
const DAC2: u16 = 0x04;
const MILLISECOND: c_int = 10000;

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const EAGAIN: c_int = 11;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const IRQ_HANDLED: irqreturn_t = 1;
const DMA_MODE_WRITE: c_int = 0x48;
const DMA_MODE_READ: c_int = 0x44;
const DMA_AUTOINIT: c_int = 0x10;
const SNDRV_CARDS: usize = 8;
const SNDRV_AUTO_PORT: c_long = -1;
const SNDRV_AUTO_IRQ: c_int = -1;
const SNDRV_AUTO_DMA: c_int = -1;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_ISAPNP: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];
const SNDRV_DEFAULT_PORT: [c_long; SNDRV_CARDS] = [SNDRV_AUTO_PORT; SNDRV_CARDS];
const SNDRV_DEFAULT_IRQ: [c_int; SNDRV_CARDS] = [SNDRV_AUTO_IRQ; SNDRV_CARDS];
const SNDRV_DEFAULT_DMA: [c_int; SNDRV_CARDS] = [SNDRV_AUTO_DMA; SNDRV_CARDS];

const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 1;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 2;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 3;
const SNDRV_PCM_INFO_JOINT_DUPLEX: c_uint = 1 << 4;
const SNDRV_PCM_INFO_HALF_DUPLEX: c_uint = 1 << 5;
const SNDRV_PCM_FMTBIT_U8: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S8: c_ulong = 1 << 1;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 2;
const SNDRV_PCM_FMTBIT_U16_LE: c_ulong = 1 << 3;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 0;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_int = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1 << 0;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const OPL3_HW_OPL3: c_int = 0;
const MPU401_HW_ES18XX: c_int = 0;
const MPU401_INFO_IRQ_HOOK: c_int = 1;
const PNP_DRIVER_RES_DISABLE: c_uint = 1;

unsafe extern "C" {
    fn inb(port: c_ulong) -> u8;
    fn outb(value: u8, port: c_ulong);
    fn udelay(usecs: c_uint);
    fn mdelay(msecs: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_es18xx;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_pcm_format_unsigned(format: c_int) -> c_int;
    fn _snd_pcm_hw_param_setempty(params: *mut snd_pcm_hw_params, var: c_int);
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_dma_program(dma: c_int, addr: c_ulong, size: c_uint, mode: c_int);
    fn snd_dma_pointer(dma: c_int, size: c_uint) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_mpu401_uart_interrupt(irq: c_int, private_data: *mut c_void);
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_kcontrol_id);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_es18xx;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, names: *const *const c_char) -> c_int;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_boolean_stereo_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_BUG();
    fn snd_pcm_hw_constraint_ratnums(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, r: *const snd_pcm_hw_constraint_ratnums) -> c_int;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, min: c_ulong, max: c_ulong);
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn devm_request_region(dev: *mut device, start: c_ulong, n: c_ulong, name: *const c_char) -> *mut c_void;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn snd_devm_request_dma(dev: *mut device, dma: c_int, name: *const c_char) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> c_long;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snd_devm_card_new(parent: *mut device, idx: c_int, xid: *mut c_char, module: *mut c_void, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_opl3_create(card: *mut snd_card, l_port: c_long, r_port: c_long, hardware: c_int, integrated: c_int, ropl3: *mut *mut snd_opl3) -> c_int;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, rrawmidi: *mut c_void) -> c_int;
    fn snd_mpu401_uart_new(card: *mut snd_card, device: c_int, hardware: c_int, port: c_long, info_flags: c_int, irq: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_legacy_find_free_irq(irqs: *const c_int) -> c_int;
    fn snd_legacy_find_free_dma(dmas: *const c_int) -> c_int;
    fn isa_register_driver(driver: *mut isa_driver, ndev: c_uint) -> c_int;
    fn isa_unregister_driver(driver: *mut isa_driver);
    fn pnp_activate_dev(dev: *mut pnp_dev) -> c_int;
    fn pnp_device_is_isapnp(dev: *mut pnp_dev) -> c_int;
    fn isapnp_cfg_begin(card: c_int, csn: c_int);
    fn isapnp_card_number(dev: *mut pnp_dev) -> c_int;
    fn isapnp_csn_number(dev: *mut pnp_dev) -> c_int;
    fn isapnp_write_byte(idx: c_int, val: c_int);
    fn isapnp_cfg_end();
    fn pnp_port_start(dev: *mut pnp_dev, bar: c_uint) -> c_ulong;
    fn pnp_dma(dev: *mut pnp_dev, idx: c_uint) -> c_int;
    fn pnp_irq(dev: *mut pnp_dev, idx: c_uint) -> c_int;
    fn pnp_request_card_device(card: *mut pnp_card_link, id: *const c_char, from: *mut pnp_dev) -> *mut pnp_dev;
    fn pnp_register_driver(driver: *mut pnp_driver) -> c_int;
    fn pnp_unregister_driver(driver: *mut pnp_driver);
    fn pnp_register_card_driver(driver: *mut pnp_card_driver) -> c_int;
    fn pnp_unregister_card_driver(driver: *mut pnp_card_driver);
    fn pnp_set_drvdata(dev: *mut pnp_dev, data: *mut c_void);
    fn pnp_get_drvdata(dev: *mut pnp_dev) -> *mut c_void;
    fn pnp_set_card_drvdata(card: *mut pnp_card_link, data: *mut c_void);
    fn pnp_get_card_drvdata(card: *mut pnp_card_link) -> *mut c_void;
}

#[inline]
unsafe fn lock_guard(_lock: *mut spinlock_t) {}

unsafe fn snd_es18xx_dsp_command(chip: *mut snd_es18xx, val: u8) -> c_int {
    let mut i = MILLISECOND;
    while i != 0 {
        if (inb((*chip).port + 0x0c) & 0x80) == 0 {
            outb(val, (*chip).port + 0x0c);
            return 0;
        }
        i -= 1;
    }
    dev_err((*(*chip).card).dev, c"dsp_command: timeout (0x%x)\n".as_ptr(), val as c_int);
    -EINVAL
}

unsafe fn snd_es18xx_dsp_get_byte(chip: *mut snd_es18xx) -> c_int {
    let mut i = MILLISECOND / 10;
    while i != 0 {
        if (inb((*chip).port + 0x0c) & 0x40) != 0 {
            return inb((*chip).port + 0x0a) as c_int;
        }
        i -= 1;
    }
    dev_err(
        (*(*chip).card).dev,
        c"dsp_get_byte failed: 0x%lx = 0x%x!!!\n".as_ptr(),
        (*chip).port + 0x0a,
        inb((*chip).port + 0x0a) as c_int,
    );
    -ENODEV
}

/* REG_DEBUG intentionally omitted unless enabled by the surrounding build. */

unsafe fn snd_es18xx_write(chip: *mut snd_es18xx, reg: u8, data: u8) -> c_int {
    lock_guard(&mut (*chip).reg_lock);
    let mut ret = snd_es18xx_dsp_command(chip, reg);
    if ret < 0 {
        return ret;
    }
    ret = snd_es18xx_dsp_command(chip, data);
    ret
}

unsafe fn snd_es18xx_read(chip: *mut snd_es18xx, reg: u8) -> c_int {
    lock_guard(&mut (*chip).reg_lock);
    let mut ret = snd_es18xx_dsp_command(chip, 0xc0);
    if ret < 0 {
        return ret;
    }
    ret = snd_es18xx_dsp_command(chip, reg);
    if ret < 0 {
        return ret;
    }
    snd_es18xx_dsp_get_byte(chip)
}

/* Return old value */
unsafe fn snd_es18xx_bits(chip: *mut snd_es18xx, reg: u8, mask: u8, val: u8) -> c_int {
    lock_guard(&mut (*chip).reg_lock);
    let mut ret = snd_es18xx_dsp_command(chip, 0xc0);
    if ret < 0 {
        return ret;
    }
    ret = snd_es18xx_dsp_command(chip, reg);
    if ret < 0 {
        return ret;
    }
    ret = snd_es18xx_dsp_get_byte(chip);
    if ret < 0 {
        return ret;
    }
    let old = ret as u8;
    let oval = old & mask;
    if val != oval {
        ret = snd_es18xx_dsp_command(chip, reg);
        if ret < 0 {
            return ret;
        }
        let new = (old & !mask) | (val & mask);
        ret = snd_es18xx_dsp_command(chip, new);
        if ret < 0 {
            return ret;
        }
    }
    oval as c_int
}

#[inline]
unsafe fn snd_es18xx_mixer_write(chip: *mut snd_es18xx, reg: u8, data: u8) {
    lock_guard(&mut (*chip).mixer_lock);
    outb(reg, (*chip).port + 0x04);
    outb(data, (*chip).port + 0x05);
}

#[inline]
unsafe fn snd_es18xx_mixer_read(chip: *mut snd_es18xx, reg: u8) -> c_int {
    lock_guard(&mut (*chip).mixer_lock);
    outb(reg, (*chip).port + 0x04);
    inb((*chip).port + 0x05) as c_int
}

/* Return old value */
#[inline]
unsafe fn snd_es18xx_mixer_bits(chip: *mut snd_es18xx, reg: u8, mask: u8, val: u8) -> c_int {
    lock_guard(&mut (*chip).mixer_lock);
    outb(reg, (*chip).port + 0x04);
    let old = inb((*chip).port + 0x05);
    let oval = old & mask;
    if val != oval {
        let new = (old & !mask) | (val & mask);
        outb(new, (*chip).port + 0x05);
    }
    oval as c_int
}

#[inline]
unsafe fn snd_es18xx_mixer_writable(chip: *mut snd_es18xx, reg: u8, mask: u8) -> c_int {
    lock_guard(&mut (*chip).mixer_lock);
    outb(reg, (*chip).port + 0x04);
    let old = inb((*chip).port + 0x05);
    let expected = old ^ mask;
    outb(expected, (*chip).port + 0x05);
    let new = inb((*chip).port + 0x05);
    (expected == new) as c_int
}

unsafe fn snd_es18xx_reset(chip: *mut snd_es18xx) -> c_int {
    outb(0x03, (*chip).port + 0x06);
    inb((*chip).port + 0x06);
    outb(0x00, (*chip).port + 0x06);
    let mut i = 0;
    while i < MILLISECOND && (inb((*chip).port + 0x0e) & 0x80) == 0 {
        i += 1;
    }
    if inb((*chip).port + 0x0a) != 0xaa {
        return -1;
    }
    0
}

unsafe fn snd_es18xx_reset_fifo(chip: *mut snd_es18xx) -> c_int {
    outb(0x02, (*chip).port + 0x06);
    inb((*chip).port + 0x06);
    outb(0x00, (*chip).port + 0x06);
    0
}

static new_clocks: [snd_ratnum; 2] = [
    snd_ratnum { num: 793800, den_min: 1, den_max: 128, den_step: 1 },
    snd_ratnum { num: 768000, den_min: 1, den_max: 128, den_step: 1 },
];

static new_hw_constraints_clocks: snd_pcm_hw_constraint_ratnums =
    snd_pcm_hw_constraint_ratnums { nrats: 2, rats: new_clocks.as_ptr() };

static old_clocks: [snd_ratnum; 2] = [
    snd_ratnum { num: 795444, den_min: 1, den_max: 128, den_step: 1 },
    snd_ratnum { num: 397722, den_min: 1, den_max: 128, den_step: 1 },
];

static old_hw_constraints_clocks: snd_pcm_hw_constraint_ratnums =
    snd_pcm_hw_constraint_ratnums { nrats: 2, rats: old_clocks.as_ptr() };

unsafe fn snd_es18xx_rate_set(chip: *mut snd_es18xx, substream: *mut snd_pcm_substream, mode: c_int) {
    let runtime = (*substream).runtime;
    let bits: c_uint = if ((*chip).caps & ES18XX_NEW_RATE) != 0 {
        if (*runtime).rate_num == new_clocks[0].num { 128 - (*runtime).rate_den } else { 256 - (*runtime).rate_den }
    } else if (*runtime).rate_num == old_clocks[0].num {
        256 - (*runtime).rate_den
    } else {
        128 - (*runtime).rate_den
    };

    /* set filter register */
    let div0 = 256u32.wrapping_sub(7160000u32.wrapping_mul(20) / (8 * 82 * (*runtime).rate));

    if ((*chip).caps & ES18XX_PCM2) != 0 && mode == DAC2 as c_int {
        snd_es18xx_mixer_write(chip, 0x70, bits as u8);
        /*
         * Comment from kernel oss driver:
         * FKS: fascinating: 0x72 doesn't seem to work.
         */
        snd_es18xx_write(chip, 0xa2, div0 as u8);
        snd_es18xx_mixer_write(chip, 0x72, div0 as u8);
    } else {
        snd_es18xx_write(chip, 0xa1, bits as u8);
        snd_es18xx_write(chip, 0xa2, div0 as u8);
    }
}

unsafe extern "C" fn snd_es18xx_playback_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut shift = 0;
    if params_channels(hw_params) == 2 { shift += 1; }
    if snd_pcm_format_width(params_format(hw_params)) == 16 { shift += 1; }
    if (*substream).number == 0 && ((*chip).caps & ES18XX_PCM2) != 0 {
        if ((*chip).caps & ES18XX_DUPLEX_MONO) != 0 && !(*chip).capture_a_substream.is_null() && params_channels(hw_params) != 1 {
            _snd_pcm_hw_param_setempty(hw_params, SNDRV_PCM_HW_PARAM_CHANNELS);
            return -EBUSY;
        }
        (*chip).dma2_shift = shift as c_uint;
    } else {
        (*chip).dma1_shift = shift as c_uint;
    }
    0
}

unsafe fn snd_es18xx_playback1_prepare(chip: *mut snd_es18xx, substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let size = snd_pcm_lib_buffer_bytes(substream);
    let mut count = snd_pcm_lib_period_bytes(substream);
    snd_es18xx_rate_set(chip, substream, DAC2 as c_int);
    /* Transfer Count Reload */
    count = 0x10000u32.wrapping_sub(count);
    snd_es18xx_mixer_write(chip, 0x74, (count & 0xff) as u8);
    snd_es18xx_mixer_write(chip, 0x76, (count >> 8) as u8);
    /* Set format */
    snd_es18xx_mixer_bits(
        chip,
        0x7a,
        0x07,
        (((*runtime).channels == 1) as u8).wrapping_sub(1) & 0x02
            | if snd_pcm_format_width((*runtime).format) == 16 { 0x01 } else { 0x00 }
            | if snd_pcm_format_unsigned((*runtime).format) != 0 { 0x00 } else { 0x04 },
    );
    /* Set DMA controller */
    snd_dma_program((*chip).dma2, (*runtime).dma_addr, size, DMA_MODE_WRITE | DMA_AUTOINIT);
    0
}

unsafe fn snd_es18xx_playback1_trigger(chip: *mut snd_es18xx, _substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            if ((*chip).active & DAC2) != 0 { return 0; }
            (*chip).active |= DAC2;
            /* Start DMA */
            if (*chip).dma2 >= 4 { snd_es18xx_mixer_write(chip, 0x78, 0xb3); } else { snd_es18xx_mixer_write(chip, 0x78, 0x93); }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            if ((*chip).active & DAC2) == 0 { return 0; }
            (*chip).active &= !DAC2;
            /* Stop DMA */
            snd_es18xx_mixer_write(chip, 0x78, 0x00);
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn snd_es18xx_capture_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut shift = 0;
    if ((*chip).caps & ES18XX_DUPLEX_MONO) != 0 && !(*chip).playback_a_substream.is_null() && params_channels(hw_params) != 1 {
        _snd_pcm_hw_param_setempty(hw_params, SNDRV_PCM_HW_PARAM_CHANNELS);
        return -EBUSY;
    }
    if params_channels(hw_params) == 2 { shift += 1; }
    if snd_pcm_format_width(params_format(hw_params)) == 16 { shift += 1; }
    (*chip).dma1_shift = shift as c_uint;
    0
}

unsafe extern "C" fn snd_es18xx_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let size = snd_pcm_lib_buffer_bytes(substream);
    let mut count = snd_pcm_lib_period_bytes(substream);
    snd_es18xx_reset_fifo(chip);
    /* Set stereo/mono */
    snd_es18xx_bits(chip, 0xa8, 0x03, if (*runtime).channels == 1 { 0x02 } else { 0x01 });
    snd_es18xx_rate_set(chip, substream, ADC1 as c_int);
    /* Transfer Count Reload */
    count = 0x10000u32.wrapping_sub(count);
    snd_es18xx_write(chip, 0xa4, (count & 0xff) as u8);
    snd_es18xx_write(chip, 0xa5, (count >> 8) as u8);
    /* Set format */
    snd_es18xx_write(chip, 0xb7, if snd_pcm_format_unsigned((*runtime).format) != 0 { 0x51 } else { 0x71 });
    snd_es18xx_write(
        chip,
        0xb7,
        0x90 | if (*runtime).channels == 1 { 0x40 } else { 0x08 }
            | if snd_pcm_format_width((*runtime).format) == 16 { 0x04 } else { 0x00 }
            | if snd_pcm_format_unsigned((*runtime).format) != 0 { 0x00 } else { 0x20 },
    );
    /* Set DMA controller */
    snd_dma_program((*chip).dma1, (*runtime).dma_addr, size, DMA_MODE_READ | DMA_AUTOINIT);
    0
}

unsafe extern "C" fn snd_es18xx_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            if ((*chip).active & ADC1) != 0 { return 0; }
            (*chip).active |= ADC1;
            /* Start DMA */
            snd_es18xx_write(chip, 0xb8, 0x0f);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            if ((*chip).active & ADC1) == 0 { return 0; }
            (*chip).active &= !ADC1;
            /* Stop DMA */
            snd_es18xx_write(chip, 0xb8, 0x00);
        }
        _ => return -EINVAL,
    }
    0
}

unsafe fn snd_es18xx_playback2_prepare(chip: *mut snd_es18xx, substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let size = snd_pcm_lib_buffer_bytes(substream);
    let mut count = snd_pcm_lib_period_bytes(substream);
    snd_es18xx_reset_fifo(chip);
    /* Set stereo/mono */
    snd_es18xx_bits(chip, 0xa8, 0x03, if (*runtime).channels == 1 { 0x02 } else { 0x01 });
    snd_es18xx_rate_set(chip, substream, DAC1 as c_int);
    /* Transfer Count Reload */
    count = 0x10000u32.wrapping_sub(count);
    snd_es18xx_write(chip, 0xa4, (count & 0xff) as u8);
    snd_es18xx_write(chip, 0xa5, (count >> 8) as u8);
    /* Set format */
    snd_es18xx_write(chip, 0xb6, if snd_pcm_format_unsigned((*runtime).format) != 0 { 0x80 } else { 0x00 });
    snd_es18xx_write(chip, 0xb7, if snd_pcm_format_unsigned((*runtime).format) != 0 { 0x51 } else { 0x71 });
    snd_es18xx_write(
        chip,
        0xb7,
        0x90 | if (*runtime).channels == 1 { 0x40 } else { 0x08 }
            | if snd_pcm_format_width((*runtime).format) == 16 { 0x04 } else { 0x00 }
            | if snd_pcm_format_unsigned((*runtime).format) != 0 { 0x00 } else { 0x20 },
    );
    /* Set DMA controller */
    snd_dma_program((*chip).dma1, (*runtime).dma_addr, size, DMA_MODE_WRITE | DMA_AUTOINIT);
    0
}

unsafe fn snd_es18xx_playback2_trigger(chip: *mut snd_es18xx, _substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            if ((*chip).active & DAC1) != 0 { return 0; }
            (*chip).active |= DAC1;
            /* Start DMA */
            snd_es18xx_write(chip, 0xb8, 0x05);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            if ((*chip).active & DAC1) == 0 { return 0; }
            (*chip).active &= !DAC1;
            /* Stop DMA */
            snd_es18xx_write(chip, 0xb8, 0x00);
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn snd_es18xx_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    if (*substream).number == 0 && ((*chip).caps & ES18XX_PCM2) != 0 {
        snd_es18xx_playback1_prepare(chip, substream)
    } else {
        snd_es18xx_playback2_prepare(chip, substream)
    }
}

unsafe extern "C" fn snd_es18xx_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    if (*substream).number == 0 && ((*chip).caps & ES18XX_PCM2) != 0 {
        snd_es18xx_playback1_trigger(chip, substream, cmd)
    } else {
        snd_es18xx_playback2_trigger(chip, substream, cmd)
    }
}

unsafe extern "C" fn snd_es18xx_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let card = dev_id as *mut snd_card;
    let chip = (*card).private_data as *mut snd_es18xx;
    let status: u8 = if ((*chip).caps & ES18XX_CONTROL) != 0 {
        /* Read Interrupt status */
        inb((*chip).ctrl_port + 6)
    } else {
        /* Read Interrupt status */
        (snd_es18xx_mixer_read(chip, 0x7f) >> 4) as u8
    };

    /* Audio 1 & Audio 2 */
    if (status & AUDIO2_IRQ) != 0 {
        if ((*chip).active & DAC2) != 0 {
            snd_pcm_period_elapsed((*chip).playback_a_substream);
        }
        /* ack interrupt */
        snd_es18xx_mixer_bits(chip, 0x7a, 0x80, 0x00);
    }
    if (status & AUDIO1_IRQ) != 0 {
        /* ok.. capture is active */
        if ((*chip).active & ADC1) != 0 {
            snd_pcm_period_elapsed((*chip).capture_a_substream);
        /* ok.. playback2 is active */
        } else if ((*chip).active & DAC1) != 0 {
            snd_pcm_period_elapsed((*chip).playback_b_substream);
        }
        /* ack interrupt */
        inb((*chip).port + 0x0e);
    }

    /* MPU */
    if (status & MPU_IRQ) != 0 && !(*chip).rmidi.is_null() {
        snd_mpu401_uart_interrupt(irq, (*(*chip).rmidi).private_data);
    }

    /* Hardware volume */
    if (status & HWV_IRQ) != 0 {
        let mut split = 0;
        if ((*chip).caps & ES18XX_HWV) != 0 {
            split = snd_es18xx_mixer_read(chip, 0x64) & 0x80;
            snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).hw_switch).id);
            snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).hw_volume).id);
        }
        if split == 0 {
            snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).master_switch).id);
            snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).master_volume).id);
        }
        /* ack interrupt */
        snd_es18xx_mixer_write(chip, 0x66, 0x00);
    }
    IRQ_HANDLED
}

unsafe extern "C" fn snd_es18xx_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let size = snd_pcm_lib_buffer_bytes(substream);
    if (*substream).number == 0 && ((*chip).caps & ES18XX_PCM2) != 0 {
        if ((*chip).active & DAC2) == 0 { return 0; }
        (snd_dma_pointer((*chip).dma2, size) as c_uint >> (*chip).dma2_shift) as snd_pcm_uframes_t
    } else {
        if ((*chip).active & DAC1) == 0 { return 0; }
        (snd_dma_pointer((*chip).dma1, size) as c_uint >> (*chip).dma1_shift) as snd_pcm_uframes_t
    }
}

unsafe extern "C" fn snd_es18xx_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let size = snd_pcm_lib_buffer_bytes(substream);
    if ((*chip).active & ADC1) == 0 { return 0; }
    (snd_dma_pointer((*chip).dma1, size) as c_uint >> (*chip).dma1_shift) as snd_pcm_uframes_t
}

static snd_es18xx_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE,
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

static snd_es18xx_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_RESUME | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE,
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

unsafe extern "C" fn snd_es18xx_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let chip = snd_pcm_substream_chip(substream);
    if (*substream).number == 0 && ((*chip).caps & ES18XX_PCM2) != 0 {
        if ((*chip).caps & ES18XX_DUPLEX_MONO) != 0 && !(*chip).capture_a_substream.is_null() && (*(*(*chip).capture_a_substream).runtime).channels != 1 {
            return -EAGAIN;
        }
        (*chip).playback_a_substream = substream;
    } else if (*substream).number <= 1 {
        if !(*chip).capture_a_substream.is_null() {
            return -EAGAIN;
        }
        (*chip).playback_b_substream = substream;
    } else {
        snd_BUG();
        return -EINVAL;
    }
    (*runtime).hw = snd_es18xx_playback;
    snd_pcm_hw_constraint_ratnums(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        if ((*chip).caps & ES18XX_NEW_RATE) != 0 { &new_hw_constraints_clocks } else { &old_hw_constraints_clocks },
    );
    0
}

unsafe extern "C" fn snd_es18xx_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let chip = snd_pcm_substream_chip(substream);
    if !(*chip).playback_b_substream.is_null() {
        return -EAGAIN;
    }
    if ((*chip).caps & ES18XX_DUPLEX_MONO) != 0 && !(*chip).playback_a_substream.is_null() && (*(*(*chip).playback_a_substream).runtime).channels != 1 {
        return -EAGAIN;
    }
    (*chip).capture_a_substream = substream;
    (*runtime).hw = snd_es18xx_capture;
    snd_pcm_hw_constraint_ratnums(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        if ((*chip).caps & ES18XX_NEW_RATE) != 0 { &new_hw_constraints_clocks } else { &old_hw_constraints_clocks },
    );
    0
}

unsafe extern "C" fn snd_es18xx_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    if (*substream).number == 0 && ((*chip).caps & ES18XX_PCM2) != 0 {
        (*chip).playback_a_substream = ptr::null_mut();
    } else {
        (*chip).playback_b_substream = ptr::null_mut();
    }
    0
}

unsafe extern "C" fn snd_es18xx_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    (*chip).capture_a_substream = ptr::null_mut();
    0
}

/*
 *  MIXER part
 */

/* Record source mux routines:
 * Depending on the chipset this mux switches between 4, 5, or 8 possible inputs.
 * bit table for the 4/5 source mux:
 * reg 1C:
 *  b2 b1 b0   muxSource
 *   x  0  x   microphone
 *   0  1  x   CD
 *   1  1  0   line
 *   1  1  1   mixer
 * if it's "mixer" and it's a 5 source mux chipset then reg 7A bit 3 determines
 * either the play mixer or the capture mixer.
 *
 * "map4Source" translates from source number to reg bit pattern
 * "invMap4Source" translates from reg bit pattern to source number
 */

unsafe extern "C" fn snd_es18xx_info_mux(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static texts5Source: [*const c_char; 5] = [c"Mic".as_ptr(), c"CD".as_ptr(), c"Line".as_ptr(), c"Master".as_ptr(), c"Mix".as_ptr()];
    static texts8Source: [*const c_char; 8] = [c"Mic".as_ptr(), c"Mic Master".as_ptr(), c"CD".as_ptr(), c"AOUT".as_ptr(), c"Mic1".as_ptr(), c"Mix".as_ptr(), c"Line".as_ptr(), c"Master".as_ptr()];
    let chip = snd_kcontrol_chip(kcontrol);
    match (*chip).version as c_int {
        0x1868 | 0x1878 => snd_ctl_enum_info(uinfo, 1, 4, texts5Source.as_ptr()),
        0x1887 | 0x1888 => snd_ctl_enum_info(uinfo, 1, 5, texts5Source.as_ptr()),
        0x1869 | 0x1879 => snd_ctl_enum_info(uinfo, 1, 8, texts8Source.as_ptr()),
        _ => -EINVAL,
    }
}

unsafe extern "C" fn snd_es18xx_get_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    static invMap4Source: [u8; 8] = [0, 0, 1, 1, 0, 0, 2, 3];
    let chip = snd_kcontrol_chip(kcontrol);
    let mut muxSource = snd_es18xx_mixer_read(chip, 0x1c) & 0x07;
    if !((*chip).version == 0x1869 || (*chip).version == 0x1879) {
        muxSource = invMap4Source[muxSource as usize] as c_int;
        if muxSource == 3 && ((*chip).version == 0x1887 || (*chip).version == 0x1888) && (snd_es18xx_mixer_read(chip, 0x7a) & 0x08) != 0 {
            muxSource = 4;
        }
    }
    (*ucontrol).value.enumerated.item[0] = muxSource as c_uint;
    0
}

unsafe extern "C" fn snd_es18xx_put_mux(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    static map4Source: [u8; 4] = [0, 2, 6, 7];
    let chip = snd_kcontrol_chip(kcontrol);
    let mut val = (*ucontrol).value.enumerated.item[0] as u8;
    let mut retVal: c_int = 0;
    match (*chip).version as c_int {
        0x1887 | 0x1888 => {
            if val > 4 { return -EINVAL; }
            if val == 4 {
                retVal = (snd_es18xx_mixer_bits(chip, 0x7a, 0x08, 0x08) != 0x08) as c_int;
                val = 3;
            } else {
                retVal = (snd_es18xx_mixer_bits(chip, 0x7a, 0x08, 0x00) != 0x00) as c_int;
            }
            if val > 3 { return -EINVAL; }
            val = map4Source[val as usize];
        }
        0x1868 | 0x1878 => {
            if val > 3 { return -EINVAL; }
            val = map4Source[val as usize];
        }
        0x1869 | 0x1879 => {
            if val > 7 { return -EINVAL; }
        }
        _ => return -EINVAL,
    }
    ((snd_es18xx_mixer_bits(chip, 0x1c, 0x07, val) != val as c_int) as c_int) | retVal
}

unsafe extern "C" fn snd_es18xx_info_spatializer_enable(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    snd_ctl_boolean_mono_info(kcontrol, uinfo)
}

unsafe extern "C" fn snd_es18xx_get_spatializer_enable(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let val = snd_es18xx_mixer_read(chip, 0x50) as u8;
    (*ucontrol).value.integer.value[0] = ((val & 8) != 0) as c_long;
    0
}

unsafe extern "C" fn snd_es18xx_put_spatializer_enable(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let nval: u8 = if (*ucontrol).value.integer.value[0] != 0 { 0x0c } else { 0x04 };
    let oval = snd_es18xx_mixer_read(chip, 0x50) as u8 & 0x0c;
    let change = nval != oval;
    if change {
        snd_es18xx_mixer_write(chip, 0x50, nval & !0x04);
        snd_es18xx_mixer_write(chip, 0x50, nval);
    }
    change as c_int
}

unsafe extern "C" fn snd_es18xx_info_hw_volume(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 63;
    0
}

unsafe extern "C" fn snd_es18xx_get_hw_volume(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = (snd_es18xx_mixer_read(chip, 0x61) & 0x3f) as c_long;
    (*ucontrol).value.integer.value[1] = (snd_es18xx_mixer_read(chip, 0x63) & 0x3f) as c_long;
    0
}

unsafe extern "C" fn snd_es18xx_info_hw_switch(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    snd_ctl_boolean_stereo_info(kcontrol, uinfo)
}

unsafe extern "C" fn snd_es18xx_get_hw_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = ((snd_es18xx_mixer_read(chip, 0x61) & 0x40) == 0) as c_long;
    (*ucontrol).value.integer.value[1] = ((snd_es18xx_mixer_read(chip, 0x63) & 0x40) == 0) as c_long;
    0
}

unsafe extern "C" fn snd_es18xx_hwv_free(kcontrol: *mut snd_kcontrol) {
    let chip = snd_kcontrol_chip(kcontrol);
    (*chip).master_volume = ptr::null_mut();
    (*chip).master_switch = ptr::null_mut();
    (*chip).hw_volume = ptr::null_mut();
    (*chip).hw_switch = ptr::null_mut();
}

unsafe fn snd_es18xx_reg_bits(chip: *mut snd_es18xx, reg: u8, mask: u8, val: u8) -> c_int {
    if reg < 0xa0 { snd_es18xx_mixer_bits(chip, reg, mask, val) } else { snd_es18xx_bits(chip, reg, mask, val) }
}

unsafe fn snd_es18xx_reg_read(chip: *mut snd_es18xx, reg: u8) -> c_int {
    if reg < 0xa0 { snd_es18xx_mixer_read(chip, reg) } else { snd_es18xx_read(chip, reg) }
}

const ES18XX_FL_INVERT: c_int = 1 << 0;
const ES18XX_FL_PMPORT: c_int = 1 << 1;

const fn ES18XX_SINGLE_VALUE(reg: c_ulong, shift: c_ulong, mask: c_ulong, flags: c_ulong) -> c_ulong {
    reg | (shift << 8) | (mask << 16) | (flags << 24)
}

const fn ES18XX_DOUBLE_VALUE(left_reg: c_ulong, right_reg: c_ulong, shift_left: c_ulong, shift_right: c_ulong, mask: c_ulong, invert: c_ulong) -> c_ulong {
    left_reg | (right_reg << 8) | (shift_left << 16) | (shift_right << 19) | (mask << 24) | (invert << 22)
}

const fn ES18XX_SINGLE_KCTL(name: *const c_char, index: c_uint, reg: c_ulong, shift: c_ulong, mask: c_ulong, flags: c_ulong) -> snd_kcontrol_new {
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name, index, access: 0, info: Some(snd_es18xx_info_single), get: Some(snd_es18xx_get_single), put: Some(snd_es18xx_put_single), private_value: ES18XX_SINGLE_VALUE(reg, shift, mask, flags) }
}

const fn ES18XX_DOUBLE_KCTL(name: *const c_char, index: c_uint, left_reg: c_ulong, right_reg: c_ulong, shift_left: c_ulong, shift_right: c_ulong, mask: c_ulong, invert: c_ulong) -> snd_kcontrol_new {
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name, index, access: 0, info: Some(snd_es18xx_info_double), get: Some(snd_es18xx_get_double), put: Some(snd_es18xx_put_double), private_value: ES18XX_DOUBLE_VALUE(left_reg, right_reg, shift_left, shift_right, mask, invert) }
}

unsafe extern "C" fn snd_es18xx_info_single(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = ((*kcontrol).private_value >> 16) & 0xff;
    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_es18xx_get_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as u8;
    let shift = ((*kcontrol).private_value >> 8) & 0xff;
    let mask = ((*kcontrol).private_value >> 16) & 0xff;
    let invert = ((*kcontrol).private_value >> 24) & ES18XX_FL_INVERT as c_ulong;
    let pm_port = ((*kcontrol).private_value >> 24) & ES18XX_FL_PMPORT as c_ulong;
    let val = if pm_port != 0 { inb((*chip).port + ES18XX_PM as c_ulong) as c_int } else { snd_es18xx_reg_read(chip, reg) };
    (*ucontrol).value.integer.value[0] = (((val as c_ulong >> shift) & mask) as c_long);
    if invert != 0 {
        (*ucontrol).value.integer.value[0] = mask as c_long - (*ucontrol).value.integer.value[0];
    }
    0
}

unsafe extern "C" fn snd_es18xx_put_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as u8;
    let shift = ((*kcontrol).private_value >> 8) & 0xff;
    let mut mask = ((*kcontrol).private_value >> 16) & 0xff;
    let invert = ((*kcontrol).private_value >> 24) & ES18XX_FL_INVERT as c_ulong;
    let pm_port = ((*kcontrol).private_value >> 24) & ES18XX_FL_PMPORT as c_ulong;
    let mut val = ((*ucontrol).value.integer.value[0] as c_ulong & mask) as u8;
    if invert != 0 { val = (mask as u8).wrapping_sub(val); }
    mask <<= shift;
    val <<= shift;
    if pm_port != 0 {
        let cur = inb((*chip).port + ES18XX_PM as c_ulong);
        if (cur & mask as u8) == val { return 0; }
        outb((cur & !(mask as u8)) | val, (*chip).port + ES18XX_PM as c_ulong);
        return 1;
    }
    (snd_es18xx_reg_bits(chip, reg, mask as u8, val) != val as c_int) as c_int
}

unsafe extern "C" fn snd_es18xx_info_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let mask = ((*kcontrol).private_value >> 24) & 0xff;
    (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_es18xx_get_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as u8;
    let right_reg = ((*kcontrol).private_value >> 8 & 0xff) as u8;
    let shift_left = ((*kcontrol).private_value >> 16) & 0x07;
    let shift_right = ((*kcontrol).private_value >> 19) & 0x07;
    let mask = ((*kcontrol).private_value >> 24) & 0xff;
    let invert = ((*kcontrol).private_value >> 22) & 1;
    let left = snd_es18xx_reg_read(chip, left_reg) as u8;
    let right = if left_reg != right_reg { snd_es18xx_reg_read(chip, right_reg) as u8 } else { left };
    (*ucontrol).value.integer.value[0] = ((left as c_ulong >> shift_left) & mask) as c_long;
    (*ucontrol).value.integer.value[1] = ((right as c_ulong >> shift_right) & mask) as c_long;
    if invert != 0 {
        (*ucontrol).value.integer.value[0] = mask as c_long - (*ucontrol).value.integer.value[0];
        (*ucontrol).value.integer.value[1] = mask as c_long - (*ucontrol).value.integer.value[1];
    }
    0
}

unsafe extern "C" fn snd_es18xx_put_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as u8;
    let right_reg = ((*kcontrol).private_value >> 8 & 0xff) as u8;
    let shift_left = ((*kcontrol).private_value >> 16) & 0x07;
    let shift_right = ((*kcontrol).private_value >> 19) & 0x07;
    let mask = ((*kcontrol).private_value >> 24) & 0xff;
    let invert = ((*kcontrol).private_value >> 22) & 1;
    let mut val1 = ((*ucontrol).value.integer.value[0] as c_ulong & mask) as u8;
    let mut val2 = ((*ucontrol).value.integer.value[1] as c_ulong & mask) as u8;
    if invert != 0 {
        val1 = (mask as u8).wrapping_sub(val1);
        val2 = (mask as u8).wrapping_sub(val2);
    }
    val1 <<= shift_left;
    val2 <<= shift_right;
    let mask1 = (mask as u8) << shift_left;
    let mask2 = (mask as u8) << shift_right;
    if left_reg != right_reg {
        let mut change = 0;
        if snd_es18xx_reg_bits(chip, left_reg, mask1, val1) != val1 as c_int { change = 1; }
        if snd_es18xx_reg_bits(chip, right_reg, mask2, val2) != val2 as c_int { change = 1; }
        change
    } else {
        (snd_es18xx_reg_bits(chip, left_reg, mask1 | mask2, val1 | val2) != (val1 | val2) as c_int) as c_int
    }
}

static snd_es18xx_base_controls: [snd_kcontrol_new; 10] = [
    ES18XX_DOUBLE_KCTL(c"Master Playback Volume".as_ptr(), 0, 0x60, 0x62, 0, 0, 63, 0),
    ES18XX_DOUBLE_KCTL(c"Master Playback Switch".as_ptr(), 0, 0x60, 0x62, 6, 6, 1, 1),
    ES18XX_DOUBLE_KCTL(c"Line Playback Volume".as_ptr(), 0, 0x3e, 0x3e, 4, 0, 15, 0),
    ES18XX_DOUBLE_KCTL(c"CD Playback Volume".as_ptr(), 0, 0x38, 0x38, 4, 0, 15, 0),
    ES18XX_DOUBLE_KCTL(c"FM Playback Volume".as_ptr(), 0, 0x36, 0x36, 4, 0, 15, 0),
    ES18XX_DOUBLE_KCTL(c"Mic Playback Volume".as_ptr(), 0, 0x1a, 0x1a, 4, 0, 15, 0),
    ES18XX_DOUBLE_KCTL(c"Aux Playback Volume".as_ptr(), 0, 0x3a, 0x3a, 4, 0, 15, 0),
    ES18XX_SINGLE_KCTL(c"Record Monitor".as_ptr(), 0, 0xa8, 3, 1, 0),
    ES18XX_DOUBLE_KCTL(c"Capture Volume".as_ptr(), 0, 0xb4, 0xb4, 4, 0, 15, 0),
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Capture Source".as_ptr(), index: 0, access: 0, info: Some(snd_es18xx_info_mux), get: Some(snd_es18xx_get_mux), put: Some(snd_es18xx_put_mux), private_value: 0 },
];

static snd_es18xx_recmix_controls: [snd_kcontrol_new; 6] = [
    ES18XX_DOUBLE_KCTL(c"PCM Capture Volume".as_ptr(), 0, 0x69, 0x69, 4, 0, 15, 0),
    ES18XX_DOUBLE_KCTL(c"Mic Capture Volume".as_ptr(), 0, 0x68, 0x68, 4, 0, 15, 0),
    ES18XX_DOUBLE_KCTL(c"Line Capture Volume".as_ptr(), 0, 0x6e, 0x6e, 4, 0, 15, 0),
    ES18XX_DOUBLE_KCTL(c"FM Capture Volume".as_ptr(), 0, 0x6b, 0x6b, 4, 0, 15, 0),
    ES18XX_DOUBLE_KCTL(c"CD Capture Volume".as_ptr(), 0, 0x6a, 0x6a, 4, 0, 15, 0),
    ES18XX_DOUBLE_KCTL(c"Aux Capture Volume".as_ptr(), 0, 0x6c, 0x6c, 4, 0, 15, 0),
];

/*
 * The chipset specific mixer controls
 */
static snd_es18xx_opt_speaker: snd_kcontrol_new = ES18XX_SINGLE_KCTL(c"Beep Playback Volume".as_ptr(), 0, 0x3c, 0, 7, 0);
static snd_es18xx_opt_1869: [snd_kcontrol_new; 4] = [
    ES18XX_SINGLE_KCTL(c"Capture Switch".as_ptr(), 0, 0x1c, 4, 1, ES18XX_FL_INVERT as c_ulong),
    ES18XX_SINGLE_KCTL(c"Video Playback Switch".as_ptr(), 0, 0x7f, 0, 1, 0),
    ES18XX_DOUBLE_KCTL(c"Mono Playback Volume".as_ptr(), 0, 0x6d, 0x6d, 4, 0, 15, 0),
    ES18XX_DOUBLE_KCTL(c"Mono Capture Volume".as_ptr(), 0, 0x6f, 0x6f, 4, 0, 15, 0),
];
static snd_es18xx_opt_1878: snd_kcontrol_new = ES18XX_DOUBLE_KCTL(c"Video Playback Volume".as_ptr(), 0, 0x68, 0x68, 4, 0, 15, 0);
static snd_es18xx_opt_1879: [snd_kcontrol_new; 3] = [
    ES18XX_SINGLE_KCTL(c"Video Playback Switch".as_ptr(), 0, 0x71, 6, 1, 0),
    ES18XX_DOUBLE_KCTL(c"Video Playback Volume".as_ptr(), 0, 0x6d, 0x6d, 4, 0, 15, 0),
    ES18XX_DOUBLE_KCTL(c"Video Capture Volume".as_ptr(), 0, 0x6f, 0x6f, 4, 0, 15, 0),
];
static snd_es18xx_pcm1_controls: [snd_kcontrol_new; 1] = [
    ES18XX_DOUBLE_KCTL(c"PCM Playback Volume".as_ptr(), 0, 0x14, 0x14, 4, 0, 15, 0),
];
static snd_es18xx_pcm2_controls: [snd_kcontrol_new; 2] = [
    ES18XX_DOUBLE_KCTL(c"PCM Playback Volume".as_ptr(), 0, 0x7c, 0x7c, 4, 0, 15, 0),
    ES18XX_DOUBLE_KCTL(c"PCM Playback Volume".as_ptr(), 1, 0x14, 0x14, 4, 0, 15, 0),
];
static snd_es18xx_spatializer_controls: [snd_kcontrol_new; 2] = [
    ES18XX_SINGLE_KCTL(c"3D Control - Level".as_ptr(), 0, 0x52, 0, 63, 0),
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"3D Control - Switch".as_ptr(), index: 0, access: 0, info: Some(snd_es18xx_info_spatializer_enable), get: Some(snd_es18xx_get_spatializer_enable), put: Some(snd_es18xx_put_spatializer_enable), private_value: 0 },
];
static snd_es18xx_micpre1_control: snd_kcontrol_new = ES18XX_SINGLE_KCTL(c"Mic Boost (+26dB)".as_ptr(), 0, 0xa9, 2, 1, 0);
static snd_es18xx_micpre2_control: snd_kcontrol_new = ES18XX_SINGLE_KCTL(c"Mic Boost (+26dB)".as_ptr(), 0, 0x7d, 3, 1, 0);
static snd_es18xx_hw_volume_controls: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Hardware Master Playback Volume".as_ptr(), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READ, info: Some(snd_es18xx_info_hw_volume), get: Some(snd_es18xx_get_hw_volume), put: None, private_value: 0 },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Hardware Master Playback Switch".as_ptr(), index: 0, access: SNDRV_CTL_ELEM_ACCESS_READ, info: Some(snd_es18xx_info_hw_switch), get: Some(snd_es18xx_get_hw_switch), put: None, private_value: 0 },
    ES18XX_SINGLE_KCTL(c"Hardware Master Volume Split".as_ptr(), 0, 0x64, 7, 1, 0),
];
static snd_es18xx_opt_gpo_2bit: [snd_kcontrol_new; 2] = [
    ES18XX_SINGLE_KCTL(c"GPO0 Switch".as_ptr(), 0, ES18XX_PM as c_ulong, 0, 1, ES18XX_FL_PMPORT as c_ulong),
    ES18XX_SINGLE_KCTL(c"GPO1 Switch".as_ptr(), 0, ES18XX_PM as c_ulong, 1, 1, ES18XX_FL_PMPORT as c_ulong),
];

unsafe fn snd_es18xx_config_read(chip: *mut snd_es18xx, reg: u8) -> c_int {
    outb(reg, (*chip).ctrl_port);
    inb((*chip).ctrl_port + 1) as c_int
}

unsafe fn snd_es18xx_config_write(chip: *mut snd_es18xx, reg: u8, data: u8) {
    /* No need for spinlocks, this function is used only in
       otherwise protected init code */
    outb(reg, (*chip).ctrl_port);
    outb(data, (*chip).ctrl_port + 1);
}

unsafe fn snd_es18xx_initialize(chip: *mut snd_es18xx, mpu_port: c_ulong, fm_port: c_ulong) -> c_int {
    let mut mask = 0;
    /* enable extended mode */
    snd_es18xx_dsp_command(chip, 0xc6);
    /* Reset mixer registers */
    snd_es18xx_mixer_write(chip, 0x00, 0x00);
    /* Audio 1 DMA demand mode (4 bytes/request) */
    snd_es18xx_write(chip, 0xb9, 2);
    if ((*chip).caps & ES18XX_CONTROL) != 0 {
        snd_es18xx_config_write(chip, 0x27, (*chip).irq as u8);
        if fm_port > 0 && fm_port != SNDRV_AUTO_PORT as c_ulong {
            snd_es18xx_config_write(chip, 0x62, (fm_port >> 8) as u8);
            snd_es18xx_config_write(chip, 0x63, (fm_port & 0xff) as u8);
        }
        if mpu_port > 0 && mpu_port != SNDRV_AUTO_PORT as c_ulong {
            snd_es18xx_config_write(chip, 0x64, (mpu_port >> 8) as u8);
            snd_es18xx_config_write(chip, 0x65, (mpu_port & 0xff) as u8);
            snd_es18xx_config_write(chip, 0x28, (*chip).irq as u8);
        }
        snd_es18xx_config_write(chip, 0x70, (*chip).irq as u8);
        snd_es18xx_config_write(chip, 0x72, (*chip).irq as u8);
        snd_es18xx_config_write(chip, 0x74, (*chip).dma1 as u8);
        snd_es18xx_config_write(chip, 0x75, (*chip).dma2 as u8);
        snd_es18xx_write(chip, 0xb1, 0x50);
        snd_es18xx_mixer_write(chip, 0x7a, 0x40);
        snd_es18xx_write(chip, 0xb2, 0x50);
        snd_es18xx_mixer_write(chip, 0x64, 0x42);
        snd_es18xx_mixer_bits(chip, 0x48, 0x10, 0x10);
    } else {
        let irqmask = match (*chip).irq { 2 | 9 => 0, 5 => 1, 7 => 2, 10 => 3, _ => { dev_err((*(*chip).card).dev, c"invalid irq %d\n".as_ptr(), (*chip).irq); return -ENODEV; } };
        let dma1mask = match (*chip).dma1 { 0 => 1, 1 => 2, 3 => 3, _ => { dev_err((*(*chip).card).dev, c"invalid dma1 %d\n".as_ptr(), (*chip).dma1); return -ENODEV; } };
        let dma2mask = match (*chip).dma2 { 0 => 0, 1 => 1, 3 => 2, 5 => 3, _ => { dev_err((*(*chip).card).dev, c"invalid dma2 %d\n".as_ptr(), (*chip).dma2); return -ENODEV; } };
        snd_es18xx_write(chip, 0xb1, (0x50 | (irqmask << 2)) as u8);
        snd_es18xx_write(chip, 0xb2, (0x50 | (dma1mask << 2)) as u8);
        snd_es18xx_mixer_bits(chip, 0x7d, 0x07, (0x04 | dma2mask) as u8);
        snd_es18xx_mixer_write(chip, 0x7a, 0x68);
        snd_es18xx_mixer_write(chip, 0x64, 0x06);
        if mpu_port > 0 && mpu_port != SNDRV_AUTO_PORT as c_ulong {
            snd_es18xx_mixer_write(chip, 0x40, (0x43 | ((mpu_port & 0xf0) >> 1)) as u8);
        }
        snd_es18xx_mixer_write(chip, 0x7f, (((irqmask + 1) << 1) | 0x01) as u8);
    }
    if ((*chip).caps & ES18XX_NEW_RATE) != 0 {
        snd_es18xx_mixer_write(chip, 0x71, 0x32);
    }
    if ((*chip).caps & ES18XX_PCM2) == 0 {
        snd_es18xx_write(chip, 0xb7, 0x80);
    }
    if ((*chip).caps & ES18XX_SPATIALIZER) != 0 {
        snd_es18xx_mixer_write(chip, 0x54, 0x8f);
        snd_es18xx_mixer_write(chip, 0x56, 0x95);
        snd_es18xx_mixer_write(chip, 0x58, 0x94);
        snd_es18xx_mixer_write(chip, 0x5a, 0x80);
    }
    match (*chip).version as c_int {
        0x1879 | 0x1878 => {
            /* Leaving I2S enabled on the 1879 screws up the PCM playback (rate effected somehow). */
            let r = snd_es18xx_config_read(chip, 0x29) | 0x40;
            snd_es18xx_config_write(chip, 0x29, r as u8);
        }
        _ => {}
    }
    if ((*chip).caps & ES18XX_MUTEREC) != 0 { mask = 0x10; }
    if ((*chip).caps & ES18XX_RECMIX) != 0 {
        snd_es18xx_mixer_write(chip, 0x1c, 0x05 | mask);
    } else {
        snd_es18xx_mixer_write(chip, 0x1c, 0x00 | mask);
        snd_es18xx_write(chip, 0xb4, 0x00);
    }
    /* Enable PCM output when AVOID_POPS is not defined. */
    snd_es18xx_dsp_command(chip, 0xd1);
    0
}

unsafe fn snd_es18xx_identify(card: *mut snd_card, chip: *mut snd_es18xx) -> c_int {
    if snd_es18xx_reset(chip) < 0 {
        dev_err((*card).dev, c"reset at 0x%lx failed!!!\n".as_ptr(), (*chip).port);
        return -ENODEV;
    }
    snd_es18xx_dsp_command(chip, 0xe7);
    let mut hi = snd_es18xx_dsp_get_byte(chip);
    if hi < 0 { return hi; }
    let mut lo = snd_es18xx_dsp_get_byte(chip);
    if (lo & 0xf0) != 0x80 { return -ENODEV; }
    if hi == 0x48 { (*chip).version = 0x488; return 0; }
    if hi != 0x68 { return -ENODEV; }
    if (lo & 0x0f) < 8 { (*chip).version = 0x688; return 0; }
    outb(0x40, (*chip).port + 0x04);
    udelay(10);
    hi = inb((*chip).port + 0x05) as c_int;
    udelay(10);
    lo = inb((*chip).port + 0x05) as c_int;
    if hi != lo {
        (*chip).version = ((hi << 8) | lo) as u16;
        (*chip).ctrl_port = (inb((*chip).port + 0x05) as c_ulong) << 8;
        udelay(10);
        (*chip).ctrl_port += inb((*chip).port + 0x05) as c_ulong;
        if devm_request_region((*card).dev, (*chip).ctrl_port, 8, c"ES18xx - CTRL".as_ptr()).is_null() {
            dev_err((*card).dev, c"unable go grab port 0x%lx\n".as_ptr(), (*chip).ctrl_port);
            return -EBUSY;
        }
        return 0;
    }
    if snd_es18xx_mixer_writable(chip, 0x64, 0x04) != 0 {
        if snd_es18xx_mixer_writable(chip, 0x70, 0x7f) != 0 {
            if snd_es18xx_mixer_writable(chip, 0x64, 0x20) != 0 { (*chip).version = 0x1887; } else { (*chip).version = 0x1888; }
        } else {
            (*chip).version = 0x1788;
        }
    } else {
        (*chip).version = 0x1688;
    }
    0
}

unsafe fn snd_es18xx_probe(card: *mut snd_card, chip: *mut snd_es18xx, mpu_port: c_ulong, fm_port: c_ulong) -> c_int {
    if snd_es18xx_identify(card, chip) < 0 {
        dev_err((*card).dev, c"[0x%lx] ESS chip not found\n".as_ptr(), (*chip).port);
        return -ENODEV;
    }
    match (*chip).version as c_int {
        0x1868 => (*chip).caps = ES18XX_DUPLEX_MONO | ES18XX_DUPLEX_SAME | ES18XX_CONTROL | ES18XX_GPO_2BIT,
        0x1869 => (*chip).caps = ES18XX_PCM2 | ES18XX_SPATIALIZER | ES18XX_RECMIX | ES18XX_NEW_RATE | ES18XX_AUXB | ES18XX_MONO | ES18XX_MUTEREC | ES18XX_CONTROL | ES18XX_HWV | ES18XX_GPO_2BIT,
        0x1878 => (*chip).caps = ES18XX_DUPLEX_MONO | ES18XX_DUPLEX_SAME | ES18XX_I2S | ES18XX_CONTROL,
        0x1879 => (*chip).caps = ES18XX_PCM2 | ES18XX_SPATIALIZER | ES18XX_RECMIX | ES18XX_NEW_RATE | ES18XX_AUXB | ES18XX_I2S | ES18XX_CONTROL | ES18XX_HWV,
        0x1887 | 0x1888 => (*chip).caps = ES18XX_PCM2 | ES18XX_RECMIX | ES18XX_AUXB | ES18XX_DUPLEX_SAME | ES18XX_GPO_2BIT,
        _ => {
            dev_err((*card).dev, c"[0x%lx] unsupported chip ES%x\n".as_ptr(), (*chip).port, (*chip).version as c_int);
            return -ENODEV;
        }
    }
    dev_dbg((*card).dev, c"[0x%lx] ESS%x chip found\n".as_ptr(), (*chip).port, (*chip).version as c_int);
    if (*chip).dma1 == (*chip).dma2 {
        (*chip).caps &= !(ES18XX_PCM2 | ES18XX_DUPLEX_SAME);
    }
    snd_es18xx_initialize(chip, mpu_port, fm_port)
}

static snd_es18xx_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_es18xx_playback_open),
    close: Some(snd_es18xx_playback_close),
    hw_params: Some(snd_es18xx_playback_hw_params),
    prepare: Some(snd_es18xx_playback_prepare),
    trigger: Some(snd_es18xx_playback_trigger),
    pointer: Some(snd_es18xx_playback_pointer),
};

static snd_es18xx_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_es18xx_capture_open),
    close: Some(snd_es18xx_capture_close),
    hw_params: Some(snd_es18xx_capture_hw_params),
    prepare: Some(snd_es18xx_capture_prepare),
    trigger: Some(snd_es18xx_capture_trigger),
    pointer: Some(snd_es18xx_capture_pointer),
};

unsafe fn snd_es18xx_pcm(card: *mut snd_card, device: c_int) -> c_int {
    let chip = (*card).private_data as *mut snd_es18xx;
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut str_: [c_char; 16] = [0; 16];
    sprintf(str_.as_mut_ptr(), c"ES%x".as_ptr(), (*chip).version as c_int);
    let err = if ((*chip).caps & ES18XX_PCM2) != 0 {
        snd_pcm_new(card, str_.as_ptr(), device, 2, 1, &mut pcm)
    } else {
        snd_pcm_new(card, str_.as_ptr(), device, 1, 1, &mut pcm)
    };
    if err < 0 { return err; }
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_es18xx_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_es18xx_capture_ops);
    /* global setup */
    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = 0;
    if ((*chip).caps & ES18XX_DUPLEX_SAME) != 0 { (*pcm).info_flags |= SNDRV_PCM_INFO_JOINT_DUPLEX; }
    if ((*chip).caps & ES18XX_PCM2) == 0 { (*pcm).info_flags |= SNDRV_PCM_INFO_HALF_DUPLEX; }
    sprintf((*pcm).name.as_mut_ptr(), c"ESS AudioDrive ES%x".as_ptr(), (*chip).version as c_int);
    (*chip).pcm = pcm;
    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        (*card).dev,
        64 * 1024,
        if (*chip).dma1 > 3 || (*chip).dma2 > 3 { 128 * 1024 } else { 64 * 1024 },
    );
    0
}

/* Power Management support functions */
unsafe fn snd_es18xx_suspend(card: *mut snd_card, _state: pm_message_t) -> c_int {
    let chip = (*card).private_data as *mut snd_es18xx;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    /* power down */
    (*chip).pm_reg = snd_es18xx_read(chip, ES18XX_PM) as u8;
    (*chip).pm_reg |= ES18XX_PM_FM | ES18XX_PM_SUS;
    snd_es18xx_write(chip, ES18XX_PM, (*chip).pm_reg);
    (*chip).pm_reg ^= ES18XX_PM_SUS;
    snd_es18xx_write(chip, ES18XX_PM, (*chip).pm_reg);
    0
}

unsafe fn snd_es18xx_resume(card: *mut snd_card) -> c_int {
    let chip = (*card).private_data as *mut snd_es18xx;
    /* restore PM register, we won't wake till (not 0x07) i/o activity though */
    (*chip).pm_reg ^= ES18XX_PM_FM;
    snd_es18xx_write(chip, ES18XX_PM, (*chip).pm_reg);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

unsafe fn snd_es18xx_new_device(card: *mut snd_card, port_: c_ulong, mpu_port: c_ulong, fm_port: c_ulong, irq_: c_int, dma1_: c_int, dma2_: c_int) -> c_int {
    let chip = (*card).private_data as *mut snd_es18xx;
    (*chip).card = card;
    spin_lock_init(&mut (*chip).reg_lock);
    spin_lock_init(&mut (*chip).mixer_lock);
    (*chip).port = port_;
    (*chip).irq = -1;
    (*chip).dma1 = -1;
    (*chip).dma2 = -1;
    (*chip).audio2_vol = 0x00;
    (*chip).active = 0;
    if devm_request_region((*card).dev, port_, 16, c"ES18xx".as_ptr()).is_null() {
        dev_err((*card).dev, c"unable to grab ports 0x%lx-0x%lx\n".as_ptr(), port_, port_ + 16 - 1);
        return -EBUSY;
    }
    if devm_request_irq((*card).dev, irq_, snd_es18xx_interrupt, 0, c"ES18xx".as_ptr(), card as *mut c_void) != 0 {
        dev_err((*card).dev, c"unable to grab IRQ %d\n".as_ptr(), irq_);
        return -EBUSY;
    }
    (*chip).irq = irq_;
    (*card).sync_irq = (*chip).irq;
    if snd_devm_request_dma((*card).dev, dma1_, c"ES18xx DMA 1".as_ptr()) != 0 {
        dev_err((*card).dev, c"unable to grab DMA1 %d\n".as_ptr(), dma1_);
        return -EBUSY;
    }
    (*chip).dma1 = dma1_;
    if dma2_ != dma1_ && snd_devm_request_dma((*card).dev, dma2_, c"ES18xx DMA 2".as_ptr()) != 0 {
        dev_err((*card).dev, c"unable to grab DMA2 %d\n".as_ptr(), dma2_);
        return -EBUSY;
    }
    (*chip).dma2 = dma2_;
    if snd_es18xx_probe(card, chip, mpu_port, fm_port) < 0 { return -ENODEV; }
    0
}

unsafe fn add_controls(card: *mut snd_card, chip: *mut snd_es18xx, controls: *const snd_kcontrol_new, count: usize) -> c_int {
    let mut idx = 0usize;
    while idx < count {
        let err = snd_ctl_add(card, snd_ctl_new1(controls.add(idx), chip as *mut c_void));
        if err < 0 { return err; }
        idx += 1;
    }
    0
}

unsafe fn snd_es18xx_mixer(card: *mut snd_card) -> c_int {
    let chip = (*card).private_data as *mut snd_es18xx;
    strscpy((*card).mixername.as_mut_ptr(), (*(*chip).pcm).name.as_ptr());
    let mut idx = 0usize;
    while idx < snd_es18xx_base_controls.len() {
        let kctl = snd_ctl_new1(&snd_es18xx_base_controls[idx], chip as *mut c_void);
        if kctl.is_null() { return -ENOMEM; }
        if ((*chip).caps & ES18XX_HWV) != 0 {
            match idx {
                0 => { (*chip).master_volume = kctl; (*kctl).private_free = Some(snd_es18xx_hwv_free); }
                1 => { (*chip).master_switch = kctl; (*kctl).private_free = Some(snd_es18xx_hwv_free); }
                _ => {}
            }
        }
        let err = snd_ctl_add(card, kctl);
        if err < 0 { return err; }
        idx += 1;
    }
    let mut err = if ((*chip).caps & ES18XX_PCM2) != 0 {
        add_controls(card, chip, snd_es18xx_pcm2_controls.as_ptr(), snd_es18xx_pcm2_controls.len())
    } else {
        add_controls(card, chip, snd_es18xx_pcm1_controls.as_ptr(), snd_es18xx_pcm1_controls.len())
    };
    if err < 0 { return err; }
    if ((*chip).caps & ES18XX_RECMIX) != 0 {
        err = add_controls(card, chip, snd_es18xx_recmix_controls.as_ptr(), snd_es18xx_recmix_controls.len());
        if err < 0 { return err; }
    }
    match (*chip).version as c_int {
        0x1869 | 0x1879 => {
            err = snd_ctl_add(card, snd_ctl_new1(&snd_es18xx_micpre2_control, chip as *mut c_void));
            if err < 0 { return err; }
        }
        _ => {
            err = snd_ctl_add(card, snd_ctl_new1(&snd_es18xx_micpre1_control, chip as *mut c_void));
            if err < 0 { return err; }
        }
    }
    if ((*chip).caps & ES18XX_SPATIALIZER) != 0 {
        err = add_controls(card, chip, snd_es18xx_spatializer_controls.as_ptr(), snd_es18xx_spatializer_controls.len());
        if err < 0 { return err; }
    }
    if ((*chip).caps & ES18XX_HWV) != 0 {
        idx = 0;
        while idx < snd_es18xx_hw_volume_controls.len() {
            let kctl = snd_ctl_new1(&snd_es18xx_hw_volume_controls[idx], chip as *mut c_void);
            if kctl.is_null() { return -ENOMEM; }
            if idx == 0 { (*chip).hw_volume = kctl; } else { (*chip).hw_switch = kctl; }
            (*kctl).private_free = Some(snd_es18xx_hwv_free);
            err = snd_ctl_add(card, kctl);
            if err < 0 { return err; }
            idx += 1;
        }
    }
    /* finish initializing other chipset specific controls */
    if (*chip).version != 0x1868 {
        err = snd_ctl_add(card, snd_ctl_new1(&snd_es18xx_opt_speaker, chip as *mut c_void));
        if err < 0 { return err; }
    }
    if (*chip).version == 0x1869 {
        err = add_controls(card, chip, snd_es18xx_opt_1869.as_ptr(), snd_es18xx_opt_1869.len());
        if err < 0 { return err; }
    } else if (*chip).version == 0x1878 {
        err = snd_ctl_add(card, snd_ctl_new1(&snd_es18xx_opt_1878, chip as *mut c_void));
        if err < 0 { return err; }
    } else if (*chip).version == 0x1879 {
        err = add_controls(card, chip, snd_es18xx_opt_1879.as_ptr(), snd_es18xx_opt_1879.len());
        if err < 0 { return err; }
    }
    if ((*chip).caps & ES18XX_GPO_2BIT) != 0 {
        err = add_controls(card, chip, snd_es18xx_opt_gpo_2bit.as_ptr(), snd_es18xx_opt_gpo_2bit.len());
        if err < 0 { return err; }
    }
    0
}

/* Card level */

/* MODULE_AUTHOR("Christian Fischbach <fishbach@pool.informatik.rwth-aachen.de>, Abramo Bagnara <abramo@alsa-project.org>"); */
/* MODULE_DESCRIPTION("ESS ES18xx AudioDrive"); */
/* MODULE_LICENSE("GPL"); */

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_ISAPNP; /* Enable this card */
static mut isapnp: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_ISAPNP;
static mut port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* 0x220,0x240,0x260,0x280 */
static mut mpu_port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
static mut fm_port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
static mut irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 5,7,9,10 */
static mut dma1: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA; /* 0,1,3 */
static mut dma2: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA; /* 0,1,3 */

/* module_param_array/module_param_hw_array/MODULE_PARM_DESC declarations are module metadata in C. */

static mut isa_registered: c_int = 0;
static mut pnp_registered: c_int = 0;
static mut pnpc_registered: c_int = 0;

static snd_audiodrive_pnpbiosids: [pnp_device_id; 3] = [
    pnp_device_id { id: *b"ESS1869\0" as [u8; 8] as [c_char; 8] },
    pnp_device_id { id: *b"ESS1879\0" as [u8; 8] as [c_char; 8] },
    pnp_device_id { id: [0; 8] },
];

unsafe fn snd_audiodrive_pnp_init_main(dev: c_int, pdev: *mut pnp_dev) -> c_int {
    if pnp_activate_dev(pdev) < 0 {
        dev_err(&mut (*pdev).dev, c"PnP configure failure (out of resources?)\n".as_ptr());
        return -EBUSY;
    }
    /* ok. hack using Vendor-Defined Card-Level registers */
    /* skip csn and logdev initialization - already done in isapnp_configure */
    if pnp_device_is_isapnp(pdev) != 0 {
        isapnp_cfg_begin(isapnp_card_number(pdev), isapnp_csn_number(pdev));
        isapnp_write_byte(0x27, pnp_irq(pdev, 0)); /* Hardware Volume IRQ Number */
        if mpu_port[dev as usize] != SNDRV_AUTO_PORT { isapnp_write_byte(0x28, pnp_irq(pdev, 0)); }
        isapnp_write_byte(0x72, pnp_irq(pdev, 0)); /* second IRQ */
        isapnp_cfg_end();
    }
    port[dev as usize] = pnp_port_start(pdev, 0) as c_long;
    fm_port[dev as usize] = pnp_port_start(pdev, 1) as c_long;
    mpu_port[dev as usize] = pnp_port_start(pdev, 2) as c_long;
    dma1[dev as usize] = pnp_dma(pdev, 0);
    dma2[dev as usize] = pnp_dma(pdev, 1);
    irq[dev as usize] = pnp_irq(pdev, 0);
    dev_dbg(&mut (*pdev).dev, c"PnP ES18xx: port=0x%lx, fm port=0x%lx, mpu port=0x%lx\n".as_ptr(), port[dev as usize], fm_port[dev as usize], mpu_port[dev as usize]);
    dev_dbg(&mut (*pdev).dev, c"PnP ES18xx: dma1=%i, dma2=%i, irq=%i\n".as_ptr(), dma1[dev as usize], dma2[dev as usize], irq[dev as usize]);
    0
}

unsafe fn snd_audiodrive_pnp(dev_: c_int, chip: *mut snd_es18xx, pdev: *mut pnp_dev) -> c_int {
    (*chip).dev = pdev;
    if snd_audiodrive_pnp_init_main(dev_, (*chip).dev) < 0 { return -EBUSY; }
    0
}

static snd_audiodrive_pnpids: [pnp_card_device_id; 8] = [
    pnp_card_device_id { id: *b"ESS1868\0" as [u8; 8] as [c_char; 8], devs: [pnp_card_device { id: *b"ESS1868\0" as [u8; 8] as [c_char; 8] }, pnp_card_device { id: *b"ESS0000\0" as [u8; 8] as [c_char; 8] }] },
    pnp_card_device_id { id: *b"ESS1868\0" as [u8; 8] as [c_char; 8], devs: [pnp_card_device { id: *b"ESS8601\0" as [u8; 8] as [c_char; 8] }, pnp_card_device { id: *b"ESS8600\0" as [u8; 8] as [c_char; 8] }] },
    pnp_card_device_id { id: *b"ESS1868\0" as [u8; 8] as [c_char; 8], devs: [pnp_card_device { id: *b"ESS8611\0" as [u8; 8] as [c_char; 8] }, pnp_card_device { id: *b"ESS8610\0" as [u8; 8] as [c_char; 8] }] },
    pnp_card_device_id { id: *b"ESS0003\0" as [u8; 8] as [c_char; 8], devs: [pnp_card_device { id: *b"ESS1869\0" as [u8; 8] as [c_char; 8] }, pnp_card_device { id: *b"ESS0006\0" as [u8; 8] as [c_char; 8] }] },
    pnp_card_device_id { id: *b"ESS1869\0" as [u8; 8] as [c_char; 8], devs: [pnp_card_device { id: *b"ESS1869\0" as [u8; 8] as [c_char; 8] }, pnp_card_device { id: *b"ESS0006\0" as [u8; 8] as [c_char; 8] }] },
    pnp_card_device_id { id: *b"ESS1878\0" as [u8; 8] as [c_char; 8], devs: [pnp_card_device { id: *b"ESS1878\0" as [u8; 8] as [c_char; 8] }, pnp_card_device { id: *b"ESS0004\0" as [u8; 8] as [c_char; 8] }] },
    pnp_card_device_id { id: *b"ESS1879\0" as [u8; 8] as [c_char; 8], devs: [pnp_card_device { id: *b"ESS1879\0" as [u8; 8] as [c_char; 8] }, pnp_card_device { id: *b"ESS0009\0" as [u8; 8] as [c_char; 8] }] },
    pnp_card_device_id { id: [0; 8], devs: [pnp_card_device { id: [0; 8] }, pnp_card_device { id: [0; 8] }] },
];

unsafe fn snd_audiodrive_pnpc(dev_: c_int, chip: *mut snd_es18xx, card: *mut pnp_card_link, id_: *const pnp_card_device_id) -> c_int {
    (*chip).dev = pnp_request_card_device(card, (*id_).devs[0].id.as_ptr(), ptr::null_mut());
    if (*chip).dev.is_null() { return -EBUSY; }
    (*chip).devc = pnp_request_card_device(card, (*id_).devs[1].id.as_ptr(), ptr::null_mut());
    if (*chip).devc.is_null() { return -EBUSY; }
    /* Control port initialization */
    if pnp_activate_dev((*chip).devc) < 0 {
        dev_err((*(*chip).card).dev, c"PnP control configure failure (out of resources?)\n".as_ptr());
        return -EAGAIN;
    }
    dev_dbg((*(*chip).card).dev, c"pnp: port=0x%llx\n".as_ptr(), pnp_port_start((*chip).devc, 0));
    if snd_audiodrive_pnp_init_main(dev_, (*chip).dev) < 0 { return -EBUSY; }
    0
}

unsafe fn is_isapnp_selected(dev: c_uint) -> c_int {
    isapnp[dev as usize] as c_int
}

unsafe fn snd_es18xx_card_new(pdev: *mut device, dev: c_int, cardp: *mut *mut snd_card) -> c_int {
    snd_devm_card_new(pdev, index[dev as usize], id[dev as usize], ptr::null_mut(), core::mem::size_of::<snd_es18xx>(), cardp)
}

unsafe fn snd_audiodrive_probe(card: *mut snd_card, dev: c_int) -> c_int {
    let chip = (*card).private_data as *mut snd_es18xx;
    let mut opl3: *mut snd_opl3 = ptr::null_mut();
    let mut err = snd_es18xx_new_device(card, port[dev as usize] as c_ulong, mpu_port[dev as usize] as c_ulong, fm_port[dev as usize] as c_ulong, irq[dev as usize], dma1[dev as usize], dma2[dev as usize]);
    if err < 0 { return err; }
    sprintf((*card).driver.as_mut_ptr(), c"ES%x".as_ptr(), (*chip).version as c_int);
    sprintf((*card).shortname.as_mut_ptr(), c"ESS AudioDrive ES%x".as_ptr(), (*chip).version as c_int);
    if dma1[dev as usize] != dma2[dev as usize] {
        sprintf((*card).longname.as_mut_ptr(), c"%s at 0x%lx, irq %d, dma1 %d, dma2 %d".as_ptr(), (*card).shortname.as_ptr(), (*chip).port, irq[dev as usize], dma1[dev as usize], dma2[dev as usize]);
    } else {
        sprintf((*card).longname.as_mut_ptr(), c"%s at 0x%lx, irq %d, dma %d".as_ptr(), (*card).shortname.as_ptr(), (*chip).port, irq[dev as usize], dma1[dev as usize]);
    }
    err = snd_es18xx_pcm(card, 0);
    if err < 0 { return err; }
    err = snd_es18xx_mixer(card);
    if err < 0 { return err; }
    if fm_port[dev as usize] > 0 && fm_port[dev as usize] != SNDRV_AUTO_PORT {
        if snd_opl3_create(card, fm_port[dev as usize], fm_port[dev as usize] + 2, OPL3_HW_OPL3, 0, &mut opl3) < 0 {
            dev_warn((*card).dev, c"opl3 not detected at 0x%lx\n".as_ptr(), fm_port[dev as usize]);
        } else {
            err = snd_opl3_hwdep_new(opl3, 0, 1, ptr::null_mut());
            if err < 0 { return err; }
        }
    }
    if mpu_port[dev as usize] > 0 && mpu_port[dev as usize] != SNDRV_AUTO_PORT {
        err = snd_mpu401_uart_new(card, 0, MPU401_HW_ES18XX, mpu_port[dev as usize], MPU401_INFO_IRQ_HOOK, -1, &mut (*chip).rmidi);
        if err < 0 { return err; }
    }
    snd_card_register(card)
}

unsafe extern "C" fn snd_es18xx_isa_match(_pdev: *mut device, dev: c_uint) -> c_int {
    (enable[dev as usize] && is_isapnp_selected(dev) == 0) as c_int
}

unsafe fn snd_es18xx_isa_probe1(dev: c_int, devptr: *mut device) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut err = snd_es18xx_card_new(devptr, dev, &mut card);
    if err < 0 { return err; }
    err = snd_audiodrive_probe(card, dev);
    if err < 0 { return err; }
    dev_set_drvdata(devptr, card as *mut c_void);
    0
}

unsafe extern "C" fn snd_es18xx_isa_probe(pdev: *mut device, dev: c_uint) -> c_int {
    let dev_usize = dev as usize;
    let mut err: c_int;
    static possible_irqs: [c_int; 7] = [5, 9, 10, 7, 11, 12, -1];
    static possible_dmas: [c_int; 5] = [1, 0, 3, 5, -1];
    if irq[dev_usize] == SNDRV_AUTO_IRQ {
        irq[dev_usize] = snd_legacy_find_free_irq(possible_irqs.as_ptr());
        if irq[dev_usize] < 0 {
            dev_err(pdev, c"unable to find a free IRQ\n".as_ptr());
            return -EBUSY;
        }
    }
    if dma1[dev_usize] == SNDRV_AUTO_DMA {
        dma1[dev_usize] = snd_legacy_find_free_dma(possible_dmas.as_ptr());
        if dma1[dev_usize] < 0 {
            dev_err(pdev, c"unable to find a free DMA1\n".as_ptr());
            return -EBUSY;
        }
    }
    if dma2[dev_usize] == SNDRV_AUTO_DMA {
        dma2[dev_usize] = snd_legacy_find_free_dma(possible_dmas.as_ptr());
        if dma2[dev_usize] < 0 {
            dev_err(pdev, c"unable to find a free DMA2\n".as_ptr());
            return -EBUSY;
        }
    }
    if port[dev_usize] != SNDRV_AUTO_PORT {
        snd_es18xx_isa_probe1(dev as c_int, pdev)
    } else {
        static possible_ports: [c_ulong; 4] = [0x220, 0x240, 0x260, 0x280];
        let mut i = 0usize;
        err = -ENODEV;
        while i < possible_ports.len() {
            port[dev_usize] = possible_ports[i] as c_long;
            err = snd_es18xx_isa_probe1(dev as c_int, pdev);
            if err == 0 { return 0; }
            i += 1;
        }
        err
    }
}

unsafe extern "C" fn snd_es18xx_isa_suspend(dev: *mut device, _n: c_uint, state: pm_message_t) -> c_int {
    snd_es18xx_suspend(dev_get_drvdata(dev) as *mut snd_card, state)
}

unsafe extern "C" fn snd_es18xx_isa_resume(dev: *mut device, _n: c_uint) -> c_int {
    snd_es18xx_resume(dev_get_drvdata(dev) as *mut snd_card)
}

const DEV_NAME: *const c_char = c"es18xx".as_ptr();

static mut snd_es18xx_isa_driver: isa_driver = isa_driver {
    match_: Some(snd_es18xx_isa_match),
    probe: Some(snd_es18xx_isa_probe),
    suspend: Some(snd_es18xx_isa_suspend),
    resume: Some(snd_es18xx_isa_resume),
    driver: isa_driver_inner { name: DEV_NAME },
};

unsafe extern "C" fn snd_audiodrive_pnp_detect(pdev: *mut pnp_dev, _id: *const pnp_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    if pnp_device_is_isapnp(pdev) != 0 { return -ENOENT; }
    while dev < SNDRV_CARDS as c_int {
        if enable[dev as usize] && isapnp[dev as usize] { break; }
        dev += 1;
    }
    if dev >= SNDRV_CARDS as c_int { return -ENODEV; }
    let mut err = snd_es18xx_card_new(&mut (*pdev).dev, dev, &mut card);
    if err < 0 { return err; }
    err = snd_audiodrive_pnp(dev, (*card).private_data as *mut snd_es18xx, pdev);
    if err < 0 { return err; }
    err = snd_audiodrive_probe(card, dev);
    if err < 0 { return err; }
    pnp_set_drvdata(pdev, card as *mut c_void);
    dev += 1;
    0
}

unsafe extern "C" fn snd_audiodrive_pnp_suspend(pdev: *mut pnp_dev, state: pm_message_t) -> c_int {
    snd_es18xx_suspend(pnp_get_drvdata(pdev) as *mut snd_card, state)
}

unsafe extern "C" fn snd_audiodrive_pnp_resume(pdev: *mut pnp_dev) -> c_int {
    snd_es18xx_resume(pnp_get_drvdata(pdev) as *mut snd_card)
}

static mut es18xx_pnp_driver: pnp_driver = pnp_driver {
    name: c"es18xx-pnpbios".as_ptr(),
    id_table: snd_audiodrive_pnpbiosids.as_ptr(),
    probe: Some(snd_audiodrive_pnp_detect),
    suspend: Some(snd_audiodrive_pnp_suspend),
    resume: Some(snd_audiodrive_pnp_resume),
};

unsafe extern "C" fn snd_audiodrive_pnpc_detect(pcard: *mut pnp_card_link, pid: *const pnp_card_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    while dev < SNDRV_CARDS as c_int {
        if enable[dev as usize] && isapnp[dev as usize] { break; }
        dev += 1;
    }
    if dev >= SNDRV_CARDS as c_int { return -ENODEV; }
    let mut res = snd_es18xx_card_new(&mut (*(*pcard).card).dev, dev, &mut card);
    if res < 0 { return res; }
    res = snd_audiodrive_pnpc(dev, (*card).private_data as *mut snd_es18xx, pcard, pid);
    if res < 0 { return res; }
    res = snd_audiodrive_probe(card, dev);
    if res < 0 { return res; }
    pnp_set_card_drvdata(pcard, card as *mut c_void);
    dev += 1;
    0
}

unsafe extern "C" fn snd_audiodrive_pnpc_suspend(pcard: *mut pnp_card_link, state: pm_message_t) -> c_int {
    snd_es18xx_suspend(pnp_get_card_drvdata(pcard) as *mut snd_card, state)
}

unsafe extern "C" fn snd_audiodrive_pnpc_resume(pcard: *mut pnp_card_link) -> c_int {
    snd_es18xx_resume(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

static mut es18xx_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DISABLE,
    name: c"es18xx".as_ptr(),
    id_table: snd_audiodrive_pnpids.as_ptr(),
    probe: Some(snd_audiodrive_pnpc_detect),
    suspend: Some(snd_audiodrive_pnpc_suspend),
    resume: Some(snd_audiodrive_pnpc_resume),
};

unsafe fn alsa_card_es18xx_init() -> c_int {
    let mut err = isa_register_driver(&mut snd_es18xx_isa_driver, SNDRV_CARDS as c_uint);
    if err == 0 { isa_registered = 1; }
    err = pnp_register_driver(&mut es18xx_pnp_driver);
    if err == 0 { pnp_registered = 1; }
    err = pnp_register_card_driver(&mut es18xx_pnpc_driver);
    if err == 0 { pnpc_registered = 1; }
    if isa_registered != 0 || pnp_registered != 0 { err = 0; }
    err
}

unsafe fn alsa_card_es18xx_exit() {
    if pnpc_registered != 0 { pnp_unregister_card_driver(&mut es18xx_pnpc_driver); }
    if pnp_registered != 0 { pnp_unregister_driver(&mut es18xx_pnp_driver); }
    if isa_registered != 0 { isa_unregister_driver(&mut snd_es18xx_isa_driver); }
}

/* module_init(alsa_card_es18xx_init) */
/* module_exit(alsa_card_es18xx_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
