// SPDX-License-Identifier: GPL-2.0-only
//
// rt9123.c -- RT9123 (SW I2C Mode) ALSA SoC Codec driver
//
// Author: ChiYuan Huang <cy_huang@richtek.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;

const RT9123_REG_AMPCTRL: c_uint = 0x01;
const RT9123_REG_I2SOPT: c_uint = 0x02;
const RT9123_REG_TDMRX: c_uint = 0x03;
const RT9123_REG_SILVOLEN: c_uint = 0x04;
const RT9123_REG_VOLGAIN: c_uint = 0x12;
const RT9123_REG_ANAFLAG: c_uint = 0x36;
const RT9123_REG_COMBOID: c_uint = 0xF7;

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const RT9123_MASK_SWRST: c_uint = BIT(15);
const RT9123_MASK_SWMUTE: c_uint = BIT(14);
const RT9123_MASK_AMPON: c_uint = BIT(12);
const RT9123_MASK_AUDBIT: c_uint = GENMASK(14, 12);
const RT9123_MASK_AUDFMT: c_uint = GENMASK(11, 8);
const RT9123_MASK_TDMRXLOC: c_uint = GENMASK(4, 0);
const RT9123_MASK_VENID: c_uint = GENMASK(15, 4);

const RT9123_FIXED_VENID: u16 = 0x340;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_HIGH: c_uint = 0;
const REGMAP_ENDIAN_BIG: c_uint = 1;
const REGCACHE_MAPLE: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;
const SND_SOC_DAPM_POST_PMU: c_int = 1 << 0;
const SND_SOC_DAPM_POST_PMD: c_int = 1 << 1;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 2;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 3;
const SND_SOC_DAIFMT_DSP_A: c_uint = 4;
const SND_SOC_DAIFMT_DSP_B: c_uint = 5;
const SNDRV_PCM_FMTBIT_S16: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S24: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S32: u64 = 1 << 2;
const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_22050: c_uint = 1 << 2;
const SNDRV_PCM_RATE_24000: c_uint = 1 << 3;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 5;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 6;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 7;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 8;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_ctl_elem_id,
}
#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: *const c_char,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub name_prefix: *const c_char,
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
struct rt9123_priv {
    enable: *mut gpio_desc,
    dai_fmt: c_uint,
    tdm_slots: c_int,
    tdm_slot_width: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_def {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}
#[repr(C)]
pub struct soc_enum {
    reg: c_uint,
    shift_l: c_uint,
    shift_r: c_uint,
    items: c_uint,
    texts: *const *const c_char,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component_driver {
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget_def,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    formats: u64,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct regmap_config {
    name: *const c_char,
    reg_bits: c_uint,
    val_bits: c_uint,
    val_format_endian: c_uint,
    readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    cache_type: c_uint,
    num_reg_defaults_raw: c_uint,
}
#[repr(C)]
pub struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}
#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
}
#[repr(C)]
pub struct acpi_device_id {
    id: *const c_char,
    driver_data: c_ulong,
}
#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    acpi_match_table: *const acpi_device_id,
    pm: *const dev_pm_ops,
}
#[repr(C)]
pub struct i2c_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
}

unsafe extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn snd_soc_component_write_field(comp: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn hweight_long(w: c_ulong) -> c_uint;
    fn ffs(x: c_int) -> c_int;
    fn params_width(param: *mut snd_pcm_hw_params) -> c_uint;
    fn params_physical_width(param: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn i2c_smbus_read_i2c_block_data(client: *mut i2c_client, command: u8, length: u8, values: *mut u8) -> c_int;
    fn i2c_smbus_write_i2c_block_data(client: *mut i2c_client, command: u8, length: u8, values: *const u8) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn regcache_sync(map: *mut regmap) -> c_int;
}

const fn FIELD_GET(mask: c_uint, reg: c_uint) -> c_uint {
    (reg & mask) >> mask.trailing_zeros()
}

unsafe extern "C" fn rt9123_enable_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let comp = unsafe { snd_soc_dapm_to_component((*w).dapm) };
    let dev = unsafe { (*comp).dev };
    let enable: c_uint;
    let ret: c_int;

    match event {
        SND_SOC_DAPM_POST_PMU => enable = 1,
        SND_SOC_DAPM_POST_PMD => enable = 0,
        _ => return -EINVAL,
    }

    ret = unsafe { pm_runtime_resume_and_get(dev) };
    if ret != 0 {
        return ret;
    }

    /* AMPON bit is located in volatile RG, use pm_runtime to guarantee the RG access */
    unsafe {
        snd_soc_component_write_field(comp, RT9123_REG_AMPCTRL, RT9123_MASK_AMPON, enable);
        pm_runtime_put_autosuspend(dev);
    }

    0
}

