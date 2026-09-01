// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * TAS571x amplifier audio driver
 *
 * Copyright (C) 2015 Google, Inc.
 * Copyright (c) 2013 Daniel Mack <zonque@gmail.com>
 *
 * TAS5721 support:
 * Copyright (C) 2016 Petr Kulhavy, Barix AG <petr@barix.com>
 *
 * TAS5707 support:
 * Copyright (C) 2018 Jerome Brunet, Baylibre SAS <jbrunet@baylibre.com>
 */

// C dependencies translated from:
// <linux/clk.h>, <linux/delay.h>, <linux/device.h>, <linux/gpio/consumer.h>,
// <linux/i2c.h>, <linux/init.h>, <linux/kernel.h>, <linux/module.h>,
// <linux/of.h>, <linux/regmap.h>, <linux/regulator/consumer.h>,
// <linux/stddef.h>, <sound/pcm_params.h>, <sound/soc.h>, <sound/tlv.h>,
// <linux/unaligned.h>, and "tas571x.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const TAS571X_MAX_SUPPLIES: usize = 6;

#[repr(C)]
pub struct tas571x_chip {
    supply_names: *const *const c_char,
    num_supply_names: c_int,
    controls: *const snd_kcontrol_new,
    num_controls: c_int,
    regmap_config: *const regmap_config,
    vol_reg_size: c_int,
}

#[repr(C)]
pub struct tas571x_private {
    chip: *const tas571x_chip,
    regmap: *mut regmap,
    supplies: [regulator_bulk_data; TAS571X_MAX_SUPPLIES],
    mclk: *mut clk,
    format: c_uint,
    reset_gpio: *mut gpio_desc,
    pdn_gpio: *mut gpio_desc,
    component_driver: snd_soc_component_driver,
}

#[allow(non_camel_case_types)]
type size_t = usize;
#[allow(non_camel_case_types)]
type u8 = u8;
#[allow(non_camel_case_types)]
type u32 = u32;
#[allow(non_camel_case_types)]
type kernel_ulong_t = c_ulong;

#[repr(C)]
pub struct i2c_client {
    dev: device,
    adapter: *mut i2c_adapter,
    addr: u16,
}

#[repr(C)]
pub struct i2c_msg {
    addr: u16,
    flags: u16,
    len: u16,
    buf: *mut u8,
}

#[repr(C)]
pub struct regulator_bulk_data {
    supply: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol {
    private_value: c_long,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    type_: c_uint,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    min: c_long,
    max: c_long,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

extern "C" {
    static tas5711_volume_tlv: [c_uint; 0];
    static tas5707_volume_tlv: [c_uint; 0];
    static tas5717_volume_tlv: [c_uint; 0];

    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_master_send(client: *mut i2c_client, buf: *const u8, count: c_int) -> c_int;
    fn i2c_transfer(adapter: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut u8;
    fn kfree(ptr: *const c_void);
    fn put_unaligned_be32(val: c_long, p: *mut u8);
    fn get_unaligned_be32(p: *const u8) -> c_long;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn to_i2c_client(dev: *mut device) -> *mut i2c_client;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn i2c_get_match_data(client: *mut i2c_client) -> *const c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn WARN_ON(condition: bool) -> bool;
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_int,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const c_void,
        bus_context: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

unsafe fn tas571x_register_size(priv_: *mut tas571x_private, reg: c_uint) -> c_int {
    match reg {
        TAS571X_MVOL_REG | TAS571X_CH1_VOL_REG | TAS571X_CH2_VOL_REG => {
            (*(*priv_).chip).vol_reg_size
        }
        TAS571X_INPUT_MUX_REG
        | TAS571X_CH4_SRC_SELECT_REG
        | TAS571X_PWM_MUX_REG
        | TAS5717_CH1_RIGHT_CH_MIX_REG
        | TAS5717_CH1_LEFT_CH_MIX_REG
        | TAS5717_CH2_LEFT_CH_MIX_REG
        | TAS5717_CH2_RIGHT_CH_MIX_REG => 4,
        _ => 1,
    }
}

unsafe extern "C" fn tas571x_reg_write(
    context: *mut c_void,
    reg: c_uint,
    mut value: c_uint,
) -> c_int {
    let client = context as *mut i2c_client;
    let priv_ = i2c_get_clientdata(client) as *mut tas571x_private;
    let mut buf = [0u8; 5];
    let ret: c_int;

    let size = tas571x_register_size(priv_, reg) as c_uint;
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

unsafe extern "C" fn tas571x_reg_read(
    context: *mut c_void,
    reg: c_uint,
    value: *mut c_uint,
) -> c_int {
    let client = context as *mut i2c_client;
    let priv_ = i2c_get_clientdata(client) as *mut tas571x_private;
    let mut send_buf: u8;
    let mut recv_buf = [0u8; 4];
    let mut msgs: [i2c_msg; 2] = core::mem::zeroed();
    let ret: c_int;

    let size = tas571x_register_size(priv_, reg) as c_uint;
    send_buf = reg as u8;

    msgs[0].addr = (*client).addr;
    msgs[0].len = size_of::<u8>() as u16;
    msgs[0].buf = &mut send_buf;
    msgs[0].flags = 0;

    msgs[1].addr = (*client).addr;
    msgs[1].len = size as u16;
    msgs[1].buf = recv_buf.as_mut_ptr();
    msgs[1].flags = I2C_M_RD;

    ret = i2c_transfer((*client).adapter, msgs.as_mut_ptr(), ARRAY_SIZE(&msgs) as c_int);
    if ret < 0 {
        return ret;
    } else if ret != ARRAY_SIZE(&msgs) as c_int {
        return -EIO;
    }

    *value = 0;

    let mut i = 0;
    while i < size {
        *value <<= 8;
        *value |= recv_buf[i as usize] as c_uint;
        i += 1;
    }

    0
}

/*
 * register write for 8- and 20-byte registers
 */
unsafe fn tas571x_reg_write_multiword(
    client: *mut i2c_client,
    reg: c_uint,
    values: *const c_long,
    len: size_t,
) -> c_int {
    let mut i: size_t;
    let mut p: *mut u8;
    let ret: c_int;
    let send_size: size_t = 1 + len * size_of::<u32>();

    let buf = kzalloc(send_size, GFP_KERNEL | GFP_DMA);
    if buf.is_null() {
        return -ENOMEM;
    }
    *buf.add(0) = reg as u8;

    i = 0;
    p = buf.add(1);
    while i < len {
        put_unaligned_be32(*values.add(i), p);
        i += 1;
        p = p.add(size_of::<u32>());
    }

    ret = i2c_master_send(client, buf, send_size as c_int);

    kfree(buf as *const c_void);

    if ret == send_size as c_int {
        0
    } else if ret < 0 {
        ret
    } else {
        -EIO
    }
}

/*
 * register read for 8- and 20-byte registers
 */
unsafe fn tas571x_reg_read_multiword(
    client: *mut i2c_client,
    reg: c_uint,
    values: *mut c_long,
    len: size_t,
) -> c_int {
    let mut send_buf: u8;
    let mut p: *mut u8;
    let mut msgs: [i2c_msg; 2] = core::mem::zeroed();
    let recv_size: c_uint = (len * size_of::<u32>()) as c_uint;
    let mut ret: c_int;

    let recv_buf = kzalloc(recv_size as size_t, GFP_KERNEL | GFP_DMA);
    if recv_buf.is_null() {
        return -ENOMEM;
    }

    send_buf = reg as u8;

    msgs[0].addr = (*client).addr;
    msgs[0].len = size_of::<u8>() as u16;
    msgs[0].buf = &mut send_buf;
    msgs[0].flags = 0;

    msgs[1].addr = (*client).addr;
    msgs[1].len = recv_size as u16;
    msgs[1].buf = recv_buf;
    msgs[1].flags = I2C_M_RD;

    ret = i2c_transfer((*client).adapter, msgs.as_mut_ptr(), ARRAY_SIZE(&msgs) as c_int);
    if ret < 0 {
        kfree(recv_buf as *const c_void);
        return ret;
    } else if ret != ARRAY_SIZE(&msgs) as c_int {
        ret = -EIO;
        kfree(recv_buf as *const c_void);
        return ret;
    }

    let mut i: c_uint = 0;
    p = recv_buf;
    while i < len as c_uint {
        *values.add(i as usize) = get_unaligned_be32(p);
        i += 1;
        p = p.add(size_of::<u32>());
    }

    kfree(recv_buf as *const c_void);
    ret
}

/*
 * Integer array controls for setting biquad, mixer, DRC coefficients.
 * According to the datasheet each coefficient is effectively 26bits,
 * i.e. stored as 32bits, where bits [31:26] are ignored.
 * TI's TAS57xx Graphical Development Environment tool however produces
 * coefficients with more than 26 bits. For this reason we allow values
 * in the full 32-bits reange.
 * The coefficients are ordered as given in the TAS571x data sheet:
 * b0, b1, b2, a1, a2
 */
unsafe extern "C" fn tas571x_coefficient_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let numcoef = (*kcontrol).private_value >> 16;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = numcoef as c_uint;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 0xffffffff;
    0
}

unsafe extern "C" fn tas571x_coefficient_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let i2c = to_i2c_client((*component).dev);
    let numcoef = (*kcontrol).private_value >> 16;
    let index = (*kcontrol).private_value & 0xffff;

    tas571x_reg_read_multiword(
        i2c,
        index as c_uint,
        (*ucontrol).value.integer.value.as_mut_ptr(),
        numcoef as size_t,
    )
}

unsafe extern "C" fn tas571x_coefficient_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let i2c = to_i2c_client((*component).dev);
    let numcoef = (*kcontrol).private_value >> 16;
    let index = (*kcontrol).private_value & 0xffff;

    tas571x_reg_write_multiword(
        i2c,
        index as c_uint,
        (*ucontrol).value.integer.value.as_ptr(),
        numcoef as size_t,
    )
}

unsafe extern "C" fn tas571x_set_dai_fmt(
    dai: *mut snd_soc_dai,
    format: c_uint,
) -> c_int {
    let priv_ = snd_soc_component_get_drvdata((*dai).component) as *mut tas571x_private;

    (*priv_).format = format;

    0
}

unsafe extern "C" fn tas571x_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_ = snd_soc_component_get_drvdata((*dai).component) as *mut tas571x_private;
    let mut val: u32;

    match (*priv_).format & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_RIGHT_J => val = 0x00,
        SND_SOC_DAIFMT_I2S => val = 0x03,
        SND_SOC_DAIFMT_LEFT_J => val = 0x06,
        _ => return -EINVAL,
    }

