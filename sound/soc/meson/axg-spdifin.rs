// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    let high = if h == 31 { u32::MAX } else { (1u32 << (h + 1)) - 1 };
    let low = if l == 0 { 0 } else { (1u32 << l) - 1 };
    high & !low
}

const fn FIELD_PREP(mask: c_uint, val: c_uint) -> c_uint {
    (val << mask.trailing_zeros()) & mask
}

fn FIELD_GET(mask: c_uint, reg: c_uint) -> c_uint {
    (reg & mask) >> mask.trailing_zeros()
}

fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

const SPDIFIN_CTRL0: c_uint = 0x00;
const SPDIFIN_CTRL0_EN: c_uint = BIT(31);
const SPDIFIN_CTRL0_RST_OUT: c_uint = BIT(29);
const SPDIFIN_CTRL0_RST_IN: c_uint = BIT(28);
const SPDIFIN_CTRL0_WIDTH_SEL: c_uint = BIT(24);
const SPDIFIN_CTRL0_STATUS_CH_SHIFT: c_uint = 11;
const SPDIFIN_CTRL0_STATUS_SEL: c_uint = GENMASK(10, 8);
const SPDIFIN_CTRL0_SRC_SEL: c_uint = GENMASK(5, 4);
const SPDIFIN_CTRL0_CHK_VALID: c_uint = BIT(3);
const SPDIFIN_CTRL1: c_uint = 0x04;
const SPDIFIN_CTRL1_BASE_TIMER: c_uint = GENMASK(19, 0);
const SPDIFIN_CTRL1_IRQ_MASK: c_uint = GENMASK(27, 20);
const SPDIFIN_CTRL2: c_uint = 0x08;
const SPDIFIN_THRES_PER_REG: c_uint = 3;
const SPDIFIN_THRES_WIDTH: c_uint = 10;
const SPDIFIN_CTRL3: c_uint = 0x0c;
const SPDIFIN_CTRL4: c_uint = 0x10;
const SPDIFIN_TIMER_PER_REG: c_uint = 4;
const SPDIFIN_TIMER_WIDTH: c_uint = 8;
const SPDIFIN_CTRL5: c_uint = 0x14;
const SPDIFIN_CTRL6: c_uint = 0x18;
const SPDIFIN_STAT0: c_uint = 0x1c;
const SPDIFIN_STAT0_MODE: c_uint = GENMASK(30, 28);
const SPDIFIN_STAT0_MAXW: c_uint = GENMASK(17, 8);
const SPDIFIN_STAT0_IRQ: c_uint = GENMASK(7, 0);
const SPDIFIN_IRQ_MODE_CHANGED: c_uint = BIT(2);
const SPDIFIN_STAT1: c_uint = 0x20;
const SPDIFIN_STAT2: c_uint = 0x24;
const SPDIFIN_MUTE_VAL: c_uint = 0x28;

