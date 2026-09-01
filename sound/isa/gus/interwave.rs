// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for AMD InterWave soundcard
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *
 *   1999/07/22		Erik Inge Bolso <knan@mo.himolde.no>
 *			* mixer group handlers
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_uint, c_uchar, c_ushort, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_t = bool;
type irqreturn_t = c_int;
type pm_message_t = c_int;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_ISAPNP: [bool_t; SNDRV_CARDS] = [true; SNDRV_CARDS];
const SNDRV_DEFAULT_PORT: [c_long; SNDRV_CARDS] = [-1; SNDRV_CARDS];
const SNDRV_DEFAULT_IRQ: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];
const SNDRV_DEFAULT_DMA: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];
const SNDRV_AUTO_PORT: c_long = -1;
const SNDRV_AUTO_IRQ: c_int = -1;
const SNDRV_AUTO_DMA: c_int = -1;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const WSS_HW_INTERWAVE: c_int = 0;
const WSS_HWSHARE_IRQ: c_uint = 1 << 0;
const WSS_HWSHARE_DMA1: c_uint = 1 << 1;
const WSS_HWSHARE_DMA2: c_uint = 1 << 2;
const PNP_DRIVER_RES_DISABLE: c_uint = 1;

const SNDRV_GF1_GB_RESET: c_int = 0;
const SNDRV_GF1_GB_VERSION_NUMBER: c_int = 1;
const SNDRV_GF1_GB_GLOBAL_MODE: c_int = 2;
const SNDRV_GF1_GB_MEMORY_CONTROL: c_int = 3;
const SNDRV_GF1_GW_MEMORY_CONFIG: c_int = 4;
const SNDRV_GF1_GB_COMPATIBILITY: c_int = 5;
const SNDRV_GF1_GB_DECODE_CONTROL: c_int = 6;
const SNDRV_GF1_GB_MPU401_CONTROL_A: c_int = 7;
const SNDRV_GF1_GB_MPU401_CONTROL_B: c_int = 8;
const SNDRV_GF1_GB_EMULATION_IRQ: c_int = 9;
const SNDRV_GF1_GB_SOUND_BLASTER_CONTROL: c_int = 10;

const CS4231_LINE_LEFT_OUTPUT: c_int = 0;
const CS4231_LINE_RIGHT_OUTPUT: c_int = 1;
const CS4231_LEFT_MIC_INPUT: c_int = 2;
const CS4231_RIGHT_MIC_INPUT: c_int = 3;

const INTERWAVE_DRIVER: *const c_char = b"snd_interwave\0".as_ptr() as *const c_char;
const INTERWAVE_PNP_DRIVER: *const c_char = b"interwave\0".as_ptr() as *const c_char;
#[cfg(SNDRV_STB)]
const INTERWAVE_DRIVER_STB: *const c_char = b"snd_interwave_stb\0".as_ptr() as *const c_char;
#[cfg(SNDRV_STB)]
const INTERWAVE_PNP_DRIVER_STB: *const c_char = b"interwave-stb\0".as_ptr() as *const c_char;

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct resource {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_card {
    dev: *mut device,
    number: c_int,
    private_data: *mut c_void,
    sync_irq: c_int,
    driver: [c_char; 32],
    shortname: [c_char; 32],
    longname: [c_char; 80],
}

#[repr(C)]
struct snd_pcm {
    name: [c_char; 80],
}

#[repr(C)]
struct snd_gf1_bank {
    address: c_uint,
    size: c_uint,
}

#[repr(C)]
struct snd_gf1_mem_alloc {
    banks_8: [snd_gf1_bank; 4],
    banks_16: [snd_gf1_bank; 4],
}

#[repr(C)]
struct snd_gf1 {
    port: c_ulong,
    reg_irqstat: c_ushort,
    enh_mode: c_int,
    mem_alloc: snd_gf1_mem_alloc,
    memory: c_int,
    rom_banks: c_int,
    rom_memory: c_uint,
    rom_present: c_uint,
}

#[repr(C)]
struct snd_gus_card {
    card: *mut snd_card,
    gf1: snd_gf1,
    reg_lock: c_int,
    interwave: c_int,
    revision: c_uchar,
    equal_irq: c_int,
    codec_flag: c_int,
    max_flag: c_int,
    joystick_dac: c_int,
    uart_enable: c_int,
}

#[repr(C)]
struct snd_wss {
    card: *mut snd_card,
    pcm: *mut snd_pcm,
    suspend: Option<unsafe extern "C" fn(*mut snd_wss) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut snd_wss) -> c_int>,
}

#[repr(C)]
struct snd_ctl_elem_id {
    iface: c_int,
    index: c_uint,
    name: [c_char; 44],
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_i2c_bit_ops {
    setlines: Option<unsafe extern "C" fn(*mut snd_i2c_bus, c_int, c_int)>,
    getclock: Option<unsafe extern "C" fn(*mut snd_i2c_bus) -> c_int>,
    getdata: Option<unsafe extern "C" fn(*mut snd_i2c_bus, c_int) -> c_int>,
}

#[repr(C)]
union snd_i2c_hw_ops {
    bit: *mut snd_i2c_bit_ops,
}

#[repr(C)]
struct snd_i2c_bus {
    card: *mut snd_card,
    private_value: c_ulong,
    hw_ops: snd_i2c_hw_ops,
}

#[repr(C)]
struct pnp_dev {
    dev: device,
}

#[repr(C)]
struct pnp_card {
    dev: device,
}

#[repr(C)]
struct pnp_card_link {
    card: *mut pnp_card,
}

#[repr(C)]
struct pnp_id {
    id: [c_char; 8],
}

#[repr(C)]
struct pnp_card_device_id {
    id: [c_char; 8],
    devs: [pnp_id; 2],
}

#[repr(C)]
struct isa_driver_inner {
    name: *const c_char,
}

#[repr(C)]
struct isa_driver {
    match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    driver: isa_driver_inner,
}

#[repr(C)]
struct pnp_card_driver {
    flags: c_uint,
    name: *const c_char,
    id_table: *const pnp_card_device_id,
    probe: Option<unsafe extern "C" fn(*mut pnp_card_link, *const pnp_card_device_id) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut pnp_card_link, pm_message_t) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut pnp_card_link) -> c_int>,
}

