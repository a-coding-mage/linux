// SPDX-License-Identifier: GPL-2.0
// Translated from perf/builtin-daemon.c. C include dependencies are preserved
// as external declarations or comments where this isolated file cannot define them.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type time_t = c_long;
type pid_t = c_int;
type ssize_t = isize;
type size_t = usize;
type off_t = c_long;
type sig_atomic_t = c_int;

const SESSION_OUTPUT: &[u8] = b"output\0";
const SESSION_CONTROL: &[u8] = b"control\0";
const SESSION_ACK: &[u8] = b"ack\0";

const PATH_MAX: usize = 4096;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EACCES: c_int = 13;
const ENOENT: c_int = 2;
const EEXIST: c_int = 17;
const EAGAIN: c_int = 11;
const R_OK: c_int = 4;

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const O_NONBLOCK: c_int = 0o4000;
const O_CLOEXEC: c_int = 0o2000000;

const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const F_SETFD: c_int = 2;
const FD_CLOEXEC: c_int = 1;
const LOCK_EX: c_int = 2;
const LOCK_NB: c_int = 4;
const F_TLOCK: c_int = 2;

const POLLIN: c_short = 0x0001;
const POLLERR: c_short = 0x0008;
const POLLHUP: c_short = 0x0010;

const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const SIGPIPE: c_int = 13;
const SIGCHLD: c_int = 17;
const SIGUSR2: c_int = 12;
const SIGKILL: c_int = 9;
const SIG_IGN: usize = 1;

const SIG_BLOCK: c_int = 0;
const SFD_NONBLOCK: c_int = O_NONBLOCK;
const SFD_CLOEXEC: c_int = O_CLOEXEC;
const WNOHANG: c_int = 1;

const IN_NONBLOCK: c_int = O_NONBLOCK;
const IN_CLOSE_WRITE: u32 = 0x00000008;
const IN_ISDIR: u32 = 0x40000000;

const S_IFMT: u32 = 0o170000;
const S_IFDIR: u32 = 0o040000;

