// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 - 2008 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Dependencies supplied by the surrounding UML codebase and libc bindings.

static mut WRITE_SIGIO_TD: *mut os_helper_thread = core::ptr::null_mut();
static mut EPOLLFD: libc::c_int = -1;

const MAX_EPOLL_EVENTS: usize = 64;

static mut EPOLL_EVENTS: [libc::epoll_event; MAX_EPOLL_EVENTS] = unsafe {
    core::mem::zeroed()
};

unsafe extern "C" {
    type os_helper_thread;

    fn os_fix_helper_thread_signals();
    fn os_run_helper_thread(
        thread: *mut *mut os_helper_thread,
        proc: unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn os_kill_helper_thread(thread: *mut os_helper_thread);
    fn os_getpid() -> libc::pid_t;
    fn sigio_lock();
    fn sigio_unlock();
    fn raw(fd: libc::c_int) -> libc::c_int;
    fn initial_thread_cb(
        proc: unsafe extern "C" fn(*mut libc::c_void),
        arg: *mut libc::c_void,
    );
    fn barrier();
    fn printk(fmt: *const libc::c_char, ...);
}

unsafe extern "C" fn write_sigio_thread(_unused: *mut libc::c_void) -> *mut libc::c_void {
    let pid = libc::getpid();
    let mut r: libc::c_int;

    os_fix_helper_thread_signals();

    loop {
        r = libc::epoll_wait(
            EPOLLFD,
            EPOLL_EVENTS.as_mut_ptr(),
            MAX_EPOLL_EVENTS as libc::c_int,
            -1,
        );
        if r < 0 {
            if *libc::__errno_location() == libc::EINTR {
                continue;
            }
            printk(c"%s: epoll_wait failed, errno = %d\n".as_ptr(), c"write_sigio_thread".as_ptr(), *libc::__errno_location());
        }

        r = libc::syscall(libc::SYS_tgkill, pid, pid, libc::SIGIO) as libc::c_int;
        if r < 0 {
            printk(c"%s: tgkill failed, errno = %d\n".as_ptr(), c"write_sigio_thread".as_ptr(), *libc::__errno_location());
        }
    }
}

pub unsafe fn __add_sigio_fd(fd: libc::c_int) -> libc::c_int {
    let mut event: libc::epoll_event = core::mem::zeroed();
    event.u64 = fd as u64;
    event.events = (libc::EPOLLIN | libc::EPOLLET) as u32;
    let r = libc::epoll_ctl(EPOLLFD, libc::EPOLL_CTL_ADD, fd, &mut event);
    if r < 0 { -*libc::__errno_location() } else { 0 }
}

pub unsafe fn add_sigio_fd(fd: libc::c_int) -> libc::c_int {
    sigio_lock();
    let err = __add_sigio_fd(fd);
    sigio_unlock();
    err
}

pub unsafe fn __ignore_sigio_fd(fd: libc::c_int) -> libc::c_int {
    let mut event: libc::epoll_event = core::mem::zeroed();
    let r = libc::epoll_ctl(EPOLLFD, libc::EPOLL_CTL_DEL, fd, &mut event);
    if r < 0 { -*libc::__errno_location() } else { 0 }
}

pub unsafe fn ignore_sigio_fd(fd: libc::c_int) -> libc::c_int {
    sigio_lock();
    let err = __ignore_sigio_fd(fd);
    sigio_unlock();
    err
}

unsafe fn write_sigio_workaround() {
    sigio_lock();
    if !WRITE_SIGIO_TD.is_null() {
        sigio_unlock();
        return;
    }

    EPOLLFD = libc::epoll_create(MAX_EPOLL_EVENTS as libc::c_int);
    if EPOLLFD < 0 {
        printk(c"%s: epoll_create failed, errno = %d\n".as_ptr(), c"write_sigio_workaround".as_ptr(), *libc::__errno_location());
        sigio_unlock();
        return;
    }

    let err = os_run_helper_thread(&mut WRITE_SIGIO_TD, write_sigio_thread, core::ptr::null_mut());
    if err < 0 {
        printk(c"%s: os_run_helper_thread failed, errno = %d\n".as_ptr(), c"write_sigio_workaround".as_ptr(), -err);
        libc::close(EPOLLFD);
        EPOLLFD = -1;
    }
    sigio_unlock();
}

pub unsafe fn sigio_broken() { write_sigio_workaround(); }

// Changed during early boot.
static mut PTY_OUTPUT_SIGIO: libc::c_int = 0;

pub unsafe fn maybe_sigio_broken(fd: libc::c_int) {
    if libc::isatty(fd) == 0 || PTY_OUTPUT_SIGIO != 0 { return; }
    sigio_broken();
}

unsafe fn sigio_cleanup() {
    if WRITE_SIGIO_TD.is_null() { return; }
    os_kill_helper_thread(WRITE_SIGIO_TD);
    WRITE_SIGIO_TD = core::ptr::null_mut();
}

// __uml_exitcall(sigio_cleanup);

// Used as a flag during SIGIO testing early in boot.
static mut GOT_SIGIO: libc::c_int = 0;

unsafe extern "C" fn handler(_sig: libc::c_int) { GOT_SIGIO = 1; }

#[repr(C)]
struct openpty_arg { master: libc::c_int, slave: libc::c_int, err: libc::c_int }

unsafe extern "C" fn openpty_cb(arg: *mut libc::c_void) {
    let info = &mut *(arg as *mut openpty_arg);
    info.err = 0;
    if libc::openpty(&mut info.master, &mut info.slave, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()) != 0 {
        info.err = -*libc::__errno_location();
    }
}

unsafe fn async_pty(master: libc::c_int, slave: libc::c_int) -> libc::c_int {
    let flags = libc::fcntl(master, libc::F_GETFL);
    if flags < 0 { return -*libc::__errno_location(); }
    if libc::fcntl(master, libc::F_SETFL, flags | libc::O_NONBLOCK | libc::O_ASYNC) < 0 || libc::fcntl(master, libc::F_SETOWN, os_getpid()) < 0 { return -*libc::__errno_location(); }
    if libc::fcntl(slave, libc::F_SETFL, flags | libc::O_NONBLOCK) < 0 { return -*libc::__errno_location(); }
    0
}

// __init handler and checks are retained below; their external signal-action types/functions are supplied by libc.
unsafe fn check_one_sigio(proc: unsafe fn(libc::c_int, libc::c_int)) {
    let mut pty = openpty_arg { master: -1, slave: -1, err: 0 };
    initial_thread_cb(openpty_cb, &mut pty as *mut _ as *mut libc::c_void);
    if pty.err != 0 {
        printk(c"check_one_sigio failed, errno = %d\n".as_ptr(), -pty.err);
        return;
    }
    let master = pty.master;
    let slave = pty.slave;
    if master == -1 || slave == -1 {
        printk(c"check_one_sigio failed to allocate a pty\n".as_ptr());
        return;
    }
    let err = raw(master);
    if err < 0 {
        printk(c"check_one_sigio : raw failed, errno = %d\n".as_ptr(), -err);
        return;
    }
    if async_pty(master, slave) < 0 {
        printk(c"check_one_sigio : sigio_async failed, err = %d\n".as_ptr(), -*libc::__errno_location());
        return;
    }
    let mut old: libc::sigaction = core::mem::zeroed();
    if libc::sigaction(libc::SIGIO, core::ptr::null(), &mut old) < 0 {
        printk(c"check_one_sigio : sigaction 1 failed, errno = %d\n".as_ptr(), *libc::__errno_location());
        return;
    }
    let mut new = old;
    new.sa_sigaction = handler as usize;
    if libc::sigaction(libc::SIGIO, &new, core::ptr::null_mut()) < 0 {
        printk(c"check_one_sigio : sigaction 2 failed, errno = %d\n".as_ptr(), *libc::__errno_location());
        return;
    }
    GOT_SIGIO = 0;
    proc(master, slave);
    libc::close(master);
    libc::close(slave);
    if libc::sigaction(libc::SIGIO, &old, core::ptr::null_mut()) < 0 {
        printk(c"check_one_sigio : sigaction 3 failed, errno = %d\n".as_ptr(), *libc::__errno_location());
    }
}

unsafe fn tty_output(master: libc::c_int, slave: libc::c_int) {
    printk(c"Checking that host ptys support output SIGIO...".as_ptr());
    let mut buf = [0u8; 512];
    buf.fill(0);
    while libc::write(master, buf.as_ptr() as *const libc::c_void, buf.len()) > 0 {}
    if *libc::__errno_location() != libc::EAGAIN {
        printk(c"tty_output : write failed, errno = %d\n".as_ptr(), *libc::__errno_location());
    }
    let mut n;
    loop {
        n = libc::read(slave, buf.as_mut_ptr() as *mut libc::c_void, buf.len());
        if n <= 0 || GOT_SIGIO != 0 { break; }
        barrier();
    }
    if GOT_SIGIO != 0 {
        printk(c"Yes\n".as_ptr());
        PTY_OUTPUT_SIGIO = 1;
    } else if n == -(libc::EAGAIN as isize) {
        printk(c"No, enabling workaround\n".as_ptr());
    } else {
        printk(c"tty_output : read failed, err = %d\n".as_ptr(), n);
    }
}

unsafe fn check_sigio() {
    if libc::access(c"/dev/ptmx".as_ptr(), libc::R_OK) < 0 && libc::access(c"/dev/ptyp0".as_ptr(), libc::R_OK) < 0 {
        printk(c"No pseudo-terminals available - skipping pty SIGIO check\n".as_ptr());
        return;
    }
    check_one_sigio(tty_output);
}

pub unsafe fn os_check_bugs() { check_sigio(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
