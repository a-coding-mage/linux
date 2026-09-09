// SPDX-License-Identifier: GPL-2.0
/* USB Serial Converter stuff */

// C header dependencies are supplied by other translation units.

pub const MAX_NUM_PORTS: usize = 16;
pub const USB_SERIAL_WRITE_BUSY: u32 = 0;
pub const USB_SERIAL_THROTTLED: u32 = 1;

#[repr(C)]
pub struct usb_serial_port {
    pub serial: *mut usb_serial,
    pub port: tty_port,
    pub lock: spinlock_t,
    pub minor: u32,
    pub port_number: u8,
    pub interrupt_in_buffer: *mut u8,
    pub interrupt_in_urb: *mut urb,
    pub interrupt_in_endpointAddress: u8,
    pub interrupt_out_buffer: *mut u8,
    pub interrupt_out_size: i32,
    pub interrupt_out_urb: *mut urb,
    pub interrupt_out_endpointAddress: u8,
    pub bulk_in_buffer: *mut u8,
    pub bulk_in_size: i32,
    pub read_urb: *mut urb,
    pub bulk_in_endpointAddress: u8,
    pub bulk_in_buffers: [*mut u8; 2],
    pub read_urbs: [*mut urb; 2],
    pub read_urbs_free: c_ulong,
    pub bulk_out_buffer: *mut u8,
    pub bulk_out_size: i32,
    pub write_urb: *mut urb,
    pub write_fifo: kfifo,
    pub bulk_out_buffers: [*mut u8; 2],
    pub write_urbs: [*mut urb; 2],
    pub write_urbs_free: c_ulong,
    pub bulk_out_endpointAddress: u8,
    pub icount: async_icount,
    pub tx_bytes: i32,
    pub flags: c_ulong,
    pub work: work_struct,
    pub sysrq: c_ulong,
    pub dev: device,
}

#[inline]
pub unsafe fn to_usb_serial_port(d: *mut device) -> *mut usb_serial_port {
    container_of!(d, usb_serial_port, dev)
}

#[inline]
pub unsafe fn usb_get_serial_port_data(port: *mut usb_serial_port) -> *mut c_void {
    dev_get_drvdata!(&mut (*port).dev)
}

#[inline]
pub unsafe fn usb_set_serial_port_data(port: *mut usb_serial_port, data: *mut c_void) {
    dev_set_drvdata!(&mut (*port).dev, data);
}

#[repr(C)]
pub struct usb_serial {
    pub dev: *mut usb_device,
    pub r#type: *mut usb_serial_driver,
    pub interface: *mut usb_interface,
    pub sibling: *mut usb_interface,
    pub suspend_count: c_uint,
    pub disconnected: u8,
    pub attached: u8,
    pub minors_reserved: u8,
    pub num_ports: u8,
    pub num_port_pointers: u8,
    pub num_interrupt_in: u8,
    pub num_interrupt_out: u8,
    pub num_bulk_in: u8,
    pub num_bulk_out: u8,
    pub port: [*mut usb_serial_port; MAX_NUM_PORTS],
    pub kref: kref,
    pub disc_mutex: mutex,
    pub private: *mut c_void,
}

#[inline]
pub unsafe fn to_usb_serial(d: *mut kref) -> *mut usb_serial {
    container_of!(d, usb_serial, kref)
}

#[inline]
pub unsafe fn usb_get_serial_data(serial: *mut usb_serial) -> *mut c_void { (*serial).private }
#[inline]
pub unsafe fn usb_set_serial_data(serial: *mut usb_serial, data: *mut c_void) { (*serial).private = data; }

#[repr(C)]
pub struct usb_serial_endpoints {
    pub num_bulk_in: u8,
    pub num_bulk_out: u8,
    pub num_interrupt_in: u8,
    pub num_interrupt_out: u8,
    pub bulk_in: [*mut usb_endpoint_descriptor; MAX_NUM_PORTS],
    pub bulk_out: [*mut usb_endpoint_descriptor; MAX_NUM_PORTS],
    pub interrupt_in: [*mut usb_endpoint_descriptor; MAX_NUM_PORTS],
    pub interrupt_out: [*mut usb_endpoint_descriptor; MAX_NUM_PORTS],
}

