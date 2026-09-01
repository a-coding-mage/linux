// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ffs-test.c -- user mode filesystem api for usb composite function
 *
 * Copyright (C) 2010 Samsung Electronics
 *                    Author: Michal Nazarewicz <mina86@mina86.com>
 */

/* $(CROSS_COMPILE)cc -Wall -Wextra -g -o ffs-test ffs-test.c -lpthread */

/*
 * C dependencies translated as external Rust dependencies:
 * endian.h, errno.h, fcntl.h, pthread.h, stdarg.h, stdbool.h, stdio.h,
 * stdlib.h, string.h, sys/ioctl.h, sys/stat.h, sys/types.h, unistd.h,
 * tools/le_byteshift.h, and include/uapi/linux/usb/functionfs.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void, CStr, CString};
use std::mem;
use std::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __le16 = u16;
type __le32 = u32;
type size_t = usize;
type ssize_t = isize;
type pthread_t = c_ulong;

/******************** External C and Linux definitions **********************/

const O_RDWR: c_int = 0o2;
const EINTR: c_int = 4;
const EAGAIN: c_int = 11;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const EILSEQ: c_int = 84;

const FUNCTIONFS_DESCRIPTORS_MAGIC: __u32 = 1;
const FUNCTIONFS_DESCRIPTORS_MAGIC_V2: __u32 = 3;
const FUNCTIONFS_STRINGS_MAGIC: __u32 = 2;
const FUNCTIONFS_HAS_FS_DESC: __u32 = 1;
const FUNCTIONFS_HAS_HS_DESC: __u32 = 2;
const FUNCTIONFS_HAS_SS_DESC: __u32 = 4;
const FUNCTIONFS_FIFO_STATUS: c_ulong = 1;
const FUNCTIONFS_FIFO_FLUSH: c_ulong = 2;

const USB_DT_INTERFACE: __u8 = 4;
const USB_DT_ENDPOINT: __u8 = 5;
const USB_DT_SS_ENDPOINT_COMP: __u8 = 48;
const USB_DT_SS_EP_COMP_SIZE: __u8 = 6;
const USB_CLASS_VENDOR_SPEC: __u8 = 0xff;
const USB_DIR_IN: __u8 = 0x80;
const USB_DIR_OUT: __u8 = 0;
const USB_ENDPOINT_XFER_BULK: __u8 = 2;

const FUNCTIONFS_BIND: usize = 0;
const FUNCTIONFS_UNBIND: usize = 1;
const FUNCTIONFS_ENABLE: usize = 2;
const FUNCTIONFS_DISABLE: usize = 3;
const FUNCTIONFS_SETUP: usize = 4;
const FUNCTIONFS_SUSPEND: usize = 5;
const FUNCTIONFS_RESUME: usize = 6;

#[repr(C, packed)]
struct usb_functionfs_descs_head_v2 {
    magic: __le32,
    length: __le32,
    flags: __le32,
}

#[repr(C, packed)]
struct usb_functionfs_descs_head {
    magic: __le32,
    length: __le32,
    fs_count: __le32,
    hs_count: __le32,
}

#[repr(C, packed)]
struct usb_functionfs_strings_head {
    magic: __le32,
    length: __le32,
    str_count: __le32,
    lang_count: __le32,
}

#[repr(C, packed)]
struct usb_interface_descriptor {
    bLength: __u8,
    bDescriptorType: __u8,
    bInterfaceNumber: __u8,
    bAlternateSetting: __u8,
    bNumEndpoints: __u8,
    bInterfaceClass: __u8,
    bInterfaceSubClass: __u8,
    bInterfaceProtocol: __u8,
    iInterface: __u8,
}

#[repr(C, packed)]
struct usb_endpoint_descriptor_no_audio {
    bLength: __u8,
    bDescriptorType: __u8,
    bEndpointAddress: __u8,
    bmAttributes: __u8,
    wMaxPacketSize: __le16,
    bInterval: __u8,
}

#[repr(C, packed)]
struct usb_ss_ep_comp_descriptor {
    bLength: __u8,
    bDescriptorType: __u8,
    bMaxBurst: __u8,
    bmAttributes: __u8,
    wBytesPerInterval: __le16,
}

