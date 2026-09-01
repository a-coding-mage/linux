// SPDX-License-Identifier: GPL-2.0
/*
 * CS4270 ALSA SoC (ASoC) codec driver
 *
 * Author: Timur Tabi <timur@freescale.com>
 *
 * Copyright 2007-2009 Freescale Semiconductor, Inc.
 *
 * This is an ASoC device driver for the Cirrus Logic CS4270 codec.
 *
 * Current features/limitations:
 *
 * - Software mode is supported.  Stand-alone mode is not supported.
 * - Only I2C is supported, not SPI
 * - Support for master and slave mode
 * - The machine driver's 'startup' function must call
 *   cs4270_set_dai_sysclk() with the value of MCLK.
 * - Only I2S and left-justified modes are supported
 * - Power management is supported
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = core::ffi::c_uchar;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
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
pub struct snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_int; 2],
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;

const SNDRV_PCM_FMTBIT_S8: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S18_3LE: c_uint = 1 << 2;
const SNDRV_PCM_FMTBIT_S20_3LE: c_uint = 1 << 3;
const SNDRV_PCM_FMTBIT_S24_3LE: c_uint = 1 << 4;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 1 << 5;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 30;

const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 2;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0x0100;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0x0200;

const CS4270_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_LE;

/* CS4270 registers addresses */
const CS4270_CHIPID: c_uint = 0x01; /* Chip ID */
const CS4270_PWRCTL: c_uint = 0x02; /* Power Control */
const CS4270_MODE: c_uint = 0x03; /* Mode Control */
const CS4270_FORMAT: c_uint = 0x04; /* Serial Format, ADC/DAC Control */
const CS4270_TRANS: c_uint = 0x05; /* Transition Control */
const CS4270_MUTE: c_uint = 0x06; /* Mute Control */
const CS4270_VOLA: c_uint = 0x07; /* DAC Channel A Volume Control */
const CS4270_VOLB: c_uint = 0x08; /* DAC Channel B Volume Control */

const CS4270_FIRSTREG: c_uint = 0x01;
const CS4270_LASTREG: c_uint = 0x08;
const CS4270_NUMREGS: c_uint = CS4270_LASTREG - CS4270_FIRSTREG + 1;
const CS4270_I2C_INCR: c_uint = 0x80;

/* Bit masks for the CS4270 registers */
const CS4270_CHIPID_ID: c_int = 0xF0;
const CS4270_CHIPID_REV: c_int = 0x0F;
const CS4270_PWRCTL_FREEZE: c_int = 0x80;
const CS4270_PWRCTL_PDN_ADC: c_int = 0x20;
const CS4270_PWRCTL_PDN_DAC: c_int = 0x02;
const CS4270_PWRCTL_PDN: c_int = 0x01;
const CS4270_PWRCTL_PDN_ALL: c_int =
    CS4270_PWRCTL_PDN_ADC | CS4270_PWRCTL_PDN_DAC | CS4270_PWRCTL_PDN;
