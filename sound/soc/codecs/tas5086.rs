// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * TAS5086 ASoC codec driver
 *
 * Copyright (c) 2013 Daniel Mack <zonque@gmail.com>
 *
 * TODO:
 *  - implement DAPM and input muxing
 *  - implement modulation limit
 *  - implement non-default PWM start
 *
 * Note that this chip has a very unusual register layout, specifically
 * because the registers are of unequal size, and multi-byte registers
 * require bulk writes to take effect. Regmap does not support that kind
 * of devices.
 *
 * Currently, the driver does not touch any of the registers >= 0x20, so
 * it doesn't matter because the entire map can be accessed as 8-bit
 * array. In case more features will be added in the future
 * that require access to higher registers, the entire regmap H/W I/O
 * routines have to be open-coded.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const TAS5086_PCM_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_3LE;

const TAS5086_PCM_RATES: c_uint = SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

/*
 * TAS5086 registers
 */
const TAS5086_CLOCK_CONTROL: c_uint = 0x00; /* Clock control register  */
const fn TAS5086_CLOCK_RATE(val: c_int) -> c_uint {
    ((val as c_uint) << 5) as c_uint
}
const TAS5086_CLOCK_RATE_MASK: c_uint = 0x7 << 5;
const fn TAS5086_CLOCK_RATIO(val: c_int) -> c_uint {
    ((val as c_uint) << 2) as c_uint
}
const TAS5086_CLOCK_RATIO_MASK: c_uint = 0x7 << 2;
const TAS5086_CLOCK_SCLK_RATIO_48: c_uint = 1 << 1;
const TAS5086_CLOCK_VALID: c_uint = 1 << 0;

const TAS5086_DEEMPH_MASK: c_uint = 0x03;
const TAS5086_SOFT_MUTE_ALL: c_uint = 0x3f;

const TAS5086_DEV_ID: c_uint = 0x01; /* Device ID register */
const TAS5086_ERROR_STATUS: c_uint = 0x02; /* Error status register */
const TAS5086_SYS_CONTROL_1: c_uint = 0x03; /* System control register 1 */
const TAS5086_SERIAL_DATA_IF: c_uint = 0x04; /* Serial data interface register  */
const TAS5086_SYS_CONTROL_2: c_uint = 0x05; /* System control register 2 */
const TAS5086_SOFT_MUTE: c_uint = 0x06; /* Soft mute register */
const TAS5086_MASTER_VOL: c_uint = 0x07; /* Master volume  */
const fn TAS5086_CHANNEL_VOL(x: c_uint) -> c_uint {
    0x08 + x
}
const TAS5086_VOLUME_CONTROL: c_uint = 0x09; /* Volume control register */
const TAS5086_MOD_LIMIT: c_uint = 0x10; /* Modulation limit register */
const TAS5086_PWM_START: c_uint = 0x18; /* PWM start register */
const TAS5086_SURROUND: c_uint = 0x19; /* Surround register */
const TAS5086_SPLIT_CAP_CHARGE: c_uint = 0x1a; /* Split cap charge period register */
const TAS5086_OSC_TRIM: c_uint = 0x1b; /* Oscillator trim register */
const TAS5086_BKNDERR: c_uint = 0x1c;
const TAS5086_INPUT_MUX: c_uint = 0x20;
const TAS5086_PWM_OUTPUT_MUX: c_uint = 0x25;

const TAS5086_MAX_REGISTER: c_uint = TAS5086_PWM_OUTPUT_MUX;

const TAS5086_PWM_START_MIDZ_FOR_START_1: c_uint = 1 << 7;
const TAS5086_PWM_START_MIDZ_FOR_START_2: c_uint = 1 << 6;
const TAS5086_PWM_START_CHANNEL_MASK: c_uint = 0x3f;