#[repr(C, packed)]
struct usb_ctrlrequest {
    bRequestType: __u8,
    bRequest: __u8,
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
    type_: __u8,
    _pad: [__u8; 3],
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stdin: *mut c_void;
    static mut stdout: *mut c_void;
    static mut stderr: *mut c_void;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut c_void) -> size_t;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut c_void) -> size_t;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void,
                      start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
                      arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_testcancel();
    fn exit(status: c_int) -> !;
}

/******************** Little Endian Handling ********************************/

/*
 * cpu_to_le16/32 are used when initializing structures, a context where a
 * function call is not allowed. To solve this, we code cpu_to_le16/32 in a way
 * that allows them to be used when initializing structures.
 */

const fn cpu_to_le16(x: __u16) -> __le16 {
    x.to_le()
}

const fn cpu_to_le32(x: __u32) -> __le32 {
    x.to_le()
}

fn le32_to_cpu(x: __le32) -> __u32 {
    __u32::from_le(x)
}

fn le16_to_cpu(x: __le16) -> __u16 {
    __u16::from_le(x)
}

/******************** Messages and Errors ***********************************/

static argv0: &[u8] = b"ffs-test\0";
static mut verbosity: c_uint = 7;

unsafe fn _msg(level: c_uint, fmt: *const c_char) {
    let mut level = level;

    if level < 2 {
        level = 2;
    } else if level > 7 {
        level = 7;
    }

    if level <= verbosity {
        static levels: [[u8; 6]; 8] = [
            *b"\0\0\0\0\0\0",
            *b"\0\0\0\0\0\0",
            *b"crit:\0",
            *b"err: \0",
            *b"warn:\0",
            *b"note:\0",
            *b"info:\0",
            *b"dbg: \0",
        ];

        let _errno = errno;
        fprintf(stderr, b"%s: %s \0".as_ptr() as *const c_char,
                argv0.as_ptr(), levels[level as usize].as_ptr());
        fprintf(stderr, b"%s\0".as_ptr() as *const c_char, fmt);

        let fmt_len = CStr::from_ptr(fmt).to_bytes().len();
        if fmt_len != 0 && *fmt.add(fmt_len - 1) != b'\n' as c_char {
            let mut buffer = [0 as c_char; 128];
            strerror_r(_errno, buffer.as_mut_ptr(), buffer.len());
            fprintf(stderr, b": (-%d) %s\n\0".as_ptr() as *const c_char,
                    _errno, buffer.as_ptr());
        }

        fflush(stderr);
    }
}

unsafe fn die_fmt(fmt: *const c_char) -> ! {
    _msg(2, fmt);
    exit(1);
}

unsafe fn err_fmt(fmt: *const c_char) {
    _msg(3, fmt);
}

unsafe fn warn_fmt(fmt: *const c_char) {
    _msg(4, fmt);
}

unsafe fn info_fmt(fmt: *const c_char) {
    _msg(6, fmt);
}

unsafe fn debug_fmt(fmt: *const c_char) {
    _msg(7, fmt);
}

/******************** Descriptors and Strings *******************************/

#[repr(C, packed)]
struct fs_hs_descs {
    intf: usb_interface_descriptor,
    sink: usb_endpoint_descriptor_no_audio,
    source: usb_endpoint_descriptor_no_audio,
}

#[repr(C, packed)]
struct ss_descs {
    intf: usb_interface_descriptor,
    sink: usb_endpoint_descriptor_no_audio,
    sink_comp: usb_ss_ep_comp_descriptor,
    source: usb_endpoint_descriptor_no_audio,
    source_comp: usb_ss_ep_comp_descriptor,
}

#[repr(C, packed)]
struct descriptors_type {
    header: usb_functionfs_descs_head_v2,
    fs_count: __le32,
    hs_count: __le32,
    ss_count: __le32,
    fs_descs: fs_hs_descs,
    hs_descs: fs_hs_descs,
    ss_descs: ss_descs,
}