const SPDIFIN_MODE_NUM: usize = 7;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 1;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1 << 0;
const SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint = 1 << 2;
const SNDRV_CTL_ELEM_IFACE_PCM: c_uint = 2;
const SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE: u64 = 1 << 18;
const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 31;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    name: *const c_char,
    ops: *const snd_soc_dai_ops,
    capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    min: c_long,
    max: c_long,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    type_: c_uint,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_iec958 {
    status: [u8; 24],
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    iec958: snd_ctl_elem_value_iec958,
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct soc_enum {
    reg: c_uint,
    shift_l: c_uint,
    items: c_uint,
    texts: *const *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    iface: c_uint,
    access: c_uint,
    name: *const c_char,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    private_value: c_ulong,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    reg_stride: c_uint,
    max_register: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: device_driver,
}

#[repr(C)]
struct axg_spdifin_cfg {
    mode_rates: *const c_uint,
    ref_rate: c_uint,
}

#[repr(C)]
struct axg_spdifin {
    conf: *const axg_spdifin_cfg,
    map: *mut regmap,
    refclk: *mut clk,
    pclk: *mut clk,
}

unsafe extern "C" {
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_get_reg_stride(map: *mut regmap) -> c_uint;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_uint;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_pcm_rate_to_rate_bit(rate: c_uint) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
}

fn IS_ERR<T>(ptr: *const T) -> bool {
    (ptr as isize) >= -4095isize
}

fn PTR_ERR<T>(ptr: *const T) -> c_long {
    ptr as c_long
}

fn ERR_PTR<T>(err: c_long) -> *mut T {
    err as isize as *mut T
}

/*
 * TODO:
 * It would have been nice to check the actual rate against the sample rate
 * requested in hw_params(). Unfortunately, I was not able to make the mode
 * detection and IRQ work reliably:
 *
 * 1. IRQs are generated on mode change only, so there is no notification
 *    on transition between no signal and mode 0 (32kHz).
 * 2. Mode detection very often has glitches, and may detects the
 *    lowest or the highest mode before zeroing in on the actual mode.
 *
 * This makes calling snd_pcm_stop() difficult to get right. Even notifying
 * the kcontrol would be very unreliable at this point.
 * Let's keep things simple until the magic spell that makes this work is
 * found.
 */

unsafe extern "C" fn axg_spdifin_get_rate(priv_: *mut axg_spdifin) -> c_uint {
    let mut stat: c_uint = 0;
    let mode: c_uint;
    let mut rate: c_uint = 0;

    unsafe {
        regmap_read((*priv_).map, SPDIFIN_STAT0, &mut stat);
    }
    mode = FIELD_GET(SPDIFIN_STAT0_MODE, stat);

    /*
     * If max width is zero, we are not capturing anything.
     * Also Sometimes, when the capture is on but there is no data,
     * mode is SPDIFIN_MODE_NUM, but not always ...
     */
    unsafe {
        if FIELD_GET(SPDIFIN_STAT0_MAXW, stat) != 0 && mode < SPDIFIN_MODE_NUM as c_uint {
            rate = *(*(*priv_).conf).mode_rates.add(mode as usize);
        }
    }

    rate
}

unsafe extern "C" fn axg_spdifin_prepare(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_ = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_spdifin };

    /* Apply both reset */
    unsafe {
        regmap_update_bits(
            (*priv_).map,
            SPDIFIN_CTRL0,
            SPDIFIN_CTRL0_RST_OUT | SPDIFIN_CTRL0_RST_IN,
            0,
        );

        /* Clear out reset before in reset */
        regmap_update_bits(
            (*priv_).map,
            SPDIFIN_CTRL0,
            SPDIFIN_CTRL0_RST_OUT,
            SPDIFIN_CTRL0_RST_OUT,
        );
        regmap_update_bits(
            (*priv_).map,
            SPDIFIN_CTRL0,
            SPDIFIN_CTRL0_RST_IN,
            SPDIFIN_CTRL0_RST_IN,
        );
    }

    0
}

unsafe extern "C" fn axg_spdifin_write_mode_param(
    map: *mut regmap,
    mode: c_int,
    val: c_uint,
    num_per_reg: c_uint,
    base_reg: c_uint,
    width: c_uint,
) {
    let mut offset = mode as u64;
    let reg: c_uint;
    let shift: c_uint;
    let rem: c_uint;

    rem = (offset % num_per_reg as u64) as c_uint;
    offset /= num_per_reg as u64;

    unsafe {
        reg = (offset as c_uint)
            .wrapping_mul(regmap_get_reg_stride(map))
            .wrapping_add(base_reg);
        shift = width.wrapping_mul(num_per_reg.wrapping_sub(1).wrapping_sub(rem));

        regmap_update_bits(
            map,
            reg,
            GENMASK(width - 1, 0) << shift,
            val << shift,
        );
    }
}

unsafe extern "C" fn axg_spdifin_write_timer(map: *mut regmap, mode: c_int, val: c_uint) {
    unsafe {
        axg_spdifin_write_mode_param(
            map,
            mode,
            val,
            SPDIFIN_TIMER_PER_REG,
            SPDIFIN_CTRL4,
            SPDIFIN_TIMER_WIDTH,
        );
    }
}

unsafe extern "C" fn axg_spdifin_write_threshold(map: *mut regmap, mode: c_int, val: c_uint) {
    unsafe {
        axg_spdifin_write_mode_param(
            map,
            mode,
            val,
            SPDIFIN_THRES_PER_REG,
            SPDIFIN_CTRL2,
            SPDIFIN_THRES_WIDTH,
        );
    }
}

unsafe extern "C" fn axg_spdifin_mode_timer(
    priv_: *mut axg_spdifin,
    mode: c_int,
    rate: c_uint,
) -> c_uint {
    /*
     * Number of period of the reference clock during a period of the
     * input signal reference clock
     */
    unsafe { rate / (128 * *(*(*priv_).conf).mode_rates.add(mode as usize)) }
}

