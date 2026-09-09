// SPDX-License-Identifier: GPL-2.0-only
/* linux/drivers/char/nsc_gpio.c

   National Semiconductor common GPIO device-file/VFS methods.
   Allows a user space process to control the GPIO pins.

   Copyright (c) 2001,2002 Christer Weinigel <wingel@nano-system.com>
   Copyright (c) 2005      Jim Cromie <jim.cromie@gmail.com>
*/

// Linux kernel dependencies supplied by other translated units.

pub const NAME: &[u8] = b"nsc_gpio\0";

#[repr(C)]
pub struct file {
    pub private_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct device;

#[repr(C)]
pub struct nsc_gpio_ops {
    pub dev: *mut device,
    pub gpio_config: unsafe extern "C" fn(*mut nsc_gpio_ops, u32, u32, u32) -> u32,
    pub gpio_get: unsafe extern "C" fn(*mut nsc_gpio_ops, u32) -> i32,
    pub gpio_set: unsafe extern "C" fn(*mut nsc_gpio_ops, u32, i32),
    pub gpio_current: unsafe extern "C" fn(*mut nsc_gpio_ops, u32) -> i32,
    pub gpio_dump: unsafe extern "C" fn(*mut nsc_gpio_ops, *mut nsc_gpio_ops, u32),
}

extern "C" {
    fn iminor(inode: *mut core::ffi::c_void) -> u32;
    fn file_inode(file: *mut file) -> *mut core::ffi::c_void;
    fn get_user(value: *mut u8, address: *const u8) -> i32;
    fn put_user(value: u8, address: *mut u8) -> i32;
    fn dev_info(dev: *mut device, format: *const u8, ...);
    fn dev_dbg(dev: *mut device, format: *const u8, ...);
    fn dev_err(dev: *mut device, format: *const u8, ...);
}

pub unsafe extern "C" fn nsc_gpio_dump(amp: *mut nsc_gpio_ops, index: u32) {
    /* retrieve current config w/o changing it */
    let config: u32 = ((*amp).gpio_config)(amp, index, !0, 0);

    /* user requested via 'v' command, so its INFO */
    dev_info(
        (*amp).dev,
        b"io%02u: 0x%04x %s %s %s %s %s %s %s\tio:%d/%d\n\0".as_ptr(),
        index, config,
        if config & 1 != 0 { b"OE\0".as_ptr() } else { b"TS\0".as_ptr() },
        if config & 2 != 0 { b"PP\0".as_ptr() } else { b"OD\0".as_ptr() },
        if config & 4 != 0 { b"PUE\0".as_ptr() } else { b"PUD\0".as_ptr() },
        if config & 8 != 0 { b"LOCKED\0".as_ptr() } else { b"\0".as_ptr() },
        if config & 16 != 0 { b"LEVEL\0".as_ptr() } else { b"EDGE\0".as_ptr() },
        if config & 32 != 0 { b"HI\0".as_ptr() } else { b"LO\0".as_ptr() },
        if config & 64 != 0 { b"DEBOUNCE\0".as_ptr() } else { b"\0".as_ptr() },
        ((*amp).gpio_get)(amp, index), ((*amp).gpio_current)(amp, index),
    );
}

pub unsafe extern "C" fn nsc_gpio_write(
    file: *mut file, data: *const u8, len: usize, _ppos: *mut i64,
) -> isize {
    let m = iminor(file_inode(file));
    let amp = (*file).private_data as *mut nsc_gpio_ops;
    let dev = (*amp).dev;
    let mut err = 0;

    for i in 0..len {
        let mut c = 0u8;
        if get_user(&mut c, data.add(i)) != 0 { return -14; }
        match c {
            b'0' => ((*amp).gpio_set)(amp, m, 0),
            b'1' => ((*amp).gpio_set)(amp, m, 1),
            b'O' => { dev_dbg(dev, b"GPIO%d output enabled\n\0".as_ptr(), m); ((*amp).gpio_config)(amp, m, !1, 1); },
            b'o' => { dev_dbg(dev, b"GPIO%d output disabled\n\0".as_ptr(), m); ((*amp).gpio_config)(amp, m, !1, 0); },
            b'T' => { dev_dbg(dev, b"GPIO%d output is push pull\n\0".as_ptr(), m); ((*amp).gpio_config)(amp, m, !2, 2); },
            b't' => { dev_dbg(dev, b"GPIO%d output is open drain\n\0".as_ptr(), m); ((*amp).gpio_config)(amp, m, !2, 0); },
            b'P' => { dev_dbg(dev, b"GPIO%d pull up enabled\n\0".as_ptr(), m); ((*amp).gpio_config)(amp, m, !4, 4); },
            b'p' => { dev_dbg(dev, b"GPIO%d pull up disabled\n\0".as_ptr(), m); ((*amp).gpio_config)(amp, m, !4, 0); },
            b'v' => nsc_gpio_dump(amp, m),
            b'\n' => {},
            _ => { dev_err(dev, b"io%2d bad setting: chr<0x%2x>\n\0".as_ptr(), m, c as i32); err += 1; },
        }
    }
    if err != 0 { return -22; }
    len as isize
}

pub unsafe extern "C" fn nsc_gpio_read(
    file: *mut file, buf: *mut u8, _len: usize, _ppos: *mut i64,
) -> isize {
    let m = iminor(file_inode(file));
    let amp = (*file).private_data as *mut nsc_gpio_ops;
    let value = ((*amp).gpio_get)(amp, m);
    if put_user(if value != 0 { b'1' } else { b'0' }, buf) != 0 { return -14; }
    1
}

// EXPORT_SYMBOL(nsc_gpio_write);
// EXPORT_SYMBOL(nsc_gpio_read);
// EXPORT_SYMBOL(nsc_gpio_dump);
// MODULE_AUTHOR("Jim Cromie <jim.cromie@gmail.com>");
// MODULE_DESCRIPTION("NatSemi GPIO Common Methods");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
