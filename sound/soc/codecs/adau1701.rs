// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for ADAU1701 SigmaDSP processor
 *
 * Copyright 2011 Analog Devices Inc.
 * Author: Lars-Peter Clausen <lars@metafoo.de>
 *	based on an inital version by Cliff Cai <cliff.cai@analog.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const I2C_M_RD: u16 = 0x0001;
const GPIOD_IN: c_int = 0;
const GPIOD_OUT_LOW: c_int = 0;
const REGCACHE_MAPLE: c_uint = 0;

const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 0;

const SNDRV_PCM_STREAM_PLAYBACK: c_uint = 0;
const SNDRV_PCM_RATE_48000: c_uint = 0;
const SNDRV_PCM_RATE_96000: c_uint = 0;
const SNDRV_PCM_RATE_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 0;

const ADAU1701_CLK_SRC_OSC: c_int = 0;
const ADAU1701_CLK_SRC_MCLK: c_int = 0;

const fn bit(n: c_uint) -> c_uint {
    1u32 << n
}

const fn adau1701_safeload_data(i: c_uint) -> c_uint {
    0x0810 + i
}

const fn adau1701_safeload_addr(i: c_uint) -> c_uint {
    0x0815 + i
}

const ADAU1701_DSPCTRL: c_uint = 0x081c;
const ADAU1701_SEROCTL: c_uint = 0x081e;
const ADAU1701_SERICTL: c_uint = 0x081f;

const ADAU1701_AUXNPOW: c_uint = 0x0822;
const ADAU1701_PINCONF_0: c_uint = 0x0820;
const ADAU1701_PINCONF_1: c_uint = 0x0821;

const ADAU1701_OSCIPOW: c_uint = 0x0826;
const ADAU1701_DACSET: c_uint = 0x0827;

const ADAU1701_MAX_REGISTER: c_uint = 0x0828;

const ADAU1701_DSPCTRL_CR: c_uint = 1 << 2;
const ADAU1701_DSPCTRL_DAM: c_uint = 1 << 3;
const ADAU1701_DSPCTRL_ADM: c_uint = 1 << 4;
const ADAU1701_DSPCTRL_IST: c_uint = 1 << 5;
const ADAU1701_DSPCTRL_SR_48: c_uint = 0x00;
const ADAU1701_DSPCTRL_SR_96: c_uint = 0x01;
const ADAU1701_DSPCTRL_SR_192: c_uint = 0x02;
const ADAU1701_DSPCTRL_SR_MASK: c_uint = 0x03;

const ADAU1701_SEROCTL_INV_LRCLK: c_uint = 0x2000;
const ADAU1701_SEROCTL_INV_BCLK: c_uint = 0x1000;
const ADAU1701_SEROCTL_MASTER: c_uint = 0x0800;

const ADAU1701_SEROCTL_OBF16: c_uint = 0x0000;
const ADAU1701_SEROCTL_OBF8: c_uint = 0x0200;
const ADAU1701_SEROCTL_OBF4: c_uint = 0x0400;
const ADAU1701_SEROCTL_OBF2: c_uint = 0x0600;
const ADAU1701_SEROCTL_OBF_MASK: c_uint = 0x0600;

const ADAU1701_SEROCTL_OLF1024: c_uint = 0x0000;
const ADAU1701_SEROCTL_OLF512: c_uint = 0x0080;
const ADAU1701_SEROCTL_OLF256: c_uint = 0x0100;
const ADAU1701_SEROCTL_OLF_MASK: c_uint = 0x0180;

const ADAU1701_SEROCTL_MSB_DEALY1: c_uint = 0x0000;
const ADAU1701_SEROCTL_MSB_DEALY0: c_uint = 0x0004;
const ADAU1701_SEROCTL_MSB_DEALY8: c_uint = 0x0008;
const ADAU1701_SEROCTL_MSB_DEALY12: c_uint = 0x000c;
const ADAU1701_SEROCTL_MSB_DEALY16: c_uint = 0x0010;
const ADAU1701_SEROCTL_MSB_DEALY_MASK: c_uint = 0x001c;

const ADAU1701_SEROCTL_WORD_LEN_24: c_uint = 0x0000;
const ADAU1701_SEROCTL_WORD_LEN_20: c_uint = 0x0001;
const ADAU1701_SEROCTL_WORD_LEN_16: c_uint = 0x0002;
const ADAU1701_SEROCTL_WORD_LEN_MASK: c_uint = 0x0003;

const ADAU1701_AUXNPOW_VBPD: c_uint = 0x40;
const ADAU1701_AUXNPOW_VRPD: c_uint = 0x20;

const ADAU1701_SERICTL_I2S: c_uint = 0;
const ADAU1701_SERICTL_LEFTJ: c_uint = 1;
const ADAU1701_SERICTL_TDM: c_uint = 2;
const ADAU1701_SERICTL_RIGHTJ_24: c_uint = 3;
const ADAU1701_SERICTL_RIGHTJ_20: c_uint = 4;
const ADAU1701_SERICTL_RIGHTJ_18: c_uint = 5;
const ADAU1701_SERICTL_RIGHTJ_16: c_uint = 6;
const ADAU1701_SERICTL_MODE_MASK: c_uint = 7;
const ADAU1701_SERICTL_INV_BCLK: c_uint = bit(3);
const ADAU1701_SERICTL_INV_LRCLK: c_uint = bit(4);

