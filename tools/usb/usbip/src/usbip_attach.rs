// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 * Copyright (C) 2015-2016 Samsung Electronics
 *               Igor Kotrasinski <i.kotrasinsk@samsung.com>
 *               Krzysztof Opasiak <k.opasiak@samsung.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

/* Dependencies originally provided by:
 * <sys/stat.h>, <limits.h>, <stdint.h>, <stdio.h>, <string.h>,
 * <fcntl.h>, <getopt.h>, <unistd.h>, <errno.h>,
 * "vhci_driver.h", "usbip_common.h", "usbip_network.h", "usbip.h"
 */

const MAX_BUFF: usize = 100;

static usbip_attach_usage_string: &[u8] = b"usbip attach <args>\n\
    -r, --remote=<host>      The machine with exported USB devices\n\
    -b, --busid=<busid>    Busid of the device on <host>\n\
    -d, --device=<devid>    Id of the virtual UDC on <host>\n\0";

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    static usbip_port_string: *mut c_char;

    fn printf(format: *const c_char, ...) -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, mode: c_uint) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn strlen(s: *const c_char) -> usize;
    fn close(fd: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;

    fn usbip_vhci_driver_open() -> c_int;
    fn usbip_vhci_driver_close();
    fn usbip_vhci_get_free_port(speed: u32) -> c_int;
    fn usbip_vhci_attach_device(
        port: c_int,
        sockfd: c_int,
        busnum: u32,
        devnum: u32,
        speed: u32,
    ) -> c_int;

    fn usbip_net_send_op_common(sockfd: c_int, code: u32, status: c_int) -> c_int;
    fn usbip_net_send(sockfd: c_int, buff: *const c_void, size: usize) -> c_int;
    fn usbip_net_recv_op_common(sockfd: c_int, code: *mut u16, status: *mut c_int) -> c_int;
    fn usbip_net_recv(sockfd: c_int, buff: *mut c_void, size: usize) -> c_int;
    fn usbip_op_common_status_string(status: c_int) -> *const c_char;
    fn usbip_net_tcp_connect(host: *mut c_char, service: *mut c_char) -> c_int;

    fn PACK_OP_IMPORT_REQUEST(pack: c_int, request: *mut op_import_request);
    fn PACK_OP_IMPORT_REPLY(pack: c_int, reply: *mut op_import_reply);

    fn err(format: *const c_char, ...);
    fn dbg(format: *const c_char, ...);
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
struct stat {
    st_mode: c_uint,
}

#[repr(C)]
struct usbip_usb_device {
    busid: [c_char; SYSFS_BUS_ID_SIZE],
    busnum: u32,
    devnum: u32,
    speed: u32,
}

#[repr(C)]
struct op_import_request {
    busid: [c_char; SYSFS_BUS_ID_SIZE],
}

#[repr(C)]
struct op_import_reply {
    udev: usbip_usb_device,
}

const PATH_MAX: usize = 4096;
const EEXIST: c_int = 17;
const EBUSY: c_int = 16;
const S_IFDIR: c_uint = 0o040000;
const S_IRWXU: c_uint = 0o700;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const required_argument: c_int = 1;
const SYSFS_BUS_ID_SIZE: usize = 32;
const OP_REP_IMPORT: u16 = 3;
const OP_REQ_IMPORT: u32 = 0x8003;

/* Original C used the string literal macro VHCI_STATE_PATH. */
const VHCI_STATE_PATH: *const c_char = b"/var/run/vhci_hcd\0".as_ptr() as *const c_char;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_attach_usage() {
    unsafe {
        printf(
            b"usage: %s\0".as_ptr() as *const c_char,
            usbip_attach_usage_string.as_ptr() as *const c_char,
        );
    }
}

unsafe fn record_connection(
    host: *mut c_char,
    port: *mut c_char,
    busid: *mut c_char,
    rhport: c_int,
) -> c_int {
    let fd: c_int;
    let mut path: [c_char; PATH_MAX + 1] = [0; PATH_MAX + 1];
    let mut buff: [c_char; MAX_BUFF + 1] = [0; MAX_BUFF + 1];
    let mut ret: c_int;

    unsafe {
        ret = mkdir(VHCI_STATE_PATH, 0o700);
        if ret < 0 {
            /* if VHCI_STATE_PATH exists, then it better be a directory */
            if errno == EEXIST {
                let mut s: stat = mem::zeroed();

                ret = stat(VHCI_STATE_PATH, &mut s);
                if ret < 0 {
                    return -1;
                }
                if !(s.st_mode & S_IFDIR) != 0 {
                    return -1;
                }
            } else {
                return -1;
            }
        }

        snprintf(
            path.as_mut_ptr(),
            PATH_MAX,
            b"%s/port%d\0".as_ptr() as *const c_char,
            VHCI_STATE_PATH,
            rhport,
        );

        fd = open(path.as_ptr(), O_WRONLY | O_CREAT | O_TRUNC, S_IRWXU);
        if fd < 0 {
            return -1;
        }

        snprintf(
            buff.as_mut_ptr(),
            MAX_BUFF,
            b"%s %s %s\n\0".as_ptr() as *const c_char,
            host,
            port,
            busid,
        );

        ret = write(fd, buff.as_ptr() as *const c_void, strlen(buff.as_ptr())) as c_int;
        if ret as isize != strlen(buff.as_ptr()) as isize {
            close(fd);
            return -1;
        }

        close(fd);
    }

    0
}

unsafe fn import_device(sockfd: c_int, udev: *mut usbip_usb_device) -> c_int {
    let mut rc: c_int;
    let port: c_int;
    let speed: u32 = unsafe { (*udev).speed };

    unsafe {
        rc = usbip_vhci_driver_open();
        if rc < 0 {
            err(b"open vhci_driver (is vhci_hcd loaded?)\0".as_ptr() as *const c_char);
            return -1;
        }

        loop {
            port = usbip_vhci_get_free_port(speed);
            if port < 0 {
                err(b"no free port\0".as_ptr() as *const c_char);
                usbip_vhci_driver_close();
                return -1;
            }

            dbg(b"got free port %d\0".as_ptr() as *const c_char, port);

            rc = usbip_vhci_attach_device(port, sockfd, (*udev).busnum, (*udev).devnum, (*udev).speed);
            if rc < 0 && errno != EBUSY {
                err(b"import device\0".as_ptr() as *const c_char);
                usbip_vhci_driver_close();
                return -1;
            }
            if rc >= 0 {
                break;
            }
        }

        usbip_vhci_driver_close();
    }

    port
}

unsafe fn query_import_device(sockfd: c_int, busid: *mut c_char) -> c_int {
    let mut rc: c_int;
    let mut request: op_import_request = unsafe { mem::zeroed() };
    let mut reply: op_import_reply = unsafe { mem::zeroed() };
    let mut code: u16 = OP_REP_IMPORT;
    let mut status: c_int = 0;

    unsafe {
        memset(
            &mut request as *mut op_import_request as *mut c_void,
            0,
            mem::size_of::<op_import_request>(),
        );
        memset(
            &mut reply as *mut op_import_reply as *mut c_void,
            0,
            mem::size_of::<op_import_reply>(),
        );

        /* send a request */
        rc = usbip_net_send_op_common(sockfd, OP_REQ_IMPORT, 0);
        if rc < 0 {
            err(b"send op_common\0".as_ptr() as *const c_char);
            return -1;
        }

        strncpy(request.busid.as_mut_ptr(), busid, SYSFS_BUS_ID_SIZE - 1);

        PACK_OP_IMPORT_REQUEST(0, &mut request);

        rc = usbip_net_send(
            sockfd,
            &request as *const op_import_request as *const c_void,
            mem::size_of::<op_import_request>(),
        );
        if rc < 0 {
            err(b"send op_import_request\0".as_ptr() as *const c_char);
            return -1;
        }

        /* receive a reply */
        rc = usbip_net_recv_op_common(sockfd, &mut code, &mut status);
        if rc < 0 {
            err(
                b"Attach Request for %s failed - %s\n\0".as_ptr() as *const c_char,
                busid,
                usbip_op_common_status_string(status),
            );
            return -1;
        }

        rc = usbip_net_recv(
            sockfd,
            &mut reply as *mut op_import_reply as *mut c_void,
            mem::size_of::<op_import_reply>(),
        );
        if rc < 0 {
            err(b"recv op_import_reply\0".as_ptr() as *const c_char);
            return -1;
        }

        PACK_OP_IMPORT_REPLY(0, &mut reply);

        /* check the reply */
        if strncmp(reply.udev.busid.as_ptr(), busid, SYSFS_BUS_ID_SIZE) != 0 {
            err(
                b"recv different busid %s\0".as_ptr() as *const c_char,
                reply.udev.busid.as_ptr(),
            );
            return -1;
        }

        /* import a device */
        import_device(sockfd, &mut reply.udev)
    }
}

unsafe fn attach_device(host: *mut c_char, busid: *mut c_char) -> c_int {
    let sockfd: c_int;
    let rc: c_int;
    let rhport: c_int;

    unsafe {
        sockfd = usbip_net_tcp_connect(host, usbip_port_string);
        if sockfd < 0 {
            err(b"tcp connect\0".as_ptr() as *const c_char);
            return -1;
        }

        rhport = query_import_device(sockfd, busid);
        if rhport < 0 {
            return -1;
        }

        close(sockfd);

        rc = record_connection(host, usbip_port_string, busid, rhport);
        if rc < 0 {
            err(b"record connection\0".as_ptr() as *const c_char);
            return -1;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usbip_attach(argc: c_int, argv: *mut *mut c_char) -> c_int {
    static opts: [option; 4] = [
        option {
            name: b"remote\0".as_ptr() as *const c_char,
            has_arg: required_argument,
            flag: ptr::null_mut(),
            val: b'r' as c_int,
        },
        option {
            name: b"busid\0".as_ptr() as *const c_char,
            has_arg: required_argument,
            flag: ptr::null_mut(),
            val: b'b' as c_int,
        },
        option {
            name: b"device\0".as_ptr() as *const c_char,
            has_arg: required_argument,
            flag: ptr::null_mut(),
            val: b'd' as c_int,
        },
        option {
            name: ptr::null(),
            has_arg: 0,
            flag: ptr::null_mut(),
            val: 0,
        },
    ];
    let mut host: *mut c_char = ptr::null_mut();
    let mut busid: *mut c_char = ptr::null_mut();
    let mut opt: c_int;
    let mut ret: c_int = -1;

    unsafe {
        loop {
            opt = getopt_long(
                argc,
                argv,
                b"d:r:b:\0".as_ptr() as *const c_char,
                opts.as_ptr(),
                ptr::null_mut(),
            );

            if opt == -1 {
                break;
            }

            match opt {
                x if x == b'r' as c_int => {
                    host = optarg;
                }
                x if x == b'd' as c_int || x == b'b' as c_int => {
                    busid = optarg;
                }
                _ => {
                    usbip_attach_usage();
                    return ret;
                }
            }
        }

        if host.is_null() || busid.is_null() {
            usbip_attach_usage();
            return ret;
        }

        ret = attach_device(host, busid);
    }

    ret
}