    if params_width(params) >= 24 {
        val += 2;
    } else if params_width(params) >= 20 {
        val += 1;
    }

    regmap_update_bits(
        (*priv_).regmap,
        TAS571X_SDI_REG,
        TAS571X_SDI_FMT_MASK,
        val,
    )
}

unsafe extern "C" fn tas571x_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = (*dai).component;
    let sysctl2: u8;
    let ret: c_int;

    sysctl2 = if mute != 0 {
        TAS571X_SYS_CTRL_2_SDN_MASK as u8
    } else {
        0
    };

    ret = snd_soc_component_update_bits(
        component,
        TAS571X_SYS_CTRL_2_REG,
        TAS571X_SYS_CTRL_2_SDN_MASK,
        sysctl2 as c_uint,
    );
    usleep_range(1000, 2000);

    ret
}

unsafe extern "C" fn tas571x_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut tas571x_private;
    let dapm = snd_soc_component_to_dapm(component);
    let ret: c_int;

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                if !IS_ERR((*priv_).mclk as *const c_void) {
                    ret = clk_prepare_enable((*priv_).mclk);
                    if ret != 0 {
                        dev_err(
                            (*component).dev,
                            b"Failed to enable master clock: %d\n\0".as_ptr()
                                as *const c_char,
                            ret,
                        );
                        return ret;
                    }
                }
            }
        }
        SND_SOC_BIAS_OFF => {
            if !IS_ERR((*priv_).mclk as *const c_void) {
                clk_disable_unprepare((*priv_).mclk);
            }
        }
        _ => {}
    }

    0
}

static tas571x_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(tas571x_set_dai_fmt),
    hw_params: Some(tas571x_hw_params),
    mute_stream: Some(tas571x_mute),
    no_capture_mute: 1,
};

macro_rules! BIQUAD_COEFS {
    ($xname:expr, $reg:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: concat!($xname, "\0").as_ptr() as *const c_char,
            info: Some(tas571x_coefficient_info),
            get: Some(tas571x_coefficient_get),
            put: Some(tas571x_coefficient_put),
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
            private_value: (($reg as c_ulong) | (5 << 16)) as c_long,
            ..snd_kcontrol_new::zero()
        }
    };
}

static tas5711_supply_names: [*const c_char; 6] = [
    b"AVDD\0".as_ptr() as *const c_char,
    b"DVDD\0".as_ptr() as *const c_char,
    b"PVDD_A\0".as_ptr() as *const c_char,
    b"PVDD_B\0".as_ptr() as *const c_char,
    b"PVDD_C\0".as_ptr() as *const c_char,
    b"PVDD_D\0".as_ptr() as *const c_char,
];

// static const DECLARE_TLV_DB_SCALE(tas5711_volume_tlv, -10350, 50, 1);

