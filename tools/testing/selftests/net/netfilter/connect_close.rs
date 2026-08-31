// SPDX-License-Identifier: GPL-2.0

use std::mem;
use std::ptr;

const PORT: libc::c_uint = 12345;
const RUNTIME: libc::c_uint = 10;

#[repr(C)]
struct opts_t {
    timeout: libc::c_uint,
    port: libc::c_uint,
}

static mut opts: opts_t = opts_t {
    timeout: RUNTIME,
    port: PORT,
};

extern "C" fn handler(sig: libc::c_int) {
    unsafe {
        libc::_exit(if sig == libc::SIGALRM { 0 } else { 1 });
    }
}

unsafe fn set_timeout() {
    let mut action: libc::sigaction = mem::zeroed();
    action.sa_sigaction = handler as usize;

    libc::sigaction(libc::SIGALRM, &action, ptr::null_mut());

    libc::alarm(opts.timeout);
}

unsafe fn do_connect(dst: *const libc::sockaddr_in) {
    let s = libc::socket(libc::AF_INET, libc::SOCK_STREAM, libc::IPPROTO_TCP);

    if s >= 0 {
        libc::fcntl(s, libc::F_SETFL, libc::O_NONBLOCK);
    }

    libc::connect(
        s,
        dst as *const libc::sockaddr,
        mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
    );
    libc::close(s);
}

unsafe fn do_accept(src: *const libc::sockaddr_in) {
    let mut one: libc::c_int = 1;
    let s = libc::socket(libc::AF_INET, libc::SOCK_STREAM, libc::IPPROTO_TCP);

    if s < 0 {
        return;
    }

    libc::setsockopt(
        s,
        libc::SOL_SOCKET,
        libc::SO_REUSEADDR,
        &mut one as *mut libc::c_int as *const libc::c_void,
        mem::size_of_val(&one) as libc::socklen_t,
    );
    libc::setsockopt(
        s,
        libc::SOL_SOCKET,
        libc::SO_REUSEPORT,
        &mut one as *mut libc::c_int as *const libc::c_void,
        mem::size_of_val(&one) as libc::socklen_t,
    );

    libc::bind(
        s,
        src as *const libc::sockaddr,
        mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
    );

    libc::listen(s, 16);

    let c = libc::accept(s, ptr::null_mut(), ptr::null_mut());
    if c >= 0 {
        libc::close(c);
    }

    libc::close(s);
}

unsafe fn accept_loop() -> libc::c_int {
    let mut src: libc::sockaddr_in = mem::zeroed();
    src.sin_family = libc::AF_INET as libc::sa_family_t;
    src.sin_port = libc::htons(opts.port as u16);

    libc::inet_pton(
        libc::AF_INET,
        b"127.0.0.1\0".as_ptr() as *const libc::c_char,
        &mut src.sin_addr as *mut libc::in_addr as *mut libc::c_void,
    );

    set_timeout();

    loop {
        do_accept(&src);
    }

    #[allow(unreachable_code)]
    1
}

unsafe fn connect_loop() -> libc::c_int {
    let mut dst: libc::sockaddr_in = mem::zeroed();
    dst.sin_family = libc::AF_INET as libc::sa_family_t;
    dst.sin_port = libc::htons(opts.port as u16);

    libc::inet_pton(
        libc::AF_INET,
        b"127.0.0.1\0".as_ptr() as *const libc::c_char,
        &mut dst.sin_addr as *mut libc::in_addr as *mut libc::c_void,
    );

    set_timeout();

    loop {
        do_connect(&dst);
    }

    #[allow(unreachable_code)]
    1
}

unsafe fn parse_opts(argc: libc::c_int, argv: *mut *mut libc::c_char) {
    let mut c: libc::c_int;

    loop {
        c = libc::getopt(argc, argv, b"t:p:\0".as_ptr() as *const libc::c_char);
        if c == -1 {
            break;
        }

        match c {
            x if x == b't' as libc::c_int => {
                opts.timeout = libc::atoi(libc::optarg) as libc::c_uint;
            }
            x if x == b'p' as libc::c_int => {
                opts.port = libc::atoi(libc::optarg) as libc::c_uint;
            }
            _ => {}
        }
    }
}

fn main() {
    unsafe {
        let argv_storage: Vec<*mut libc::c_char> = std::env::args()
            .map(|arg| {
                std::ffi::CString::new(arg)
                    .unwrap()
                    .into_raw()
            })
            .collect();
        let mut argv = argv_storage;

        parse_opts(argv.len() as libc::c_int, argv.as_mut_ptr());

        for arg in argv {
            let _ = std::ffi::CString::from_raw(arg);
        }

        let p = libc::fork();
        if p < 0 {
            std::process::exit(111);
        }

        if p > 0 {
            std::process::exit(accept_loop());
        }

        std::process::exit(connect_loop());
    }
}
