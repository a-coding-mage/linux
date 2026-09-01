// SPDX-License-Identifier: GPL-2.0
//
// peb2466.c  --  Infineon PEB2466 ALSA SoC driver
//
// Copyright 2023 CS GROUP France
//
// Author: Herve Codina <herve.codina@bootlin.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type s32 = i32;
type size_t = usize;

const EINVAL: c_int = 22;
const EILSEQ: c_int = 84;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const U32_MAX: u32 = u32::MAX;
const REGCACHE_NONE: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const GPIO_LINE_DIRECTION_IN: c_int = 1;
const GPIO_LINE_DIRECTION_OUT: c_int = 0;
const CONFIG_GPIOLIB: bool = true;

#[repr(C)]
pub struct device {
    pub of_node: *mut c_void,
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
    pub bits_per_word: u8,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
    pub size: size_t,
    pub data: *const u8,
}

#[repr(C)]
pub struct spi_transfer {
    pub tx_buf: *const c_void,
    pub rx_buf: *mut c_void,
    pub len: c_uint,
}

#[repr(C)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
    pub reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
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
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub access: c_uint,
    pub tlv_p: *const c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct gpio_chip {
    pub owner: *mut c_void,
    pub label: *const c_char,
    pub parent: *mut device,
    pub base: c_int,
    pub ngpio: c_uint,
    pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub can_sleep: bool,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct spi_device_id {
    pub name: *const c_char,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: driver_inner,
    pub id_table: *const spi_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    fn spi_sync_transfer(spi: *mut spi_device, xfers: *mut spi_transfer, num: c_uint) -> c_int;
    fn spi_setup(spi: *mut spi_device) -> c_int;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, control: *const snd_kcontrol_new, num: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, num_regs: c_uint) -> c_int;
    fn devm_regmap_init(dev: *mut device, bus: *const c_void, context: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn release_firmware(fw: *const firmware);
    fn of_property_read_string(np: *mut c_void, propname: *const c_char, out_string: *mut *const c_char) -> c_int;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut c_void, var: c_uint, min: c_uint, max: c_uint) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut c_void, cond: c_uint, var: c_uint, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn udelay(usecs: c_uint);
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn devm_gpiochip_add_data(dev: *mut device, chip: *mut gpio_chip, data: *mut c_void) -> c_int;
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut c_void;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const PEB2466_NB_CHANNEL: usize = 4;

#[repr(C)]
pub struct peb2466_lookup {
    pub table: *mut [u8; 4],
    pub count: c_uint,
}

const PEB2466_TLV_SIZE: usize = 4;

#[repr(C)]
pub struct peb2466_lkup_ctrl {
    pub reg: c_int,
    pub index: c_uint,
    pub lookup: *const peb2466_lookup,
    pub tlv_array: [c_uint; PEB2466_TLV_SIZE],
}

#[repr(C)]
pub struct peb2466_channel {
    pub ax_lookup: peb2466_lookup,
    pub ar_lookup: peb2466_lookup,
    pub ax_lkup_ctrl: peb2466_lkup_ctrl,
    pub ar_lkup_ctrl: peb2466_lkup_ctrl,
    pub tg1_freq_item: c_uint,
    pub tg2_freq_item: c_uint,
}

#[repr(C)]
pub struct peb2466_gpio_cache {
    pub xr0: c_uint,
    pub xr1: c_uint,
    pub xr2: c_uint,
    pub xr3: c_uint,
}

#[repr(C)]
pub struct peb2466_gpio {
    pub gpio_chip: gpio_chip,
    pub lock: mutex,
    pub cache: peb2466_gpio_cache,
}

#[repr(C)]
pub struct peb2466 {
    pub spi: *mut spi_device,
    pub mclk: *mut clk,
    pub reset_gpio: *mut gpio_desc,
    pub spi_tx_buf: [u8; 2 + 8], /* Cannot use stack area for SPI (dma-safe memory) */
    pub spi_rx_buf: [u8; 2 + 8], /* Cannot use stack area for SPI (dma-safe memory) */
    pub regmap: *mut regmap,
    pub ch: [peb2466_channel; PEB2466_NB_CHANNEL],
    pub max_chan_playback: c_int,
    pub max_chan_capture: c_int,
    pub gpio: peb2466_gpio,
}

const PEB2466_CMD_R: c_uint = 1 << 5;
const PEB2466_CMD_W: c_uint = 0 << 5;

const PEB2466_CMD_MASK: c_uint = 0x18;
const PEB2466_CMD_XOP: c_uint = 0x18; /* XOP is 0bxxx11xxx */
const PEB2466_CMD_SOP: c_uint = 0x10; /* SOP is 0bxxx10xxx */
const PEB2466_CMD_COP: c_uint = 0x00; /* COP is 0bxxx0xxxx, handle 0bxxx00xxx */
const PEB2466_CMD_COP1: c_uint = 0x08; /* COP is 0bxxx0xxxx, handle 0bxxx01xxx */

const fn PEB2466_MAKE_XOP(_lsel: c_uint) -> c_uint { PEB2466_CMD_XOP | _lsel }
const fn PEB2466_MAKE_SOP(_ad: c_uint, _lsel: c_uint) -> c_uint { PEB2466_CMD_SOP | (_ad << 6) | _lsel }
const fn PEB2466_MAKE_COP(_ad: c_uint, _code: c_uint) -> c_uint { PEB2466_CMD_COP | (_ad << 6) | _code }

const fn PEB2466_CR0(_ch: c_uint) -> c_uint { PEB2466_MAKE_SOP(_ch, 0x0) }
const PEB2466_CR0_TH: c_uint = 1 << 7;
const PEB2466_CR0_IMR1: c_uint = 1 << 6;
const PEB2466_CR0_FRX: c_uint = 1 << 5;
const PEB2466_CR0_FRR: c_uint = 1 << 4;
const PEB2466_CR0_AX: c_uint = 1 << 3;
const PEB2466_CR0_AR: c_uint = 1 << 2;
const PEB2466_CR0_THSEL_MASK: c_uint = 0x3 << 0;
const fn PEB2466_CR0_THSEL(_set: c_uint) -> c_uint { _set << 0 }

const fn PEB2466_CR1(_ch: c_uint) -> c_uint { PEB2466_MAKE_SOP(_ch, 0x1) }
const PEB2466_CR1_ETG2: c_uint = 1 << 7;
const PEB2466_CR1_ETG1: c_uint = 1 << 6;
const PEB2466_CR1_PTG2: c_uint = 1 << 5;
const PEB2466_CR1_PTG1: c_uint = 1 << 4;
const PEB2466_CR1_LAW_MASK: c_uint = 1 << 3;
const PEB2466_CR1_LAW_ALAW: c_uint = 0 << 3;
const PEB2466_CR1_LAW_MULAW: c_uint = 1 << 3;
const PEB2466_CR1_PU: c_uint = 1 << 0;

const fn PEB2466_CR2(_ch: c_uint) -> c_uint { PEB2466_MAKE_SOP(_ch, 0x2) }
const fn PEB2466_CR3(_ch: c_uint) -> c_uint { PEB2466_MAKE_SOP(_ch, 0x3) }
const fn PEB2466_CR4(_ch: c_uint) -> c_uint { PEB2466_MAKE_SOP(_ch, 0x4) }
const fn PEB2466_CR5(_ch: c_uint) -> c_uint { PEB2466_MAKE_SOP(_ch, 0x5) }

const PEB2466_XR0: c_uint = PEB2466_MAKE_XOP(0x0);
const PEB2466_XR1: c_uint = PEB2466_MAKE_XOP(0x1);
const PEB2466_XR2: c_uint = PEB2466_MAKE_XOP(0x2);
const PEB2466_XR3: c_uint = PEB2466_MAKE_XOP(0x3);
const PEB2466_XR4: c_uint = PEB2466_MAKE_XOP(0x4);
const PEB2466_XR5: c_uint = PEB2466_MAKE_XOP(0x5);
const PEB2466_XR5_MCLK_1536: c_uint = 0x0 << 6;
const PEB2466_XR5_MCLK_2048: c_uint = 0x1 << 6;
const PEB2466_XR5_MCLK_4096: c_uint = 0x2 << 6;
const PEB2466_XR5_MCLK_8192: c_uint = 0x3 << 6;

const PEB2466_XR6: c_uint = PEB2466_MAKE_XOP(0x6);
const fn PEB2466_XR6_PCM_OFFSET(_off: c_uint) -> c_uint { _off << 0 }

const PEB2466_XR7: c_uint = PEB2466_MAKE_XOP(0x7);

const fn PEB2466_TH_FILTER_P1(_ch: c_uint) -> c_uint { PEB2466_MAKE_COP(_ch, 0x0) }
const fn PEB2466_TH_FILTER_P2(_ch: c_uint) -> c_uint { PEB2466_MAKE_COP(_ch, 0x1) }
const fn PEB2466_TH_FILTER_P3(_ch: c_uint) -> c_uint { PEB2466_MAKE_COP(_ch, 0x2) }
const fn PEB2466_IMR1_FILTER_P1(_ch: c_uint) -> c_uint { PEB2466_MAKE_COP(_ch, 0x4) }
const fn PEB2466_IMR1_FILTER_P2(_ch: c_uint) -> c_uint { PEB2466_MAKE_COP(_ch, 0x5) }
const fn PEB2466_FRX_FILTER(_ch: c_uint) -> c_uint { PEB2466_MAKE_COP(_ch, 0x6) }
const fn PEB2466_FRR_FILTER(_ch: c_uint) -> c_uint { PEB2466_MAKE_COP(_ch, 0x7) }
const fn PEB2466_AX_FILTER(_ch: c_uint) -> c_uint { PEB2466_MAKE_COP(_ch, 0x8) }
const fn PEB2466_AR_FILTER(_ch: c_uint) -> c_uint { PEB2466_MAKE_COP(_ch, 0x9) }
const fn PEB2466_TG1(_ch: c_uint) -> c_uint { PEB2466_MAKE_COP(_ch, 0xc) }
const fn PEB2466_TG2(_ch: c_uint) -> c_uint { PEB2466_MAKE_COP(_ch, 0xd) }

unsafe extern "C" fn peb2466_write_byte(peb2466: *mut peb2466, cmd: u8, val: u8) -> c_int {
    let mut xfer = spi_transfer {
        tx_buf: (*peb2466).spi_tx_buf.as_ptr() as *const c_void,
        rx_buf: ptr::null_mut(),
        len: 2,
    };

    (*peb2466).spi_tx_buf[0] = cmd | PEB2466_CMD_W as u8;
    (*peb2466).spi_tx_buf[1] = val;

    dev_dbg(&mut (*(*peb2466).spi).dev, c!("write byte (cmd %02x) %02x\n"),
            (*peb2466).spi_tx_buf[0] as c_uint, (*peb2466).spi_tx_buf[1] as c_uint);

    spi_sync_transfer((*peb2466).spi, &mut xfer, 1)
}

unsafe extern "C" fn peb2466_read_byte(peb2466: *mut peb2466, cmd: u8, val: *mut u8) -> c_int {
    let mut xfer = spi_transfer {
        tx_buf: (*peb2466).spi_tx_buf.as_ptr() as *const c_void,
        rx_buf: (*peb2466).spi_rx_buf.as_mut_ptr() as *mut c_void,
        len: 3,
    };
    let ret: c_int;

    (*peb2466).spi_tx_buf[0] = cmd | PEB2466_CMD_R as u8;

    ret = spi_sync_transfer((*peb2466).spi, &mut xfer, 1);
    if ret != 0 {
        return ret;
    }

    if (*peb2466).spi_rx_buf[1] != 0x81 {
        dev_err(&mut (*(*peb2466).spi).dev,
                c!("spi xfer rd (cmd %02x) invalid ident byte (0x%02x)\n"),
                (*peb2466).spi_tx_buf[0] as c_uint, (*peb2466).spi_rx_buf[1] as c_uint);
        return -EILSEQ;
    }

    *val = (*peb2466).spi_rx_buf[2];

    dev_dbg(&mut (*(*peb2466).spi).dev, c!("read byte (cmd %02x) %02x\n"),
            (*peb2466).spi_tx_buf[0] as c_uint, *val as c_uint);

    0
}

unsafe extern "C" fn peb2466_write_buf(peb2466: *mut peb2466, cmd: u8, buf: *const u8, len: c_uint) -> c_int {
    let mut xfer = spi_transfer {
        tx_buf: (*peb2466).spi_tx_buf.as_ptr() as *const c_void,
        rx_buf: ptr::null_mut(),
        len: len + 1,
    };

    if len > 8 {
        return -EINVAL;
    }

    (*peb2466).spi_tx_buf[0] = cmd | PEB2466_CMD_W as u8;
    ptr::copy_nonoverlapping(buf, (*peb2466).spi_tx_buf.as_mut_ptr().add(1), len as usize);

    dev_dbg(&mut (*(*peb2466).spi).dev, c!("write buf (cmd %02x, %u) %*ph\n"),
            (*peb2466).spi_tx_buf[0] as c_uint, len, len as c_int,
            (*peb2466).spi_tx_buf.as_ptr().add(1));

    spi_sync_transfer((*peb2466).spi, &mut xfer, 1)
}

unsafe extern "C" fn peb2466_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    let peb2466 = context as *mut peb2466;
    let ret: c_int;

    /*
     * Only XOP and SOP commands can be handled as registers.
     * COP commands are handled using direct peb2466_write_buf() calls.
     */
    match reg & PEB2466_CMD_MASK {
        PEB2466_CMD_XOP | PEB2466_CMD_SOP => ret = peb2466_write_byte(peb2466, reg as u8, val as u8),
        _ => {
            dev_err(&mut (*(*peb2466).spi).dev, c!("Not a XOP or SOP command\n"));
            ret = -EINVAL;
        }
    }
    ret
}

