// SPDX-License-Identifier: GPL-2.0-or-later
/* $(CROSS_COMPILE)cc -Wall -Wextra -g -lpthread -o testusb testusb.c */

/*
 * Copyright (c) 2002 by David Brownell
 * Copyright (c) 2010 by Samsung Electronics
 * Author: Michal Nazarewicz <mina86@mina86.com>
 */

/*
 * This program issues ioctls to perform the tests implemented by the
 * kernel driver.  It can generate a variety of transfer patterns; you
 * should make sure to test both regular streaming and mixes of
 * transfer sizes (including short transfers).
 *
 * For more information on how this can be used and on USB testing
 * refer to <URL:http://www.linux-usb.org/usbtest/>.
 */

/* C dependencies: stdio.h, string.h, ftw.h, stdlib.h, pthread.h, unistd.h,
 * errno.h, limits.h, sys/types.h, sys/stat.h, fcntl.h, sys/ioctl.h,
 * linux/usbdevice_fs.h
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

const TEST_CASES: c_int = 30;

// FIXME make these public somewhere; usbdevfs.h?

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbtest_param {
    // inputs
    pub test_num: c_uint, /* 0..(TEST_CASES-1) */
    pub iterations: c_uint,
    pub length: c_uint,
    pub vary: c_uint,
    pub sglen: c_uint,

    // outputs
    pub duration: timeval,
}

const IOC_NRBITS: c_ulong = 8;
const IOC_TYPEBITS: c_ulong = 8;
const IOC_SIZEBITS: c_ulong = 14;
const IOC_NRSHIFT: c_ulong = 0;
const IOC_TYPESHIFT: c_ulong = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: c_ulong = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: c_ulong = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: c_ulong = 1;
const IOC_READ: c_ulong = 2;

