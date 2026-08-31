// SPDX-License-Identifier: GPL-2.0
/*
 * This times how long it takes to bind to a port when the port already
 * has multiple sockets in its bhash table.
 *
 * In the setup(), we populate the port's bhash table with
 * MAX_THREADS * MAX_CONNECTIONS number of entries.
 */

use core::ffi::{c_char, c_int, c_void};

const MAX_THREADS: usize = 600;
const MAX_CONNECTIONS: usize = 40;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_REUSEPORT: c_int = 15;
const CLOCKS_PER_SEC: clock_t = 1_000_000;

type socklen_t = u32;
type clock_t = i64;
type pthread_t = usize;

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct addrinfo {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: socklen_t,
    ai_addr: *mut sockaddr,
    ai_canonname: *mut c_char,
    ai_next: *mut addrinfo,
}

unsafe extern "C" {
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn perror(s: *const c_char);
    fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn freeaddrinfo(res: *mut addrinfo);
    fn close(fildes: c_int) -> c_int;
    fn pthread_exit(value_ptr: *mut c_void) -> !;
    fn listen(socket: c_int, backlog: c_int) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, value_ptr: *mut *mut c_void) -> c_int;
    fn clock() -> clock_t;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

static SETUP_ADDR_V6: &[u8] = b"::1\0";
static SETUP_ADDR_V4: &[u8] = b"127.0.0.1\0";
static mut SETUP_ADDR: *const c_char = core::ptr::null();
static mut BIND_ADDR: *const c_char = core::ptr::null();
static mut PORT: *const c_char = core::ptr::null();
static mut USE_V6: bool = false;
static mut RET: c_int = 0;

static mut FD_ARRAY: [[c_int; MAX_CONNECTIONS]; MAX_THREADS] =
    [[0; MAX_CONNECTIONS]; MAX_THREADS];

unsafe fn bind_socket(opt: c_int, addr: *const c_char) -> c_int {
    let mut res: *mut addrinfo = core::ptr::null_mut();
    let mut hint: addrinfo = core::mem::zeroed();
    let sock_fd: c_int;
    let mut reuse: c_int = 1;
    let mut err: c_int;
    let domain: c_int = if USE_V6 { AF_INET6 } else { AF_INET };

    sock_fd = socket(domain, SOCK_STREAM, 0);
    if sock_fd < 0 {
        perror(c"socket fd err".as_ptr());
        return sock_fd;
    }

    hint.ai_family = domain;
    hint.ai_socktype = SOCK_STREAM;

    err = getaddrinfo(addr, PORT, &hint, &mut res);
    if err != 0 {
        perror(c"getaddrinfo failed".as_ptr());
        close(sock_fd);
        return err;
    }

    if opt != 0 {
        err = setsockopt(
            sock_fd,
            SOL_SOCKET,
            opt,
            (&mut reuse as *mut c_int).cast::<c_void>(),
            core::mem::size_of_val(&reuse) as socklen_t,
        );
        if err != 0 {
            perror(c"setsockopt failed".as_ptr());
            freeaddrinfo(res);
            close(sock_fd);
            return err;
        }
    }

    err = bind(sock_fd, (*res).ai_addr, (*res).ai_addrlen);
    if err != 0 {
        perror(c"failed to bind to port".as_ptr());
        freeaddrinfo(res);
        close(sock_fd);
        return err;
    }
    freeaddrinfo(res);
    sock_fd
}

unsafe extern "C" fn setup(arg: *mut c_void) -> *mut c_void {
    let mut sock_fd: c_int;
    let mut i: c_int;
    let array = arg.cast::<c_int>();

    i = 0;
    while i < MAX_CONNECTIONS as c_int {
        sock_fd = bind_socket(SO_REUSEPORT, SETUP_ADDR);
        if sock_fd < 0 {
            RET = sock_fd;
            pthread_exit((&mut RET as *mut c_int).cast::<c_void>());
        }
        *array.add(i as usize) = sock_fd;
        i += 1;
    }

    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    let listener_fd: c_int;
    let mut sock_fd: c_int = 0;
    let mut i: c_int;
    let mut j: c_int;
    let mut tid: [pthread_t; MAX_THREADS] = [0; MAX_THREADS];
    let begin: clock_t;
    let end: clock_t;

    if argc != 4 {
        printf(c"Usage: listener <port> <ipv6 | ipv4> <bind-addr>\n".as_ptr());
        return -1;
    }

    PORT = *argv.add(1);
    USE_V6 = strcmp(*argv.add(2), c"ipv6".as_ptr()) == 0;
    BIND_ADDR = *argv.add(3);

    SETUP_ADDR = if USE_V6 {
        SETUP_ADDR_V6.as_ptr().cast::<c_char>()
    } else {
        SETUP_ADDR_V4.as_ptr().cast::<c_char>()
    };

    listener_fd = bind_socket(SO_REUSEPORT, SETUP_ADDR);
    if listen(listener_fd, 100) < 0 {
        perror(c"listen failed".as_ptr());
        return -1;
    }

    /* Set up threads to populate the bhash table entry for the port */
    i = 0;
    while i < MAX_THREADS as c_int {
        pthread_create(
            &mut tid[i as usize],
            core::ptr::null(),
            setup,
            FD_ARRAY[i as usize].as_mut_ptr().cast::<c_void>(),
        );
        i += 1;
    }

    i = 0;
    while i < MAX_THREADS as c_int {
        pthread_join(tid[i as usize], core::ptr::null_mut());
        i += 1;
    }

    if RET != 0 {
        close(listener_fd);
        i = 0;
        while i < MAX_THREADS as c_int {
            j = 0;
            while i < MAX_THREADS as c_int {
                close(FD_ARRAY[i as usize][j as usize]);
                i += 1;
            }
            i += 1;
        }
        return 0;
    }

    begin = clock();

    /* Bind to the same port on a different address */
    sock_fd = bind_socket(0, BIND_ADDR);
    if sock_fd < 0 {
        close(listener_fd);
        i = 0;
        while i < MAX_THREADS as c_int {
            j = 0;
            while i < MAX_THREADS as c_int {
                close(FD_ARRAY[i as usize][j as usize]);
                i += 1;
            }
            i += 1;
        }
        return 0;
    }

    end = clock();

    printf(
        c"time spent = %f\n".as_ptr(),
        (end - begin) as f64 / CLOCKS_PER_SEC as f64,
    );

    /* clean up */
    close(sock_fd);

    close(listener_fd);
    i = 0;
    while i < MAX_THREADS as c_int {
        j = 0;
        while i < MAX_THREADS as c_int {
            close(FD_ARRAY[i as usize][j as usize]);
            i += 1;
        }
        i += 1;
    }

    0
}