static tas5711_controls: [snd_kcontrol_new; 3] = [
    SOC_SINGLE_TLV!("Master Volume", TAS571X_MVOL_REG, 0, 0xff, 1, tas5711_volume_tlv),
    SOC_DOUBLE_R_TLV!(
        "Speaker Volume",
        TAS571X_CH1_VOL_REG,
        TAS571X_CH2_VOL_REG,
        0,
        0xff,
        1,
        tas5711_volume_tlv
    ),
    SOC_DOUBLE!(
        "Speaker Switch",
        TAS571X_SOFT_MUTE_REG,
        TAS571X_SOFT_MUTE_CH1_SHIFT,
        TAS571X_SOFT_MUTE_CH2_SHIFT,
        1,
        1
    ),
];

static tas571x_readonly_regs_range: [regmap_range; 1] = [
    regmap_reg_range!(TAS571X_CLK_CTRL_REG, TAS571X_DEV_ID_REG),
];

static tas571x_volatile_regs_range: [regmap_range; 2] = [
    regmap_reg_range!(TAS571X_CLK_CTRL_REG, TAS571X_ERR_STATUS_REG),
    regmap_reg_range!(TAS571X_OSC_TRIM_REG, TAS571X_OSC_TRIM_REG),
];

static tas571x_write_regs: regmap_access_table = regmap_access_table {
    no_ranges: tas571x_readonly_regs_range.as_ptr(),
    n_no_ranges: ARRAY_SIZE(&tas571x_readonly_regs_range) as c_uint,
    ..regmap_access_table::zero()
};

static tas571x_volatile_regs: regmap_access_table = regmap_access_table {
    yes_ranges: tas571x_volatile_regs_range.as_ptr(),
    n_yes_ranges: ARRAY_SIZE(&tas571x_volatile_regs_range) as c_uint,
    ..regmap_access_table::zero()
};

static tas5711_reg_defaults: [reg_default; 7] = [
    reg_default { reg: 0x04, def: 0x05 },
    reg_default { reg: 0x05, def: 0x40 },
    reg_default { reg: 0x06, def: 0x00 },
    reg_default { reg: 0x07, def: 0xff },
    reg_default { reg: 0x08, def: 0x30 },
    reg_default { reg: 0x09, def: 0x30 },
    reg_default { reg: 0x1b, def: 0x82 },
];

static tas5711_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 32,
    max_register: 0xff,
    reg_read: Some(tas571x_reg_read),
    reg_write: Some(tas571x_reg_write),
    reg_defaults: tas5711_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&tas5711_reg_defaults) as c_uint,
    cache_type: REGCACHE_RBTREE,
    wr_table: &tas571x_write_regs,
    volatile_table: &tas571x_volatile_regs,
    ..regmap_config::zero()
};

static tas5711_chip: tas571x_chip = tas571x_chip {
    supply_names: tas5711_supply_names.as_ptr(),
    num_supply_names: ARRAY_SIZE(&tas5711_supply_names) as c_int,
    controls: tas5711_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&tas5711_controls) as c_int,
    regmap_config: &tas5711_regmap_config,
    vol_reg_size: 1,
};

static tas5707_volatile_regs_range: [regmap_range; 3] = [
    regmap_reg_range!(TAS571X_CLK_CTRL_REG, TAS571X_ERR_STATUS_REG),
    regmap_reg_range!(TAS571X_OSC_TRIM_REG, TAS571X_OSC_TRIM_REG),
    regmap_reg_range!(TAS5707_CH1_BQ0_REG, TAS5707_CH2_BQ6_REG),
];

static tas5707_volatile_regs: regmap_access_table = regmap_access_table {
    yes_ranges: tas5707_volatile_regs_range.as_ptr(),
    n_yes_ranges: ARRAY_SIZE(&tas5707_volatile_regs_range) as c_uint,
    ..regmap_access_table::zero()
};

// static const DECLARE_TLV_DB_SCALE(tas5707_volume_tlv, -7900, 50, 1);

static tas5707_volume_slew_step_txt: [*const c_char; 4] = [
    b"256\0".as_ptr() as *const c_char,
    b"512\0".as_ptr() as *const c_char,
    b"1024\0".as_ptr() as *const c_char,
    b"2048\0".as_ptr() as *const c_char,
];

static tas5707_volume_slew_step_values: [c_uint; 4] = [3, 0, 1, 2];

SOC_VALUE_ENUM_SINGLE_DECL!(
    tas5707_volume_slew_step_enum,
    TAS571X_VOL_CFG_REG,
    0,
    0x3,
    tas5707_volume_slew_step_txt,
    tas5707_volume_slew_step_values
);

static tas5707_controls: [snd_kcontrol_new; 18] = [
    SOC_SINGLE_TLV!("Master Volume", TAS571X_MVOL_REG, 0, 0xff, 1, tas5707_volume_tlv),
    SOC_DOUBLE_R_TLV!("Speaker Volume", TAS571X_CH1_VOL_REG, TAS571X_CH2_VOL_REG, 0, 0xff, 1, tas5707_volume_tlv),
    SOC_DOUBLE!("Speaker Switch", TAS571X_SOFT_MUTE_REG, TAS571X_SOFT_MUTE_CH1_SHIFT, TAS571X_SOFT_MUTE_CH2_SHIFT, 1, 1),
    SOC_ENUM!("Slew Rate Steps", tas5707_volume_slew_step_enum),
    BIQUAD_COEFS!("CH1 - Biquad 0", TAS5707_CH1_BQ0_REG),
    BIQUAD_COEFS!("CH1 - Biquad 1", TAS5707_CH1_BQ1_REG),
    BIQUAD_COEFS!("CH1 - Biquad 2", TAS5707_CH1_BQ2_REG),
    BIQUAD_COEFS!("CH1 - Biquad 3", TAS5707_CH1_BQ3_REG),
    BIQUAD_COEFS!("CH1 - Biquad 4", TAS5707_CH1_BQ4_REG),
    BIQUAD_COEFS!("CH1 - Biquad 5", TAS5707_CH1_BQ5_REG),
    BIQUAD_COEFS!("CH1 - Biquad 6", TAS5707_CH1_BQ6_REG),
    BIQUAD_COEFS!("CH2 - Biquad 0", TAS5707_CH2_BQ0_REG),
    BIQUAD_COEFS!("CH2 - Biquad 1", TAS5707_CH2_BQ1_REG),
    BIQUAD_COEFS!("CH2 - Biquad 2", TAS5707_CH2_BQ2_REG),
    BIQUAD_COEFS!("CH2 - Biquad 3", TAS5707_CH2_BQ3_REG),
    BIQUAD_COEFS!("CH2 - Biquad 4", TAS5707_CH2_BQ4_REG),
    BIQUAD_COEFS!("CH2 - Biquad 5", TAS5707_CH2_BQ5_REG),
    BIQUAD_COEFS!("CH2 - Biquad 6", TAS5707_CH2_BQ6_REG),
];

