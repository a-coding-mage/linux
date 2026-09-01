// SPDX-License-Identifier: GPL-2.0

// C source used _GNU_SOURCE and Linux networking headers.

use libc::{
    addrinfo, c_char, c_int, c_uint, c_void, cmsghdr, iovec, msghdr, pid_t, size_t, ssize_t,
    timespec,
};
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

const IPPROTO_MPTCP: c_int = 262;
const SOL_MPTCP: c_int = 284;

const TCP_CM_INQ: c_int = libc::TCP_CM_INQ;
const TCP_INQ: c_int = libc::TCP_INQ;
const SIOCOUTQNSD: libc::c_ulong = 0x894B;

static mut PF: c_int = libc::AF_INET;
static mut PROTO_TX: c_int = IPPROTO_MPTCP;
static mut PROTO_RX: c_int = IPPROTO_MPTCP;

unsafe fn die_perror(msg: *const c_char) -> ! {
    libc::perror(msg);
    libc::exit(1);
}

unsafe fn die_usage(r: c_int) -> ! {
    libc::fprintf(
        libc::stderr,
        c"Usage: mptcp_inq [-6] [ -t tcp|mptcp ] [ -r tcp|mptcp]\n".as_ptr(),
    );
    libc::exit(r);
}

macro_rules! xerror {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            libc::fprintf(libc::stderr, $fmt.as_ptr() $(, $arg)*);
            libc::fputc('\n' as c_int, libc::stderr);
            libc::exit(1);
        }
    }};
}

unsafe fn getxinfo_strerr(err: c_int) -> *const c_char {
    if err == libc::EAI_SYSTEM {
        return libc::strerror(*libc::__errno_location());
    }

    libc::gai_strerror(err)
}

unsafe fn xgetaddrinfo(
    node: *const c_char,
    service: *const c_char,
    hints: *mut addrinfo,
    res: *mut *mut addrinfo,
) {
    let mut err: c_int;

    loop {
        err = libc::getaddrinfo(node, service, hints, res);
        if err == 0 {
            break;
        }

        if err == libc::EAI_SOCKTYPE {
            (*hints).ai_protocol = libc::IPPROTO_TCP;
            continue;
        }

        let errstr = getxinfo_strerr(err);

        libc::fprintf(
            libc::stderr,
            c"Fatal: getaddrinfo(%s:%s): %s\n".as_ptr(),
            if !node.is_null() { node } else { c"".as_ptr() },
            if !service.is_null() { service } else { c"".as_ptr() },
            errstr,
        );
        libc::exit(1);
    }
}

unsafe fn sock_listen_mptcp(listenaddr: *const c_char, port: *const c_char) -> c_int {
    let mut sock: c_int = -1;
    let mut hints: addrinfo = mem::zeroed();
    hints.ai_protocol = IPPROTO_MPTCP;
    hints.ai_socktype = libc::SOCK_STREAM;
    hints.ai_flags = libc::AI_PASSIVE | libc::AI_NUMERICHOST;
    hints.ai_family = PF;

    let mut addr: *mut addrinfo = ptr::null_mut();
    let mut one: c_int = 1;

    xgetaddrinfo(listenaddr, port, &mut hints, &mut addr);
    hints.ai_family = PF;

    let mut a = addr;
    while !a.is_null() {
        sock = libc::socket((*a).ai_family, (*a).ai_socktype, PROTO_RX);
        if sock < 0 {
            a = (*a).ai_next;
            continue;
        }

        if -1
            == libc::setsockopt(
                sock,
                libc::SOL_SOCKET,
                libc::SO_REUSEADDR,
                &mut one as *mut _ as *const c_void,
                mem::size_of_val(&one) as libc::socklen_t,
            )
        {
            libc::perror(c"setsockopt".as_ptr());
        }

        if libc::bind(sock, (*a).ai_addr, (*a).ai_addrlen) == 0 {
            break; /* success */
        }

        libc::perror(c"bind".as_ptr());
        libc::close(sock);
        sock = -1;
        a = (*a).ai_next;
    }

    libc::freeaddrinfo(addr);

    if sock < 0 {
        xerror!(c"could not create listen socket");
    }

    if libc::listen(sock, 20) != 0 {
        die_perror(c"listen".as_ptr());
    }

    sock
}

