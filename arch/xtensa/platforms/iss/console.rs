/*
 * arch/xtensa/platforms/iss/console.c
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001-2005 Tensilica Inc.
 *   Authors\tChristian Zankel, Joe Taylor
 */

// Linux and platform headers from the original implementation are supplied by
// the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_uint, c_void};

const SERIAL_MAX_NUM_LINES: c_uint = 1;
const SERIAL_TIMER_VALUE: c_uint = HZ / 10;

type U8 = u8;
type SizeT = usize;
type SsizeT = isize;

#[repr(C)] pub struct TimerList { _private: [u8; 0] }
#[repr(C)] pub struct TtyStruct { pub count: c_int }
#[repr(C)] pub struct File { _private: [u8; 0] }
#[repr(C)] pub struct TtyDriver { _private: [u8; 0] }
#[repr(C)] pub struct TtyPort { _private: [u8; 0] }
#[repr(C)] pub struct SeqFile { _private: [u8; 0] }
#[repr(C)] pub struct Console { pub index: c_int }

extern "C" {
    static mut serial_timer: TimerList;
    static mut serial_driver: *mut TtyDriver;
    static mut serial_port: TtyPort;
    static mut jiffies: c_uint;
    static tty_std_termios: Termios;

    fn simc_write(fd: c_int, buf: *const c_void, count: SizeT);
    fn simc_poll(fd: c_int) -> c_int;
    fn simc_read(fd: c_int, buf: *mut U8, count: SizeT) -> c_int;
    fn mod_timer(timer: *mut TimerList, expires: c_uint);
    fn timer_delete_sync(timer: *mut TimerList);
    fn tty_insert_flip_char(port: *mut TtyPort, c: U8, flag: c_uint);
    fn tty_flip_buffer_push(port: *mut TtyPort);
    fn seq_printf(m: *mut SeqFile, fmt: *const c_char, ...);
    fn tty_alloc_driver(lines: c_uint, flags: c_uint) -> *mut TtyDriver;
    fn tty_port_init(port: *mut TtyPort);
    fn tty_set_operations(driver: *mut TtyDriver, ops: *const TtyOperations);
    fn tty_port_link_device(port: *mut TtyPort, driver: *mut TtyDriver, index: c_uint);
    fn tty_register_driver(driver: *mut TtyDriver) -> c_int;
    fn tty_driver_kref_put(driver: *mut TtyDriver);
    fn tty_port_destroy(port: *mut TtyPort);
    fn tty_unregister_driver(driver: *mut TtyDriver);
    fn register_console(console: *mut Console);
}

#[repr(C)] struct Termios { pub c_cflag: c_uint }
#[repr(C)] struct TtyOperations {
    pub open: Option<unsafe extern "C" fn(*mut TtyStruct, *mut File) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut TtyStruct, *mut File)>,
    pub write: Option<unsafe extern "C" fn(*mut TtyStruct, *const U8, SizeT) -> SsizeT>,
    pub write_room: Option<unsafe extern "C" fn(*mut TtyStruct) -> c_uint>,
    pub proc_show: Option<unsafe extern "C" fn(*mut SeqFile, *mut c_void) -> c_int>,
}

extern "C" {
    static HZ: c_uint;
}

unsafe extern "C" fn rs_open(tty: *mut TtyStruct, _filp: *mut File) -> c_int {
    if (*tty).count == 1 {
        mod_timer(&mut serial_timer, jiffies.wrapping_add(SERIAL_TIMER_VALUE));
    }
    0
}

unsafe extern "C" fn rs_close(tty: *mut TtyStruct, _filp: *mut File) {
    if (*tty).count == 1 { timer_delete_sync(&mut serial_timer); }
}

unsafe extern "C" fn rs_write(_tty: *mut TtyStruct, buf: *const U8, count: SizeT) -> SsizeT {
    // See drivers/char/serialX.c to reference original version.
    simc_write(1, buf as *const c_void, count);
    count as SsizeT
}

unsafe extern "C" fn rs_poll(_unused: *mut TimerList) {
    let port = &mut serial_port as *mut TtyPort;
    let mut i = 0;
    let mut rd = 1;
    let mut c: U8 = 0;
    while simc_poll(0) != 0 {
        rd = simc_read(0, &mut c, 1);
        if rd <= 0 { break; }
        tty_insert_flip_char(port, c, TTY_NORMAL);
        i += 1;
    }
    if i != 0 { tty_flip_buffer_push(port); }
    if rd != 0 { mod_timer(&mut serial_timer, jiffies.wrapping_add(SERIAL_TIMER_VALUE)); }
}

unsafe extern "C" fn rs_write_room(_tty: *mut TtyStruct) -> c_uint { 2 * 1024 }

unsafe extern "C" fn rs_proc_show(m: *mut SeqFile, _v: *mut c_void) -> c_int {
    seq_printf(m, b"serinfo:1.0 driver:0.1\n\0".as_ptr() as *const c_char);
    0
}

static SERIAL_OPS: TtyOperations = TtyOperations {
    open: Some(rs_open), close: Some(rs_close), write: Some(rs_write),
    write_room: Some(rs_write_room), proc_show: Some(rs_proc_show),
};

unsafe extern "C" fn rs_init() -> c_int {
    let driver = tty_alloc_driver(SERIAL_MAX_NUM_LINES, TTY_DRIVER_REAL_RAW);
    if driver.is_null() { return -1; }
    tty_port_init(&mut serial_port);
    // Initialize the tty_driver structure.
    tty_set_operations(driver, &SERIAL_OPS);
    tty_port_link_device(&mut serial_port, driver, 0);
    let ret = tty_register_driver(driver);
    if ret != 0 {
        tty_driver_kref_put(driver);
        tty_port_destroy(&mut serial_port);
        return ret;
    }
    serial_driver = driver;
    0
}

unsafe extern "C" fn rs_exit() {
    tty_unregister_driver(serial_driver);
    tty_driver_kref_put(serial_driver);
    tty_port_destroy(&mut serial_port);
}

// late_initcall(rs_init); registration is provided by the kernel build system.

#[cfg(CONFIG_SERIAL_CONSOLE)]
unsafe extern "C" fn iss_console_write(_co: *mut Console, s: *const c_char, mut count: c_uint) {
    if !s.is_null() {
        let mut n = 0;
        while n < count && *s.add(n as usize) != 0 { n += 1; }
        count = n;
    } else { count = 0; }
    if count != 0 { simc_write(1, s as *const c_void, count as usize); }
}

#[cfg(CONFIG_SERIAL_CONSOLE)]
unsafe extern "C" fn iss_console_device(c: *mut Console, index: *mut c_int) -> *mut TtyDriver {
    *index = (*c).index;
    serial_driver
}

#[cfg(CONFIG_SERIAL_CONSOLE)]
#[repr(C)] struct Sercons { pub console: Console }

// console_initcall(iss_console_init); CONFIG_SERIAL_CONSOLE registration is
// supplied by the surrounding kernel translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