unsafe extern "C" fn axg_spdifin_sample_mode_config(
    dai: *mut snd_soc_dai,
    priv_: *mut axg_spdifin,
) -> c_int {
    let rate: c_uint;
    let mut t_next: c_uint;
    let mut ret: c_int;
    let mut i: c_int = SPDIFIN_MODE_NUM as c_int - 1;

    /* Set spdif input reference clock */
    unsafe {
        ret = clk_set_rate((*priv_).refclk, (*(*priv_).conf).ref_rate);
        if ret != 0 {
            dev_err((*dai).dev, c"reference clock rate set failed\n".as_ptr());
            return ret;
        }

        /*
         * The rate actually set might be slightly different, get
         * the actual rate for the following mode calculation
         */
        rate = clk_get_rate((*priv_).refclk);

        /* HW will update mode every 1ms */
        regmap_update_bits(
            (*priv_).map,
            SPDIFIN_CTRL1,
            SPDIFIN_CTRL1_BASE_TIMER,
            FIELD_PREP(SPDIFIN_CTRL1_BASE_TIMER, rate / 1000),
        );

        /* Threshold based on the maximum width between two edges */
        regmap_update_bits((*priv_).map, SPDIFIN_CTRL0, SPDIFIN_CTRL0_WIDTH_SEL, 0);

        /* Calculate the last timer which has no threshold */
        t_next = axg_spdifin_mode_timer(priv_, i, rate);
        axg_spdifin_write_timer((*priv_).map, i, t_next);

        loop {
            let t: c_uint;

            i -= 1;

            /* Calculate the timer */
            t = axg_spdifin_mode_timer(priv_, i, rate);

            /* Set the timer value */
            axg_spdifin_write_timer((*priv_).map, i, t);

            /* Set the threshold value */
            axg_spdifin_write_threshold((*priv_).map, i, 3 * (t + t_next));

            /* Save the current timer for the next threshold calculation */
            t_next = t;

            if !(i > 0) {
                break;
            }
        }
    }

    0
}

unsafe extern "C" fn axg_spdifin_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let priv_ = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_spdifin };
    let mut ret: c_int;

    unsafe {
        ret = clk_prepare_enable((*priv_).pclk);
        if ret != 0 {
            dev_err((*dai).dev, c"failed to enable pclk\n".as_ptr());
            return ret;
        }

        ret = axg_spdifin_sample_mode_config(dai, priv_);
        if ret != 0 {
            dev_err((*dai).dev, c"mode configuration failed\n".as_ptr());
            clk_disable_unprepare((*priv_).pclk);
            return ret;
        }

        ret = clk_prepare_enable((*priv_).refclk);
        if ret != 0 {
            dev_err(
                (*dai).dev,
                c"failed to enable spdifin reference clock\n".as_ptr(),
            );
            clk_disable_unprepare((*priv_).pclk);
            return ret;
        }

        regmap_update_bits(
            (*priv_).map,
            SPDIFIN_CTRL0,
            SPDIFIN_CTRL0_EN,
            SPDIFIN_CTRL0_EN,
        );
    }

    0
}

unsafe extern "C" fn axg_spdifin_dai_remove(dai: *mut snd_soc_dai) -> c_int {
    let priv_ = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_spdifin };

    unsafe {
        regmap_update_bits((*priv_).map, SPDIFIN_CTRL0, SPDIFIN_CTRL0_EN, 0);
        clk_disable_unprepare((*priv_).refclk);
        clk_disable_unprepare((*priv_).pclk);
    }
    0
}

static axg_spdifin_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(axg_spdifin_dai_probe),
    remove: Some(axg_spdifin_dai_remove),
    prepare: Some(axg_spdifin_prepare),
};

unsafe extern "C" fn axg_spdifin_iec958_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
        (*uinfo).count = 1;
    }

    0
}

