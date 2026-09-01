// SPDX-License-Identifier: GPL-2.0
/*
 * This tool is used by the utimer test, and it allows us to
 * count the ticks of a global timer in a certain time frame
 * (which is set by `timeout` parameter).
 *
 * Author: Ivan Orlov <ivan.orlov0322@gmail.com>
 */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_long};
use std::ptr;

#[repr(C)]
pub struct snd_timer_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_timer_params_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_async_handler_t {
    _private: [u8; 0],
}

const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;

const SND_TIMER_OPEN_NONBLOCK: c_int = 1;

/* Constants from <alsa/asoundlib.h>. */
const SND_TIMER_CLASS_GLOBAL: c_int = 0;
const SND_TIMER_SCLASS_NONE: c_int = 0;

type time_t = c_long;

unsafe extern "C" {
    static mut stdout: *mut c_void;

    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn atoi(nptr: *const c_char) -> c_int;
    fn setlinebuf(stream: *mut c_void);
    fn time(tloc: *mut time_t) -> time_t;

    fn snd_timer_open(
        timer: *mut *mut snd_timer_t,
        name: *const c_char,
        mode: c_int,
    ) -> c_int;
    fn snd_timer_close(timer: *mut snd_timer_t) -> c_int;
    fn snd_timer_start(timer: *mut snd_timer_t) -> c_int;
    fn snd_timer_stop(timer: *mut snd_timer_t) -> c_int;
    fn snd_timer_params(timer: *mut snd_timer_t, params: *mut snd_timer_params_t) -> c_int;
    fn snd_timer_params_set_auto_start(params: *mut snd_timer_params_t, auto_start: c_int);
    fn snd_timer_params_set_ticks(params: *mut snd_timer_params_t, ticks: c_long);
    fn snd_async_add_timer_handler(
        handler: *mut *mut snd_async_handler_t,
        timer: *mut snd_timer_t,
        callback: Option<unsafe extern "C" fn(*mut snd_async_handler_t)>,
        private_data: *mut c_void,
    ) -> c_int;

    /*
     * C uses snd_timer_params_alloca(&params), an ALSA macro which allocates
     * stack storage. Keep the dependency as an external symbol for this
     * source-level translation.
     */
    fn snd_timer_params_alloca(ptr: *mut *mut snd_timer_params_t);
}

static mut ticked: c_int = 0;

unsafe extern "C" fn async_callback(_ahandler: *mut snd_async_handler_t) {
    unsafe {
        ticked += 1;
    }
}

static mut timer_name: [c_char; 64] = [0; 64];

unsafe fn bind_to_timer(device: c_int, subdevice: c_int, timeout: c_int) {
    let mut handle: *mut snd_timer_t = ptr::null_mut();
    let mut params: *mut snd_timer_params_t = ptr::null_mut();
    let mut ahandler: *mut snd_async_handler_t = ptr::null_mut();

    let mut end: time_t;

    unsafe {
        sprintf(
            timer_name.as_mut_ptr(),
            c"hw:CLASS=%d,SCLASS=%d,DEV=%d,SUBDEV=%d".as_ptr(),
            SND_TIMER_CLASS_GLOBAL,
            SND_TIMER_SCLASS_NONE,
            device,
            subdevice,
        );

        snd_timer_params_alloca(&mut params);

        if snd_timer_open(&mut handle, timer_name.as_mut_ptr(), SND_TIMER_OPEN_NONBLOCK) < 0 {
            perror(c"Can't open the timer".as_ptr());
            exit(EXIT_FAILURE);
        }

        snd_timer_params_set_auto_start(params, 1);
        snd_timer_params_set_ticks(params, 1);
        if snd_timer_params(handle, params) < 0 {
            perror(c"Can't set timer params".as_ptr());
            exit(EXIT_FAILURE);
        }

        if snd_async_add_timer_handler(
            &mut ahandler,
            handle,
            Some(async_callback),
            ptr::null_mut(),
        ) < 0
        {
            perror(c"Can't create a handler".as_ptr());
            exit(EXIT_FAILURE);
        }
        end = time(ptr::null_mut()) + timeout as time_t;
        if snd_timer_start(handle) < 0 {
            perror(c"Failed to start the timer".as_ptr());
            exit(EXIT_FAILURE);
        }
        printf(c"Timer has started\n".as_ptr());
        while time(ptr::null_mut()) <= end {
            /*
             * Waiting for the timeout to elapse. Can't use sleep here, as it gets
             * constantly interrupted by the signal from the timer (SIGIO)
             */
        }
        snd_timer_stop(handle);
        snd_timer_close(handle);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let device: c_int;
    let subdevice: c_int;
    let timeout: c_int;

    unsafe {
        if argc < 4 {
            perror(c"Usage: %s <device> <subdevice> <timeout>".as_ptr());
            return EXIT_FAILURE;
        }

        setlinebuf(stdout);

        device = atoi(*argv.add(1));
        subdevice = atoi(*argv.add(2));
        timeout = atoi(*argv.add(3));

        bind_to_timer(device, subdevice, timeout);

        printf(c"Total ticks count: %d\n".as_ptr(), ticked);
    }

    EXIT_SUCCESS
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
