// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020 Collabora Ltd.
 *
 * Test code for syscall user dispatch
 */

/*
 * C dependencies translated as external ABI declarations:
 * <sys/prctl.h>, <sys/sysinfo.h>, <sys/syscall.h>, <signal.h>,
 * <stdbool.h>, <stdlib.h>, <asm/unistd.h>, "kselftest_harness.h"
 */

use core::ffi::{c_int, c_long, c_ulong, c_void};

type size_t = usize;
type sighandler_t = unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void);

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigset_t {
    _private: [usize; 16],
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: Option<sighandler_t>,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    _pad: [c_int; 29],
    pub si_call_addr: *mut c_void,
    pub si_syscall: c_int,
    pub si_arch: c_uint,
}

type c_uint = u32;

#[repr(C)]
pub struct sysinfo {
    pub uptime: c_long,
    pub loads: [c_ulong; 3],
    pub totalram: c_ulong,
    pub freeram: c_ulong,
    pub sharedram: c_ulong,
    pub bufferram: c_ulong,
    pub totalswap: c_ulong,
    pub freeswap: c_ulong,
    pub procs: u16,
    pub pad: u16,
    pub totalhigh: c_ulong,
    pub freehigh: c_ulong,
    pub mem_unit: c_uint,
    _f: [u8; 0],
}

unsafe extern "C" {
    fn prctl(option: c_int, ...) -> c_int;
    fn sysinfo(info: *mut sysinfo) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
}

const PR_SET_SYSCALL_USER_DISPATCH: c_int = 59;
const PR_SYS_DISPATCH_OFF: c_ulong = 0;
const SYSCALL_DISPATCH_FILTER_ALLOW: i8 = 0;
const SYSCALL_DISPATCH_FILTER_BLOCK: i8 = 1;

const PR_SYS_DISPATCH_EXCLUSIVE_ON: c_ulong = 1;
const PR_SYS_DISPATCH_INCLUSIVE_ON: c_ulong = 2;

const SYS_USER_DISPATCH: c_int = 2;

/*
 * C used __NR_syscalls when present, otherwise 0xff00. This isolated Rust
 * translation keeps the fallback value because build-time asm constants are
 * external to this file.
 */
const MAGIC_SYSCALL_1: c_long = 0xff00;

const SIGSYS: c_int = 31;
const SA_SIGINFO: c_int = 4;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;

/*
 * __NR_sysinfo is supplied by <asm/unistd.h> in C. Keep a local Linux value
 * for the direct syscall expression translated below.
 */
const __NR_sysinfo: c_long = 99;

unsafe extern "C" {
    static mut errno: c_int;
}

macro_rules! SYSCALL_DISPATCH_ON {
    ($x:expr) => {
        $x = SYSCALL_DISPATCH_FILTER_BLOCK
    };
}

macro_rules! SYSCALL_DISPATCH_OFF {
    ($x:expr) => {
        $x = SYSCALL_DISPATCH_FILTER_ALLOW
    };
}

/*
 * Test Summary:
 *
 * - dispatch_trigger_sigsys: Verify if PR_SET_SYSCALL_USER_DISPATCH is
 *   able to trigger SIGSYS on a syscall.
 *
 * - bad_selector: Test that a bad selector value triggers SIGSYS with
 *   si_errno EINVAL.
 *
 * - bad_prctl_param: Test that the API correctly rejects invalid
 *   parameters on prctl
 *
 * - dispatch_and_return: Test that a syscall is selectively dispatched
 *   to userspace depending on the value of selector.
 *
 * - disable_dispatch: Test that the PR_SYS_DISPATCH_OFF correctly
 *   disables the dispatcher
 *
 * - direct_dispatch_range: Test that a syscall within the allowed range
 *   can bypass the dispatcher.
 */

unsafe fn dispatch_trigger_sigsys(_metadata: *mut __test_metadata) {
    let mut sel: i8 = SYSCALL_DISPATCH_FILTER_ALLOW;
    let mut info: sysinfo = core::mem::zeroed();
    let mut ret: c_int;

    ret = sysinfo(&mut info);
    ASSERT_EQ!(0, ret);

    ret = prctl(
        PR_SET_SYSCALL_USER_DISPATCH,
        PR_SYS_DISPATCH_EXCLUSIVE_ON,
        0,
        0,
        &mut sel,
    );
    ASSERT_EQ!(0, ret, {
        TH_LOG!("Kernel does not support CONFIG_SYSCALL_USER_DISPATCH");
    });

    SYSCALL_DISPATCH_ON!(sel);

    sysinfo(&mut info);

    EXPECT_FALSE!(true, {
        TH_LOG!("Unreachable!");
    });
}

