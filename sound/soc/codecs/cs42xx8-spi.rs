// SPDX-License-Identifier: GPL-2.0
/*
 * Cirrus Logic CS42448/CS42888 Audio CODEC DAI SPI driver
 *
 * Copyright 2026 NXP
 *
 */

// C dependencies: linux/module.h, linux/pm_runtime.h, linux/regmap.h,
// linux/spi/spi.h, sound/soc.h, and "cs42xx8.h".

/*
 * CS42448/CS42888 SPI register access (from datasheet Figure 23):
 *
 * The SPI frame is 3 bytes:
 *   Byte 0: chip address [7:1] = 1001111, bit[0] = R/W (0=write, 1=read)
 *           Write: 0x9E,  Read: 0x9F
 *   Byte 1: MAP - Memory Address Pointer
 *           bit[7] = INCR (auto-increment for burst), bits[6:0] = address
 *   Byte 2: data byte
 *
 * We configure reg_bits=16 so that regmap treats the address field as 2 bytes
 * (big-endian). The chip address byte (0x9E/0x9F) is placed in the high byte
 * via write_flag_mask / read_flag_mask, and the MAP register address occupies
 * the low byte. Currently INCR (MAP bit[7]) is not set and use_single_read/write
 * are enabled. This produces the correct 3-byte on-wire frame without any
 * custom bus implementation:
 *
 *   write: [0x9E, MAP_addr, data]
 *   read:  [0x9F, MAP_addr] -> [data]
 */

type c_int = i32;
type c_char = i8;
type c_void = core::ffi::c_void;
type kernel_ulong_t = core::ffi::c_ulong;

const EINVAL: c_int = 22;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
pub struct cs42xx8_driver_data {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub write_flag_mask: u32,
    pub read_flag_mask: u32,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_device_id {
    pub name: [c_char; 32],
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut spi_device)>,
    pub id_table: *const spi_device_id,
}

unsafe extern "C" {
    static cs42xx8_regmap_config: regmap_config;
    static cs42448_data: cs42xx8_driver_data;
    static cs42888_data: cs42xx8_driver_data;
    static cs42xx8_pm: dev_pm_ops;

    fn spi_get_device_match_data(spi: *mut spi_device) -> *const c_void;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn cs42xx8_probe(
        dev: *mut device,
        regmap: *mut regmap,
        drvdata: *const cs42xx8_driver_data,
    ) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_request_idle(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn module_spi_driver(driver: *mut spi_driver);
}

const fn pm_ptr(pm: *const dev_pm_ops) -> *const dev_pm_ops {
    pm
}

unsafe extern "C" fn cs42xx8_spi_probe(spi: *mut spi_device) -> c_int {
    let drvdata: *const cs42xx8_driver_data;
    let mut config: regmap_config;
    let ret: c_int;

    drvdata = spi_get_device_match_data(spi) as *const cs42xx8_driver_data;
    if drvdata.is_null() {
        return dev_err_probe(
            &mut (*spi).dev,
            -EINVAL,
            c"failed to find driver data\n".as_ptr(),
        );
    }

    config = cs42xx8_regmap_config;
    /*
     * reg_bits=16 makes regmap send a 2-byte address field (big-endian).
     * write_flag_mask/read_flag_mask are OR'd into that address field:
     */
    config.reg_bits = 16;
    config.write_flag_mask = 0x9E;
    config.read_flag_mask = 0x9F;

    ret = cs42xx8_probe(
        &mut (*spi).dev,
        devm_regmap_init_spi(spi, &config),
        drvdata,
    );
    if ret != 0 {
        return ret;
    }

    pm_runtime_enable(&mut (*spi).dev);
    pm_request_idle(&mut (*spi).dev);

    0
}

unsafe extern "C" fn cs42xx8_spi_remove(spi: *mut spi_device) {
    pm_runtime_disable(&mut (*spi).dev);
}

static cs42xx8_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: c"cirrus,cs42448".as_ptr(),
        data: unsafe { &cs42448_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: c"cirrus,cs42888".as_ptr(),
        data: unsafe { &cs42888_data as *const _ as *const c_void },
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, cs42xx8_of_match);

static cs42xx8_spi_id: [spi_device_id; 3] = [
    spi_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'4' as c_char,
            b'2' as c_char,
            b'4' as c_char,
            b'4' as c_char,
            b'8' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        driver_data: unsafe { &cs42448_data as *const _ as kernel_ulong_t },
    },
    spi_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'4' as c_char,
            b'2' as c_char,
            b'8' as c_char,
            b'8' as c_char,
            b'8' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        driver_data: unsafe { &cs42888_data as *const _ as kernel_ulong_t },
    },
    spi_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(spi, cs42xx8_spi_id);

static mut cs42xx8_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c"cs42xx8".as_ptr(),
        pm: pm_ptr(unsafe { &cs42xx8_pm }),
        of_match_table: cs42xx8_of_match.as_ptr(),
    },
    probe: Some(cs42xx8_spi_probe),
    remove: Some(cs42xx8_spi_remove),
    id_table: cs42xx8_spi_id.as_ptr(),
};

unsafe fn __register_cs42xx8_spi_driver() {
    module_spi_driver(&raw mut cs42xx8_spi_driver);
}

// MODULE_DESCRIPTION("Cirrus Logic CS42448/CS42888 ALSA SoC Codec SPI Driver");
// MODULE_AUTHOR("Chancel Liu <chancel.liu@nxp.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
