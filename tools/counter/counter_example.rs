// SPDX-License-Identifier: GPL-2.0-only
/* Counter - example userspace application
 *
 * The userspace application opens /dev/counter0, configures the
 * COUNTER_EVENT_INDEX event channel 0 to gather Count 0 count and Count
 * 1 count, and prints out the data as it becomes available on the
 * character device node.
 *
 * Copyright (C) 2021 William Breathitt Gray
 */
// C dependencies translated from:
// #include <errno.h>
// #include <fcntl.h>
// #include <linux/counter.h>
// #include <stdio.h>
// #include <string.h>
// #include <sys/ioctl.h>
// #include <unistd.h>

use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};

type __u8 = u8;
type __aligned_u64 = u64;

const O_RDWR: c_int = 0o00000002;
const EIO: c_int = 5;

// From <linux/counter.h>.
const COUNTER_COMPONENT_COUNT: __u8 = 2;
const COUNTER_SCOPE_COUNT: __u8 = 2;
const COUNTER_EVENT_INDEX: __u8 = 4;

// ioctl request values are C preprocessor macros supplied by <linux/counter.h>.
// They are referenced here as external values to preserve the dependency.
unsafe extern "C" {
    static COUNTER_ADD_WATCH_IOCTL: c_ulong;
    static COUNTER_ENABLE_EVENTS_IOCTL: c_ulong;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct counter_component {
    type_: __u8,
    scope: __u8,
    parent: __u8,
    id: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct counter_watch {
    component: counter_component,
    event: __u8,
    channel: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct counter_event {
    timestamp: __aligned_u64,
    value: __aligned_u64,
    status: __u8,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn perror(s: *const c_char);
    fn strerror(errnum: c_int) -> *mut c_char;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;

    static mut stderr: *mut FILE;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

static mut watches: [counter_watch; 2] = [
    counter_watch {
        /* Component data: Count 0 count */
        component: counter_component {
            type_: COUNTER_COMPONENT_COUNT,
            scope: COUNTER_SCOPE_COUNT,
            parent: 0,
            id: 0,
        },
        /* Event type: Index */
        event: COUNTER_EVENT_INDEX,
        /* Device event channel 0 */
        channel: 0,
    },
    counter_watch {
        /* Component data: Count 1 count */
        component: counter_component {
            type_: COUNTER_COMPONENT_COUNT,
            scope: COUNTER_SCOPE_COUNT,
            parent: 1,
            id: 0,
        },
        /* Event type: Index */
        event: COUNTER_EVENT_INDEX,
        /* Device event channel 0 */
        channel: 0,
    },
];

unsafe fn c_str(ptr: *const c_char) -> *const c_char {
    ptr
}

fn main() {
    unsafe {
        let mut fd: c_int;
        let mut ret: c_int;
        let mut i: c_int;
        let mut event_data: [counter_event; 2] = [counter_event {
            timestamp: 0,
            value: 0,
            status: 0,
        }; 2];

        fd = open(c_str(b"/dev/counter0\0".as_ptr() as *const c_char), O_RDWR);
        if fd == -1 {
            perror(c_str(b"Unable to open /dev/counter0\0".as_ptr() as *const c_char));
            std::process::exit(1);
        }

        i = 0;
        while i < 2 {
            ret = ioctl(
                fd,
                COUNTER_ADD_WATCH_IOCTL,
                watches.as_mut_ptr().offset(i as isize),
            );
            if ret == -1 {
                fprintf(
                    stderr,
                    c_str(b"Error adding watches[%d]: %s\n\0".as_ptr() as *const c_char),
                    i,
                    strerror(errno),
                );
                std::process::exit(1);
            }
            i += 1;
        }
        ret = ioctl(fd, COUNTER_ENABLE_EVENTS_IOCTL);
        if ret == -1 {
            perror(c_str(b"Error enabling events\0".as_ptr() as *const c_char));
            std::process::exit(1);
        }

        loop {
            let read_ret: isize = read(
                fd,
                event_data.as_mut_ptr() as *mut c_void,
                size_of::<[counter_event; 2]>(),
            );
            ret = read_ret as c_int;
            if ret == -1 {
                perror(c_str(b"Failed to read event data\0".as_ptr() as *const c_char));
                std::process::exit(1);
            }

            if read_ret as usize != size_of::<[counter_event; 2]>() {
                fprintf(
                    stderr,
                    c_str(b"Failed to read event data\n\0".as_ptr() as *const c_char),
                );
                std::process::exit((-EIO) as i32);
            }

            printf(
                c_str(
                    b"Timestamp 0: %llu\tCount 0: %llu\n\
                      Error Message 0: %s\n\
                      Timestamp 1: %llu\tCount 1: %llu\n\
                      Error Message 1: %s\n\0"
                        .as_ptr() as *const c_char,
                ),
                event_data[0].timestamp,
                event_data[0].value,
                strerror(event_data[0].status as c_int),
                event_data[1].timestamp,
                event_data[1].value,
                strerror(event_data[1].status as c_int),
            );
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
