// SPDX-License-Identifier: GPL-2.0-only
/*
 * MEN 16Z127 GPIO driver
 *
 * Copyright (C) 2016 MEN Mikroelektronik GmbH (www.men.de)
 */

// Linux kernel dependencies are supplied by the surrounding crate.

const MEN_Z127_CTRL: usize = 0x00;
const MEN_Z127_PSR: usize = 0x04;
const MEN_Z127_IRQR: usize = 0x08;
const MEN_Z127_GPIODR: usize = 0x0c;
const MEN_Z127_IER1: usize = 0x10;
const MEN_Z127_IER2: usize = 0x14;
const MEN_Z127_DBER: usize = 0x18;
const MEN_Z127_ODER: usize = 0x1C;

#[inline]
const fn gpio_to_dbcnt_reg(gpio: usize) -> usize {
    (gpio * 4) + 0x80
}

/* MEN Z127 supported model ids */
const MEN_Z127_ID: i32 = 0x7f;
const MEN_Z034_ID: i32 = 0x22;
const MEN_Z037_ID: i32 = 0x25;

const MEN_Z127_DB_MIN_US: u32 = 50;
/* 16 bit compare register. Each bit represents 50us */
const MEN_Z127_DB_MAX_US: u32 = 0xffff * MEN_Z127_DB_MIN_US;

#[repr(C)]
pub struct men_z127_gpio {
    pub chip: gpio_generic_chip,
    pub reg_base: *mut core::ffi::c_void,
    pub mem: *mut resource,
}

#[allow(non_camel_case_types)]
pub struct gpio_generic_chip { _private: [u8; 0] }
#[allow(non_camel_case_types)]
pub struct gpio_chip { pub parent: *mut device, pub set_config: Option<unsafe extern "C" fn(*mut gpio_chip, u32, usize) -> i32> }
#[allow(non_camel_case_types)]
pub struct resource { pub start: usize }
#[allow(non_camel_case_types)]
pub struct device { _private: [u8; 0] }
#[allow(non_camel_case_types)]
pub struct mcb_device { pub dev: device, pub id: i32 }
#[allow(non_camel_case_types)]
pub struct mcb_device_id { pub device: i32 }
#[allow(non_camel_case_types)]
pub struct gpio_generic_chip_config {
    pub dev: *mut device,
    pub sz: usize,
    pub dat: *mut u8,
    pub set: *mut u8,
    pub dirout: *mut u8,
}
#[allow(non_camel_case_types)]
pub enum pin_config_param { PIN_CONFIG_DRIVE_OPEN_DRAIN, PIN_CONFIG_DRIVE_PUSH_PULL, PIN_CONFIG_INPUT_DEBOUNCE }

extern "C" {
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut men_z127_gpio;
    fn fls(x: u32) -> u32;
    fn roundup(x: u32, y: u32) -> u32;
    fn rounddown(x: u32, y: u32) -> u32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn pinconf_to_config_param(config: usize) -> pin_config_param;
    fn pinconf_to_config_argument(config: usize) -> u32;
    fn mcb_release_mem(res: *mut resource);
    fn mcb_request_mem(mdev: *mut mcb_device, name: *const core::ffi::c_char) -> *mut resource;
    fn dev_name(dev: *mut device) -> *const core::ffi::c_char;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut men_z127_gpio;
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut core::ffi::c_void), data: *mut core::ffi::c_void) -> i32;
    fn devm_ioremap(dev: *mut device, start: usize, size: usize) -> *mut core::ffi::c_void;
    fn resource_size(res: *mut resource) -> usize;
    fn mcb_set_drvdata(mdev: *mut mcb_device, data: *mut men_z127_gpio);
    fn gpio_generic_chip_init(chip: *mut gpio_generic_chip, config: *mut gpio_generic_chip_config) -> i32;
    fn devm_gpiochip_add_data(dev: *mut device, gc: *mut gpio_chip, data: *mut men_z127_gpio) -> i32;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const core::ffi::c_char, ... ) -> i32;
}

unsafe fn men_z127_debounce(gc: *mut gpio_chip, gpio: u32, mut debounce: u32) -> i32 {
    let priv_ = &mut *gpiochip_get_data(gc);
    let dev = (*gc).parent;
    if !(debounce >= MEN_Z127_DB_MIN_US && debounce <= MEN_Z127_DB_MAX_US) {
        dev_err(dev, b"debounce value %u out of range\0".as_ptr() as _, debounce);
        return -22;
    }
    if debounce > 0 {
        let rnd = fls(debounce) - 1;
        if rnd != 0 && (debounce & (1 << (rnd - 1))) != 0 { debounce = roundup(debounce, MEN_Z127_DB_MIN_US); }
        else { debounce = rounddown(debounce, MEN_Z127_DB_MIN_US); }
        if debounce > MEN_Z127_DB_MAX_US { debounce = MEN_Z127_DB_MAX_US; }
        debounce /= 50;
    }
    // Corresponds to guard(gpio_generic_lock)(&priv_->chip).
    let db_en_addr = (priv_.reg_base as *mut u8).add(MEN_Z127_DBER) as *mut core::ffi::c_void;
    let mut db_en = readl(db_en_addr);
    let db_cnt;
    if debounce == 0 { db_en &= !(1 << gpio); db_cnt = 0; }
    else { db_en |= 1 << gpio; db_cnt = debounce; }
    writel(db_en, db_en_addr);
    writel(db_cnt, (priv_.reg_base as *mut u8).add(gpio_to_dbcnt_reg(gpio as usize)) as *mut _);
    0
}