type c_short = i16;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_config_set {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fdarray_entry {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[repr(C)]
pub struct fdarray {
    pub entries: *mut fdarray_entry,
    pub nr: c_int,
    pub nr_alloc: c_int,
}

#[repr(C)]
pub struct stat {
    pub st_dev: c_long,
    pub st_ino: c_long,
    pub st_nlink: c_long,
    pub st_mode: u32,
    _rest: [u8; 128],
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: u16,
    pub sun_path: [c_char; 108],
}

#[repr(C)]
pub struct signalfd_siginfo {
    _private: [u8; 128],
}

#[repr(C)]
pub struct sigset_t {
    _private: [u64; 16],
}

#[repr(C)]
pub struct inotify_event {
    pub wd: c_int,
    pub mask: u32,
    pub cookie: u32,
    pub len: u32,
    pub name: [c_char; 0],
}

/*
 * Session states:
 *
 *   OK       - session is up and running
 *   RECONFIG - session is pending for reconfiguration,
 *              new values are already loaded in session object
 *   KILL     - session is pending to be killed
 *
 * Session object life and its state is maintained by
 * following functions:
 *
 *  setup_server_config
 *    - reads config file and setup session objects
 *      with following states:
 *
 *      OK       - no change needed
 *      RECONFIG - session needs to be changed
 *                 (run variable changed)
 *      KILL     - session needs to be killed
 *                 (session is no longer in config file)
 *
 *  daemon__reconfig
 *    - scans session objects and does following actions
 *      for states:
 *
 *      OK       - skip
 *      RECONFIG - session is killed and re-run with new config
 *      KILL     - session is killed
 *
 *    - all sessions have OK state on the function exit
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum daemon_session_state {
    OK,
    RECONFIG,
    KILL,
}

#[repr(C)]
pub struct daemon_session {
    pub base: *mut c_char,
    pub name: *mut c_char,
    pub run: *mut c_char,
    pub control: *mut c_char,
    pub pid: c_int,
    pub list: list_head,
    pub state: daemon_session_state,
    pub start: time_t,
}

#[repr(C)]
pub struct daemon {
    pub config: *const c_char,
    pub config_real: *mut c_char,
    pub config_base: *mut c_char,
    pub csv_sep: *const c_char,
    pub base_user: *const c_char,
    pub base: *mut c_char,
    pub sessions: list_head,
    pub out: *mut FILE,
    pub perf: *mut c_char,
    pub signal_fd: c_int,
    pub start: time_t,
}

#[repr(C)]
pub struct list_cmd {
    pub cmd: c_int,
    pub verbose: c_int,
    pub csv_sep: c_char,
}

#[repr(C)]
pub struct signal_cmd {
    pub cmd: c_int,
    pub sig: c_int,
    pub name: [c_char; SESSION_MAX],
}

#[repr(C)]
pub struct ping_cmd {
    pub cmd: c_int,
    pub name: [c_char; SESSION_MAX],
}

#[repr(C)]
pub union cmd {
    pub cmd: c_int,
    pub list: list_cmd,
    pub signal: signal_cmd,
    pub ping: ping_cmd,
}

const CMD_LIST: c_int = 0;
const CMD_SIGNAL: c_int = 1;
const CMD_STOP: c_int = 2;
const CMD_PING: c_int = 3;
const CMD_MAX: c_int = 4;

const SESSION_MAX: usize = 64;

const PING_OK: c_int = 0;
const PING_FAIL: c_int = 1;
const PING_MAX: usize = 2;

static mut __daemon: daemon = daemon {
    config: null(),
    config_real: null_mut(),
    config_base: null_mut(),
    csv_sep: null(),
    base_user: null(),
    base: null_mut(),
    sessions: list_head {
        next: null_mut(),
        prev: null_mut(),
    },
    out: null_mut(),
    perf: null_mut(),
    signal_fd: 0,
    start: 0,
};

static daemon_usage_0: &[u8] = b"perf daemon {start|signal|stop|ping} [<options>]\0";
static daemon_usage_1: &[u8] = b"perf daemon [<options>]\0";
static mut daemon_usage: [*const c_char; 3] = [
    daemon_usage_0.as_ptr() as *const c_char,
    daemon_usage_1.as_ptr() as *const c_char,
    null(),
];

static mut done: sig_atomic_t = 0;

unsafe extern "C" {
    static mut errno: c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut verbose: c_int;

    fn zalloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlcpy(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fflush(stream: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn setbuf(stream: *mut FILE, buf: *mut c_char);

    fn stat(path: *const c_char, st: *mut stat) -> c_int;
    fn mkdir(path: *const c_char, mode: u32) -> c_int;
    fn mkfifo(path: *const c_char, mode: u32) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn unlink(path: *const c_char) -> c_int;
    fn ftruncate(fd: c_int, length: off_t) -> c_int;
    fn flock(fd: c_int, operation: c_int) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn access(path: *const c_char, mode: c_int) -> c_int;
    fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char;
    fn dirname(path: *mut c_char) -> *mut c_char;
    fn chdir(path: *const c_char) -> c_int;
    fn umask(mask: u32) -> u32;

    fn fork() -> pid_t;
    fn setsid() -> pid_t;
    fn getpid() -> pid_t;
    fn execve(path: *const c_char, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;

    fn poll(fds: *mut pollfd, nfds: c_uint, timeout: c_int) -> c_int;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: c_uint) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut c_uint) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: c_uint) -> c_int;

    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    fn signalfd(fd: c_int, mask: *const sigset_t, flags: c_int) -> c_int;
    fn signal(signum: c_int, handler: usize) -> usize;

    fn time(tloc: *mut time_t) -> time_t;
    fn inotify_init1(flags: c_int) -> c_int;
    fn inotify_add_watch(fd: c_int, pathname: *const c_char, mask: u32) -> c_int;

    fn perf_config_set__load_file(file: *const c_char) -> *mut perf_config_set;
    fn perf_config_set(
        set: *mut perf_config_set,
        cb: unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
    fn perf_config_set__delete(set: *mut perf_config_set);
    fn perf_config_system() -> bool;
    fn perf_config_global() -> bool;
    fn perf_etc_perfconfig() -> *const c_char;
    fn perf_home_perfconfig() -> *const c_char;
    fn perf_basename(path: *mut c_char) -> *const c_char;
    fn perf_exe(path: *mut c_char, len: size_t);

    fn argv_split(str: *const c_char, argcp: *mut c_int) -> *mut *mut c_char;
    fn readn(fd: c_int, buf: *mut c_void, n: size_t) -> ssize_t;
    fn writen(fd: c_int, buf: *const c_void, n: size_t) -> ssize_t;
    fn filename__read_int(filename: *const c_char, value: *mut c_int) -> c_int;
    fn debug_set_file(file: *mut FILE);
    fn debug_set_display_time(set: bool);
    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *mut option,
        usagestr: *mut *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *mut *const c_char, options: *mut option) -> !;
    fn fdarray__init(fda: *mut fdarray, nr_autogrow: c_int);
    fn fdarray__add(fda: *mut fdarray, fd: c_int, events: c_short, priv_: c_int) -> c_int;
    fn fdarray__poll(fda: *mut fdarray, timeout: c_int) -> c_int;
    fn fdarray__exit(fda: *mut fdarray);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

unsafe fn list_add_tail(new_: *mut list_head, head: *mut list_head) {
    (*new_).prev = (*head).prev;
    (*new_).next = head;
    (*(*head).prev).next = new_;
    (*head).prev = new_;
}

unsafe fn list_del(entry: *mut list_head) {
    (*(*entry).next).prev = (*entry).prev;
    (*(*entry).prev).next = (*entry).next;
}

unsafe fn zfree(pptr: *mut *mut c_char) {
    if !(*pptr).is_null() {
        free(*pptr as *mut c_void);
        *pptr = null_mut();
    }
}

unsafe fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool {
    strncmp(str_, prefix, strlen(prefix)) == 0
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn WIFSIGNALED(status: c_int) -> bool {
    (((status & 0x7f) + 1) >> 1) > 0
}

unsafe fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

unsafe fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

unsafe fn WSTOPSIG(status: c_int) -> c_int {
    WEXITSTATUS(status)
}

unsafe extern "C" fn sig_handler(_sig: c_int) {
    done = 1;
}

unsafe fn daemon__add_session(config: *mut daemon, name: *mut c_char) -> *mut daemon_session {
    let session = zalloc(size_of::<daemon_session>()) as *mut daemon_session;
    if session.is_null() {
        return null_mut();
    }
    (*session).name = strdup(name);
    if (*session).name.is_null() {
        free(session as *mut c_void);
        return null_mut();
    }
    (*session).pid = -1;
    list_add_tail(&mut (*session).list, &mut (*config).sessions);
    session
}

unsafe fn daemon__find_session(daemon: *mut daemon, name: *mut c_char) -> *mut daemon_session {
    let mut pos = (*daemon).sessions.next;
    while pos != &mut (*daemon).sessions {
        let session = (pos as *mut u8).sub(core::mem::offset_of!(daemon_session, list)) as *mut daemon_session;
        if strcmp((*session).name, name) == 0 {
            return session;
        }
        pos = (*pos).next;
    }
    null_mut()
}

unsafe fn get_session_name(var: *const c_char, mut session: *mut c_char, mut len: c_int) -> c_int {
    let mut p = var.add(b"session-".len());
    while *p != b'.' as c_char && *p != 0 && {
        let old = len;
        len -= 1;
        old != 0
    } {
        *session = *p;
        session = session.add(1);
        p = p.add(1);
    }
    *session = 0;
    if *p == b'.' as c_char { 0 } else { -EINVAL }
}

unsafe extern "C" fn session_config(daemon: *mut daemon, mut var: *const c_char, value: *const c_char) -> c_int {
    let mut name = [0 as c_char; 100];
    if get_session_name(var, name.as_mut_ptr(), (name.len() - 1) as c_int) != 0 {
        return -EINVAL;
    }
    var = strchr(var, b'.' as c_int);
    if var.is_null() {
        return -EINVAL;
    }
    var = var.add(1);

    let mut session = daemon__find_session(daemon, name.as_mut_ptr());
    if session.is_null() {
        /* New session is defined. */
        session = daemon__add_session(daemon, name.as_mut_ptr());
        if session.is_null() {
            return -ENOMEM;
        }
        pr_debug(b"reconfig: found new session %s\n\0".as_ptr() as *const c_char, name.as_ptr());
        /* Trigger reconfig to start it. */
        (*session).state = daemon_session_state::RECONFIG;
    } else if (*session).state == daemon_session_state::KILL {
        /* Current session is defined, no action needed. */
        pr_debug(b"reconfig: found current session %s\n\0".as_ptr() as *const c_char, name.as_ptr());
        (*session).state = daemon_session_state::OK;
    }

    if strcmp(var, b"run\0".as_ptr() as *const c_char) == 0 {
        let mut same = false;
        if !(*session).run.is_null() {
            same = strcmp((*session).run, value) == 0;
        }
        if !same {
            if !(*session).run.is_null() {
                zfree(&mut (*session).run);
                pr_debug(b"reconfig: session %s is changed\n\0".as_ptr() as *const c_char, name.as_ptr());
            }
            (*session).run = strdup(value);
            if (*session).run.is_null() {
                return -ENOMEM;
            }
            /*
             * Either new or changed run value is defined,
             * trigger reconfig for the session.
             */
            (*session).state = daemon_session_state::RECONFIG;
        }
    }
    0
}

