// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type ssize_t = isize;
type socklen_t = u32;

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub union epoll_data {
    pub ptr: *mut c_void,
    pub fd: c_int,
    pub u32_: u32,
    pub u64_: u64,
}

#[repr(C)]
pub struct epoll_event {
    pub events: u32,
    pub data: epoll_data,
}

/* The below ifdef blob is required because:
 *
 * - sys/epoll.h does not (yet) have the ioctl definitions included. So,
 *   systems with older glibcs will not have them available. However,
 *   sys/epoll.h does include the type definition for epoll_data, which is
 *   needed by the user program (e.g. epoll_event.data.fd)
 *
 * - linux/eventpoll.h does not define the epoll_data type, it is simply an
 *   opaque __u64. It does, however, include the ioctl definition.
 *
 * Including both headers is impossible (types would be redefined), so I've
 * opted instead to take sys/epoll.h, and include the blob below.
 *
 * Someday, when glibc is globally up to date, the blob below can be removed.
 */
#[repr(C)]
pub struct epoll_params {
    pub busy_poll_usecs: u32,
    pub busy_poll_budget: u16,
    pub prefer_busy_poll: u8,

    /* pad the struct to a multiple of 64bits */
    pub __pad: u8,
}

const EPOLL_IOC_TYPE: c_uint = 0x8A;

const IOC_NRBITS: c_uint = 8;
const IOC_TYPEBITS: c_uint = 8;
const IOC_SIZEBITS: c_uint = 14;
const IOC_DIRBITS: c_uint = 2;
const IOC_NRSHIFT: c_uint = 0;
const IOC_TYPESHIFT: c_uint = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: c_uint = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: c_uint = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: c_uint = 1;
const IOC_READ: c_uint = 2;

const fn ioc(dir: c_uint, type_: c_uint, nr: c_uint, size: c_uint) -> c_ulong {
    ((dir << IOC_DIRSHIFT)
        | (type_ << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)) as c_ulong
}

const EPIOCSPARAMS: c_ulong = ioc(
    IOC_WRITE,
    EPOLL_IOC_TYPE,
    0x01,
    mem::size_of::<epoll_params>() as c_uint,
);
const EPIOCGPARAMS: c_ulong = ioc(
    IOC_READ,
    EPOLL_IOC_TYPE,
    0x02,
    mem::size_of::<epoll_params>() as c_uint,
);

const INADDR_ANY: u32 = 0;
const ULLONG_MAX: u64 = u64::MAX;
const UINT32_MAX: u64 = u32::MAX as u64;
const UINT16_MAX: u64 = u16::MAX as u64;
const INT_MAX: u64 = c_int::MAX as u64;
const INT32_MAX: u64 = i32::MAX as u64;

const F_GETFL: c_int = 3;
const F_SETFL: c_int = 4;
const O_NONBLOCK: c_int = 0o4000;
const O_WRONLY: c_int = 0o1;
const O_CREAT: c_int = 0o100;

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const IPPROTO_TCP: c_int = 6;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;

const EPOLLIN: u32 = 0x001;
const EPOLLOUT: u32 = 0x004;
const EPOLL_CTL_ADD: c_int = 1;
const EPOLL_CTL_DEL: c_int = 2;
const EPOLLET: u32 = 1u32 << 31;
const EPOLLHUP: u32 = 0x010;
const EPOLLRDHUP: u32 = 0x2000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_napi_threaded {
    NETDEV_NAPI_THREADED_DISABLED = 0,
}

#[repr(C)]
pub struct ynl_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ynl_error {
    pub msg: *const c_char,
}