unsafe fn sock_connect_mptcp(remoteaddr: *const c_char, port: *const c_char, proto: c_int) -> c_int {
    let mut hints: addrinfo = mem::zeroed();
    hints.ai_protocol = IPPROTO_MPTCP;
    hints.ai_socktype = libc::SOCK_STREAM;
    let mut addr: *mut addrinfo = ptr::null_mut();
    let mut sock: c_int = -1;

    hints.ai_family = PF;

    xgetaddrinfo(remoteaddr, port, &mut hints, &mut addr);
    let mut a = addr;
    while !a.is_null() {
        sock = libc::socket((*a).ai_family, (*a).ai_socktype, proto);
        if sock < 0 {
            a = (*a).ai_next;
            continue;
        }

        if libc::connect(sock, (*a).ai_addr, (*a).ai_addrlen) == 0 {
            break; /* success */
        }

        die_perror(c"connect".as_ptr());
    }

    if sock < 0 {
        xerror!(c"could not create connect socket");
    }

    libc::freeaddrinfo(addr);
    sock
}

unsafe fn protostr_to_num(s: *const c_char) -> c_int {
    if libc::strcasecmp(s, c"tcp".as_ptr()) == 0 {
        return libc::IPPROTO_TCP;
    }
    if libc::strcasecmp(s, c"mptcp".as_ptr()) == 0 {
        return IPPROTO_MPTCP;
    }

    die_usage(1);
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut c: c_int;

    loop {
        c = libc::getopt(argc, argv, c"h6t:r:".as_ptr());
        if c == -1 {
            break;
        }

        match c as u8 as char {
            'h' => die_usage(0),
            '6' => PF = libc::AF_INET6,
            't' => PROTO_TX = protostr_to_num(libc::optarg),
            'r' => PROTO_RX = protostr_to_num(libc::optarg),
            _ => die_usage(1),
        }
    }
}

/* wait up to timeout milliseconds */
unsafe fn wait_for_ack(fd: c_int, timeout: c_int, total: size_t) {
    let mut i: c_int;

    i = 0;
    while i < timeout {
        let mut nsd: c_int = 0;
        let mut queued: c_int = -1;
        let mut req: timespec = mem::zeroed();

        let mut ret = libc::ioctl(fd, libc::TIOCOUTQ, &mut queued);
        if ret < 0 {
            die_perror(c"TIOCOUTQ".as_ptr());
        }

        ret = libc::ioctl(fd, SIOCOUTQNSD, &mut nsd);
        if ret < 0 {
            die_perror(c"SIOCOUTQNSD".as_ptr());
        }

        if queued as size_t > total {
            xerror!(c"TIOCOUTQ %u, but only %zu expected\n", queued as c_uint, total);
        }
        assert!(nsd <= queued);

        if queued == 0 {
            return;
        }

        /* wait for peer to ack rx of all data */
        req.tv_sec = 0;
        req.tv_nsec = 1 * 1000 * 1000; /* 1ms */
        libc::nanosleep(&req, ptr::null_mut());

        i += 1;
    }

    xerror!(c"still tx data queued after %u ms\n", timeout as c_uint);
}

