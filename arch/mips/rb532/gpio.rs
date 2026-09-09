/*
 *  Miscellaneous functions for IDT EB434 board
 *
 *  Copyright 2004 IDT Inc. (rischelp@idt.com)
 *  Copyright 2006 Phil Sutter <n0-1@freewrt.org>
 *  Copyright 2007 Florian Fainelli <florian@openwrt.org>
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under the terms of the GNU General  Public License as published by the
 *  Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 */

/* Linux and platform-specific declarations are supplied by external dependencies. */

const GPIOFUNC: usize = 0x00;
const GPIOCFG: usize = 0x04;
const GPIOD: usize = 0x08;
const GPIOILEVEL: usize = 0x0C;
const GPIOISTAT: usize = 0x10;
const GPIONMIEN: usize = 0x14;
const IMASK6: usize = 0x38;

#[repr(C)]
struct Rb532GpioChip {
    chip: gpio_chip,
    regbase: *mut core::ffi::c_void,
}

/* rb532_set_bit - sanely set a bit
 *
 * bitval: new value for the bit
 * offset: bit index in the 4 byte address range
 * ioaddr: 4 byte aligned address being altered
 */
#[inline]
unsafe fn rb532_set_bit(bitval: c_uint, offset: c_uint, ioaddr: *mut core::ffi::c_void) {
    let mut flags: c_ulong = 0;
    let mut val: u32;

    local_irq_save(&mut flags);

    val = readl(ioaddr);
    val &= !(((!bitval) != 0) as u32 << offset);
    val |= (((bitval != 0) as u32) << offset);
    writel(val, ioaddr);

    local_irq_restore(flags);
}

/* rb532_get_bit - read a bit
 *
 * returns the boolean state of the bit, which may be > 1
 */
#[inline]
unsafe fn rb532_get_bit(offset: c_uint, ioaddr: *mut core::ffi::c_void) -> c_int {
    (readl(ioaddr) & (1u32 << offset)) as c_int
}

/* Return GPIO level */
unsafe extern "C" fn rb532_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let gpch = gpiochip_get_data(chip) as *mut Rb532GpioChip;
    (rb532_get_bit(offset, (*gpch).regbase.add(GPIOD)) != 0) as c_int
}

/* Set output GPIO level */
unsafe extern "C" fn rb532_gpio_set(
    chip: *mut gpio_chip,
    offset: c_uint,
    value: c_int,
) {
    let gpch = gpiochip_get_data(chip) as *mut Rb532GpioChip;
    rb532_set_bit(value as c_uint, offset, (*gpch).regbase.add(GPIOD));
}

/* Set GPIO direction to input */
unsafe extern "C" fn rb532_gpio_direction_input(
    chip: *mut gpio_chip,
    offset: c_uint,
) -> c_int {
    let gpch = gpiochip_get_data(chip) as *mut Rb532GpioChip;

    /* disable alternate function in case it's set */
    rb532_set_bit(0, offset, (*gpch).regbase.add(GPIOFUNC));
    rb532_set_bit(0, offset, (*gpch).regbase.add(GPIOCFG));
    0
}

/* Set GPIO direction to output */
unsafe extern "C" fn rb532_gpio_direction_output(
    chip: *mut gpio_chip,
    offset: c_uint,
    value: c_int,
) -> c_int {
    let gpch = gpiochip_get_data(chip) as *mut Rb532GpioChip;

    /* disable alternate function in case it's set */
    rb532_set_bit(0, offset, (*gpch).regbase.add(GPIOFUNC));
    /* set the initial output value */
    rb532_set_bit(value as c_uint, offset, (*gpch).regbase.add(GPIOD));
    rb532_set_bit(1, offset, (*gpch).regbase.add(GPIOCFG));
    0
}

unsafe extern "C" fn rb532_gpio_to_irq(_chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    (8 + 4 * 32 + gpio) as c_int
}

static mut RB532_GPIO_CHIP: [Rb532GpioChip; 1] = [Rb532GpioChip {
    chip: gpio_chip {
        label: b"gpio0\0".as_ptr() as *const c_char,
        direction_input: Some(rb532_gpio_direction_input),
        direction_output: Some(rb532_gpio_direction_output),
        get: Some(rb532_gpio_get),
        set: Some(rb532_gpio_set),
        to_irq: Some(rb532_gpio_to_irq),
        base: 0,
        ngpio: 32,
    },
    regbase: core::ptr::null_mut(),
}];

/* Set GPIO interrupt level */
#[no_mangle]
pub unsafe extern "C" fn rb532_gpio_set_ilevel(bit: c_int, gpio: c_uint) {
    rb532_set_bit(bit as c_uint, gpio, RB532_GPIO_CHIP[0].regbase.add(GPIOILEVEL));
}

/* Set GPIO interrupt status */
#[no_mangle]
pub unsafe extern "C" fn rb532_gpio_set_istat(bit: c_int, gpio: c_uint) {
    rb532_set_bit(bit as c_uint, gpio, RB532_GPIO_CHIP[0].regbase.add(GPIOISTAT));
}

/* Configure GPIO alternate function */
#[no_mangle]
pub unsafe extern "C" fn rb532_gpio_set_func(gpio: c_uint) {
    rb532_set_bit(1, gpio, RB532_GPIO_CHIP[0].regbase.add(GPIOFUNC));
}

unsafe extern "C" fn rb532_gpio_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let res: *mut resource;

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        return -EINVAL;
    }

    RB532_GPIO_CHIP[0].regbase = devm_ioremap_resource(dev, res);
    if is_err(RB532_GPIO_CHIP[0].regbase) {
        return ptr_err(RB532_GPIO_CHIP[0].regbase);
    }

    /* Register our GPIO chip */
    devm_gpiochip_add_data(dev, &mut RB532_GPIO_CHIP[0].chip, &mut RB532_GPIO_CHIP[0])
}

static mut RB532_GPIO_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: b"rb532-gpio\0".as_ptr() as *const c_char,
    },
    probe: Some(rb532_gpio_probe),
};

unsafe extern "C" fn rb532_gpio_init() -> c_int {
    platform_driver_register(&mut RB532_GPIO_DRIVER)
}

/* arch_initcall(rb532_gpio_init); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
