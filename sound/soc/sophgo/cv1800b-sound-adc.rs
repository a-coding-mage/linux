// SPDX-License-Identifier: GPL-2.0
/*
 * Internal adc codec for cv1800b compatible SoC
 *
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type c_int = core::ffi::c_int;
type c_uint = core::ffi::c_uint;
type c_ulong = core::ffi::c_ulong;
type c_char = core::ffi::c_char;
type c_void = core::ffi::c_void;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 10;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 2;

const fn genmask(h: u32, l: u32) -> u32 {
    let high = if h == 31 { u32::MAX } else { (1u32 << (h + 1)) - 1 };
    let low = if l == 0 { 0 } else { (1u32 << l) - 1 };
    high & !low
}

const fn mask_shift(mask: u32) -> u32 {
    mask.trailing_zeros()
}

fn field_get(mask: u32, reg: u32) -> u32 {
    (reg & mask) >> mask_shift(mask)
}

fn u32_replace_bits(old: u32, val: u32, mask: u32) -> u32 {
    (old & !mask) | ((val << mask_shift(mask)) & mask)
}

fn div_u64(dividend: u64, divisor: u64) -> u64 {
    dividend / divisor
}

fn __ffs(word: u32) -> u32 {
    word.trailing_zeros()
}

fn min_u32(a: u32, b: u32) -> u32 {
    if a < b { a } else { b }
}

fn clamp_u32(val: c_long, min: u32, max: u32) -> u32 {
    let mut v = val as u32;
    if v < min {
        v = min;
    }
    if v > max {
        v = max;
    }
    v
}

type c_long = core::ffi::c_long;

const CV1800B_RXADC_WORD_LEN: u32 = 16;
const CV1800B_RXADC_CHANNELS: u32 = 2;

const CV1800B_RXADC_CTRL0: usize = 0x00;
const CV1800B_RXADCC_CTRL1: usize = 0x04;
const CV1800B_RXADC_STATUS: usize = 0x08;
const CV1800B_RXADC_CLK: usize = 0x0c;
const CV1800B_RXADC_ANA0: usize = 0x10;
const CV1800B_RXADC_ANA1: usize = 0x14;
const CV1800B_RXADC_ANA2: usize = 0x18;
const CV1800B_RXADC_ANA3: usize = 0x1c;
const CV1800B_RXADC_ANA4: usize = 0x20;

/* CV1800B_RXADC_CTRL0 */
const REG_RXADC_EN: u32 = genmask(0, 0);
const REG_I2S_TX_EN: u32 = genmask(1, 1);

/* CV1800B_RXADCC_CTRL1 */
const REG_RXADC_CIC_OPT: u32 = genmask(1, 0);
const REG_RXADC_IGR_INIT: u32 = genmask(8, 8);

/* CV1800B_RXADC_ANA0 */
const REG_GSTEPL_RXPGA: u32 = genmask(12, 0);
const REG_G6DBL_RXPGA: u32 = genmask(13, 13);
const REG_GAINL_RXADC: u32 = genmask(15, 14);
const REG_GSTEPR_RXPGA: u32 = genmask(28, 16);
const REG_G6DBR_RXPGA: u32 = genmask(29, 29);
const REG_GAINR_RXADC: u32 = genmask(31, 30);
const REG_COMB_LEFT_VOLUME: u32 = genmask(15, 0);
const REG_COMB_RIGHT_VOLUME: u32 = genmask(31, 16);

/* CV1800B_RXADC_ANA2 */
const REG_MUTEL_RXPGA: u32 = genmask(0, 0);
const REG_MUTER_RXPGA: u32 = genmask(1, 1);

/* CV1800B_RXADC_CLK */
const REG_RXADC_CLK_INV: u32 = genmask(0, 0);
const REG_RXADC_SCK_DIV: u32 = genmask(15, 8);
const REG_RXADC_DLYEN: u32 = genmask(23, 16);

#[repr(C)]
enum decimation_values {
    DECIMATION_64 = 0,
    DECIMATION_128,
    DECIMATION_256,
    DECIMATION_512,
}

static cv1800b_gains: [u32; 25] = [
    0x0001, /* 0dB */
    0x0002, /* 2dB */
    0x0004, /* 4dB */
    0x0008, /* 6dB */
    0x0010, /* 8dB */
    0x0020, /* 10dB */
    0x0040, /* 12dB */
    0x0080, /* 14dB */
    0x0100, /* 16dB */
    0x0200, /* 18dB */
    0x0400, /* 20dB */
    0x0800, /* 22dB */
    0x1000, /* 24dB */
    0x2400, /* 26dB */
    0x2800, /* 28dB */
    0x3000, /* 30dB */
    0x6400, /* 32dB */
    0x6800, /* 34dB */
    0x7000, /* 36dB */
    0xA400, /* 38dB */
    0xA800, /* 40dB */
    0xB000, /* 42dB */
    0xE400, /* 44dB */
    0xE800, /* 46dB */
    0xF000, /* 48dB */
];

