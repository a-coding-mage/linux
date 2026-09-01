// SPDX-License-Identifier: GPL-2.0
//
// Translated from testing/selftests/net/lib/xdp_helper.c.
// C includes required errno/stdio/stdlib/string/unistd/sys mmap/socket,
// linux if_xdp/if_link, net if, inttypes, and "ksft.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const UMEM_SZ: c_uint = 1_u32 << 16;
const NUM_DESC: c_uint = UMEM_SZ / 2048;

const AF_XDP: c_int = 44;
const SOCK_RAW: c_int = 3;
const SOL_XDP: c_int = 283;
const XDP_UMEM_REG: c_int = 4;
const XDP_UMEM_FILL_RING: c_int = 5;
const XDP_UMEM_COMPLETION_RING: c_int = 6;
const XDP_RX_RING: c_int = 2;
const XDP_ZEROCOPY: u16 = 1 << 2;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const EAFNOSUPPORT: c_int = 97;
const EBUSY: c_int = 16;

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_xdp {
    sxdp_family: u16,
    sxdp_flags: u16,
    sxdp_ifindex: u32,
    sxdp_queue_id: u32,
    sxdp_shared_umem_fd: u32,
}

#[repr(C)]
struct xdp_umem_reg {
    addr: u64,
    len: u64,
    chunk_size: u32,
    headroom: u32,
    flags: u32,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut c_void;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn atoi(nptr: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: c_uint,
    ) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: c_uint) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;

    fn ksft_ready();
    fn ksft_wait();
}

unsafe fn print_usage(bin: *const c_char) {
    unsafe {
        fprintf(
            stderr,
            b"Usage: %s ifindex queue_id [-z]\n\nwhere:\n\t-z: force zerocopy mode\0"
                .as_ptr() as *const c_char,
            bin,
        );
    }
}

/* this is a simple helper program that creates an XDP socket and does the
 * minimum necessary to get bind() to succeed.
 *
 * this test program is not intended to actually process packets, but could be
 * extended in the future if that is actually needed.
 *
 * it is used by queues.py to ensure the xsk netlinux attribute is set
 * correctly.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut umem_reg: xdp_umem_reg = unsafe { core::mem::zeroed() };
    let mut sxdp: sockaddr_xdp = unsafe { core::mem::zeroed() };
    let mut num_desc: c_int = NUM_DESC as c_int;
    let umem_area: *mut c_void;
    let mut retry: c_int = 0;
    let ifindex: c_int;
    let sock_fd: c_int;
    let queue: c_int;

    unsafe {
        if argc != 3 && argc != 4 {
            print_usage(*argv.offset(0));
            return 1;
        }

        sock_fd = socket(AF_XDP, SOCK_RAW, 0);
        if sock_fd < 0 {
            perror(b"socket creation failed\0".as_ptr() as *const c_char);
            /* if the kernel doesn't support AF_XDP, let the test program
             * know with -1. All other error paths return 1.
             */
            if errno == EAFNOSUPPORT {
                return -1;
            }
            return 1;
        }

        /* "Probing mode", just checking if AF_XDP sockets are supported */
        if strcmp(*argv.offset(1), b"-\0".as_ptr() as *const c_char) == 0
            && strcmp(*argv.offset(2), b"-\0".as_ptr() as *const c_char) == 0
        {
            printf(b"AF_XDP support detected\n\0".as_ptr() as *const c_char);
            close(sock_fd);
            return 0;
        }

        ifindex = atoi(*argv.offset(1));
        queue = atoi(*argv.offset(2));

        umem_area = mmap(
            ptr::null_mut(),
            UMEM_SZ as usize,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        );
        if umem_area == MAP_FAILED {
            perror(b"mmap failed\0".as_ptr() as *const c_char);
            return 1;
        }

        umem_reg.addr = umem_area as usize as u64;
        umem_reg.len = UMEM_SZ as u64;
        umem_reg.chunk_size = 2048;
        umem_reg.headroom = 0;

        setsockopt(
            sock_fd,
            SOL_XDP,
            XDP_UMEM_REG,
            &umem_reg as *const xdp_umem_reg as *const c_void,
            size_of::<xdp_umem_reg>() as c_uint,
        );
        setsockopt(
            sock_fd,
            SOL_XDP,
            XDP_UMEM_FILL_RING,
            &num_desc as *const c_int as *const c_void,
            size_of::<c_int>() as c_uint,
        );
        setsockopt(
            sock_fd,
            SOL_XDP,
            XDP_UMEM_COMPLETION_RING,
            &num_desc as *const c_int as *const c_void,
            size_of::<c_int>() as c_uint,
        );
        setsockopt(
            sock_fd,
            SOL_XDP,
            XDP_RX_RING,
            &num_desc as *const c_int as *const c_void,
            size_of::<c_int>() as c_uint,
        );

        sxdp.sxdp_family = AF_XDP as u16;
        sxdp.sxdp_ifindex = ifindex as u32;
        sxdp.sxdp_queue_id = queue as u32;
        sxdp.sxdp_flags = 0;

        if argc > 3 {
            if strcmp(*argv.offset(3), b"-z\0".as_ptr() as *const c_char) == 0 {
                sxdp.sxdp_flags = XDP_ZEROCOPY;
            } else {
                print_usage(*argv.offset(0));
                return 1;
            }
        }

        loop {
            if bind(
                sock_fd,
                &sxdp as *const sockaddr_xdp as *const sockaddr,
                size_of::<sockaddr_xdp>() as c_uint,
            ) == 0
            {
                break;
            }

            if errno == EBUSY && retry < 3 {
                retry += 1;
                sleep(1);
                continue;
            } else {
                perror(b"bind failed\0".as_ptr() as *const c_char);
                munmap(umem_area, UMEM_SZ as usize);
                close(sock_fd);
                return 1;
            }
        }

        ksft_ready();
        ksft_wait();

        /* parent program will write a byte to stdin when its ready for this
         * helper to exit
         */

        close(sock_fd);
        return 0;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
