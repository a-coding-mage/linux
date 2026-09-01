// SPDX-License-Identifier: GPL-2.0-only
/*
 * Pistachio internal dac driver
 *
 * Copyright (C) 2015 Imagination Technologies Ltd.
 *
 * Author: Damien Horsley <Damien.Horsley@imgtec.com>
 */

// C includes translated as external dependencies:
// linux/clk.h, linux/delay.h, linux/mfd/syscon.h, linux/module.h,
// linux/pm_runtime.h, linux/regmap.h, linux/regulator/consumer.h,
// sound/pcm_params.h, sound/soc.h

use core::ffi::{c_char, c_int, c_void};

const PISTACHIO_INTERNAL_DAC_CTRL: u32 = 0x40;
const PISTACHIO_INTERNAL_DAC_CTRL_PWR_SEL_MASK: u32 = 0x2;
const PISTACHIO_INTERNAL_DAC_CTRL_PWRDN_MASK: u32 = 0x1;

const PISTACHIO_INTERNAL_DAC_SRST: u32 = 0x44;
const PISTACHIO_INTERNAL_DAC_SRST_MASK: u32 = 0x1;

const PISTACHIO_INTERNAL_DAC_GTI_CTRL: u32 = 0x48;
const PISTACHIO_INTERNAL_DAC_GTI_CTRL_ADDR_SHIFT: u32 = 0;
const PISTACHIO_INTERNAL_DAC_GTI_CTRL_ADDR_MASK: u32 = 0xFFF;
const PISTACHIO_INTERNAL_DAC_GTI_CTRL_WE_MASK: u32 = 0x1000;
const PISTACHIO_INTERNAL_DAC_GTI_CTRL_WDATA_SHIFT: u32 = 13;
const PISTACHIO_INTERNAL_DAC_GTI_CTRL_WDATA_MASK: u32 = 0x1FE000;

const PISTACHIO_INTERNAL_DAC_PWR: u32 = 0x1;
const PISTACHIO_INTERNAL_DAC_PWR_MASK: u32 = 0x1;

const PISTACHIO_INTERNAL_DAC_FORMATS: u64 = SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const SND_SOC_NOPM: c_int = 0;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1u64 << 6;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1u64 << 10;
const SNDRV_PCM_RATE_8000_48000: u32 = 0;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub info: *const c_void,
    pub get: *const c_void,
    pub put: *const c_void,
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub sname: *const c_char,
    pub reg: c_int,
    pub shift: u8,
    pub invert: u8,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: u32,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: u32,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: u32,
    pub use_pmdown_time: c_int,
    pub endianness: c_int,
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
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

/* codec private data */
#[repr(C)]
pub struct pistachio_internal_dac {
    pub regmap: *mut regmap,
    pub supply: *mut regulator,
    pub mute: bool,
}

unsafe extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn syscon_regmap_lookup_by_phandle(np: *mut device_node, property: *const c_char) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_get_voltage(regulator: *mut regulator) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_idle(dev: *mut device);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
}

const fn array_size<T, const N: usize>(_: &[T; N]) -> u32 {
    N as u32
}

const fn SOC_SINGLE_VALUE(reg: u32, shift: u32, max: u32, invert: u32, _autodisable: u32) -> usize {
    ((reg as usize) << 16) | ((shift as usize) << 8) | ((max as usize) << 1) | (invert as usize)
}

static pistachio_internal_dac_snd_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    iface: 0,
    name: b"Playback Switch\0".as_ptr() as *const c_char,
    info: core::ptr::null(),
    get: core::ptr::null(),
    put: core::ptr::null(),
    private_value: SOC_SINGLE_VALUE(PISTACHIO_INTERNAL_DAC_CTRL, 2, 1, 1, 0),
}];

static pistachio_internal_dac_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_widget {
        id: 0,
        name: b"DAC\0".as_ptr() as *const c_char,
        sname: b"Playback\0".as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: b"AOUTL\0".as_ptr() as *const c_char,
        sname: core::ptr::null(),
        reg: 0,
        shift: 0,
        invert: 0,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: b"AOUTR\0".as_ptr() as *const c_char,
        sname: core::ptr::null(),
        reg: 0,
        shift: 0,
        invert: 0,
    },
];

static pistachio_internal_dac_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"AOUTL\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"AOUTR\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
];

