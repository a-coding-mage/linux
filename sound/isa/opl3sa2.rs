// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for Yamaha OPL3-SA[2,3] soundcards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

/* C include dependencies removed: linux/init.h, linux/err.h, linux/isa.h,
 * linux/interrupt.h, linux/pm.h, linux/pnp.h, linux/module.h, linux/io.h,
 * sound/core.h, sound/wss.h, sound/mpu401.h, sound/opl3.h, sound/initval.h,
 * sound/tlv.h.
 */
/* MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>"); */
/* MODULE_DESCRIPTION("Yamaha OPL3SA2+"); */
/* MODULE_LICENSE("GPL"); */

type bool_t = bool;
type irqreturn_t = c_int;
type pm_message_t = c_int;
type spinlock_t = c_int;

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_hwdep {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_rawmidi {
    pub private_data: *mut c_void,
}
#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_id {
    pub numid: c_uint,
    pub iface: c_int,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: [c_char; 44],
    pub index: c_uint,
}
#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_ctl_elem_id,
    pub private_value: c_ulong,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_kcontrol)>,
}
#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub struct snd_kcontrol_new_tlv {
    pub p: *const c_uint,
}
#[repr(C)]
pub union snd_kcontrol_new_tlv_union {
    pub p: *const c_uint,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub access: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut c_void) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
    pub tlv: snd_kcontrol_new_tlv_union,
}
#[repr(C)]
pub struct snd_wss {
    pub suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
}
#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub dev: *mut device,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
    pub sync_irq: c_int,
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
pub struct pnp_card_devs {
    pub id: [c_char; 8],
}
#[repr(C)]
pub struct pnp_card_device_id {
    pub id: [c_char; 8],
    pub devs: [pnp_card_devs; 1],
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
pub struct pnp_driver {
    pub name: *const c_char,
    pub id_table: *const pnp_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pnp_dev, *const pnp_device_id) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut pnp_dev, pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut pnp_dev) -> c_int>,
}
#[repr(C)]
pub struct pnp_card_driver {
    pub flags: c_int,
    pub name: *const c_char,
    pub id_table: *const pnp_card_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pnp_card_link, *const pnp_card_device_id) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut pnp_card_link, pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut pnp_card_link) -> c_int>,
}
#[repr(C)]
pub struct driver_inner {
    pub name: *const c_char,
}
#[repr(C)]
pub struct isa_driver {
    pub match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    pub driver: driver_inner,
}