unsafe fn connect_one_server(fd: c_int, unixfd: c_int) {
    let mut len: size_t;
    let mut i: size_t;
    let mut total: size_t;
    let sent: size_t;
    let mut buf = [0 as c_char; 4096];
    let mut buf2 = [0 as c_char; 4096];
    let mut ret: ssize_t;

    len = (libc::rand() as usize % (mem::size_of_val(&buf) - 1)) as size_t;

    if len < 128 {
        len = 128;
    }

    i = 0;
    while i < len {
        buf[i] = (libc::rand() % 26) as c_char;
        buf[i] += b'A' as c_char;
        i += 1;
    }

    buf[i] = b'\n' as c_char;

    /* un-block server */
    ret = libc::read(unixfd, buf2.as_mut_ptr() as *mut c_void, 4);
    assert!(ret == 4);

    assert!(libc::strncmp(buf2.as_ptr(), c"xmit".as_ptr(), 4) == 0);

    ret = libc::write(unixfd, &len as *const _ as *const c_void, mem::size_of_val(&len));
    assert!(ret == mem::size_of_val(&len) as ssize_t);

    ret = libc::write(fd, buf.as_ptr() as *const c_void, len);
    if ret < 0 {
        die_perror(c"write".as_ptr());
    }

    if ret != len as ssize_t {
        xerror!(c"short write");
    }

    ret = libc::read(unixfd, buf2.as_mut_ptr() as *mut c_void, 4);
    assert!(libc::strncmp(buf2.as_ptr(), c"huge".as_ptr(), 4) == 0);

    total = (libc::rand() as usize % (16 * 1024 * 1024)) as size_t;
    total += 1 * 1024 * 1024;
    sent = total;

    ret = libc::write(unixfd, &total as *const _ as *const c_void, mem::size_of_val(&total));
    assert!(ret == mem::size_of_val(&total) as ssize_t);

    wait_for_ack(fd, 5000, len);

    while total > 0 {
        if total > mem::size_of_val(&buf) {
            len = mem::size_of_val(&buf);
        } else {
            len = total;
        }

        ret = libc::write(fd, buf.as_ptr() as *const c_void, len);
        if ret < 0 {
            die_perror(c"write".as_ptr());
        }
        total -= ret as size_t;

        /* we don't have to care about buf content, only
         * number of total bytes sent
         */
    }

    ret = libc::read(unixfd, buf2.as_mut_ptr() as *mut c_void, 4);
    assert!(ret == 4);
    assert!(libc::strncmp(buf2.as_ptr(), c"shut".as_ptr(), 4) == 0);

    wait_for_ack(fd, 5000, sent);

    ret = libc::write(fd, buf.as_ptr() as *const c_void, 1);
    assert!(ret == 1);
    libc::close(fd);
    ret = libc::write(unixfd, c"closed".as_ptr() as *const c_void, 6);
    assert!(ret == 6);

    libc::close(unixfd);
}

unsafe fn get_tcp_inq(msgh: *mut msghdr, inqv: *mut c_uint) {
    let mut cmsg: *mut cmsghdr;

    cmsg = libc::CMSG_FIRSTHDR(msgh);
    while !cmsg.is_null() {
        if (*cmsg).cmsg_level == libc::IPPROTO_TCP && (*cmsg).cmsg_type == TCP_CM_INQ {
            ptr::copy_nonoverlapping(
                libc::CMSG_DATA(cmsg) as *const c_void,
                inqv as *mut c_void,
                mem::size_of_val(&*inqv),
            );
            return;
        }

        cmsg = libc::CMSG_NXTHDR(msgh, cmsg);
    }

    xerror!(c"could not find TCP_CM_INQ cmsg type");
}