static descriptors: descriptors_type = descriptors_type {
    header: usb_functionfs_descs_head_v2 {
        magic: cpu_to_le32(FUNCTIONFS_DESCRIPTORS_MAGIC_V2),
        flags: cpu_to_le32(FUNCTIONFS_HAS_FS_DESC |
                           FUNCTIONFS_HAS_HS_DESC |
                           FUNCTIONFS_HAS_SS_DESC),
        length: cpu_to_le32(mem::size_of::<descriptors_type>() as __u32),
    },
    fs_count: cpu_to_le32(3),
    fs_descs: fs_hs_descs {
        intf: usb_interface_descriptor {
            bLength: mem::size_of::<usb_interface_descriptor>() as __u8,
            bDescriptorType: USB_DT_INTERFACE,
            bInterfaceNumber: 0,
            bAlternateSetting: 0,
            bNumEndpoints: 2,
            bInterfaceClass: USB_CLASS_VENDOR_SPEC,
            bInterfaceSubClass: 0,
            bInterfaceProtocol: 0,
            iInterface: 1,
        },
        sink: usb_endpoint_descriptor_no_audio {
            bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as __u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 1 | USB_DIR_IN,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: 0, /* autoconfiguration (kernel) */
            bInterval: 0,
        },
        source: usb_endpoint_descriptor_no_audio {
            bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as __u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 2 | USB_DIR_OUT,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: 0, /* autoconfiguration (kernel) */
            bInterval: 0,
        },
    },
    hs_count: cpu_to_le32(3),
    hs_descs: fs_hs_descs {
        intf: usb_interface_descriptor {
            bLength: mem::size_of::<usb_interface_descriptor>() as __u8,
            bDescriptorType: USB_DT_INTERFACE,
            bInterfaceNumber: 0,
            bAlternateSetting: 0,
            bNumEndpoints: 2,
            bInterfaceClass: USB_CLASS_VENDOR_SPEC,
            bInterfaceSubClass: 0,
            bInterfaceProtocol: 0,
            iInterface: 1,
        },
        sink: usb_endpoint_descriptor_no_audio {
            bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as __u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 1 | USB_DIR_IN,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: cpu_to_le16(512),
            bInterval: 0,
        },
        source: usb_endpoint_descriptor_no_audio {
            bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as __u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 2 | USB_DIR_OUT,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: cpu_to_le16(512),
            bInterval: 1, /* NAK every 1 uframe */
        },
    },
    ss_count: cpu_to_le32(5),
    ss_descs: ss_descs {
        intf: usb_interface_descriptor {
            bLength: mem::size_of::<usb_interface_descriptor>() as __u8,
            bDescriptorType: USB_DT_INTERFACE,
            bInterfaceNumber: 0,
            bAlternateSetting: 0,
            bNumEndpoints: 2,
            bInterfaceClass: USB_CLASS_VENDOR_SPEC,
            bInterfaceSubClass: 0,
            bInterfaceProtocol: 0,
            iInterface: 1,
        },
        sink: usb_endpoint_descriptor_no_audio {
            bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as __u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 1 | USB_DIR_IN,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: cpu_to_le16(1024),
            bInterval: 0,
        },
        sink_comp: usb_ss_ep_comp_descriptor {
            bLength: USB_DT_SS_EP_COMP_SIZE,
            bDescriptorType: USB_DT_SS_ENDPOINT_COMP,
            bMaxBurst: 0,
            bmAttributes: 0,
            wBytesPerInterval: 0,
        },
        source: usb_endpoint_descriptor_no_audio {
            bLength: mem::size_of::<usb_endpoint_descriptor_no_audio>() as __u8,
            bDescriptorType: USB_DT_ENDPOINT,
            bEndpointAddress: 2 | USB_DIR_OUT,
            bmAttributes: USB_ENDPOINT_XFER_BULK,
            wMaxPacketSize: cpu_to_le16(1024),
            bInterval: 1, /* NAK every 1 uframe */
        },
        source_comp: usb_ss_ep_comp_descriptor {
            bLength: USB_DT_SS_EP_COMP_SIZE,
            bDescriptorType: USB_DT_SS_ENDPOINT_COMP,
            bMaxBurst: 0,
            bmAttributes: 0,
            wBytesPerInterval: 0,
        },
    },
};

#[repr(C, packed)]
struct legacy_descs_prefix {
    header: usb_functionfs_descs_head,
}

