/*
 * This is free and unencumbered software released into the public domain.
 *
 * Anyone is free to copy, modify, publish, use, compile, sell, or
 * distribute this software, either in source code form or as a compiled
 * binary, for any purpose, commercial or non-commercial, and by any
 * means.
 *
 * In jurisdictions that recognize copyright laws, the author or authors
 * of this software dedicate any and all copyright interest in the
 * software to the public domain. We make this dedication for the benefit
 * of the public at large and to the detriment of our heirs and
 * successors. We intend this dedication to be an overt act of
 * relinquishment in perpetuity of all present and future rights to this
 * software under copyright law.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * For more information, please refer to <http://unlicense.org/>
 */

/* $(CROSS_COMPILE)cc -g -o aio_simple aio_simple.c -laio */

/* _DEFAULT_SOURCE was defined in C for endian.h. */
/* C dependencies: endian.h, errno.h, fcntl.h, stdarg.h, stdio.h, stdlib.h,
 * string.h, sys/ioctl.h, sys/stat.h, sys/types.h, sys/poll.h, unistd.h,
 * stdbool.h, sys/eventfd.h, libaio.h, linux/usb/functionfs.h.
 */

use core::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __le16 = u16;
type __le32 = u32;
type size_t = usize;
type ssize_t = isize;
type nfds_t = c_ulong;
type io_context_t = *mut c_void;

const IOCB_FLAG_RESFD: u32 = 1 << 0;
const BUF_LEN: usize = 8192;

const POLLIN: c_short = 0x0001;
const O_RDWR: c_int = 0o2;
const EINTR: c_int = 4;

const FUNCTIONFS_DESCRIPTORS_MAGIC_V2: u32 = 3;
const FUNCTIONFS_STRINGS_MAGIC: u32 = 2;
const FUNCTIONFS_HAS_FS_DESC: u32 = 1;
const FUNCTIONFS_HAS_HS_DESC: u32 = 2;

const USB_DT_INTERFACE: u8 = 4;
const USB_DT_ENDPOINT: u8 = 5;
const USB_DIR_OUT: u8 = 0;
const USB_DIR_IN: u8 = 0x80;
const USB_CLASS_VENDOR_SPEC: u8 = 0xff;
const USB_ENDPOINT_XFER_BULK: u8 = 2;

const FUNCTIONFS_BIND: usize = 0;
const FUNCTIONFS_UNBIND: usize = 1;
const FUNCTIONFS_ENABLE: usize = 2;
const FUNCTIONFS_DISABLE: usize = 3;
const FUNCTIONFS_SETUP: usize = 4;
const FUNCTIONFS_SUSPEND: usize = 5;
const FUNCTIONFS_RESUME: usize = 6;

/*
 * cpu_to_le16/32 are used when initializing structures, a context where a
 * function call is not allowed. To solve this, we code cpu_to_le16/32 in a way
 * that allows them to be used when initializing structures.
 */
const fn cpu_to_le16(x: u16) -> u16 {
    x.to_le()
}

const fn cpu_to_le32(x: u32) -> u32 {
    x.to_le()
}

#[repr(C)]
struct usb_functionfs_descs_head_v2 {
    magic: __le32,
    length: __le32,
    flags: __le32,
}

#[repr(C, packed)]
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

#[repr(C, packed)]
struct usb_endpoint_descriptor_no_audio {
    bLength: u8,
    bDescriptorType: u8,
    bEndpointAddress: u8,
    bmAttributes: u8,
    wMaxPacketSize: __le16,
    bInterval: u8,
}

#[repr(C, packed)]
struct descs {
    intf: usb_interface_descriptor,
    bulk_sink: usb_endpoint_descriptor_no_audio,
    bulk_source: usb_endpoint_descriptor_no_audio,
}

#[repr(C, packed)]
struct descriptors_type {
    header: usb_functionfs_descs_head_v2,
    fs_count: __le32,
    hs_count: __le32,
    fs_descs: descs,
    hs_descs: descs,
}

#[repr(C)]
struct usb_functionfs_strings_head {
    magic: __le32,
    length: __le32,
    str_count: __le32,
    lang_count: __le32,
}