#[repr(C)]
struct snd_interwave {
    irq: c_int,
    card: *mut snd_card,
    gus: *mut snd_gus_card,
    wss: *mut snd_wss,
    #[cfg(SNDRV_STB)]
    i2c_bus: *mut snd_i2c_bus,
    #[cfg(SNDRV_STB)]
    i2c_res: *mut resource,
    gus_status_reg: c_ushort,
    pcm_status_reg: c_ushort,
    #[cfg(CONFIG_PNP)]
    dev: *mut pnp_dev,
    #[cfg(all(CONFIG_PNP, SNDRV_STB))]
    devtc: *mut pnp_dev,
}

#[repr(C)]
struct rom_hdr {
    /* 000 */ iwave: [c_uchar; 8],
    /* 008 */ rom_hdr_revision: c_uchar,
    /* 009 */ series_number: c_uchar,
    /* 010 */ series_name: [c_uchar; 16],
    /* 026 */ date: [c_uchar; 10],
    /* 036 */ vendor_revision_major: c_ushort,
    /* 038 */ vendor_revision_minor: c_ushort,
    /* 040 */ rom_size: c_uint,
    /* 044 */ copyright: [c_uchar; 128],
    /* 172 */ vendor_name: [c_uchar; 64],
    /* 236 */ rom_description: [c_uchar; 128],
    /* 364 */ pad: [c_uchar; 147],
    /* 511 */ csum: c_uchar,
}

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool_t; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_ISAPNP; /* Enable this card */
#[cfg(CONFIG_PNP)]
static mut isapnp: [bool_t; SNDRV_CARDS] = [true; SNDRV_CARDS];
static mut port: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* 0x210,0x220,0x230,0x240,0x250,0x260 */
#[cfg(SNDRV_STB)]
static mut port_tc: [c_long; SNDRV_CARDS] = SNDRV_DEFAULT_PORT; /* 0x350,0x360,0x370,0x380 */
static mut irq: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IRQ; /* 2,3,5,9,11,12,15 */
static mut dma1: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA; /* 0,1,3,5,6,7 */
static mut dma2: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_DMA; /* 0,1,3,5,6,7 */
static mut joystick_dac: [c_int; SNDRV_CARDS] = [29; SNDRV_CARDS];
/* 0 to 31, (0.59V-4.52V or 0.389V-2.98V) */
static mut midi: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
static mut pcm_channels: [c_int; SNDRV_CARDS] = [2; SNDRV_CARDS];
static mut effect: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];

#[cfg(CONFIG_PNP)]
static mut isa_registered: c_int = 0;
#[cfg(CONFIG_PNP)]
static mut pnp_registered: c_int = 0;

#[cfg(CONFIG_PNP)]
static snd_interwave_pnpids: [pnp_card_device_id; 7] = [
    pnp_card_device_id { id: *b"GRV0001\0", devs: [pnp_id { id: *b"GRV0000\0" }, pnp_id { id: [0; 8] }] },
    pnp_card_device_id { id: *b"STB011a\0", devs: [pnp_id { id: *b"STB0010\0" }, pnp_id { id: [0; 8] }] },
    pnp_card_device_id { id: *b"DXP3201\0", devs: [pnp_id { id: *b"DXP0010\0" }, pnp_id { id: [0; 8] }] },
    pnp_card_device_id { id: *b"CDC1111\0", devs: [pnp_id { id: *b"CDC1112\0" }, pnp_id { id: [0; 8] }] },
    pnp_card_device_id { id: *b"ADV55ff\0", devs: [pnp_id { id: *b"ADV0010\0" }, pnp_id { id: [0; 8] }] },
    pnp_card_device_id { id: *b"ADV550a\0", devs: [pnp_id { id: *b"ADV0010\0" }, pnp_id { id: [0; 8] }] },
    pnp_card_device_id { id: [0; 8], devs: [pnp_id { id: [0; 8] }, pnp_id { id: [0; 8] }] },
];

static snd_interwave_memory_configs: [c_uint; 13] = [
    0x00000001, 0x00000101, 0x01010101, 0x00000401,
    0x04040401, 0x00040101, 0x04040101, 0x00000004,
    0x00000404, 0x04040404, 0x00000010, 0x00001010,
    0x10101010,
];