const ARRAY_SIZE_SUPPLY_NAMES: usize = 2;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_adapter {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub addr: u16,
    pub adapter: *mut i2c_adapter,
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct i2c_msg {
    pub addr: u16,
    pub flags: u16,
    pub len: u16,
    pub buf: *mut u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

extern "C" {
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_3LE: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static EINVAL: c_int;
    static EIO: c_int;
    static ENOENT: c_int;
    static ENOMEM: c_int;
    static ENODEV: c_int;
    static I2C_M_RD: u16;
    static TAS5086_CLK_IDX_MCLK: c_int;
    static TAS5086_CLK_IDX_SCLK: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_HIGH: c_uint;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn i2c_master_send(client: *mut i2c_client, buf: *const u8, count: c_int) -> c_int;
    fn i2c_transfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn udelay(usecs: c_uint);
    fn msleep(msecs: c_uint);
    fn regulator_bulk_enable(num: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: c_int, consumers: *mut regulator_bulk_data) -> c_int;
}

/*
 * Default TAS5086 power-up configuration
 */
static tas5086_reg_defaults: [reg_default; 29] = [
    reg_default { reg: 0x00, def: 0x6c },
    reg_default { reg: 0x01, def: 0x03 },
    reg_default { reg: 0x02, def: 0x00 },
    reg_default { reg: 0x03, def: 0xa0 },
    reg_default { reg: 0x04, def: 0x05 },
    reg_default { reg: 0x05, def: 0x60 },
    reg_default { reg: 0x06, def: 0x00 },
    reg_default { reg: 0x07, def: 0xff },
    reg_default { reg: 0x08, def: 0x30 },
    reg_default { reg: 0x09, def: 0x30 },
    reg_default { reg: 0x0a, def: 0x30 },
    reg_default { reg: 0x0b, def: 0x30 },
    reg_default { reg: 0x0c, def: 0x30 },
    reg_default { reg: 0x0d, def: 0x30 },
    reg_default { reg: 0x0e, def: 0xb1 },
    reg_default { reg: 0x0f, def: 0x00 },
    reg_default { reg: 0x10, def: 0x02 },
    reg_default { reg: 0x11, def: 0x00 },
    reg_default { reg: 0x12, def: 0x00 },
    reg_default { reg: 0x13, def: 0x00 },
    reg_default { reg: 0x14, def: 0x00 },
    reg_default { reg: 0x15, def: 0x00 },
    reg_default { reg: 0x16, def: 0x00 },
    reg_default { reg: 0x17, def: 0x00 },
    reg_default { reg: 0x18, def: 0x3f },
    reg_default { reg: 0x19, def: 0x00 },
    reg_default { reg: 0x1a, def: 0x18 },
    reg_default { reg: 0x1b, def: 0x82 },
    reg_default { reg: 0x1c, def: 0x05 },
];

unsafe fn tas5086_register_size(dev: *mut device, reg: c_uint) -> c_int {
    match reg {
        TAS5086_CLOCK_CONTROL..=TAS5086_BKNDERR => 1,
        TAS5086_INPUT_MUX | TAS5086_PWM_OUTPUT_MUX => 4,
        _ => {
            dev_err(dev, c"Unsupported register address: %d\n".as_ptr(), reg);
            0
        }
    }
}

unsafe fn tas5086_accessible_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x0f | 0x11..=0x17 | 0x1d..=0x1f => false,
        _ => true,
    }
}

unsafe fn tas5086_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TAS5086_DEV_ID | TAS5086_ERROR_STATUS => true,
        _ => false,
    }
}

unsafe fn tas5086_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    tas5086_accessible_reg(dev, reg) && reg != TAS5086_DEV_ID
}

unsafe fn tas5086_reg_write(context: *mut c_void, reg: c_uint, mut value: c_uint) -> c_int {
    let client = context as *mut i2c_client;
    let size = tas5086_register_size(&mut (*client).dev, reg) as c_uint;
    let mut buf = [0u8; 5];
    let ret: c_int;

    if size == 0 {
        return -EINVAL;
    }

    buf[0] = reg as u8;

    let mut i = size;
    while i >= 1 {
        buf[i as usize] = value as u8;
        value >>= 8;
        i -= 1;
    }

    ret = i2c_master_send(client, buf.as_ptr(), (size + 1) as c_int);
    if ret == (size + 1) as c_int {
        0
    } else if ret < 0 {
        ret
    } else {
        -EIO
    }
}

unsafe fn tas5086_reg_read(context: *mut c_void, reg: c_uint, value: *mut c_uint) -> c_int {
    let client = context as *mut i2c_client;
    let mut send_buf: u8;
    let mut recv_buf = [0u8; 4];
    let mut msgs = [
        i2c_msg {
            addr: 0,
            flags: 0,
            len: 0,
            buf: ptr::null_mut(),
        },
        i2c_msg {
            addr: 0,
            flags: 0,
            len: 0,
            buf: ptr::null_mut(),
        },
    ];
    let size = tas5086_register_size(&mut (*client).dev, reg) as c_uint;
    let ret: c_int;

    if size == 0 {
        return -EINVAL;
    }

    send_buf = reg as u8;

    msgs[0].addr = (*client).addr;
    msgs[0].len = size_of::<u8>() as u16;
    msgs[0].buf = &mut send_buf;
    msgs[0].flags = 0;

    msgs[1].addr = (*client).addr;
    msgs[1].len = size as u16;
    msgs[1].buf = recv_buf.as_mut_ptr();
    msgs[1].flags = I2C_M_RD;

    ret = i2c_transfer((*client).adapter, msgs.as_mut_ptr(), msgs.len() as c_int);
    if ret < 0 {
        return ret;
    } else if ret != msgs.len() as c_int {
        return -EIO;
    }

    *value = 0;

    for i in 0..size {
        *value <<= 8;
        *value |= recv_buf[i as usize] as c_uint;
    }

    0
}

static supply_names: [*const c_char; ARRAY_SIZE_SUPPLY_NAMES] = [c"dvdd".as_ptr(), c"avdd".as_ptr()];

#[repr(C)]
struct tas5086_private {
    regmap: *mut regmap,
    mclk: c_uint,
    sclk: c_uint,
    format: c_uint,
    deemph: bool,
    charge_period: c_uint,
    pwm_start_mid_z: c_uint,
    /* Current sample rate for de-emphasis control */
    rate: c_int,
    /* GPIO driving Reset pin, if any */
    reset: *mut gpio_desc,
    supplies: [regulator_bulk_data; ARRAY_SIZE_SUPPLY_NAMES],
}

static mut tas5086_deemph: [c_int; 4] = [0, 32000, 44100, 48000];

unsafe fn tas5086_set_deemph(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut tas5086_private;
    let mut val: c_int = 0;

    if (*priv_).deemph {
        for i in 0..tas5086_deemph.len() {
            if tas5086_deemph[i] == (*priv_).rate {
                val = i as c_int;
                break;
            }
        }
    }

    regmap_update_bits(
        (*priv_).regmap,
        TAS5086_SYS_CONTROL_1,
        TAS5086_DEEMPH_MASK,
        val as c_uint,
    )
}