#[repr(C, packed)]
struct lang0_type {
    code: __le16,
    str1: [c_char; STR_INTERFACE_LEN],
}

#[repr(C, packed)]
struct strings_type {
    header: usb_functionfs_strings_head,
    lang0: lang0_type,
}

#[repr(C)]
struct usb_ctrlrequest {
    bRequestType: u8,
    bRequest: u8,
    wValue: __le16,
    wIndex: __le16,
    wLength: __le16,
}

#[repr(C)]
union usb_functionfs_event_union {
    setup: usb_ctrlrequest,
}

#[repr(C)]
struct usb_functionfs_event {
    u: usb_functionfs_event_union,
    type_: u8,
    _pad: [u8; 3],
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
struct fd_set {
    fds_bits: [c_long; 16],
}

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct io_event {
    data: u64,
    obj: *mut iocb,
    res: c_long,
    res2: c_long,
}

#[repr(C)]
struct io_iocb_common {
    buf: *mut c_void,
    nbytes: c_ulong,
    offset: i64,
    __pad3: u64,
    flags: u32,
    resfd: u32,
}

#[repr(C)]
union iocb_u {
    c: io_iocb_common,
}

#[repr(C)]
struct iocb {
    aio_data: u64,
    aio_key: u32,
    aio_rw_flags: u32,
    aio_lio_opcode: u16,
    aio_reqprio: i16,
    aio_fildes: u32,
    u: iocb_u,
}

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> size_t;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn strerror(errnum: c_int) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int;
    fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *mut c_void,
    ) -> c_int;
    fn eventfd(initval: c_uint, flags: c_int) -> c_int;

    fn io_setup(nr_events: c_uint, ctxp: *mut io_context_t) -> c_int;
    fn io_destroy(ctx: io_context_t) -> c_int;
    fn io_submit(ctx: io_context_t, nr: c_long, iocbpp: *mut *mut iocb) -> c_int;
    fn io_getevents(
        ctx: io_context_t,
        min_nr: c_long,
        nr: c_long,
        events: *mut io_event,
        timeout: *mut timespec,
    ) -> c_int;
    fn io_prep_pwrite(cb: *mut iocb, fd: c_int, buf: *mut c_void, count: size_t, offset: i64);
    fn io_prep_pread(cb: *mut iocb, fd: c_int, buf: *mut c_void, count: size_t, offset: i64);
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn FD_ZERO(set: *mut fd_set) {
    ptr::write_bytes(set as *mut u8, 0, size_of::<fd_set>());
}

unsafe fn FD_SET(fd: c_int, set: *mut fd_set) {
    let fd = fd as usize;
    let bits_per_word = 8 * size_of::<c_long>();
    (*set).fds_bits[fd / bits_per_word] |= (1 as c_long) << (fd % bits_per_word);
}

unsafe fn FD_ISSET(fd: c_int, set: *const fd_set) -> bool {
    let fd = fd as usize;
    let bits_per_word = 8 * size_of::<c_long>();
    ((*set).fds_bits[fd / bits_per_word] & ((1 as c_long) << (fd % bits_per_word))) != 0
}

/******************** Descriptors and Strings *******************************/