unsafe fn pistachio_internal_dac_reg_writel(top_regs: *mut regmap, val: u32, reg: u32) {
    unsafe {
        regmap_update_bits(
            top_regs,
            PISTACHIO_INTERNAL_DAC_GTI_CTRL,
            PISTACHIO_INTERNAL_DAC_GTI_CTRL_ADDR_MASK,
            reg << PISTACHIO_INTERNAL_DAC_GTI_CTRL_ADDR_SHIFT,
        );

        regmap_update_bits(
            top_regs,
            PISTACHIO_INTERNAL_DAC_GTI_CTRL,
            PISTACHIO_INTERNAL_DAC_GTI_CTRL_WDATA_MASK,
            val << PISTACHIO_INTERNAL_DAC_GTI_CTRL_WDATA_SHIFT,
        );

        regmap_update_bits(
            top_regs,
            PISTACHIO_INTERNAL_DAC_GTI_CTRL,
            PISTACHIO_INTERNAL_DAC_GTI_CTRL_WE_MASK,
            PISTACHIO_INTERNAL_DAC_GTI_CTRL_WE_MASK,
        );

        regmap_update_bits(
            top_regs,
            PISTACHIO_INTERNAL_DAC_GTI_CTRL,
            PISTACHIO_INTERNAL_DAC_GTI_CTRL_WE_MASK,
            0,
        );
    }
}

unsafe fn pistachio_internal_dac_pwr_off(dac: *mut pistachio_internal_dac) {
    unsafe {
        regmap_update_bits(
            (*dac).regmap,
            PISTACHIO_INTERNAL_DAC_CTRL,
            PISTACHIO_INTERNAL_DAC_CTRL_PWRDN_MASK,
            PISTACHIO_INTERNAL_DAC_CTRL_PWRDN_MASK,
        );

        pistachio_internal_dac_reg_writel((*dac).regmap, 0, PISTACHIO_INTERNAL_DAC_PWR);
    }
}

unsafe fn pistachio_internal_dac_pwr_on(dac: *mut pistachio_internal_dac) {
    unsafe {
        regmap_update_bits(
            (*dac).regmap,
            PISTACHIO_INTERNAL_DAC_SRST,
            PISTACHIO_INTERNAL_DAC_SRST_MASK,
            PISTACHIO_INTERNAL_DAC_SRST_MASK,
        );

        regmap_update_bits(
            (*dac).regmap,
            PISTACHIO_INTERNAL_DAC_SRST,
            PISTACHIO_INTERNAL_DAC_SRST_MASK,
            0,
        );

        pistachio_internal_dac_reg_writel(
            (*dac).regmap,
            PISTACHIO_INTERNAL_DAC_PWR_MASK,
            PISTACHIO_INTERNAL_DAC_PWR,
        );

        regmap_update_bits(
            (*dac).regmap,
            PISTACHIO_INTERNAL_DAC_CTRL,
            PISTACHIO_INTERNAL_DAC_CTRL_PWRDN_MASK,
            0,
        );
    }
}

static mut pistachio_internal_dac_dais: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"pistachio_internal_dac\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: PISTACHIO_INTERNAL_DAC_FORMATS,
    },
}];

unsafe extern "C" fn pistachio_internal_dac_codec_probe(
    component: *mut snd_soc_component,
) -> c_int {
    unsafe {
        let dac = snd_soc_component_get_drvdata(component) as *mut pistachio_internal_dac;

        snd_soc_component_init_regmap(component, (*dac).regmap);

        0
    }
}