extern "C" {
    static mut THIS_MODULE: *mut module;
    static mut snd_wss_info_single: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut c_void) -> c_int>;
    static mut snd_wss_info_double: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut c_void) -> c_int>;
    fn outb(value: u8, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn devm_request_region(dev: *mut device, start: c_ulong, n: c_ulong, name: *const c_char) -> *mut resource;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snd_opl3_interrupt(hw: *mut snd_hwdep);
    fn snd_mpu401_uart_interrupt(irq: c_int, private_data: *mut c_void);
    fn snd_wss_interrupt(irq: c_int, wss: *mut snd_wss);
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_ctl_rename_id(card: *mut snd_card, src_id: *mut snd_ctl_elem_id, dst_id: *mut snd_ctl_elem_id) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn pnp_activate_dev(dev: *mut pnp_dev) -> c_int;
    fn pnp_port_start(dev: *mut pnp_dev, bar: c_uint) -> c_ulong;
    fn pnp_dma(dev: *mut pnp_dev, n: c_uint) -> c_int;
    fn pnp_irq(dev: *mut pnp_dev, n: c_uint) -> c_int;
    fn pnp_device_is_pnpbios(dev: *mut pnp_dev) -> c_int;
    fn pnp_device_is_isapnp(dev: *mut pnp_dev) -> c_int;
    fn snd_devm_card_new(parent: *mut device, idx: c_int, xid: *mut c_char, module: *mut module, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn snd_wss_create(card: *mut snd_card, port: c_ulong, cport: c_long, irq: c_int, dma1: c_int, dma2: c_int, hardware: c_int, hwshare: c_int, rchip: *mut *mut snd_wss) -> c_int;
    fn snd_wss_pcm(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_wss_mixer(chip: *mut snd_wss) -> c_int;
    fn snd_wss_timer(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_opl3_create(card: *mut snd_card, l_port: c_ulong, r_port: c_ulong, hardware: c_int, integrated: c_int, ropl3: *mut *mut snd_opl3) -> c_int;
    fn snd_opl3_timer_new(opl3: *mut snd_opl3, timer1_dev: c_int, timer2_dev: c_int) -> c_int;
    fn snd_opl3_hwdep_new(opl3: *mut snd_opl3, device: c_int, seq_device: c_int, rhwdep: *mut *mut snd_hwdep) -> c_int;
    fn snd_mpu401_uart_new(card: *mut snd_card, device: c_int, hardware: c_int, port: c_ulong, info_flags: c_uint, irq: c_int, rrawmidi: *mut *mut snd_rawmidi) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pnp_set_drvdata(dev: *mut pnp_dev, data: *mut c_void);
    fn pnp_get_drvdata(dev: *mut pnp_dev) -> *mut c_void;
    fn pnp_request_card_device(card: *mut pnp_card_link, id: *const c_char, from: *mut pnp_dev) -> *mut pnp_dev;
    fn pnp_set_card_drvdata(card: *mut pnp_card_link, data: *mut c_void);
    fn pnp_get_card_drvdata(card: *mut pnp_card_link) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn isa_register_driver(driver: *mut isa_driver, ndev: c_uint) -> c_int;
    fn isa_unregister_driver(driver: *mut isa_driver);
    fn pnp_register_driver(driver: *mut pnp_driver) -> c_int;
    fn pnp_unregister_driver(driver: *mut pnp_driver);
    fn pnp_register_card_driver(driver: *mut pnp_card_driver) -> c_int;
    fn pnp_unregister_card_driver(driver: *mut pnp_card_driver);
}

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: c_int = -1;
const SNDRV_DEFAULT_PORT: c_long = -1;
const SNDRV_DEFAULT_IRQ: c_int = -1;
const SNDRV_DEFAULT_DMA: c_int = -1;
const SNDRV_AUTO_PORT: c_long = -1;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const IRQ_NONE: irqreturn_t = 0;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x3;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0x40000;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const WSS_HW_OPL3SA2: c_int = 0;
const WSS_HWSHARE_IRQ: c_int = 1;
const OPL3_HW_OPL3: c_int = 0;
const MPU401_HW_OPL3SA2: c_int = 0;
const MPU401_INFO_IRQ_HOOK: c_uint = 1;
const PNP_DRIVER_RES_DISABLE: c_int = 1;

static mut index: [c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_IDX; SNDRV_CARDS]; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS]; /* ID for this card */
static mut enable: [bool_t; SNDRV_CARDS] = [true; SNDRV_CARDS]; /* Enable this card */
/* #ifdef CONFIG_PNP */
static mut isapnp: [bool_t; SNDRV_CARDS] = [true; SNDRV_CARDS];
/* #endif */
static mut port: [c_long; SNDRV_CARDS] = [SNDRV_DEFAULT_PORT; SNDRV_CARDS]; /* 0xf86,0x370,0x100 */
static mut sb_port: [c_long; SNDRV_CARDS] = [SNDRV_DEFAULT_PORT; SNDRV_CARDS]; /* 0x220,0x240,0x260 */
static mut wss_port: [c_long; SNDRV_CARDS] = [SNDRV_DEFAULT_PORT; SNDRV_CARDS]; /* 0x530,0xe80,0xf40,0x604 */
static mut fm_port: [c_long; SNDRV_CARDS] = [SNDRV_DEFAULT_PORT; SNDRV_CARDS]; /* 0x388 */
static mut midi_port: [c_long; SNDRV_CARDS] = [SNDRV_DEFAULT_PORT; SNDRV_CARDS]; /* 0x330,0x300 */
static mut irq: [c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_IRQ; SNDRV_CARDS]; /* 0,1,3,5,9,11,12,15 */
static mut dma1: [c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_DMA; SNDRV_CARDS]; /* 1,3,5,6,7 */
static mut dma2: [c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_DMA; SNDRV_CARDS]; /* 1,3,5,6,7 */
static mut opl3sa3_ymode: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS]; /* 0,1,2,3 */ /*SL Added*/

/* module_param_array/module_param_hw_array declarations and descriptions preserved by comments. */

/* #ifdef CONFIG_PNP */
static mut isa_registered: c_int = 0;
static mut pnp_registered: c_int = 0;
static mut pnpc_registered: c_int = 0;
/* #endif */

/* control ports */
const OPL3SA2_PM_CTRL: u8 = 0x01;
const OPL3SA2_SYS_CTRL: u8 = 0x02;
const OPL3SA2_IRQ_CONFIG: u8 = 0x03;
const OPL3SA2_IRQ_STATUS: u8 = 0x04;
const OPL3SA2_DMA_CONFIG: u8 = 0x06;
const OPL3SA2_MASTER_LEFT: u8 = 0x07;
const OPL3SA2_MASTER_RIGHT: u8 = 0x08;
const OPL3SA2_MIC: u8 = 0x09;
const OPL3SA2_MISC: u8 = 0x0A;

/* opl3sa3 only */
const OPL3SA3_DGTL_DOWN: u8 = 0x12;
const OPL3SA3_ANLG_DOWN: u8 = 0x13;
const OPL3SA3_WIDE: u8 = 0x14;
const OPL3SA3_BASS: u8 = 0x15;
const OPL3SA3_TREBLE: u8 = 0x16;

/* power management bits */
const OPL3SA2_PM_ADOWN: u8 = 0x20;
const OPL3SA2_PM_PSV: u8 = 0x04;
const OPL3SA2_PM_PDN: u8 = 0x02;
const OPL3SA2_PM_PDX: u8 = 0x01;

const OPL3SA2_PM_D0: u8 = 0x00;
const OPL3SA2_PM_D3: u8 = OPL3SA2_PM_ADOWN | OPL3SA2_PM_PSV | OPL3SA2_PM_PDN | OPL3SA2_PM_PDX;

#[repr(C)]
pub struct snd_opl3sa2 {
    pub version: c_int,             /* 2 or 3 */
    pub port: c_ulong,              /* control port */
    pub res_port: *mut resource,    /* control port resource */
    pub irq: c_int,
    pub single_dma: c_int,
    pub reg_lock: spinlock_t,
    pub card: *mut snd_card,
    pub synth: *mut snd_hwdep,
    pub rmidi: *mut snd_rawmidi,
    pub wss: *mut snd_wss,
    pub ctlregs: [u8; 0x20],
    pub ymode: c_int,               /* SL added */
    pub master_switch: *mut snd_kcontrol,
    pub master_volume: *mut snd_kcontrol,
}

const PFX: &[u8] = b"opl3sa2: \0";

/* #ifdef CONFIG_PNP */
static snd_opl3sa2_pnpbiosids: [pnp_device_id; 3] = [
    pnp_device_id { id: *b"YMH0021\0" },
    pnp_device_id { id: *b"NMX2210\0" }, /* Gateway Solo 2500 */
    pnp_device_id { id: *b"\0\0\0\0\0\0\0\0" }, /* end */
];

static snd_opl3sa2_pnpids: [pnp_card_device_id; 7] = [
    /* Yamaha YMF719E-S (Genius Sound Maker 3DX) */
    pnp_card_device_id { id: *b"YMH0020\0", devs: [pnp_card_devs { id: *b"YMH0021\0" }] },
    /* Yamaha OPL3-SA3 (integrated on Intel's Pentium II AL440LX motherboard) */
    pnp_card_device_id { id: *b"YMH0030\0", devs: [pnp_card_devs { id: *b"YMH0021\0" }] },
    /* Yamaha OPL3-SA2 */
    pnp_card_device_id { id: *b"YMH0800\0", devs: [pnp_card_devs { id: *b"YMH0021\0" }] },
    /* Yamaha OPL3-SA2 */
    pnp_card_device_id { id: *b"YMH0801\0", devs: [pnp_card_devs { id: *b"YMH0021\0" }] },
    /* NeoMagic MagicWave 3DX */
    pnp_card_device_id { id: *b"NMX2200\0", devs: [pnp_card_devs { id: *b"YMH2210\0" }] },
    /* NeoMagic MagicWave 3D */
    pnp_card_device_id { id: *b"NMX2200\0", devs: [pnp_card_devs { id: *b"NMX2210\0" }] },
    pnp_card_device_id { id: *b"\0\0\0\0\0\0\0\0", devs: [pnp_card_devs { id: *b"\0\0\0\0\0\0\0\0" }] }, /* end */
];
/* MODULE_DEVICE_TABLE entries preserved by comments. */
/* #endif */

unsafe fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_opl3sa2 {
    (*kcontrol).private_data as *mut snd_opl3sa2
}

unsafe fn IRQ_RETVAL(handled: c_int) -> irqreturn_t {
    if handled != 0 { 1 } else { IRQ_NONE }
}

/* read control port (w/o spinlock) */
unsafe fn __snd_opl3sa2_read(chip: *mut snd_opl3sa2, reg: u8) -> u8 {
    let result: u8;
    outb(reg, (*chip).port); /* register */
    result = inb((*chip).port + 1);
    result
}

/* read control port (with spinlock) */
unsafe fn snd_opl3sa2_read(chip: *mut snd_opl3sa2, reg: u8) -> u8 {
    /* guard(spinlock_irqsave)(&chip->reg_lock); */
    __snd_opl3sa2_read(chip, reg)
}

/* write control port (w/o spinlock) */
unsafe fn __snd_opl3sa2_write(chip: *mut snd_opl3sa2, reg: u8, value: u8) {
    outb(reg, (*chip).port); /* register */
    outb(value, (*chip).port + 1);
    (*chip).ctlregs[reg as usize] = value;
}

/* write control port (with spinlock) */
unsafe fn snd_opl3sa2_write(chip: *mut snd_opl3sa2, reg: u8, value: u8) {
    /* guard(spinlock_irqsave)(&chip->reg_lock); */
    __snd_opl3sa2_write(chip, reg, value);
}

unsafe fn snd_opl3sa2_detect(card: *mut snd_card) -> c_int {
    let chip = (*card).private_data as *mut snd_opl3sa2;
    let port_: c_ulong;
    let mut tmp: u8;
    let mut tmp1: u8;
    let mut str_: [c_char; 2] = [0; 2];

    port_ = (*chip).port;
    (*chip).res_port = devm_request_region((*card).dev, port_, 2, b"OPL3-SA control\0".as_ptr() as *const c_char);
    if (*chip).res_port.is_null() {
        dev_err((*card).dev, b"can't grab port 0x%lx\n\0".as_ptr() as *const c_char, port_);
        return -EBUSY;
    }
    (*chip).version = 0;
    tmp = snd_opl3sa2_read(chip, OPL3SA2_MISC);
    if tmp == 0xff {
        dev_dbg((*card).dev, b"OPL3-SA [0x%lx] detect = 0x%x\n\0".as_ptr() as *const c_char, port_, tmp as c_int);
        return -ENODEV;
    }
    match tmp & 0x07 {
        0x01 => {
            (*chip).version = 2; /* YMF711 */
        }
        _ => {
            (*chip).version = 3;
            /* 0x02 - standard */
            /* 0x03 - YM715B */
            /* 0x04 - YM719 - OPL-SA4? */
            /* 0x05 - OPL3-SA3 - Libretto 100 */
            /* 0x07 - unknown - Neomagic MagicWave 3D */
        }
    }
    str_[0] = ((*chip).version + b'0' as c_int) as c_char;
    str_[1] = 0;
    strcat((*card).shortname.as_mut_ptr(), str_.as_ptr());
    snd_opl3sa2_write(chip, OPL3SA2_MISC, tmp ^ 7);
    tmp1 = snd_opl3sa2_read(chip, OPL3SA2_MISC);
    if tmp1 != tmp {
        dev_dbg((*card).dev, b"OPL3-SA [0x%lx] detect (1) = 0x%x (0x%x)\n\0".as_ptr() as *const c_char, port_, tmp as c_int, tmp1 as c_int);
        return -ENODEV;
    }
    /* try if the MIC register is accessible */
    tmp = snd_opl3sa2_read(chip, OPL3SA2_MIC);
    snd_opl3sa2_write(chip, OPL3SA2_MIC, 0x8a);
    tmp1 = snd_opl3sa2_read(chip, OPL3SA2_MIC);
    if (tmp1 & 0x9f) != 0x8a {
        dev_dbg((*card).dev, b"OPL3-SA [0x%lx] detect (2) = 0x%x (0x%x)\n\0".as_ptr() as *const c_char, port_, tmp as c_int, tmp1 as c_int);
        return -ENODEV;
    }
    snd_opl3sa2_write(chip, OPL3SA2_MIC, 0x9f);
    /* initialization */
    /* Power Management - full on */
    snd_opl3sa2_write(chip, OPL3SA2_PM_CTRL, OPL3SA2_PM_D0);
    if (*chip).version > 2 {
        /* ymode is bits 4&5 (of 0 to 7) on all but opl3sa2 versions */
        snd_opl3sa2_write(chip, OPL3SA2_SYS_CTRL, ((*chip).ymode << 4) as u8);
    } else {
        /* default for opl3sa2 versions */
        snd_opl3sa2_write(chip, OPL3SA2_SYS_CTRL, 0x00);
    }
    snd_opl3sa2_write(chip, OPL3SA2_IRQ_CONFIG, 0x0d); /* Interrupt Channel Configuration - IRQ A = OPL3 + MPU + WSS */
    if (*chip).single_dma != 0 {
        snd_opl3sa2_write(chip, OPL3SA2_DMA_CONFIG, 0x03); /* DMA Configuration - DMA A = WSS-R + WSS-P */
    } else {
        snd_opl3sa2_write(chip, OPL3SA2_DMA_CONFIG, 0x21); /* DMA Configuration - DMA B = WSS-R, DMA A = WSS-P */
    }
    snd_opl3sa2_write(chip, OPL3SA2_MISC, 0x80 | (tmp & 7)); /* Miscellaneous - default */
    if (*chip).version > 2 {
        snd_opl3sa2_write(chip, OPL3SA3_DGTL_DOWN, 0x00); /* Digital Block Partial Power Down - default */
        snd_opl3sa2_write(chip, OPL3SA3_ANLG_DOWN, 0x00); /* Analog Block Partial Power Down - default */
    }
    0
}

unsafe extern "C" fn snd_opl3sa2_interrupt(irq_: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let status: u16;
    let card = dev_id as *mut snd_card;
    let chip: *mut snd_opl3sa2;
    let mut handled: c_int = 0;

    if card.is_null() {
        return IRQ_NONE;
    }

    chip = (*card).private_data as *mut snd_opl3sa2;
    status = snd_opl3sa2_read(chip, OPL3SA2_IRQ_STATUS) as u16;

    if (status & 0x20) != 0 {
        handled = 1;
        snd_opl3_interrupt((*chip).synth);
    }

    if (status & 0x10) != 0 && !(*chip).rmidi.is_null() {
        handled = 1;
        snd_mpu401_uart_interrupt(irq_, (*(*chip).rmidi).private_data);
    }

    if (status & 0x07) != 0 { /* TI,CI,PI */
        handled = 1;
        snd_wss_interrupt(irq_, (*chip).wss);
    }

    if (status & 0x40) != 0 { /* hardware volume change */
        handled = 1;
        /* reading from Master Lch register at 0x07 clears this bit */
        snd_opl3sa2_read(chip, OPL3SA2_MASTER_RIGHT);
        snd_opl3sa2_read(chip, OPL3SA2_MASTER_LEFT);
        if !(*chip).master_switch.is_null() && !(*chip).master_volume.is_null() {
            snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).master_switch).id);
            snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).master_volume).id);
        }
    }
    IRQ_RETVAL(handled)
}

