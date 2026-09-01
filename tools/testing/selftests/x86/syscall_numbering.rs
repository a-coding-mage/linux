/* SPDX-License-Identifier: GPL-2.0 */
/*
 * syscall_numbering.c - test calling the x86-64 kernel with various
 * valid and invalid system call numbers.
 *
 * Copyright (c) 2018 Andrew Lutomirski
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

/* Common system call numbers */
const SYS_READ: c_int = 0;
const SYS_WRITE: c_int = 1;
const SYS_GETPID: c_int = 39;
/* x64-only system call numbers */
const X64_IOCTL: c_int = 16;
const X64_READV: c_int = 19;
const X64_WRITEV: c_int = 20;
/* x32-only system call numbers (without X32_BIT) */
const X32_IOCTL: c_int = 514;
const X32_READV: c_int = 515;
const X32_WRITEV: c_int = 516;

const X32_BIT: c_int = 0x40000000;
const MODIFIED_BY_PTRACE: i64 = -9999;

const ENOSYS: c_int = 38;
const EINTR: c_int = 4;
const EX_OSERR: c_int = 71;
const O_RDWR: c_int = 2;
const SIGSTOP: c_int = 19;
const SIGTRAP: c_int = 5;
const PTRACE_TRACEME: c_int = 0;
const PTRACE_SYSCALL: c_int = 24;
const PTRACE_GETREGS: c_int = 12;
const PTRACE_SETREGS: c_int = 13;
const PTRACE_DETACH: c_int = 17;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const _IONBF: c_int = 2;
const _SC_PAGE_SIZE: c_int = 30;
const INT_MAX: c_int = 2147483647;
const INT_MIN: c_int = -2147483648;

static mut NULLFD: c_int = -1; /* File descriptor for /dev/null */
static mut WITH_X32: bool = false; /* x32 supported on this kernel? */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum ptrace_pass {
    PTP_NOTHING,
    PTP_GETREGS,
    PTP_WRITEBACK,
    PTP_FUZZRET,
    PTP_FUZZHIGH,
    PTP_INTNUM,
    PTP_DONE,
}

static PTRACE_PASS_NAME: [*const c_char; ptrace_pass::PTP_DONE as usize] = [
    b"just stop, no data read\0".as_ptr() as *const c_char,
    b"only getregs\0".as_ptr() as *const c_char,
    b"getregs, unmodified setregs\0".as_ptr() as *const c_char,
    b"modifying the default return\0".as_ptr() as *const c_char,
    b"clobbering the top 32 bits\0".as_ptr() as *const c_char,
    b"sign-extending the syscall number\0".as_ptr() as *const c_char,
];

/*
 * Shared memory block between tracer and test
 */
#[repr(C)]
struct shared {
    nerr: c_uint, /* Total error count */
    indent: c_uint, /* Message indentation level */
    ptrace_pass: ptrace_pass,
    probing_syscall: bool, /* In probe_syscall() */
}

static mut SH: *mut shared = ptr::null_mut();

#[repr(C)]
struct user_regs_struct {
    r15: c_ulong,
    r14: c_ulong,
    r13: c_ulong,
    r12: c_ulong,
    rbp: c_ulong,
    rbx: c_ulong,
    r11: c_ulong,
    r10: c_ulong,
    r9: c_ulong,
    r8: c_ulong,
    rax: c_ulong,
    rcx: c_ulong,
    rdx: c_ulong,
    rsi: c_ulong,
    rdi: c_ulong,
    orig_rax: c_ulong,
    rip: c_ulong,
    cs: c_ulong,
    eflags: c_ulong,
    rsp: c_ulong,
    ss: c_ulong,
    fs_base: c_ulong,
    gs_base: c_ulong,
    ds: c_ulong,
    es: c_ulong,
    fs: c_ulong,
    gs: c_ulong,
}

unsafe extern "C" {
    static mut stdout: *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn _exit(status: c_int) -> !;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn getpid() -> c_int;
    fn raise(sig: c_int) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, wstatus: *mut c_int, options: c_int) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn sysconf(name: c_int) -> c_long;
    fn setvbuf(stream: *mut c_void, buf: *mut c_char, mode: c_int, size: usize) -> c_int;
    fn ptrace(request: c_int, pid: c_int, addr: *mut c_void, data: *mut c_void) -> c_long;
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

