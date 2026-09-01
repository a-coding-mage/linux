// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, "udp_limit.skel.h",
// <sys/types.h>, and <sys/socket.h>.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct udp_limit {
    pub links: udp_limit_links,
    pub progs: udp_limit_progs,
    pub bss: *mut udp_limit_bss,
}

#[repr(C)]
pub struct udp_limit_links {
    pub sock: *mut bpf_link,
    pub sock_release: *mut bpf_link,
}

#[repr(C)]
pub struct udp_limit_progs {
    pub sock: *mut bpf_program,
    pub sock_release: *mut bpf_program,
}

#[repr(C)]
pub struct udp_limit_bss {
    pub invocations: c_int,
    pub in_use: c_int,
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

pub const AF_INET: c_int = 2;
pub const SOCK_DGRAM: c_int = 2;

unsafe extern "C" {
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn udp_limit__open_and_load() -> *mut udp_limit;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn udp_limit__destroy(obj: *mut udp_limit);
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_LT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

pub unsafe fn test_udp_limit() {
    let mut skel: *mut udp_limit;
    let mut fd1: c_int = -1;
    let mut fd2: c_int = -1;
    let cgroup_fd: c_int;

    cgroup_fd = unsafe { test__join_cgroup(c"/udp_limit".as_ptr()) };
    if !unsafe { ASSERT_GE(cgroup_fd, 0, c"cg-join".as_ptr()) } {
        return;
    }

    skel = unsafe { udp_limit__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel as *const c_void, c"skel-load".as_ptr()) } {
        unsafe { close(cgroup_fd) };
        return;
    }

    unsafe {
        (*skel).links.sock = bpf_program__attach_cgroup((*skel).progs.sock, cgroup_fd);
    }
    if !unsafe { ASSERT_OK_PTR((*skel).links.sock as *const c_void, c"cg_attach_sock".as_ptr()) } {
        unsafe {
            if fd1 >= 0 {
                close(fd1);
            }
            if fd2 >= 0 {
                close(fd2);
            }
            udp_limit__destroy(skel);
            close(cgroup_fd);
        }
        return;
    }
    unsafe {
        (*skel).links.sock_release =
            bpf_program__attach_cgroup((*skel).progs.sock_release, cgroup_fd);
    }
    if !unsafe {
        ASSERT_OK_PTR(
            (*skel).links.sock_release as *const c_void,
            c"cg_attach_sock_release".as_ptr(),
        )
    } {
        unsafe {
            if fd1 >= 0 {
                close(fd1);
            }
            if fd2 >= 0 {
                close(fd2);
            }
            udp_limit__destroy(skel);
            close(cgroup_fd);
        }
        return;
    }

    /* BPF program enforces a single UDP socket per cgroup,
     * verify that.
     */
    fd1 = unsafe { socket(AF_INET, SOCK_DGRAM, 0) };
    if !unsafe { ASSERT_GE(fd1, 0, c"socket(fd1)".as_ptr()) } {
        unsafe {
            if fd1 >= 0 {
                close(fd1);
            }
            if fd2 >= 0 {
                close(fd2);
            }
            udp_limit__destroy(skel);
            close(cgroup_fd);
        }
        return;
    }

    fd2 = unsafe { socket(AF_INET, SOCK_DGRAM, 0) };
    if !unsafe { ASSERT_LT(fd2, 0, c"socket(fd2)".as_ptr()) } {
        unsafe {
            if fd1 >= 0 {
                close(fd1);
            }
            if fd2 >= 0 {
                close(fd2);
            }
            udp_limit__destroy(skel);
            close(cgroup_fd);
        }
        return;
    }

    /* We can reopen again after close. */
    unsafe { close(fd1) };
    fd1 = -1;

    fd1 = unsafe { socket(AF_INET, SOCK_DGRAM, 0) };
    if !unsafe { ASSERT_GE(fd1, 0, c"socket(fd1-again)".as_ptr()) } {
        unsafe {
            if fd1 >= 0 {
                close(fd1);
            }
            if fd2 >= 0 {
                close(fd2);
            }
            udp_limit__destroy(skel);
            close(cgroup_fd);
        }
        return;
    }

    /* Make sure the program was invoked the expected
     * number of times:
     * - open fd1           - BPF_CGROUP_INET_SOCK_CREATE
     * - attempt to openfd2 - BPF_CGROUP_INET_SOCK_CREATE
     * - close fd1          - BPF_CGROUP_INET_SOCK_RELEASE
     * - open fd1 again     - BPF_CGROUP_INET_SOCK_CREATE
     */
    if !unsafe { ASSERT_EQ((*(*skel).bss).invocations, 4, c"bss-invocations".as_ptr()) } {
        unsafe {
            if fd1 >= 0 {
                close(fd1);
            }
            if fd2 >= 0 {
                close(fd2);
            }
            udp_limit__destroy(skel);
            close(cgroup_fd);
        }
        return;
    }

    /* We should still have a single socket in use */
    if !unsafe { ASSERT_EQ((*(*skel).bss).in_use, 1, c"bss-in_use".as_ptr()) } {
        unsafe {
            if fd1 >= 0 {
                close(fd1);
            }
            if fd2 >= 0 {
                close(fd2);
            }
            udp_limit__destroy(skel);
            close(cgroup_fd);
        }
        return;
    }

    unsafe {
        if fd1 >= 0 {
            close(fd1);
        }
        if fd2 >= 0 {
            close(fd2);
        }
        udp_limit__destroy(skel);
        close(cgroup_fd);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
