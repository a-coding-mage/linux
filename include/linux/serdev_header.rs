/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of linux/serdev.h. */

use core::ffi::c_void;

#[repr(C)]
pub struct serdev_controller;
#[repr(C)]
pub struct serdev_device;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct device_driver;
#[repr(C)]
pub struct completion;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct module;
#[repr(C)]
pub struct tty_port;
#[repr(C)]
pub struct tty_driver;
#[repr(C)]
pub struct acpi_resource;
#[repr(C)]
pub struct acpi_resource_uart_serialbus;
#[repr(C)]
pub struct device_node;

#[repr(C)]
pub struct serdev_device_ops {
    pub receive_buf: Option<unsafe extern "C" fn(*mut serdev_device, *const u8, usize) -> usize>,
    pub write_wakeup: Option<unsafe extern "C" fn(*mut serdev_device)>,
}

#[repr(C)]
pub struct serdev_device {
    pub dev: device,
    pub nr: i32,
    pub ctrl: *mut serdev_controller,
    pub ops: *const serdev_device_ops,
    pub write_comp: completion,
    pub write_lock: mutex,
}

#[repr(C)]
pub struct serdev_device_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut serdev_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut serdev_device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut serdev_device)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum serdev_parity {
    SERDEV_PARITY_NONE,
    SERDEV_PARITY_EVEN,
    SERDEV_PARITY_ODD,
}

#[repr(C)]
pub struct serdev_controller_ops {
    pub write_buf: Option<unsafe extern "C" fn(*mut serdev_controller, *const u8, usize) -> isize>,
    pub write_flush: Option<unsafe extern "C" fn(*mut serdev_controller)>,
    pub open: Option<unsafe extern "C" fn(*mut serdev_controller) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut serdev_controller)>,
    pub set_flow_control: Option<unsafe extern "C" fn(*mut serdev_controller, bool)>,
    pub set_parity: Option<unsafe extern "C" fn(*mut serdev_controller, serdev_parity) -> i32>,
    pub set_baudrate: Option<unsafe extern "C" fn(*mut serdev_controller, u32) -> u32>,
    pub wait_until_sent: Option<unsafe extern "C" fn(*mut serdev_controller, i64)>,
    pub get_tiocm: Option<unsafe extern "C" fn(*mut serdev_controller) -> i32>,
    pub set_tiocm: Option<unsafe extern "C" fn(*mut serdev_controller, u32, u32) -> i32>,
    pub break_ctl: Option<unsafe extern "C" fn(*mut serdev_controller, u32) -> i32>,
}

#[repr(C)]
pub struct serdev_controller {
    pub dev: device,
    pub host: *mut device,
    pub nr: u32,
    pub serdev: *mut serdev_device,
    pub ops: *const serdev_controller_ops,
}

extern "C" {
    pub fn dev_get_drvdata(dev: *const device) -> *mut c_void;
    pub fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    pub fn put_device(dev: *mut device);
    pub fn driver_unregister(driver: *mut device_driver);
    pub fn serdev_device_alloc(ctrl: *mut serdev_controller) -> *mut serdev_device;
    pub fn serdev_device_add(dev: *mut serdev_device) -> i32;
    pub fn serdev_device_remove(dev: *mut serdev_device);
    pub fn serdev_controller_alloc(host: *mut device, parent: *mut device, size: usize) -> *mut serdev_controller;
    pub fn serdev_controller_add(ctrl: *mut serdev_controller) -> i32;
    pub fn serdev_controller_remove(ctrl: *mut serdev_controller);
}

#[inline]
pub unsafe fn serdev_device_get_drvdata(serdev: *const serdev_device) -> *mut c_void {
    dev_get_drvdata(core::ptr::addr_of!((*serdev).dev))
}
#[inline]
pub unsafe fn serdev_device_set_drvdata(serdev: *mut serdev_device, data: *mut c_void) {
    dev_set_drvdata(core::ptr::addr_of_mut!((*serdev).dev), data)
}
#[inline]
pub unsafe fn serdev_device_put(serdev: *mut serdev_device) { if !serdev.is_null() { put_device(core::ptr::addr_of_mut!((*serdev).dev)); } }
#[inline]
pub unsafe fn serdev_device_set_client_ops(serdev: *mut serdev_device, ops: *const serdev_device_ops) { (*serdev).ops = ops; }
#[inline]
pub unsafe fn serdev_controller_get_drvdata(ctrl: *const serdev_controller) -> *mut c_void {
    if ctrl.is_null() { core::ptr::null_mut() } else { dev_get_drvdata(core::ptr::addr_of!((*ctrl).dev)) }
}
#[inline]
pub unsafe fn serdev_controller_set_drvdata(ctrl: *mut serdev_controller, data: *mut c_void) { dev_set_drvdata(core::ptr::addr_of_mut!((*ctrl).dev), data); }
#[inline]
pub unsafe fn serdev_controller_put(ctrl: *mut serdev_controller) { if !ctrl.is_null() { put_device(core::ptr::addr_of_mut!((*ctrl).dev)); } }

