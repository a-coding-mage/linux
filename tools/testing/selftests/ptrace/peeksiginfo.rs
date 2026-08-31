// SPDX-License-Identifier: GPL-2.0
// C source used _GNU_SOURCE and included libc/Linux ptrace headers.

use libc::{
    c_int, c_long, c_void, fork, getppid, mmap, munmap, pid_t, printf, sigaddset, sigemptyset,
    siginfo_t, sigprocmask, sigset_t, sleep, syscall, waitpid, MAP_ANONYMOUS, MAP_FAILED,
    MAP_FIXED, MAP_PRIVATE, PROT_READ, PROT_WRITE, SIGRTMIN, SIG_BLOCK, SYS_ptrace,
    SYS_rt_sigqueueinfo, SYS_rt_tgsigqueueinfo, EFAULT, EINVAL, PTRACE_ATTACH, PTRACE_KILL,
    PTRACE_PEEKSIGINFO, PTRACE_PEEKSIGINFO_SHARED,
};

#[repr(C)]
struct ptrace_peeksiginfo_args {
    off: u64,
    flags: u32,
    nr: i32,
}

unsafe fn sys_rt_sigqueueinfo(tgid: pid_t, sig: c_int, uinfo: *mut siginfo_t) -> c_int {
    syscall(SYS_rt_sigqueueinfo as c_long, tgid, sig, uinfo) as c_int
}

unsafe fn sys_rt_tgsigqueueinfo(
    tgid: pid_t,
    tid: pid_t,
    sig: c_int,
    uinfo: *mut siginfo_t,
) -> c_int {
    syscall(SYS_rt_tgsigqueueinfo as c_long, tgid, tid, sig, uinfo) as c_int
}

unsafe fn sys_ptrace(request: c_int, pid: pid_t, addr: *mut c_void, data: *mut c_void) -> c_int {
    syscall(SYS_ptrace as c_long, request, pid, addr, data) as c_int
}

const SIGNR: c_int = 10;
const TEST_SICODE_PRIV: c_int = -1;
const TEST_SICODE_SHARE: c_int = -2;

unsafe fn page_size() -> usize {
    libc::sysconf(libc::_SC_PAGESIZE) as usize
}

unsafe fn errno_location() -> *mut c_int {
    libc::__errno_location()
}

unsafe fn errno_value() -> c_int {
    *errno_location()
}

macro_rules! err {
    ($($arg:tt)*) => {{
        eprint!("Error ({}:{}): ", file!(), line!());
        eprint!($($arg)*);
    }};
}

unsafe fn check_error_paths(child: pid_t) -> c_int {
    let mut arg: ptrace_peeksiginfo_args = std::mem::zeroed();
    let mut exit_code: c_int = -1;
    let addr_rw: *mut c_void;
    let addr_ro: *mut c_void;
    let ps = page_size();

    /*
     * Allocate two contiguous pages. The first one is for read-write,
     * another is for read-only.
     */
    addr_rw = mmap(
        std::ptr::null_mut(),
        2 * ps,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    );
    if addr_rw == MAP_FAILED {
        err!("mmap() failed: {}\n", std::io::Error::last_os_error());
        return 1;
    }

    addr_ro = mmap(
        (addr_rw as *mut u8).add(ps) as *mut c_void,
        ps,
        PROT_READ,
        MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED,
        -1,
        0,
    );
    if addr_ro == MAP_FAILED {
        err!("mmap() failed: {}\n", std::io::Error::last_os_error());
        goto_out(addr_rw, ps);
        return exit_code;
    }

    arg.nr = SIGNR;
    arg.off = 0;

    /* Unsupported flags */
    arg.flags = !0;
    let mut ret = sys_ptrace(
        PTRACE_PEEKSIGINFO,
        child,
        &mut arg as *mut _ as *mut c_void,
        addr_rw,
    );
    if ret != -1 || errno_value() != EINVAL {
        err!(
            "sys_ptrace() returns {} (expected -1), errno {} (expected {}): {}\n",
            ret,
            errno_value(),
            EINVAL,
            std::io::Error::last_os_error()
        );
        goto_out(addr_rw, ps);
        return exit_code;
    }
    arg.flags = 0;

    /* A part of the buffer is read-only */
    ret = sys_ptrace(
        PTRACE_PEEKSIGINFO,
        child,
        &mut arg as *mut _ as *mut c_void,
        (addr_ro as *mut u8).sub(std::mem::size_of::<siginfo_t>() * 2) as *mut c_void,
    );
    if ret != 2 {
        err!(
            "sys_ptrace() returns {} (expected 2): {}\n",
            ret,
            std::io::Error::last_os_error()
        );
        goto_out(addr_rw, ps);
        return exit_code;
    }

    /* Read-only buffer */
    ret = sys_ptrace(
        PTRACE_PEEKSIGINFO,
        child,
        &mut arg as *mut _ as *mut c_void,
        addr_ro,
    );
    if ret != -1 && errno_value() != EFAULT {
        err!(
            "sys_ptrace() returns {} (expected -1), errno {} (expected {}): {}\n",
            ret,
            errno_value(),
            EFAULT,
            std::io::Error::last_os_error()
        );
        goto_out(addr_rw, ps);
        return exit_code;
    }

    exit_code = 0;
    goto_out(addr_rw, ps);
    exit_code
}

