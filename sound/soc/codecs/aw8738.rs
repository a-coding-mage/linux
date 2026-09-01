// SPDX-License-Identifier: GPL-2.0-only

// C dependencies: <linux/gpio/consumer.h>, <linux/module.h>,
// <linux/regulator/consumer.h>, <sound/soc.h>

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct gpio_desc {
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
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

#[repr(C)]
pub struct aw8738_priv {
    pub gpiod_mode: *mut gpio_desc,
    pub mode: c_uint,
}

const SND_SOC_DAPM_POST_PMU: c_int = 1 << 0;
const SND_SOC_DAPM_PRE_PMD: c_int = 1 << 1;
const SND_SOC_NOPM: c_int = 0;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_int = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

unsafe extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn udelay(usecs: c_uint);
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_uint, max: c_uint);
    fn WARN(condition: c_int, format: *const c_char) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const c_char) -> c_int;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *const c_void,
        num_dai: c_int,
    ) -> c_int;
    fn of_match_ptr(matches: *const of_device_id) -> *const of_device_id;
}

unsafe extern "C" fn aw8738_drv_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let c = snd_soc_dapm_to_component((*w).dapm);
    let aw = snd_soc_component_get_drvdata(c) as *mut aw8738_priv;
    let mut i: c_int;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            i = 0;
            while i < (*aw).mode as c_int {
                gpiod_set_value_cansleep((*aw).gpiod_mode, 0);
                udelay(2);
                gpiod_set_value_cansleep((*aw).gpiod_mode, 1);
                udelay(2);
                i += 1;
            }
            msleep(40);
        }
        SND_SOC_DAPM_PRE_PMD => {
            gpiod_set_value_cansleep((*aw).gpiod_mode, 0);
            usleep_range(1000, 2000);
        }
        _ => {
            WARN(1, c"Unexpected event".as_ptr());
            return -EINVAL;
        }
    }

    0
}

// The original C uses SND_SOC_DAPM_INPUT, SND_SOC_DAPM_OUT_DRV_E, and
// SND_SOC_DAPM_OUTPUT initializer macros. Their file-local semantic content is
// preserved here as static widget descriptors, with the event callback noted.
static AW8738_DAPM_WIDGETS: [snd_soc_dapm_widget_desc; 3] = [
    snd_soc_dapm_widget_desc {
        name: c"IN".as_ptr(),
    },
    snd_soc_dapm_widget_desc {
        name: c"DRV".as_ptr(),
    },
    snd_soc_dapm_widget_desc {
        name: c"OUT".as_ptr(),
    },
];

const AW8738_DRV_EVENT: Option<
    unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int,
> = Some(aw8738_drv_event);
const AW8738_DRV_EVENT_MASK: c_int = SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD;
const AW8738_DRV_EVENT_REG: c_int = SND_SOC_NOPM;
const AW8738_DRV_EVENT_SHIFT: c_int = 0;
const AW8738_DRV_EVENT_INVERT: c_int = 0;

static AW8738_DAPM_ROUTES: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c"DRV".as_ptr(),
        control: ptr::null(),
        source: c"IN".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OUT".as_ptr(),
        control: ptr::null(),
        source: c"DRV".as_ptr(),
    },
];

static AW8738_COMPONENT_DRIVER: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: AW8738_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: AW8738_DAPM_WIDGETS.len() as c_uint,
    dapm_routes: AW8738_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: AW8738_DAPM_ROUTES.len() as c_uint,
};

unsafe extern "C" fn aw8738_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let aw: *mut aw8738_priv;
    let ret: c_int;

    aw = devm_kzalloc(dev, core::mem::size_of::<aw8738_priv>(), GFP_KERNEL) as *mut aw8738_priv;
    if aw.is_null() {
        return -ENOMEM;
    }
    platform_set_drvdata(pdev, aw as *mut c_void);

    (*aw).gpiod_mode = devm_gpiod_get(dev, c"mode".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*aw).gpiod_mode as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*aw).gpiod_mode as *const c_void),
            c"Failed to get 'mode' gpio".as_ptr(),
        );
    }

    ret = device_property_read_u32(dev, c"awinic,mode".as_ptr(), &mut (*aw).mode);
    if ret != 0 {
        return -EINVAL;
    }

    devm_snd_soc_register_component(
        &mut (*pdev).dev as *mut device,
        &AW8738_COMPONENT_DRIVER,
        ptr::null(),
        0,
    )
}

// Original C condition: #ifdef CONFIG_OF
static AW8738_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: c"awinic,aw8738".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, aw8738_of_match);

static AW8738_DRIVER: platform_driver = platform_driver {
    probe: Some(aw8738_probe),
    driver: device_driver {
        name: c"aw8738".as_ptr(),
        of_match_table: unsafe { of_match_ptr(AW8738_OF_MATCH.as_ptr()) },
    },
};

// module_platform_driver(aw8738_driver);
// MODULE_DESCRIPTION("Awinic AW8738 Amplifier Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