static tas5707_reg_defaults: [reg_default; 21] = [
    reg_default { reg: TAS571X_CLK_CTRL_REG, def: 0x6c },
    reg_default { reg: TAS571X_DEV_ID_REG, def: 0x70 },
    reg_default { reg: TAS571X_ERR_STATUS_REG, def: 0x00 },
    reg_default { reg: TAS571X_SYS_CTRL_1_REG, def: 0xa0 },
    reg_default { reg: TAS571X_SDI_REG, def: 0x05 },
    reg_default { reg: TAS571X_SYS_CTRL_2_REG, def: 0x40 },
    reg_default { reg: TAS571X_SOFT_MUTE_REG, def: 0x00 },
    reg_default { reg: TAS571X_MVOL_REG, def: 0xff },
    reg_default { reg: TAS571X_CH1_VOL_REG, def: 0x30 },
    reg_default { reg: TAS571X_CH2_VOL_REG, def: 0x30 },
    reg_default { reg: TAS571X_VOL_CFG_REG, def: 0x91 },
    reg_default { reg: TAS571X_MODULATION_LIMIT_REG, def: 0x02 },
    reg_default { reg: TAS571X_IC_DELAY_CH1_REG, def: 0xac },
    reg_default { reg: TAS571X_IC_DELAY_CH2_REG, def: 0x54 },
    reg_default { reg: TAS571X_IC_DELAY_CH3_REG, def: 0xac },
    reg_default { reg: TAS571X_IC_DELAY_CH4_REG, def: 0x54 },
    reg_default { reg: TAS571X_START_STOP_PERIOD_REG, def: 0x0f },
    reg_default { reg: TAS571X_OSC_TRIM_REG, def: 0x82 },
    reg_default { reg: TAS571X_BKND_ERR_REG, def: 0x02 },
    reg_default { reg: TAS571X_INPUT_MUX_REG, def: 0x17772 },
    reg_default { reg: TAS571X_PWM_MUX_REG, def: 0x1021345 },
];

static tas5707_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 32,
    max_register: 0xff,
    reg_read: Some(tas571x_reg_read),
    reg_write: Some(tas571x_reg_write),
    reg_defaults: tas5707_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&tas5707_reg_defaults) as c_uint,
    cache_type: REGCACHE_RBTREE,
    wr_table: &tas571x_write_regs,
    volatile_table: &tas5707_volatile_regs,
    ..regmap_config::zero()
};

static tas5707_chip: tas571x_chip = tas571x_chip {
    supply_names: tas5711_supply_names.as_ptr(),
    num_supply_names: ARRAY_SIZE(&tas5711_supply_names) as c_int,
    controls: tas5707_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&tas5707_controls) as c_int,
    regmap_config: &tas5707_regmap_config,
    vol_reg_size: 1,
};

static tas5717_supply_names: [*const c_char; 5] = [
    b"AVDD\0".as_ptr() as *const c_char,
    b"DVDD\0".as_ptr() as *const c_char,
    b"HPVDD\0".as_ptr() as *const c_char,
    b"PVDD_AB\0".as_ptr() as *const c_char,
    b"PVDD_CD\0".as_ptr() as *const c_char,
];

// static const DECLARE_TLV_DB_SCALE(tas5717_volume_tlv, -10375, 25, 0);

static tas5717_controls: [snd_kcontrol_new; 35] = [
    /* MVOL LSB is ignored - see comments in tas571x_i2c_probe() */
    SOC_SINGLE_TLV!("Master Volume", TAS571X_MVOL_REG, 1, 0x1ff, 1, tas5717_volume_tlv),
    SOC_DOUBLE_R_TLV!("Speaker Volume", TAS571X_CH1_VOL_REG, TAS571X_CH2_VOL_REG, 1, 0x1ff, 1, tas5717_volume_tlv),
    SOC_DOUBLE!("Speaker Switch", TAS571X_SOFT_MUTE_REG, TAS571X_SOFT_MUTE_CH1_SHIFT, TAS571X_SOFT_MUTE_CH2_SHIFT, 1, 1),
    SOC_DOUBLE_R_RANGE!("CH1 Mixer Volume", TAS5717_CH1_LEFT_CH_MIX_REG, TAS5717_CH1_RIGHT_CH_MIX_REG, 16, 0, 0x80, 0),
    SOC_DOUBLE_R_RANGE!("CH2 Mixer Volume", TAS5717_CH2_LEFT_CH_MIX_REG, TAS5717_CH2_RIGHT_CH_MIX_REG, 16, 0, 0x80, 0),
    /*
     * The biquads are named according to the register names.
     * Please note that TI's TAS57xx Graphical Development Environment
     * tool names them different.
     */
    BIQUAD_COEFS!("CH1 - Biquad 0", TAS5717_CH1_BQ0_REG),
    BIQUAD_COEFS!("CH1 - Biquad 1", TAS5717_CH1_BQ1_REG),
    BIQUAD_COEFS!("CH1 - Biquad 2", TAS5717_CH1_BQ2_REG),
    BIQUAD_COEFS!("CH1 - Biquad 3", TAS5717_CH1_BQ3_REG),
    BIQUAD_COEFS!("CH1 - Biquad 4", TAS5717_CH1_BQ4_REG),
    BIQUAD_COEFS!("CH1 - Biquad 5", TAS5717_CH1_BQ5_REG),
    BIQUAD_COEFS!("CH1 - Biquad 6", TAS5717_CH1_BQ6_REG),
    BIQUAD_COEFS!("CH1 - Biquad 7", TAS5717_CH1_BQ7_REG),
    BIQUAD_COEFS!("CH1 - Biquad 8", TAS5717_CH1_BQ8_REG),
    BIQUAD_COEFS!("CH1 - Biquad 9", TAS5717_CH1_BQ9_REG),
    BIQUAD_COEFS!("CH1 - Biquad 10", TAS5717_CH1_BQ10_REG),
    BIQUAD_COEFS!("CH1 - Biquad 11", TAS5717_CH1_BQ11_REG),
    BIQUAD_COEFS!("CH2 - Biquad 0", TAS5717_CH2_BQ0_REG),
    BIQUAD_COEFS!("CH2 - Biquad 1", TAS5717_CH2_BQ1_REG),
    BIQUAD_COEFS!("CH2 - Biquad 2", TAS5717_CH2_BQ2_REG),
    BIQUAD_COEFS!("CH2 - Biquad 3", TAS5717_CH2_BQ3_REG),
    BIQUAD_COEFS!("CH2 - Biquad 4", TAS5717_CH2_BQ4_REG),
    BIQUAD_COEFS!("CH2 - Biquad 5", TAS5717_CH2_BQ5_REG),
    BIQUAD_COEFS!("CH2 - Biquad 6", TAS5717_CH2_BQ6_REG),
    BIQUAD_COEFS!("CH2 - Biquad 7", TAS5717_CH2_BQ7_REG),
    BIQUAD_COEFS!("CH2 - Biquad 8", TAS5717_CH2_BQ8_REG),
    BIQUAD_COEFS!("CH2 - Biquad 9", TAS5717_CH2_BQ9_REG),
    BIQUAD_COEFS!("CH2 - Biquad 10", TAS5717_CH2_BQ10_REG),
    BIQUAD_COEFS!("CH2 - Biquad 11", TAS5717_CH2_BQ11_REG),
    BIQUAD_COEFS!("CH3 - Biquad 0", TAS5717_CH3_BQ0_REG),
    BIQUAD_COEFS!("CH3 - Biquad 1", TAS5717_CH3_BQ1_REG),
    BIQUAD_COEFS!("CH4 - Biquad 0", TAS5717_CH4_BQ0_REG),
    BIQUAD_COEFS!("CH4 - Biquad 1", TAS5717_CH4_BQ1_REG),
];

