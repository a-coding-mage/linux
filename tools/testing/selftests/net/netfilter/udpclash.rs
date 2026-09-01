// SPDX-License-Identifier: GPL-2.0

/* Usage: ./udpclash <IP> <PORT>
 *
 * Emit THREAD_COUNT UDP packets sharing the same saddr:daddr pair.
 *
 * This mimics DNS resolver libraries that emit A and AAAA requests
 * in parallel.
 *
 * This exercises conntrack clash resolution logic added and later
 * refined in
 *
 *  71d8c47fc653 ("netfilter: conntrack: introduce clash resolution on insertion race")
 *  ed07d9a021df ("netfilter: nf_conntrack: resolve clash for matching conntracks")
 *  6a757c07e51f ("netfilter: conntrack: allow insertion of clashing entries")
 */

use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

const THREAD_COUNT: usize = 128;

type socklen_t = u32;
type ssize_t = isize;
type size_t = usize;
type pthread_t = usize;

const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;
const SOCK_CLOEXEC: c_int = 0o2000000;
const SOCK_NONBLOCK: c_int = 0o0004000;
const IPPROTO_UDP: c_int = 17;
const MSG_NOSIGNAL: c_int = 0x4000;

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct pthread_attr_t {
    __size: [u64; 7],
    __align: i64,
}

struct thread_args {
    si_remote: *const sockaddr_in,
    sockfd: c_int,
}

static mut wait: c_int = 1;

