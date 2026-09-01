// SPDX-License-Identifier: GPL-2.0
/*
 * rv tool, the interface for the Linux kernel RV subsystem and home of
 * user-space controlled monitors.
 *
 * Copyright (C) 2022 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};

type size_t = usize;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static VERSION: *const c_char;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn geteuid() -> c_int;
    fn signal(signum: c_int, handler: Option<unsafe extern "C" fn(c_int)>)
        -> Option<unsafe extern "C" fn(c_int)>;

    fn ikm_list_monitors(container: *mut c_char);
    fn ikm_run_monitor(monitor_name: *mut c_char, argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn err_msg(format: *const c_char, ...);
}

static mut stop_session: c_int = 0;

/*
 * stop_rv - tell monitors to stop
 */
unsafe extern "C" fn stop_rv(_sig: c_int) {
    unsafe {
        stop_session = 1;
    }
}

/**
 * should_stop - check if the monitor should stop.
 *
 * Returns 1 if the monitor should stop, 0 otherwise.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn should_stop() -> c_int {
    unsafe { stop_session }
}

/*
 * rv_list - list all available monitors
 */
unsafe fn rv_list(argc: c_int, argv: *mut *mut c_char) {
    let usage: [*const c_char; 9] = [
        b"\0".as_ptr() as *const c_char,
        b"  usage: rv list [-h] [container]\0".as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
        b"\tlist all available monitors\0".as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
        b"\t-h/--help: print this menu\0".as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
        b"\t[container]: list only monitors in this container\0".as_ptr() as *const c_char,
        core::ptr::null(),
    ];
    let mut i: c_int;
    let mut print_help: c_int = 0;
    let mut retval: c_int = EXIT_SUCCESS;
    let mut container: *mut c_char = core::ptr::null_mut();

    unsafe {
        if argc == 2 {
            if strcmp(*argv.add(1), b"-h\0".as_ptr() as *const c_char) == 0
                || strcmp(*argv.add(1), b"--help\0".as_ptr() as *const c_char) == 0
            {
                print_help = 1;
                retval = EXIT_SUCCESS;
            } else if *(*argv.add(1)) == b'-' as c_char {
                /* assume invalid option */
                print_help = 1;
                retval = EXIT_FAILURE;
            } else {
                container = *argv.add(1);
            }
        } else if argc > 2 {
            /* more than 2 is always usage */
            print_help = 1;
            retval = EXIT_FAILURE;
        }
        if print_help != 0 {
            fprintf(
                stderr,
                b"rv version %s\n\0".as_ptr() as *const c_char,
                VERSION,
            );
            i = 0;
            while !usage[i as usize].is_null() {
                fprintf(stderr, b"%s\n\0".as_ptr() as *const c_char, usage[i as usize]);
                i += 1;
            }
            exit(retval);
        }

        ikm_list_monitors(container);

        exit(EXIT_SUCCESS);
    }
}

/*
 * rv_mon - try to run a monitor passed as argument
 */
unsafe fn rv_mon(argc: c_int, argv: *mut *mut c_char) {
    let monitor_name: *mut c_char;
    let mut i: c_int;
    let mut run: c_int = 0;

    let usage: [*const c_char; 10] = [
        b"\0".as_ptr() as *const c_char,
        b"  usage: rv mon [-h] monitor [monitor options]\0".as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
        b"\trun a monitor\0".as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
        b"\t-h/--help: print this menu\0".as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
        b"\tmonitor [monitor options]: the monitor, passing\0".as_ptr() as *const c_char,
        b"\tthe arguments to the [monitor options]\0".as_ptr() as *const c_char,
        core::ptr::null(),
    ];

    unsafe {
        /* requires at least one argument */
        if argc == 1 {
            fprintf(
                stderr,
                b"rv version %s\n\0".as_ptr() as *const c_char,
                VERSION,
            );

            i = 0;
            while !usage[i as usize].is_null() {
                fprintf(stderr, b"%s\n\0".as_ptr() as *const c_char, usage[i as usize]);
                i += 1;
            }
            exit(EXIT_FAILURE);
        } else if strcmp(*argv.add(1), b"-h\0".as_ptr() as *const c_char) == 0
            || strcmp(*argv.add(1), b"--help\0".as_ptr() as *const c_char) == 0
        {
            fprintf(
                stderr,
                b"rv version %s\n\0".as_ptr() as *const c_char,
                VERSION,
            );

            i = 0;
            while !usage[i as usize].is_null() {
                fprintf(stderr, b"%s\n\0".as_ptr() as *const c_char, usage[i as usize]);
                i += 1;
            }
            exit(EXIT_SUCCESS);
        }

        monitor_name = *argv.add(1);
        /*
         * Call all possible monitor implementations, looking
         * for the [monitor].
         */
        run += ikm_run_monitor(monitor_name, argc - 1, argv.add(1));

        if run == 0 {
            err_msg(
                b"rv: monitor %s does not exist\n\0".as_ptr() as *const c_char,
                monitor_name,
            );
        }
        exit(if run > 0 { EXIT_SUCCESS } else { EXIT_FAILURE });
    }
}

