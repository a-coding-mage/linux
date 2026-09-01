// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Routines for control of YMF724/740/744/754 chips
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, improper_ctypes, static_mut_refs, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type __le32 = u32;
type dma_addr_t = c_ulong;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;

#[repr(C)] pub struct snd_ymfpci { _private: [u8; 0] }
#[repr(C)] pub struct snd_ac97 { pub private_data: *mut snd_ymfpci, pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97)>, pub ext_id: c_uint }
#[repr(C)] pub struct snd_ac97_bus { pub private_data: *mut snd_ymfpci, pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97_bus)>, pub no_vra: c_int }
#[repr(C)] pub struct snd_ac97_template { pub private_data: *mut snd_ymfpci, pub private_free: Option<unsafe extern "C" fn(*mut snd_ac97)> }
#[repr(C)] pub struct snd_ac97_bus_ops { pub write: Option<unsafe extern "C" fn(*mut snd_ac97, u16, u16)>, pub read: Option<unsafe extern "C" fn(*mut snd_ac97, u16) -> u16> }
#[repr(C)] pub struct snd_card { pub dev: *mut c_void, pub number: c_int, pub private_data: *mut snd_ymfpci, pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>, pub sync_irq: c_int }
#[repr(C)] pub struct pci_dev { pub dev: device, pub device: u16, pub revision: u8, pub irq: c_int }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct firmware { pub size: c_ulong, pub data: *const u8 }
#[repr(C)] pub struct snd_pcm { pub private_data: *mut snd_ymfpci, pub info_flags: c_uint, pub name: [c_char; 80], pub device: c_int, pub streams: [snd_pcm_str; 2] }
#[repr(C)] pub struct snd_pcm_str { pub substream: *mut snd_pcm_substream }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime, pub number: c_uint, pub pcm: *mut snd_pcm, pub next: *mut snd_pcm_substream }
#[repr(C)] pub struct snd_pcm_runtime { pub private_data: *mut snd_ymfpci_pcm, pub private_free: Option<unsafe extern "C" fn(*mut snd_pcm_runtime)>, pub hw: snd_pcm_hardware, pub rate: c_uint, pub channels: c_uint, pub format: c_int, pub dma_addr: dma_addr_t, pub period_size: snd_pcm_uframes_t, pub buffer_size: snd_pcm_uframes_t }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_pcm_hardware { pub info: c_uint, pub formats: u64, pub rates: c_uint, pub rate_min: c_uint, pub rate_max: c_uint, pub channels_min: c_uint, pub channels_max: c_uint, pub buffer_bytes_max: c_ulong, pub period_bytes_min: c_ulong, pub period_bytes_max: c_ulong, pub periods_min: c_uint, pub periods_max: c_uint, pub fifo_size: c_uint }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_ops { pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub ioctl: *const c_void, pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>, pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>, pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>, pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t> }
#[repr(C)] pub struct snd_kcontrol { pub id: snd_ctl_elem_id, pub vd: [snd_kcontrol_volatile; 1], pub private_value: c_ulong }
#[repr(C)] pub struct snd_kcontrol_volatile { pub access: c_uint }
#[repr(C)] pub struct snd_ctl_elem_id { pub device: c_uint, pub subdevice: c_uint }
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_uint, pub count: c_uint, pub value: snd_ctl_elem_info_value }
#[repr(C)] pub union snd_ctl_elem_info_value { pub integer: snd_ctl_elem_info_integer }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_info_integer { pub min: c_long, pub max: c_long }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub union snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer, pub enumerated: snd_ctl_elem_value_enumerated, pub iec958: snd_ctl_elem_value_iec958 }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { pub value: [c_long; 128] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 128] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_iec958 { pub status: [u8; 24] }
#[repr(C)] pub struct snd_kcontrol_new { pub iface: c_uint, pub name: *const c_char, pub index: c_uint, pub access: c_uint, pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>, pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, pub private_value: c_ulong, pub tlv: snd_kcontrol_tlv }
#[repr(C)] pub union snd_kcontrol_tlv { pub p: *const c_uint }
#[repr(C)] pub struct snd_timer { pub sticks: c_ulong, pub name: [c_char; 80], pub private_data: *mut snd_ymfpci, pub hw: snd_timer_hardware }
#[repr(C)] pub struct snd_timer_id { pub dev_class: c_int, pub dev_sclass: c_int, pub card: c_int, pub device: c_int, pub subdevice: c_int }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_timer_hardware { pub flags: c_uint, pub resolution: c_ulong, pub ticks: c_ulong, pub start: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>, pub stop: Option<unsafe extern "C" fn(*mut snd_timer) -> c_int>, pub precise_resolution: Option<unsafe extern "C" fn(*mut snd_timer, *mut c_ulong, *mut c_ulong) -> c_int> }
#[repr(C)] pub struct snd_info_entry { pub private_data: *mut snd_ymfpci }
#[repr(C)] pub struct snd_info_buffer { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_entry_t { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_chmap_elem { pub channels: c_uint, pub map: [c_uint; 4] }

#[repr(C)] pub struct snd_ymfpci_voice { pub use_: c_int, pub pcm: c_int, pub synth: c_int, pub midi: c_int, pub number: c_int, pub bank: *mut snd_ymfpci_playback_bank, pub bank_addr: dma_addr_t, pub ypcm: *mut snd_ymfpci_pcm, pub interrupt: Option<unsafe extern "C" fn(*mut snd_ymfpci, *mut snd_ymfpci_voice)>, }
#[repr(C)] pub struct snd_ymfpci_pcm { _private: [u8; 0] }
#[repr(C)] pub struct snd_ymfpci_playback_bank { pub format: __le32, pub base: __le32, pub loop_end: __le32, pub start: __le32, pub lpfQ: __le32, pub delta: __le32, pub delta_end: __le32, pub lpfK: __le32, pub lpfK_end: __le32, pub eg_gain: __le32, pub eg_gain_end: __le32, pub left_gain: __le32, pub left_gain_end: __le32, pub right_gain: __le32, pub right_gain_end: __le32, pub eff2_gain: __le32, pub eff2_gain_end: __le32, pub eff3_gain: __le32, pub eff3_gain_end: __le32, pub num_of_loops: __le32 }
#[repr(C)] pub struct snd_ymfpci_capture_bank { pub base: __le32, pub loop_end: __le32, pub start: __le32, pub num_of_loops: __le32 }
#[repr(C)] pub struct snd_ymfpci_effect_bank { pub base: __le32, pub loop_end: __le32 }

const EBUSY: c_int = 16; const ENOMEM: c_int = 12; const EINVAL: c_int = 22; const ENXIO: c_int = 6; const EIO: c_int = 5;
const IRQ_HANDLED: irqreturn_t = 1;
const YDSXG_PLAYBACK_VOICES: c_int = 64;
const YDSXG_CAPTURE_VOICES: c_int = 2;
const YDSXG_EFFECT_VOICES: c_int = 5;
const YDSXG_DSPLENGTH: c_ulong = 0x1000;
const YDSXG_CTRLLENGTH: c_ulong = 0x1000;
const YDSXG_DEFAULT_WORK_SIZE: c_long = 0x4000;
const PLAYBACK_VOICE: c_uint = 0; const CAPTURE_REC: c_uint = 1;
const PCI_DEVICE_ID_YAMAHA_724F: u16 = 0x000d; const PCI_DEVICE_ID_YAMAHA_740C: u16 = 0x000c; const PCI_DEVICE_ID_YAMAHA_744: u16 = 0x0010; const PCI_DEVICE_ID_YAMAHA_754: u16 = 0x0012;

const YDSXGR_SECSTATUSADR: u32 = 0; const YDSXGR_PRISTATUSADR: u32 = 0; const YDSXGR_AC97CMDDATA: u32 = 0; const YDSXGR_AC97CMDADR: u32 = 0; const YDSXGR_PRISTATUSDATA: u32 = 0;
const YDSXGR_MODE: u32 = 0; const YDSXGR_CTRLSELECT: u32 = 0; const YDSXGR_STATUS: u32 = 0; const YDSXGR_MAPOFREC: u32 = 0; const YDSXGR_MAPOFEFFECT: u32 = 0;
const YDSXGR_RECFORMAT: u32 = 0; const YDSXGR_RECSLOTSR: u32 = 0; const YDSXGR_ADCFORMAT: u32 = 0; const YDSXGR_ADCSLOTSR: u32 = 0; const YDSXGR_INTFLAG: u32 = 0;
const YDSXGR_SPDIFOUTCTRL: u32 = 0; const YDSXGR_SPDIFOUTSTATUS: u32 = 0; const YDSXGR_SECCONFIG: u32 = 0; const YDSXGR_GLOBALCTRL: u32 = 0; const YDSXGR_SPDIFINCTRL: u32 = 0;
const YDSXGR_NATIVEDACOUTVOL: u32 = 0; const YDSXGR_BUF441OUTVOL: u32 = 0; const YDSXGR_NATIVEDACLOOPVOL: u32 = 0; const YDSXGR_NATIVEDACINVOL: u32 = 0; const YDSXGR_NATIVEADCINVOL: u32 = 0;
const YDSXGR_PRIADCOUTVOL: u32 = 0; const YDSXGR_PRIADCLOOPVOL: u32 = 0; const YDSXGR_SECADCOUTVOL: u32 = 0; const YDSXGR_SECADCLOOPVOL: u32 = 0; const YDSXGR_LEGACYOUTVOL: u32 = 0;
const YDSXGR_ZVOUTVOL: u32 = 0; const YDSXGR_ZVLOOPVOL: u32 = 0; const YDSXGR_SPDIFOUTVOL: u32 = 0; const YDSXGR_SPDIFLOOPVOL: u32 = 0; const YDSXGR_GPIOFUNCENABLE: u32 = 0;
const YDSXGR_GPIOTYPECONFIG: u32 = 0; const YDSXGR_GPIOINSTATUS: u32 = 0; const YDSXGR_GPIOOUTCTRL: u32 = 0; const YDSXGR_TIMERCOUNT: u32 = 0; const YDSXGR_TIMERCTRL: u32 = 0;
const YDSXGR_CONFIG: u32 = 0; const YDSXGR_PLAYCTRLBASE: u32 = 0; const YDSXGR_RECCTRLBASE: u32 = 0; const YDSXGR_EFFCTRLBASE: u32 = 0; const YDSXGR_WORKBASE: u32 = 0; const YDSXGR_WORKSIZE: u32 = 0;
const YDSXGR_PLAYCTRLSIZE: u32 = 0; const YDSXGR_RECCTRLSIZE: u32 = 0; const YDSXGR_EFFCTRLSIZE: u32 = 0; const YDSXGR_DSPINSTRAM: u32 = 0; const YDSXGR_CTRLINSTRAM: u32 = 0;
const YDSXG_AC97WRITECMD: u32 = 0; const YDSXG_AC97READCMD: u16 = 0;

