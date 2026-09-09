// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO driver for the TS-4800 board
 *
 * Copyright (c) 2016 - Savoir-faire Linux
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const INPUT_REG_OFFSET: usize = 0x00;
const OUTPUT_REG_OFFSET: usize = 0x02;
const DIRECTION_REG_OFFSET: usize = 0x04;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct gpio_generic_chip_config {
    pub dev: *mut device,
    pub sz: usize,
    pub dat: *mut core::ffi::c_void,
    pub set: *mut core::ffi::c_void,
    pub dirout: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct gpio_chip {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_generic_chip {
    pub gc: gpio_chip,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
    ) -> *mut core::ffi::c_void;
    fn is_err(ptr: *const core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *const core::ffi::c_void) -> i32;
    fn gpio_generic_chip_init(
        chip: *mut gpio_generic_chip,
        config: *mut gpio_generic_chip_config,
    ) -> i32;
    fn dev_err_probe(
        dev: *mut device,
        err: i32,
        fmt: *const core::ffi::c_char,
    ) -> i32;
    fn devm_gpiochip_add_data(
        dev: *mut device,
        gc: *mut gpio_chip,
        data: *mut core::ffi::c_void,
    ) -> i32;
}

const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;

unsafe extern "C" fn ts4800_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut config: gpio_generic_chip_config;
    let dev: *mut device = &mut (*pdev).dev;
    let chip: *mut gpio_generic_chip;
    let base_addr: *mut core::ffi::c_void;
    let retval: i32;

    chip = devm_kzalloc(dev, core::mem::size_of::<gpio_generic_chip>(), GFP_KERNEL)
        as *mut gpio_generic_chip;
    if chip.is_null() {
        return -ENOMEM;
    }

    base_addr = devm_platform_ioremap_resource(pdev, 0);
    if is_err(base_addr) {
        return ptr_err(base_addr);
    }

    config = gpio_generic_chip_config {
        dev,
        sz: 2,
        dat: base_addr.add(INPUT_REG_OFFSET),
        set: base_addr.add(OUTPUT_REG_OFFSET),
        dirout: base_addr.add(DIRECTION_REG_OFFSET),
    };

    retval = gpio_generic_chip_init(chip, &mut config);
    if retval != 0 {
        return dev_err_probe(
            dev,
            retval,
            b"failed to initialize the generic GPIO chip\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    devm_gpiochip_add_data(dev, &mut (*chip).gc, core::ptr::null_mut())
}

static TS4800_GPIO_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: b"technologic,ts4800-gpio\0".as_ptr() as *const core::ffi::c_char,
    },
    of_device_id { compatible: core::ptr::null() },
];

static mut TS4800_GPIO_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: b"ts4800-gpio\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: TS4800_GPIO_OF_MATCH.as_ptr(),
    },
    probe: Some(ts4800_gpio_probe),
};

// Equivalent of module_platform_driver_probe(ts4800_gpio_driver, ts4800_gpio_probe).

// MODULE_AUTHOR("Julien Grossholtz <julien.grossholtz@savoirfairelinux.com>");
// MODULE_DESCRIPTION("TS4800 FPGA GPIO driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