/* SND_SOC_DAPM_OUTPUT/SND_SOC_DAPM_OUT_DRV_E initializers are supplied by the ASoC framework. */
static rt9123_dapm_widgets: [snd_soc_dapm_widget_def; 2] = [
    snd_soc_dapm_widget_def { _private: [] },
    snd_soc_dapm_widget_def { _private: [] },
];

static rt9123_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"Amp Drv\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"HiFi Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"SPK\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"Amp Drv\0".as_ptr() as *const c_char,
    },
];

/* DECLARE_TLV_DB_SCALE/DECLARE_TLV_DB_RANGE data from the C source. */
static dig_tlv: [c_uint; 4] = [0, 0, (-10375i32) as c_uint, 25];
static ana_tlv: [c_uint; 12] = [
    0, 0, (-1200i32) as c_uint, 0, 0,
    1, 9, 0, 150, 0,
    10, 10,
];
static pwmfreq_text: [*const c_char; 4] = [
    b"300KHz\0".as_ptr() as *const c_char,
    b"325KHz\0".as_ptr() as *const c_char,
    b"350KHz\0".as_ptr() as *const c_char,
    b"375KHz\0".as_ptr() as *const c_char,
];
static rt9123_pwm_freq_enum: soc_enum = soc_enum {
    reg: RT9123_REG_AMPCTRL,
    shift_l: 4,
    shift_r: 4,
    items: pwmfreq_text.len() as c_uint,
    texts: pwmfreq_text.as_ptr(),
};
static i2sch_text: [*const c_char; 4] = [
    b"(L+R)/2\0".as_ptr() as *const c_char,
    b"LCH\0".as_ptr() as *const c_char,
    b"RCH\0".as_ptr() as *const c_char,
    b"(L+R)/2\0".as_ptr() as *const c_char,
];
static rt9123_i2sch_select_enum: soc_enum = soc_enum {
    reg: RT9123_REG_I2SOPT,
    shift_l: 4,
    shift_r: 4,
    items: i2sch_text.len() as c_uint,
    texts: i2sch_text.as_ptr(),
};

unsafe extern "C" fn rt9123_kcontrol_name_comp(
    kcontrol: *mut snd_kcontrol,
    s: *const c_char,
) -> c_int {
    let comp = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut kctlname = unsafe { (*kcontrol).id.name };

    if !comp.is_null() && unsafe { !(*comp).name_prefix.is_null() } {
        kctlname = unsafe { kctlname.add(strlen((*comp).name_prefix) + 1) };
    }

    unsafe { strcmp(kctlname, s) }
}

unsafe extern "C" fn rt9123_xhandler_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp = unsafe { snd_kcontrol_chip(kcontrol) };
    let dev = unsafe { (*comp).dev };
    let mut ret: c_int;

    ret = unsafe { pm_runtime_resume_and_get(dev) };
    if ret != 0 {
        return ret;
    }

    /*
     * Since the RG bitfield for 'Speaker Volume' and 'PWM Frequency Select' are located in
     * volatile RG address, special handling here with pm runtime API to guarantee RG read
     * operation.
     */
    if unsafe { rt9123_kcontrol_name_comp(kcontrol, b"Speaker Volume\0".as_ptr() as *const c_char) } == 0 {
        ret = unsafe { snd_soc_get_volsw(kcontrol, ucontrol) };
    } else {
        ret = unsafe { snd_soc_get_enum_double(kcontrol, ucontrol) };
    }

    if ret < 0 {
        unsafe { dev_err(dev, b"Failed to get control (%d)\n\0".as_ptr() as *const c_char, ret) };
    }

    unsafe { pm_runtime_put_autosuspend(dev) };
    ret
}

