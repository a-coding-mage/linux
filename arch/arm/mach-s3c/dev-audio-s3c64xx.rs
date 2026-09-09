// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2009 Wolfson Microelectronics
//      Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// Dependencies supplied by the Linux platform, GPIO, resource, and ASoC
// headers are intentionally referenced here rather than reimplemented.

use core::ffi::c_void;

extern "C" {
    fn s3c_gpio_cfgpin(pin: u32, function: u32);
    fn s3c_gpio_cfgpin_range(start: u32, nr: u32, function: u32);
    fn printk(format: *const u8, ...);
}

#[repr(C)]
pub struct Resource {
    pub start: usize,
    pub end: usize,
    pub flags: u64,
}

#[repr(C)]
pub struct Device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct PlatformDevice {
    pub name: *const u8,
    pub id: i32,
    pub num_resources: usize,
    pub resource: *mut Resource,
    pub dev: Device,
}

#[repr(C)]
pub struct S3cAudioPdata {
    pub cfg_gpio: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
}

unsafe extern "C" fn s3c64xx_i2s_cfg_gpio(pdev: *mut PlatformDevice) -> i32 {
    let mut base: u32;

    match (*pdev).id {
        0 => {
            base = S3C64XX_GPD(0);
        }
        1 => {
            base = S3C64XX_GPE(0);
        }
        2 => {
            s3c_gpio_cfgpin(S3C64XX_GPC(4), S3C_GPIO_SFN(5));
            s3c_gpio_cfgpin(S3C64XX_GPC(5), S3C_GPIO_SFN(5));
            s3c_gpio_cfgpin(S3C64XX_GPC(7), S3C_GPIO_SFN(5));
            s3c_gpio_cfgpin_range(S3C64XX_GPH(6), 4, S3C_GPIO_SFN(5));
            return 0;
        }
        _ => {
            printk(b"Invalid I2S Controller number: %d\n\0".as_ptr(), (*pdev).id);
            return -22;
        }
    }

    s3c_gpio_cfgpin_range(base, 5, S3C_GPIO_SFN(3));

    0
}

static mut s3c64xx_iis0_resource: [Resource; 1] = [
    DEFINE_RES_MEM(S3C64XX_PA_IIS0, SZ_256),
];

static mut i2s0_pdata: S3cAudioPdata = S3cAudioPdata {
    cfg_gpio: Some(s3c64xx_i2s_cfg_gpio),
};

#[no_mangle]
pub static mut s3c64xx_device_iis0: PlatformDevice = PlatformDevice {
    name: b"samsung-i2s\0".as_ptr(),
    id: 0,
    num_resources: ARRAY_SIZE(&s3c64xx_iis0_resource),
    resource: s3c64xx_iis0_resource.as_mut_ptr(),
    dev: Device {
        platform_data: &mut i2s0_pdata as *mut S3cAudioPdata as *mut c_void,
    },
};

static mut s3c64xx_iis1_resource: [Resource; 1] = [
    DEFINE_RES_MEM(S3C64XX_PA_IIS1, SZ_256),
];

static mut i2s1_pdata: S3cAudioPdata = S3cAudioPdata {
    cfg_gpio: Some(s3c64xx_i2s_cfg_gpio),
};

#[no_mangle]
pub static mut s3c64xx_device_iis1: PlatformDevice = PlatformDevice {
    name: b"samsung-i2s\0".as_ptr(),
    id: 1,
    num_resources: ARRAY_SIZE(&s3c64xx_iis1_resource),
    resource: s3c64xx_iis1_resource.as_mut_ptr(),
    dev: Device {
        platform_data: &mut i2s1_pdata as *mut S3cAudioPdata as *mut c_void,
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
