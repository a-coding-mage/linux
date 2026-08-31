// SPDX-License-Identifier: GPL-2.0 OR MIT
// Translated from scm_pidfd.c. C include dependencies are represented by libc
// items and external constants/types where the isolated file requires them.

use libc::*;
use std::ffi::CStr;
use std::mem;
use std::ptr;

const SCM_PIDFD: c_int = 0x04;
const CHILD_EXIT_CODE_OK: c_int = 123;

const PIDFD_INFO_EXIT: u64 = 1 << 1;

extern "C" {
    static mut errno: c_int;
}

#[repr(C)]
struct pidfd_info {
    mask: u64,
    cgroupid: u64,
    pid: u32,
    tgid: u32,
    ppid: u32,
    ruid: u32,
    rgid: u32,
    euid: u32,
    egid: u32,
    suid: u32,
    sgid: u32,
    fsuid: u32,
    fsgid: u32,
    exit_code: i32,
}

// From pidfd/pidfd.h.
extern "C" {
    static PIDFD_GET_INFO: c_ulong;
}

unsafe fn clean_errno() -> *const c_char {
    if errno == 0 {
        b"None\0".as_ptr() as *const c_char
    } else {
        strerror(errno)
    }
}

unsafe fn log_err(msg: &str) {
    let clean = CStr::from_ptr(clean_errno()).to_string_lossy();
    eprintln!("(scm_pidfd.rs: errno: {}) {}", clean, msg);
}

unsafe fn log_err_fmt(args: std::fmt::Arguments<'_>) {
    let clean = CStr::from_ptr(clean_errno()).to_string_lossy();
    eprintln!("(scm_pidfd.rs: errno: {}) {}", clean, args);
}

unsafe fn child_die() {
    exit(1);
}

unsafe fn safe_int(numstr: *const c_char, converted: *mut c_int) -> c_int {
    let mut err: *mut c_char = ptr::null_mut();
    let sli: c_long;

    errno = 0;
    sli = strtol(numstr, &mut err, 0);
    if errno == ERANGE && (sli == c_long::MAX || sli == c_long::MIN) {
        return -ERANGE;
    }

    if errno != 0 && sli == 0 {
        return -EINVAL;
    }

    if err == numstr as *mut c_char || *err != b'\0' as c_char {
        return -EINVAL;
    }

    if sli > c_int::MAX as c_long || sli < c_int::MIN as c_long {
        return -ERANGE;
    }

    *converted = sli as c_int;
    0
}

unsafe fn char_left_gc(buffer: *const c_char, len: size_t) -> c_int {
    let mut i: size_t = 0;

    while i < len {
        if *buffer.add(i) == b' ' as c_char || *buffer.add(i) == b'\t' as c_char {
            i += 1;
            continue;
        }

        return i as c_int;
    }

    0
}

unsafe fn char_right_gc(buffer: *const c_char, len: size_t) -> c_int {
    let mut i: c_int = len as c_int - 1;

    while i >= 0 {
        let ch = *buffer.add(i as usize);
        if ch == b' ' as c_char || ch == b'\t' as c_char ||
           ch == b'\n' as c_char || ch == b'\0' as c_char {
            i -= 1;
            continue;
        }

        return i + 1;
    }

    0
}

unsafe fn trim_whitespace_in_place(mut buffer: *mut c_char) -> *mut c_char {
    buffer = buffer.add(char_left_gc(buffer, strlen(buffer)) as usize);
    *buffer.add(char_right_gc(buffer, strlen(buffer)) as usize) = b'\0' as c_char;
    buffer
}

/* borrowed (with all helpers) from pidfd/pidfd_open_test.c */
unsafe fn get_pid_from_fdinfo_file(pidfd: c_int, key: *const c_char, keylen: size_t) -> pid_t {
    let mut ret: c_int;
    let mut path = [0 as c_char; 512];
    let mut n: size_t = 0;
    let mut result: pid_t = -1;
    let mut line: *mut c_char = ptr::null_mut();

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"/proc/self/fdinfo/%d\0".as_ptr() as *const c_char,
        pidfd,
    );

    let f = fopen(path.as_ptr(), b"re\0".as_ptr() as *const c_char);
    if f.is_null() {
        return -1;
    }

    while getline(&mut line, &mut n, f) != -1 {
        let numstr: *mut c_char;

        if strncmp(line, key, keylen) != 0 {
            continue;
        }

        numstr = trim_whitespace_in_place(line.add(4));
        ret = safe_int(numstr, &mut result);
        if ret < 0 {
            break;
        }

        break;
    }

    free(line as *mut c_void);
    fclose(f);
    result
}

