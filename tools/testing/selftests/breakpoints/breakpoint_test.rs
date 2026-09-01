// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011 Red Hat, Inc., Frederic Weisbecker <fweisbec@redhat.com>
 *
 * Selftests for breakpoints (and more generally the do_debug() path) in x86.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type pid_t = c_int;

const COUNT_ISN_BPS: c_int = 4;
const COUNT_WPS: c_int = 4;

/* Breakpoint access modes */
const BP_X: c_int = 1;
const BP_RW: c_int = 2;
const BP_W: c_int = 4;

const PTRACE_TRACEME: c_int = 0;
const PTRACE_PEEKDATA: c_int = 2;
const PTRACE_POKEDATA: c_int = 5;
const PTRACE_CONT: c_int = 7;
const PTRACE_PEEKUSER: c_int = 3;
const PTRACE_POKEUSER: c_int = 6;
const SIGTRAP: c_int = 5;
const SIGUSR1: c_int = 10;

/* From <sys/user.h>: offsetof(struct user, u_debugreg[n]) on x86_64. */
const USER_U_DEBUGREG_OFFSET: usize = 848;
const USER_U_DEBUGREG_STRIDE: usize = size_of::<c_ulong>();

static mut child_pid: pid_t = 0;

/*
 * Ensures the child and parent are always "talking" about
 * the same test sequence. (ie: that we haven't forgotten
 * to call check_trapped() somewhere).
 */
static mut nr_tests: c_int = 0;

/* Dummy variables to test read/write accesses */
static mut dummy_var: [u64; 4] = [0; 4];

/* Dummy functions to test execution accesses */
unsafe extern "C" fn dummy_func() {}
unsafe extern "C" fn dummy_func1() {}
unsafe extern "C" fn dummy_func2() {}
unsafe extern "C" fn dummy_func3() {}

static mut dummy_funcs: [unsafe extern "C" fn(); 4] = [
    dummy_func,
    dummy_func1,
    dummy_func2,
    dummy_func3,
];

static mut trapped: c_int = 0;

unsafe extern "C" {
    static mut errno: c_int;

    fn ptrace(request: c_int, pid: pid_t, addr: *mut c_void, data: *mut c_void) -> c_long;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn getpid() -> pid_t;
    fn wait(status: *mut c_int) -> pid_t;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_msg(msg: *const c_char, ...);
    fn ksft_test_result_pass(msg: *const c_char, ...);
    fn ksft_test_result_fail(msg: *const c_char, ...);
    fn ksft_exit_pass() -> !;
    fn ksft_exit_fail_msg(msg: *const c_char, ...) -> !;
}