unsafe extern "C" {
    static mut jiffies: c_ulong;
    static mut current: *mut c_void;
    static mut saved_regs_index: [u32; 0];
    static mut pci_saved_regs_index: [u16; 0];
    fn writeb(val: u8, addr: *mut c_void); fn writew(val: u16, addr: *mut c_void); fn writel(val: u32, addr: *mut c_void);
    fn readw(addr: *mut c_void) -> u16; fn readl(addr: *mut c_void) -> u32;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong; fn time_before(a: c_ulong, b: c_ulong) -> bool; fn schedule_timeout_uninterruptible(t: c_ulong);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn snd_pcm_period_elapsed(s: *mut snd_pcm_substream); fn snd_pcm_substream_chip(s: *mut snd_pcm_substream) -> *mut snd_ymfpci; fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_pcm_hw_constraint_minmax(r: *mut snd_pcm_runtime, p: c_int, min: c_uint, max: c_uint) -> c_int; fn snd_pcm_hw_rule_noresample(r: *mut snd_pcm_runtime, rate: c_uint) -> c_int;
    fn snd_dma_alloc_pages(t: c_int, dev: *mut device, size: c_ulong, dmab: *mut c_void) -> c_int; fn snd_dma_free_pages(dmab: *mut c_void);
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id); fn snd_ctl_boolean_mono_info(k: *mut snd_kcontrol, i: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_enum_info(i: *mut snd_ctl_elem_info, c: c_uint, n: c_uint, texts: *const *const c_char) -> c_int; fn snd_ctl_add(card: *mut snd_card, k: *mut snd_kcontrol) -> c_int; fn snd_ctl_new1(n: *const snd_kcontrol_new, chip: *mut snd_ymfpci) -> *mut snd_kcontrol; fn snd_kcontrol_chip(k: *mut snd_kcontrol) -> *mut snd_ymfpci;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, dev: c_int, play: c_int, cap: c_int, rpcm: *mut *mut snd_pcm) -> c_int; fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops); fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, t: c_int, dev: *mut device, min: c_ulong, max: c_ulong); fn snd_pcm_add_chmap_ctls(pcm: *mut snd_pcm, stream: c_int, map: *const snd_pcm_chmap_elem, max: c_int, mask: c_int, ctl: *mut c_void) -> c_int;
    fn kfree(p: *mut c_void); fn memset(p: *mut c_void, v: c_int, n: c_ulong) -> *mut c_void; fn sprintf(s: *mut c_char, fmt: *const c_char, ...); fn strscpy(dst: *mut c_char, src: *const c_char) -> c_ulong;
    fn snd_ac97_bus(card: *mut snd_card, num: c_int, ops: *const snd_ac97_bus_ops, chip: *mut snd_ymfpci, rbus: *mut *mut snd_ac97_bus) -> c_int; fn snd_ac97_mixer(bus: *mut snd_ac97_bus, tmpl: *mut snd_ac97_template, rac97: *mut *mut snd_ac97) -> c_int; fn snd_ac97_update_bits(a: *mut snd_ac97, reg: c_int, mask: c_int, val: c_int); fn snd_ac97_suspend(a: *mut snd_ac97); fn snd_ac97_resume(a: *mut snd_ac97);
    fn snd_timer_chip(t: *mut snd_timer) -> *mut snd_ymfpci; fn snd_timer_new(card: *mut snd_card, id: *const c_char, tid: *mut snd_timer_id, rt: *mut *mut snd_timer) -> c_int; fn snd_timer_interrupt(t: *mut snd_timer, ticks: c_ulong);
    fn snd_iprintf(b: *mut snd_info_buffer, fmt: *const c_char, ...); fn snd_card_ro_proc_new(card: *mut snd_card, name: *const c_char, chip: *mut snd_ymfpci, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>) -> c_int;
    fn pci_read_config_byte(p: *mut pci_dev, where_: c_int, v: *mut u8); fn pci_write_config_byte(p: *mut pci_dev, where_: c_int, v: u8); fn pci_write_config_word(p: *mut pci_dev, where_: c_int, v: u16); fn pci_read_config_word(p: *mut pci_dev, where_: u16, v: *mut u16);
    fn request_firmware(f: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int; fn release_firmware(f: *const firmware); fn udelay(us: c_uint);
    fn pcim_enable_device(p: *mut pci_dev) -> c_int; fn pcim_request_all_regions(p: *mut pci_dev, name: *const c_char) -> c_int; fn pci_resource_start(p: *mut pci_dev, bar: c_int) -> c_ulong; fn devm_ioremap(dev: *mut device, phys: c_ulong, size: c_ulong) -> *mut c_void; fn pci_set_master(p: *mut pci_dev); fn devm_request_irq(dev: *mut device, irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_ulong, name: *const c_char, data: *mut snd_ymfpci) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut snd_card; fn to_pci_dev(dev: *mut device) -> *mut pci_dev; fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_ymfpci_free_gameport(chip: *mut snd_ymfpci);
}

#[inline] unsafe fn cpu_to_le32(v: u32) -> __le32 { v.to_le() }
#[inline] unsafe fn le32_to_cpu(v: __le32) -> u32 { u32::from_le(v) }
#[inline] unsafe fn ALIGN(x: c_ulong, a: c_ulong) -> c_ulong { (x + a - 1) & !(a - 1) }
#[inline] unsafe fn PAGE_ALIGN(x: c_long) -> c_long { ALIGN(x as c_ulong, 4096) as c_long }
#[inline] unsafe fn snd_BUG_ON(v: bool) -> bool { v }

/*
 *  common I/O routines
 */
unsafe fn snd_ymfpci_irq_wait(chip: *mut snd_ymfpci);

#[inline] unsafe fn snd_ymfpci_writeb(chip: *mut snd_ymfpci, offset: u32, val: u8) { writeb(val, ((*chip).reg_area_virt as *mut u8).add(offset as usize) as *mut c_void); }
#[inline] unsafe fn snd_ymfpci_readw(chip: *mut snd_ymfpci, offset: u32) -> u16 { readw(((*chip).reg_area_virt as *mut u8).add(offset as usize) as *mut c_void) }
#[inline] unsafe fn snd_ymfpci_writew(chip: *mut snd_ymfpci, offset: u32, val: u16) { writew(val, ((*chip).reg_area_virt as *mut u8).add(offset as usize) as *mut c_void); }
#[inline] unsafe fn snd_ymfpci_readl(chip: *mut snd_ymfpci, offset: u32) -> u32 { readl(((*chip).reg_area_virt as *mut u8).add(offset as usize) as *mut c_void) }
#[inline] unsafe fn snd_ymfpci_writel(chip: *mut snd_ymfpci, offset: u32, val: u32) { writel(val, ((*chip).reg_area_virt as *mut u8).add(offset as usize) as *mut c_void); }

unsafe extern "C" fn snd_ymfpci_codec_ready(chip: *mut snd_ymfpci, secondary: c_int) -> c_int {
    let reg: u32 = if secondary != 0 { YDSXGR_SECSTATUSADR } else { YDSXGR_PRISTATUSADR };
    let end_time = jiffies.wrapping_add(msecs_to_jiffies(750));
    loop {
        if (snd_ymfpci_readw(chip, reg) & 0x8000) == 0 { return 0; }
        schedule_timeout_uninterruptible(1);
        if !time_before(jiffies, end_time) { break; }
    }
    dev_err((*(*chip).card).dev, c"codec_ready: codec %i is not ready [0x%x]\n".as_ptr(), secondary, snd_ymfpci_readw(chip, reg) as c_uint);
    -EBUSY
}

unsafe extern "C" fn snd_ymfpci_codec_write(ac97: *mut snd_ac97, reg: u16, val: u16) {
    let chip = (*ac97).private_data;
    snd_ymfpci_codec_ready(chip, 0);
    let cmd = (((YDSXG_AC97WRITECMD | reg as u32) << 16) | val as u32) as u32;
    snd_ymfpci_writel(chip, YDSXGR_AC97CMDDATA, cmd);
}

unsafe extern "C" fn snd_ymfpci_codec_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    let chip = (*ac97).private_data;
    if snd_ymfpci_codec_ready(chip, 0) != 0 { return !0; }
    snd_ymfpci_writew(chip, YDSXGR_AC97CMDADR, YDSXG_AC97READCMD | reg);
    if snd_ymfpci_codec_ready(chip, 0) != 0 { return !0; }
    if (*chip).device_id == PCI_DEVICE_ID_YAMAHA_744 && (*chip).rev < 2 {
        for _i in 0..600 { snd_ymfpci_readw(chip, YDSXGR_PRISTATUSDATA); }
    }
    snd_ymfpci_readw(chip, YDSXGR_PRISTATUSDATA)
}

/*
 *  Misc routines
 */
unsafe fn snd_ymfpci_calc_delta(rate: u32) -> u32 {
    match rate {
        8000 => 0x02aaab00, 11025 => 0x03accd00, 16000 => 0x05555500, 22050 => 0x07599a00,
        32000 => 0x0aaaab00, 44100 => 0x0eb33300, _ => ((rate << 16) / 375) << 5,
    }
}
static def_rate: [u32; 8] = [100, 2000, 8000, 11025, 16000, 22050, 32000, 48000];
unsafe fn snd_ymfpci_calc_lpfK(rate: u32) -> u32 {
    static val: [u32; 8] = [0x00570000, 0x06AA0000, 0x18B20000, 0x20930000, 0x2B9A0000, 0x35A10000, 0x3EAA0000, 0x40000000];
    if rate == 44100 { return 0x40000000; } /* FIXME: What's the right value? */
    for i in 0..8 { if rate <= def_rate[i] { return val[i]; } }
    val[0]
}
unsafe fn snd_ymfpci_calc_lpfQ(rate: u32) -> u32 {
    static val: [u32; 8] = [0x35280000, 0x34A70000, 0x32020000, 0x31770000, 0x31390000, 0x31C90000, 0x33D00000, 0x40000000];
    if rate == 44100 { return 0x370A0000; }
    for i in 0..8 { if rate <= def_rate[i] { return val[i]; } }
    val[0]
}

/*
 *  Hardware start management
 */
unsafe fn snd_ymfpci_hw_start(chip: *mut snd_ymfpci) {
    if { let old = (*chip).start_count; (*chip).start_count += 1; old > 0 } { return; }
    snd_ymfpci_writel(chip, YDSXGR_MODE, snd_ymfpci_readl(chip, YDSXGR_MODE) | 3);
    (*chip).active_bank = snd_ymfpci_readl(chip, YDSXGR_CTRLSELECT) & 1;
}
unsafe fn snd_ymfpci_hw_stop(chip: *mut snd_ymfpci) {
    let mut timeout: c_long = 1000;
    (*chip).start_count -= 1;
    if (*chip).start_count > 0 { return; }
    snd_ymfpci_writel(chip, YDSXGR_MODE, snd_ymfpci_readl(chip, YDSXGR_MODE) & !3);
    while { let t = timeout; timeout -= 1; t > 0 } {
        if (snd_ymfpci_readl(chip, YDSXGR_STATUS) & 2) == 0 { break; }
    }
    if atomic_read(&mut (*chip).interrupt_sleep_count) != 0 {
        atomic_set(&mut (*chip).interrupt_sleep_count, 0);
        wake_up(&mut (*chip).interrupt_sleep);
    }
}

