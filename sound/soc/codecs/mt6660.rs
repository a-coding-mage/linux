// SPDX-License-Identifier: GPL-2.0

// Copyright (c) 2019 MediaTek Inc.

// Translated from C implementation source. External Linux/ASoC/regmap/i2c
// symbols and MT6660 register definitions are expected to be supplied by the
// surrounding repository bindings.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type uint32_t = u32;
type size_t = usize;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
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
pub struct mt6660_chip {
    pub i2c: *mut i2c_client,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub io_lock: mutex,
    pub chip_rev: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
    pub reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
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
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

type c_long = isize;

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: bool,
    pub endianness: c_uint,
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
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
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
    /* dai properties */
    pub symmetric_rate: c_uint,
    pub symmetric_channels: c_uint,
    pub symmetric_sample_bits: c_uint,
    /* dai operations */
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    static MT6660_REG_HPF1_COEF: c_uint;
    static MT6660_REG_HPF2_COEF: c_uint;
    static MT6660_REG_TDM_CFG3: c_uint;
    static MT6660_REG_RESV17: c_uint;
    static MT6660_REG_RESV23: c_uint;
    static MT6660_REG_SIGMAX: c_uint;
    static MT6660_REG_DEVID: c_uint;
    static MT6660_REG_HCLIP_CTRL: c_uint;
    static MT6660_REG_DA_GAIN: c_uint;
    static MT6660_REG_PLL_CFG1: c_uint;
    static MT6660_REG_SYSTEM_CTRL: c_uint;
    static MT6660_REG_BST_CTRL: c_uint;
    static MT6660_REG_RESV7: c_uint;
    static MT6660_REG_RESV10: c_uint;
    static MT6660_REG_VOL_CTRL: c_uint;
    static MT6660_REG_SPS_CTRL: c_uint;
    static MT6660_REG_DRE_CTRL: c_uint;
    static MT6660_REG_DC_PROTECT_CTRL: c_uint;
    static MT6660_REG_DATAO_SEL: c_uint;
    static MT6660_REG_CALI_T0: c_uint;
    static MT6660_REG_SERIAL_CFG1: c_uint;
    static SND_SOC_NOPM: c_uint;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_U16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_U24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_FMTBIT_U32_LE: u64;
    static GFP_KERNEL: c_uint;
    static ENOTSUPP: c_int;
    static ENODEV: c_int;
    static ENOMEM: c_int;