const ADAU1701_OSCIPOW_OPD: c_uint = 0x04;
const ADAU1701_DACSET_DACINIT: c_uint = 1;

const ADAU1707_CLKDIV_UNSET: c_uint = !0u32;

static ADAU1701_FIRMWARE: &[u8] = b"adau1701.bin\0";

static SUPPLY_DVDD: &[u8] = b"dvdd\0";
static SUPPLY_AVDD: &[u8] = b"avdd\0";
static SUPPLY_NAMES: [*const c_char; 2] = [
    SUPPLY_DVDD.as_ptr() as *const c_char,
    SUPPLY_AVDD.as_ptr() as *const c_char,
];

#[repr(C)]
struct device {
    of_node: *mut device_node,
}

#[repr(C)]
struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
struct gpio_descs {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct i2c_adapter {
    _private: [u8; 0],
}

#[repr(C)]
struct i2c_client {
    dev: device,
    addr: u16,
    adapter: *mut i2c_adapter,
}

#[repr(C)]
struct i2c_msg {
    addr: u16,
    flags: u16,
    len: u16,
    buf: *mut u8,
}

#[repr(C)]
struct sigmadsp {
    dev: *mut device,
}

#[repr(C)]
struct regulator_bulk_data {
    supply: *const c_char,
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    stream: c_uint,
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
#[derive(Copy, Clone)]
enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

#[repr(C)]
struct sigmadsp_ops {
    safeload: Option<
        unsafe extern "C" fn(*mut sigmadsp, c_uint, *const u8, usize) -> c_int,
    >,
}

#[repr(C)]
struct snd_soc_dai_ops {
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    no_capture_mute: c_uint,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    set_bias_level:
        Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    set_sysclk:
        Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    cache_type: c_uint,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
    reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
}

#[repr(C)]
struct i2c_device_id {
    name: *const c_char,
}

#[repr(C)]
struct driver_data {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct i2c_driver {
    driver: driver_data,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    id_table: *const i2c_device_id,
}

#[repr(C)]
struct adau1701 {
    gpio_nreset: *mut gpio_desc,
    gpio_pll_mode: *mut gpio_descs,
    dai_fmt: c_uint,
    pll_clkdiv: c_uint,
    sysclk: c_uint,
    regmap: *mut regmap,
    client: *mut i2c_client,
    pin_config: [u8; 12],

