// SPDX-License-Identifier: GPL-2.0-only
/*
 * SPEAr platform SPI chipselect abstraction over gpiolib
 *
 * Copyright (C) 2012 ST Microelectronics
 * Shiraz Hashim <shiraz.linux.kernel@gmail.com>
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* maximum chipselects */
const NUM_OF_GPIO: c_uint = 4;

/*
 * Provision is available on some SPEAr SoCs to control ARM PL022 spi cs
 * through system registers. This register lies outside spi (pl022)
 * address space into system registers.
 *
 * It provides control for spi chip select lines so that any chipselect
 * (out of 4 possible chipselects in pl022) can be made low to select the
 * particular slave.
 */

#[repr(C)]
pub struct gpio_chip {
    pub ngpio: c_uint,
    pub base: c_int,
    pub request: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint)>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, c_uint, c_int)>,
    pub label: *const c_char,
    pub parent: *mut device,
    pub owner: *mut c_void,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct spear_spics {
    pub base: *mut c_void,
    pub perip_cfg: u32,
    pub sw_enable_bit: u32,
    pub cs_value_bit: u32,
    pub cs_enable_mask: u32,
    pub cs_enable_shift: u32,
    pub use_count: c_ulong,
    pub last_off: c_int,
    pub chip: gpio_chip,
}

extern "C" {
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut c_void;
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_property_read_u32(np: *mut device_node, name: *const c_char, value: *mut u32) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_gpiochip_add_data(dev: *mut device, chip: *mut gpio_chip, data: *mut c_void) -> c_int;
    fn dev_err(dev: *mut device, format: *const c_char, ...);
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
}

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: driver,
}

unsafe fn spics_set_value(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    let spics = gpiochip_get_data(chip) as *mut spear_spics;
    let mut tmp = readl_relaxed((*spics).base.add((*spics).perip_cfg as usize));
    if (*spics).last_off != offset as c_int {
        (*spics).last_off = offset as c_int;
        tmp &= !((*spics).cs_enable_mask << (*spics).cs_enable_shift);
        tmp |= offset << (*spics).cs_enable_shift;
    }
    tmp &= !(0x1 << (*spics).cs_value_bit);
    tmp |= (value as u32) << (*spics).cs_value_bit;
    writel_relaxed(tmp, (*spics).base.add((*spics).perip_cfg as usize));
    0
}

unsafe fn spics_direction_output(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    spics_set_value(chip, offset, value)
}

unsafe fn spics_request(chip: *mut gpio_chip, _offset: c_uint) -> c_int {
    let spics = gpiochip_get_data(chip) as *mut spear_spics;
    let was_zero = (*spics).use_count == 0;
    (*spics).use_count = (*spics).use_count.wrapping_add(1);
    if was_zero {
        let mut tmp = readl_relaxed((*spics).base.add((*spics).perip_cfg as usize));
        tmp |= 0x1 << (*spics).sw_enable_bit;
        tmp |= 0x1 << (*spics).cs_value_bit;
        writel_relaxed(tmp, (*spics).base.add((*spics).perip_cfg as usize));
    }
    0
}

unsafe fn spics_free(chip: *mut gpio_chip, _offset: c_uint) {
    let spics = gpiochip_get_data(chip) as *mut spear_spics;
    (*spics).use_count = (*spics).use_count.wrapping_sub(1);
    if (*spics).use_count == 0 {
        let mut tmp = readl_relaxed((*spics).base.add((*spics).perip_cfg as usize));
        tmp &= !(0x1 << (*spics).sw_enable_bit);
        writel_relaxed(tmp, (*spics).base.add((*spics).perip_cfg as usize));
    }
}

unsafe fn spics_gpio_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let spics = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<spear_spics>(), GFP_KERNEL)
        as *mut spear_spics;
    if spics.is_null() { return -ENOMEM; }
    (*spics).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*spics).base) { return PTR_ERR((*spics).base); }
    let props = [
        (c"st-spics,peripcfg-reg", &mut (*spics).perip_cfg),
        (c"st-spics,sw-enable-bit", &mut (*spics).sw_enable_bit),
        (c"st-spics,cs-value-bit", &mut (*spics).cs_value_bit),
        (c"st-spics,cs-enable-mask", &mut (*spics).cs_enable_mask),
        (c"st-spics,cs-enable-shift", &mut (*spics).cs_enable_shift),
    ];
    for (name, value) in props {
        if of_property_read_u32(np, name.as_ptr(), value) != 0 {
            dev_err(&mut (*pdev).dev, c"DT probe failed\n".as_ptr());
            return -EINVAL;
        }
    }
    (*spics).chip.ngpio = NUM_OF_GPIO;
    (*spics).chip.base = -1;
    (*spics).chip.request = Some(spics_request);
    (*spics).chip.free = Some(spics_free);
    (*spics).chip.direction_output = Some(spics_direction_output);
    (*spics).chip.set = Some(spics_set_value);
    (*spics).chip.label = dev_name(&mut (*pdev).dev);
    (*spics).chip.parent = &mut (*pdev).dev;
    (*spics).last_off = -1;
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*spics).chip, spics as *mut c_void)
}

static SPICS_GPIO_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: c"st,spear-spics-gpio".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static mut SPICS_GPIO_DRIVER: platform_driver = platform_driver {
    probe: Some(spics_gpio_probe),
    driver: driver {
        name: c"spear-spics-gpio".as_ptr(),
        of_match_table: SPICS_GPIO_OF_MATCH.as_ptr(),
    },
};

unsafe fn spics_gpio_init() -> c_int {
    platform_driver_register(&mut SPICS_GPIO_DRIVER)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