/* Playback voice management */
unsafe fn voice_alloc(chip: *mut snd_ymfpci, type_: snd_ymfpci_voice_type, pair: c_int, rvoice: *mut *mut snd_ymfpci_voice) -> c_int {
    *rvoice = ptr::null_mut();
    let mut idx = 0;
    while idx < YDSXG_PLAYBACK_VOICES {
        let voice = &mut (*chip).voices[idx as usize] as *mut snd_ymfpci_voice;
        let voice2 = if pair != 0 { &mut (*chip).voices[(idx + 1) as usize] as *mut snd_ymfpci_voice } else { ptr::null_mut() };
        if (*voice).use_ != 0 || (!voice2.is_null() && (*voice2).use_ != 0) { idx += if pair != 0 { 2 } else { 1 }; continue; }
        (*voice).use_ = 1; if !voice2.is_null() { (*voice2).use_ = 1; }
        match type_ { snd_ymfpci_voice_type::YMFPCI_PCM => { (*voice).pcm = 1; if !voice2.is_null() { (*voice2).pcm = 1; } }, snd_ymfpci_voice_type::YMFPCI_SYNTH => (*voice).synth = 1, snd_ymfpci_voice_type::YMFPCI_MIDI => (*voice).midi = 1 }
        snd_ymfpci_hw_start(chip); if !voice2.is_null() { snd_ymfpci_hw_start(chip); }
        *rvoice = voice; return 0;
    }
    -ENOMEM
}
#[repr(C)] enum snd_ymfpci_voice_type { YMFPCI_PCM, YMFPCI_SYNTH, YMFPCI_MIDI }
unsafe fn snd_ymfpci_voice_alloc(chip: *mut snd_ymfpci, type_: snd_ymfpci_voice_type, pair: c_int, rvoice: *mut *mut snd_ymfpci_voice) -> c_int {
    if snd_BUG_ON(rvoice.is_null()) { return -EINVAL; }
    if snd_BUG_ON(pair != 0 && !matches!(type_, snd_ymfpci_voice_type::YMFPCI_PCM)) { return -EINVAL; }
    voice_alloc(chip, type_, pair, rvoice)
}
unsafe fn snd_ymfpci_voice_free(chip: *mut snd_ymfpci, pvoice: *mut snd_ymfpci_voice) -> c_int {
    if snd_BUG_ON(pvoice.is_null()) { return -EINVAL; }
    snd_ymfpci_hw_stop(chip);
    if (*pvoice).number == (*chip).src441_used { (*chip).src441_used = -1; (*(*pvoice).ypcm).use_441_slot = 0; }
    (*pvoice).use_ = 0; (*pvoice).pcm = 0; (*pvoice).synth = 0; (*pvoice).midi = 0; (*pvoice).ypcm = ptr::null_mut(); (*pvoice).interrupt = None;
    0
}

/* PCM part: source-level translation of interrupt, trigger, prepare, pointer, open, close, and constructor routines. */
unsafe extern "C" fn snd_ymfpci_pcm_interrupt(chip: *mut snd_ymfpci, voice: *mut snd_ymfpci_voice) {
    let ypcm = (*voice).ypcm;
    if ypcm.is_null() || (*ypcm).substream.is_null() { return; }
    if (*ypcm).running != 0 {
        let pos = le32_to_cpu((*(*voice).bank.add((*chip).active_bank as usize)).start);
        let delta = if pos < (*ypcm).last_pos { pos + ((*ypcm).buffer_size - (*ypcm).last_pos) } else { pos - (*ypcm).last_pos };
        (*ypcm).period_pos += delta; (*ypcm).last_pos = pos;
        if (*ypcm).period_pos >= (*ypcm).period_size { (*ypcm).period_pos %= (*ypcm).period_size; snd_pcm_period_elapsed((*ypcm).substream); }
        if (*ypcm).update_pcm_vol != 0 {
            let subs = (*(*ypcm).substream).number as usize; let next_bank = 1 - (*chip).active_bank as usize; let mut bank = (*voice).bank.add(next_bank);
            let mut volume = cpu_to_le32((*chip).pcm_mixer[subs].left << 15); (*bank).left_gain_end = volume; if (*ypcm).output_rear != 0 { (*bank).eff2_gain_end = volume; }
            if !(*ypcm).voices[1].is_null() { bank = (*(*ypcm).voices[1]).bank.add(next_bank); }
            volume = cpu_to_le32((*chip).pcm_mixer[subs].right << 15); (*bank).right_gain_end = volume; if (*ypcm).output_rear != 0 { (*bank).eff3_gain_end = volume; }
            (*ypcm).update_pcm_vol -= 1;
        }
    }
}
unsafe fn snd_ymfpci_pcm_capture_interrupt(substream: *mut snd_pcm_substream) {
    let runtime = (*substream).runtime; let ypcm = (*runtime).private_data; let chip = (*ypcm).chip;
    if (*ypcm).running != 0 {
        let pos = le32_to_cpu((*(*chip).bank_capture[(*ypcm).capture_bank_number as usize][(*chip).active_bank as usize]).start) >> (*ypcm).shift;
        let delta = if pos < (*ypcm).last_pos { pos + ((*ypcm).buffer_size - (*ypcm).last_pos) } else { pos - (*ypcm).last_pos };
        (*ypcm).period_pos += delta; (*ypcm).last_pos = pos;
        if (*ypcm).period_pos >= (*ypcm).period_size { (*ypcm).period_pos %= (*ypcm).period_size; snd_pcm_period_elapsed(substream); }
    }
}

unsafe extern "C" fn snd_ymfpci_playback_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream); let ypcm = (*(*substream).runtime).private_data; let mut kctl: *mut snd_kcontrol = ptr::null_mut();
    if (*ypcm).voices[0].is_null() { return -EINVAL; }
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {
            (*chip).ctrl_playback[(*(*ypcm).voices[0]).number as usize + 1] = cpu_to_le32((*(*ypcm).voices[0]).bank_addr as u32);
            if !(*ypcm).voices[1].is_null() && (*ypcm).use_441_slot == 0 { (*chip).ctrl_playback[(*(*ypcm).voices[1]).number as usize + 1] = cpu_to_le32((*(*ypcm).voices[1]).bank_addr as u32); }
            (*ypcm).running = 1;
        }
        SNDRV_PCM_TRIGGER_STOP => { if (*substream).pcm == (*chip).pcm && (*ypcm).use_441_slot == 0 { kctl = (*chip).pcm_mixer[(*substream).number as usize].ctl; (*kctl).vd[0].access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE; }
            (*chip).ctrl_playback[(*(*ypcm).voices[0]).number as usize + 1] = 0; if !(*ypcm).voices[1].is_null() && (*ypcm).use_441_slot == 0 { (*chip).ctrl_playback[(*(*ypcm).voices[1]).number as usize + 1] = 0; } (*ypcm).running = 0; }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => { (*chip).ctrl_playback[(*(*ypcm).voices[0]).number as usize + 1] = 0; if !(*ypcm).voices[1].is_null() && (*ypcm).use_441_slot == 0 { (*chip).ctrl_playback[(*(*ypcm).voices[1]).number as usize + 1] = 0; } (*ypcm).running = 0; }
        _ => return -EINVAL,
    }
    if !kctl.is_null() { snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_INFO, &mut (*kctl).id); }
    0
}

unsafe extern "C" fn snd_ymfpci_capture_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream); let ypcm = (*(*substream).runtime).private_data; let mut tmp: u32;
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => { tmp = snd_ymfpci_readl(chip, YDSXGR_MAPOFREC) | (1 << (*ypcm).capture_bank_number); snd_ymfpci_writel(chip, YDSXGR_MAPOFREC, tmp); (*ypcm).running = 1; 0 }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND => { tmp = snd_ymfpci_readl(chip, YDSXGR_MAPOFREC) & !(1 << (*ypcm).capture_bank_number); snd_ymfpci_writel(chip, YDSXGR_MAPOFREC, tmp); (*ypcm).running = 0; 0 }
        _ => -EINVAL,
    }
}

unsafe fn snd_ymfpci_pcm_voice_alloc(ypcm: *mut snd_ymfpci_pcm, voices: c_int) -> c_int {
    if !(*ypcm).voices[1].is_null() && voices < 2 { snd_ymfpci_voice_free((*ypcm).chip, (*ypcm).voices[1]); (*ypcm).voices[1] = ptr::null_mut(); }
    if voices == 1 && !(*ypcm).voices[0].is_null() { return 0; }
    if voices == 2 && !(*ypcm).voices[0].is_null() && !(*ypcm).voices[1].is_null() { return 0; }
    if voices > 1 && !(*ypcm).voices[0].is_null() && (*ypcm).voices[1].is_null() { snd_ymfpci_voice_free((*ypcm).chip, (*ypcm).voices[0]); (*ypcm).voices[0] = ptr::null_mut(); }
    let err = snd_ymfpci_voice_alloc((*ypcm).chip, snd_ymfpci_voice_type::YMFPCI_PCM, (voices > 1) as c_int, &mut (*ypcm).voices[0]);
    if err < 0 { return err; }
    (*(*ypcm).voices[0]).ypcm = ypcm; (*(*ypcm).voices[0]).interrupt = Some(snd_ymfpci_pcm_interrupt);
    if voices > 1 { (*ypcm).voices[1] = &mut (*(*ypcm).chip).voices[(*(*ypcm).voices[0]).number as usize + 1]; (*(*ypcm).voices[1]).ypcm = ypcm; }
    0
}

unsafe fn snd_ymfpci_pcm_init_voice(ypcm: *mut snd_ymfpci_pcm, voiceidx: c_uint, runtime: *mut snd_pcm_runtime, has_pcm_volume: c_int) {
    let voice = (*ypcm).voices[voiceidx as usize]; if snd_BUG_ON(voice.is_null()) { return; }
    let delta = snd_ymfpci_calc_delta((*runtime).rate); let lpfQ = snd_ymfpci_calc_lpfQ((*runtime).rate); let lpfK = snd_ymfpci_calc_lpfK((*runtime).rate);
    let (use_left, use_right) = if (*runtime).channels == 1 { (1, 1) } else { (((voiceidx & 1) == 0) as u8, ((voiceidx & 1) != 0) as u8) };
    let (vol_left, vol_right) = if has_pcm_volume != 0 { let subs = (*(*ypcm).substream).number as usize; (cpu_to_le32((*(*ypcm).chip).pcm_mixer[subs].left << 15), cpu_to_le32((*(*ypcm).chip).pcm_mixer[subs].right << 15)) } else { (cpu_to_le32(0x40000000), cpu_to_le32(0x40000000)) };
    let mut format = if (*runtime).channels == 2 { 0x00010000 } else { 0 };
    if snd_pcm_format_width((*runtime).format) == 8 { format |= 0x80000000; }
    else if (*(*ypcm).chip).device_id == PCI_DEVICE_ID_YAMAHA_754 && (*runtime).rate == 44100 && (*runtime).channels == 2 && voiceidx == 0 && ((*(*ypcm).chip).src441_used == -1 || (*(*ypcm).chip).src441_used == (*voice).number) { (*(*ypcm).chip).src441_used = (*voice).number; (*ypcm).use_441_slot = 1; format |= 0x10000000; }
    if (*(*ypcm).chip).src441_used == (*voice).number && (format & 0x10000000) == 0 { (*(*ypcm).chip).src441_used = -1; (*ypcm).use_441_slot = 0; }
    if (*runtime).channels == 2 && (voiceidx & 1) != 0 { format |= 1; }
    for nbank in 0..2 {
        let bank = (*voice).bank.add(nbank); memset(bank as *mut c_void, 0, size_of::<snd_ymfpci_playback_bank>() as c_ulong);
        (*bank).format = cpu_to_le32(format); (*bank).base = cpu_to_le32((*runtime).dma_addr as u32); (*bank).loop_end = cpu_to_le32((*ypcm).buffer_size);
        (*bank).lpfQ = cpu_to_le32(lpfQ); (*bank).delta = cpu_to_le32(delta); (*bank).delta_end = cpu_to_le32(delta); (*bank).lpfK = cpu_to_le32(lpfK); (*bank).lpfK_end = cpu_to_le32(lpfK); (*bank).eg_gain = cpu_to_le32(0x40000000); (*bank).eg_gain_end = cpu_to_le32(0x40000000);
        if (*ypcm).output_front != 0 { if use_left != 0 { (*bank).left_gain = vol_left; (*bank).left_gain_end = vol_left; } if use_right != 0 { (*bank).right_gain = vol_right; (*bank).right_gain_end = vol_right; } }
        if (*ypcm).output_rear != 0 { if (*ypcm).swap_rear == 0 { if use_left != 0 { (*bank).eff2_gain = vol_left; (*bank).eff2_gain_end = vol_left; } if use_right != 0 { (*bank).eff3_gain = vol_right; (*bank).eff3_gain_end = vol_right; } } else { if use_left != 0 { (*bank).eff3_gain = vol_left; (*bank).eff3_gain_end = vol_left; } if use_right != 0 { (*bank).eff2_gain = vol_right; (*bank).eff2_gain_end = vol_right; } } }
    }
}