const CS4270_MODE_SPEED_MASK: c_int = 0x30;
const CS4270_MODE_1X: u8 = 0x00;
const CS4270_MODE_2X: u8 = 0x10;
const CS4270_MODE_4X: u8 = 0x20;
const CS4270_MODE_SLAVE: c_int = 0x30;
const CS4270_MODE_DIV_MASK: c_int = 0x0E;
const CS4270_MODE_DIV1: u8 = 0x00;
const CS4270_MODE_DIV15: u8 = 0x02;
const CS4270_MODE_DIV2: u8 = 0x04;
const CS4270_MODE_DIV3: u8 = 0x06;
const CS4270_MODE_DIV4: u8 = 0x08;
const CS4270_MODE_POPGUARD: c_int = 0x01;
const CS4270_FORMAT_FREEZE_A: c_int = 0x80;
const CS4270_FORMAT_FREEZE_B: c_int = 0x40;
const CS4270_FORMAT_LOOPBACK: c_int = 0x20;
const CS4270_FORMAT_DAC_MASK: c_int = 0x18;
const CS4270_FORMAT_DAC_LJ: c_int = 0x00;
const CS4270_FORMAT_DAC_I2S: c_int = 0x08;
const CS4270_FORMAT_DAC_RJ16: c_int = 0x18;
const CS4270_FORMAT_DAC_RJ24: c_int = 0x10;
const CS4270_FORMAT_ADC_MASK: c_int = 0x01;
const CS4270_FORMAT_ADC_LJ: c_int = 0x00;
const CS4270_FORMAT_ADC_I2S: c_int = 0x01;
const CS4270_TRANS_ONE_VOL: c_int = 0x80;
const CS4270_TRANS_SOFT: c_int = 0x40;
const CS4270_TRANS_ZERO: c_int = 0x20;
const CS4270_TRANS_INV_ADC_A: c_int = 0x08;
const CS4270_TRANS_INV_ADC_B: c_int = 0x10;
const CS4270_TRANS_INV_DAC_A: c_int = 0x02;
const CS4270_TRANS_INV_DAC_B: c_int = 0x04;
const CS4270_TRANS_DEEMPH: c_int = 0x01;
const CS4270_MUTE_AUTO: c_int = 0x20;
const CS4270_MUTE_ADC_A: c_int = 0x08;
const CS4270_MUTE_ADC_B: c_int = 0x10;
const CS4270_MUTE_POLARITY: c_int = 0x04;
const CS4270_MUTE_DAC_A: c_int = 0x01;
const CS4270_MUTE_DAC_B: c_int = 0x02;

/* Power-on default values for the registers
 *
 * This array contains the power-on default values of the registers, with the
 * exception of the "CHIPID" register (01h).  The lower four bits of that
 * register contain the hardware revision, so it is treated as volatile.
 */
static cs4270_reg_defaults: [reg_default; 7] = [
    reg_default { reg: 2, def: 0x00 },
    reg_default { reg: 3, def: 0x30 },
    reg_default { reg: 4, def: 0x00 },
    reg_default { reg: 5, def: 0x60 },
    reg_default { reg: 6, def: 0x20 },
    reg_default { reg: 7, def: 0x00 },
    reg_default { reg: 8, def: 0x00 },
];

static supply_names: [*const c_char; 3] = [
    b"va\0".as_ptr() as *const c_char,
    b"vd\0".as_ptr() as *const c_char,
    b"vlc\0".as_ptr() as *const c_char,
];

/* Private data for the CS4270 */
#[repr(C)]
pub struct cs4270_private {
    pub regmap: *mut regmap,
    pub mclk: c_uint, /* Input frequency of the MCLK pin */
    pub mode: c_uint, /* The mode (I2S or left-justified) */
    pub slave_mode: c_uint,
    pub manual_mute: c_uint,

    /* power domain regulators */
    pub supplies: [regulator_bulk_data; 3],

    /* reset gpio */
    pub reset_gpio: *mut gpio_desc,
}

/*
 * Static DAPM widgets, routes, controls, DAI driver, component driver, OF
 * matches, regmap config, I2C IDs, and I2C driver are direct translations of
 * Linux macro-populated objects in the source C file. Their concrete Rust
 * layouts depend on external kernel bindings supplied by the final tree.
 */