static descriptors: descriptors_type = descriptors_type {
    header: usb_functionfs_descs_head_v2 {
        magic: cpu_to_le32(FUNCTIONFS_DESCRIPTORS_MAGIC_V2),
        flags: cpu_to_le32(FUNCTIONFS_HAS_FS_DESC | FUNCTIONFS_HAS_HS_DESC),
        length: cpu_to_le32(size_of::<descriptors_type>() as u32),
    },
    fs_count: cpu_to_le32(3),
    fs_descs: descs {
        intf: usb_interface_descriptor {
            bLength: size_of::<usb_interface_descriptor>() as u8,
            bDescriptorType: USB_DT_INTERFACE,
            bNumEndpoints: 2,
            bInterfaceClass: USB_CLASS_VENDOR_SPEC,
            iInterface: 1,
            bInterfaceNumber: 0,
            bAlternateSetting: 0,
            bInterfaceSubClass: 0,
            bInterfaceProtocol: 0,
        },
        bulk_sink: usb_endpoint_descriptor_no_audio {
            bLength: size_of::<usb_endpoint_descriptor_no_audio>() as u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 1 | USB_DIR_IN,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: 0,
            bInterval: 0,
        },
        bulk_source: usb_endpoint_descriptor_no_audio {
            bLength: size_of::<usb_endpoint_descriptor_no_audio>() as u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 2 | USB_DIR_OUT,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: 0,
            bInterval: 0,
        },
    },
    hs_count: cpu_to_le32(3),
    hs_descs: descs {
        intf: usb_interface_descriptor {
            bLength: size_of::<usb_interface_descriptor>() as u8,
            bDescriptorType: USB_DT_INTERFACE,
            bNumEndpoints: 2,
            bInterfaceClass: USB_CLASS_VENDOR_SPEC,
            iInterface: 1,
            bInterfaceNumber: 0,
            bAlternateSetting: 0,
            bInterfaceSubClass: 0,
            bInterfaceProtocol: 0,
        },
        bulk_sink: usb_endpoint_descriptor_no_audio {
            bLength: size_of::<usb_endpoint_descriptor_no_audio>() as u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 1 | USB_DIR_IN,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: cpu_to_le16(512),
            bInterval: 0,
        },
        bulk_source: usb_endpoint_descriptor_no_audio {
            bLength: size_of::<usb_endpoint_descriptor_no_audio>() as u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 2 | USB_DIR_OUT,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: cpu_to_le16(512),
            bInterval: 0,
        },
    },
};

const STR_INTERFACE: &[u8; STR_INTERFACE_LEN] = b"AIO Test\0";
const STR_INTERFACE_LEN: usize = 9;

static strings: strings_type = strings_type {
    header: usb_functionfs_strings_head {
        magic: cpu_to_le32(FUNCTIONFS_STRINGS_MAGIC),
        length: cpu_to_le32(size_of::<strings_type>() as u32),
        str_count: cpu_to_le32(1),
        lang_count: cpu_to_le32(1),
    },
    lang0: lang0_type {
        code: cpu_to_le16(0x0409), /* en-us */
        str1: [
            STR_INTERFACE[0] as c_char,
            STR_INTERFACE[1] as c_char,
            STR_INTERFACE[2] as c_char,
            STR_INTERFACE[3] as c_char,
            STR_INTERFACE[4] as c_char,
            STR_INTERFACE[5] as c_char,
            STR_INTERFACE[6] as c_char,
            STR_INTERFACE[7] as c_char,
            STR_INTERFACE[8] as c_char,
        ],
    },
};

/******************** Endpoints handling *******************************/

unsafe fn display_event(event: *mut usb_functionfs_event) {
    static NAMES: [*const c_char; 7] = [
        b"BIND\0".as_ptr() as *const c_char,
        b"UNBIND\0".as_ptr() as *const c_char,
        b"ENABLE\0".as_ptr() as *const c_char,
        b"DISABLE\0".as_ptr() as *const c_char,
        b"SETUP\0".as_ptr() as *const c_char,
        b"SUSPEND\0".as_ptr() as *const c_char,
        b"RESUME\0".as_ptr() as *const c_char,
    ];
    match (*event).type_ as usize {
        FUNCTIONFS_BIND
        | FUNCTIONFS_UNBIND
        | FUNCTIONFS_ENABLE
        | FUNCTIONFS_DISABLE
        | FUNCTIONFS_SETUP
        | FUNCTIONFS_SUSPEND
        | FUNCTIONFS_RESUME => {
            printf(
                b"Event %s\n\0".as_ptr() as *const c_char,
                NAMES[(*event).type_ as usize],
            );
        }
        _ => {}
    }
}

