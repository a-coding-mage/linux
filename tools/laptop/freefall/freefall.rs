// SPDX-License-Identifier: GPL-2.0-only
/* Disk protection for HP/DELL machines.
 *
 * Copyright 2008 Eric Piel
 * Copyright 2009 Pavel Machek <pavel@ucw.cz>
 * Copyright 2012 Sonal Santan
 * Copyright 2014 Pali Rohár <pali@kernel.org>
 */

/* C dependencies translated as Rust libc references:
 * stdio.h, stdlib.h, unistd.h, fcntl.h, sys/stat.h, sys/types.h,
 * string.h, stdint.h, errno.h, signal.h, sys/mman.h, sched.h, syslog.h
 */

use core::ffi::{c_char, c_int};
use core::mem::{size_of, MaybeUninit};
use core::ptr;

static mut noled: c_int = 0;
static mut unload_heads_path: [c_char; 64] = [0; 64];
static mut device_path: [c_char; 32] = [0; 32];
static app_name: &[u8] = b"FREE FALL\0";

unsafe fn set_unload_heads_path(device: *mut c_char) -> c_int {
    if libc::strlen(device) <= 5 || libc::strncmp(device, b"/dev/\0".as_ptr() as *const c_char, 5) != 0 {
        return -libc::EINVAL;
    }
    libc::strncpy(
        device_path.as_mut_ptr(),
        device,
        size_of::<[c_char; 32]>() - 1,
    );

    libc::snprintf(
        unload_heads_path.as_mut_ptr(),
        size_of::<[c_char; 64]>() - 1,
        b"/sys/block/%s/device/unload_heads\0".as_ptr() as *const c_char,
        device.add(5),
    );
    0
}

unsafe fn valid_disk() -> c_int {
    let fd = libc::open(unload_heads_path.as_ptr(), libc::O_RDONLY);

    if fd < 0 {
        libc::perror(unload_heads_path.as_ptr());
        return 0;
    }

    libc::close(fd);
    1
}

unsafe fn write_int(path: *mut c_char, i: c_int) {
    let mut buf: [c_char; 1024] = [0; 1024];
    let fd = libc::open(path, libc::O_RDWR);

    if fd < 0 {
        libc::perror(b"open\0".as_ptr() as *const c_char);
        libc::exit(1);
    }

    libc::sprintf(buf.as_mut_ptr(), b"%d\0".as_ptr() as *const c_char, i);

    if libc::write(fd, buf.as_ptr() as *const libc::c_void, libc::strlen(buf.as_ptr()))
        != libc::strlen(buf.as_ptr()) as isize
    {
        libc::perror(b"write\0".as_ptr() as *const c_char);
        libc::exit(1);
    }

    libc::close(fd);
}

unsafe fn set_led(on: c_int) {
    if noled != 0 {
        return;
    }
    write_int(
        b"/sys/class/leds/hp::hddprotect/brightness\0".as_ptr() as *mut c_char,
        on,
    );
}

unsafe fn protect(seconds: c_int) {
    let str_: *const c_char = if seconds == 0 {
        b"Unparked\0".as_ptr() as *const c_char
    } else {
        b"Parked\0".as_ptr() as *const c_char
    };

    write_int(unload_heads_path.as_mut_ptr(), seconds * 1000);
    libc::syslog(
        libc::LOG_INFO,
        b"%s %s disk head\n\0".as_ptr() as *const c_char,
        str_,
        device_path.as_ptr(),
    );
}

unsafe fn on_ac() -> c_int {
    /* /sys/class/power_supply/AC0/online */
    1
}

unsafe fn lid_open() -> c_int {
    /* /proc/acpi/button/lid/LID/state */
    1
}

extern "C" fn ignore_me(_signum: c_int) {
    unsafe {
        protect(0);
        set_led(0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let fd: c_int;
    let mut ret: c_int;
    let mut st = MaybeUninit::<libc::stat>::uninit();
    let mut param = MaybeUninit::<libc::sched_param>::uninit();

    if argc == 1 {
        ret = set_unload_heads_path(b"/dev/sda\0".as_ptr() as *mut c_char);
    } else if argc == 2 {
        ret = set_unload_heads_path(*argv.add(1));
    } else {
        ret = -libc::EINVAL;
    }

    if ret != 0 || valid_disk() == 0 {
        libc::fprintf(
            libc::stderr,
            b"usage: %s <device> (default: /dev/sda)\n\0".as_ptr() as *const c_char,
            *argv.add(0),
        );
        libc::exit(1);
    }

    fd = libc::open(b"/dev/freefall\0".as_ptr() as *const c_char, libc::O_RDONLY);
    if fd < 0 {
        libc::perror(b"/dev/freefall\0".as_ptr() as *const c_char);
        return libc::EXIT_FAILURE;
    }

    if libc::stat(
        b"/sys/class/leds/hp::hddprotect/brightness\0".as_ptr() as *const c_char,
        st.as_mut_ptr(),
    ) != 0
    {
        noled = 1;
    }

    if libc::daemon(0, 0) != 0 {
        libc::perror(b"daemon\0".as_ptr() as *const c_char);
        return libc::EXIT_FAILURE;
    }

    libc::openlog(
        app_name.as_ptr() as *const c_char,
        libc::LOG_CONS | libc::LOG_PID | libc::LOG_NDELAY,
        libc::LOG_LOCAL1,
    );

    (*param.as_mut_ptr()).sched_priority = libc::sched_get_priority_max(libc::SCHED_FIFO);
    libc::sched_setscheduler(0, libc::SCHED_FIFO, param.as_ptr());
    libc::mlockall(libc::MCL_CURRENT | libc::MCL_FUTURE);

    libc::signal(libc::SIGALRM, ignore_me as libc::sighandler_t);

    loop {
        let mut count: u8 = 0;

        ret = libc::read(
            fd,
            &mut count as *mut u8 as *mut libc::c_void,
            size_of::<u8>(),
        ) as c_int;
        libc::alarm(0);
        if ret == -1 && *libc::__errno_location() == libc::EINTR {
            /* Alarm expired, time to unpark the heads */
            continue;
        }

        if ret != size_of::<u8>() as c_int {
            libc::perror(b"read\0".as_ptr() as *const c_char);
            break;
        }

        protect(21);
        set_led(1);
        if 1 != 0 || on_ac() != 0 || lid_open() != 0 {
            libc::alarm(2);
        } else {
            libc::alarm(20);
        }
    }

    libc::closelog();
    libc::close(fd);
    libc::EXIT_SUCCESS
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