unsafe fn descs_to_legacy(legacy: *mut *mut c_void, descriptors_v2: *const c_void) -> size_t {
    let descs_start: *const __u8;
    let mut length: __u32;
    let mut fs_count: __u32 = 0;
    let mut hs_count: __u32 = 0;
    let mut count: __u32;

    /* Read v2 header */
    {
        let header = descriptors_v2 as *const usb_functionfs_descs_head_v2;
        let mut counts = (header as *const __u8).add(mem::size_of::<usb_functionfs_descs_head_v2>())
            as *const __le32;
        let flags: __u32;

        if le32_to_cpu((*header).magic) != FUNCTIONFS_DESCRIPTORS_MAGIC_V2 {
            return 0;
        }

        length = le32_to_cpu((*header).length);
        if length <= mem::size_of::<usb_functionfs_descs_head_v2>() as __u32 {
            return 0;
        }
        length -= mem::size_of::<usb_functionfs_descs_head_v2>() as __u32;
        flags = le32_to_cpu((*header).flags);
        if flags & !(FUNCTIONFS_HAS_FS_DESC | FUNCTIONFS_HAS_HS_DESC | FUNCTIONFS_HAS_SS_DESC) != 0 {
            return 0;
        }

        if flags & FUNCTIONFS_HAS_FS_DESC != 0 {
            if length < 4 {
                return 0;
            }
            fs_count = le32_to_cpu(*counts);
            length -= 4;
            counts = counts.add(1);
        }
        if flags & FUNCTIONFS_HAS_HS_DESC != 0 {
            if length < 4 {
                return 0;
            }
            hs_count = le32_to_cpu(*counts);
            length -= 4;
            counts = counts.add(1);
        }
        if flags & FUNCTIONFS_HAS_SS_DESC != 0 {
            if length < 4 {
                return 0;
            }
            count = le32_to_cpu(*counts);
            length -= 4;
            counts = counts.add(1);
        } else {
            count = 0;
        }

        count = fs_count + hs_count;
        if count == 0 {
            return 0;
        }
        descs_start = counts as *const __u8;
    }

    /*
     * Find the end of FS and HS USB descriptors.  SS descriptors
     * are ignored since legacy format does not support them.
     */
    let mut descs_end = descs_start;
    loop {
        if length < *descs_end as __u32 {
            return 0;
        }
        length -= *descs_end as __u32;
        descs_end = descs_end.add(*descs_end as usize);
        count -= 1;
        if count == 0 {
            break;
        }
    }

    /* Allocate legacy descriptors and copy the data. */
    {
        let desc_bytes = descs_end.offset_from(descs_start) as size_t;
        length = (mem::size_of::<usb_functionfs_descs_head>() + desc_bytes) as __u32;
        let out = malloc(length as size_t) as *mut legacy_descs_prefix;
        (*out).header.magic = cpu_to_le32(FUNCTIONFS_DESCRIPTORS_MAGIC);
        (*out).header.length = cpu_to_le32(length);
        (*out).header.fs_count = cpu_to_le32(fs_count);
        (*out).header.hs_count = cpu_to_le32(hs_count);
        let out_descriptors = (out as *mut __u8).add(mem::size_of::<usb_functionfs_descs_head>());
        memcpy(out_descriptors as *mut c_void, descs_start as *const c_void, desc_bytes);
        *legacy = out as *mut c_void;
    }

    length as size_t
}

const STR_INTERFACE_: &[u8; 12] = b"Source/Sink\0";

#[repr(C, packed)]
struct strings_lang0 {
    code: __le16,
    str1: [c_char; 12],
}

#[repr(C, packed)]
struct strings_type {
    header: usb_functionfs_strings_head,
    lang0: strings_lang0,
}

static strings: strings_type = strings_type {
    header: usb_functionfs_strings_head {
        magic: cpu_to_le32(FUNCTIONFS_STRINGS_MAGIC),
        length: cpu_to_le32(mem::size_of::<strings_type>() as __u32),
        str_count: cpu_to_le32(1),
        lang_count: cpu_to_le32(1),
    },
    lang0: strings_lang0 {
        code: cpu_to_le16(0x0409), /* en-us */
        str1: *b"Source/Sink\0" as [c_char; 12],
    },
};

