// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * command structure borrowed from udev
 * (git://git.kernel.org/pub/scm/linux/hotplug/udev.git)
 *
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 */

use core::ptr;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

// Dependencies from C headers:
// stdio.h, stdlib.h, getopt.h, syslog.h,
// usbip_common.h, usbip_network.h, usbip.h.

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

type CommandFn = unsafe extern "C" fn(argc: c_int, argv: *mut *mut c_char) -> c_int;
type UsageFn = unsafe extern "C" fn();

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const no_argument: c_int = 0;
const required_argument: c_int = 1;
const LOG_PID: c_int = 0x01;
const LOG_USER: c_int = 1 << 3;

// Build/header macros in the original C source.
// TODO: supplied by build headers in the full repository.
const PACKAGE_STRING: &[u8] = b"PACKAGE_STRING\0";
const PROGNAME: &[u8] = b"usbip\0";
const USBIP_HOST_DRV_NAME: &[u8] = b"USBIP_HOST_DRV_NAME\0";

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn openlog(ident: *const c_char, option: c_int, facility: c_int);

    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut opterr: c_int;

    static mut usbip_use_stderr: c_int;
    static mut usbip_use_debug: c_int;
    static mut usbip_use_syslog: c_int;

    fn usbip_setup_port_number(arg: *mut c_char);
    fn usbip_attach(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn usbip_attach_usage();
    fn usbip_detach(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn usbip_detach_usage();
    fn usbip_list(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn usbip_list_usage();
    fn usbip_bind(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn usbip_bind_usage();
    fn usbip_unbind(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn usbip_unbind_usage();
    fn usbip_port_show(argc: c_int, argv: *mut *mut c_char) -> c_int;

    fn dbg(format: *const c_char, ...);
}

unsafe extern "C" fn usbip_help(argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut cmd: *const command;
    let mut i: c_int;
    let mut ret: c_int = 0;

    if argc > 1 && {
        let old = argv;
        argv = argv.add(1);
        !old.is_null()
    } {
        i = 0;
        while !(*cmds.as_ptr().add(i as usize)).name.is_null() {
            if strcmp((*cmds.as_ptr().add(i as usize)).name, *argv) == 0
                && (*cmds.as_ptr().add(i as usize)).usage.is_some()
            {
                ((*cmds.as_ptr().add(i as usize)).usage.unwrap())();
                goto_done(ret);
                return ret;
            }
            i += 1;
        }
        ret = -1;
    }

    usbip_usage();
    printf(c"\n".as_ptr());
    cmd = cmds.as_ptr();
    while !(*cmd).name.is_null() {
        if !(*cmd).help.is_null() {
            printf(c"  %-10s %s\n".as_ptr(), (*cmd).name, (*cmd).help);
        }
        cmd = cmd.add(1);
    }
    printf(c"\n".as_ptr());

    ret
}

#[inline]
unsafe fn goto_done(_ret: c_int) {}

unsafe extern "C" fn usbip_version(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let _ = argc;
    let _ = argv;

    printf(c"%s (%s)\n".as_ptr(), PROGNAME.as_ptr(), usbip_version_string.as_ptr());
    0
}

static usbip_version_string: &[u8] = PACKAGE_STRING;

static usbip_usage_string: &[u8] = b"usbip [--debug] [--log] [--tcp-port PORT] [version]\n\
             [help] <command> <args>\n\0";

unsafe fn usbip_usage() {
    printf(c"usage: %s".as_ptr(), usbip_usage_string.as_ptr());
}

#[repr(C)]
struct command {
    name: *const c_char,
    fn_: Option<CommandFn>,
    help: *const c_char,
    usage: Option<UsageFn>,
}

unsafe impl Sync for command {}

static cmds: [command; 9] = [
    command {
        name: c"help".as_ptr(),
        fn_: Some(usbip_help),
        help: ptr::null(),
        usage: None,
    },
    command {
        name: c"version".as_ptr(),
        fn_: Some(usbip_version),
        help: ptr::null(),
        usage: None,
    },
    command {
        name: c"attach".as_ptr(),
        fn_: Some(usbip_attach),
        help: c"Attach a remote USB device".as_ptr(),
        usage: Some(usbip_attach_usage),
    },
    command {
        name: c"detach".as_ptr(),
        fn_: Some(usbip_detach),
        help: c"Detach a remote USB device".as_ptr(),
        usage: Some(usbip_detach_usage),
    },
    command {
        name: c"list".as_ptr(),
        fn_: Some(usbip_list),
        help: c"List exportable or local USB devices".as_ptr(),
        usage: Some(usbip_list_usage),
    },
    command {
        name: c"bind".as_ptr(),
        fn_: Some(usbip_bind),
        help: c"Bind device to USBIP_HOST_DRV_NAME.ko".as_ptr(),
        usage: Some(usbip_bind_usage),
    },
    command {
        name: c"unbind".as_ptr(),
        fn_: Some(usbip_unbind),
        help: c"Unbind device from USBIP_HOST_DRV_NAME.ko".as_ptr(),
        usage: Some(usbip_unbind_usage),
    },
    command {
        name: c"port".as_ptr(),
        fn_: Some(usbip_port_show),
        help: c"Show imported USB devices".as_ptr(),
        usage: None,
    },
    command {
        name: ptr::null(),
        fn_: None,
        help: ptr::null(),
        usage: None,
    },
];

unsafe fn run_command(cmd: *const command, argc: c_int, argv: *mut *mut c_char) -> c_int {
    dbg(c"running command: `%s'".as_ptr(), (*cmd).name);
    ((*cmd).fn_.unwrap())(argc, argv)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    static opts: [option; 4] = [
        option {
            name: c"debug".as_ptr(),
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: 'd' as c_int,
        },
        option {
            name: c"log".as_ptr(),
            has_arg: no_argument,
            flag: ptr::null_mut(),
            val: 'l' as c_int,
        },
        option {
            name: c"tcp-port".as_ptr(),
            has_arg: required_argument,
            flag: ptr::null_mut(),
            val: 't' as c_int,
        },
        option {
            name: ptr::null(),
            has_arg: 0,
            flag: ptr::null_mut(),
            val: 0,
        },
    ];

    let mut cmd: *mut c_char;
    let mut opt: c_int;
    let mut i: c_int;
    let mut rc: c_int = -1;

    usbip_use_stderr = 1;
    opterr = 0;
    loop {
        opt = getopt_long(argc, argv, c"+dlt:".as_ptr(), opts.as_ptr(), ptr::null_mut());

        if opt == -1 {
            break;
        }

        match opt {
            x if x == 'd' as c_int => {
                usbip_use_debug = 1;
            }
            x if x == 'l' as c_int => {
                usbip_use_syslog = 1;
                openlog(c"".as_ptr(), LOG_PID, LOG_USER);
            }
            x if x == 't' as c_int => {
                usbip_setup_port_number(optarg);
            }
            x if x == '?' as c_int => {
                printf(c"usbip: invalid option\n".as_ptr());
                /*
                 * Terminate after printing error
                 * FALLTHRU
                 */
                usbip_usage();
                return if rc > -1 { EXIT_SUCCESS } else { EXIT_FAILURE };
            }
            _ => {
                usbip_usage();
                return if rc > -1 { EXIT_SUCCESS } else { EXIT_FAILURE };
            }
        }
    }

    cmd = *argv.add(optind as usize);
    if !cmd.is_null() {
        i = 0;
        while !(*cmds.as_ptr().add(i as usize)).name.is_null() {
            if strcmp((*cmds.as_ptr().add(i as usize)).name, cmd) == 0 {
                argc -= optind;
                argv = argv.add(optind as usize);
                optind = 0;
                rc = run_command(cmds.as_ptr().add(i as usize), argc, argv);
                return if rc > -1 { EXIT_SUCCESS } else { EXIT_FAILURE };
            }
            i += 1;
        }
    }

    /* invalid command */
    usbip_help(0, ptr::null_mut());
    if rc > -1 {
        EXIT_SUCCESS
    } else {
        EXIT_FAILURE
    }
}
