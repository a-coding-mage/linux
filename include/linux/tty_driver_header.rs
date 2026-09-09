/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/tty_driver.h. C includes and build-time configuration
// are supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)] pub struct tty_struct { _private: [u8; 0] }
#[repr(C)] pub struct serial_icounter_struct { _private: [u8; 0] }
#[repr(C)] pub struct serial_struct { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct ktermios { _private: [u8; 0] }
#[repr(C)] pub struct winsize { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct proc_dir_entry { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct cdev { _private: [u8; 0] }
#[repr(C)] pub struct tty_port { _private: [u8; 0] }
#[repr(C)] pub struct kref { pub refcount: c_int }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }

pub type tty_driver_flag = c_ulong;
pub const TTY_DRIVER_INSTALLED: tty_driver_flag = 1 << 0;
pub const TTY_DRIVER_RESET_TERMIOS: tty_driver_flag = 1 << 1;
pub const TTY_DRIVER_REAL_RAW: tty_driver_flag = 1 << 2;
pub const TTY_DRIVER_DYNAMIC_DEV: tty_driver_flag = 1 << 3;
pub const TTY_DRIVER_DEVPTS_MEM: tty_driver_flag = 1 << 4;
pub const TTY_DRIVER_HARDWARE_BREAK: tty_driver_flag = 1 << 5;
pub const TTY_DRIVER_DYNAMIC_ALLOC: tty_driver_flag = 1 << 6;
pub const TTY_DRIVER_UNNUMBERED_NODE: tty_driver_flag = 1 << 7;
pub const TTY_DRIVER_NO_WORKQUEUE: tty_driver_flag = 1 << 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tty_driver_type { TTY_DRIVER_TYPE_SYSTEM, TTY_DRIVER_TYPE_CONSOLE, TTY_DRIVER_TYPE_SERIAL, TTY_DRIVER_TYPE_PTY, TTY_DRIVER_TYPE_SCC, TTY_DRIVER_TYPE_SYSCONS }

pub type tty_driver_subtype = c_int;
pub const SYSTEM_TYPE_TTY: tty_driver_subtype = 1;
pub const SYSTEM_TYPE_CONSOLE: tty_driver_subtype = 2;
pub const SYSTEM_TYPE_SYSCONS: tty_driver_subtype = 3;
pub const SYSTEM_TYPE_SYSPTMX: tty_driver_subtype = 4;
pub const PTY_TYPE_MASTER: tty_driver_subtype = 1;
pub const PTY_TYPE_SLAVE: tty_driver_subtype = 2;
pub const SERIAL_TYPE_NORMAL: tty_driver_subtype = 1;

pub type ssize_t = isize;
pub type u8_ = u8;