unsafe extern "C" fn rt9123_xhandler_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let comp = unsafe { snd_kcontrol_chip(kcontrol) };
    let dev = unsafe { (*comp).dev };
    let mut ret: c_int;

    ret = unsafe { pm_runtime_resume_and_get(dev) };
    if ret != 0 {
        return ret;
    }

    /*
     * Since the RG bitfield for 'Speaker Volume' and 'PWM Frequency Select' are located in
     * volatile RG address, special handling here with pm runtime API to guarantee RG write
     * operation.
     */
    if unsafe { rt9123_kcontrol_name_comp(kcontrol, b"Speaker Volume\0".as_ptr() as *const c_char) } == 0 {
        ret = unsafe { snd_soc_put_volsw(kcontrol, ucontrol) };
    } else {
        ret = unsafe { snd_soc_put_enum_double(kcontrol, ucontrol) };
    }

    if ret < 0 {
        unsafe { dev_err(dev, b"Failed to put control (%d)\n\0".as_ptr() as *const c_char, ret) };
    }

    unsafe { pm_runtime_put_autosuspend(dev) };
    ret
}

/* SOC_* control macro initializers are supplied by the ASoC framework. */
static rt9123_controls: [snd_kcontrol_new; 5] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

static rt9123_comp_driver: snd_soc_component_driver = snd_soc_component_driver {
    controls: rt9123_controls.as_ptr(),
    num_controls: rt9123_controls.len() as c_uint,
    dapm_widgets: rt9123_dapm_widgets.as_ptr(),
    num_dapm_widgets: rt9123_dapm_widgets.len() as c_uint,
    dapm_routes: rt9123_dapm_routes.as_ptr(),
    num_dapm_routes: rt9123_dapm_routes.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn rt9123_dai_set_format(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let rt9123 = unsafe { snd_soc_dai_get_drvdata(dai) as *mut rt9123_priv };

    unsafe { (*rt9123).dai_fmt = fmt };
    0
}

unsafe extern "C" fn rt9123_dai_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let rt9123 = unsafe { snd_soc_dai_get_drvdata(dai) as *mut rt9123_priv };
    let comp = unsafe { (*dai).component };
    let dev = unsafe { (*dai).dev };
    let rx_loc: c_uint;

    unsafe {
        dev_dbg(
            dev,
            b"(slots, slot_width) = (%d, %d), (txmask, rxmask) = 0x%x, 0x%x\n\0".as_ptr() as *const c_char,
            slots,
            slot_width,
            tx_mask,
            rx_mask,
        );
    }

    if slots <= 0 || slot_width <= 0 || slots % 2 != 0 || slot_width % 8 != 0 || slots * slot_width > 256 {
        unsafe {
            dev_err(
                dev,
                b"Invalid slot parameter (%d, %d)\n\0".as_ptr() as *const c_char,
                slots,
                slot_width,
            );
        }
        return -EINVAL;
    }

    if rx_mask == 0 || unsafe { hweight_long(rx_mask as c_ulong) } > 1 || unsafe { ffs(rx_mask as c_int) } > slots {
        unsafe {
            dev_err(
                dev,
                b"Invalid rx_mask 0x%08x, slots = %d\n\0".as_ptr() as *const c_char,
                rx_mask,
                slots,
            );
        }
        return -EINVAL;
    }

    /* Configure rx channel data location */
    rx_loc = ((unsafe { ffs(rx_mask as c_int) } - 1) * slot_width / 8) as c_uint;
    unsafe {
        snd_soc_component_write_field(comp, RT9123_REG_TDMRX, RT9123_MASK_TDMRXLOC, rx_loc);

        (*rt9123).tdm_slots = slots;
        (*rt9123).tdm_slot_width = slot_width;
    }

    0
}