unsafe fn tas5086_get_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut tas5086_private;

    (*ucontrol).value.integer.value[0] = (*priv_).deemph as i64;

    0
}

unsafe fn tas5086_put_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut tas5086_private;

    (*priv_).deemph = (*ucontrol).value.integer.value[0] != 0;

    tas5086_set_deemph(component)
}

unsafe fn tas5086_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut tas5086_private;

    if clk_id == TAS5086_CLK_IDX_MCLK {
        (*priv_).mclk = freq;
    } else if clk_id == TAS5086_CLK_IDX_SCLK {
        (*priv_).sclk = freq;
    }

    0
}

unsafe fn tas5086_set_dai_fmt(codec_dai: *mut snd_soc_dai, format: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut tas5086_private;

    /* The TAS5086 can only be slave to all clocks */
    if (format & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC {
        dev_err((*component).dev, c"Invalid clocking mode\n".as_ptr());
        return -EINVAL;
    }

    /* we need to refer to the data format from hw_params() */
    (*priv_).format = format;

    0
}

static tas5086_sample_rates: [c_int; 8] = [32000, 38000, 44100, 48000, 88200, 96000, 176400, 192000];

static tas5086_ratios: [c_int; 6] = [64, 128, 192, 256, 384, 512];

fn index_in_array(array: *const c_int, len: c_int, needle: c_int) -> c_int {
    for i in 0..len {
        unsafe {
            if *array.offset(i as isize) == needle {
                return i;
            }
        }
    }

    unsafe { -ENOENT }
}

unsafe fn tas5086_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut tas5086_private;
    let mut val: c_int;
    let mut ret: c_int;

    (*priv_).rate = params_rate(params);

    /* Look up the sample rate and refer to the offset in the list */
    val = index_in_array(
        tas5086_sample_rates.as_ptr(),
        tas5086_sample_rates.len() as c_int,
        (*priv_).rate,
    );

    if val < 0 {
        dev_err((*component).dev, c"Invalid sample rate\n".as_ptr());
        return -EINVAL;
    }

    ret = regmap_update_bits(
        (*priv_).regmap,
        TAS5086_CLOCK_CONTROL,
        TAS5086_CLOCK_RATE_MASK,
        TAS5086_CLOCK_RATE(val),
    );
    if ret < 0 {
        return ret;
    }

    /* MCLK / Fs ratio */
    val = index_in_array(
        tas5086_ratios.as_ptr(),
        tas5086_ratios.len() as c_int,
        ((*priv_).mclk / (*priv_).rate as c_uint) as c_int,
    );
    if val < 0 {
        dev_err((*component).dev, c"Invalid MCLK / Fs ratio\n".as_ptr());
        return -EINVAL;
    }

    ret = regmap_update_bits(
        (*priv_).regmap,
        TAS5086_CLOCK_CONTROL,
        TAS5086_CLOCK_RATIO_MASK,
        TAS5086_CLOCK_RATIO(val),
    );
    if ret < 0 {
        return ret;
    }

    ret = regmap_update_bits(
        (*priv_).regmap,
        TAS5086_CLOCK_CONTROL,
        TAS5086_CLOCK_SCLK_RATIO_48,
        if (*priv_).sclk == 48 * (*priv_).rate as c_uint {
            TAS5086_CLOCK_SCLK_RATIO_48
        } else {
            0
        },
    );
    if ret < 0 {
        return ret;
    }

    /*
     * The chip has a very unituitive register mapping and muxes information
     * about data format and sample depth into the same register, but not on
     * a logical bit-boundary. Hence, we have to refer to the format passed
     * in the set_dai_fmt() callback and set up everything from here.
     *
     * First, determine the 'base' value, using the format ...
     */
    match (*priv_).format & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_RIGHT_J => val = 0x00,
        x if x == SND_SOC_DAIFMT_I2S => val = 0x03,
        x if x == SND_SOC_DAIFMT_LEFT_J => val = 0x06,
        _ => {
            dev_err((*component).dev, c"Invalid DAI format\n".as_ptr());
            return -EINVAL;
        }
    }

    /* ... then add the offset for the sample bit depth. */
    match params_width(params) {
        16 => {
            val += 0;
        }
        20 => {
            val += 1;
        }
        24 => {
            val += 2;
        }
        _ => {
            dev_err((*component).dev, c"Invalid bit width\n".as_ptr());
            return -EINVAL;
        }
    }

    ret = regmap_write((*priv_).regmap, TAS5086_SERIAL_DATA_IF, val as c_uint);
    if ret < 0 {
        return ret;
    }

    /* clock is considered valid now */
    ret = regmap_update_bits(
        (*priv_).regmap,
        TAS5086_CLOCK_CONTROL,
        TAS5086_CLOCK_VALID,
        TAS5086_CLOCK_VALID,
    );
    if ret < 0 {
        return ret;
    }

    tas5086_set_deemph(component)
}

unsafe fn tas5086_mute_stream(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _stream: c_int,
) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut tas5086_private;
    let mut val: c_uint = 0;

    if mute != 0 {
        val = TAS5086_SOFT_MUTE_ALL;
    }

    regmap_write((*priv_).regmap, TAS5086_SOFT_MUTE, val)
}