/******************** Files and Threads Handling ****************************/

#[repr(C)]
struct thread {
    filename: *const c_char,
    buf_size: size_t,

    in_: unsafe fn(*mut thread, *mut c_void, size_t) -> ssize_t,
    in_name: *const c_char,

    out: unsafe fn(*mut thread, *const c_void, size_t) -> ssize_t,
    out_name: *const c_char,

    fd: c_int,
    id: pthread_t,
    buf: *mut c_void,
    status: ssize_t,
}

unsafe impl Sync for thread {}

static mut threads: [thread; 3] = [
    thread {
        filename: b"ep0\0".as_ptr() as *const c_char,
        buf_size: 4 * mem::size_of::<usb_functionfs_event>(),
        in_: read_wrap,
        in_name: ptr::null(),
        out: ep0_consume,
        out_name: b"<consume>\0".as_ptr() as *const c_char,
        fd: 0,
        id: 0,
        buf: ptr::null_mut(),
        status: 0,
    },
    thread {
        filename: b"ep1\0".as_ptr() as *const c_char,
        buf_size: 8 * 1024,
        in_: fill_in_buf,
        in_name: b"<in>\0".as_ptr() as *const c_char,
        out: write_wrap,
        out_name: ptr::null(),
        fd: 0,
        id: 0,
        buf: ptr::null_mut(),
        status: 0,
    },
    thread {
        filename: b"ep2\0".as_ptr() as *const c_char,
        buf_size: 8 * 1024,
        in_: read_wrap,
        in_name: ptr::null(),
        out: empty_out_buf,
        out_name: b"<out>\0".as_ptr() as *const c_char,
        fd: 0,
        id: 0,
        buf: ptr::null_mut(),
        status: 0,
    },
];

unsafe fn init_thread(t: *mut thread) {
    (*t).buf = malloc((*t).buf_size);
    if (*t).buf.is_null() {
        die_fmt(b"malloc\0".as_ptr() as *const c_char);
    }

    (*t).fd = open((*t).filename, O_RDWR);
    if (*t).fd < 0 {
        die_fmt((*t).filename);
    }
}

unsafe extern "C" fn cleanup_thread(arg: *mut c_void) {
    let t = arg as *mut thread;
    let ret: c_int;
    let fd: c_int;

    fd = (*t).fd;
    if (*t).fd < 0 {
        return;
    }
    (*t).fd = -1;

    /* test the FIFO ioctls (non-ep0 code paths) */
    if t != threads.as_mut_ptr() {
        ret = ioctl(fd, FUNCTIONFS_FIFO_STATUS);
        if ret < 0 {
            /* ENODEV reported after disconnect */
            if errno != ENODEV {
                err_fmt((*t).filename);
            }
        } else if ret != 0 {
            warn_fmt((*t).filename);
            if ioctl(fd, FUNCTIONFS_FIFO_FLUSH) < 0 {
                err_fmt((*t).filename);
            }
        }
    }

    if close(fd) < 0 {
        err_fmt((*t).filename);
    }

    free((*t).buf);
    (*t).buf = ptr::null_mut();
}

unsafe extern "C" fn start_thread_helper(arg: *mut c_void) -> *mut c_void {
    let mut name: *const c_char;
    let mut op: *const c_char;
    let in_name: *const c_char;
    let out_name: *const c_char;
    let t = arg as *mut thread;
    let mut ret: ssize_t;

    info_fmt((*t).filename);
    in_name = if !(*t).in_name.is_null() { (*t).in_name } else { (*t).filename };
    out_name = if !(*t).out_name.is_null() { (*t).out_name } else { (*t).filename };

    loop {
        pthread_testcancel();

        ret = ((*t).in_)(t, (*t).buf, (*t).buf_size);
        if ret > 0 {
            ret = ((*t).out)(t, (*t).buf, ret as size_t);
            name = out_name;
            op = b"write\0".as_ptr() as *const c_char;
        } else {
            name = in_name;
            op = b"read\0".as_ptr() as *const c_char;
        }

        if ret > 0 {
            /* nop */
        } else if ret == 0 {
            debug_fmt(name);
            break;
        } else if errno == EINTR || errno == EAGAIN {
            debug_fmt(name);
        } else {
            warn_fmt(name);
            break;
        }
    }

    cleanup_thread(arg);

    (*t).status = ret;
    info_fmt((*t).filename);
    ptr::null_mut()
}