const fn OPL3SA2_SINGLE_VALUE(reg: c_ulong, shift: c_ulong, mask: c_ulong, invert: c_ulong) -> c_ulong {
    reg | (shift << 8) | (mask << 16) | (invert << 24)
}
const fn OPL3SA2_DOUBLE_VALUE(left_reg: c_ulong, right_reg: c_ulong, shift_left: c_ulong, shift_right: c_ulong, mask: c_ulong, invert: c_ulong) -> c_ulong {
    left_reg | (right_reg << 8) | (shift_left << 16) | (shift_right << 19) | (mask << 24) | (invert << 22)
}

unsafe extern "C" fn snd_opl3sa2_get_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as c_int;
    let shift = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let mask = (((*kcontrol).private_value >> 16) & 0xff) as c_int;
    let invert = (((*kcontrol).private_value >> 24) & 0xff) as c_int;

    /* guard(spinlock_irqsave)(&chip->reg_lock); */
    (*ucontrol).value.integer.value[0] = (((*chip).ctlregs[reg as usize] as c_int >> shift) & mask) as c_long;
    if invert != 0 {
        (*ucontrol).value.integer.value[0] = (mask as c_long) - (*ucontrol).value.integer.value[0];
    }
    0
}

unsafe extern "C" fn snd_opl3sa2_put_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as c_int;
    let shift = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let mask = (((*kcontrol).private_value >> 16) & 0xff) as c_int;
    let invert = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let change: c_int;
    let mut val: u16;
    let oval: u16;

    val = ((*ucontrol).value.integer.value[0] & mask as c_long) as u16;
    if invert != 0 {
        val = (mask as u16).wrapping_sub(val);
    }
    val <<= shift;
    /* guard(spinlock_irqsave)(&chip->reg_lock); */
    oval = (*chip).ctlregs[reg as usize] as u16;
    val = (oval & !((mask as u16) << shift)) | val;
    change = (val != oval) as c_int;
    __snd_opl3sa2_write(chip, reg as u8, val as u8);
    change
}