unsafe fn process_one_client(fd: c_int, unixfd: c_int) {
    let mut tcp_inq: c_uint = 0;
    let mut expect_len: size_t = 0;
    let mut msg_buf = [0 as c_char; 4096];
    let mut buf = [0 as c_char; 4096];
    let mut tmp = [0 as c_char; 16];
    let mut iov = iovec {
        iov_base: buf.as_mut_ptr() as *mut c_void,
        iov_len: 1,
    };
    let mut msg: msghdr = mem::zeroed();
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = msg_buf.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = mem::size_of_val(&msg_buf);
    let mut ret: ssize_t;
    let mut tot: ssize_t;

    ret = libc::write(unixfd, c"xmit".as_ptr() as *const c_void, 4);
    assert!(ret == 4);

    ret = libc::read(
        unixfd,
        &mut expect_len as *mut _ as *mut c_void,
        mem::size_of_val(&expect_len),
    );
    assert!(ret == mem::size_of_val(&expect_len) as ssize_t);

    if expect_len > mem::size_of_val(&buf) {
        xerror!(c"expect len %zu exceeds buffer size", expect_len);
    }

    loop {
        let mut req: timespec = mem::zeroed();
        let mut queued: c_uint = 0;

        ret = libc::ioctl(fd, libc::FIONREAD, &mut queued);
        if ret < 0 {
            die_perror(c"FIONREAD".as_ptr());
        }
        if queued as size_t > expect_len {
            xerror!(
                c"FIONREAD returned %u, but only %zu expected\n",
                queued,
                expect_len
            );
        }
        if queued as size_t == expect_len {
            break;
        }

        req.tv_sec = 0;
        req.tv_nsec = 1000 * 1000;
        libc::nanosleep(&req, ptr::null_mut());
    }

    /* read one byte, expect cmsg to return expected - 1 */
    ret = libc::recvmsg(fd, &mut msg, 0);
    if ret < 0 {
        die_perror(c"recvmsg".as_ptr());
    }

    if msg.msg_controllen == 0 {
        xerror!(c"msg_controllen is 0");
    }

    get_tcp_inq(&mut msg, &mut tcp_inq);

    assert!(tcp_inq as size_t == expect_len - 1);

    iov.iov_len = mem::size_of_val(&buf);
    ret = libc::recvmsg(fd, &mut msg, 0);
    if ret < 0 {
        die_perror(c"recvmsg".as_ptr());
    }

    /* should have gotten exact remainder of all pending data */
    assert!(ret == tcp_inq as ssize_t);

    /* should be 0, all drained */
    get_tcp_inq(&mut msg, &mut tcp_inq);
    assert!(tcp_inq == 0);

    /* request a large swath of data. */
    ret = libc::write(unixfd, c"huge".as_ptr() as *const c_void, 4);
    assert!(ret == 4);

    ret = libc::read(
        unixfd,
        &mut expect_len as *mut _ as *mut c_void,
        mem::size_of_val(&expect_len),
    );
    assert!(ret == mem::size_of_val(&expect_len) as ssize_t);

    /* peer should send us a few mb of data */
    if expect_len <= mem::size_of_val(&buf) {
        xerror!(c"expect len %zu too small\n", expect_len);
    }

    tot = 0;
    loop {
        iov.iov_len = mem::size_of_val(&buf);
        ret = libc::recvmsg(fd, &mut msg, 0);
        if ret < 0 {
            die_perror(c"recvmsg".as_ptr());
        }

        tot += ret;

        get_tcp_inq(&mut msg, &mut tcp_inq);

        if (tcp_inq as size_t) > expect_len - tot as size_t {
            xerror!(
                c"inq %d, remaining %d total_len %d\n",
                tcp_inq as c_int,
                (expect_len - tot as size_t) as c_int,
                expect_len as c_int
            );
        }

        assert!((tcp_inq as size_t) <= expect_len - tot as size_t);

        if tot as size_t >= expect_len {
            break;
        }
    }

    ret = libc::write(unixfd, c"shut".as_ptr() as *const c_void, 4);
    assert!(ret == 4);

    /* wait for hangup. Should have received one more byte of data. */
    ret = libc::read(unixfd, tmp.as_mut_ptr() as *mut c_void, mem::size_of_val(&tmp));
    assert!(ret == 6);
    assert!(libc::strncmp(tmp.as_ptr(), c"closed".as_ptr(), 6) == 0);

    libc::sleep(1);

    iov.iov_len = 1;
    ret = libc::recvmsg(fd, &mut msg, 0);
    if ret < 0 {
        die_perror(c"recvmsg".as_ptr());
    }
    assert!(ret == 1);

    get_tcp_inq(&mut msg, &mut tcp_inq);

    /* tcp_inq should be 1 due to received fin. */
    assert!(tcp_inq == 1);

    iov.iov_len = 1;
    ret = libc::recvmsg(fd, &mut msg, 0);
    if ret < 0 {
        die_perror(c"recvmsg".as_ptr());
    }

    /* expect EOF */
    assert!(ret == 0);
    get_tcp_inq(&mut msg, &mut tcp_inq);
    assert!(tcp_inq == 1);

    libc::close(fd);
}

unsafe fn xaccept(s: c_int) -> c_int {
    let fd = libc::accept(s, ptr::null_mut(), ptr::null_mut());

    if fd < 0 {
        die_perror(c"accept".as_ptr());
    }

    fd
}

unsafe fn server(unixfd: c_int) -> c_int {
    let mut fd: c_int = -1;
    let mut on: c_int = 1;

    match PF {
        libc::AF_INET => {
            fd = sock_listen_mptcp(c"127.0.0.1".as_ptr(), c"15432".as_ptr());
        }
        libc::AF_INET6 => {
            fd = sock_listen_mptcp(c"::1".as_ptr(), c"15432".as_ptr());
        }
        _ => {
            xerror!(c"Unknown pf %d\n", PF);
        }
    }

    let mut r = libc::write(unixfd, c"conn".as_ptr() as *const c_void, 4) as c_int;
    assert!(r == 4);

    libc::alarm(15);
    r = xaccept(fd);

    if -1
        == libc::setsockopt(
            r,
            libc::IPPROTO_TCP,
            TCP_INQ,
            &mut on as *mut _ as *const c_void,
            mem::size_of_val(&on) as libc::socklen_t,
        )
    {
        die_perror(c"setsockopt".as_ptr());
    }

    process_one_client(r, unixfd);

    libc::close(fd);
    0
}