unsafe fn handle_ep0(ep0: c_int, ready: *mut bool) {
    let mut event: usb_functionfs_event = zeroed();
    let mut ret: c_int;

    let mut pfds: [pollfd; 1] = [pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    }];
    pfds[0].fd = ep0;
    pfds[0].events = POLLIN;

    ret = poll(pfds.as_mut_ptr(), 1, 0);

    if ret != 0 && (pfds[0].revents & POLLIN) != 0 {
        ret = read(
            ep0,
            &mut event as *mut usb_functionfs_event as *mut c_void,
            size_of::<usb_functionfs_event>(),
        ) as c_int;
        if ret == 0 {
            perror(b"unable to read event from ep0\0".as_ptr() as *const c_char);
            return;
        }
        display_event(&mut event);
        match event.type_ as usize {
            FUNCTIONFS_SETUP => {
                if (event.u.setup.bRequestType & USB_DIR_IN) != 0 {
                    write(ep0, ptr::null(), 0);
                } else {
                    read(ep0, ptr::null_mut(), 0);
                }
            }

            FUNCTIONFS_ENABLE => {
                *ready = true;
            }

            FUNCTIONFS_DISABLE => {
                *ready = false;
            }

            _ => {}
        }
    }
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut i: c_int;
    let mut ret: c_int;
    let ep_path: *mut c_char;

    let ep0: c_int;
    let mut ep: [c_int; 2] = [0; 2];

    let mut ctx: io_context_t;

    let evfd: c_int;
    let mut rfds: fd_set;

    let buf_in: *mut c_char;
    let buf_out: *mut c_char;
    let iocb_in: *mut iocb;
    let iocb_out: *mut iocb;
    let mut req_in: c_int = 0;
    let mut req_out: c_int = 0;
    let mut ready: bool;

    if argc != 2 {
        printf(b"ffs directory not specified!\n\0".as_ptr() as *const c_char);
        return 1;
    }

    ep_path = malloc(strlen(*argv.add(1)) + 4 /* "/ep#" */ + 1 /* '\0' */) as *mut c_char;
    if ep_path.is_null() {
        perror(b"malloc\0".as_ptr() as *const c_char);
        return 1;
    }

    /* open endpoint files */
    sprintf(
        ep_path,
        b"%s/ep0\0".as_ptr() as *const c_char,
        *argv.add(1),
    );
    ep0 = open(ep_path, O_RDWR);
    if ep0 < 0 {
        perror(b"unable to open ep0\0".as_ptr() as *const c_char);
        return 1;
    }
    if write(
        ep0,
        &descriptors as *const descriptors_type as *const c_void,
        size_of::<descriptors_type>(),
    ) < 0
    {
        perror(b"unable do write descriptors\0".as_ptr() as *const c_char);
        return 1;
    }
    if write(
        ep0,
        &strings as *const strings_type as *const c_void,
        size_of::<strings_type>(),
    ) < 0
    {
        perror(b"unable to write strings\0".as_ptr() as *const c_char);
        return 1;
    }
    i = 0;
    while i < 2 {
        sprintf(
            ep_path,
            b"%s/ep%d\0".as_ptr() as *const c_char,
            *argv.add(1),
            i + 1,
        );
        ep[i as usize] = open(ep_path, O_RDWR);
        if ep[i as usize] < 0 {
            printf(
                b"unable to open ep%d: %s\n\0".as_ptr() as *const c_char,
                i + 1,
                strerror(errno()),
            );
            return 1;
        }
        i += 1;
    }

    free(ep_path as *mut c_void);

    ctx = zeroed();
    /* setup aio context to handle up to 2 requests */
    if io_setup(2, &mut ctx) < 0 {
        perror(b"unable to setup aio\0".as_ptr() as *const c_char);
        return 1;
    }

    evfd = eventfd(0, 0);
    if evfd < 0 {
        perror(b"unable to open eventfd\0".as_ptr() as *const c_char);
        return 1;
    }

    /* alloc buffers and requests */
    buf_in = malloc(BUF_LEN) as *mut c_char;
    buf_out = malloc(BUF_LEN) as *mut c_char;
    iocb_in = malloc(size_of::<iocb>()) as *mut iocb;
    iocb_out = malloc(size_of::<iocb>()) as *mut iocb;

    loop {
        FD_ZERO(&mut rfds);
        FD_SET(ep0, &mut rfds);
        FD_SET(evfd, &mut rfds);

        ret = select(
            (if ep0 > evfd { ep0 } else { evfd }) + 1,
            &mut rfds,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        if ret < 0 {
            if errno() == EINTR {
                continue;
            }
            perror(b"select\0".as_ptr() as *const c_char);
            break;
        }

        if FD_ISSET(ep0, &rfds) {
            handle_ep0(ep0, &mut ready);
        }

        /* we are waiting for function ENABLE */
        if !ready {
            continue;
        }

        /* if something was submitted we wait for event */
        if FD_ISSET(evfd, &rfds) {
            let mut ev_cnt: u64 = 0;
            ret = read(
                evfd,
                &mut ev_cnt as *mut u64 as *mut c_void,
                size_of::<u64>(),
            ) as c_int;
            if ret < 0 {
                perror(b"unable to read eventfd\0".as_ptr() as *const c_char);
                break;
            }

            let mut e: [io_event; 2] = zeroed();
            /* we wait for one event */
            ret = io_getevents(ctx, 1, 2, e.as_mut_ptr(), ptr::null_mut());
            /* if we got event */
            i = 0;
            while i < ret {
                if (*e[i as usize].obj).aio_fildes == ep[0] as u32 {
                    printf(
                        b"ev=in; ret=%lu\n\0".as_ptr() as *const c_char,
                        e[i as usize].res as c_ulong,
                    );
                    req_in = 0;
                } else if (*e[i as usize].obj).aio_fildes == ep[1] as u32 {
                    printf(
                        b"ev=out; ret=%lu\n\0".as_ptr() as *const c_char,
                        e[i as usize].res as c_ulong,
                    );
                    req_out = 0;
                }
                i += 1;
            }
        }

        if req_in == 0 {
            /* if IN transfer not requested*/
            /* prepare write request */
            io_prep_pwrite(iocb_in, ep[0], buf_in as *mut c_void, BUF_LEN, 0);
            /* enable eventfd notification */
            (*iocb_in).u.c.flags |= IOCB_FLAG_RESFD;
            (*iocb_in).u.c.resfd = evfd as u32;
            /* submit table of requests */
            ret = io_submit(ctx, 1, &mut (iocb_in as *mut iocb));
            if ret >= 0 {
                /* if ret > 0 request is queued */
                req_in = 1;
                printf(b"submit: in\n\0".as_ptr() as *const c_char);
            } else {
                perror(b"unable to submit request\0".as_ptr() as *const c_char);
            }
        }
        if req_out == 0 {
            /* if OUT transfer not requested */
            /* prepare read request */
            io_prep_pread(iocb_out, ep[1], buf_out as *mut c_void, BUF_LEN, 0);
            /* enable eventfs notification */
            (*iocb_out).u.c.flags |= IOCB_FLAG_RESFD;
            (*iocb_out).u.c.resfd = evfd as u32;
            /* submit table of requests */
            ret = io_submit(ctx, 1, &mut (iocb_out as *mut iocb));
            if ret >= 0 {
                /* if ret > 0 request is queued */
                req_out = 1;
                printf(b"submit: out\n\0".as_ptr() as *const c_char);
            } else {
                perror(b"unable to submit request\0".as_ptr() as *const c_char);
            }
        }
    }

    /* free resources */

    io_destroy(ctx);

    free(buf_in as *mut c_void);
    free(buf_out as *mut c_void);
    free(iocb_in as *mut c_void);
    free(iocb_out as *mut c_void);

    i = 0;
    while i < 2 {
        close(ep[i as usize]);
        i += 1;
    }
    close(ep0);

    0
}

fn main() {
    unsafe {
        let args: Vec<std::ffi::CString> = std::env::args()
            .map(|arg| std::ffi::CString::new(arg).unwrap())
            .collect();
        let mut argv: Vec<*mut c_char> = args.iter().map(|arg| arg.as_ptr() as *mut c_char).collect();
        argv.push(ptr::null_mut());
        std::process::exit(c_main(args.len() as c_int, argv.as_mut_ptr()));
    }
}