#[repr(C)]
pub struct usb_serial_driver {
    pub description: *const c_char,
    pub id_table: *const usb_device_id,
    pub driver_list: list_head,
    pub driver: device_driver,
    pub usb_driver: *mut usb_driver,
    pub dynids: usb_dynids,
    pub num_ports: u8,
    pub num_bulk_in: u8,
    pub num_bulk_out: u8,
    pub num_interrupt_in: u8,
    pub num_interrupt_out: u8,
    pub bulk_in_size: usize,
    pub bulk_out_size: usize,
    pub probe: Option<unsafe extern "C" fn(*mut usb_serial, *const usb_device_id) -> i32>,
    pub attach: Option<unsafe extern "C" fn(*mut usb_serial) -> i32>,
    pub calc_num_ports: Option<unsafe extern "C" fn(*mut usb_serial, *mut usb_serial_endpoints) -> i32>,
    pub disconnect: Option<unsafe extern "C" fn(*mut usb_serial)>,
    pub release: Option<unsafe extern "C" fn(*mut usb_serial)>,
    pub port_probe: Option<unsafe extern "C" fn(*mut usb_serial_port) -> i32>,
    pub port_remove: Option<unsafe extern "C" fn(*mut usb_serial_port)>,
    pub suspend: Option<unsafe extern "C" fn(*mut usb_serial, pm_message_t) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut usb_serial) -> i32>,
    pub reset_resume: Option<unsafe extern "C" fn(*mut usb_serial) -> i32>,
    pub open: Option<unsafe extern "C" fn(*mut tty_struct, *mut usb_serial_port) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut usb_serial_port)>,
    pub write: Option<unsafe extern "C" fn(*mut tty_struct, *mut usb_serial_port, *const u8, i32) -> i32>,
    pub write_room: Option<unsafe extern "C" fn(*mut tty_struct) -> c_uint>,
    pub ioctl: Option<unsafe extern "C" fn(*mut tty_struct, c_uint, c_ulong) -> i32>,
    pub get_serial: Option<unsafe extern "C" fn(*mut tty_struct, *mut serial_struct)>,
    pub set_serial: Option<unsafe extern "C" fn(*mut tty_struct, *mut serial_struct) -> i32>,
    pub set_termios: Option<unsafe extern "C" fn(*mut tty_struct, *mut usb_serial_port, *const ktermios)>,
    pub break_ctl: Option<unsafe extern "C" fn(*mut tty_struct, i32) -> i32>,
    pub chars_in_buffer: Option<unsafe extern "C" fn(*mut tty_struct) -> c_uint>,
    pub wait_until_sent: Option<unsafe extern "C" fn(*mut tty_struct, c_long)>,
    pub tx_empty: Option<unsafe extern "C" fn(*mut usb_serial_port) -> bool>,
    pub throttle: Option<unsafe extern "C" fn(*mut tty_struct)>,
    pub unthrottle: Option<unsafe extern "C" fn(*mut tty_struct)>,
    pub tiocmget: Option<unsafe extern "C" fn(*mut tty_struct) -> i32>,
    pub tiocmset: Option<unsafe extern "C" fn(*mut tty_struct, c_uint, c_uint) -> i32>,
    pub tiocmiwait: Option<unsafe extern "C" fn(*mut tty_struct, c_ulong) -> i32>,
    pub get_icount: Option<unsafe extern "C" fn(*mut tty_struct, *mut serial_icounter_struct) -> i32>,
    pub dtr_rts: Option<unsafe extern "C" fn(*mut usb_serial_port, i32)>,
    pub carrier_raised: Option<unsafe extern "C" fn(*mut usb_serial_port) -> i32>,
    pub init_termios: Option<unsafe extern "C" fn(*mut tty_struct)>,
    pub read_int_callback: Option<unsafe extern "C" fn(*mut urb)>,
    pub write_int_callback: Option<unsafe extern "C" fn(*mut urb)>,
    pub read_bulk_callback: Option<unsafe extern "C" fn(*mut urb)>,
    pub write_bulk_callback: Option<unsafe extern "C" fn(*mut urb)>,
    pub process_read_urb: Option<unsafe extern "C" fn(*mut urb)>,
    pub prepare_write_buffer: Option<unsafe extern "C" fn(*mut usb_serial_port, *mut c_void, usize) -> i32>,
}

#[inline]
pub unsafe fn to_usb_serial_driver(d: *mut device_driver) -> *mut usb_serial_driver {
    container_of!(d, usb_serial_driver, driver)
}

