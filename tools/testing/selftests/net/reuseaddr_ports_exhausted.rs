// SPDX-License-Identifier: GPL-2.0-only
/*
 * Check if we can fully utilize 4-tuples for connect().
 *
 * Rules to bind sockets to the same port when all ephemeral ports are
 * exhausted.
 *
 *   1. if there are TCP_LISTEN sockets on the port, fail to bind.
 *   2. if there are sockets without SO_REUSEADDR, fail to bind.
 *   3. if SO_REUSEADDR is disabled, fail to bind.
 *   4. if SO_REUSEADDR is enabled and SO_REUSEPORT is disabled,
 *        succeed to bind.
 *   5. if SO_REUSEADDR and SO_REUSEPORT are enabled and
 *        there is no socket having the both options and the same EUID,
 *        succeed to bind.
 *   6. fail to bind.
 *
 * Author: Kuniyuki Iwashima <kuniyu@amazon.co.jp>
 */

// C dependencies translated from:
// <arpa/inet.h>, <netinet/in.h>, <sys/socket.h>, <sys/types.h>, <unistd.h>,
// and "kselftest_harness.h".

#[repr(C)]
struct reuse_opts {
    reuseaddr: [libc::c_int; 2],
    reuseport: [libc::c_int; 2],
}

static mut unreusable_opts: [reuse_opts; 12] = [
    reuse_opts {
        reuseaddr: [0, 0],
        reuseport: [0, 0],
    },
    reuse_opts {
        reuseaddr: [0, 0],
        reuseport: [0, 1],
    },
    reuse_opts {
        reuseaddr: [0, 0],
        reuseport: [1, 0],
    },
    reuse_opts {
        reuseaddr: [0, 0],
        reuseport: [1, 1],
    },
    reuse_opts {
        reuseaddr: [0, 1],
        reuseport: [0, 0],
    },
    reuse_opts {
        reuseaddr: [0, 1],
        reuseport: [0, 1],
    },
    reuse_opts {
        reuseaddr: [0, 1],
        reuseport: [1, 0],
    },
    reuse_opts {
        reuseaddr: [0, 1],
        reuseport: [1, 1],
    },
    reuse_opts {
        reuseaddr: [1, 0],
        reuseport: [0, 0],
    },
    reuse_opts {
        reuseaddr: [1, 0],
        reuseport: [0, 1],
    },
    reuse_opts {
        reuseaddr: [1, 0],
        reuseport: [1, 0],
    },
    reuse_opts {
        reuseaddr: [1, 0],
        reuseport: [1, 1],
    },
];

static mut reusable_opts: [reuse_opts; 4] = [
    reuse_opts {
        reuseaddr: [1, 1],
        reuseport: [0, 0],
    },
    reuse_opts {
        reuseaddr: [1, 1],
        reuseport: [0, 1],
    },
    reuse_opts {
        reuseaddr: [1, 1],
        reuseport: [1, 0],
    },
    reuse_opts {
        reuseaddr: [1, 1],
        reuseport: [1, 1],
    },
];

#[allow(non_camel_case_types)]
enum __test_metadata {}

unsafe fn bind_port(
    _metadata: *mut __test_metadata,
    reuseaddr: libc::c_int,
    reuseport: libc::c_int,
) -> libc::c_int {
    let mut local_addr: libc::sockaddr_in = std::mem::zeroed();
    let len = std::mem::size_of_val(&local_addr) as libc::socklen_t;
    let mut ret: libc::c_int;

    let fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
    assert_ne!(-1, fd, "failed to open socket.");

    ret = libc::setsockopt(
        fd,
        libc::SOL_SOCKET,
        libc::SO_REUSEADDR,
        &reuseaddr as *const libc::c_int as *const libc::c_void,
        std::mem::size_of::<libc::c_int>() as libc::socklen_t,
    );
    assert_eq!(0, ret, "failed to setsockopt: SO_REUSEADDR.");

    ret = libc::setsockopt(
        fd,
        libc::SOL_SOCKET,
        libc::SO_REUSEPORT,
        &reuseport as *const libc::c_int as *const libc::c_void,
        std::mem::size_of::<libc::c_int>() as libc::socklen_t,
    );
    assert_eq!(0, ret, "failed to setsockopt: SO_REUSEPORT.");

    local_addr.sin_family = libc::AF_INET as libc::sa_family_t;
    local_addr.sin_addr.s_addr = libc::inet_addr(b"127.0.0.1\0".as_ptr() as *const libc::c_char);
    local_addr.sin_port = 0;

    if libc::bind(
        fd,
        &local_addr as *const libc::sockaddr_in as *const libc::sockaddr,
        len,
    ) == -1
    {
        libc::close(fd);
        return -1;
    }

    fd
}