unsafe extern "C" fn peb2466_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    let peb2466 = context as *mut peb2466;
    let ret: c_int;
    let mut tmp: u8 = 0;

    /* Only XOP and SOP commands can be handled as registers */
    match reg & PEB2466_CMD_MASK {
        PEB2466_CMD_XOP | PEB2466_CMD_SOP => {
            ret = peb2466_read_byte(peb2466, reg as u8, &mut tmp);
            if ret == 0 {
                *val = tmp as c_uint;
            }
        }
        _ => {
            dev_err(&mut (*(*peb2466).spi).dev, c!("Not a XOP or SOP command\n"));
            ret = -EINVAL;
        }
    }
    ret
}

static peb2466_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: 0xFF,
    reg_write: Some(peb2466_reg_write),
    reg_read: Some(peb2466_reg_read),
    cache_type: REGCACHE_NONE,
};

unsafe extern "C" fn peb2466_lkup_ctrl_info(kcontrol: *mut snd_kcontrol,
                                            uinfo: *mut snd_ctl_elem_info) -> c_int {
    let lkup_ctrl = (*kcontrol).private_value as *mut peb2466_lkup_ctrl;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = ((*(*lkup_ctrl).lookup).count - 1) as i64;
    0
}

unsafe extern "C" fn peb2466_lkup_ctrl_get(kcontrol: *mut snd_kcontrol,
                                           ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let lkup_ctrl = (*kcontrol).private_value as *mut peb2466_lkup_ctrl;

    (*ucontrol).value.integer.value[0] = (*lkup_ctrl).index as i64;
    0
}

unsafe extern "C" fn peb2466_lkup_ctrl_put(kcontrol: *mut snd_kcontrol,
                                           ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let lkup_ctrl = (*kcontrol).private_value as *mut peb2466_lkup_ctrl;
    let component = snd_kcontrol_chip(kcontrol);
    let peb2466 = snd_soc_component_get_drvdata(component) as *mut peb2466;
    let index: c_uint;
    let ret: c_int;

    index = (*ucontrol).value.integer.value[0] as c_uint;
    if index >= (*(*lkup_ctrl).lookup).count {
        return -EINVAL;
    }

    if index == (*lkup_ctrl).index {
        return 0;
    }

    ret = peb2466_write_buf(peb2466, (*lkup_ctrl).reg as u8,
                            (*(*lkup_ctrl).lookup).table.add(index as usize) as *const u8, 4);
    if ret != 0 {
        return ret;
    }

    (*lkup_ctrl).index = index;
    1 /* The value changed */
}

unsafe extern "C" fn peb2466_add_lkup_ctrl(component: *mut snd_soc_component,
                                           lkup_ctrl: *mut peb2466_lkup_ctrl,
                                           name: *const c_char, min_val: c_int, step: c_int) -> c_int {
    let tlv_array: [c_uint; PEB2466_TLV_SIZE] = tlv_db_scale(min_val, step, 0);
    let mut control: snd_kcontrol_new = core::mem::zeroed();

    /* BUILD_BUG_ON(sizeof(lkup_ctrl->tlv_array) < sizeof(tlv_array)); */
    ptr::copy_nonoverlapping(tlv_array.as_ptr(), (*lkup_ctrl).tlv_array.as_mut_ptr(), tlv_array.len());

    control.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    control.name = name;
    control.access = SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_READWRITE;
    control.tlv_p = (*lkup_ctrl).tlv_array.as_ptr();
    control.info = Some(peb2466_lkup_ctrl_info);
    control.get = Some(peb2466_lkup_ctrl_get);
    control.put = Some(peb2466_lkup_ctrl_put);
    control.private_value = lkup_ctrl as c_ulong;

    snd_soc_add_component_controls(component, &control, 1)
}

const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 1 << 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 3;

const fn tlv_db_scale(min: c_int, step: c_int, mute: c_int) -> [c_uint; PEB2466_TLV_SIZE] {
    [0, 2 * size_of::<c_uint>() as c_uint, min as c_uint, ((step as c_uint) << 16) | mute as c_uint]
}

#[repr(C)]
enum peb2466_tone_freq {
    PEB2466_TONE_697HZ,
    PEB2466_TONE_800HZ,
    PEB2466_TONE_950HZ,
    PEB2466_TONE_1000HZ,
    PEB2466_TONE_1008HZ,
    PEB2466_TONE_2000HZ,
}

const PEB2466_TONE_697HZ: c_uint = peb2466_tone_freq::PEB2466_TONE_697HZ as c_uint;
const PEB2466_TONE_800HZ: c_uint = peb2466_tone_freq::PEB2466_TONE_800HZ as c_uint;
const PEB2466_TONE_950HZ: c_uint = peb2466_tone_freq::PEB2466_TONE_950HZ as c_uint;
const PEB2466_TONE_1000HZ: c_uint = peb2466_tone_freq::PEB2466_TONE_1000HZ as c_uint;
const PEB2466_TONE_1008HZ: c_uint = peb2466_tone_freq::PEB2466_TONE_1008HZ as c_uint;
const PEB2466_TONE_2000HZ: c_uint = peb2466_tone_freq::PEB2466_TONE_2000HZ as c_uint;

static peb2466_tone_lookup: [[u8; 4]; 6] = [
    [0x0a, 0x33, 0x5a, 0x2c],
    [0x12, 0xD6, 0x5a, 0xc0],
    [0x1c, 0xf0, 0x5c, 0xc0],
    [0; 4], /* lookup value not used for 1000Hz */
    [0x1a, 0xae, 0x57, 0x70],
    [0x00, 0x80, 0x50, 0x09],
];

static peb2466_tone_freq_txt: [*const c_char; 6] = [
    c!("697Hz"),
    c!("800Hz"),
    c!("950Hz"),
    c!("1000Hz"),
    c!("1008Hz"),
    c!("2000Hz"),
];

static peb2466_tg_freq: [[soc_enum; 2]; 4] = [
    [
        soc_enum { reg: PEB2466_TG1(0), shift_l: 0, items: 6, texts: peb2466_tone_freq_txt.as_ptr() },
        soc_enum { reg: PEB2466_TG2(0), shift_l: 0, items: 6, texts: peb2466_tone_freq_txt.as_ptr() },
    ],
    [
        soc_enum { reg: PEB2466_TG1(1), shift_l: 0, items: 6, texts: peb2466_tone_freq_txt.as_ptr() },
        soc_enum { reg: PEB2466_TG2(1), shift_l: 0, items: 6, texts: peb2466_tone_freq_txt.as_ptr() },
    ],
    [
        soc_enum { reg: PEB2466_TG1(2), shift_l: 0, items: 6, texts: peb2466_tone_freq_txt.as_ptr() },
        soc_enum { reg: PEB2466_TG2(2), shift_l: 0, items: 6, texts: peb2466_tone_freq_txt.as_ptr() },
    ],
    [
        soc_enum { reg: PEB2466_TG1(3), shift_l: 0, items: 6, texts: peb2466_tone_freq_txt.as_ptr() },
        soc_enum { reg: PEB2466_TG2(3), shift_l: 0, items: 6, texts: peb2466_tone_freq_txt.as_ptr() },
    ],
];

