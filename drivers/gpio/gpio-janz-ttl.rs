// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Janz MODULbus VMOD-TTL GPIO Driver
 *
 * Copyright (c) 2010 Ira W. Snyder <iws@ovro.caltech.edu>
 */

// Linux kernel dependencies supplied by the surrounding kernel environment.

const DRV_NAME: &str = "janz-ttl";

const PORTA_DIRECTION: u8 = 0x23;
const PORTB_DIRECTION: u8 = 0x2b;
const PORTC_DIRECTION: u8 = 0x06;
const PORTA_IOCTL: u8 = 0x24;
const PORTB_IOCTL: u8 = 0x2c;
const PORTC_IOCTL: u8 = 0x07;

const MASTER_INT_CTL: u8 = 0x00;
const MASTER_CONF_CTL: u8 = 0x01;

const CONF_PAE: u16 = 1 << 2;
const CONF_PBE: u16 = 1 << 7;
const CONF_PCE: u16 = 1 << 4;

#[repr(C)]
struct TtlControlRegs {
    portc: u16,
    portb: u16,
    porta: u16,
    control: u16,
}

#[repr(C)]
struct TtlModule {
    gpio: GpioChip,

    /* base address of registers */
    regs: *mut TtlControlRegs,

    portc_shadow: u8,
    portb_shadow: u8,
    porta_shadow: u8,

    lock: Spinlock,
}

// These types and functions are supplied by the Linux kernel environment.
#[repr(C)]
struct GpioChip {
    parent: *mut Device,
    label: *const u8,
    get: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    set: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32)>,
    owner: *mut Module,
    base: i32,
    ngpio: u32,
}

#[repr(C)]
struct Spinlock;
#[repr(C)]
struct Device;
#[repr(C)]
struct Module;
#[repr(C)]
struct PlatformDevice {
    dev: Device,
    name: *const u8,
}
#[repr(C)]
struct PlatformDriver;
#[repr(C)]
struct JanzPlatformData;

extern "C" {
    fn dev_get_drvdata(dev: *mut Device) -> *mut core::ffi::c_void;
    fn dev_get_platdata(dev: *mut Device) -> *mut JanzPlatformData;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut core::ffi::c_void);
    fn spin_lock_init(lock: *mut Spinlock);
    fn spin_lock(lock: *mut Spinlock);
    fn spin_unlock(lock: *mut Spinlock);
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut TtlControlRegs;
    fn iowrite16be(value: u16, address: *mut u16);
    fn devm_gpiochip_add_data(dev: *mut Device, gpio: *mut GpioChip, data: *mut core::ffi::c_void) -> i32;
    fn dev_err(dev: *mut Device, message: *const u8);
}

const GFP_KERNEL: u32 = 0;
const ENXIO: i32 = 6;
const ENOMEM: i32 = 12;

unsafe extern "C" fn ttl_get_value(gpio: *mut GpioChip, mut offset: u32) -> i32 {
    let mod_ = dev_get_drvdata((*gpio).parent) as *mut TtlModule;
    let shadow: *mut u8;
    let mut ret: u8;

    if offset < 8 {
        shadow = &mut (*mod_).porta_shadow;
    } else if offset < 16 {
        shadow = &mut (*mod_).portb_shadow;
        offset -= 8;
    } else {
        shadow = &mut (*mod_).portc_shadow;
        offset -= 16;
    }

    spin_lock(&mut (*mod_).lock);
    ret = *shadow & (1u8 << offset);
    spin_unlock(&mut (*mod_).lock);
    (ret != 0) as i32
}

unsafe extern "C" fn ttl_set_value(gpio: *mut GpioChip, mut offset: u32, value: i32) {
    let mod_ = dev_get_drvdata((*gpio).parent) as *mut TtlModule;
    let port: *mut u16;
    let shadow: *mut u8;

    if offset < 8 {
        port = &mut (*(*mod_).regs).porta;
        shadow = &mut (*mod_).porta_shadow;
    } else if offset < 16 {
        port = &mut (*(*mod_).regs).portb;
        shadow = &mut (*mod_).portb_shadow;
        offset -= 8;
    } else {
        port = &mut (*(*mod_).regs).portc;
        shadow = &mut (*mod_).portc_shadow;
        offset -= 16;
    }

    spin_lock(&mut (*mod_).lock);
    if value != 0 {
        *shadow |= 1u8 << offset;
    } else {
        *shadow &= !(1u8 << offset);
    }
    iowrite16be(*shadow as u16, port);
    spin_unlock(&mut (*mod_).lock);
}

