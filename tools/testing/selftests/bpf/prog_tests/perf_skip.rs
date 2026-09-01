// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and included:
// <test_progs.h>, "test_perf_skip.skel.h", <linux/compiler.h>,
// <linux/hw_breakpoint.h>, and <sys/mman.h>.

use core::ffi::{c_int, c_long, c_ulong, c_void};

const TRAP_PERF: c_int = 6;

static mut sigio_count: c_int = 0;
static mut sigtrap_count: c_int = 0;

#[repr(C)]
pub struct siginfo_t {
    pub si_code: c_int,
}

#[repr(C)]
pub struct sigset_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
}

pub type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

#[repr(C)]
pub struct test_perf_skip {
    pub progs: test_perf_skip_progs,
    pub bss: *mut test_perf_skip_bss,
}

#[repr(C)]
pub struct test_perf_skip_progs {
    pub handler: *mut bpf_program,
}

#[repr(C)]
pub struct test_perf_skip_bss {
    pub ip: usize,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
    pub wakeup_events: u32,
    pub bp_type: u32,
    pub bp_addr: u64,
    pub bp_len: u64,
}

#[repr(C)]
pub struct f_owner_ex {
    pub type_: c_int,
    pub pid: c_int,
}

const SA_SIGINFO: c_int = 4;
const SA_NODEFER: c_int = 0x40000000;
const SIGTRAP: c_int = 5;
const SIGIO: c_int = 29;
const SIG_ERR: sighandler_t = None;

const PERF_TYPE_BREAKPOINT: u32 = 5;
const HW_BREAKPOINT_X: u32 = 4;
const PERF_SAMPLE_IP: u64 = 1;

const __NR_PERF_EVENT_OPEN: c_long = 298;
const __NR_GETTID: c_long = 186;

const ENOENT: c_int = 2;
const EOPNOTSUPP: c_int = 95;

const F_SETFL: c_int = 4;
const O_ASYNC: c_int = 0o20000;
const F_OWNER_TID: c_int = 0;
const F_SETOWN_EX: c_int = 15;
const PERF_EVENT_IOC_REFRESH: c_ulong = 0x2402;

extern "C" {
    static mut errno: c_int;

    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(
        signum: c_int,
        act: *const sigaction,
        oldact: *mut sigaction,
    ) -> c_int;
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    fn syscall(number: c_long, ...) -> c_long;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn printf(format: *const u8, ...) -> c_int;

    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const u8) -> bool;
    fn ASSERT_OK(res: c_int, name: *const u8) -> bool;
    fn ASSERT_NEQ(actual: sighandler_t, expected: sighandler_t, name: *const u8) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const u8) -> bool;
    fn test__skip();

    fn test_perf_skip__open_and_load() -> *mut test_perf_skip;
    fn test_perf_skip__destroy(skel: *mut test_perf_skip);
    fn bpf_program__attach_perf_event(prog: *mut bpf_program, perf_fd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
}

unsafe extern "C" fn handle_sigio(_sig: c_int) {
    sigio_count += 1;
}

unsafe extern "C" fn handle_sigtrap(
    _signum: c_int,
    info: *mut siginfo_t,
    _ucontext: *mut c_void,
) {
    ASSERT_EQ((*info).si_code, TRAP_PERF, c"si_code".as_ptr() as *const u8);
    sigtrap_count += 1;
}

#[inline(never)]
unsafe fn test_function() -> c_int {
    core::arch::asm!("", options(nomem, nostack, preserves_flags));
    0
}