unsafe extern "C" fn peb2466_tg_freq_get(kcontrol: *mut snd_kcontrol,
                                         ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let peb2466 = snd_soc_component_get_drvdata(component) as *mut peb2466;
    let e = (*kcontrol).private_value as *mut soc_enum;

    match (*e).reg {
        x if x == PEB2466_TG1(0) => (*ucontrol).value.enumerated.item[0] = (*peb2466).ch[0].tg1_freq_item,
        x if x == PEB2466_TG2(0) => (*ucontrol).value.enumerated.item[0] = (*peb2466).ch[0].tg2_freq_item,
        x if x == PEB2466_TG1(1) => (*ucontrol).value.enumerated.item[0] = (*peb2466).ch[1].tg1_freq_item,
        x if x == PEB2466_TG2(1) => (*ucontrol).value.enumerated.item[0] = (*peb2466).ch[1].tg2_freq_item,
        x if x == PEB2466_TG1(2) => (*ucontrol).value.enumerated.item[0] = (*peb2466).ch[2].tg1_freq_item,
        x if x == PEB2466_TG2(2) => (*ucontrol).value.enumerated.item[0] = (*peb2466).ch[2].tg2_freq_item,
        x if x == PEB2466_TG1(3) => (*ucontrol).value.enumerated.item[0] = (*peb2466).ch[3].tg1_freq_item,
        x if x == PEB2466_TG2(3) => (*ucontrol).value.enumerated.item[0] = (*peb2466).ch[3].tg2_freq_item,
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn peb2466_tg_freq_put(kcontrol: *mut snd_kcontrol,
                                         ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let peb2466 = snd_soc_component_get_drvdata(component) as *mut peb2466;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let tg_freq_item: *mut c_uint;
    let cr1_reg: u8;
    let cr1_mask: u8;
    let index: c_uint;
    let mut ret: c_int;

    index = (*ucontrol).value.enumerated.item[0];

    if index >= peb2466_tone_lookup.len() as c_uint {
        return -EINVAL;
    }

    match (*e).reg {
        x if x == PEB2466_TG1(0) => { tg_freq_item = &mut (*peb2466).ch[0].tg1_freq_item; cr1_reg = PEB2466_CR1(0) as u8; cr1_mask = PEB2466_CR1_PTG1 as u8; }
        x if x == PEB2466_TG2(0) => { tg_freq_item = &mut (*peb2466).ch[0].tg2_freq_item; cr1_reg = PEB2466_CR1(0) as u8; cr1_mask = PEB2466_CR1_PTG2 as u8; }
        x if x == PEB2466_TG1(1) => { tg_freq_item = &mut (*peb2466).ch[1].tg1_freq_item; cr1_reg = PEB2466_CR1(1) as u8; cr1_mask = PEB2466_CR1_PTG1 as u8; }
        x if x == PEB2466_TG2(1) => { tg_freq_item = &mut (*peb2466).ch[1].tg2_freq_item; cr1_reg = PEB2466_CR1(1) as u8; cr1_mask = PEB2466_CR1_PTG2 as u8; }
        x if x == PEB2466_TG1(2) => { tg_freq_item = &mut (*peb2466).ch[2].tg1_freq_item; cr1_reg = PEB2466_CR1(2) as u8; cr1_mask = PEB2466_CR1_PTG1 as u8; }
        x if x == PEB2466_TG2(2) => { tg_freq_item = &mut (*peb2466).ch[2].tg2_freq_item; cr1_reg = PEB2466_CR1(2) as u8; cr1_mask = PEB2466_CR1_PTG2 as u8; }
        x if x == PEB2466_TG1(3) => { tg_freq_item = &mut (*peb2466).ch[3].tg1_freq_item; cr1_reg = PEB2466_CR1(3) as u8; cr1_mask = PEB2466_CR1_PTG1 as u8; }
        x if x == PEB2466_TG2(3) => { tg_freq_item = &mut (*peb2466).ch[3].tg2_freq_item; cr1_reg = PEB2466_CR1(3) as u8; cr1_mask = PEB2466_CR1_PTG2 as u8; }
        _ => return -EINVAL,
    }

    if index == *tg_freq_item {
        return 0;
    }

    if index == PEB2466_TONE_1000HZ {
        ret = regmap_update_bits((*peb2466).regmap, cr1_reg as c_uint, cr1_mask as c_uint, 0);
        if ret != 0 { return ret; }
    } else {
        ret = peb2466_write_buf(peb2466, (*e).reg as u8, peb2466_tone_lookup[index as usize].as_ptr(), 4);
        if ret != 0 { return ret; }
        ret = regmap_update_bits((*peb2466).regmap, cr1_reg as c_uint, cr1_mask as c_uint, cr1_mask as c_uint);
        if ret != 0 { return ret; }
    }

    *tg_freq_item = index;
    1 /* The value changed */
}

/* ALSA SoC declarative controls/widgets are macro-created in C; the Rust
 * translation keeps their externally meaningful arrays as zero-sized placeholders
 * with comments preserving the original declarations. */
static peb2466_ch0_out_mix_controls: [snd_kcontrol_new; 0] = [];
static peb2466_ch1_out_mix_controls: [snd_kcontrol_new; 0] = [];
static peb2466_ch2_out_mix_controls: [snd_kcontrol_new; 0] = [];
static peb2466_ch3_out_mix_controls: [snd_kcontrol_new; 0] = [];

static peb2466_gain_p_tlv: [c_uint; 4] = [0, 2 * size_of::<c_uint>() as c_uint, (-600i32) as c_uint, 0];
static peb2466_gain_c_tlv: [c_uint; 4] = [0, 2 * size_of::<c_uint>() as c_uint, 0, 600];

static peb2466_controls: [snd_kcontrol_new; 0] = [];
static peb2466_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static peb2466_dapm_routes: [snd_soc_dapm_route; 48] = [
    snd_soc_dapm_route { sink: c!("CH0 DIN"), control: ptr::null(), source: c!("CH0 PWR") },
    snd_soc_dapm_route { sink: c!("CH1 DIN"), control: ptr::null(), source: c!("CH1 PWR") },
    snd_soc_dapm_route { sink: c!("CH2 DIN"), control: ptr::null(), source: c!("CH2 PWR") },
    snd_soc_dapm_route { sink: c!("CH3 DIN"), control: ptr::null(), source: c!("CH3 PWR") },
    snd_soc_dapm_route { sink: c!("CH0 TG1"), control: ptr::null(), source: c!("CH0 PWR") },
    snd_soc_dapm_route { sink: c!("CH1 TG1"), control: ptr::null(), source: c!("CH1 PWR") },
    snd_soc_dapm_route { sink: c!("CH2 TG1"), control: ptr::null(), source: c!("CH2 PWR") },
    snd_soc_dapm_route { sink: c!("CH3 TG1"), control: ptr::null(), source: c!("CH3 PWR") },
    snd_soc_dapm_route { sink: c!("CH0 TG2"), control: ptr::null(), source: c!("CH0 PWR") },
    snd_soc_dapm_route { sink: c!("CH1 TG2"), control: ptr::null(), source: c!("CH1 PWR") },
    snd_soc_dapm_route { sink: c!("CH2 TG2"), control: ptr::null(), source: c!("CH2 PWR") },
    snd_soc_dapm_route { sink: c!("CH3 TG2"), control: ptr::null(), source: c!("CH3 PWR") },
    snd_soc_dapm_route { sink: c!("DAC0 Mixer"), control: c!("TG1 Switch"), source: c!("CH0 TG1") },
    snd_soc_dapm_route { sink: c!("DAC0 Mixer"), control: c!("TG2 Switch"), source: c!("CH0 TG2") },
    snd_soc_dapm_route { sink: c!("DAC0 Mixer"), control: c!("Voice Switch"), source: c!("CH0 DIN") },
    snd_soc_dapm_route { sink: c!("DAC0 Mixer"), control: ptr::null(), source: c!("CH0 DIN") },
    snd_soc_dapm_route { sink: c!("DAC1 Mixer"), control: c!("TG1 Switch"), source: c!("CH1 TG1") },
    snd_soc_dapm_route { sink: c!("DAC1 Mixer"), control: c!("TG2 Switch"), source: c!("CH1 TG2") },
    snd_soc_dapm_route { sink: c!("DAC1 Mixer"), control: c!("Voice Switch"), source: c!("CH1 DIN") },
    snd_soc_dapm_route { sink: c!("DAC1 Mixer"), control: ptr::null(), source: c!("CH1 DIN") },
    snd_soc_dapm_route { sink: c!("DAC2 Mixer"), control: c!("TG1 Switch"), source: c!("CH2 TG1") },
    snd_soc_dapm_route { sink: c!("DAC2 Mixer"), control: c!("TG2 Switch"), source: c!("CH2 TG2") },
    snd_soc_dapm_route { sink: c!("DAC2 Mixer"), control: c!("Voice Switch"), source: c!("CH2 DIN") },
    snd_soc_dapm_route { sink: c!("DAC2 Mixer"), control: ptr::null(), source: c!("CH2 DIN") },
    snd_soc_dapm_route { sink: c!("DAC3 Mixer"), control: c!("TG1 Switch"), source: c!("CH3 TG1") },
    snd_soc_dapm_route { sink: c!("DAC3 Mixer"), control: c!("TG2 Switch"), source: c!("CH3 TG2") },
    snd_soc_dapm_route { sink: c!("DAC3 Mixer"), control: c!("Voice Switch"), source: c!("CH3 DIN") },
    snd_soc_dapm_route { sink: c!("DAC3 Mixer"), control: ptr::null(), source: c!("CH3 DIN") },
    snd_soc_dapm_route { sink: c!("DAC0 PGA"), control: ptr::null(), source: c!("DAC0 Mixer") },
    snd_soc_dapm_route { sink: c!("DAC1 PGA"), control: ptr::null(), source: c!("DAC1 Mixer") },
    snd_soc_dapm_route { sink: c!("DAC2 PGA"), control: ptr::null(), source: c!("DAC2 Mixer") },
    snd_soc_dapm_route { sink: c!("DAC3 PGA"), control: ptr::null(), source: c!("DAC3 Mixer") },
    snd_soc_dapm_route { sink: c!("OUT0"), control: ptr::null(), source: c!("DAC0 PGA") },
    snd_soc_dapm_route { sink: c!("OUT1"), control: ptr::null(), source: c!("DAC1 PGA") },
    snd_soc_dapm_route { sink: c!("OUT2"), control: ptr::null(), source: c!("DAC2 PGA") },
    snd_soc_dapm_route { sink: c!("OUT3"), control: ptr::null(), source: c!("DAC3 PGA") },
    snd_soc_dapm_route { sink: c!("ADC0"), control: ptr::null(), source: c!("IN0") },
    snd_soc_dapm_route { sink: c!("ADC1"), control: ptr::null(), source: c!("IN1") },
    snd_soc_dapm_route { sink: c!("ADC2"), control: ptr::null(), source: c!("IN2") },
    snd_soc_dapm_route { sink: c!("ADC3"), control: ptr::null(), source: c!("IN3") },
    snd_soc_dapm_route { sink: c!("ADC0"), control: ptr::null(), source: c!("CH0 PWR") },
    snd_soc_dapm_route { sink: c!("ADC1"), control: ptr::null(), source: c!("CH1 PWR") },
    snd_soc_dapm_route { sink: c!("ADC2"), control: ptr::null(), source: c!("CH2 PWR") },
    snd_soc_dapm_route { sink: c!("ADC3"), control: ptr::null(), source: c!("CH3 PWR") },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
];

const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0x0004;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0x0005;
const SNDRV_PCM_FORMAT_MU_LAW: c_uint = 1;
const SNDRV_PCM_FORMAT_A_LAW: c_uint = 2;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_uint = 10;
const SNDRV_PCM_HW_PARAM_SAMPLE_BITS: c_uint = 8;
const SND_SOC_POSSIBLE_DAIFMT_DSP_A: u64 = 1 << 0;
const SND_SOC_POSSIBLE_DAIFMT_DSP_B: u64 = 1 << 1;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_MU_LAW: u64 = 1 << SNDRV_PCM_FORMAT_MU_LAW;
const SNDRV_PCM_FMTBIT_A_LAW: u64 = 1 << SNDRV_PCM_FORMAT_A_LAW;

unsafe extern "C" fn peb2466_dai_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint,
                                              rx_mask: c_uint, _slots: c_int, width: c_int) -> c_int {
    let peb2466 = snd_soc_component_get_drvdata((*dai).component) as *mut peb2466;
    let mut chan: c_uint;
    let mut mask: c_uint;
    let mut slot: u8;
    let ret: c_int;

    match width {
        0 => {}
        8 => {}
        _ => {
            dev_err((*dai).dev, c!("tdm slot width %d not supported\n"), width);
            return -EINVAL;
        }
    }

    mask = tx_mask;
    slot = 0;
    chan = 0;
    while mask != 0 && chan < PEB2466_NB_CHANNEL as c_uint {
        if (mask & 0x1) != 0 {
            ret = regmap_write((*peb2466).regmap, PEB2466_CR5(chan), slot as c_uint);
            if ret != 0 {
                dev_err((*dai).dev, c!("chan %d set tx tdm slot failed (%d)\n"), chan, ret);
                return ret;
            }
            chan += 1;
        }
        mask >>= 1;
        slot = slot.wrapping_add(1);
    }
    if mask != 0 {
        dev_err((*dai).dev, c!("too much tx slots defined (mask = 0x%x) support max %d\n"),
                tx_mask, PEB2466_NB_CHANNEL as c_int);
        return -EINVAL;
    }
    (*peb2466).max_chan_playback = chan as c_int;

    mask = rx_mask;
    slot = 0;
    chan = 0;
    while mask != 0 && chan < PEB2466_NB_CHANNEL as c_uint {
        if (mask & 0x1) != 0 {
            ret = regmap_write((*peb2466).regmap, PEB2466_CR4(chan), slot as c_uint);
            if ret != 0 {
                dev_err((*dai).dev, c!("chan %d set rx tdm slot failed (%d)\n"), chan, ret);
                return ret;
            }
            chan += 1;
        }
        mask >>= 1;
        slot = slot.wrapping_add(1);
    }
    if mask != 0 {
        dev_err((*dai).dev, c!("too much rx slots defined (mask = 0x%x) support max %d\n"),
                rx_mask, PEB2466_NB_CHANNEL as c_int);
        return -EINVAL;
    }
    (*peb2466).max_chan_capture = chan as c_int;

    0
}

unsafe extern "C" fn peb2466_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let peb2466 = snd_soc_component_get_drvdata((*dai).component) as *mut peb2466;
    let xr6: u8;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => xr6 = PEB2466_XR6_PCM_OFFSET(1) as u8,
        SND_SOC_DAIFMT_DSP_B => xr6 = PEB2466_XR6_PCM_OFFSET(0) as u8,
        _ => {
            dev_err((*dai).dev, c!("Unsupported format 0x%x\n"), fmt & SND_SOC_DAIFMT_FORMAT_MASK);
            return -EINVAL;
        }
    }
    regmap_write((*peb2466).regmap, PEB2466_XR6, xr6 as c_uint)
}