unsafe extern "C" fn rt9123_dai_hw_params(
    _substream: *mut snd_pcm_substream,
    param: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rt9123 = unsafe { snd_soc_dai_get_drvdata(dai) as *mut rt9123_priv };
    let comp = unsafe { (*dai).component };
    let fmtval: c_uint;
    let width: c_uint;
    let slot_width: c_uint;
    let dev = unsafe { (*dai).dev };
    let audfmt: c_uint;
    let audbit: c_uint;

    fmtval = unsafe { FIELD_GET(SND_SOC_DAIFMT_FORMAT_MASK, (*rt9123).dai_fmt) };
    if unsafe { (*rt9123).tdm_slots } != 0 && fmtval != SND_SOC_DAIFMT_DSP_A && fmtval != SND_SOC_DAIFMT_DSP_B {
        unsafe { dev_err(dev, b"TDM only can support DSP_A or DSP_B format\n\0".as_ptr() as *const c_char) };
        return -EINVAL;
    }

    match fmtval {
        SND_SOC_DAIFMT_I2S => audfmt = 0,
        SND_SOC_DAIFMT_LEFT_J => audfmt = 1,
        SND_SOC_DAIFMT_RIGHT_J => audfmt = 2,
        SND_SOC_DAIFMT_DSP_B => audfmt = if unsafe { (*rt9123).tdm_slots } != 0 { 4 } else { 3 },
        SND_SOC_DAIFMT_DSP_A => audfmt = if unsafe { (*rt9123).tdm_slots } != 0 { 12 } else { 11 },
        _ => {
            unsafe { dev_err(dev, b"Unsupported format %d\n\0".as_ptr() as *const c_char, fmtval) };
            return -EINVAL;
        }
    }

    width = unsafe { params_width(param) };
    match width {
        16 => audbit = 0,
        20 => audbit = 1,
        24 => audbit = 2,
        32 => audbit = 3,
        8 => audbit = 4,
        _ => {
            unsafe { dev_err(dev, b"Unsupported width %d\n\0".as_ptr() as *const c_char, width) };
            return -EINVAL;
        }
    }

    slot_width = unsafe { params_physical_width(param) };
    if unsafe { (*rt9123).tdm_slots } != 0 && slot_width > unsafe { (*rt9123).tdm_slot_width as c_uint } {
        unsafe { dev_err(dev, b"Slot width is larger than TDM slot width\n\0".as_ptr() as *const c_char) };
        return -EINVAL;
    }

    unsafe {
        snd_soc_component_write_field(comp, RT9123_REG_I2SOPT, RT9123_MASK_AUDFMT, audfmt);
        snd_soc_component_write_field(comp, RT9123_REG_I2SOPT, RT9123_MASK_AUDBIT, audbit);
    }

    0
}

static rt9123_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(rt9123_dai_set_format),
    set_tdm_slot: Some(rt9123_dai_set_tdm_slot),
    hw_params: Some(rt9123_dai_hw_params),
};

static mut rt9123_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"HiFi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"HiFi Playback\0".as_ptr() as *const c_char,
        formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
        rates: SNDRV_PCM_RATE_8000
            | SNDRV_PCM_RATE_16000
            | SNDRV_PCM_RATE_22050
            | SNDRV_PCM_RATE_24000
            | SNDRV_PCM_RATE_32000
            | SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_88200
            | SNDRV_PCM_RATE_96000,
        rate_min: 8000,
        rate_max: 96000,
        channels_min: 1,
        channels_max: 2,
    },
    ops: &rt9123_dai_ops,
};

unsafe extern "C" fn rt9123_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x00..=0x05 | 0x12..=0x13 | 0x20..=0x21 | 0x36 => true,
        _ => false,
    }
}

unsafe extern "C" fn rt9123_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x01..=0x05 | 0x12..=0x13 | 0x20..=0x21 => true,
        _ => false,
    }
}

unsafe extern "C" fn rt9123_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        0x01 | 0x20 | 0x36 => true,
        _ => false,
    }
}

static rt9123_regmap_config: regmap_config = regmap_config {
    name: b"rt9123\0".as_ptr() as *const c_char,
    reg_bits: 8,
    val_bits: 16,
    val_format_endian: REGMAP_ENDIAN_BIG,
    readable_reg: Some(rt9123_readable_reg),
    writeable_reg: Some(rt9123_writeable_reg),
    volatile_reg: Some(rt9123_volatile_reg),
    cache_type: REGCACHE_MAPLE,
    num_reg_defaults_raw: RT9123_REG_ANAFLAG + 1,
};

