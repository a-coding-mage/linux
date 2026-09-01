// SPDX-License-Identifier: GPL-2.0-only
/*
 * fsgsbase.c, an fsgsbase test
 * Copyright (c) 2014-2016 Andy Lutomirski
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr::{null, null_mut};

// C includes translated as external dependencies:
// stdio.h, stdlib.h, stdbool.h, string.h, sys/syscall.h, unistd.h, err.h,
// sys/user.h, asm/prctl.h, sys/prctl.h, signal.h, limits.h, sys/ucontext.h,
// sched.h, linux/futex.h, pthread.h, asm/ldt.h, sys/mman.h, stddef.h,
// sys/ptrace.h, sys/wait.h, setjmp.h, and "helpers.h".
// Original build condition: this test is 64-bit x86 only.

type sig_atomic_t = c_int;
type pid_t = c_int;
type pthread_t = c_ulong;
type size_t = usize;
type sigjmp_buf = [c_long; 32];

const SIGSEGV: c_int = 11;
const SIGILL: c_int = 4;
const SIGTRAP: c_int = 5;
const REG_RIP: usize = 16;

const SYS_arch_prctl: c_long = 158;
const SYS_modify_ldt: c_long = 154;
const SYS_futex: c_long = 202;
const ARCH_SET_GS: c_int = 0x1001;
const ARCH_GET_GS: c_int = 0x1004;
const FUTEX_WAIT: c_int = 0;
const FUTEX_WAKE: c_int = 1;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_32BIT: c_int = 0x40;

const PTRACE_TRACEME: c_int = 0;
const PTRACE_PEEKUSER: c_int = 3;
const PTRACE_POKEUSER: c_int = 6;
const PTRACE_CONT: c_int = 7;

const ULONG_MAX: c_ulong = c_ulong::MAX;

#[repr(C)]
struct siginfo_t {
    _data: [usize; 16],
}

impl siginfo_t {
    unsafe fn si_addr(&self) -> *mut c_void {
        *(self as *const siginfo_t as *const *mut c_void).add(2)
    }
}

#[repr(C)]
struct mcontext_t {
    gregs: [c_long; 23],
}

#[repr(C)]
struct ucontext_t {
    uc_flags: c_ulong,
    uc_link: *mut ucontext_t,
    uc_stack: [usize; 3],
    uc_mcontext: mcontext_t,
}

#[repr(C)]
struct user_desc {
    entry_number: c_uint,
    base_addr: c_uint,
    limit: c_uint,
    flags: c_uint,
}

impl user_desc {
    fn new(
        entry_number: c_uint,
        base_addr: c_uint,
        limit: c_uint,
        seg_32bit: c_uint,
        contents: c_uint,
        read_exec_only: c_uint,
        limit_in_pages: c_uint,
        seg_not_present: c_uint,
        useable: c_uint,
    ) -> user_desc {
        user_desc {
            entry_number,
            base_addr,
            limit,
            flags: (seg_32bit & 1)
                | ((contents & 3) << 1)
                | ((read_exec_only & 1) << 3)
                | ((limit_in_pages & 1) << 4)
                | ((seg_not_present & 1) << 5)
                | ((useable & 1) << 6),
        }
    }
}

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

#[repr(C)]
struct cpu_set_t {
    bits: [c_ulong; 16],
}

unsafe extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn abort() -> !;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn errx(eval: c_int, fmt: *const c_char, ...) -> !;
    fn syscall(num: c_long, ...) -> c_long;
    fn usleep(usec: c_uint) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn fork() -> pid_t;
    fn ptrace(request: c_int, ...) -> c_long;
    fn raise(sig: c_int) -> c_int;
    fn _exit(status: c_int) -> !;
    fn wait(status: *mut c_int) -> pid_t;
    fn sched_setaffinity(pid: pid_t, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn sigsetjmp(env: *mut sigjmp_buf, savesigs: c_int) -> c_int;
    fn siglongjmp(env: *mut sigjmp_buf, val: c_int) -> !;

    // From helpers.h.
    fn sethandler(
        sig: c_int,
        handler: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
        flags: c_int,
    );
    fn clearhandler(sig: c_int);
}

static mut want_segv: sig_atomic_t = 0;
static mut segv_addr: c_ulong = 0;

static mut shared_scratch: *mut c_ushort = null_mut();

static mut nerrs: c_int = 0;

type c_ushort = u16;

extern "C" fn sigsegv(_sig: c_int, si: *mut siginfo_t, ctx_void: *mut c_void) {
    unsafe {
        let ctx = ctx_void as *mut ucontext_t;

        if want_segv == 0 {
            clearhandler(SIGSEGV);
            return; /* Crash cleanly. */
        }

        want_segv = 0;
        segv_addr = (*si).si_addr() as c_ulong;

        (*ctx).uc_mcontext.gregs[REG_RIP] += 4; /* Skip the faulting mov */
    }
}

