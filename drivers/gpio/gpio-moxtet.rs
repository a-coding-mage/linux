// SPDX-License-Identifier: GPL-2.0
/*
 *  Turris Mox Moxtet GPIO expander
 *
 *  Copyright (C) 2018 Marek Behún <kabel@kernel.org>
 */

// Linux kernel dependencies supplied by the surrounding translation.

const MOXTET_GPIO_NGPIOS: u32 = 12;
const MOXTET_GPIO_INPUTS: u32 = 4;

#[repr(C)]
struct moxtet_gpio_desc {
    in_mask: u16,
    out_mask: u16,
}

const TURRIS_MOX_MODULE_SFP: usize = 0;

static descs: [moxtet_gpio_desc; 1] = [
    moxtet_gpio_desc {
        in_mask: (1u16 << 3) - 1,
        out_mask: ((1u16 << 6) - 1) & !((1u16 << 4) - 1),
    },
];

#[repr(C)]
struct moxtet_gpio_chip {
    dev: *mut device,
    gpio_chip: gpio_chip,
    desc: *const moxtet_gpio_desc,
}

// These types and functions are provided by other kernel translation units.
#[repr(C)]
struct device;
#[repr(C)]
struct device_node;
#[repr(C)]
struct gpio_chip {
    parent: *mut device,
    label: *const core::ffi::c_char,
    get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    base: i32,
    ngpio: u32,
    can_sleep: bool,
    owner: *mut core::ffi::c_void,
}

#[repr(C)]
struct moxtet_device { id: i32 }
#[repr(C)]
struct moxtet_driver {
    driver: driver,
    id_table: *const i32,
}
#[repr(C)]
struct driver {
    name: *const core::ffi::c_char,
    of_match_table: *const of_device_id,
    probe: Option<unsafe extern "C" fn(*mut device) -> i32>,
}
#[repr(C)]
struct of_device_id { compatible: *const core::ffi::c_char }

extern "C" {
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut moxtet_gpio_chip;
    fn moxtet_device_read(dev: *mut device) -> i32;
    fn moxtet_device_written(dev: *mut device) -> i32;
    fn moxtet_device_write(dev: *mut device, value: i32) -> i32;
    fn to_moxtet_device(dev: *mut device) -> *mut moxtet_device;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn dev_name(dev: *mut device) -> *const core::ffi::c_char;
    fn devm_gpiochip_add_data(dev: *mut device, gc: *mut gpio_chip, data: *mut core::ffi::c_void) -> i32;
}

const GPIO_LINE_DIRECTION_IN: i32 = 1;
const GPIO_LINE_DIRECTION_OUT: i32 = 0;
const ENOTSUPP: i32 = 524;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;

unsafe extern "C" fn moxtet_gpio_get_value(gc: *mut gpio_chip, offset: u32) -> i32 {
    let chip = &*gpiochip_get_data(gc);
    let ret: i32;
    if (*chip.desc).in_mask & (1u16 << offset) != 0 {
        ret = moxtet_device_read(chip.dev);
    } else if (*chip.desc).out_mask & (1u16 << offset) != 0 {
        ret = moxtet_device_written(chip.dev);
        if ret >= 0 { ret <<= MOXTET_GPIO_INPUTS; }
    } else { return -EINVAL; }
    if ret < 0 { return ret; }
    ((ret & (1 << offset)) != 0) as i32
}

unsafe extern "C" fn moxtet_gpio_set_value(gc: *mut gpio_chip, mut offset: u32, val: i32) -> i32 {
    let chip = &*gpiochip_get_data(gc);
    let mut state = moxtet_device_written(chip.dev);
    if state < 0 { return state; }
    offset -= MOXTET_GPIO_INPUTS;
    if val != 0 { state |= 1 << offset; } else { state &= !(1 << offset); }
    moxtet_device_write(chip.dev, state)
}

unsafe extern "C" fn moxtet_gpio_get_direction(gc: *mut gpio_chip, offset: u32) -> i32 {
    let chip = &*gpiochip_get_data(gc);
    if (*chip.desc).in_mask & (1u16 << offset) != 0 { GPIO_LINE_DIRECTION_IN }
    else if (*chip.desc).out_mask & (1u16 << offset) != 0 { GPIO_LINE_DIRECTION_OUT }
    else { -EINVAL }
}

unsafe extern "C" fn moxtet_gpio_direction_input(gc: *mut gpio_chip, offset: u32) -> i32 {
    let chip = &*gpiochip_get_data(gc);
    if (*chip.desc).in_mask & (1u16 << offset) != 0 { 0 }
    else if (*chip.desc).out_mask & (1u16 << offset) != 0 { -ENOTSUPP }
    else { -EINVAL }
}

unsafe extern "C" fn moxtet_gpio_direction_output(gc: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    let chip = &*gpiochip_get_data(gc);
    if (*chip.desc).out_mask & (1u16 << offset) != 0 {
        moxtet_gpio_set_value(gc, offset, val)
    } else if (*chip.desc).in_mask & (1u16 << offset) != 0 { -ENOTSUPP }
    else { -EINVAL }
}

unsafe extern "C" fn moxtet_gpio_probe(dev: *mut device) -> i32 {
    let id = (*to_moxtet_device(dev)).id;
    if id >= descs.len() as i32 { return -ENOTSUPP; }
    let chip = devm_kzalloc(dev, core::mem::size_of::<moxtet_gpio_chip>(), 0) as *mut moxtet_gpio_chip;
    if chip.is_null() { return -ENOMEM; }
    (*chip).dev = dev;
    (*chip).gpio_chip.parent = dev;
    (*chip).desc = &descs[id as usize];
    dev_set_drvdata(dev, chip as *mut core::ffi::c_void);
    (*chip).gpio_chip.label = dev_name(dev);
    (*chip).gpio_chip.get_direction = Some(moxtet_gpio_get_direction);
    (*chip).gpio_chip.direction_input = Some(moxtet_gpio_direction_input);
    (*chip).gpio_chip.direction_output = Some(moxtet_gpio_direction_output);
    (*chip).gpio_chip.get = Some(moxtet_gpio_get_value);
    (*chip).gpio_chip.set = Some(moxtet_gpio_set_value);
    (*chip).gpio_chip.base = -1;
    (*chip).gpio_chip.ngpio = MOXTET_GPIO_NGPIOS;
    (*chip).gpio_chip.can_sleep = true;
    devm_gpiochip_add_data(dev, &mut (*chip).gpio_chip, chip as *mut core::ffi::c_void)
}

static moxtet_gpio_dt_ids: [of_device_id; 2] = [
    of_device_id { compatible: b"cznic,moxtet-gpio\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

static moxtet_gpio_module_table: [i32; 2] = [TURRIS_MOX_MODULE_SFP as i32, 0];

static mut moxtet_gpio_driver: moxtet_driver = moxtet_driver {
    driver: driver {
        name: b"moxtet-gpio\0".as_ptr() as *const _,
        of_match_table: moxtet_gpio_dt_ids.as_ptr(),
        probe: Some(moxtet_gpio_probe),
    },
    id_table: moxtet_gpio_module_table.as_ptr(),
};

// module_moxtet_driver(moxtet_gpio_driver);
// MODULE_AUTHOR("Marek Behun <kabel@kernel.org>");
// MODULE_DESCRIPTION("Turris Mox Moxtet GPIO expander");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