#[repr(C)]
struct cmsg_data {
    ucred: *mut ucred,
    pidfd: *mut c_int,
}

unsafe fn cmsg_align(len: usize) -> usize {
    (len + mem::size_of::<size_t>() - 1) & !(mem::size_of::<size_t>() - 1)
}

unsafe fn cmsg_space(len: usize) -> usize {
    cmsg_align(mem::size_of::<cmsghdr>()) + cmsg_align(len)
}

unsafe fn parse_cmsg(msg: *mut msghdr, res: *mut cmsg_data) -> c_int {
    let mut cmsg: *mut cmsghdr;

    if (*msg).msg_flags & (MSG_TRUNC | MSG_CTRUNC) != 0 {
        log_err("recvmsg: truncated");
        return 1;
    }

    cmsg = CMSG_FIRSTHDR(msg);
    while !cmsg.is_null() {
        if (*cmsg).cmsg_level == SOL_SOCKET && (*cmsg).cmsg_type == SCM_PIDFD {
            if (*cmsg).cmsg_len < mem::size_of::<c_int>() as size_t {
                log_err("CMSG parse: SCM_PIDFD wrong len");
                return 1;
            }

            (*res).pidfd = CMSG_DATA(cmsg) as *mut c_int;
        }

        if (*cmsg).cmsg_level == SOL_SOCKET && (*cmsg).cmsg_type == SCM_CREDENTIALS {
            if (*cmsg).cmsg_len < mem::size_of::<ucred>() as size_t {
                log_err("CMSG parse: SCM_CREDENTIALS wrong len");
                return 1;
            }

            (*res).ucred = CMSG_DATA(cmsg) as *mut ucred;
        }

        cmsg = CMSG_NXTHDR(msg, cmsg);
    }

    if (*res).pidfd.is_null() {
        log_err("CMSG parse: SCM_PIDFD not found");
        return 1;
    }

    if (*res).ucred.is_null() {
        log_err("CMSG parse: SCM_CREDENTIALS not found");
        return 1;
    }

    0
}

unsafe fn cmsg_check(fd: c_int) -> c_int {
    let mut msg: msghdr = mem::zeroed();
    let mut res: cmsg_data = mem::zeroed();
    let mut iov: iovec = mem::zeroed();
    let mut data: c_int = 0;
    let mut control = vec![0 as c_char; cmsg_space(mem::size_of::<ucred>()) + cmsg_space(mem::size_of::<c_int>())];
    let parent_pid: pid_t;
    let mut err: c_int;

    iov.iov_base = &mut data as *mut _ as *mut c_void;
    iov.iov_len = mem::size_of_val(&data);

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = control.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = control.len();

    err = recvmsg(fd, &mut msg, 0);
    if err < 0 {
        log_err("recvmsg");
        return 1;
    }

    if msg.msg_flags & (MSG_TRUNC | MSG_CTRUNC) != 0 {
        log_err("recvmsg: truncated");
        return 1;
    }

    /* send(pfd, "x", sizeof(char), 0) */
    if data != b'x' as c_int {
        log_err("recvmsg: data corruption");
        return 1;
    }

    if parse_cmsg(&mut msg, &mut res) != 0 {
        log_err("CMSG parse: parse_cmsg() failed");
        return 1;
    }

    /* pidfd from SCM_PIDFD should point to the parent process PID */
    parent_pid = get_pid_from_fdinfo_file(*res.pidfd, b"Pid:\0".as_ptr() as *const c_char, mem::size_of_val(b"Pid:") - 1);
    if parent_pid != getppid() {
        log_err_fmt(format_args!("wrong SCM_PIDFD {} != {}", parent_pid, getppid()));
        close(*res.pidfd);
        return 1;
    }

    close(*res.pidfd);
    0
}