unsafe fn reuseaddr_ports_exhausted_unreusable(_metadata: *mut __test_metadata) {
    let mut opts: *mut reuse_opts;
    let mut fd: [libc::c_int; 2] = [0; 2];

    for i in 0..12 {
        opts = &mut unreusable_opts[i];

        for j in 0..2 {
            fd[j] = bind_port(_metadata, (*opts).reuseaddr[j], (*opts).reuseport[j]);
        }

        assert_ne!(-1, fd[0], "failed to bind.");
        assert_eq!(-1, fd[1], "should fail to bind.");

        for j in 0..2 {
            if fd[j] != -1 {
                libc::close(fd[j]);
            }
        }
    }
}

unsafe fn reuseaddr_ports_exhausted_reusable_same_euid(_metadata: *mut __test_metadata) {
    let mut opts: *mut reuse_opts;
    let mut fd: [libc::c_int; 2] = [0; 2];

    for i in 0..4 {
        opts = &mut reusable_opts[i];

        for j in 0..2 {
            fd[j] = bind_port(_metadata, (*opts).reuseaddr[j], (*opts).reuseport[j]);
        }

        assert_ne!(-1, fd[0], "failed to bind.");

        if (*opts).reuseport[0] != 0 && (*opts).reuseport[1] != 0 {
            assert_eq!(
                -1, fd[1],
                "should fail to bind because both sockets successfully listened."
            );
        } else {
            assert_ne!(
                -1, fd[1],
                "should succeed to bind to connect to different destinations."
            );
        }

        for j in 0..2 {
            if fd[j] != -1 {
                libc::close(fd[j]);
            }
        }
    }
}

unsafe fn reuseaddr_ports_exhausted_reusable_different_euid(_metadata: *mut __test_metadata) {
    let mut opts: *mut reuse_opts;
    let mut ret: libc::c_int;
    let mut fd: [libc::c_int; 2] = [0; 2];
    let euid: [libc::uid_t; 2] = [10, 20];

    for i in 0..4 {
        opts = &mut reusable_opts[i];

        for j in 0..2 {
            ret = libc::seteuid(euid[j]);
            assert_eq!(0, ret, "failed to seteuid: {}.", euid[j]);

            fd[j] = bind_port(_metadata, (*opts).reuseaddr[j], (*opts).reuseport[j]);

            ret = libc::seteuid(0);
            assert_eq!(0, ret, "failed to seteuid: 0.");
        }

        assert_ne!(-1, fd[0], "failed to bind.");
        assert_ne!(
            -1, fd[1],
            "should succeed to bind because one socket can be bound in each euid."
        );

        if fd[1] != -1 {
            ret = libc::listen(fd[0], 5);
            assert_eq!(0, ret, "failed to listen.");

            ret = libc::listen(fd[1], 5);
            assert_eq!(
                -1, ret,
                "should fail to listen because only one uid reserves the port in TCP_LISTEN."
            );
        }

        for j in 0..2 {
            if fd[j] != -1 {
                libc::close(fd[j]);
            }
        }
    }
}

fn main() {
    unsafe {
        reuseaddr_ports_exhausted_unreusable(std::ptr::null_mut());
        reuseaddr_ports_exhausted_reusable_same_euid(std::ptr::null_mut());
        reuseaddr_ports_exhausted_reusable_different_euid(std::ptr::null_mut());
    }
}
