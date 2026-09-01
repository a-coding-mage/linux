// SPDX-License-Identifier: GPL-2.0-only
/*
 * lsgpio - example on how to list the GPIO lines on a system
 *
 * Copyright (C) 2015 Linus Walleij
 *
 * Usage:
 *	lsgpio <-n device-name>
 */

use libc::{
    c_char, c_int, c_ulong, c_void, close, closedir, free, getopt, ioctl, open, opendir, perror,
    readdir, ENOENT, ENOMEM,
};

// C dependencies: <linux/gpio.h> and "gpio-utils.h"

#[repr(C)]
pub struct gpio_flag {
    pub name: *const c_char,
    pub mask: ::std::os::raw::c_ulonglong,
}

pub static mut flagnames: [gpio_flag; 10] = [
    gpio_flag {
        name: b"used\0".as_ptr() as *const c_char,
        mask: GPIO_V2_LINE_FLAG_USED,
    },
    gpio_flag {
        name: b"input\0".as_ptr() as *const c_char,
        mask: GPIO_V2_LINE_FLAG_INPUT,
    },
    gpio_flag {
        name: b"output\0".as_ptr() as *const c_char,
        mask: GPIO_V2_LINE_FLAG_OUTPUT,
    },
    gpio_flag {
        name: b"active-low\0".as_ptr() as *const c_char,
        mask: GPIO_V2_LINE_FLAG_ACTIVE_LOW,
    },
    gpio_flag {
        name: b"open-drain\0".as_ptr() as *const c_char,
        mask: GPIO_V2_LINE_FLAG_OPEN_DRAIN,
    },
    gpio_flag {
        name: b"open-source\0".as_ptr() as *const c_char,
        mask: GPIO_V2_LINE_FLAG_OPEN_SOURCE,
    },
    gpio_flag {
        name: b"pull-up\0".as_ptr() as *const c_char,
        mask: GPIO_V2_LINE_FLAG_BIAS_PULL_UP,
    },
    gpio_flag {
        name: b"pull-down\0".as_ptr() as *const c_char,
        mask: GPIO_V2_LINE_FLAG_BIAS_PULL_DOWN,
    },
    gpio_flag {
        name: b"bias-disabled\0".as_ptr() as *const c_char,
        mask: GPIO_V2_LINE_FLAG_BIAS_DISABLED,
    },
    gpio_flag {
        name: b"clock-realtime\0".as_ptr() as *const c_char,
        mask: GPIO_V2_LINE_FLAG_EVENT_CLOCK_REALTIME,
    },
];