unsafe fn cmsg_check_dead(fd: c_int, _expected_pid: c_int) -> c_int {
    let mut err: c_int;
    let mut msg: msghdr = mem::zeroed();
    let mut res: cmsg_data = mem::zeroed();
    let mut iov: iovec = mem::zeroed();
    let mut data: c_int = 0;
    let mut control = vec![0 as c_char; cmsg_space(mem::size_of::<ucred>()) + cmsg_space(mem::size_of::<c_int>())];
    let mut info = pidfd_info {
        mask: PIDFD_INFO_EXIT,
        ..mem::zeroed()
    };

    iov.iov_base = &mut data as *mut _ as *mut c_void;
    iov.iov_len = mem::size_of_val(&data);

    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = control.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = control.len();

    err = recvmsg(fd, &mut msg, 0);
    if err < 0 {
        log_err("recvmsg");
        return 1;
    }

    if msg.msg_flags & (MSG_TRUNC | MSG_CTRUNC) != 0 {
        log_err("recvmsg: truncated");
        return 1;
    }

    /* send(cfd, "y", sizeof(char), 0) */
    if data != b'y' as c_int {
        log_err("recvmsg: data corruption");
        return 1;
    }

    if parse_cmsg(&mut msg, &mut res) != 0 {
        log_err("CMSG parse: parse_cmsg() failed");
        return 1;
    }

    /*
     * pidfd from SCM_PIDFD should point to the client_pid.
     * Let's read exit information and check if it's what
     * we expect to see.
     */
    if ioctl(*res.pidfd, PIDFD_GET_INFO, &mut info) != 0 {
        log_err("cmsg_check_dead: ioctl(PIDFD_GET_INFO) failed");
        close(*res.pidfd);
        return 1;
    }

    if info.mask & PIDFD_INFO_EXIT == 0 {
        log_err("cmsg_check_dead: No exit information from ioctl(PIDFD_GET_INFO)");
        close(*res.pidfd);
        return 1;
    }

    err = if WIFEXITED(info.exit_code) { WEXITSTATUS(info.exit_code) } else { 1 };
    if err != CHILD_EXIT_CODE_OK {
        log_err_fmt(format_args!("cmsg_check_dead: wrong exit_code {} != {}", err, CHILD_EXIT_CODE_OK));
        close(*res.pidfd);
        return 1;
    }

    close(*res.pidfd);
    0
}

#[repr(C)]
struct sock_addr {
    sock_name: [c_char; 32],
    listen_addr: sockaddr_un,
    addrlen: socklen_t,
}

#[repr(C)]
struct scm_pidfd {
    server: c_int,
    client_pid: pid_t,
    startup_pipe: [c_int; 2],
    server_addr: sock_addr,
    client_addr: *mut sock_addr,
}

#[repr(C)]
struct scm_pidfd_variant {
    type_: c_int,
    abstract_: bool,
}

static STREAM_PATHNAME: scm_pidfd_variant = scm_pidfd_variant {
    type_: SOCK_STREAM,
    abstract_: false,
};

static STREAM_ABSTRACT: scm_pidfd_variant = scm_pidfd_variant {
    type_: SOCK_STREAM,
    abstract_: true,
};

static DGRAM_PATHNAME: scm_pidfd_variant = scm_pidfd_variant {
    type_: SOCK_DGRAM,
    abstract_: false,
};

static DGRAM_ABSTRACT: scm_pidfd_variant = scm_pidfd_variant {
    type_: SOCK_DGRAM,
    abstract_: true,
};