    sigmadsp: *mut sigmadsp,
    supplies: [regulator_bulk_data; 2],
}

unsafe extern "C" {
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn i2c_master_send(client: *mut i2c_client, buf: *const u8, count: c_int) -> c_int;
    fn i2c_transfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int;
    fn to_i2c_client(dev: *mut device) -> *mut i2c_client;
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn msleep(msecs: c_uint);
    fn udelay(usecs: c_uint);
    fn mdelay(msecs: c_uint);
    fn sigmadsp_reset(sigmadsp: *mut sigmadsp);
    fn sigmadsp_setup(sigmadsp: *mut sigmadsp, rate: c_uint) -> c_int;
    fn sigmadsp_attach(sigmadsp: *mut sigmadsp, component: *mut snd_soc_component) -> c_int;
    fn sigmadsp_restrict_params(
        sigmadsp: *mut sigmadsp,
        substream: *mut snd_pcm_substream,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn gpiod_multi_set_value_cansleep(descs: *mut gpio_descs, values: *const u64);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regulator_bulk_enable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_uint,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const c_void,
        bus_context: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_property_read_u32(
        np: *mut device_node,
        propname: *const c_char,
        out_value: *mut c_uint,
    ) -> c_int;
    fn of_property_read_u8_array(
        np: *mut device_node,
        propname: *const c_char,
        out_values: *mut u8,
        sz: usize,
    ) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_int)
        -> *mut gpio_desc;
    fn devm_gpiod_get_array_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_int,
    ) -> *mut gpio_descs;
    fn devm_sigmadsp_init_i2c(
        client: *mut i2c_client,
        ops: *const sigmadsp_ops,
        firmware: *const c_char,
    ) -> *mut sigmadsp;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn of_match_ptr(ids: *const of_device_id) -> *const of_device_id;
}

const fn array_size<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

unsafe fn put_unaligned_le16(value: c_uint, p: *mut u8) {
    unsafe {
        *p.add(0) = (value & 0xff) as u8;
        *p.add(1) = ((value >> 8) & 0xff) as u8;
    }
}

unsafe fn __assign_bit(nr: usize, addr: *mut u64, value: c_int) {
    unsafe {
        let mask = 1u64 << nr;
        if value != 0 {
            *addr |= mask;
        } else {
            *addr &= !mask;
        }
    }
}

static ADAU1701_CONTROLS: [snd_kcontrol_new; 1] = [
    /* SOC_SINGLE("Master Capture Switch", ADAU1701_DSPCTRL, 4, 1, 0) */
    snd_kcontrol_new { _private: [] },
];

static ADAU1701_DAPM_WIDGETS: [snd_soc_dapm_widget; 11] = [
    /* SND_SOC_DAPM_DAC("DAC0", "Playback", ADAU1701_AUXNPOW, 3, 1) */
    snd_soc_dapm_widget { _private: [] },
    /* SND_SOC_DAPM_DAC("DAC1", "Playback", ADAU1701_AUXNPOW, 2, 1) */
    snd_soc_dapm_widget { _private: [] },
    /* SND_SOC_DAPM_DAC("DAC2", "Playback", ADAU1701_AUXNPOW, 1, 1) */
    snd_soc_dapm_widget { _private: [] },
    /* SND_SOC_DAPM_DAC("DAC3", "Playback", ADAU1701_AUXNPOW, 0, 1) */
    snd_soc_dapm_widget { _private: [] },
    /* SND_SOC_DAPM_ADC("ADC", "Capture", ADAU1701_AUXNPOW, 7, 1) */
    snd_soc_dapm_widget { _private: [] },
    /* SND_SOC_DAPM_OUTPUT("OUT0") */
    snd_soc_dapm_widget { _private: [] },
    /* SND_SOC_DAPM_OUTPUT("OUT1") */
    snd_soc_dapm_widget { _private: [] },
    /* SND_SOC_DAPM_OUTPUT("OUT2") */
    snd_soc_dapm_widget { _private: [] },
    /* SND_SOC_DAPM_OUTPUT("OUT3") */
    snd_soc_dapm_widget { _private: [] },
    /* SND_SOC_DAPM_INPUT("IN0") */
    snd_soc_dapm_widget { _private: [] },
    /* SND_SOC_DAPM_INPUT("IN1") */
    snd_soc_dapm_widget { _private: [] },
];

static OUT0: &[u8] = b"OUT0\0";
static OUT1: &[u8] = b"OUT1\0";
static OUT2: &[u8] = b"OUT2\0";
static OUT3: &[u8] = b"OUT3\0";
static DAC0: &[u8] = b"DAC0\0";
static DAC1: &[u8] = b"DAC1\0";
static DAC2: &[u8] = b"DAC2\0";
static DAC3: &[u8] = b"DAC3\0";
static ADC: &[u8] = b"ADC\0";
static IN0: &[u8] = b"IN0\0";
static IN1: &[u8] = b"IN1\0";

static ADAU1701_DAPM_ROUTES: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: OUT0.as_ptr() as *const c_char, control: ptr::null(), source: DAC0.as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: OUT1.as_ptr() as *const c_char, control: ptr::null(), source: DAC1.as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: OUT2.as_ptr() as *const c_char, control: ptr::null(), source: DAC2.as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: OUT3.as_ptr() as *const c_char, control: ptr::null(), source: DAC3.as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: ADC.as_ptr() as *const c_char, control: ptr::null(), source: IN0.as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: ADC.as_ptr() as *const c_char, control: ptr::null(), source: IN1.as_ptr() as *const c_char },
];

unsafe extern "C" fn adau1701_register_size(dev: *mut device, reg: c_uint) -> c_uint {
    match reg {
        ADAU1701_PINCONF_0 | ADAU1701_PINCONF_1 => 3,
        ADAU1701_DSPCTRL
        | ADAU1701_SEROCTL
        | ADAU1701_AUXNPOW
        | ADAU1701_OSCIPOW
        | ADAU1701_DACSET => 2,
        ADAU1701_SERICTL => 1,
        _ => {
            unsafe { dev_err(dev, b"Unsupported register address: %d\n\0".as_ptr() as *const c_char, reg) };
            0
        }
    }
}

unsafe extern "C" fn adau1701_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        ADAU1701_DACSET | ADAU1701_DSPCTRL => true,
        _ => false,
    }
}

unsafe extern "C" fn adau1701_reg_write(
    context: *mut c_void,
    reg: c_uint,
    mut value: c_uint,
) -> c_int {
    unsafe {
        let client = context as *mut i2c_client;
        let mut buf = [0u8; 5];

        let size = adau1701_register_size(&mut (*client).dev, reg);
        if size == 0 {
            return -EINVAL;
        }

        buf[0] = (reg >> 8) as u8;
        buf[1] = (reg & 0xff) as u8;

        let mut i = size + 1;
        loop {
            buf[i as usize] = value as u8;
            value >>= 8;
            if i == 2 {
                break;
            }
            i -= 1;
        }

        let ret = i2c_master_send(client, buf.as_ptr(), (size + 2) as c_int);
        if ret == (size + 2) as c_int {
            0
        } else if ret < 0 {
            ret
        } else {
            -EIO
        }
    }
}