static mut jmpbuf: sigjmp_buf = [0; 32];

extern "C" fn sigill(_sig: c_int, _si: *mut siginfo_t, _ctx_void: *mut c_void) {
    unsafe {
        siglongjmp(&raw mut jmpbuf, 1);
    }
}

static mut have_fsgsbase: bool = false;

#[inline]
unsafe fn rdgsbase() -> c_ulong {
    let gsbase: c_ulong;
    asm!("rdgsbase {}", out(reg) gsbase, options(nostack, preserves_flags));
    gsbase
}

#[inline]
unsafe fn rdfsbase() -> c_ulong {
    let fsbase: c_ulong;
    asm!("rdfsbase {}", out(reg) fsbase, options(nostack, preserves_flags));
    fsbase
}

#[inline]
unsafe fn wrgsbase(gsbase: c_ulong) {
    asm!("wrgsbase {}", in(reg) gsbase, options(nostack, preserves_flags));
}

#[derive(Copy, Clone, PartialEq)]
enum which_base {
    FS,
    GS,
}

unsafe fn read_base(which: which_base) -> c_ulong {
    let mut offset: c_ulong;
    /*
     * Unless we have FSGSBASE, there's no direct way to do this from
     * user mode.  We can get at it indirectly using signals, though.
     */

    want_segv = 1;

    offset = 0;
    if which == which_base::FS {
        /* Use a constant-length instruction here. */
        asm!("mov rax, fs:[rcx]", in("rcx") offset, out("rax") _);
    } else {
        asm!("mov rax, gs:[rcx]", in("rcx") offset, out("rax") _);
    }
    if want_segv == 0 {
        return segv_addr.wrapping_add(offset);
    }

    /*
     * If that didn't segfault, try the other end of the address space.
     * Unless we get really unlucky and run into the vsyscall page, this
     * is guaranteed to segfault.
     */

    offset = (ULONG_MAX >> 1) + 1;
    if which == which_base::FS {
        asm!("mov rax, fs:[rcx]", in("rcx") offset, out("rax") _);
    } else {
        asm!("mov rax, gs:[rcx]", in("rcx") offset, out("rax") _);
    }
    if want_segv == 0 {
        return segv_addr.wrapping_add(offset);
    }

    abort();
}

unsafe fn check_gs_value(value: c_ulong) {
    let mut base: c_ulong;
    let sel: c_ushort;

    printf(c"[RUN]\tARCH_SET_GS to 0x%lx\n".as_ptr(), value);
    if syscall(SYS_arch_prctl, ARCH_SET_GS, value) != 0 {
        err(1, c"ARCH_SET_GS".as_ptr());
    }

    asm!("mov {0:x}, gs", out(reg) sel, options(nostack, preserves_flags));
    base = read_base(which_base::GS);
    if base == value {
        printf(c"[OK]\tGSBASE was set as expected (selector 0x%hx)\n".as_ptr(), sel as c_int);
    } else {
        nerrs += 1;
        printf(
            c"[FAIL]\tGSBASE was not as expected: got 0x%lx (selector 0x%hx)\n".as_ptr(),
            base,
            sel as c_int,
        );
    }

    if syscall(SYS_arch_prctl, ARCH_GET_GS, &mut base as *mut c_ulong) != 0 {
        err(1, c"ARCH_GET_GS".as_ptr());
    }
    if base == value {
        printf(c"[OK]\tARCH_GET_GS worked as expected (selector 0x%hx)\n".as_ptr(), sel as c_int);
    } else {
        nerrs += 1;
        printf(
            c"[FAIL]\tARCH_GET_GS was not as expected: got 0x%lx (selector 0x%hx)\n".as_ptr(),
            base,
            sel as c_int,
        );
    }
}

