// SPDX-License-Identifier: GPL-2.0-only OR MIT
/*
 * Apple SMC GPIO driver
 * Copyright The Asahi Linux Contributors
 *
 * This driver implements basic SMC PMU GPIO support that can read inputs
 * and write outputs. Mode changes and IRQ config are not yet implemented.
 */

// C dependencies supplied by the surrounding kernel/Rust bindings.

const MAX_GPIO: usize = 64;

const CMD_ACTION: u32 = 0 << 24;
const CMD_OUTPUT: u32 = 1 << 24;
const CMD_INPUT: u32 = 2 << 24;
const CMD_PINMODE: u32 = 3 << 24;
const CMD_IRQ_ENABLE: u32 = 4 << 24;
const CMD_IRQ_ACK: u32 = 5 << 24;
const CMD_IRQ_MODE: u32 = 6 << 24;
const CMD_CONFIG: u32 = 0xff << 24;

const MODE_INPUT: u32 = 0;
const MODE_OUTPUT: u32 = 1;
const MODE_VALUE_0: u32 = 0;
const MODE_VALUE_1: u32 = 2;

const IRQ_MODE_HIGH: u32 = 0;
const IRQ_MODE_LOW: u32 = 1;
const IRQ_MODE_RISING: u32 = 2;
const IRQ_MODE_FALLING: u32 = 3;
const IRQ_MODE_BOTH: u32 = 4;

const CONFIG_MASK: u32 = 0x00ff0000;
const CONFIG_VAL: u32 = 0x000000ff;
const CONFIG_OUTMODE: u32 = 0x000000c0;
const CONFIG_IRQMODE: u32 = 0x00000038;
const CONFIG_PULLDOWN: u32 = 1 << 2;
const CONFIG_PULLUP: u32 = 1 << 1;
const CONFIG_OUTVAL: u32 = 1 << 0;

#[repr(C)]
struct MacsmcGpio {
    dev: *mut Device,
    smc: *mut AppleSmc,
    gc: GpioChip,
    first_index: i32,
}

type SmcKey = u32;

#[repr(C)]
struct Device;
#[repr(C)]
struct PlatformDevice { dev: Device }
#[repr(C)]
struct AppleSmc { key_count: i32 }
#[repr(C)]
struct GpioChip {
    label: *const i8,
    owner: *mut core::ffi::c_void,
    get: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    set: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32)>,
    get_direction: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    init_valid_mask: Option<unsafe extern "C" fn(*mut GpioChip, *mut usize, u32) -> i32>,
    can_sleep: bool,
    ngpio: u32,
    base: i32,
    parent: *mut Device,
}

unsafe extern "C" {
    fn hex_to_bin(c: u32) -> i32;
    fn hex_asc_hi(x: u32) -> u32;
    fn hex_asc_lo(x: u32) -> u32;
    fn apple_smc_get_key_by_index(smc: *mut AppleSmc, index: i32, key: *mut SmcKey) -> i32;
    fn apple_smc_rw_u32(smc: *mut AppleSmc, key: SmcKey, cmd: u32, val: *mut u32) -> i32;
    fn apple_smc_write_u32(smc: *mut AppleSmc, key: SmcKey, val: u32) -> i32;
    fn gpiochip_get_data(gc: *mut GpioChip) -> *mut MacsmcGpio;
    fn dev_get_drvdata(dev: *mut Device) -> *mut AppleSmc;
    fn devm_gpiochip_add_data(dev: *mut Device, gc: *mut GpioChip, data: *mut MacsmcGpio) -> i32;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn bitmap_zero(mask: *mut usize, nbits: u32);
    fn set_bit(bit: u32, mask: *mut usize);
}

const GFP_KERNEL: u32 = 0;

const GPIO_LINE_DIRECTION_IN: i32 = 1;
const GPIO_LINE_DIRECTION_OUT: i32 = 0;
const ENODEV: i32 = 19;

#[inline]
unsafe fn smc_key(s: u32) -> SmcKey { s }

unsafe fn macsmc_gpio_nr(key: SmcKey) -> i32 {
    let low = hex_to_bin(key & 0xff);
    let high = hex_to_bin((key >> 8) & 0xff);
    if low < 0 || high < 0 { return -1; }
    low | (high << 4)
}

unsafe fn macsmc_gpio_key(offset: u32) -> i32 {
    (smc_key(u32::from_be_bytes(*b"gP\0\0")) | (hex_asc_hi(offset) << 8) | hex_asc_lo(offset)) as i32
}

