// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source. C include dependencies map to libc.

use libc::{
    accept, bind, c_char, c_int, c_ulong, c_void, close, fclose, fopen, fprintf, getopt,
    getsockopt, htons, inet_pton, listen, read, sendto, setsockopt, sockaddr, sockaddr_in6,
    socklen_t, socket, strdup, strlen, strtoul, AF_INET, AF_INET6, FILE, MSG_FASTOPEN, SOCK_STREAM,
    SOL_SOCKET, SOL_TCP, SO_REUSEADDR, TCP_FASTOPEN,
};
use std::mem;
use std::ptr;

const SO_INCOMING_NAPI_ID: c_int = 56;

static mut CFG_SERVER: c_int = 0;
static mut CFG_CLIENT: c_int = 0;
static mut CFG_PORT: c_int = 8000;
static mut CFG_ADDR: sockaddr_in6 = sockaddr_in6 {
    sin6_family: 0,
    sin6_port: 0,
    sin6_flowinfo: 0,
    sin6_addr: libc::in6_addr { s6_addr: [0; 16] },
    sin6_scope_id: 0,
};
static mut CFG_OUTFILE: *mut c_char = ptr::null_mut();

unsafe extern "C" {
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    static mut errno: c_int;
    static mut optarg: *mut c_char;
}

unsafe fn parse_address(str_: *const c_char, port: c_int, sin6: *mut sockaddr_in6) -> c_int {
    let mut ret: c_int;

    (*sin6).sin6_family = AF_INET6 as libc::sa_family_t;
    (*sin6).sin6_port = htons(port as u16);

    ret = inet_pton(
        (*sin6).sin6_family as c_int,
        str_,
        (*sin6).sin6_addr.s6_addr.as_mut_ptr() as *mut c_void,
    );
    if ret != 1 {
        /* fallback to plain IPv4 */
        ret = inet_pton(
            AF_INET,
            str_,
            (*sin6).sin6_addr.s6_addr.as_mut_ptr().add(12) as *mut c_void,
        );
        if ret != 1 {
            return -1;
        }

        /* add ::ffff prefix */
        (*sin6).sin6_addr.s6_addr[0] = 0;
        (*sin6).sin6_addr.s6_addr[1] = 0;
        (*sin6).sin6_addr.s6_addr[2] = 0;
        (*sin6).sin6_addr.s6_addr[3] = 0;
        (*sin6).sin6_addr.s6_addr[4] = 0;
        (*sin6).sin6_addr.s6_addr[5] = 0;
        (*sin6).sin6_addr.s6_addr[6] = 0;
        (*sin6).sin6_addr.s6_addr[7] = 0;
        (*sin6).sin6_addr.s6_addr[8] = 0;
        (*sin6).sin6_addr.s6_addr[9] = 0;
        (*sin6).sin6_addr.s6_addr[10] = 0xff;
        (*sin6).sin6_addr.s6_addr[11] = 0xff;
    }

    0
}

unsafe fn run_server() {
    let mut qlen: c_ulong = 32;
    let mut fd: c_int;
    let mut opt: c_int;
    let mut connfd: c_int;
    let mut len: socklen_t;
    let mut buf: [c_char; 64] = [0; 64];
    let mut outfile: *mut FILE;

    outfile = fopen(CFG_OUTFILE, c"w".as_ptr());
    if outfile.is_null() {
        error(1, errno, c"fopen() outfile".as_ptr());
    }

    fd = socket(AF_INET6, SOCK_STREAM, 0);
    if fd == -1 {
        error(1, errno, c"socket()".as_ptr());
    }

    opt = 1;
    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_REUSEADDR,
        &mut opt as *mut c_int as *const c_void,
        mem::size_of_val(&opt) as socklen_t,
    ) < 0
    {
        error(1, errno, c"setsockopt(SO_REUSEADDR)".as_ptr());
    }

    if setsockopt(
        fd,
        SOL_TCP,
        TCP_FASTOPEN,
        &mut qlen as *mut c_ulong as *const c_void,
        mem::size_of_val(&qlen) as socklen_t,
    ) < 0
    {
        error(1, errno, c"setsockopt(TCP_FASTOPEN)".as_ptr());
    }

    if bind(
        fd,
        &raw const CFG_ADDR as *const sockaddr,
        mem::size_of_val(&CFG_ADDR) as socklen_t,
    ) < 0
    {
        error(1, errno, c"bind()".as_ptr());
    }

    if listen(fd, 5) < 0 {
        error(1, errno, c"listen()".as_ptr());
    }

    len = mem::size_of_val(&CFG_ADDR) as socklen_t;
    connfd = accept(fd, &raw mut CFG_ADDR as *mut sockaddr, &mut len);
    if connfd < 0 {
        error(1, errno, c"accept()".as_ptr());
    }

    len = mem::size_of_val(&opt) as socklen_t;
    if getsockopt(
        connfd,
        SOL_SOCKET,
        SO_INCOMING_NAPI_ID,
        &mut opt as *mut c_int as *mut c_void,
        &mut len,
    ) < 0
    {
        error(1, errno, c"getsockopt(SO_INCOMING_NAPI_ID)".as_ptr());
    }

    if read(connfd, buf.as_mut_ptr() as *mut c_void, 64) < 0 {
        error(1, errno, c"read()".as_ptr());
    }

    if fprintf(outfile, c"%d\n".as_ptr(), opt) < 0 {
        error(1, errno, c"fprintf()".as_ptr());
    }

    fclose(outfile);
    close(connfd);
    close(fd);
}