/**
 * struct cs4270_mode_ratios - clock ratio tables
 * @ratio: the ratio of MCLK to the sample rate
 * @speed_mode: the Speed Mode bits to set in the Mode Control register for
 *              this ratio
 * @mclk: the Ratio Select bits to set in the Mode Control register for this
 *        ratio
 *
 * The data for this chart is taken from Table 5 of the CS4270 reference
 * manual.
 *
 * This table is used to determine how to program the Mode Control register.
 * It is also used by cs4270_set_dai_sysclk() to tell ALSA which sampling
 * rates the CS4270 currently supports.
 *
 * @speed_mode is the corresponding bit pattern to be written to the
 * MODE bits of the Mode Control Register
 *
 * @mclk is the corresponding bit pattern to be wirten to the MCLK bits of
 * the Mode Control Register.
 *
 * In situations where a single ratio is represented by multiple speed
 * modes, we favor the slowest speed.  E.g, for a ratio of 128, we pick
 * double-speed instead of quad-speed.  However, the CS4270 errata states
 * that divide-By-1.5 can cause failures, so we avoid that mode where
 * possible.
 *
 * Errata: There is an errata for the CS4270 where divide-by-1.5 does not
 * work if Vd is 3.3V.  If this effects you, select the
 * CONFIG_SND_SOC_CS4270_VD33_ERRATA Kconfig option, and the driver will
 * never select any sample rates that require divide-by-1.5.
 */
#[repr(C)]
pub struct cs4270_mode_ratios {
    pub ratio: c_uint,
    pub speed_mode: u8,
    pub mclk: u8,
}

static mut cs4270_mode_ratios: [cs4270_mode_ratios; 9] = [
    cs4270_mode_ratios { ratio: 64, speed_mode: CS4270_MODE_4X, mclk: CS4270_MODE_DIV1 },
    /* Excluded in C when CONFIG_SND_SOC_CS4270_VD33_ERRATA is enabled. */
    cs4270_mode_ratios { ratio: 96, speed_mode: CS4270_MODE_4X, mclk: CS4270_MODE_DIV15 },
    cs4270_mode_ratios { ratio: 128, speed_mode: CS4270_MODE_2X, mclk: CS4270_MODE_DIV1 },
    cs4270_mode_ratios { ratio: 192, speed_mode: CS4270_MODE_4X, mclk: CS4270_MODE_DIV3 },
    cs4270_mode_ratios { ratio: 256, speed_mode: CS4270_MODE_1X, mclk: CS4270_MODE_DIV1 },
    cs4270_mode_ratios { ratio: 384, speed_mode: CS4270_MODE_2X, mclk: CS4270_MODE_DIV3 },
    cs4270_mode_ratios { ratio: 512, speed_mode: CS4270_MODE_1X, mclk: CS4270_MODE_DIV2 },
    cs4270_mode_ratios { ratio: 768, speed_mode: CS4270_MODE_1X, mclk: CS4270_MODE_DIV3 },
    cs4270_mode_ratios { ratio: 1024, speed_mode: CS4270_MODE_1X, mclk: CS4270_MODE_DIV4 },
];

/* The number of MCLK/LRCK ratios supported by the CS4270 */
const NUM_MCLK_RATIOS: usize = 9;

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_int) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_int,
        val: c_int,
    ) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn ndelay(nsecs: c_uint);
    fn regcache_sync(map: *mut regmap) -> c_int;
}

static mut current_i2c_client_data: *mut c_void = core::ptr::null_mut();

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub addr: c_uint,
}

unsafe extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_uint,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const c_void) -> *mut regmap;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const c_void,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

static cs4270_regmap: *const c_void = core::ptr::null();
static soc_component_device_cs4270: *const c_void = core::ptr::null();
static mut cs4270_dai: *mut c_void = core::ptr::null_mut();

unsafe fn cs4270_reg_is_readable(_dev: *mut device, reg: c_uint) -> bool {
    (reg >= CS4270_FIRSTREG) && (reg <= CS4270_LASTREG)
}

unsafe fn cs4270_reg_is_volatile(_dev: *mut device, reg: c_uint) -> bool {
    /* Unreadable registers are considered volatile */
    if (reg < CS4270_FIRSTREG) || (reg > CS4270_LASTREG) {
        return true;
    }

    reg == CS4270_CHIPID
}

/**
 * cs4270_set_dai_sysclk - determine the CS4270 samples rates.
 * @codec_dai: the codec DAI
 * @clk_id: the clock ID (ignored)
 * @freq: the MCLK input frequency
 * @dir: the clock direction (ignored)
 *
 * This function is used to tell the codec driver what the input MCLK
 * frequency is.
 */
