// SPDX-License-Identifier: GPL-2.0

/*
 * video_device_test - Video Device Test
 *
 * Copyright (c) 2016 Shuah Khan <shuahkh@osg.samsung.com>
 * Copyright (c) 2016 Samsung Electronics Co., Ltd.
 *
 */

/*
 * This file adds a test for Video Device. This test should not be included
 * in the Kselftest run. This test should be run when hardware and driver
 * that makes use of V4L2 API is present.
 *
 * This test opens user specified Video Device and calls video ioctls in a
 * loop once every 10 seconds.
 *
 * Usage:
 *	sudo ./video_device_test -d /dev/videoX
 *
 *	While test is running, remove the device or unbind the driver and
 *	ensure there are no use after free errors and other Oops in the
 *	dmesg.
 *	When possible, enable KaSan kernel config option for use-after-free
 *	error detection.
*/

/*
 * C dependencies removed from executable Rust:
 * stdio.h, unistd.h, stdlib.h, errno.h, string.h, fcntl.h, sys/ioctl.h,
 * sys/stat.h, time.h, linux/videodev2.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const PRIORITY_MAX: c_uint = 4;

type v4l2_priority = c_uint;

#[repr(C)]
pub struct v4l2_tuner {
    pub index: c_uint,
    pub name: [c_char; 32],
    pub type_: c_uint,
    pub capability: c_uint,
    pub rangelow: c_uint,
    pub rangehigh: c_uint,
    pub rxsubchans: c_uint,
    pub audmode: c_uint,
    pub signal: c_int,
    pub afc: c_int,
    pub reserved: [c_uint; 4],
}

#[repr(C)]
pub struct v4l2_capability {
    pub driver: [c_char; 16],
    pub card: [c_char; 32],
    pub bus_info: [c_char; 32],
    pub version: c_uint,
    pub capabilities: c_uint,
    pub device_caps: c_uint,
    pub reserved: [c_uint; 3],
}

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn printf(format: *const c_char, ...) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn srand(seed: c_uint);
    fn rand() -> c_int;
    fn time(tloc: *mut isize) -> isize;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
}

unsafe extern "C" {
    static VIDIOC_G_PRIORITY: c_ulong;
    static VIDIOC_S_PRIORITY: c_ulong;
    static VIDIOC_QUERYCAP: c_ulong;
    static VIDIOC_G_TUNER: c_ulong;
    static O_RDWR: c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

pub unsafe extern "C" fn priority_test(fd: c_int) -> c_int {
    /* This test will try to update the priority associated with a file descriptor */

    let mut old_priority: v4l2_priority = 0;
    let mut new_priority: v4l2_priority;
    let mut priority_to_compare: v4l2_priority = 0;
    let mut ret: c_int;
    let mut result: c_int = 0;

    ret = ioctl(fd, VIDIOC_G_PRIORITY, &mut old_priority as *mut v4l2_priority);
    if ret < 0 {
        printf(
            c"Failed to get priority: %s\n".as_ptr(),
            strerror(errno()),
        );
        return -1;
    }
    new_priority = (old_priority + 1) % PRIORITY_MAX;
    ret = ioctl(fd, VIDIOC_S_PRIORITY, &mut new_priority as *mut v4l2_priority);
    if ret < 0 {
        printf(
            c"Failed to set priority: %s\n".as_ptr(),
            strerror(errno()),
        );
        return -1;
    }
    ret = ioctl(
        fd,
        VIDIOC_G_PRIORITY,
        &mut priority_to_compare as *mut v4l2_priority,
    );
    if ret < 0 {
        printf(
            c"Failed to get new priority: %s\n".as_ptr(),
            strerror(errno()),
        );
        result = -1;
    } else if priority_to_compare != new_priority {
        printf(c"Priority wasn't set - test failed\n".as_ptr());
        result = -1;
    }

    ret = ioctl(fd, VIDIOC_S_PRIORITY, &mut old_priority as *mut v4l2_priority);
    if ret < 0 {
        printf(
            c"Failed to restore priority: %s\n".as_ptr(),
            strerror(errno()),
        );
        return -1;
    }
    result
}

pub unsafe extern "C" fn loop_test(fd: c_int) -> c_int {
    let mut count: c_int;
    let mut vtuner: v4l2_tuner = core::mem::zeroed();
    let mut vcap: v4l2_capability = core::mem::zeroed();
    let mut ret: c_int;

    /* Generate random number of interations */
    srand(time(core::ptr::null_mut()) as c_uint);
    count = rand();

    printf(
        c"\nNote:\nWhile test is running, remove the device or unbind\ndriver and ensure there are no use after free errors\nand other Oops in the dmesg. When possible, enable KaSan\nkernel config option for use-after-free error detection.\n\n".as_ptr(),
    );

    while count > 0 {
        ret = ioctl(fd, VIDIOC_QUERYCAP, &mut vcap as *mut v4l2_capability);
        if ret < 0 {
            printf(
                c"VIDIOC_QUERYCAP errno %s\n".as_ptr(),
                strerror(errno()),
            );
        } else {
            printf(
                c"Video device driver %s\n".as_ptr(),
                vcap.driver.as_mut_ptr(),
            );
        }

        ret = ioctl(fd, VIDIOC_G_TUNER, &mut vtuner as *mut v4l2_tuner);
        if ret < 0 {
            printf(
                c"VIDIOC_G_TUNER, errno %s\n".as_ptr(),
                strerror(errno()),
            );
        } else {
            printf(
                c"type %d rangelow %d rangehigh %d\n".as_ptr(),
                vtuner.type_,
                vtuner.rangelow,
                vtuner.rangehigh,
            );
        }
        sleep(10);
        count -= 1;
    }
    0
}

pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut opt: c_int;
    let mut video_dev: [c_char; 256] = core::mem::zeroed();
    let mut fd: c_int;
    let mut test_result: c_int;

    if argc < 2 {
        printf(c"Usage: %s [-d </dev/videoX>]\n".as_ptr(), *argv.offset(0));
        exit(-1);
    }

    /* Process arguments */
    loop {
        opt = getopt(argc, argv, c"d:".as_ptr());
        if opt == -1 {
            break;
        }
        match opt {
            100 => {
                strncpy(
                    video_dev.as_mut_ptr(),
                    optarg,
                    core::mem::size_of_val(&video_dev) - 1,
                );
                video_dev[core::mem::size_of_val(&video_dev) - 1] = b'\0' as c_char;
            }
            _ => {
                printf(c"Usage: %s [-d </dev/videoX>]\n".as_ptr(), *argv.offset(0));
                exit(-1);
            }
        }
    }

    /* Open Video device and keep it open */
    fd = open(video_dev.as_mut_ptr(), O_RDWR);
    if fd == -1 {
        printf(
            c"Video Device open errno %s\n".as_ptr(),
            strerror(errno()),
        );
        exit(-1);
    }

    test_result = priority_test(fd);
    if test_result == 0 {
        printf(c"Priority test - PASSED\n".as_ptr());
    } else {
        printf(c"Priority test - FAILED\n".as_ptr());
    }

    loop_test(fd);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
