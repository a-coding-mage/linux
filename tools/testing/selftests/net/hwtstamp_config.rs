// SPDX-License-Identifier: GPL-2.0
/* Test program for SIOC{G,S}HWTSTAMP
 * Copyright 2013 Solarflare Communications
 * Author: Ben Hutchings
 */

use std::env;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_ulong, c_void};
use std::ptr;

const IFNAMSIZ: usize = 16;
const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;
const SIOCSHWTSTAMP: c_ulong = 0x89b0;
const SIOCGHWTSTAMP: c_ulong = 0x89b1;

const HWTSTAMP_TX_OFF: usize = 0;
const HWTSTAMP_TX_ON: usize = 1;
const HWTSTAMP_TX_ONESTEP_SYNC: usize = 2;

const HWTSTAMP_FILTER_NONE: usize = 0;
const HWTSTAMP_FILTER_ALL: usize = 1;
const HWTSTAMP_FILTER_SOME: usize = 2;
const HWTSTAMP_FILTER_PTP_V1_L4_EVENT: usize = 3;
const HWTSTAMP_FILTER_PTP_V1_L4_SYNC: usize = 4;
const HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ: usize = 5;
const HWTSTAMP_FILTER_PTP_V2_L4_EVENT: usize = 6;
const HWTSTAMP_FILTER_PTP_V2_L4_SYNC: usize = 7;
const HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ: usize = 8;
const HWTSTAMP_FILTER_PTP_V2_L2_EVENT: usize = 9;
const HWTSTAMP_FILTER_PTP_V2_L2_SYNC: usize = 10;
const HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ: usize = 11;
const HWTSTAMP_FILTER_PTP_V2_EVENT: usize = 12;
const HWTSTAMP_FILTER_PTP_V2_SYNC: usize = 13;
const HWTSTAMP_FILTER_PTP_V2_DELAY_REQ: usize = 14;

#[repr(C)]
struct hwtstamp_config {
    flags: c_int,
    tx_type: c_int,
    rx_filter: c_int,
}

#[repr(C)]
union ifr_ifru {
    ifru_addr: [u8; 24],
    ifru_data: *mut c_void,
}

#[repr(C)]
struct ifreq {
    ifr_name: [c_char; IFNAMSIZ],
    ifr_ifru: ifr_ifru,
}

unsafe extern "C" {
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

fn lookup_value(names: &[*const c_char], size: c_int, name: *const c_char) -> c_int {
    let mut value: c_int;

    value = 0;
    while value < size {
        unsafe {
            if !names[value as usize].is_null()
                && strcasecmp(names[value as usize], name) == 0
            {
                return value;
            }
        }
        value += 1;
    }

    -1
}

fn lookup_name(names: &[*const c_char], size: c_int, value: c_int) -> *const c_char {
    if value >= 0 && value < size {
        names[value as usize]
    } else {
        ptr::null()
    }
}

fn list_names(f: &mut dyn std::io::Write, names: &[*const c_char], size: c_int) {
    let mut value: c_int;

    value = 0;
    while value < size {
        if !names[value as usize].is_null() {
            unsafe {
                let _ = writeln!(
                    f,
                    "    {}",
                    CStr::from_ptr(names[value as usize]).to_string_lossy()
                );
            }
        }
        value += 1;
    }
}

static TX_OFF: &[u8] = b"OFF\0";
static TX_ON: &[u8] = b"ON\0";
static TX_ONESTEP_SYNC: &[u8] = b"ONESTEP_SYNC\0";

fn tx_types() -> [*const c_char; N_TX_TYPES as usize] {
    let mut names = [ptr::null(); N_TX_TYPES as usize];
    names[HWTSTAMP_TX_OFF] = TX_OFF.as_ptr() as *const c_char;
    names[HWTSTAMP_TX_ON] = TX_ON.as_ptr() as *const c_char;
    names[HWTSTAMP_TX_ONESTEP_SYNC] = TX_ONESTEP_SYNC.as_ptr() as *const c_char;
    names
}
const N_TX_TYPES: c_int = 3;

static RX_NONE: &[u8] = b"NONE\0";
static RX_ALL: &[u8] = b"ALL\0";
static RX_SOME: &[u8] = b"SOME\0";
static RX_PTP_V1_L4_EVENT: &[u8] = b"PTP_V1_L4_EVENT\0";
static RX_PTP_V1_L4_SYNC: &[u8] = b"PTP_V1_L4_SYNC\0";
static RX_PTP_V1_L4_DELAY_REQ: &[u8] = b"PTP_V1_L4_DELAY_REQ\0";
static RX_PTP_V2_L4_EVENT: &[u8] = b"PTP_V2_L4_EVENT\0";
static RX_PTP_V2_L4_SYNC: &[u8] = b"PTP_V2_L4_SYNC\0";
static RX_PTP_V2_L4_DELAY_REQ: &[u8] = b"PTP_V2_L4_DELAY_REQ\0";
static RX_PTP_V2_L2_EVENT: &[u8] = b"PTP_V2_L2_EVENT\0";
static RX_PTP_V2_L2_SYNC: &[u8] = b"PTP_V2_L2_SYNC\0";
static RX_PTP_V2_L2_DELAY_REQ: &[u8] = b"PTP_V2_L2_DELAY_REQ\0";
static RX_PTP_V2_EVENT: &[u8] = b"PTP_V2_EVENT\0";
static RX_PTP_V2_SYNC: &[u8] = b"PTP_V2_SYNC\0";
static RX_PTP_V2_DELAY_REQ: &[u8] = b"PTP_V2_DELAY_REQ\0";

fn rx_filters() -> [*const c_char; N_RX_FILTERS as usize] {
    let mut names = [ptr::null(); N_RX_FILTERS as usize];
    names[HWTSTAMP_FILTER_NONE] = RX_NONE.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_ALL] = RX_ALL.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_SOME] = RX_SOME.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_PTP_V1_L4_EVENT] = RX_PTP_V1_L4_EVENT.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_PTP_V1_L4_SYNC] = RX_PTP_V1_L4_SYNC.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ] =
        RX_PTP_V1_L4_DELAY_REQ.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_PTP_V2_L4_EVENT] = RX_PTP_V2_L4_EVENT.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_PTP_V2_L4_SYNC] = RX_PTP_V2_L4_SYNC.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ] =
        RX_PTP_V2_L4_DELAY_REQ.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_PTP_V2_L2_EVENT] = RX_PTP_V2_L2_EVENT.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_PTP_V2_L2_SYNC] = RX_PTP_V2_L2_SYNC.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ] =
        RX_PTP_V2_L2_DELAY_REQ.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_PTP_V2_EVENT] = RX_PTP_V2_EVENT.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_PTP_V2_SYNC] = RX_PTP_V2_SYNC.as_ptr() as *const c_char;
    names[HWTSTAMP_FILTER_PTP_V2_DELAY_REQ] = RX_PTP_V2_DELAY_REQ.as_ptr() as *const c_char;
    names
}
const N_RX_FILTERS: c_int = 15;