unsafe fn WSTOPSIG(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn offset() -> c_uint {
    let level = if !SH.is_null() { (*SH).indent } else { 0 };

    8 + level * 4
}

unsafe fn msg(lvl: *const c_char, fmt: *const c_char) {
    printf(b"%-*s%s\0".as_ptr() as *const c_char, offset(), lvl, fmt);
}

unsafe fn msg1_ll(lvl: *const c_char, fmt: *const c_char, a1: i64) {
    printf(b"%-*s\0".as_ptr() as *const c_char, offset(), lvl);
    printf(fmt, a1);
}

unsafe fn msg1_s(lvl: *const c_char, fmt: *const c_char, a1: *const c_char) {
    printf(b"%-*s\0".as_ptr() as *const c_char, offset(), lvl);
    printf(fmt, a1);
}

unsafe fn msg2_s_s(lvl: *const c_char, fmt: *const c_char, a1: *const c_char, a2: *const c_char) {
    printf(b"%-*s\0".as_ptr() as *const c_char, offset(), lvl);
    printf(fmt, a1, a2);
}

unsafe fn msg2_i(lvl: *const c_char, fmt: *const c_char, a1: c_int, a2: c_int) {
    printf(b"%-*s\0".as_ptr() as *const c_char, offset(), lvl);
    printf(fmt, a1, a2);
}

unsafe fn msg2_ll(lvl: *const c_char, fmt: *const c_char, a1: i64, a2: i64) {
    printf(b"%-*s\0".as_ptr() as *const c_char, offset(), lvl);
    printf(fmt, a1, a2);
}

unsafe fn msg3_s_l_s(lvl: *const c_char, fmt: *const c_char, a1: *const c_char, a2: i64, a3: *const c_char) {
    printf(b"%-*s\0".as_ptr() as *const c_char, offset(), lvl);
    printf(fmt, a1, a2, a3);
}

unsafe fn msg3_s_u_s(lvl: *const c_char, fmt: *const c_char, a1: *const c_char, a2: c_uint, a3: *const c_char) {
    printf(b"%-*s\0".as_ptr() as *const c_char, offset(), lvl);
    printf(fmt, a1, a2, a3);
}

unsafe fn msg2_ull(lvl: *const c_char, fmt: *const c_char, a1: c_ulong, a2: c_ulong) {
    printf(b"%-*s\0".as_ptr() as *const c_char, offset(), lvl);
    printf(fmt, a1, a2);
}

unsafe fn fail_count() {
    (*SH).nerr += 1;
}

unsafe fn crit(fmt: *const c_char) -> ! {
    (*SH).indent = 0;
    msg(b"FAIL\0".as_ptr() as *const c_char, fmt);
    msg(
        b"SKIP\0".as_ptr() as *const c_char,
        b"Unable to run test\n\0".as_ptr() as *const c_char,
    );
    exit(EX_OSERR);
}

unsafe fn crit1_s(fmt: *const c_char, a1: *const c_char) -> ! {
    (*SH).indent = 0;
    msg1_s(b"FAIL\0".as_ptr() as *const c_char, fmt, a1);
    msg(
        b"SKIP\0".as_ptr() as *const c_char,
        b"Unable to run test\n\0".as_ptr() as *const c_char,
    );
    exit(EX_OSERR);
}

/*
 * Directly invokes the given syscall with nullfd as the first argument
 * and the rest zero. Avoids involving glibc wrappers in case they ever
 * end up intercepting some system calls for some reason, or modify
 * the system call number itself.
 */
unsafe fn probe_syscall(msb: c_int, lsb: c_int) -> i64 {
    let arg1: i64 = NULLFD as i64;
    let arg2: i64 = 0;
    let arg3: i64 = 0;
    let arg4: i64 = 0;
    let arg5: i64 = 0;
    let arg6: i64 = 0;
    let nr: i64 = ((msb as i64) << 32) | (lsb as c_uint as i64);
    let ret: i64;

    /*
     * We pass in an extra copy of the extended system call number
     * in %rbx, so we can examine it from the ptrace handler without
     * worrying about it being possibly modified. This is to test
     * the validity of struct user regs.orig_rax a.k.a.
     * struct pt_regs.orig_ax.
     */
    (*SH).probing_syscall = true;
    asm!(
        "push rbx",
        "mov rbx, {nr_copy}",
        "syscall",
        "pop rbx",
        nr_copy = in(reg) nr,
        inlateout("rax") nr => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        in("r9") arg6,
        lateout("rcx") _,
        lateout("r11") _,
        options(preserves_flags),
    );
    (*SH).probing_syscall = false;

    ret
}

unsafe fn syscall_str(msb: c_int, start: c_int, end: c_int) -> *const c_char {
    static mut BUF: [c_char; 64] = [0; 64];
    let type_ = if (start & X32_BIT) != 0 {
        b"x32\0".as_ptr() as *const c_char
    } else {
        b"x64\0".as_ptr() as *const c_char
    };
    let mut lsb = start;

    /*
     * Improve readability by stripping the x32 bit, but round
     * toward zero so we don't display -1 as -1073741825.
     */
    if lsb < 0 {
        lsb |= X32_BIT;
    } else {
        lsb &= !X32_BIT;
    }

    if start == end {
        snprintf(
            BUF.as_mut_ptr(),
            BUF.len(),
            b"%s syscall %d:%d\0".as_ptr() as *const c_char,
            type_,
            msb,
            lsb,
        );
    } else {
        snprintf(
            BUF.as_mut_ptr(),
            BUF.len(),
            b"%s syscalls %d:%d..%d\0".as_ptr() as *const c_char,
            type_,
            msb,
            lsb,
            lsb + (end - start),
        );
    }

    BUF.as_ptr()
}

unsafe fn _check_for(
    msb: c_int,
    start: c_int,
    end: c_int,
    expect: i64,
    expect_str: *const c_char,
) -> c_uint {
    let mut err: c_uint = 0;

    (*SH).indent += 1;
    if start != end {
        (*SH).indent += 1;
    }

    let mut nr = start;
    while nr <= end {
        let ret = probe_syscall(msb, nr);

        if ret != expect {
            msg3_s_l_s(
                b"FAIL\0".as_ptr() as *const c_char,
                b"%s returned %lld, but it should have returned %s\n\0".as_ptr() as *const c_char,
                syscall_str(msb, nr, nr),
                ret,
                expect_str,
            );
            fail_count();
            err += 1;
        }
        nr += 1;
    }

    if start != end {
        (*SH).indent -= 1;
    }

    if err != 0 {
        if start != end {
            msg3_s_u_s(
                b"FAIL\0".as_ptr() as *const c_char,
                b"%s had %u failure%s\n\0".as_ptr() as *const c_char,
                syscall_str(msb, start, end),
                err,
                if err == 1 {
                    b"s\0".as_ptr() as *const c_char
                } else {
                    b"\0".as_ptr() as *const c_char
                },
            );
            fail_count();
        }
    } else {
        msg2_s_s(
            b"OK\0".as_ptr() as *const c_char,
            b"%s returned %s as expected\n\0".as_ptr() as *const c_char,
            syscall_str(msb, start, end),
            expect_str,
        );
    }

    (*SH).indent -= 1;

    err
}

unsafe fn check_for(msb: c_int, start: c_int, end: c_int, expect: i64, expect_str: *const c_char) -> c_uint {
    _check_for(msb, start, end, expect, expect_str)
}

unsafe fn check_zero(msb: c_int, nr: c_int) -> bool {
    check_for(msb, nr, nr, 0, b"0\0".as_ptr() as *const c_char) != 0
}

unsafe fn check_enosys(msb: c_int, nr: c_int) -> bool {
    check_for(msb, nr, nr, -(ENOSYS as i64), b"-ENOSYS\0".as_ptr() as *const c_char) != 0
}

/*
 * Anyone diagnosing a failure will want to know whether the kernel
 * supports x32. Tell them. This can also be used to conditionalize
 * tests based on existence or nonexistence of x32.
 */
unsafe fn test_x32() -> bool {
    let ret: i64;
    let mypid = getpid();

    msg(
        b"RUN\0".as_ptr() as *const c_char,
        b"Checking for x32 by calling x32 getpid()\n\0".as_ptr() as *const c_char,
    );
    ret = probe_syscall(0, SYS_GETPID | X32_BIT);

    (*SH).indent += 1;
    if ret == mypid as i64 {
        msg(
            b"INFO\0".as_ptr() as *const c_char,
            b"x32 is supported\n\0".as_ptr() as *const c_char,
        );
        WITH_X32 = true;
    } else if ret == -(ENOSYS as i64) {
        msg(
            b"INFO\0".as_ptr() as *const c_char,
            b"x32 is not supported\n\0".as_ptr() as *const c_char,
        );
        WITH_X32 = false;
    } else {
        msg2_ll(
            b"FAIL\0".as_ptr() as *const c_char,
            b"x32 getpid() returned %lld, but it should have returned either %lld or -ENOSYS\n\0".as_ptr() as *const c_char,
            ret,
            mypid as i64,
        );
        fail_count();
        WITH_X32 = false;
    }
    (*SH).indent -= 1;
    WITH_X32
}

unsafe fn test_syscalls_common(msb: c_int) {
    let pass = (*SH).ptrace_pass;

    msg(
        b"RUN\0".as_ptr() as *const c_char,
        b"Checking some common syscalls as 64 bit\n\0".as_ptr() as *const c_char,
    );
    check_zero(msb, SYS_READ);
    check_zero(msb, SYS_WRITE);

    msg(
        b"RUN\0".as_ptr() as *const c_char,
        b"Checking some 64-bit only syscalls as 64 bit\n\0".as_ptr() as *const c_char,
    );
    check_zero(msb, X64_READV);
    check_zero(msb, X64_WRITEV);

    msg(
        b"RUN\0".as_ptr() as *const c_char,
        b"Checking out of range system calls\n\0".as_ptr() as *const c_char,
    );
    check_for(msb, -64, -2, -(ENOSYS as i64), b"-ENOSYS\0".as_ptr() as *const c_char);
    if pass as c_int >= ptrace_pass::PTP_FUZZRET as c_int {
        check_for(msb, -1, -1, MODIFIED_BY_PTRACE, b"MODIFIED_BY_PTRACE\0".as_ptr() as *const c_char);
    } else {
        check_for(msb, -1, -1, -(ENOSYS as i64), b"-ENOSYS\0".as_ptr() as *const c_char);
    }
    check_for(msb, X32_BIT - 64, X32_BIT - 1, -(ENOSYS as i64), b"-ENOSYS\0".as_ptr() as *const c_char);
    check_for(msb, -64 - X32_BIT, -1 - X32_BIT, -(ENOSYS as i64), b"-ENOSYS\0".as_ptr() as *const c_char);
    check_for(msb, INT_MAX - 64, INT_MAX - 1, -(ENOSYS as i64), b"-ENOSYS\0".as_ptr() as *const c_char);
}

unsafe fn test_syscalls_with_x32(msb: c_int) {
    /*
     * Syscalls 512-547 are "x32" syscalls.  They are
     * intended to be called with the x32 (0x40000000) bit
     * set.  Calling them without the x32 bit set is
     * nonsense and should not work.
     */
    msg(
        b"RUN\0".as_ptr() as *const c_char,
        b"Checking x32 syscalls as 64 bit\n\0".as_ptr() as *const c_char,
    );
    check_for(msb, 512, 547, -(ENOSYS as i64), b"-ENOSYS\0".as_ptr() as *const c_char);

    msg(
        b"RUN\0".as_ptr() as *const c_char,
        b"Checking some common syscalls as x32\n\0".as_ptr() as *const c_char,
    );
    check_zero(msb, SYS_READ | X32_BIT);
    check_zero(msb, SYS_WRITE | X32_BIT);

    msg(
        b"RUN\0".as_ptr() as *const c_char,
        b"Checking some x32 syscalls as x32\n\0".as_ptr() as *const c_char,
    );
    check_zero(msb, X32_READV | X32_BIT);
    check_zero(msb, X32_WRITEV | X32_BIT);

    msg(
        b"RUN\0".as_ptr() as *const c_char,
        b"Checking some 64-bit syscalls as x32\n\0".as_ptr() as *const c_char,
    );
    check_enosys(msb, X64_IOCTL | X32_BIT);
    check_enosys(msb, X64_READV | X32_BIT);
    check_enosys(msb, X64_WRITEV | X32_BIT);
}

unsafe fn test_syscalls_without_x32(msb: c_int) {
    msg(
        b"RUN\0".as_ptr() as *const c_char,
        b"Checking for absence of x32 system calls\n\0".as_ptr() as *const c_char,
    );
    check_for(msb, 0 | X32_BIT, 999 | X32_BIT, -(ENOSYS as i64), b"-ENOSYS\0".as_ptr() as *const c_char);
}

unsafe fn test_syscall_numbering() {
    static MSBS: [c_int; 10] = [
        0,
        1,
        -1,
        X32_BIT - 1,
        X32_BIT,
        X32_BIT - 1,
        -X32_BIT,
        INT_MAX,
        INT_MIN,
        INT_MIN + 1,
    ];

    (*SH).indent += 1;

    /*
     * The MSB is supposed to be ignored, so we loop over a few
     * to test that out.
     */
    let mut i: usize = 0;
    while i < MSBS.len() {
        let msb = MSBS[i];
        msg2_i(
            b"RUN\0".as_ptr() as *const c_char,
            b"Checking system calls with msb = %d (0x%x)\n\0".as_ptr() as *const c_char,
            msb,
            msb,
        );

        (*SH).indent += 1;

        test_syscalls_common(msb);
        if WITH_X32 {
            test_syscalls_with_x32(msb);
        } else {
            test_syscalls_without_x32(msb);
        }

        (*SH).indent -= 1;
        i += 1;
    }

    (*SH).indent -= 1;
}

unsafe fn syscall_numbering_tracee() {
    let mut pass: ptrace_pass;

    if ptrace(PTRACE_TRACEME, 0, ptr::null_mut(), ptr::null_mut()) != 0 {
        crit(b"Failed to request tracing\n\0".as_ptr() as *const c_char);
    }
    raise(SIGSTOP);

    (*SH).ptrace_pass = ptrace_pass::PTP_NOTHING;
    pass = ptrace_pass::PTP_NOTHING;
    while (pass as c_int) < (ptrace_pass::PTP_DONE as c_int) {
        msg1_s(
            b"RUN\0".as_ptr() as *const c_char,
            b"Running tests under ptrace: %s\n\0".as_ptr() as *const c_char,
            PTRACE_PASS_NAME[pass as usize],
        );
        test_syscall_numbering();
        pass = match (pass as c_int) + 1 {
            1 => ptrace_pass::PTP_GETREGS,
            2 => ptrace_pass::PTP_WRITEBACK,
            3 => ptrace_pass::PTP_FUZZRET,
            4 => ptrace_pass::PTP_FUZZHIGH,
            5 => ptrace_pass::PTP_INTNUM,
            _ => ptrace_pass::PTP_DONE,
        };
        (*SH).ptrace_pass = pass;
    }
}

unsafe fn mess_with_syscall(testpid: c_int, pass: ptrace_pass) {
    let mut regs: user_regs_struct = core::mem::zeroed();

    (*SH).probing_syscall = false; /* Do this on entry only */

    /* For these, don't even getregs */
    if pass == ptrace_pass::PTP_NOTHING || pass == ptrace_pass::PTP_DONE {
        return;
    }

    ptrace(
        PTRACE_GETREGS,
        testpid,
        ptr::null_mut(),
        &mut regs as *mut _ as *mut c_void,
    );

    if regs.orig_rax != regs.rbx {
        msg2_ull(
            b"FAIL\0".as_ptr() as *const c_char,
            b"orig_rax %#llx doesn't match syscall number %#llx\n\0".as_ptr() as *const c_char,
            regs.orig_rax,
            regs.rbx,
        );
        fail_count();
    }

    match pass {
        ptrace_pass::PTP_GETREGS => {
            /* Just read, no writeback */
            return;
        }
        ptrace_pass::PTP_WRITEBACK => {
            /* Write back the same register state verbatim */
        }
        ptrace_pass::PTP_FUZZRET => {
            regs.rax = MODIFIED_BY_PTRACE as c_ulong;
        }
        ptrace_pass::PTP_FUZZHIGH => {
            regs.rax = MODIFIED_BY_PTRACE as c_ulong;
            regs.orig_rax = regs.orig_rax | 0xffffffff00000000u64 as c_ulong;
        }
        ptrace_pass::PTP_INTNUM => {
            regs.rax = MODIFIED_BY_PTRACE as c_ulong;
            regs.orig_rax = regs.orig_rax as c_int as c_ulong;
        }
        _ => {
            crit(b"invalid ptrace_pass\n\0".as_ptr() as *const c_char);
        }
    }

    ptrace(
        PTRACE_SETREGS,
        testpid,
        ptr::null_mut(),
        &mut regs as *mut _ as *mut c_void,
    );
}

unsafe fn syscall_numbering_tracer(testpid: c_int) {
    let mut wstatus: c_int = 0;

    loop {
        let wpid = waitpid(testpid, &mut wstatus, 0);
        if wpid < 0 && errno() != EINTR {
            break;
        }
        if wpid != testpid {
            continue;
        }
        if !WIFSTOPPED(wstatus) {
            break; /* Thread exited? */
        }

        if (*SH).probing_syscall && WSTOPSIG(wstatus) == SIGTRAP {
            mess_with_syscall(testpid, (*SH).ptrace_pass);
        }

        if (*SH).ptrace_pass == ptrace_pass::PTP_DONE
            || ptrace(PTRACE_SYSCALL, testpid, ptr::null_mut(), ptr::null_mut()) != 0
        {
            break;
        }
    }

    ptrace(PTRACE_DETACH, testpid, ptr::null_mut(), ptr::null_mut());

    /* Wait for the child process to terminate */
    while waitpid(testpid, &mut wstatus, 0) != testpid || !WIFEXITED(wstatus) {
        /* wait some more */
    }
}

unsafe fn test_traced_syscall_numbering() {
    let testpid: c_int;

    /* Launch the test thread; this thread continues as the tracer thread */
    testpid = fork();

    if testpid < 0 {
        crit(b"Unable to launch tracer process\n\0".as_ptr() as *const c_char);
    } else if testpid == 0 {
        syscall_numbering_tracee();
        _exit(0);
    } else {
        syscall_numbering_tracer(testpid);
    }
}

fn main() {
    unsafe {
        let nerr: c_uint;

        /*
         * It is quite likely to get a segfault on a failure, so make
         * sure the message gets out by setting stdout to nonbuffered.
         */
        setvbuf(stdout, ptr::null_mut(), _IONBF, 0);

        /*
         * Harmless file descriptor to work on...
         */
        NULLFD = open(b"/dev/null\0".as_ptr() as *const c_char, O_RDWR);
        if NULLFD < 0 {
            crit1_s(
                b"Unable to open /dev/null: %s\n\0".as_ptr() as *const c_char,
                strerror(errno()),
            );
        }

        /*
         * Set up a block of shared memory...
         */
        SH = mmap(
            ptr::null_mut(),
            sysconf(_SC_PAGE_SIZE) as usize,
            PROT_READ | PROT_WRITE,
            MAP_ANONYMOUS | MAP_SHARED,
            0,
            0,
        ) as *mut shared;
        if SH as *mut c_void == MAP_FAILED {
            crit1_s(
                b"Unable to allocated shared memory block: %s\n\0".as_ptr() as *const c_char,
                strerror(errno()),
            );
        }

        WITH_X32 = test_x32();

        msg(
            b"RUN\0".as_ptr() as *const c_char,
            b"Running tests without ptrace...\n\0".as_ptr() as *const c_char,
        );
        test_syscall_numbering();

        test_traced_syscall_numbering();

        nerr = (*SH).nerr;
        if nerr == 0 {
            msg(
                b"OK\0".as_ptr() as *const c_char,
                b"All system calls succeeded or failed as expected\n\0".as_ptr() as *const c_char,
            );
            std::process::exit(0);
        } else {
            printf(b"%-*s\0".as_ptr() as *const c_char, offset(), b"FAIL\0".as_ptr() as *const c_char);
            printf(
                b"A total of %u system call%s had incorrect behavior\n\0".as_ptr() as *const c_char,
                nerr,
                if nerr != 1 {
                    b"s\0".as_ptr() as *const c_char
                } else {
                    b"\0".as_ptr() as *const c_char
                },
            );
            fail_count();
            std::process::exit(1);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