unsafe extern "C" fn rt9123_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = unsafe { &mut (*i2c).dev as *mut device };
    let rt9123: *mut rt9123_priv;
    let regmap: *mut regmap;
    let mut value: u16 = 0;
    let venid: u16;
    let mut ret: c_int;

    rt9123 = unsafe { devm_kzalloc(dev, size_of::<rt9123_priv>(), GFP_KERNEL) as *mut rt9123_priv };
    if rt9123.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*rt9123).enable = devm_gpiod_get_optional(dev, b"enable\0".as_ptr() as *const c_char, GPIOD_OUT_HIGH);
    }
    if unsafe { IS_ERR((*rt9123).enable as *const c_void) } {
        return unsafe { PTR_ERR((*rt9123).enable as *const c_void) };
    } else if unsafe { !(*rt9123).enable.is_null() } {
        unsafe { usleep_range(250, 350) };
    } else {
        unsafe { dev_dbg(dev, b"No 'enable' GPIO specified, treat it as default on\n\0".as_ptr() as *const c_char) };
    }

    /* Check vendor id information */
    ret = unsafe {
        i2c_smbus_read_i2c_block_data(
            i2c,
            RT9123_REG_COMBOID as u8,
            size_of::<u16>() as u8,
            &mut value as *mut u16 as *mut u8,
        )
    };
    if ret < 0 {
        return unsafe { dev_err_probe(dev, ret, b"Failed to read vendor-id\n\0".as_ptr() as *const c_char) };
    }

    venid = u16::from_be(value);
    if (venid & RT9123_MASK_VENID as u16) != RT9123_FIXED_VENID {
        return unsafe {
            dev_err_probe(
                dev,
                -ENODEV,
                b"Incorrect vendor-id 0x%04x\n\0".as_ptr() as *const c_char,
                venid as c_uint,
            )
        };
    }

    /* Trigger RG reset before regmap init cache */
    value = (RT9123_MASK_SWRST as u16).to_be();
    ret = unsafe {
        i2c_smbus_write_i2c_block_data(
            i2c,
            RT9123_REG_AMPCTRL as u8,
            size_of::<u16>() as u8,
            &value as *const u16 as *const u8,
        )
    };
    if ret != 0 {
        return unsafe { dev_err_probe(dev, ret, b"Failed to trigger RG reset\n\0".as_ptr() as *const c_char) };
    }

    /* Need to wait 10ms for the reset to complete */
    unsafe { usleep_range(10000, 11000) };

    regmap = unsafe { devm_regmap_init_i2c(i2c, &rt9123_regmap_config) };
    if unsafe { IS_ERR(regmap as *const c_void) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR(regmap as *const c_void),
                b"Failed to init regmap\n\0".as_ptr() as *const c_char,
            )
        };
    }

    unsafe {
        i2c_set_clientdata(i2c, rt9123 as *mut c_void);

        pm_runtime_set_autosuspend_delay(dev, 500);
        pm_runtime_use_autosuspend(dev);
        pm_runtime_set_active(dev);
    }
    ret = unsafe { devm_pm_runtime_enable(dev) };
    if ret != 0 {
        return unsafe { dev_err_probe(dev, ret, b"Failed to enable pm runtime\n\0".as_ptr() as *const c_char) };
    }

    unsafe { devm_snd_soc_register_component(dev, &rt9123_comp_driver, &raw mut rt9123_dai_driver, 1) }
}

/* CONFIG_PM */
unsafe extern "C" fn rt9123_runtime_suspend(dev: *mut device) -> c_int {
    let rt9123 = unsafe { dev_get_drvdata(dev) as *mut rt9123_priv };
    let regmap = unsafe { dev_get_regmap(dev, core::ptr::null()) };

    if unsafe { !(*rt9123).enable.is_null() } {
        unsafe {
            regcache_cache_only(regmap, true);
            regcache_mark_dirty(regmap);
            gpiod_set_value((*rt9123).enable, 0);
        }
    }

    0
}

unsafe extern "C" fn rt9123_runtime_resume(dev: *mut device) -> c_int {
    let rt9123 = unsafe { dev_get_drvdata(dev) as *mut rt9123_priv };
    let regmap = unsafe { dev_get_regmap(dev, core::ptr::null()) };
    let ret: c_int;

    if unsafe { !(*rt9123).enable.is_null() } {
        unsafe {
            gpiod_set_value((*rt9123).enable, 1);
            usleep_range(250, 350);

            regcache_cache_only(regmap, false);
        }
        ret = unsafe { regcache_sync(regmap) };
        if ret != 0 {
            return ret;
        }
    }

    0
}

static rt9123_dev_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(rt9123_runtime_suspend),
    runtime_resume: Some(rt9123_runtime_resume),
};

/* CONFIG_OF */
static rt9123_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: b"richtek,rt9123\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, rt9123_device_id); */

/* CONFIG_ACPI */
static rt9123_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id {
        id: b"RT9123\0".as_ptr() as *const c_char,
        driver_data: 0,
    },
    acpi_device_id {
        id: core::ptr::null(),
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(acpi, rt9123_acpi_match); */

static mut rt9123_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"rt9123\0".as_ptr() as *const c_char,
        of_match_table: rt9123_device_id.as_ptr(),
        acpi_match_table: rt9123_acpi_match.as_ptr(),
        pm: &rt9123_dev_pm_ops,
    },
    probe: Some(rt9123_i2c_probe),
};
/* module_i2c_driver(rt9123_i2c_driver); */

/* MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>"); */
/* MODULE_DESCRIPTION("ASoC rt9123 Driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