unsafe extern "C" fn peb2466_dai_hw_params(_substream: *mut snd_pcm_substream,
                                           params: *mut snd_pcm_hw_params,
                                           dai: *mut snd_soc_dai) -> c_int {
    let peb2466 = snd_soc_component_get_drvdata((*dai).component) as *mut peb2466;
    let mut ch: c_uint;
    let ret: c_int;
    let cr1: u8;

    match params_format(params) {
        SNDRV_PCM_FORMAT_MU_LAW => cr1 = PEB2466_CR1_LAW_MULAW as u8,
        SNDRV_PCM_FORMAT_A_LAW => cr1 = PEB2466_CR1_LAW_ALAW as u8,
        _ => {
            dev_err(&mut (*(*peb2466).spi).dev, c!("Unsupported format 0x%x\n"), params_format(params));
            return -EINVAL;
        }
    }

    ch = 0;
    while ch < PEB2466_NB_CHANNEL as c_uint {
        ret = regmap_update_bits((*peb2466).regmap, PEB2466_CR1(ch), PEB2466_CR1_LAW_MASK, cr1 as c_uint);
        if ret != 0 { return ret; }
        ch += 1;
    }

    0
}

static peb2466_sample_bits: [c_uint; 1] = [8];

static peb2466_sample_bits_constr: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: peb2466_sample_bits.as_ptr(),
    count: 1,
    mask: 0,
};

unsafe extern "C" fn peb2466_dai_startup(substream: *mut snd_pcm_substream,
                                         dai: *mut snd_soc_dai) -> c_int {
    let peb2466 = snd_soc_component_get_drvdata((*dai).component) as *mut peb2466;
    let max_ch: c_uint;
    let ret: c_int;

    max_ch = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*peb2466).max_chan_playback as c_uint
    } else {
        (*peb2466).max_chan_capture as c_uint
    };

    /*
     * Disable stream support (min = 0, max = 0) if no timeslots were
     * configured.
     */
    ret = snd_pcm_hw_constraint_minmax((*substream).runtime,
                                      SNDRV_PCM_HW_PARAM_CHANNELS,
                                      if max_ch != 0 { 1 } else { 0 }, max_ch);
    if ret < 0 {
        return ret;
    }

    snd_pcm_hw_constraint_list((*substream).runtime, 0,
                               SNDRV_PCM_HW_PARAM_SAMPLE_BITS,
                               &peb2466_sample_bits_constr)
}

static peb2466_dai_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_DSP_A |
    SND_SOC_POSSIBLE_DAIFMT_DSP_B;

static peb2466_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(peb2466_dai_startup),
    hw_params: Some(peb2466_dai_hw_params),
    set_tdm_slot: Some(peb2466_dai_set_tdm_slot),
    set_fmt: Some(peb2466_dai_set_fmt),
    auto_selectable_formats: &peb2466_dai_formats,
    num_auto_selectable_formats: 1,
};

static mut peb2466_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c!("peb2466"),
    playback: snd_soc_pcm_stream {
        stream_name: c!("Playback"),
        channels_min: 1,
        channels_max: PEB2466_NB_CHANNEL as c_uint,
        rates: SNDRV_PCM_RATE_8000,
        formats: SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c!("Capture"),
        channels_min: 1,
        channels_max: PEB2466_NB_CHANNEL as c_uint,
        rates: SNDRV_PCM_RATE_8000,
        formats: SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW,
    },
    ops: &peb2466_dai_ops,
};

