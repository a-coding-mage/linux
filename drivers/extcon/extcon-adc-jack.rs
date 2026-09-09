// SPDX-License-Identifier: GPL-2.0-only
/*
 * drivers/extcon/extcon-adc-jack.c
 *
 * Analog Jack extcon driver with ADC-based detection capability.
 *
 * Copyright (C) 2016 Samsung Electronics
 * Chanwoo Choi <cw00.choi@samsung.com>
 *
 * Copyright (C) 2012 Samsung Electronics
 * MyungJoo Ham <myungjoo.ham@samsung.com>
 *
 * Modified for calling to IIO to get adc by <anish.singh@samsung.com>
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::c_void;

#[repr(C)]
pub struct adc_jack_data {
    pub dev: *mut device,
    pub edev: *mut extcon_dev,
    pub cable_names: *const *const u32,
    pub adc_conditions: *mut adc_jack_cond,
    pub num_conditions: i32,
    pub irq: i32,
    pub handling_delay: usize, // in jiffies
    pub handler: delayed_work,
    pub chan: *mut iio_channel,
    pub wakeup_source: bool,
}

#[repr(C)]
pub struct adc_jack_cond {
    pub id: u32,
    pub min_adc: i32,
    pub max_adc: i32,
}

#[repr(C)]
pub struct adc_jack_pdata {
    pub cable_names: *const *const u32,
    pub adc_conditions: *mut adc_jack_cond,
    pub consumer_channel: *const i8,
    pub handling_delay_ms: u32,
    pub wakeup_source: bool,
    pub irq_flags: u64,
    pub name: *const i8,
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct extcon_dev { _private: [u8; 0] }
#[repr(C)] pub struct iio_channel { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }

type irqreturn_t = i32;

const IRQ_HANDLED: irqreturn_t = 1;
const EXTCON_NONE: u32 = !0;

extern "C" {
    static mut system_power_efficient_wq: *mut c_void;
    fn dev_get_platdata(dev: *mut device) -> *mut adc_jack_pdata;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn devm_extcon_dev_allocate(dev: *mut device, names: *const *const u32) -> *mut extcon_dev;
    fn devm_extcon_dev_register(dev: *mut device, edev: *mut extcon_dev) -> i32;
    fn devm_iio_channel_get(dev: *mut device, name: *const i8) -> *mut iio_channel;
    fn iio_read_channel_raw(chan: *mut iio_channel, val: *mut i32) -> i32;
    fn extcon_set_state_sync(edev: *mut extcon_dev, id: u32, state: bool) -> i32;
    fn msecs_to_jiffies(ms: u32) -> usize;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut adc_jack_data);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut adc_jack_data;
    fn platform_get_irq(pdev: *mut platform_device, index: u32) -> i32;
    fn request_any_context_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut c_void) -> irqreturn_t, flags: u64, name: *const i8, data: *mut adc_jack_data) -> i32;
    fn free_irq(irq: i32, data: *mut adc_jack_data);
    fn device_init_wakeup(dev: *mut device, enable: bool) -> i32;
    fn device_may_wakeup(dev: *mut device) -> bool;
    fn enable_irq_wake(irq: i32) -> i32;
    fn disable_irq_wake(irq: i32) -> i32;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn queue_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: usize) -> bool;
    fn init_deferrable_work(work: *mut delayed_work, handler: unsafe extern "C" fn(*mut work_struct));
}

unsafe extern "C" fn adc_jack_handler(work: *mut work_struct) {
    // container_of(to_delayed_work(work), struct adc_jack_data, handler)
    let data = work as *mut adc_jack_data;
    let mut adc_val: i32 = 0;
    let ret = iio_read_channel_raw((*data).chan, &mut adc_val);
    if ret < 0 { return; }

    for i in 0..(*data).num_conditions {
        let def = &*(*data).adc_conditions.add(i as usize);
        if def.min_adc <= adc_val && def.max_adc >= adc_val {
            extcon_set_state_sync((*data).edev, def.id, true);
            return;
        }
    }
    for i in 0..(*data).num_conditions {
        let def = &*(*data).adc_conditions.add(i as usize);
        extcon_set_state_sync((*data).edev, def.id, false);
    }
}

unsafe extern "C" fn adc_jack_irq_thread(_irq: i32, data: *mut c_void) -> irqreturn_t {
    let data = data as *mut adc_jack_data;
    queue_delayed_work(system_power_efficient_wq, &mut (*data).handler, (*data).handling_delay);
    IRQ_HANDLED
}

unsafe extern "C" fn adc_jack_probe(pdev: *mut platform_device) -> i32 {
    let dev = pdev as *mut device;
    let pdata = dev_get_platdata(dev);
    let data = devm_kzalloc(dev, core::mem::size_of::<adc_jack_data>(), 0) as *mut adc_jack_data;
    if data.is_null() { return -12; } // -ENOMEM
    if (*pdata).cable_names.is_null() { return -22; } // -EINVAL
    (*data).dev = dev;
    (*data).edev = devm_extcon_dev_allocate(dev, (*pdata).cable_names);
    if (*data).edev.is_null() { return -12; }
    if (*pdata).adc_conditions.is_null() { return -22; }
    (*data).adc_conditions = (*pdata).adc_conditions;
    let mut i = 0;
    while (*data).adc_conditions.add(i).id != EXTCON_NONE { i += 1; }
    (*data).num_conditions = i as i32;
    (*data).chan = devm_iio_channel_get(dev, (*pdata).consumer_channel);
    if (*data).chan.is_null() { return -19; }
    (*data).handling_delay = msecs_to_jiffies((*pdata).handling_delay_ms);
    (*data).wakeup_source = (*pdata).wakeup_source;
    init_deferrable_work(&mut (*data).handler, adc_jack_handler);
    platform_set_drvdata(pdev, data);
    let err = devm_extcon_dev_register(dev, (*data).edev);
    if err != 0 { return err; }
    (*data).irq = platform_get_irq(pdev, 0);
    if (*data).irq < 0 { return -19; }
    let err = request_any_context_irq((*data).irq, adc_jack_irq_thread, (*pdata).irq_flags, (*pdata).name, data);
    if err < 0 { return err; }
    if (*data).wakeup_source { device_init_wakeup(dev, true); }
    adc_jack_handler(&mut (*data).handler.work);
    0
}

unsafe extern "C" fn adc_jack_remove(pdev: *mut platform_device) {
    let data = platform_get_drvdata(pdev);
    if (*data).wakeup_source { device_init_wakeup((*data).dev, false); }
    free_irq((*data).irq, data);
    cancel_work_sync(&mut (*data).handler.work);
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe extern "C" fn adc_jack_suspend(dev: *mut device) -> i32 {
    let data = dev_get_drvdata(dev);
    cancel_delayed_work_sync(&mut (*data).handler);
    if device_may_wakeup((*data).dev) { enable_irq_wake((*data).irq); }
    0
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe extern "C" fn adc_jack_resume(dev: *mut device) -> i32 {
    let data = dev_get_drvdata(dev);
    if device_may_wakeup((*data).dev) { disable_irq_wake((*data).irq); }
    0
}

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut adc_jack_data;
}

// SIMPLE_DEV_PM_OPS(adc_jack_pm_ops, adc_jack_suspend, adc_jack_resume)
// static struct platform_driver adc_jack_driver = {
//     .probe = adc_jack_probe, .remove = adc_jack_remove,
//     .driver = { .name = "adc-jack", .pm = &adc_jack_pm_ops },
// };
// module_platform_driver(adc_jack_driver)
// MODULE_AUTHOR("MyungJoo Ham <myungjoo.ham@samsung.com>");
// MODULE_DESCRIPTION("ADC Jack extcon driver");
// MODULE_LICENSE("GPL v2");
// MODULE_IMPORT_NS("IIO_CONSUMER");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
