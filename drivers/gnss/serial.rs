// SPDX-License-Identifier: GPL-2.0
/*
 * Generic serial GNSS receiver driver
 *
 * Copyright (C) 2018 Johan Hovold <johan@kernel.org>
 */

use core::ffi::{c_char, c_int, c_void};

/* Kernel headers and "serial.h" provide the declarations used below. */

extern "C" {
    fn gnss_get_drvdata(gdev: *mut gnss_device) -> *mut gnss_serial;
    fn serdev_device_open(serdev: *mut serdev_device) -> c_int;
    fn serdev_device_set_baudrate(serdev: *mut serdev_device, speed: u32);
    fn serdev_device_set_flow_control(serdev: *mut serdev_device, enable: bool);
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_noidle(dev: *mut device);
    fn serdev_device_close(serdev: *mut serdev_device);
    fn pm_runtime_put(dev: *mut device);
    fn serdev_device_write(serdev: *mut serdev_device, buf: *const u8, count: usize, timeout: c_int) -> c_int;
    fn serdev_device_wait_until_sent(serdev: *mut serdev_device, timeout: c_int);
    fn serdev_device_get_drvdata(serdev: *mut serdev_device) -> *mut gnss_serial;
    fn gnss_insert_raw(gdev: *mut gnss_device, buf: *const u8, count: usize) -> usize;
    fn serdev_device_write_wakeup(serdev: *mut serdev_device);
    fn of_property_read_u32(node: *mut device_node, name: *const c_char, value: *mut u32) -> c_int;
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn gnss_allocate_device(dev: *mut device) -> *mut gnss_device;
    fn gnss_set_drvdata(gdev: *mut gnss_device, data: *mut gnss_serial);
    fn serdev_device_set_drvdata(serdev: *mut serdev_device, data: *mut gnss_serial);
    fn serdev_device_set_client_ops(serdev: *mut serdev_device, ops: *const serdev_device_ops);
    fn gnss_put_device(gdev: *mut gnss_device);
    fn gnss_register_device(gdev: *mut gnss_device) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn gnss_deregister_device(gdev: *mut gnss_device);
    fn dev_get_drvdata(dev: *mut device) -> *mut gnss_serial;
    fn pm_runtime_suspended(dev: *mut device) -> bool;
}

const ENOMEM: c_int = 12;
const MAX_SCHEDULE_TIMEOUT: c_int = c_int::MAX;
const GFP_KERNEL: c_int = 0;

unsafe extern "C" fn gnss_serial_open(gdev: *mut gnss_device) -> c_int {
    let gserial = gnss_get_drvdata(gdev);
    let serdev = (*gserial).serdev;
    let mut ret = serdev_device_open(serdev);
    if ret != 0 { return ret; }
    serdev_device_set_baudrate(serdev, (*gserial).speed);
    serdev_device_set_flow_control(serdev, false);
    ret = pm_runtime_get_sync(&mut (*serdev).dev);
    if ret < 0 {
        pm_runtime_put_noidle(&mut (*serdev).dev);
        serdev_device_close(serdev);
        return ret;
    }
    0
}

unsafe extern "C" fn gnss_serial_close(gdev: *mut gnss_device) {
    let gserial = gnss_get_drvdata(gdev);
    let serdev = (*gserial).serdev;
    serdev_device_close(serdev);
    pm_runtime_put(&mut (*serdev).dev);
}

unsafe extern "C" fn gnss_serial_write_raw(gdev: *mut gnss_device, buf: *const u8, count: usize) -> c_int {
    let serdev = (*gnss_get_drvdata(gdev)).serdev;
    let ret = serdev_device_write(serdev, buf, count, MAX_SCHEDULE_TIMEOUT);
    if ret < 0 || (ret as usize) < count { return ret; }
    serdev_device_wait_until_sent(serdev, 0);
    count as c_int
}

static GNSS_SERIAL_GNSS_OPS: gnss_operations = gnss_operations {
    open: Some(gnss_serial_open), close: Some(gnss_serial_close), write_raw: Some(gnss_serial_write_raw),
};

unsafe extern "C" fn gnss_serial_receive_buf(serdev: *mut serdev_device, buf: *const u8, count: usize) -> usize {
    gnss_insert_raw((*serdev_device_get_drvdata(serdev)).gdev, buf, count)
}

static GNSS_SERIAL_SERDEV_OPS: serdev_device_ops = serdev_device_ops {
    receive_buf: Some(gnss_serial_receive_buf), write_wakeup: Some(serdev_device_write_wakeup),
};

unsafe fn gnss_serial_set_power(gserial: *mut gnss_serial, state: gnss_serial_pm_state) -> c_int {
    match (*gserial).ops {
        Some(ops) if !ops.set_power.is_none() => (ops.set_power.unwrap())(gserial, state),
        _ => 0,
    }
}

