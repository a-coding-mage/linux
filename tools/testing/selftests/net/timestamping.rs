// SPDX-License-Identifier: GPL-2.0-only
/*
 * This program demonstrates how the various time stamping features in
 * the Linux kernel work. It emulates the behavior of a PTP
 * implementation in stand-alone master mode by sending PTPv1 Sync
 * multicasts once every second. It looks for similar packets, but
 * beyond that doesn't actually implement PTP.
 *
 * Outgoing packets are time stamped with SO_TIMESTAMPING with or
 * without hardware support.
 *
 * Incoming packets are time stamped with SO_TIMESTAMPING with or
 * without hardware support, SIOCGSTAMP[NS] (per-socket time stamp) and
 * SO_TIMESTAMP[NS].
 *
 * Copyright (C) 2009 Intel Corporation.
 * Author: Patrick Ohly <patrick.ohly@intel.com>
 */

use libc::*;
use std::env;
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

const SO_TIMESTAMPING: c_int = 37;
const SCM_TIMESTAMPING: c_int = SO_TIMESTAMPING;
const SO_TIMESTAMPNS: c_int = 35;

const SOF_TIMESTAMPING_TX_HARDWARE: c_int = 1 << 0;
const SOF_TIMESTAMPING_TX_SOFTWARE: c_int = 1 << 1;
const SOF_TIMESTAMPING_RX_HARDWARE: c_int = 1 << 2;
const SOF_TIMESTAMPING_RX_SOFTWARE: c_int = 1 << 3;
const SOF_TIMESTAMPING_SOFTWARE: c_int = 1 << 4;
const SOF_TIMESTAMPING_RAW_HARDWARE: c_int = 1 << 6;
const SOF_TIMESTAMPING_BIND_PHC: c_int = 1 << 15;

