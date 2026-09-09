// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2021~2022 NXP
 *
 * The driver exports a standard gpiochip interface
 * to control the PIN resources on SCU domain.
 */

// Linux kernel dependencies supplied by other translation units/headers.

#[repr(C)]
pub struct gpio_chip {
    pub base: i32,
    pub parent: *mut device,
    pub ngpio: usize,
    pub label: *const core::ffi::c_char,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)>,
    pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct imx_sc_ipc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

extern "C" {
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn imx_sc_misc_get_control(handle: *mut imx_sc_ipc, resource: u32,
                                control: u32, level: *mut i32) -> i32;
    fn imx_sc_misc_set_control(handle: *mut imx_sc_ipc, resource: u32,
                                control: u32, value: i32) -> i32;
    fn imx_scu_get_handle(handle: *mut *mut imx_sc_ipc) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_mutex_init(dev: *mut device, lock: *mut mutex) -> i32;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn devm_gpiochip_add_data(dev: *mut device, chip: *mut gpio_chip,
                              data: *mut core::ffi::c_void) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn dev_name(dev: *mut device) -> *const core::ffi::c_char;
    fn dev_err(dev: *mut device, format: *const core::ffi::c_char, ...);
}

const GPIO_LINE_DIRECTION_OUT: i32 = 0;
const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;

// Resource constants supplied by dt-bindings/firmware/imx/rsrc.h.
extern "C" {
    static IMX_SC_R_BOARD_R0: u32;
    static IMX_SC_R_BOARD_R1: u32;
    static IMX_SC_R_BOARD_R2: u32;
    static IMX_SC_R_BOARD_R3: u32;
    static IMX_SC_R_BOARD_R4: u32;
    static IMX_SC_R_BOARD_R5: u32;
    static IMX_SC_R_BOARD_R6: u32;
    static IMX_SC_R_BOARD_R7: u32;
}

#[repr(C)]
pub struct scu_gpio_priv {
    pub chip: gpio_chip,
    pub lock: mutex,
    pub dev: *mut device,
    pub handle: *mut imx_sc_ipc,
}

static mut scu_rsrc_arr: [u32; 8] = [
    0, 0, 0, 0, 0, 0, 0, 0,
];

unsafe extern "C" fn imx_scu_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let priv_ = gpiochip_get_data(chip) as *mut scu_gpio_priv;
    let mut level: i32 = 0;
    let err: i32;

    // scoped_guard(mutex, &priv->lock)
    err = imx_sc_misc_get_control((*priv_).handle, scu_rsrc_arr[offset as usize], 0, &mut level);
    if err != 0 {
        dev_err((*priv_).dev, b"SCU get failed: %d\0".as_ptr() as *const _, err);
        return err;
    }
    level
}

unsafe extern "C" fn imx_scu_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) {
    let priv_ = gpiochip_get_data(chip) as *mut scu_gpio_priv;
    let err: i32;

    // scoped_guard(mutex, &priv->lock)
    err = imx_sc_misc_set_control((*priv_).handle, scu_rsrc_arr[offset as usize], 0, value);
    if err != 0 {
        dev_err((*priv_).dev, b"SCU set (%d) failed: %d\n\0".as_ptr() as *const _,
                scu_rsrc_arr[offset as usize], err);
    }
}

unsafe extern "C" fn imx_scu_gpio_get_direction(_chip: *mut gpio_chip, _offset: u32) -> i32 {
    GPIO_LINE_DIRECTION_OUT
}

unsafe extern "C" fn imx_scu_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<scu_gpio_priv>(), GFP_KERNEL)
        as *mut scu_gpio_priv;
    if priv_.is_null() { return -ENOMEM; }

    let ret = imx_scu_get_handle(&mut (*priv_).handle);
    if ret != 0 { return ret; }
    (*priv_).dev = dev;
    let ret = devm_mutex_init(dev, &mut (*priv_).lock);
    if ret != 0 { return ret; }

    let gc = &mut (*priv_).chip;
    gc.base = -1;
    gc.parent = dev;
    gc.ngpio = 8;
    gc.label = dev_name(dev);
    gc.get = Some(imx_scu_gpio_get);
    gc.set = Some(imx_scu_gpio_set);
    gc.get_direction = Some(imx_scu_gpio_get_direction);
    platform_set_drvdata(pdev, priv_ as *mut core::ffi::c_void);
    devm_gpiochip_add_data(dev, gc, priv_ as *mut core::ffi::c_void)
}

static mut imx_scu_gpio_dt_ids: [of_device_id; 2] = [
    of_device_id { compatible: b"fsl,imx8qxp-sc-gpio\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

static mut imx_scu_gpio_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: b"gpio-imx-scu\0".as_ptr() as *const _,
        of_match_table: unsafe { imx_scu_gpio_dt_ids.as_ptr() },
    },
    probe: Some(imx_scu_gpio_probe),
};

unsafe extern "C" fn _imx_scu_gpio_init() -> i32 {
    platform_driver_register(&mut imx_scu_gpio_driver)
}

// subsys_initcall_sync(_imx_scu_gpio_init);
// MODULE_AUTHOR("Shenwei Wang <shenwei.wang@nxp.com>");
// MODULE_DESCRIPTION("NXP GPIO over IMX SCU API");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
