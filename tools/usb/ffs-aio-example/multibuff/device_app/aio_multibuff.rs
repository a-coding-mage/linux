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

// C dependencies: endian.h, errno.h, fcntl.h, stdio.h, stdlib.h, string.h,
// sys/select.h, sys/eventfd.h, unistd.h, libaio.h, linux/usb/functionfs.h.

use libc::{
    c_char, c_int, c_long, c_uint, c_ulong, c_void, fd_set, size_t, ssize_t, time_t, timeval,
};
use std::mem;
use std::ptr;

const IOCB_FLAG_RESFD: u32 = 1 << 0;

const BUF_LEN: c_uint = 8192;
const BUFS_MAX: c_uint = 128;
const AIO_MAX: c_uint = BUFS_MAX * 2;

const FUNCTIONFS_DESCRIPTORS_MAGIC_V2: u32 = 3;
const FUNCTIONFS_STRINGS_MAGIC: u32 = 2;
const FUNCTIONFS_HAS_FS_DESC: u32 = 1;
const FUNCTIONFS_HAS_HS_DESC: u32 = 2;

const USB_DT_INTERFACE: u8 = 4;
const USB_DT_ENDPOINT: u8 = 5;
const USB_CLASS_VENDOR_SPEC: u8 = 0xff;
const USB_DIR_OUT: u8 = 0;
const USB_DIR_IN: u8 = 0x80;
const USB_ENDPOINT_XFER_BULK: u8 = 2;

const FUNCTIONFS_BIND: u8 = 0;
const FUNCTIONFS_UNBIND: u8 = 1;
const FUNCTIONFS_ENABLE: u8 = 2;
const FUNCTIONFS_DISABLE: u8 = 3;
const FUNCTIONFS_SETUP: u8 = 4;
const FUNCTIONFS_SUSPEND: u8 = 5;
const FUNCTIONFS_RESUME: u8 = 6;

const STR_INTERFACE: &[u8; 9] = b"AIO Test\0";

#[inline]
const fn htole16(x: u16) -> u16 {
    u16::to_le(x)
}