unsafe extern "C" fn peb2466_reset_audio(peb2466: *mut peb2466) -> c_int {
    static reg_reset: [reg_sequence; 25] = [
        reg_sequence { reg: PEB2466_XR6,    def: 0x00 },
        reg_sequence { reg: PEB2466_CR5(0), def: 0x00 }, reg_sequence { reg: PEB2466_CR4(0), def: 0x00 },
        reg_sequence { reg: PEB2466_CR3(0), def: 0x00 }, reg_sequence { reg: PEB2466_CR2(0), def: 0x00 },
        reg_sequence { reg: PEB2466_CR1(0), def: 0x00 }, reg_sequence { reg: PEB2466_CR0(0), def: PEB2466_CR0_IMR1 },
        reg_sequence { reg: PEB2466_CR5(1), def: 0x00 }, reg_sequence { reg: PEB2466_CR4(1), def: 0x00 },
        reg_sequence { reg: PEB2466_CR3(1), def: 0x00 }, reg_sequence { reg: PEB2466_CR2(1), def: 0x00 },
        reg_sequence { reg: PEB2466_CR1(1), def: 0x00 }, reg_sequence { reg: PEB2466_CR0(1), def: PEB2466_CR0_IMR1 },
        reg_sequence { reg: PEB2466_CR5(2), def: 0x00 }, reg_sequence { reg: PEB2466_CR4(2), def: 0x00 },
        reg_sequence { reg: PEB2466_CR3(2), def: 0x00 }, reg_sequence { reg: PEB2466_CR2(2), def: 0x00 },
        reg_sequence { reg: PEB2466_CR1(2), def: 0x00 }, reg_sequence { reg: PEB2466_CR0(2), def: PEB2466_CR0_IMR1 },
        reg_sequence { reg: PEB2466_CR5(3), def: 0x00 }, reg_sequence { reg: PEB2466_CR4(3), def: 0x00 },
        reg_sequence { reg: PEB2466_CR3(3), def: 0x00 }, reg_sequence { reg: PEB2466_CR2(3), def: 0x00 },
        reg_sequence { reg: PEB2466_CR1(3), def: 0x00 }, reg_sequence { reg: PEB2466_CR0(3), def: PEB2466_CR0_IMR1 },
    ];
    static imr1_p1: [u8; 8] = [0x00, 0x90, 0x09, 0x00, 0x90, 0x09, 0x00, 0x00];
    static imr1_p2: [u8; 8] = [0x7F, 0xFF, 0x00, 0x00, 0x90, 0x14, 0x40, 0x08];
    static zero: [u8; 8] = [0; 8];
    let mut ret: c_int;
    let mut i: c_int = 0;

    while i < PEB2466_NB_CHANNEL as c_int {
        (*peb2466).ch[i as usize].tg1_freq_item = PEB2466_TONE_1000HZ;
        (*peb2466).ch[i as usize].tg2_freq_item = PEB2466_TONE_1000HZ;

        /*
         * Even if not used, disabling IM/R1 filter is not recommended.
         * Instead, we must configure it with default coefficients and
         * enable it.
         * The filter will be enabled right after (in the following
         * regmap_multi_reg_write() call).
         */
        ret = peb2466_write_buf(peb2466, PEB2466_IMR1_FILTER_P1(i as c_uint) as u8, imr1_p1.as_ptr(), 8);
        if ret != 0 { return ret; }
        ret = peb2466_write_buf(peb2466, PEB2466_IMR1_FILTER_P2(i as c_uint) as u8, imr1_p2.as_ptr(), 8);
        if ret != 0 { return ret; }

        /* Set all other filters coefficients to zero */
        ret = peb2466_write_buf(peb2466, PEB2466_TH_FILTER_P1(i as c_uint) as u8, zero.as_ptr(), 8); if ret != 0 { return ret; }
        ret = peb2466_write_buf(peb2466, PEB2466_TH_FILTER_P2(i as c_uint) as u8, zero.as_ptr(), 8); if ret != 0 { return ret; }
        ret = peb2466_write_buf(peb2466, PEB2466_TH_FILTER_P3(i as c_uint) as u8, zero.as_ptr(), 8); if ret != 0 { return ret; }
        ret = peb2466_write_buf(peb2466, PEB2466_FRX_FILTER(i as c_uint) as u8, zero.as_ptr(), 8); if ret != 0 { return ret; }
        ret = peb2466_write_buf(peb2466, PEB2466_FRR_FILTER(i as c_uint) as u8, zero.as_ptr(), 8); if ret != 0 { return ret; }
        ret = peb2466_write_buf(peb2466, PEB2466_AX_FILTER(i as c_uint) as u8, zero.as_ptr(), 4); if ret != 0 { return ret; }
        ret = peb2466_write_buf(peb2466, PEB2466_AR_FILTER(i as c_uint) as u8, zero.as_ptr(), 4); if ret != 0 { return ret; }
        i += 1;
    }

    regmap_multi_reg_write((*peb2466).regmap, reg_reset.as_ptr(), reg_reset.len() as c_uint)
}

unsafe fn get_unaligned_be16(p: *const u8) -> u16 {
    ((*p as u16) << 8) | (*p.add(1) as u16)
}

unsafe fn get_unaligned_be32(p: *const u8) -> u32 {
    ((*p as u32) << 24) | ((*p.add(1) as u32) << 16) | ((*p.add(2) as u32) << 8) | (*p.add(3) as u32)
}

unsafe extern "C" fn peb2466_fw_parse_thfilter(component: *mut snd_soc_component,
                                               _tag: u16, lng: u32, data: *const u8) -> c_int {
    let peb2466 = snd_soc_component_get_drvdata(component) as *mut peb2466;
    let mask: u8;
    let mut ret: c_int;
    let mut i: c_int;

    dev_info((*component).dev, c!("fw TH filter: mask %x, %*phN\n"), *data as c_uint,
             (lng - 1) as c_int, data.add(1));

    /*
     * TH_FILTER TLV data:
     *   - @0  1 byte:  Chan mask (bit set means related channel is concerned)
     *   - @1  8 bytes: TH-Filter coefficients part1
     *   - @9  8 bytes: TH-Filter coefficients part2
     *   - @17 8 bytes: TH-Filter coefficients part3
     */
    mask = *data;
    i = 0;
    while i < PEB2466_NB_CHANNEL as c_int {
        if (mask & (1 << i)) == 0 { i += 1; continue; }
        ret = regmap_update_bits((*peb2466).regmap, PEB2466_CR0(i as c_uint), PEB2466_CR0_TH, 0); if ret != 0 { return ret; }
        ret = peb2466_write_buf(peb2466, PEB2466_TH_FILTER_P1(i as c_uint) as u8, data.add(1), 8); if ret != 0 { return ret; }
        ret = peb2466_write_buf(peb2466, PEB2466_TH_FILTER_P2(i as c_uint) as u8, data.add(9), 8); if ret != 0 { return ret; }
        ret = peb2466_write_buf(peb2466, PEB2466_TH_FILTER_P3(i as c_uint) as u8, data.add(17), 8); if ret != 0 { return ret; }
        ret = regmap_update_bits((*peb2466).regmap, PEB2466_CR0(i as c_uint),
                                 PEB2466_CR0_TH | PEB2466_CR0_THSEL_MASK,
                                 PEB2466_CR0_TH | PEB2466_CR0_THSEL(i as c_uint));
        if ret != 0 { return ret; }
        i += 1;
    }
    0
}

macro_rules! filter_parser {
    ($name:ident, $label:literal, $comment:literal, $bit:expr, $cmd:ident, $len:expr) => {
        unsafe extern "C" fn $name(component: *mut snd_soc_component,
                                   _tag: u16, lng: u32, data: *const u8) -> c_int {
            let peb2466 = snd_soc_component_get_drvdata(component) as *mut peb2466;
            let mask: u8;
            let mut ret: c_int;
            let mut i: c_int;

            dev_info((*component).dev, c!($label), *data as c_uint, (lng - 1) as c_int, data.add(1));
            let _ = $comment;
            mask = *data;
            i = 0;
            while i < PEB2466_NB_CHANNEL as c_int {
                if (mask & (1 << i)) == 0 { i += 1; continue; }
                ret = regmap_update_bits((*peb2466).regmap, PEB2466_CR0(i as c_uint), $bit, 0); if ret != 0 { return ret; }
                ret = peb2466_write_buf(peb2466, $cmd(i as c_uint) as u8, data.add(1), $len); if ret != 0 { return ret; }
                ret = regmap_update_bits((*peb2466).regmap, PEB2466_CR0(i as c_uint), $bit, $bit); if ret != 0 { return ret; }
                i += 1;
            }
            0
        }
    };
}

unsafe extern "C" fn peb2466_fw_parse_imr1filter(component: *mut snd_soc_component,
                                                 _tag: u16, lng: u32, data: *const u8) -> c_int {
    let peb2466 = snd_soc_component_get_drvdata(component) as *mut peb2466;
    let mask: u8;
    let mut ret: c_int;
    let mut i: c_int;

    dev_info((*component).dev, c!("fw IM/R1 filter: mask %x, %*phN\n"), *data as c_uint,
             (lng - 1) as c_int, data.add(1));

    /*
     * IMR1_FILTER TLV data:
     *   - @0 1 byte:  Chan mask (bit set means related channel is concerned)
     *   - @1 8 bytes: IM/R1-Filter coefficients part1
     *   - @9 8 bytes: IM/R1-Filter coefficients part2
     */
    mask = *data;
    i = 0;
    while i < PEB2466_NB_CHANNEL as c_int {
        if (mask & (1 << i)) == 0 { i += 1; continue; }
        ret = regmap_update_bits((*peb2466).regmap, PEB2466_CR0(i as c_uint), PEB2466_CR0_IMR1, 0); if ret != 0 { return ret; }
        ret = peb2466_write_buf(peb2466, PEB2466_IMR1_FILTER_P1(i as c_uint) as u8, data.add(1), 8); if ret != 0 { return ret; }
        ret = peb2466_write_buf(peb2466, PEB2466_IMR1_FILTER_P2(i as c_uint) as u8, data.add(9), 8); if ret != 0 { return ret; }
        ret = regmap_update_bits((*peb2466).regmap, PEB2466_CR0(i as c_uint), PEB2466_CR0_IMR1, PEB2466_CR0_IMR1); if ret != 0 { return ret; }
        i += 1;
    }
    0
}

filter_parser!(peb2466_fw_parse_frxfilter, "fw FRX filter: mask %x, %*phN\n",
               "FRX_FILTER TLV data: mask then 8 bytes FRX-Filter coefficients", PEB2466_CR0_FRX, PEB2466_FRX_FILTER, 8);
filter_parser!(peb2466_fw_parse_frrfilter, "fw FRR filter: mask %x, %*phN\n",
               "FRR_FILTER TLV data: mask then 8 bytes FRR-Filter coefficients", PEB2466_CR0_FRR, PEB2466_FRR_FILTER, 8);
filter_parser!(peb2466_fw_parse_axfilter, "fw AX filter: mask %x, %*phN\n",
               "AX_FILTER TLV data: mask then 4 bytes AX-Filter coefficients", PEB2466_CR0_AX, PEB2466_AX_FILTER, 4);
filter_parser!(peb2466_fw_parse_arfilter, "fw AR filter: mask %x, %*phN\n",
               "AR_FILTER TLV data: mask then 4 bytes AR-Filter coefficients", PEB2466_CR0_AR, PEB2466_AR_FILTER, 4);

static peb2466_ax_ctrl_names: [*const c_char; 4] = [
    c!("ADC0 Capture Volume"), c!("ADC1 Capture Volume"),
    c!("ADC2 Capture Volume"), c!("ADC3 Capture Volume"),
];