static tas5717_reg_defaults: [reg_default; 11] = [
    reg_default { reg: 0x04, def: 0x05 },
    reg_default { reg: 0x05, def: 0x40 },
    reg_default { reg: 0x06, def: 0x00 },
    reg_default { reg: 0x07, def: 0x03ff },
    reg_default { reg: 0x08, def: 0x00c0 },
    reg_default { reg: 0x09, def: 0x00c0 },
    reg_default { reg: 0x1b, def: 0x82 },
    reg_default { reg: TAS5717_CH1_RIGHT_CH_MIX_REG, def: 0x0 },
    reg_default { reg: TAS5717_CH1_LEFT_CH_MIX_REG, def: 0x800000 },
    reg_default { reg: TAS5717_CH2_LEFT_CH_MIX_REG, def: 0x0 },
    reg_default { reg: TAS5717_CH2_RIGHT_CH_MIX_REG, def: 0x800000 },
];

static tas5717_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 32,
    max_register: 0xff,
    reg_read: Some(tas571x_reg_read),
    reg_write: Some(tas571x_reg_write),
    reg_defaults: tas5717_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&tas5717_reg_defaults) as c_uint,
    cache_type: REGCACHE_RBTREE,
    wr_table: &tas571x_write_regs,
    volatile_table: &tas571x_volatile_regs,
    ..regmap_config::zero()
};

/* This entry is reused for tas5719 as the software interface is identical. */
static tas5717_chip: tas571x_chip = tas571x_chip {
    supply_names: tas5717_supply_names.as_ptr(),
    num_supply_names: ARRAY_SIZE(&tas5717_supply_names) as c_int,
    controls: tas5717_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&tas5717_controls) as c_int,
    regmap_config: &tas5717_regmap_config,
    vol_reg_size: 2,
};

static tas5721_supply_names: [*const c_char; 4] = [
    b"AVDD\0".as_ptr() as *const c_char,
    b"DVDD\0".as_ptr() as *const c_char,
    b"DRVDD\0".as_ptr() as *const c_char,
    b"PVDD\0".as_ptr() as *const c_char,
];

static tas5721_controls: [snd_kcontrol_new; 3] = [
    SOC_SINGLE_TLV!("Master Volume", TAS571X_MVOL_REG, 0, 0xff, 1, tas5711_volume_tlv),
    SOC_DOUBLE_R_TLV!("Speaker Volume", TAS571X_CH1_VOL_REG, TAS571X_CH2_VOL_REG, 0, 0xff, 1, tas5711_volume_tlv),
    SOC_DOUBLE!("Speaker Switch", TAS571X_SOFT_MUTE_REG, TAS571X_SOFT_MUTE_CH1_SHIFT, TAS571X_SOFT_MUTE_CH2_SHIFT, 1, 1),
];

static tas5721_reg_defaults: [reg_default; 24] = [
    reg_default { reg: TAS571X_CLK_CTRL_REG, def: 0x6c },
    reg_default { reg: TAS571X_DEV_ID_REG, def: 0x00 },
    reg_default { reg: TAS571X_ERR_STATUS_REG, def: 0x00 },
    reg_default { reg: TAS571X_SYS_CTRL_1_REG, def: 0xa0 },
    reg_default { reg: TAS571X_SDI_REG, def: 0x05 },
    reg_default { reg: TAS571X_SYS_CTRL_2_REG, def: 0x40 },
    reg_default { reg: TAS571X_SOFT_MUTE_REG, def: 0x00 },
    reg_default { reg: TAS571X_MVOL_REG, def: 0xff },
    reg_default { reg: TAS571X_CH1_VOL_REG, def: 0x30 },
    reg_default { reg: TAS571X_CH2_VOL_REG, def: 0x30 },
    reg_default { reg: TAS571X_CH3_VOL_REG, def: 0x30 },
    reg_default { reg: TAS571X_VOL_CFG_REG, def: 0x91 },
    reg_default { reg: TAS571X_MODULATION_LIMIT_REG, def: 0x02 },
    reg_default { reg: TAS571X_IC_DELAY_CH1_REG, def: 0xac },
    reg_default { reg: TAS571X_IC_DELAY_CH2_REG, def: 0x54 },
    reg_default { reg: TAS571X_IC_DELAY_CH3_REG, def: 0xac },
    reg_default { reg: TAS571X_IC_DELAY_CH4_REG, def: 0x54 },
    reg_default { reg: TAS571X_PWM_CH_SDN_GROUP_REG, def: 0x30 },
    reg_default { reg: TAS571X_START_STOP_PERIOD_REG, def: 0x0f },
    reg_default { reg: TAS571X_OSC_TRIM_REG, def: 0x82 },
    reg_default { reg: TAS571X_BKND_ERR_REG, def: 0x02 },
    reg_default { reg: TAS571X_INPUT_MUX_REG, def: 0x17772 },
    reg_default { reg: TAS571X_CH4_SRC_SELECT_REG, def: 0x4303 },
    reg_default { reg: TAS571X_PWM_MUX_REG, def: 0x1021345 },
];

static tas5721_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 32,
    max_register: 0xff,
    reg_read: Some(tas571x_reg_read),
    reg_write: Some(tas571x_reg_write),
    reg_defaults: tas5721_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&tas5721_reg_defaults) as c_uint,
    cache_type: REGCACHE_RBTREE,
    wr_table: &tas571x_write_regs,
    volatile_table: &tas571x_volatile_regs,
    ..regmap_config::zero()
};