#[repr(C)]
struct cv1800b_priv {
    regs: *mut c_void,
    dev: *mut device,
    mclk_rate: c_uint,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

#[repr(C)]
struct snd_soc_dai_ops {
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_kcontrol_new {
    iface: c_uint,
    name: *const c_char,
    info: Option<unsafe extern "C" fn()>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    private_value: c_ulong,
    tlv: *const c_uint,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct driver_private {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: driver_private,
}

unsafe extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn module_platform_driver(driver: *mut platform_driver);
}

unsafe extern "C" fn cv1800b_adc_setbclk_div(
    priv_: *mut cv1800b_priv,
    rate: c_uint,
) -> c_int {
    let mut val: u32;
    let bclk_div: u32;
    let tmp: u64;

    if (*priv_).mclk_rate == 0 || rate == 0 {
        return -EINVAL;
    }

    tmp = div_u64(
        (*priv_).mclk_rate as u64,
        (CV1800B_RXADC_WORD_LEN * CV1800B_RXADC_CHANNELS * rate * 2) as u64,
    );

    if tmp == 0 {
        dev_err(
            (*priv_).dev,
            c"computed BCLK divider is zero\n".as_ptr(),
        );
        return -EINVAL;
    }

    if tmp > 256 {
        dev_err(
            (*priv_).dev,
            c"BCLK divider %llu out of range\n".as_ptr(),
            tmp,
        );
        return -EINVAL;
    }

    bclk_div = (tmp - 1) as u32;
    val = readl((*priv_).regs.add(CV1800B_RXADC_CLK));
    val = u32_replace_bits(val, bclk_div, REG_RXADC_SCK_DIV);
    /* Vendor value for 48kHz, tested on SG2000/SG2002 */
    val = u32_replace_bits(val, 0x19, REG_RXADC_DLYEN);
    writel(val, (*priv_).regs.add(CV1800B_RXADC_CLK));

    0
}

unsafe extern "C" fn cv1800b_adc_enable(priv_: *mut cv1800b_priv, enable: bool) {
    let mut val: u32;

    val = readl((*priv_).regs.add(CV1800B_RXADC_CTRL0));
    val = u32_replace_bits(val, enable as u32, REG_RXADC_EN);
    val = u32_replace_bits(val, enable as u32, REG_I2S_TX_EN);
    writel(val, (*priv_).regs.add(CV1800B_RXADC_CTRL0));
}

fn cv1800b_adc_calc_db(ana0: u32, right: bool) -> c_uint {
    let step_mask: u32 = if right {
        field_get(REG_GSTEPR_RXPGA, ana0)
    } else {
        field_get(REG_GSTEPL_RXPGA, ana0)
    };
    let mut coarse: u32 = if right {
        field_get(REG_GAINR_RXADC, ana0)
    } else {
        field_get(REG_GAINL_RXADC, ana0)
    };
    let g6db: bool = if right {
        field_get(REG_G6DBR_RXPGA, ana0) != 0
    } else {
        field_get(REG_G6DBL_RXPGA, ana0) != 0
    };

    let mut step: u32 = if step_mask != 0 { __ffs(step_mask) } else { 0 };

    step = min_u32(step, 12);
    coarse = min_u32(coarse, 3);

    2 * step + 6 * coarse + if g6db { 6 } else { 0 }
}

unsafe extern "C" fn cv1800b_adc_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_: *mut cv1800b_priv = snd_soc_dai_get_drvdata(dai) as *mut cv1800b_priv;
    let rate: c_uint = params_rate(params);
    let mut val: u32;
    let ret: c_int;

    ret = cv1800b_adc_setbclk_div(priv_, rate);
    if ret != 0 {
        dev_err(
            (*priv_).dev,
            c"could not set rate, check DT node for fixed clock\n".as_ptr(),
        );
        return ret;
    }

    /* init adc */
    val = readl((*priv_).regs.add(CV1800B_RXADCC_CTRL1));
    val = u32_replace_bits(val, 1, REG_RXADC_IGR_INIT);
    val = u32_replace_bits(val, decimation_values::DECIMATION_64 as u32, REG_RXADC_CIC_OPT);
    writel(val, (*priv_).regs.add(CV1800B_RXADCC_CTRL1));
    0
}

unsafe extern "C" fn cv1800b_adc_dai_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_: *mut cv1800b_priv = snd_soc_dai_get_drvdata(dai) as *mut cv1800b_priv;