unsafe fn men_z127_set_single_ended(gc: *mut gpio_chip, offset: u32, param: pin_config_param) -> i32 {
    let priv_ = &mut *gpiochip_get_data(gc);
    let addr = (priv_.reg_base as *mut u8).add(MEN_Z127_ODER) as *mut _;
    let mut od_en = readl(addr);
    if matches!(param, pin_config_param::PIN_CONFIG_DRIVE_OPEN_DRAIN) { od_en |= 1 << offset; }
    else { od_en &= !(1 << offset); }
    writel(od_en, addr);
    0
}

unsafe extern "C" fn men_z127_set_config(gc: *mut gpio_chip, offset: u32, config: usize) -> i32 {
    match pinconf_to_config_param(config) {
        pin_config_param::PIN_CONFIG_DRIVE_OPEN_DRAIN | pin_config_param::PIN_CONFIG_DRIVE_PUSH_PULL => men_z127_set_single_ended(gc, offset, pinconf_to_config_param(config)),
        pin_config_param::PIN_CONFIG_INPUT_DEBOUNCE => men_z127_debounce(gc, offset, pinconf_to_config_argument(config)),
    }
}

unsafe extern "C" fn men_z127_release_mem(data: *mut core::ffi::c_void) { mcb_release_mem(data as *mut resource); }

unsafe extern "C" fn men_z127_probe(mdev: *mut mcb_device, _id: *const mcb_device_id) -> i32 {
    let dev = &mut (*mdev).dev as *mut device;
    let gpio = devm_kzalloc(dev, core::mem::size_of::<men_z127_gpio>(), 0);
    if gpio.is_null() { return -12; }
    (*gpio).mem = mcb_request_mem(mdev, dev_name(dev));
    if (*gpio).mem.is_null() { return dev_err_probe(dev, -1, b"failed to request device memory\0".as_ptr() as _); }
    let ret = devm_add_action_or_reset(dev, men_z127_release_mem, (*gpio).mem as *mut _);
    if ret != 0 { return ret; }
    (*gpio).reg_base = devm_ioremap(dev, (*gpio).mem).add(resource_size((*gpio).mem));
    if (*gpio).reg_base.is_null() { return -6; }
    mcb_set_drvdata(mdev, gpio);
    let sz = match (*mdev).id {
        MEN_Z127_ID => 4,
        MEN_Z034_ID | MEN_Z037_ID => 1,
        _ => return dev_err_probe(dev, -22, b"no size found for id %d\0".as_ptr() as _, (*mdev).id),
    };
    let config = gpio_generic_chip_config {
        dev,
        sz,
        dat: ((*gpio).reg_base as *mut u8).add(MEN_Z127_PSR),
        set: ((*gpio).reg_base as *mut u8).add(MEN_Z127_CTRL),
        dirout: ((*gpio).reg_base as *mut u8).add(MEN_Z127_GPIODR),
    };
    let ret = gpio_generic_chip_init(&mut (*gpio).chip, &mut { config });
    if ret != 0 { return ret; }
    // gpio_generic_chip embeds gpio_chip; the surrounding kernel bindings provide this field.
    let gc = (&mut (*gpio).chip as *mut gpio_generic_chip) as *mut gpio_chip;
    (*gc).set_config = Some(men_z127_set_config);
    let ret = devm_gpiochip_add_data(dev, gc, gpio);
    if ret != 0 { return dev_err_probe(dev, ret, b"failed to register MEN 16Z127 GPIO controller\0".as_ptr() as _); }
    0
}

#[repr(C)]
pub struct mcb_driver { pub probe: Option<unsafe extern "C" fn(*mut mcb_device, *const mcb_device_id) -> i32> }

#[no_mangle]
pub static men_z127_ids: [mcb_device_id; 4] = [
    mcb_device_id { device: MEN_Z127_ID },
    mcb_device_id { device: MEN_Z034_ID },
    mcb_device_id { device: MEN_Z037_ID },
    mcb_device_id { device: 0 },
];

#[no_mangle]
pub static mut men_z127_driver: mcb_driver = mcb_driver { probe: Some(men_z127_probe) };

// MODULE_DEVICE_TABLE(mcb, men_z127_ids);
// module_mcb_driver(men_z127_driver);
// MODULE_AUTHOR("Andreas Werner <andreas.werner@men.de>");
// MODULE_DESCRIPTION("MEN GPIO Controller");
// MODULE_LICENSE("GPL v2");
// MODULE_IMPORT_NS("MCB");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