unsafe extern "C" fn snd_opl3sa2_get_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as c_int;
    let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
    let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let invert = (((*kcontrol).private_value >> 22) & 1) as c_int;

    /* guard(spinlock_irqsave)(&chip->reg_lock); */
    (*ucontrol).value.integer.value[0] = (((*chip).ctlregs[left_reg as usize] as c_int >> shift_left) & mask) as c_long;
    (*ucontrol).value.integer.value[1] = (((*chip).ctlregs[right_reg as usize] as c_int >> shift_right) & mask) as c_long;
    if invert != 0 {
        (*ucontrol).value.integer.value[0] = (mask as c_long) - (*ucontrol).value.integer.value[0];
        (*ucontrol).value.integer.value[1] = (mask as c_long) - (*ucontrol).value.integer.value[1];
    }
    0
}

unsafe extern "C" fn snd_opl3sa2_put_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as c_int;
    let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
    let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let invert = (((*kcontrol).private_value >> 22) & 1) as c_int;
    let change: c_int;
    let mut val1: u16;
    let mut val2: u16;
    let oval1: u16;
    let oval2: u16;

    val1 = ((*ucontrol).value.integer.value[0] & mask as c_long) as u16;
    val2 = ((*ucontrol).value.integer.value[1] & mask as c_long) as u16;
    if invert != 0 {
        val1 = (mask as u16).wrapping_sub(val1);
        val2 = (mask as u16).wrapping_sub(val2);
    }
    val1 <<= shift_left;
    val2 <<= shift_right;
    /* guard(spinlock_irqsave)(&chip->reg_lock); */
    if left_reg != right_reg {
        oval1 = (*chip).ctlregs[left_reg as usize] as u16;
        oval2 = (*chip).ctlregs[right_reg as usize] as u16;
        val1 = (oval1 & !((mask as u16) << shift_left)) | val1;
        val2 = (oval2 & !((mask as u16) << shift_right)) | val2;
        change = (val1 != oval1 || val2 != oval2) as c_int;
        __snd_opl3sa2_write(chip, left_reg as u8, val1 as u8);
        __snd_opl3sa2_write(chip, right_reg as u8, val2 as u8);
    } else {
        oval1 = (*chip).ctlregs[left_reg as usize] as u16;
        val1 = (oval1 & !(((mask as u16) << shift_left) | ((mask as u16) << shift_right))) | val1 | val2;
        change = (val1 != oval1) as c_int;
        __snd_opl3sa2_write(chip, left_reg as u8, val1 as u8);
    }
    change
}