unsafe extern "C" fn peb2466_fw_parse_table(component: *mut snd_soc_component, lng: u32, data: *const u8,
                                            is_ax: bool) -> c_int {
    let peb2466 = snd_soc_component_get_drvdata(component) as *mut peb2466;
    let mut lkup_ctrl: *mut peb2466_lkup_ctrl;
    let mut lookup: *mut peb2466_lookup;
    let table: *mut [u8; 4];
    let table_size: u32;
    let init_index: u32;
    let min_val: s32;
    let step: s32;
    let mask: u8;
    let mut ret: c_int;
    let mut i: c_int;

    if lng < 13 || ((lng - 13) % 4) != 0 {
        dev_err((*component).dev, if is_ax { c!("fw AX table lng %u invalid\n") } else { c!("fw AR table lng %u invalid\n") }, lng);
        return -EINVAL;
    }
    table_size = lng - 13;

    min_val = get_unaligned_be32(data.add(1)) as s32;
    step = get_unaligned_be32(data.add(5)) as s32;
    init_index = get_unaligned_be32(data.add(9));
    if init_index >= table_size / 4 {
        dev_err((*component).dev,
                if is_ax { c!("fw AX table index %u out of table[%u]\n") } else { c!("fw AR table index %u out of table[%u]\n") },
                init_index, table_size / 4);
        return -EINVAL;
    }

    dev_info((*component).dev,
             if is_ax { c!("fw AX table: mask %x, min %d, step %d, %u items, tbl[%u] %*phN\n") }
             else { c!("fw AR table: mask %x, min %d, step %d, %u items, tbl[%u] %*phN\n") },
             *data as c_uint, min_val, step, table_size / 4, init_index, 4, data.add(13 + (init_index * 4) as usize));

    /* BUILD_BUG_ON(sizeof(*table) != 4); */
    table = devm_kzalloc(&mut (*(*peb2466).spi).dev, table_size as size_t, GFP_KERNEL) as *mut [u8; 4];
    if table.is_null() {
        return -ENOMEM;
    }
    ptr::copy_nonoverlapping(data.add(13), table as *mut u8, table_size as usize);

    mask = *data;
    i = 0;
    while i < PEB2466_NB_CHANNEL as c_int {
        if (mask & (1 << i)) == 0 { i += 1; continue; }

        if is_ax {
            lookup = &mut (*peb2466).ch[i as usize].ax_lookup;
        } else {
            lookup = &mut (*peb2466).ch[i as usize].ar_lookup;
        }
        (*lookup).table = table;
        (*lookup).count = table_size / 4;

        ret = regmap_update_bits((*peb2466).regmap, PEB2466_CR0(i as c_uint),
                                 if is_ax { PEB2466_CR0_AX } else { PEB2466_CR0_AR }, 0);
        if ret != 0 { return ret; }

        ret = peb2466_write_buf(peb2466,
                                if is_ax { PEB2466_AX_FILTER(i as c_uint) as u8 } else { PEB2466_AR_FILTER(i as c_uint) as u8 },
                                table.add(init_index as usize) as *const u8, 4);
        if ret != 0 { return ret; }

        ret = regmap_update_bits((*peb2466).regmap, PEB2466_CR0(i as c_uint),
                                 if is_ax { PEB2466_CR0_AX } else { PEB2466_CR0_AR },
                                 if is_ax { PEB2466_CR0_AX } else { PEB2466_CR0_AR });
        if ret != 0 { return ret; }

        if is_ax {
            lkup_ctrl = &mut (*peb2466).ch[i as usize].ax_lkup_ctrl;
            (*lkup_ctrl).reg = PEB2466_AX_FILTER(i as c_uint) as c_int;
            ret = peb2466_add_lkup_ctrl(component, lkup_ctrl, peb2466_ax_ctrl_names[i as usize], min_val, step);
        } else {
            lkup_ctrl = &mut (*peb2466).ch[i as usize].ar_lkup_ctrl;
            (*lkup_ctrl).reg = PEB2466_AR_FILTER(i as c_uint) as c_int;
            ret = peb2466_add_lkup_ctrl(component, lkup_ctrl, peb2466_ar_ctrl_names[i as usize], min_val, step);
        }
        (*lkup_ctrl).lookup = lookup;
        (*lkup_ctrl).index = init_index;
        if ret != 0 { return ret; }
        i += 1;
    }
    0
}

unsafe extern "C" fn peb2466_fw_parse_axtable(component: *mut snd_soc_component,
                                              _tag: u16, lng: u32, data: *const u8) -> c_int {
    /*
     * AX_TABLE TLV data:
     *   - @0 1 byte:  Chan mask (bit set means related channel is concerned)
     *   - @1 32bits signed: Min table value in centi dB (MinVal)
     *                       ie -300 means -3.0 dB
     *   - @5 32bits signed: Step from on item to other item in centi dB (Step)
     *                       ie 25 means 0.25 dB)
     *   - @9 32bits unsigned: Item index in the table to use for the initial
     *                         value
     *   - @13 N*4 bytes: Table composed of 4 bytes items.
     *                    Each item correspond to an AX filter value.
     *
     * The conversion from raw value item in the table to/from the value in
     * dB is: Raw value at index i <-> (MinVal + i * Step) in centi dB.
     */
    peb2466_fw_parse_table(component, lng, data, true)
}

static peb2466_ar_ctrl_names: [*const c_char; 4] = [
    c!("DAC0 Playback Volume"), c!("DAC1 Playback Volume"),
    c!("DAC2 Playback Volume"), c!("DAC3 Playback Volume"),
];

unsafe extern "C" fn peb2466_fw_parse_artable(component: *mut snd_soc_component,
                                              _tag: u16, lng: u32, data: *const u8) -> c_int {
    /*
     * AR_TABLE TLV data:
     *   - @0 1 byte:  Chan mask (bit set means related channel is concerned)
     *   - @1 32bits signed: Min table value in centi dB (MinVal)
     *                       ie -300 means -3.0 dB
     *   - @5 32bits signed: Step from on item to other item in centi dB (Step)
     *                       ie 25 means 0.25 dB)
     *   - @9 32bits unsigned: Item index in the table to use for the initial
     *                         value
     *   - @13 N*4 bytes: Table composed of 4 bytes items.
     *                    Each item correspond to an AR filter value.
     *
     * The conversion from raw value item in the table to/from the value in
     * dB is: Raw value at index i <-> (MinVal + i * Step) in centi dB.
     */
    peb2466_fw_parse_table(component, lng, data, false)
}

#[repr(C)]
pub struct peb2466_fw_tag_def {
    pub tag: u16,
    pub lng_min: u32,
    pub lng_max: u32,
    pub parse: Option<unsafe extern "C" fn(*mut snd_soc_component, u16, u32, *const u8) -> c_int>,
}

const fn PEB2466_TAG_DEF_LNG_EQ(__tag: u16, __lng: u32,
                                __parse: unsafe extern "C" fn(*mut snd_soc_component, u16, u32, *const u8) -> c_int) -> peb2466_fw_tag_def {
    peb2466_fw_tag_def { tag: __tag, lng_min: __lng, lng_max: __lng, parse: Some(__parse) }
}

const fn PEB2466_TAG_DEF_LNG_MIN(__tag: u16, __lng_min: u32,
                                 __parse: unsafe extern "C" fn(*mut snd_soc_component, u16, u32, *const u8) -> c_int) -> peb2466_fw_tag_def {
    peb2466_fw_tag_def { tag: __tag, lng_min: __lng_min, lng_max: U32_MAX, parse: Some(__parse) }
}

static peb2466_fw_tag_defs: [peb2466_fw_tag_def; 8] = [
    /* TH FILTER */
    PEB2466_TAG_DEF_LNG_EQ(0x0001, 1 + 3 * 8, peb2466_fw_parse_thfilter),
    /* IMR1 FILTER */
    PEB2466_TAG_DEF_LNG_EQ(0x0002, 1 + 2 * 8, peb2466_fw_parse_imr1filter),
    /* FRX FILTER */
    PEB2466_TAG_DEF_LNG_EQ(0x0003, 1 + 8, peb2466_fw_parse_frxfilter),
    /* FRR FILTER */
    PEB2466_TAG_DEF_LNG_EQ(0x0004, 1 + 8, peb2466_fw_parse_frrfilter),
    /* AX FILTER */
    PEB2466_TAG_DEF_LNG_EQ(0x0005, 1 + 4, peb2466_fw_parse_axfilter),
    /* AR FILTER */
    PEB2466_TAG_DEF_LNG_EQ(0x0006, 1 + 4, peb2466_fw_parse_arfilter),
    /* AX TABLE */
    PEB2466_TAG_DEF_LNG_MIN(0x0105, 1 + 3 * 4, peb2466_fw_parse_axtable),
    /* AR TABLE */
    PEB2466_TAG_DEF_LNG_MIN(0x0106, 1 + 3 * 4, peb2466_fw_parse_artable),
];

unsafe fn peb2466_fw_get_tag_def(tag: u16) -> *const peb2466_fw_tag_def {
    let mut i: c_int = 0;

    while i < peb2466_fw_tag_defs.len() as c_int {
        if peb2466_fw_tag_defs[i as usize].tag == tag {
            return &peb2466_fw_tag_defs[i as usize];
        }
        i += 1;
    }
    ptr::null()
}