fn usage() {
    let mut stderr = std::io::stderr();
    let _ = write!(
        stderr,
        "Usage: hwtstamp_config if_name [tx_type rx_filter]\n\
         tx_type is any of (case-insensitive):\n"
    );
    let tx_types = tx_types();
    list_names(&mut stderr, &tx_types, N_TX_TYPES);
    let _ = write!(stderr, "rx_filter is any of (case-insensitive):\n");
    let rx_filters = rx_filters();
    list_names(&mut stderr, &rx_filters, N_RX_FILTERS);
}

fn main() {
    let args: Vec<CString> = env::args()
        .map(|arg| CString::new(arg).unwrap_or_else(|_| CString::new("").unwrap()))
        .collect();
    let argc = args.len() as c_int;
    let argv: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();
    let mut ifr: ifreq = unsafe { mem::zeroed() };
    let mut config: hwtstamp_config = unsafe { mem::zeroed() };
    let mut name: *const c_char;
    let sock: c_int;
    let tx_types = tx_types();
    let rx_filters = rx_filters();

    if (argc != 2 && argc != 4)
        || unsafe { CStr::from_ptr(argv[1]).to_bytes().len() >= IFNAMSIZ }
    {
        usage();
        std::process::exit(2);
    }

    if argc == 4 {
        config.flags = 0;
        config.tx_type = lookup_value(&tx_types, N_TX_TYPES, argv[2]);
        config.rx_filter = lookup_value(&rx_filters, N_RX_FILTERS, argv[3]);
        if config.tx_type < 0 || config.rx_filter < 0 {
            usage();
            std::process::exit(2);
        }
    }

    sock = unsafe { socket(AF_INET, SOCK_DGRAM, 0) };
    if sock < 0 {
        unsafe {
            perror(c"socket".as_ptr());
        }
        std::process::exit(1);
    }

    unsafe {
        ptr::copy_nonoverlapping(
            argv[1],
            ifr.ifr_name.as_mut_ptr(),
            CStr::from_ptr(argv[1]).to_bytes_with_nul().len(),
        );
    }
    ifr.ifr_ifru.ifru_data = &mut config as *mut hwtstamp_config as *mut c_void;

    if unsafe {
        ioctl(
            sock,
            if argc == 2 {
                SIOCGHWTSTAMP
            } else {
                SIOCSHWTSTAMP
            },
            &mut ifr as *mut ifreq,
        )
    } != 0
    {
        unsafe {
            perror(c"ioctl".as_ptr());
        }
        std::process::exit(1);
    }

    println!("flags = {:#x}", config.flags);
    name = lookup_name(&tx_types, N_TX_TYPES, config.tx_type);
    if !name.is_null() {
        unsafe {
            println!("tx_type = {}", CStr::from_ptr(name).to_string_lossy());
        }
    } else {
        println!("tx_type = {}", config.tx_type);
    }
    name = lookup_name(&rx_filters, N_RX_FILTERS, config.rx_filter);
    if !name.is_null() {
        unsafe {
            println!("rx_filter = {}", CStr::from_ptr(name).to_string_lossy());
        }
    } else {
        println!("rx_filter = {}", config.rx_filter);
    }

    std::process::exit(0);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
