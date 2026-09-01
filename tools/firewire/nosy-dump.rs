// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * nosy-dump - Interface to snoop mode driver for TI PCILynx 1394 controllers
 * Copyright (C) 2002-2006 Kristian Høgsberg
 */

use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void, CStr, CString};
use std::mem;
use std::ptr;

type size_t = usize;
type uint32_t = u32;
type FILE = c_void;
type poptContext = *mut c_void;
type sig_t = Option<unsafe extern "C" fn(c_int)>;

const EXIT_FAILURE: c_int = 1;
const STDIN_FILENO: c_int = 0;
const SIGINT: c_int = 2;
const SIG_DFL: sig_t = None;
const O_RDWR: c_int = 0x0002;
const POLLIN: i16 = 0x0001;
const _IOLBF: c_int = 1;
const BUFSIZ: size_t = 8192;
const TCSANOW: c_int = 0;
const TCSAFLUSH: c_int = 2;
const ICANON: c_uint = 0x0002;
const ECHO: c_uint = 0x0008;
const VMIN: usize = 6;
const VTIME: usize = 5;

const POPT_ARG_NONE: c_int = 0;
const POPT_ARG_STRING: c_int = 1;

const PACKET_FIELD_DETAIL: c_int = 0x01;
const PACKET_FIELD_DATA_LENGTH: c_int = 0x02;
/* Marks the fields we print in transaction view. */
const PACKET_FIELD_TRANSACTION: c_int = 0x04;

const VIEW_TRANSACTION: c_int = 0;
const VIEW_PACKET: c_int = 1;
const VIEW_STATS: c_int = 2;

const PACKET_RESERVED: c_int = 0;
const PACKET_REQUEST: c_int = 1;
const PACKET_RESPONSE: c_int = 2;
const PACKET_OTHER: c_int = 3;

const HIDE_CURSOR: &[u8] = b"\x1b[?25l\0";
const SHOW_CURSOR: &[u8] = b"\x1b[?25h\0";
const CLEAR: &[u8] = b"\x1b[H\x1b[2J\0";

/* Constants, ioctls, packet layouts, list primitives, VERSION, and decode_fcp
 * are supplied by the translated equivalents of linux/firewire-constants.h,
 * list.h, nosy-dump.h, and nosy-user.h.
 */
extern "C" {
    static VERSION: *const c_char;
    static NOSY_IOC_FILTER: c_ulong;
    static NOSY_IOC_START: c_ulong;

    static TCODE_WRITE_QUADLET_REQUEST: c_int;
    static TCODE_WRITE_BLOCK_REQUEST: c_int;
    static TCODE_WRITE_RESPONSE: c_int;
    static TCODE_READ_QUADLET_RESPONSE: c_int;
    static TCODE_READ_BLOCK_RESPONSE: c_int;
    static TCODE_STREAM_DATA: c_int;
    static TCODE_CYCLE_START: c_int;

    static ACK_COMPLETE: uint32_t;
    static ACK_NO_ACK: uint32_t;
    static ACK_PENDING: uint32_t;
    static ACK_BUSY_X: uint32_t;
    static ACK_BUSY_A: uint32_t;
    static ACK_BUSY_B: uint32_t;
    static ACK_DATA_ERROR: uint32_t;
    static ACK_TYPE_ERROR: uint32_t;

    static PHY_PACKET_CONFIGURATION: c_int;
    static PHY_PACKET_LINK_ON: c_int;
    static PHY_PACKET_SELF_ID: c_int;

    fn ACK_BUSY(ack: uint32_t) -> bool;
    fn decode_fcp(t: *mut link_transaction) -> c_int;

    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn setvbuf(stream: *mut FILE, buf: *mut c_char, mode: c_int, size: size_t) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn signal(signum: c_int, handler: sig_t) -> sig_t;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn isatty(fd: c_int) -> c_int;
    fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
    fn atexit(function: unsafe extern "C" fn()) -> c_int;
    fn poptGetContext(
        name: *const c_char,
        argc: c_int,
        argv: *const *const c_char,
        options: *const poptOption,
        flags: c_int,
    ) -> poptContext;
    fn poptGetNextOpt(con: poptContext) -> c_int;
    fn poptPrintUsage(con: poptContext, fp: *mut FILE, flags: c_int);
    fn poptFreeContext(con: poptContext);

    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
}

#[repr(C)]
struct list {
    next: *mut list,
    prev: *mut list,
}

#[repr(C)]
struct subaction {
    link: list,
    ack: uint32_t,
    length: size_t,
    packet: link_packet,
}

#[repr(C)]
struct link_transaction {
    link: list,
    request_node: c_int,
    response_node: c_int,
    tlabel: c_int,
    request_list: list,
    response_list: list,
    request: *mut subaction,
    response: *mut subaction,
}

#[repr(C)]
struct link_packet {
    common: link_packet_common,
}

#[repr(C)]
struct link_packet_common {
    destination: c_int,
    tlabel: c_int,
    tcode: usize,
    source: c_int,
}

#[repr(C)]
struct phy_packet {
    common: phy_packet_common,
    phy_config: phy_packet_phy_config,
    link_on: phy_packet_link_on,
    self_id: phy_packet_self_id,
    ext_self_id: phy_packet_ext_self_id,
}

#[repr(C)]
struct phy_packet_common {
    identifier: c_int,
}

#[repr(C)]
struct phy_packet_phy_config {
    set_root: bool,
    set_gap_count: bool,
    root_id: c_int,
    gap_count: c_uint,
}

#[repr(C)]
struct phy_packet_link_on {
    phy_id: c_int,
}

#[repr(C)]
struct phy_packet_self_id {
    extended: bool,
    phy_id: c_int,
    link_active: bool,
    gap_count: c_uint,
    phy_speed: usize,
    contender: bool,
    initiated_reset: bool,
}

#[repr(C)]
struct phy_packet_ext_self_id {
    phy_id: c_int,
    sequence: c_uint,
}

#[repr(C)]
struct packet_info {
    name: *const c_char,
    type_: c_int,
    response_tcode: c_int,
    fields: *const packet_field,
    field_count: c_int,
}