unsafe extern "C" fn peb2466_fw_parse(component: *mut snd_soc_component,
                                      data: *const u8, size: size_t) -> c_int {
    let mut tag_def: *const peb2466_fw_tag_def;
    let mut left: size_t;
    let mut buf: *const u8;
    let mut val16: u16;
    let tag: u16;
    let lng: u32;
    let ret: c_int;

    /*
     * Coefficients firmware binary structure (16bits and 32bits are
     * big-endian values).
     *
     * @0, 16bits: Magic (0x2466)
     * @2, 16bits: Version (0x0100 for version 1.0)
     * @4, 2+4+N bytes: TLV block
     * @4+(2+4+N) bytes: Next TLV block
     * ...
     *
     * Detail of a TLV block:
     *   @0, 16bits: Tag
     *   @2, 32bits: Lng
     *   @6, lng bytes: Data
     *
     * The detail the Data for a given TLV Tag is provided in the related
     * parser.
     */

    left = size;
    buf = data;

    if left < 4 {
        dev_err((*component).dev, c!("fw size %zu, exp at least 4\n"), left);
        return -EINVAL;
    }

    /* Check magic */
    val16 = get_unaligned_be16(buf);
    if val16 != 0x2466 {
        dev_err((*component).dev, c!("fw magic 0x%04x exp 0x2466\n"), val16 as c_uint);
        return -EINVAL;
    }
    buf = buf.add(2);
    left -= 2;

    /* Check version */
    val16 = get_unaligned_be16(buf);
    if val16 != 0x0100 {
        dev_err((*component).dev, c!("fw magic 0x%04x exp 0x0100\n"), val16 as c_uint);
        return -EINVAL;
    }
    buf = buf.add(2);
    left -= 2;

    while left != 0 {
        if left < 6 {
            dev_err((*component).dev, c!("fw %td/%zu left %zu, exp at least 6\n"),
                    buf.offset_from(data), size, left);
            return -EINVAL;
        }
        /* Check tag and lng */
        tag = get_unaligned_be16(buf);
        lng = get_unaligned_be32(buf.add(2));
        tag_def = peb2466_fw_get_tag_def(tag);
        if tag_def.is_null() {
            dev_err((*component).dev, c!("fw %td/%zu tag 0x%04x unknown\n"),
                    buf.offset_from(data), size, tag as c_uint);
            return -EINVAL;
        }
        if lng < (*tag_def).lng_min || lng > (*tag_def).lng_max {
            dev_err((*component).dev, c!("fw %td/%zu tag 0x%04x lng %u, exp [%u;%u]\n"),
                    buf.offset_from(data), size, tag as c_uint, lng, (*tag_def).lng_min, (*tag_def).lng_max);
            return -EINVAL;
        }
        buf = buf.add(6);
        left -= 6;
        if left < lng as size_t {
            dev_err((*component).dev, c!("fw %td/%zu tag 0x%04x lng %u, left %zu\n"),
                    buf.offset_from(data), size, tag as c_uint, lng, left);
            return -EINVAL;
        }

        /* TLV block is valid -> parse the data part */
        ret = ((*tag_def).parse.unwrap())(component, tag, lng, buf);
        if ret != 0 {
            dev_err((*component).dev, c!("fw %td/%zu tag 0x%04x lng %u parse failed\n"),
                    buf.offset_from(data), size, tag as c_uint, lng);
            return ret;
        }

        buf = buf.add(lng as usize);
        left -= lng as size_t;
    }
    0
}

unsafe extern "C" fn peb2466_load_coeffs(component: *mut snd_soc_component, fw_name: *const c_char) -> c_int {
    let mut fw: *const firmware = ptr::null();
    let ret: c_int;

    ret = request_firmware(&mut fw, fw_name, (*component).dev);
    if ret != 0 {
        return ret;
    }

    let parse_ret = peb2466_fw_parse(component, (*fw).data, (*fw).size);
    release_firmware(fw);
    parse_ret
}

unsafe extern "C" fn peb2466_component_probe(component: *mut snd_soc_component) -> c_int {
    let peb2466 = snd_soc_component_get_drvdata(component) as *mut peb2466;
    let mut firmware_name: *const c_char = ptr::null();
    let mut ret: c_int;

    /* reset peb2466 audio part */
    ret = peb2466_reset_audio(peb2466);
    if ret != 0 {
        return ret;
    }

    ret = of_property_read_string((*(*peb2466).spi).dev.of_node,
                                  c!("firmware-name"), &mut firmware_name);
    if ret != 0 {
        return if ret == -EINVAL { 0 } else { ret };
    }

    peb2466_load_coeffs(component, firmware_name)
}

static peb2466_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(peb2466_component_probe),
    controls: peb2466_controls.as_ptr(),
    num_controls: 0,
    dapm_widgets: peb2466_dapm_widgets.as_ptr(),
    num_dapm_widgets: 0,
    dapm_routes: peb2466_dapm_routes.as_ptr(),
    num_dapm_routes: peb2466_dapm_routes.len() as c_uint,
    endianness: 1,
};

/*
 * The mapping used for the relationship between the gpio offset and the
 * physical pin is the following:
 *
 * offset     pin
 *      0     SI1_0
 *      1     SI1_1
 *      2     SI2_0
 *      3     SI2_1
 *      4     SI3_0
 *      5     SI3_1
 *      6     SI4_0
 *      7     SI4_1
 *      8     SO1_0
 *      9     SO1_1
 *     10     SO2_0
 *     11     SO2_1
 *     12     SO3_0
 *     13     SO3_1
 *     14     SO4_0
 *     15     SO4_1
 *     16     SB1_0
 *     17     SB1_1
 *     18     SB2_0
 *     19     SB2_1
 *     20     SB3_0
 *     21     SB3_1
 *     22     SB4_0
 *     23     SB4_1
 *     24     SB1_2
 *     25     SB2_2
 *     26     SB3_2
 *     27     SB4_2
 */

unsafe extern "C" fn peb2466_chip_gpio_offset_to_data_regmask(offset: c_uint,
                                                              xr_reg: *mut c_uint,
                                                              mask: *mut c_uint) -> c_int {
    if offset < 16 {
        /*
         * SIx_{0,1} and SOx_{0,1}
         *   Read accesses read SIx_{0,1} values
         *   Write accesses write SOx_{0,1} values
         */
        *xr_reg = PEB2466_XR0;
        *mask = 1 << (offset % 8);
        return 0;
    }
    if offset < 24 {
        /* SBx_{0,1} */
        *xr_reg = PEB2466_XR1;
        *mask = 1 << (offset - 16);
        return 0;
    }
    if offset < 28 {
        /* SBx_2 */
        *xr_reg = PEB2466_XR3;
        *mask = 1 << (offset - 24 + 4);
        return 0;
    }
    -EINVAL
}

unsafe extern "C" fn peb2466_chip_gpio_offset_to_dir_regmask(offset: c_uint,
                                                             xr_reg: *mut c_uint,
                                                             mask: *mut c_uint) -> c_int {
    if offset < 16 {
        /* Direction cannot be changed for these GPIOs */
        return -EINVAL;
    }
    if offset < 24 {
        *xr_reg = PEB2466_XR2;
        *mask = 1 << (offset - 16);
        return 0;
    }
    if offset < 28 {
        *xr_reg = PEB2466_XR3;
        *mask = 1 << (offset - 24);
        return 0;
    }
    -EINVAL
}

unsafe extern "C" fn peb2466_chip_gpio_get_cache(peb2466: *mut peb2466,
                                                 xr_reg: c_uint) -> *mut c_uint {
    match xr_reg {
        PEB2466_XR0 => &mut (*peb2466).gpio.cache.xr0,
        PEB2466_XR1 => &mut (*peb2466).gpio.cache.xr1,
        PEB2466_XR2 => &mut (*peb2466).gpio.cache.xr2,
        PEB2466_XR3 => &mut (*peb2466).gpio.cache.xr3,
        _ => ptr::null_mut(),
    }
}

unsafe extern "C" fn peb2466_chip_gpio_update_bits(peb2466: *mut peb2466, xr_reg: c_uint,
                                                   mask: c_uint, val: c_uint) -> c_int {
    let mut tmp: c_uint;
    let cache: *mut c_uint;
    let ret: c_int;

    /*
     * Read and write accesses use different peb2466 internal signals (input
     * signals on reads and output signals on writes). regmap_update_bits
     * cannot be used to read/modify/write the value.
     * So, a specific cache value is used.
     */

    mutex_lock(&mut (*peb2466).gpio.lock);

    cache = peb2466_chip_gpio_get_cache(peb2466, xr_reg);
    if cache.is_null() {
        mutex_unlock(&mut (*peb2466).gpio.lock);
        return -EINVAL;
    }

    tmp = *cache;
    tmp &= !mask;
    tmp |= val;

    ret = regmap_write((*peb2466).regmap, xr_reg, tmp);
    if ret != 0 {
        mutex_unlock(&mut (*peb2466).gpio.lock);
        return ret;
    }

    *cache = tmp;

    mutex_unlock(&mut (*peb2466).gpio.lock);
    0
}

unsafe extern "C" fn peb2466_chip_gpio_set(c: *mut gpio_chip, offset: c_uint,
                                           val: c_int) -> c_int {
    let peb2466 = gpiochip_get_data(c) as *mut peb2466;
    let mut xr_reg: c_uint = 0;
    let mut mask: c_uint = 0;
    let mut ret: c_int;

    if offset < 8 {
        /*
         * SIx_{0,1} signals cannot be set and writing the related
         * register will change the SOx_{0,1} signals
         */
        dev_warn(&mut (*(*peb2466).spi).dev, c!("cannot set gpio %d (read-only)\n"), offset);
        return -EINVAL;
    }

    ret = peb2466_chip_gpio_offset_to_data_regmask(offset, &mut xr_reg, &mut mask);
    if ret != 0 {
        dev_err(&mut (*(*peb2466).spi).dev, c!("cannot set gpio %d (%d)\n"), offset, ret);
        return ret;
    }

    ret = peb2466_chip_gpio_update_bits(peb2466, xr_reg, mask, if val != 0 { mask } else { 0 });
    if ret != 0 {
        dev_err(&mut (*(*peb2466).spi).dev, c!("set gpio %d (0x%x, 0x%x) failed (%d)\n"),
                offset, xr_reg, mask, ret);
    }

    ret
}

unsafe extern "C" fn peb2466_chip_gpio_get(c: *mut gpio_chip, offset: c_uint) -> c_int {
    let peb2466 = gpiochip_get_data(c) as *mut peb2466;
    let mut use_cache = false;
    let cache: *mut c_uint;
    let mut xr_reg: c_uint = 0;
    let mut mask: c_uint = 0;
    let mut val: c_uint = 0;
    let mut ret: c_int;

    if offset >= 8 && offset < 16 {
        /*
         * SOx_{0,1} signals cannot be read. Reading the related
         * register will read the SIx_{0,1} signals.
         * Use the cache to get value;
         */
        use_cache = true;
    }

    ret = peb2466_chip_gpio_offset_to_data_regmask(offset, &mut xr_reg, &mut mask);
    if ret != 0 {
        dev_err(&mut (*(*peb2466).spi).dev, c!("cannot get gpio %d (%d)\n"), offset, ret);
        return -EINVAL;
    }

    if use_cache {
        cache = peb2466_chip_gpio_get_cache(peb2466, xr_reg);
        if cache.is_null() {
            return -EINVAL;
        }
        val = *cache;
    } else {
        ret = regmap_read((*peb2466).regmap, xr_reg, &mut val);
        if ret != 0 {
            dev_err(&mut (*(*peb2466).spi).dev, c!("get gpio %d (0x%x, 0x%x) failed (%d)\n"),
                    offset, xr_reg, mask, ret);
            return ret;
        }
    }

    if (val & mask) != 0 { 1 } else { 0 }
}