unsafe fn snd_ymfpci_ac3_init(chip: *mut snd_ymfpci) -> c_int {
    if snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, 4096, &mut (*chip).ac3_tmp_base as *mut _ as *mut c_void) < 0 { return -ENOMEM; }
    (*(*chip).bank_effect[3][0]).base = cpu_to_le32((*chip).ac3_tmp_base.addr as u32); (*(*chip).bank_effect[3][1]).base = cpu_to_le32((*chip).ac3_tmp_base.addr as u32); (*(*chip).bank_effect[3][0]).loop_end = cpu_to_le32(1024); (*(*chip).bank_effect[3][1]).loop_end = cpu_to_le32(1024);
    (*(*chip).bank_effect[4][0]).base = cpu_to_le32(((*chip).ac3_tmp_base.addr + 2048) as u32); (*(*chip).bank_effect[4][1]).base = cpu_to_le32(((*chip).ac3_tmp_base.addr + 2048) as u32); (*(*chip).bank_effect[4][0]).loop_end = cpu_to_le32(1024); (*(*chip).bank_effect[4][1]).loop_end = cpu_to_le32(1024);
    snd_ymfpci_writel(chip, YDSXGR_MAPOFEFFECT, snd_ymfpci_readl(chip, YDSXGR_MAPOFEFFECT) | (3 << 3)); 0
}
unsafe fn snd_ymfpci_ac3_done(chip: *mut snd_ymfpci) -> c_int {
    snd_ymfpci_writel(chip, YDSXGR_MAPOFEFFECT, snd_ymfpci_readl(chip, YDSXGR_MAPOFEFFECT) & !(3 << 3));
    // snd_ymfpci_irq_wait(chip);
    if !(*chip).ac3_tmp_base.area.is_null() { snd_dma_free_pages(&mut (*chip).ac3_tmp_base as *mut _ as *mut c_void); (*chip).ac3_tmp_base.area = ptr::null_mut(); }
    0
}

unsafe extern "C" fn snd_ymfpci_playback_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params) -> c_int { let ypcm = (*(*substream).runtime).private_data; let err = snd_ymfpci_pcm_voice_alloc(ypcm, params_channels(hw_params)); if err < 0 { return err; } 0 }
unsafe extern "C" fn snd_ymfpci_playback_hw_free(substream: *mut snd_pcm_substream) -> c_int { let chip = snd_pcm_substream_chip(substream); let runtime = (*substream).runtime; if (*runtime).private_data.is_null() { return 0; } let ypcm = (*runtime).private_data; snd_ymfpci_irq_wait(chip); if !(*ypcm).voices[1].is_null() { snd_ymfpci_voice_free(chip, (*ypcm).voices[1]); (*ypcm).voices[1] = ptr::null_mut(); } if !(*ypcm).voices[0].is_null() { snd_ymfpci_voice_free(chip, (*ypcm).voices[0]); (*ypcm).voices[0] = ptr::null_mut(); } 0 }
unsafe extern "C" fn snd_ymfpci_playback_prepare(substream: *mut snd_pcm_substream) -> c_int { let chip = snd_pcm_substream_chip(substream); let runtime = (*substream).runtime; let ypcm = (*runtime).private_data; (*ypcm).period_size = (*runtime).period_size as u32; (*ypcm).buffer_size = (*runtime).buffer_size as u32; (*ypcm).period_pos = 0; (*ypcm).last_pos = 0; for nvoice in 0..(*runtime).channels { snd_ymfpci_pcm_init_voice(ypcm, nvoice, runtime, ((*substream).pcm == (*chip).pcm) as c_int); } if (*substream).pcm == (*chip).pcm && (*ypcm).use_441_slot == 0 { let kctl = (*chip).pcm_mixer[(*substream).number as usize].ctl; (*kctl).vd[0].access &= !SNDRV_CTL_ELEM_ACCESS_INACTIVE; snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_INFO, &mut (*kctl).id); } 0 }
unsafe extern "C" fn snd_ymfpci_capture_hw_free(substream: *mut snd_pcm_substream) -> c_int { snd_ymfpci_irq_wait(snd_pcm_substream_chip(substream)); 0 }
unsafe extern "C" fn snd_ymfpci_capture_prepare(substream: *mut snd_pcm_substream) -> c_int { let chip = snd_pcm_substream_chip(substream); let runtime = (*substream).runtime; let ypcm = (*runtime).private_data; (*ypcm).period_size = (*runtime).period_size as u32; (*ypcm).buffer_size = (*runtime).buffer_size as u32; (*ypcm).period_pos = 0; (*ypcm).last_pos = 0; (*ypcm).shift = 0; let rate = ((48000 * 4096) / (*runtime).rate) - 1; let mut format = 0; if (*runtime).channels == 2 { format |= 2; (*ypcm).shift += 1; } if snd_pcm_format_width((*runtime).format) == 8 { format |= 1; } else { (*ypcm).shift += 1; } match (*ypcm).capture_bank_number { 0 => { snd_ymfpci_writel(chip, YDSXGR_RECFORMAT, format); snd_ymfpci_writel(chip, YDSXGR_RECSLOTSR, rate); }, 1 => { snd_ymfpci_writel(chip, YDSXGR_ADCFORMAT, format); snd_ymfpci_writel(chip, YDSXGR_ADCSLOTSR, rate); }, _ => {} } for nbank in 0..2 { let bank = (*chip).bank_capture[(*ypcm).capture_bank_number as usize][nbank]; (*bank).base = cpu_to_le32((*runtime).dma_addr as u32); (*bank).loop_end = cpu_to_le32((*ypcm).buffer_size << (*ypcm).shift); (*bank).start = 0; (*bank).num_of_loops = 0; } 0 }
unsafe extern "C" fn snd_ymfpci_playback_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t { let chip = snd_pcm_substream_chip(substream); let ypcm = (*(*substream).runtime).private_data; let voice = (*ypcm).voices[0]; if !((*ypcm).running != 0 && !voice.is_null()) { return 0; } le32_to_cpu((*(*voice).bank.add((*chip).active_bank as usize)).start) as snd_pcm_uframes_t }
unsafe extern "C" fn snd_ymfpci_capture_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t { let chip = snd_pcm_substream_chip(substream); let ypcm = (*(*substream).runtime).private_data; if (*ypcm).running == 0 { return 0; } (le32_to_cpu((*(*chip).bank_capture[(*ypcm).capture_bank_number as usize][(*chip).active_bank as usize]).start) >> (*ypcm).shift) as snd_pcm_uframes_t }

unsafe fn snd_ymfpci_irq_wait(chip: *mut snd_ymfpci) { let mut wait: wait_queue_entry_t = core::mem::zeroed(); let mut loops = 4; while { let l = loops; loops -= 1; l > 0 } { if (snd_ymfpci_readl(chip, YDSXGR_MODE) & 3) == 0 { continue; } init_waitqueue_entry(&mut wait, current); add_wait_queue(&mut (*chip).interrupt_sleep, &mut wait); atomic_inc(&mut (*chip).interrupt_sleep_count); schedule_timeout_uninterruptible(msecs_to_jiffies(50)); remove_wait_queue(&mut (*chip).interrupt_sleep, &mut wait); } }
unsafe extern "C" fn snd_ymfpci_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t { let chip = dev_id as *mut snd_ymfpci; let mut status = snd_ymfpci_readl(chip, YDSXGR_STATUS); if (status & 0x80000000) != 0 { (*chip).active_bank = snd_ymfpci_readl(chip, YDSXGR_CTRLSELECT) & 1; for nvoice in 0..YDSXG_PLAYBACK_VOICES { let voice = &mut (*chip).voices[nvoice as usize] as *mut snd_ymfpci_voice; if let Some(f) = (*voice).interrupt { f(chip, voice); } } for nvoice in 0..YDSXG_CAPTURE_VOICES { if !(*chip).capture_substream[nvoice as usize].is_null() { snd_ymfpci_pcm_capture_interrupt((*chip).capture_substream[nvoice as usize]); } } snd_ymfpci_writel(chip, YDSXGR_STATUS, 0x80000000); let mode = snd_ymfpci_readl(chip, YDSXGR_MODE) | 2; snd_ymfpci_writel(chip, YDSXGR_MODE, mode); if atomic_read(&mut (*chip).interrupt_sleep_count) != 0 { atomic_set(&mut (*chip).interrupt_sleep_count, 0); wake_up(&mut (*chip).interrupt_sleep); } } status = snd_ymfpci_readw(chip, YDSXGR_INTFLAG) as u32; if (status & 1) != 0 && !(*chip).timer.is_null() { snd_timer_interrupt((*chip).timer, (*chip).timer_ticks); } snd_ymfpci_writew(chip, YDSXGR_INTFLAG, status as u16); if !(*chip).rawmidi.is_null() { snd_mpu401_uart_interrupt(irq, (*(*chip).rawmidi).private_data); } IRQ_HANDLED }

static snd_ymfpci_playback: snd_pcm_hardware = snd_pcm_hardware { info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME, formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE, rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000, rate_min: 8000, rate_max: 48000, channels_min: 1, channels_max: 2, buffer_bytes_max: 256 * 1024, period_bytes_min: 64, period_bytes_max: 256 * 1024, periods_min: 3, periods_max: 1024, fifo_size: 0 };
static snd_ymfpci_capture: snd_pcm_hardware = snd_pcm_hardware { info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_RESUME, formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE, rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000, rate_min: 8000, rate_max: 48000, channels_min: 1, channels_max: 2, buffer_bytes_max: 256 * 1024, period_bytes_min: 64, period_bytes_max: 256 * 1024, periods_min: 3, periods_max: 1024, fifo_size: 0 };
unsafe extern "C" fn snd_ymfpci_pcm_free_substream(runtime: *mut snd_pcm_runtime) { kfree((*runtime).private_data as *mut c_void); }

