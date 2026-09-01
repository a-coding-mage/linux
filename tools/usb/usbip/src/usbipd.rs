// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 * Copyright (C) 2015-2016 Samsung Electronics
 *               Igor Kotrasinski <i.kotrasinsk@samsung.com>
 *               Krzysztof Opasiak <k.opasiak@samsung.com>
 */

// Translated from usbipd.c. C include dependencies are expected to be supplied
// by the surrounding usbip build/bindings.

use core::ffi::{c_char, c_int, c_short, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

const PROGNAME: &[u8] = b"usbipd\0";
const MAXSOCKFD: usize = 20;
const MAIN_LOOP_TIMEOUT: i64 = 10;
const DEFAULT_PID_FILE: &[u8] = b"/var/run/usbipd.pid\0";

const OP_UNSPEC: u16 = 0x0000;
const OP_REQ_DEVLIST: u16 = 0x8005;
const OP_REP_DEVLIST: u16 = 0x0005;
const OP_REQ_IMPORT: u16 = 0x8003;
const OP_REP_IMPORT: u16 = 0x0003;
const OP_REQ_DEVINFO: u16 = 0x8002;
const OP_REQ_CRYPKEY: u16 = 0x8004;

const ST_OK: c_int = 0x00;
const ST_NA: c_int = 0x01;
const ST_NODEV: c_int = 0x02;

const SDEV_ST_USED: c_int = 0x02;
const SYSFS_BUS_ID_SIZE: usize = 32;

const SOCK_STREAM: c_int = 1;
const AF_UNSPEC: c_int = 0;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const AI_PASSIVE: c_int = 0x0001;
const NI_MAXHOST: usize = 1025;
const NI_MAXSERV: usize = 32;
const NI_NUMERICHOST: c_int = 1;
const NI_NUMERICSERV: c_int = 2;
const SOMAXCONN: c_int = 4096;
const POLLIN: c_short = 0x0001;
const SIGTERM: c_int = 15;
const SIGINT: c_int = 2;
const SIGCHLD: c_int = 17;
const SIG_IGN: usize = 1;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

const NO_ARGUMENT: c_int = 0;
const REQUIRED_ARGUMENT: c_int = 1;
const OPTIONAL_ARGUMENT: c_int = 2;

static USBIP_VERSION_STRING: &[u8] = b"\0"; // PACKAGE_STRING

static USBIPD_HELP_STRING: &[u8] =
    b"usage: usbipd [options]\n\
\n\
\t-4, --ipv4\n\
\t\tBind to IPv4. Default is both.\n\
\n\
\t-6, --ipv6\n\
\t\tBind to IPv6. Default is both.\n\
\n\
\t-e, --device\n\
\t\tRun in device mode.\n\
\t\tRather than drive an attached device, create\n\
\t\ta virtual UDC to bind gadgets to.\n\
\n\
\t-D, --daemon\n\
\t\tRun as a daemon process.\n\
\n\
\t-d, --debug\n\
\t\tPrint debugging information.\n\
\n\
\t-PFILE, --pid FILE\n\
\t\tWrite process id to FILE.\n\
\t\tIf no FILE specified, use /var/run/usbipd.pid\n\
\n\
\t-tPORT, --tcp-port PORT\n\
\t\tListen on TCP/IP port PORT.\n\
\n\
\t-h, --help\n\
\t\tPrint this help.\n\
\n\
\t-v, --version\n\
\t\tShow version.\n\0";

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct usbip_usb_device {
    pub path: [c_char; 256],
    pub busid: [c_char; SYSFS_BUS_ID_SIZE],
    pub busnum: u32,
    pub devnum: u32,
    pub speed: u32,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bConfigurationValue: u8,
    pub bNumConfigurations: u8,
    pub bNumInterfaces: u8,
}

#[repr(C)]
pub struct usbip_usb_interface {
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub padding: u8,
}

#[repr(C)]
pub struct usbip_exported_device {
    pub node: list_head,
    pub udev: usbip_usb_device,
    pub uinf: *mut usbip_usb_interface,
    pub status: c_int,
}

#[repr(C)]
pub struct usbip_host_driver {
    pub edev_list: list_head,
}

#[repr(C)]
pub struct op_import_request {
    pub busid: [c_char; SYSFS_BUS_ID_SIZE],
}

#[repr(C)]
pub struct op_devlist_request {
    pub dummy: c_int,
}

#[repr(C)]
pub struct op_devlist_reply {
    pub ndev: c_int,
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: u16,
    pub __ss_padding: [u8; 118],
    pub __ss_align: u64,
}

#[repr(C)]
pub struct addrinfo {
    pub ai_flags: c_int,
    pub ai_family: c_int,
    pub ai_socktype: c_int,
    pub ai_protocol: c_int,
    pub ai_addrlen: u32,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut c_char,
    pub ai_next: *mut addrinfo,
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct sigset_t {
    pub __val: [u64; 16],
}

#[repr(C)]
pub struct sigaction {
    pub sa_handler: usize,
    pub sa_flags: c_uint,
    pub sa_restorer: *mut c_void,
    pub sa_mask: sigset_t,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

unsafe extern "C" {
    static mut host_driver: usbip_host_driver;
    static mut device_driver: usbip_host_driver;
    static mut usbip_use_stderr: c_int;
    static mut usbip_use_syslog: c_int;
    static mut usbip_use_debug: c_int;
    static mut usbip_port_string: *const c_char;
    static mut optarg: *mut c_char;
    static mut errno: c_int;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strsignal(sig: c_int) -> *mut c_char;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut u32) -> c_int;
    fn getnameinfo(
        sa: *const sockaddr,
        salen: u32,
        host: *mut c_char,
        hostlen: u32,
        serv: *mut c_char,
        servlen: u32,
        flags: c_int,
    ) -> c_int;
    fn gai_strerror(errcode: c_int) -> *const c_char;
    fn close(fd: c_int) -> c_int;
    fn fork() -> c_int;
    fn exit(status: c_int) -> !;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: u32) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> c_int;
    fn freeaddrinfo(res: *mut addrinfo);
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigfillset(set: *mut sigset_t) -> c_int;
    fn sigdelset(set: *mut sigset_t, signum: c_int) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getpid() -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn daemon(nochdir: c_int, noclose: c_int) -> c_int;
    fn umask(mask: c_uint) -> c_uint;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn ppoll(fds: *mut pollfd, nfds: usize, timeout_ts: *const timespec, sigmask: *const sigset_t) -> c_int;
    fn geteuid() -> c_uint;
    fn getopt_long(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int;

    fn usbip_net_recv(sockfd: c_int, buff: *mut c_void, size: usize) -> c_int;
    fn usbip_net_send(sockfd: c_int, buff: *const c_void, size: usize) -> c_int;
    fn usbip_net_recv_op_common(sockfd: c_int, code: *mut u16, status: *mut c_int) -> c_int;
    fn usbip_net_send_op_common(sockfd: c_int, code: u16, status: c_int) -> c_int;
    fn usbip_net_pack_usb_device(pack: c_int, udev: *mut usbip_usb_device);
    fn usbip_net_pack_usb_interface(pack: c_int, uinf: *mut usbip_usb_interface);
    fn usbip_net_set_nodelay(sockfd: c_int);
    fn usbip_net_set_reuseaddr(sockfd: c_int);
    fn usbip_net_set_v6only(sockfd: c_int);
    fn usbip_export_device(edev: *mut usbip_exported_device, sockfd: c_int) -> c_int;
    fn usbip_refresh_device_list(driver: *mut usbip_host_driver) -> c_int;
    fn usbip_driver_open(driver: *mut usbip_host_driver) -> c_int;
    fn usbip_driver_close(driver: *mut usbip_host_driver);
    fn usbip_setup_port_number(port: *mut c_char);
    fn dump_usb_device(udev: *const usbip_usb_device);
    fn dump_usb_interface(uinf: *const usbip_usb_interface);
    fn dbg(fmt: *const c_char, ...);
    fn info(fmt: *const c_char, ...);
    fn err(fmt: *const c_char, ...);
}

static mut driver: *mut usbip_host_driver = ptr::null_mut();
static mut pid_file: *const c_char = ptr::null();

unsafe fn PACK_OP_IMPORT_REQUEST(pack: c_int, req: *mut op_import_request) {
    let _ = (pack, req);
}

unsafe fn PACK_OP_DEVLIST_REPLY(pack: c_int, reply: *mut op_devlist_reply) {
    let _ = (pack, reply);
}

unsafe fn list_entry_usbip_exported_device(ptr: *mut list_head) -> *mut usbip_exported_device {
    (ptr as *mut u8).sub(offset_of!(usbip_exported_device, node)) as *mut usbip_exported_device
}

unsafe fn usbipd_help() {
    printf(b"%s\n\0".as_ptr() as *const c_char, USBIPD_HELP_STRING.as_ptr() as *const c_char);
}

unsafe fn recv_request_import(sockfd: c_int) -> c_int {
    let mut req: op_import_request = core::mem::zeroed();
    let mut edev: *mut usbip_exported_device = ptr::null_mut();
    let mut pdu_udev: usbip_usb_device = core::mem::zeroed();
    let mut i: *mut list_head;
    let mut found = 0;
    let mut status = ST_OK;
    let mut rc: c_int;

    memset(&mut req as *mut _ as *mut c_void, 0, size_of::<op_import_request>());

    rc = usbip_net_recv(sockfd, &mut req as *mut _ as *mut c_void, size_of::<op_import_request>());
    if rc < 0 {
        dbg(b"usbip_net_recv failed: import request\0".as_ptr() as *const c_char);
        return -1;
    }
    PACK_OP_IMPORT_REQUEST(0, &mut req);

    i = (*driver).edev_list.next;
    while i != &mut (*driver).edev_list as *mut list_head {
        edev = list_entry_usbip_exported_device(i);
        if strncmp(req.busid.as_ptr(), (*edev).udev.busid.as_ptr(), SYSFS_BUS_ID_SIZE) == 0 {
            info(b"found requested device: %s\0".as_ptr() as *const c_char, req.busid.as_ptr());
            found = 1;
            break;
        }
        i = (*i).next;
    }

    if found != 0 {
        /* should set TCP_NODELAY for usbip */
        usbip_net_set_nodelay(sockfd);

        /* export device needs a TCP/IP socket descriptor */
        status = usbip_export_device(edev, sockfd);
        if status < 0 {
            status = ST_NA;
        }
    } else {
        info(b"requested device not found: %s\0".as_ptr() as *const c_char, req.busid.as_ptr());
        status = ST_NODEV;
    }

    rc = usbip_net_send_op_common(sockfd, OP_REP_IMPORT, status);
    if rc < 0 {
        dbg(b"usbip_net_send_op_common failed: %#0x\0".as_ptr() as *const c_char, OP_REP_IMPORT as c_int);
        return -1;
    }

    if status != 0 {
        dbg(b"import request busid %s: failed\0".as_ptr() as *const c_char, req.busid.as_ptr());
        return -1;
    }

    memcpy(
        &mut pdu_udev as *mut _ as *mut c_void,
        &(*edev).udev as *const _ as *const c_void,
        size_of::<usbip_usb_device>(),
    );
    usbip_net_pack_usb_device(1, &mut pdu_udev);

    rc = usbip_net_send(sockfd, &pdu_udev as *const _ as *const c_void, size_of::<usbip_usb_device>());
    if rc < 0 {
        dbg(b"usbip_net_send failed: devinfo\0".as_ptr() as *const c_char);
        return -1;
    }

    dbg(b"import request busid %s: complete\0".as_ptr() as *const c_char, req.busid.as_ptr());

    0
}

unsafe fn send_reply_devlist(connfd: c_int) -> c_int {
    let mut edev: *mut usbip_exported_device;
    let mut pdu_udev: usbip_usb_device = core::mem::zeroed();
    let mut pdu_uinf: usbip_usb_interface = core::mem::zeroed();
    let mut reply: op_devlist_reply = core::mem::zeroed();
    let mut j: *mut list_head;
    let mut rc: c_int;
    let mut i: c_int;

    /*
     * Exclude devices that are already exported to a client from
     * the exportable device list to avoid:
     *      - import requests for devices that are exported only to
     *        fail the request.
     *      - revealing devices that are imported by a client to
     *        another client.
     */

    reply.ndev = 0;
    /* number of exported devices */
    j = (*driver).edev_list.next;
    while j != &mut (*driver).edev_list as *mut list_head {
        edev = list_entry_usbip_exported_device(j);
        if (*edev).status != SDEV_ST_USED {
            reply.ndev += 1;
        }
        j = (*j).next;
    }
    info(b"exportable devices: %d\0".as_ptr() as *const c_char, reply.ndev);

    rc = usbip_net_send_op_common(connfd, OP_REP_DEVLIST, ST_OK);
    if rc < 0 {
        dbg(b"usbip_net_send_op_common failed: %#0x\0".as_ptr() as *const c_char, OP_REP_DEVLIST as c_int);
        return -1;
    }
    PACK_OP_DEVLIST_REPLY(1, &mut reply);

    rc = usbip_net_send(connfd, &reply as *const _ as *const c_void, size_of::<op_devlist_reply>());
    if rc < 0 {
        dbg(b"usbip_net_send failed: %#0x\0".as_ptr() as *const c_char, OP_REP_DEVLIST as c_int);
        return -1;
    }

    j = (*driver).edev_list.next;
    while j != &mut (*driver).edev_list as *mut list_head {
        edev = list_entry_usbip_exported_device(j);
        if (*edev).status == SDEV_ST_USED {
            j = (*j).next;
            continue;
        }

        dump_usb_device(&(*edev).udev);
        memcpy(
            &mut pdu_udev as *mut _ as *mut c_void,
            &(*edev).udev as *const _ as *const c_void,
            size_of::<usbip_usb_device>(),
        );
        usbip_net_pack_usb_device(1, &mut pdu_udev);

        rc = usbip_net_send(connfd, &pdu_udev as *const _ as *const c_void, size_of::<usbip_usb_device>());
        if rc < 0 {
            dbg(b"usbip_net_send failed: pdu_udev\0".as_ptr() as *const c_char);
            return -1;
        }

        i = 0;
        while i < (*edev).udev.bNumInterfaces as c_int {
            let uinf = (*edev).uinf.add(i as usize);
            dump_usb_interface(uinf);
            memcpy(
                &mut pdu_uinf as *mut _ as *mut c_void,
                uinf as *const c_void,
                size_of::<usbip_usb_interface>(),
            );
            usbip_net_pack_usb_interface(1, &mut pdu_uinf);

            rc = usbip_net_send(connfd, &pdu_uinf as *const _ as *const c_void, size_of::<usbip_usb_interface>());
            if rc < 0 {
                err(b"usbip_net_send failed: pdu_uinf\0".as_ptr() as *const c_char);
                return -1;
            }
            i += 1;
        }
        j = (*j).next;
    }

    0
}

unsafe fn recv_request_devlist(connfd: c_int) -> c_int {
    let mut req: op_devlist_request = core::mem::zeroed();
    let mut rc: c_int;

    memset(&mut req as *mut _ as *mut c_void, 0, size_of::<op_devlist_request>());

    rc = usbip_net_recv(connfd, &mut req as *mut _ as *mut c_void, size_of::<op_devlist_request>());
    if rc < 0 {
        dbg(b"usbip_net_recv failed: devlist request\0".as_ptr() as *const c_char);
        return -1;
    }

    rc = send_reply_devlist(connfd);
    if rc < 0 {
        dbg(b"send_reply_devlist failed\0".as_ptr() as *const c_char);
        return -1;
    }

    0
}

unsafe fn recv_pdu(connfd: c_int) -> c_int {
    let mut code: u16 = OP_UNSPEC;
    let mut ret: c_int;
    let mut status: c_int = 0;

    ret = usbip_net_recv_op_common(connfd, &mut code, &mut status);
    if ret < 0 {
        dbg(b"could not receive opcode: %#0x\0".as_ptr() as *const c_char, code as c_int);
        return -1;
    }

    ret = usbip_refresh_device_list(driver);
    if ret < 0 {
        dbg(b"could not refresh device list: %d\0".as_ptr() as *const c_char, ret);
        return -1;
    }

    info(b"received request: %#0x(%d)\0".as_ptr() as *const c_char, code as c_int, connfd);
    match code {
        OP_REQ_DEVLIST => ret = recv_request_devlist(connfd),
        OP_REQ_IMPORT => ret = recv_request_import(connfd),
        OP_REQ_DEVINFO | OP_REQ_CRYPKEY | _ => {
            err(b"received an unknown opcode: %#0x\0".as_ptr() as *const c_char, code as c_int);
            ret = -1;
        }
    }

    if ret == 0 {
        info(b"request %#0x(%d): complete\0".as_ptr() as *const c_char, code as c_int, connfd);
    } else {
        info(b"request %#0x(%d): failed\0".as_ptr() as *const c_char, code as c_int, connfd);
    }

    ret
}

// HAVE_LIBWRAP: tcpd_auth() is omitted unless the surrounding build enables
// libwrap and supplies request_info/request_init/fromhost/hosts_access bindings.

unsafe fn do_accept(listenfd: c_int) -> c_int {
    let mut connfd: c_int;
    let mut ss: sockaddr_storage = core::mem::zeroed();
    let mut len: u32 = size_of::<sockaddr_storage>() as u32;
    let mut host = [0 as c_char; NI_MAXHOST];
    let mut port = [0 as c_char; NI_MAXSERV];
    let mut rc: c_int;

    memset(&mut ss as *mut _ as *mut c_void, 0, size_of::<sockaddr_storage>());

    connfd = accept(listenfd, &mut ss as *mut _ as *mut sockaddr, &mut len);
    if connfd < 0 {
        err(b"failed to accept connection\0".as_ptr() as *const c_char);
        return -1;
    }

    rc = getnameinfo(
        &ss as *const _ as *const sockaddr,
        len,
        host.as_mut_ptr(),
        host.len() as u32,
        port.as_mut_ptr(),
        port.len() as u32,
        NI_NUMERICHOST | NI_NUMERICSERV,
    );
    if rc != 0 {
        err(b"getnameinfo: %s\0".as_ptr() as *const c_char, gai_strerror(rc));
    }

    // HAVE_LIBWRAP: tcpd_auth(connfd) denial would close connfd and return -1.
    info(b"connection from %s:%s\0".as_ptr() as *const c_char, host.as_ptr(), port.as_ptr());

    connfd
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_request(listenfd: c_int) -> c_int {
    let childpid: c_int;
    let connfd: c_int;

    connfd = do_accept(listenfd);
    if connfd < 0 {
        return -1;
    }
    childpid = fork();
    if childpid == 0 {
        close(listenfd);
        recv_pdu(connfd);
        exit(0);
    }
    close(connfd);
    0
}

unsafe fn addrinfo_to_text(ai: *mut addrinfo, buf: *mut c_char, buf_size: usize) {
    let mut hbuf = [0 as c_char; NI_MAXHOST];
    let mut sbuf = [0 as c_char; NI_MAXSERV];
    let rc: c_int;

    *buf = 0;

    rc = getnameinfo(
        (*ai).ai_addr,
        (*ai).ai_addrlen,
        hbuf.as_mut_ptr(),
        hbuf.len() as u32,
        sbuf.as_mut_ptr(),
        sbuf.len() as u32,
        NI_NUMERICHOST | NI_NUMERICSERV,
    );
    if rc != 0 {
        err(b"getnameinfo: %s\0".as_ptr() as *const c_char, gai_strerror(rc));
    }

    snprintf(buf, buf_size, b"%s:%s\0".as_ptr() as *const c_char, hbuf.as_ptr(), sbuf.as_ptr());
}

unsafe fn listen_all_addrinfo(ai_head: *mut addrinfo, sockfdlist: *mut c_int, maxsockfd: c_int) -> c_int {
    let mut ai: *mut addrinfo;
    let mut ret: c_int;
    let mut nsockfd: c_int = 0;
    const AI_BUF_SIZE: usize = NI_MAXHOST + NI_MAXSERV + 2;
    let mut ai_buf = [0 as c_char; AI_BUF_SIZE];

    ai = ai_head;
    while !ai.is_null() && nsockfd < maxsockfd {
        let sock: c_int;

        addrinfo_to_text(ai, ai_buf.as_mut_ptr(), AI_BUF_SIZE);
        dbg(b"opening %s\0".as_ptr() as *const c_char, ai_buf.as_ptr());
        sock = socket((*ai).ai_family, (*ai).ai_socktype, (*ai).ai_protocol);
        if sock < 0 {
            err(
                b"socket: %s: %d (%s)\0".as_ptr() as *const c_char,
                ai_buf.as_ptr(),
                errno,
                strerror(errno),
            );
            ai = (*ai).ai_next;
            continue;
        }

        usbip_net_set_reuseaddr(sock);
        usbip_net_set_nodelay(sock);
        /* We use separate sockets for IPv4 and IPv6
         * (see do_standalone_mode()) */
        usbip_net_set_v6only(sock);

        ret = bind(sock, (*ai).ai_addr, (*ai).ai_addrlen);
        if ret < 0 {
            err(
                b"bind: %s: %d (%s)\0".as_ptr() as *const c_char,
                ai_buf.as_ptr(),
                errno,
                strerror(errno),
            );
            close(sock);
            ai = (*ai).ai_next;
            continue;
        }

        ret = listen(sock, SOMAXCONN);
        if ret < 0 {
            err(
                b"listen: %s: %d (%s)\0".as_ptr() as *const c_char,
                ai_buf.as_ptr(),
                errno,
                strerror(errno),
            );
            close(sock);
            ai = (*ai).ai_next;
            continue;
        }

        info(b"listening on %s\0".as_ptr() as *const c_char, ai_buf.as_ptr());
        *sockfdlist.add(nsockfd as usize) = sock;
        nsockfd += 1;
        ai = (*ai).ai_next;
    }

    nsockfd
}

unsafe fn do_getaddrinfo(host: *mut c_char, ai_family: c_int) -> *mut addrinfo {
    let mut hints: addrinfo = core::mem::zeroed();
    let mut ai_head: *mut addrinfo = ptr::null_mut();
    let rc: c_int;

    memset(&mut hints as *mut _ as *mut c_void, 0, size_of::<addrinfo>());
    hints.ai_family = ai_family;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_flags = AI_PASSIVE;

    rc = getaddrinfo(host, usbip_port_string, &hints, &mut ai_head);
    if rc != 0 {
        err(
            b"failed to get a network address %s: %s\0".as_ptr() as *const c_char,
            usbip_port_string,
            gai_strerror(rc),
        );
        return ptr::null_mut();
    }

    ai_head
}

unsafe extern "C" fn signal_handler(i: c_int) {
    dbg(b"received '%s' signal\0".as_ptr() as *const c_char, strsignal(i));
}

unsafe fn set_signal() {
    let mut act: sigaction = core::mem::zeroed();

    memset(&mut act as *mut _ as *mut c_void, 0, size_of::<sigaction>());
    act.sa_handler = signal_handler as usize;
    sigemptyset(&mut act.sa_mask);
    sigaction(SIGTERM, &act, ptr::null_mut());
    sigaction(SIGINT, &act, ptr::null_mut());
    act.sa_handler = SIG_IGN;
    sigaction(SIGCHLD, &act, ptr::null_mut());
}

unsafe fn write_pid_file() {
    if !pid_file.is_null() {
        dbg(b"creating pid file %s\0".as_ptr() as *const c_char, pid_file);
        let fp: *mut FILE;

        fp = fopen(pid_file, b"w\0".as_ptr() as *const c_char);
        if fp.is_null() {
            err(
                b"pid_file: %s: %d (%s)\0".as_ptr() as *const c_char,
                pid_file,
                errno,
                strerror(errno),
            );
            return;
        }
        fprintf(fp, b"%d\n\0".as_ptr() as *const c_char, getpid());
        fclose(fp);
    }
}

unsafe fn remove_pid_file() {
    if !pid_file.is_null() {
        dbg(b"removing pid file %s\0".as_ptr() as *const c_char, pid_file);
        unlink(pid_file);
    }
}

unsafe fn do_standalone_mode(daemonize: c_int, ipv4: c_int, ipv6: c_int) -> c_int {
    let mut ai_head: *mut addrinfo;
    let mut sockfdlist = [0 as c_int; MAXSOCKFD];
    let mut nsockfd: c_int;
    let family: c_int;
    let mut i: c_int;
    let mut terminate: c_int;
    let fds: *mut pollfd;
    let mut timeout = timespec { tv_sec: MAIN_LOOP_TIMEOUT, tv_nsec: 0 };
    let mut sigmask: sigset_t = core::mem::zeroed();

    if usbip_driver_open(driver) != 0 {
        return -1;
    }

    if daemonize != 0 {
        if daemon(0, 0) < 0 {
            err(b"daemonizing failed: %s\0".as_ptr() as *const c_char, strerror(errno));
            usbip_driver_close(driver);
            return -1;
        }
        umask(0);
        usbip_use_syslog = 1;
    }
    set_signal();
    write_pid_file();

    info(b"starting usbipd (%s)\0".as_ptr() as *const c_char, USBIP_VERSION_STRING.as_ptr() as *const c_char);

    /*
     * To suppress warnings on systems with bindv6only disabled
     * (default), we use separate sockets for IPv6 and IPv4 and set
     * IPV6_V6ONLY on the IPv6 sockets.
     */
    if ipv4 != 0 && ipv6 != 0 {
        family = AF_UNSPEC;
    } else if ipv4 != 0 {
        family = AF_INET;
    } else {
        family = AF_INET6;
    }

    ai_head = do_getaddrinfo(ptr::null_mut(), family);
    if ai_head.is_null() {
        usbip_driver_close(driver);
        return -1;
    }
    nsockfd = listen_all_addrinfo(ai_head, sockfdlist.as_mut_ptr(), (size_of::<[c_int; MAXSOCKFD]>() / size_of::<c_int>()) as c_int);
    freeaddrinfo(ai_head);
    if nsockfd <= 0 {
        err(b"failed to open a listening socket\0".as_ptr() as *const c_char);
        usbip_driver_close(driver);
        return -1;
    }

    dbg(
        b"listening on %d address%s\0".as_ptr() as *const c_char,
        nsockfd,
        if nsockfd == 1 { b"\0".as_ptr() } else { b"es\0".as_ptr() } as *const c_char,
    );

    fds = calloc(nsockfd as usize, size_of::<pollfd>()) as *mut pollfd;
    i = 0;
    while i < nsockfd {
        (*fds.add(i as usize)).fd = sockfdlist[i as usize];
        (*fds.add(i as usize)).events = POLLIN;
        i += 1;
    }
    timeout.tv_sec = MAIN_LOOP_TIMEOUT;
    timeout.tv_nsec = 0;

    sigfillset(&mut sigmask);
    sigdelset(&mut sigmask, SIGTERM);
    sigdelset(&mut sigmask, SIGINT);

    terminate = 0;
    while terminate == 0 {
        let r: c_int;

        r = ppoll(fds, nsockfd as usize, &timeout, &sigmask);
        if r < 0 {
            dbg(b"%s\0".as_ptr() as *const c_char, strerror(errno));
            terminate = 1;
        } else if r != 0 {
            i = 0;
            while i < nsockfd {
                if ((*fds.add(i as usize)).revents & POLLIN) != 0 {
                    dbg(
                        b"read event on fd[%d]=%d\0".as_ptr() as *const c_char,
                        i,
                        sockfdlist[i as usize],
                    );
                    process_request(sockfdlist[i as usize]);
                }
                i += 1;
            }
        } else {
            dbg(b"heartbeat timeout on ppoll()\0".as_ptr() as *const c_char);
        }
    }

    info(b"shutting down usbipd\0".as_ptr() as *const c_char);
    free(fds as *mut c_void);
    usbip_driver_close(driver);

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    static LONGOPTS: [option; 11] = [
        option { name: b"ipv4\0".as_ptr() as *const c_char, has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: '4' as c_int },
        option { name: b"ipv6\0".as_ptr() as *const c_char, has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: '6' as c_int },
        option { name: b"daemon\0".as_ptr() as *const c_char, has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: 'D' as c_int },
        option { name: b"daemon\0".as_ptr() as *const c_char, has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: 'D' as c_int },
        option { name: b"debug\0".as_ptr() as *const c_char, has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: 'd' as c_int },
        option { name: b"device\0".as_ptr() as *const c_char, has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: 'e' as c_int },
        option { name: b"pid\0".as_ptr() as *const c_char, has_arg: OPTIONAL_ARGUMENT, flag: ptr::null_mut(), val: 'P' as c_int },
        option { name: b"tcp-port\0".as_ptr() as *const c_char, has_arg: REQUIRED_ARGUMENT, flag: ptr::null_mut(), val: 't' as c_int },
        option { name: b"help\0".as_ptr() as *const c_char, has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: 'h' as c_int },
        option { name: b"version\0".as_ptr() as *const c_char, has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: 'v' as c_int },
        option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
    ];

    const CMD_STANDALONE_MODE: c_int = 1;
    const CMD_HELP: c_int = 2;
    const CMD_VERSION: c_int = 3;

    let mut cmd: c_int;
    let mut daemonize: c_int = 0;
    let mut ipv4: c_int = 0;
    let mut ipv6: c_int = 0;
    let mut opt: c_int;
    let mut rc: c_int = -1;

    pid_file = ptr::null();

    usbip_use_stderr = 1;
    usbip_use_syslog = 0;

    if geteuid() != 0 {
        err(b"not running as root?\0".as_ptr() as *const c_char);
    }

    cmd = CMD_STANDALONE_MODE;
    driver = &mut host_driver;
    loop {
        opt = getopt_long(
            argc,
            argv,
            b"46DdeP::t:hv\0".as_ptr() as *const c_char,
            LONGOPTS.as_ptr(),
            ptr::null_mut(),
        );

        if opt == -1 {
            break;
        }

        match opt {
            x if x == '4' as c_int => ipv4 = 1,
            x if x == '6' as c_int => ipv6 = 1,
            x if x == 'D' as c_int => daemonize = 1,
            x if x == 'd' as c_int => usbip_use_debug = 1,
            x if x == 'h' as c_int => cmd = CMD_HELP,
            x if x == 'P' as c_int => {
                pid_file = if !optarg.is_null() {
                    optarg
                } else {
                    DEFAULT_PID_FILE.as_ptr() as *const c_char
                };
            }
            x if x == 't' as c_int => usbip_setup_port_number(optarg),
            x if x == 'v' as c_int => cmd = CMD_VERSION,
            x if x == 'e' as c_int => driver = &mut device_driver,
            x if x == '?' as c_int => usbipd_help(),
            _ => return if rc > -1 { EXIT_SUCCESS } else { EXIT_FAILURE },
        }
    }

    if ipv4 == 0 && ipv6 == 0 {
        ipv4 = 1;
        ipv6 = 1;
    }

    match cmd {
        CMD_STANDALONE_MODE => {
            rc = do_standalone_mode(daemonize, ipv4, ipv6);
            remove_pid_file();
        }
        CMD_VERSION => {
            printf(b"usbipd (%s)\n\0".as_ptr() as *const c_char, USBIP_VERSION_STRING.as_ptr() as *const c_char);
            rc = 0;
        }
        CMD_HELP => {
            usbipd_help();
            rc = 0;
        }
        _ => {
            usbipd_help();
            return if rc > -1 { EXIT_SUCCESS } else { EXIT_FAILURE };
        }
    }

    if rc > -1 { EXIT_SUCCESS } else { EXIT_FAILURE }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