unsafe fn run_client() {
    let mut fd: c_int;
    let mut ret: c_int;
    let msg: *mut c_char = c"Hello, world!".as_ptr() as *mut c_char;

    fd = socket(AF_INET6, SOCK_STREAM, 0);
    if fd == -1 {
        error(1, errno, c"socket()".as_ptr());
    }

    ret = sendto(
        fd,
        msg as *const c_void,
        strlen(msg),
        MSG_FASTOPEN,
        &raw const CFG_ADDR as *const sockaddr,
        mem::size_of_val(&CFG_ADDR) as socklen_t,
    ) as c_int;
    if ret < 0 {
        error(1, errno, c"sendto()".as_ptr());
    }

    close(fd);
}

unsafe fn usage(filepath: *const c_char) {
    error(
        1,
        0,
        c"Usage: %s (-s|-c) -h<server_ip> -p<port> -o<outfile> ".as_ptr(),
        filepath,
    );
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let addr6: *mut sockaddr_in6 = &raw mut CFG_ADDR;
    let mut addr: *mut c_char = ptr::null_mut();
    let mut ret: c_int;
    let mut c: c_int;

    if argc <= 1 {
        usage(*argv.add(0));
    }

    loop {
        c = getopt(argc, argv, c"sch:p:o:".as_ptr());
        if c == -1 {
            break;
        }

        match c as u8 as char {
            's' => {
                if CFG_CLIENT != 0 {
                    error(1, 0, c"Pass one of -s or -c".as_ptr());
                }
                CFG_SERVER = 1;
            }
            'c' => {
                if CFG_SERVER != 0 {
                    error(1, 0, c"Pass one of -s or -c".as_ptr());
                }
                CFG_CLIENT = 1;
            }
            'h' => {
                addr = optarg;
            }
            'p' => {
                CFG_PORT = strtoul(optarg, ptr::null_mut(), 0) as c_int;
            }
            'o' => {
                CFG_OUTFILE = strdup(optarg);
                if CFG_OUTFILE.is_null() {
                    error(1, 0, c"outfile invalid".as_ptr());
                }
            }
            _ => {}
        }
    }

    if CFG_SERVER != 0 && !addr.is_null() {
        error(1, 0, c"Server cannot have -h specified".as_ptr());
    }

    ptr::write_bytes(addr6 as *mut c_void, 0, mem::size_of_val(&*addr6));
    (*addr6).sin6_family = AF_INET6 as libc::sa_family_t;
    (*addr6).sin6_port = htons(CFG_PORT as u16);
    (*addr6).sin6_addr = libc::in6_addr { s6_addr: [0; 16] };
    if !addr.is_null() {
        ret = parse_address(addr, CFG_PORT, addr6);
        if ret != 0 {
            error(1, 0, c"Client address parse error: %s".as_ptr(), addr);
        }
    }
}

fn main() {
    unsafe {
        let args: Vec<*mut c_char> = std::env::args()
            .map(|arg| {
                let cstr = std::ffi::CString::new(arg).unwrap();
                cstr.into_raw()
            })
            .collect();
        let mut argv = args;

        parse_opts(argv.len() as c_int, argv.as_mut_ptr());

        if CFG_SERVER != 0 {
            run_server();
        } else if CFG_CLIENT != 0 {
            run_client();
        }

        for arg in argv {
            let _ = std::ffi::CString::from_raw(arg);
        }
    }
}