static db_scale_master: [c_uint; 4] = [0, (-3000i32) as c_uint, 200, 0];
static db_scale_5bit_12db_max: [c_uint; 4] = [0, (-3450i32) as c_uint, 150, 0];

macro_rules! kcontrol_new {
    ($name:expr, $index:expr, $info:expr, $get:expr, $put:expr, $private_value:expr, $access:expr, $tlv:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            access: $access,
            name: $name.as_ptr() as *const c_char,
            index: $index,
            info: $info,
            get: $get,
            put: $put,
            private_value: $private_value,
            tlv: snd_kcontrol_new_tlv_union { p: $tlv },
        }
    };
}

static snd_opl3sa2_controls: [snd_kcontrol_new; 5] = [
    kcontrol_new!(b"Master Playback Switch\0", 0, unsafe { snd_wss_info_double }, Some(snd_opl3sa2_get_double), Some(snd_opl3sa2_put_double), OPL3SA2_DOUBLE_VALUE(0x07, 0x08, 7, 7, 1, 1), 0, ptr::null()),
    kcontrol_new!(b"Master Playback Volume\0", 0, unsafe { snd_wss_info_double }, Some(snd_opl3sa2_get_double), Some(snd_opl3sa2_put_double), OPL3SA2_DOUBLE_VALUE(0x07, 0x08, 0, 0, 15, 1), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_master.as_ptr()),
    kcontrol_new!(b"Mic Playback Switch\0", 0, unsafe { snd_wss_info_single }, Some(snd_opl3sa2_get_single), Some(snd_opl3sa2_put_single), OPL3SA2_SINGLE_VALUE(0x09, 7, 1, 1), 0, ptr::null()),
    kcontrol_new!(b"Mic Playback Volume\0", 0, unsafe { snd_wss_info_single }, Some(snd_opl3sa2_get_single), Some(snd_opl3sa2_put_single), OPL3SA2_SINGLE_VALUE(0x09, 0, 31, 1), SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, db_scale_5bit_12db_max.as_ptr()),
    kcontrol_new!(b"ZV Port Switch\0", 0, unsafe { snd_wss_info_single }, Some(snd_opl3sa2_get_single), Some(snd_opl3sa2_put_single), OPL3SA2_SINGLE_VALUE(0x02, 0, 1, 0), 0, ptr::null()),
];

static snd_opl3sa2_tone_controls: [snd_kcontrol_new; 3] = [
    kcontrol_new!(b"3D Control - Wide\0", 0, unsafe { snd_wss_info_double }, Some(snd_opl3sa2_get_double), Some(snd_opl3sa2_put_double), OPL3SA2_DOUBLE_VALUE(0x14, 0x14, 4, 0, 7, 0), 0, ptr::null()),
    kcontrol_new!(b"Tone Control - Bass\0", 0, unsafe { snd_wss_info_double }, Some(snd_opl3sa2_get_double), Some(snd_opl3sa2_put_double), OPL3SA2_DOUBLE_VALUE(0x15, 0x15, 4, 0, 7, 0), 0, ptr::null()),
    kcontrol_new!(b"Tone Control - Treble\0", 0, unsafe { snd_wss_info_double }, Some(snd_opl3sa2_get_double), Some(snd_opl3sa2_put_double), OPL3SA2_DOUBLE_VALUE(0x16, 0x16, 4, 0, 7, 0), 0, ptr::null()),
];

unsafe extern "C" fn snd_opl3sa2_master_free(kcontrol: *mut snd_kcontrol) {
    let chip = snd_kcontrol_chip(kcontrol);
    (*chip).master_switch = ptr::null_mut();
    (*chip).master_volume = ptr::null_mut();
}

unsafe fn snd_opl3sa2_mixer(card: *mut snd_card) -> c_int {
    let chip = (*card).private_data as *mut snd_opl3sa2;
    let mut id1: snd_ctl_elem_id = core::mem::zeroed();
    let mut id2: snd_ctl_elem_id = core::mem::zeroed();
    let mut kctl: *mut snd_kcontrol;
    let mut idx: c_uint;
    let mut err: c_int;

    memset(&mut id1 as *mut _ as *mut c_void, 0, size_of::<snd_ctl_elem_id>());
    memset(&mut id2 as *mut _ as *mut c_void, 0, size_of::<snd_ctl_elem_id>());
    id1.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    id2.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    /* reassign AUX0 to CD */
    strscpy(id1.name.as_mut_ptr(), b"Aux Playback Switch\0".as_ptr() as *const c_char);
    strscpy(id2.name.as_mut_ptr(), b"CD Playback Switch\0".as_ptr() as *const c_char);
    err = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if err < 0 {
        dev_err((*card).dev, b"Cannot rename opl3sa2 control\n\0".as_ptr() as *const c_char);
        return err;
    }
    strscpy(id1.name.as_mut_ptr(), b"Aux Playback Volume\0".as_ptr() as *const c_char);
    strscpy(id2.name.as_mut_ptr(), b"CD Playback Volume\0".as_ptr() as *const c_char);
    err = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if err < 0 {
        dev_err((*card).dev, b"Cannot rename opl3sa2 control\n\0".as_ptr() as *const c_char);
        return err;
    }
    /* reassign AUX1 to FM */
    strscpy(id1.name.as_mut_ptr(), b"Aux Playback Switch\0".as_ptr() as *const c_char);
    id1.index = 1;
    strscpy(id2.name.as_mut_ptr(), b"FM Playback Switch\0".as_ptr() as *const c_char);
    err = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if err < 0 {
        dev_err((*card).dev, b"Cannot rename opl3sa2 control\n\0".as_ptr() as *const c_char);
        return err;
    }
    strscpy(id1.name.as_mut_ptr(), b"Aux Playback Volume\0".as_ptr() as *const c_char);
    strscpy(id2.name.as_mut_ptr(), b"FM Playback Volume\0".as_ptr() as *const c_char);
    err = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if err < 0 {
        dev_err((*card).dev, b"Cannot rename opl3sa2 control\n\0".as_ptr() as *const c_char);
        return err;
    }
    /* add OPL3SA2 controls */
    idx = 0;
    while (idx as usize) < snd_opl3sa2_controls.len() {
        kctl = snd_ctl_new1(&snd_opl3sa2_controls[idx as usize], chip as *mut c_void);
        err = snd_ctl_add(card, kctl);
        if err < 0 {
            return err;
        }
        match idx {
            0 => {
                (*chip).master_switch = kctl;
                (*kctl).private_free = Some(snd_opl3sa2_master_free);
            }
            1 => {
                (*chip).master_volume = kctl;
                (*kctl).private_free = Some(snd_opl3sa2_master_free);
            }
            _ => {}
        }
        idx += 1;
    }
    if (*chip).version > 2 {
        idx = 0;
        while (idx as usize) < snd_opl3sa2_tone_controls.len() {
            err = snd_ctl_add(card, snd_ctl_new1(&snd_opl3sa2_tone_controls[idx as usize], chip as *mut c_void));
            if err < 0 {
                return err;
            }
            idx += 1;
        }
    }
    0
}