unsafe extern "C" {
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn perror(s: *const c_char);
    fn fputs(s: *const c_char, stream: *mut c_void) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn strlen(s: *const c_char) -> size_t;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn sendto(
        sockfd: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
        dest_addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> ssize_t;
    fn recvfrom(
        sockfd: c_int,
        buf: *mut c_void,
        len: size_t,
        flags: c_int,
        src_addr: *mut sockaddr,
        addrlen: *mut socklen_t,
    ) -> ssize_t;
    fn usleep(usec: c_uint) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;
    fn atoi(nptr: *const c_char) -> c_int;
    fn inet_addr(cp: *const c_char) -> u32;
    fn inet_ntop(af: c_int, src: *const c_void, dst: *mut c_char, size: socklen_t) -> *const c_char;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    static mut stderr: *mut c_void;
}

unsafe extern "C" fn thread_main(varg: *mut c_void) -> *mut c_void {
    let si_remote: *const sockaddr_in;
    let args: *const thread_args = varg as *const thread_args;
    static MSG: &[u8; 4] = b"foo\0";

    si_remote = unsafe { (*args).si_remote };

    while unsafe { ptr::read_volatile(ptr::addr_of!(wait)) } == 1 {}

    if unsafe {
        sendto(
            (*args).sockfd,
            MSG.as_ptr() as *const c_void,
            strlen(MSG.as_ptr() as *const c_char),
            MSG_NOSIGNAL,
            si_remote as *const sockaddr,
            mem::size_of_val(&*si_remote) as socklen_t,
        )
    } < 0
    {
        unsafe { exit(111) };
    }

    varg
}

unsafe fn run_test(fd: c_int, si_remote: *const sockaddr_in) -> c_int {
    let mut thread_args = thread_args {
        si_remote,
        sockfd: fd,
    };
    let tid: *mut pthread_t =
        unsafe { calloc(THREAD_COUNT, mem::size_of::<pthread_t>()) as *mut pthread_t };
    let mut repl_count: c_uint = 0;
    let mut timeout: c_uint = 0;
    let mut i: c_int;

    if tid.is_null() {
        unsafe { perror(c"calloc".as_ptr()) };
        return 1;
    }

    i = 0;
    while i < THREAD_COUNT as c_int {
        let err: c_int = unsafe {
            pthread_create(
                tid.add(i as usize),
                ptr::null(),
                Some(thread_main),
                &mut thread_args as *mut thread_args as *mut c_void,
            )
        };

        if err != 0 {
            unsafe { perror(c"pthread_create".as_ptr()) };
            unsafe { exit(1) };
        }

        i += 1;
    }

    unsafe { ptr::write_volatile(ptr::addr_of_mut!(wait), 0) };

    i = 0;
    while i < THREAD_COUNT as c_int {
        unsafe { pthread_join(*tid.add(i as usize), ptr::null_mut()) };
        i += 1;
    }

    while repl_count < THREAD_COUNT as c_uint {
        let mut si_repl: sockaddr_in = unsafe { mem::zeroed() };
        let mut si_repl_len: socklen_t = mem::size_of_val(&si_repl) as socklen_t;
        let mut repl: [c_char; 512] = [0; 512];
        let ret: ssize_t;

        ret = unsafe {
            recvfrom(
                fd,
                repl.as_mut_ptr() as *mut c_void,
                mem::size_of_val(&repl),
                MSG_NOSIGNAL,
                &mut si_repl as *mut sockaddr_in as *mut sockaddr,
                &mut si_repl_len,
            )
        };
        if ret < 0 {
            if {
                let old = timeout;
                timeout = timeout.wrapping_add(1);
                old
            } > 5000
            {
                unsafe {
                    fputs(
                        c"timed out while waiting for reply from thread\n".as_ptr(),
                        stderr,
                    )
                };
                break;
            }

            /* give reply time to pass though the stack */
            unsafe { usleep(1000) };
            continue;
        }

        if si_repl_len != mem::size_of_val(&*si_remote) as socklen_t {
            unsafe {
                fprintf(
                    stderr,
                    c"warning: reply has unexpected repl_len %d vs %d\n".as_ptr(),
                    si_repl_len as c_int,
                    mem::size_of_val(&si_repl) as c_int,
                )
            };
        } else if unsafe { (*si_remote).sin_addr.s_addr } != si_repl.sin_addr.s_addr
            || unsafe { (*si_remote).sin_port } != si_repl.sin_port
        {
            let mut a: [c_char; 64] = [0; 64];
            let mut b: [c_char; 64] = [0; 64];

            unsafe {
                inet_ntop(
                    AF_INET,
                    ptr::addr_of!((*si_remote).sin_addr) as *const c_void,
                    a.as_mut_ptr(),
                    mem::size_of_val(&a) as socklen_t,
                )
            };
            unsafe {
                inet_ntop(
                    AF_INET,
                    ptr::addr_of!(si_repl.sin_addr) as *const c_void,
                    b.as_mut_ptr(),
                    mem::size_of_val(&b) as socklen_t,
                )
            };

            unsafe {
                fprintf(
                    stderr,
                    c"reply from wrong source: want %s:%d got %s:%d\n".as_ptr(),
                    a.as_ptr(),
                    ntohs((*si_remote).sin_port) as c_int,
                    b.as_ptr(),
                    ntohs(si_repl.sin_port) as c_int,
                )
            };
        }

        repl_count = repl_count.wrapping_add(1);
    }

    unsafe {
        printf(
            c"got %d of %d replies\n".as_ptr(),
            repl_count as c_int,
            THREAD_COUNT as c_int,
        )
    };

    unsafe { free(tid as *mut c_void) };

    if repl_count == THREAD_COUNT as c_uint {
        0
    } else {
        1
    }
}

fn main() {
    let argv_storage: Vec<*mut c_char> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
        .collect();
    let argc: c_int = argv_storage.len() as c_int;
    let argv: *const *mut c_char = argv_storage.as_ptr();
    let mut si_local = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut si_remote = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let fd: c_int;
    let ret: c_int;

    unsafe {
        if argc < 3 {
            fputs(c"Usage: send_udp <daddr> <dport>\n".as_ptr(), stderr);
            std::process::exit(1);
        }

        si_remote.sin_port = htons(atoi(*argv.add(2)) as u16);
        si_remote.sin_addr.s_addr = inet_addr(*argv.add(1));

        fd = socket(
            AF_INET,
            SOCK_DGRAM | SOCK_CLOEXEC | SOCK_NONBLOCK,
            IPPROTO_UDP,
        );
        if fd < 0 {
            perror(c"socket".as_ptr());
            std::process::exit(1);
        }

        if bind(
            fd,
            &mut si_local as *mut sockaddr_in as *const sockaddr,
            mem::size_of_val(&si_local) as socklen_t,
        ) < 0
        {
            perror(c"bind".as_ptr());
            std::process::exit(1);
        }

        ret = run_test(fd, &si_remote);

        close(fd);

        for arg in argv_storage {
            drop(std::ffi::CString::from_raw(arg));
        }

        std::process::exit(ret);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