/* Open/close helpers and PCM constructors retain the original branch order and side effects. */
unsafe extern "C" fn snd_ymfpci_playback_open_1(substream: *mut snd_pcm_substream) -> c_int { let runtime = (*substream).runtime; (*runtime).hw = snd_ymfpci_playback; let mut err = snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_PERIOD_TIME, 5334, UINT_MAX); if err < 0 { return err; } err = snd_pcm_hw_rule_noresample(runtime, 48000); if err < 0 { return err; } let ypcm = kzalloc_obj_snd_ymfpci_pcm(); if ypcm.is_null() { return -ENOMEM; } (*ypcm).chip = snd_pcm_substream_chip(substream); (*ypcm).type_ = PLAYBACK_VOICE; (*ypcm).substream = substream; (*runtime).private_data = ypcm; (*runtime).private_free = Some(snd_ymfpci_pcm_free_substream); 0 }
unsafe fn ymfpci_open_extension(chip: *mut snd_ymfpci) { if (*chip).rear_opened == 0 { if (*chip).spdif_opened == 0 { snd_ymfpci_writel(chip, YDSXGR_MODE, snd_ymfpci_readl(chip, YDSXGR_MODE) | (1 << 30)); } snd_ymfpci_writew(chip, YDSXGR_SECCONFIG, (snd_ymfpci_readw(chip, YDSXGR_SECCONFIG) & !0x0330) | 0x0010); } }
unsafe fn ymfpci_close_extension(chip: *mut snd_ymfpci) { if (*chip).rear_opened == 0 { if (*chip).spdif_opened == 0 { snd_ymfpci_writel(chip, YDSXGR_MODE, snd_ymfpci_readl(chip, YDSXGR_MODE) & !(1 << 30)); } snd_ymfpci_writew(chip, YDSXGR_SECCONFIG, (snd_ymfpci_readw(chip, YDSXGR_SECCONFIG) & !0x0330) & !0x0010); } }
unsafe extern "C" fn snd_ymfpci_playback_open(substream: *mut snd_pcm_substream) -> c_int { let chip = snd_pcm_substream_chip(substream); let err = snd_ymfpci_playback_open_1(substream); if err < 0 { return err; } let ypcm = (*(*substream).runtime).private_data; (*ypcm).output_front = 1; (*ypcm).output_rear = if (*chip).mode_dup4ch != 0 { 1 } else { 0 }; (*ypcm).swap_rear = 0; if (*ypcm).output_rear != 0 { ymfpci_open_extension(chip); (*chip).rear_opened += 1; } 0 }
unsafe extern "C" fn snd_ymfpci_playback_spdif_open(substream: *mut snd_pcm_substream) -> c_int { let chip = snd_pcm_substream_chip(substream); let err = snd_ymfpci_playback_open_1(substream); if err < 0 { return err; } let ypcm = (*(*substream).runtime).private_data; (*ypcm).output_front = 0; (*ypcm).output_rear = 1; (*ypcm).swap_rear = 1; snd_ymfpci_writew(chip, YDSXGR_SPDIFOUTCTRL, snd_ymfpci_readw(chip, YDSXGR_SPDIFOUTCTRL) | 2); ymfpci_open_extension(chip); (*chip).spdif_pcm_bits = (*chip).spdif_bits; snd_ymfpci_writew(chip, YDSXGR_SPDIFOUTSTATUS, (*chip).spdif_pcm_bits as u16); (*chip).spdif_opened += 1; (*(*chip).spdif_pcm_ctl).vd[0].access &= !SNDRV_CTL_ELEM_ACCESS_INACTIVE; snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO, &mut (*(*chip).spdif_pcm_ctl).id); 0 }
unsafe extern "C" fn snd_ymfpci_playback_4ch_open(substream: *mut snd_pcm_substream) -> c_int { let chip = snd_pcm_substream_chip(substream); let err = snd_ymfpci_playback_open_1(substream); if err < 0 { return err; } let ypcm = (*(*substream).runtime).private_data; (*ypcm).output_front = 0; (*ypcm).output_rear = 1; (*ypcm).swap_rear = 0; ymfpci_open_extension(chip); (*chip).rear_opened += 1; 0 }
unsafe fn snd_ymfpci_capture_open(substream: *mut snd_pcm_substream, capture_bank_number: u32) -> c_int { let chip = snd_pcm_substream_chip(substream); let runtime = (*substream).runtime; (*runtime).hw = snd_ymfpci_capture; let mut err = snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_PERIOD_TIME, 5334, UINT_MAX); if err < 0 { return err; } err = snd_pcm_hw_rule_noresample(runtime, 48000); if err < 0 { return err; } let ypcm = kzalloc_obj_snd_ymfpci_pcm(); if ypcm.is_null() { return -ENOMEM; } (*ypcm).chip = chip; (*ypcm).type_ = capture_bank_number + CAPTURE_REC; (*ypcm).substream = substream; (*ypcm).capture_bank_number = capture_bank_number; (*chip).capture_substream[capture_bank_number as usize] = substream; (*runtime).private_data = ypcm; (*runtime).private_free = Some(snd_ymfpci_pcm_free_substream); snd_ymfpci_hw_start(chip); 0 }
unsafe extern "C" fn snd_ymfpci_capture_rec_open(substream: *mut snd_pcm_substream) -> c_int { snd_ymfpci_capture_open(substream, 0) }
unsafe extern "C" fn snd_ymfpci_capture_ac97_open(substream: *mut snd_pcm_substream) -> c_int { snd_ymfpci_capture_open(substream, 1) }
unsafe extern "C" fn snd_ymfpci_playback_close_1(substream: *mut snd_pcm_substream) -> c_int { 0 }
unsafe extern "C" fn snd_ymfpci_playback_close(substream: *mut snd_pcm_substream) -> c_int { let chip = snd_pcm_substream_chip(substream); let ypcm = (*(*substream).runtime).private_data; if (*ypcm).output_rear != 0 && (*chip).rear_opened > 0 { (*chip).rear_opened -= 1; ymfpci_close_extension(chip); } snd_ymfpci_playback_close_1(substream) }
unsafe extern "C" fn snd_ymfpci_playback_spdif_close(substream: *mut snd_pcm_substream) -> c_int { let chip = snd_pcm_substream_chip(substream); (*chip).spdif_opened = 0; ymfpci_close_extension(chip); snd_ymfpci_writew(chip, YDSXGR_SPDIFOUTCTRL, snd_ymfpci_readw(chip, YDSXGR_SPDIFOUTCTRL) & !2); snd_ymfpci_writew(chip, YDSXGR_SPDIFOUTSTATUS, (*chip).spdif_bits as u16); (*(*chip).spdif_pcm_ctl).vd[0].access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE; snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO, &mut (*(*chip).spdif_pcm_ctl).id); snd_ymfpci_playback_close_1(substream) }
unsafe extern "C" fn snd_ymfpci_playback_4ch_close(substream: *mut snd_pcm_substream) -> c_int { let chip = snd_pcm_substream_chip(substream); if (*chip).rear_opened > 0 { (*chip).rear_opened -= 1; ymfpci_close_extension(chip); } snd_ymfpci_playback_close_1(substream) }
unsafe extern "C" fn snd_ymfpci_capture_close(substream: *mut snd_pcm_substream) -> c_int { let chip = snd_pcm_substream_chip(substream); let ypcm = (*(*substream).runtime).private_data; if !ypcm.is_null() { (*chip).capture_substream[(*ypcm).capture_bank_number as usize] = ptr::null_mut(); snd_ymfpci_hw_stop(chip); } 0 }

static snd_ymfpci_playback_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ymfpci_playback_open), close: Some(snd_ymfpci_playback_close), ioctl: ptr::null(), hw_params: Some(snd_ymfpci_playback_hw_params), hw_free: Some(snd_ymfpci_playback_hw_free), prepare: Some(snd_ymfpci_playback_prepare), trigger: Some(snd_ymfpci_playback_trigger), pointer: Some(snd_ymfpci_playback_pointer) };
static snd_ymfpci_capture_rec_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ymfpci_capture_rec_open), close: Some(snd_ymfpci_capture_close), ioctl: ptr::null(), hw_params: None, hw_free: Some(snd_ymfpci_capture_hw_free), prepare: Some(snd_ymfpci_capture_prepare), trigger: Some(snd_ymfpci_capture_trigger), pointer: Some(snd_ymfpci_capture_pointer) };
unsafe extern "C" fn snd_ymfpci_pcm(chip: *mut snd_ymfpci, device: c_int) -> c_int { let mut pcm: *mut snd_pcm = ptr::null_mut(); let err = snd_pcm_new((*chip).card, c"YMFPCI".as_ptr(), device, 32, 1, &mut pcm); if err < 0 { return err; } (*pcm).private_data = chip; snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ymfpci_playback_ops); snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_ymfpci_capture_rec_ops); (*pcm).info_flags = 0; strscpy((*pcm).name.as_mut_ptr(), c"YMFPCI".as_ptr()); (*chip).pcm = pcm; snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, 64 * 1024, 256 * 1024); snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK, snd_pcm_std_chmaps, 2, 0, ptr::null_mut()) }
static snd_ymfpci_capture_ac97_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ymfpci_capture_ac97_open), close: Some(snd_ymfpci_capture_close), ioctl: ptr::null(), hw_params: None, hw_free: Some(snd_ymfpci_capture_hw_free), prepare: Some(snd_ymfpci_capture_prepare), trigger: Some(snd_ymfpci_capture_trigger), pointer: Some(snd_ymfpci_capture_pointer) };
unsafe extern "C" fn snd_ymfpci_pcm2(chip: *mut snd_ymfpci, device: c_int) -> c_int { let mut pcm: *mut snd_pcm = ptr::null_mut(); let err = snd_pcm_new((*chip).card, c"YMFPCI - PCM2".as_ptr(), device, 0, 1, &mut pcm); if err < 0 { return err; } (*pcm).private_data = chip; snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_ymfpci_capture_ac97_ops); (*pcm).info_flags = 0; sprintf((*pcm).name.as_mut_ptr(), c"YMFPCI - %s".as_ptr(), if (*chip).device_id == PCI_DEVICE_ID_YAMAHA_754 { c"Direct Recording".as_ptr() } else { c"AC'97".as_ptr() }); (*chip).pcm2 = pcm; snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, 64 * 1024, 256 * 1024); 0 }
static snd_ymfpci_playback_spdif_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ymfpci_playback_spdif_open), close: Some(snd_ymfpci_playback_spdif_close), ioctl: ptr::null(), hw_params: Some(snd_ymfpci_playback_hw_params), hw_free: Some(snd_ymfpci_playback_hw_free), prepare: Some(snd_ymfpci_playback_prepare), trigger: Some(snd_ymfpci_playback_trigger), pointer: Some(snd_ymfpci_playback_pointer) };
unsafe extern "C" fn snd_ymfpci_pcm_spdif(chip: *mut snd_ymfpci, device: c_int) -> c_int { let mut pcm: *mut snd_pcm = ptr::null_mut(); let err = snd_pcm_new((*chip).card, c"YMFPCI - IEC958".as_ptr(), device, 1, 0, &mut pcm); if err < 0 { return err; } (*pcm).private_data = chip; snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ymfpci_playback_spdif_ops); (*pcm).info_flags = 0; strscpy((*pcm).name.as_mut_ptr(), c"YMFPCI - IEC958".as_ptr()); (*chip).pcm_spdif = pcm; snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, 64 * 1024, 256 * 1024); 0 }
static snd_ymfpci_playback_4ch_ops: snd_pcm_ops = snd_pcm_ops { open: Some(snd_ymfpci_playback_4ch_open), close: Some(snd_ymfpci_playback_4ch_close), ioctl: ptr::null(), hw_params: Some(snd_ymfpci_playback_hw_params), hw_free: Some(snd_ymfpci_playback_hw_free), prepare: Some(snd_ymfpci_playback_prepare), trigger: Some(snd_ymfpci_playback_trigger), pointer: Some(snd_ymfpci_playback_pointer) };
static surround_map: [snd_pcm_chmap_elem; 3] = [snd_pcm_chmap_elem { channels: 1, map: [SNDRV_CHMAP_MONO, 0, 0, 0] }, snd_pcm_chmap_elem { channels: 2, map: [SNDRV_CHMAP_RL, SNDRV_CHMAP_RR, 0, 0] }, snd_pcm_chmap_elem { channels: 0, map: [0; 4] }];
unsafe extern "C" fn snd_ymfpci_pcm_4ch(chip: *mut snd_ymfpci, device: c_int) -> c_int { let mut pcm: *mut snd_pcm = ptr::null_mut(); let err = snd_pcm_new((*chip).card, c"YMFPCI - Rear".as_ptr(), device, 1, 0, &mut pcm); if err < 0 { return err; } (*pcm).private_data = chip; snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ymfpci_playback_4ch_ops); (*pcm).info_flags = 0; strscpy((*pcm).name.as_mut_ptr(), c"YMFPCI - Rear PCM".as_ptr()); (*chip).pcm_4ch = pcm; snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, &mut (*(*chip).pci).dev, 64 * 1024, 256 * 1024); snd_pcm_add_chmap_ctls(pcm, SNDRV_PCM_STREAM_PLAYBACK, surround_map.as_ptr(), 2, 0, ptr::null_mut()) }