unsafe fn start_thread(t: *mut thread) {
    debug_fmt((*t).filename);

    if pthread_create(&mut (*t).id, ptr::null(), start_thread_helper, t as *mut c_void) < 0 {
        die_fmt(b"pthread_create\0".as_ptr() as *const c_char);
    }
}

unsafe fn join_thread(t: *mut thread) {
    let ret = pthread_join((*t).id, ptr::null_mut());

    if ret < 0 {
        err_fmt((*t).filename);
    } else {
        debug_fmt((*t).filename);
    }
}

unsafe fn read_wrap(t: *mut thread, buf: *mut c_void, nbytes: size_t) -> ssize_t {
    read((*t).fd, buf, nbytes)
}

unsafe fn write_wrap(t: *mut thread, buf: *const c_void, nbytes: size_t) -> ssize_t {
    write((*t).fd, buf, nbytes)
}

/******************** Empty/Fill buffer routines ****************************/

/* 0 -- stream of zeros, 1 -- i % 63, 2 -- pipe */
#[repr(C)]
enum pattern {
    PAT_ZERO,
    PAT_SEQ,
    PAT_PIPE,
}

static mut pattern: pattern = pattern::PAT_ZERO;

unsafe fn fill_in_buf(ignore: *mut thread, buf: *mut c_void, nbytes: size_t) -> ssize_t {
    let mut i: size_t;
    let mut p: *mut __u8;

    let _ = ignore;

    match pattern {
        pattern::PAT_ZERO => {
            memset(buf, 0, nbytes);
        }

        pattern::PAT_SEQ => {
            p = buf as *mut __u8;
            i = 0;
            while i < nbytes {
                *p = (i % 63) as __u8;
                i += 1;
                p = p.add(1);
            }
        }

        pattern::PAT_PIPE => {
            return fread(buf, 1, nbytes, stdin) as ssize_t;
        }
    }

    nbytes as ssize_t
}

unsafe fn empty_out_buf(ignore: *mut thread, buf: *const c_void, nbytes: size_t) -> ssize_t {
    let mut p: *const __u8;
    let mut expected: __u8 = 0;
    let ret: ssize_t;
    let mut len: size_t = 0;

    let _ = ignore;

    match pattern {
        pattern::PAT_ZERO => {
            expected = 0;
            p = buf as *const __u8;
            len = 0;
            while len < nbytes {
                if *p != 0 {
                    break;
                }
                p = p.add(1);
                len += 1;
            }
            if len < nbytes {
                err_fmt(b"bad OUT byte\0".as_ptr() as *const c_char);
                errno = EILSEQ;
                return -1;
            }
        }

        pattern::PAT_SEQ => {
            p = buf as *const __u8;
            len = 0;
            while len < nbytes {
                if *p != (len % 63) as __u8 {
                    expected = (len % 63) as __u8;
                    err_fmt(b"bad OUT byte\0".as_ptr() as *const c_char);
                    errno = EILSEQ;
                    return -1;
                }
                p = p.add(1);
                len += 1;
            }
        }

        pattern::PAT_PIPE => {
            ret = fwrite(buf, nbytes, 1, stdout) as ssize_t;
            if ret > 0 {
                fflush(stdout);
            }
        }
    }

    len as ssize_t
}

/******************** Endpoints routines ************************************/

unsafe fn handle_setup(setup: *const usb_ctrlrequest) {
    printf(b"bRequestType = %d\n\0".as_ptr() as *const c_char, (*setup).bRequestType as c_int);
    printf(b"bRequest     = %d\n\0".as_ptr() as *const c_char, (*setup).bRequest as c_int);
    printf(b"wValue       = %d\n\0".as_ptr() as *const c_char, le16_to_cpu((*setup).wValue) as c_int);
    printf(b"wIndex       = %d\n\0".as_ptr() as *const c_char, le16_to_cpu((*setup).wIndex) as c_int);
    printf(b"wLength      = %d\n\0".as_ptr() as *const c_char, le16_to_cpu((*setup).wLength) as c_int);
}

