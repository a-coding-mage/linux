// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, "cgroup_helpers.h", <netinet/tcp.h>,
// <linux/netlink.h>, and "sockopt_sk.skel.h".

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

const SOL_TCP: c_int = libc::IPPROTO_TCP;
const SOL_CUSTOM: c_int = 0xdeadbeefu32 as c_int;

#[repr(C)]
union SockoptSkBuf {
    u8_: [c_char; 4],
    u32_: u32,
    cc: [c_char; 16], /* TCP_CA_NAME_MAX */
    zc: tcp_zerocopy_receive,
}

#[repr(C)]
struct sockopt_sk_bss {
    page_size: c_int,
}

#[repr(C)]
struct sockopt_sk_progs {
    _setsockopt: *mut bpf_program,
    _getsockopt: *mut bpf_program,
}

#[repr(C)]
struct sockopt_sk_links {
    _setsockopt: *mut bpf_link,
    _getsockopt: *mut bpf_link,
}

#[repr(C)]
struct sockopt_sk {
    bss: *mut sockopt_sk_bss,
    progs: sockopt_sk_progs,
    links: sockopt_sk_links,
}

enum bpf_program {}
enum bpf_link {}

unsafe extern "C" {
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: libc::socklen_t,
    ) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut libc::socklen_t,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn getpagesize() -> c_int;
    fn __errno_location() -> *mut c_int;

    fn log_err(fmt: *const c_char, ...);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;

    fn sockopt_sk__open_and_load() -> *mut sockopt_sk;
    fn sockopt_sk__destroy(obj: *mut sockopt_sk);
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn test__join_cgroup(path: *const c_char) -> c_int;

    fn ASSERT_EQ(actual: libc::socklen_t, expected: libc::socklen_t, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

unsafe fn errno_ptr() -> *mut c_int {
    unsafe { __errno_location() }
}

unsafe fn getsetsockopt() -> c_int {
    let mut fd: c_int;
    let mut err: c_int;
    let mut buf: SockoptSkBuf = unsafe { zeroed() };
    let mut optlen: libc::socklen_t;
    let mut big_buf: *mut c_char = null_mut();

    fd = unsafe { socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    if fd < 0 {
        unsafe { log_err(b"Failed to create socket\0".as_ptr() as *const c_char) };
        return -1;
    }

    /* IP_TOS - BPF bypass */

    optlen = (unsafe { getpagesize() } * 2) as libc::socklen_t;
    big_buf = unsafe { calloc(1, optlen as usize) as *mut c_char };
    if big_buf.is_null() {
        unsafe { log_err(b"Couldn't allocate two pages\0".as_ptr() as *const c_char) };
        goto_err(fd, big_buf);
        return -1;
    }

    unsafe { *(big_buf as *mut c_int) = 0x08 };
    err = unsafe {
        setsockopt(
            fd,
            libc::SOL_IP,
            libc::IP_TOS,
            big_buf as *const c_void,
            optlen,
        )
    };
    if err != 0 {
        unsafe { log_err(b"Failed to call setsockopt(IP_TOS)\0".as_ptr() as *const c_char) };
        goto_err(fd, big_buf);
        return -1;
    }

    unsafe { memset(big_buf as *mut c_void, 0, optlen as usize) };
    optlen = 1;
    err = unsafe {
        getsockopt(
            fd,
            libc::SOL_IP,
            libc::IP_TOS,
            big_buf as *mut c_void,
            &mut optlen,
        )
    };
    if err != 0 {
        unsafe { log_err(b"Failed to call getsockopt(IP_TOS)\0".as_ptr() as *const c_char) };
        goto_err(fd, big_buf);
        return -1;
    }

    if unsafe { *big_buf } != 0x08 {
        unsafe {
            log_err(
                b"Unexpected getsockopt(IP_TOS) optval 0x%x != 0x08\0".as_ptr() as *const c_char,
                *big_buf as c_int,
            )
        };
        goto_err(fd, big_buf);
        return -1;
    }

    /* IP_TTL - EPERM */

    unsafe { buf.u8_[0] = 1 };
    err = unsafe {
        setsockopt(
            fd,
            libc::SOL_IP,
            libc::IP_TTL,
            &mut buf as *mut SockoptSkBuf as *const c_void,
            1,
        )
    };
    if err == 0 || unsafe { *errno_ptr() } != libc::EPERM {
        unsafe { log_err(b"Unexpected success from setsockopt(IP_TTL)\0".as_ptr() as *const c_char) };
        goto_err(fd, big_buf);
        return -1;
    }

    /* SOL_CUSTOM - handled by BPF */

    unsafe { buf.u8_[0] = 0x01 };
    err = unsafe {
        setsockopt(
            fd,
            SOL_CUSTOM,
            0,
            &mut buf as *mut SockoptSkBuf as *const c_void,
            1,
        )
    };
    if err != 0 {
        unsafe { log_err(b"Failed to call setsockopt\0".as_ptr() as *const c_char) };
        goto_err(fd, big_buf);
        return -1;
    }

    buf.u32_ = 0x00;
    optlen = 4;
    err = unsafe {
        getsockopt(
            fd,
            SOL_CUSTOM,
            0,
            &mut buf as *mut SockoptSkBuf as *mut c_void,
            &mut optlen,
        )
    };
    if err != 0 {
        unsafe { log_err(b"Failed to call getsockopt\0".as_ptr() as *const c_char) };
        goto_err(fd, big_buf);
        return -1;
    }

    if optlen != 1 {
        unsafe {
            log_err(
                b"Unexpected optlen %d != 1\0".as_ptr() as *const c_char,
                optlen as c_int,
            )
        };
        goto_err(fd, big_buf);
        return -1;
    }
    if unsafe { buf.u8_[0] } != 0x01 {
        unsafe {
            log_err(
                b"Unexpected buf[0] 0x%02x != 0x01\0".as_ptr() as *const c_char,
                buf.u8_[0] as c_int,
            )
        };
        goto_err(fd, big_buf);
        return -1;
    }

    /* IP_FREEBIND - BPF can't access optval past PAGE_SIZE */

    optlen = (unsafe { getpagesize() } * 2) as libc::socklen_t;
    unsafe { memset(big_buf as *mut c_void, 0, optlen as usize) };

    err = unsafe {
        setsockopt(
            fd,
            libc::SOL_IP,
            libc::IP_FREEBIND,
            big_buf as *const c_void,
            optlen,
        )
    };
    if err != 0 {
        unsafe {
            log_err(
                b"Failed to call setsockopt, ret=%d\0".as_ptr() as *const c_char,
                err,
            )
        };
        goto_err(fd, big_buf);
        return -1;
    }

    err = unsafe {
        getsockopt(
            fd,
            libc::SOL_IP,
            libc::IP_FREEBIND,
            big_buf as *mut c_void,
            &mut optlen,
        )
    };
    if err != 0 {
        unsafe {
            log_err(
                b"Failed to call getsockopt, ret=%d\0".as_ptr() as *const c_char,
                err,
            )
        };
        goto_err(fd, big_buf);
        return -1;
    }

    if optlen != 1 || unsafe { *(big_buf as *mut u8) } != 0x55 {
        unsafe {
            log_err(
                b"Unexpected IP_FREEBIND getsockopt, optlen=%d, optval=0x%x\0".as_ptr()
                    as *const c_char,
                optlen as c_int,
                *(big_buf as *mut u8) as c_int,
            )
        };
    }

    /* SO_SNDBUF is overwritten */

    buf.u32_ = 0x01010101;
    err = unsafe {
        setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_SNDBUF,
            &mut buf as *mut SockoptSkBuf as *const c_void,
            4,
        )
    };
    if err != 0 {
        unsafe { log_err(b"Failed to call setsockopt(SO_SNDBUF)\0".as_ptr() as *const c_char) };
        goto_err(fd, big_buf);
        return -1;
    }

    buf.u32_ = 0x00;
    optlen = 4;
    err = unsafe {
        getsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_SNDBUF,
            &mut buf as *mut SockoptSkBuf as *mut c_void,
            &mut optlen,
        )
    };
    if err != 0 {
        unsafe { log_err(b"Failed to call getsockopt(SO_SNDBUF)\0".as_ptr() as *const c_char) };
        goto_err(fd, big_buf);
        return -1;
    }

    if unsafe { buf.u32_ } != 0x55AA * 2 {
        unsafe {
            log_err(
                b"Unexpected getsockopt(SO_SNDBUF) 0x%x != 0x55AA*2\0".as_ptr()
                    as *const c_char,
                buf.u32_,
            )
        };
        goto_err(fd, big_buf);
        return -1;
    }

    /* TCP_CONGESTION can extend the string */

    unsafe { strscpy(buf.cc.as_mut_ptr(), b"nv\0".as_ptr() as *const c_char) };
    err = unsafe {
        setsockopt(
            fd,
            SOL_TCP,
            libc::TCP_CONGESTION,
            &mut buf as *mut SockoptSkBuf as *const c_void,
            strlen(b"nv\0".as_ptr() as *const c_char) as libc::socklen_t,
        )
    };
    if err != 0 {
        unsafe {
            log_err(b"Failed to call setsockopt(TCP_CONGESTION)\0".as_ptr() as *const c_char)
        };
        goto_err(fd, big_buf);
        return -1;
    }

    optlen = size_of::<[c_char; 16]>() as libc::socklen_t;
    err = unsafe {
        getsockopt(
            fd,
            SOL_TCP,
            libc::TCP_CONGESTION,
            &mut buf as *mut SockoptSkBuf as *mut c_void,
            &mut optlen,
        )
    };
    if err != 0 {
        unsafe {
            log_err(b"Failed to call getsockopt(TCP_CONGESTION)\0".as_ptr() as *const c_char)
        };
        goto_err(fd, big_buf);
        return -1;
    }

    if unsafe { strcmp(buf.cc.as_ptr(), b"cubic\0".as_ptr() as *const c_char) } != 0 {
        unsafe {
            log_err(
                b"Unexpected getsockopt(TCP_CONGESTION) %s != %s\0".as_ptr() as *const c_char,
                buf.cc.as_ptr(),
                b"cubic\0".as_ptr() as *const c_char,
            )
        };
        goto_err(fd, big_buf);
        return -1;
    }

    /* TCP_ZEROCOPY_RECEIVE triggers */
    unsafe {
        memset(
            &mut buf as *mut SockoptSkBuf as *mut c_void,
            0,
            size_of::<SockoptSkBuf>(),
        )
    };
    optlen = size_of::<tcp_zerocopy_receive>() as libc::socklen_t;
    err = unsafe {
        getsockopt(
            fd,
            SOL_TCP,
            libc::TCP_ZEROCOPY_RECEIVE,
            &mut buf as *mut SockoptSkBuf as *mut c_void,
            &mut optlen,
        )
    };
    if err != 0 {
        unsafe {
            log_err(
                b"Unexpected getsockopt(TCP_ZEROCOPY_RECEIVE) err=%d errno=%d\0".as_ptr()
                    as *const c_char,
                err,
                *errno_ptr(),
            )
        };
        goto_err(fd, big_buf);
        return -1;
    }

    unsafe {
        memset(
            &mut buf as *mut SockoptSkBuf as *mut c_void,
            0,
            size_of::<SockoptSkBuf>(),
        )
    };
    unsafe {
        buf.zc.address = 12345; /* Not page aligned. Rejected by tcp_zerocopy_receive() */
    }
    optlen = size_of::<tcp_zerocopy_receive>() as libc::socklen_t;
    unsafe { *errno_ptr() = 0 };
    err = unsafe {
        getsockopt(
            fd,
            SOL_TCP,
            libc::TCP_ZEROCOPY_RECEIVE,
            &mut buf as *mut SockoptSkBuf as *mut c_void,
            &mut optlen,
        )
    };
    if unsafe { *errno_ptr() } != libc::EINVAL {
        unsafe {
            log_err(
                b"Unexpected getsockopt(TCP_ZEROCOPY_RECEIVE) err=%d errno=%d\0".as_ptr()
                    as *const c_char,
                err,
                *errno_ptr(),
            )
        };
        goto_err(fd, big_buf);
        return -1;
    }

    /* optval=NULL case is handled correctly */

    unsafe { close(fd) };
    fd = unsafe { socket(libc::AF_NETLINK, libc::SOCK_RAW, 0) };
    if fd < 0 {
        unsafe { log_err(b"Failed to create AF_NETLINK socket\0".as_ptr() as *const c_char) };
        goto_err(fd, big_buf);
        return -1;
    }

    buf.u32_ = 1;
    optlen = size_of::<u32>() as libc::socklen_t;
    err = unsafe {
        setsockopt(
            fd,
            libc::SOL_NETLINK,
            libc::NETLINK_ADD_MEMBERSHIP,
            &mut buf as *mut SockoptSkBuf as *const c_void,
            optlen,
        )
    };
    if err != 0 {
        unsafe {
            log_err(
                b"Unexpected getsockopt(NETLINK_ADD_MEMBERSHIP) err=%d errno=%d\0".as_ptr()
                    as *const c_char,
                err,
                *errno_ptr(),
            )
        };
        goto_err(fd, big_buf);
        return -1;
    }

    optlen = 0;
    err = unsafe {
        getsockopt(
            fd,
            libc::SOL_NETLINK,
            libc::NETLINK_LIST_MEMBERSHIPS,
            null_mut(),
            &mut optlen,
        )
    };
    if err != 0 {
        unsafe {
            log_err(
                b"Unexpected getsockopt(NETLINK_LIST_MEMBERSHIPS) err=%d errno=%d\0".as_ptr()
                    as *const c_char,
                err,
                *errno_ptr(),
            )
        };
        goto_err(fd, big_buf);
        return -1;
    }
    unsafe {
        ASSERT_EQ(
            optlen,
            8,
            b"Unexpected NETLINK_LIST_MEMBERSHIPS value\0".as_ptr() as *const c_char,
        )
    };

    /* Trick bpf_tcp_sock() with IPPROTO_TCP */
    unsafe { close(fd) };
    fd = unsafe { socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_TCP) };
    if !unsafe { ASSERT_OK_FD(fd, b"socket\0".as_ptr() as *const c_char) } {
        goto_err(fd, big_buf);
        return -1;
    }

    /* The BPF prog intercepts this before the kernel sees it, any
     * optlen works. Go with 4 bytes for simplicity.
     */
    buf.u32_ = 1;
    optlen = size_of::<u32>() as libc::socklen_t;
    err = unsafe {
        setsockopt(
            fd,
            SOL_TCP,
            libc::TCP_SAVED_SYN,
            &mut buf as *mut SockoptSkBuf as *const c_void,
            optlen,
        )
    };
    if !unsafe { ASSERT_ERR(err, b"setsockopt(TCP_SAVED_SYN)\0".as_ptr() as *const c_char) } {
        goto_err(fd, big_buf);
        return -1;
    }

    unsafe {
        free(big_buf as *mut c_void);
        close(fd);
    }
    0
}