const HWTSTAMP_TX_OFF: c_int = 0;
const HWTSTAMP_TX_ON: c_int = 1;
const HWTSTAMP_FILTER_NONE: c_int = 0;
const HWTSTAMP_FILTER_PTP_V1_L4_SYNC: c_int = 6;
const HWTSTAMP_FILTER_PTP_V2_L4_SYNC: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone)]
struct hwtstamp_config {
    flags: c_int,
    tx_type: c_int,
    rx_filter: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct so_timestamping {
    flags: c_int,
    bind_phc: c_int,
}

#[repr(C)]
struct sock_extended_err {
    ee_errno: u32,
    ee_origin: u8,
    ee_type: u8,
    ee_code: u8,
    ee_pad: u8,
    ee_info: u32,
    ee_data: u32,
}

fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn strerror_string(err: c_int) -> String {
    CStr::from_ptr(strerror(err)).to_string_lossy().into_owned()
}

fn usage(error: Option<&str>) -> ! {
    if let Some(error) = error {
        println!("invalid option: {}", error);
    }
    print!(
        "timestamping <interface> [bind_phc_index] [option]*\n\n\
Options:\n\
  IP_MULTICAST_LOOP - looping outgoing multicasts\n\
  SO_TIMESTAMP - normal software time stamping, ms resolution\n\
  SO_TIMESTAMPNS - more accurate software time stamping\n\
  SOF_TIMESTAMPING_TX_HARDWARE - hardware time stamping of outgoing packets\n\
  SOF_TIMESTAMPING_TX_SOFTWARE - software fallback for outgoing packets\n\
  SOF_TIMESTAMPING_RX_HARDWARE - hardware time stamping of incoming packets\n\
  SOF_TIMESTAMPING_RX_SOFTWARE - software fallback for incoming packets\n\
  SOF_TIMESTAMPING_SOFTWARE - request reporting of software time stamps\n\
  SOF_TIMESTAMPING_RAW_HARDWARE - request reporting of raw HW time stamps\n\
  SOF_TIMESTAMPING_BIND_PHC - request to bind a PHC of PTP vclock\n\
  SIOCGSTAMP - check last socket time stamp\n\
  SIOCGSTAMPNS - more accurate socket time stamp\n\
  PTPV2 - use PTPv2 messages\n"
    );
    std::process::exit(1);
}

fn bail(error: &str) -> ! {
    unsafe {
        println!("{}: {}", error, strerror_string(errno()));
    }
    std::process::exit(1);
}

static SYNC: [c_uchar; 124] = [
    0x00, 0x01, 0x00, 0x01, 0x5f, 0x44, 0x46, 0x4c, 0x54, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01,
    /* fake uuid */
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x01, 0x00, 0x37, 0x00, 0x00, 0x00, 0x08,
    0x00, 0x00, 0x00, 0x00, 0x49, 0x05, 0xcd, 0x01, 0x29, 0xb1, 0x8d, 0xb0, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x01,
    /* fake uuid */
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x00, 0x00, 0x37, 0x00, 0x00, 0x00, 0x04,
    0x44, 0x46, 0x4c, 0x54, 0x00, 0x00, 0xf0, 0x60, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x01, 0x00, 0x00, 0xf0, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
    0x44, 0x46, 0x4c, 0x54, 0x00, 0x01,
    /* fake uuid */
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

static SYNC_V2: [c_uchar; 44] = [
    0x00, 0x02, 0x00, 0x2C, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFE, 0x00, 0x00, 0x00,
    0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00,
];

unsafe fn sendpacket(sock: c_int, addr: *mut sockaddr, addr_len: socklen_t, ptpv2: c_int) {
    let sync_len = if ptpv2 != 0 { SYNC_V2.len() } else { SYNC.len() };
    let sync_p = if ptpv2 != 0 { SYNC_V2.as_ptr() } else { SYNC.as_ptr() };
    let mut now: timeval = mem::zeroed();
    let res = sendto(sock, sync_p as *const c_void, sync_len, 0, addr, addr_len);
    gettimeofday(&mut now, ptr::null_mut());
    if res < 0 {
        println!("{}: {}", "send", strerror_string(errno()));
    } else {
        println!("{}.{:06}: sent {} bytes", now.tv_sec as c_long, now.tv_usec as c_long, res);
    }
}

unsafe fn cmsg_align(len: usize) -> usize {
    (len + mem::size_of::<usize>() - 1) & !(mem::size_of::<usize>() - 1)
}

unsafe fn cmsg_firsthdr(msg: *mut msghdr) -> *mut cmsghdr {
    if (*msg).msg_controllen < mem::size_of::<cmsghdr>() {
        ptr::null_mut()
    } else {
        (*msg).msg_control as *mut cmsghdr
    }
}

unsafe fn cmsg_nxthdr(msg: *mut msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr {
    let next = (cmsg as *mut u8).add(cmsg_align((*cmsg).cmsg_len as usize)) as *mut cmsghdr;
    let max = ((*msg).msg_control as *mut u8).add((*msg).msg_controllen as usize);
    if (next as *mut u8).add(mem::size_of::<cmsghdr>()) > max {
        ptr::null_mut()
    } else {
        next
    }
}

unsafe fn cmsg_data(cmsg: *mut cmsghdr) -> *mut c_uchar {
    (cmsg as *mut u8).add(cmsg_align(mem::size_of::<cmsghdr>())) as *mut c_uchar
}

unsafe fn printpacket(
    msg: *mut msghdr,
    res: c_int,
    data: *mut c_char,
    sock: c_int,
    recvmsg_flags: c_int,
    siocgstamp: c_int,
    siocgstampns: c_int,
    ptpv2: c_int,
) {
    let from_addr = (*msg).msg_name as *mut sockaddr_in;
    let sync_len = if ptpv2 != 0 { SYNC_V2.len() } else { SYNC.len() };
    let sync_p = if ptpv2 != 0 { SYNC_V2.as_ptr() } else { SYNC.as_ptr() };
    let mut tv: timeval = mem::zeroed();
    let mut ts: timespec = mem::zeroed();
    let mut now: timeval = mem::zeroed();

    gettimeofday(&mut now, ptr::null_mut());

    println!(
        "{}.{:06}: received {} data, {} bytes from {}, {} bytes control messages",
        now.tv_sec as c_long,
        now.tv_usec as c_long,
        if (recvmsg_flags & MSG_ERRQUEUE) != 0 { "error" } else { "regular" },
        res,
        CStr::from_ptr(inet_ntoa((*from_addr).sin_addr)).to_string_lossy(),
        (*msg).msg_controllen
    );

    let mut cmsg = cmsg_firsthdr(msg);
    while !cmsg.is_null() {
        print!("   cmsg len {}: ", (*cmsg).cmsg_len);
        match (*cmsg).cmsg_level {
            SOL_SOCKET => {
                print!("SOL_SOCKET ");
                match (*cmsg).cmsg_type {
                    SO_TIMESTAMP => {
                        let stamp = cmsg_data(cmsg) as *mut timeval;
                        print!("SO_TIMESTAMP {}.{:06}", (*stamp).tv_sec as c_long, (*stamp).tv_usec as c_long);
                    }
                    SO_TIMESTAMPNS => {
                        let stamp = cmsg_data(cmsg) as *mut timespec;
                        print!("SO_TIMESTAMPNS {}.{:09}", (*stamp).tv_sec as c_long, (*stamp).tv_nsec as c_long);
                    }
                    SO_TIMESTAMPING => {
                        let mut stamp = cmsg_data(cmsg) as *mut timespec;
                        print!("SO_TIMESTAMPING ");
                        print!("SW {}.{:09} ", (*stamp).tv_sec as c_long, (*stamp).tv_nsec as c_long);
                        stamp = stamp.add(1);
                        /* skip deprecated HW transformed */
                        stamp = stamp.add(1);
                        print!("HW raw {}.{:09}", (*stamp).tv_sec as c_long, (*stamp).tv_nsec as c_long);
                    }
                    _ => {
                        print!("type {}", (*cmsg).cmsg_type);
                    }
                }
            }
            IPPROTO_IP => {
                print!("IPPROTO_IP ");
                match (*cmsg).cmsg_type {
                    IP_RECVERR => {
                        let err = cmsg_data(cmsg) as *mut sock_extended_err;
                        print!(
                            "IP_RECVERR ee_errno '{}' ee_origin {} => {}",
                            strerror_string((*err).ee_errno as c_int),
                            (*err).ee_origin,
                            /* SO_EE_ORIGIN_TIMESTAMPING conditional from C source. */
                            "probably SO_EE_ORIGIN_TIMESTAMPING"
                        );
                        if (res as usize) < sync_len {
                            print!(" => truncated data?!");
                        } else if memcmp(
                            sync_p as *const c_void,
                            data.add(res as usize - sync_len) as *const c_void,
                            sync_len,
                        ) == 0
                        {
                            print!(" => GOT OUR DATA BACK (HURRAY!)");
                        }
                    }
                    IP_PKTINFO => {
                        let pktinfo = cmsg_data(cmsg) as *mut in_pktinfo;
                        print!("IP_PKTINFO interface index {}", (*pktinfo).ipi_ifindex);
                    }
                    _ => {
                        print!("type {}", (*cmsg).cmsg_type);
                    }
                }
            }
            _ => {
                print!("level {} type {}", (*cmsg).cmsg_level, (*cmsg).cmsg_type);
            }
        }
        println!();
        cmsg = cmsg_nxthdr(msg, cmsg);
    }

    if siocgstamp != 0 {
        if ioctl(sock, SIOCGSTAMP, &mut tv) != 0 {
            println!("   {}: {}", "SIOCGSTAMP", strerror_string(errno()));
        } else {
            println!("SIOCGSTAMP {}.{:06}", tv.tv_sec as c_long, tv.tv_usec as c_long);
        }
    }
    if siocgstampns != 0 {
        if ioctl(sock, SIOCGSTAMPNS, &mut ts) != 0 {
            println!("   {}: {}", "SIOCGSTAMPNS", strerror_string(errno()));
        } else {
            println!("SIOCGSTAMPNS {}.{:09}", ts.tv_sec as c_long, ts.tv_nsec as c_long);
        }
    }
}

#[repr(C)]
struct Control {
    cm: cmsghdr,
    control: [c_char; 512],
}

unsafe fn recvpacket(sock: c_int, recvmsg_flags: c_int, siocgstamp: c_int, siocgstampns: c_int, ptpv2: c_int) {
    let mut data = [0 as c_char; 256];
    let mut msg: msghdr = mem::zeroed();
    let mut entry: iovec = mem::zeroed();
    let mut from_addr: sockaddr_in = mem::zeroed();
    let mut control: Control = mem::zeroed();

    msg.msg_iov = &mut entry;
    msg.msg_iovlen = 1;
    entry.iov_base = data.as_mut_ptr() as *mut c_void;
    entry.iov_len = data.len();
    msg.msg_name = &mut from_addr as *mut _ as *mut c_void;
    msg.msg_namelen = mem::size_of::<sockaddr_in>() as socklen_t;
    msg.msg_control = &mut control as *mut _ as *mut c_void;
    msg.msg_controllen = mem::size_of::<Control>();

    let res = recvmsg(sock, &mut msg, recvmsg_flags | MSG_DONTWAIT);
    if res < 0 {
        println!(
            "{} {}: {}",
            "recvmsg",
            if (recvmsg_flags & MSG_ERRQUEUE) != 0 { "error" } else { "regular" },
            strerror_string(errno())
        );
    } else {
        printpacket(&mut msg, res as c_int, data.as_mut_ptr(), sock, recvmsg_flags, siocgstamp, siocgstampns, ptpv2);
    }
}

fn main() {
    unsafe {
        let mut so_timestamp = 0;
        let mut so_timestampns = 0;
        let mut siocgstamp = 0;
        let mut siocgstampns = 0;
        let mut ip_multicast_loop = 0;
        let mut ptpv2 = 0;
        let mut enabled: c_int = 1;
        let mut device: ifreq = mem::zeroed();
        let mut hwtstamp: ifreq = mem::zeroed();
        let mut hwconfig: hwtstamp_config = mem::zeroed();
        let mut hwconfig_requested: hwtstamp_config;
        let mut so_timestamping_get = so_timestamping { flags: 0, bind_phc: 0 };
        let mut so_timestamping = so_timestamping { flags: 0, bind_phc: 0 };
        let mut addr: sockaddr_in = mem::zeroed();
        let mut imr: ip_mreq = mem::zeroed();
        let mut iaddr: in_addr = mem::zeroed();
        let mut val: c_int;
        let mut len: socklen_t;
        let mut next: timeval = mem::zeroed();

        let args: Vec<CString> = env::args().map(|arg| CString::new(arg).unwrap()).collect();
        let argc = args.len() as c_int;
        if argc < 2 {
            usage(None);
        }
        let interface = args[1].as_ptr();
        let if_len = strlen(interface);
        if if_len >= IFNAMSIZ {
            println!("interface name exceeds IFNAMSIZ");
            std::process::exit(1);
        }

        if argc >= 3 && sscanf(args[2].as_ptr(), b"%d\0".as_ptr() as *const c_char, &mut so_timestamping.bind_phc) == 1 {
            val = 3;
        } else {
            val = 2;
        }

        let mut i = val;
        while i < argc {
            let arg = args[i as usize].as_ptr();
            if strcasecmp(arg, b"SO_TIMESTAMP\0".as_ptr() as *const c_char) == 0 {
                so_timestamp = 1;
            } else if strcasecmp(arg, b"SO_TIMESTAMPNS\0".as_ptr() as *const c_char) == 0 {
                so_timestampns = 1;
            } else if strcasecmp(arg, b"SIOCGSTAMP\0".as_ptr() as *const c_char) == 0 {
                siocgstamp = 1;
            } else if strcasecmp(arg, b"SIOCGSTAMPNS\0".as_ptr() as *const c_char) == 0 {
                siocgstampns = 1;
            } else if strcasecmp(arg, b"IP_MULTICAST_LOOP\0".as_ptr() as *const c_char) == 0 {
                ip_multicast_loop = 1;
            } else if strcasecmp(arg, b"PTPV2\0".as_ptr() as *const c_char) == 0 {
                ptpv2 = 1;
            } else if strcasecmp(arg, b"SOF_TIMESTAMPING_TX_HARDWARE\0".as_ptr() as *const c_char) == 0 {
                so_timestamping.flags |= SOF_TIMESTAMPING_TX_HARDWARE;
            } else if strcasecmp(arg, b"SOF_TIMESTAMPING_TX_SOFTWARE\0".as_ptr() as *const c_char) == 0 {
                so_timestamping.flags |= SOF_TIMESTAMPING_TX_SOFTWARE;
            } else if strcasecmp(arg, b"SOF_TIMESTAMPING_RX_HARDWARE\0".as_ptr() as *const c_char) == 0 {
                so_timestamping.flags |= SOF_TIMESTAMPING_RX_HARDWARE;
            } else if strcasecmp(arg, b"SOF_TIMESTAMPING_RX_SOFTWARE\0".as_ptr() as *const c_char) == 0 {
                so_timestamping.flags |= SOF_TIMESTAMPING_RX_SOFTWARE;
            } else if strcasecmp(arg, b"SOF_TIMESTAMPING_SOFTWARE\0".as_ptr() as *const c_char) == 0 {
                so_timestamping.flags |= SOF_TIMESTAMPING_SOFTWARE;
            } else if strcasecmp(arg, b"SOF_TIMESTAMPING_RAW_HARDWARE\0".as_ptr() as *const c_char) == 0 {
                so_timestamping.flags |= SOF_TIMESTAMPING_RAW_HARDWARE;
            } else if strcasecmp(arg, b"SOF_TIMESTAMPING_BIND_PHC\0".as_ptr() as *const c_char) == 0 {
                so_timestamping.flags |= SOF_TIMESTAMPING_BIND_PHC;
            } else {
                usage(Some(&CStr::from_ptr(arg).to_string_lossy()));
            }
            i += 1;
        }

        let sock = socket(PF_INET, SOCK_DGRAM, IPPROTO_UDP);
        if sock < 0 {
            bail("socket");
        }

        ptr::write_bytes(&mut device as *mut _ as *mut u8, 0, mem::size_of::<ifreq>());
        memcpy(device.ifr_name.as_mut_ptr() as *mut c_void, interface as *const c_void, if_len + 1);
        if ioctl(sock, SIOCGIFADDR, &mut device) < 0 {
            bail("getting interface IP address");
        }

        ptr::write_bytes(&mut hwtstamp as *mut _ as *mut u8, 0, mem::size_of::<ifreq>());
        memcpy(hwtstamp.ifr_name.as_mut_ptr() as *mut c_void, interface as *const c_void, if_len + 1);
        hwtstamp.ifr_ifru.ifru_data = &mut hwconfig as *mut _ as *mut c_char;
        ptr::write_bytes(&mut hwconfig as *mut _ as *mut u8, 0, mem::size_of::<hwtstamp_config>());
        hwconfig.tx_type = if (so_timestamping.flags & SOF_TIMESTAMPING_TX_HARDWARE) != 0 {
            HWTSTAMP_TX_ON
        } else {
            HWTSTAMP_TX_OFF
        };
        hwconfig.rx_filter = if (so_timestamping.flags & SOF_TIMESTAMPING_RX_HARDWARE) != 0 {
            if ptpv2 != 0 {
                HWTSTAMP_FILTER_PTP_V2_L4_SYNC
            } else {
                HWTSTAMP_FILTER_PTP_V1_L4_SYNC
            }
        } else {
            HWTSTAMP_FILTER_NONE
        };
        hwconfig_requested = hwconfig;
        if ioctl(sock, SIOCSHWTSTAMP, &mut hwtstamp) < 0 {
            if (errno() == EINVAL || errno() == ENOTSUP)
                && hwconfig_requested.tx_type == HWTSTAMP_TX_OFF
                && hwconfig_requested.rx_filter == HWTSTAMP_FILTER_NONE
            {
                println!("SIOCSHWTSTAMP: disabling hardware time stamping not possible");
            } else {
                bail("SIOCSHWTSTAMP");
            }
        }
        println!(
            "SIOCSHWTSTAMP: tx_type {} requested, got {}; rx_filter {} requested, got {}",
            hwconfig_requested.tx_type, hwconfig.tx_type, hwconfig_requested.rx_filter, hwconfig.rx_filter
        );

        /* bind to PTP port */
        addr.sin_family = AF_INET as sa_family_t;
        addr.sin_addr.s_addr = htonl(INADDR_ANY);
        addr.sin_port = htons(319 /* PTP event port */);
        if bind(sock, &mut addr as *mut _ as *mut sockaddr, mem::size_of::<sockaddr_in>() as socklen_t) < 0 {
            bail("bind");
        }

        if setsockopt(sock, SOL_SOCKET, SO_BINDTODEVICE, interface as *const c_void, if_len as socklen_t) != 0 {
            bail("bind device");
        }

        /* set multicast group for outgoing packets */
        inet_aton(b"224.0.1.130\0".as_ptr() as *const c_char, &mut iaddr); /* alternate PTP domain 1 */
        addr.sin_addr = iaddr;
        imr.imr_multiaddr.s_addr = iaddr.s_addr;
        imr.imr_interface.s_addr = (*( &device.ifr_ifru.ifru_addr as *const _ as *const sockaddr_in)).sin_addr.s_addr;
        if setsockopt(
            sock,
            IPPROTO_IP,
            IP_MULTICAST_IF,
            &mut imr.imr_interface.s_addr as *mut _ as *mut c_void,
            mem::size_of::<in_addr>() as socklen_t,
        ) < 0
        {
            bail("set multicast");
        }

        /* join multicast group, loop our own packet */
        if setsockopt(sock, IPPROTO_IP, IP_ADD_MEMBERSHIP, &mut imr as *mut _ as *mut c_void, mem::size_of::<ip_mreq>() as socklen_t) < 0 {
            bail("join multicast group");
        }

        if setsockopt(
            sock,
            IPPROTO_IP,
            IP_MULTICAST_LOOP,
            &mut ip_multicast_loop as *mut _ as *mut c_void,
            mem::size_of_val(&enabled) as socklen_t,
        ) < 0
        {
            bail("loop multicast");
        }

        /* set socket options for time stamping */
        if so_timestamp != 0
            && setsockopt(sock, SOL_SOCKET, SO_TIMESTAMP, &mut enabled as *mut _ as *mut c_void, mem::size_of_val(&enabled) as socklen_t) < 0
        {
            bail("setsockopt SO_TIMESTAMP");
        }

        if so_timestampns != 0
            && setsockopt(sock, SOL_SOCKET, SO_TIMESTAMPNS, &mut enabled as *mut _ as *mut c_void, mem::size_of_val(&enabled) as socklen_t) < 0
        {
            bail("setsockopt SO_TIMESTAMPNS");
        }

        if so_timestamping.flags != 0
            && setsockopt(
                sock,
                SOL_SOCKET,
                SO_TIMESTAMPING,
                &mut so_timestamping as *mut _ as *mut c_void,
                mem::size_of_val(&so_timestamping) as socklen_t,
            ) < 0
        {
            bail("setsockopt SO_TIMESTAMPING");
        }

        /* request IP_PKTINFO for debugging purposes */
        if setsockopt(sock, SOL_IP, IP_PKTINFO, &mut enabled as *mut _ as *mut c_void, mem::size_of_val(&enabled) as socklen_t) < 0 {
            println!("{}: {}", "setsockopt IP_PKTINFO", strerror_string(errno()));
        }

        /* verify socket options */
        len = mem::size_of_val(&val) as socklen_t;
        if getsockopt(sock, SOL_SOCKET, SO_TIMESTAMP, &mut val as *mut _ as *mut c_void, &mut len) < 0 {
            println!("{}: {}", "getsockopt SO_TIMESTAMP", strerror_string(errno()));
        } else {
            println!("SO_TIMESTAMP {}", val);
        }

        if getsockopt(sock, SOL_SOCKET, SO_TIMESTAMPNS, &mut val as *mut _ as *mut c_void, &mut len) < 0 {
            println!("{}: {}", "getsockopt SO_TIMESTAMPNS", strerror_string(errno()));
        } else {
            println!("SO_TIMESTAMPNS {}", val);
        }

        len = mem::size_of_val(&so_timestamping_get) as socklen_t;
        if getsockopt(
            sock,
            SOL_SOCKET,
            SO_TIMESTAMPING,
            &mut so_timestamping_get as *mut _ as *mut c_void,
            &mut len,
        ) < 0
        {
            println!("{}: {}", "getsockopt SO_TIMESTAMPING", strerror_string(errno()));
        } else {
            println!(
                "SO_TIMESTAMPING flags {}, bind phc {}",
                so_timestamping_get.flags, so_timestamping_get.bind_phc
            );
            if so_timestamping_get.flags != so_timestamping.flags || so_timestamping_get.bind_phc != so_timestamping.bind_phc {
                println!("   not expected, flags {}, bind phc {}", so_timestamping.flags, so_timestamping.bind_phc);
            }
        }

        /* send packets forever every five seconds */
        gettimeofday(&mut next, ptr::null_mut());
        next.tv_sec = (next.tv_sec + 1) / 5 * 5;
        next.tv_usec = 0;
        loop {
            let mut now: timeval = mem::zeroed();
            let mut delta: timeval = mem::zeroed();
            let mut readfs: fd_set = mem::zeroed();
            let mut errorfs: fd_set = mem::zeroed();

            gettimeofday(&mut now, ptr::null_mut());
            let delta_us = (next.tv_sec - now.tv_sec) as c_long * 1000000 + (next.tv_usec - now.tv_usec) as c_long;
            if delta_us > 0 {
                /* continue waiting for timeout or data */
                delta.tv_sec = delta_us / 1000000;
                delta.tv_usec = delta_us % 1000000;

                FD_ZERO(&mut readfs);
                FD_ZERO(&mut errorfs);
                FD_SET(sock, &mut readfs);
                FD_SET(sock, &mut errorfs);
                println!("{}.{:06}: select {}us", now.tv_sec as c_long, now.tv_usec as c_long, delta_us);
                let res = select(sock + 1, &mut readfs, ptr::null_mut(), &mut errorfs, &mut delta);
                gettimeofday(&mut now, ptr::null_mut());
                println!(
                    "{}.{:06}: select returned: {}, {}",
                    now.tv_sec as c_long,
                    now.tv_usec as c_long,
                    res,
                    if res < 0 { strerror_string(errno()) } else { "success".to_string() }
                );
                if res > 0 {
                    if FD_ISSET(sock, &mut readfs) {
                        println!("ready for reading");
                    }
                    if FD_ISSET(sock, &mut errorfs) {
                        println!("has error");
                    }
                    recvpacket(sock, 0, siocgstamp, siocgstampns, ptpv2);
                    recvpacket(sock, MSG_ERRQUEUE, siocgstamp, siocgstampns, ptpv2);
                }
            } else {
                /* write one packet */
                sendpacket(sock, &mut addr as *mut _ as *mut sockaddr, mem::size_of_val(&addr) as socklen_t, ptpv2);
                next.tv_sec += 5;
                continue;
            }
        }
    }
}