unsafe fn ttl_write_reg(mod_: *mut TtlModule, reg: u8, val: u16) {
    iowrite16be(reg as u16, &mut (*(*mod_).regs).control);
    iowrite16be(val, &mut (*(*mod_).regs).control);
}

unsafe fn ttl_setup_device(mod_: *mut TtlModule) {
    /* reset the device to a known state */
    iowrite16be(0x0000, &mut (*(*mod_).regs).control);
    iowrite16be(0x0001, &mut (*(*mod_).regs).control);
    iowrite16be(0x0000, &mut (*(*mod_).regs).control);

    /* put all ports in open-drain mode */
    ttl_write_reg(mod_, PORTA_IOCTL, 0x00ff);
    ttl_write_reg(mod_, PORTB_IOCTL, 0x00ff);
    ttl_write_reg(mod_, PORTC_IOCTL, 0x000f);

    /* set all ports as outputs */
    ttl_write_reg(mod_, PORTA_DIRECTION, 0x0000);
    ttl_write_reg(mod_, PORTB_DIRECTION, 0x0000);
    ttl_write_reg(mod_, PORTC_DIRECTION, 0x0000);

    /* set all ports to drive zeroes */
    iowrite16be(0x0000, &mut (*(*mod_).regs).porta);
    iowrite16be(0x0000, &mut (*(*mod_).regs).portb);
    iowrite16be(0x0000, &mut (*(*mod_).regs).portc);

    /* enable all ports */
    ttl_write_reg(mod_, MASTER_CONF_CTL, CONF_PAE | CONF_PBE | CONF_PCE);
}

unsafe extern "C" fn ttl_probe(pdev: *mut PlatformDevice) -> i32 {
    let pdata = dev_get_platdata(&mut (*pdev).dev);
    let mod_: *mut TtlModule;
    let gpio: *mut GpioChip;
    let mut ret: i32;

    if pdata.is_null() {
        dev_err(&mut (*pdev).dev, b"no platform data\0".as_ptr());
        return -ENXIO;
    }

    mod_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<TtlModule>(), GFP_KERNEL) as *mut TtlModule;
    if mod_.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, mod_ as *mut core::ffi::c_void);
    spin_lock_init(&mut (*mod_).lock);

    /* get access to the MODULbus registers for this module */
    (*mod_).regs = devm_platform_ioremap_resource(pdev, 0);
    if (*mod_).regs.is_null() {
        return -1;
    }

    ttl_setup_device(mod_);

    /* Initialize the GPIO data structures */
    gpio = &mut (*mod_).gpio;
    (*gpio).parent = &mut (*pdev).dev;
    (*gpio).label = (*pdev).name;
    (*gpio).get = Some(ttl_get_value);
    (*gpio).set = Some(ttl_set_value);
    (*gpio).owner = core::ptr::null_mut();

    /* request dynamic allocation */
    (*gpio).base = -1;
    (*gpio).ngpio = 20;

    ret = devm_gpiochip_add_data(&mut (*pdev).dev, gpio, core::ptr::null_mut());
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"unable to add GPIO chip\0".as_ptr());
        return ret;
    }

    0
}

#[repr(C)]
struct DriverInfo {
    name: *const u8,
}

#[repr(C)]
struct TtlDriver {
    driver: DriverInfo,
    probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
}

static mut TTL_DRIVER: TtlDriver = TtlDriver {
    driver: DriverInfo { name: DRV_NAME.as_ptr() },
    probe: Some(ttl_probe),
};

// Equivalent of module_platform_driver(ttl_driver), registered by the kernel module.

// MODULE_AUTHOR("Ira W. Snyder <iws@ovro.caltech.edu>");
// MODULE_DESCRIPTION("Janz MODULbus VMOD-TTL Driver");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:janz-ttl");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