unsafe extern "C" fn axg_spdifin_get_status_mask(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mut i: c_int;

    i = 0;
    while i < 24 {
        unsafe {
            (*ucontrol).value.iec958.status[i as usize] = 0xff;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn axg_spdifin_get_status(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let c = unsafe { snd_kcontrol_chip(kcontrol) };
    let priv_ = unsafe { snd_soc_component_get_drvdata(c) as *mut axg_spdifin };
    let mut i: c_int;
    let mut j: c_int;

    i = 0;
    while i < 6 {
        let mut val: c_uint = 0;

        unsafe {
            regmap_update_bits(
                (*priv_).map,
                SPDIFIN_CTRL0,
                SPDIFIN_CTRL0_STATUS_SEL,
                FIELD_PREP(SPDIFIN_CTRL0_STATUS_SEL, i as c_uint),
            );

            regmap_read((*priv_).map, SPDIFIN_STAT1, &mut val);
        }

        j = 0;
        while j < 4 {
            let offset: c_uint = (i * 4 + j) as c_uint;

            unsafe {
                (*ucontrol).value.iec958.status[offset as usize] =
                    ((val >> (j * 8)) & 0xff) as u8;
            }
            j += 1;
        }
        i += 1;
    }

    0
}

const fn AXG_SPDIFIN_IEC958_MASK() -> snd_kcontrol_new {
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Capture Mask".as_ptr(),
        info: Some(axg_spdifin_iec958_info),
        get: Some(axg_spdifin_get_status_mask),
        private_value: 0,
    }
}

const fn AXG_SPDIFIN_IEC958_STATUS() -> snd_kcontrol_new {
    snd_kcontrol_new {
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Capture Default".as_ptr(),
        info: Some(axg_spdifin_iec958_info),
        get: Some(axg_spdifin_get_status),
        private_value: 0,
    }
}

static spdifin_chsts_src_texts: [*const c_char; 2] = [c"A".as_ptr(), c"B".as_ptr()];

static axg_spdifin_chsts_src_enum: soc_enum = soc_enum {
    reg: SPDIFIN_CTRL0,
    shift_l: SPDIFIN_CTRL0_STATUS_CH_SHIFT,
    items: 2,
    texts: spdifin_chsts_src_texts.as_ptr(),
};

unsafe extern "C" fn axg_spdifin_rate_lock_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 192000;
    }

    0
}

unsafe extern "C" fn axg_spdifin_rate_lock_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let c = unsafe { snd_kcontrol_chip(kcontrol) };
    let priv_ = unsafe { snd_soc_component_get_drvdata(c) as *mut axg_spdifin };

    unsafe {
        (*ucontrol).value.integer.value[0] = axg_spdifin_get_rate(priv_) as c_long;
    }

    0
}

const fn AXG_SPDIFIN_LOCK_RATE(xname: *const c_char) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        get: Some(axg_spdifin_rate_lock_get),
        info: Some(axg_spdifin_rate_lock_info),
        name: xname,
        private_value: 0,
    }
}

const fn SOC_DOUBLE(
    name: *const c_char,
    reg: c_uint,
    shift_left: c_uint,
    shift_right: c_uint,
    max: c_uint,
    invert: c_uint,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        name,
        info: None,
        get: None,
        private_value: reg as c_ulong
            | ((shift_left as c_ulong) << 8)
            | ((shift_right as c_ulong) << 12)
            | ((max as c_ulong) << 16)
            | ((invert as c_ulong) << 24),
    }
}

const fn SOC_ENUM(name: *const c_char, xenum: *const soc_enum) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        name,
        info: None,
        get: None,
        private_value: xenum as c_ulong,
    }
}

static axg_spdifin_controls: [snd_kcontrol_new; 5] = [
    AXG_SPDIFIN_LOCK_RATE(c"Capture Rate Lock".as_ptr()),
    SOC_DOUBLE(c"Capture Switch".as_ptr(), SPDIFIN_CTRL0, 7, 6, 1, 1),
    SOC_ENUM(
        c"IEC958 Capture DefaultSrc".as_ptr(),
        &axg_spdifin_chsts_src_enum,
    ),
    AXG_SPDIFIN_IEC958_MASK(),
    AXG_SPDIFIN_IEC958_STATUS(),
];

static axg_spdifin_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    controls: axg_spdifin_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&axg_spdifin_controls) as c_uint,
    legacy_dai_naming: 1,
};

static axg_spdifin_regmap_cfg: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    max_register: SPDIFIN_MUTE_VAL,
};

static axg_spdifin_mode_rates: [c_uint; SPDIFIN_MODE_NUM] =
    [32000, 44100, 48000, 88200, 96000, 176400, 192000];

static axg_cfg: axg_spdifin_cfg = axg_spdifin_cfg {
    mode_rates: axg_spdifin_mode_rates.as_ptr(),
    ref_rate: 333333333,
};

