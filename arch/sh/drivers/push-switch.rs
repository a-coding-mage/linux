// SPDX-License-Identifier: GPL-2.0
/*
 * Generic push-switch framework
 *
 * Copyright (C) 2006  Paul Mundt
 */

// Linux kernel dependencies supplied by other translation units.

const DRV_NAME: *const u8 = b"push-switch\0".as_ptr();
const DRV_VERSION: *const u8 = b"0.1.1\0".as_ptr();

unsafe extern "C" {
    fn sprintf(buf: *mut u8, format: *const u8, ...) -> isize;
    fn schedule_work(work: *mut work_struct);
    fn kobject_uevent(kobj: *mut kobject, action: u32) -> i32;
    fn platform_get_irq(pdev: *mut platform_device, index: u32) -> i32;
    fn request_irq(
        irq: i32,
        handler: Option<unsafe extern "C" fn()>,
        flags: u32,
        name: *const u8,
        dev: *mut platform_device,
    ) -> i32;
    fn device_create_file(dev: *mut device, attr: *const device_attribute) -> i32;
    fn free_irq(irq: i32, dev: *mut platform_device);
    fn device_remove_file(dev: *mut device, attr: *const device_attribute);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut push_switch);
    fn timer_shutdown_sync(timer: *mut timer_list);
    fn flush_work(work: *mut work_struct);
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn printk(format: *const u8, ...);
    fn kzalloc(size: usize, flags: u32) -> *mut push_switch;
    fn kfree(ptr: *mut push_switch);
}

#[repr(C)]
struct device {
    platform_data: *mut core::ffi::c_void,
    kobj: kobject,
}

#[repr(C)]
struct kobject;
#[repr(C)]
struct device_attribute;
#[repr(C)]
struct work_struct;
#[repr(C)]
struct timer_list;

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: driver,
}

#[repr(C)]
struct driver {
    name: *const u8,
}

#[repr(C)]
struct push_switch_platform_info {
    name: *const u8,
    irq_handler: Option<unsafe extern "C" fn()>,
    irq_flags: u32,
}

#[repr(C)]
struct push_switch {
    work: work_struct,
    debounce: timer_list,
    pdev: *mut platform_device,
    state: i32,
}

unsafe fn switch_show(
    dev: *mut device,
    _attr: *mut device_attribute,
    buf: *mut u8,
) -> isize {
    let psw_info = (*dev).platform_data as *mut push_switch_platform_info;
    sprintf(buf, b"%s\n\0".as_ptr(), (*psw_info).name)
}

static mut DEV_ATTR_SWITCH: device_attribute = device_attribute;

unsafe extern "C" fn switch_timer(t: *mut timer_list) {
    // timer_container_of(psw, t, debounce)
    let psw = (t as *mut u8).sub(core::mem::offset_of!(push_switch, debounce))
        as *mut push_switch;
    schedule_work(&mut (*psw).work);
}

unsafe extern "C" fn switch_work_handler(work: *mut work_struct) {
    let psw = (work as *mut u8).sub(core::mem::offset_of!(push_switch, work)) as *mut push_switch;
    let pdev = (*psw).pdev;

    (*psw).state = 0;
    kobject_uevent(&mut (*pdev).dev.kobj, 1);
}

unsafe extern "C" fn switch_drv_probe(pdev: *mut platform_device) -> i32 {
    let mut psw = kzalloc(core::mem::size_of::<push_switch>(), 0);
    if psw.is_null() {
        return -12;
    }

    let irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        kfree(psw);
        return -19;
    }

    let psw_info = (*pdev).dev.platform_data as *mut push_switch_platform_info;
    if psw_info.is_null() {
        kfree(psw);
        core::hint::unreachable_unchecked();
    }

    let name = if !(*psw_info).name.is_null() { (*psw_info).name } else { DRV_NAME };
    let mut ret = request_irq(irq, (*psw_info).irq_handler, (*psw_info).irq_flags, name, pdev);
    if ret < 0 {
        kfree(psw);
        return ret;
    }

    if !(*psw_info).name.is_null() {
        ret = device_create_file(&mut (*pdev).dev, &raw const DEV_ATTR_SWITCH);
        if ret != 0 {
            printk(b"Failed creating device attrs\n\0".as_ptr());
            free_irq(irq, pdev);
            kfree(psw);
            return -22;
        }
    }

    // INIT_WORK and timer_setup are supplied by the kernel translation layer.
    (*psw).pdev = pdev;
    platform_set_drvdata(pdev, psw);
    0
}

unsafe extern "C" fn switch_drv_remove(pdev: *mut platform_device) {
    let psw = platform_get_drvdata(pdev);
    let psw_info = (*pdev).dev.platform_data as *mut push_switch_platform_info;
    let irq = platform_get_irq(pdev, 0);

    if !(*psw_info).name.is_null() {
        device_remove_file(&mut (*pdev).dev, &raw const DEV_ATTR_SWITCH);
    }

    platform_set_drvdata(pdev, core::ptr::null_mut());
    timer_shutdown_sync(&mut (*psw).debounce);
    flush_work(&mut (*psw).work);
    free_irq(irq, pdev);
    kfree(psw);
}

static mut SWITCH_DRIVER: platform_driver = platform_driver {
    probe: Some(switch_drv_probe),
    remove: Some(switch_drv_remove),
    driver: driver { name: DRV_NAME },
};

unsafe extern "C" fn switch_init() -> i32 {
    printk(b"push-switch: version %s loaded\n\0".as_ptr(), DRV_VERSION);
    platform_driver_register(&raw mut SWITCH_DRIVER)
}

unsafe extern "C" fn switch_exit() {
    platform_driver_unregister(&raw mut SWITCH_DRIVER);
}

// module_init(switch_init);
// module_exit(switch_exit);
// MODULE_VERSION(DRV_VERSION);
// MODULE_AUTHOR("Paul Mundt");
// MODULE_DESCRIPTION("Generic push-switch framework");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