unsafe extern "C" fn adau1701_reg_read(
    context: *mut c_void,
    reg: c_uint,
    value: *mut c_uint,
) -> c_int {
    unsafe {
        let client = context as *mut i2c_client;
        let mut send_buf = [0u8; 2];
        let mut recv_buf = [0u8; 3];
        let mut msgs = [
            i2c_msg { addr: 0, flags: 0, len: 0, buf: ptr::null_mut() },
            i2c_msg { addr: 0, flags: 0, len: 0, buf: ptr::null_mut() },
        ];

        let size = adau1701_register_size(&mut (*client).dev, reg);
        if size == 0 {
            return -EINVAL;
        }

        send_buf[0] = (reg >> 8) as u8;
        send_buf[1] = (reg & 0xff) as u8;

        msgs[0].addr = (*client).addr;
        msgs[0].len = size_of::<[u8; 2]>() as u16;
        msgs[0].buf = send_buf.as_mut_ptr();
        msgs[0].flags = 0;

        msgs[1].addr = (*client).addr;
        msgs[1].len = size as u16;
        msgs[1].buf = recv_buf.as_mut_ptr();
        msgs[1].flags = I2C_M_RD;

        let ret = i2c_transfer((*client).adapter, msgs.as_mut_ptr(), msgs.len() as c_int);
        if ret < 0 {
            return ret;
        } else if ret != msgs.len() as c_int {
            return -EIO;
        }

        *value = 0;
        for i in 0..size as usize {
            *value <<= 8;
            *value |= recv_buf[i] as c_uint;
        }

        0
    }
}

unsafe extern "C" fn adau1701_safeload(
    sigmadsp: *mut sigmadsp,
    addr: c_uint,
    bytes: *const u8,
    len: usize,
) -> c_int {
    unsafe {
        let client = to_i2c_client((*sigmadsp).dev);
        let adau1701 = i2c_get_clientdata(client) as *mut adau1701;
        let mut val: c_uint = 0;
        let mut buf = [0u8; 10];

        let mut ret = regmap_read((*adau1701).regmap, ADAU1701_DSPCTRL, &mut val);
        if ret != 0 {
            return ret;
        }

        if (val & ADAU1701_DSPCTRL_IST) != 0 {
            msleep(50);
        }

        for i in 0..(len / 4) {
            put_unaligned_le16(adau1701_safeload_data(i as c_uint), buf.as_mut_ptr());
            buf[2] = 0x00;
            ptr::copy_nonoverlapping(bytes.add(i * 4), buf.as_mut_ptr().add(3), 4);
            ret = i2c_master_send(client, buf.as_ptr(), 7);
            if ret < 0 {
                return ret;
            } else if ret != 7 {
                return -EIO;
            }

            put_unaligned_le16(adau1701_safeload_addr(i as c_uint), buf.as_mut_ptr());
            put_unaligned_le16(addr + i as c_uint, buf.as_mut_ptr().add(2));
            ret = i2c_master_send(client, buf.as_ptr(), 4);
            if ret < 0 {
                return ret;
            } else if ret != 4 {
                return -EIO;
            }
        }

        regmap_update_bits(
            (*adau1701).regmap,
            ADAU1701_DSPCTRL,
            ADAU1701_DSPCTRL_IST,
            ADAU1701_DSPCTRL_IST,
        )
    }
}

static ADAU1701_SIGMADSP_OPS: sigmadsp_ops = sigmadsp_ops {
    safeload: Some(adau1701_safeload),
};

unsafe extern "C" fn adau1701_reset(
    component: *mut snd_soc_component,
    clkdiv: c_uint,
    rate: c_uint,
) -> c_int {
    unsafe {
        let adau1701 = snd_soc_component_get_drvdata(component) as *mut adau1701;
        let mut values = [0u64; 1];

        sigmadsp_reset((*adau1701).sigmadsp);

        if clkdiv != ADAU1707_CLKDIV_UNSET && !(*adau1701).gpio_pll_mode.is_null() {
            match clkdiv {
                64 => {
                    __assign_bit(0, values.as_mut_ptr(), 0);
                    __assign_bit(1, values.as_mut_ptr(), 0);
                }
                256 => {
                    __assign_bit(0, values.as_mut_ptr(), 0);
                    __assign_bit(1, values.as_mut_ptr(), 1);
                }
                384 => {
                    __assign_bit(0, values.as_mut_ptr(), 1);
                    __assign_bit(1, values.as_mut_ptr(), 0);
                }
                0 | 512 => {
                    __assign_bit(0, values.as_mut_ptr(), 1);
                    __assign_bit(1, values.as_mut_ptr(), 1);
                }
                _ => {}
            }
            gpiod_multi_set_value_cansleep((*adau1701).gpio_pll_mode, values.as_ptr());
        }

        (*adau1701).pll_clkdiv = clkdiv;

        if !(*adau1701).gpio_nreset.is_null() {
            gpiod_set_value_cansleep((*adau1701).gpio_nreset, 0);
            /* minimum reset time is 20ns */
            udelay(1);
            gpiod_set_value_cansleep((*adau1701).gpio_nreset, 1);
            /* power-up time may be as long as 85ms */
            mdelay(85);
        }

        /*
         * Postpone the firmware download to a point in time when we
         * know the correct PLL setup
         */
        if clkdiv != ADAU1707_CLKDIV_UNSET {
            let ret = sigmadsp_setup((*adau1701).sigmadsp, rate);
            if ret != 0 {
                dev_warn((*component).dev, b"Failed to load firmware\n\0".as_ptr() as *const c_char);
                return ret;
            }
        }

        regmap_write((*adau1701).regmap, ADAU1701_DACSET, ADAU1701_DACSET_DACINIT);
        regmap_write((*adau1701).regmap, ADAU1701_DSPCTRL, ADAU1701_DSPCTRL_CR);

        regcache_mark_dirty((*adau1701).regmap);
        regcache_sync((*adau1701).regmap);

        0
    }
}