unsafe fn usage_print(exit_val: c_int) -> ! {
    let mut i: c_int;

    let usage: [*const c_char; 12] = [
        b"\0".as_ptr() as *const c_char,
        b"  usage: rv command [-h] [command_options]\0".as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
        b"\t-h/--help: print this menu\0".as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
        b"\tcommand: run one of the following command:\0".as_ptr() as *const c_char,
        b"\t  list: list all available monitors\0".as_ptr() as *const c_char,
        b"\t  mon:  run a monitor\0".as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
        b"\t[command options]: each command has its own set of options\0".as_ptr() as *const c_char,
        b"\t\t           run rv command -h for further information\0".as_ptr() as *const c_char,
        core::ptr::null(),
    ];

    unsafe {
        i = 0;
        while !usage[i as usize].is_null() {
            fprintf(stderr, b"%s\n\0".as_ptr() as *const c_char, usage[i as usize]);
            i += 1;
        }

        exit(exit_val);
    }
}

unsafe fn usage(exit_val: c_int, message: *const c_char) -> ! {
    unsafe {
        fprintf(
            stderr,
            b"rv version %s: %s\n\0".as_ptr() as *const c_char,
            VERSION,
            message,
        );
        usage_print(exit_val);
    }
}

unsafe fn usage_needs_root(exit_val: c_int, arg0: *mut c_char) -> ! {
    unsafe {
        fprintf(
            stderr,
            b"rv version %s: %s needs root permission\n\0".as_ptr() as *const c_char,
            VERSION,
            arg0,
        );
        usage_print(exit_val);
    }
}

unsafe fn usage_requires_command(exit_val: c_int, arg0: *mut c_char) -> ! {
    unsafe {
        fprintf(
            stderr,
            b"rv version %s: %s requires a command\n\0".as_ptr() as *const c_char,
            VERSION,
            arg0,
        );
        usage_print(exit_val);
    }
}

unsafe fn usage_unknown_command(exit_val: c_int, arg0: *mut c_char, arg1: *mut c_char) -> ! {
    unsafe {
        fprintf(
            stderr,
            b"rv version %s: %s does not know the %s command, old version?\n\0".as_ptr()
                as *const c_char,
            VERSION,
            arg0,
            arg1,
        );
        usage_print(exit_val);
    }
}

/*
 * main - select which main sending the command
 *
 * main itself redirects the arguments to the sub-commands
 * to handle the options.
 *
 * subcommands should exit.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(mut argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        if geteuid() != 0 {
            usage_needs_root(EXIT_FAILURE, *argv.add(0));
        }

        if argc <= 1 {
            usage_requires_command(EXIT_FAILURE, *argv.add(0));
        }

        if strcmp(*argv.add(1), b"-h\0".as_ptr() as *const c_char) == 0
            || strcmp(*argv.add(1), b"--help\0".as_ptr() as *const c_char) == 0
        {
            usage(EXIT_SUCCESS, b"help\0".as_ptr() as *const c_char);
        }

        if strcmp(*argv.add(1), b"list\0".as_ptr() as *const c_char) == 0 {
            argc -= 1;
            rv_list(argc, argv.add(1));
        }

        if strcmp(*argv.add(1), b"mon\0".as_ptr() as *const c_char) == 0 {
            /*
             * monitor's main should monitor should_stop() function.
             * and exit.
             */
            signal(SIGINT, Some(stop_rv));
            signal(SIGTERM, Some(stop_rv));

            rv_mon(argc - 1, argv.add(1));
        }

        /* invalid sub-command */
        usage_unknown_command(EXIT_FAILURE, *argv.add(0), *argv.add(1));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