#[repr(C)]
pub struct tty_operations {
    pub lookup: Option<unsafe extern "C" fn(*mut tty_driver, *mut file, c_int) -> *mut tty_struct>,
    pub install: Option<unsafe extern "C" fn(*mut tty_driver, *mut tty_struct) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut tty_driver, *mut tty_struct)>,
    pub open: Option<unsafe extern "C" fn(*mut tty_struct, *mut file) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut tty_struct, *mut file)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut tty_struct)>,
    pub cleanup: Option<unsafe extern "C" fn(*mut tty_struct)>,
    pub write: Option<unsafe extern "C" fn(*mut tty_struct, *const u8, usize) -> ssize_t>,
    pub put_char: Option<unsafe extern "C" fn(*mut tty_struct, u8) -> c_int>,
    pub flush_chars: Option<unsafe extern "C" fn(*mut tty_struct)>,
    pub write_room: Option<unsafe extern "C" fn(*mut tty_struct) -> u32>,
    pub chars_in_buffer: Option<unsafe extern "C" fn(*mut tty_struct) -> u32>,
    pub ioctl: Option<unsafe extern "C" fn(*mut tty_struct, u32, c_ulong) -> c_int>,
    pub compat_ioctl: Option<unsafe extern "C" fn(*mut tty_struct, u32, c_ulong) -> c_long>,
    pub set_termios: Option<unsafe extern "C" fn(*mut tty_struct, *const ktermios)>,
    pub throttle: Option<unsafe extern "C" fn(*mut tty_struct)>,
    pub unthrottle: Option<unsafe extern "C" fn(*mut tty_struct)>,
    pub stop: Option<unsafe extern "C" fn(*mut tty_struct)>,
    pub start: Option<unsafe extern "C" fn(*mut tty_struct)>,
    pub hangup: Option<unsafe extern "C" fn(*mut tty_struct)>,
    pub break_ctl: Option<unsafe extern "C" fn(*mut tty_struct, c_int) -> c_int>,
    pub flush_buffer: Option<unsafe extern "C" fn(*mut tty_struct)>,
    pub ldisc_ok: Option<unsafe extern "C" fn(*mut tty_struct, c_int) -> c_int>,
    pub set_ldisc: Option<unsafe extern "C" fn(*mut tty_struct)>,
    pub wait_until_sent: Option<unsafe extern "C" fn(*mut tty_struct, c_int)>,
    pub send_xchar: Option<unsafe extern "C" fn(*mut tty_struct, u8)>,
    pub tiocmget: Option<unsafe extern "C" fn(*mut tty_struct) -> c_int>,
    pub tiocmset: Option<unsafe extern "C" fn(*mut tty_struct, u32, u32) -> c_int>,
    pub resize: Option<unsafe extern "C" fn(*mut tty_struct, *mut winsize) -> c_int>,
    pub get_icount: Option<unsafe extern "C" fn(*mut tty_struct, *mut serial_icounter_struct) -> c_int>,
    pub get_serial: Option<unsafe extern "C" fn(*mut tty_struct, *mut serial_struct) -> c_int>,
    pub set_serial: Option<unsafe extern "C" fn(*mut tty_struct, *mut serial_struct) -> c_int>,
    pub show_fdinfo: Option<unsafe extern "C" fn(*mut tty_struct, *mut seq_file)>,
    // CONFIG_CONSOLE_POLL conditionally supplies these three fields.
    pub poll_init: Option<unsafe extern "C" fn(*mut tty_driver, c_int, *mut c_char) -> c_int>,
    pub poll_get_char: Option<unsafe extern "C" fn(*mut tty_driver, c_int) -> c_int>,
    pub poll_put_char: Option<unsafe extern "C" fn(*mut tty_driver, c_int, c_char)>,
    pub proc_show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct tty_driver {
    pub kref: kref,
    pub cdevs: *mut *mut cdev,
    pub owner: *mut module,
    pub driver_name: *const c_char,
    pub name: *const c_char,
    pub name_base: c_int,
    pub major: c_int,
    pub minor_start: c_int,
    pub num: u32,
    pub type_: tty_driver_type,
    pub subtype: tty_driver_subtype,
    pub init_termios: ktermios,
    pub flags: c_ulong,
    pub proc_entry: *mut proc_dir_entry,
    pub other: *mut tty_driver,
    pub flip_wq: *mut workqueue_struct,
    pub ttys: *mut *mut tty_struct,
    pub ports: *mut *mut tty_port,
    pub termios: *mut *mut ktermios,
    pub driver_state: *mut c_void,
    pub ops: *const tty_operations,
    pub tty_drivers: list_head,
}

extern "C" {
    pub static mut tty_drivers: list_head;
    pub fn __tty_alloc_driver(lines: u32, owner: *mut module, flags: c_ulong) -> *mut tty_driver;
    pub fn tty_find_polling_driver(name: *mut c_char, line: *mut c_int) -> *mut tty_driver;
    pub fn tty_driver_kref_put(driver: *mut tty_driver);
    pub fn tty_register_driver(driver: *mut tty_driver) -> c_int;
    pub fn tty_unregister_driver(driver: *mut tty_driver);
    pub fn tty_register_device(driver: *mut tty_driver, index: u32, dev: *mut device) -> *mut device;
    pub fn tty_register_device_attr(driver: *mut tty_driver, index: u32, device: *mut device, drvdata: *mut c_void, attr_grp: *const *const attribute_group) -> *mut device;
    pub fn tty_unregister_device(driver: *mut tty_driver, index: u32);
}

#[inline] pub unsafe fn tty_driver_kref_get(d: *mut tty_driver) -> *mut tty_driver { d }
#[inline] pub unsafe fn tty_set_operations(driver: *mut tty_driver, op: *const tty_operations) { (*driver).ops = op; }
#[inline] pub unsafe fn proc_tty_register_driver(_: *mut tty_driver) {}
#[inline] pub unsafe fn proc_tty_unregister_driver(_: *mut tty_driver) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