static tas5733_controls: [snd_kcontrol_new; 35] = [
    /* MVOL LSB is ignored - see comments in tas571x_i2c_probe() */
    SOC_SINGLE_TLV!("Master Volume", TAS571X_MVOL_REG, 1, 0x1ff, 1, tas5717_volume_tlv),
    SOC_DOUBLE_R_TLV!("Speaker Volume", TAS571X_CH1_VOL_REG, TAS571X_CH2_VOL_REG, 1, 0x1ff, 1, tas5717_volume_tlv),
    SOC_DOUBLE!("Speaker Switch", TAS571X_SOFT_MUTE_REG, TAS571X_SOFT_MUTE_CH1_SHIFT, TAS571X_SOFT_MUTE_CH2_SHIFT, 1, 1),
    SOC_DOUBLE_R_RANGE!("CH1 Mixer Volume", TAS5717_CH1_LEFT_CH_MIX_REG, TAS5717_CH1_RIGHT_CH_MIX_REG, 16, 0, 0x80, 0),
    SOC_DOUBLE_R_RANGE!("CH2 Mixer Volume", TAS5717_CH2_LEFT_CH_MIX_REG, TAS5717_CH2_RIGHT_CH_MIX_REG, 16, 0, 0x80, 0),
    /*
     * The biquads are named according to the register names.
     * Please note that TI's TAS57xx Graphical Development Environment
     * tool names them different.
     */
    BIQUAD_COEFS!("CH1 - Biquad 0", TAS5733_CH1_BQ0_REG),
    BIQUAD_COEFS!("CH1 - Biquad 1", TAS5733_CH1_BQ1_REG),
    BIQUAD_COEFS!("CH1 - Biquad 2", TAS5733_CH1_BQ2_REG),
    BIQUAD_COEFS!("CH1 - Biquad 3", TAS5733_CH1_BQ3_REG),
    BIQUAD_COEFS!("CH1 - Biquad 4", TAS5733_CH1_BQ4_REG),
    BIQUAD_COEFS!("CH1 - Biquad 5", TAS5733_CH1_BQ5_REG),
    BIQUAD_COEFS!("CH1 - Biquad 6", TAS5733_CH1_BQ6_REG),
    BIQUAD_COEFS!("CH1 - Biquad 7", TAS5733_CH1_BQ7_REG),
    BIQUAD_COEFS!("CH1 - Biquad 8", TAS5733_CH1_BQ8_REG),
    BIQUAD_COEFS!("CH1 - Biquad 9", TAS5733_CH1_BQ9_REG),
    BIQUAD_COEFS!("CH1 - Biquad 10", TAS5733_CH1_BQ10_REG),
    BIQUAD_COEFS!("CH2 - Biquad 0", TAS5733_CH2_BQ0_REG),
    BIQUAD_COEFS!("CH2 - Biquad 1", TAS5733_CH2_BQ1_REG),
    BIQUAD_COEFS!("CH2 - Biquad 2", TAS5733_CH2_BQ2_REG),
    BIQUAD_COEFS!("CH2 - Biquad 3", TAS5733_CH2_BQ3_REG),
    BIQUAD_COEFS!("CH2 - Biquad 4", TAS5733_CH2_BQ4_REG),
    BIQUAD_COEFS!("CH2 - Biquad 5", TAS5733_CH2_BQ5_REG),
    BIQUAD_COEFS!("CH2 - Biquad 6", TAS5733_CH2_BQ6_REG),
    BIQUAD_COEFS!("CH2 - Biquad 7", TAS5733_CH2_BQ7_REG),
    BIQUAD_COEFS!("CH2 - Biquad 8", TAS5733_CH2_BQ8_REG),
    BIQUAD_COEFS!("CH2 - Biquad 9", TAS5733_CH2_BQ9_REG),
    BIQUAD_COEFS!("CH2 - Biquad 10", TAS5733_CH2_BQ10_REG),
    BIQUAD_COEFS!("CH1 - Cross Biquad 0", TAS5733_CH1_CBQ0_REG),
    BIQUAD_COEFS!("CH1 - Cross Biquad 1", TAS5733_CH1_CBQ1_REG),
    BIQUAD_COEFS!("CH1 - Cross Biquad 2", TAS5733_CH1_CBQ2_REG),
    BIQUAD_COEFS!("CH1 - Cross Biquad 3", TAS5733_CH1_CBQ3_REG),
    BIQUAD_COEFS!("CH2 - Cross Biquad 0", TAS5733_CH2_CBQ0_REG),
    BIQUAD_COEFS!("CH2 - Cross Biquad 1", TAS5733_CH2_CBQ1_REG),
    BIQUAD_COEFS!("CH2 - Cross Biquad 2", TAS5733_CH2_CBQ2_REG),
    BIQUAD_COEFS!("CH2 - Cross Biquad 3", TAS5733_CH2_CBQ3_REG),
];

static tas5733_supply_names: [*const c_char; 3] = [
    b"AVDD\0".as_ptr() as *const c_char,
    b"DVDD\0".as_ptr() as *const c_char,
    b"PVDD\0".as_ptr() as *const c_char,
];

static tas5733_reg_defaults: [reg_default; 27] = [
    reg_default { reg: TAS571X_CLK_CTRL_REG, def: 0x6c },
    reg_default { reg: TAS571X_DEV_ID_REG, def: 0x00 },
    reg_default { reg: TAS571X_ERR_STATUS_REG, def: 0x00 },
    reg_default { reg: TAS571X_SYS_CTRL_1_REG, def: 0xa0 },
    reg_default { reg: TAS571X_SDI_REG, def: 0x05 },
    reg_default { reg: TAS571X_SYS_CTRL_2_REG, def: 0x40 },
    reg_default { reg: TAS571X_SOFT_MUTE_REG, def: 0x07 },
    reg_default { reg: TAS571X_MVOL_REG, def: 0x03ff },
    reg_default { reg: TAS571X_CH1_VOL_REG, def: 0x00c0 },
    reg_default { reg: TAS571X_CH2_VOL_REG, def: 0x00c0 },
    reg_default { reg: TAS571X_CH3_VOL_REG, def: 0x00c0 },
    reg_default { reg: TAS571X_VOL_CFG_REG, def: 0xf0 },
    reg_default { reg: TAS571X_MODULATION_LIMIT_REG, def: 0x07 },
    reg_default { reg: TAS571X_IC_DELAY_CH1_REG, def: 0xb8 },
    reg_default { reg: TAS571X_IC_DELAY_CH2_REG, def: 0x60 },
    reg_default { reg: TAS571X_IC_DELAY_CH3_REG, def: 0xa0 },
    reg_default { reg: TAS571X_IC_DELAY_CH4_REG, def: 0x48 },
    reg_default { reg: TAS571X_PWM_CH_SDN_GROUP_REG, def: 0x30 },
    reg_default { reg: TAS571X_START_STOP_PERIOD_REG, def: 0x68 },
    reg_default { reg: TAS571X_OSC_TRIM_REG, def: 0x82 },
    reg_default { reg: TAS571X_BKND_ERR_REG, def: 0x02 },
    reg_default { reg: TAS571X_INPUT_MUX_REG, def: 0x00897772 },
    reg_default { reg: TAS571X_PWM_MUX_REG, def: 0x01021345 },
    reg_default { reg: TAS5717_CH1_RIGHT_CH_MIX_REG, def: 0x00 },
    reg_default { reg: TAS5717_CH1_LEFT_CH_MIX_REG, def: 0x800000 },
    reg_default { reg: TAS5717_CH2_LEFT_CH_MIX_REG, def: 0x00 },
    reg_default { reg: TAS5717_CH2_RIGHT_CH_MIX_REG, def: 0x800000 },
];