/* Power Management support functions */
/* #ifdef CONFIG_PM */
unsafe fn snd_opl3sa2_suspend(card: *mut snd_card, _state: pm_message_t) -> c_int {
    if !card.is_null() {
        let chip = (*card).private_data as *mut snd_opl3sa2;

        snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
        if let Some(suspend) = (*(*chip).wss).suspend {
            suspend((*chip).wss);
        }
        /* power down */
        snd_opl3sa2_write(chip, OPL3SA2_PM_CTRL, OPL3SA2_PM_D3);
    }

    0
}

unsafe fn snd_opl3sa2_resume(card: *mut snd_card) -> c_int {
    let chip: *mut snd_opl3sa2;
    let mut i: c_int;

    if card.is_null() {
        return 0;
    }

    chip = (*card).private_data as *mut snd_opl3sa2;
    /* power up */
    snd_opl3sa2_write(chip, OPL3SA2_PM_CTRL, OPL3SA2_PM_D0);

    /* restore registers */
    i = 2;
    while i <= 0x0a {
        if i != OPL3SA2_IRQ_STATUS as c_int {
            snd_opl3sa2_write(chip, i as u8, (*chip).ctlregs[i as usize]);
        }
        i += 1;
    }
    if (*chip).version > 2 {
        i = 0x12;
        while i <= 0x16 {
            snd_opl3sa2_write(chip, i as u8, (*chip).ctlregs[i as usize]);
            i += 1;
        }
    }
    /* restore wss */
    if let Some(resume) = (*(*chip).wss).resume {
        resume((*chip).wss);
    }

    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}
/* #endif */

/* #ifdef CONFIG_PNP */
unsafe fn snd_opl3sa2_pnp(dev: c_int, chip: *mut snd_opl3sa2, pdev: *mut pnp_dev) -> c_int {
    if pnp_activate_dev(pdev) < 0 {
        dev_err((*(*chip).card).dev, b"PnP configure failure (out of resources?)\n\0".as_ptr() as *const c_char);
        return -EBUSY;
    }
    sb_port[dev as usize] = pnp_port_start(pdev, 0) as c_long;
    wss_port[dev as usize] = pnp_port_start(pdev, 1) as c_long;
    fm_port[dev as usize] = pnp_port_start(pdev, 2) as c_long;
    midi_port[dev as usize] = pnp_port_start(pdev, 3) as c_long;
    port[dev as usize] = pnp_port_start(pdev, 4) as c_long;
    dma1[dev as usize] = pnp_dma(pdev, 0);
    dma2[dev as usize] = pnp_dma(pdev, 1);
    irq[dev as usize] = pnp_irq(pdev, 0);
    dev_dbg((*(*chip).card).dev, b"%sPnP OPL3-SA: sb port=0x%lx, wss port=0x%lx, fm port=0x%lx, midi port=0x%lx\n\0".as_ptr() as *const c_char,
        if pnp_device_is_pnpbios(pdev) != 0 { b"BIOS\0".as_ptr() } else { b"ISA\0".as_ptr() } as *const c_char,
        sb_port[dev as usize], wss_port[dev as usize], fm_port[dev as usize], midi_port[dev as usize]);
    dev_dbg((*(*chip).card).dev, b"%sPnP OPL3-SA: control port=0x%lx, dma1=%i, dma2=%i, irq=%i\n\0".as_ptr() as *const c_char,
        if pnp_device_is_pnpbios(pdev) != 0 { b"BIOS\0".as_ptr() } else { b"ISA\0".as_ptr() } as *const c_char,
        port[dev as usize], dma1[dev as usize], dma2[dev as usize], irq[dev as usize]);
    0
}
/* #endif */

unsafe fn snd_opl3sa2_card_new(pdev: *mut device, dev: c_int, cardp: *mut *mut snd_card) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut snd_opl3sa2;
    let err: c_int;

    err = snd_devm_card_new(pdev, index[dev as usize], id[dev as usize], THIS_MODULE, size_of::<snd_opl3sa2>(), &mut card);
    if err < 0 {
        return err;
    }
    strscpy((*card).driver.as_mut_ptr(), b"OPL3SA2\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"Yamaha OPL3-SA\0".as_ptr() as *const c_char);
    chip = (*card).private_data as *mut snd_opl3sa2;
    spin_lock_init(&mut (*chip).reg_lock);
    (*chip).irq = -1;
    *cardp = card;
    0
}