    match cmd {
        SNDRV_PCM_TRIGGER_START
        | SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            cv1800b_adc_enable(priv_, true);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            cv1800b_adc_enable(priv_, false);
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn cv1800b_adc_dai_set_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let priv_: *mut cv1800b_priv = snd_soc_dai_get_drvdata(dai) as *mut cv1800b_priv;

    (*priv_).mclk_rate = freq;
    dev_dbg((*priv_).dev, c"mclk is set to %u\n".as_ptr(), freq);
    0
}

static cv1800b_adc_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(cv1800b_adc_hw_params),
    set_sysclk: Some(cv1800b_adc_dai_set_sysclk),
    trigger: Some(cv1800b_adc_dai_trigger),
};

static mut cv1800b_adc_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"adc-hifi".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"ADC Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    ops: &cv1800b_adc_dai_ops,
};

unsafe extern "C" fn cv1800b_adc_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let priv_: *mut cv1800b_priv = snd_soc_component_get_drvdata(component) as *mut cv1800b_priv;
    let ana0: u32 = readl((*priv_).regs.add(CV1800B_RXADC_ANA0));

    let left: c_uint = cv1800b_adc_calc_db(ana0, false);
    let right: c_uint = cv1800b_adc_calc_db(ana0, true);

    (*ucontrol).value.integer.value[0] = min_u32(left / 2, 24) as c_long;
    (*ucontrol).value.integer.value[1] = min_u32(right / 2, 24) as c_long;
    0
}

unsafe extern "C" fn cv1800b_adc_volume_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let priv_: *mut cv1800b_priv = snd_soc_component_get_drvdata(component) as *mut cv1800b_priv;

    let v_left: u32 = clamp_u32((*ucontrol).value.integer.value[0], 0, 24);
    let v_right: u32 = clamp_u32((*ucontrol).value.integer.value[1], 0, 24);
    let mut val: u32;
    let old_val: u32;

    val = readl((*priv_).regs.add(CV1800B_RXADC_ANA0));
    old_val = val;

    val = u32_replace_bits(
        val,
        cv1800b_gains[v_left as usize],
        REG_COMB_LEFT_VOLUME,
    );
    val = u32_replace_bits(
        val,
        cv1800b_gains[v_right as usize],
        REG_COMB_RIGHT_VOLUME,
    );

    if val == old_val {
        return 0;
    }

    writel(val, (*priv_).regs.add(CV1800B_RXADC_ANA0));

    1
}

/* DECLARE_TLV_DB_SCALE(cv1800b_volume_tlv, 0, 200, 0) */
static cv1800b_volume_tlv: [u32; 4] = [0, 0, 200, 0];

/* SOC_DOUBLE_EXT_TLV("Internal I2S Capture Volume", SND_SOC_NOPM, 0, 16, 24, false,
 *                    cv1800b_adc_volume_get, cv1800b_adc_volume_set,
 *                    cv1800b_volume_tlv)
 */
static cv1800b_adc_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    iface: 0,
    name: c"Internal I2S Capture Volume".as_ptr(),
    info: None,
    get: Some(cv1800b_adc_volume_get),
    put: Some(cv1800b_adc_volume_set),
    private_value: SND_SOC_NOPM as c_ulong
        | ((0 as c_ulong) << 8)
        | ((16 as c_ulong) << 16)
        | ((24 as c_ulong) << 24),
    tlv: cv1800b_volume_tlv.as_ptr(),
}];

static cv1800b_adc_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"cv1800b-adc-codec".as_ptr(),
    controls: cv1800b_adc_controls.as_ptr(),
    num_controls: cv1800b_adc_controls.len() as c_uint,
};

unsafe extern "C" fn cv1800b_adc_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let priv_: *mut cv1800b_priv;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<cv1800b_priv>(), GFP_KERNEL) as *mut cv1800b_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).dev = dev;
    (*priv_).regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*priv_).regs) {
        return PTR_ERR((*priv_).regs);
    }

    platform_set_drvdata(pdev, priv_ as *mut c_void);
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &cv1800b_adc_component,
        &raw mut cv1800b_adc_dai,
        1,
    )
}

static cv1800b_adc_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"sophgo,cv1800b-sound-adc".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, cv1800b_adc_of_match) */

static mut cv1800b_adc_driver: platform_driver = platform_driver {
    probe: Some(cv1800b_adc_probe),
    driver: driver_private {
        name: c"cv1800b-sound-adc".as_ptr(),
        of_match_table: cv1800b_adc_of_match.as_ptr(),
    },
};

unsafe fn cv1800b_adc_driver_init() {
    module_platform_driver(&raw mut cv1800b_adc_driver);
}

/* module_platform_driver(cv1800b_adc_driver); */

/* MODULE_DESCRIPTION("ADC codec for CV1800B"); */
/* MODULE_AUTHOR("Anton D. Stavinskii <stavinsky@gmail.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