static tas5733_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 32,
    max_register: 0xff,
    reg_read: Some(tas571x_reg_read),
    reg_write: Some(tas571x_reg_write),
    reg_defaults: tas5733_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&tas5733_reg_defaults) as c_uint,
    cache_type: REGCACHE_RBTREE,
    wr_table: &tas571x_write_regs,
    volatile_table: &tas571x_volatile_regs,
    ..regmap_config::zero()
};

static tas5733_chip: tas571x_chip = tas571x_chip {
    supply_names: tas5733_supply_names.as_ptr(),
    num_supply_names: ARRAY_SIZE(&tas5733_supply_names) as c_int,
    controls: tas5733_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&tas5733_controls) as c_int,
    regmap_config: &tas5733_regmap_config,
    vol_reg_size: 2,
};

static tas5753_reg_defaults: [reg_default; 25] = [
    reg_default { reg: TAS571X_CLK_CTRL_REG, def: 0x6c },
    reg_default { reg: TAS571X_DEV_ID_REG, def: 0x41 },
    reg_default { reg: TAS571X_ERR_STATUS_REG, def: 0x00 },
    reg_default { reg: TAS571X_SYS_CTRL_1_REG, def: 0xa0 },
    reg_default { reg: TAS571X_SDI_REG, def: 0x05 },
    reg_default { reg: TAS571X_SYS_CTRL_2_REG, def: 0x40 },
    reg_default { reg: TAS571X_SOFT_MUTE_REG, def: 0x00 },
    reg_default { reg: TAS571X_MVOL_REG, def: 0x03ff },
    reg_default { reg: TAS571X_CH1_VOL_REG, def: 0x00c0 },
    reg_default { reg: TAS571X_CH2_VOL_REG, def: 0x00c0 },
    reg_default { reg: TAS571X_CH3_VOL_REG, def: 0x00c0 },
    reg_default { reg: TAS571X_VOL_CFG_REG, def: 0xf0 },
    reg_default { reg: TAS571X_MODULATION_LIMIT_REG, def: 0x01 },
    reg_default { reg: TAS571X_IC_DELAY_CH1_REG, def: 0xac },
    reg_default { reg: TAS571X_IC_DELAY_CH2_REG, def: 0x54 },
    reg_default { reg: TAS571X_IC_DELAY_CH3_REG, def: 0xac },
    reg_default { reg: TAS571X_IC_DELAY_CH4_REG, def: 0x54 },
    reg_default { reg: TAS571X_OSC_TRIM_REG, def: 0x82 },
    reg_default { reg: TAS571X_BKND_ERR_REG, def: 0x57 },
    reg_default { reg: TAS571X_INPUT_MUX_REG, def: 0x00017772 },
    reg_default { reg: TAS571X_PWM_MUX_REG, def: 0x01021345 },
    reg_default { reg: TAS5717_CH1_RIGHT_CH_MIX_REG, def: 0x00 },
    reg_default { reg: TAS5717_CH1_LEFT_CH_MIX_REG, def: 0x800000 },
    reg_default { reg: TAS5717_CH2_LEFT_CH_MIX_REG, def: 0x00 },
    reg_default { reg: TAS5717_CH2_RIGHT_CH_MIX_REG, def: 0x800000 },
];

static tas5753_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 32,
    max_register: 0xff,
    reg_read: Some(tas571x_reg_read),
    reg_write: Some(tas571x_reg_write),
    reg_defaults: tas5753_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&tas5753_reg_defaults) as c_uint,
    cache_type: REGCACHE_RBTREE,
    wr_table: &tas571x_write_regs,
    volatile_table: &tas571x_volatile_regs,
    ..regmap_config::zero()
};

static tas5753_chip: tas571x_chip = tas571x_chip {
    supply_names: tas5721_supply_names.as_ptr(),
    num_supply_names: ARRAY_SIZE(&tas5721_supply_names) as c_int,
    controls: tas5733_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&tas5733_controls) as c_int,
    regmap_config: &tas5753_regmap_config,
    vol_reg_size: 2,
};

static tas5721_chip: tas571x_chip = tas571x_chip {
    supply_names: tas5721_supply_names.as_ptr(),
    num_supply_names: ARRAY_SIZE(&tas5721_supply_names) as c_int,
    controls: tas5721_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&tas5721_controls) as c_int,
    regmap_config: &tas5721_regmap_config,
    vol_reg_size: 1,
};

static tas571x_dapm_widgets: [snd_soc_dapm_widget; 6] = [
    SND_SOC_DAPM_DAC!("DACL", ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC!("DACR", ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_OUTPUT!("OUT_A"),
    SND_SOC_DAPM_OUTPUT!("OUT_B"),
    SND_SOC_DAPM_OUTPUT!("OUT_C"),
    SND_SOC_DAPM_OUTPUT!("OUT_D"),
];

static tas571x_dapm_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: b"DACL\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DACR\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"OUT_A\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DACL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"OUT_B\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DACL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"OUT_C\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DACR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"OUT_D\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DACR\0".as_ptr() as *const c_char },
];

static tas571x_component: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(tas571x_set_bias_level),
    dapm_widgets: tas571x_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&tas571x_dapm_widgets) as c_uint,
    dapm_routes: tas571x_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&tas571x_dapm_routes) as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
    ..snd_soc_component_driver::zero()
};

static mut tas571x_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"tas571x-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE,
    },
    ops: &tas571x_dai_ops,
    ..snd_soc_dai_driver::zero()
};