unsafe fn ep0_consume(ignore: *mut thread, buf: *const c_void, nbytes: size_t) -> ssize_t {
    static names: [*const c_char; 7] = [
        b"BIND\0".as_ptr() as *const c_char,
        b"UNBIND\0".as_ptr() as *const c_char,
        b"ENABLE\0".as_ptr() as *const c_char,
        b"DISABLE\0".as_ptr() as *const c_char,
        b"SETUP\0".as_ptr() as *const c_char,
        b"SUSPEND\0".as_ptr() as *const c_char,
        b"RESUME\0".as_ptr() as *const c_char,
    ];

    let mut event = buf as *const usb_functionfs_event;
    let mut n: size_t;

    let _ = ignore;

    n = nbytes / mem::size_of::<usb_functionfs_event>();
    while n != 0 {
        match (*event).type_ as usize {
            FUNCTIONFS_BIND |
            FUNCTIONFS_UNBIND |
            FUNCTIONFS_ENABLE |
            FUNCTIONFS_DISABLE |
            FUNCTIONFS_SETUP |
            FUNCTIONFS_SUSPEND |
            FUNCTIONFS_RESUME => {
                printf(b"Event %s\n\0".as_ptr() as *const c_char, names[(*event).type_ as usize]);
                if (*event).type_ as usize == FUNCTIONFS_SETUP {
                    handle_setup(&(*event).u.setup);
                }
            }

            _ => {
                printf(b"Event %03u (unknown)\n\0".as_ptr() as *const c_char, (*event).type_ as c_uint);
            }
        }
        n -= 1;
        event = event.add(1);
    }

    nbytes as ssize_t
}

unsafe fn ep0_init(t: *mut thread, legacy_descriptors: bool) {
    let mut legacy: *mut c_void = ptr::null_mut();
    let mut ret: ssize_t = 0;
    let len: size_t;

    if legacy_descriptors {
        info_fmt((*t).filename);
    } else {
        info_fmt((*t).filename);
        ret = write((*t).fd, &descriptors as *const _ as *const c_void,
                    mem::size_of::<descriptors_type>());
    }

    if legacy_descriptors || (ret < 0 && errno == EINVAL) {
        if !legacy_descriptors {
            warn_fmt((*t).filename);
        }
        len = descs_to_legacy(&mut legacy, &descriptors as *const _ as *const c_void);
        if len != 0 {
            ret = write((*t).fd, legacy, len);
            free(legacy);
        }
    }
    if ret < 0 {
        die_fmt((*t).filename);
    }

    info_fmt((*t).filename);
    ret = write((*t).fd, &strings as *const _ as *const c_void, mem::size_of::<strings_type>());
    if ret < 0 {
        die_fmt((*t).filename);
    }
}

/******************** Main **************************************************/

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let legacy_descriptors: bool;
    let mut i: c_uint;

    legacy_descriptors = argc > 2 &&
        strcmp(*argv.add(1), b"-l\0".as_ptr() as *const c_char) == 0;

    init_thread(threads.as_mut_ptr());
    ep0_init(threads.as_mut_ptr(), legacy_descriptors);

    i = 1;
    while (i as usize) < threads.len() {
        init_thread(threads.as_mut_ptr().add(i as usize));
        i += 1;
    }

    i = 1;
    while (i as usize) < threads.len() {
        start_thread(threads.as_mut_ptr().add(i as usize));
        i += 1;
    }

    start_thread_helper(threads.as_mut_ptr() as *mut c_void);

    i = 1;
    while (i as usize) < threads.len() {
        join_thread(threads.as_mut_ptr().add(i as usize));
        i += 1;
    }

    0
}

fn main() {
    let args: Vec<CString> = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();
    let mut argv: Vec<*mut c_char> = args.iter()
        .map(|arg| arg.as_ptr() as *mut c_char)
        .collect();
    argv.push(ptr::null_mut());
    unsafe {
        c_main(args.len() as c_int, argv.as_mut_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
