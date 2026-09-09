// SPDX-License-Identifier: GPL-2.0-only
/*
 * PPC44x gpio driver
 *
 * Copyright (c) 2008 Harris Corporation
 * Copyright (c) 2008 Sascha Hauer <s.hauer@pengutronix.de>, Pengutronix
 * Copyright (c) MontaVista Software, Inc. 2008.
 *
 * Author: Steve Falco <sfalco@harris.com>
 */

const fn gpio_mask(gpio: u32) -> u32 { 0x8000_0000u32 >> gpio }
const fn gpio_mask2(gpio: u32) -> u32 { 0xc000_0000u32 >> ((gpio % 16) * 2) }

#[repr(C)]
pub struct Ppc44xGpio {
    pub or: u32,
    pub tcr: u32,
    pub osrl: u32,
    pub osrh: u32,
    pub tsrl: u32,
    pub tsrh: u32,
    pub odr: u32,
    pub ir: u32,
    pub rr1: u32,
    pub rr2: u32,
    pub rr3: u32,
    pub reserved1: u32,
    pub isr1l: u32,
    pub isr1h: u32,
    pub isr2l: u32,
    pub isr2h: u32,
    pub isr3l: u32,
    pub isr3h: u32,
}

#[repr(C)]
pub struct Ppc44xGpioChip {
    pub chip: gpio_generic_chip,
    pub regs: *mut core::ffi::c_void,
}

#[inline]
unsafe fn ppc44x_clrbits32(addr: *mut core::ffi::c_void, mask: u32) {
    let mut val = ioread32be(addr);
    val &= !mask;
    iowrite32be(val, addr);
}

#[inline]
unsafe fn ppc44x_setbits32(addr: *mut core::ffi::c_void, mask: u32) {
    let mut val = ioread32be(addr);
    val |= mask;
    iowrite32be(val, addr);
}

/*
 * GPIO LIB API implementation for GPIOs
 *
 * There are a maximum of 32 gpios in each gpio controller.
 */

#[inline]
unsafe fn __ppc44x_gpio_set(gc: *mut gpio_chip, gpio: u32, val: i32) {
    let chip = gpiochip_get_data(gc) as *mut Ppc44xGpioChip;
    let gen_gc = &mut (*chip).chip;

    if val != 0 {
        (*gen_gc).sdata |= gpio_mask(gpio);
    } else {
        (*gen_gc).sdata &= !gpio_mask(gpio);
    }
    gpio_generic_write_reg(gen_gc, (*gen_gc).reg_set, (*gen_gc).sdata);
}

unsafe fn ppc44x_gpio_dir_in(gc: *mut gpio_chip, gpio: u32) -> i32 {
    let chip = gpiochip_get_data(gc) as *mut Ppc44xGpioChip;
    let gen_gc = &mut (*chip).chip;
    let regs = (*chip).regs as *mut Ppc44xGpio;
    let _lock = gpio_generic_lock_irqsave(gen_gc);

    /* Disable open-drain function */
    ppc44x_clrbits32(&mut (*regs).odr as *mut u32 as *mut core::ffi::c_void, gpio_mask(gpio));
    /* Float the pin */
    ppc44x_clrbits32(&mut (*regs).tcr as *mut u32 as *mut core::ffi::c_void, gpio_mask(gpio));
    (*gen_gc).sdir &= !gpio_mask(gpio);

    /* Bits 0-15 use TSRL/OSRL, bits 16-31 use TSRH/OSRH */
    if gpio < 16 {
        ppc44x_clrbits32(&mut (*regs).osrl as *mut u32 as *mut core::ffi::c_void, gpio_mask2(gpio));
        ppc44x_clrbits32(&mut (*regs).tsrl as *mut u32 as *mut core::ffi::c_void, gpio_mask2(gpio));
    } else {
        ppc44x_clrbits32(&mut (*regs).osrh as *mut u32 as *mut core::ffi::c_void, gpio_mask2(gpio));
        ppc44x_clrbits32(&mut (*regs).tsrh as *mut u32 as *mut core::ffi::c_void, gpio_mask2(gpio));
    }
    0
}

unsafe fn ppc44x_gpio_dir_out(gc: *mut gpio_chip, gpio: u32, val: i32) -> i32 {
    let chip = gpiochip_get_data(gc) as *mut Ppc44xGpioChip;
    let gen_gc = &mut (*chip).chip;
    let regs = (*chip).regs as *mut Ppc44xGpio;
    let _lock = gpio_generic_lock_irqsave(gen_gc);

    /* First set initial value */
    __ppc44x_gpio_set(gc, gpio, val);
    /* Disable open-drain function */
    ppc44x_clrbits32(&mut (*regs).odr as *mut u32 as *mut core::ffi::c_void, gpio_mask(gpio));
    /* Drive the pin */
    ppc44x_setbits32(&mut (*regs).tcr as *mut u32 as *mut core::ffi::c_void, gpio_mask(gpio));
    (*gen_gc).sdir |= gpio_mask(gpio);

    /* Bits 0-15 use TSRL, bits 16-31 use TSRH */
    if gpio < 16 {
        ppc44x_clrbits32(&mut (*regs).osrl as *mut u32 as *mut core::ffi::c_void, gpio_mask2(gpio));
        ppc44x_clrbits32(&mut (*regs).tsrl as *mut u32 as *mut core::ffi::c_void, gpio_mask2(gpio));
    } else {
        ppc44x_clrbits32(&mut (*regs).osrh as *mut u32 as *mut core::ffi::c_void, gpio_mask2(gpio));
        ppc44x_clrbits32(&mut (*regs).tsrh as *mut u32 as *mut core::ffi::c_void, gpio_mask2(gpio));
    }
    pr_debug("%s: gpio: %d val: %d\n", "ppc44x_gpio_dir_out", gpio, val);
    0
}