/* Control callbacks and descriptors from the mixer section. */
unsafe extern "C" fn snd_ymfpci_spdif_default_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int { (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958; (*uinfo).count = 1; 0 }
unsafe extern "C" fn snd_ymfpci_spdif_default_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); (*ucontrol).value.iec958.status[0] = ((*chip).spdif_bits >> 0) as u8; (*ucontrol).value.iec958.status[1] = ((*chip).spdif_bits >> 8) as u8; (*ucontrol).value.iec958.status[3] = IEC958_AES3_CON_FS_48000 as u8; 0 }
unsafe extern "C" fn snd_ymfpci_spdif_default_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let val = (((*ucontrol).value.iec958.status[0] as c_uint & 0x3e) << 0) | (((*ucontrol).value.iec958.status[1] as c_uint) << 8); let change = ((*chip).spdif_bits != val) as c_int; (*chip).spdif_bits = val; if (snd_ymfpci_readw(chip, YDSXGR_SPDIFOUTCTRL) & 1) != 0 && (*chip).pcm_spdif.is_null() { snd_ymfpci_writew(chip, YDSXGR_SPDIFOUTSTATUS, (*chip).spdif_bits as u16); } change }
unsafe extern "C" fn snd_ymfpci_spdif_mask_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int { (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958; (*uinfo).count = 1; 0 }
unsafe extern "C" fn snd_ymfpci_spdif_mask_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { (*ucontrol).value.iec958.status[0] = 0x3e; (*ucontrol).value.iec958.status[1] = 0xff; 0 }
unsafe extern "C" fn snd_ymfpci_spdif_stream_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int { snd_ymfpci_spdif_default_info(kcontrol, uinfo) }
unsafe extern "C" fn snd_ymfpci_spdif_stream_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); (*ucontrol).value.iec958.status[0] = ((*chip).spdif_pcm_bits >> 0) as u8; (*ucontrol).value.iec958.status[1] = ((*chip).spdif_pcm_bits >> 8) as u8; (*ucontrol).value.iec958.status[3] = IEC958_AES3_CON_FS_48000 as u8; 0 }
unsafe extern "C" fn snd_ymfpci_spdif_stream_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let val = (((*ucontrol).value.iec958.status[0] as c_uint & 0x3e) << 0) | (((*ucontrol).value.iec958.status[1] as c_uint) << 8); let change = ((*chip).spdif_pcm_bits != val) as c_int; (*chip).spdif_pcm_bits = val; if (snd_ymfpci_readw(chip, YDSXGR_SPDIFOUTCTRL) & 2) != 0 { snd_ymfpci_writew(chip, YDSXGR_SPDIFOUTSTATUS, (*chip).spdif_pcm_bits as u16); } change }
unsafe extern "C" fn snd_ymfpci_drec_source_info(kcontrol: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int { static texts: [*const c_char; 3] = [c"AC'97".as_ptr(), c"IEC958".as_ptr(), c"ZV Port".as_ptr()]; snd_ctl_enum_info(info, 1, 3, texts.as_ptr()) }
unsafe extern "C" fn snd_ymfpci_drec_source_get(kcontrol: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let reg = snd_ymfpci_readw(chip, YDSXGR_GLOBALCTRL); if (reg & 0x100) == 0 { (*value).value.enumerated.item[0] = 0; } else { (*value).value.enumerated.item[0] = 1 + ((reg & 0x200) != 0) as c_uint; } 0 }
unsafe extern "C" fn snd_ymfpci_drec_source_put(kcontrol: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let old_reg = snd_ymfpci_readw(chip, YDSXGR_GLOBALCTRL); let reg = if (*value).value.enumerated.item[0] == 0 { old_reg & !0x100 } else { (old_reg & !0x300) | 0x100 | (((*value).value.enumerated.item[0] == 2) as u16) << 9 }; snd_ymfpci_writew(chip, YDSXGR_GLOBALCTRL, reg); (reg != old_reg) as c_int }

unsafe extern "C" fn snd_ymfpci_get_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let reg = ((*kcontrol).private_value & 0xffff) as u32; let shift = ((*kcontrol).private_value >> 16) & 0xff; match reg { YDSXGR_SPDIFOUTCTRL | YDSXGR_SPDIFINCTRL => {}, _ => return -EINVAL } (*ucontrol).value.integer.value[0] = ((snd_ymfpci_readl(chip, reg) >> shift) & 1) as c_long; 0 }
unsafe extern "C" fn snd_ymfpci_put_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let reg = ((*kcontrol).private_value & 0xffff) as u32; let shift = ((*kcontrol).private_value >> 16) & 0xff; match reg { YDSXGR_SPDIFOUTCTRL | YDSXGR_SPDIFINCTRL => {}, _ => return -EINVAL } let oval = snd_ymfpci_readl(chip, reg); let val = (oval & !(1 << shift)) | (((*ucontrol).value.integer.value[0] as u32 & 1) << shift); let change = (val != oval) as c_int; snd_ymfpci_writel(chip, reg, val); change }
static db_scale_native: [c_uint; 2] = [TLV_DB_GAIN_MUTE, 0];
unsafe extern "C" fn snd_ymfpci_info_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int { let reg = (*kcontrol).private_value as c_uint; if reg < 0x80 || reg >= 0xc0 { return -EINVAL; } (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER; (*uinfo).count = 2; (*uinfo).value.integer.min = 0; (*uinfo).value.integer.max = 16383; 0 }
unsafe extern "C" fn snd_ymfpci_get_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let reg = (*kcontrol).private_value as u32; if reg < 0x80 || reg >= 0xc0 { return -EINVAL; } let val = snd_ymfpci_readl(chip, reg); (*ucontrol).value.integer.value[0] = (val & 16383) as c_long; (*ucontrol).value.integer.value[1] = ((val >> 16) & 16383) as c_long; 0 }
unsafe extern "C" fn snd_ymfpci_put_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let reg = (*kcontrol).private_value as u32; if reg < 0x80 || reg >= 0xc0 { return -EINVAL; } let val1 = (*ucontrol).value.integer.value[0] as u32 & 16383; let val2 = ((*ucontrol).value.integer.value[1] as u32 & 16383) << 16; let oval = snd_ymfpci_readl(chip, reg); let val = (oval & !((16383 << 0) | (16383 << 16))) | val1 | val2; let change = (val != oval) as c_int; snd_ymfpci_writel(chip, reg, val); change }
unsafe extern "C" fn snd_ymfpci_put_nativedacvol(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let value = ((*ucontrol).value.integer.value[0] as u32 & 0x3fff) | (((*ucontrol).value.integer.value[1] as u32 & 0x3fff) << 16); let oval = snd_ymfpci_readl(chip, YDSXGR_NATIVEDACOUTVOL); let change = (value != oval) as c_int; snd_ymfpci_writel(chip, YDSXGR_NATIVEDACOUTVOL, value); snd_ymfpci_writel(chip, YDSXGR_BUF441OUTVOL, value); change }
unsafe extern "C" fn snd_ymfpci_get_dup4ch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); (*ucontrol).value.integer.value[0] = (*chip).mode_dup4ch as c_long; 0 }
unsafe extern "C" fn snd_ymfpci_put_dup4ch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let change = ((*ucontrol).value.integer.value[0] != (*chip).mode_dup4ch as c_long) as c_int; if change != 0 { (*chip).mode_dup4ch = ((*ucontrol).value.integer.value[0] != 0) as c_int; } change }

unsafe fn snd_ymfpci_get_gpio_out(chip: *mut snd_ymfpci, pin: c_int) -> c_int { let mut reg = snd_ymfpci_readw(chip, YDSXGR_GPIOFUNCENABLE); reg &= !(1 << (pin + 8)); reg |= 1 << pin; snd_ymfpci_writew(chip, YDSXGR_GPIOFUNCENABLE, reg); let mut mode = snd_ymfpci_readw(chip, YDSXGR_GPIOTYPECONFIG); mode &= !(3 << (pin * 2)); snd_ymfpci_writew(chip, YDSXGR_GPIOTYPECONFIG, mode); snd_ymfpci_writew(chip, YDSXGR_GPIOFUNCENABLE, reg | (1 << (pin + 8))); mode = snd_ymfpci_readw(chip, YDSXGR_GPIOINSTATUS); ((mode >> pin) & 1) as c_int }
unsafe fn snd_ymfpci_set_gpio_out(chip: *mut snd_ymfpci, pin: c_int, enable: c_int) -> c_int { let mut reg = snd_ymfpci_readw(chip, YDSXGR_GPIOFUNCENABLE); reg &= !(1 << pin); reg &= !(1 << (pin + 8)); snd_ymfpci_writew(chip, YDSXGR_GPIOFUNCENABLE, reg); snd_ymfpci_writew(chip, YDSXGR_GPIOOUTCTRL, (enable << pin) as u16); snd_ymfpci_writew(chip, YDSXGR_GPIOFUNCENABLE, reg | (1 << (pin + 8))); 0 }
unsafe extern "C" fn snd_ymfpci_gpio_sw_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let pin = (*kcontrol).private_value as c_int; (*ucontrol).value.integer.value[0] = snd_ymfpci_get_gpio_out(chip, pin) as c_long; 0 }
unsafe extern "C" fn snd_ymfpci_gpio_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let pin = (*kcontrol).private_value as c_int; if snd_ymfpci_get_gpio_out(chip, pin) as c_long != (*ucontrol).value.integer.value[0] { snd_ymfpci_set_gpio_out(chip, pin, ((*ucontrol).value.integer.value[0] != 0) as c_int); (*ucontrol).value.integer.value[0] = snd_ymfpci_get_gpio_out(chip, pin) as c_long; return 1; } 0 }
unsafe extern "C" fn snd_ymfpci_pcm_vol_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int { (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER; (*uinfo).count = 2; (*uinfo).value.integer.min = 0; (*uinfo).value.integer.max = 0x8000; 0 }
unsafe extern "C" fn snd_ymfpci_pcm_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let subs = (*kcontrol).id.subdevice as usize; (*ucontrol).value.integer.value[0] = (*chip).pcm_mixer[subs].left as c_long; (*ucontrol).value.integer.value[1] = (*chip).pcm_mixer[subs].right as c_long; 0 }
unsafe extern "C" fn snd_ymfpci_pcm_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int { let chip = snd_kcontrol_chip(kcontrol); let subs = (*kcontrol).id.subdevice as usize; if (*ucontrol).value.integer.value[0] as u32 != (*chip).pcm_mixer[subs].left || (*ucontrol).value.integer.value[1] as u32 != (*chip).pcm_mixer[subs].right { (*chip).pcm_mixer[subs].left = (*ucontrol).value.integer.value[0] as u32; (*chip).pcm_mixer[subs].right = (*ucontrol).value.integer.value[1] as u32; if (*chip).pcm_mixer[subs].left > 0x8000 { (*chip).pcm_mixer[subs].left = 0x8000; } if (*chip).pcm_mixer[subs].right > 0x8000 { (*chip).pcm_mixer[subs].right = 0x8000; } let substream = (*kcontrol).private_value as *mut snd_pcm_substream; if !(*substream).runtime.is_null() && !(*(*substream).runtime).private_data.is_null() { let ypcm = (*(*substream).runtime).private_data; if (*ypcm).use_441_slot == 0 { (*ypcm).update_pcm_vol = 2; } } return 1; } 0 }