unsafe fn client(unixfd: c_int) -> c_int {
    let mut fd: c_int = -1;

    libc::alarm(15);

    match PF {
        libc::AF_INET => {
            fd = sock_connect_mptcp(c"127.0.0.1".as_ptr(), c"15432".as_ptr(), PROTO_TX);
        }
        libc::AF_INET6 => {
            fd = sock_connect_mptcp(c"::1".as_ptr(), c"15432".as_ptr(), PROTO_TX);
        }
        _ => {
            xerror!(c"Unknown pf %d\n", PF);
        }
    }

    connect_one_server(fd, unixfd);

    0
}

unsafe fn init_rng() {
    let mut foo: c_uint = 0;

    if libc::getrandom(
        &mut foo as *mut _ as *mut c_void,
        mem::size_of_val(&foo),
        0,
    ) == -1
    {
        libc::perror(c"getrandom".as_ptr());
        libc::exit(1);
    }

    libc::srand(foo);
}

unsafe fn xfork() -> pid_t {
    let p = libc::fork();

    if p < 0 {
        die_perror(c"fork".as_ptr());
    } else if p == 0 {
        init_rng();
    }

    p
}

unsafe fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn wifsignaled(status: c_int) -> bool {
    ((status & 0x7f) + 1) as i8 >= 2
}

unsafe fn wtermsig(status: c_int) -> c_int {
    status & 0x7f
}

unsafe fn wifstopped(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

unsafe fn wstopsig(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn rcheck(wstatus: c_int, what: *const c_char) -> c_int {
    if wifexited(wstatus) {
        if wexitstatus(wstatus) == 0 {
            return 0;
        }
        libc::fprintf(
            libc::stderr,
            c"%s exited, status=%d\n".as_ptr(),
            what,
            wexitstatus(wstatus),
        );
        return wexitstatus(wstatus);
    } else if wifsignaled(wstatus) {
        xerror!(c"%s killed by signal %d\n", what, wtermsig(wstatus));
    } else if wifstopped(wstatus) {
        xerror!(c"%s stopped by signal %d\n", what, wstopsig(wstatus));
    }

    111
}

fn main() {
    unsafe {
        let args: Vec<CString> = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect();
        let mut argv: Vec<*mut c_char> = args.iter().map(|arg| arg.as_ptr() as *mut c_char).collect();
        argv.push(ptr::null_mut());
        let argc = args.len() as c_int;

        let mut e1: c_int;
        let e2: c_int;
        let mut wstatus: c_int = 0;
        let s: pid_t;
        let c: pid_t;
        let mut ret: pid_t;
        let mut unixfds = [0 as c_int; 2];

        parse_opts(argc, argv.as_mut_ptr());

        e1 = libc::socketpair(libc::AF_UNIX, libc::SOCK_DGRAM, 0, unixfds.as_mut_ptr());
        if e1 < 0 {
            die_perror(c"pipe".as_ptr());
        }

        s = xfork();
        if s == 0 {
            libc::close(unixfds[0]);
            ret = server(unixfds[1]);
            libc::close(unixfds[1]);
            std::process::exit(ret);
        }

        libc::close(unixfds[1]);

        /* wait until server bound a socket */
        e1 = libc::read(unixfds[0], &mut e1 as *mut _ as *mut c_void, 4) as c_int;
        assert!(e1 == 4);

        c = xfork();
        if c == 0 {
            std::process::exit(client(unixfds[0]));
        }

        libc::close(unixfds[0]);

        ret = libc::waitpid(s, &mut wstatus, 0);
        if ret == -1 {
            die_perror(c"waitpid".as_ptr());
        }
        e1 = rcheck(wstatus, c"server".as_ptr());
        ret = libc::waitpid(c, &mut wstatus, 0);
        if ret == -1 {
            die_perror(c"waitpid".as_ptr());
        }
        e2 = rcheck(wstatus, c"client".as_ptr());

        std::process::exit(if e1 != 0 { e1 } else { e2 });
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