const fn ioc(dir: c_ulong, type_: c_ulong, nr: c_ulong, size: c_ulong) -> c_ulong {
    (dir << IOC_DIRSHIFT) | (type_ << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}

const fn iowr(type_: c_ulong, nr: c_ulong, size: c_ulong) -> c_ulong {
    ioc(IOC_READ | IOC_WRITE, type_, nr, size)
}

const USBTEST_REQUEST: c_ulong = iowr(b'U' as c_ulong, 100, size_of::<usbtest_param>() as c_ulong);

/*-------------------------------------------------------------------------*/

/* #include <linux/usb_ch9.h> */

const USB_DT_DEVICE: u8 = 0x01;
const USB_DT_INTERFACE: u8 = 0x04;

const USB_CLASS_PER_INTERFACE: u8 = 0; /* for DeviceClass */
const USB_CLASS_VENDOR_SPEC: u8 = 0xff;

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct usb_device_descriptor {
    bLength: u8,
    bDescriptorType: u8,
    bcdUSB: u16,
    bDeviceClass: u8,
    bDeviceSubClass: u8,
    bDeviceProtocol: u8,
    bMaxPacketSize0: u8,
    idVendor: u16,
    idProduct: u16,
    bcdDevice: u16,
    iManufacturer: u8,
    iProduct: u8,
    iSerialNumber: u8,
    bNumConfigurations: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct usb_interface_descriptor {
    bLength: u8,
    bDescriptorType: u8,

    bInterfaceNumber: u8,
    bAlternateSetting: u8,
    bNumEndpoints: u8,
    bInterfaceClass: u8,
    bInterfaceSubClass: u8,
    bInterfaceProtocol: u8,
    iInterface: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum usb_device_speed {
    USB_SPEED_UNKNOWN = 0, /* enumerating */
    USB_SPEED_LOW,
    USB_SPEED_FULL, /* usb 1.1 */
    USB_SPEED_HIGH, /* usb 2.0 */
    USB_SPEED_WIRELESS, /* wireless (usb 2.5) */
    USB_SPEED_SUPER, /* usb 3.0 */
    USB_SPEED_SUPER_PLUS, /* usb 3.1 */
}

/*-------------------------------------------------------------------------*/

unsafe fn speed(s: usb_device_speed) -> *mut c_char {
    match s {
        usb_device_speed::USB_SPEED_UNKNOWN => c"unknown".as_ptr() as *mut c_char,
        usb_device_speed::USB_SPEED_LOW => c"low".as_ptr() as *mut c_char,
        usb_device_speed::USB_SPEED_FULL => c"full".as_ptr() as *mut c_char,
        usb_device_speed::USB_SPEED_HIGH => c"high".as_ptr() as *mut c_char,
        usb_device_speed::USB_SPEED_WIRELESS => c"wireless".as_ptr() as *mut c_char,
        usb_device_speed::USB_SPEED_SUPER => c"super".as_ptr() as *mut c_char,
        usb_device_speed::USB_SPEED_SUPER_PLUS => c"super-plus".as_ptr() as *mut c_char,
    }
}

type pthread_t = c_ulong;

#[repr(C)]
struct testdev {
    next: *mut testdev,
    name: *mut c_char,
    thread: pthread_t,
    speed: usb_device_speed,
    ifnum: c_uint,
    forever: c_uint,
    test: c_int,

    param: usbtest_param,
}

static mut TESTDEVS: *mut testdev = null_mut();

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct stat {
    _private: [u8; 0],
}

#[repr(C)]
struct usbdevfs_ioctl {
    ifno: c_int,
    ioctl_code: c_int,
    data: *mut c_void,
}

const FTW_F: c_int = 1;
const O_RDWR: c_int = 0o2;
const F_OK: c_int = 0;
const EOF: c_int = -1;
const EOPNOTSUPP: c_int = 95;
const UINT_MAX: c_ulong = c_uint::MAX as c_ulong;
const USBDEVFS_IOCTL: c_ulong = iowr(b'U' as c_ulong, 18, size_of::<usbdevfs_ioctl>() as c_ulong);
const USBDEVFS_GET_SPEED: c_ulong = 21780;

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut optarg: *mut c_char;
    static mut optind: c_int;

    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn perror(s: *const c_char);
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> c_int;
    fn snprintf(str: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    fn ftw(
        dirpath: *const c_char,
        fn_: Option<unsafe extern "C" fn(*const c_char, *const stat, c_int) -> c_int>,
        nopenfd: c_int,
    ) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
}

unsafe fn testdev_ffs_ifnum(fd: *mut FILE) -> c_int {
    #[repr(C)]
    union U {
        buf: [c_char; 255],
        intf: usb_interface_descriptor,
    }

    let mut u = U { buf: [0; 255] };

    loop {
        if fread(unsafe { u.buf.as_mut_ptr() } as *mut c_void, 1, 1, fd) != 1 {
            return -1;
        }
        if fread(
            unsafe { u.buf.as_mut_ptr().add(1) } as *mut c_void,
            (unsafe { u.buf[0] } as u8).wrapping_sub(1) as usize,
            1,
            fd,
        ) != 1
        {
            return -1;
        }

        let intf = unsafe { u.intf };
        if intf.bLength as usize == size_of::<usb_interface_descriptor>()
            && intf.bDescriptorType == USB_DT_INTERFACE
            && intf.bNumEndpoints == 2
            && intf.bInterfaceClass == USB_CLASS_VENDOR_SPEC
            && intf.bInterfaceSubClass == 0
            && intf.bInterfaceProtocol == 0
        {
            return intf.bInterfaceNumber as u8 as c_int;
        }
    }
}

unsafe fn testdev_ifnum(fd: *mut FILE) -> c_int {
    let mut dev: usb_device_descriptor = unsafe { zeroed() };

    if fread(
        &mut dev as *mut usb_device_descriptor as *mut c_void,
        size_of::<usb_device_descriptor>(),
        1,
        fd,
    ) != 1
    {
        return -1;
    }

    if dev.bLength as usize != size_of::<usb_device_descriptor>() || dev.bDescriptorType != USB_DT_DEVICE {
        return -1;
    }

    /* FX2 with (tweaked) bulksrc firmware */
    if dev.idVendor == 0x0547 && dev.idProduct == 0x1002 {
        return 0;
    }

    /*----------------------------------------------------*/

    /* devices that start up using the EZ-USB default device and
     * which we can use after loading simple firmware.  hotplug
     * can fxload it, and then run this test driver.
     *
     * we return false positives in two cases:
     * - the device has a "real" driver (maybe usb-serial) that
     *   renumerates.  the device should vanish quickly.
     * - the device doesn't have the test firmware installed.
     */

    /* generic EZ-USB FX controller */
    if dev.idVendor == 0x0547 && dev.idProduct == 0x2235 {
        return 0;
    }

    /* generic EZ-USB FX2 controller */
    if dev.idVendor == 0x04b4 && dev.idProduct == 0x8613 {
        return 0;
    }

    /* CY3671 development board with EZ-USB FX */
    if dev.idVendor == 0x0547 && dev.idProduct == 0x0080 {
        return 0;
    }

    /* Keyspan 19Qi uses an21xx (original EZ-USB) */
    if dev.idVendor == 0x06cd && dev.idProduct == 0x010b {
        return 0;
    }

    /*----------------------------------------------------*/

    /* "gadget zero", Linux-USB test software */
    if dev.idVendor == 0x0525 && dev.idProduct == 0xa4a0 {
        return 0;
    }

    /* user mode subset of that */
    if dev.idVendor == 0x0525 && dev.idProduct == 0xa4a4 {
        return testdev_ffs_ifnum(fd);
        /* return 0; */
    }

    /* iso version of usermode code */
    if dev.idVendor == 0x0525 && dev.idProduct == 0xa4a3 {
        return 0;
    }

    /* some GPL'd test firmware uses these IDs */

    if dev.idVendor == 0xfff0 && dev.idProduct == 0xfff0 {
        return 0;
    }

    /*----------------------------------------------------*/

    /* iBOT2 high speed webcam */
    if dev.idVendor == 0x0b62 && dev.idProduct == 0x0059 {
        return 0;
    }

    /*----------------------------------------------------*/

    /* the FunctionFS gadget can have the source/sink interface
     * anywhere.  We look for an interface descriptor that match
     * what we expect.  We ignore configuratiens thou. */

    if dev.idVendor == 0x0525
        && dev.idProduct == 0xa4ac
        && (dev.bDeviceClass == USB_CLASS_PER_INTERFACE || dev.bDeviceClass == USB_CLASS_VENDOR_SPEC)
    {
        return testdev_ffs_ifnum(fd);
    }

    -1
}

unsafe extern "C" fn find_testdev(name: *const c_char, sb: *const stat, flag: c_int) -> c_int {
    let mut fd: *mut FILE;
    let ifnum: c_int;
    let mut entry: *mut testdev;

    let _ = sb; /* unused */

    if flag != FTW_F {
        return 0;
    }

    fd = fopen(name, c"rb".as_ptr());
    if fd.is_null() {
        perror(name);
        return 0;
    }

    ifnum = testdev_ifnum(fd);
    fclose(fd);
    if ifnum < 0 {
        return 0;
    }

    entry = calloc(1, size_of::<testdev>()) as *mut testdev;
    if entry.is_null() {
        perror(c"malloc".as_ptr());
        return 0;
    }

    (*entry).name = strdup(name);
    if (*entry).name.is_null() {
        free(entry as *mut c_void);
        perror(c"malloc".as_ptr());
        return 0;
    }

    (*entry).ifnum = ifnum as c_uint;
    (*entry).next = TESTDEVS;
    TESTDEVS = entry;
    0
}

unsafe fn usbdev_ioctl(fd: c_int, ifno: c_int, request: c_uint, param: *mut c_void) -> c_int {
    let mut wrapper: usbdevfs_ioctl = unsafe { zeroed() };

    wrapper.ifno = ifno;
    wrapper.ioctl_code = request as c_int;
    wrapper.data = param;

    ioctl(fd, USBDEVFS_IOCTL, &mut wrapper as *mut usbdevfs_ioctl)
}

unsafe extern "C" fn handle_testdev(arg: *mut c_void) -> *mut c_void {
    let dev = arg as *mut testdev;
    let mut fd: c_int;
    let mut i: c_int;
    let mut status: c_int;

    fd = open((*dev).name, O_RDWR);
    if fd < 0 {
        perror(c"can't open dev file r/w".as_ptr());
        return null_mut();
    }

    status = ioctl(fd, USBDEVFS_GET_SPEED, null::<c_void>());
    if status < 0 {
        fprintf(stderr, c"USBDEVFS_GET_SPEED failed %d\n".as_ptr(), status);
    } else {
        (*dev).speed = core::mem::transmute::<c_int, usb_device_speed>(status);
    }
    fprintf(
        stderr,
        c"%s speed\t%s\t%u\n".as_ptr(),
        speed((*dev).speed),
        (*dev).name,
        (*dev).ifnum,
    );

    loop {
        i = 0;
        while i < TEST_CASES {
            if (*dev).test != -1 && (*dev).test != i {
                i += 1;
                continue;
            }
            (*dev).param.test_num = i as c_uint;

            status = usbdev_ioctl(
                fd,
                (*dev).ifnum as c_int,
                USBTEST_REQUEST as c_uint,
                &mut (*dev).param as *mut usbtest_param as *mut c_void,
            );
            if status < 0 && errno == EOPNOTSUPP {
                i += 1;
                continue;
            }

            /* FIXME need a "syslog it" option for background testing */

            /* NOTE: each thread emits complete lines; no fragments! */
            if status < 0 {
                let mut buf = [0 as c_char; 80];
                let err = errno;

                if strerror_r(errno, buf.as_mut_ptr(), size_of::<[c_char; 80]>()) != 0 {
                    snprintf(buf.as_mut_ptr(), size_of::<[c_char; 80]>(), c"error %d".as_ptr(), err);
                    errno = err;
                }
                printf(
                    c"%s test %d --> %d (%s)\n".as_ptr(),
                    (*dev).name,
                    i,
                    errno,
                    buf.as_mut_ptr(),
                );
            } else {
                printf(
                    c"%s test %d, %4d.%.06d secs\n".as_ptr(),
                    (*dev).name,
                    i,
                    (*dev).param.duration.tv_sec as c_int,
                    (*dev).param.duration.tv_usec as c_int,
                );
            }

            fflush(stdout);
            i += 1;
        }
        if (*dev).forever == 0 {
            break;
        }
    }

    close(fd);
    arg
}

unsafe fn usb_dir_find() -> *const c_char {
    static UDEV_USB_PATH: &[u8] = b"/dev/bus/usb\0";

    if access(UDEV_USB_PATH.as_ptr() as *const c_char, F_OK) == 0 {
        return UDEV_USB_PATH.as_ptr() as *const c_char;
    }

    null()
}

unsafe fn parse_num(num: *mut c_uint, str_: *const c_char) -> c_int {
    let mut val: c_ulong;
    let mut end: *mut c_char = null_mut();

    errno = 0;
    val = strtoul(str_, &mut end as *mut *mut c_char, 0);
    if errno != 0 || *end != 0 || val > UINT_MAX {
        return -1;
    }
    *num = val as c_uint;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut c: c_int;
    let mut entry: *mut testdev;
    let mut device: *mut c_char;
    let mut usb_dir: *const c_char = null();
    let mut all: c_int = 0;
    let mut forever: c_int = 0;
    let mut not: c_int = 0;
    let mut test: c_int = -1; /* all */
    let mut param: usbtest_param = unsafe { zeroed() };

    /* pick defaults that works with all speeds, without short packets.
     *
     * Best per-frame data rates:
     *     super speed,bulk      1024 * 16 * 8 = 131072
     *                 interrupt 1024 *  3 * 8 =  24576
     *     high speed, bulk       512 * 13 * 8 =  53248
     *                 interrupt 1024 *  3 * 8 =  24576
     *     full speed, bulk/intr   64 * 19     =   1216
     *                 interrupt   64 *  1     =     64
     *      low speed, interrupt    8 *  1     =      8
     */
    param.iterations = 1000;
    param.length = 1024;
    param.vary = 1024;
    param.sglen = 32;

    /* for easy use when hotplugging */
    device = getenv(c"DEVICE".as_ptr());

    loop {
        c = getopt(argc, argv as *const *mut c_char, c"D:aA:c:g:hlns:t:v:".as_ptr());
        if c == EOF {
            break;
        }
        match c as u8 as char {
            'D' => {
                /* device, if only one */
                device = optarg;
                continue;
            }
            'A' => {
                /* use all devices with specified USB dir */
                usb_dir = optarg;
                /* FALL THROUGH */
                device = null_mut();
                all = 1;
                continue;
            }
            'a' => {
                /* use all devices */
                device = null_mut();
                all = 1;
                continue;
            }
            'c' => {
                /* count iterations */
                if parse_num(&mut param.iterations as *mut c_uint, optarg) != 0 {
                    return usage(argv);
                }
                continue;
            }
            'g' => {
                /* scatter/gather entries */
                if parse_num(&mut param.sglen as *mut c_uint, optarg) != 0 {
                    return usage(argv);
                }
                continue;
            }
            'l' => {
                /* loop forever */
                forever = 1;
                continue;
            }
            'n' => {
                /* no test running! */
                not = 1;
                continue;
            }
            's' => {
                /* size of packet */
                if parse_num(&mut param.length as *mut c_uint, optarg) != 0 {
                    return usage(argv);
                }
                continue;
            }
            't' => {
                /* run just one test */
                test = atoi(optarg);
                if test < 0 {
                    return usage(argv);
                }
                continue;
            }
            'v' => {
                /* vary packet size by ... */
                if parse_num(&mut param.vary as *mut c_uint, optarg) != 0 {
                    return usage(argv);
                }
                continue;
            }
            '?' | 'h' | _ => return usage(argv),
        }
    }
    if optind != argc {
        return usage(argv);
    }
    if all == 0 && device.is_null() {
        fprintf(
            stderr,
            c"must specify '-a' or '-D dev', or DEVICE=/dev/bus/usb/BBB/DDD in env\n".as_ptr(),
        );
        return usage(argv);
    }

    /* Find usb device subdirectory */
    if usb_dir.is_null() {
        usb_dir = usb_dir_find();
        if usb_dir.is_null() {
            fputs(c"USB device files are missing\n".as_ptr(), stderr);
            return -1;
        }
    }

    /* collect and list the test devices */
    if ftw(usb_dir, Some(find_testdev), 3) != 0 {
        fputs(c"ftw failed; are USB device files missing?\n".as_ptr(), stderr);
        return -1;
    }

    /* quit, run single test, or create test threads */
    if TESTDEVS.is_null() && device.is_null() {
        fputs(c"no test devices recognized\n".as_ptr(), stderr);
        return -1;
    }
    if not != 0 {
        return 0;
    }
    if !TESTDEVS.is_null() && (*TESTDEVS).next.is_null() && device.is_null() {
        device = (*TESTDEVS).name;
    }
    entry = TESTDEVS;
    while !entry.is_null() {
        let mut status: c_int;

        (*entry).param = param;
        (*entry).forever = forever as c_uint;
        (*entry).test = test;

        if !device.is_null() {
            if strcmp((*entry).name, device) != 0 {
                entry = (*entry).next;
                continue;
            }
            return (handle_testdev(entry as *mut c_void) != entry as *mut c_void) as c_int;
        }
        status = pthread_create(
            &mut (*entry).thread as *mut pthread_t,
            null(),
            handle_testdev,
            entry as *mut c_void,
        );
        if status != 0 {
            perror(c"pthread_create".as_ptr());
        }
        entry = (*entry).next;
    }
    if !device.is_null() {
        let mut dev: testdev = unsafe { zeroed() };

        /* kernel can recognize test devices we don't */
        fprintf(
            stderr,
            c"%s: %s may see only control tests\n".as_ptr(),
            *argv.add(0),
            device,
        );

        memset(&mut dev as *mut testdev as *mut c_void, 0, size_of::<testdev>());
        dev.name = device;
        dev.param = param;
        dev.forever = forever as c_uint;
        dev.test = test;
        return (handle_testdev(&mut dev as *mut testdev as *mut c_void)
            != &mut dev as *mut testdev as *mut c_void) as c_int;
    }

    /* wait for tests to complete */
    entry = TESTDEVS;
    while !entry.is_null() {
        let mut retval: *mut c_void = null_mut();

        if pthread_join((*entry).thread, &mut retval as *mut *mut c_void) != 0 {
            perror(c"pthread_join".as_ptr());
        }
        /* testing errors discarded! */
        entry = (*entry).next;
    }

    0
}

unsafe fn usage(argv: *mut *mut c_char) -> c_int {
    fprintf(
        stderr,
        c"usage: %s [options]\nOptions:\n\t-D dev\t\tonly test specific device\n\t-A usb-dir\n\t-a\t\ttest all recognized devices\n\t-l\t\tloop forever(for stress test)\n\t-t testnum\tonly run specified case\n\t-n\t\tno test running, show devices to be tested\nCase arguments:\n\t-c iterations\t\tdefault 1000\n\t-s transfer length\tdefault 1024\n\t-g sglen\t\tdefault 32\n\t-v vary\t\t\tdefault 1024\n".as_ptr(),
        *argv.add(0),
    );
    1
}