pub unsafe fn serial_test_perf_skip() {
    let mut action: sigaction = core::mem::zeroed();
    let mut previous_sigtrap: sigaction = core::mem::zeroed();
    let mut previous_sigio: sighandler_t = SIG_ERR;
    let mut skel: *mut test_perf_skip = core::ptr::null_mut();
    let mut attr: perf_event_attr = core::mem::zeroed();
    let mut perf_fd: c_int = -1;
    let mut err: c_int;
    let mut owner: f_owner_ex = core::mem::zeroed();
    let mut prog_link: *mut bpf_link = core::ptr::null_mut();

    action.sa_flags = SA_SIGINFO | SA_NODEFER;
    action.sa_sigaction = Some(handle_sigtrap);
    sigemptyset(&mut action.sa_mask);
    if !ASSERT_OK(
        sigaction(SIGTRAP, &action, &mut previous_sigtrap),
        c"sigaction".as_ptr() as *const u8,
    ) {
        return;
    }

    previous_sigio = signal(SIGIO, Some(handle_sigio));
    if !ASSERT_NEQ(previous_sigio, SIG_ERR, c"signal".as_ptr() as *const u8) {
        goto_cleanup(
            prog_link,
            perf_fd,
            skel,
            previous_sigio,
            &mut previous_sigtrap,
        );
        return;
    }

    skel = test_perf_skip__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"skel_load".as_ptr() as *const u8) {
        goto_cleanup(
            prog_link,
            perf_fd,
            skel,
            previous_sigio,
            &mut previous_sigtrap,
        );
        return;
    }

    attr.type_ = PERF_TYPE_BREAKPOINT;
    attr.size = core::mem::size_of_val(&attr) as u32;
    attr.bp_type = HW_BREAKPOINT_X;
    attr.bp_addr = test_function as usize as u64;
    attr.bp_len = core::mem::size_of::<c_long>() as u64;
    attr.sample_period = 1;
    attr.sample_type = PERF_SAMPLE_IP;
    attr.flags |= 1 << 2; // pinned
    attr.flags |= 1 << 5; // exclude_kernel
    attr.flags |= 1 << 6; // exclude_hv
    attr.flags |= 3 << 15; // precise_ip
    attr.flags |= 1 << 51; // sigtrap
    attr.flags |= 1 << 55; // remove_on_exec

    perf_fd = syscall(
        __NR_PERF_EVENT_OPEN,
        &mut attr as *mut perf_event_attr,
        0,
        -1,
        -1,
        0,
    ) as c_int;
    if perf_fd < 0 && (errno == ENOENT || errno == EOPNOTSUPP) {
        printf(c"SKIP:no PERF_TYPE_BREAKPOINT/HW_BREAKPOINT_X\n".as_ptr() as *const u8);
        test__skip();
        goto_cleanup(
            prog_link,
            perf_fd,
            skel,
            previous_sigio,
            &mut previous_sigtrap,
        );
        return;
    }
    if !ASSERT_OK((perf_fd < 0) as c_int, c"perf_event_open".as_ptr() as *const u8) {
        goto_cleanup(
            prog_link,
            perf_fd,
            skel,
            previous_sigio,
            &mut previous_sigtrap,
        );
        return;
    }

    /* Configure the perf event to signal on sample. */
    err = fcntl(perf_fd, F_SETFL, O_ASYNC);
    if !ASSERT_OK(err, c"fcntl(F_SETFL, O_ASYNC)".as_ptr() as *const u8) {
        goto_cleanup(
            prog_link,
            perf_fd,
            skel,
            previous_sigio,
            &mut previous_sigtrap,
        );
        return;
    }

    owner.type_ = F_OWNER_TID;
    owner.pid = syscall(__NR_GETTID) as c_int;
    err = fcntl(perf_fd, F_SETOWN_EX, &mut owner as *mut f_owner_ex);
    if !ASSERT_OK(err, c"fcntl(F_SETOWN_EX)".as_ptr() as *const u8) {
        goto_cleanup(
            prog_link,
            perf_fd,
            skel,
            previous_sigio,
            &mut previous_sigtrap,
        );
        return;
    }

    /* Allow at most one sample. A sample rejected by bpf should
     * not count against this.
     */
    err = ioctl(perf_fd, PERF_EVENT_IOC_REFRESH, 1);
    if !ASSERT_OK(err, c"ioctl(PERF_EVENT_IOC_REFRESH)".as_ptr() as *const u8) {
        goto_cleanup(
            prog_link,
            perf_fd,
            skel,
            previous_sigio,
            &mut previous_sigtrap,
        );
        return;
    }

    prog_link = bpf_program__attach_perf_event((*skel).progs.handler, perf_fd);
    if !ASSERT_OK_PTR(
        prog_link as *mut c_void,
        c"bpf_program__attach_perf_event".as_ptr() as *const u8,
    ) {
        goto_cleanup(
            prog_link,
            perf_fd,
            skel,
            previous_sigio,
            &mut previous_sigtrap,
        );
        return;
    }

    /* Configure the bpf program to suppress the sample. */
    (*(*skel).bss).ip = test_function as usize;
    test_function();

    ASSERT_EQ(sigio_count, 0, c"sigio_count".as_ptr() as *const u8);
    ASSERT_EQ(sigtrap_count, 0, c"sigtrap_count".as_ptr() as *const u8);

    /* Configure the bpf program to allow the sample. */
    (*(*skel).bss).ip = 0;
    test_function();

    ASSERT_EQ(sigio_count, 1, c"sigio_count".as_ptr() as *const u8);
    ASSERT_EQ(sigtrap_count, 1, c"sigtrap_count".as_ptr() as *const u8);

    /* Test that the sample above is the only one allowed (by perf, not
     * by bpf)
     */
    test_function();

    ASSERT_EQ(sigio_count, 1, c"sigio_count".as_ptr() as *const u8);
    ASSERT_EQ(sigtrap_count, 1, c"sigtrap_count".as_ptr() as *const u8);

    goto_cleanup(
        prog_link,
        perf_fd,
        skel,
        previous_sigio,
        &mut previous_sigtrap,
    );
}

unsafe fn goto_cleanup(
    prog_link: *mut bpf_link,
    perf_fd: c_int,
    skel: *mut test_perf_skip,
    previous_sigio: sighandler_t,
    previous_sigtrap: *mut sigaction,
) {
    bpf_link__destroy(prog_link);
    if perf_fd >= 0 {
        close(perf_fd);
    }
    test_perf_skip__destroy(skel);

    if previous_sigio != SIG_ERR {
        signal(SIGIO, previous_sigio);
    }
    sigaction(SIGTRAP, previous_sigtrap, core::ptr::null_mut());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