unsafe fn tas5086_reset(priv_: *mut tas5086_private) {
    if !(*priv_).reset.is_null() {
        /* Reset codec - minimum assertion time is 400ns */
        gpiod_set_value_cansleep((*priv_).reset, 1);
        udelay(1);
        gpiod_set_value_cansleep((*priv_).reset, 0);

        /* Codec needs ~15ms to wake up */
        msleep(15);
    }
}

/* charge period values in microseconds */
static tas5086_charge_period: [c_int; 24] = [
    13000, 16900, 23400, 31200, 41600, 54600, 72800, 96200, 130000, 156000, 234000, 312000,
    416000, 546000, 728000, 962000, 1300000, 169000, 2340000, 3120000, 4160000, 5460000,
    7280000, 9620000,
];

unsafe fn tas5086_init(dev: *mut device, priv_: *mut tas5086_private) -> c_int {
    let mut ret: c_int;
    let i: c_int;

    /*
     * If any of the channels is configured to start in Mid-Z mode,
     * configure 'part 1' of the PWM starts to use Mid-Z, and tell
     * all configured mid-z channels to start under 'part 1'.
     */
    if (*priv_).pwm_start_mid_z != 0 {
        regmap_write(
            (*priv_).regmap,
            TAS5086_PWM_START,
            TAS5086_PWM_START_MIDZ_FOR_START_1 | (*priv_).pwm_start_mid_z,
        );
    }

    /* lookup and set split-capacitor charge period */
    if (*priv_).charge_period == 0 {
        regmap_write((*priv_).regmap, TAS5086_SPLIT_CAP_CHARGE, 0);
    } else {
        i = index_in_array(
            tas5086_charge_period.as_ptr(),
            tas5086_charge_period.len() as c_int,
            (*priv_).charge_period as c_int,
        );
        if i >= 0 {
            regmap_write((*priv_).regmap, TAS5086_SPLIT_CAP_CHARGE, (i + 0x08) as c_uint);
        } else {
            dev_warn(
                dev,
                c"Invalid split-cap charge period of %d ns.\n".as_ptr(),
                (*priv_).charge_period,
            );
        }
    }

    /* enable factory trim */
    ret = regmap_write((*priv_).regmap, TAS5086_OSC_TRIM, 0x00);
    if ret < 0 {
        return ret;
    }

    /* start all channels */
    ret = regmap_write((*priv_).regmap, TAS5086_SYS_CONTROL_2, 0x20);
    if ret < 0 {
        return ret;
    }

    /* mute all channels for now */
    ret = regmap_write((*priv_).regmap, TAS5086_SOFT_MUTE, TAS5086_SOFT_MUTE_ALL);
    if ret < 0 {
        return ret;
    }

    0
}

/* TAS5086 controls */
// static const DECLARE_TLV_DB_SCALE(tas5086_dac_tlv, -10350, 50, 1);
DECLARE_TLV_DB_SCALE!(tas5086_dac_tlv, -10350, 50, 1);

static tas5086_controls: &[snd_kcontrol_new] = &[
    SOC_SINGLE_TLV!("Master Playback Volume", TAS5086_MASTER_VOL, 0, 0xff, 1, tas5086_dac_tlv),
    SOC_DOUBLE_R_TLV!(
        "Channel 1/2 Playback Volume",
        TAS5086_CHANNEL_VOL(0),
        TAS5086_CHANNEL_VOL(1),
        0,
        0xff,
        1,
        tas5086_dac_tlv
    ),
    SOC_DOUBLE_R_TLV!(
        "Channel 3/4 Playback Volume",
        TAS5086_CHANNEL_VOL(2),
        TAS5086_CHANNEL_VOL(3),
        0,
        0xff,
        1,
        tas5086_dac_tlv
    ),
    SOC_DOUBLE_R_TLV!(
        "Channel 5/6 Playback Volume",
        TAS5086_CHANNEL_VOL(4),
        TAS5086_CHANNEL_VOL(5),
        0,
        0xff,
        1,
        tas5086_dac_tlv
    ),
    SOC_SINGLE_BOOL_EXT!("De-emphasis Switch", 0, tas5086_get_deemph, tas5086_put_deemph),
];

/* Input mux controls */
static tas5086_dapm_sdin_texts: [*const c_char; 8] = [
    c"SDIN1-L".as_ptr(),
    c"SDIN1-R".as_ptr(),
    c"SDIN2-L".as_ptr(),
    c"SDIN2-R".as_ptr(),
    c"SDIN3-L".as_ptr(),
    c"SDIN3-R".as_ptr(),
    c"Ground (0)".as_ptr(),
    c"nc".as_ptr(),
];

static tas5086_dapm_input_mux_enum: &[soc_enum] = &[
    SOC_ENUM_SINGLE!(TAS5086_INPUT_MUX, 20, 8, tas5086_dapm_sdin_texts),
    SOC_ENUM_SINGLE!(TAS5086_INPUT_MUX, 16, 8, tas5086_dapm_sdin_texts),
    SOC_ENUM_SINGLE!(TAS5086_INPUT_MUX, 12, 8, tas5086_dapm_sdin_texts),
    SOC_ENUM_SINGLE!(TAS5086_INPUT_MUX, 8, 8, tas5086_dapm_sdin_texts),
    SOC_ENUM_SINGLE!(TAS5086_INPUT_MUX, 4, 8, tas5086_dapm_sdin_texts),
    SOC_ENUM_SINGLE!(TAS5086_INPUT_MUX, 0, 8, tas5086_dapm_sdin_texts),
];