unsafe fn mov_0_gs(initial_base: c_ulong, schedule: bool) {
    let base: c_ulong;
    let mut arch_base: c_ulong = 0;

    printf(
        c"[RUN]\tARCH_SET_GS to 0x%lx then mov 0 to %%gs%s\n".as_ptr(),
        initial_base,
        if schedule { c" and schedule ".as_ptr() } else { c"".as_ptr() },
    );
    if syscall(SYS_arch_prctl, ARCH_SET_GS, initial_base) != 0 {
        err(1, c"ARCH_SET_GS".as_ptr());
    }

    if schedule {
        usleep(10);
    }

    asm!("mov gs, {0:x}", in(reg) 0_u16, options(nostack, preserves_flags));
    base = read_base(which_base::GS);
    if syscall(SYS_arch_prctl, ARCH_GET_GS, &mut arch_base as *mut c_ulong) != 0 {
        err(1, c"ARCH_GET_GS".as_ptr());
    }
    if base == arch_base {
        printf(c"[OK]\tGSBASE is 0x%lx\n".as_ptr(), base);
    } else {
        nerrs += 1;
        printf(
            c"[FAIL]\tGSBASE changed to 0x%lx but kernel reports 0x%lx\n".as_ptr(),
            base,
            arch_base,
        );
    }
}

static mut remote_base: c_ulong = 0;
static mut ftx: c_uint = 0;

/*
 * ARCH_SET_FS/GS(0) may or may not program a selector of zero.  HARD_ZERO
 * means to force the selector to zero to improve test coverage.
 */
const HARD_ZERO: c_ulong = 0xa1fa5f343cb85fa4;

unsafe fn do_remote_base() {
    let mut to_set = remote_base;
    let mut hard_zero = false;
    if to_set == HARD_ZERO {
        to_set = 0;
        hard_zero = true;
    }

    if syscall(SYS_arch_prctl, ARCH_SET_GS, to_set) != 0 {
        err(1, c"ARCH_SET_GS".as_ptr());
    }

    if hard_zero {
        asm!("mov gs, {0:x}", in(reg) 0_u16, options(nostack, preserves_flags));
    }

    let sel: c_ushort;
    asm!("mov {0:x}, gs", out(reg) sel, options(nostack, preserves_flags));
    printf(
        c"\tother thread: ARCH_SET_GS(0x%lx)%s -- sel is 0x%hx\n".as_ptr(),
        to_set,
        if hard_zero { c" and clear gs".as_ptr() } else { c"".as_ptr() },
        sel as c_int,
    );
}

#[thread_local]
static mut set_thread_area_entry_number: c_int = -1;

unsafe fn load_gs() -> c_ushort {
    /*
     * Sets GS != 0 and GSBASE != 0 but arranges for the kernel to think
     * that GSBASE == 0 (i.e. thread.gsbase == 0).
     */

    /* Step 1: tell the kernel that we have GSBASE == 0. */
    if syscall(SYS_arch_prctl, ARCH_SET_GS, 0) != 0 {
        err(1, c"ARCH_SET_GS".as_ptr());
    }

    /* Step 2: change GSBASE without telling the kernel. */
    let mut desc = user_desc::new(
        0,
        0xBAADF00D,
        0xfffff,
        1,
        0, /* Data, grow-up */
        0,
        1,
        0,
        0,
    );
    if syscall(
        SYS_modify_ldt,
        1,
        &mut desc as *mut user_desc,
        size_of::<user_desc>(),
    ) == 0
    {
        printf(c"\tusing LDT slot 0\n".as_ptr());
        asm!("mov gs, {0:x}", in(reg) 0x7_u16, options(nostack, preserves_flags));
        return 0x7;
    } else {
        /* No modify_ldt for us (configured out, perhaps) */

        let low_desc = mmap(
            null_mut(),
            size_of::<user_desc>(),
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS | MAP_32BIT,
            -1,
            0,
        ) as *mut user_desc;
        memcpy(
            low_desc as *mut c_void,
            &desc as *const user_desc as *const c_void,
            size_of::<user_desc>(),
        );

        (*low_desc).entry_number = set_thread_area_entry_number as c_uint;

        /* 32-bit set_thread_area */
        let ret: c_long;
        asm!(
            "int 0x80",
            inlateout("eax") 243_i32 => ret,
            in("ebx") low_desc,
            inout("m") *low_desc,
            lateout("r8") _,
            lateout("r9") _,
            lateout("r10") _,
            lateout("r11") _,
        );
        memcpy(
            &mut desc as *mut user_desc as *mut c_void,
            low_desc as *const c_void,
            size_of::<user_desc>(),
        );
        munmap(low_desc as *mut c_void, size_of::<user_desc>());

        if ret != 0 {
            printf(c"[NOTE]\tcould not create a segment -- test won't do anything\n".as_ptr());
            return 0;
        }
        printf(c"\tusing GDT slot %d\n".as_ptr(), desc.entry_number as c_int);
        set_thread_area_entry_number = desc.entry_number as c_int;

        let gs: c_ushort = ((desc.entry_number << 3) | 0x3) as c_ushort;
        asm!("mov gs, {0:x}", in(reg) gs, options(nostack, preserves_flags));
        return gs;
    }
}