unsafe fn goto_err(fd: c_int, big_buf: *mut c_char) {
    unsafe {
        free(big_buf as *mut c_void);
        close(fd);
    }
}

unsafe fn run_test(cgroup_fd: c_int) {
    let mut skel: *mut sockopt_sk;

    skel = unsafe { sockopt_sk__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel as *const c_void, b"skel_load\0".as_ptr() as *const c_char) } {
        unsafe { sockopt_sk__destroy(skel) };
        return;
    }

    unsafe {
        (*(*skel).bss).page_size = getpagesize();
    }

    unsafe {
        (*skel).links._setsockopt =
            bpf_program__attach_cgroup((*skel).progs._setsockopt, cgroup_fd);
    }
    if !unsafe {
        ASSERT_OK_PTR(
            (*skel).links._setsockopt as *const c_void,
            b"setsockopt_link\0".as_ptr() as *const c_char,
        )
    } {
        unsafe { sockopt_sk__destroy(skel) };
        return;
    }

    unsafe {
        (*skel).links._getsockopt =
            bpf_program__attach_cgroup((*skel).progs._getsockopt, cgroup_fd);
    }
    if !unsafe {
        ASSERT_OK_PTR(
            (*skel).links._getsockopt as *const c_void,
            b"getsockopt_link\0".as_ptr() as *const c_char,
        )
    } {
        unsafe { sockopt_sk__destroy(skel) };
        return;
    }

    unsafe { ASSERT_OK(getsetsockopt(), b"getsetsockopt\0".as_ptr() as *const c_char) };

    unsafe { sockopt_sk__destroy(skel) };
}

#[no_mangle]
pub unsafe extern "C" fn test_sockopt_sk() {
    let cgroup_fd: c_int;

    cgroup_fd = unsafe { test__join_cgroup(b"/sockopt_sk\0".as_ptr() as *const c_char) };
    if !unsafe {
        ASSERT_GE(
            cgroup_fd,
            0,
            b"join_cgroup /sockopt_sk\0".as_ptr() as *const c_char,
        )
    } {
        return;
    }

    unsafe { run_test(cgroup_fd) };
    unsafe { close(cgroup_fd) };
}