static tas5086_dapm_input_mux_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_ENUM!("Channel 1 input", tas5086_dapm_input_mux_enum[0]),
    SOC_DAPM_ENUM!("Channel 2 input", tas5086_dapm_input_mux_enum[1]),
    SOC_DAPM_ENUM!("Channel 3 input", tas5086_dapm_input_mux_enum[2]),
    SOC_DAPM_ENUM!("Channel 4 input", tas5086_dapm_input_mux_enum[3]),
    SOC_DAPM_ENUM!("Channel 5 input", tas5086_dapm_input_mux_enum[4]),
    SOC_DAPM_ENUM!("Channel 6 input", tas5086_dapm_input_mux_enum[5]),
];

/* Output mux controls */
static tas5086_dapm_channel_texts: [*const c_char; 6] = [
    c"Channel 1 Mux".as_ptr(),
    c"Channel 2 Mux".as_ptr(),
    c"Channel 3 Mux".as_ptr(),
    c"Channel 4 Mux".as_ptr(),
    c"Channel 5 Mux".as_ptr(),
    c"Channel 6 Mux".as_ptr(),
];

static tas5086_dapm_output_mux_enum: &[soc_enum] = &[
    SOC_ENUM_SINGLE!(TAS5086_PWM_OUTPUT_MUX, 20, 6, tas5086_dapm_channel_texts),
    SOC_ENUM_SINGLE!(TAS5086_PWM_OUTPUT_MUX, 16, 6, tas5086_dapm_channel_texts),
    SOC_ENUM_SINGLE!(TAS5086_PWM_OUTPUT_MUX, 12, 6, tas5086_dapm_channel_texts),
    SOC_ENUM_SINGLE!(TAS5086_PWM_OUTPUT_MUX, 8, 6, tas5086_dapm_channel_texts),
    SOC_ENUM_SINGLE!(TAS5086_PWM_OUTPUT_MUX, 4, 6, tas5086_dapm_channel_texts),
    SOC_ENUM_SINGLE!(TAS5086_PWM_OUTPUT_MUX, 0, 6, tas5086_dapm_channel_texts),
];

static tas5086_dapm_output_mux_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_ENUM!("PWM1 Output", tas5086_dapm_output_mux_enum[0]),
    SOC_DAPM_ENUM!("PWM2 Output", tas5086_dapm_output_mux_enum[1]),
    SOC_DAPM_ENUM!("PWM3 Output", tas5086_dapm_output_mux_enum[2]),
    SOC_DAPM_ENUM!("PWM4 Output", tas5086_dapm_output_mux_enum[3]),
    SOC_DAPM_ENUM!("PWM5 Output", tas5086_dapm_output_mux_enum[4]),
    SOC_DAPM_ENUM!("PWM6 Output", tas5086_dapm_output_mux_enum[5]),
];

static tas5086_dapm_widgets: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_INPUT!("SDIN1-L"),
    SND_SOC_DAPM_INPUT!("SDIN1-R"),
    SND_SOC_DAPM_INPUT!("SDIN2-L"),
    SND_SOC_DAPM_INPUT!("SDIN2-R"),
    SND_SOC_DAPM_INPUT!("SDIN3-L"),
    SND_SOC_DAPM_INPUT!("SDIN3-R"),
    SND_SOC_DAPM_INPUT!("SDIN4-L"),
    SND_SOC_DAPM_INPUT!("SDIN4-R"),
    SND_SOC_DAPM_OUTPUT!("PWM1"),
    SND_SOC_DAPM_OUTPUT!("PWM2"),
    SND_SOC_DAPM_OUTPUT!("PWM3"),
    SND_SOC_DAPM_OUTPUT!("PWM4"),
    SND_SOC_DAPM_OUTPUT!("PWM5"),
    SND_SOC_DAPM_OUTPUT!("PWM6"),
    SND_SOC_DAPM_MUX!("Channel 1 Mux", SND_SOC_NOPM, 0, 0, &tas5086_dapm_input_mux_controls[0]),
    SND_SOC_DAPM_MUX!("Channel 2 Mux", SND_SOC_NOPM, 0, 0, &tas5086_dapm_input_mux_controls[1]),
    SND_SOC_DAPM_MUX!("Channel 3 Mux", SND_SOC_NOPM, 0, 0, &tas5086_dapm_input_mux_controls[2]),
    SND_SOC_DAPM_MUX!("Channel 4 Mux", SND_SOC_NOPM, 0, 0, &tas5086_dapm_input_mux_controls[3]),
    SND_SOC_DAPM_MUX!("Channel 5 Mux", SND_SOC_NOPM, 0, 0, &tas5086_dapm_input_mux_controls[4]),
    SND_SOC_DAPM_MUX!("Channel 6 Mux", SND_SOC_NOPM, 0, 0, &tas5086_dapm_input_mux_controls[5]),
    SND_SOC_DAPM_MUX!("PWM1 Mux", SND_SOC_NOPM, 0, 0, &tas5086_dapm_output_mux_controls[0]),
    SND_SOC_DAPM_MUX!("PWM2 Mux", SND_SOC_NOPM, 0, 0, &tas5086_dapm_output_mux_controls[1]),
    SND_SOC_DAPM_MUX!("PWM3 Mux", SND_SOC_NOPM, 0, 0, &tas5086_dapm_output_mux_controls[2]),
    SND_SOC_DAPM_MUX!("PWM4 Mux", SND_SOC_NOPM, 0, 0, &tas5086_dapm_output_mux_controls[3]),
    SND_SOC_DAPM_MUX!("PWM5 Mux", SND_SOC_NOPM, 0, 0, &tas5086_dapm_output_mux_controls[4]),
    SND_SOC_DAPM_MUX!("PWM6 Mux", SND_SOC_NOPM, 0, 0, &tas5086_dapm_output_mux_controls[5]),
];