unsafe fn snd_opl3sa2_probe(card: *mut snd_card, dev: c_int) -> c_int {
    let xirq: c_int;
    let xdma1: c_int;
    let xdma2: c_int;
    let chip: *mut snd_opl3sa2;
    let mut wss: *mut snd_wss = ptr::null_mut();
    let mut opl3: *mut snd_opl3 = ptr::null_mut();
    let mut err: c_int;

    /* initialise this card from supplied (or default) parameter*/
    chip = (*card).private_data as *mut snd_opl3sa2;
    (*chip).card = card;
    (*chip).ymode = opl3sa3_ymode[dev as usize] & 0x03;
    (*chip).port = port[dev as usize] as c_ulong;
    xirq = irq[dev as usize];
    xdma1 = dma1[dev as usize];
    xdma2 = dma2[dev as usize];
    if xdma2 < 0 {
        (*chip).single_dma = 1;
    }
    err = snd_opl3sa2_detect(card);
    if err < 0 {
        return err;
    }
    err = devm_request_irq((*card).dev, xirq, snd_opl3sa2_interrupt, 0, b"OPL3-SA2\0".as_ptr() as *const c_char, card as *mut c_void);
    if err != 0 {
        dev_err((*card).dev, b"can't grab IRQ %d\n\0".as_ptr() as *const c_char, xirq);
        return -ENODEV;
    }
    (*chip).irq = xirq;
    (*card).sync_irq = (*chip).irq;
    err = snd_wss_create(card, (wss_port[dev as usize] + 4) as c_ulong, -1, xirq, xdma1, xdma2, WSS_HW_OPL3SA2, WSS_HWSHARE_IRQ, &mut wss);
    if err < 0 {
        dev_dbg((*card).dev, b"Oops, WSS not detected at 0x%lx\n\0".as_ptr() as *const c_char, wss_port[dev as usize] + 4);
        return err;
    }
    (*chip).wss = wss;
    err = snd_wss_pcm(wss, 0);
    if err < 0 {
        return err;
    }
    err = snd_wss_mixer(wss);
    if err < 0 {
        return err;
    }
    err = snd_opl3sa2_mixer(card);
    if err < 0 {
        return err;
    }
    err = snd_wss_timer(wss, 0);
    if err < 0 {
        return err;
    }
    if fm_port[dev as usize] >= 0x340 && fm_port[dev as usize] < 0x400 {
        err = snd_opl3_create(card, fm_port[dev as usize] as c_ulong, (fm_port[dev as usize] + 2) as c_ulong, OPL3_HW_OPL3, 0, &mut opl3);
        if err < 0 {
            return err;
        }
        err = snd_opl3_timer_new(opl3, 1, 2);
        if err < 0 {
            return err;
        }
        err = snd_opl3_hwdep_new(opl3, 0, 1, &mut (*chip).synth);
        if err < 0 {
            return err;
        }
    }
    if midi_port[dev as usize] >= 0x300 && midi_port[dev as usize] < 0x340 {
        err = snd_mpu401_uart_new(card, 0, MPU401_HW_OPL3SA2, midi_port[dev as usize] as c_ulong, MPU401_INFO_IRQ_HOOK, -1, &mut (*chip).rmidi);
        if err < 0 {
            return err;
        }
    }
    sprintf((*card).longname.as_mut_ptr(), b"%s at 0x%lx, irq %d, dma %d\0".as_ptr() as *const c_char, (*card).shortname.as_ptr(), (*chip).port, xirq, xdma1);
    if xdma2 >= 0 {
        let len = strlen((*card).longname.as_ptr());
        sprintf((*card).longname.as_mut_ptr().add(len), b"&%d\0".as_ptr() as *const c_char, xdma2);
    }

    snd_card_register(card)
}

/* #ifdef CONFIG_PNP */
unsafe extern "C" fn snd_opl3sa2_pnp_detect(pdev: *mut pnp_dev, _id: *const pnp_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut err: c_int;
    let mut card: *mut snd_card = ptr::null_mut();

    if pnp_device_is_isapnp(pdev) != 0 {
        return -ENOENT; /* we have another procedure - card */
    }
    while dev < SNDRV_CARDS as c_int {
        if enable[dev as usize] && isapnp[dev as usize] {
            break;
        }
        dev += 1;
    }
    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }

    err = snd_opl3sa2_card_new(&mut (*pdev).dev, dev, &mut card);
    if err < 0 {
        return err;
    }
    err = snd_opl3sa2_pnp(dev, (*card).private_data as *mut snd_opl3sa2, pdev);
    if err < 0 {
        return err;
    }
    err = snd_opl3sa2_probe(card, dev);
    if err < 0 {
        return err;
    }
    pnp_set_drvdata(pdev, card as *mut c_void);
    dev += 1;
    0
}

/* #ifdef CONFIG_PM */
unsafe extern "C" fn snd_opl3sa2_pnp_suspend(pdev: *mut pnp_dev, state: pm_message_t) -> c_int {
    snd_opl3sa2_suspend(pnp_get_drvdata(pdev) as *mut snd_card, state)
}
unsafe extern "C" fn snd_opl3sa2_pnp_resume(pdev: *mut pnp_dev) -> c_int {
    snd_opl3sa2_resume(pnp_get_drvdata(pdev) as *mut snd_card)
}
/* #endif */

static mut opl3sa2_pnp_driver: pnp_driver = pnp_driver {
    name: b"snd-opl3sa2-pnpbios\0".as_ptr() as *const c_char,
    id_table: snd_opl3sa2_pnpbiosids.as_ptr(),
    probe: Some(snd_opl3sa2_pnp_detect),
    suspend: Some(snd_opl3sa2_pnp_suspend),
    resume: Some(snd_opl3sa2_pnp_resume),
};

