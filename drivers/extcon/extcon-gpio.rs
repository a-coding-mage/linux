// SPDX-License-Identifier: GPL-2.0-only
/*
 * extcon_gpio.c - Single-state GPIO extcon driver based on extcon class
 *
 * Copyright (C) 2008 Google, Inc.
 * Author: Mike Lockwood <lockwood@android.com>
 *
 * Modified by MyungJoo Ham <myungjoo.ham@samsung.com> to support extcon
 * (originally switch class is supported)
 */

// External Linux kernel types, constants, and functions are supplied by the
// surrounding kernel bindings.
use core::ffi::c_void;

#[repr(C)]
pub struct extcon_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub name: *const u8,
    _private: [u8; 0],
}

pub type irqreturn_t = i32;
pub type irq_handler_t = unsafe extern "C" fn(i32, *mut c_void) -> irqreturn_t;

extern "C" {
    static mut system_power_efficient_wq: *mut c_void;
    fn gpiod_get_value_cansleep(gpiod: *mut gpio_desc) -> i32;
    fn extcon_set_state_sync(edev: *mut extcon_dev, id: u32, state: i32) -> i32;
    fn to_delayed_work(work: *mut work_struct) -> *mut delayed_work;
    fn gpiod_to_irq(gpiod: *mut gpio_desc) -> i32;
    fn gpiod_is_active_low(gpiod: *mut gpio_desc) -> bool;
    fn devm_gpiod_get(dev: *mut device, con_id: *const u8, flags: u32) -> *mut gpio_desc;
    fn devm_extcon_dev_allocate(dev: *mut device, ids: *const u32) -> *mut extcon_dev;
    fn devm_extcon_dev_register(dev: *mut device, edev: *mut extcon_dev) -> i32;
    fn devm_delayed_work_autocancel(
        dev: *mut device,
        work: *mut delayed_work,
        func: unsafe extern "C" fn(*mut work_struct),
    ) -> i32;
    fn devm_request_any_context_irq(
        dev: *mut device,
        irq: i32,
        handler: irq_handler_t,
        flags: u32,
        name: *const u8,
        data: *mut c_void,
    ) -> i32;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn queue_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: usize) -> bool;
}

const EXTCON_NONE: u32 = 0;
const GPIOD_IN: u32 = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_TRIGGER_FALLING: u32 = 0x0000_0002;
const IRQF_TRIGGER_RISING: u32 = 0x0000_0001;

#[repr(C)]
pub struct gpio_extcon_data {
    pub edev: *mut extcon_dev,
    pub work: delayed_work,
    pub debounce_jiffies: usize,
    pub gpiod: *mut gpio_desc,
    pub extcon_id: u32,
    pub debounce: usize,
    pub check_on_resume: bool,
}

unsafe extern "C" fn gpio_extcon_work(work: *mut work_struct) {
    let data = (work as *mut u8)
        .sub(core::mem::offset_of!(gpio_extcon_data, work))
        as *mut gpio_extcon_data;
    let state = gpiod_get_value_cansleep((*data).gpiod);
    extcon_set_state_sync((*data).edev, (*data).extcon_id, state);
}

unsafe extern "C" fn gpio_irq_handler(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let data = dev_id as *mut gpio_extcon_data;
    queue_delayed_work(
        system_power_efficient_wq,
        &mut (*data).work,
        (*data).debounce_jiffies,
    );
    IRQ_HANDLED
}

unsafe extern "C" fn gpio_extcon_probe(pdev: *mut platform_device) -> i32 {
    let data = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<gpio_extcon_data>(), 0);
    if data.is_null() {
        return -12;
    }
    let data = data as *mut gpio_extcon_data;
    let dev = &mut (*pdev).dev as *mut device;

    if (*data).extcon_id > EXTCON_NONE {
        return -22;
    }
    (*data).gpiod = devm_gpiod_get(dev, b"extcon\0".as_ptr(), GPIOD_IN);
    let irq = gpiod_to_irq((*data).gpiod);
    if irq <= 0 {
        return irq;
    }
    let irq_flags = if gpiod_is_active_low((*data).gpiod) {
        IRQF_TRIGGER_FALLING
    } else {
        IRQF_TRIGGER_RISING
    };
    (*data).edev = devm_extcon_dev_allocate(dev, &(*data).extcon_id);
    if (*data).edev.is_null() {
        return -12;
    }
    let mut ret = devm_extcon_dev_register(dev, (*data).edev);
    if ret < 0 { return ret; }
    ret = devm_delayed_work_autocancel(dev, &mut (*data).work, gpio_extcon_work);
    if ret != 0 { return ret; }
    ret = devm_request_any_context_irq(dev, irq, gpio_irq_handler, irq_flags,
        (*pdev).name, data as *mut c_void);
    if ret < 0 { return ret; }
    platform_set_drvdata(pdev, data as *mut c_void);
    gpio_extcon_work(&mut (*data).work.work);
    0
}

// CONFIG_PM_SLEEP: resume queues a debounced state check when requested.
unsafe extern "C" fn gpio_extcon_resume(dev: *mut device) -> i32 {
    let data = dev_get_drvdata(dev) as *mut gpio_extcon_data;
    if (*data).check_on_resume {
        queue_delayed_work(system_power_efficient_wq, &mut (*data).work,
            (*data).debounce_jiffies);
    }
    0
}

#[repr(C)]
pub struct device_driver {
    pub name: *const u8,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub driver: device_driver,
}

static gpio_extcon_pm_ops: [u8; 0] = [];

static mut gpio_extcon_driver: platform_driver = platform_driver {
    probe: Some(gpio_extcon_probe),
    driver: device_driver {
        name: b"extcon-gpio\0".as_ptr(),
        pm: gpio_extcon_pm_ops.as_ptr() as *const c_void,
    },
};

// module_platform_driver(gpio_extcon_driver);
// MODULE_AUTHOR("Mike Lockwood <lockwood@android.com>");
// MODULE_DESCRIPTION("GPIO extcon driver");
// MODULE_LICENSE("GPL");

extern "C" { fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