static tas5086_dapm_routes: &[snd_soc_dapm_route] = &[
    /* SDIN inputs -> channel muxes */
    route!("Channel 1 Mux", "SDIN1-L", "SDIN1-L"),
    route!("Channel 1 Mux", "SDIN1-R", "SDIN1-R"),
    route!("Channel 1 Mux", "SDIN2-L", "SDIN2-L"),
    route!("Channel 1 Mux", "SDIN2-R", "SDIN2-R"),
    route!("Channel 1 Mux", "SDIN3-L", "SDIN3-L"),
    route!("Channel 1 Mux", "SDIN3-R", "SDIN3-R"),
    route!("Channel 2 Mux", "SDIN1-L", "SDIN1-L"),
    route!("Channel 2 Mux", "SDIN1-R", "SDIN1-R"),
    route!("Channel 2 Mux", "SDIN2-L", "SDIN2-L"),
    route!("Channel 2 Mux", "SDIN2-R", "SDIN2-R"),
    route!("Channel 2 Mux", "SDIN3-L", "SDIN3-L"),
    route!("Channel 2 Mux", "SDIN3-R", "SDIN3-R"),
    route!("Channel 2 Mux", "SDIN1-L", "SDIN1-L"),
    route!("Channel 2 Mux", "SDIN1-R", "SDIN1-R"),
    route!("Channel 2 Mux", "SDIN2-L", "SDIN2-L"),
    route!("Channel 2 Mux", "SDIN2-R", "SDIN2-R"),
    route!("Channel 2 Mux", "SDIN3-L", "SDIN3-L"),
    route!("Channel 2 Mux", "SDIN3-R", "SDIN3-R"),
    route!("Channel 3 Mux", "SDIN1-L", "SDIN1-L"),
    route!("Channel 3 Mux", "SDIN1-R", "SDIN1-R"),
    route!("Channel 3 Mux", "SDIN2-L", "SDIN2-L"),
    route!("Channel 3 Mux", "SDIN2-R", "SDIN2-R"),
    route!("Channel 3 Mux", "SDIN3-L", "SDIN3-L"),
    route!("Channel 3 Mux", "SDIN3-R", "SDIN3-R"),
    route!("Channel 4 Mux", "SDIN1-L", "SDIN1-L"),
    route!("Channel 4 Mux", "SDIN1-R", "SDIN1-R"),
    route!("Channel 4 Mux", "SDIN2-L", "SDIN2-L"),
    route!("Channel 4 Mux", "SDIN2-R", "SDIN2-R"),
    route!("Channel 4 Mux", "SDIN3-L", "SDIN3-L"),
    route!("Channel 4 Mux", "SDIN3-R", "SDIN3-R"),
    route!("Channel 5 Mux", "SDIN1-L", "SDIN1-L"),
    route!("Channel 5 Mux", "SDIN1-R", "SDIN1-R"),
    route!("Channel 5 Mux", "SDIN2-L", "SDIN2-L"),
    route!("Channel 5 Mux", "SDIN2-R", "SDIN2-R"),
    route!("Channel 5 Mux", "SDIN3-L", "SDIN3-L"),
    route!("Channel 5 Mux", "SDIN3-R", "SDIN3-R"),
    route!("Channel 6 Mux", "SDIN1-L", "SDIN1-L"),
    route!("Channel 6 Mux", "SDIN1-R", "SDIN1-R"),
    route!("Channel 6 Mux", "SDIN2-L", "SDIN2-L"),
    route!("Channel 6 Mux", "SDIN2-R", "SDIN2-R"),
    route!("Channel 6 Mux", "SDIN3-L", "SDIN3-L"),
    route!("Channel 6 Mux", "SDIN3-R", "SDIN3-R"),
    /* Channel muxes -> PWM muxes */
    route!("PWM1 Mux", "Channel 1 Mux", "Channel 1 Mux"),
    route!("PWM2 Mux", "Channel 1 Mux", "Channel 1 Mux"),
    route!("PWM3 Mux", "Channel 1 Mux", "Channel 1 Mux"),
    route!("PWM4 Mux", "Channel 1 Mux", "Channel 1 Mux"),
    route!("PWM5 Mux", "Channel 1 Mux", "Channel 1 Mux"),
    route!("PWM6 Mux", "Channel 1 Mux", "Channel 1 Mux"),
    route!("PWM1 Mux", "Channel 2 Mux", "Channel 2 Mux"),
    route!("PWM2 Mux", "Channel 2 Mux", "Channel 2 Mux"),
    route!("PWM3 Mux", "Channel 2 Mux", "Channel 2 Mux"),
    route!("PWM4 Mux", "Channel 2 Mux", "Channel 2 Mux"),
    route!("PWM5 Mux", "Channel 2 Mux", "Channel 2 Mux"),
    route!("PWM6 Mux", "Channel 2 Mux", "Channel 2 Mux"),
    route!("PWM1 Mux", "Channel 3 Mux", "Channel 3 Mux"),
    route!("PWM2 Mux", "Channel 3 Mux", "Channel 3 Mux"),
    route!("PWM3 Mux", "Channel 3 Mux", "Channel 3 Mux"),
    route!("PWM4 Mux", "Channel 3 Mux", "Channel 3 Mux"),
    route!("PWM5 Mux", "Channel 3 Mux", "Channel 3 Mux"),
    route!("PWM6 Mux", "Channel 3 Mux", "Channel 3 Mux"),
    route!("PWM1 Mux", "Channel 4 Mux", "Channel 4 Mux"),
    route!("PWM2 Mux", "Channel 4 Mux", "Channel 4 Mux"),
    route!("PWM3 Mux", "Channel 4 Mux", "Channel 4 Mux"),
    route!("PWM4 Mux", "Channel 4 Mux", "Channel 4 Mux"),
    route!("PWM5 Mux", "Channel 4 Mux", "Channel 4 Mux"),
    route!("PWM6 Mux", "Channel 4 Mux", "Channel 4 Mux"),
    route!("PWM1 Mux", "Channel 5 Mux", "Channel 5 Mux"),
    route!("PWM2 Mux", "Channel 5 Mux", "Channel 5 Mux"),
    route!("PWM3 Mux", "Channel 5 Mux", "Channel 5 Mux"),
    route!("PWM4 Mux", "Channel 5 Mux", "Channel 5 Mux"),
    route!("PWM5 Mux", "Channel 5 Mux", "Channel 5 Mux"),
    route!("PWM6 Mux", "Channel 5 Mux", "Channel 5 Mux"),
    route!("PWM1 Mux", "Channel 6 Mux", "Channel 6 Mux"),
    route!("PWM2 Mux", "Channel 6 Mux", "Channel 6 Mux"),
    route!("PWM3 Mux", "Channel 6 Mux", "Channel 6 Mux"),
    route!("PWM4 Mux", "Channel 6 Mux", "Channel 6 Mux"),
    route!("PWM5 Mux", "Channel 6 Mux", "Channel 6 Mux"),
    route!("PWM6 Mux", "Channel 6 Mux", "Channel 6 Mux"),
    /* The PWM muxes are directly connected to the PWM outputs */
    route_null!("PWM1", "PWM1 Mux"),
    route_null!("PWM2", "PWM2 Mux"),
    route_null!("PWM3", "PWM3 Mux"),
    route_null!("PWM4", "PWM4 Mux"),
    route_null!("PWM5", "PWM5 Mux"),
    route_null!("PWM6", "PWM6 Mux"),
];