unsafe extern "C" fn tas571x_i2c_probe(client: *mut i2c_client) -> c_int {
    let mut priv_: *mut tas571x_private;
    let dev = &mut (*client).dev as *mut device;
    let mut i: c_int;
    let mut ret: c_int;

    priv_ = devm_kzalloc(dev, size_of::<tas571x_private>(), GFP_KERNEL) as *mut tas571x_private;
    if priv_.is_null() {
        return -ENOMEM;
    }
    i2c_set_clientdata(client, priv_ as *mut c_void);

    (*priv_).chip = i2c_get_match_data(client) as *const tas571x_chip;

    (*priv_).mclk = devm_clk_get(dev, b"mclk\0".as_ptr() as *const c_char);
    if IS_ERR((*priv_).mclk as *const c_void) && PTR_ERR((*priv_).mclk as *const c_void) != -ENOENT as c_long {
        dev_err(dev, b"Failed to request mclk: %ld\n\0".as_ptr() as *const c_char, PTR_ERR((*priv_).mclk as *const c_void));
        return PTR_ERR((*priv_).mclk as *const c_void) as c_int;
    }

    if WARN_ON((*(*priv_).chip).num_supply_names > TAS571X_MAX_SUPPLIES as c_int) {
        return -EINVAL;
    }
    i = 0;
    while i < (*(*priv_).chip).num_supply_names {
        (*priv_).supplies[i as usize].supply = *(*(*priv_).chip).supply_names.add(i as usize);
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, (*(*priv_).chip).num_supply_names, (*priv_).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"Failed to get supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = regulator_bulk_enable((*(*priv_).chip).num_supply_names, (*priv_).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    (*priv_).regmap = devm_regmap_init(dev, ptr::null(), client as *mut c_void, (*(*priv_).chip).regmap_config);
    if IS_ERR((*priv_).regmap as *const c_void) {
        ret = PTR_ERR((*priv_).regmap as *const c_void) as c_int;
        regulator_bulk_disable((*(*priv_).chip).num_supply_names, (*priv_).supplies.as_mut_ptr());
        return ret;
    }

    (*priv_).pdn_gpio = devm_gpiod_get_optional(dev, b"pdn\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*priv_).pdn_gpio as *const c_void) {
        dev_err(dev, b"error requesting pdn_gpio: %ld\n\0".as_ptr() as *const c_char, PTR_ERR((*priv_).pdn_gpio as *const c_void));
        ret = PTR_ERR((*priv_).pdn_gpio as *const c_void) as c_int;
        regulator_bulk_disable((*(*priv_).chip).num_supply_names, (*priv_).supplies.as_mut_ptr());
        return ret;
    }

    (*priv_).reset_gpio = devm_gpiod_get_optional(dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_HIGH);
    if IS_ERR((*priv_).reset_gpio as *const c_void) {
        dev_err(dev, b"error requesting reset_gpio: %ld\n\0".as_ptr() as *const c_char, PTR_ERR((*priv_).reset_gpio as *const c_void));
        ret = PTR_ERR((*priv_).reset_gpio as *const c_void) as c_int;
        regulator_bulk_disable((*(*priv_).chip).num_supply_names, (*priv_).supplies.as_mut_ptr());
        return ret;
    } else if !(*priv_).reset_gpio.is_null() {
        /* pulse the active low reset line for ~100us */
        usleep_range(100, 200);
        gpiod_set_value((*priv_).reset_gpio, 0);
        usleep_range(13500, 20000);
    }

    ret = regmap_write((*priv_).regmap, TAS571X_OSC_TRIM_REG, 0);
    if ret != 0 {
        regulator_bulk_disable((*(*priv_).chip).num_supply_names, (*priv_).supplies.as_mut_ptr());
        return ret;
    }

    usleep_range(50000, 60000);

    memcpy(
        &mut (*priv_).component_driver as *mut _ as *mut c_void,
        &tas571x_component as *const _ as *const c_void,
        size_of::<snd_soc_component_driver>(),
    );
    (*priv_).component_driver.controls = (*(*priv_).chip).controls;
    (*priv_).component_driver.num_controls = (*(*priv_).chip).num_controls;

    if (*(*priv_).chip).vol_reg_size == 2 {
        /*
         * The master volume defaults to 0x3ff (mute), but we ignore
         * (zero) the LSB because the hardware step size is 0.125 dB
         * and TLV_DB_SCALE_ITEM has a resolution of 0.01 dB.
         */
        ret = regmap_update_bits((*priv_).regmap, TAS571X_MVOL_REG, 1, 0);
        if ret != 0 {
            regulator_bulk_disable((*(*priv_).chip).num_supply_names, (*priv_).supplies.as_mut_ptr());
            return ret;
        }
    }

    ret = devm_snd_soc_register_component(
        &mut (*client).dev,
        &(*priv_).component_driver,
        &mut tas571x_dai,
        1,
    );
    if ret != 0 {
        regulator_bulk_disable((*(*priv_).chip).num_supply_names, (*priv_).supplies.as_mut_ptr());
        return ret;
    }

    ret
}

unsafe extern "C" fn tas571x_i2c_remove(client: *mut i2c_client) {
    let priv_ = i2c_get_clientdata(client) as *mut tas571x_private;

    regulator_bulk_disable((*(*priv_).chip).num_supply_names, (*priv_).supplies.as_mut_ptr());
}

static tas571x_of_match: [of_device_id; 8] = [
    of_device_id { compatible: b"ti,tas5707\0".as_ptr() as *const c_char, data: &tas5707_chip as *const _ as *const c_void, ..of_device_id::zero() },
    of_device_id { compatible: b"ti,tas5711\0".as_ptr() as *const c_char, data: &tas5711_chip as *const _ as *const c_void, ..of_device_id::zero() },
    of_device_id { compatible: b"ti,tas5717\0".as_ptr() as *const c_char, data: &tas5717_chip as *const _ as *const c_void, ..of_device_id::zero() },
    of_device_id { compatible: b"ti,tas5719\0".as_ptr() as *const c_char, data: &tas5717_chip as *const _ as *const c_void, ..of_device_id::zero() },
    of_device_id { compatible: b"ti,tas5721\0".as_ptr() as *const c_char, data: &tas5721_chip as *const _ as *const c_void, ..of_device_id::zero() },
    of_device_id { compatible: b"ti,tas5733\0".as_ptr() as *const c_char, data: &tas5733_chip as *const _ as *const c_void, ..of_device_id::zero() },
    of_device_id { compatible: b"ti,tas5753\0".as_ptr() as *const c_char, data: &tas5753_chip as *const _ as *const c_void, ..of_device_id::zero() },
    of_device_id::zero(),
];
// MODULE_DEVICE_TABLE(of, tas571x_of_match);

static tas571x_i2c_id: [i2c_device_id; 8] = [
    i2c_device_id { name: *b"tas5707\0", driver_data: &tas5707_chip as *const _ as kernel_ulong_t },
    i2c_device_id { name: *b"tas5711\0", driver_data: &tas5711_chip as *const _ as kernel_ulong_t },
    i2c_device_id { name: *b"tas5717\0", driver_data: &tas5717_chip as *const _ as kernel_ulong_t },
    i2c_device_id { name: *b"tas5719\0", driver_data: &tas5717_chip as *const _ as kernel_ulong_t },
    i2c_device_id { name: *b"tas5721\0", driver_data: &tas5721_chip as *const _ as kernel_ulong_t },
    i2c_device_id { name: *b"tas5733\0", driver_data: &tas5733_chip as *const _ as kernel_ulong_t },
    i2c_device_id { name: *b"tas5753\0", driver_data: &tas5753_chip as *const _ as kernel_ulong_t },
    i2c_device_id::zero(),
];
// MODULE_DEVICE_TABLE(i2c, tas571x_i2c_id);

static mut tas571x_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"tas571x\0".as_ptr() as *const c_char,
        of_match_table: of_match_ptr!(tas571x_of_match),
        ..device_driver::zero()
    },
    probe: Some(tas571x_i2c_probe),
    remove: Some(tas571x_i2c_remove),
    id_table: tas571x_i2c_id.as_ptr(),
};
module_i2c_driver!(tas571x_i2c_driver);

// MODULE_DESCRIPTION("ASoC TAS571x driver");
// MODULE_AUTHOR("Kevin Cernekee <cernekee@chromium.org>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