unsafe extern "C" fn peb2466_chip_get_direction(c: *mut gpio_chip, offset: c_uint) -> c_int {
    let peb2466 = gpiochip_get_data(c) as *mut peb2466;
    let mut xr_reg: c_uint = 0;
    let mut mask: c_uint = 0;
    let mut val: c_uint = 0;
    let ret: c_int;

    if offset < 8 {
        /* SIx_{0,1} */
        return GPIO_LINE_DIRECTION_IN;
    }
    if offset < 16 {
        /* SOx_{0,1} */
        return GPIO_LINE_DIRECTION_OUT;
    }

    ret = peb2466_chip_gpio_offset_to_dir_regmask(offset, &mut xr_reg, &mut mask);
    if ret != 0 {
        dev_err(&mut (*(*peb2466).spi).dev, c!("cannot get gpio %d direction (%d)\n"), offset, ret);
        return ret;
    }

    let r = regmap_read((*peb2466).regmap, xr_reg, &mut val);
    if r != 0 {
        dev_err(&mut (*(*peb2466).spi).dev, c!("get dir gpio %d (0x%x, 0x%x) failed (%d)\n"),
                offset, xr_reg, mask, r);
        return r;
    }

    if (val & mask) != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe extern "C" fn peb2466_chip_direction_input(c: *mut gpio_chip, offset: c_uint) -> c_int {
    let peb2466 = gpiochip_get_data(c) as *mut peb2466;
    let mut xr_reg: c_uint = 0;
    let mut mask: c_uint = 0;
    let mut ret: c_int;

    if offset < 8 {
        /* SIx_{0,1} */
        return 0;
    }
    if offset < 16 {
        /* SOx_{0,1} */
        return -EINVAL;
    }

    ret = peb2466_chip_gpio_offset_to_dir_regmask(offset, &mut xr_reg, &mut mask);
    if ret != 0 {
        dev_err(&mut (*(*peb2466).spi).dev, c!("cannot set gpio %d direction (%d)\n"), offset, ret);
        return ret;
    }

    ret = peb2466_chip_gpio_update_bits(peb2466, xr_reg, mask, 0);
    if ret != 0 {
        dev_err(&mut (*(*peb2466).spi).dev, c!("Set dir in gpio %d (0x%x, 0x%x) failed (%d)\n"),
                offset, xr_reg, mask, ret);
        return ret;
    }

    0
}

unsafe extern "C" fn peb2466_chip_direction_output(c: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    let peb2466 = gpiochip_get_data(c) as *mut peb2466;
    let mut xr_reg: c_uint = 0;
    let mut mask: c_uint = 0;
    let mut ret: c_int;

    if offset < 8 {
        /* SIx_{0,1} */
        return -EINVAL;
    }

    ret = peb2466_chip_gpio_set(c, offset, val);
    if ret != 0 {
        return ret;
    }

    if offset < 16 {
        /* SOx_{0,1} */
        return 0;
    }

    ret = peb2466_chip_gpio_offset_to_dir_regmask(offset, &mut xr_reg, &mut mask);
    if ret != 0 {
        dev_err(&mut (*(*peb2466).spi).dev, c!("cannot set gpio %d direction (%d)\n"), offset, ret);
        return ret;
    }

    ret = peb2466_chip_gpio_update_bits(peb2466, xr_reg, mask, mask);
    if ret != 0 {
        dev_err(&mut (*(*peb2466).spi).dev, c!("Set dir in gpio %d (0x%x, 0x%x) failed (%d)\n"),
                offset, xr_reg, mask, ret);
        return ret;
    }

    0
}

unsafe extern "C" fn peb2466_reset_gpio(peb2466: *mut peb2466) -> c_int {
    static reg_reset: [reg_sequence; 4] = [
        /* Output pins at 0, input/output pins as input */
        reg_sequence { reg: PEB2466_XR0, def: 0 },
        reg_sequence { reg: PEB2466_XR1, def: 0 },
        reg_sequence { reg: PEB2466_XR2, def: 0 },
        reg_sequence { reg: PEB2466_XR3, def: 0 },
    ];

    (*peb2466).gpio.cache.xr0 = 0;
    (*peb2466).gpio.cache.xr1 = 0;
    (*peb2466).gpio.cache.xr2 = 0;
    (*peb2466).gpio.cache.xr3 = 0;

    regmap_multi_reg_write((*peb2466).regmap, reg_reset.as_ptr(), reg_reset.len() as c_uint)
}

unsafe extern "C" fn peb2466_gpio_init(peb2466: *mut peb2466) -> c_int {
    let ret: c_int;

    mutex_init(&mut (*peb2466).gpio.lock);

    ret = peb2466_reset_gpio(peb2466);
    if ret != 0 {
        return ret;
    }

    (*peb2466).gpio.gpio_chip.owner = THIS_MODULE;
    (*peb2466).gpio.gpio_chip.label = dev_name(&mut (*(*peb2466).spi).dev);
    (*peb2466).gpio.gpio_chip.parent = &mut (*(*peb2466).spi).dev;
    (*peb2466).gpio.gpio_chip.base = -1;
    (*peb2466).gpio.gpio_chip.ngpio = 28;
    (*peb2466).gpio.gpio_chip.get_direction = Some(peb2466_chip_get_direction);
    (*peb2466).gpio.gpio_chip.direction_input = Some(peb2466_chip_direction_input);
    (*peb2466).gpio.gpio_chip.direction_output = Some(peb2466_chip_direction_output);
    (*peb2466).gpio.gpio_chip.get = Some(peb2466_chip_gpio_get);
    (*peb2466).gpio.gpio_chip.set = Some(peb2466_chip_gpio_set);
    (*peb2466).gpio.gpio_chip.can_sleep = true;

    devm_gpiochip_add_data(&mut (*(*peb2466).spi).dev, &mut (*peb2466).gpio.gpio_chip,
                           peb2466 as *mut c_void)
}

unsafe extern "C" fn peb2466_spi_probe(spi: *mut spi_device) -> c_int {
    let peb2466: *mut peb2466;
    let mclk_rate: c_ulong;
    let mut ret: c_int;
    let xr5: u8;

    (*spi).bits_per_word = 8;
    ret = spi_setup(spi);
    if ret < 0 {
        return ret;
    }

    peb2466 = devm_kzalloc(&mut (*spi).dev, size_of::<peb2466>(), GFP_KERNEL) as *mut peb2466;
    if peb2466.is_null() {
        return -ENOMEM;
    }

    (*peb2466).spi = spi;

    (*peb2466).regmap = devm_regmap_init(&mut (*peb2466).spi.as_mut().unwrap().dev, ptr::null(), peb2466 as *mut c_void,
                                         &peb2466_regmap_config);
    if IS_ERR((*peb2466).regmap as *const c_void) {
        return PTR_ERR((*peb2466).regmap as *const c_void);
    }

    (*peb2466).reset_gpio = devm_gpiod_get_optional(&mut (*(*peb2466).spi).dev,
                                                    c!("reset"), GPIOD_OUT_LOW);
    if IS_ERR((*peb2466).reset_gpio as *const c_void) {
        return PTR_ERR((*peb2466).reset_gpio as *const c_void);
    }

    (*peb2466).mclk = devm_clk_get_enabled(&mut (*(*peb2466).spi).dev, c!("mclk"));
    if IS_ERR((*peb2466).mclk as *const c_void) {
        return PTR_ERR((*peb2466).mclk as *const c_void);
    }

    if !(*peb2466).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*peb2466).reset_gpio, 1);
        udelay(4);
        gpiod_set_value_cansleep((*peb2466).reset_gpio, 0);
        udelay(4);
    }

    spi_set_drvdata(spi, peb2466 as *mut c_void);

    mclk_rate = clk_get_rate((*peb2466).mclk);
    match mclk_rate {
        1536000 => xr5 = PEB2466_XR5_MCLK_1536 as u8,
        2048000 => xr5 = PEB2466_XR5_MCLK_2048 as u8,
        4096000 => xr5 = PEB2466_XR5_MCLK_4096 as u8,
        8192000 => xr5 = PEB2466_XR5_MCLK_8192 as u8,
        _ => {
            dev_err(&mut (*(*peb2466).spi).dev, c!("Unsupported clock rate %lu\n"), mclk_rate);
            ret = -EINVAL;
            return ret;
        }
    }
    ret = regmap_write((*peb2466).regmap, PEB2466_XR5, xr5 as c_uint);
    if ret != 0 {
        dev_err(&mut (*(*peb2466).spi).dev, c!("Setting MCLK failed (%d)\n"), ret);
        return ret;
    }

    ret = devm_snd_soc_register_component(&mut (*spi).dev, &peb2466_component_driver,
                                          &mut peb2466_dai_driver, 1);
    if ret != 0 {
        return ret;
    }

    if CONFIG_GPIOLIB {
        ret = peb2466_gpio_init(peb2466);
        if ret != 0 {
            return ret;
        }
    }

    0
}

static peb2466_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c!("infineon,peb2466") },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, peb2466_of_match); */

static peb2466_id_table: [spi_device_id; 2] = [
    spi_device_id { name: c!("peb2466"), driver_data: 0 },
    spi_device_id { name: ptr::null(), driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(spi, peb2466_id_table); */

static mut peb2466_spi_driver: spi_driver = spi_driver {
    driver: driver_inner {
        name: c!("peb2466"),
        of_match_table: peb2466_of_match.as_ptr(),
    },
    id_table: peb2466_id_table.as_ptr(),
    probe: Some(peb2466_spi_probe),
};

/* module_spi_driver(peb2466_spi_driver); */

/* MODULE_AUTHOR("Herve Codina <herve.codina@bootlin.com>"); */
/* MODULE_DESCRIPTION("PEB2466 ALSA SoC driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