static tas5086_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tas5086_hw_params),
    set_sysclk: Some(tas5086_set_dai_sysclk),
    set_fmt: Some(tas5086_set_dai_fmt),
    mute_stream: Some(tas5086_mute_stream),
};

static mut tas5086_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"tas5086-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 6,
        rates: TAS5086_PCM_RATES,
        formats: TAS5086_PCM_FORMATS,
    },
    ops: &tas5086_dai_ops,
};

/* CONFIG_PM conditional in C: suspend/resume callbacks exist only when PM is enabled. */
unsafe fn tas5086_soc_suspend(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut tas5086_private;
    let mut ret: c_int;

    /* Shut down all channels */
    ret = regmap_write((*priv_).regmap, TAS5086_SYS_CONTROL_2, 0x60);
    if ret < 0 {
        return ret;
    }

    regulator_bulk_disable((*priv_).supplies.len() as c_int, (*priv_).supplies.as_mut_ptr());

    0
}

unsafe fn tas5086_soc_resume(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut tas5086_private;
    let mut ret: c_int;

    ret = regulator_bulk_enable((*priv_).supplies.len() as c_int, (*priv_).supplies.as_mut_ptr());
    if ret < 0 {
        return ret;
    }

    tas5086_reset(priv_);
    regcache_mark_dirty((*priv_).regmap);

    ret = tas5086_init((*component).dev, priv_);
    if ret < 0 {
        return ret;
    }

    ret = regcache_sync((*priv_).regmap);
    if ret < 0 {
        return ret;
    }

    0
}