#[repr(C)]
struct packet_field {
    name: *const c_char, /* Short name for field. */
    offset: c_int,      /* Location of field, specified in bits; */
    /* negative means from end of packet.    */
    width: c_int, /* Width of field, 0 means use data_length. */
    flags: c_int, /* Show options. */
    value_names: *const *const c_char,
}

#[repr(C)]
struct protocol_decoder {
    name: *const c_char,
    decode: unsafe extern "C" fn(*mut link_transaction) -> c_int,
}

#[repr(C)]
struct poptOption {
    longName: *const c_char,
    shortName: c_char,
    argInfo: c_int,
    arg: *mut c_void,
    val: c_int,
    descrip: *const c_char,
    argDescrip: *const c_char,
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: i16,
    revents: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timeval {
    tv_sec: isize,
    tv_usec: isize,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct termios {
    c_iflag: c_uint,
    c_oflag: c_uint,
    c_cflag: c_uint,
    c_lflag: c_uint,
    c_line: u8,
    c_cc: [u8; 32],
    c_ispeed: c_uint,
    c_ospeed: c_uint,
}

static mut run: c_int = 1;
static mut sys_sigint_handler: sig_t = None;

static mut option_nosy_device: *const c_char = b"/dev/nosy\0".as_ptr() as *const c_char;
static mut option_view: *const c_char = b"packet\0".as_ptr() as *const c_char;
static mut option_output: *const c_char = ptr::null();
static mut option_input: *const c_char = ptr::null();
static mut option_hex: c_int = 0;
static mut option_iso: c_int = 0;
static mut option_cycle_start: c_int = 0;
static mut option_version: c_int = 0;
static mut option_verbose: c_int = 0;

static mut options: [poptOption; 10] = [
    poptOption { longName: b"device\0".as_ptr() as *const c_char, shortName: b'd' as c_char, argInfo: POPT_ARG_STRING, arg: unsafe { &mut option_nosy_device as *mut _ as *mut c_void }, val: 0, descrip: b"Path to nosy device.\0".as_ptr() as *const c_char, argDescrip: b"DEVICE\0".as_ptr() as *const c_char },
    poptOption { longName: b"view\0".as_ptr() as *const c_char, shortName: 0, argInfo: POPT_ARG_STRING, arg: unsafe { &mut option_view as *mut _ as *mut c_void }, val: 0, descrip: b"Specify view of bus traffic: packet, transaction or stats.\0".as_ptr() as *const c_char, argDescrip: b"VIEW\0".as_ptr() as *const c_char },
    poptOption { longName: b"hex\0".as_ptr() as *const c_char, shortName: b'x' as c_char, argInfo: POPT_ARG_NONE, arg: unsafe { &mut option_hex as *mut _ as *mut c_void }, val: 0, descrip: b"Print each packet in hex.\0".as_ptr() as *const c_char, argDescrip: ptr::null() },
    poptOption { longName: b"iso\0".as_ptr() as *const c_char, shortName: 0, argInfo: POPT_ARG_NONE, arg: unsafe { &mut option_iso as *mut _ as *mut c_void }, val: 0, descrip: b"Print iso packets.\0".as_ptr() as *const c_char, argDescrip: ptr::null() },
    poptOption { longName: b"cycle-start\0".as_ptr() as *const c_char, shortName: 0, argInfo: POPT_ARG_NONE, arg: unsafe { &mut option_cycle_start as *mut _ as *mut c_void }, val: 0, descrip: b"Print cycle start packets.\0".as_ptr() as *const c_char, argDescrip: ptr::null() },
    poptOption { longName: b"verbose\0".as_ptr() as *const c_char, shortName: b'v' as c_char, argInfo: POPT_ARG_NONE, arg: unsafe { &mut option_verbose as *mut _ as *mut c_void }, val: 0, descrip: b"Verbose packet view.\0".as_ptr() as *const c_char, argDescrip: ptr::null() },
    poptOption { longName: b"output\0".as_ptr() as *const c_char, shortName: b'o' as c_char, argInfo: POPT_ARG_STRING, arg: unsafe { &mut option_output as *mut _ as *mut c_void }, val: 0, descrip: b"Log to output file.\0".as_ptr() as *const c_char, argDescrip: b"FILENAME\0".as_ptr() as *const c_char },
    poptOption { longName: b"input\0".as_ptr() as *const c_char, shortName: b'i' as c_char, argInfo: POPT_ARG_STRING, arg: unsafe { &mut option_input as *mut _ as *mut c_void }, val: 0, descrip: b"Decode log from file.\0".as_ptr() as *const c_char, argDescrip: b"FILENAME\0".as_ptr() as *const c_char },
    poptOption { longName: b"version\0".as_ptr() as *const c_char, shortName: 0, argInfo: POPT_ARG_NONE, arg: unsafe { &mut option_version as *mut _ as *mut c_void }, val: 0, descrip: b"Specify print version info.\0".as_ptr() as *const c_char, argDescrip: ptr::null() },
    poptOption { longName: ptr::null(), shortName: 0, argInfo: 0, arg: ptr::null_mut(), val: 0, descrip: ptr::null(), argDescrip: ptr::null() },
];

/* Allow all ^C except the first to interrupt the program in the usual way. */
unsafe extern "C" fn sigint_handler(_signal_num: c_int) {
    if run == 1 {
        run = 0;
        signal(SIGINT, SIG_DFL);
    }
}

unsafe fn list_init(l: *mut list) {
    (*l).next = l;
    (*l).prev = l;
}

unsafe fn list_empty(l: *const list) -> bool {
    (*l).next == l as *mut list
}

unsafe fn list_append(head: *mut list, elem: *mut list) {
    (*elem).prev = (*head).prev;
    (*elem).next = head;
    (*(*head).prev).next = elem;
    (*head).prev = elem;
}

unsafe fn list_remove(elem: *mut list) {
    (*(*elem).prev).next = (*elem).next;
    (*(*elem).next).prev = (*elem).prev;
}

unsafe fn container_of_subaction_link(link: *mut list) -> *mut subaction {
    (link as *mut u8).offset(-(mem::offset_of!(subaction, link) as isize)) as *mut subaction
}

unsafe fn container_of_link_transaction_link(link: *mut list) -> *mut link_transaction {
    (link as *mut u8).offset(-(mem::offset_of!(link_transaction, link) as isize)) as *mut link_transaction
}

unsafe fn list_head_subaction(head: *mut list) -> *mut subaction {
    container_of_subaction_link((*head).next)
}

unsafe fn list_tail_subaction(head: *mut list) -> *mut subaction {
    container_of_subaction_link((*head).prev)
}

unsafe fn list_head_link_transaction(head: *mut list) -> *mut link_transaction {
    container_of_link_transaction_link((*head).next)
}

unsafe fn subaction_create(data: *mut uint32_t, length: size_t) -> *mut subaction {
    let size = mem::size_of::<subaction>() - mem::size_of::<link_packet>() + length;
    let sa = malloc(size) as *mut subaction;
    if sa.is_null() {
        exit(EXIT_FAILURE);
    }
    (*sa).ack = *data.add(length / 4 - 1);
    (*sa).length = length;
    memcpy(&mut (*sa).packet as *mut _ as *mut c_void, data as *const c_void, length);
    sa
}

unsafe fn subaction_destroy(sa: *mut subaction) {
    free(sa as *mut c_void);
}

static mut pending_transaction_list: list = list {
    next: unsafe { &mut pending_transaction_list as *mut list },
    prev: unsafe { &mut pending_transaction_list as *mut list },
};

unsafe fn link_transaction_lookup(
    request_node: c_int,
    response_node: c_int,
    tlabel: c_int,
) -> *mut link_transaction {
    let mut pos = pending_transaction_list.next;
    while pos != &mut pending_transaction_list as *mut list {
        let t = container_of_link_transaction_link(pos);
        if (*t).request_node == request_node
            && (*t).response_node == response_node
            && (*t).tlabel == tlabel
        {
            return t;
        }
        pos = (*pos).next;
    }

    let t = malloc(mem::size_of::<link_transaction>()) as *mut link_transaction;
    if t.is_null() {
        exit(EXIT_FAILURE);
    }
    (*t).request_node = request_node;
    (*t).response_node = response_node;
    (*t).tlabel = tlabel;
    (*t).request = ptr::null_mut();
    (*t).response = ptr::null_mut();
    list_init(&mut (*t).request_list);
    list_init(&mut (*t).response_list);

    list_append(&mut pending_transaction_list, &mut (*t).link);
    t
}

unsafe fn link_transaction_destroy(t: *mut link_transaction) {
    while !list_empty(&(*t).request_list) {
        let sa = list_head_subaction(&mut (*t).request_list);
        list_remove(&mut (*sa).link);
        subaction_destroy(sa);
    }
    while !list_empty(&(*t).response_list) {
        let sa = list_head_subaction(&mut (*t).response_list);
        list_remove(&mut (*sa).link);
        subaction_destroy(sa);
    }
    free(t as *mut c_void);
}

static protocol_decoders: [protocol_decoder; 1] = [protocol_decoder {
    name: b"FCP\0".as_ptr() as *const c_char,
    decode: decode_fcp,
}];

unsafe fn handle_transaction(t: *mut link_transaction) {
    if (*t).request.is_null() {
        printf(b"BUG in handle_transaction\n\0".as_ptr() as *const c_char);
        return;
    }

    let mut i = 0;
    while i < protocol_decoders.len() {
        if (protocol_decoders[i].decode)(t) != 0 {
            break;
        }
        i += 1;
    }

    /* HACK: decode only fcp right now. */
    return;

    #[allow(unreachable_code)]
    {
        decode_link_packet(
            &mut (*(*t).request).packet,
            (*(*t).request).length,
            PACKET_FIELD_TRANSACTION,
            0,
        );
        if !(*t).response.is_null() {
            decode_link_packet(
                &mut (*(*t).response).packet,
                (*(*t).request).length,
                PACKET_FIELD_TRANSACTION,
                0,
            );
        } else {
            printf(b"[no response]\0".as_ptr() as *const c_char);
        }

        if option_verbose != 0 {
            let mut pos = (*t).request_list.next;
            while pos != &mut (*t).request_list as *mut list {
                let sa = container_of_subaction_link(pos);
                print_packet(&mut (*sa).packet as *mut _ as *mut uint32_t, (*sa).length);
                pos = (*pos).next;
            }
            pos = (*t).response_list.next;
            while pos != &mut (*t).response_list as *mut list {
                let sa = container_of_subaction_link(pos);
                print_packet(&mut (*sa).packet as *mut _ as *mut uint32_t, (*sa).length);
                pos = (*pos).next;
            }
        }
        printf(b"\r\n\0".as_ptr() as *const c_char);

        link_transaction_destroy(t);
    }
}

unsafe fn clear_pending_transaction_list() {
    while !list_empty(&pending_transaction_list) {
        let t = list_head_link_transaction(&mut pending_transaction_list);
        list_remove(&mut (*t).link);
        link_transaction_destroy(t);
        /* print unfinished transactions */
    }
}

static tcode_names: [*const c_char; 12] = [
    b"write_quadlet_request\0".as_ptr() as *const c_char,
    b"write_block_request\0".as_ptr() as *const c_char,
    b"write_response\0".as_ptr() as *const c_char,
    b"reserved\0".as_ptr() as *const c_char,
    b"read_quadlet_request\0".as_ptr() as *const c_char,
    b"read_block_request\0".as_ptr() as *const c_char,
    b"read_quadlet_response\0".as_ptr() as *const c_char,
    b"read_block_response\0".as_ptr() as *const c_char,
    b"cycle_start\0".as_ptr() as *const c_char,
    b"lock_request\0".as_ptr() as *const c_char,
    b"iso_data\0".as_ptr() as *const c_char,
    b"lock_response\0".as_ptr() as *const c_char,
];

static ack_names: [*const c_char; 16] = [
    b"no ack\0".as_ptr() as *const c_char,
    b"ack_complete\0".as_ptr() as *const c_char,
    b"ack_pending\0".as_ptr() as *const c_char,
    b"reserved (0x03)\0".as_ptr() as *const c_char,
    b"ack_busy_x\0".as_ptr() as *const c_char,
    b"ack_busy_a\0".as_ptr() as *const c_char,
    b"ack_busy_b\0".as_ptr() as *const c_char,
    b"reserved (0x07)\0".as_ptr() as *const c_char,
    b"reserved (0x08)\0".as_ptr() as *const c_char,
    b"reserved (0x09)\0".as_ptr() as *const c_char,
    b"reserved (0x0a)\0".as_ptr() as *const c_char,
    b"reserved (0x0b)\0".as_ptr() as *const c_char,
    b"reserved (0x0c)\0".as_ptr() as *const c_char,
    b"ack_data_error\0".as_ptr() as *const c_char,
    b"ack_type_error\0".as_ptr() as *const c_char,
    b"reserved (0x0f)\0".as_ptr() as *const c_char,
];

static rcode_names: [*const c_char; 8] = [
    b"complete\0".as_ptr() as *const c_char,
    b"reserved (0x01)\0".as_ptr() as *const c_char,
    b"reserved (0x02)\0".as_ptr() as *const c_char,
    b"reserved (0x03)\0".as_ptr() as *const c_char,
    b"conflict_error\0".as_ptr() as *const c_char,
    b"data_error\0".as_ptr() as *const c_char,
    b"type_error\0".as_ptr() as *const c_char,
    b"address_error\0".as_ptr() as *const c_char,
];

static retry_names: [*const c_char; 4] = [
    b"retry_1\0".as_ptr() as *const c_char,
    b"retry_x\0".as_ptr() as *const c_char,
    b"retry_a\0".as_ptr() as *const c_char,
    b"retry_b\0".as_ptr() as *const c_char,
];

macro_rules! pf {
    ($name:expr, $offset:expr, $width:expr) => {
        packet_field {
            name: $name.as_ptr() as *const c_char,
            offset: $offset,
            width: $width,
            flags: 0,
            value_names: ptr::null(),
        }
    };
    ($name:expr, $offset:expr, $width:expr, $flags:expr) => {
        packet_field {
            name: $name.as_ptr() as *const c_char,
            offset: $offset,
            width: $width,
            flags: $flags,
            value_names: ptr::null(),
        }
    };
    ($name:expr, $offset:expr, $width:expr, $flags:expr, $values:expr) => {
        packet_field {
            name: $name.as_ptr() as *const c_char,
            offset: $offset,
            width: $width,
            flags: $flags,
            value_names: $values.as_ptr(),
        }
    };
}

macro_rules! common_request_fields {
    () => {
        pf!(b"dest\0", 0, 16, PACKET_FIELD_TRANSACTION),
        pf!(b"tl\0", 16, 6),
        pf!(b"rt\0", 22, 2, PACKET_FIELD_DETAIL, retry_names),
        pf!(b"tcode\0", 24, 4, PACKET_FIELD_TRANSACTION, tcode_names),
        pf!(b"pri\0", 28, 4, PACKET_FIELD_DETAIL),
        pf!(b"src\0", 32, 16, PACKET_FIELD_TRANSACTION),
        pf!(b"offs\0", 48, 48, PACKET_FIELD_TRANSACTION)
    };
}

macro_rules! common_response_fields {
    () => {
        pf!(b"dest\0", 0, 16),
        pf!(b"tl\0", 16, 6),
        pf!(b"rt\0", 22, 2, PACKET_FIELD_DETAIL, retry_names),
        pf!(b"tcode\0", 24, 4, 0, tcode_names),
        pf!(b"pri\0", 28, 4, PACKET_FIELD_DETAIL),
        pf!(b"src\0", 32, 16),
        pf!(b"rcode\0", 48, 4, PACKET_FIELD_TRANSACTION, rcode_names)
    };
}

static read_quadlet_request_fields: [packet_field; 9] = [
    common_request_fields!(),
    pf!(b"crc\0", 96, 32, PACKET_FIELD_DETAIL),
    pf!(b"ack\0", 156, 4, 0, ack_names),
];

static read_quadlet_response_fields: [packet_field; 10] = [
    common_response_fields!(),
    pf!(b"data\0", 96, 32, PACKET_FIELD_TRANSACTION),
    pf!(b"crc\0", 128, 32, PACKET_FIELD_DETAIL),
    pf!(b"ack\0", 188, 4, 0, ack_names),
];

static read_block_request_fields: [packet_field; 11] = [
    common_request_fields!(),
    pf!(b"data_length\0", 96, 16, PACKET_FIELD_TRANSACTION),
    pf!(b"extended_tcode\0", 112, 16),
    pf!(b"crc\0", 128, 32, PACKET_FIELD_DETAIL),
    pf!(b"ack\0", 188, 4, 0, ack_names),
];

static block_response_fields: [packet_field; 13] = [
    common_response_fields!(),
    pf!(b"data_length\0", 96, 16, PACKET_FIELD_DATA_LENGTH),
    pf!(b"extended_tcode\0", 112, 16),
    pf!(b"crc\0", 128, 32, PACKET_FIELD_DETAIL),
    pf!(b"data\0", 160, 0, PACKET_FIELD_TRANSACTION),
    pf!(b"crc\0", -64, 32, PACKET_FIELD_DETAIL),
    pf!(b"ack\0", -4, 4, 0, ack_names),
];

static write_quadlet_request_fields: [packet_field; 9] = [
    common_request_fields!(),
    pf!(b"data\0", 96, 32, PACKET_FIELD_TRANSACTION),
    pf!(b"ack\0", -4, 4, 0, ack_names),
];

static block_request_fields: [packet_field; 13] = [
    common_request_fields!(),
    pf!(b"data_length\0", 96, 16, PACKET_FIELD_DATA_LENGTH | PACKET_FIELD_TRANSACTION),
    pf!(b"extended_tcode\0", 112, 16, PACKET_FIELD_TRANSACTION),
    pf!(b"crc\0", 128, 32, PACKET_FIELD_DETAIL),
    pf!(b"data\0", 160, 0, PACKET_FIELD_TRANSACTION),
    pf!(b"crc\0", -64, 32, PACKET_FIELD_DETAIL),
    pf!(b"ack\0", -4, 4, 0, ack_names),
];

static write_response_fields: [packet_field; 9] = [
    common_response_fields!(),
    pf!(b"reserved\0", 64, 32, PACKET_FIELD_DETAIL),
    pf!(b"ack\0", -4, 4, 0, ack_names),
];

static iso_data_fields: [packet_field; 9] = [
    pf!(b"data_length\0", 0, 16, PACKET_FIELD_DATA_LENGTH),
    pf!(b"tag\0", 16, 2),
    pf!(b"channel\0", 18, 6),
    pf!(b"tcode\0", 24, 4, 0, tcode_names),
    pf!(b"sy\0", 28, 4),
    pf!(b"crc\0", 32, 32, PACKET_FIELD_DETAIL),
    pf!(b"data\0", 64, 0),
    pf!(b"crc\0", -64, 32, PACKET_FIELD_DETAIL),
    pf!(b"ack\0", -4, 4, 0, ack_names),
];

static mut packet_info: [packet_info; 12] = [
    packet_info { name: b"write_quadlet_request\0".as_ptr() as *const c_char, type_: PACKET_REQUEST, response_tcode: 0, fields: write_quadlet_request_fields.as_ptr(), field_count: write_quadlet_request_fields.len() as c_int },
    packet_info { name: b"write_block_request\0".as_ptr() as *const c_char, type_: PACKET_REQUEST, response_tcode: 0, fields: block_request_fields.as_ptr(), field_count: block_request_fields.len() as c_int },
    packet_info { name: b"write_response\0".as_ptr() as *const c_char, type_: PACKET_RESPONSE, response_tcode: 0, fields: write_response_fields.as_ptr(), field_count: write_response_fields.len() as c_int },
    packet_info { name: b"reserved\0".as_ptr() as *const c_char, type_: PACKET_RESERVED, response_tcode: 0, fields: ptr::null(), field_count: 0 },
    packet_info { name: b"read_quadlet_request\0".as_ptr() as *const c_char, type_: PACKET_REQUEST, response_tcode: 0, fields: read_quadlet_request_fields.as_ptr(), field_count: read_quadlet_request_fields.len() as c_int },
    packet_info { name: b"read_block_request\0".as_ptr() as *const c_char, type_: PACKET_REQUEST, response_tcode: 0, fields: read_block_request_fields.as_ptr(), field_count: read_block_request_fields.len() as c_int },
    packet_info { name: b"read_quadlet_response\0".as_ptr() as *const c_char, type_: PACKET_RESPONSE, response_tcode: 0, fields: read_quadlet_response_fields.as_ptr(), field_count: read_quadlet_response_fields.len() as c_int },
    packet_info { name: b"read_block_response\0".as_ptr() as *const c_char, type_: PACKET_RESPONSE, response_tcode: 0, fields: block_response_fields.as_ptr(), field_count: block_response_fields.len() as c_int },
    packet_info { name: b"cycle_start\0".as_ptr() as *const c_char, type_: PACKET_OTHER, response_tcode: 0, fields: write_quadlet_request_fields.as_ptr(), field_count: write_quadlet_request_fields.len() as c_int },
    packet_info { name: b"lock_request\0".as_ptr() as *const c_char, type_: PACKET_REQUEST, response_tcode: 0, fields: block_request_fields.as_ptr(), field_count: block_request_fields.len() as c_int },
    packet_info { name: b"iso_data\0".as_ptr() as *const c_char, type_: PACKET_OTHER, response_tcode: 0, fields: iso_data_fields.as_ptr(), field_count: iso_data_fields.len() as c_int },
    packet_info { name: b"lock_response\0".as_ptr() as *const c_char, type_: PACKET_RESPONSE, response_tcode: 0, fields: block_response_fields.as_ptr(), field_count: block_response_fields.len() as c_int },
];

unsafe fn init_packet_info_response_tcodes() {
    packet_info[0].response_tcode = TCODE_WRITE_RESPONSE;
    packet_info[1].response_tcode = TCODE_WRITE_RESPONSE;
    packet_info[4].response_tcode = TCODE_READ_QUADLET_RESPONSE;
    packet_info[5].response_tcode = TCODE_READ_BLOCK_RESPONSE;
}

unsafe fn handle_request_packet(data: *mut uint32_t, length: size_t) -> c_int {
    let p = data as *mut link_packet;
    let t = link_transaction_lookup((*p).common.source, (*p).common.destination, (*p).common.tlabel);
    let sa = subaction_create(data, length);
    (*t).request = sa;

    if !list_empty(&(*t).request_list) {
        let prev = list_tail_subaction(&mut (*t).request_list);
        if !ACK_BUSY((*prev).ack) {
            /*
             * error, we should only see ack_busy_* before the
             * ack_pending/ack_complete -- this is an ack_pending
             * instead (ack_complete would have finished the
             * transaction).
             */
        }

        if (*prev).packet.common.tcode != (*sa).packet.common.tcode
            || (*prev).packet.common.tlabel != (*sa).packet.common.tlabel
        {
            /* memcmp() ? */
            /* error, these should match for retries. */
        }
    }

    list_append(&mut (*t).request_list, &mut (*sa).link);

    if (*sa).ack == ACK_COMPLETE {
        if (*p).common.tcode != TCODE_WRITE_QUADLET_REQUEST as usize
            && (*p).common.tcode != TCODE_WRITE_BLOCK_REQUEST as usize
        {
            /* error, unified transactions only allowed for write */
        }
        list_remove(&mut (*t).link);
        handle_transaction(t);
    } else if (*sa).ack == ACK_NO_ACK || (*sa).ack == ACK_DATA_ERROR || (*sa).ack == ACK_TYPE_ERROR {
        list_remove(&mut (*t).link);
        handle_transaction(t);
    } else if (*sa).ack == ACK_PENDING {
        /* request subaction phase over, wait for response. */
    } else if (*sa).ack == ACK_BUSY_X || (*sa).ack == ACK_BUSY_A || (*sa).ack == ACK_BUSY_B {
        /* ok, wait for retry. */
        /* check that retry protocol is respected. */
    }

    1
}

unsafe fn handle_response_packet(data: *mut uint32_t, length: size_t) -> c_int {
    let p = data as *mut link_packet;
    let t = link_transaction_lookup((*p).common.destination, (*p).common.source, (*p).common.tlabel);
    if list_empty(&(*t).request_list) {
        /* unsolicited response */
    }

    let sa = subaction_create(data, length);
    (*t).response = sa;

    if !list_empty(&(*t).response_list) {
        let prev = list_tail_subaction(&mut (*t).response_list);
        if !ACK_BUSY((*prev).ack) {
            /*
             * error, we should only see ack_busy_* before the
             * ack_pending/ack_complete
             */
        }

        if (*prev).packet.common.tcode != (*sa).packet.common.tcode
            || (*prev).packet.common.tlabel != (*sa).packet.common.tlabel
        {
            /* use memcmp() instead? */
            /* error, these should match for retries. */
        }
    } else {
        let prev = list_tail_subaction(&mut (*t).request_list);
        if (*prev).ack != ACK_PENDING {
            /*
             * error, should not get response unless last request got
             * ack_pending.
             */
        }

        if packet_info[(*prev).packet.common.tcode].response_tcode != (*sa).packet.common.tcode as c_int {
            /* error, tcode mismatch */
        }
    }

    list_append(&mut (*t).response_list, &mut (*sa).link);

    if (*sa).ack == ACK_COMPLETE || (*sa).ack == ACK_NO_ACK || (*sa).ack == ACK_DATA_ERROR || (*sa).ack == ACK_TYPE_ERROR {
        list_remove(&mut (*t).link);
        handle_transaction(t);
        /* transaction complete, remove t from pending list. */
    } else if (*sa).ack == ACK_PENDING {
        /* error for responses. */
    } else if (*sa).ack == ACK_BUSY_X || (*sa).ack == ACK_BUSY_A || (*sa).ack == ACK_BUSY_B {
        /* no problem, wait for next retry */
    }

    1
}

unsafe fn handle_packet(data: *mut uint32_t, length: size_t) -> c_int {
    if length == 0 {
        printf(b"bus reset\r\n\0".as_ptr() as *const c_char);
        clear_pending_transaction_list();
    } else if length > mem::size_of::<phy_packet>() {
        let p = data as *mut link_packet;
        match packet_info[(*p).common.tcode].type_ {
            PACKET_REQUEST => return handle_request_packet(data, length),
            PACKET_RESPONSE => return handle_response_packet(data, length),
            PACKET_OTHER | PACKET_RESERVED => return 0,
            _ => {}
        }
    }
    1
}

unsafe fn get_bits(packet: *mut link_packet, offset: c_int, width: c_int) -> c_uint {
    let data = packet as *mut uint32_t;
    let index = offset / 32 + 1;
    let shift = 32 - (offset & 31) - width;
    let mask: uint32_t = if width == 32 { !0 } else { (1u32 << width) - 1 };
    (*data.add(index as usize) >> shift) & mask
}

#[cfg(target_endian = "little")]
fn byte_index(i: c_int) -> c_int {
    i ^ 3
}

#[cfg(target_endian = "big")]
fn byte_index(i: c_int) -> c_int {
    i
}

unsafe fn dump_data(data: *mut u8, length: c_int) {
    let print_length = if length > 128 { 128 } else { length };

    let mut i = 0;
    while i < print_length {
        printf(
            b"%s%02hhx\0".as_ptr() as *const c_char,
            if i % 4 == 0 && i != 0 {
                b" \0".as_ptr() as *const c_char
            } else {
                b"\0".as_ptr() as *const c_char
            },
            *data.add(byte_index(i) as usize) as c_uint,
        );
        i += 1;
    }

    if print_length < length {
        printf(b" (%d more bytes)\0".as_ptr() as *const c_char, length - print_length);
    }
}

unsafe fn decode_link_packet(
    packet: *mut link_packet,
    length: size_t,
    include_flags: c_int,
    exclude_flags: c_int,
) {
    let pi = &packet_info[(*packet).common.tcode] as *const packet_info;
    let mut data_length: c_int = 0;
    let mut i = 0;

    while i < (*pi).field_count {
        let f = (*pi).fields.add(i as usize);
        let offset: c_int;

        if ((*f).flags & exclude_flags) != 0 {
            i += 1;
            continue;
        }
        if include_flags != 0 && ((*f).flags & include_flags) == 0 {
            i += 1;
            continue;
        }

        if (*f).offset < 0 {
            offset = (length * 8) as c_int + (*f).offset - 32;
        } else {
            offset = (*f).offset;
        }

        if !(*f).value_names.is_null() {
            let bits = get_bits(packet, offset, (*f).width);
            printf(b"%s\0".as_ptr() as *const c_char, *(*f).value_names.add(bits as usize));
        } else if (*f).width == 0 {
            printf(b"%s=[\0".as_ptr() as *const c_char, (*f).name);
            dump_data((packet as *mut u8).add((offset / 8 + 4) as usize), data_length);
            printf(b"]\0".as_ptr() as *const c_char);
        } else {
            let bits: u64;
            let high_width: c_int;
            let low_width: c_int;

            if (offset & !31) != ((offset + (*f).width - 1) & !31) {
                /* Bit field spans quadlet boundary. */
                high_width = ((offset + 31) & !31) - offset;
                low_width = (*f).width - high_width;

                bits = ((get_bits(packet, offset, high_width) as u64) << low_width)
                    | get_bits(packet, offset + high_width, low_width) as u64;
            } else {
                bits = get_bits(packet, offset, (*f).width) as u64;
            }

            printf(
                b"%s=0x%0*llx\0".as_ptr() as *const c_char,
                (*f).name,
                ((*f).width + 3) / 4,
                bits,
            );

            if ((*f).flags & PACKET_FIELD_DATA_LENGTH) != 0 {
                data_length = bits as c_int;
            }
        }

        if i < (*pi).field_count - 1 {
            printf(b", \0".as_ptr() as *const c_char);
        }
        i += 1;
    }
}

unsafe fn print_packet(data: *mut uint32_t, length: size_t) {
    printf(b"%6u  \0".as_ptr() as *const c_char, *data);

    if length == 4 {
        printf(b"bus reset\0".as_ptr() as *const c_char);
    } else if length < mem::size_of::<phy_packet>() {
        printf(b"short packet: \0".as_ptr() as *const c_char);
        let mut i = 1;
        while i < length / 4 {
            printf(
                b"%s%08x\0".as_ptr() as *const c_char,
                if i == 0 { b"[\0".as_ptr() as *const c_char } else { b" \0".as_ptr() as *const c_char },
                *data.add(i),
            );
            i += 1;
        }
        printf(b"]\0".as_ptr() as *const c_char);
    } else if length == mem::size_of::<phy_packet>() && *data.add(1) == !*data.add(2) {
        let pp = data as *mut phy_packet;

        /* phy packet are 3 quadlets: the 1 quadlet payload,
         * the bitwise inverse of the payload and the snoop
         * mode ack */
        if (*pp).common.identifier == PHY_PACKET_CONFIGURATION {
            if !(*pp).phy_config.set_root && !(*pp).phy_config.set_gap_count {
                printf(b"ext phy config: phy_id=%02x\0".as_ptr() as *const c_char, (*pp).phy_config.root_id);
            } else {
                printf(b"phy config:\0".as_ptr() as *const c_char);
                if (*pp).phy_config.set_root {
                    printf(b" set_root_id=%02x\0".as_ptr() as *const c_char, (*pp).phy_config.root_id);
                }
                if (*pp).phy_config.set_gap_count {
                    printf(b" set_gap_count=%u\0".as_ptr() as *const c_char, (*pp).phy_config.gap_count);
                }
            }
        } else if (*pp).common.identifier == PHY_PACKET_LINK_ON {
            printf(b"link-on packet, phy_id=%02x\0".as_ptr() as *const c_char, (*pp).link_on.phy_id);
        } else if (*pp).common.identifier == PHY_PACKET_SELF_ID {
            if (*pp).self_id.extended {
                printf(
                    b"extended self id: phy_id=%02x, seq=%u\0".as_ptr() as *const c_char,
                    (*pp).ext_self_id.phy_id,
                    (*pp).ext_self_id.sequence,
                );
            } else {
                static speed_names: [*const c_char; 4] = [
                    b"S100\0".as_ptr() as *const c_char,
                    b"S200\0".as_ptr() as *const c_char,
                    b"S400\0".as_ptr() as *const c_char,
                    b"BETA\0".as_ptr() as *const c_char,
                ];
                printf(
                    b"self id: phy_id=%02x, link %s, gap_count=%u speed=%s%s%s\0".as_ptr()
                        as *const c_char,
                    (*pp).self_id.phy_id,
                    if (*pp).self_id.link_active {
                        b"active\0".as_ptr() as *const c_char
                    } else {
                        b"not active\0".as_ptr() as *const c_char
                    },
                    (*pp).self_id.gap_count,
                    speed_names[(*pp).self_id.phy_speed],
                    if (*pp).self_id.contender {
                        b", irm contender\0".as_ptr() as *const c_char
                    } else {
                        b"\0".as_ptr() as *const c_char
                    },
                    if (*pp).self_id.initiated_reset {
                        b", initiator\0".as_ptr() as *const c_char
                    } else {
                        b"\0".as_ptr() as *const c_char
                    },
                );
            }
        } else {
            printf(b"unknown phy packet: \0".as_ptr() as *const c_char);
            let mut i = 1;
            while i < length / 4 {
                printf(
                    b"%s%08x\0".as_ptr() as *const c_char,
                    if i == 0 { b"[\0".as_ptr() as *const c_char } else { b" \0".as_ptr() as *const c_char },
                    *data.add(i),
                );
                i += 1;
            }
            printf(b"]\0".as_ptr() as *const c_char);
        }
    } else {
        let packet = data as *mut link_packet;
        decode_link_packet(
            packet,
            length,
            0,
            if option_verbose != 0 { 0 } else { PACKET_FIELD_DETAIL },
        );
    }

    if option_hex != 0 {
        printf(b"  [\0".as_ptr() as *const c_char);
        dump_data((data as *mut u8).add(4), (length - 4) as c_int);
        printf(b"]\0".as_ptr() as *const c_char);
    }

    printf(b"\r\n\0".as_ptr() as *const c_char);
}

static mut bus_reset_count: c_int = 0;
static mut short_packet_count: c_int = 0;
static mut phy_packet_count: c_int = 0;
static mut tcode_count: [c_int; 16] = [0; 16];
static mut last_update: timeval = timeval { tv_sec: 0, tv_usec: 0 };

unsafe fn print_stats(data: *mut uint32_t, length: size_t) {
    if length == 0 {
        bus_reset_count += 1;
    } else if length < mem::size_of::<phy_packet>() {
        short_packet_count += 1;
    } else if length == mem::size_of::<phy_packet>() && *data.add(1) == !*data.add(2) {
        phy_packet_count += 1;
    } else {
        let packet = data as *mut link_packet;
        tcode_count[(*packet).common.tcode] += 1;
    }

    let mut now = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut now, ptr::null_mut());
    if now.tv_sec <= last_update.tv_sec && now.tv_usec < last_update.tv_usec + 500000 {
        return;
    }

    last_update = now;
    printf(
        b"\x1b[H\x1b[2J\x1b[?25l  bus resets              : %8d\n  short packets           : %8d\n  phy packets             : %8d\n\0"
            .as_ptr() as *const c_char,
        bus_reset_count,
        short_packet_count,
        phy_packet_count,
    );

    let mut i = 0;
    while i < packet_info.len() {
        if packet_info[i].type_ != PACKET_RESERVED {
            printf(
                b"  %-24s: %8d\n\0".as_ptr() as *const c_char,
                packet_info[i].name,
                tcode_count[i],
            );
        }
        i += 1;
    }
    printf(b"\x1b[?25h\n\0".as_ptr() as *const c_char);
}

static mut saved_attributes: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};