unsafe fn test_wrbase(index: c_ushort, base: c_ulong) {
    let newindex: c_ushort;
    let newbase: c_ulong;

    printf(c"[RUN]\tGS = 0x%hx, GSBASE = 0x%lx\n".as_ptr(), index as c_int, base);

    asm!("mov gs, {0:x}", in(reg) index, options(nostack, preserves_flags));
    wrgsbase(base);

    remote_base = 0;
    ftx = 1;
    syscall(SYS_futex, &raw mut ftx, FUTEX_WAKE, 0, null::<c_void>(), null::<c_void>(), 0);
    while ftx != 0 {
        syscall(SYS_futex, &raw mut ftx, FUTEX_WAIT, 1, null::<c_void>(), null::<c_void>(), 0);
    }

    asm!("mov {0:x}, gs", out(reg) newindex, options(nostack, preserves_flags));
    newbase = rdgsbase();

    if newindex == index && newbase == base {
        printf(c"[OK]\tIndex and base were preserved\n".as_ptr());
    } else {
        printf(
            c"[FAIL]\tAfter switch, GS = 0x%hx and GSBASE = 0x%lx\n".as_ptr(),
            newindex as c_int,
            newbase,
        );
        nerrs += 1;
    }
}

extern "C" fn threadproc(_ctx: *mut c_void) -> *mut c_void {
    unsafe {
        loop {
            while ftx == 0 {
                syscall(SYS_futex, &raw mut ftx, FUTEX_WAIT, 0, null::<c_void>(), null::<c_void>(), 0);
            }
            if ftx == 3 {
                return null_mut();
            }

            if ftx == 1 {
                do_remote_base();
            } else if ftx == 2 {
                /*
                 * On AMD chips, this causes GSBASE != 0, GS == 0, and
                 * thread.gsbase == 0.
                 */

                load_gs();
                asm!("mov gs, {0:x}", in(reg) 0_u16, options(nostack, preserves_flags));
            } else {
                errx(1, c"helper thread got bad command".as_ptr());
            }

            ftx = 0;
            syscall(SYS_futex, &raw mut ftx, FUTEX_WAKE, 0, null::<c_void>(), null::<c_void>(), 0);
        }
    }
}