/* External kernel declarations supplied by the surrounding translation unit. */
extern "C" {
    fn ioread32be(addr: *mut core::ffi::c_void) -> u32;
    fn iowrite32be(value: u32, addr: *mut core::ffi::c_void);
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn gpio_generic_write_reg(gc: *mut gpio_generic_chip, reg: *mut u32, value: u32);
    fn gpio_generic_lock_irqsave(gc: *mut gpio_generic_chip) -> *mut core::ffi::c_void;
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
}

#[allow(non_camel_case_types)]
pub struct gpio_generic_chip { pub gc: gpio_chip, pub sdata: u32, pub sdir: u32, pub reg_set: *mut u32 }
#[allow(non_camel_case_types)]
pub struct gpio_chip { pub label: *const core::ffi::c_char, pub parent: *mut device, pub direction_input: Option<unsafe fn(*mut gpio_chip, u32) -> i32>, pub direction_output: Option<unsafe fn(*mut gpio_chip, u32, i32) -> i32> }
#[allow(non_camel_case_types)] pub struct device;

unsafe fn ppc44x_gpio_probe(ofdev: *mut platform_device) -> i32 {
    let dev = &mut (*ofdev).dev;
    let regs = devm_platform_ioremap_resource(ofdev, 0);
    if is_err(regs) { return ptr_err(regs); }
    let chip = devm_kzalloc(dev, core::mem::size_of::<Ppc44xGpioChip>(), GFP_KERNEL) as *mut Ppc44xGpioChip;
    if chip.is_null() { return -12; }
    (*chip).regs = regs;
    let config = gpio_generic_chip_config {
        dev,
        sz: 4,
        dat: &mut (*(regs as *mut Ppc44xGpio)).ir,
        set: &mut (*(regs as *mut Ppc44xGpio)).or,
        dirout: &mut (*(regs as *mut Ppc44xGpio)).tcr,
        flags: GPIO_GENERIC_BIG_ENDIAN | GPIO_GENERIC_BIG_ENDIAN_BYTE_ORDER,
    };
    let ret = gpio_generic_chip_init(&mut (*chip).chip, &config);
    if ret != 0 { return ret; }
    let gc = &mut (*chip).chip.gc;
    gc.label = dev_name(dev);
    gc.parent = dev;
    gc.direction_input = Some(ppc44x_gpio_dir_in);
    gc.direction_output = Some(ppc44x_gpio_dir_out);
    devm_gpiochip_add_data(dev, gc, chip)
}

#[repr(C)] pub struct of_device_id { pub compatible: *const core::ffi::c_char }
static PPC44X_GPIO_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"ibm,ppc4xx-gpio\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct platform_driver { pub probe: Option<unsafe fn(*mut platform_device) -> i32>, pub name: *const core::ffi::c_char, pub of_match_table: *const of_device_id }
static PPC44X_GPIO_DRIVER: platform_driver = platform_driver { probe: Some(ppc44x_gpio_probe), name: b"ppc44x-gpio\0".as_ptr() as *const _, of_match_table: PPC44X_GPIO_MATCH.as_ptr() };

#[allow(non_upper_case_globals)] const GFP_KERNEL: u32 = 0;
#[allow(non_upper_case_globals)] const GPIO_GENERIC_BIG_ENDIAN: u32 = 1;
#[allow(non_upper_case_globals)] const GPIO_GENERIC_BIG_ENDIAN_BYTE_ORDER: u32 = 2;
#[repr(C)] pub struct gpio_generic_chip_config { pub dev: *mut device, pub sz: u32, pub dat: *mut u32, pub set: *mut u32, pub dirout: *mut u32, pub flags: u32 }

extern "C" {
    fn devm_platform_ioremap_resource(dev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn is_err(ptr: *mut core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn gpio_generic_chip_init(chip: *mut gpio_generic_chip, config: *const gpio_generic_chip_config) -> i32;
    fn dev_name(dev: *mut device) -> *const core::ffi::c_char;
    fn devm_gpiochip_add_data(dev: *mut device, gc: *mut gpio_chip, data: *mut Ppc44xGpioChip) -> i32;
}

/* MODULE_DEVICE_TABLE(of, ppc44x_gpio_match); */
/* MODULE_DESCRIPTION("PPC44x gpio driver"); */
/* MODULE_LICENSE("GPL"); */
/* module_platform_driver(ppc44x_gpio_driver); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
