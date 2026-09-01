// SPDX-License-Identifier: GPL-2.0
// C dependency intent: _GNU_SOURCE; errno, fcntl, linux/perf_event, sched,
// signal, stdlib, string, ioctl, mmap, syscall, wait, unistd, and
// kselftest_harness.

use core::ffi::c_void;
use core::mem;
use core::ptr;

static mut sigio_count: libc::c_int = 0;

unsafe extern "C" fn handle_sigio(
    _signum: libc::c_int,
    _oh: *mut libc::siginfo_t,
    _uc: *mut c_void,
) {
    sigio_count += 1;
}

unsafe fn do_child() {
    libc::raise(libc::SIGSTOP);

    for _i in 0..20 {
        libc::sleep(1);
    }

    libc::raise(libc::SIGSTOP);

    libc::exit(0);
}

// Translated from TEST(watermark_signal) in kselftest_harness.
unsafe fn watermark_signal() {
    let mut attr: libc::perf_event_attr = mem::zeroed();
    let mut p: *mut libc::perf_event_mmap_page = ptr::null_mut();
    let mut previous_sigio: libc::sigaction = mem::zeroed();
    let mut sigio: libc::sigaction = mem::zeroed();
    let mut child: libc::pid_t = -1;
    let mut child_status: libc::c_int = 0;
    let mut fd: libc::c_int = -1;
    let page_size: libc::c_long = libc::sysconf(libc::_SC_PAGE_SIZE);

    sigio.sa_sigaction = handle_sigio as usize;
    libc::sigemptyset(&mut sigio.sa_mask);
    // EXPECT_EQ(sigaction(SIGIO, &sigio, &previous_sigio), 0);
    assert_eq!(
        libc::sigaction(libc::SIGIO, &sigio, &mut previous_sigio),
        0
    );

    ptr::write_bytes(
        &mut attr as *mut libc::perf_event_attr as *mut u8,
        0,
        mem::size_of::<libc::perf_event_attr>(),
    );
    attr.size = mem::size_of::<libc::perf_event_attr>() as u32;
    attr.type_ = libc::PERF_TYPE_SOFTWARE;
    attr.config = libc::PERF_COUNT_SW_DUMMY as u64;
    attr.sample_period = 1;
    attr.set_disabled(1);
    attr.set_watermark(1);
    attr.set_context_switch(1);
    attr.wakeup_watermark = 1;

    child = libc::fork();
    // EXPECT_GE(child, 0);
    assert!(child >= 0);
    if child == 0 {
        do_child();
    } else if child < 0 {
        libc::perror(c"fork()".as_ptr());
        goto_cleanup(
            p,
            page_size,
            fd,
            child,
            &mut previous_sigio,
            &mut child_status,
        );
        return;
    }

    if libc::waitpid(child, &mut child_status, libc::WSTOPPED) != child
        || !(libc::WIFSTOPPED(child_status)
            && libc::WSTOPSIG(child_status) == libc::SIGSTOP)
    {
        libc::fprintf(
            libc::stderr,
            c"failed to synchronize with child errno=%d status=%x\n".as_ptr(),
            *libc::__errno_location(),
            child_status,
        );
        goto_cleanup(
            p,
            page_size,
            fd,
            child,
            &mut previous_sigio,
            &mut child_status,
        );
        return;
    }

    fd = libc::syscall(
        libc::SYS_perf_event_open,
        &mut attr as *mut libc::perf_event_attr,
        child,
        -1,
        -1,
        libc::PERF_FLAG_FD_CLOEXEC,
    ) as libc::c_int;
    if fd < 0 {
        libc::fprintf(
            libc::stderr,
            c"failed opening event %llx\n".as_ptr(),
            attr.config,
        );
        goto_cleanup(
            p,
            page_size,
            fd,
            child,
            &mut previous_sigio,
            &mut child_status,
        );
        return;
    }

    if libc::fcntl(fd, libc::F_SETFL, libc::FASYNC) != 0 {
        libc::perror(c"F_SETFL FASYNC".as_ptr());
        goto_cleanup(
            p,
            page_size,
            fd,
            child,
            &mut previous_sigio,
            &mut child_status,
        );
        return;
    }

    if libc::fcntl(fd, libc::F_SETOWN, libc::getpid()) != 0 {
        libc::perror(c"F_SETOWN getpid()".as_ptr());
        goto_cleanup(
            p,
            page_size,
            fd,
            child,
            &mut previous_sigio,
            &mut child_status,
        );
        return;
    }

    if libc::fcntl(fd, libc::F_SETSIG, libc::SIGIO) != 0 {
        libc::perror(c"F_SETSIG SIGIO".as_ptr());
        goto_cleanup(
            p,
            page_size,
            fd,
            child,
            &mut previous_sigio,
            &mut child_status,
        );
        return;
    }

    p = libc::mmap(
        ptr::null_mut(),
        (2 * page_size) as usize,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_SHARED,
        fd,
        0,
    ) as *mut libc::perf_event_mmap_page;
    if p as *mut c_void == libc::MAP_FAILED {
        libc::perror(c"mmap".as_ptr());
        goto_cleanup(
            p,
            page_size,
            fd,
            child,
            &mut previous_sigio,
            &mut child_status,
        );
        return;
    }

    if libc::ioctl(fd, libc::PERF_EVENT_IOC_ENABLE(), 0) != 0 {
        libc::perror(c"PERF_EVENT_IOC_ENABLE".as_ptr());
        goto_cleanup(
            p,
            page_size,
            fd,
            child,
            &mut previous_sigio,
            &mut child_status,
        );
        return;
    }

    if libc::kill(child, libc::SIGCONT) < 0 {
        libc::perror(c"SIGCONT".as_ptr());
        goto_cleanup(
            p,
            page_size,
            fd,
            child,
            &mut previous_sigio,
            &mut child_status,
        );
        return;
    }

    if libc::waitpid(child, &mut child_status, libc::WSTOPPED) != -1
        || *libc::__errno_location() != libc::EINTR
    {
        libc::fprintf(
            libc::stderr,
            c"expected SIGIO to terminate wait errno=%d status=%x\n%d".as_ptr(),
            *libc::__errno_location(),
            child_status,
            sigio_count,
        );
    }

    // EXPECT_GE(sigio_count, 1);
    assert!(sigio_count >= 1);

    goto_cleanup(
        p,
        page_size,
        fd,
        child,
        &mut previous_sigio,
        &mut child_status,
    );
}

unsafe fn goto_cleanup(
    p: *mut libc::perf_event_mmap_page,
    page_size: libc::c_long,
    fd: libc::c_int,
    child: libc::pid_t,
    previous_sigio: *mut libc::sigaction,
    _child_status: *mut libc::c_int,
) {
    if !p.is_null() {
        libc::munmap(p as *mut c_void, (2 * page_size) as usize);
    }

    if fd >= 0 {
        libc::close(fd);
    }

    if child > 0 {
        libc::kill(child, libc::SIGKILL);
        libc::waitpid(child, ptr::null_mut(), 0);
    }

    libc::sigaction(libc::SIGIO, previous_sigio, ptr::null_mut());
}

// Translated from TEST_HARNESS_MAIN.
fn main() {
    unsafe {
        watermark_signal();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