unsafe fn goto_out(addr_rw: *mut c_void, ps: usize) {
    munmap(addr_rw, 2 * ps);
}

unsafe fn check_direct_path(child: pid_t, shared: c_int, nr: c_int) -> c_int {
    let mut arg = ptrace_peeksiginfo_args {
        flags: 0,
        nr,
        off: 0,
    };
    let mut exit_code: c_int = -1;
    let mut siginfo: [siginfo_t; SIGNR as usize] = std::mem::zeroed();
    let si_code: c_int;

    if shared == 1 {
        arg.flags = PTRACE_PEEKSIGINFO_SHARED;
        si_code = TEST_SICODE_SHARE;
    } else {
        arg.flags = 0;
        si_code = TEST_SICODE_PRIV;
    }

    let mut i: c_int = 0;
    while i < SIGNR {
        arg.off = i as u64;
        let ret = sys_ptrace(
            PTRACE_PEEKSIGINFO,
            child,
            &mut arg as *mut _ as *mut c_void,
            siginfo.as_mut_ptr() as *mut c_void,
        );
        if ret == -1 {
            err!("ptrace() failed: {}\n", std::io::Error::last_os_error());
            return exit_code;
        }

        if ret == 0 {
            break;
        }

        let mut j: c_int = 0;
        while j < ret {
            if siginfo[j as usize].si_code == si_code && siginfo[j as usize].si_int == i {
                j += 1;
                i += 1;
                continue;
            }

            err!(
                "{}: Wrong siginfo i={} si_code={} si_int={}\n",
                shared,
                i,
                siginfo[j as usize].si_code,
                siginfo[j as usize].si_int
            );
            return exit_code;
        }
    }

    if i != SIGNR {
        err!("Only {} signals were read\n", i);
        return exit_code;
    }

    exit_code = 0;
    exit_code
}

unsafe fn real_main() -> c_int {
    let mut siginfo: siginfo_t = std::mem::zeroed();
    let mut exit_code: c_int = 1;
    let mut blockmask: sigset_t = std::mem::zeroed();
    let child: pid_t;

    sigemptyset(&mut blockmask);
    sigaddset(&mut blockmask, SIGRTMIN());
    sigprocmask(SIG_BLOCK, &blockmask, std::ptr::null_mut());

    child = fork();
    if child == -1 {
        err!("fork() failed: {}", std::io::Error::last_os_error());
        return 1;
    } else if child == 0 {
        let ppid = getppid();
        loop {
            if ppid != getppid() {
                break;
            }
            sleep(1);
        }
        return 1;
    }

    /* Send signals in process-wide and per-thread queues */
    for i in 0..SIGNR {
        siginfo.si_code = TEST_SICODE_SHARE;
        siginfo.si_int = i;
        sys_rt_sigqueueinfo(child, SIGRTMIN(), &mut siginfo);

        siginfo.si_code = TEST_SICODE_PRIV;
        siginfo.si_int = i;
        sys_rt_tgsigqueueinfo(child, child, SIGRTMIN(), &mut siginfo);
    }

    if sys_ptrace(
        PTRACE_ATTACH,
        child,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ) == -1
    {
        return 1;
    }

    waitpid(child, std::ptr::null_mut(), 0);

    /* Dump signals one by one*/
    if check_direct_path(child, 0, 1) != 0 {
        goto_main_out(child);
        return exit_code;
    }
    /* Dump all signals for one call */
    if check_direct_path(child, 0, SIGNR) != 0 {
        goto_main_out(child);
        return exit_code;
    }

    /*
     * Dump signal from the process-wide queue.
     * The number of signals is not multiple to the buffer size
     */
    if check_direct_path(child, 1, 3) != 0 {
        goto_main_out(child);
        return exit_code;
    }

    if check_error_paths(child) != 0 {
        goto_main_out(child);
        return exit_code;
    }

    printf(b"PASS\n\0".as_ptr() as *const i8);
    exit_code = 0;

    goto_main_out(child);
    exit_code
}

unsafe fn goto_main_out(child: pid_t) -> c_int {
    if sys_ptrace(
        PTRACE_KILL,
        child,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ) == -1
    {
        return 1;
    }

    waitpid(child, std::ptr::null_mut(), 0);
    0
}

fn main() {
    unsafe {
        std::process::exit(real_main());
    }
}