unsafe extern "C" fn adau1701_set_capture_pcm_format(
    component: *mut snd_soc_component,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    unsafe {
        let adau1701 = snd_soc_component_get_drvdata(component) as *mut adau1701;
        let mut mask = ADAU1701_SEROCTL_WORD_LEN_MASK;
        let mut val: c_uint;

        match params_width(params) {
            16 => val = ADAU1701_SEROCTL_WORD_LEN_16,
            20 => val = ADAU1701_SEROCTL_WORD_LEN_20,
            24 => val = ADAU1701_SEROCTL_WORD_LEN_24,
            _ => return -EINVAL,
        }

        if (*adau1701).dai_fmt == SND_SOC_DAIFMT_RIGHT_J {
            match params_width(params) {
                16 => val |= ADAU1701_SEROCTL_MSB_DEALY16,
                20 => val |= ADAU1701_SEROCTL_MSB_DEALY12,
                24 => val |= ADAU1701_SEROCTL_MSB_DEALY8,
                _ => {}
            }
            mask |= ADAU1701_SEROCTL_MSB_DEALY_MASK;
        }

        regmap_update_bits((*adau1701).regmap, ADAU1701_SEROCTL, mask, val);

        0
    }
}

unsafe extern "C" fn adau1701_set_playback_pcm_format(
    component: *mut snd_soc_component,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    unsafe {
        let adau1701 = snd_soc_component_get_drvdata(component) as *mut adau1701;
        let val: c_uint;

        if (*adau1701).dai_fmt != SND_SOC_DAIFMT_RIGHT_J {
            return 0;
        }

        match params_width(params) {
            16 => val = ADAU1701_SERICTL_RIGHTJ_16,
            20 => val = ADAU1701_SERICTL_RIGHTJ_20,
            24 => val = ADAU1701_SERICTL_RIGHTJ_24,
            _ => return -EINVAL,
        }

        regmap_update_bits(
            (*adau1701).regmap,
            ADAU1701_SERICTL,
            ADAU1701_SERICTL_MODE_MASK,
            val,
        );

        0
    }
}

unsafe extern "C" fn adau1701_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let component = (*dai).component;
        let adau1701 = snd_soc_component_get_drvdata(component) as *mut adau1701;
        let clkdiv = (*adau1701).sysclk / params_rate(params);
        let val: c_uint;

        /*
         * If the mclk/lrclk ratio changes, the chip needs updated PLL
         * mode GPIO settings, and a full reset cycle, including a new
         * firmware upload.
         */
        if clkdiv != (*adau1701).pll_clkdiv {
            let ret = adau1701_reset(component, clkdiv, params_rate(params));
            if ret < 0 {
                return ret;
            }
        }

        match params_rate(params) {
            192000 => val = ADAU1701_DSPCTRL_SR_192,
            96000 => val = ADAU1701_DSPCTRL_SR_96,
            48000 => val = ADAU1701_DSPCTRL_SR_48,
            _ => return -EINVAL,
        }

        regmap_update_bits(
            (*adau1701).regmap,
            ADAU1701_DSPCTRL,
            ADAU1701_DSPCTRL_SR_MASK,
            val,
        );

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            adau1701_set_playback_pcm_format(component, params)
        } else {
            adau1701_set_capture_pcm_format(component, params)
        }
    }
}