unsafe fn prctl_valid(
    _metadata: *mut __test_metadata,
    op: c_ulong,
    off: c_ulong,
    size: c_ulong,
    sel: *mut c_void,
) {
    EXPECT_EQ!(
        0,
        prctl(PR_SET_SYSCALL_USER_DISPATCH, op, off, size, sel)
    );
}

unsafe fn prctl_invalid(
    _metadata: *mut __test_metadata,
    op: c_ulong,
    off: c_ulong,
    size: c_ulong,
    sel: *mut c_void,
    err: c_int,
) {
    EXPECT_EQ!(
        -1,
        prctl(PR_SET_SYSCALL_USER_DISPATCH, op, off, size, sel)
    );
    EXPECT_EQ!(err, errno);
}

unsafe fn bad_prctl_param(_metadata: *mut __test_metadata) {
    let mut sel: i8 = SYSCALL_DISPATCH_FILTER_ALLOW;
    let mut op: c_long;

    /* Invalid op */
    op = -1;
    prctl_invalid(_metadata, op as c_ulong, 0, 0, &mut sel as *mut _ as *mut c_void, EINVAL);

    /* PR_SYS_DISPATCH_OFF */
    op = PR_SYS_DISPATCH_OFF as c_long;

    /* offset != 0 */
    prctl_invalid(_metadata, op as c_ulong, 0x1, 0x0, core::ptr::null_mut(), EINVAL);

    /* len != 0 */
    prctl_invalid(_metadata, op as c_ulong, 0x0, 0xff, core::ptr::null_mut(), EINVAL);

    /* sel != NULL */
    prctl_invalid(_metadata, op as c_ulong, 0x0, 0x0, &mut sel as *mut _ as *mut c_void, EINVAL);

    /* Valid parameter */
    prctl_valid(_metadata, op as c_ulong, 0x0, 0x0, core::ptr::null_mut());

    /* PR_SYS_DISPATCH_EXCLUSIVE_ON */
    op = PR_SYS_DISPATCH_EXCLUSIVE_ON as c_long;

    /* Dispatcher region is bad (offset > 0 && len == 0) */
    prctl_invalid(_metadata, op as c_ulong, 0x1, 0x0, &mut sel as *mut _ as *mut c_void, EINVAL);
    prctl_invalid(
        _metadata,
        op as c_ulong,
        (-1_i64) as c_ulong,
        0x0,
        &mut sel as *mut _ as *mut c_void,
        EINVAL,
    );

    /* Invalid selector */
    prctl_invalid(_metadata, op as c_ulong, 0x0, 0x1, (-1_isize) as *mut c_void, EFAULT);

    /*
     * Dispatcher range overflows unsigned long
     */
    prctl_invalid(
        _metadata,
        PR_SYS_DISPATCH_EXCLUSIVE_ON,
        1,
        (-1_i64) as c_ulong,
        &mut sel as *mut _ as *mut c_void,
        EINVAL,
    );

    /*
     * Allowed range overflows usigned long
     */
    prctl_invalid(
        _metadata,
        PR_SYS_DISPATCH_EXCLUSIVE_ON,
        (-1_i64) as c_ulong,
        0x1,
        &mut sel as *mut _ as *mut c_void,
        EINVAL,
    );

    /* 0 len should fail for PR_SYS_DISPATCH_INCLUSIVE_ON */
    prctl_invalid(_metadata, PR_SYS_DISPATCH_INCLUSIVE_ON, 1, 0, core::ptr::null_mut(), EINVAL);

    /* Range wrap-around should fail */
    prctl_invalid(
        _metadata,
        PR_SYS_DISPATCH_INCLUSIVE_ON,
        (-1_i64) as c_ulong,
        2,
        core::ptr::null_mut(),
        EINVAL,
    );

    /* Normal range shouldn't fail */
    prctl_valid(_metadata, PR_SYS_DISPATCH_INCLUSIVE_ON, 2, 3, core::ptr::null_mut());

    /* Invalid selector */
    prctl_invalid(
        _metadata,
        PR_SYS_DISPATCH_INCLUSIVE_ON,
        2,
        3,
        (-1_isize) as *mut c_void,
        EFAULT,
    );
}

/*
 * Use global selector for handle_sigsys tests, to avoid passing
 * selector to signal handler
 */
static mut glob_sel: i8 = 0;
static mut nr_syscalls_emulated: c_int = 0;
static mut si_code: c_int = 0;
static mut si_errno: c_int = 0;
static mut syscall_addr: c_ulong = 0;