extern "C" {
    static mut stdout: *mut libc::FILE;
    static mut stderr: *mut libc::FILE;
    static mut errno: c_int;
    static mut optarg: *mut c_char;

    fn fprintf(stream: *mut libc::FILE, format: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn check_prefix(str_: *const c_char, prefix: *const c_char) -> bool;
}

unsafe fn print_attributes(info: *mut gpio_v2_line_info) {
    let mut i: c_int;
    let mut field_format: *const c_char = b"%s\0".as_ptr() as *const c_char;

    i = 0;
    while (i as usize) < flagnames.len() {
        if ((*info).flags & flagnames[i as usize].mask) != 0 {
            fprintf(stdout, field_format, flagnames[i as usize].name);
            field_format = b", %s\0".as_ptr() as *const c_char;
        }
        i += 1;
    }

    if ((*info).flags & GPIO_V2_LINE_FLAG_EDGE_RISING) != 0
        && ((*info).flags & GPIO_V2_LINE_FLAG_EDGE_FALLING) != 0
    {
        fprintf(
            stdout,
            field_format,
            b"both-edges\0".as_ptr() as *const c_char,
        );
    } else if ((*info).flags & GPIO_V2_LINE_FLAG_EDGE_RISING) != 0 {
        fprintf(
            stdout,
            field_format,
            b"rising-edge\0".as_ptr() as *const c_char,
        );
    } else if ((*info).flags & GPIO_V2_LINE_FLAG_EDGE_FALLING) != 0 {
        fprintf(
            stdout,
            field_format,
            b"falling-edge\0".as_ptr() as *const c_char,
        );
    }

    i = 0;
    while i < (*info).num_attrs as c_int {
        if (*info).attrs[i as usize].id == GPIO_V2_LINE_ATTR_ID_DEBOUNCE {
            fprintf(
                stdout,
                b", debounce_period=%dusec\0".as_ptr() as *const c_char,
                (*info).attrs[i as usize].debounce_period_us,
            );
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn list_device(device_name: *const c_char) -> c_int {
    let mut cinfo: gpiochip_info = ::std::mem::zeroed();
    let mut chrdev_name: *mut c_char = ::std::ptr::null_mut();
    let mut fd: c_int;
    let mut ret: c_int;
    let mut i: c_int;

    ret = asprintf(
        &mut chrdev_name as *mut *mut c_char,
        b"/dev/%s\0".as_ptr() as *const c_char,
        device_name,
    );
    if ret < 0 {
        return -ENOMEM;
    }

    fd = open(chrdev_name, 0);
    if fd == -1 {
        ret = -errno;
        fprintf(
            stderr,
            b"Failed to open %s\n\0".as_ptr() as *const c_char,
            chrdev_name,
        );
        goto_exit_free_name(ret, chrdev_name);
        return ret;
    }

    /* Inspect this GPIO chip */
    ret = ioctl(
        fd,
        GPIO_GET_CHIPINFO_IOCTL as c_ulong,
        &mut cinfo as *mut gpiochip_info,
    );
    if ret == -1 {
        ret = -errno;
        perror(b"Failed to issue CHIPINFO IOCTL\n\0".as_ptr() as *const c_char);
        goto_exit_close_error(fd, chrdev_name);
        return ret;
    }
    fprintf(
        stdout,
        b"GPIO chip: %s, \"%s\", %u GPIO lines\n\0".as_ptr() as *const c_char,
        cinfo.name.as_ptr(),
        cinfo.label.as_ptr(),
        cinfo.lines,
    );

    /* Loop over the lines and print info */
    i = 0;
    while i < cinfo.lines as c_int {
        let mut linfo: gpio_v2_line_info = ::std::mem::zeroed();

        memset(
            &mut linfo as *mut gpio_v2_line_info as *mut c_void,
            0,
            ::std::mem::size_of_val(&linfo),
        );
        linfo.offset = i as _;

        ret = ioctl(
            fd,
            GPIO_V2_GET_LINEINFO_IOCTL as c_ulong,
            &mut linfo as *mut gpio_v2_line_info,
        );
        if ret == -1 {
            ret = -errno;
            perror(b"Failed to issue LINEINFO IOCTL\n\0".as_ptr() as *const c_char);
            goto_exit_close_error(fd, chrdev_name);
            return ret;
        }
        fprintf(
            stdout,
            b"\tline %2d:\0".as_ptr() as *const c_char,
            linfo.offset,
        );
        if linfo.name[0] != 0 {
            fprintf(
                stdout,
                b" \"%s\"\0".as_ptr() as *const c_char,
                linfo.name.as_ptr(),
            );
        } else {
            fprintf(stdout, b" unnamed\0".as_ptr() as *const c_char);
        }
        if linfo.consumer[0] != 0 {
            fprintf(
                stdout,
                b" \"%s\"\0".as_ptr() as *const c_char,
                linfo.consumer.as_ptr(),
            );
        } else {
            fprintf(stdout, b" unused\0".as_ptr() as *const c_char);
        }
        if linfo.flags != 0 {
            fprintf(stdout, b" [\0".as_ptr() as *const c_char);
            print_attributes(&mut linfo as *mut gpio_v2_line_info);
            fprintf(stdout, b"]\0".as_ptr() as *const c_char);
        }
        fprintf(stdout, b"\n\0".as_ptr() as *const c_char);

        i += 1;
    }

    if close(fd) == -1 {
        perror(b"Failed to close GPIO character device file\0".as_ptr() as *const c_char);
    }
    free(chrdev_name as *mut c_void);
    ret
}

unsafe fn goto_exit_close_error(fd: c_int, chrdev_name: *mut c_char) {
    if close(fd) == -1 {
        perror(b"Failed to close GPIO character device file\0".as_ptr() as *const c_char);
    }
    free(chrdev_name as *mut c_void);
}

unsafe fn goto_exit_free_name(_ret: c_int, chrdev_name: *mut c_char) {
    free(chrdev_name as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn print_usage() {
    fprintf(
        stderr,
        b"Usage: lsgpio [options]...\nList GPIO chips, lines and states\n  -n <name>  List GPIOs on a named device\n  -?         This helptext\n\0"
            .as_ptr() as *const c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut device_name: *const c_char = ::std::ptr::null();
    let mut ret: c_int;
    let mut c: c_int;

    loop {
        c = getopt(argc, argv, b"n:\0".as_ptr() as *const c_char);
        if c == -1 {
            break;
        }
        match c {
            110 => {
                device_name = optarg;
            }
            63 => {
                print_usage();
                return -1;
            }
            _ => {}
        }
    }

    if !device_name.is_null() {
        ret = list_device(device_name);
    } else {
        let mut ent: *const libc::dirent;
        let mut dp: *mut libc::DIR;

        /* List all GPIO devices one at a time */
        dp = opendir(b"/dev\0".as_ptr() as *const c_char);
        if dp.is_null() {
            ret = -errno;
            return ret;
        }

        ret = -ENOENT;
        loop {
            ent = readdir(dp);
            if ent.is_null() {
                break;
            }
            if check_prefix(
                (*ent).d_name.as_ptr() as *const c_char,
                b"gpiochip\0".as_ptr() as *const c_char,
            ) {
                ret = list_device((*ent).d_name.as_ptr() as *const c_char);
                if ret != 0 {
                    break;
                }
            }
        }

        ret = 0;
        if closedir(dp) == -1 {
            perror(b"scanning devices: Failed to close directory\0".as_ptr() as *const c_char);
            ret = -errno;
        }
    }
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