unsafe fn set_gs_and_switch_to(mut local: c_ulong, force_sel: c_ushort, remote: c_ulong) {
    let base: c_ulong;
    let mut sel_pre_sched: c_ushort;
    let sel_post_sched: c_ushort;

    let mut hard_zero = false;
    if local == HARD_ZERO {
        hard_zero = true;
        local = 0;
    }

    printf(
        c"[RUN]\tARCH_SET_GS(0x%lx)%s, then schedule to 0x%lx\n".as_ptr(),
        local,
        if hard_zero { c" and clear gs".as_ptr() } else { c"".as_ptr() },
        remote,
    );
    if force_sel != 0 {
        printf(c"\tBefore schedule, set selector to 0x%hx\n".as_ptr(), force_sel as c_int);
    }
    if syscall(SYS_arch_prctl, ARCH_SET_GS, local) != 0 {
        err(1, c"ARCH_SET_GS".as_ptr());
    }
    if hard_zero {
        asm!("mov gs, {0:x}", in(reg) 0_u16, options(nostack, preserves_flags));
    }

    if read_base(which_base::GS) != local {
        nerrs += 1;
        printf(c"[FAIL]\tGSBASE wasn't set as expected\n".as_ptr());
    }

    if force_sel != 0 {
        asm!("mov gs, {0:x}", in(reg) force_sel, options(nostack, preserves_flags));
        sel_pre_sched = force_sel;
        local = read_base(which_base::GS);

        /*
         * Signal delivery is quite likely to change a selector
         * of 1, 2, or 3 back to 0 due to IRET being defective.
         */
        asm!("mov gs, {0:x}", in(reg) force_sel, options(nostack, preserves_flags));
    } else {
        asm!("mov {0:x}, gs", out(reg) sel_pre_sched, options(nostack, preserves_flags));
    }

    remote_base = remote;
    ftx = 1;
    syscall(SYS_futex, &raw mut ftx, FUTEX_WAKE, 0, null::<c_void>(), null::<c_void>(), 0);
    while ftx != 0 {
        syscall(SYS_futex, &raw mut ftx, FUTEX_WAIT, 1, null::<c_void>(), null::<c_void>(), 0);
    }

    asm!("mov {0:x}, gs", out(reg) sel_post_sched, options(nostack, preserves_flags));
    base = read_base(which_base::GS);
    if base == local && sel_pre_sched == sel_post_sched {
        printf(
            c"[OK]\tGS/BASE remained 0x%hx/0x%lx\n".as_ptr(),
            sel_pre_sched as c_int,
            local,
        );
    } else if base == local && sel_pre_sched >= 1 && sel_pre_sched <= 3 && sel_post_sched == 0 {
        /*
         * IRET is misdesigned and will squash selectors 1, 2, or 3
         * to zero.  Don't fail the test just because this happened.
         */
        printf(
            c"[OK]\tGS/BASE changed from 0x%hx/0x%lx to 0x%hx/0x%lx because IRET is defective\n".as_ptr(),
            sel_pre_sched as c_int,
            local,
            sel_post_sched as c_int,
            base,
        );
    } else {
        nerrs += 1;
        printf(
            c"[FAIL]\tGS/BASE changed from 0x%hx/0x%lx to 0x%hx/0x%lx\n".as_ptr(),
            sel_pre_sched as c_int,
            local,
            sel_post_sched as c_int,
            base,
        );
    }
}

unsafe fn test_unexpected_base() {
    let base: c_ulong;

    printf(c"[RUN]\tARCH_SET_GS(0), clear gs, then manipulate GSBASE in a different thread\n".as_ptr());
    if syscall(SYS_arch_prctl, ARCH_SET_GS, 0) != 0 {
        err(1, c"ARCH_SET_GS".as_ptr());
    }
    asm!("mov gs, {0:x}", in(reg) 0_u16, options(nostack, preserves_flags));

    ftx = 2;
    syscall(SYS_futex, &raw mut ftx, FUTEX_WAKE, 0, null::<c_void>(), null::<c_void>(), 0);
    while ftx != 0 {
        syscall(SYS_futex, &raw mut ftx, FUTEX_WAIT, 1, null::<c_void>(), null::<c_void>(), 0);
    }

    base = read_base(which_base::GS);
    if base == 0 {
        printf(c"[OK]\tGSBASE remained 0\n".as_ptr());
    } else {
        nerrs += 1;
        printf(c"[FAIL]\tGSBASE changed to 0x%lx\n".as_ptr(), base);
    }
}

macro_rules! USER_REGS_OFFSET {
    ($r:ident) => {
        offset_of!(user_regs_struct, $r)
    };
}

