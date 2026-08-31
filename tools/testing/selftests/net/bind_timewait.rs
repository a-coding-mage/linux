// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates. */

// C dependencies: <sys/socket.h>, <netinet/in.h>, "kselftest_harness.h".

#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [i8; 14],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

pub type socklen_t = u32;
pub type sa_family_t = u16;
pub type in_port_t = u16;
pub type __u32 = u32;

pub const AF_INET: i32 = 2;
pub const SOCK_STREAM: i32 = 1;
pub const INADDR_ANY: __u32 = 0x00000000;
pub const INADDR_LOOPBACK: __u32 = 0x7f000001;
pub const EADDRINUSE: i32 = 98;

unsafe extern "C" {
    fn socket(domain: i32, type_: i32, protocol: i32) -> i32;
    fn bind(sockfd: i32, addr: *const sockaddr, addrlen: socklen_t) -> i32;
    fn listen(sockfd: i32, backlog: i32) -> i32;
    fn getsockname(sockfd: i32, addr: *mut sockaddr, addrlen: *mut socklen_t) -> i32;
    fn connect(sockfd: i32, addr: *const sockaddr, addrlen: socklen_t) -> i32;
    fn accept(sockfd: i32, addr: *mut sockaddr, addrlen: *mut socklen_t) -> i32;
    fn close(fd: i32) -> i32;
    fn htonl(hostlong: u32) -> u32;
    fn __errno_location() -> *mut i32;
}

macro_rules! ASSERT_GT {
    ($left:expr, $right:expr) => {
        assert!(($left) > ($right))
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

#[inline]
unsafe fn errno() -> i32 {
    unsafe { *__errno_location() }
}

// FIXTURE(bind_timewait)
#[repr(C)]
pub struct bind_timewait {
    pub addr: sockaddr_in,
    pub addrlen: socklen_t,
}

// FIXTURE_VARIANT(bind_timewait)
#[repr(C)]
pub struct bind_timewait_variant {
    pub addr_const: __u32,
}

// FIXTURE_VARIANT_ADD(bind_timewait, localhost)
pub static localhost: bind_timewait_variant = bind_timewait_variant {
    addr_const: INADDR_LOOPBACK,
};

// FIXTURE_VARIANT_ADD(bind_timewait, addrany)
pub static addrany: bind_timewait_variant = bind_timewait_variant {
    addr_const: INADDR_ANY,
};

// FIXTURE_SETUP(bind_timewait)
pub unsafe fn bind_timewait_setup(self_: *mut bind_timewait, variant: *const bind_timewait_variant) {
    unsafe {
        (*self_).addr.sin_family = AF_INET as sa_family_t;
        (*self_).addr.sin_port = 0;
        (*self_).addr.sin_addr.s_addr = htonl((*variant).addr_const);
        (*self_).addrlen = core::mem::size_of_val(&(*self_).addr) as socklen_t;
    }
}

// FIXTURE_TEARDOWN(bind_timewait)
pub unsafe fn bind_timewait_teardown(_self: *mut bind_timewait) {}

pub unsafe fn create_timewait_socket(
    _metadata: *mut __test_metadata,
    self_: *mut bind_timewait,
) {
    let server_fd: i32;
    let client_fd: i32;
    let child_fd: i32;
    let mut ret: i32;
    let mut addr: sockaddr_in = unsafe { core::mem::zeroed() };
    let mut addrlen: socklen_t;

    unsafe {
        server_fd = socket(AF_INET, SOCK_STREAM, 0);
    }
    ASSERT_GT!(server_fd, 0);

    unsafe {
        ret = bind(
            server_fd,
            &(*self_).addr as *const sockaddr_in as *const sockaddr,
            (*self_).addrlen,
        );
    }
    ASSERT_EQ!(ret, 0);

    unsafe {
        ret = listen(server_fd, 1);
    }
    ASSERT_EQ!(ret, 0);

    unsafe {
        ret = getsockname(
            server_fd,
            &mut (*self_).addr as *mut sockaddr_in as *mut sockaddr,
            &mut (*self_).addrlen,
        );
    }
    ASSERT_EQ!(ret, 0);

    unsafe {
        client_fd = socket(AF_INET, SOCK_STREAM, 0);
    }
    ASSERT_GT!(client_fd, 0);

    unsafe {
        ret = connect(
            client_fd,
            &(*self_).addr as *const sockaddr_in as *const sockaddr,
            (*self_).addrlen,
        );
    }
    ASSERT_EQ!(ret, 0);

    addrlen = core::mem::size_of_val(&addr) as socklen_t;
    unsafe {
        child_fd = accept(
            server_fd,
            &mut addr as *mut sockaddr_in as *mut sockaddr,
            &mut addrlen,
        );
    }
    ASSERT_GT!(child_fd, 0);

    unsafe {
        close(child_fd);
        close(client_fd);
        close(server_fd);
    }
}

// TEST_F(bind_timewait, 1)
pub unsafe fn bind_timewait_1(_metadata: *mut __test_metadata, self_: *mut bind_timewait) {
    let fd: i32;
    let ret: i32;

    unsafe {
        create_timewait_socket(_metadata, self_);
    }

    unsafe {
        fd = socket(AF_INET, SOCK_STREAM, 0);
    }
    ASSERT_GT!(fd, 0);

    unsafe {
        ret = bind(
            fd,
            &(*self_).addr as *const sockaddr_in as *const sockaddr,
            (*self_).addrlen,
        );
    }
    ASSERT_EQ!(ret, -1);
    ASSERT_EQ!(unsafe { errno() }, EADDRINUSE);

    unsafe {
        close(fd);
    }
}

// TEST_HARNESS_MAIN