extern "C" {
    static THIS_MODULE: c_int;
    fn outb(value: c_uchar, port: c_ulong);
    fn inb(port: c_ulong) -> c_uchar;
    fn udelay(usecs: c_ulong);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn devm_request_region(dev: *mut device, start: c_ulong, n: c_ulong, name: *const c_char) -> *mut resource;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn snd_devm_card_new(dev: *mut device, idx: c_int, id: *mut c_char, module: *const c_int, extra_size: usize, card_ret: *mut *mut snd_card) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_gus_create(card: *mut snd_card, port: c_long, irq: c_int, dma1: c_int, dma2: c_int, timer_dev: c_int, voices: c_int, pcm_channels: c_int, effect: c_int, gusp: *mut *mut snd_gus_card) -> c_int;
    fn snd_gus_initialize(gus: *mut snd_gus_card) -> c_int;
    fn snd_gus_interrupt(irq: c_int, gus: *mut snd_gus_card);
    fn snd_gus_suspend(gus: *mut snd_gus_card) -> c_int;
    fn snd_gus_resume(gus: *mut snd_gus_card) -> c_int;
    fn snd_gf1_i_write8(gus: *mut snd_gus_card, reg: c_int, data: c_uchar);
    fn snd_gf1_i_look8(gus: *mut snd_gus_card, reg: c_int) -> c_uchar;
    fn snd_gf1_look8(gus: *mut snd_gus_card, reg: c_int) -> c_uchar;
    fn snd_gf1_write8(gus: *mut snd_gus_card, reg: c_int, data: c_uchar);
    fn snd_gf1_read8(gus: *mut snd_gus_card, reg: c_int) -> c_uchar;
    fn snd_gf1_look16(gus: *mut snd_gus_card, reg: c_int) -> c_ushort;
    fn snd_gf1_write16(gus: *mut snd_gus_card, reg: c_int, data: c_ushort);
    fn snd_gf1_poke(gus: *mut snd_gus_card, addr: c_uint, data: c_uchar);
    fn snd_gf1_peek(gus: *mut snd_gus_card, addr: c_uint) -> c_uchar;
    fn snd_gf1_pcm_new(gus: *mut snd_gus_card, pcm_dev: c_int, control_index: c_int) -> c_int;
    fn snd_gf1_rawmidi_new(gus: *mut snd_gus_card, device: c_int) -> c_int;
    fn snd_wss_interrupt(irq: c_int, chip: *mut snd_wss);
    fn snd_wss_create(card: *mut snd_card, port: c_ulong, cport: c_long, irq: c_int, dma1: c_int, dma2: c_int, hardware: c_int, hwshare: c_uint, rchip: *mut *mut snd_wss) -> c_int;
    fn snd_wss_pcm(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_wss_timer(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_wss_mixer(chip: *mut snd_wss) -> c_int;
    fn snd_wss_out(chip: *mut snd_wss, reg: c_int, val: c_uchar);
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut c_void;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut c_void) -> c_int;
    fn snd_ctl_remove_id(card: *mut snd_card, id: *mut snd_ctl_elem_id) -> c_int;
    fn snd_ctl_rename_id(card: *mut snd_card, src_id: *mut snd_ctl_elem_id, dst_id: *mut snd_ctl_elem_id) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncmp(s1: *const c_uchar, s2: *const c_char, n: usize) -> c_int;
    fn snd_legacy_find_free_irq(irqs: *const c_int) -> c_int;
    fn snd_legacy_find_free_dma(dmas: *const c_int) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn isa_register_driver(driver: *mut isa_driver, ndev: c_uint) -> c_int;
    fn isa_unregister_driver(driver: *mut isa_driver);
    fn pnp_request_card_device(card: *mut pnp_card_link, id: *const c_char, from: *mut pnp_dev) -> *mut pnp_dev;
    fn pnp_activate_dev(dev: *mut pnp_dev) -> c_int;
    fn pnp_port_start(dev: *mut pnp_dev, bar: c_uint) -> c_ulong;
    fn pnp_dma(dev: *mut pnp_dev, bar: c_uint) -> c_int;
    fn pnp_irq(dev: *mut pnp_dev, bar: c_uint) -> c_int;
    fn pnp_set_card_drvdata(card: *mut pnp_card_link, data: *mut c_void);
    fn pnp_get_card_drvdata(card: *mut pnp_card_link) -> *mut c_void;
    fn pnp_register_card_driver(driver: *mut pnp_card_driver) -> c_int;
    fn pnp_unregister_card_driver(driver: *mut pnp_card_driver);
    fn snd_i2c_bus_create(card: *mut snd_card, name: *const c_char, ops: *mut c_void, rbus: *mut *mut snd_i2c_bus) -> c_int;
    fn snd_tea6330t_detect(bus: *mut snd_i2c_bus, equalizer: c_int) -> c_int;
    fn snd_tea6330t_update_mixer(card: *mut snd_card, bus: *mut snd_i2c_bus, equalizer: c_int, fader: c_int) -> c_int;
    fn snd_tea6330t_restore_mixer(bus: *mut snd_i2c_bus) -> c_int;
}

unsafe fn IRQ_RETVAL(x: c_int) -> irqreturn_t {
    if x != 0 { IRQ_HANDLED } else { IRQ_NONE }
}

unsafe extern "C" fn snd_interwave_i2c_setlines(bus: *mut snd_i2c_bus, ctrl: c_int, data: c_int) {
    let port = (*bus).private_value;
    outb(((data << 1) | ctrl) as c_uchar, port);
    udelay(10);
}

unsafe extern "C" fn snd_interwave_i2c_getclockline(bus: *mut snd_i2c_bus) -> c_int {
    let port = (*bus).private_value;
    let res: c_uchar = inb(port) & 1;
    res as c_int
}

unsafe extern "C" fn snd_interwave_i2c_getdataline(bus: *mut snd_i2c_bus, ack: c_int) -> c_int {
    let port = (*bus).private_value;
    let res: c_uchar;

    if ack != 0 {
        udelay(10);
    }
    res = (inb(port) & 2) >> 1;
    res as c_int
}

static mut snd_interwave_i2c_bit_ops: snd_i2c_bit_ops = snd_i2c_bit_ops {
    setlines: Some(snd_interwave_i2c_setlines),
    getclock: Some(snd_interwave_i2c_getclockline),
    getdata: Some(snd_interwave_i2c_getdataline),
};

#[cfg(SNDRV_STB)]
unsafe fn snd_interwave_detect_stb(iwcard: *mut snd_interwave, gus: *mut snd_gus_card, dev: c_int, rbus: *mut *mut snd_i2c_bus) -> c_int {
    let mut portv: c_ulong;
    let mut bus: *mut snd_i2c_bus = ptr::null_mut();
    let card = (*iwcard).card;
    let mut name = [0 as c_char; 32];
    let mut err: c_int;

    *rbus = ptr::null_mut();
    portv = port_tc[dev as usize] as c_ulong;
    if portv as c_long == SNDRV_AUTO_PORT {
        portv = 0x350;
        if (*gus).gf1.port == 0x250 {
            portv = 0x360;
        }
        while portv <= 0x380 {
            (*iwcard).i2c_res = devm_request_region((*card).dev, portv, 1, b"InterWave (I2C bus)\0".as_ptr() as *const c_char);
            if !(*iwcard).i2c_res.is_null() {
                break;
            }
            portv += 0x10;
        }
    } else {
        (*iwcard).i2c_res = devm_request_region((*card).dev, portv, 1, b"InterWave (I2C bus)\0".as_ptr() as *const c_char);
    }
    if (*iwcard).i2c_res.is_null() {
        dev_err((*card).dev, b"interwave: can't grab i2c bus port\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    sprintf(name.as_mut_ptr(), b"InterWave-%i\0".as_ptr() as *const c_char, (*card).number);
    err = snd_i2c_bus_create(card, name.as_ptr(), ptr::null_mut(), &mut bus);
    if err < 0 {
        return err;
    }
    (*bus).private_value = portv;
    (*bus).hw_ops.bit = &mut snd_interwave_i2c_bit_ops;
    err = snd_tea6330t_detect(bus, 0);
    if err < 0 {
        return err;
    }
    *rbus = bus;
    0
}

unsafe fn snd_interwave_detect(iwcard: *mut snd_interwave, gus: *mut snd_gus_card, dev: c_int, #[cfg(SNDRV_STB)] rbus: *mut *mut snd_i2c_bus) -> c_int {
    let mut rev1: c_uchar;
    let rev2: c_uchar;
    let mut d: c_int;

    snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 0); /* reset GF1 */
    d = snd_gf1_i_look8(gus, SNDRV_GF1_GB_RESET) as c_int;
    if (d & 0x07) != 0 {
        dev_dbg((*(*gus).card).dev, b"[0x%lx] check 1 failed - 0x%x\n\0".as_ptr() as *const c_char, (*gus).gf1.port, d);
        return -ENODEV;
    }
    udelay(160);
    snd_gf1_i_write8(gus, SNDRV_GF1_GB_RESET, 1); /* release reset */
    udelay(160);
    d = snd_gf1_i_look8(gus, SNDRV_GF1_GB_RESET) as c_int;
    if (d & 0x07) != 1 {
        dev_dbg((*(*gus).card).dev, b"[0x%lx] check 2 failed - 0x%x\n\0".as_ptr() as *const c_char, (*gus).gf1.port, d);
        return -ENODEV;
    }
    rev1 = snd_gf1_look8(gus, SNDRV_GF1_GB_VERSION_NUMBER);
    snd_gf1_write8(gus, SNDRV_GF1_GB_VERSION_NUMBER, !rev1);
    rev2 = snd_gf1_look8(gus, SNDRV_GF1_GB_VERSION_NUMBER);
    snd_gf1_write8(gus, SNDRV_GF1_GB_VERSION_NUMBER, rev1);
    dev_dbg((*(*gus).card).dev, b"[0x%lx] InterWave check - rev1=0x%x, rev2=0x%x\n\0".as_ptr() as *const c_char, (*gus).gf1.port, rev1 as c_int, rev2 as c_int);
    if (rev1 & 0xf0) == (rev2 & 0xf0) && (rev1 & 0x0f) != (rev2 & 0x0f) {
        dev_dbg((*(*gus).card).dev, b"[0x%lx] InterWave check - passed\n\0".as_ptr() as *const c_char, (*gus).gf1.port);
        (*gus).interwave = 1;
        strscpy((*(*gus).card).shortname.as_mut_ptr(), b"AMD InterWave\0".as_ptr() as *const c_char);
        (*gus).revision = rev1 >> 4;
        #[cfg(not(SNDRV_STB))]
        {
            return 0; /* ok.. We have an InterWave board */
        }
        #[cfg(SNDRV_STB)]
        {
            return snd_interwave_detect_stb(iwcard, gus, dev, rbus);
        }
    }
    dev_dbg((*(*gus).card).dev, b"[0x%lx] InterWave check - failed\n\0".as_ptr() as *const c_char, (*gus).gf1.port);
    -ENODEV
}

unsafe extern "C" fn snd_interwave_interrupt(irqv: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let iwcard = dev_id as *mut snd_interwave;
    let mut loopv: c_int;
    let mut max: c_int = 5;
    let mut handled: c_int = 0;

    loop {
        loopv = 0;
        if inb((*iwcard).gus_status_reg as c_ulong) != 0 {
            handled = 1;
            snd_gus_interrupt(irqv, (*iwcard).gus);
            loopv += 1;
        }
        if (inb((*iwcard).pcm_status_reg as c_ulong) & 0x01) != 0 {
            handled = 1;
            snd_wss_interrupt(irqv, (*iwcard).wss);
            loopv += 1;
        }
        max -= 1;
        if !(loopv != 0 && max > 0) {
            break;
        }
    }
    IRQ_RETVAL(handled)
}

unsafe fn snd_interwave_reset(gus: *mut snd_gus_card) {
    snd_gf1_write8(gus, SNDRV_GF1_GB_RESET, 0x00);
    udelay(160);
    snd_gf1_write8(gus, SNDRV_GF1_GB_RESET, 0x01);
    udelay(160);
}

unsafe fn snd_interwave_bank_sizes(gus: *mut snd_gus_card, sizes: *mut c_int) {
    let mut idx: c_uint;
    let mut local: c_uint;
    let mut d: c_uchar;

    idx = 0;
    while idx < 4 {
        *sizes.add(idx as usize) = 0;
        d = 0x55;
        local = idx << 22;
        while local < (idx << 22) + 0x400000 {
            snd_gf1_poke(gus, local, d);
            snd_gf1_poke(gus, local + 1, d.wrapping_add(1));
            if snd_gf1_peek(gus, local) != d ||
                snd_gf1_peek(gus, local + 1) != d.wrapping_add(1) ||
                snd_gf1_peek(gus, idx << 22) != 0x55 {
                break;
            }
            *sizes.add(idx as usize) += 1;
            local += 0x40000;
            d = d.wrapping_add(1);
        }
        idx += 1;
    }
}

unsafe fn snd_interwave_find_memory_config(lmct: c_uint) -> c_int {
    let mut i: c_uint = 0;
    while (i as usize) < snd_interwave_memory_configs.len() {
        if lmct == snd_interwave_memory_configs[i as usize] {
            return i as c_int;
        }
        i += 1;
    }
    -EINVAL
}

unsafe fn snd_interwave_detect_memory(gus: *mut snd_gus_card) {
    let mut bank_pos: c_int;
    let mut pages: c_int;
    let mut i: c_uint;
    let mut lmct: c_uint;
    let mut lmc_cfg: c_int;
    let mut psizes = [0 as c_int; 4];
    let mut iwave = [0 as c_uchar; 8];
    let mut csum: c_uchar;

    snd_interwave_reset(gus);
    snd_gf1_write8(gus, SNDRV_GF1_GB_GLOBAL_MODE, snd_gf1_read8(gus, SNDRV_GF1_GB_GLOBAL_MODE) | 0x01); /* enhanced mode */
    snd_gf1_write8(gus, SNDRV_GF1_GB_MEMORY_CONTROL, 0x01); /* DRAM I/O cycles selected */
    snd_gf1_write16(gus, SNDRV_GF1_GW_MEMORY_CONFIG, ((snd_gf1_look16(gus, SNDRV_GF1_GW_MEMORY_CONFIG) & 0xff10) | 0x004c) as c_ushort);
    /* ok.. simple test of memory size */
    pages = 0;
    snd_gf1_poke(gus, 0, 0x55);
    snd_gf1_poke(gus, 1, 0xaa);
    if snd_gf1_peek(gus, 0) == 0x55 && snd_gf1_peek(gus, 1) == 0xaa {
        snd_interwave_bank_sizes(gus, psizes.as_mut_ptr());
        lmct = ((psizes[3] as c_uint) << 24) | ((psizes[2] as c_uint) << 16) |
            ((psizes[1] as c_uint) << 8) | psizes[0] as c_uint;
        lmc_cfg = snd_interwave_find_memory_config(lmct);
        if lmc_cfg >= 0 {
            snd_gf1_write16(gus, SNDRV_GF1_GW_MEMORY_CONFIG,
                ((snd_gf1_look16(gus, SNDRV_GF1_GW_MEMORY_CONFIG) & 0xfff0) | lmc_cfg as c_ushort) as c_ushort);
            snd_interwave_bank_sizes(gus, psizes.as_mut_ptr());
        } else if (*gus).gf1.enh_mode == 0 {
            snd_gf1_write16(gus, SNDRV_GF1_GW_MEMORY_CONFIG,
                ((snd_gf1_look16(gus, SNDRV_GF1_GW_MEMORY_CONFIG) & 0xfff0) | 2) as c_ushort);
        }
        i = 0;
        while i < 4 {
            (*gus).gf1.mem_alloc.banks_16[i as usize].address = i << 22;
            (*gus).gf1.mem_alloc.banks_8[i as usize].address = (*gus).gf1.mem_alloc.banks_16[i as usize].address;
            (*gus).gf1.mem_alloc.banks_16[i as usize].size = (psizes[i as usize] as c_uint) << 18;
            (*gus).gf1.mem_alloc.banks_8[i as usize].size = (*gus).gf1.mem_alloc.banks_16[i as usize].size;
            pages += psizes[i as usize];
            i += 1;
        }
    }
    pages <<= 18;
    (*gus).gf1.memory = pages;

    snd_gf1_write8(gus, SNDRV_GF1_GB_MEMORY_CONTROL, 0x03); /* select ROM */
    snd_gf1_write16(gus, SNDRV_GF1_GW_MEMORY_CONFIG, ((snd_gf1_look16(gus, SNDRV_GF1_GW_MEMORY_CONFIG) & 0xff1f) | (4 << 5)) as c_ushort);
    (*gus).gf1.rom_banks = 0;
    (*gus).gf1.rom_memory = 0;
    bank_pos = 0;
    while bank_pos < 16 * 1024 * 1024 {
        i = 0;
        while i < 8 {
            iwave[i as usize] = snd_gf1_peek(gus, bank_pos as c_uint + i);
            i += 1;
        }
        if strncmp(iwave.as_ptr(), b"INTRWAVE\0".as_ptr() as *const c_char, 8) == 0 {
            csum = 0;
            i = 0;
            while (i as usize) < size_of::<rom_hdr>() {
                csum = csum.wrapping_add(snd_gf1_peek(gus, bank_pos as c_uint + i));
                i += 1;
            }
            if csum == 0 {
                (*gus).gf1.rom_banks += 1;
                (*gus).gf1.rom_present |= 1 << (bank_pos >> 22);
                (*gus).gf1.rom_memory = snd_gf1_peek(gus, bank_pos as c_uint + 40) as c_uint |
                    ((snd_gf1_peek(gus, bank_pos as c_uint + 41) as c_uint) << 8) |
                    ((snd_gf1_peek(gus, bank_pos as c_uint + 42) as c_uint) << 16) |
                    ((snd_gf1_peek(gus, bank_pos as c_uint + 43) as c_uint) << 24);
            }
        }
        bank_pos += 4 * 1024 * 1024;
    }
    snd_gf1_write8(gus, SNDRV_GF1_GB_MEMORY_CONTROL, 0x00); /* select RAM */

    if (*gus).gf1.enh_mode == 0 {
        snd_interwave_reset(gus);
    }
}

unsafe fn __snd_interwave_restore_regs(gus: *mut snd_gus_card) {
    snd_gf1_write8(gus, SNDRV_GF1_GB_COMPATIBILITY, 0x1f);
    snd_gf1_write8(gus, SNDRV_GF1_GB_DECODE_CONTROL, 0x49);
    snd_gf1_write8(gus, SNDRV_GF1_GB_VERSION_NUMBER, 0x11);
    snd_gf1_write8(gus, SNDRV_GF1_GB_MPU401_CONTROL_A, 0x00);
    snd_gf1_write8(gus, SNDRV_GF1_GB_MPU401_CONTROL_B, 0x30);
    snd_gf1_write8(gus, SNDRV_GF1_GB_EMULATION_IRQ, 0x00);
}

unsafe fn snd_interwave_init(dev: c_int, gus: *mut snd_gus_card) {
    /* Probe-time setup also clears the timer control register. */
    snd_gf1_write8(gus, SNDRV_GF1_GB_SOUND_BLASTER_CONTROL, 0x00);
    __snd_interwave_restore_regs(gus);
    (*gus).equal_irq = 1;
    (*gus).codec_flag = 1;
    (*gus).interwave = 1;
    (*gus).max_flag = 1;
    (*gus).joystick_dac = joystick_dac[dev as usize];
}

/* WSS_DOUBLE-generated controls supplied by ALSA headers in the original C file. */
static snd_interwave_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe fn snd_interwave_mixer(chip: *mut snd_wss) -> c_int {
    let card = (*chip).card;
    let mut id1: snd_ctl_elem_id = zeroed();
    let mut id2: snd_ctl_elem_id = zeroed();
    let mut idx: c_uint;
    let mut err: c_int;

    id1.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    id2.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    /* add new master and mic controls */
    idx = 0;
    while (idx as usize) < snd_interwave_controls.len() {
        err = snd_ctl_add(card, snd_ctl_new1(&snd_interwave_controls[idx as usize], chip as *mut c_void));
        if err < 0 {
            return err;
        }
        idx += 1;
    }
    snd_wss_out(chip, CS4231_LINE_LEFT_OUTPUT, 0x9f);
    snd_wss_out(chip, CS4231_LINE_RIGHT_OUTPUT, 0x9f);
    snd_wss_out(chip, CS4231_LEFT_MIC_INPUT, 0x9f);
    snd_wss_out(chip, CS4231_RIGHT_MIC_INPUT, 0x9f);
    /* reassign AUXA to SYNTHESIZER */
    strscpy(id1.name.as_mut_ptr(), b"Aux Playback Switch\0".as_ptr() as *const c_char);
    strscpy(id2.name.as_mut_ptr(), b"Synth Playback Switch\0".as_ptr() as *const c_char);
    err = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if err < 0 {
        return err;
    }
    strscpy(id1.name.as_mut_ptr(), b"Aux Playback Volume\0".as_ptr() as *const c_char);
    strscpy(id2.name.as_mut_ptr(), b"Synth Playback Volume\0".as_ptr() as *const c_char);
    err = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if err < 0 {
        return err;
    }
    /* reassign AUXB to CD */
    strscpy(id1.name.as_mut_ptr(), b"Aux Playback Switch\0".as_ptr() as *const c_char);
    id1.index = 1;
    strscpy(id2.name.as_mut_ptr(), b"CD Playback Switch\0".as_ptr() as *const c_char);
    err = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if err < 0 {
        return err;
    }
    strscpy(id1.name.as_mut_ptr(), b"Aux Playback Volume\0".as_ptr() as *const c_char);
    strscpy(id2.name.as_mut_ptr(), b"CD Playback Volume\0".as_ptr() as *const c_char);
    err = snd_ctl_rename_id(card, &mut id1, &mut id2);
    if err < 0 {
        return err;
    }
    0
}

#[cfg(CONFIG_PNP)]
unsafe fn snd_interwave_pnp(dev: c_int, iwcard: *mut snd_interwave, card: *mut pnp_card_link, idp: *const pnp_card_device_id) -> c_int {
    let mut pdev: *mut pnp_dev;
    let mut err: c_int;

    (*iwcard).dev = pnp_request_card_device(card, (*idp).devs[0].id.as_ptr(), ptr::null_mut());
    if (*iwcard).dev.is_null() {
        return -EBUSY;
    }
    #[cfg(SNDRV_STB)]
    {
        (*iwcard).devtc = pnp_request_card_device(card, (*idp).devs[1].id.as_ptr(), ptr::null_mut());
        if (*iwcard).devtc.is_null() {
            return -EBUSY;
        }
    }
    /* Synth & Codec initialization */
    pdev = (*iwcard).dev;

    err = pnp_activate_dev(pdev);
    if err < 0 {
        dev_err(&mut (*pdev).dev, b"InterWave PnP configure failure (out of resources?)\n\0".as_ptr() as *const c_char);
        return err;
    }
    if pnp_port_start(pdev, 0) + 0x100 != pnp_port_start(pdev, 1) ||
        pnp_port_start(pdev, 0) + 0x10c != pnp_port_start(pdev, 2) {
        dev_err(&mut (*pdev).dev, b"PnP configure failure (wrong ports)\n\0".as_ptr() as *const c_char);
        return -ENOENT;
    }
    port[dev as usize] = pnp_port_start(pdev, 0) as c_long;
    dma1[dev as usize] = pnp_dma(pdev, 0);
    if dma2[dev as usize] >= 0 {
        dma2[dev as usize] = pnp_dma(pdev, 1);
    }
    irq[dev as usize] = pnp_irq(pdev, 0);
    #[cfg(SNDRV_STB)]
    {
        /* Tone Control initialization */
        pdev = (*iwcard).devtc;
        err = pnp_activate_dev(pdev);
        if err < 0 {
            dev_err(&mut (*pdev).dev, b"InterWave ToneControl PnP configure failure (out of resources?)\n\0".as_ptr() as *const c_char);
            return err;
        }
        port_tc[dev as usize] = pnp_port_start(pdev, 0) as c_long;
        dev_dbg(&mut (*pdev).dev, b"isapnp IW: tone control port=0x%lx\n\0".as_ptr() as *const c_char, port_tc[dev as usize]);
    }
    0
}

unsafe fn snd_interwave_card_new(pdev: *mut device, dev: c_int, cardp: *mut *mut snd_card) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let iwcard: *mut snd_interwave;
    let err: c_int;

    err = snd_devm_card_new(pdev, index[dev as usize], id[dev as usize], &THIS_MODULE,
        size_of::<snd_interwave>(), &mut card);
    if err < 0 {
        return err;
    }
    iwcard = (*card).private_data as *mut snd_interwave;
    (*iwcard).card = card;
    (*iwcard).irq = -1;
    *cardp = card;
    0
}

unsafe fn snd_interwave_probe_gus(card: *mut snd_card, dev: c_int, gusp: *mut *mut snd_gus_card) -> c_int {
    snd_gus_create(card, port[dev as usize], -irq[dev as usize], dma1[dev as usize], dma2[dev as usize],
        0, 32, pcm_channels[dev as usize], effect[dev as usize], gusp)
}

unsafe fn snd_interwave_probe(card: *mut snd_card, dev: c_int, gus: *mut snd_gus_card) -> c_int {
    let xirq: c_int = irq[dev as usize];
    let xdma1: c_int = dma1[dev as usize];
    let xdma2: c_int = dma2[dev as usize];
    let iwcard = (*card).private_data as *mut snd_interwave;
    let mut wss: *mut snd_wss = ptr::null_mut();
    #[cfg(SNDRV_STB)]
    let mut i2c_bus: *mut snd_i2c_bus = ptr::null_mut();
    let mut strp: *const c_char;
    let mut err: c_int;

    err = snd_interwave_detect(iwcard, gus, dev, #[cfg(SNDRV_STB)] &mut i2c_bus);
    if err < 0 {
        return err;
    }

    (*iwcard).gus_status_reg = (*gus).gf1.reg_irqstat;
    (*iwcard).pcm_status_reg = ((*gus).gf1.port + 0x10c + 2) as c_ushort;

    snd_interwave_init(dev, gus);
    snd_interwave_detect_memory(gus);
    err = snd_gus_initialize(gus);
    if err < 0 {
        return err;
    }

    if devm_request_irq((*card).dev, xirq, snd_interwave_interrupt, 0, b"InterWave\0".as_ptr() as *const c_char, iwcard as *mut c_void) != 0 {
        dev_err((*card).dev, b"unable to grab IRQ %d\n\0".as_ptr() as *const c_char, xirq);
        return -EBUSY;
    }
    (*iwcard).irq = xirq;
    (*card).sync_irq = (*iwcard).irq;

    err = snd_wss_create(card,
        (*gus).gf1.port + 0x10c, -1, xirq,
        if xdma2 < 0 { xdma1 } else { xdma2 }, xdma1,
        WSS_HW_INTERWAVE,
        WSS_HWSHARE_IRQ | WSS_HWSHARE_DMA1 | WSS_HWSHARE_DMA2,
        &mut wss);
    if err < 0 {
        return err;
    }

    err = snd_wss_pcm(wss, 0);
    if err < 0 {
        return err;
    }

    sprintf((*wss).pcm.as_mut().unwrap().name.as_mut_ptr().add(strlen((*(*wss).pcm).name.as_ptr())), b" rev %c\0".as_ptr() as *const c_char, (*gus).revision as c_int + 'A' as c_int);
    strcat((*(*wss).pcm).name.as_mut_ptr(), b" (codec)\0".as_ptr() as *const c_char);

    err = snd_wss_timer(wss, 2);
    if err < 0 {
        return err;
    }

    err = snd_wss_mixer(wss);
    if err < 0 {
        return err;
    }

    if pcm_channels[dev as usize] > 0 {
        err = snd_gf1_pcm_new(gus, 1, 1);
        if err < 0 {
            return err;
        }
    }
    err = snd_interwave_mixer(wss);
    if err < 0 {
        return err;
    }

    #[cfg(SNDRV_STB)]
    {
        let mut id1: snd_ctl_elem_id = zeroed();
        let mut id2: snd_ctl_elem_id = zeroed();
        id1.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
        id2.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
        strscpy(id1.name.as_mut_ptr(), b"Master Playback Switch\0".as_ptr() as *const c_char);
        strscpy(id2.name.as_mut_ptr(), id1.name.as_ptr());
        id2.index = 1;
        err = snd_ctl_rename_id(card, &mut id1, &mut id2);
        if err < 0 {
            return err;
        }
        strscpy(id1.name.as_mut_ptr(), b"Master Playback Volume\0".as_ptr() as *const c_char);
        strscpy(id2.name.as_mut_ptr(), id1.name.as_ptr());
        err = snd_ctl_rename_id(card, &mut id1, &mut id2);
        if err < 0 {
            return err;
        }
        err = snd_tea6330t_update_mixer(card, i2c_bus, 0, 1);
        if err < 0 {
            return err;
        }
        (*iwcard).i2c_bus = i2c_bus;
    }

    (*gus).uart_enable = midi[dev as usize];
    err = snd_gf1_rawmidi_new(gus, 0);
    if err < 0 {
        return err;
    }

    strp = b"AMD InterWave\0".as_ptr() as *const c_char;
    if (*gus).gf1.rom_banks == 1 && (*gus).gf1.rom_present == 8 {
        strp = b"Dynasonic 3-D\0".as_ptr() as *const c_char;
    }
    #[cfg(SNDRV_STB)]
    {
        strp = b"InterWave STB\0".as_ptr() as *const c_char;
    }
    strscpy((*card).driver.as_mut_ptr(), strp);
    strscpy((*card).shortname.as_mut_ptr(), strp);
    sprintf((*card).longname.as_mut_ptr(), b"%s at 0x%lx, irq %i, dma %d\0".as_ptr() as *const c_char,
        strp, (*gus).gf1.port, xirq, xdma1);
    if xdma2 >= 0 {
        sprintf((*card).longname.as_mut_ptr().add(strlen((*card).longname.as_ptr())), b"&%d\0".as_ptr() as *const c_char, xdma2);
    }

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    (*iwcard).wss = wss;
    (*iwcard).gus = gus;
    0
}

unsafe extern "C" fn snd_interwave_isa_match(_pdev: *mut device, dev: c_uint) -> c_int {
    if !enable[dev as usize] {
        return 0;
    }
    #[cfg(CONFIG_PNP)]
    {
        if isapnp[dev as usize] {
            return 0;
        }
    }
    1
}

unsafe extern "C" fn snd_interwave_isa_probe(pdev: *mut device, dev: c_uint) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut gus: *mut snd_gus_card = ptr::null_mut();
    let mut err: c_int;
    static possible_irqs: [c_int; 8] = [5, 11, 12, 9, 7, 15, 3, -1];
    static possible_dmas: [c_int; 7] = [0, 1, 3, 5, 6, 7, -1];

    if irq[dev as usize] == SNDRV_AUTO_IRQ {
        irq[dev as usize] = snd_legacy_find_free_irq(possible_irqs.as_ptr());
        if irq[dev as usize] < 0 {
            dev_err(pdev, b"unable to find a free IRQ\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
    }
    if dma1[dev as usize] == SNDRV_AUTO_DMA {
        dma1[dev as usize] = snd_legacy_find_free_dma(possible_dmas.as_ptr());
        if dma1[dev as usize] < 0 {
            dev_err(pdev, b"unable to find a free DMA1\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
    }
    if dma2[dev as usize] == SNDRV_AUTO_DMA {
        dma2[dev as usize] = snd_legacy_find_free_dma(possible_dmas.as_ptr());
        if dma2[dev as usize] < 0 {
            dev_err(pdev, b"unable to find a free DMA2\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }
    }

    err = snd_interwave_card_new(pdev, dev as c_int, &mut card);
    if err < 0 {
        return err;
    }

    if port[dev as usize] != SNDRV_AUTO_PORT {
        err = snd_interwave_probe_gus(card, dev as c_int, &mut gus);
    } else {
        static possible_ports: [c_long; 6] = [0x210, 0x220, 0x230, 0x240, 0x250, 0x260];
        let mut i: usize = 0;
        err = -ENODEV;
        while i < possible_ports.len() {
            port[dev as usize] = possible_ports[i];
            err = snd_interwave_probe_gus(card, dev as c_int, &mut gus);
            if err == 0 {
                return 0;
            }
            i += 1;
        }
    }
    if err < 0 {
        return err;
    }

    err = snd_interwave_probe(card, dev as c_int, gus);
    if err < 0 {
        return err;
    }

    dev_set_drvdata(pdev, card as *mut c_void);
    0
}

#[cfg(CONFIG_PM)]
unsafe fn snd_interwave_restore_regs(gus: *mut snd_gus_card) {
    __snd_interwave_restore_regs(gus);
}

#[cfg(CONFIG_PM)]
unsafe fn snd_interwave_restore_memory(gus: *mut snd_gus_card) {
    let mut mem_cfg: c_ushort;
    let mut lmct: c_uint = 0;
    let mut i: c_int;
    let mut lmc_cfg: c_int;

    if (*gus).gf1.memory == 0 {
        return;
    }

    i = 0;
    while i < 4 {
        lmct |= ((*gus).gf1.mem_alloc.banks_16[i as usize].size >> 18) << (i * 8);
        i += 1;
    }

    lmc_cfg = snd_interwave_find_memory_config(lmct);
    if lmc_cfg < 0 {
        if (*gus).gf1.enh_mode == 0 {
            lmc_cfg = 2;
        } else {
            dev_warn((*(*gus).card).dev, b"cannot restore InterWave memory layout 0x%08x\n\0".as_ptr() as *const c_char, lmct);
            return;
        }
    }

    mem_cfg = snd_gf1_look16(gus, SNDRV_GF1_GW_MEMORY_CONFIG);
    mem_cfg = (mem_cfg & 0xfff0) | lmc_cfg as c_ushort;
    mem_cfg = (mem_cfg & 0xff1f) | (4 << 5);
    snd_gf1_write16(gus, SNDRV_GF1_GW_MEMORY_CONFIG, mem_cfg);
}

#[cfg(CONFIG_PM)]
unsafe fn snd_interwave_card_suspend(card: *mut snd_card) -> c_int {
    let iwcard = (*card).private_data as *mut snd_interwave;

    ((*(*iwcard).wss).suspend.unwrap())((*iwcard).wss);
    snd_gus_suspend((*iwcard).gus)
}

#[cfg(CONFIG_PM)]
unsafe fn snd_interwave_card_resume(card: *mut snd_card) -> c_int {
    let iwcard = (*card).private_data as *mut snd_interwave;
    let mut err: c_int;

    err = snd_gus_resume((*iwcard).gus);
    if err < 0 {
        return err;
    }

    snd_interwave_restore_regs((*iwcard).gus);
    snd_interwave_restore_memory((*iwcard).gus);
    ((*(*iwcard).wss).resume.unwrap())((*iwcard).wss);
    #[cfg(SNDRV_STB)]
    {
        if !(*iwcard).i2c_bus.is_null() {
            err = snd_tea6330t_restore_mixer((*iwcard).i2c_bus);
            if err < 0 {
                dev_warn((*card).dev, b"failed to restore TEA6330T mixer state: %d\n\0".as_ptr() as *const c_char, err);
            }
        }
    }

    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_interwave_isa_suspend(pdev: *mut device, _dev: c_uint, _state: pm_message_t) -> c_int {
    snd_interwave_card_suspend(dev_get_drvdata(pdev) as *mut snd_card)
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn snd_interwave_isa_resume(pdev: *mut device, _dev: c_uint) -> c_int {
    snd_interwave_card_resume(dev_get_drvdata(pdev) as *mut snd_card)
}

static mut snd_interwave_driver: isa_driver = isa_driver {
    match_: Some(snd_interwave_isa_match),
    probe: Some(snd_interwave_isa_probe),
    suspend: None,
    resume: None,
    driver: isa_driver_inner {
        name: INTERWAVE_DRIVER,
    },
};

#[cfg(CONFIG_PNP)]
unsafe extern "C" fn snd_interwave_pnp_detect(pcard: *mut pnp_card_link, pid: *const pnp_card_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    let mut gus: *mut snd_gus_card = ptr::null_mut();
    let mut res: c_int;

    while dev < SNDRV_CARDS as c_int {
        if enable[dev as usize] && isapnp[dev as usize] {
            break;
        }
        dev += 1;
    }
    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }

    res = snd_interwave_card_new(&mut (*(*pcard).card).dev, dev, &mut card);
    if res < 0 {
        return res;
    }

    res = snd_interwave_pnp(dev, (*card).private_data as *mut snd_interwave, pcard, pid);
    if res < 0 {
        return res;
    }
    res = snd_interwave_probe_gus(card, dev, &mut gus);
    if res < 0 {
        return res;
    }
    res = snd_interwave_probe(card, dev, gus);
    if res < 0 {
        return res;
    }
    pnp_set_card_drvdata(pcard, card as *mut c_void);
    dev += 1;
    0
}

#[cfg(all(CONFIG_PNP, CONFIG_PM))]
unsafe extern "C" fn snd_interwave_pnpc_suspend(pcard: *mut pnp_card_link, _state: pm_message_t) -> c_int {
    snd_interwave_card_suspend(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

#[cfg(all(CONFIG_PNP, CONFIG_PM))]
unsafe extern "C" fn snd_interwave_pnpc_resume(pcard: *mut pnp_card_link) -> c_int {
    snd_interwave_card_resume(pnp_get_card_drvdata(pcard) as *mut snd_card)
}

#[cfg(CONFIG_PNP)]
static mut interwave_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DISABLE,
    name: INTERWAVE_PNP_DRIVER,
    id_table: snd_interwave_pnpids.as_ptr(),
    probe: Some(snd_interwave_pnp_detect),
    suspend: None,
    resume: None,
};

unsafe fn alsa_card_interwave_init() -> c_int {
    let mut err: c_int;

    err = isa_register_driver(&mut snd_interwave_driver, SNDRV_CARDS as c_uint);
    #[cfg(CONFIG_PNP)]
    {
        if err == 0 {
            isa_registered = 1;
        }

        err = pnp_register_card_driver(&mut interwave_pnpc_driver);
        if err == 0 {
            pnp_registered = 1;
        }

        if isa_registered != 0 {
            err = 0;
        }
    }
    err
}

unsafe fn alsa_card_interwave_exit() {
    #[cfg(CONFIG_PNP)]
    {
        if pnp_registered != 0 {
            pnp_unregister_card_driver(&mut interwave_pnpc_driver);
        }
        if isa_registered != 0 {
            isa_unregister_driver(&mut snd_interwave_driver);
        }
    }
    #[cfg(not(CONFIG_PNP))]
    {
        isa_unregister_driver(&mut snd_interwave_driver);
    }
}

/* module_init(alsa_card_interwave_init) */
/* module_exit(alsa_card_interwave_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