unsafe extern "C" fn handle_sigsys(_sig: c_int, info: *mut siginfo_t, _ucontext: *mut c_void) {
    si_code = (*info).si_code;
    si_errno = (*info).si_errno;
    syscall_addr = (*info).si_call_addr as c_ulong;

    if (*info).si_syscall as c_long == MAGIC_SYSCALL_1 {
        nr_syscalls_emulated += 1;
    }

    /* In preparation for sigreturn. */
    SYSCALL_DISPATCH_OFF!(glob_sel);

    /*
     * The tests for argument handling assume that `syscall(x) == x`. This
     * is a NOP on x86 because the syscall number is passed in %rax, which
     * happens to also be the function ABI return register.  Other
     * architectures may need to swizzle the arguments around.
     */
    /*
     * C conditional for riscv:
     * REG_A7 is not defined in libc headers.
     * ((ucontext_t *)ucontext)->uc_mcontext.__gregs[REG_A0] =
     *         ((ucontext_t *)ucontext)->uc_mcontext.__gregs[REG_A7];
     */
}

unsafe fn setup_sigsys_handler() -> c_int {
    let mut act: sigaction = core::mem::zeroed();
    let mut mask: sigset_t = core::mem::zeroed();

    memset(
        &mut act as *mut _ as *mut c_void,
        0,
        core::mem::size_of::<sigaction>(),
    );
    sigemptyset(&mut mask);
    act.sa_sigaction = Some(handle_sigsys);
    act.sa_flags = SA_SIGINFO;
    act.sa_mask = mask;
    sigaction(SIGSYS, &act, core::ptr::null_mut())
}

unsafe fn dispatch_and_return(_metadata: *mut __test_metadata) {
    let mut ret: c_long;

    glob_sel = 0;
    nr_syscalls_emulated = 0;
    si_code = 0;
    si_errno = 0;

    ASSERT_EQ!(0, setup_sigsys_handler());

    /* Make sure selector is good prior to prctl. */
    SYSCALL_DISPATCH_OFF!(glob_sel);

    ret = prctl(
        PR_SET_SYSCALL_USER_DISPATCH,
        PR_SYS_DISPATCH_EXCLUSIVE_ON,
        0,
        0,
        &mut glob_sel,
    ) as c_long;
    ASSERT_EQ!(0, ret, {
        TH_LOG!("Kernel does not support CONFIG_SYSCALL_USER_DISPATCH");
    });

    /* MAGIC_SYSCALL_1 doesn't exist. */
    SYSCALL_DISPATCH_OFF!(glob_sel);
    ret = syscall(MAGIC_SYSCALL_1);
    EXPECT_EQ!(-1, ret, {
        TH_LOG!("Dispatch triggered unexpectedly");
    });

    /* MAGIC_SYSCALL_1 should be emulated. */
    nr_syscalls_emulated = 0;
    SYSCALL_DISPATCH_ON!(glob_sel);

    ret = syscall(MAGIC_SYSCALL_1);
    EXPECT_EQ!(MAGIC_SYSCALL_1, ret, {
        TH_LOG!("Failed to intercept syscall");
    });
    EXPECT_EQ!(1, nr_syscalls_emulated, {
        TH_LOG!("Failed to emulate syscall");
    });
    ASSERT_EQ!(SYS_USER_DISPATCH, si_code, {
        TH_LOG!("Bad si_code in SIGSYS");
    });
    ASSERT_EQ!(0, si_errno, {
        TH_LOG!("Bad si_errno in SIGSYS");
    });
}

unsafe fn bad_selector(_metadata: *mut __test_metadata) {
    let mut ret: c_long;
    let mut act: sigaction = core::mem::zeroed();
    let mut mask: sigset_t = core::mem::zeroed();
    let mut info: sysinfo = core::mem::zeroed();

    glob_sel = SYSCALL_DISPATCH_FILTER_ALLOW;
    nr_syscalls_emulated = 0;
    si_code = 0;
    si_errno = 0;

    memset(
        &mut act as *mut _ as *mut c_void,
        0,
        core::mem::size_of::<sigaction>(),
    );
    sigemptyset(&mut mask);

    act.sa_sigaction = Some(handle_sigsys);
    act.sa_flags = SA_SIGINFO;
    act.sa_mask = mask;

    ret = sigaction(SIGSYS, &act, core::ptr::null_mut()) as c_long;
    ASSERT_EQ!(0, ret);

    /* Make sure selector is good prior to prctl. */
    SYSCALL_DISPATCH_OFF!(glob_sel);

    ret = prctl(
        PR_SET_SYSCALL_USER_DISPATCH,
        PR_SYS_DISPATCH_EXCLUSIVE_ON,
        0,
        0,
        &mut glob_sel,
    ) as c_long;
    ASSERT_EQ!(0, ret, {
        TH_LOG!("Kernel does not support CONFIG_SYSCALL_USER_DISPATCH");
    });

    glob_sel = -1;

    sysinfo(&mut info);

    /* Even though it is ready to catch SIGSYS, the signal is
     * supposed to be uncatchable.
     */

    EXPECT_FALSE!(true, {
        TH_LOG!("Unreachable!");
    });
}