unsafe extern "C" fn reset_input_mode() {
    tcsetattr(STDIN_FILENO, TCSANOW, &saved_attributes);
}

unsafe fn set_input_mode() {
    let mut tattr: termios = mem::zeroed();

    /* Make sure stdin is a terminal. */
    if isatty(STDIN_FILENO) == 0 {
        fprintf(stderr, b"Not a terminal.\n\0".as_ptr() as *const c_char);
        exit(EXIT_FAILURE);
    }

    /* Save the terminal attributes so we can restore them later. */
    tcgetattr(STDIN_FILENO, &mut saved_attributes);
    atexit(reset_input_mode);

    /* Set the funny terminal modes. */
    tcgetattr(STDIN_FILENO, &mut tattr);
    tattr.c_lflag &= !(ICANON | ECHO); /* Clear ICANON and ECHO. */
    tattr.c_cc[VMIN] = 1;
    tattr.c_cc[VTIME] = 0;
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &tattr);
}

unsafe fn main_impl(argc: c_int, argv: *const *const c_char) -> c_int {
    init_packet_info_response_tcodes();

    let mut buf: [uint32_t; 128 * 1024] = [0; 128 * 1024];
    let mut filter: uint32_t;
    let mut length: c_int = 0;
    let mut retval: c_int;
    let view: c_int;
    let mut fd: c_int = -1;
    let mut output: *mut FILE = ptr::null_mut();
    let mut input: *mut FILE = ptr::null_mut();
    let con: poptContext;
    let mut c: c_char = 0;
    let mut pollfds = [
        pollfd { fd: 0, events: 0, revents: 0 },
        pollfd { fd: 0, events: 0, revents: 0 },
    ];

    sys_sigint_handler = signal(SIGINT, Some(sigint_handler));

    con = poptGetContext(ptr::null(), argc, argv, options.as_ptr(), 0);
    retval = poptGetNextOpt(con);
    if retval < -1 {
        poptPrintUsage(con, stdout, 0);
        return -1;
    }

    if option_version != 0 {
        printf(
            b"dump tool for nosy sniffer, version %s\n\0".as_ptr() as *const c_char,
            VERSION,
        );
        return 0;
    }

    if cfg!(not(target_endian = "little")) {
        fprintf(
            stderr,
            b"warning: nosy has only been tested on little endian machines\n\0".as_ptr()
                as *const c_char,
        );
    }

    if !option_input.is_null() {
        input = fopen(option_input, b"r\0".as_ptr() as *const c_char);
        if input.is_null() {
            fprintf(stderr, b"Could not open %s, %m\n\0".as_ptr() as *const c_char, option_input);
            return -1;
        }
    } else {
        fd = open(option_nosy_device, O_RDWR);
        if fd < 0 {
            fprintf(stderr, b"Could not open %s, %m\n\0".as_ptr() as *const c_char, option_nosy_device);
            return -1;
        }
        set_input_mode();
    }

    if strcmp(option_view, b"transaction\0".as_ptr() as *const c_char) == 0 {
        view = VIEW_TRANSACTION;
    } else if strcmp(option_view, b"stats\0".as_ptr() as *const c_char) == 0 {
        view = VIEW_STATS;
    } else {
        view = VIEW_PACKET;
    }

    if !option_output.is_null() {
        output = fopen(option_output, b"w\0".as_ptr() as *const c_char);
        if output.is_null() {
            fprintf(stderr, b"Could not open %s, %m\n\0".as_ptr() as *const c_char, option_output);
            return -1;
        }
    }

    setvbuf(stdout, ptr::null_mut(), _IOLBF, BUFSIZ);

    filter = !0;
    if option_iso == 0 {
        filter &= !(1u32 << TCODE_STREAM_DATA);
    }
    if option_cycle_start == 0 {
        filter &= !(1u32 << TCODE_CYCLE_START);
    }
    if view == VIEW_STATS {
        filter = !(1u32 << TCODE_CYCLE_START);
    }

    ioctl(fd, NOSY_IOC_FILTER, filter);
    ioctl(fd, NOSY_IOC_START);

    pollfds[0].fd = fd;
    pollfds[0].events = POLLIN;
    pollfds[1].fd = STDIN_FILENO;
    pollfds[1].events = POLLIN;

    while run != 0 {
        if !input.is_null() {
            if fread(
                &mut length as *mut _ as *mut c_void,
                mem::size_of_val(&length),
                1,
                input,
            ) != 1
            {
                return 0;
            }
            fread(buf.as_mut_ptr() as *mut c_void, 1, length as size_t, input);
        } else {
            poll(pollfds.as_mut_ptr(), 2, -1);
            if pollfds[1].revents != 0 {
                read(STDIN_FILENO, &mut c as *mut _ as *mut c_void, mem::size_of_val(&c));
                match c as u8 as char {
                    'q' => {
                        if !output.is_null() {
                            fclose(output);
                        }
                        return 0;
                    }
                    _ => {}
                }
            }

            if pollfds[0].revents != 0 {
                length = read(fd, buf.as_mut_ptr() as *mut c_void, mem::size_of_val(&buf)) as c_int;
            } else {
                continue;
            }
        }

        if !output.is_null() {
            fwrite(
                &length as *const _ as *const c_void,
                mem::size_of_val(&length),
                1,
                output,
            );
            fwrite(buf.as_ptr() as *const c_void, 1, length as size_t, output);
        }

        match view {
            VIEW_TRANSACTION => {
                handle_packet(buf.as_mut_ptr(), length as size_t);
            }
            VIEW_PACKET => {
                print_packet(buf.as_mut_ptr(), length as size_t);
            }
            VIEW_STATS => {
                print_stats(buf.as_mut_ptr(), length as size_t);
            }
            _ => {}
        }
    }

    if !output.is_null() {
        fclose(output);
    }

    close(fd);
    poptFreeContext(con);

    0
}

fn main() {
    unsafe {
        let args: Vec<CString> = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect();
        let mut argv: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();
        argv.push(ptr::null());
        std::process::exit(main_impl(args.len() as c_int, argv.as_ptr()));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