#[inline]
pub unsafe fn serdev_controller_write_wakeup(ctrl: *mut serdev_controller) {
    let serdev = (*ctrl).serdev;
    if serdev.is_null() || (*serdev).ops.is_null() || (*(*serdev).ops).write_wakeup.is_none() { return; }
    ((*(*serdev).ops).write_wakeup.unwrap())(serdev);
}
#[inline]
pub unsafe fn serdev_controller_receive_buf(ctrl: *mut serdev_controller, data: *const u8, count: usize) -> usize {
    let serdev = (*ctrl).serdev;
    if serdev.is_null() || (*serdev).ops.is_null() || (*(*serdev).ops).receive_buf.is_none() { return 0; }
    ((*(*serdev).ops).receive_buf.unwrap())(serdev, data, count)
}

extern "C" {
    pub fn serdev_device_open(dev: *mut serdev_device) -> i32;
    pub fn serdev_device_close(dev: *mut serdev_device);
    pub fn devm_serdev_device_open(dev: *mut device, serdev: *mut serdev_device) -> i32;
    pub fn serdev_device_set_baudrate(dev: *mut serdev_device, baudrate: u32) -> u32;
    pub fn serdev_device_set_flow_control(dev: *mut serdev_device, enable: bool);
    pub fn serdev_device_write_buf(dev: *mut serdev_device, buf: *const u8, count: usize) -> i32;
    pub fn serdev_device_wait_until_sent(dev: *mut serdev_device, timeout: i64);
    pub fn serdev_device_get_tiocm(dev: *mut serdev_device) -> i32;
    pub fn serdev_device_set_tiocm(dev: *mut serdev_device, set: i32, clear: i32) -> i32;
    pub fn serdev_device_break_ctl(dev: *mut serdev_device, break_state: i32) -> i32;
    pub fn serdev_device_write_wakeup(dev: *mut serdev_device);
    pub fn serdev_device_write(dev: *mut serdev_device, buf: *const u8, count: usize, timeout: u64) -> isize;
    pub fn serdev_device_write_flush(dev: *mut serdev_device);
    pub fn serdev_device_set_parity(dev: *mut serdev_device, parity: serdev_parity) -> i32;
    pub fn __serdev_device_driver_register(driver: *mut serdev_device_driver, module: *mut module) -> i32;
}

#[inline]
pub unsafe fn serdev_device_driver_unregister(sdrv: *mut serdev_device_driver) { if !sdrv.is_null() { driver_unregister(core::ptr::addr_of_mut!((*sdrv).driver)); } }

#[inline]
pub unsafe fn serdev_device_get_cts(serdev: *mut serdev_device) -> bool {
    (serdev_device_get_tiocm(serdev) & TIOCM_CTS) != 0
}
#[inline]
pub unsafe fn serdev_device_set_rts(serdev: *mut serdev_device, enable: bool) -> i32 {
    if enable { serdev_device_set_tiocm(serdev, TIOCM_RTS, 0) } else { serdev_device_set_tiocm(serdev, 0, TIOCM_RTS) }
}

pub const TIOCM_CTS: i32 = 0x020;
pub const TIOCM_RTS: i32 = 0x004;

extern "C" {
    pub fn serdev_tty_port_register(port: *mut tty_port, host: *mut device, parent: *mut device, drv: *mut tty_driver, idx: i32) -> *mut device;
    pub fn serdev_tty_port_unregister(port: *mut tty_port) -> i32;
    pub fn serdev_acpi_get_uart_resource(ares: *mut acpi_resource, uart: *mut *mut acpi_resource_uart_serialbus) -> bool;
    pub fn of_find_serdev_controller_by_node(node: *mut device_node) -> *mut serdev_controller;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