pub unsafe extern "C" fn __usb_serial_register_drivers(
    serial_drivers: *const *mut usb_serial_driver, owner: *mut module,
    name: *const c_char, id_table: *const usb_device_id) -> i32;
pub unsafe extern "C" fn usb_serial_deregister_drivers(serial_drivers: *const *mut usb_serial_driver);
pub unsafe extern "C" fn usb_serial_port_softint(port: *mut usb_serial_port);
pub unsafe extern "C" fn usb_serial_suspend(intf: *mut usb_interface, message: pm_message_t) -> i32;
pub unsafe extern "C" fn usb_serial_resume(intf: *mut usb_interface) -> i32;

// CONFIG_USB_SERIAL_CONSOLE declarations are conditional in the C header.
pub unsafe extern "C" fn usb_serial_console_init(minor: i32);
pub unsafe extern "C" fn usb_serial_console_exit();
pub unsafe extern "C" fn usb_serial_console_disconnect(serial: *mut usb_serial);

pub unsafe extern "C" fn usb_serial_port_get_by_minor(minor: c_uint) -> *mut usb_serial_port;
pub unsafe extern "C" fn usb_serial_put(serial: *mut usb_serial);
pub unsafe extern "C" fn usb_serial_claim_interface(serial: *mut usb_serial, intf: *mut usb_interface) -> i32;
pub unsafe extern "C" fn usb_serial_generic_open(tty: *mut tty_struct, port: *mut usb_serial_port) -> i32;
pub unsafe extern "C" fn usb_serial_generic_write_start(port: *mut usb_serial_port, mem_flags: gfp_t) -> i32;
pub unsafe extern "C" fn usb_serial_generic_write(tty: *mut tty_struct, port: *mut usb_serial_port, buf: *const u8, count: i32) -> i32;
pub unsafe extern "C" fn usb_serial_generic_close(port: *mut usb_serial_port);
pub unsafe extern "C" fn usb_serial_generic_resume(serial: *mut usb_serial) -> i32;
pub unsafe extern "C" fn usb_serial_generic_write_room(tty: *mut tty_struct) -> c_uint;
pub unsafe extern "C" fn usb_serial_generic_chars_in_buffer(tty: *mut tty_struct) -> c_uint;
pub unsafe extern "C" fn usb_serial_generic_wait_until_sent(tty: *mut tty_struct, timeout: c_long);
pub unsafe extern "C" fn usb_serial_generic_read_bulk_callback(urb: *mut urb);
pub unsafe extern "C" fn usb_serial_generic_write_bulk_callback(urb: *mut urb);
pub unsafe extern "C" fn usb_serial_generic_throttle(tty: *mut tty_struct);
pub unsafe extern "C" fn usb_serial_generic_unthrottle(tty: *mut tty_struct);
pub unsafe extern "C" fn usb_serial_generic_tiocmiwait(tty: *mut tty_struct, arg: c_ulong) -> i32;
pub unsafe extern "C" fn usb_serial_generic_get_icount(tty: *mut tty_struct, icount: *mut serial_icounter_struct) -> i32;
pub unsafe extern "C" fn usb_serial_generic_register() -> i32;
pub unsafe extern "C" fn usb_serial_generic_deregister();
pub unsafe extern "C" fn usb_serial_generic_submit_read_urbs(port: *mut usb_serial_port, mem_flags: gfp_t) -> i32;
pub unsafe extern "C" fn usb_serial_generic_process_read_urb(urb: *mut urb);
pub unsafe extern "C" fn usb_serial_generic_prepare_write_buffer(port: *mut usb_serial_port, dest: *mut c_void, size: usize) -> i32;
pub unsafe extern "C" fn usb_serial_handle_sysrq_char(port: *mut usb_serial_port, ch: c_uint) -> i32;
pub unsafe extern "C" fn usb_serial_handle_break(port: *mut usb_serial_port) -> i32;
pub unsafe extern "C" fn usb_serial_handle_dcd_change(port: *mut usb_serial_port, tty: *mut tty_struct, status: c_uint);
pub unsafe extern "C" fn usb_serial_bus_register(device: *mut usb_serial_driver) -> i32;
pub unsafe extern "C" fn usb_serial_bus_deregister(device: *mut usb_serial_driver);
pub static mut usb_serial_bus_type: bus_type;
pub static mut usb_serial_tty_driver: *mut tty_driver;

// The C header's logging and module-registration macros depend on kernel macros
// and are retained as declarations/comments rather than introducing implementations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
