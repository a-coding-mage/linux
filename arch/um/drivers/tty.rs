// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)
 */

// C dependencies: errno.h, fcntl.h, termios.h, chan_user.h, os.h,
// and um_malloc.h provide the external types, constants, and functions used below.

#[repr(C)]
pub struct tty_chan {
    pub dev: *mut ::std::os::raw::c_char,
    pub raw: ::std::os::raw::c_int,
    pub tt: crate::termios,
}

unsafe fn tty_chan_init(
    mut str_: *mut ::std::os::raw::c_char,
    _device: ::std::os::raw::c_int,
    opts: *const crate::chan_opts,
) -> *mut ::std::ffi::c_void {
    let mut data: *mut tty_chan;

    unsafe {
        if *str_ != b':' as ::std::os::raw::c_char {
            crate::printk(concat!(
                crate::UM_KERN_ERR,
                "tty_init : channel type 'tty' must specify a device\n"
            ));
            return ::std::ptr::null_mut();
        }
        str_ = str_.add(1);

        data = crate::uml_kmalloc(
            ::std::mem::size_of::<tty_chan>(),
            crate::UM_GFP_KERNEL,
        ) as *mut tty_chan;
        if data.is_null() {
            return ::std::ptr::null_mut();
        }
        *data = tty_chan {
            dev: str_,
            raw: (*opts).raw,
            tt: ::std::mem::zeroed(),
        };
    }

    data as *mut ::std::ffi::c_void
}

unsafe fn tty_open(
    input: ::std::os::raw::c_int,
    output: ::std::os::raw::c_int,
    _primary: ::std::os::raw::c_int,
    d: *mut ::std::ffi::c_void,
    dev_out: *mut *mut ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    let data = d as *mut tty_chan;
    let mut fd: ::std::os::raw::c_int;
    let mut err: ::std::os::raw::c_int;
    let mut mode: ::std::os::raw::c_int = 0;

    unsafe {
        if input != 0 && output != 0 {
            mode = crate::O_RDWR;
        } else if input != 0 {
            mode = crate::O_RDONLY;
        } else if output != 0 {
            mode = crate::O_WRONLY;
        }

        fd = crate::open((*data).dev, mode);
        if fd < 0 {
            return -crate::errno;
        }

        if (*data).raw != 0 {
            err = crate::tcgetattr(fd, &mut (*data).tt);
            if err != 0 {
                return err;
            }

            err = crate::raw(fd);
            if err != 0 {
                return err;
            }
        }

        *dev_out = (*data).dev;
    }
    fd
}

pub static tty_ops: crate::chan_ops = crate::chan_ops {
    type_: "tty",
    init: Some(tty_chan_init),
    open: Some(tty_open),
    close: Some(crate::generic_close),
    read: Some(crate::generic_read),
    write: Some(crate::generic_write),
    console_write: Some(crate::generic_console_write),
    window_size: Some(crate::generic_window_size),
    free: Some(crate::generic_free),
    winch: 0,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