/* CONFIG_OF conditional in C: of_device_id table exists only when OF is enabled. */
static tas5086_dt_ids: &[of_device_id] = &[
    of_device_id {
        compatible: c"ti,tas5086".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
MODULE_DEVICE_TABLE!(of, tas5086_dt_ids);

unsafe fn tas5086_probe(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut tas5086_private;
    let mut i: c_int;
    let mut ret: c_int;

    ret = regulator_bulk_enable((*priv_).supplies.len() as c_int, (*priv_).supplies.as_mut_ptr());
    if ret < 0 {
        dev_err((*component).dev, c"Failed to enable regulators: %d\n".as_ptr(), ret);
        return ret;
    }

    (*priv_).pwm_start_mid_z = 0;
    (*priv_).charge_period = 1300000; /* hardware default is 1300 ms */

    if !of_match_device(of_match_ptr(tas5086_dt_ids.as_ptr()), (*component).dev).is_null() {
        let of_node = (*(*component).dev).of_node;

        of_property_read_u32(of_node, c"ti,charge-period".as_ptr(), &mut (*priv_).charge_period);

        i = 0;
        while i < 6 {
            let mut name = [0i8; 25];

            snprintf(
                name.as_mut_ptr(),
                name.len(),
                c"ti,mid-z-channel-%d".as_ptr(),
                i + 1,
            );

            if of_property_read_bool(of_node, name.as_ptr()) {
                (*priv_).pwm_start_mid_z |= 1 << i;
            }
            i += 1;
        }
    }

    tas5086_reset(priv_);
    ret = tas5086_init((*component).dev, priv_);
    if ret < 0 {
        regulator_bulk_disable((*priv_).supplies.len() as c_int, (*priv_).supplies.as_mut_ptr());
        return ret;
    }

    /* set master volume to 0 dB */
    ret = regmap_write((*priv_).regmap, TAS5086_MASTER_VOL, 0x30);
    if ret < 0 {
        regulator_bulk_disable((*priv_).supplies.len() as c_int, (*priv_).supplies.as_mut_ptr());
        return ret;
    }

    0
}

unsafe fn tas5086_remove(component: *mut snd_soc_component) {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut tas5086_private;

    if !(*priv_).reset.is_null() {
        /* Set codec to the reset state */
        gpiod_set_value_cansleep((*priv_).reset, 1);
    }

    regulator_bulk_disable((*priv_).supplies.len() as c_int, (*priv_).supplies.as_mut_ptr());
}

static soc_component_dev_tas5086: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tas5086_probe),
    remove: Some(tas5086_remove),
    suspend: Some(tas5086_soc_suspend),
    resume: Some(tas5086_soc_resume),
    controls: tas5086_controls.as_ptr(),
    num_controls: tas5086_controls.len(),
    dapm_widgets: tas5086_dapm_widgets.as_ptr(),
    num_dapm_widgets: tas5086_dapm_widgets.len(),
    dapm_routes: tas5086_dapm_routes.as_ptr(),
    num_dapm_routes: tas5086_dapm_routes.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static tas5086_i2c_id: &[i2c_device_id] = &[
    i2c_device_id {
        name: c"tas5086".as_ptr(),
    },
    i2c_device_id { name: ptr::null() },
];
MODULE_DEVICE_TABLE!(i2c, tas5086_i2c_id);

static tas5086_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 32,
    max_register: TAS5086_MAX_REGISTER,
    reg_defaults: tas5086_reg_defaults.as_ptr(),
    num_reg_defaults: tas5086_reg_defaults.len(),
    cache_type: REGCACHE_RBTREE,
    volatile_reg: Some(tas5086_volatile_reg),
    writeable_reg: Some(tas5086_writeable_reg),
    readable_reg: Some(tas5086_accessible_reg),
    reg_read: Some(tas5086_reg_read),
    reg_write: Some(tas5086_reg_write),
};

unsafe fn tas5086_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let priv_: *mut tas5086_private;
    let dev = &mut (*i2c).dev as *mut device;
    let mut i: c_int = 0;
    let mut ret: c_int;

    priv_ = devm_kzalloc(dev, size_of::<tas5086_private>(), GFP_KERNEL) as *mut tas5086_private;
    if priv_.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while (i as usize) < supply_names.len() {
        (*priv_).supplies[i as usize].supply = supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, (*priv_).supplies.len() as c_int, (*priv_).supplies.as_mut_ptr());
    if ret < 0 {
        dev_err(dev, c"Failed to get regulators: %d\n".as_ptr(), ret);
        return ret;
    }

    (*priv_).regmap = devm_regmap_init(dev, ptr::null_mut(), i2c as *mut c_void, &tas5086_regmap);
    if IS_ERR((*priv_).regmap as *const c_void) {
        ret = PTR_ERR((*priv_).regmap as *const c_void) as c_int;
        dev_err(&mut (*i2c).dev, c"Failed to create regmap: %d\n".as_ptr(), ret);
        return ret;
    }

    i2c_set_clientdata(i2c, priv_ as *mut c_void);

    /* Request line asserted */
    (*priv_).reset = devm_gpiod_get_optional(dev, c"reset".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*priv_).reset as *const c_void) {
        return PTR_ERR((*priv_).reset as *const c_void) as c_int;
    }
    gpiod_set_consumer_name((*priv_).reset, c"TAS5086 Reset".as_ptr());

    ret = regulator_bulk_enable((*priv_).supplies.len() as c_int, (*priv_).supplies.as_mut_ptr());
    if ret < 0 {
        dev_err(dev, c"Failed to enable regulators: %d\n".as_ptr(), ret);
        return ret;
    }

    tas5086_reset(priv_);

    /* The TAS5086 always returns 0x03 in its TAS5086_DEV_ID register */
    ret = regmap_read((*priv_).regmap, TAS5086_DEV_ID, &mut i);
    if ret == 0 && i != 0x3 {
        dev_err(dev, c"Failed to identify TAS5086 codec (got %02x)\n".as_ptr(), i);
        ret = -ENODEV;
    }

    /*
     * The chip has been identified, so we can turn off the power
     * again until the dai link is set up.
     */
    regulator_bulk_disable((*priv_).supplies.len() as c_int, (*priv_).supplies.as_mut_ptr());

    if ret == 0 {
        ret = devm_snd_soc_register_component(
            &mut (*i2c).dev,
            &soc_component_dev_tas5086,
            &mut tas5086_dai,
            1,
        );
    }

    ret
}

static mut tas5086_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"tas5086".as_ptr(),
        of_match_table: of_match_ptr(tas5086_dt_ids.as_ptr()),
    },
    id_table: tas5086_i2c_id.as_ptr(),
    probe: Some(tas5086_i2c_probe),
};

module_i2c_driver!(tas5086_i2c_driver);

MODULE_AUTHOR!("Daniel Mack <zonque@gmail.com>");
MODULE_DESCRIPTION!("Texas Instruments TAS5086 ALSA SoC Codec Driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