    fn i2c_smbus_write_i2c_block_data(
        client: *mut i2c_client,
        command: c_uint,
        length: c_int,
        values: *const u8,
    ) -> c_int;
    fn i2c_smbus_read_i2c_block_data(
        client: *mut i2c_client,
        command: c_uint,
        length: c_int,
        values: *mut u8,
    ) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn msleep(msecs: c_uint);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn snd_soc_component_exit_regmap(component: *mut snd_soc_component);
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn regmap_write_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(regmap: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(regmap: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const c_void,
        bus_context: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_set_suspended(dev: *mut device);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn params_physical_width(hw_params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(hw_params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(hw_params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(hw_params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn SND_SOC_DAPM_DAC_E(
        name: *const c_char,
        stream: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
        flags: c_int,
    ) -> snd_soc_dapm_widget_desc;
    fn SND_SOC_DAPM_ADC(
        name: *const c_char,
        stream: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
    ) -> snd_soc_dapm_widget_desc;
    fn SND_SOC_DAPM_PGA(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> snd_soc_dapm_widget_desc;
    fn SND_SOC_DAPM_OUT_DRV_E(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
        event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
        flags: c_int,
    ) -> snd_soc_dapm_widget_desc;
    fn SND_SOC_DAPM_OUTPUT(name: *const c_char) -> snd_soc_dapm_widget_desc;
    fn SOC_SINGLE_TLV(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
        tlv: *const c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_SINGLE(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_SINGLE_EXT(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
        get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
        put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    ) -> snd_kcontrol_new;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

#[repr(C)]
struct reg_size_table {
    addr: u32,
    size: u8,
}

static mut mt6660_reg_size_table: [reg_size_table; 9] = unsafe {
    [
        reg_size_table { addr: MT6660_REG_HPF1_COEF, size: 4 },
        reg_size_table { addr: MT6660_REG_HPF2_COEF, size: 4 },
        reg_size_table { addr: MT6660_REG_TDM_CFG3, size: 2 },
        reg_size_table { addr: MT6660_REG_RESV17, size: 2 },
        reg_size_table { addr: MT6660_REG_RESV23, size: 2 },
        reg_size_table { addr: MT6660_REG_SIGMAX, size: 2 },
        reg_size_table { addr: MT6660_REG_DEVID, size: 2 },
        reg_size_table { addr: MT6660_REG_HCLIP_CTRL, size: 2 },
        reg_size_table { addr: MT6660_REG_DA_GAIN, size: 2 },
    ]
};

unsafe extern "C" fn mt6660_get_reg_size(addr: uint32_t) -> c_int {
    let mut i: c_int = 0;

    while (i as usize) < mt6660_reg_size_table.len() {
        if mt6660_reg_size_table[i as usize].addr == addr {
            return mt6660_reg_size_table[i as usize].size as c_int;
        }
        i += 1;
    }
    1
}

unsafe extern "C" fn mt6660_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    let chip: *mut mt6660_chip = context as *mut mt6660_chip;
    let size: c_int = mt6660_get_reg_size(reg);
    let mut reg_data: [u8; 4] = [0; 4];
    let mut i: c_int = 0;

    while i < size {
        reg_data[(size - i - 1) as usize] = ((val >> (8 * i)) & 0xff) as u8;
        i += 1;
    }

    i2c_smbus_write_i2c_block_data((*chip).i2c, reg, size, reg_data.as_ptr())
}

unsafe extern "C" fn mt6660_reg_read(
    context: *mut c_void,
    reg: c_uint,
    val: *mut c_uint,
) -> c_int {
    let chip: *mut mt6660_chip = context as *mut mt6660_chip;
    let size: c_int = mt6660_get_reg_size(reg);
    let mut i: c_int = 0;
    let mut ret: c_int;
    let mut data: [u8; 4] = [0; 4];
    let mut reg_data: u32 = 0;

    ret = i2c_smbus_read_i2c_block_data((*chip).i2c, reg, size, data.as_mut_ptr());
    if ret < 0 {
        return ret;
    }
    while i < size {
        reg_data <<= 8;
        reg_data |= data[i as usize] as u32;
        i += 1;
    }
    *val = reg_data;
    0
}

static mt6660_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 32,
    reg_write: Some(mt6660_reg_write),
    reg_read: Some(mt6660_reg_read),
};

unsafe extern "C" fn mt6660_codec_dac_event(
    _w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    if event == SND_SOC_DAPM_POST_PMU {
        usleep_range(1000, 1100);
    }
    0
}

unsafe extern "C" fn mt6660_codec_classd_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let mut ret: c_int;

    match event {
        x if x == SND_SOC_DAPM_PRE_PMU => {
            dev_dbg((*component).dev, cstr!("%s: before classd turn on\n"), cstr!("mt6660_codec_classd_event"));
            /* config to adaptive mode */
            ret = snd_soc_component_update_bits(component, MT6660_REG_BST_CTRL, 0x03, 0x03);
            if ret < 0 {
                dev_err((*component).dev, cstr!("config mode adaptive fail\n"));
                return ret;
            }
        }
        x if x == SND_SOC_DAPM_POST_PMU => {
            /* voltage sensing enable */
            ret = snd_soc_component_update_bits(component, MT6660_REG_RESV7, 0x04, 0x04);
            if ret < 0 {
                dev_err((*component).dev, cstr!("enable voltage sensing fail\n"));
                return ret;
            }
            dev_dbg((*component).dev, cstr!("Amp on\n"));
        }
        x if x == SND_SOC_DAPM_PRE_PMD => {
            dev_dbg((*component).dev, cstr!("Amp off\n"));
            /* voltage sensing disable */
            ret = snd_soc_component_update_bits(component, MT6660_REG_RESV7, 0x04, 0x00);
            if ret < 0 {
                dev_err((*component).dev, cstr!("disable voltage sensing fail\n"));
                return ret;
            }
            /* pop-noise improvement 1 */
            ret = snd_soc_component_update_bits(component, MT6660_REG_RESV10, 0x10, 0x10);
            if ret < 0 {
                dev_err((*component).dev, cstr!("pop-noise improvement 1 fail\n"));
                return ret;
            }
        }
        x if x == SND_SOC_DAPM_POST_PMD => {
            dev_dbg((*component).dev, cstr!("%s: after classd turn off\n"), cstr!("mt6660_codec_classd_event"));
            /* pop-noise improvement 2 */
            ret = snd_soc_component_update_bits(component, MT6660_REG_RESV10, 0x10, 0x00);
            if ret < 0 {
                dev_err((*component).dev, cstr!("pop-noise improvement 2 fail\n"));
                return ret;
            }
            /* config to off mode */
            ret = snd_soc_component_update_bits(component, MT6660_REG_BST_CTRL, 0x03, 0x00);
            if ret < 0 {
                dev_err((*component).dev, cstr!("config mode off fail\n"));
                return ret;
            }
        }
        _ => {}
    }
    0
}

static mut mt6660_component_dapm_widgets: [snd_soc_dapm_widget_desc; 6] = unsafe {
    [
        SND_SOC_DAPM_DAC_E(
            cstr!("DAC"),
            ptr::null(),
            MT6660_REG_PLL_CFG1,
            0,
            1,
            Some(mt6660_codec_dac_event),
            SND_SOC_DAPM_POST_PMU,
        ),
        SND_SOC_DAPM_ADC(cstr!("VI ADC"), ptr::null(), SND_SOC_NOPM, 0, 0),
        SND_SOC_DAPM_PGA(cstr!("PGA"), SND_SOC_NOPM, 0, 0, ptr::null(), 0),
        SND_SOC_DAPM_OUT_DRV_E(
            cstr!("ClassD"),
            MT6660_REG_SYSTEM_CTRL,
            2,
            0,
            ptr::null(),
            0,
            Some(mt6660_codec_classd_event),
            SND_SOC_DAPM_PRE_PMU
                | SND_SOC_DAPM_POST_PMU
                | SND_SOC_DAPM_PRE_PMD
                | SND_SOC_DAPM_POST_PMD,
        ),
        SND_SOC_DAPM_OUTPUT(cstr!("OUTP")),
        SND_SOC_DAPM_OUTPUT(cstr!("OUTN")),
    ]
};

static mt6660_component_dapm_routes: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("aif_playback") },
    snd_soc_dapm_route { sink: cstr!("PGA"), control: ptr::null(), source: cstr!("DAC") },
    snd_soc_dapm_route { sink: cstr!("ClassD"), control: ptr::null(), source: cstr!("PGA") },
    snd_soc_dapm_route { sink: cstr!("OUTP"), control: ptr::null(), source: cstr!("ClassD") },
    snd_soc_dapm_route { sink: cstr!("OUTN"), control: ptr::null(), source: cstr!("ClassD") },
    snd_soc_dapm_route { sink: cstr!("VI ADC"), control: ptr::null(), source: cstr!("ClassD") },
    snd_soc_dapm_route { sink: cstr!("aif_capture"), control: ptr::null(), source: cstr!("VI ADC") },
];

unsafe extern "C" fn mt6660_component_get_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let chip: *mut mt6660_chip = snd_soc_component_get_drvdata(component) as *mut mt6660_chip;

    (*ucontrol).value.integer.value[0] = ((*chip).chip_rev & 0x0f) as c_long;
    0
}

static vol_ctl_tlv: [c_uint; 4] = [0, 0, (-1155i32) as c_uint, 5];

static mut mt6660_component_snd_controls: [snd_kcontrol_new; 10] = unsafe {
    [
        SOC_SINGLE_TLV(cstr!("Digital Volume"), MT6660_REG_VOL_CTRL, 0, 255, 1, vol_ctl_tlv.as_ptr()),
        SOC_SINGLE(cstr!("Hard Clip Switch"), MT6660_REG_HCLIP_CTRL, 8, 1, 0),
        SOC_SINGLE(cstr!("Clip Switch"), MT6660_REG_SPS_CTRL, 0, 1, 0),
        SOC_SINGLE(cstr!("Boost Mode"), MT6660_REG_BST_CTRL, 0, 3, 0),
        SOC_SINGLE(cstr!("DRE Switch"), MT6660_REG_DRE_CTRL, 0, 1, 0),
        SOC_SINGLE(cstr!("DC Protect Switch"), MT6660_REG_DC_PROTECT_CTRL, 3, 1, 0),
        SOC_SINGLE(cstr!("Data Output Left Channel Selection"), MT6660_REG_DATAO_SEL, 3, 7, 0),
        SOC_SINGLE(cstr!("Data Output Right Channel Selection"), MT6660_REG_DATAO_SEL, 0, 7, 0),
        SOC_SINGLE_EXT(cstr!("T0 SEL"), MT6660_REG_CALI_T0, 0, 7, 0, Some(snd_soc_get_volsw), None),
        SOC_SINGLE_EXT(cstr!("Chip Rev"), MT6660_REG_DEVID, 8, 15, 0, Some(mt6660_component_get_volsw), None),
    ]
};

unsafe extern "C" fn _mt6660_chip_power_on(chip: *mut mt6660_chip, on_off: c_int) -> c_int {
    regmap_write_bits(
        (*chip).regmap,
        MT6660_REG_SYSTEM_CTRL,
        0x01,
        if on_off != 0 { 0x00 } else { 0x01 },
    )
}

#[repr(C)]
struct reg_table {
    addr: uint32_t,
    mask: uint32_t,
    val: uint32_t,
}

static mt6660_setting_table: [reg_table; 26] = [
    reg_table { addr: 0x20, mask: 0x80, val: 0x00 },
    reg_table { addr: 0x30, mask: 0x01, val: 0x00 },
    reg_table { addr: 0x50, mask: 0x1c, val: 0x04 },
    reg_table { addr: 0xB1, mask: 0x0c, val: 0x00 },
    reg_table { addr: 0xD3, mask: 0x03, val: 0x03 },
    reg_table { addr: 0xE0, mask: 0x01, val: 0x00 },
    reg_table { addr: 0x98, mask: 0x44, val: 0x04 },
    reg_table { addr: 0xB9, mask: 0xff, val: 0x82 },
    reg_table { addr: 0xB7, mask: 0x7777, val: 0x7273 },
    reg_table { addr: 0xB6, mask: 0x07, val: 0x03 },
    reg_table { addr: 0x6B, mask: 0xe0, val: 0x20 },
    reg_table { addr: 0x07, mask: 0xff, val: 0x70 },
    reg_table { addr: 0xBB, mask: 0xff, val: 0x20 },
    reg_table { addr: 0x69, mask: 0xff, val: 0x40 },
    reg_table { addr: 0xBD, mask: 0xffff, val: 0x17f8 },
    reg_table { addr: 0x70, mask: 0xff, val: 0x15 },
    reg_table { addr: 0x7C, mask: 0xff, val: 0x00 },
    reg_table { addr: 0x46, mask: 0xff, val: 0x1d },
    reg_table { addr: 0x1A, mask: 0xffffffff, val: 0x7fdb7ffe },
    reg_table { addr: 0x1B, mask: 0xffffffff, val: 0x7fdb7ffe },
    reg_table { addr: 0x51, mask: 0xff, val: 0x58 },
    reg_table { addr: 0xA2, mask: 0xff, val: 0xce },
    reg_table { addr: 0x33, mask: 0xffff, val: 0x7fff },
    reg_table { addr: 0x4C, mask: 0xffff, val: 0x0116 },
    reg_table { addr: 0x16, mask: 0x1800, val: 0x0800 },
    reg_table { addr: 0x68, mask: 0x1f, val: 0x07 },
];

unsafe extern "C" fn mt6660_component_setting(component: *mut snd_soc_component) -> c_int {
    let chip: *mut mt6660_chip = snd_soc_component_get_drvdata(component) as *mut mt6660_chip;
    let mut ret: c_int = 0;
    let mut i: size_t = 0;

    ret = _mt6660_chip_power_on(chip, 1);
    if ret < 0 {
        dev_err((*component).dev, cstr!("%s chip power on failed\n"), cstr!("mt6660_component_setting"));
        return ret;
    }

    while i < mt6660_setting_table.len() {
        ret = snd_soc_component_update_bits(
            component,
            mt6660_setting_table[i].addr,
            mt6660_setting_table[i].mask,
            mt6660_setting_table[i].val,
        );
        if ret < 0 {
            dev_err(
                (*component).dev,
                cstr!("%s update 0x%02x failed\n"),
                cstr!("mt6660_component_setting"),
                mt6660_setting_table[i].addr,
            );
            return ret;
        }
        i += 1;
    }

    ret = _mt6660_chip_power_on(chip, 0);
    if ret < 0 {
        dev_err((*component).dev, cstr!("%s chip power off failed\n"), cstr!("mt6660_component_setting"));
        return ret;
    }

    0
}

unsafe extern "C" fn mt6660_component_probe(component: *mut snd_soc_component) -> c_int {
    let chip: *mut mt6660_chip = snd_soc_component_get_drvdata(component) as *mut mt6660_chip;
    let ret: c_int;

    dev_dbg((*component).dev, cstr!("%s\n"), cstr!("mt6660_component_probe"));
    snd_soc_component_init_regmap(component, (*chip).regmap);

    ret = mt6660_component_setting(component);
    if ret < 0 {
        dev_err((*chip).dev, cstr!("mt6660 component setting failed\n"));
    }

    ret
}

unsafe extern "C" fn mt6660_component_remove(component: *mut snd_soc_component) {
    dev_dbg((*component).dev, cstr!("%s\n"), cstr!("mt6660_component_remove"));
    snd_soc_component_exit_regmap(component);
}

static mt6660_component_driver: snd_soc_component_driver = unsafe {
    snd_soc_component_driver {
        probe: Some(mt6660_component_probe),
        remove: Some(mt6660_component_remove),
        controls: mt6660_component_snd_controls.as_ptr(),
        num_controls: mt6660_component_snd_controls.len() as c_uint,
        dapm_widgets: mt6660_component_dapm_widgets.as_ptr(),
        num_dapm_widgets: mt6660_component_dapm_widgets.len() as c_uint,
        dapm_routes: mt6660_component_dapm_routes.as_ptr(),
        num_dapm_routes: mt6660_component_dapm_routes.len() as c_uint,
        idle_bias_on: false, /* idle_bias_off = true */
        endianness: 1,
    }
};

unsafe extern "C" fn mt6660_component_aif_hw_params(
    _substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let word_len: c_int = params_physical_width(hw_params);
    let aud_bit: c_int = params_width(hw_params);
    let mut reg_data: u16 = 0;
    let mut ret: c_int;

    dev_dbg((*dai).dev, cstr!("%s: ++\n"), cstr!("mt6660_component_aif_hw_params"));
    dev_dbg((*dai).dev, cstr!("format: 0x%08x\n"), params_format(hw_params));
    dev_dbg((*dai).dev, cstr!("rate: 0x%08x\n"), params_rate(hw_params));
    dev_dbg((*dai).dev, cstr!("word_len: %d, aud_bit: %d\n"), word_len, aud_bit);
    if word_len > 32 || word_len < 16 {
        dev_err((*dai).dev, cstr!("not supported word length\n"));
        return -ENOTSUPP;
    }
    match aud_bit {
        16 => {
            reg_data = 3;
        }
        18 => {
            reg_data = 2;
        }
        20 => {
            reg_data = 1;
        }
        24 | 32 => {
            reg_data = 0;
        }
        _ => {
            return -ENOTSUPP;
        }
    }
    ret = snd_soc_component_update_bits(
        (*dai).component,
        MT6660_REG_SERIAL_CFG1,
        0xc0,
        (reg_data << 6) as c_uint,
    );
    if ret < 0 {
        dev_err((*dai).dev, cstr!("config aud bit fail\n"));
        return ret;
    }
    ret = snd_soc_component_update_bits(
        (*dai).component,
        MT6660_REG_TDM_CFG3,
        0x3f0,
        (word_len << 4) as c_uint,
    );
    if ret < 0 {
        dev_err((*dai).dev, cstr!("config word len fail\n"));
        return ret;
    }
    dev_dbg((*dai).dev, cstr!("%s: --\n"), cstr!("mt6660_component_aif_hw_params"));
    0
}

static mt6660_component_aif_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mt6660_component_aif_hw_params),
};

// #define STUB_RATES SNDRV_PCM_RATE_8000_192000
// #define STUB_FORMATS (SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE |
// SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_U24_LE |
// SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_U32_LE)

static mut mt6660_codec_dai: snd_soc_dai_driver = unsafe {
    snd_soc_dai_driver {
        name: cstr!("mt6660-aif"),
        playback: snd_soc_pcm_stream {
            stream_name: cstr!("aif_playback"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_U16_LE
                | SNDRV_PCM_FMTBIT_S24_LE
                | SNDRV_PCM_FMTBIT_U24_LE
                | SNDRV_PCM_FMTBIT_S32_LE
                | SNDRV_PCM_FMTBIT_U32_LE,
        },
        capture: snd_soc_pcm_stream {
            stream_name: cstr!("aif_capture"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_U16_LE
                | SNDRV_PCM_FMTBIT_S24_LE
                | SNDRV_PCM_FMTBIT_U24_LE
                | SNDRV_PCM_FMTBIT_S32_LE
                | SNDRV_PCM_FMTBIT_U32_LE,
        },
        /* dai properties */
        symmetric_rate: 1,
        symmetric_channels: 1,
        symmetric_sample_bits: 1,
        /* dai operations */
        ops: &mt6660_component_aif_ops,
    }
};

unsafe extern "C" fn _mt6660_chip_id_check(chip: *mut mt6660_chip) -> c_int {
    let mut ret: c_int;
    let mut val: c_uint = 0;

    ret = regmap_read((*chip).regmap, MT6660_REG_DEVID, &mut val);
    if ret < 0 {
        return ret;
    }
    val &= 0x0ff0;
    if val != 0x00e0 && val != 0x01e0 {
        dev_err((*chip).dev, cstr!("%s id(%x) not match\n"), cstr!("_mt6660_chip_id_check"), val);
        return -ENODEV;
    }
    0
}

unsafe extern "C" fn _mt6660_chip_sw_reset(chip: *mut mt6660_chip) -> c_int {
    let mut ret: c_int;

    /* turn on main pll first, then trigger reset */
    ret = regmap_write((*chip).regmap, MT6660_REG_SYSTEM_CTRL, 0x00);
    if ret < 0 {
        return ret;
    }
    ret = regmap_write((*chip).regmap, MT6660_REG_SYSTEM_CTRL, 0x80);
    if ret < 0 {
        return ret;
    }
    msleep(30);
    0
}

unsafe extern "C" fn _mt6660_read_chip_revision(chip: *mut mt6660_chip) -> c_int {
    let mut ret: c_int;
    let mut val: c_uint = 0;

    ret = regmap_read((*chip).regmap, MT6660_REG_DEVID, &mut val);
    if ret < 0 {
        dev_err((*chip).dev, cstr!("get chip revision fail\n"));
        return ret;
    }
    (*chip).chip_rev = val & 0xff;
    dev_info((*chip).dev, cstr!("%s chip_rev = %x\n"), cstr!("_mt6660_read_chip_revision"), (*chip).chip_rev);
    0
}

unsafe extern "C" fn mt6660_i2c_probe(client: *mut i2c_client) -> c_int {
    let mut chip: *mut mt6660_chip = ptr::null_mut();
    let mut ret: c_int;

    dev_dbg(&mut (*client).dev, cstr!("%s\n"), cstr!("mt6660_i2c_probe"));
    chip = devm_kzalloc(
        &mut (*client).dev,
        core::mem::size_of::<mt6660_chip>(),
        GFP_KERNEL,
    ) as *mut mt6660_chip;
    if chip.is_null() {
        return -ENOMEM;
    }
    (*chip).i2c = client;
    (*chip).dev = &mut (*client).dev;
    mutex_init(&mut (*chip).io_lock);
    i2c_set_clientdata(client, chip as *mut c_void);

    (*chip).regmap = devm_regmap_init(
        &mut (*client).dev,
        ptr::null(),
        chip as *mut c_void,
        &mt6660_regmap_config,
    );
    if IS_ERR((*chip).regmap as *const c_void) {
        ret = PTR_ERR((*chip).regmap as *const c_void);
        dev_err(&mut (*client).dev, cstr!("failed to initialise regmap: %d\n"), ret);
        return ret;
    }

    /* chip reset first */
    ret = _mt6660_chip_sw_reset(chip);
    if ret < 0 {
        dev_err((*chip).dev, cstr!("chip reset fail\n"));
        _mt6660_chip_power_on(chip, 0);
        mutex_destroy(&mut (*chip).io_lock);
        return ret;
    }
    /* chip power on */
    ret = _mt6660_chip_power_on(chip, 1);
    if ret < 0 {
        dev_err((*chip).dev, cstr!("chip power on 2 fail\n"));
        _mt6660_chip_power_on(chip, 0);
        mutex_destroy(&mut (*chip).io_lock);
        return ret;
    }
    /* chip devid check */
    ret = _mt6660_chip_id_check(chip);
    if ret < 0 {
        dev_err((*chip).dev, cstr!("chip id check fail\n"));
        _mt6660_chip_power_on(chip, 0);
        mutex_destroy(&mut (*chip).io_lock);
        return ret;
    }
    /* chip revision get */
    ret = _mt6660_read_chip_revision(chip);
    if ret < 0 {
        dev_err((*chip).dev, cstr!("read chip revision fail\n"));
        _mt6660_chip_power_on(chip, 0);
        mutex_destroy(&mut (*chip).io_lock);
        return ret;
    }
    pm_runtime_set_active((*chip).dev);
    pm_runtime_enable((*chip).dev);

    ret = devm_snd_soc_register_component(
        (*chip).dev,
        &mt6660_component_driver,
        &mut mt6660_codec_dai,
        1,
    );
    if ret != 0 {
        pm_runtime_disable((*chip).dev);
    }

    ret
}

unsafe extern "C" fn mt6660_i2c_remove(client: *mut i2c_client) {
    let chip: *mut mt6660_chip = i2c_get_clientdata(client) as *mut mt6660_chip;

    pm_runtime_disable((*chip).dev);
    pm_runtime_set_suspended((*chip).dev);
    mutex_destroy(&mut (*chip).io_lock);
}

unsafe extern "C" fn mt6660_i2c_runtime_suspend(dev: *mut device) -> c_int {
    let chip: *mut mt6660_chip = dev_get_drvdata(dev) as *mut mt6660_chip;

    dev_dbg(dev, cstr!("enter low power mode\n"));
    regmap_update_bits((*chip).regmap, MT6660_REG_SYSTEM_CTRL, 0x01, 0x01)
}

unsafe extern "C" fn mt6660_i2c_runtime_resume(dev: *mut device) -> c_int {
    let chip: *mut mt6660_chip = dev_get_drvdata(dev) as *mut mt6660_chip;

    dev_dbg(dev, cstr!("exit low power mode\n"));
    regmap_update_bits((*chip).regmap, MT6660_REG_SYSTEM_CTRL, 0x01, 0x00)
}

static mt6660_dev_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(mt6660_i2c_runtime_suspend),
    runtime_resume: Some(mt6660_i2c_runtime_resume),
    runtime_idle: None,
};

static mt6660_of_id: [of_device_id; 2] = [
    of_device_id { compatible: cstr!("mediatek,mt6660") },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, mt6660_of_id);

static mt6660_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: cstr!("mt6660") },
    i2c_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, mt6660_i2c_id);

static mut mt6660_i2c_driver: i2c_driver = unsafe {
    i2c_driver {
        driver: device_driver {
            name: cstr!("mt6660"),
            of_match_table: mt6660_of_id.as_ptr(),
            pm: &mt6660_dev_pm_ops,
        },
        probe: Some(mt6660_i2c_probe),
        remove: Some(mt6660_i2c_remove),
        id_table: mt6660_i2c_id.as_ptr(),
    }
};
// module_i2c_driver(mt6660_i2c_driver);

// MODULE_AUTHOR("Jeff Chang <jeff_chang@richtek.com>");
// MODULE_DESCRIPTION("MT6660 SPKAMP Driver");
// MODULE_LICENSE("GPL");
// MODULE_VERSION("1.0.8_G");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
