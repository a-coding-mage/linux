// SPDX-License-Identifier: GPL-2.0-only

// Translated from includes:
// ../../kselftest.h, sys/wait.h, signal.h, fcntl.h, asm-generic/unistd.h,
// sys/mman.h, shadowstack.h, cfi_rv_test.h

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type CallFuncPtr = unsafe extern "C" fn();
type ShadowStackTestFn = unsafe extern "C" fn(c_ulong, *mut c_void) -> bool;

#[repr(C)]
pub struct shadow_stack_tests {
    pub name: *const c_char,
    pub t_func: Option<ShadowStackTestFn>,
}

unsafe extern "C" {
    fn csr_read(csr: c_ulong) -> c_ulong;
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_set_plan(cnt: c_ulong);
    fn ksft_test_result(pass: bool, name: *const c_char);
    fn ksft_finished();

    fn my_syscall3(nr: c_long, arg1: *mut c_void, arg2: c_ulong, arg3: c_ulong) -> c_ulong;
    fn my_syscall5(
        nr: c_long,
        arg1: c_ulong,
        arg2: *mut c_ulong,
        arg3: c_ulong,
        arg4: c_ulong,
        arg5: c_ulong,
    ) -> c_int;

    fn getpid() -> c_int;
    fn fork() -> c_int;
    fn wait(wstatus: *mut c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn lseek(fd: c_int, offset: c_long, whence: c_int) -> c_long;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn kill(pid: c_int, sig: c_int) -> c_int;
}

type c_uint = u32;

const CSR_SSP: c_ulong = 0x011;
const __NR_PRCTL: c_long = 167;
const __NR_MAP_SHADOW_STACK: c_long = 453;
const PR_GET_SHADOW_STACK_STATUS: c_ulong = 74;
const PR_SHADOW_STACK_ENABLE: c_ulong = 1 << 0;
const CHILD_EXIT_CODE_SSWRITE: c_int = 10;
const O_RDWR: c_int = 0o2;
const SEEK_SET: c_int = 0;
const SIGUSR1: c_int = 10;

static mut SHSTK_TESTS: [shadow_stack_tests; 5] = [
    shadow_stack_tests {
        name: c"shstk fork test\n".as_ptr(),
        t_func: Some(shadow_stack_fork_test),
    },
    shadow_stack_tests {
        name: c"map shadow stack syscall\n".as_ptr(),
        t_func: Some(shadow_stack_map_test),
    },
    shadow_stack_tests {
        name: c"shadow stack gup tests\n".as_ptr(),
        t_func: Some(shadow_stack_gup_tests),
    },
    shadow_stack_tests {
        name: c"shadow stack signal tests\n".as_ptr(),
        t_func: Some(shadow_stack_signal_test),
    },
    shadow_stack_tests {
        name: c"memory protections of shadow stack memory\n".as_ptr(),
        t_func: Some(shadow_stack_protection_test),
    },
];

const RISCV_SHADOW_STACK_TESTS: c_ulong = SHSTK_TESTS.len() as c_ulong;

// do not optimize shadow stack related test functions
// C source used:
// #pragma GCC push_options
// #pragma GCC optimize("O0")

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zar() {
    let mut ssp: c_ulong = 0;

    ssp = unsafe { csr_read(CSR_SSP) };
    unsafe {
        ksft_print_msg(
            c"Spewing out shadow stack ptr: %lx\n  This is to ensure shadow stack is indeed enabled and working\n"
                .as_ptr(),
            ssp,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bar() {
    unsafe {
        zar();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn foo() {
    unsafe {
        bar();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zar_child() {
    let mut ssp: c_ulong = 0;

    ssp = unsafe { csr_read(CSR_SSP) };
    unsafe {
        ksft_print_msg(
            c"Spewing out shadow stack ptr: %lx\n  This is to ensure shadow stack is indeed enabled and working\n"
                .as_ptr(),
            ssp,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bar_child() {
    unsafe {
        zar_child();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn foo_child() {
    unsafe {
        bar_child();
    }
}

/*
 * call couple of functions to test push/pop.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shadow_stack_call_tests(fn_ptr: CallFuncPtr, parent: bool) -> c_int {
    unsafe {
        ksft_print_msg(
            c"dummy calls for sspush and sspopchk in context of %s\n".as_ptr(),
            if parent {
                c"parent".as_ptr()
            } else {
                c"child".as_ptr()
            },
        );

        fn_ptr();
    }

    0
}

/* forks a thread, and ensure shadow stacks fork out */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shadow_stack_fork_test(_test_num: c_ulong, _ctx: *mut c_void) -> bool {
    let mut pid: c_int = 0;
    let mut child_status: c_int = 0;
    let mut parent_pid: c_int = 0;
    let mut ret: c_int = 0;
    let mut ss_status: c_ulong = 0;

    unsafe {
        ksft_print_msg(c"Exercising shadow stack fork test\n".as_ptr());
    }

    ret = unsafe {
        my_syscall5(
            __NR_PRCTL,
            PR_GET_SHADOW_STACK_STATUS,
            &mut ss_status,
            0,
            0,
            0,
        )
    };
    if ret != 0 {
        unsafe {
            ksft_exit_skip(
                c"Shadow stack get status prctl failed with errorcode %d\n".as_ptr(),
                ret,
            );
        }
    }

    if (ss_status & PR_SHADOW_STACK_ENABLE) == 0 {
        unsafe {
            ksft_exit_skip(c"Shadow stack is not enabled, should be enabled via glibc\n".as_ptr());
        }
    }

    parent_pid = unsafe { getpid() };
    pid = unsafe { fork() };

    if pid != 0 {
        unsafe {
            ksft_print_msg(c"Parent pid %d and child pid %d\n".as_ptr(), parent_pid, pid);
            shadow_stack_call_tests(foo, true);
        }
    } else {
        unsafe {
            shadow_stack_call_tests(foo_child, false);
        }
    }

    if pid != 0 {
        unsafe {
            ksft_print_msg(c"Waiting on child to finish\n".as_ptr());
            wait(&mut child_status);
        }
    } else {
        /* exit child gracefully */
        unsafe {
            exit(0);
        }
    }

    if pid != 0 && wifsignaled(child_status) {
        unsafe {
            ksft_print_msg(c"Child faulted, fork test failed\n".as_ptr());
        }
        return false;
    }

    true
}

/* exercise 'map_shadow_stack', pivot to it and call some functions to ensure it works */
const SHADOW_STACK_ALLOC_SIZE: usize = 4096;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shadow_stack_map_test(_test_num: c_ulong, _ctx: *mut c_void) -> bool {
    let mut shdw_addr: c_ulong;
    let mut ret: c_int = 0;

    unsafe {
        ksft_print_msg(c"Exercising shadow stack map test\n".as_ptr());
    }

    shdw_addr =
        unsafe { my_syscall3(__NR_MAP_SHADOW_STACK, core::ptr::null_mut(), SHADOW_STACK_ALLOC_SIZE as c_ulong, 0) };

    if (shdw_addr as c_long) <= 0 {
        unsafe {
            ksft_print_msg(
                c"map_shadow_stack failed with error code %d\n".as_ptr(),
                shdw_addr as c_int,
            );
        }
        return false;
    }

    ret = unsafe { munmap(shdw_addr as *mut c_void, SHADOW_STACK_ALLOC_SIZE) };

    if ret != 0 {
        unsafe {
            ksft_print_msg(c"munmap failed with error code %d\n".as_ptr(), ret);
        }
        return false;
    }

    true
}

/*
 * shadow stack protection tests. map a shadow stack and
 * validate all memory protections work on it
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shadow_stack_protection_test(
    _test_num: c_ulong,
    _ctx: *mut c_void,
) -> bool {
    let mut shdw_addr: c_ulong;
    let mut write_addr: *mut c_ulong = core::ptr::null_mut();
    let mut ret: c_int = 0;
    let mut pid: c_int = 0;
    let mut child_status: c_int = 0;

    unsafe {
        ksft_print_msg(c"Exercising shadow stack protection test (WPT)\n".as_ptr());
    }

    shdw_addr =
        unsafe { my_syscall3(__NR_MAP_SHADOW_STACK, core::ptr::null_mut(), SHADOW_STACK_ALLOC_SIZE as c_ulong, 0) };

    if (shdw_addr as c_long) <= 0 {
        unsafe {
            ksft_print_msg(
                c"map_shadow_stack failed with error code %d\n".as_ptr(),
                shdw_addr as c_int,
            );
        }
        return false;
    }

    write_addr = shdw_addr as *mut c_ulong;
    pid = unsafe { fork() };

    /* no child was created, return false */
    if pid == -1 {
        return false;
    }

    /*
     * try to perform a store from child on shadow stack memory
     * it should result in SIGSEGV
     */
    if pid == 0 {
        /* below write must lead to SIGSEGV */
        unsafe {
            *write_addr = 0xdeadbeef;
        }
    } else {
        unsafe {
            wait(&mut child_status);
        }
    }

    /* test fail, if 0xdeadbeef present on shadow stack address */
    if unsafe { *write_addr } == 0xdeadbeef {
        unsafe {
            ksft_print_msg(c"Shadow stack WPT failed\n".as_ptr());
        }
        return false;
    }

    /* if child reached here, then fail */
    if pid == 0 {
        unsafe {
            ksft_print_msg(c"Shadow stack WPT failed: child reached unreachable state\n".as_ptr());
        }
        return false;
    }

    /* if child exited via signal handler but not for write on ss */
    if wifexited(child_status) && wexitstatus(child_status) != CHILD_EXIT_CODE_SSWRITE {
        unsafe {
            ksft_print_msg(c"Shadow stack WPT failed: child wasn't signaled for write\n".as_ptr());
        }
        return false;
    }

    ret = unsafe { munmap(write_addr as *mut c_void, SHADOW_STACK_ALLOC_SIZE) };
    if ret != 0 {
        unsafe {
            ksft_print_msg(
                c"Shadow stack WPT failed: munmap failed, error code %d\n".as_ptr(),
                ret,
            );
        }
        return false;
    }

    true
}

const SS_MAGIC_WRITE_VAL: c_ulong = 0xbeefdead;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gup_tests(mem_fd: c_int, shdw_addr: *mut c_ulong) -> c_int {
    let mut val: c_ulong = 0;

    unsafe {
        lseek(mem_fd, shdw_addr as c_ulong as c_long, SEEK_SET);
    }
    if unsafe {
        read(
            mem_fd,
            &mut val as *mut c_ulong as *mut c_void,
            core::mem::size_of_val(&val),
        )
    } < 0
    {
        unsafe {
            ksft_print_msg(c"Reading shadow stack mem via gup failed\n".as_ptr());
        }
        return 1;
    }

    val = SS_MAGIC_WRITE_VAL;
    unsafe {
        lseek(mem_fd, shdw_addr as c_ulong as c_long, SEEK_SET);
    }
    if unsafe {
        write(
            mem_fd,
            &val as *const c_ulong as *const c_void,
            core::mem::size_of_val(&val),
        )
    } < 0
    {
        unsafe {
            ksft_print_msg(c"Writing shadow stack mem via gup failed\n".as_ptr());
        }
        return 1;
    }

    if unsafe { *shdw_addr } != SS_MAGIC_WRITE_VAL {
        unsafe {
            ksft_print_msg(c"GUP write to shadow stack memory failed\n".as_ptr());
        }
        return 1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shadow_stack_gup_tests(_test_num: c_ulong, _ctx: *mut c_void) -> bool {
    let mut shdw_addr: c_ulong = 0;
    let mut write_addr: *mut c_ulong = core::ptr::null_mut();
    let mut fd: c_int = 0;
    let mut ret: bool = false;

    unsafe {
        ksft_print_msg(c"Exercising shadow stack gup tests\n".as_ptr());
    }
    shdw_addr =
        unsafe { my_syscall3(__NR_MAP_SHADOW_STACK, core::ptr::null_mut(), SHADOW_STACK_ALLOC_SIZE as c_ulong, 0) };

    if (shdw_addr as c_long) <= 0 {
        unsafe {
            ksft_print_msg(
                c"map_shadow_stack failed with error code %d\n".as_ptr(),
                shdw_addr as c_int,
            );
        }
        return false;
    }

    write_addr = shdw_addr as *mut c_ulong;

    fd = unsafe { open(c"/proc/self/mem".as_ptr(), O_RDWR) };
    if fd == -1 {
        return false;
    }

    if unsafe { gup_tests(fd, write_addr) } != 0 {
        unsafe {
            ksft_print_msg(c"gup tests failed\n".as_ptr());
        }
    } else {
        ret = true;
    }

    if shdw_addr != 0 && unsafe { munmap(write_addr as *mut c_void, SHADOW_STACK_ALLOC_SIZE) } != 0 {
        unsafe {
            ksft_print_msg(c"munmap failed with error code %d\n".as_ptr(), ret);
        }
        ret = false;
    }

    ret
}

#[unsafe(no_mangle)]
pub static mut break_loop: bool = false;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigusr1_handler(_signo: c_int) {
    unsafe {
        break_loop = true;
    }
}

#[repr(C)]
pub struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
pub struct sigaction {
    pub sa_handler: unsafe extern "C" fn(c_int),
    pub sa_flags: c_ulong,
    pub sa_restorer: *mut c_void,
    pub sa_mask: sigset_t,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigusr1_signal_test() -> bool {
    let mut sa: sigaction = unsafe { core::mem::zeroed() };

    sa.sa_handler = sigusr1_handler;
    sa.sa_flags = 0;
    unsafe {
        sigemptyset(&mut sa.sa_mask);
    }
    if unsafe { sigaction(SIGUSR1, &sa, core::ptr::null_mut()) } != 0 {
        unsafe {
            ksft_print_msg(c"Registering signal handler for SIGUSR1 failed\n".as_ptr());
        }
        return false;
    }

    true
}

/*
 * shadow stack signal test. shadow stack must be enabled.
 * register a signal, fork another thread which is waiting
 * on signal. Send a signal from parent to child, verify
 * that signal was received by child. If not test fails
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shadow_stack_signal_test(_test_num: c_ulong, _ctx: *mut c_void) -> bool {
    let mut pid: c_int = 0;
    let mut child_status: c_int = 0;
    let mut ret: c_int = 0;
    let mut ss_status: c_ulong = 0;

    unsafe {
        ksft_print_msg(c"Exercising shadow stack signal test\n".as_ptr());
    }

    ret = unsafe {
        my_syscall5(
            __NR_PRCTL,
            PR_GET_SHADOW_STACK_STATUS,
            &mut ss_status,
            0,
            0,
            0,
        )
    };
    if ret != 0 {
        unsafe {
            ksft_print_msg(
                c"Shadow stack get status prctl failed with errorcode %d\n".as_ptr(),
                ret,
            );
        }
        return false;
    }

    if (ss_status & PR_SHADOW_STACK_ENABLE) == 0 {
        unsafe {
            ksft_print_msg(c"Shadow stack is not enabled, should be enabled via glibc\n".as_ptr());
        }
    }

    /* this should be caught by signal handler and do an exit */
    if !unsafe { sigusr1_signal_test() } {
        unsafe {
            ksft_print_msg(c"Registering sigusr1 handler failed\n".as_ptr());
            exit(-1);
        }
    }

    pid = unsafe { fork() };

    if pid == -1 {
        unsafe {
            ksft_print_msg(c"Signal test: fork failed\n".as_ptr());
        }
    } else if pid == 0 {
        while unsafe { !break_loop } {
            unsafe {
                sleep(1);
            }
        }

        unsafe {
            exit(11);
        }
        /* child shouldn't go beyond here */
    } else {
        /* send SIGUSR1 to child */
        unsafe {
            kill(pid, SIGUSR1);
            wait(&mut child_status);
        }
    }

    wifexited(child_status) && wexitstatus(child_status) == 11
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn execute_shadow_stack_tests() -> c_int {
    let mut ret: c_int = 0;
    let mut test_count: c_ulong = 0;
    let mut shstk_status: c_ulong = 0;
    let mut test_pass: bool = false;

    unsafe {
        ksft_print_msg(c"Executing RISC-V shadow stack self tests\n".as_ptr());
        ksft_set_plan(RISCV_SHADOW_STACK_TESTS);
    }

    ret = unsafe {
        my_syscall5(
            __NR_PRCTL,
            PR_GET_SHADOW_STACK_STATUS,
            &mut shstk_status,
            0,
            0,
            0,
        )
    };

    if ret != 0 {
        unsafe {
            ksft_exit_fail_msg(c"Get shadow stack status failed with %d\n".as_ptr(), ret);
        }
    }

    /*
     * If we are here that means get shadow stack status succeeded and
     * thus shadow stack support is baked in the kernel.
     */
    while test_count < RISCV_SHADOW_STACK_TESTS {
        let test = unsafe { &SHSTK_TESTS[test_count as usize] };
        test_pass = unsafe { (test.t_func.unwrap())(test_count, core::ptr::null_mut()) };
        unsafe {
            ksft_test_result(test_pass, test.name);
        }
        test_count += 1;
    }

    unsafe {
        ksft_finished();
    }

    0
}

// C source used:
// #pragma GCC pop_options

#[inline]
fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

#[inline]
fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

#[inline]
fn wifsignaled(status: c_int) -> bool {
    ((status & 0x7f) + 1) >> 1 > 0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
