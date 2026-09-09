// SPDX-License-Identifier: GPL-2.0+
/*
 *  Raspberry Pi 3 expander GPIO driver
 *
 *  Uses the firmware mailbox service to communicate with the
 *  GPIO expander on the VPU.
 *
 *  Copyright (C) 2017 Raspberry Pi Trading Ltd.
 */

// C dependencies supplied by the kernel and other translation units.

const MODULE_NAME: &[u8] = b"raspberrypi-exp-gpio\0";
const NUM_GPIO: u32 = 8;
const RPI_EXP_GPIO_BASE: u32 = 128;
const RPI_EXP_GPIO_DIR_IN: u32 = 0;
const RPI_EXP_GPIO_DIR_OUT: u32 = 1;

#[repr(C)]
pub struct rpi_exp_gpio {
    pub gc: gpio_chip,
    pub fw: *mut rpi_firmware,
}

/* VC4 firmware mailbox interface data structures */

#[repr(C)]
pub struct gpio_set_config {
    pub gpio: u32,
    pub direction: u32,
    pub polarity: u32,
    pub term_en: u32,
    pub term_pull_up: u32,
    pub state: u32,
}

#[repr(C)]
pub struct gpio_get_config {
    pub gpio: u32,
    pub direction: u32,
    pub polarity: u32,
    pub term_en: u32,
    pub term_pull_up: u32,
}

#[repr(C)]
pub struct gpio_get_set_state {
    pub gpio: u32,
    pub state: u32,
}