static axg_spdifin_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"amlogic,axg-spdifin".as_ptr(),
        data: &axg_cfg as *const axg_spdifin_cfg as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, axg_spdifin_of_match); */

unsafe extern "C" fn axg_spdifin_get_dai_drv(
    dev: *mut device,
    priv_: *mut axg_spdifin,
) -> *mut snd_soc_dai_driver {
    let drv: *mut snd_soc_dai_driver;
    let mut i: c_int;

    unsafe {
        drv = devm_kzalloc(dev, size_of::<snd_soc_dai_driver>(), GFP_KERNEL)
            as *mut snd_soc_dai_driver;
        if drv.is_null() {
            return ERR_PTR(-(ENOMEM as c_long));
        }

        (*drv).name = c"SPDIF Input".as_ptr();
        (*drv).ops = &axg_spdifin_ops;
        (*drv).capture.stream_name = c"Capture".as_ptr();
        (*drv).capture.channels_min = 1;
        (*drv).capture.channels_max = 2;
        (*drv).capture.formats = SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE;

        i = 0;
        while i < SPDIFIN_MODE_NUM as c_int {
            let rb: c_uint =
                snd_pcm_rate_to_rate_bit(*(*(*priv_).conf).mode_rates.add(i as usize));

            if rb == SNDRV_PCM_RATE_KNOT {
                return ERR_PTR(-(EINVAL as c_long));
            }

            (*drv).capture.rates |= rb;
            i += 1;
        }
    }

    drv
}

unsafe extern "C" fn axg_spdifin_probe(pdev: *mut platform_device) -> c_int {
    let dev = unsafe { &mut (*pdev).dev as *mut device };
    let priv_: *mut axg_spdifin;
    let dai_drv: *mut snd_soc_dai_driver;
    let regs: *mut c_void;

    unsafe {
        priv_ = devm_kzalloc(dev, size_of::<axg_spdifin>(), GFP_KERNEL) as *mut axg_spdifin;
        if priv_.is_null() {
            return -ENOMEM;
        }
        platform_set_drvdata(pdev, priv_ as *mut c_void);

        (*priv_).conf = of_device_get_match_data(dev) as *const axg_spdifin_cfg;
        if (*priv_).conf.is_null() {
            dev_err(dev, c"failed to match device\n".as_ptr());
            return -ENODEV;
        }

        regs = devm_platform_ioremap_resource(pdev, 0);
        if IS_ERR(regs) {
            return PTR_ERR(regs) as c_int;
        }

        (*priv_).map = devm_regmap_init_mmio(dev, regs, &axg_spdifin_regmap_cfg);
        if IS_ERR((*priv_).map) {
            dev_err(
                dev,
                c"failed to init regmap: %ld\n".as_ptr(),
                PTR_ERR((*priv_).map),
            );
            return PTR_ERR((*priv_).map) as c_int;
        }

        (*priv_).pclk = devm_clk_get(dev, c"pclk".as_ptr());
        if IS_ERR((*priv_).pclk) {
            return dev_err_probe(dev, PTR_ERR((*priv_).pclk), c"failed to get pclk\n".as_ptr());
        }

        (*priv_).refclk = devm_clk_get(dev, c"refclk".as_ptr());
        if IS_ERR((*priv_).refclk) {
            return dev_err_probe(dev, PTR_ERR((*priv_).refclk), c"failed to get mclk\n".as_ptr());
        }

        dai_drv = axg_spdifin_get_dai_drv(dev, priv_);
        if IS_ERR(dai_drv) {
            dev_err(
                dev,
                c"failed to get dai driver: %ld\n".as_ptr(),
                PTR_ERR(dai_drv),
            );
            return PTR_ERR(dai_drv) as c_int;
        }

        devm_snd_soc_register_component(dev, &axg_spdifin_component_drv, dai_drv, 1)
    }
}

static mut axg_spdifin_pdrv: platform_driver = platform_driver {
    probe: Some(axg_spdifin_probe),
    driver: device_driver {
        name: c"axg-spdifin".as_ptr(),
        of_match_table: axg_spdifin_of_match.as_ptr(),
    },
};
/* module_platform_driver(axg_spdifin_pdrv); */

/* MODULE_DESCRIPTION("Amlogic AXG SPDIF Input driver"); */
/* MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