unsafe fn macsmc_gpio_find_first_gpio_index(smcgp: *mut MacsmcGpio) -> i32 {
    let smc = (*smcgp).smc;
    let key = macsmc_gpio_key(0) as u32;
    let (mut first_key, mut last_key) = (0, 0);
    let mut ret = apple_smc_get_key_by_index(smc, 0, &mut first_key);
    if ret != 0 { return ret; }
    if key <= first_key { return -ENODEV; }
    ret = apple_smc_get_key_by_index(smc, (*smc).key_count - 1, &mut last_key);
    if ret != 0 { return ret; }
    if key > last_key { return -ENODEV; }
    let (mut start, mut count) = (0, (*smc).key_count);
    while count > 1 {
        let mut pkey = 0;
        let mut pivot = start + ((count - 1) >> 1);
        ret = apple_smc_get_key_by_index(smc, pivot, &mut pkey);
        if ret < 0 { return ret; }
        if pkey == key { return pivot; }
        pivot += 1;
        if pkey < key { count -= pivot - start; start = pivot; } else { count = pivot - start; }
    }
    start
}

unsafe fn macsmc_gpio_get_direction(gc: *mut GpioChip, offset: u32) -> i32 {
    let smcgp = gpiochip_get_data(gc);
    let key = macsmc_gpio_key(offset) as u32;
    let mut val = 0;
    let mut ret = apple_smc_rw_u32((*smcgp).smc, key, CMD_PINMODE, &mut val);
    if ret == 0 { return if (val & MODE_OUTPUT) != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }; }
    ret = apple_smc_rw_u32((*smcgp).smc, key, CMD_IRQ_MODE, &mut val);
    if ret != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe fn macsmc_gpio_get(gc: *mut GpioChip, offset: u32) -> i32 {
    let smcgp = gpiochip_get_data(gc);
    let key = macsmc_gpio_key(offset) as u32;
    let direction = macsmc_gpio_get_direction(gc, offset);
    if direction < 0 { return direction; }
    let mut val = 0;
    let ret = apple_smc_rw_u32((*smcgp).smc, key, if direction == GPIO_LINE_DIRECTION_OUT { CMD_OUTPUT } else { CMD_INPUT }, &mut val);
    if ret < 0 { return ret; }
    if val != 0 { 1 } else { 0 }
}

unsafe fn macsmc_gpio_set(gc: *mut GpioChip, offset: u32, mut value: i32) {
    let smcgp = gpiochip_get_data(gc);
    let key = macsmc_gpio_key(offset) as u32;
    value |= CMD_OUTPUT as i32;
    let _ret = apple_smc_write_u32((*smcgp).smc, key, CMD_OUTPUT | value as u32);
}

unsafe fn macsmc_gpio_init_valid_mask(gc: *mut GpioChip, valid_mask: *mut usize, ngpios: u32) -> i32 {
    let smcgp = gpiochip_get_data(gc);
    let count = core::cmp::min((*smcgp).smc.as_ref().unwrap().key_count as usize, MAX_GPIO);
    bitmap_zero(valid_mask, ngpios);
    for i in 0..count {
        let mut key = 0;
        let ret = apple_smc_get_key_by_index((*smcgp).smc, (*smcgp).first_index + i as i32, &mut key);
        if ret < 0 { return ret; }
        if key > macsmc_gpio_key((MAX_GPIO - 1) as u32) as u32 { break; }
        let gpio_nr = macsmc_gpio_nr(key);
        if gpio_nr < 0 || gpio_nr > MAX_GPIO as i32 { continue; }
        set_bit(gpio_nr as u32, valid_mask);
    }
    0
}

unsafe fn macsmc_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let smc = dev_get_drvdata(core::ptr::addr_of_mut!((*pdev).dev));
    let mut smcgp = devm_kzalloc(core::ptr::addr_of_mut!((*pdev).dev), core::mem::size_of::<MacsmcGpio>(), GFP_KERNEL) as *mut MacsmcGpio;
    if smcgp.is_null() { return -12; }
    (*smcgp).dev = &mut (*pdev).dev;
    (*smcgp).smc = smc;
    (*smcgp).first_index = macsmc_gpio_find_first_gpio_index(smcgp);
    if (*smcgp).first_index < 0 { return (*smcgp).first_index; }
    let mut key = 0;
    let ret = apple_smc_get_key_by_index(smc, (*smcgp).first_index, &mut key);
    if ret < 0 { return ret; }
    if key > macsmc_gpio_key((MAX_GPIO - 1) as u32) as u32 { return -ENODEV; }
    (*smcgp).gc.can_sleep = true;
    (*smcgp).gc.ngpio = MAX_GPIO as u32;
    (*smcgp).gc.base = -1;
    (*smcgp).gc.parent = &mut (*pdev).dev;
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*smcgp).gc, smcgp)
}

#[repr(C)]
struct OfDeviceId { compatible: *const i8 }
static MACSMC_GPIO_OF_TABLE: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"apple,smc-gpio\0".as_ptr() as *const i8 },
    OfDeviceId { compatible: core::ptr::null() },
];

// MODULE_DEVICE_TABLE(of, macsmc_gpio_of_table);
// module_platform_driver(macsmc_gpio_driver);
// MODULE_AUTHOR("Hector Martin <marcan@marcan.st>");
// MODULE_LICENSE("Dual MIT/GPL");
// MODULE_DESCRIPTION("Apple SMC GPIO driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