unsafe fn disable_dispatch(_metadata: *mut __test_metadata) {
    let mut ret: c_int;
    let mut info: sysinfo = core::mem::zeroed();
    let mut sel: i8 = 0;

    ret = prctl(
        PR_SET_SYSCALL_USER_DISPATCH,
        PR_SYS_DISPATCH_EXCLUSIVE_ON,
        0,
        0,
        &mut sel,
    );
    ASSERT_EQ!(0, ret, {
        TH_LOG!("Kernel does not support CONFIG_SYSCALL_USER_DISPATCH");
    });

    /* MAGIC_SYSCALL_1 doesn't exist. */
    SYSCALL_DISPATCH_OFF!(glob_sel);

    ret = prctl(
        PR_SET_SYSCALL_USER_DISPATCH,
        PR_SYS_DISPATCH_OFF,
        0,
        0,
        0,
    );
    EXPECT_EQ!(0, ret, {
        TH_LOG!("Failed to unset syscall user dispatch");
    });

    /* Shouldn't have any effect... */
    SYSCALL_DISPATCH_ON!(glob_sel);

    ret = syscall(__NR_sysinfo, &mut info) as c_int;
    EXPECT_EQ!(0, ret, {
        TH_LOG!("Dispatch triggered unexpectedly");
    });
}

unsafe fn direct_dispatch_range(_metadata: *mut __test_metadata) {
    let mut ret: c_int = 0;
    let mut info: sysinfo = core::mem::zeroed();
    let mut sel: i8 = SYSCALL_DISPATCH_FILTER_ALLOW;

    /*
     * Instead of calculating libc addresses; allow the entire
     * memory map and lock the selector.
     */
    ret = prctl(
        PR_SET_SYSCALL_USER_DISPATCH,
        PR_SYS_DISPATCH_EXCLUSIVE_ON,
        0,
        (-1_i64) as c_ulong,
        &mut sel,
    );
    ASSERT_EQ!(0, ret, {
        TH_LOG!("Kernel does not support CONFIG_SYSCALL_USER_DISPATCH");
    });

    SYSCALL_DISPATCH_ON!(sel);

    ret = sysinfo(&mut info);
    ASSERT_EQ!(0, ret, {
        TH_LOG!("Dispatch triggered unexpectedly");
    });
}

unsafe fn test_range(
    _metadata: *mut __test_metadata,
    op: c_ulong,
    off: c_ulong,
    size: c_ulong,
    dispatch: bool,
) {
    nr_syscalls_emulated = 0;
    SYSCALL_DISPATCH_OFF!(glob_sel);
    EXPECT_EQ!(
        0,
        prctl(PR_SET_SYSCALL_USER_DISPATCH, op, off, size, &mut glob_sel)
    );
    SYSCALL_DISPATCH_ON!(glob_sel);
    if dispatch {
        EXPECT_EQ!(syscall(MAGIC_SYSCALL_1), MAGIC_SYSCALL_1);
        EXPECT_EQ!(nr_syscalls_emulated, 1);
    } else {
        EXPECT_EQ!(syscall(MAGIC_SYSCALL_1), -1);
        EXPECT_EQ!(nr_syscalls_emulated, 0);
    }
}

unsafe fn dispatch_range(_metadata: *mut __test_metadata) {
    ASSERT_EQ!(0, setup_sigsys_handler());
    test_range(_metadata, PR_SYS_DISPATCH_EXCLUSIVE_ON, 0, 0, true);
    test_range(
        _metadata,
        PR_SYS_DISPATCH_EXCLUSIVE_ON,
        syscall_addr,
        1,
        false,
    );
    test_range(
        _metadata,
        PR_SYS_DISPATCH_EXCLUSIVE_ON,
        syscall_addr.wrapping_sub(100),
        200,
        false,
    );
    test_range(
        _metadata,
        PR_SYS_DISPATCH_EXCLUSIVE_ON,
        syscall_addr.wrapping_add(1),
        100,
        true,
    );
    test_range(
        _metadata,
        PR_SYS_DISPATCH_EXCLUSIVE_ON,
        syscall_addr.wrapping_sub(100),
        100,
        true,
    );
    test_range(_metadata, PR_SYS_DISPATCH_INCLUSIVE_ON, syscall_addr, 1, true);
    test_range(
        _metadata,
        PR_SYS_DISPATCH_INCLUSIVE_ON,
        syscall_addr.wrapping_sub(1),
        1,
        false,
    );
    test_range(
        _metadata,
        PR_SYS_DISPATCH_INCLUSIVE_ON,
        syscall_addr.wrapping_add(1),
        1,
        false,
    );
    SYSCALL_DISPATCH_OFF!(glob_sel);
}

/* TEST_HARNESS_MAIN */
