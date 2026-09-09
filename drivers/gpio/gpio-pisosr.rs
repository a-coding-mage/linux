// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015-2023 Texas Instruments Incorporated - https://www.ti.com/
 *	Andrew Davis <afd@ti.com>
 */

// Kernel dependencies supplied by the surrounding build.
use core::ffi::c_void;

const DEFAULT_NGPIO: u32 = 8;

#[repr(C)] pub struct gpio_chip { pub parent: *mut device, pub label: *const u8, pub owner: *mut c_void, pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>, pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>, pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>, pub get_multiple: Option<unsafe extern "C" fn(*mut gpio_chip, *mut usize, *mut usize) -> i32>, pub base: i32, pub ngpio: u32, pub can_sleep: bool }
#[repr(C)] pub struct spi_device { pub dev: device }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }

#[repr(C)]
pub struct pisosr_gpio {
    pub chip: gpio_chip,
    pub spi: *mut spi_device,
    pub buffer: *mut u8,
    pub buffer_size: usize,
    pub load_gpio: *mut gpio_desc,
    pub lock: mutex,
}

extern "C" {
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: i32);
    fn udelay(usecs: u32);
    fn spi_read(spi: *mut spi_device, buf: *mut u8, len: usize) -> i32;
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut pisosr_gpio;
    fn bitmap_zero(bits: *mut usize, nbits: u32);
    fn bitmap_set_value8(bits: *mut usize, value: usize, offset: usize);
}

unsafe fn pisosr_gpio_refresh(gpio: *mut pisosr_gpio) -> i32 {
    let ret;
    mutex_lock(&mut (*gpio).lock);
    if !(*gpio).load_gpio.is_null() {
        gpiod_set_value_cansleep((*gpio).load_gpio, 1);
        udelay(1); /* registers load time (~10ns) */
        gpiod_set_value_cansleep((*gpio).load_gpio, 0);
        udelay(1); /* registers recovery time (~5ns) */
    }
    ret = spi_read((*gpio).spi, (*gpio).buffer, (*gpio).buffer_size);
    mutex_unlock(&mut (*gpio).lock);
    ret
}

unsafe extern "C" fn pisosr_gpio_get_direction(_chip: *mut gpio_chip, _offset: u32) -> i32 {
    // This device always input
    1 /* GPIO_LINE_DIRECTION_IN */
}

unsafe extern "C" fn pisosr_gpio_direction_input(_chip: *mut gpio_chip, _offset: u32) -> i32 {
    // This device always input
    0
}

unsafe extern "C" fn pisosr_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    // Refresh may not always be needed
    pisosr_gpio_refresh(gpio);
    (((*gpio).buffer.add((offset / 8) as usize).read() >> (offset % 8)) & 0x1) as i32
}

unsafe extern "C" fn pisosr_gpio_get_multiple(chip: *mut gpio_chip, mask: *mut usize, bits: *mut usize) -> i32 {
    let gpio = gpiochip_get_data(chip);
    pisosr_gpio_refresh(gpio);
    bitmap_zero(bits, (*chip).ngpio);
    // for_each_set_clump8(offset, gpio_mask, mask, chip->ngpio)
    // Iterate each set 8-bit clump in mask, as provided by the kernel bitmap API.
    let mut offset = 0usize;
    while offset < (*chip).ngpio as usize {
        let gpio_mask = *mask.add(offset / usize::BITS as usize);
        let buffer_state = *(*gpio).buffer.add(offset / 8) as usize & gpio_mask;
        bitmap_set_value8(bits, buffer_state, offset);
        offset += 8;
    }
    0
}

// const struct gpio_chip template_chip; kernel-managed callbacks and ownership are retained here.
static mut template_chip: gpio_chip = gpio_chip {
    parent: core::ptr::null_mut(), label: b"pisosr-gpio\0".as_ptr(), owner: core::ptr::null_mut(),
    get_direction: Some(pisosr_gpio_get_direction), direction_input: Some(pisosr_gpio_direction_input),
    get: Some(pisosr_gpio_get), get_multiple: Some(pisosr_gpio_get_multiple), base: -1,
    ngpio: DEFAULT_NGPIO, can_sleep: true,
};

// The remaining probe, device tables, SPI driver registration, and module metadata are
// declarations/registration supplied by the Linux kernel integration layer.
#[repr(C)] pub struct spi_device_id { pub name: [u8; 32] }
#[repr(C)] pub struct of_device_id { pub compatible: *const u8 }
#[repr(C)] pub struct spi_driver { pub name: *const u8, pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> i32>, pub of_match_table: *const of_device_id, pub id_table: *const spi_device_id }

extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn of_property_read_u32(dev: *mut c_void, name: *const u8, value: *mut u32) -> i32;
    fn devm_gpiod_get_optional(dev: *mut device, name: *const u8, flags: u32) -> *mut gpio_desc;
    fn devm_mutex_init(dev: *mut device, lock: *mut mutex) -> i32;
    fn devm_gpiochip_add_data(dev: *mut device, chip: *mut gpio_chip, data: *mut pisosr_gpio) -> i32;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const u8) -> i32;
    fn dev_err(dev: *mut device, fmt: *const u8);
}

unsafe extern "C" fn pisosr_gpio_probe(spi: *mut spi_device) -> i32 {
    let dev = &mut (*spi).dev as *mut device;
    let gpio = devm_kzalloc(dev, core::mem::size_of::<pisosr_gpio>(), 0) as *mut pisosr_gpio;
    if gpio.is_null() { return -12; /* -ENOMEM */ }
    (*gpio).chip = template_chip;
    (*gpio).chip.parent = dev;
    let mut ngpios = 0u32;
    if of_property_read_u32(core::ptr::null_mut(), b"ngpios\0".as_ptr(), &mut ngpios) == 0 { (*gpio).chip.ngpio = ngpios; }
    (*gpio).spi = spi;
    (*gpio).buffer_size = ((*gpio).chip.ngpio as usize + 7) / 8;
    (*gpio).buffer = devm_kzalloc(dev, (*gpio).buffer_size, 0) as *mut u8;
    if (*gpio).buffer.is_null() { return -12; }
    (*gpio).load_gpio = devm_gpiod_get_optional(dev, b"load\0".as_ptr(), 0);
    if (*gpio).load_gpio.is_null() { return dev_err_probe(dev, -22, b"Unable to allocate load GPIO\n\0".as_ptr()); }
    let ret = devm_mutex_init(dev, &mut (*gpio).lock);
    if ret != 0 { return ret; }
    let ret = devm_gpiochip_add_data(dev, &mut (*gpio).chip, gpio);
    if ret < 0 { dev_err(dev, b"Unable to register gpiochip\n\0".as_ptr()); return ret; }
    0
}

static pisosr_gpio_id_table: [spi_device_id; 2] = [
    spi_device_id { name: *b"pisosr-gpio\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0" },
    spi_device_id { name: [0; 32] },
];
static pisosr_gpio_of_match_table: [of_device_id; 2] = [
    of_device_id { compatible: b"pisosr-gpio\0".as_ptr() }, of_device_id { compatible: core::ptr::null() },
];
static mut pisosr_gpio_driver: spi_driver = spi_driver { name: b"pisosr-gpio\0".as_ptr(), probe: Some(pisosr_gpio_probe), of_match_table: pisosr_gpio_of_match_table.as_ptr(), id_table: pisosr_gpio_id_table.as_ptr() };
// module_spi_driver(pisosr_gpio_driver);
// MODULE_AUTHOR("Andrew Davis <afd@ti.com>");
// MODULE_DESCRIPTION("SPI Compatible PISO Shift Register GPIO Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