#[repr(C)]
pub struct ynl_family {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netdev_napi_get_list_obj_present {
    pub id: bool,
}

#[repr(C)]
pub struct netdev_napi_get_list_obj {
    pub _present: netdev_napi_get_list_obj_present,
    pub id: u32,
}

#[repr(C)]
pub struct netdev_napi_get_list {
    pub obj: netdev_napi_get_list_obj,
}

#[repr(C)]
pub struct netdev_napi_get_req_dump {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netdev_napi_set_req {
    _private: [u8; 0],
}

unsafe extern "C" {
    static ynl_netdev_family: ynl_family;
    static mut optarg: *mut c_char;
    static mut optind: c_int;

    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn __errno_location() -> *mut c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn inet_aton(cp: *const c_char, inp: *mut in_addr) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn open(pathname: *const c_char, flags: c_int, mode: c_uint) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn epoll_create1(flags: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn epoll_wait(
        epfd: c_int,
        events: *mut epoll_event,
        maxevents: c_int,
        timeout: c_int,
    ) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn ynl_sock_create(family: *const ynl_family, yerr: *mut ynl_error) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);
    fn netdev_napi_get_req_dump_alloc() -> *mut netdev_napi_get_req_dump;
    fn netdev_napi_get_req_dump_set_ifindex(req: *mut netdev_napi_get_req_dump, ifindex: u32);
    fn netdev_napi_get_dump(
        ys: *mut ynl_sock,
        req: *mut netdev_napi_get_req_dump,
    ) -> *mut netdev_napi_get_list;
    fn netdev_napi_set_req_alloc() -> *mut netdev_napi_set_req;
    fn netdev_napi_set_req_set_id(req: *mut netdev_napi_set_req, id: u32);
    fn netdev_napi_set_req_set_defer_hard_irqs(req: *mut netdev_napi_set_req, defer_hard_irqs: u32);
    fn netdev_napi_set_req_set_gro_flush_timeout(
        req: *mut netdev_napi_set_req,
        gro_flush_timeout: u64,
    );
    fn netdev_napi_set_req_set_irq_suspend_timeout(
        req: *mut netdev_napi_set_req,
        irq_suspend_timeout: u64,
    );
    fn netdev_napi_set_req_set_threaded(
        req: *mut netdev_napi_set_req,
        threaded: netdev_napi_threaded,
    );
    fn netdev_napi_set(ys: *mut ynl_sock, req: *mut netdev_napi_set_req) -> c_int;
    fn netdev_napi_get_list_free(list: *mut netdev_napi_get_list);
    fn netdev_napi_get_req_dump_free(req: *mut netdev_napi_get_req_dump);
    fn netdev_napi_set_req_free(req: *mut netdev_napi_set_req);
}

static mut cfg_port: u16 = 8000;
static mut cfg_bind_addr: in_addr = in_addr { s_addr: INADDR_ANY };
static mut cfg_outfile: *mut c_char = ptr::null_mut();
static mut cfg_max_events: c_int = 8;
static mut cfg_ifindex: u32 = 0;

/* busy poll params */
static mut cfg_busy_poll_usecs: u32 = 0;
static mut cfg_busy_poll_budget: u16 = 0;
static mut cfg_prefer_busy_poll: u8 = 0;

/* NAPI params */
static mut cfg_defer_hard_irqs: u32 = 0;
static mut cfg_gro_flush_timeout: u64 = 0;
static mut cfg_irq_suspend_timeout: u64 = 0;
static mut cfg_napi_threaded_poll: netdev_napi_threaded =
    netdev_napi_threaded::NETDEV_NAPI_THREADED_DISABLED;

unsafe fn usage(filepath: *const c_char) {
    error(
        1,
        0,
        b"Usage: %s -p<port> -b<addr> -m<max_events> -u<busy_poll_usecs> -P<prefer_busy_poll> -g<busy_poll_budget> -o<outfile> -d<defer_hard_irqs> -r<gro_flush_timeout> -s<irq_suspend_timeout> -t<napi_threaded_poll> -i<ifindex>\0"
            .as_ptr() as *const c_char,
        filepath,
    );
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut tmp: u64 = 0;
    let mut ret: c_int;
    let mut c: c_int;

    if argc <= 1 {
        usage(*argv.add(0));
    }

    loop {
        c = getopt(argc, argv, b"p:m:b:u:P:g:o:d:r:s:i:t:\0".as_ptr() as *const c_char);
        if c == -1 {
            break;
        }

        /* most options take integer values, except o and b, so reduce
         * code duplication a bit for the common case by calling
         * strtoull here and leave bounds checking and casting per
         * option below.
         */
        if c != b'o' as c_int && c != b'b' as c_int {
            tmp = strtoull(optarg, ptr::null_mut(), 0);
        }

        match c {
            x if x == b'u' as c_int => {
                if tmp == ULLONG_MAX || tmp > UINT32_MAX {
                    error(1, 34, b"busy_poll_usecs too large\0".as_ptr() as *const c_char);
                }

                cfg_busy_poll_usecs = tmp as u32;
            }
            x if x == b'P' as c_int => {
                if tmp == ULLONG_MAX || tmp > 1 {
                    error(
                        1,
                        34,
                        b"prefer busy poll should be 0 or 1\0".as_ptr() as *const c_char,
                    );
                }

                cfg_prefer_busy_poll = tmp as u8;
            }
            x if x == b'g' as c_int => {
                if tmp == ULLONG_MAX || tmp > UINT16_MAX {
                    error(
                        1,
                        34,
                        b"busy poll budget must be [0, UINT16_MAX]\0".as_ptr() as *const c_char,
                    );
                }

                cfg_busy_poll_budget = tmp as u16;
            }
            x if x == b'p' as c_int => {
                if tmp == ULLONG_MAX || tmp > UINT16_MAX {
                    error(1, 34, b"port must be <= 65535\0".as_ptr() as *const c_char);
                }

                cfg_port = tmp as u16;
            }
            x if x == b'b' as c_int => {
                ret = inet_aton(optarg, &mut cfg_bind_addr);
                if ret == 0 {
                    error(
                        1,
                        *__errno_location(),
                        b"bind address %s invalid\0".as_ptr() as *const c_char,
                        optarg,
                    );
                }
            }
            x if x == b'o' as c_int => {
                cfg_outfile = strdup(optarg);
                if cfg_outfile.is_null() {
                    error(1, 0, b"outfile invalid\0".as_ptr() as *const c_char);
                }
            }
            x if x == b'm' as c_int => {
                if tmp == ULLONG_MAX || tmp > INT_MAX {
                    error(
                        1,
                        34,
                        b"max events must be > 0 and <= INT_MAX\0".as_ptr() as *const c_char,
                    );
                }

                cfg_max_events = tmp as c_int;
            }
            x if x == b'd' as c_int => {
                if tmp == ULLONG_MAX || tmp > INT32_MAX {
                    error(
                        1,
                        34,
                        b"defer_hard_irqs must be <= INT32_MAX\0".as_ptr() as *const c_char,
                    );
                }

                cfg_defer_hard_irqs = tmp as u32;
            }
            x if x == b'r' as c_int => {
                if tmp == ULLONG_MAX || tmp > u64::MAX {
                    error(
                        1,
                        34,
                        b"gro_flush_timeout must be < UINT64_MAX\0".as_ptr() as *const c_char,
                    );
                }

                cfg_gro_flush_timeout = tmp as u64;
            }
            x if x == b's' as c_int => {
                if tmp == ULLONG_MAX || tmp > u64::MAX {
                    error(
                        1,
                        34,
                        b"irq_suspend_timeout must be < ULLONG_MAX\0".as_ptr() as *const c_char,
                    );
                }

                cfg_irq_suspend_timeout = tmp as u64;
            }
            x if x == b'i' as c_int => {
                if tmp == ULLONG_MAX || tmp > INT_MAX {
                    error(1, 34, b"ifindex must be <= INT_MAX\0".as_ptr() as *const c_char);
                }

                cfg_ifindex = tmp as c_int as u32;
            }
            x if x == b't' as c_int => {
                if tmp > 2 {
                    error(
                        1,
                        34,
                        b"napi threaded poll value must be 0-2\0".as_ptr() as *const c_char,
                    );
                }

                cfg_napi_threaded_poll = mem::transmute::<u32, netdev_napi_threaded>(tmp as u32);
            }
            _ => {}
        }
    }

    if cfg_ifindex == 0 {
        usage(*argv.add(0));
    }

    if optind != argc {
        usage(*argv.add(0));
    }
}

unsafe fn epoll_ctl_add(epfd: c_int, fd: c_int, events: u32) {
    let mut ev = epoll_event {
        events,
        data: epoll_data { fd },
    };

    if epoll_ctl(epfd, EPOLL_CTL_ADD, fd, &mut ev) == -1 {
        error(
            1,
            *__errno_location(),
            b"epoll_ctl add fd: %d\0".as_ptr() as *const c_char,
            fd,
        );
    }
}

unsafe fn setnonblock(sockfd: c_int) {
    let mut flags: c_int;

    flags = fcntl(sockfd, F_GETFL, 0);

    if fcntl(sockfd, F_SETFL, flags | O_NONBLOCK) == -1 {
        error(
            1,
            *__errno_location(),
            b"unable to set socket to nonblocking mode\0".as_ptr() as *const c_char,
        );
    }
}

unsafe fn write_chunk(fd: c_int, buf: *mut c_char, buflen: ssize_t) {
    let mut remaining: ssize_t = buflen;
    let mut buf_offset: *mut c_char = buf;
    let mut writelen: ssize_t = 0;
    let mut write_result: ssize_t;

    while writelen < buflen {
        write_result = write(fd, buf_offset as *const c_void, remaining as usize);
        if write_result == -1 {
            error(
                1,
                *__errno_location(),
                b"unable to write data to outfile\0".as_ptr() as *const c_char,
            );
        }

        writelen += write_result;
        remaining -= write_result;
        buf_offset = buf_offset.offset(write_result);
    }
}

unsafe fn setup_queue() {
    let mut napi_list: *mut netdev_napi_get_list = ptr::null_mut();
    let mut req: *mut netdev_napi_get_req_dump = ptr::null_mut();
    let mut set_req: *mut netdev_napi_set_req = ptr::null_mut();
    let mut ys: *mut ynl_sock;
    let mut yerr: ynl_error = mem::zeroed();
    let mut napi_id: u32 = 0;

    ys = ynl_sock_create(&ynl_netdev_family, &mut yerr);
    if ys.is_null() {
        error(1, 0, b"YNL: %s\0".as_ptr() as *const c_char, yerr.msg);
    }

    req = netdev_napi_get_req_dump_alloc();
    netdev_napi_get_req_dump_set_ifindex(req, cfg_ifindex);
    napi_list = netdev_napi_get_dump(ys, req);

    /* assume there is 1 NAPI configured and take the first */
    if (*napi_list).obj._present.id {
        napi_id = (*napi_list).obj.id;
    } else {
        error(1, 0, b"napi ID not present?\0".as_ptr() as *const c_char);
    }

    set_req = netdev_napi_set_req_alloc();
    netdev_napi_set_req_set_id(set_req, napi_id);
    netdev_napi_set_req_set_defer_hard_irqs(set_req, cfg_defer_hard_irqs);
    netdev_napi_set_req_set_gro_flush_timeout(set_req, cfg_gro_flush_timeout);
    netdev_napi_set_req_set_irq_suspend_timeout(set_req, cfg_irq_suspend_timeout);

    if cfg_napi_threaded_poll as u32 != 0 {
        netdev_napi_set_req_set_threaded(set_req, cfg_napi_threaded_poll);
    }

    if netdev_napi_set(ys, set_req) != 0 {
        error(
            1,
            0,
            b"can't set NAPI params: %s\n\0".as_ptr() as *const c_char,
            yerr.msg,
        );
    }

    netdev_napi_get_list_free(napi_list);
    netdev_napi_get_req_dump_free(req);
    netdev_napi_set_req_free(set_req);
    ynl_sock_destroy(ys);
}

unsafe fn run_poller() {
    let mut events: Vec<epoll_event> = Vec::with_capacity(cfg_max_events as usize);
    events.set_len(cfg_max_events as usize);
    let mut epoll_params: epoll_params = mem::zeroed();
    let mut server_addr: sockaddr_in = mem::zeroed();
    let mut i: c_int;
    let mut epfd: c_int;
    let mut nfds: c_int;
    let mut readlen: ssize_t;
    let mut outfile_fd: c_int;
    let mut buf: [c_char; 1024] = [0; 1024];
    let mut sockfd: c_int;
    let mut conn: c_int;
    let mut val: c_int;

    outfile_fd = open(cfg_outfile, O_WRONLY | O_CREAT, 0o644);
    if outfile_fd == -1 {
        error(
            1,
            *__errno_location(),
            b"unable to open outfile: %s\0".as_ptr() as *const c_char,
            cfg_outfile,
        );
    }

    sockfd = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if sockfd == -1 {
        error(
            1,
            *__errno_location(),
            b"unable to create listen socket\0".as_ptr() as *const c_char,
        );
    }

    server_addr.sin_family = AF_INET as u16;
    server_addr.sin_port = htons(cfg_port);
    server_addr.sin_addr = cfg_bind_addr;

    /* these values are range checked during parse_opts, so casting is safe
     * here
     */
    epoll_params.busy_poll_usecs = cfg_busy_poll_usecs;
    epoll_params.busy_poll_budget = cfg_busy_poll_budget;
    epoll_params.prefer_busy_poll = cfg_prefer_busy_poll;
    epoll_params.__pad = 0;

    val = 1;
    if setsockopt(
        sockfd,
        SOL_SOCKET,
        SO_REUSEADDR,
        &val as *const c_int as *const c_void,
        mem::size_of_val(&val) as socklen_t,
    ) != 0
    {
        error(
            1,
            *__errno_location(),
            b"poller setsockopt reuseaddr\0".as_ptr() as *const c_char,
        );
    }

    setnonblock(sockfd);

    if bind(
        sockfd,
        &server_addr as *const sockaddr_in as *const sockaddr,
        mem::size_of::<sockaddr_in>() as socklen_t,
    ) != 0
    {
        error(
            0,
            *__errno_location(),
            b"poller bind to port: %d\n\0".as_ptr() as *const c_char,
            cfg_port as c_int,
        );
    }

    if listen(sockfd, 1) != 0 {
        error(1, *__errno_location(), b"poller listen\0".as_ptr() as *const c_char);
    }

    epfd = epoll_create1(0);
    if ioctl(epfd, EPIOCSPARAMS, &mut epoll_params) == -1 {
        error(
            1,
            *__errno_location(),
            b"unable to set busy poll params\0".as_ptr() as *const c_char,
        );
    }

    epoll_ctl_add(epfd, sockfd, EPOLLIN | EPOLLOUT | EPOLLET);

    loop {
        nfds = epoll_wait(epfd, events.as_mut_ptr(), cfg_max_events, -1);
        i = 0;
        while i < nfds {
            if events[i as usize].data.fd == sockfd {
                conn = accept(sockfd, ptr::null_mut(), ptr::null_mut());
                if conn == -1 {
                    error(
                        1,
                        *__errno_location(),
                        b"accepting incoming connection failed\0".as_ptr() as *const c_char,
                    );
                }

                setnonblock(conn);
                epoll_ctl_add(
                    epfd,
                    conn,
                    EPOLLIN | EPOLLET | EPOLLRDHUP | EPOLLHUP,
                );
            } else if events[i as usize].events & EPOLLIN != 0 {
                loop {
                    readlen = read(
                        events[i as usize].data.fd,
                        buf.as_mut_ptr() as *mut c_void,
                        mem::size_of_val(&buf),
                    );
                    if readlen > 0 {
                        write_chunk(outfile_fd, buf.as_mut_ptr(), readlen);
                    } else {
                        break;
                    }
                }
            } else {
                /* spurious event ? */
            }
            if events[i as usize].events & (EPOLLRDHUP | EPOLLHUP) != 0 {
                epoll_ctl(
                    epfd,
                    EPOLL_CTL_DEL,
                    events[i as usize].data.fd,
                    ptr::null_mut(),
                );
                close(events[i as usize].data.fd);
                close(outfile_fd);
                return;
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    parse_opts(argc, argv);
    setup_queue();
    run_poller();

    if !cfg_outfile.is_null() {
        free(cfg_outfile as *mut c_void);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