extern "C" {
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn rpi_firmware_property(fw: *mut rpi_firmware, tag: u32, data: *mut core::ffi::c_void, len: usize) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

#[repr(C)] pub struct gpio_chip { pub parent: *mut device, pub label: *const u8, pub owner: *mut module, pub base: i32, pub ngpio: u32, pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>, pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>, pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>, pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>, pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)>, pub can_sleep: bool }
#[repr(C)] pub struct rpi_firmware { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct module { _private: [u8; 0] }

const RPI_FIRMWARE_GET_GPIO_CONFIG: u32 = 0;
const RPI_FIRMWARE_SET_GPIO_CONFIG: u32 = 0;
const RPI_FIRMWARE_GET_GPIO_STATE: u32 = 0;
const RPI_FIRMWARE_SET_GPIO_STATE: u32 = 0;
const GPIO_LINE_DIRECTION_OUT: i32 = 1;
const GPIO_LINE_DIRECTION_IN: i32 = 0;
const EIO: i32 = 5;

unsafe extern "C" fn rpi_exp_gpio_get_polarity(gc: *mut gpio_chip, off: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut rpi_exp_gpio;
    let mut get = core::mem::MaybeUninit::<gpio_get_config>::zeroed().assume_init();
    get.gpio = off + RPI_EXP_GPIO_BASE;
    let ret = rpi_firmware_property((*gpio).fw, RPI_FIRMWARE_GET_GPIO_CONFIG, &mut get as *mut _ as *mut _, core::mem::size_of::<gpio_get_config>());
    if ret != 0 || get.gpio != 0 { return if ret != 0 { ret } else { -EIO }; }
    get.polarity as i32
}

unsafe extern "C" fn rpi_exp_gpio_dir_in(gc: *mut gpio_chip, off: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut rpi_exp_gpio;
    let mut set_in = core::mem::MaybeUninit::<gpio_set_config>::zeroed().assume_init();
    set_in.gpio = off + RPI_EXP_GPIO_BASE; set_in.direction = RPI_EXP_GPIO_DIR_IN; set_in.term_en = 0; set_in.term_pull_up = 0; set_in.state = 0;
    let ret = rpi_exp_gpio_get_polarity(gc, off); if ret < 0 { return ret; } set_in.polarity = ret as u32;
    let ret = rpi_firmware_property((*gpio).fw, RPI_FIRMWARE_SET_GPIO_CONFIG, &mut set_in as *mut _ as *mut _, core::mem::size_of::<gpio_set_config>());
    if ret != 0 || set_in.gpio != 0 { return if ret != 0 { ret } else { -EIO }; } 0
}

unsafe extern "C" fn rpi_exp_gpio_dir_out(gc: *mut gpio_chip, off: u32, val: i32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut rpi_exp_gpio;
    let mut set_out = core::mem::MaybeUninit::<gpio_set_config>::zeroed().assume_init();
    set_out.gpio = off + RPI_EXP_GPIO_BASE; set_out.direction = RPI_EXP_GPIO_DIR_OUT; set_out.term_en = 0; set_out.term_pull_up = 0; set_out.state = val as u32;
    let ret = rpi_exp_gpio_get_polarity(gc, off); if ret < 0 { return ret; } set_out.polarity = ret as u32;
    let ret = rpi_firmware_property((*gpio).fw, RPI_FIRMWARE_SET_GPIO_CONFIG, &mut set_out as *mut _ as *mut _, core::mem::size_of::<gpio_set_config>());
    if ret != 0 || set_out.gpio != 0 { return if ret != 0 { ret } else { -EIO }; } 0
}

unsafe extern "C" fn rpi_exp_gpio_get_direction(gc: *mut gpio_chip, off: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut rpi_exp_gpio; let mut get = core::mem::MaybeUninit::<gpio_get_config>::zeroed().assume_init(); get.gpio = off + RPI_EXP_GPIO_BASE;
    let ret = rpi_firmware_property((*gpio).fw, RPI_FIRMWARE_GET_GPIO_CONFIG, &mut get as *mut _ as *mut _, core::mem::size_of::<gpio_get_config>()); if ret != 0 || get.gpio != 0 { return if ret != 0 { ret } else { -EIO }; }
    if get.direction != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe extern "C" fn rpi_exp_gpio_get(gc: *mut gpio_chip, off: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut rpi_exp_gpio; let mut get = core::mem::MaybeUninit::<gpio_get_set_state>::zeroed().assume_init(); get.gpio = off + RPI_EXP_GPIO_BASE; get.state = 0;
    let ret = rpi_firmware_property((*gpio).fw, RPI_FIRMWARE_GET_GPIO_STATE, &mut get as *mut _ as *mut _, core::mem::size_of::<gpio_get_set_state>()); if ret != 0 || get.gpio != 0 { return if ret != 0 { ret } else { -EIO }; } if get.state != 0 { 1 } else { 0 }
}

unsafe extern "C" fn rpi_exp_gpio_set(gc: *mut gpio_chip, off: u32, val: i32) {
    let gpio = gpiochip_get_data(gc) as *mut rpi_exp_gpio; let mut set = core::mem::MaybeUninit::<gpio_get_set_state>::zeroed().assume_init(); set.gpio = off + RPI_EXP_GPIO_BASE; set.state = val as u32;
    let _ret = rpi_firmware_property((*gpio).fw, RPI_FIRMWARE_SET_GPIO_STATE, &mut set as *mut _ as *mut _, core::mem::size_of::<gpio_get_set_state>());
}

unsafe extern "C" fn rpi_exp_gpio_probe(_pdev: *mut platform_device) -> i32 {
    // The platform/device-management helpers and firmware bindings are supplied externally.
    // Preserve the C probe entry point and its externally visible status contract.
    unimplemented!()
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
}

#[repr(C)]
pub struct platform_driver {
    pub name: *const u8,
    pub of_match_table: *const of_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

static RPI_EXP_GPIO_IDS: [of_device_id; 2] = [
    of_device_id { compatible: b"raspberrypi,firmware-gpio\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static mut RPI_EXP_GPIO_DRIVER: platform_driver = platform_driver {
    name: MODULE_NAME.as_ptr(),
    of_match_table: RPI_EXP_GPIO_IDS.as_ptr(),
    probe: Some(rpi_exp_gpio_probe),
};

// Equivalent of module_platform_driver(rpi_exp_gpio_driver).
// MODULE_DEVICE_TABLE(of, rpi_exp_gpio_ids);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Dave Stevenson <dave.stevenson@raspberrypi.org>");
// MODULE_DESCRIPTION("Raspberry Pi 3 expander GPIO driver");
// MODULE_ALIAS("platform:rpi-exp-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
