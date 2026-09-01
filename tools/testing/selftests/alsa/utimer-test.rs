// SPDX-License-Identifier: GPL-2.0
/*
 * This test covers the functionality of userspace-driven ALSA timers. Such timers
 * are purely virtual (so they don't directly depend on the hardware), and they could be
 * created and triggered by userspace applications.
 *
 * Author: Ivan Orlov <ivan.orlov0322@gmail.com>
 */

// C dependencies: "kselftest_harness.h", <sound/asound.h>, <unistd.h>,
// <fcntl.h>, <limits.h>, <sys/ioctl.h>, <stdlib.h>, <pthread.h>,
// <string.h>, <errno.h>.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const FRAME_RATE: c_ulong = 8000;
const PERIOD_SIZE: c_ulong = 4410;
const UTIMER_DEFAULT_ID: c_int = -1;
const UTIMER_DEFAULT_FD: c_int = -1;
const NANO: c_ulong = 1000000000;
const TICKS_COUNT: c_int = 10;
const TICKS_RECORDING_DELTA: c_int = 5;
const TIMER_OUTPUT_BUF_LEN: usize = 1024;
const TIMER_FREQ_SEC: c_uint = 1;
const RESULT_PREFIX: &[u8] = b"Total ticks count: \0";
const RESULT_PREFIX_LEN: usize = "Total ticks count: ".len();

type c_uint = u32;
type FILE = c_void;
type pthread_t = c_ulong;

// Provided by <sound/asound.h>.
const SNDRV_TIMER_IOCTL_CREATE: c_ulong = 0;
const SNDRV_TIMER_IOCTL_TRIGGER: c_ulong = 0;
const SNDRV_TIMER_GLOBAL_UDRIVEN: c_int = 0;

// Provided by <fcntl.h> and <errno.h>.
const O_RDONLY: c_int = 0;
const ENOTTY: c_int = 25;
const ENXIO: c_int = 6;

#[repr(C)]
struct snd_timer_uinfo {
    resolution: c_ulong,
    id: c_int,
    fd: c_int,
}

#[repr(C)]
enum timer_app_event {
    TIMER_APP_STARTED,
    TIMER_APP_RESULT,
    TIMER_NO_EVENT,
}

#[repr(C)]
struct timer_f {
    utimer_info: *mut snd_timer_uinfo,
}

unsafe extern "C" {
    fn geteuid() -> c_uint;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn malloc(size: usize) -> *mut c_void;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn popen(command: *const c_char, type_: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn pclose(stream: *mut FILE) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn __errno_location() -> *mut c_int;
}

// Provided by kselftest_harness.h.
macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {};
}
macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {};
}
macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {};
}
macro_rules! ASSERT_LT {
    ($left:expr, $right:expr) => {};
}
macro_rules! SKIP {
    ($ret:expr, $msg:expr) => {};
}

unsafe fn timer_f_setup(self_: *mut timer_f) {
    let timer_dev_fd: c_int;

    if unsafe { geteuid() } != 0 {
        SKIP!(return, "This test needs root to run!");
    }

    unsafe {
        (*self_).utimer_info = calloc(1, core::mem::size_of::<snd_timer_uinfo>()) as *mut snd_timer_uinfo;
    }
    ASSERT_NE!(core::ptr::null_mut::<snd_timer_uinfo>(), unsafe { (*self_).utimer_info });

    /* Resolution is the time the period of frames takes in nanoseconds */
    unsafe {
        (*(*self_).utimer_info).resolution = NANO / FRAME_RATE * PERIOD_SIZE;
    }

    timer_dev_fd = unsafe { open(c"/dev/snd/timer".as_ptr(), O_RDONLY) };
    ASSERT_GE!(timer_dev_fd, 0);

    if unsafe { ioctl(timer_dev_fd, SNDRV_TIMER_IOCTL_CREATE, (*self_).utimer_info) } < 0 {
        let err: c_int = unsafe { *__errno_location() };

        unsafe {
            close(timer_dev_fd);
        }
        if err == ENOTTY || err == ENXIO {
            SKIP!(return, "CONFIG_SND_UTIMER not enabled");
        }
        ASSERT_EQ!(err, 0);
    }
    ASSERT_GE!(unsafe { (*(*self_).utimer_info).fd }, 0);

    unsafe {
        close(timer_dev_fd);
    }
}

unsafe fn timer_f_teardown(self_: *mut timer_f) {
    unsafe {
        close((*(*self_).utimer_info).fd);
        free((*self_).utimer_info as *mut c_void);
    }
}