unsafe fn fixture_setup_scm_pidfd(self_: *mut scm_pidfd) {
    (*self_).client_addr = mmap(
        ptr::null_mut(),
        mem::size_of::<sock_addr>(),
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut sock_addr;
    assert_ne!(MAP_FAILED, (*self_).client_addr as *mut c_void);
}

unsafe fn fixture_teardown_scm_pidfd(self_: *mut scm_pidfd, variant: *const scm_pidfd_variant) {
    close((*self_).server);

    kill((*self_).client_pid, SIGKILL);
    waitpid((*self_).client_pid, ptr::null_mut(), 0);

    if !(*variant).abstract_ {
        unlink((*self_).server_addr.sock_name.as_ptr());
        unlink((*(*self_).client_addr).sock_name.as_ptr());
    }
}

unsafe fn fill_sockaddr(addr: *mut sock_addr, abstract_: bool) {
    let mut sun_path_buf = (*addr).listen_addr.sun_path.as_mut_ptr();

    (*addr).listen_addr.sun_family = AF_UNIX as sa_family_t;
    (*addr).addrlen = memoffset_sun_path() as socklen_t;
    snprintf(
        (*addr).sock_name.as_mut_ptr(),
        (*addr).sock_name.len(),
        b"scm_pidfd_%d\0".as_ptr() as *const c_char,
        getpid(),
    );
    (*addr).addrlen += strlen((*addr).sock_name.as_ptr()) as socklen_t;
    if abstract_ {
        *sun_path_buf = b'\0' as c_char;
        (*addr).addrlen += 1;
        sun_path_buf = sun_path_buf.add(1);
    } else {
        unlink((*addr).sock_name.as_ptr());
    }
    memcpy(
        sun_path_buf as *mut c_void,
        (*addr).sock_name.as_ptr() as *const c_void,
        strlen((*addr).sock_name.as_ptr()),
    );
}

unsafe fn memoffset_sun_path() -> usize {
    let un: sockaddr_un = mem::zeroed();
    let base = &un as *const _ as usize;
    let field = &un.sun_path as *const _ as usize;
    field - base
}

unsafe fn sk_enable_cred_pass(sk: c_int) -> c_int {
    let mut on: c_int = 0;

    on = 1;
    if setsockopt(
        sk,
        SOL_SOCKET,
        SO_PASSCRED,
        &on as *const _ as *const c_void,
        mem::size_of_val(&on) as socklen_t,
    ) != 0 {
        log_err("Failed to set SO_PASSCRED");
        return 1;
    }

    if setsockopt(
        sk,
        SOL_SOCKET,
        SO_PASSPIDFD,
        &on as *const _ as *const c_void,
        mem::size_of_val(&on) as socklen_t,
    ) != 0 {
        log_err("Failed to set SO_PASSPIDFD");
        return 1;
    }

    0
}

unsafe fn client(self_: *mut scm_pidfd, variant: *const scm_pidfd_variant) {
    let cfd: c_int;
    let mut len: socklen_t;
    let mut peer_cred: ucred = mem::zeroed();
    let mut peer_pidfd: c_int = 0;
    let peer_pid: pid_t;

    cfd = socket(AF_UNIX, (*variant).type_, 0);
    if cfd < 0 {
        log_err("socket");
        child_die();
    }

    if (*variant).type_ == SOCK_DGRAM {
        fill_sockaddr((*self_).client_addr, (*variant).abstract_);

        if bind(
            cfd,
            &(*(*self_).client_addr).listen_addr as *const _ as *const sockaddr,
            (*(*self_).client_addr).addrlen,
        ) != 0 {
            log_err("bind");
            child_die();
        }
    }

    if connect(
        cfd,
        &(*self_).server_addr.listen_addr as *const _ as *const sockaddr,
        (*self_).server_addr.addrlen,
    ) != 0 {
        log_err("connect");
        child_die();
    }

    if sk_enable_cred_pass(cfd) != 0 {
        log_err("sk_enable_cred_pass() failed");
        child_die();
    }

    close((*self_).startup_pipe[1]);

    if cmsg_check(cfd) != 0 {
        log_err("cmsg_check failed");
        child_die();
    }

    /* send something to the parent so it can receive SCM_PIDFD too and validate it */
    if send(cfd, b"y\0".as_ptr() as *const c_void, mem::size_of::<c_char>(), 0) == -1 {
        log_err("Failed to send(cfd, \"y\", sizeof(char), 0)");
        child_die();
    }

    /* skip further for SOCK_DGRAM as it's not applicable */
    if (*variant).type_ == SOCK_DGRAM {
        return;
    }

    len = mem::size_of_val(&peer_cred) as socklen_t;
    if getsockopt(
        cfd,
        SOL_SOCKET,
        SO_PEERCRED,
        &mut peer_cred as *mut _ as *mut c_void,
        &mut len,
    ) != 0 {
        log_err("Failed to get SO_PEERCRED");
        child_die();
    }

    len = mem::size_of_val(&peer_pidfd) as socklen_t;
    if getsockopt(
        cfd,
        SOL_SOCKET,
        SO_PEERPIDFD,
        &mut peer_pidfd as *mut _ as *mut c_void,
        &mut len,
    ) != 0 {
        log_err("Failed to get SO_PEERPIDFD");
        child_die();
    }

    /* pid from SO_PEERCRED should point to the parent process PID */
    if peer_cred.pid != getppid() {
        log_err_fmt(format_args!("peer_cred.pid != getppid(): {} != {}", peer_cred.pid, getppid()));
        child_die();
    }

    peer_pid = get_pid_from_fdinfo_file(peer_pidfd, b"Pid:\0".as_ptr() as *const c_char, mem::size_of_val(b"Pid:") - 1);
    if peer_pid != peer_cred.pid {
        log_err_fmt(format_args!("peer_pid != peer_cred.pid: {} != {}", peer_pid, peer_cred.pid));
        child_die();
    }
}

unsafe fn test_scm_pidfd_test(self_: *mut scm_pidfd, variant: *const scm_pidfd_variant) {
    let mut err: c_int;
    let pfd: c_int;
    let mut child_status: c_int = 0;

    (*self_).server = socket(AF_UNIX, (*variant).type_, 0);
    assert_ne!(-1, (*self_).server);

    fill_sockaddr(&mut (*self_).server_addr, (*variant).abstract_);

    err = bind(
        (*self_).server,
        &(*self_).server_addr.listen_addr as *const _ as *const sockaddr,
        (*self_).server_addr.addrlen,
    );
    assert_eq!(0, err);

    if (*variant).type_ == SOCK_STREAM {
        err = listen((*self_).server, 1);
        assert_eq!(0, err);
    }

    err = pipe((*self_).startup_pipe.as_mut_ptr());
    assert_ne!(-1, err);

    (*self_).client_pid = fork();
    assert_ne!(-1, (*self_).client_pid);
    if (*self_).client_pid == 0 {
        close((*self_).server);
        close((*self_).startup_pipe[0]);
        client(self_, variant);

        /*
         * It's a bit unusual, but in case of success we return non-zero
         * exit code (CHILD_EXIT_CODE_OK) and then we expect to read it
         * from ioctl(PIDFD_GET_INFO) in cmsg_check_dead().
         */
        exit(CHILD_EXIT_CODE_OK);
    }
    close((*self_).startup_pipe[1]);

    if (*variant).type_ == SOCK_STREAM {
        pfd = accept((*self_).server, ptr::null_mut(), ptr::null_mut());
        assert_ne!(-1, pfd);
    } else {
        pfd = (*self_).server;
    }

    /* wait until the child arrives at checkpoint */
    read(
        (*self_).startup_pipe[0],
        &mut err as *mut _ as *mut c_void,
        mem::size_of::<c_int>(),
    );
    close((*self_).startup_pipe[0]);

    if (*variant).type_ == SOCK_DGRAM {
        err = sendto(
            pfd,
            b"x\0".as_ptr() as *const c_void,
            mem::size_of::<c_char>(),
            0,
            &(*(*self_).client_addr).listen_addr as *const _ as *const sockaddr,
            (*(*self_).client_addr).addrlen,
        ) as c_int;
        assert_ne!(-1, err);
    } else {
        err = send(pfd, b"x\0".as_ptr() as *const c_void, mem::size_of::<c_char>(), 0) as c_int;
        assert_ne!(-1, err);
    }

    waitpid((*self_).client_pid, &mut child_status, 0);
    /* see comment before exit(CHILD_EXIT_CODE_OK) */
    assert_eq!(
        CHILD_EXIT_CODE_OK,
        if WIFEXITED(child_status) {
            WEXITSTATUS(child_status)
        } else {
            1
        }
    );

    err = sk_enable_cred_pass(pfd);
    assert_eq!(0, err);

    err = cmsg_check_dead(pfd, (*self_).client_pid);
    assert_eq!(0, err);

    close(pfd);
}

// TEST_HARNESS_MAIN: the original C file relies on kselftest_harness.h macros
// to instantiate and run the fixture variants above.
