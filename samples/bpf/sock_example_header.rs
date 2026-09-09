/* SPDX-License-Identifier: GPL-2.0 */

// C headers provide the following constants, types, and functions.
// They remain external dependencies of this translation.

#[repr(C)]
pub struct sockaddr_ll {
    pub sll_family: u16,
    pub sll_protocol: u16,
    pub sll_ifindex: i32,
    pub sll_hatype: u16,
    pub sll_pkttype: u8,
    pub sll_halen: u8,
    pub sll_addr: [u8; 8],
}

unsafe extern "C" {
    fn socket(domain: i32, type_: i32, protocol: i32) -> i32;
    fn htons(hostshort: u16) -> u16;
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn if_nametoindex(ifname: *const core::ffi::c_char) -> u32;
    fn bind(sockfd: i32, addr: *const sockaddr, addrlen: u32) -> i32;
    fn printf(format: *const core::ffi::c_char, ...) -> i32;
    fn strerror(errnum: i32) -> *mut core::ffi::c_char;
    fn close(fd: i32) -> i32;
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [core::ffi::c_char; 14],
}

pub const PF_PACKET: i32 = 17;
pub const SOCK_RAW: i32 = 3;
pub const SOCK_NONBLOCK: i32 = 0x800;
pub const SOCK_CLOEXEC: i32 = 0x80000;
pub const ETH_P_ALL: u16 = 0x0003;
pub const AF_PACKET: u16 = 17;

#[inline]
pub unsafe fn open_raw_sock(name: *const core::ffi::c_char) -> i32 {
    let mut sll: sockaddr_ll = core::mem::zeroed();
    let sock: i32;

    sock = socket(
        PF_PACKET,
        SOCK_RAW | SOCK_NONBLOCK | SOCK_CLOEXEC,
        htons(ETH_P_ALL) as i32,
    );
    if sock < 0 {
        printf(b"cannot create raw socket\0".as_ptr() as *const core::ffi::c_char);
        return -1;
    }

    memset(
        &mut sll as *mut sockaddr_ll as *mut core::ffi::c_void,
        0,
        core::mem::size_of::<sockaddr_ll>(),
    );
    sll.sll_family = AF_PACKET;
    sll.sll_ifindex = if_nametoindex(name) as i32;
    sll.sll_protocol = htons(ETH_P_ALL);
    if bind(
        sock,
        &sll as *const sockaddr_ll as *const sockaddr,
        core::mem::size_of::<sockaddr_ll>() as u32,
    ) < 0
    {
        printf(
            b"bind to %s: %s\n\0".as_ptr() as *const core::ffi::c_char,
            name,
            strerror(errno()),
        );
        close(sock);
        return -1;
    }

    sock
}

unsafe extern "C" {
    fn errno() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
