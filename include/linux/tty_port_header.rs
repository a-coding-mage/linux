/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/kfifo.h, linux/kref.h, linux/mutex.h, linux/tty_buffer.h,
// linux/tty_driver.h, and linux/wait.h.
use core::ffi::c_void;

type c_ulong = usize;

pub struct attribute_group;
pub struct tty_port;
pub struct tty_struct;

#[repr(C)]
pub struct tty_port_operations {
    pub carrier_raised: Option<unsafe extern "C" fn(port: *mut tty_port) -> bool>,
    pub dtr_rts: Option<unsafe extern "C" fn(port: *mut tty_port, active: bool)>,
    pub shutdown: Option<unsafe extern "C" fn(port: *mut tty_port)>,
    pub activate: Option<unsafe extern "C" fn(port: *mut tty_port, tty: *mut tty_struct) -> i32>,
    pub destruct: Option<unsafe extern "C" fn(port: *mut tty_port)>,
}

#[repr(C)]
pub struct tty_port_client_operations {
    pub receive_buf: Option<unsafe extern "C" fn(
        port: *mut tty_port,
        cp: *const u8,
        fp: *const u8,
        count: usize,
    ) -> usize>,
    pub lookahead_buf: Option<unsafe extern "C" fn(
        port: *mut tty_port,
        cp: *const u8,
        fp: *const u8,
        count: usize,
    )>,
    pub write_wakeup: Option<unsafe extern "C" fn(port: *mut tty_port)>,
}

extern "C" {
    pub static tty_port_default_client_ops: tty_port_client_operations;
}

// The following field types are supplied by the surrounding translation unit.
#[repr(C)]
pub struct tty_port {
    pub buf: tty_bufhead,
    pub tty: *mut tty_struct,
    pub itty: *mut tty_struct,
    pub ops: *const tty_port_operations,
    pub client_ops: *const tty_port_client_operations,
    pub lock: spinlock_t,
    pub blocked_open: i32,
    pub count: i32,
    pub open_wait: wait_queue_head_t,
    pub delta_msr_wait: wait_queue_head_t,
    pub flags: c_ulong,
    pub iflags: c_ulong,
    pub console: u8,
    pub mutex: mutex,
    pub buf_mutex: mutex,
    pub xmit_buf: *mut u8,
    pub xmit_fifo: kfifo_u8,
    pub close_delay: u32,
    pub closing_wait: u32,
    pub drain_delay: i32,
    pub kref: kref,
    pub client_data: *mut c_void,
}

pub const TTY_PORT_INITIALIZED: u32 = 0;
pub const TTY_PORT_SUSPENDED: u32 = 1;
pub const TTY_PORT_ACTIVE: u32 = 2;
pub const TTY_PORT_CTS_FLOW: u32 = 3;
pub const TTY_PORT_CHECK_CD: u32 = 4;
pub const TTY_PORT_KOPENED: u32 = 5;

extern "C" {
    pub fn tty_port_init(port: *mut tty_port);
    pub fn tty_port_link_wq(port: *mut tty_port, flip_wq: *mut workqueue_struct);
    pub fn tty_port_link_device(port: *mut tty_port, driver: *mut tty_driver, index: u32);
    pub fn tty_port_register_device(
        port: *mut tty_port,
        driver: *mut tty_driver,
        index: u32,
        device: *mut device,
    ) -> *mut device;
    pub fn tty_port_register_device_attr(
        port: *mut tty_port,
        driver: *mut tty_driver,
        index: u32,
        device: *mut device,
        drvdata: *mut c_void,
        attr_grp: *const *const attribute_group,
    ) -> *mut device;
    pub fn tty_port_register_device_attr_serdev(
        port: *mut tty_port,
        driver: *mut tty_driver,
        index: u32,
        host: *mut device,
        parent: *mut device,
        drvdata: *mut c_void,
        attr_grp: *const *const attribute_group,
    ) -> *mut device;
    pub fn tty_port_unregister_device(port: *mut tty_port, driver: *mut tty_driver, index: u32);
    pub fn tty_port_alloc_xmit_buf(port: *mut tty_port) -> i32;
    pub fn tty_port_free_xmit_buf(port: *mut tty_port);
    pub fn tty_port_destroy(port: *mut tty_port);
    pub fn tty_port_put(port: *mut tty_port);
    pub fn kref_get_unless_zero(kref: *mut kref) -> bool;
    pub fn test_bit(nr: u32, addr: *const c_ulong) -> bool;
    pub fn assign_bit(nr: u32, addr: *mut c_ulong, value: bool);
}

#[inline]
pub unsafe fn tty_port_get(port: *mut tty_port) -> *mut tty_port {
    if !port.is_null() && kref_get_unless_zero(&mut (*port).kref) {
        port
    } else {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn tty_port_link_driver_wq(port: *mut tty_port, driver: *mut tty_driver) {
    if (*port).buf.flip_wq.is_null() {
        tty_port_link_wq(port, (*driver).flip_wq);
    }
}

#[inline]
pub unsafe fn tty_port_cts_enabled(port: *const tty_port) -> bool {
    test_bit(TTY_PORT_CTS_FLOW, &(*port).iflags)
}

#[inline]
pub unsafe fn tty_port_set_cts_flow(port: *mut tty_port, val: bool) {
    assign_bit(TTY_PORT_CTS_FLOW, &mut (*port).iflags, val);
}

#[inline]
pub unsafe fn tty_port_active(port: *const tty_port) -> bool {
    test_bit(TTY_PORT_ACTIVE, &(*port).iflags)
}

#[inline]
pub unsafe fn tty_port_set_active(port: *mut tty_port, val: bool) {
    assign_bit(TTY_PORT_ACTIVE, &mut (*port).iflags, val);
}

#[inline]
pub unsafe fn tty_port_check_carrier(port: *const tty_port) -> bool {
    test_bit(TTY_PORT_CHECK_CD, &(*port).iflags)
}

#[inline]
pub unsafe fn tty_port_set_check_carrier(port: *mut tty_port, val: bool) {
    assign_bit(TTY_PORT_CHECK_CD, &mut (*port).iflags, val);
}

#[inline]
pub unsafe fn tty_port_suspended(port: *const tty_port) -> bool {
    test_bit(TTY_PORT_SUSPENDED, &(*port).iflags)
}

#[inline]
pub unsafe fn tty_port_set_suspended(port: *mut tty_port, val: bool) {
    assign_bit(TTY_PORT_SUSPENDED, &mut (*port).iflags, val);
}

#[inline]
pub unsafe fn tty_port_initialized(port: *const tty_port) -> bool {
    test_bit(TTY_PORT_INITIALIZED, &(*port).iflags)
}

#[inline]
pub unsafe fn tty_port_set_initialized(port: *mut tty_port, val: bool) {
    assign_bit(TTY_PORT_INITIALIZED, &mut (*port).iflags, val);
}

#[inline]
pub unsafe fn tty_port_kopened(port: *const tty_port) -> bool {
    test_bit(TTY_PORT_KOPENED, &(*port).iflags)
}

#[inline]
pub unsafe fn tty_port_set_kopened(port: *mut tty_port, val: bool) {
    assign_bit(TTY_PORT_KOPENED, &mut (*port).iflags, val);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