unsafe extern "C" fn snd_opl3sa2_pnp_cdetect(pcard: *mut pnp_card_link, id_: *const pnp_card_device_id) -> c_int {
    static mut dev: c_int = 0;
    let pdev: *mut pnp_dev;
    let mut err: c_int;
    let mut card: *mut snd_card = ptr::null_mut();

    pdev = pnp_request_card_device(pcard, (*id_).devs[0].id.as_ptr(), ptr::null_mut());
    if pdev.is_null() {
        dev_err(&mut (*(*pcard).card).dev, b"can't get pnp device from id '%s'\n\0".as_ptr() as *const c_char, (*id_).devs[0].id.as_ptr());
        return -EBUSY;
    }
    while dev < SNDRV_CARDS as c_int {
        if enable[dev as usize] && isapnp[dev as usize] {
            break;
        }
        dev += 1;
    }
    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }

    err = snd_opl3sa2_card_new(&mut (*pdev).dev, dev, &mut card);
    if err < 0 {
        return err;
    }
    err = snd_opl3sa2_pnp(dev, (*card).private_data as *mut snd_opl3sa2, pdev);
    if err < 0 {
        return err;
    }
    err = snd_opl3sa2_probe(card, dev);
    if err < 0 {
        return err;
    }
    pnp_set_card_drvdata(pcard, card as *mut c_void);
    dev += 1;
    0
}

/* #ifdef CONFIG_PM */
unsafe extern "C" fn snd_opl3sa2_pnp_csuspend(pcard: *mut pnp_card_link, state: pm_message_t) -> c_int {
    snd_opl3sa2_suspend(pnp_get_card_drvdata(pcard) as *mut snd_card, state)
}
unsafe extern "C" fn snd_opl3sa2_pnp_cresume(pcard: *mut pnp_card_link) -> c_int {
    snd_opl3sa2_resume(pnp_get_card_drvdata(pcard) as *mut snd_card)
}
/* #endif */

static mut opl3sa2_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DISABLE,
    name: b"snd-opl3sa2-cpnp\0".as_ptr() as *const c_char,
    id_table: snd_opl3sa2_pnpids.as_ptr(),
    probe: Some(snd_opl3sa2_pnp_cdetect),
    suspend: Some(snd_opl3sa2_pnp_csuspend),
    resume: Some(snd_opl3sa2_pnp_cresume),
};
/* #endif */

unsafe extern "C" fn snd_opl3sa2_isa_match(pdev: *mut device, dev: c_uint) -> c_int {
    if !enable[dev as usize] {
        return 0;
    }
    /* #ifdef CONFIG_PNP */
    if isapnp[dev as usize] {
        return 0;
    }
    /* #endif */
    if port[dev as usize] == SNDRV_AUTO_PORT {
        dev_err(pdev, b"specify port\n\0".as_ptr() as *const c_char);
        return 0;
    }
    if wss_port[dev as usize] == SNDRV_AUTO_PORT {
        dev_err(pdev, b"specify wss_port\n\0".as_ptr() as *const c_char);
        return 0;
    }
    if fm_port[dev as usize] == SNDRV_AUTO_PORT {
        dev_err(pdev, b"specify fm_port\n\0".as_ptr() as *const c_char);
        return 0;
    }
    if midi_port[dev as usize] == SNDRV_AUTO_PORT {
        dev_err(pdev, b"specify midi_port\n\0".as_ptr() as *const c_char);
        return 0;
    }
    1
}

unsafe extern "C" fn snd_opl3sa2_isa_probe(pdev: *mut device, dev: c_uint) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut err: c_int;

    err = snd_opl3sa2_card_new(pdev, dev as c_int, &mut card);
    if err < 0 {
        return err;
    }
    err = snd_opl3sa2_probe(card, dev as c_int);
    if err < 0 {
        return err;
    }
    dev_set_drvdata(pdev, card as *mut c_void);
    0
}

/* #ifdef CONFIG_PM */
unsafe extern "C" fn snd_opl3sa2_isa_suspend(dev: *mut device, _n: c_uint, state: pm_message_t) -> c_int {
    snd_opl3sa2_suspend(dev_get_drvdata(dev) as *mut snd_card, state)
}

unsafe extern "C" fn snd_opl3sa2_isa_resume(dev: *mut device, _n: c_uint) -> c_int {
    snd_opl3sa2_resume(dev_get_drvdata(dev) as *mut snd_card)
}
/* #endif */

const DEV_NAME: &[u8] = b"opl3sa2\0";

static mut snd_opl3sa2_isa_driver: isa_driver = isa_driver {
    match_: Some(snd_opl3sa2_isa_match),
    probe: Some(snd_opl3sa2_isa_probe),
    suspend: Some(snd_opl3sa2_isa_suspend),
    resume: Some(snd_opl3sa2_isa_resume),
    driver: driver_inner {
        name: DEV_NAME.as_ptr() as *const c_char,
    },
};

unsafe fn alsa_card_opl3sa2_init() -> c_int {
    let mut err: c_int;

    err = isa_register_driver(&mut snd_opl3sa2_isa_driver, SNDRV_CARDS as c_uint);
    /* #ifdef CONFIG_PNP */
    if err == 0 {
        isa_registered = 1;
    }

    err = pnp_register_driver(&mut opl3sa2_pnp_driver);
    if err == 0 {
        pnp_registered = 1;
    }

    err = pnp_register_card_driver(&mut opl3sa2_pnpc_driver);
    if err == 0 {
        pnpc_registered = 1;
    }

    if isa_registered != 0 || pnp_registered != 0 {
        err = 0;
    }
    /* #endif */
    err
}

unsafe fn alsa_card_opl3sa2_exit() {
    /* #ifdef CONFIG_PNP */
    if pnpc_registered != 0 {
        pnp_unregister_card_driver(&mut opl3sa2_pnpc_driver);
    }
    if pnp_registered != 0 {
        pnp_unregister_driver(&mut opl3sa2_pnp_driver);
    }
    if isa_registered != 0 {
        isa_unregister_driver(&mut snd_opl3sa2_isa_driver);
    }
    /* #else
     * isa_unregister_driver(&snd_opl3sa2_isa_driver);
     * #endif
     */
}

/* module_init(alsa_card_opl3sa2_init) */
/* module_exit(alsa_card_opl3sa2_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