unsafe fn cs4270_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let cs4270 = snd_soc_component_get_drvdata(component) as *mut cs4270_private;

    (*cs4270).mclk = freq;
    0
}

unsafe fn cs4270_set_dai_fmt(codec_dai: *mut snd_soc_dai, format: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let cs4270 = snd_soc_component_get_drvdata(component) as *mut cs4270_private;

    /* set DAI format */
    match format & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_LEFT_J => {
            (*cs4270).mode = format & SND_SOC_DAIFMT_FORMAT_MASK;
        }
        _ => {
            dev_err((*component).dev, b"invalid dai format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    /* set master/slave audio interface */
    match format & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {
            (*cs4270).slave_mode = 1;
        }
        SND_SOC_DAIFMT_CBP_CFP => {
            (*cs4270).slave_mode = 0;
        }
        _ => {
            /* all other modes are unsupported by the hardware */
            dev_err(
                (*component).dev,
                b"Unknown master/slave configuration\n\0".as_ptr() as *const c_char,
            );
            return -EINVAL;
        }
    }

    0
}

unsafe fn cs4270_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let cs4270 = snd_soc_component_get_drvdata(component) as *mut cs4270_private;
    let mut ret: c_int;
    let mut i: c_uint;
    let rate: c_uint;
    let ratio: c_uint;
    let mut reg: c_int;

    /* Figure out which MCLK/LRCK ratio to use */

    rate = params_rate(params); /* Sampling rate, in Hz */
    ratio = (*cs4270).mclk / rate; /* MCLK/LRCK ratio */

    i = 0;
    while (i as usize) < NUM_MCLK_RATIOS {
        if cs4270_mode_ratios[i as usize].ratio == ratio {
            break;
        }
        i += 1;
    }

    if (i as usize) == NUM_MCLK_RATIOS {
        /* We did not find a matching ratio */
        dev_err((*component).dev, b"could not find matching ratio\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    /* Set the sample rate */

    reg = snd_soc_component_read(component, CS4270_MODE);
    reg &= !(CS4270_MODE_SPEED_MASK | CS4270_MODE_DIV_MASK);
    reg |= cs4270_mode_ratios[i as usize].mclk as c_int;

    if (*cs4270).slave_mode != 0 {
        reg |= CS4270_MODE_SLAVE;
    } else {
        reg |= cs4270_mode_ratios[i as usize].speed_mode as c_int;
    }

    ret = snd_soc_component_write(component, CS4270_MODE, reg);
    if ret < 0 {
        dev_err((*component).dev, b"i2c write failed\n\0".as_ptr() as *const c_char);
        return ret;
    }

    /* Set the DAI format */

    reg = snd_soc_component_read(component, CS4270_FORMAT);
    reg &= !(CS4270_FORMAT_DAC_MASK | CS4270_FORMAT_ADC_MASK);

    match (*cs4270).mode {
        SND_SOC_DAIFMT_I2S => {
            reg |= CS4270_FORMAT_DAC_I2S | CS4270_FORMAT_ADC_I2S;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            reg |= CS4270_FORMAT_DAC_LJ | CS4270_FORMAT_ADC_LJ;
        }
        _ => {
            dev_err((*component).dev, b"unknown dai format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    ret = snd_soc_component_write(component, CS4270_FORMAT, reg);
    if ret < 0 {
        dev_err((*component).dev, b"i2c write failed\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ret
}

unsafe fn cs4270_dai_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let cs4270 = snd_soc_component_get_drvdata(component) as *mut cs4270_private;
    let mut reg6: c_int;

    reg6 = snd_soc_component_read(component, CS4270_MUTE);

    if mute != 0 {
        reg6 |= CS4270_MUTE_DAC_A | CS4270_MUTE_DAC_B;
    } else {
        reg6 &= !(CS4270_MUTE_DAC_A | CS4270_MUTE_DAC_B);
        reg6 |= (*cs4270).manual_mute as c_int;
    }

    snd_soc_component_write(component, CS4270_MUTE, reg6)
}

unsafe fn cs4270_soc_put_mute(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let cs4270 = snd_soc_component_get_drvdata(component) as *mut cs4270_private;
    let left = ((*ucontrol).value.integer.value[0] == 0) as c_int;
    let right = ((*ucontrol).value.integer.value[1] == 0) as c_int;

    (*cs4270).manual_mute = (if left != 0 { CS4270_MUTE_DAC_A } else { 0 }
        | if right != 0 { CS4270_MUTE_DAC_B } else { 0 }) as c_uint;

    snd_soc_put_volsw(kcontrol, ucontrol)
}

unsafe fn cs4270_probe(component: *mut snd_soc_component) -> c_int {
    let cs4270 = snd_soc_component_get_drvdata(component) as *mut cs4270_private;
    let mut ret: c_int;

    /* Disable auto-mute.  This feature appears to be buggy.  In some
     * situations, auto-mute will not deactivate when it should, so we want
     * this feature disabled by default.  An application (e.g. alsactl) can
     * re-enabled it by using the controls.
     */
    ret = snd_soc_component_update_bits(component, CS4270_MUTE, CS4270_MUTE_AUTO, 0);
    if ret < 0 {
        dev_err((*component).dev, b"i2c write failed\n\0".as_ptr() as *const c_char);
        return ret;
    }

    /* Disable automatic volume control.  The hardware enables, and it
     * causes volume change commands to be delayed, sometimes until after
     * playback has started.  An application (e.g. alsactl) can
     * re-enabled it by using the controls.
     */
    ret = snd_soc_component_update_bits(
        component,
        CS4270_TRANS,
        CS4270_TRANS_SOFT | CS4270_TRANS_ZERO,
        0,
    );
    if ret < 0 {
        dev_err((*component).dev, b"i2c write failed\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ret = regulator_bulk_enable((*cs4270).supplies.len() as c_uint, (*cs4270).supplies.as_mut_ptr());

    ret
}

unsafe fn cs4270_remove(component: *mut snd_soc_component) {
    let cs4270 = snd_soc_component_get_drvdata(component) as *mut cs4270_private;

    regulator_bulk_disable((*cs4270).supplies.len() as c_uint, (*cs4270).supplies.as_mut_ptr());
}

/* CONFIG_PM */

unsafe fn cs4270_soc_suspend(component: *mut snd_soc_component) -> c_int {
    let cs4270 = snd_soc_component_get_drvdata(component) as *mut cs4270_private;
    let mut reg: c_int;
    let ret: c_int;

    reg = snd_soc_component_read(component, CS4270_PWRCTL) | CS4270_PWRCTL_PDN_ALL;
    if reg < 0 {
        return reg;
    }

    ret = snd_soc_component_write(component, CS4270_PWRCTL, reg);
    if ret < 0 {
        return ret;
    }

    regulator_bulk_disable((*cs4270).supplies.len() as c_uint, (*cs4270).supplies.as_mut_ptr());

    0
}

unsafe fn cs4270_soc_resume(component: *mut snd_soc_component) -> c_int {
    let cs4270 = snd_soc_component_get_drvdata(component) as *mut cs4270_private;
    let mut reg: c_int;
    let ret: c_int;

    ret = regulator_bulk_enable((*cs4270).supplies.len() as c_uint, (*cs4270).supplies.as_mut_ptr());
    if ret != 0 {
        return ret;
    }

    /* In case the device was put to hard reset during sleep, we need to
     * wait 500ns here before any I2C communication. */
    ndelay(500);

    /* first restore the entire register cache ... */
    regcache_sync((*cs4270).regmap);

    /* ... then disable the power-down bits */
    reg = snd_soc_component_read(component, CS4270_PWRCTL);
    reg &= !CS4270_PWRCTL_PDN_ALL;

    snd_soc_component_write(component, CS4270_PWRCTL, reg)
}

/**
 * cs4270_i2c_remove - deinitialize the I2C interface of the CS4270
 * @i2c_client: the I2C client object
 *
 * This function puts the chip into low power mode when the i2c device
 * is removed.
 */
unsafe fn cs4270_i2c_remove(i2c_client: *mut i2c_client) {
    let cs4270 = i2c_get_clientdata(i2c_client) as *mut cs4270_private;

    gpiod_set_value_cansleep((*cs4270).reset_gpio, 0);
}

/**
 * cs4270_i2c_probe - initialize the I2C interface of the CS4270
 * @i2c_client: the I2C client object
 *
 * This function is called whenever the I2C subsystem finds a device that
 * matches the device ID given via a prior call to i2c_add_driver().
 */
unsafe fn cs4270_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let mut cs4270: *mut cs4270_private;
    let mut val: c_uint = 0;
    let mut ret: c_int;
    let mut i: c_int;

    cs4270 = devm_kzalloc(
        &mut (*i2c_client).dev,
        core::mem::size_of::<cs4270_private>(),
        GFP_KERNEL,
    ) as *mut cs4270_private;
    if cs4270.is_null() {
        return -ENOMEM;
    }

    /* get the power supply regulators */
    i = 0;
    while (i as usize) < supply_names.len() {
        (*cs4270).supplies[i as usize].supply = supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        &mut (*i2c_client).dev,
        (*cs4270).supplies.len() as c_uint,
        (*cs4270).supplies.as_mut_ptr(),
    );
    if ret < 0 {
        return ret;
    }

    /* reset the device */
    (*cs4270).reset_gpio = devm_gpiod_get_optional(
        &mut (*i2c_client).dev,
        b"reset\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR((*cs4270).reset_gpio as *const c_void) {
        dev_dbg(
            &mut (*i2c_client).dev,
            b"Error getting CS4270 reset GPIO\n\0".as_ptr() as *const c_char,
        );
        return PTR_ERR((*cs4270).reset_gpio as *const c_void);
    }

    if !(*cs4270).reset_gpio.is_null() {
        dev_dbg(
            &mut (*i2c_client).dev,
            b"Found reset GPIO\n\0".as_ptr() as *const c_char,
        );
        gpiod_set_value_cansleep((*cs4270).reset_gpio, 1);
    }

    /* Sleep 500ns before i2c communications */
    ndelay(500);

    (*cs4270).regmap = devm_regmap_init_i2c(i2c_client, cs4270_regmap);
    if IS_ERR((*cs4270).regmap as *const c_void) {
        return PTR_ERR((*cs4270).regmap as *const c_void);
    }

    /* Verify that we have a CS4270 */
    ret = regmap_read((*cs4270).regmap, CS4270_CHIPID, &mut val);
    if ret < 0 {
        dev_err(
            &mut (*i2c_client).dev,
            b"failed to read i2c at addr %X\n\0".as_ptr() as *const c_char,
            (*i2c_client).addr,
        );
        return ret;
    }
    /* The top four bits of the chip ID should be 1100. */
    if (val & 0xF0) != 0xC0 {
        dev_err(
            &mut (*i2c_client).dev,
            b"device at addr %X is not a CS4270\n\0".as_ptr() as *const c_char,
            (*i2c_client).addr,
        );
        return -ENODEV;
    }

    dev_info(
        &mut (*i2c_client).dev,
        b"found device at i2c address %X\n\0".as_ptr() as *const c_char,
        (*i2c_client).addr,
    );
    dev_info(
        &mut (*i2c_client).dev,
        b"hardware revision %X\n\0".as_ptr() as *const c_char,
        val & 0xF,
    );

    i2c_set_clientdata(i2c_client, cs4270 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*i2c_client).dev,
        soc_component_device_cs4270,
        cs4270_dai,
        1,
    );
    ret
}

/*
 * module_i2c_driver(cs4270_i2c_driver);
 *
 * MODULE_AUTHOR("Timur Tabi <timur@freescale.com>");
 * MODULE_DESCRIPTION("Cirrus Logic CS4270 ALSA SoC Codec Driver");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