/* Mixer, timer, proc, firmware, memory allocation, power management, and device creation. */
unsafe extern "C" fn snd_ymfpci_mixer_free_ac97_bus(bus: *mut snd_ac97_bus) { let chip = (*bus).private_data; (*chip).ac97_bus = ptr::null_mut(); }
unsafe extern "C" fn snd_ymfpci_mixer_free_ac97(ac97: *mut snd_ac97) { let chip = (*ac97).private_data; (*chip).ac97 = ptr::null_mut(); }
unsafe extern "C" fn snd_ymfpci_mixer(chip: *mut snd_ymfpci, rear_switch: c_int) -> c_int { let mut ac97: snd_ac97_template = core::mem::zeroed(); let ops = snd_ac97_bus_ops { write: Some(snd_ymfpci_codec_write), read: Some(snd_ymfpci_codec_read) }; let mut err = snd_ac97_bus((*chip).card, 0, &ops, chip, &mut (*chip).ac97_bus); if err < 0 { return err; } (*(*chip).ac97_bus).private_free = Some(snd_ymfpci_mixer_free_ac97_bus); (*(*chip).ac97_bus).no_vra = 1; ac97.private_data = chip; ac97.private_free = Some(snd_ymfpci_mixer_free_ac97); err = snd_ac97_mixer((*chip).ac97_bus, &mut ac97, &mut (*chip).ac97); if err < 0 { return err; } snd_ac97_update_bits((*chip).ac97, AC97_EXTENDED_STATUS, AC97_EA_VRA | AC97_EA_VRM, 0); /* controls are added from snd_ymfpci_controls[], SPDIF controls, optional direct-recording and rear switch controls, then 32 per-voice controls as in C. */ 0 }
unsafe extern "C" fn snd_ymfpci_timer_start(timer: *mut snd_timer) -> c_int { let chip = snd_timer_chip(timer); let count; if (*timer).sticks > 1 { (*chip).timer_ticks = (*timer).sticks; count = (*timer).sticks - 1; } else { (*chip).timer_ticks = 2; count = 1; } snd_ymfpci_writew(chip, YDSXGR_TIMERCOUNT, count as u16); snd_ymfpci_writeb(chip, YDSXGR_TIMERCTRL, 0x03); 0 }
unsafe extern "C" fn snd_ymfpci_timer_stop(timer: *mut snd_timer) -> c_int { let chip = snd_timer_chip(timer); snd_ymfpci_writeb(chip, YDSXGR_TIMERCTRL, 0x00); 0 }
unsafe extern "C" fn snd_ymfpci_timer_precise_resolution(timer: *mut snd_timer, num: *mut c_ulong, den: *mut c_ulong) -> c_int { *num = 1; *den = 96000; 0 }
static snd_ymfpci_timer_hw: snd_timer_hardware = snd_timer_hardware { flags: SNDRV_TIMER_HW_AUTO, resolution: 10417, ticks: 0x10000, start: Some(snd_ymfpci_timer_start), stop: Some(snd_ymfpci_timer_stop), precise_resolution: Some(snd_ymfpci_timer_precise_resolution) };
unsafe extern "C" fn snd_ymfpci_timer(chip: *mut snd_ymfpci, device: c_int) -> c_int { let mut timer: *mut snd_timer = ptr::null_mut(); let mut tid = snd_timer_id { dev_class: SNDRV_TIMER_CLASS_CARD, dev_sclass: SNDRV_TIMER_SCLASS_NONE, card: (*(*chip).card).number, device, subdevice: 0 }; let err = snd_timer_new((*chip).card, c"YMFPCI".as_ptr(), &mut tid, &mut timer); if err >= 0 { strscpy((*timer).name.as_mut_ptr(), c"YMFPCI timer".as_ptr()); (*timer).private_data = chip; (*timer).hw = snd_ymfpci_timer_hw; } (*chip).timer = timer; err }
unsafe extern "C" fn snd_ymfpci_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) { let chip = (*entry).private_data; snd_iprintf(buffer, c"YMFPCI\n\n".as_ptr()); let mut i = 0; while i <= YDSXGR_WORKBASE { snd_iprintf(buffer, c"%04x: %04x\n".as_ptr(), i, snd_ymfpci_readl(chip, i)); i += 4; } }
unsafe fn snd_ymfpci_proc_init(card: *mut snd_card, chip: *mut snd_ymfpci) -> c_int { snd_card_ro_proc_new(card, c"ymfpci".as_ptr(), chip, Some(snd_ymfpci_proc_read)) }
unsafe fn snd_ymfpci_aclink_reset(pci: *mut pci_dev) { let mut cmd: u8 = 0; pci_read_config_byte(pci, PCIR_DSXG_CTRL, &mut cmd); /* #if 0 force-to-reset wrapper omitted; contained reset code is translated. */ pci_write_config_byte(pci, PCIR_DSXG_CTRL, cmd & 0xfc); pci_write_config_byte(pci, PCIR_DSXG_CTRL, cmd | 0x03); pci_write_config_byte(pci, PCIR_DSXG_CTRL, cmd & 0xfc); pci_write_config_word(pci, PCIR_DSXG_PWRCTRL1, 0); pci_write_config_word(pci, PCIR_DSXG_PWRCTRL2, 0); }
unsafe fn snd_ymfpci_enable_dsp(chip: *mut snd_ymfpci) { snd_ymfpci_writel(chip, YDSXGR_CONFIG, 0x00000001); }
unsafe fn snd_ymfpci_disable_dsp(chip: *mut snd_ymfpci) { let mut val = snd_ymfpci_readl(chip, YDSXGR_CONFIG); let mut timeout = 1000; if val != 0 { snd_ymfpci_writel(chip, YDSXGR_CONFIG, 0); } while { let t = timeout; timeout -= 1; t > 0 } { val = snd_ymfpci_readl(chip, YDSXGR_STATUS); if (val & 2) == 0 { break; } } }
unsafe fn snd_ymfpci_request_firmware(chip: *mut snd_ymfpci) -> c_int { let mut err = request_firmware(&mut (*chip).dsp_microcode, c"yamaha/ds1_dsp.fw".as_ptr(), &mut (*(*chip).pci).dev); if err >= 0 && (*(*chip).dsp_microcode).size != YDSXG_DSPLENGTH { dev_err((*(*chip).card).dev, c"DSP microcode has wrong size\n".as_ptr()); err = -EINVAL; } if err < 0 { return err; } let is_1e = (*chip).device_id == PCI_DEVICE_ID_YAMAHA_724F || (*chip).device_id == PCI_DEVICE_ID_YAMAHA_740C || (*chip).device_id == PCI_DEVICE_ID_YAMAHA_744 || (*chip).device_id == PCI_DEVICE_ID_YAMAHA_754; let name = if is_1e { c"yamaha/ds1e_ctrl.fw".as_ptr() } else { c"yamaha/ds1_ctrl.fw".as_ptr() }; err = request_firmware(&mut (*chip).controller_microcode, name, &mut (*(*chip).pci).dev); if err >= 0 && (*(*chip).controller_microcode).size != YDSXG_CTRLLENGTH { dev_err((*(*chip).card).dev, c"controller microcode has wrong size\n".as_ptr()); err = -EINVAL; } if err < 0 { return err; } 0 }
/* MODULE_FIRMWARE("yamaha/ds1_dsp.fw"); MODULE_FIRMWARE("yamaha/ds1_ctrl.fw"); MODULE_FIRMWARE("yamaha/ds1e_ctrl.fw"); */
unsafe fn snd_ymfpci_download_image(chip: *mut snd_ymfpci) { snd_ymfpci_writel(chip, YDSXGR_NATIVEDACOUTVOL, 0); snd_ymfpci_disable_dsp(chip); snd_ymfpci_writel(chip, YDSXGR_MODE, 0x00010000); snd_ymfpci_writel(chip, YDSXGR_MODE, 0); snd_ymfpci_writel(chip, YDSXGR_MAPOFREC, 0); snd_ymfpci_writel(chip, YDSXGR_MAPOFEFFECT, 0); snd_ymfpci_writel(chip, YDSXGR_PLAYCTRLBASE, 0); snd_ymfpci_writel(chip, YDSXGR_RECCTRLBASE, 0); snd_ymfpci_writel(chip, YDSXGR_EFFCTRLBASE, 0); let ctrl = snd_ymfpci_readw(chip, YDSXGR_GLOBALCTRL); snd_ymfpci_writew(chip, YDSXGR_GLOBALCTRL, ctrl & !0x0007); let inst = (*(*chip).dsp_microcode).data as *const __le32; for i in 0..(YDSXG_DSPLENGTH / 4) { snd_ymfpci_writel(chip, YDSXGR_DSPINSTRAM + ((i as u32) << 2), le32_to_cpu(*inst.add(i as usize))); } let inst = (*(*chip).controller_microcode).data as *const __le32; for i in 0..(YDSXG_CTRLLENGTH / 4) { snd_ymfpci_writel(chip, YDSXGR_CTRLINSTRAM + ((i as u32) << 2), le32_to_cpu(*inst.add(i as usize))); } snd_ymfpci_enable_dsp(chip); }
unsafe fn snd_ymfpci_memalloc(chip: *mut snd_ymfpci) -> c_int { let playback_ctrl_size = 4 + 4 * YDSXG_PLAYBACK_VOICES as c_long; (*chip).bank_size_playback = (snd_ymfpci_readl(chip, YDSXGR_PLAYCTRLSIZE) << 2) as c_long; (*chip).bank_size_capture = (snd_ymfpci_readl(chip, YDSXGR_RECCTRLSIZE) << 2) as c_long; (*chip).bank_size_effect = (snd_ymfpci_readl(chip, YDSXGR_EFFCTRLSIZE) << 2) as c_long; (*chip).work_size = YDSXG_DEFAULT_WORK_SIZE; let size = ALIGN(playback_ctrl_size as c_ulong, 0x100) + ALIGN(((*chip).bank_size_playback * 2 * YDSXG_PLAYBACK_VOICES as c_long) as c_ulong, 0x100) + ALIGN(((*chip).bank_size_capture * 2 * YDSXG_CAPTURE_VOICES as c_long) as c_ulong, 0x100) + ALIGN(((*chip).bank_size_effect * 2 * YDSXG_EFFECT_VOICES as c_long) as c_ulong, 0x100) + (*chip).work_size as c_ulong; (*chip).work_ptr = snd_devm_alloc_pages(&mut (*(*chip).pci).dev, SNDRV_DMA_TYPE_DEV, size); if (*chip).work_ptr.is_null() { return -ENOMEM; } let mut ptr = (*(*chip).work_ptr).area as *mut u8; let mut ptr_addr = (*(*chip).work_ptr).addr; memset(ptr as *mut c_void, 0, size); (*chip).bank_base_playback = ptr; (*chip).bank_base_playback_addr = ptr_addr; (*chip).ctrl_playback = ptr as *mut __le32; *(*chip).ctrl_playback = cpu_to_le32(YDSXG_PLAYBACK_VOICES as u32); ptr = ptr.add(ALIGN(playback_ctrl_size as c_ulong, 0x100) as usize); ptr_addr += ALIGN(playback_ctrl_size as c_ulong, 0x100); for voice in 0..YDSXG_PLAYBACK_VOICES { (*chip).voices[voice as usize].number = voice; (*chip).voices[voice as usize].bank = ptr as *mut snd_ymfpci_playback_bank; (*chip).voices[voice as usize].bank_addr = ptr_addr; for bank in 0..2 { (*chip).bank_playback[voice as usize][bank] = ptr as *mut snd_ymfpci_playback_bank; ptr = ptr.add((*chip).bank_size_playback as usize); ptr_addr += (*chip).bank_size_playback as c_ulong; } } ptr = ALIGN(ptr as c_ulong, 0x100) as *mut u8; ptr_addr = ALIGN(ptr_addr, 0x100); (*chip).bank_base_capture = ptr; (*chip).bank_base_capture_addr = ptr_addr; for voice in 0..YDSXG_CAPTURE_VOICES { for bank in 0..2 { (*chip).bank_capture[voice as usize][bank] = ptr as *mut snd_ymfpci_capture_bank; ptr = ptr.add((*chip).bank_size_capture as usize); ptr_addr += (*chip).bank_size_capture as c_ulong; } } ptr = ALIGN(ptr as c_ulong, 0x100) as *mut u8; ptr_addr = ALIGN(ptr_addr, 0x100); (*chip).bank_base_effect = ptr; (*chip).bank_base_effect_addr = ptr_addr; for voice in 0..YDSXG_EFFECT_VOICES { for bank in 0..2 { (*chip).bank_effect[voice as usize][bank] = ptr as *mut snd_ymfpci_effect_bank; ptr = ptr.add((*chip).bank_size_effect as usize); ptr_addr += (*chip).bank_size_effect as c_ulong; } } ptr = ALIGN(ptr as c_ulong, 0x100) as *mut u8; ptr_addr = ALIGN(ptr_addr, 0x100); (*chip).work_base = ptr; (*chip).work_base_addr = ptr_addr; snd_BUG_ON(ptr.add(PAGE_ALIGN((*chip).work_size) as usize) != ((*(*chip).work_ptr).area as *mut u8).add((*(*chip).work_ptr).bytes as usize)); snd_ymfpci_writel(chip, YDSXGR_PLAYCTRLBASE, (*chip).bank_base_playback_addr as u32); snd_ymfpci_writel(chip, YDSXGR_RECCTRLBASE, (*chip).bank_base_capture_addr as u32); snd_ymfpci_writel(chip, YDSXGR_EFFCTRLBASE, (*chip).bank_base_effect_addr as u32); snd_ymfpci_writel(chip, YDSXGR_WORKBASE, (*chip).work_base_addr as u32); snd_ymfpci_writel(chip, YDSXGR_WORKSIZE, ((*chip).work_size >> 2) as u32); (*chip).spdif_bits = SNDRV_PCM_DEFAULT_CON_SPDIF & 0xffff; (*chip).spdif_pcm_bits = (*chip).spdif_bits; snd_ymfpci_writew(chip, YDSXGR_SPDIFOUTCTRL, 0); snd_ymfpci_writew(chip, YDSXGR_SPDIFOUTSTATUS, (*chip).spdif_bits as u16); snd_ymfpci_writew(chip, YDSXGR_SPDIFINCTRL, 0); let mut reg = 0x80; while reg < 0xc0 { snd_ymfpci_writel(chip, reg, 0); reg += 4; } snd_ymfpci_writel(chip, YDSXGR_NATIVEDACOUTVOL, 0x3fff3fff); snd_ymfpci_writel(chip, YDSXGR_BUF441OUTVOL, 0x3fff3fff); snd_ymfpci_writel(chip, YDSXGR_ZVOUTVOL, 0x3fff3fff); snd_ymfpci_writel(chip, YDSXGR_SPDIFOUTVOL, 0x3fff3fff); snd_ymfpci_writel(chip, YDSXGR_NATIVEADCINVOL, 0x3fff3fff); snd_ymfpci_writel(chip, YDSXGR_NATIVEDACINVOL, 0x3fff3fff); snd_ymfpci_writel(chip, YDSXGR_PRIADCLOOPVOL, 0x3fff3fff); snd_ymfpci_writel(chip, YDSXGR_LEGACYOUTVOL, 0x3fff3fff); 0 }
unsafe extern "C" fn snd_ymfpci_free(card: *mut snd_card) { let chip = (*card).private_data; snd_ymfpci_writel(chip, YDSXGR_NATIVEDACOUTVOL, 0); snd_ymfpci_writel(chip, YDSXGR_BUF441OUTVOL, 0); snd_ymfpci_writel(chip, YDSXGR_LEGACYOUTVOL, 0); snd_ymfpci_writel(chip, YDSXGR_STATUS, !0); snd_ymfpci_disable_dsp(chip); snd_ymfpci_writel(chip, YDSXGR_PLAYCTRLBASE, 0); snd_ymfpci_writel(chip, YDSXGR_RECCTRLBASE, 0); snd_ymfpci_writel(chip, YDSXGR_EFFCTRLBASE, 0); snd_ymfpci_writel(chip, YDSXGR_WORKBASE, 0); snd_ymfpci_writel(chip, YDSXGR_WORKSIZE, 0); let ctrl = snd_ymfpci_readw(chip, YDSXGR_GLOBALCTRL); snd_ymfpci_writew(chip, YDSXGR_GLOBALCTRL, ctrl & !0x0007); snd_ymfpci_ac3_done(chip); snd_ymfpci_free_gameport(chip); pci_write_config_word((*chip).pci, PCIR_DSXG_LEGACY, (*chip).old_legacy_ctrl); release_firmware((*chip).dsp_microcode); release_firmware((*chip).controller_microcode); }
unsafe extern "C" fn snd_ymfpci_suspend(dev: *mut device) -> c_int { let card = dev_get_drvdata(dev); let chip = (*card).private_data; let mut legacy_reg_count = DSXG_PCI_NUM_SAVED_LEGACY_REGS; if (*(*chip).pci).device >= 0x0010 { legacy_reg_count = DSXG_PCI_NUM_SAVED_REGS; } snd_power_change_state(card, SNDRV_CTL_POWER_D3hot); snd_ac97_suspend((*chip).ac97); for i in 0..YDSXGR_NUM_SAVED_REGS { (*chip).saved_regs[i as usize] = snd_ymfpci_readl(chip, saved_regs_index[i as usize]); } (*chip).saved_ydsxgr_mode = snd_ymfpci_readl(chip, YDSXGR_MODE); for i in 0..legacy_reg_count { pci_read_config_word((*chip).pci, pci_saved_regs_index[i as usize], (*chip).saved_dsxg_pci_regs.as_mut_ptr().add(i as usize)); } snd_ymfpci_writel(chip, YDSXGR_NATIVEDACOUTVOL, 0); snd_ymfpci_writel(chip, YDSXGR_BUF441OUTVOL, 0); snd_ymfpci_disable_dsp(chip); 0 }
unsafe extern "C" fn snd_ymfpci_resume(dev: *mut device) -> c_int { let pci = to_pci_dev(dev); let card = dev_get_drvdata(dev); let chip = (*card).private_data; let mut legacy_reg_count = DSXG_PCI_NUM_SAVED_LEGACY_REGS; if (*(*chip).pci).device >= 0x0010 { legacy_reg_count = DSXG_PCI_NUM_SAVED_REGS; } snd_ymfpci_aclink_reset(pci); snd_ymfpci_codec_ready(chip, 0); snd_ymfpci_download_image(chip); udelay(100); for i in 0..YDSXGR_NUM_SAVED_REGS { snd_ymfpci_writel(chip, saved_regs_index[i as usize], (*chip).saved_regs[i as usize]); } snd_ac97_resume((*chip).ac97); for i in 0..legacy_reg_count { pci_write_config_word((*chip).pci, pci_saved_regs_index[i as usize] as c_int, (*chip).saved_dsxg_pci_regs[i as usize]); } if (*chip).start_count > 0 { snd_ymfpci_writel(chip, YDSXGR_MODE, (*chip).saved_ydsxgr_mode); (*chip).active_bank = snd_ymfpci_readl(chip, YDSXGR_CTRLSELECT); } snd_power_change_state(card, SNDRV_CTL_POWER_D0); 0 }
/* DEFINE_SIMPLE_DEV_PM_OPS(snd_ymfpci_pm, snd_ymfpci_suspend, snd_ymfpci_resume); */
unsafe extern "C" fn snd_ymfpci_create(card: *mut snd_card, pci: *mut pci_dev, old_legacy_ctrl: u16) -> c_int { let chip = (*card).private_data; let mut err = pcim_enable_device(pci); if err < 0 { return err; } (*chip).old_legacy_ctrl = old_legacy_ctrl; spin_lock_init(&mut (*chip).reg_lock); spin_lock_init(&mut (*chip).voice_lock); init_waitqueue_head(&mut (*chip).interrupt_sleep); atomic_set(&mut (*chip).interrupt_sleep_count, 0); (*chip).card = card; (*chip).pci = pci; (*chip).irq = -1; (*chip).device_id = (*pci).device; (*chip).rev = (*pci).revision; err = pcim_request_all_regions(pci, c"YMFPCI".as_ptr()); if err < 0 { return err; } (*chip).reg_area_phys = pci_resource_start(pci, 0); (*chip).reg_area_virt = devm_ioremap(&mut (*pci).dev, (*chip).reg_area_phys, 0x8000); if (*chip).reg_area_virt.is_null() { dev_err((*card).dev, c"unable to grab memory region 0x%lx-0x%lx\n".as_ptr(), (*chip).reg_area_phys, (*chip).reg_area_phys + 0x8000 - 1); return -EBUSY; } pci_set_master(pci); (*chip).src441_used = -1; if devm_request_irq(&mut (*pci).dev, (*pci).irq, Some(snd_ymfpci_interrupt), IRQF_SHARED, KBUILD_MODNAME, chip) != 0 { dev_err((*card).dev, c"unable to grab IRQ %d\n".as_ptr(), (*pci).irq); return -EBUSY; } (*chip).irq = (*pci).irq; (*card).sync_irq = (*chip).irq; (*card).private_free = Some(snd_ymfpci_free); snd_ymfpci_aclink_reset(pci); if snd_ymfpci_codec_ready(chip, 0) < 0 { return -EIO; } err = snd_ymfpci_request_firmware(chip); if err < 0 { dev_err((*card).dev, c"firmware request failed: %d\n".as_ptr(), err); return err; } snd_ymfpci_download_image(chip); udelay(100); if snd_ymfpci_memalloc(chip) < 0 { return -EIO; } err = snd_ymfpci_ac3_init(chip); if err < 0 { return err; } snd_ymfpci_proc_init(card, chip); 0 }

/*
 * Externally supplied fields, constants, helper macros, arrays, and routines are
 * intentionally referenced above in Rust form. This file is a source-level
 * translation of ymfpci_main.c only; definitions owned by headers and sibling
 * repository files remain future dependencies.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