fn wstopsig(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

fn debugreg_offset(n: c_int) -> *mut c_void {
    (USER_U_DEBUGREG_OFFSET + n as usize * USER_U_DEBUGREG_STRIDE) as *mut c_void
}

unsafe fn set_breakpoint_addr(addr: *mut c_void, n: c_int) {
    let ret: c_int;

    ret = ptrace(
        PTRACE_POKEUSER,
        child_pid,
        debugreg_offset(n),
        addr,
    ) as c_int;
    if ret != 0 {
        ksft_exit_fail_msg(
            c"Can't set breakpoint addr: %s\n".as_ptr(),
            strerror(errno),
        );
    }
}

unsafe fn toggle_breakpoint(n: c_int, type_: c_int, len: c_int, local: c_int, global: c_int, set: c_int) {
    let ret: c_int;

    let xtype: c_int;
    let xlen: c_int;
    let vdr7: c_ulong;
    let mut dr7: c_ulong;

    match type_ {
        BP_X => {
            xtype = 0;
        }
        BP_W => {
            xtype = 1;
        }
        BP_RW => {
            xtype = 3;
        }
        _ => {
            xtype = 0;
        }
    }

    match len {
        1 => {
            xlen = 0;
        }
        2 => {
            xlen = 4;
        }
        4 => {
            xlen = 0xc;
        }
        8 => {
            xlen = 8;
        }
        _ => {
            xlen = 0;
        }
    }

    dr7 = ptrace(
        PTRACE_PEEKUSER,
        child_pid,
        debugreg_offset(7),
        ptr::null_mut(),
    ) as c_ulong;

    vdr7 = ((xlen | xtype) as c_ulong) << 16;
    let mut vdr7 = vdr7 << (4 * n);

    if local != 0 {
        vdr7 |= (1 as c_ulong) << (2 * n);
        vdr7 |= (1 as c_ulong) << 8;
    }
    if global != 0 {
        vdr7 |= (2 as c_ulong) << (2 * n);
        vdr7 |= (1 as c_ulong) << 9;
    }

    if set != 0 {
        dr7 |= vdr7;
    } else {
        dr7 &= !vdr7;
    }

    ret = ptrace(
        PTRACE_POKEUSER,
        child_pid,
        debugreg_offset(7),
        dr7 as *mut c_void,
    ) as c_int;
    if ret != 0 {
        ksft_print_msg(c"Can't set dr7: %s\n".as_ptr(), strerror(errno));
        exit(-1);
    }
}

unsafe fn check_trapped() {
    /*
     * If we haven't trapped, wake up the parent
     * so that it notices the failure.
     */
    if trapped == 0 {
        kill(getpid(), SIGUSR1);
    }
    trapped = 0;

    nr_tests += 1;
}

unsafe fn write_var(len: c_int) {
    let pcval: *mut c_char;
    let psval: *mut i16;
    let pival: *mut c_int;
    let plval: *mut i64;
    let mut i: c_int;

    i = 0;
    while i < 4 {
        match len {
            1 => {
                pcval = (&mut dummy_var[i as usize] as *mut u64).cast::<c_char>();
                *pcval = 0xff_u8 as c_char;
            }
            2 => {
                psval = (&mut dummy_var[i as usize] as *mut u64).cast::<i16>();
                *psval = 0xffff_u16 as i16;
            }
            4 => {
                pival = (&mut dummy_var[i as usize] as *mut u64).cast::<c_int>();
                *pival = 0xffffffff_u32 as c_int;
            }
            8 => {
                plval = (&mut dummy_var[i as usize] as *mut u64).cast::<i64>();
                *plval = 0xffffffffffffffff_u64 as i64;
            }
            _ => {}
        }
        check_trapped();
        i += 1;
    }
}

unsafe fn read_var(len: c_int) {
    let cval: c_char;
    let sval: i16;
    let ival: c_int;
    let lval: i64;
    let mut i: c_int;

    i = 0;
    while i < 4 {
        match len {
            1 => {
                cval = *(&dummy_var[i as usize] as *const u64).cast::<c_char>();
                let _ = cval;
            }
            2 => {
                sval = *(&dummy_var[i as usize] as *const u64).cast::<i16>();
                let _ = sval;
            }
            4 => {
                ival = *(&dummy_var[i as usize] as *const u64).cast::<c_int>();
                let _ = ival;
            }
            8 => {
                lval = *(&dummy_var[i as usize] as *const u64).cast::<i64>();
                let _ = lval;
            }
            _ => {}
        }
        check_trapped();
        i += 1;
    }
}

/*
 * Do the r/w/x accesses to trigger the breakpoints. And run
 * the usual traps.
 */
unsafe fn trigger_tests() {
    let mut len: c_int;
    let mut local: c_int;
    let mut global: c_int;
    let mut i: c_int;
    let _val: c_char;
    let ret: c_int;

    ret = ptrace(PTRACE_TRACEME, 0, ptr::null_mut(), ptr::null_mut()) as c_int;
    if ret != 0 {
        ksft_print_msg(c"Can't be traced? %s\n".as_ptr(), strerror(errno));
        return;
    }

    /* Wake up father so that it sets up the first test */
    kill(getpid(), SIGUSR1);

    /* Test instruction breakpoints */
    local = 0;
    while local < 2 {
        global = 0;
        while global < 2 {
            if local == 0 && global == 0 {
                global += 1;
                continue;
            }

            i = 0;
            while i < COUNT_ISN_BPS {
                dummy_funcs[i as usize]();
                check_trapped();
                i += 1;
            }
            global += 1;
        }
        local += 1;
    }

    /* Test write watchpoints */
    len = 1;
    while len <= size_of::<c_long>() as c_int {
        local = 0;
        while local < 2 {
            global = 0;
            while global < 2 {
                if local == 0 && global == 0 {
                    global += 1;
                    continue;
                }
                write_var(len);
                global += 1;
            }
            local += 1;
        }
        len <<= 1;
    }

    /* Test read/write watchpoints (on read accesses) */
    len = 1;
    while len <= size_of::<c_long>() as c_int {
        local = 0;
        while local < 2 {
            global = 0;
            while global < 2 {
                if local == 0 && global == 0 {
                    global += 1;
                    continue;
                }
                read_var(len);
                global += 1;
            }
            local += 1;
        }
        len <<= 1;
    }

    /* Icebp trap */
    core::arch::asm!(".byte 0xf1");
    check_trapped();

    /* Int 3 trap */
    core::arch::asm!("int3");
    check_trapped();

    kill(getpid(), SIGUSR1);
}

unsafe fn check_success(msg: *const c_char) {
    let child_nr_tests: c_int;
    let mut status: c_int = 0;
    let mut ret: c_int;

    /* Wait for the child to SIGTRAP */
    wait(&mut status);

    ret = 0;

    if wstopsig(status) == SIGTRAP {
        child_nr_tests = ptrace(
            PTRACE_PEEKDATA,
            child_pid,
            (&mut nr_tests as *mut c_int).cast::<c_void>(),
            ptr::null_mut(),
        ) as c_int;
        if child_nr_tests == nr_tests {
            ret = 1;
        }
        if ptrace(
            PTRACE_POKEDATA,
            child_pid,
            (&mut trapped as *mut c_int).cast::<c_void>(),
            1 as *mut c_void,
        ) != 0 {
            ksft_exit_fail_msg(c"Can't poke: %s\n".as_ptr(), strerror(errno));
        }
    }

    nr_tests += 1;

    if ret != 0 {
        ksft_test_result_pass(c"%s".as_ptr(), msg);
    } else {
        ksft_test_result_fail(c"%s".as_ptr(), msg);
    }
}

unsafe fn launch_instruction_breakpoints(buf: *mut c_char, local: c_int, global: c_int) {
    let mut i: c_int;

    i = 0;
    while i < COUNT_ISN_BPS {
        set_breakpoint_addr(dummy_funcs[i as usize] as *mut c_void, i);
        toggle_breakpoint(i, BP_X, 1, local, global, 1);
        ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
        sprintf(
            buf,
            c"Test breakpoint %d with local: %d global: %d\n".as_ptr(),
            i,
            local,
            global,
        );
        check_success(buf);
        toggle_breakpoint(i, BP_X, 1, local, global, 0);
        i += 1;
    }
}

unsafe fn launch_watchpoints(buf: *mut c_char, mode: c_int, len: c_int, local: c_int, global: c_int) {
    let mode_str: *const c_char;
    let mut i: c_int;

    if mode == BP_W {
        mode_str = c"write".as_ptr();
    } else {
        mode_str = c"read".as_ptr();
    }

    i = 0;
    while i < COUNT_WPS {
        set_breakpoint_addr((&mut dummy_var[i as usize] as *mut u64).cast::<c_void>(), i);
        toggle_breakpoint(i, mode, len, local, global, 1);
        ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
        sprintf(
            buf,
            c"Test %s watchpoint %d with len: %d local: %d global: %d\n".as_ptr(),
            mode_str,
            i,
            len,
            local,
            global,
        );
        check_success(buf);
        toggle_breakpoint(i, mode, len, local, global, 0);
        i += 1;
    }
}

/* Set the breakpoints and check the child successfully trigger them */
unsafe fn launch_tests() {
    let mut buf: [c_char; 1024] = [0; 1024];
    let mut tests: c_uint = 0;
    let mut len: c_int;
    let mut local: c_int;
    let mut global: c_int;
    let _i: c_int;

    tests += (3 * COUNT_ISN_BPS) as c_uint;
    tests += (size_of::<c_long>() / 2 * 3 * COUNT_WPS as usize) as c_uint;
    tests += (size_of::<c_long>() / 2 * 3 * COUNT_WPS as usize) as c_uint;
    tests += 2;
    ksft_set_plan(tests);

    /* Instruction breakpoints */
    local = 0;
    while local < 2 {
        global = 0;
        while global < 2 {
            if local == 0 && global == 0 {
                global += 1;
                continue;
            }
            launch_instruction_breakpoints(buf.as_mut_ptr(), local, global);
            global += 1;
        }
        local += 1;
    }

    /* Write watchpoint */
    len = 1;
    while len <= size_of::<c_long>() as c_int {
        local = 0;
        while local < 2 {
            global = 0;
            while global < 2 {
                if local == 0 && global == 0 {
                    global += 1;
                    continue;
                }
                launch_watchpoints(buf.as_mut_ptr(), BP_W, len, local, global);
                global += 1;
            }
            local += 1;
        }
        len <<= 1;
    }

    /* Read-Write watchpoint */
    len = 1;
    while len <= size_of::<c_long>() as c_int {
        local = 0;
        while local < 2 {
            global = 0;
            while global < 2 {
                if local == 0 && global == 0 {
                    global += 1;
                    continue;
                }
                launch_watchpoints(buf.as_mut_ptr(), BP_RW, len, local, global);
                global += 1;
            }
            local += 1;
        }
        len <<= 1;
    }

    /* Icebp traps */
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(c"Test icebp\n".as_ptr());

    /* Int 3 traps */
    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
    check_success(c"Test int 3 trap\n".as_ptr());

    ptrace(PTRACE_CONT, child_pid, ptr::null_mut(), ptr::null_mut());
}

unsafe fn main_impl(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let pid: pid_t;
    let _ret: c_int;

    ksft_print_header();

    pid = fork();
    if pid == 0 {
        trigger_tests();
        exit(0);
    }

    child_pid = pid;

    wait(ptr::null_mut());

    launch_tests();

    wait(ptr::null_mut());

    ksft_exit_pass();
}

fn main() {
    unsafe {
        main_impl(0, ptr::null_mut());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
