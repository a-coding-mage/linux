// SPDX-License-Identifier: GPL-2.0
/*
 * Author: Brett A C Sheffield <bacs@librecast.net>
 *
 * Kernel selftest for the IPv6 fragmentation regression which affected stable
 * kernels:
 *
 *   https://lore.kernel.org/stable/aElivdUXqd1OqgMY@karahi.gladserv.com
 *
 * Commit: a18dfa9925b9 ("ipv6: save dontfrag in cork") was backported to stable
 * without some prerequisite commits.
 *
 * This caused a regression when sending IPv6 UDP packets by preventing
 * fragmentation and instead returning -1 (EMSGSIZE).
 *
 * This selftest demonstrates the issue by sending an IPv6 UDP packet to
 * localhost (::1) on the loopback interface from the autoconfigured link-local
 * address.
 *
 * sendmsg(2) returns bytes sent correctly on a working kernel, and returns -1
 * (EMSGSIZE) when the regression is present.
 *
 * The regression was not present in the mainline kernel, but add this test to
 * catch similar breakage in future.
 */

// C dependencies translated from:
// <error.h>, <net/if.h>, <netinet/in.h>, <sched.h>, <stdio.h>,
// <sys/ioctl.h>, <sys/socket.h>, <unistd.h>, and "kselftest.h".

use libc::{
    c_char, c_int, c_short, c_uint, c_ulong, c_void, in6_addr, iovec, msghdr, sockaddr,
    sockaddr_in6, ssize_t, AF_INET6, AF_LOCAL, EADDRNOTAVAIL, IFF_UP, SIOCGIFFLAGS, SIOCSIFFLAGS,
    SIOCSIFMTU, SOCK_DGRAM, SOCK_STREAM,
};

const MTU: c_int = 1500;
const LARGER_THAN_MTU: usize = 8192;

const IFNAMSIZ: usize = 16;
const CLONE_NEWNET: c_int = 0x40000000;

const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;

#[repr(C)]
union IfreqIfru {
    ifru_addr: sockaddr,
    ifru_dstaddr: sockaddr,
    ifru_broadaddr: sockaddr,
    ifru_netmask: sockaddr,
    ifru_hwaddr: sockaddr,
    ifru_flags: c_short,
    ifru_ivalue: c_int,
    ifru_mtu: c_int,
    ifru_map: [u8; 24],
    ifru_slave: [c_char; IFNAMSIZ],
    ifru_newname: [c_char; IFNAMSIZ],
    ifru_data: *mut c_void,
}

#[repr(C)]
struct Ifreq {
    ifr_name: [c_char; IFNAMSIZ],
    ifr_ifru: IfreqIfru,
}

unsafe extern "C" {
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...) -> !;
    fn printf(format: *const c_char, ...) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn usleep(usec: c_uint) -> c_int;
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn setup() {
    let mut ifr = Ifreq {
        ifr_name: [0; IFNAMSIZ],
        ifr_ifru: IfreqIfru { ifru_ivalue: 0 },
    };
    let mut ctl: c_int;

    ifr.ifr_name[0] = b'l' as c_char;
    ifr.ifr_name[1] = b'o' as c_char;

    /* we need to set MTU, so do this in a namespace to play nicely */
    if unsafe { unshare(CLONE_NEWNET) } == -1 {
        unsafe { error(KSFT_FAIL, errno(), c"unshare".as_ptr()) };
    }

    ctl = unsafe { socket(AF_LOCAL, SOCK_STREAM, 0) };
    if ctl == -1 {
        unsafe { error(KSFT_FAIL, errno(), c"socket".as_ptr()) };
    }

    /* ensure MTU is smaller than what we plan to send */
    ifr.ifr_ifru.ifru_mtu = MTU;
    if unsafe { ioctl(ctl, SIOCSIFFLAGS as c_ulong, &mut ifr as *mut Ifreq) } == -1 {
        unsafe { error(KSFT_FAIL, errno(), c"ioctl: set MTU".as_ptr()) };
    }

    /* bring up interface */
    if unsafe { ioctl(ctl, SIOCGIFFLAGS as c_ulong, &mut ifr as *mut Ifreq) } == -1 {
        unsafe { error(KSFT_FAIL, errno(), c"ioctl SIOCGIFFLAGS".as_ptr()) };
    }
    ifr.ifr_ifru.ifru_flags = unsafe { ifr.ifr_ifru.ifru_flags } | IFF_UP as c_short;
    if unsafe { ioctl(ctl, SIOCSIFMTU as c_ulong, &mut ifr as *mut Ifreq) } == -1 {
        unsafe { error(KSFT_FAIL, errno(), c"ioctl: bring interface up".as_ptr()) };
    }

    if unsafe { close(ctl) } == -1 {
        unsafe { error(KSFT_FAIL, errno(), c"close".as_ptr()) };
    }
}

fn main() -> c_int {
    unsafe {
        let addr = in6_addr {
            s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x01], /* ::1 */
        };
        let mut sa = sockaddr_in6 {
            sin6_family: AF_INET6 as _,
            sin6_port: 9u16.to_be(), /* port 9/udp (DISCARD) */
            sin6_flowinfo: 0,
            sin6_addr: addr,
            sin6_scope_id: 0,
        };
        static mut BUF: [c_char; LARGER_THAN_MTU] = [0; LARGER_THAN_MTU];
        let mut iov = iovec {
            iov_base: (&raw mut BUF).cast::<c_void>(),
            iov_len: core::mem::size_of_val(&*(&raw const BUF)),
        };
        let msg = msghdr {
            msg_name: (&mut sa as *mut sockaddr_in6).cast::<c_void>(),
            msg_namelen: core::mem::size_of_val(&sa) as _,
            msg_iov: &mut iov as *mut iovec,
            msg_iovlen: 1,
            msg_control: core::ptr::null_mut(),
            msg_controllen: 0,
            msg_flags: 0,
        };
        let mut rc: ssize_t;
        let s: c_int;

        printf(c"Testing IPv6 fragmentation\n".as_ptr());
        setup();
        s = socket(AF_INET6, SOCK_DGRAM, 0);

        loop {
            rc = sendmsg(s, &msg as *const msghdr, 0);
            if rc == -1 {
                /* if interface wasn't ready, try again */
                if errno() == EADDRNOTAVAIL {
                    usleep(1000);
                    continue;
                }
                error(KSFT_FAIL, errno(), c"sendmsg".as_ptr());
            } else if rc != LARGER_THAN_MTU as ssize_t {
                error(
                    KSFT_FAIL,
                    errno(),
                    c"sendmsg returned %zi, expected %i".as_ptr(),
                    rc,
                    LARGER_THAN_MTU as c_int,
                );
            }
            break;
        }

        printf(c"[PASS] sendmsg() returned %zi\n".as_ptr(), rc);
        if close(s) == -1 {
            error(KSFT_FAIL, errno(), c"close".as_ptr());
        }
        KSFT_PASS
    }
}
