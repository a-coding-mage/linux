// SPDX-License-Identifier: GPL-2.0-only
/*
 * MC33880 high-side/low-side switch GPIO driver
 * Copyright (c) 2009 Intel Corporation
 */

/* Supports:
 * Freescale MC33880 high-side/low-side switch
 */

// External Linux kernel dependencies corresponding to the C includes.

const DRIVER_NAME: &str = "mc33880";

/*
 * Pin configurations, see MAX7301 datasheet page 6
 */
const PIN_CONFIG_MASK: u8 = 0x03;
const PIN_CONFIG_IN_PULLUP: u8 = 0x03;
const PIN_CONFIG_IN_WO_PULLUP: u8 = 0x02;
const PIN_CONFIG_OUT: u8 = 0x01;

const PIN_NUMBER: u32 = 8;

/*
 * Some registers must be read back to modify.
 * To save time we cache them here in memory
 */
#[repr(C)]
struct Mc33880 {
    lock: Mutex, /* protect from simultaneous accesses */
    port_config: u8,
    chip: GpioChip,
    spi: *mut SpiDevice,
}

#[repr(C)]
struct Mutex {
    _opaque: [u8; 0],
}

#[repr(C)]
struct GpioChip {
    label: *const u8,
    set: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32)>,
    base: i32,
    ngpio: u32,
    can_sleep: bool,
    parent: *mut Device,
    owner: *mut Module,
}

#[repr(C)]
struct SpiDevice {
    dev: Device,
    bits_per_word: u8,
}

#[repr(C)]
struct Device {
    _opaque: [u8; 0],
}

#[repr(C)]
struct Module {
    _opaque: [u8; 0],
}

#[repr(C)]
struct Mc33880PlatformData {
    base: i32,
}

extern "C" {
    static mut THIS_MODULE: Module;
    fn spi_write(spi: *mut SpiDevice, buf: *const u8, len: usize) -> i32;
    fn gpiochip_get_data(chip: *mut GpioChip) -> *mut Mc33880;
    fn mutex_lock(lock: *mut Mutex);
    fn mutex_unlock(lock: *mut Mutex);
    fn mutex_init(lock: *mut Mutex);
    fn mutex_destroy(lock: *mut Mutex);
    fn dev_get_platdata(dev: *mut Device) -> *mut Mc33880PlatformData;
    fn spi_setup(spi: *mut SpiDevice) -> i32;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut Mc33880;
    fn spi_set_drvdata(spi: *mut SpiDevice, data: *mut Mc33880);
    fn gpiochip_add_data(chip: *mut GpioChip, data: *mut Mc33880) -> i32;
    fn gpiochip_remove(chip: *mut GpioChip);
    fn spi_get_drvdata(spi: *mut SpiDevice) -> *mut Mc33880;
    fn spi_register_driver(driver: *mut SpiDriver) -> i32;
    fn spi_unregister_driver(driver: *mut SpiDriver);
}

const GFP_KERNEL: u32 = 0;

unsafe fn mc33880_write_config(mc: *mut Mc33880) -> i32 {
    spi_write((*mc).spi, &(*mc).port_config, core::mem::size_of_val(&(*mc).port_config))
}

unsafe fn __mc33880_set(mc: *mut Mc33880, offset: u32, value: i32) -> i32 {
    if value != 0 {
        (*mc).port_config |= 1u8.wrapping_shl(offset);
    } else {
        (*mc).port_config &= !(1u8.wrapping_shl(offset));
    }

    mc33880_write_config(mc)
}

unsafe extern "C" fn mc33880_set(chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let mc = gpiochip_get_data(chip);
    let ret: i32;

    mutex_lock(&mut (*mc).lock);
    ret = __mc33880_set(mc, offset, value);
    mutex_unlock(&mut (*mc).lock);

    ret
}

unsafe fn mc33880_probe(spi: *mut SpiDevice) -> i32 {
    let mut mc: *mut Mc33880;
    let pdata: *mut Mc33880PlatformData;
    let mut ret: i32;

    pdata = dev_get_platdata(&mut (*spi).dev);
    if pdata.is_null() || (*pdata).base == 0 {
        return -22;
    }

    /* bits_per_word cannot be configured in platform data */
    (*spi).bits_per_word = 8;

    ret = spi_setup(spi);
    if ret < 0 {
        return ret;
    }

    mc = devm_kzalloc(&mut (*spi).dev, core::mem::size_of::<Mc33880>(), GFP_KERNEL);
    if mc.is_null() {
        return -12;
    }

    mutex_init(&mut (*mc).lock);
    spi_set_drvdata(spi, mc);
    (*mc).spi = spi;

    (*mc).chip.label = DRIVER_NAME.as_ptr();
    (*mc).chip.set = Some(mc33880_set);
    (*mc).chip.base = (*pdata).base;
    (*mc).chip.ngpio = PIN_NUMBER;
    (*mc).chip.can_sleep = true;
    (*mc).chip.parent = &mut (*spi).dev;
    (*mc).chip.owner = &mut THIS_MODULE;

    (*mc).port_config = 0x00;
    /* write twice, because during initialisation the first setting
     * is just for testing SPI communication, and the second is the
     * "real" configuration
     */
    ret = mc33880_write_config(mc);
    (*mc).port_config = 0x00;
    if ret == 0 {
        ret = mc33880_write_config(mc);
    }

    if ret != 0 {
        mutex_destroy(&mut (*mc).lock);
        return ret;
    }

    ret = gpiochip_add_data(&mut (*mc).chip, mc);
    if ret != 0 {
        mutex_destroy(&mut (*mc).lock);
    }
    ret
}

unsafe fn mc33880_remove(spi: *mut SpiDevice) {
    let mc = spi_get_drvdata(spi);
    gpiochip_remove(&mut (*mc).chip);
    mutex_destroy(&mut (*mc).lock);
}

#[repr(C)]
struct SpiDriver {
    driver: Driver,
    probe: Option<unsafe fn(*mut SpiDevice) -> i32>,
    remove: Option<unsafe fn(*mut SpiDevice)>,
}

#[repr(C)]
struct Driver {
    name: *const u8,
}

static mut MC33880_DRIVER: SpiDriver = SpiDriver {
    driver: Driver { name: DRIVER_NAME.as_ptr() },
    probe: Some(mc33880_probe),
    remove: Some(mc33880_remove),
};

unsafe fn mc33880_init() -> i32 {
    spi_register_driver(&mut MC33880_DRIVER)
}

/* register after spi postcore initcall and before
 * subsys initcalls that may rely on these GPIOs
 */
// Equivalent to subsys_initcall(mc33880_init).

unsafe fn mc33880_exit() {
    spi_unregister_driver(&mut MC33880_DRIVER);
}

// Equivalent to module_exit(mc33880_exit).
// MODULE_AUTHOR("Mocean Laboratories <info@mocean-labs.com>");
// MODULE_DESCRIPTION("MC33880 high-side/low-side switch GPIO driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