static pistachio_internal_dac_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(pistachio_internal_dac_codec_probe),
    controls: pistachio_internal_dac_snd_controls.as_ptr(),
    num_controls: array_size(&pistachio_internal_dac_snd_controls),
    dapm_widgets: pistachio_internal_dac_widgets.as_ptr(),
    num_dapm_widgets: array_size(&pistachio_internal_dac_widgets),
    dapm_routes: pistachio_internal_dac_routes.as_ptr(),
    num_dapm_routes: array_size(&pistachio_internal_dac_routes),
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn pistachio_internal_dac_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        let mut ret: c_int;
        let voltage: c_int;
        let dev = &mut (*pdev).dev as *mut device;
        let mut reg: u32;

        let dac = devm_kzalloc(
            dev,
            core::mem::size_of::<pistachio_internal_dac>(),
            GFP_KERNEL,
        ) as *mut pistachio_internal_dac;

        if dac.is_null() {
            return -ENOMEM;
        }

        platform_set_drvdata(pdev, dac as *mut c_void);

        (*dac).regmap =
            syscon_regmap_lookup_by_phandle((*pdev).dev.of_node, b"img,cr-top\0".as_ptr() as *const c_char);
        if IS_ERR((*dac).regmap as *const c_void) {
            return PTR_ERR((*dac).regmap as *const c_void);
        }

        (*dac).supply = devm_regulator_get(dev, b"VDD\0".as_ptr() as *const c_char);
        if IS_ERR((*dac).supply as *const c_void) {
            return dev_err_probe(
                dev,
                PTR_ERR((*dac).supply as *const c_void),
                b"failed to acquire supply 'VDD-supply'\n\0".as_ptr() as *const c_char,
            );
        }

        ret = regulator_enable((*dac).supply);
        if ret != 0 {
            dev_err(
                dev,
                b"failed to enable supply: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        voltage = regulator_get_voltage((*dac).supply);

        match voltage {
            1800000 => {
                reg = 0;
            }
            3300000 => {
                reg = PISTACHIO_INTERNAL_DAC_CTRL_PWR_SEL_MASK;
            }
            _ => {
                dev_err(
                    dev,
                    b"invalid voltage: %d\n\0".as_ptr() as *const c_char,
                    voltage,
                );
                ret = -EINVAL;
                regulator_disable((*dac).supply);
                return ret;
            }
        }

        regmap_update_bits(
            (*dac).regmap,
            PISTACHIO_INTERNAL_DAC_CTRL,
            PISTACHIO_INTERNAL_DAC_CTRL_PWR_SEL_MASK,
            reg,
        );

        pistachio_internal_dac_pwr_off(dac);
        pistachio_internal_dac_pwr_on(dac);

        pm_runtime_set_active(dev);
        pm_runtime_enable(dev);
        pm_runtime_idle(dev);

        ret = devm_snd_soc_register_component(
            dev,
            &pistachio_internal_dac_driver,
            core::ptr::addr_of_mut!(pistachio_internal_dac_dais) as *mut snd_soc_dai_driver,
            array_size(&pistachio_internal_dac_dais) as c_int,
        );
        if ret != 0 {
            dev_err(
                dev,
                b"failed to register component: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            pm_runtime_disable(&mut (*pdev).dev as *mut device);
            pistachio_internal_dac_pwr_off(dac);
            regulator_disable((*dac).supply);
            return ret;
        }

        0
    }
}

unsafe extern "C" fn pistachio_internal_dac_remove(pdev: *mut platform_device) {
    unsafe {
        let dac = dev_get_drvdata(&mut (*pdev).dev as *mut device) as *mut pistachio_internal_dac;

        pm_runtime_disable(&mut (*pdev).dev as *mut device);
        pistachio_internal_dac_pwr_off(dac);
        regulator_disable((*dac).supply);
    }
}

unsafe extern "C" fn pistachio_internal_dac_rt_resume(dev: *mut device) -> c_int {
    unsafe {
        let dac = dev_get_drvdata(dev) as *mut pistachio_internal_dac;
        let ret: c_int;

        ret = regulator_enable((*dac).supply);
        if ret != 0 {
            dev_err(
                dev,
                b"failed to enable supply: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        pistachio_internal_dac_pwr_on(dac);

        0
    }
}

unsafe extern "C" fn pistachio_internal_dac_rt_suspend(dev: *mut device) -> c_int {
    unsafe {
        let dac = dev_get_drvdata(dev) as *mut pistachio_internal_dac;

        pistachio_internal_dac_pwr_off(dac);

        regulator_disable((*dac).supply);

        0
    }
}

static pistachio_internal_dac_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(pistachio_internal_dac_rt_suspend),
    runtime_resume: Some(pistachio_internal_dac_rt_resume),
    runtime_idle: None,
};

static pistachio_internal_dac_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"img,pistachio-internal-dac\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, pistachio_internal_dac_of_match);

static pistachio_internal_dac_plat_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"img-pistachio-internal-dac\0".as_ptr() as *const c_char,
        of_match_table: pistachio_internal_dac_of_match.as_ptr(),
        pm: &pistachio_internal_dac_pm_ops,
    },
    probe: Some(pistachio_internal_dac_probe),
    remove: Some(pistachio_internal_dac_remove),
};
// module_platform_driver(pistachio_internal_dac_plat_driver);

// MODULE_DESCRIPTION("Pistachio Internal DAC driver");
// MODULE_AUTHOR("Damien Horsley <Damien.Horsley@imgtec.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