#[inline]
const fn htole32(x: u32) -> u32 {
    u32::to_le(x)
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct usb_functionfs_descs_head_v2 {
    magic: u32,
    length: u32,
    flags: u32,
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

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct usb_endpoint_descriptor_no_audio {
    bLength: u8,
    bDescriptorType: u8,
    bEndpointAddress: u8,
    bmAttributes: u8,
    wMaxPacketSize: u16,
    bInterval: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct desc_set {
    intf: usb_interface_descriptor,
    bulk_sink: usb_endpoint_descriptor_no_audio,
    bulk_source: usb_endpoint_descriptor_no_audio,
}

#[repr(C, packed)]
struct descriptors_type {
    header: usb_functionfs_descs_head_v2,
    fs_count: u32,
    hs_count: u32,
    fs_descs: desc_set,
    hs_descs: desc_set,
}

static descriptors: descriptors_type = descriptors_type {
    header: usb_functionfs_descs_head_v2 {
        magic: htole32(FUNCTIONFS_DESCRIPTORS_MAGIC_V2),
        flags: htole32(FUNCTIONFS_HAS_FS_DESC | FUNCTIONFS_HAS_HS_DESC),
        length: htole32(mem::size_of::<descriptors_type>() as u32),
    },
    fs_count: htole32(3),
    fs_descs: desc_set {
        intf: usb_interface_descriptor {
            bLength: mem::size_of::<usb_interface_descriptor>() as u8,
            bDescriptorType: USB_DT_INTERFACE,
            bInterfaceNumber: 0,
            bAlternateSetting: 0,
            bNumEndpoints: 2,
            bInterfaceClass: USB_CLASS_VENDOR_SPEC,
            bInterfaceSubClass: 0,
            bInterfaceProtocol: 0,
            iInterface: 1,
        },
        bulk_sink: usb_endpoint_descriptor_no_audio {
            bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 1 | USB_DIR_IN,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: 0,
            bInterval: 0,
        },
        bulk_source: usb_endpoint_descriptor_no_audio {
            bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 2 | USB_DIR_OUT,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: 0,
            bInterval: 0,
        },
    },
    hs_count: htole32(3),
    hs_descs: desc_set {
        intf: usb_interface_descriptor {
            bLength: mem::size_of::<usb_interface_descriptor>() as u8,
            bDescriptorType: USB_DT_INTERFACE,
            bInterfaceNumber: 0,
            bAlternateSetting: 0,
            bNumEndpoints: 2,
            bInterfaceClass: USB_CLASS_VENDOR_SPEC,
            bInterfaceSubClass: 0,
            bInterfaceProtocol: 0,
            iInterface: 1,
        },
        bulk_sink: usb_endpoint_descriptor_no_audio {
            bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 1 | USB_DIR_IN,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: htole16(512),
            bInterval: 0,
        },
        bulk_source: usb_endpoint_descriptor_no_audio {
            bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 2 | USB_DIR_OUT,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: htole16(512),
            bInterval: 0,
        },
    },
};

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct usb_functionfs_strings_head {
    magic: u32,
    length: u32,
    str_count: u32,
    lang_count: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct lang0_type {
    code: u16,
    str1: [c_char; 9],
}

#[repr(C, packed)]
struct strings_type {
    header: usb_functionfs_strings_head,
    lang0: lang0_type,
}

static strings: strings_type = strings_type {
    header: usb_functionfs_strings_head {
        magic: htole32(FUNCTIONFS_STRINGS_MAGIC),
        length: htole32(mem::size_of::<strings_type>() as u32),
        str_count: htole32(1),
        lang_count: htole32(1),
    },
    lang0: lang0_type {
        code: htole16(0x0409), /* en-us */
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

#[repr(C)]
struct io_buffer {
    iocb: *mut *mut iocb,
    buf: *mut *mut u8,
    cnt: c_uint,
    len: c_uint,
    requested: c_uint,
}

type io_context_t = c_ulong;

#[repr(C)]
struct io_event {
    data: u64,
    obj: u64,
    res: i64,
    res2: i64,
}

#[repr(C)]
struct io_c {
    flags: u32,
    resfd: u32,
}

#[repr(C)]
union iocb_u {
    c: io_c,
}

#[repr(C)]
struct iocb {
    _prefix: [u8; 48],
    u: iocb_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct usb_ctrlrequest {
    bRequestType: u8,
    bRequest: u8,
    wValue: u16,
    wIndex: u16,
    wLength: u16,
}

#[repr(C)]
union usb_functionfs_event_u {
    setup: usb_ctrlrequest,
}

#[repr(C)]
struct usb_functionfs_event {
    u: usb_functionfs_event_u,
    type_: u8,
    _pad: [u8; 3],
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *mut timeval,
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
    fn io_prep_pwrite(iocb: *mut iocb, fd: c_int, buf: *mut c_void, count: size_t, offset: i64);
    fn __errno_location() -> *mut c_int;
}

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

unsafe fn FD_ZERO(set: *mut fd_set) {
    libc::FD_ZERO(set);
}

unsafe fn FD_SET(fd: c_int, set: *mut fd_set) {
    libc::FD_SET(fd, set);
}

unsafe fn FD_ISSET(fd: c_int, set: *mut fd_set) -> bool {
    libc::FD_ISSET(fd, set)
}

unsafe fn display_event(event: *mut usb_functionfs_event) {
    static N_BIND: &[u8] = b"BIND\0";
    static N_UNBIND: &[u8] = b"UNBIND\0";
    static N_ENABLE: &[u8] = b"ENABLE\0";
    static N_DISABLE: &[u8] = b"DISABLE\0";
    static N_SETUP: &[u8] = b"SETUP\0";
    static N_SUSPEND: &[u8] = b"SUSPEND\0";
    static N_RESUME: &[u8] = b"RESUME\0";
    static NAMES: [*const c_char; 7] = [
        N_BIND.as_ptr() as *const c_char,
        N_UNBIND.as_ptr() as *const c_char,
        N_ENABLE.as_ptr() as *const c_char,
        N_DISABLE.as_ptr() as *const c_char,
        N_SETUP.as_ptr() as *const c_char,
        N_SUSPEND.as_ptr() as *const c_char,
        N_RESUME.as_ptr() as *const c_char,
    ];

    match (*event).type_ {
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
    let mut ret: c_int;
    let mut event: usb_functionfs_event = mem::zeroed();

    ret = read(
        ep0,
        &mut event as *mut usb_functionfs_event as *mut c_void,
        mem::size_of::<usb_functionfs_event>(),
    ) as c_int;
    if ret == 0 {
        perror(b"unable to read event from ep0\0".as_ptr() as *const c_char);
        return;
    }
    display_event(&mut event);
    match event.type_ {
        FUNCTIONFS_SETUP => {
            if event.u.setup.bRequestType & USB_DIR_IN != 0 {
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_bufs(iobuf: *mut io_buffer, n: c_uint, len: c_uint) {
    let mut i: c_uint;
    (*iobuf).buf = malloc((n as usize) * mem::size_of::<*mut u8>()) as *mut *mut u8;
    (*iobuf).iocb = malloc((n as usize) * mem::size_of::<*mut iocb>()) as *mut *mut iocb;
    (*iobuf).cnt = n;
    (*iobuf).len = len;
    (*iobuf).requested = 0;
    i = 0;
    while i < n {
        *(*iobuf).buf.add(i as usize) =
            malloc((len as usize) * mem::size_of::<u8>()) as *mut u8;
        *(*iobuf).iocb.add(i as usize) = malloc(mem::size_of::<iocb>()) as *mut iocb;
        i += 1;
    }
    (*iobuf).cnt = n;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_bufs(iobuf: *mut io_buffer) {
    let mut i: c_uint;
    i = 0;
    while i < (*iobuf).cnt {
        free(*(*iobuf).buf.add(i as usize) as *mut c_void);
        free(*(*iobuf).iocb.add(i as usize) as *mut c_void);
        i += 1;
    }
    free((*iobuf).buf as *mut c_void);
    free((*iobuf).iocb as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int;
    let mut i: c_uint;
    let mut j: c_uint;
    let mut ep_path: *mut c_char;

    let mut ep0: c_int;
    let mut ep1: c_int;

    let mut ctx: io_context_t;

    let mut evfd: c_int;
    let mut rfds: fd_set = mem::zeroed();

    let mut iobuf: [io_buffer; 2] = mem::zeroed();
    let mut actual: c_int = 0;
    let mut ready: bool = false;

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
    ep0 = open(ep_path, libc::O_RDWR);
    if ep0 < 0 {
        perror(b"unable to open ep0\0".as_ptr() as *const c_char);
        return 1;
    }
    if write(
        ep0,
        &descriptors as *const descriptors_type as *const c_void,
        mem::size_of::<descriptors_type>(),
    ) < 0
    {
        perror(b"unable do write descriptors\0".as_ptr() as *const c_char);
        return 1;
    }
    if write(
        ep0,
        &strings as *const strings_type as *const c_void,
        mem::size_of::<strings_type>(),
    ) < 0
    {
        perror(b"unable to write strings\0".as_ptr() as *const c_char);
        return 1;
    }
    sprintf(
        ep_path,
        b"%s/ep1\0".as_ptr() as *const c_char,
        *argv.add(1),
    );
    ep1 = open(ep_path, libc::O_RDWR);
    if ep1 < 0 {
        perror(b"unable to open ep1\0".as_ptr() as *const c_char);
        return 1;
    }

    free(ep_path as *mut c_void);

    ctx = 0;
    memset(
        &mut ctx as *mut io_context_t as *mut c_void,
        0,
        mem::size_of::<io_context_t>(),
    );
    /* setup aio context to handle up to AIO_MAX requests */
    if io_setup(AIO_MAX, &mut ctx) < 0 {
        perror(b"unable to setup aio\0".as_ptr() as *const c_char);
        return 1;
    }

    evfd = eventfd(0, 0);
    if evfd < 0 {
        perror(b"unable to open eventfd\0".as_ptr() as *const c_char);
        return 1;
    }

    i = 0;
    while (i as usize) < mem::size_of_val(&iobuf) / mem::size_of::<io_buffer>() {
        init_bufs(&mut iobuf[i as usize], BUFS_MAX, BUF_LEN);
        i += 1;
    }

    loop {
        FD_ZERO(&mut rfds);
        FD_SET(ep0, &mut rfds);
        FD_SET(evfd, &mut rfds);

        ret = select(
            if ep0 > evfd { ep0 } else { evfd } + 1,
            &mut rfds,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        if ret < 0 {
            if *__errno_location() == libc::EINTR {
                continue;
            }
            perror(b"select\0".as_ptr() as *const c_char);
            break;
        }

        if FD_ISSET(ep0, &mut rfds) {
            handle_ep0(ep0, &mut ready);
        }

        /* we are waiting for function ENABLE */
        if !ready {
            continue;
        }

        /*
         * when we're preparing new data to submit,
         * second buffer being transmitted
         */
        i = 0;
        while (i as usize) < mem::size_of_val(&iobuf) / mem::size_of::<io_buffer>() {
            if iobuf[i as usize].requested != 0 {
                i += 1;
                continue;
            }
            /* prepare requests */
            j = 0;
            while j < iobuf[i as usize].cnt {
                io_prep_pwrite(
                    *iobuf[i as usize].iocb.add(j as usize),
                    ep1,
                    *iobuf[i as usize].buf.add(j as usize) as *mut c_void,
                    iobuf[i as usize].len as size_t,
                    0,
                );
                /* enable eventfd notification */
                (*(*iobuf[i as usize].iocb.add(j as usize))).u.c.flags |= IOCB_FLAG_RESFD;
                (*(*iobuf[i as usize].iocb.add(j as usize))).u.c.resfd = evfd as u32;
                j += 1;
            }
            /* submit table of requests */
            ret = io_submit(ctx, iobuf[i as usize].cnt as c_long, iobuf[i as usize].iocb);
            if ret >= 0 {
                iobuf[i as usize].requested = ret as c_uint;
                printf(
                    b"submit: %d requests buf: %d\n\0".as_ptr() as *const c_char,
                    ret,
                    i,
                );
            } else {
                perror(b"unable to submit requests\0".as_ptr() as *const c_char);
            }
            i += 1;
        }

        /* if event is ready to read */
        if !FD_ISSET(evfd, &mut rfds) {
            continue;
        }

        let mut ev_cnt: u64 = 0;
        ret = read(
            evfd,
            &mut ev_cnt as *mut u64 as *mut c_void,
            mem::size_of::<u64>(),
        ) as c_int;
        if ret < 0 {
            perror(b"unable to read eventfd\0".as_ptr() as *const c_char);
            break;
        }

        let mut e: [io_event; BUFS_MAX as usize] = mem::zeroed();
        /* we read aio events */
        ret = io_getevents(ctx, 1, BUFS_MAX as c_long, e.as_mut_ptr(), ptr::null_mut());
        if ret > 0 {
            /* if we got events */
            iobuf[actual as usize].requested -= ret as c_uint;
        }

        /* if all req's from iocb completed */
        if iobuf[actual as usize].requested == 0 {
            actual = (actual + 1)
                % (mem::size_of_val(&iobuf) / mem::size_of::<io_buffer>()) as c_int;
        }
    }

    /* free resources */

    i = 0;
    while (i as usize) < mem::size_of_val(&iobuf) / mem::size_of::<io_buffer>() {
        delete_bufs(&mut iobuf[i as usize]);
        i += 1;
    }
    io_destroy(ctx);

    close(ep1);
    close(ep0);

    0
}