unsafe extern "C" fn server_config(var: *const c_char, value: *const c_char, cb: *mut c_void) -> c_int {
    let daemon = cb as *mut daemon;
    if strstarts(var, b"session-\0".as_ptr() as *const c_char) {
        session_config(daemon, var, value)
    } else if strcmp(var, b"daemon.base\0".as_ptr() as *const c_char) == 0 && (*daemon).base_user.is_null() {
        if !(*daemon).base.is_null() && strcmp((*daemon).base, value) != 0 {
            pr_err(b"failed: can't redefine base, bailing out\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        (*daemon).base = strdup(value);
        if (*daemon).base.is_null() {
            return -ENOMEM;
        }
        0
    } else {
        0
    }
}

unsafe extern "C" fn client_config(var: *const c_char, value: *const c_char, cb: *mut c_void) -> c_int {
    let daemon = cb as *mut daemon;
    if strcmp(var, b"daemon.base\0".as_ptr() as *const c_char) == 0 && (*daemon).base_user.is_null() {
        (*daemon).base = strdup(value);
        if (*daemon).base.is_null() {
            return -ENOMEM;
        }
    }
    0
}

unsafe fn check_base(daemon: *mut daemon) -> c_int {
    let mut st: stat = zeroed();
    if (*daemon).base.is_null() {
        pr_err(b"failed: base not defined\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if stat((*daemon).base, &mut st) != 0 {
        match errno {
            EACCES => {
                pr_err(b"failed: permission denied for '%s' base\n\0".as_ptr() as *const c_char, (*daemon).base);
                return -EACCES;
            }
            ENOENT => {
                pr_err(b"failed: base '%s' does not exists\n\0".as_ptr() as *const c_char, (*daemon).base);
                return -EACCES;
            }
            _ => {
                pr_err(b"failed: can't access base '%s': %m\n\0".as_ptr() as *const c_char, (*daemon).base);
                return -errno;
            }
        }
    }
    if (st.st_mode & S_IFMT) != S_IFDIR {
        pr_err(b"failed: base '%s' is not directory\n\0".as_ptr() as *const c_char, (*daemon).base);
        return -EINVAL;
    }
    0
}

unsafe fn setup_client_config(daemon: *mut daemon) -> c_int {
    let set = perf_config_set__load_file((*daemon).config_real);
    let mut err = -ENOMEM;
    if !set.is_null() {
        err = perf_config_set(set, client_config, daemon as *mut c_void);
        perf_config_set__delete(set);
    }
    if err != 0 { err } else { check_base(daemon) }
}

unsafe fn setup_server_config(daemon: *mut daemon) -> c_int {
    let mut err = -ENOMEM;
    pr_debug(b"reconfig: started\n\0".as_ptr() as *const c_char);
    /*
     * Mark all sessions for kill, the server config
     * will set following states, see explanation at
     * enum daemon_session_state declaration.
     */
    let mut pos = (*daemon).sessions.next;
    while pos != &mut (*daemon).sessions {
        let session = (pos as *mut u8).sub(core::mem::offset_of!(daemon_session, list)) as *mut daemon_session;
        (*session).state = daemon_session_state::KILL;
        pos = (*pos).next;
    }
    let set = perf_config_set__load_file((*daemon).config_real);
    if !set.is_null() {
        err = perf_config_set(set, server_config, daemon as *mut c_void);
        perf_config_set__delete(set);
    }
    if err != 0 { err } else { check_base(daemon) }
}

unsafe fn daemon_session__run(session: *mut daemon_session, daemon: *mut daemon) -> c_int {
    let mut buf = [0 as c_char; PATH_MAX];
    let mut argc: c_int = 0;
    let mut fd: c_int;
    if asprintf(
        &mut (*session).base,
        b"%s/session-%s\0".as_ptr() as *const c_char,
        (*daemon).base,
        (*session).name,
    ) < 0 {
        perror(b"failed: asprintf\0".as_ptr() as *const c_char);
        return -1;
    }
    if mkdir((*session).base, 0o755) != 0 && errno != EEXIST {
        perror(b"failed: mkdir\0".as_ptr() as *const c_char);
        return -1;
    }
    (*session).start = time(null_mut());
    (*session).pid = fork();
    if (*session).pid < 0 {
        return -1;
    }
    if (*session).pid > 0 {
        pr_info(
            b"reconfig: ruining session [%s:%d]: %s\n\0".as_ptr() as *const c_char,
            (*session).name,
            (*session).pid,
            (*session).run,
        );
        return 0;
    }
    if chdir((*session).base) != 0 {
        perror(b"failed: chdir\0".as_ptr() as *const c_char);
        return -1;
    }
    fd = open(b"/dev/null\0".as_ptr() as *const c_char, O_RDONLY);
    if fd < 0 {
        perror(b"failed: open /dev/null\0".as_ptr() as *const c_char);
        return -1;
    }
    dup2(fd, 0);
    close(fd);
    fd = open(SESSION_OUTPUT.as_ptr() as *const c_char, O_RDWR | O_CREAT | O_TRUNC, 0o644);
    if fd < 0 {
        perror(b"failed: open session output\0".as_ptr() as *const c_char);
        return -1;
    }
    dup2(fd, 1);
    dup2(fd, 2);
    close(fd);
    if mkfifo(SESSION_CONTROL.as_ptr() as *const c_char, 0o600) != 0 && errno != EEXIST {
        perror(b"failed: create control fifo\0".as_ptr() as *const c_char);
        return -1;
    }
    if mkfifo(SESSION_ACK.as_ptr() as *const c_char, 0o600) != 0 && errno != EEXIST {
        perror(b"failed: create ack fifo\0".as_ptr() as *const c_char);
        return -1;
    }
    scnprintf(
        buf.as_mut_ptr(),
        buf.len(),
        b"%s record --control=fifo:%s,%s %s\0".as_ptr() as *const c_char,
        (*daemon).perf,
        SESSION_CONTROL.as_ptr() as *const c_char,
        SESSION_ACK.as_ptr() as *const c_char,
        (*session).run,
    );
    let argv = argv_split(buf.as_ptr(), &mut argc);
    if argv.is_null() {
        exit(-1);
    }
    exit(execve((*daemon).perf, argv as *const *mut c_char, null()));
}

unsafe fn handle_signalfd(daemon: *mut daemon) -> pid_t {
    let mut si: signalfd_siginfo = zeroed();
    let mut status: c_int = 0;
    /*
     * Take signal fd data as pure signal notification and check all
     * the sessions state. The reason is that multiple signals can get
     * coalesced in kernel and we can receive only single signal even
     * if multiple SIGCHLD were generated.
     */
    let err = read((*daemon).signal_fd, &mut si as *mut _ as *mut c_void, size_of::<signalfd_siginfo>());
    if err != size_of::<signalfd_siginfo>() as ssize_t {
        pr_err(b"failed to read signal fd\n\0".as_ptr() as *const c_char);
        return -1;
    }
    let mut pos = (*daemon).sessions.next;
    while pos != &mut (*daemon).sessions {
        let session = (pos as *mut u8).sub(core::mem::offset_of!(daemon_session, list)) as *mut daemon_session;
        pos = (*pos).next;
        if (*session).pid == -1 {
            continue;
        }
        let pid = waitpid((*session).pid, &mut status, WNOHANG);
        if pid <= 0 {
            continue;
        }
        if WIFEXITED(status) {
            pr_info(b"session '%s' exited, status=%d\n\0".as_ptr() as *const c_char, (*session).name, WEXITSTATUS(status));
        } else if WIFSIGNALED(status) {
            pr_info(b"session '%s' killed (signal %d)\n\0".as_ptr() as *const c_char, (*session).name, WTERMSIG(status));
        } else if WIFSTOPPED(status) {
            pr_info(b"session '%s' stopped (signal %d)\n\0".as_ptr() as *const c_char, (*session).name, WSTOPSIG(status));
        } else {
            pr_info(b"session '%s' Unexpected status (0x%x)\n\0".as_ptr() as *const c_char, (*session).name, status);
        }
        (*session).state = daemon_session_state::KILL;
        (*session).pid = -1;
    }
    0
}

unsafe fn daemon_session__wait(session: *mut daemon_session, daemon: *mut daemon, secs: c_int) -> c_int {
    let mut pollfd_ = pollfd { fd: (*daemon).signal_fd, events: POLLIN, revents: 0 };
    let start = time(null_mut());
    loop {
        let err = poll(&mut pollfd_, 1, 1000);
        if err > 0 {
            handle_signalfd(daemon);
        } else if err < 0 {
            perror(b"failed: poll\n\0".as_ptr() as *const c_char);
            return -1;
        }
        if start + secs as time_t < time(null_mut()) {
            return -1;
        }
        if (*session).pid == -1 {
            break;
        }
    }
    0
}

unsafe fn daemon__has_alive_session(daemon: *mut daemon) -> bool {
    let mut pos = (*daemon).sessions.next;
    while pos != &mut (*daemon).sessions {
        let session = (pos as *mut u8).sub(core::mem::offset_of!(daemon_session, list)) as *mut daemon_session;
        if (*session).pid != -1 {
            return true;
        }
        pos = (*pos).next;
    }
    false
}

unsafe fn daemon__wait(daemon: *mut daemon, secs: c_int) -> c_int {
    let mut pollfd_ = pollfd { fd: (*daemon).signal_fd, events: POLLIN, revents: 0 };
    let start = time(null_mut());
    loop {
        let err = poll(&mut pollfd_, 1, 1000);
        if err > 0 {
            handle_signalfd(daemon);
        } else if err < 0 {
            perror(b"failed: poll\n\0".as_ptr() as *const c_char);
            return -1;
        }
        if start + secs as time_t < time(null_mut()) {
            return -1;
        }
        if !daemon__has_alive_session(daemon) {
            break;
        }
    }
    0
}

unsafe fn daemon_session__control(session: *mut daemon_session, msg: *const c_char, do_ack: bool) -> c_int {
    let mut pollfd_ = pollfd { fd: 0, events: POLLIN, revents: 0 };
    let mut control_path = [0 as c_char; PATH_MAX];
    let mut ack_path = [0 as c_char; PATH_MAX];
    let mut ack: c_int = -1;
    let mut buf = [0 as c_char; 20];
    let mut ret = -1;
    /* open the control file */
    scnprintf(control_path.as_mut_ptr(), control_path.len(), b"%s/%s\0".as_ptr() as *const c_char, (*session).base, SESSION_CONTROL.as_ptr() as *const c_char);
    let control = open(control_path.as_ptr(), O_WRONLY | O_NONBLOCK);
    if control < 0 {
        return -1;
    }
    if do_ack {
        /* open the ack file */
        scnprintf(ack_path.as_mut_ptr(), ack_path.len(), b"%s/%s\0".as_ptr() as *const c_char, (*session).base, SESSION_ACK.as_ptr() as *const c_char);
        ack = open(ack_path.as_ptr(), O_RDONLY, O_NONBLOCK);
        if ack < 0 {
            close(control);
            return -1;
        }
    }
    /* write the command */
    let len = strlen(msg);
    let err = writen(control, msg as *const c_void, len);
    if err != len as ssize_t {
        pr_err(b"failed: write to control pipe: %m (%s)\n\0".as_ptr() as *const c_char, control_path.as_ptr());
        goto_out_control(ack, control);
        return ret;
    }
    if !do_ack {
        goto_out_control(ack, control);
        return ret;
    }
    /* wait for an ack */
    pollfd_.fd = ack;
    if poll(&mut pollfd_, 1, 2000) == 0 {
        pr_err(b"failed: control ack timeout\n\0".as_ptr() as *const c_char);
        goto_out_control(ack, control);
        return ret;
    }
    if (pollfd_.revents & POLLIN) == 0 {
        pr_err(b"failed: did not received an ack\n\0".as_ptr() as *const c_char);
        goto_out_control(ack, control);
        return ret;
    }
    let err = read(ack, buf.as_mut_ptr() as *mut c_void, buf.len());
    if err > 0 {
        ret = strcmp(buf.as_ptr(), b"ack\n\0".as_ptr() as *const c_char);
    } else {
        perror(b"failed: read ack %d\n\0".as_ptr() as *const c_char);
    }
    goto_out_control(ack, control);
    ret
}

unsafe fn goto_out_control(ack: c_int, control: c_int) {
    if ack != -1 {
        close(ack);
    }
    close(control);
}

unsafe fn setup_server_socket(daemon: *mut daemon) -> c_int {
    let mut addr: sockaddr_un = zeroed();
    let mut path = [0 as c_char; PATH_MAX];
    let fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if fd < 0 {
        fprintf(stderr, b"socket: %m\n\0".as_ptr() as *const c_char);
        return -1;
    }
    if fcntl(fd, F_SETFD, FD_CLOEXEC) != 0 {
        perror(b"failed: fcntl FD_CLOEXEC\0".as_ptr() as *const c_char);
        close(fd);
        return -1;
    }
    scnprintf(path.as_mut_ptr(), path.len(), b"%s/control\0".as_ptr() as *const c_char, (*daemon).base);
    if strlen(path.as_ptr()) + 1 >= addr.sun_path.len() {
        pr_err(b"failed: control path too long '%s'\n\0".as_ptr() as *const c_char, path.as_ptr());
        close(fd);
        return -1;
    }
    memset(&mut addr as *mut _ as *mut c_void, 0, size_of::<sockaddr_un>());
    addr.sun_family = AF_UNIX as u16;
    strlcpy(addr.sun_path.as_mut_ptr(), path.as_ptr(), addr.sun_path.len() - 1);
    unlink(path.as_ptr());
    if bind(fd, &addr as *const _ as *const sockaddr, size_of::<sockaddr_un>() as c_uint) == -1 {
        perror(b"failed: bind\0".as_ptr() as *const c_char);
        close(fd);
        return -1;
    }
    if listen(fd, 1) == -1 {
        perror(b"failed: listen\0".as_ptr() as *const c_char);
        close(fd);
        return -1;
    }
    fd
}

unsafe fn daemon_session__ping(session: *mut daemon_session) -> c_int {
    if daemon_session__control(session, b"ping\0".as_ptr() as *const c_char, true) != 0 { PING_FAIL } else { PING_OK }
}

static ping_str_ok: &[u8] = b"OK\0";
static ping_str_fail: &[u8] = b"FAIL\0";
static mut ping_str: [*const c_char; PING_MAX] = [
    ping_str_ok.as_ptr() as *const c_char,
    ping_str_fail.as_ptr() as *const c_char,
];

unsafe fn cmd_session_list(daemon: *mut daemon, cmd: *mut cmd, out: *mut FILE) -> c_int {
    let csv_sep = (*cmd).list.csv_sep;
    let curr = time(null_mut());
    if csv_sep != 0 {
        fprintf(out, b"%d%c%s%c%s%c%s/%s\0".as_ptr() as *const c_char, getpid(), csv_sep as c_int, b"daemon\0".as_ptr() as *const c_char, csv_sep as c_int, (*daemon).base, csv_sep as c_int, (*daemon).base, SESSION_OUTPUT.as_ptr() as *const c_char);
        fprintf(out, b"%c%s/%s\0".as_ptr() as *const c_char, csv_sep as c_int, (*daemon).base, b"lock\0".as_ptr() as *const c_char);
        fprintf(out, b"%c%llu\0".as_ptr() as *const c_char, csv_sep as c_int, ((curr - (*daemon).start) / 60) as u64);
        fprintf(out, b"\n\0".as_ptr() as *const c_char);
    } else {
        fprintf(out, b"[%d:daemon] base: %s\n\0".as_ptr() as *const c_char, getpid(), (*daemon).base);
        if (*cmd).list.verbose != 0 {
            fprintf(out, b"  output:  %s/%s\n\0".as_ptr() as *const c_char, (*daemon).base, SESSION_OUTPUT.as_ptr() as *const c_char);
            fprintf(out, b"  lock:    %s/lock\n\0".as_ptr() as *const c_char, (*daemon).base);
            fprintf(out, b"  up:      %llu minutes\n\0".as_ptr() as *const c_char, ((curr - (*daemon).start) / 60) as u64);
        }
    }
    let mut pos = (*daemon).sessions.next;
    while pos != &mut (*daemon).sessions {
        let session = (pos as *mut u8).sub(core::mem::offset_of!(daemon_session, list)) as *mut daemon_session;
        if csv_sep != 0 {
            fprintf(out, b"%d%c%s%c%s\0".as_ptr() as *const c_char, (*session).pid, csv_sep as c_int, (*session).name, csv_sep as c_int, (*session).run);
            fprintf(out, b"%c%s%c%s/%s\0".as_ptr() as *const c_char, csv_sep as c_int, (*session).base, csv_sep as c_int, (*session).base, SESSION_OUTPUT.as_ptr() as *const c_char);
            fprintf(out, b"%c%s/%s%c%s/%s\0".as_ptr() as *const c_char, csv_sep as c_int, (*session).base, SESSION_CONTROL.as_ptr() as *const c_char, csv_sep as c_int, (*session).base, SESSION_ACK.as_ptr() as *const c_char);
            fprintf(out, b"%c%llu\0".as_ptr() as *const c_char, csv_sep as c_int, ((curr - (*session).start) / 60) as u64);
            fprintf(out, b"\n\0".as_ptr() as *const c_char);
        } else {
            fprintf(out, b"[%d:%s] perf record %s\n\0".as_ptr() as *const c_char, (*session).pid, (*session).name, (*session).run);
            if (*cmd).list.verbose == 0 {
                pos = (*pos).next;
                continue;
            }
            fprintf(out, b"  base:    %s\n\0".as_ptr() as *const c_char, (*session).base);
            fprintf(out, b"  output:  %s/%s\n\0".as_ptr() as *const c_char, (*session).base, SESSION_OUTPUT.as_ptr() as *const c_char);
            fprintf(out, b"  control: %s/%s\n\0".as_ptr() as *const c_char, (*session).base, SESSION_CONTROL.as_ptr() as *const c_char);
            fprintf(out, b"  ack:     %s/%s\n\0".as_ptr() as *const c_char, (*session).base, SESSION_ACK.as_ptr() as *const c_char);
            fprintf(out, b"  up:      %llu minutes\n\0".as_ptr() as *const c_char, ((curr - (*session).start) / 60) as u64);
        }
        pos = (*pos).next;
    }
    0
}

unsafe fn daemon_session__signal(session: *mut daemon_session, sig: c_int) -> c_int {
    if (*session).pid < 0 {
        return -1;
    }
    kill((*session).pid, sig)
}

unsafe fn cmd_session_kill(daemon: *mut daemon, cmd: *mut cmd, out: *mut FILE) -> c_int {
    let all = strcmp((*cmd).signal.name.as_ptr(), b"all\0".as_ptr() as *const c_char) == 0;
    let mut pos = (*daemon).sessions.next;
    while pos != &mut (*daemon).sessions {
        let session = (pos as *mut u8).sub(core::mem::offset_of!(daemon_session, list)) as *mut daemon_session;
        if all || strcmp((*cmd).signal.name.as_ptr(), (*session).name) == 0 {
            daemon_session__signal(session, (*cmd).signal.sig);
            fprintf(out, b"signal %d sent to session '%s [%d]'\n\0".as_ptr() as *const c_char, (*cmd).signal.sig, (*session).name, (*session).pid);
        }
        pos = (*pos).next;
    }
    0
}

unsafe fn cmd_session_ping(daemon: *mut daemon, cmd: *mut cmd, out: *mut FILE) -> c_int {
    let mut found = false;
    let all = strcmp((*cmd).ping.name.as_ptr(), b"all\0".as_ptr() as *const c_char) == 0;
    let mut pos = (*daemon).sessions.next;
    while pos != &mut (*daemon).sessions {
        let session = (pos as *mut u8).sub(core::mem::offset_of!(daemon_session, list)) as *mut daemon_session;
        if all || strcmp((*cmd).ping.name.as_ptr(), (*session).name) == 0 {
            let state = daemon_session__ping(session);
            fprintf(out, b"%-4s %s\n\0".as_ptr() as *const c_char, ping_str[state as usize], (*session).name);
            found = true;
        }
        pos = (*pos).next;
    }
    if !found && !all {
        fprintf(out, b"%-4s %s (not found)\n\0".as_ptr() as *const c_char, ping_str[PING_FAIL as usize], (*cmd).ping.name.as_ptr());
    }
    0
}

unsafe fn handle_server_socket(daemon: *mut daemon, sock_fd: c_int) -> c_int {
    let mut ret = -1;
    let mut out: *mut FILE = null_mut();
    let mut cmd_: cmd = zeroed();
    let fd = accept(sock_fd, null_mut(), null_mut());
    if fd < 0 {
        perror(b"failed: accept\0".as_ptr() as *const c_char);
        return -1;
    }
    if size_of::<cmd>() as ssize_t != readn(fd, &mut cmd_ as *mut _ as *mut c_void, size_of::<cmd>()) {
        perror(b"failed: read\0".as_ptr() as *const c_char);
        close(fd);
        return ret;
    }
    out = fdopen(fd, b"w\0".as_ptr() as *const c_char);
    if out.is_null() {
        perror(b"failed: fdopen\0".as_ptr() as *const c_char);
        close(fd);
        return ret;
    }
    match cmd_.cmd {
        CMD_LIST => ret = cmd_session_list(daemon, &mut cmd_, out),
        CMD_SIGNAL => ret = cmd_session_kill(daemon, &mut cmd_, out),
        CMD_STOP => {
            done = 1;
            ret = 0;
            pr_debug(b"perf daemon is exciting\n\0".as_ptr() as *const c_char);
        }
        CMD_PING => ret = cmd_session_ping(daemon, &mut cmd_, out),
        _ => {}
    }
    fclose(out);
    ret
}

unsafe fn setup_client_socket(daemon: *mut daemon) -> c_int {
    let mut addr: sockaddr_un = zeroed();
    let mut path = [0 as c_char; PATH_MAX];
    let fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if fd == -1 {
        perror(b"failed: socket\0".as_ptr() as *const c_char);
        return -1;
    }
    scnprintf(path.as_mut_ptr(), path.len(), b"%s/control\0".as_ptr() as *const c_char, (*daemon).base);
    if strlen(path.as_ptr()) + 1 >= addr.sun_path.len() {
        pr_err(b"failed: control path too long '%s'\n\0".as_ptr() as *const c_char, path.as_ptr());
        close(fd);
        return -1;
    }
    memset(&mut addr as *mut _ as *mut c_void, 0, size_of::<sockaddr_un>());
    addr.sun_family = AF_UNIX as u16;
    strlcpy(addr.sun_path.as_mut_ptr(), path.as_ptr(), addr.sun_path.len() - 1);
    if connect(fd, &addr as *const _ as *const sockaddr, size_of::<sockaddr_un>() as c_uint) == -1 {
        perror(b"failed: connect\0".as_ptr() as *const c_char);
        close(fd);
        return -1;
    }
    fd
}

unsafe fn daemon_session__kill(session: *mut daemon_session, daemon: *mut daemon) {
    let mut how = 0;
    loop {
        match how {
            0 => { daemon_session__control(session, b"stop\0".as_ptr() as *const c_char, false); }
            1 => { daemon_session__signal(session, SIGTERM); }
            2 => { daemon_session__signal(session, SIGKILL); }
            _ => {
                pr_err(b"failed to wait for session %s\n\0".as_ptr() as *const c_char, (*session).name);
                return;
            }
        }
        how += 1;
        if daemon_session__wait(session, daemon, 10) == 0 {
            break;
        }
    }
}

unsafe fn daemon__signal(daemon: *mut daemon, sig: c_int) {
    let mut pos = (*daemon).sessions.next;
    while pos != &mut (*daemon).sessions {
        let session = (pos as *mut u8).sub(core::mem::offset_of!(daemon_session, list)) as *mut daemon_session;
        daemon_session__signal(session, sig);
        pos = (*pos).next;
    }
}

unsafe fn daemon_session__delete(session: *mut daemon_session) {
    zfree(&mut (*session).base);
    zfree(&mut (*session).name);
    zfree(&mut (*session).run);
    free(session as *mut c_void);
}

unsafe fn daemon_session__remove(session: *mut daemon_session) {
    list_del(&mut (*session).list);
    daemon_session__delete(session);
}

unsafe fn daemon__stop(daemon: *mut daemon) {
    let mut pos = (*daemon).sessions.next;
    while pos != &mut (*daemon).sessions {
        let session = (pos as *mut u8).sub(core::mem::offset_of!(daemon_session, list)) as *mut daemon_session;
        daemon_session__control(session, b"stop\0".as_ptr() as *const c_char, false);
        pos = (*pos).next;
    }
}

unsafe fn daemon__kill(daemon: *mut daemon) {
    let mut how = 0;
    loop {
        match how {
            0 => daemon__stop(daemon),
            1 => daemon__signal(daemon, SIGTERM),
            2 => daemon__signal(daemon, SIGKILL),
            _ => {
                pr_err(b"failed to wait for sessions\n\0".as_ptr() as *const c_char);
                return;
            }
        }
        how += 1;
        if daemon__wait(daemon, 10) == 0 {
            break;
        }
    }
}

unsafe fn daemon__exit(daemon: *mut daemon) {
    let mut pos = (*daemon).sessions.next;
    while pos != &mut (*daemon).sessions {
        let next = (*pos).next;
        let session = (pos as *mut u8).sub(core::mem::offset_of!(daemon_session, list)) as *mut daemon_session;
        daemon_session__remove(session);
        pos = next;
    }
    zfree(&mut (*daemon).config_real);
    zfree(&mut (*daemon).config_base);
    zfree(&mut (*daemon).base);
}

unsafe fn daemon__reconfig(daemon: *mut daemon) -> c_int {
    let mut pos = (*daemon).sessions.next;
    while pos != &mut (*daemon).sessions {
        let next = (*pos).next;
        let session = (pos as *mut u8).sub(core::mem::offset_of!(daemon_session, list)) as *mut daemon_session;
        /* No change. */
        if (*session).state == daemon_session_state::OK {
            pos = next;
            continue;
        }
        /* Remove session. */
        if (*session).state == daemon_session_state::KILL {
            if (*session).pid > 0 {
                daemon_session__kill(session, daemon);
                pr_info(b"reconfig: session '%s' killed\n\0".as_ptr() as *const c_char, (*session).name);
            }
            daemon_session__remove(session);
            pos = next;
            continue;
        }
        /* Reconfig session. */
        if (*session).pid > 0 {
            daemon_session__kill(session, daemon);
            pr_info(b"reconfig: session '%s' killed\n\0".as_ptr() as *const c_char, (*session).name);
        }
        if daemon_session__run(session, daemon) != 0 {
            return -1;
        }
        (*session).state = daemon_session_state::OK;
        pos = next;
    }
    0
}

unsafe fn setup_config_changes(daemon: *mut daemon) -> c_int {
    let basen = strdup((*daemon).config_real);
    let dirn = strdup((*daemon).config_real);
    let mut wd = -1;
    let mut fd = -1;
    if dirn.is_null() || basen.is_null() {
        free(basen as *mut c_void);
        free(dirn as *mut c_void);
        return -1;
    }
    fd = inotify_init1(IN_NONBLOCK | O_CLOEXEC);
    if fd < 0 {
        perror(b"failed: inotify_init\0".as_ptr() as *const c_char);
        free(basen as *mut c_void);
        free(dirn as *mut c_void);
        return -1;
    }
    let dir = dirname(dirn);
    let base = perf_basename(basen);
    pr_debug(b"config file: %s, dir: %s\n\0".as_ptr() as *const c_char, base, dir);
    wd = inotify_add_watch(fd, dir, IN_CLOSE_WRITE);
    if wd >= 0 {
        (*daemon).config_base = strdup(base);
        if (*daemon).config_base.is_null() {
            close(fd);
            wd = -1;
        }
    } else {
        perror(b"failed: inotify_add_watch\0".as_ptr() as *const c_char);
    }
    free(basen as *mut c_void);
    free(dirn as *mut c_void);
    if wd < 0 { -1 } else { fd }
}

unsafe fn process_inotify_event(daemon: *mut daemon, buf: *mut c_char, len: ssize_t) -> bool {
    let mut p = buf;
    while p < buf.offset(len) {
        let event = p as *mut inotify_event;
        /*
         * We monitor config directory, check if our
         * config file was changes.
         */
        if ((*event).mask & IN_CLOSE_WRITE) != 0 && ((*event).mask & IN_ISDIR) == 0 {
            if strcmp((*event).name.as_ptr(), (*daemon).config_base) == 0 {
                return true;
            }
        }
        p = p.add(size_of::<inotify_event>() + (*event).len as usize);
    }
    false
}

unsafe fn handle_config_changes(daemon: *mut daemon, conf_fd: c_int, config_changed: *mut bool) -> c_int {
    let mut buf = [0 as c_char; 4096];
    while !*config_changed {
        let len = read(conf_fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        if len == -1 {
            if errno != EAGAIN {
                perror(b"failed: read\0".as_ptr() as *const c_char);
                return -1;
            }
            return 0;
        }
        *config_changed = process_inotify_event(daemon, buf.as_mut_ptr(), len);
    }
    0
}

unsafe fn setup_config(daemon: *mut daemon) -> c_int {
    if !(*daemon).base_user.is_null() {
        (*daemon).base = strdup((*daemon).base_user);
        if (*daemon).base.is_null() {
            return -ENOMEM;
        }
    }
    if !(*daemon).config.is_null() {
        let real = realpath((*daemon).config, null_mut());
        if real.is_null() {
            perror(b"failed: realpath\0".as_ptr() as *const c_char);
            return -1;
        }
        (*daemon).config_real = real;
        return 0;
    }
    if perf_config_system() && access(perf_etc_perfconfig(), R_OK) == 0 {
        (*daemon).config_real = strdup(perf_etc_perfconfig());
    } else if perf_config_global() && !perf_home_perfconfig().is_null() {
        (*daemon).config_real = strdup(perf_home_perfconfig());
    }
    if !(*daemon).config_real.is_null() { 0 } else { -1 }
}

// F_TLOCK fallback from C: if the platform lacks F_TLOCK, lockf maps to flock.
unsafe fn lockf(fd: c_int, cmd: c_int, len: off_t) -> c_int {
    if cmd != F_TLOCK || len != 0 {
        return -1;
    }
    flock(fd, LOCK_EX | LOCK_NB)
}

/*
 * Each daemon tries to create and lock BASE/lock file,
 * if it's successful we are sure we're the only daemon
 * running over the BASE.
 *
 * Once daemon is finished, file descriptor to lock file
 * is closed and lock is released.
 */
unsafe fn check_lock(daemon: *mut daemon) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    let mut buf = [0 as c_char; 20];
    let mut pid: c_int = 0;
    scnprintf(path.as_mut_ptr(), path.len(), b"%s/lock\0".as_ptr() as *const c_char, (*daemon).base);
    let fd = open(path.as_ptr(), O_RDWR | O_CREAT | O_CLOEXEC, 0o640);
    if fd < 0 {
        return -1;
    }
    if lockf(fd, F_TLOCK, 0) < 0 {
        filename__read_int(path.as_ptr(), &mut pid);
        fprintf(stderr, b"failed: another perf daemon (pid %d) owns %s\n\0".as_ptr() as *const c_char, pid, (*daemon).base);
        close(fd);
        return -1;
    }
    scnprintf(buf.as_mut_ptr(), buf.len(), b"%d\0".as_ptr() as *const c_char, getpid());
    let len = strlen(buf.as_ptr());
    if write(fd, buf.as_ptr() as *const c_void, len) != len as ssize_t {
        perror(b"failed: write\0".as_ptr() as *const c_char);
        close(fd);
        return -1;
    }
    if ftruncate(fd, len as off_t) != 0 {
        perror(b"failed: ftruncate\0".as_ptr() as *const c_char);
        close(fd);
        return -1;
    }
    0
}

unsafe fn go_background(daemon: *mut daemon) -> c_int {
    let pid = fork();
    if pid < 0 {
        return -1;
    }
    if pid > 0 {
        return 1;
    }
    if setsid() < 0 {
        return -1;
    }
    if check_lock(daemon) != 0 {
        return -1;
    }
    umask(0);
    if chdir((*daemon).base) != 0 {
        perror(b"failed: chdir\0".as_ptr() as *const c_char);
        return -1;
    }
    let fd = open(b"output\0".as_ptr() as *const c_char, O_RDWR | O_CREAT | O_TRUNC, 0o644);
    if fd < 0 {
        perror(b"failed: open\0".as_ptr() as *const c_char);
        return -1;
    }
    if fcntl(fd, F_SETFD, FD_CLOEXEC) != 0 {
        perror(b"failed: fcntl FD_CLOEXEC\0".as_ptr() as *const c_char);
        close(fd);
        return -1;
    }
    close(0);
    dup2(fd, 1);
    dup2(fd, 2);
    close(fd);
    (*daemon).out = fdopen(1, b"w\0".as_ptr() as *const c_char);
    if (*daemon).out.is_null() {
        close(1);
        close(2);
        return -1;
    }
    setbuf((*daemon).out, null_mut());
    0
}

unsafe fn setup_signalfd(daemon: *mut daemon) -> c_int {
    let mut mask: sigset_t = zeroed();
    sigemptyset(&mut mask);
    sigaddset(&mut mask, SIGCHLD);
    if sigprocmask(SIG_BLOCK, &mask, null_mut()) == -1 {
        return -1;
    }
    (*daemon).signal_fd = signalfd(-1, &mask, SFD_NONBLOCK | SFD_CLOEXEC);
    (*daemon).signal_fd
}

unsafe fn __cmd_start(daemon: *mut daemon, parent_options: *mut option, mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut foreground = false;
    /* OPT_BOOLEAN('f', "foreground", &foreground, "stay on console"),
     * OPT_PARENT(parent_options),
     * OPT_END()
     */
    let mut start_options: [option; 1] = [zeroed()];
    let mut sock_fd = -1;
    let mut conf_fd = -1;
    let mut signal_fd = -1;
    let mut fda: fdarray = zeroed();
    let mut err = 0;
    argc = parse_options(argc, argv, start_options.as_mut_ptr(), daemon_usage.as_mut_ptr(), 0);
    if argc != 0 {
        usage_with_options(daemon_usage.as_mut_ptr(), start_options.as_mut_ptr());
    }
    (*daemon).start = time(null_mut());
    if setup_config(daemon) != 0 {
        pr_err(b"failed: config not found\n\0".as_ptr() as *const c_char);
        return -1;
    }
    if setup_server_config(daemon) != 0 {
        return -1;
    }
    if foreground && check_lock(daemon) != 0 {
        return -1;
    }
    if !foreground {
        err = go_background(daemon);
        if err != 0 {
            /* original process, exit normally */
            if err == 1 {
                err = 0;
            }
            daemon__exit(daemon);
            return err;
        }
    }
    debug_set_file((*daemon).out);
    debug_set_display_time(true);
    pr_info(b"daemon started (pid %d)\n\0".as_ptr() as *const c_char, getpid());
    fdarray__init(&mut fda, 3);
    sock_fd = setup_server_socket(daemon);
    if sock_fd < 0 { goto_out_start(daemon, &mut fda, sock_fd, conf_fd, signal_fd); return err; }
    conf_fd = setup_config_changes(daemon);
    if conf_fd < 0 { goto_out_start(daemon, &mut fda, sock_fd, conf_fd, signal_fd); return err; }
    signal_fd = setup_signalfd(daemon);
    if signal_fd < 0 { goto_out_start(daemon, &mut fda, sock_fd, conf_fd, signal_fd); return err; }
    let sock_pos = fdarray__add(&mut fda, sock_fd, POLLIN | POLLERR | POLLHUP, 0);
    if sock_pos < 0 { goto_out_start(daemon, &mut fda, sock_fd, conf_fd, signal_fd); return err; }
    let file_pos = fdarray__add(&mut fda, conf_fd, POLLIN | POLLERR | POLLHUP, 0);
    if file_pos < 0 { goto_out_start(daemon, &mut fda, sock_fd, conf_fd, signal_fd); return err; }
    let signal_pos = fdarray__add(&mut fda, signal_fd, POLLIN | POLLERR | POLLHUP, 0);
    if signal_pos < 0 { goto_out_start(daemon, &mut fda, sock_fd, conf_fd, signal_fd); return err; }
    signal(SIGINT, sig_handler as usize);
    signal(SIGTERM, sig_handler as usize);
    signal(SIGPIPE, SIG_IGN);
    while done == 0 && err == 0 {
        err = daemon__reconfig(daemon);
        if err == 0 && fdarray__poll(&mut fda, -1) != 0 {
            let mut reconfig = false;
            let entries = fda.entries;
            if ((*entries.add(sock_pos as usize)).revents & POLLIN) != 0 {
                err = handle_server_socket(daemon, sock_fd);
            }
            if ((*entries.add(file_pos as usize)).revents & POLLIN) != 0 {
                err = handle_config_changes(daemon, conf_fd, &mut reconfig);
            }
            if ((*entries.add(signal_pos as usize)).revents & POLLIN) != 0 {
                err = (handle_signalfd(daemon) < 0) as c_int;
            }
            if reconfig {
                err = setup_server_config(daemon);
            }
        }
    }
    goto_out_start(daemon, &mut fda, sock_fd, conf_fd, signal_fd);
    err
}

unsafe fn goto_out_start(daemon: *mut daemon, fda: *mut fdarray, sock_fd: c_int, conf_fd: c_int, signal_fd: c_int) {
    fdarray__exit(fda);
    daemon__kill(daemon);
    daemon__exit(daemon);
    if sock_fd != -1 { close(sock_fd); }
    if conf_fd != -1 { close(conf_fd); }
    if signal_fd != -1 { close(signal_fd); }
    pr_info(b"daemon exited\n\0".as_ptr() as *const c_char);
    fclose((*daemon).out);
}

unsafe fn send_cmd(daemon: *mut daemon, cmd: *mut cmd) -> c_int {
    let mut ret = -1;
    let mut line: *mut c_char = null_mut();
    let mut len: size_t = 0;
    let mut in_: *mut FILE = null_mut();
    if setup_client_config(daemon) != 0 {
        return -1;
    }
    let fd = setup_client_socket(daemon);
    if fd < 0 {
        return -1;
    }
    if size_of::<cmd>() as ssize_t != writen(fd, cmd as *const c_void, size_of::<cmd>()) {
        perror(b"failed: write\0".as_ptr() as *const c_char);
        close(fd);
        return ret;
    }
    in_ = fdopen(fd, b"r\0".as_ptr() as *const c_char);
    if in_.is_null() {
        perror(b"failed: fdopen\0".as_ptr() as *const c_char);
        close(fd);
        return ret;
    }
    loop {
        let nread = getline(&mut line, &mut len, in_);
        if nread == -1 {
            break;
        }
        if fwrite(line as *const c_void, nread as size_t, 1, stdout) != 1 {
            fclose(in_);
            free(line as *mut c_void);
            return ret;
        }
        fflush(stdout);
    }
    ret = 0;
    fclose(in_);
    free(line as *mut c_void);
    ret
}

unsafe fn send_cmd_list(daemon: *mut daemon) -> c_int {
    let mut cmd_: cmd = zeroed();
    cmd_.list.cmd = CMD_LIST;
    cmd_.list.verbose = verbose;
    cmd_.list.csv_sep = if !(*daemon).csv_sep.is_null() { *(*daemon).csv_sep } else { 0 };
    send_cmd(daemon, &mut cmd_)
}

unsafe fn __cmd_signal(daemon: *mut daemon, parent_options: *mut option, mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut name = b"all\0".as_ptr() as *const c_char;
    /* OPT_STRING(0, "session", &name, "session", "Sent signal to specific session"),
     * OPT_PARENT(parent_options),
     * OPT_END()
     */
    let mut start_options: [option; 1] = [zeroed()];
    argc = parse_options(argc, argv, start_options.as_mut_ptr(), daemon_usage.as_mut_ptr(), 0);
    if argc != 0 {
        usage_with_options(daemon_usage.as_mut_ptr(), start_options.as_mut_ptr());
    }
    if setup_config(daemon) != 0 {
        pr_err(b"failed: config not found\n\0".as_ptr() as *const c_char);
        return -1;
    }
    let mut cmd_: cmd = zeroed();
    cmd_.signal.cmd = CMD_SIGNAL;
    cmd_.signal.sig = SIGUSR2;
    strncpy(cmd_.signal.name.as_mut_ptr(), name, cmd_.signal.name.len() - 1);
    send_cmd(daemon, &mut cmd_)
}

unsafe fn __cmd_stop(daemon: *mut daemon, parent_options: *mut option, mut argc: c_int, argv: *const *const c_char) -> c_int {
    /* OPT_PARENT(parent_options), OPT_END() */
    let mut start_options: [option; 1] = [zeroed()];
    argc = parse_options(argc, argv, start_options.as_mut_ptr(), daemon_usage.as_mut_ptr(), 0);
    if argc != 0 {
        usage_with_options(daemon_usage.as_mut_ptr(), start_options.as_mut_ptr());
    }
    if setup_config(daemon) != 0 {
        pr_err(b"failed: config not found\n\0".as_ptr() as *const c_char);
        return -1;
    }
    let mut cmd_: cmd = zeroed();
    cmd_.cmd = CMD_STOP;
    send_cmd(daemon, &mut cmd_)
}

unsafe fn __cmd_ping(daemon: *mut daemon, parent_options: *mut option, mut argc: c_int, argv: *const *const c_char) -> c_int {
    let mut name = b"all\0".as_ptr() as *const c_char;
    /* OPT_STRING(0, "session", &name, "session", "Ping to specific session"),
     * OPT_PARENT(parent_options),
     * OPT_END()
     */
    let mut ping_options: [option; 1] = [zeroed()];
    argc = parse_options(argc, argv, ping_options.as_mut_ptr(), daemon_usage.as_mut_ptr(), 0);
    if argc != 0 {
        usage_with_options(daemon_usage.as_mut_ptr(), ping_options.as_mut_ptr());
    }
    if setup_config(daemon) != 0 {
        pr_err(b"failed: config not found\n\0".as_ptr() as *const c_char);
        return -1;
    }
    let mut cmd_: cmd = zeroed();
    cmd_.cmd = CMD_PING;
    scnprintf(cmd_.ping.name.as_mut_ptr(), cmd_.ping.name.len(), b"%s\0".as_ptr() as *const c_char, name);
    send_cmd(daemon, &mut cmd_)
}

unsafe fn alloc_perf_exe_path() -> *mut c_char {
    let mut path = [0 as c_char; PATH_MAX];
    perf_exe(path.as_mut_ptr(), path.len());
    strdup(path.as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn cmd_daemon(mut argc: c_int, argv: *const *const c_char) -> c_int {
    /* struct option daemon_options[] = {
     *     OPT_INCR('v', "verbose", &verbose, "be more verbose"),
     *     OPT_STRING(0, "config", &__daemon.config, "config file", "config file path"),
     *     OPT_STRING(0, "base", &__daemon.base_user, "directory", "base directory"),
     *     OPT_STRING_OPTARG('x', "field-separator", &__daemon.csv_sep,
     *                       "field separator", "print counts with custom separator", ","),
     *     OPT_END()
     * };
     */
    let mut daemon_options: [option; 1] = [zeroed()];
    let mut ret = -1;
    if __daemon.sessions.next.is_null() {
        __daemon.sessions.next = &mut __daemon.sessions;
        __daemon.sessions.prev = &mut __daemon.sessions;
    }
    __daemon.perf = alloc_perf_exe_path();
    if __daemon.perf.is_null() {
        return -ENOMEM;
    }
    __daemon.out = stdout;
    argc = parse_options(argc, argv, daemon_options.as_mut_ptr(), daemon_usage.as_mut_ptr(), 0);
    if argc != 0 {
        let first = *argv;
        if strcmp(first, b"start\0".as_ptr() as *const c_char) == 0 {
            ret = __cmd_start(&mut __daemon, daemon_options.as_mut_ptr(), argc, argv);
        } else if strcmp(first, b"signal\0".as_ptr() as *const c_char) == 0 {
            ret = __cmd_signal(&mut __daemon, daemon_options.as_mut_ptr(), argc, argv);
        } else if strcmp(first, b"stop\0".as_ptr() as *const c_char) == 0 {
            ret = __cmd_stop(&mut __daemon, daemon_options.as_mut_ptr(), argc, argv);
        } else if strcmp(first, b"ping\0".as_ptr() as *const c_char) == 0 {
            ret = __cmd_ping(&mut __daemon, daemon_options.as_mut_ptr(), argc, argv);
        } else {
            pr_err(b"failed: unknown command '%s'\n\0".as_ptr() as *const c_char, first);
        }
    } else {
        ret = setup_config(&mut __daemon);
        if ret != 0 {
            pr_err(b"failed: config not found\n\0".as_ptr() as *const c_char);
        } else {
            ret = send_cmd_list(&mut __daemon);
        }
    }
    zfree(&mut __daemon.perf);
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
