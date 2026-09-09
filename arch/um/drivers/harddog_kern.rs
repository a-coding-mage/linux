/* UML hardware watchdog, shamelessly stolen from:
 *
 *	SoftDog 0.05: A Software Watchdog Device
 *
 * (c) Copyright 1996 Alan Cox <alan@redhat.com>, All Rights Reserved.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 *
 * UML hardware watchdog driver.
 */

/* Linux kernel headers and local headers supplied by other translation units. */

const EBUSY: i32 = 16;
const ENOTTY: i32 = 25;
const EFAULT: i32 = 14;
const WATCHDOG_MINOR: i32 = 130;
const WDIOC_SETTIMEOUT: u32 = 0xC004_5706;
const WDIOC_GETSUPPORT: u32 = 0x8028_5700;
const WDIOC_GETSTATUS: u32 = 0x8004_5701;
const WDIOC_GETBOOTSTATUS: u32 = 0x8004_5702;
const WDIOC_KEEPALIVE: u32 = 0x8004_5705;

/* External kernel and UML interfaces. */
extern "C" {
    fn mconsole_notify_socket() -> *mut core::ffi::c_char;
    fn start_watchdog(
        in_fd: *mut i32,
        out_fd: *mut i32,
        sock: *mut core::ffi::c_char,
    ) -> i32;
    fn stop_watchdog(in_fd: i32, out_fd: i32);
    fn ping_watchdog(out_fd: i32) -> isize;
    fn stream_open(inode: *mut inode, file: *mut file) -> i32;
    fn compat_ptr_ioctl(file: *mut file, cmd: u32, arg: u64) -> i64;
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
struct watchdog_info {
    options: u32,
    firmware_version: u32,
    identity: [u8; 32],
}

#[repr(C)]
struct file_operations {
    owner: *const core::ffi::c_void,
    write: Option<unsafe extern "C" fn(*mut file, *const u8, usize, *mut i64) -> isize>,
    unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, u32, u64) -> i64>,
    compat_ioctl: Option<unsafe extern "C" fn(*mut file, u32, u64) -> i64>,
    open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
    release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
}

#[repr(C)]
struct miscdevice {
    minor: i32,
    name: *const u8,
    fops: *const file_operations,
}

static mut harddog_mutex: u8 = 0;
static mut lock: u8 = 0;
static mut timer_alive: i32 = 0;
static mut harddog_in_fd: i32 = -1;
static mut harddog_out_fd: i32 = -1;

/* CONFIG_WATCHDOG_NOWAYOUT and CONFIG_MCONSOLE are build-time conditions. */

#[no_mangle]
unsafe extern "C" fn harddog_open(inode: *mut inode, file: *mut file) -> i32 {
    let mut err: i32 = -EBUSY;
    let mut sock: *mut core::ffi::c_char = core::ptr::null_mut();

    /* mutex_lock(&harddog_mutex); spin_lock(&lock); */
    if timer_alive != 0 {
        /* spin_unlock(&lock); mutex_unlock(&harddog_mutex); */
        return err;
    }

    /* CONFIG_WATCHDOG_NOWAYOUT: __module_get(THIS_MODULE); */
    /* CONFIG_MCONSOLE: sock = mconsole_notify_socket(); */
    err = start_watchdog(&mut harddog_in_fd, &mut harddog_out_fd, sock);
    if err != 0 {
        /* spin_unlock(&lock); mutex_unlock(&harddog_mutex); */
        return err;
    }

    timer_alive = 1;
    /* spin_unlock(&lock); mutex_unlock(&harddog_mutex); */
    stream_open(inode, file)
}

#[no_mangle]
unsafe extern "C" fn harddog_release(_inode: *mut inode, _file: *mut file) -> i32 {
    /* spin_lock(&lock); */
    stop_watchdog(harddog_in_fd, harddog_out_fd);
    harddog_in_fd = -1;
    harddog_out_fd = -1;
    timer_alive = 0;
    /* spin_unlock(&lock); */
    0
}

#[no_mangle]
unsafe extern "C" fn harddog_write(
    _file: *mut file,
    _data: *const u8,
    len: usize,
    _ppos: *mut i64,
) -> isize {
    if len != 0 {
        return ping_watchdog(harddog_out_fd);
    }
    0
}

#[no_mangle]
unsafe extern "C" fn harddog_ioctl_unlocked(
    _file: *mut file,
    cmd: u32,
    arg: u64,
) -> i64 {
    let argp = arg as *mut core::ffi::c_void;
    static mut ident: watchdog_info = watchdog_info {
        options: WDIOC_SETTIMEOUT,
        firmware_version: 0,
        identity: [0; 32],
    };

    match cmd {
        WDIOC_GETSUPPORT => {
            /* copy_to_user(argp, &ident, sizeof(ident)); */
            let _ = (argp, &raw const ident);
            0
        }
        WDIOC_GETSTATUS | WDIOC_GETBOOTSTATUS => {
            /* put_user(0, (int __user *)argp); */
            let _ = argp;
            0
        }
        WDIOC_KEEPALIVE => ping_watchdog(harddog_out_fd) as i64,
        _ => -ENOTTY as i64,
    }
}

#[no_mangle]
unsafe extern "C" fn harddog_ioctl(_file: *mut file, cmd: u32, arg: u64) -> i64 {
    /* mutex_lock(&harddog_mutex); */
    let ret = harddog_ioctl_unlocked(_file, cmd, arg);
    /* mutex_unlock(&harddog_mutex); */
    ret
}

static harddog_fops: file_operations = file_operations {
    owner: core::ptr::null(),
    write: Some(harddog_write),
    unlocked_ioctl: Some(harddog_ioctl),
    compat_ioctl: Some(compat_ptr_ioctl),
    open: Some(harddog_open),
    release: Some(harddog_release),
};

static mut harddog_miscdev: miscdevice = miscdevice {
    minor: WATCHDOG_MINOR,
    name: b"watchdog\0".as_ptr(),
    fops: &harddog_fops,
};

/* module_misc_device(harddog_miscdev); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