unsafe extern "C" fn ticking_func(data: *mut c_void) -> *mut c_void {
    let mut i: c_int;
    let fd: *mut c_int = data as *mut c_int;

    i = 0;
    while i < TICKS_COUNT {
        /* Well, trigger the timer! */
        unsafe {
            ioctl(*fd, SNDRV_TIMER_IOCTL_TRIGGER, core::ptr::null_mut::<c_void>());
            sleep(TIMER_FREQ_SEC);
        }
        i += 1;
    }

    core::ptr::null_mut()
}

unsafe fn parse_timer_output(s: *const c_char) -> timer_app_event {
    if !unsafe { strstr(s, c"Timer has started".as_ptr()) }.is_null() {
        return timer_app_event::TIMER_APP_STARTED;
    }
    if !unsafe { strstr(s, c"Total ticks count".as_ptr()) }.is_null() {
        return timer_app_event::TIMER_APP_RESULT;
    }

    timer_app_event::TIMER_NO_EVENT
}

unsafe fn parse_timer_result(s: *const c_char) -> c_int {
    let mut end: *mut c_char = core::ptr::null_mut();
    let d: c_long;

    d = unsafe { strtol(s.add(RESULT_PREFIX_LEN), &mut end, 10) };
    if end == unsafe { s.add(RESULT_PREFIX_LEN) } as *mut c_char {
        return -1;
    }

    d as c_int
}

/*
 * This test triggers the timer and counts ticks at the same time. The amount
 * of the timer trigger calls should be equal to the amount of ticks received.
 */
unsafe fn test_timer_f_utimer(self_: *mut timer_f) {
    let mut command: [c_char; 64] = [0; 64];
    let mut ticking_thread: pthread_t = 0;
    let mut total_ticks: c_int = 0;
    let rfp: *mut FILE;
    let buf: *mut c_char = unsafe { malloc(TIMER_OUTPUT_BUF_LEN) as *mut c_char };

    ASSERT_NE!(buf, core::ptr::null_mut::<c_char>());

    /* The timeout should be the ticks interval * count of ticks + some delta */
    unsafe {
        sprintf(
            command.as_mut_ptr(),
            c"./global-timer %d %d %d".as_ptr(),
            SNDRV_TIMER_GLOBAL_UDRIVEN,
            (*(*self_).utimer_info).id,
            TICKS_COUNT * TIMER_FREQ_SEC as c_int + TICKS_RECORDING_DELTA,
        );
    }

    rfp = unsafe { popen(command.as_ptr(), c"r".as_ptr()) };
    while !unsafe { fgets(buf, TIMER_OUTPUT_BUF_LEN as c_int, rfp) }.is_null() {
        unsafe {
            *buf.add(TIMER_OUTPUT_BUF_LEN - 1) = 0;
        }
        match unsafe { parse_timer_output(buf) } {
            timer_app_event::TIMER_APP_STARTED => {
                /* global-timer waits for timer to trigger, so start the ticking thread */
                unsafe {
                    pthread_create(
                        &mut ticking_thread,
                        core::ptr::null(),
                        ticking_func,
                        &mut (*(*self_).utimer_info).fd as *mut c_int as *mut c_void,
                    );
                }
            }
            timer_app_event::TIMER_APP_RESULT => {
                total_ticks = unsafe { parse_timer_result(buf) };
            }
            timer_app_event::TIMER_NO_EVENT => {}
        }
    }
    unsafe {
        pthread_join(ticking_thread, core::ptr::null_mut());
    }
    ASSERT_EQ!(total_ticks, TICKS_COUNT);
    unsafe {
        pclose(rfp);
        free(buf as *mut c_void);
    }
}

unsafe fn test_wrong_timers_test() {
    let timer_dev_fd: c_int;
    let utimer_fd: c_int;
    let mut wrong_timer = snd_timer_uinfo {
        resolution: 0,
        id: UTIMER_DEFAULT_ID,
        fd: UTIMER_DEFAULT_FD,
    };

    timer_dev_fd = unsafe { open(c"/dev/snd/timer".as_ptr(), O_RDONLY) };
    ASSERT_GE!(timer_dev_fd, 0);

    utimer_fd = unsafe { ioctl(timer_dev_fd, SNDRV_TIMER_IOCTL_CREATE, &mut wrong_timer) };
    ASSERT_LT!(utimer_fd, 0);
    /* Check that id was not updated */
    ASSERT_EQ!(wrong_timer.id, UTIMER_DEFAULT_ID);

    /* Test the NULL as an argument is processed correctly */
    ASSERT_LT!(
        unsafe { ioctl(timer_dev_fd, SNDRV_TIMER_IOCTL_CREATE, core::ptr::null_mut::<c_void>()) },
        0
    );

    unsafe {
        close(timer_dev_fd);
    }
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