fn WSTOPSIG(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn test_ptrace_write_gs_read_base() {
    let mut status: c_int = 0;
    let child: pid_t = fork();

    if child < 0 {
        err(1, c"fork".as_ptr());
    }

    if child == 0 {
        printf(c"[RUN]\tPTRACE_POKE GS, read GSBASE back\n".as_ptr());

        printf(c"[RUN]\tARCH_SET_GS to 1\n".as_ptr());
        if syscall(SYS_arch_prctl, ARCH_SET_GS, 1) != 0 {
            err(1, c"ARCH_SET_GS".as_ptr());
        }

        if ptrace(PTRACE_TRACEME, 0, null::<c_void>(), null::<c_void>()) != 0 {
            err(1, c"PTRACE_TRACEME".as_ptr());
        }

        raise(SIGTRAP);
        _exit(0);
    }

    wait(&mut status as *mut c_int);

    if WSTOPSIG(status) == SIGTRAP {
        let mut base: c_ulong;
        let gs_offset: c_ulong = USER_REGS_OFFSET!(gs) as c_ulong;
        let base_offset: c_ulong = USER_REGS_OFFSET!(gs_base) as c_ulong;

        /* Read the initial base.  It should be 1. */
        base = ptrace(PTRACE_PEEKUSER, child, base_offset, null::<c_void>()) as c_ulong;
        if base == 1 {
            printf(c"[OK]\tGSBASE started at 1\n".as_ptr());
        } else {
            nerrs += 1;
            printf(c"[FAIL]\tGSBASE started at 0x%lx\n".as_ptr(), base);
        }

        printf(c"[RUN]\tSet GS = 0x7, read GSBASE\n".as_ptr());

        /* Poke an LDT selector into GS. */
        if ptrace(PTRACE_POKEUSER, child, gs_offset, 0x7) != 0 {
            err(1, c"PTRACE_POKEUSER".as_ptr());
        }

        /* And read the base. */
        base = ptrace(PTRACE_PEEKUSER, child, base_offset, null::<c_void>()) as c_ulong;

        if base == 0 || base == 1 {
            printf(c"[OK]\tGSBASE reads as 0x%lx with invalid GS\n".as_ptr(), base);
        } else {
            nerrs += 1;
            printf(c"[FAIL]\tGSBASE=0x%lx (should be 0 or 1)\n".as_ptr(), base);
        }
    }

    ptrace(PTRACE_CONT, child, null::<c_void>(), null::<c_void>());

    wait(&mut status as *mut c_int);
    if !WIFEXITED(status) {
        printf(c"[WARN]\tChild didn't exit cleanly.\n".as_ptr());
    }
}

unsafe fn test_ptrace_write_gsbase() {
    let mut status: c_int = 0;
    let child: pid_t = fork();

    if child < 0 {
        err(1, c"fork".as_ptr());
    }

    if child == 0 {
        printf(c"[RUN]\tPTRACE_POKE(), write GSBASE from ptracer\n".as_ptr());

        *shared_scratch = load_gs();

        if ptrace(PTRACE_TRACEME, 0, null::<c_void>(), null::<c_void>()) != 0 {
            err(1, c"PTRACE_TRACEME".as_ptr());
        }

        raise(SIGTRAP);
        _exit(0);
    }

    wait(&mut status as *mut c_int);

    if WSTOPSIG(status) == SIGTRAP {
        let mut gs: c_ulong;
        let base: c_ulong;
        let gs_offset: c_ulong = USER_REGS_OFFSET!(gs) as c_ulong;
        let base_offset: c_ulong = USER_REGS_OFFSET!(gs_base) as c_ulong;

        gs = ptrace(PTRACE_PEEKUSER, child, gs_offset, null::<c_void>()) as c_ulong;

        if gs != *shared_scratch as c_ulong {
            nerrs += 1;
            printf(c"[FAIL]\tGS is not prepared with nonzero\n".as_ptr());
            ptrace(PTRACE_CONT, child, null::<c_void>(), null::<c_void>());
            wait(&mut status as *mut c_int);
            if !WIFEXITED(status) {
                printf(c"[WARN]\tChild didn't exit cleanly.\n".as_ptr());
            }
            return;
        }

        if ptrace(PTRACE_POKEUSER, child, base_offset, 0xFF) != 0 {
            err(1, c"PTRACE_POKEUSER".as_ptr());
        }

        gs = ptrace(PTRACE_PEEKUSER, child, gs_offset, null::<c_void>()) as c_ulong;
        base = ptrace(PTRACE_PEEKUSER, child, base_offset, null::<c_void>()) as c_ulong;

        /*
         * In a non-FSGSBASE system, the nonzero selector will load
         * GSBASE (again). But what is tested here is whether the
         * selector value is changed or not by the GSBASE write in
         * a ptracer.
         */
        if gs != *shared_scratch as c_ulong {
            nerrs += 1;
            printf(c"[FAIL]\tGS changed to %lx\n".as_ptr(), gs);

            /*
             * On older kernels, poking a nonzero value into the
             * base would zero the selector.  On newer kernels,
             * this behavior has changed -- poking the base
             * changes only the base and, if FSGSBASE is not
             * available, this may have no effect once the tracee
             * is resumed.
             */
            if gs == 0 {
                printf(c"\tNote: this is expected behavior on older kernels.\n".as_ptr());
            }
        } else if have_fsgsbase && base != 0xFF {
            nerrs += 1;
            printf(c"[FAIL]\tGSBASE changed to %lx\n".as_ptr(), base);
        } else {
            printf(c"[OK]\tGS remained 0x%hx".as_ptr(), *shared_scratch as c_int);
            if have_fsgsbase {
                printf(c" and GSBASE changed to 0xFF".as_ptr());
            }
            printf(c"\n".as_ptr());
        }
    }

    ptrace(PTRACE_CONT, child, null::<c_void>(), null::<c_void>());
    wait(&mut status as *mut c_int);
    if !WIFEXITED(status) {
        printf(c"[WARN]\tChild didn't exit cleanly.\n".as_ptr());
    }
}

fn CPU_ZERO(cpuset: &mut cpu_set_t) {
    for word in cpuset.bits.iter_mut() {
        *word = 0;
    }
}

fn CPU_SET(cpu: usize, cpuset: &mut cpu_set_t) {
    let bits_per_word = 8 * size_of::<c_ulong>();
    cpuset.bits[cpu / bits_per_word] |= 1_c_ulong << (cpu % bits_per_word);
}

fn main() {
    unsafe {
        let mut thread: pthread_t = 0;

        shared_scratch = mmap(
            null_mut(),
            4096,
            PROT_READ | PROT_WRITE,
            MAP_ANONYMOUS | MAP_SHARED,
            -1,
            0,
        ) as *mut c_ushort;

        /* Do these tests before we have an LDT. */
        test_ptrace_write_gs_read_base();

        /* Probe FSGSBASE */
        sethandler(SIGILL, sigill, 0);
        if sigsetjmp(&raw mut jmpbuf, 1) == 0 {
            rdfsbase();
            have_fsgsbase = true;
            printf(c"\tFSGSBASE instructions are enabled\n".as_ptr());
        } else {
            printf(c"\tFSGSBASE instructions are disabled\n".as_ptr());
        }
        clearhandler(SIGILL);

        sethandler(SIGSEGV, sigsegv, 0);

        check_gs_value(0);
        check_gs_value(1);
        check_gs_value(0x200000000);
        check_gs_value(0);
        check_gs_value(0x200000000);
        check_gs_value(1);

        for sched in 0..2 {
            mov_0_gs(0, sched != 0);
            mov_0_gs(1, sched != 0);
            mov_0_gs(0x200000000, sched != 0);
        }

        /* Set up for multithreading. */

        let mut cpuset = cpu_set_t { bits: [0; 16] };
        CPU_ZERO(&mut cpuset);
        CPU_SET(0, &mut cpuset);
        if sched_setaffinity(0, size_of::<cpu_set_t>(), &cpuset as *const cpu_set_t) != 0 {
            err(1, c"sched_setaffinity to CPU 0".as_ptr()); /* should never fail */
        }

        if pthread_create(&mut thread as *mut pthread_t, null(), threadproc, null_mut()) != 0 {
            err(1, c"pthread_create".as_ptr());
        }

        static bases_with_hard_zero: [c_ulong; 4] = [0, HARD_ZERO, 1, 0x200000000];

        for local in 0..4 {
            for remote in 0..4 {
                for s in 0_u16..5 {
                    let mut sel: c_ushort = s;
                    if s == 4 {
                        asm!("mov {0:x}, ss", out(reg) sel, options(nostack, preserves_flags));
                    }
                    set_gs_and_switch_to(
                        bases_with_hard_zero[local],
                        sel,
                        bases_with_hard_zero[remote],
                    );
                }
            }
        }

        test_unexpected_base();

        if have_fsgsbase {
            let ss: c_ushort;

            asm!("mov {0:x}, ss", out(reg) ss, options(nostack, preserves_flags));

            test_wrbase(0, 0);
            test_wrbase(0, 1);
            test_wrbase(0, 0x200000000);
            test_wrbase(0, 0xffffffffffffffff);
            test_wrbase(ss, 0);
            test_wrbase(ss, 1);
            test_wrbase(ss, 0x200000000);
            test_wrbase(ss, 0xffffffffffffffff);
        }

        ftx = 3; /* Kill the thread. */
        syscall(SYS_futex, &raw mut ftx, FUTEX_WAKE, 0, null::<c_void>(), null::<c_void>(), 0);

        if pthread_join(thread, null_mut()) != 0 {
            err(1, c"pthread_join".as_ptr());
        }

        test_ptrace_write_gsbase();

        std::process::exit(if nerrs == 0 { 0 } else { 1 });
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