unsafe fn gnss_serial_parse_dt(serdev: *mut serdev_device) -> c_int {
    let gserial = serdev_device_get_drvdata(serdev);
    let node = (*serdev).dev.of_node;
    let mut speed = 4800u32;
    of_property_read_u32(node, b"current-speed\0".as_ptr() as *const c_char, &mut speed);
    (*gserial).speed = speed;
    0
}

#[no_mangle]
pub unsafe extern "C" fn gnss_serial_allocate(serdev: *mut serdev_device, data_size: usize) -> *mut gnss_serial {
    let gserial = kzalloc(core::mem::size_of::<gnss_serial>() + data_size, GFP_KERNEL) as *mut gnss_serial;
    if gserial.is_null() { return (-ENOMEM) as isize as *mut gnss_serial; }
    let gdev = gnss_allocate_device(&mut (*serdev).dev);
    if gdev.is_null() { kfree(gserial as *mut c_void); return (-ENOMEM) as isize as *mut gnss_serial; }
    (*gdev).ops = &GNSS_SERIAL_GNSS_OPS;
    gnss_set_drvdata(gdev, gserial);
    (*gserial).serdev = serdev; (*gserial).gdev = gdev;
    serdev_device_set_drvdata(serdev, gserial);
    serdev_device_set_client_ops(serdev, &GNSS_SERIAL_SERDEV_OPS);
    let ret = gnss_serial_parse_dt(serdev);
    if ret != 0 { gnss_put_device((*gserial).gdev); kfree(gserial as *mut c_void); return (ret as isize) as *mut gnss_serial; }
    gserial
}

#[no_mangle]
pub unsafe extern "C" fn gnss_serial_free(gserial: *mut gnss_serial) { gnss_put_device((*gserial).gdev); kfree(gserial as *mut c_void); }

#[no_mangle]
pub unsafe extern "C" fn gnss_serial_register(gserial: *mut gnss_serial) -> c_int {
    let serdev = (*gserial).serdev;
    let ret;
    /* CONFIG_PM selects the runtime-PM path at build time. */
    if cfg!(feature = "CONFIG_PM") {
        pm_runtime_enable(&mut (*serdev).dev);
    } else {
        ret = gnss_serial_set_power(gserial, GNSS_SERIAL_ACTIVE);
        if ret < 0 { return ret; }
    }
    ret = gnss_register_device((*gserial).gdev);
    if ret != 0 {
        if cfg!(feature = "CONFIG_PM") { pm_runtime_disable(&mut (*serdev).dev); }
        else { gnss_serial_set_power(gserial, GNSS_SERIAL_OFF); }
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn gnss_serial_deregister(gserial: *mut gnss_serial) {
    let serdev = (*gserial).serdev;
    gnss_deregister_device((*gserial).gdev);
    if cfg!(feature = "CONFIG_PM") { pm_runtime_disable(&mut (*serdev).dev); }
    else { gnss_serial_set_power(gserial, GNSS_SERIAL_OFF); }
}

#[cfg(feature = "CONFIG_PM")]
unsafe extern "C" fn gnss_serial_runtime_suspend(dev: *mut device) -> c_int {
    gnss_serial_set_power(dev_get_drvdata(dev), GNSS_SERIAL_STANDBY)
}

#[cfg(feature = "CONFIG_PM")]
unsafe extern "C" fn gnss_serial_runtime_resume(dev: *mut device) -> c_int {
    gnss_serial_set_power(dev_get_drvdata(dev), GNSS_SERIAL_ACTIVE)
}

unsafe extern "C" fn gnss_serial_prepare(dev: *mut device) -> c_int {
    if pm_runtime_suspended(dev) { 1 } else { 0 }
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe extern "C" fn gnss_serial_suspend(dev: *mut device) -> c_int {
    let gserial = dev_get_drvdata(dev);
    if pm_runtime_suspended(dev) { 0 } else { gnss_serial_set_power(gserial, GNSS_SERIAL_STANDBY) }
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe extern "C" fn gnss_serial_resume(dev: *mut device) -> c_int {
    let gserial = dev_get_drvdata(dev);
    if pm_runtime_suspended(dev) { 0 } else { gnss_serial_set_power(gserial, GNSS_SERIAL_ACTIVE) }
}

/* SET_SYSTEM_SLEEP_PM_OPS and SET_RUNTIME_PM_OPS are represented by the
 * corresponding fields of the dependency-provided dev_pm_ops type. */
#[no_mangle]
pub static gnss_serial_pm_ops: dev_pm_ops = dev_pm_ops {
    prepare: Some(gnss_serial_prepare),
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    suspend: Some(gnss_serial_suspend),
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    resume: Some(gnss_serial_resume),
    #[cfg(feature = "CONFIG_PM")]
    runtime_suspend: Some(gnss_serial_runtime_suspend),
    #[cfg(feature = "CONFIG_PM")]
    runtime_resume: Some(gnss_serial_runtime_resume),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