unsafe extern "C" fn adau1701_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    unsafe {
        let component = (*codec_dai).component;
        let adau1701 = snd_soc_component_get_drvdata(component) as *mut adau1701;
        let mut serictl: c_uint = 0x00;
        let mut seroctl: c_uint = 0x00;
        let mut invert_lrclk: bool;

        match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
            SND_SOC_DAIFMT_CBP_CFP => {
                /* master, 64-bits per sample, 1 frame per sample */
                seroctl |= ADAU1701_SEROCTL_MASTER
                    | ADAU1701_SEROCTL_OBF16
                    | ADAU1701_SEROCTL_OLF1024;
            }
            SND_SOC_DAIFMT_CBC_CFC => {}
            _ => return -EINVAL,
        }

        /* clock inversion */
        match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => invert_lrclk = false,
            SND_SOC_DAIFMT_NB_IF => invert_lrclk = true,
            SND_SOC_DAIFMT_IB_NF => {
                invert_lrclk = false;
                serictl |= ADAU1701_SERICTL_INV_BCLK;
                seroctl |= ADAU1701_SEROCTL_INV_BCLK;
            }
            SND_SOC_DAIFMT_IB_IF => {
                invert_lrclk = true;
                serictl |= ADAU1701_SERICTL_INV_BCLK;
                seroctl |= ADAU1701_SEROCTL_INV_BCLK;
            }
            _ => return -EINVAL,
        }

        match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            SND_SOC_DAIFMT_I2S => {}
            SND_SOC_DAIFMT_LEFT_J => {
                serictl |= ADAU1701_SERICTL_LEFTJ;
                seroctl |= ADAU1701_SEROCTL_MSB_DEALY0;
                invert_lrclk = !invert_lrclk;
            }
            SND_SOC_DAIFMT_RIGHT_J => {
                serictl |= ADAU1701_SERICTL_RIGHTJ_24;
                seroctl |= ADAU1701_SEROCTL_MSB_DEALY8;
                invert_lrclk = !invert_lrclk;
            }
            _ => return -EINVAL,
        }

        if invert_lrclk {
            seroctl |= ADAU1701_SEROCTL_INV_LRCLK;
            serictl |= ADAU1701_SERICTL_INV_LRCLK;
        }

        (*adau1701).dai_fmt = fmt & SND_SOC_DAIFMT_FORMAT_MASK;

        regmap_write((*adau1701).regmap, ADAU1701_SERICTL, serictl);
        regmap_update_bits(
            (*adau1701).regmap,
            ADAU1701_SEROCTL,
            !ADAU1701_SEROCTL_WORD_LEN_MASK,
            seroctl,
        );

        0
    }
}

unsafe extern "C" fn adau1701_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    unsafe {
        let mask = ADAU1701_AUXNPOW_VBPD | ADAU1701_AUXNPOW_VRPD;
        let adau1701 = snd_soc_component_get_drvdata(component) as *mut adau1701;

        match level {
            snd_soc_bias_level::SND_SOC_BIAS_ON => {}
            snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
            snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
                /* Enable VREF and VREF buffer */
                regmap_update_bits((*adau1701).regmap, ADAU1701_AUXNPOW, mask, 0x00);
            }
            snd_soc_bias_level::SND_SOC_BIAS_OFF => {
                /* Disable VREF and VREF buffer */
                regmap_update_bits((*adau1701).regmap, ADAU1701_AUXNPOW, mask, mask);
            }
        }

        0
    }
}

unsafe extern "C" fn adau1701_mute_stream(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    unsafe {
        let component = (*dai).component;
        let mask = ADAU1701_DSPCTRL_DAM;
        let adau1701 = snd_soc_component_get_drvdata(component) as *mut adau1701;
        let val: c_uint = if mute != 0 { 0 } else { mask };

        regmap_update_bits((*adau1701).regmap, ADAU1701_DSPCTRL, mask, val);

        0
    }
}

unsafe extern "C" fn adau1701_set_sysclk(
    component: *mut snd_soc_component,
    clk_id: c_int,
    _source: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    unsafe {
        let val: c_uint;
        let adau1701 = snd_soc_component_get_drvdata(component) as *mut adau1701;

        match clk_id {
            ADAU1701_CLK_SRC_OSC => val = 0x0,
            ADAU1701_CLK_SRC_MCLK => val = ADAU1701_OSCIPOW_OPD,
            _ => return -EINVAL,
        }

        regmap_update_bits(
            (*adau1701).regmap,
            ADAU1701_OSCIPOW,
            ADAU1701_OSCIPOW_OPD,
            val,
        );
        (*adau1701).sysclk = freq;

        0
    }
}

unsafe extern "C" fn adau1701_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let adau1701 = snd_soc_component_get_drvdata((*dai).component) as *mut adau1701;
        sigmadsp_restrict_params((*adau1701).sigmadsp, substream)
    }
}

const ADAU1701_RATES: c_uint =
    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;

const ADAU1701_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static ADAU1701_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(adau1701_set_dai_fmt),
    hw_params: Some(adau1701_hw_params),
    mute_stream: Some(adau1701_mute_stream),
    startup: Some(adau1701_startup),
    no_capture_mute: 1,
};

static ADAU1701_NAME: &[u8] = b"adau1701\0";
static PLAYBACK: &[u8] = b"Playback\0";
static CAPTURE: &[u8] = b"Capture\0";

static mut ADAU1701_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: ADAU1701_NAME.as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: PLAYBACK.as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 8,
        rates: ADAU1701_RATES,
        formats: ADAU1701_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: CAPTURE.as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 8,
        rates: ADAU1701_RATES,
        formats: ADAU1701_FORMATS,
    },
    ops: &ADAU1701_DAI_OPS,
    symmetric_rate: 1,
};

/* CONFIG_OF */
static ADI_ADAU1701: &[u8] = b"adi,adau1701\0";
static ADAU1701_DT_IDS: [of_device_id; 2] = [
    of_device_id { compatible: ADI_ADAU1701.as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, adau1701_dt_ids) */

unsafe extern "C" fn adau1701_probe(component: *mut snd_soc_component) -> c_int {
    unsafe {
        let mut ret: c_int;
        let mut val: c_uint;
        let adau1701 = snd_soc_component_get_drvdata(component) as *mut adau1701;

        ret = sigmadsp_attach((*adau1701).sigmadsp, component);
        if ret != 0 {
            return ret;
        }

        ret = regulator_bulk_enable(
            array_size(&(*adau1701).supplies),
            (*adau1701).supplies.as_mut_ptr(),
        );
        if ret < 0 {
            dev_err(
                (*component).dev,
                b"Failed to enable regulators: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        /*
         * Let the pll_clkdiv variable default to something that won't happen
         * at runtime. That way, we can postpone the firmware download from
         * adau1701_reset() to a point in time when we know the correct PLL
         * mode parameters.
         */
        (*adau1701).pll_clkdiv = ADAU1707_CLKDIV_UNSET;

        /* initalize with pre-configured pll mode settings */
        ret = adau1701_reset(component, (*adau1701).pll_clkdiv, 0);
        if ret < 0 {
            regulator_bulk_disable(
                array_size(&(*adau1701).supplies),
                (*adau1701).supplies.as_mut_ptr(),
            );
            return ret;
        }

        /* set up pin config */
        val = 0;
        for i in 0..6usize {
            val |= ((*adau1701).pin_config[i] as c_uint) << (i * 4);
        }

        regmap_write((*adau1701).regmap, ADAU1701_PINCONF_0, val);

        val = 0;
        for i in 0..6usize {
            val |= ((*adau1701).pin_config[i + 6] as c_uint) << (i * 4);
        }

        regmap_write((*adau1701).regmap, ADAU1701_PINCONF_1, val);

        0
    }
}

unsafe extern "C" fn adau1701_remove(component: *mut snd_soc_component) {
    unsafe {
        let adau1701 = snd_soc_component_get_drvdata(component) as *mut adau1701;

        if !(*adau1701).gpio_nreset.is_null() {
            gpiod_set_value_cansleep((*adau1701).gpio_nreset, 0);
        }

        regulator_bulk_disable(
            array_size(&(*adau1701).supplies),
            (*adau1701).supplies.as_mut_ptr(),
        );
    }
}

/* CONFIG_PM */
unsafe extern "C" fn adau1701_suspend(component: *mut snd_soc_component) -> c_int {
    unsafe {
        let adau1701 = snd_soc_component_get_drvdata(component) as *mut adau1701;

        regulator_bulk_disable(
            array_size(&(*adau1701).supplies),
            (*adau1701).supplies.as_mut_ptr(),
        );

        0
    }
}

unsafe extern "C" fn adau1701_resume(component: *mut snd_soc_component) -> c_int {
    unsafe {
        let adau1701 = snd_soc_component_get_drvdata(component) as *mut adau1701;

        let ret = regulator_bulk_enable(
            array_size(&(*adau1701).supplies),
            (*adau1701).supplies.as_mut_ptr(),
        );
        if ret < 0 {
            dev_err(
                (*component).dev,
                b"Failed to enable regulators: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        adau1701_reset(component, (*adau1701).pll_clkdiv, 0)
    }
}
/* Without CONFIG_PM, adau1701_resume and adau1701_suspend are NULL. */

static ADAU1701_COMPONENT_DRV: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(adau1701_probe),
    remove: Some(adau1701_remove),
    resume: Some(adau1701_resume),
    suspend: Some(adau1701_suspend),
    set_bias_level: Some(adau1701_set_bias_level),
    controls: ADAU1701_CONTROLS.as_ptr(),
    num_controls: array_size(&ADAU1701_CONTROLS),
    dapm_widgets: ADAU1701_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: array_size(&ADAU1701_DAPM_WIDGETS),
    dapm_routes: ADAU1701_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: array_size(&ADAU1701_DAPM_ROUTES),
    set_sysclk: Some(adau1701_set_sysclk),
    use_pmdown_time: 1,
    endianness: 1,
};

static ADAU1701_REGMAP: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 32,
    max_register: ADAU1701_MAX_REGISTER,
    cache_type: REGCACHE_MAPLE,
    volatile_reg: Some(adau1701_volatile_reg),
    reg_write: Some(adau1701_reg_write),
    reg_read: Some(adau1701_reg_read),
};

unsafe extern "C" fn adau1701_i2c_probe(client: *mut i2c_client) -> c_int {
    unsafe {
        let dev = &mut (*client).dev as *mut device;
        let mut ret: c_int;

        let adau1701 = devm_kzalloc(dev, size_of::<adau1701>(), GFP_KERNEL) as *mut adau1701;
        if adau1701.is_null() {
            return -ENOMEM;
        }

        for i in 0..SUPPLY_NAMES.len() {
            (*adau1701).supplies[i].supply = SUPPLY_NAMES[i];
        }

        ret = devm_regulator_bulk_get(
            dev,
            array_size(&(*adau1701).supplies),
            (*adau1701).supplies.as_mut_ptr(),
        );
        if ret < 0 {
            dev_err(dev, b"Failed to get regulators: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        ret = regulator_bulk_enable(
            array_size(&(*adau1701).supplies),
            (*adau1701).supplies.as_mut_ptr(),
        );
        if ret < 0 {
            dev_err(dev, b"Failed to enable regulators: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        (*adau1701).client = client;
        (*adau1701).regmap = devm_regmap_init(
            dev,
            ptr::null(),
            client as *mut c_void,
            &ADAU1701_REGMAP,
        );
        if IS_ERR((*adau1701).regmap as *const c_void) {
            ret = PTR_ERR((*adau1701).regmap as *const c_void);
            regulator_bulk_disable(
                array_size(&(*adau1701).supplies),
                (*adau1701).supplies.as_mut_ptr(),
            );
            return ret;
        }

        if !(*dev).of_node.is_null() {
            of_property_read_u32(
                (*dev).of_node,
                b"adi,pll-clkdiv\0".as_ptr() as *const c_char,
                &mut (*adau1701).pll_clkdiv,
            );

            of_property_read_u8_array(
                (*dev).of_node,
                b"adi,pin-config\0".as_ptr() as *const c_char,
                (*adau1701).pin_config.as_mut_ptr(),
                array_size(&(*adau1701).pin_config) as usize,
            );
        }

        (*adau1701).gpio_nreset =
            devm_gpiod_get_optional(dev, b"reset\0".as_ptr() as *const c_char, GPIOD_IN);

        if IS_ERR((*adau1701).gpio_nreset as *const c_void) {
            ret = PTR_ERR((*adau1701).gpio_nreset as *const c_void);
            regulator_bulk_disable(
                array_size(&(*adau1701).supplies),
                (*adau1701).supplies.as_mut_ptr(),
            );
            return ret;
        }

        (*adau1701).gpio_pll_mode = devm_gpiod_get_array_optional(
            dev,
            b"adi,pll-mode\0".as_ptr() as *const c_char,
            GPIOD_OUT_LOW,
        );

        if IS_ERR((*adau1701).gpio_pll_mode as *const c_void) {
            ret = PTR_ERR((*adau1701).gpio_pll_mode as *const c_void);
            regulator_bulk_disable(
                array_size(&(*adau1701).supplies),
                (*adau1701).supplies.as_mut_ptr(),
            );
            return ret;
        }

        i2c_set_clientdata(client, adau1701 as *mut c_void);

        (*adau1701).sigmadsp = devm_sigmadsp_init_i2c(
            client,
            &ADAU1701_SIGMADSP_OPS,
            ADAU1701_FIRMWARE.as_ptr() as *const c_char,
        );
        if IS_ERR((*adau1701).sigmadsp as *const c_void) {
            ret = PTR_ERR((*adau1701).sigmadsp as *const c_void);
            regulator_bulk_disable(
                array_size(&(*adau1701).supplies),
                (*adau1701).supplies.as_mut_ptr(),
            );
            return ret;
        }

        ret = devm_snd_soc_register_component(
            &mut (*client).dev,
            &ADAU1701_COMPONENT_DRV,
            &raw mut ADAU1701_DAI,
            1,
        );

        regulator_bulk_disable(
            array_size(&(*adau1701).supplies),
            (*adau1701).supplies.as_mut_ptr(),
        );
        ret
    }
}

static ADAU1401: &[u8] = b"adau1401\0";
static ADAU1401A: &[u8] = b"adau1401a\0";
static ADAU1702: &[u8] = b"adau1702\0";

static ADAU1701_I2C_ID: [i2c_device_id; 5] = [
    i2c_device_id { name: ADAU1401.as_ptr() as *const c_char },
    i2c_device_id { name: ADAU1401A.as_ptr() as *const c_char },
    i2c_device_id { name: ADAU1701_NAME.as_ptr() as *const c_char },
    i2c_device_id { name: ADAU1702.as_ptr() as *const c_char },
    i2c_device_id { name: ptr::null() },
];
/* MODULE_DEVICE_TABLE(i2c, adau1701_i2c_id) */

static mut ADAU1701_I2C_DRIVER: i2c_driver = i2c_driver {
    driver: driver_data {
        name: ADAU1701_NAME.as_ptr() as *const c_char,
        of_match_table: ptr::null(),
    },
    probe: Some(adau1701_i2c_probe),
    id_table: ADAU1701_I2C_ID.as_ptr(),
};

unsafe fn init_adau1701_i2c_driver_match_table() {
    unsafe {
        ADAU1701_I2C_DRIVER.driver.of_match_table = of_match_ptr(ADAU1701_DT_IDS.as_ptr());
    }
}

/* module_i2c_driver(adau1701_i2c_driver); */

/* MODULE_DESCRIPTION("ASoC ADAU1701 SigmaDSP driver"); */
/* MODULE_AUTHOR("Cliff Cai <cliff.cai@analog.com>"); */
/* MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
