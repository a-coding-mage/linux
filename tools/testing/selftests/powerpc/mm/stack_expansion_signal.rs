// SPDX-License-Identifier: GPL-2.0
/*
 * Test that signal delivery is able to expand the stack segment without
 * triggering a SEGV.
 *
 * Based on test code by Tom Lane.
 */

// C dependencies: <err.h>, <stdio.h>, <stdlib.h>, <signal.h>,
// <sys/types.h>, <unistd.h>, "../pmu/lib.h", and "utils.h".

const _KB: u32 = 1024;
const _MB: u32 = 1024 * 1024;

static mut stack_base_ptr: *mut libc::c_char = core::ptr::null_mut();
static mut stack_top_ptr: *mut libc::c_char = core::ptr::null_mut();

static mut sig_occurred: libc::sig_atomic_t = 0;

extern "C" {
    fn notify_parent(write_pipe: r#pipe) -> libc::c_int;
    fn sync_with_child(read_pipe: r#pipe, write_pipe: r#pipe) -> libc::c_int;
    fn wait_for_child(pid: libc::pid_t) -> libc::c_int;
    fn test_harness(test: unsafe extern "C" fn() -> libc::c_int, name: *const libc::c_char) -> libc::c_int;
    fn barrier();
}

#[repr(C)]
#[derive(Copy, Clone)]
union r#pipe {
    fds: [libc::c_int; 2],
    read_fd: libc::c_int,
    write_fd: libc::c_int,
}

macro_rules! FAIL_IF {
    ($cond:expr) => {{
        if $cond != 0 {
            return 1;
        }
    }};
}

unsafe extern "C" fn sigusr1_handler(signal_arg: libc::c_int) {
    let _ = signal_arg;
    sig_occurred = 1;
}

unsafe fn consume_stack(stack_size: libc::c_uint, write_pipe: r#pipe) -> libc::c_int {
    let stack_cur: libc::c_char = 0;

    if stack_base_ptr.offset_from(&stack_cur as *const libc::c_char as *mut libc::c_char)
        < stack_size as isize
    {
        return consume_stack(stack_size, write_pipe);
    } else {
        stack_top_ptr = &stack_cur as *const libc::c_char as *mut libc::c_char;

        FAIL_IF!(notify_parent(write_pipe));

        while sig_occurred == 0 {
            barrier();
        }
    }

    0
}

unsafe fn child(stack_size: libc::c_uint, write_pipe: r#pipe) -> libc::c_int {
    let mut act: libc::sigaction = core::mem::zeroed();
    let stack_base: libc::c_char = 0;

    act.sa_sigaction = sigusr1_handler as usize;
    libc::sigemptyset(&mut act.sa_mask);
    act.sa_flags = 0;
    if libc::sigaction(libc::SIGUSR1, &act, core::ptr::null_mut()) < 0 {
        libc::err(1, b"sigaction\0".as_ptr() as *const libc::c_char);
    }

    stack_base_ptr =
        (((&stack_base as *const libc::c_char as usize) + 65535) & !(65535usize)) as *mut libc::c_char;

    FAIL_IF!(consume_stack(stack_size, write_pipe));

    libc::printf(
        b"size 0x%06x: OK, stack base %p top %p (%zx used)\n\0".as_ptr()
            as *const libc::c_char,
        stack_size,
        stack_base_ptr,
        stack_top_ptr,
        stack_base_ptr.offset_from(stack_top_ptr) as usize,
    );

    0
}

unsafe fn test_one_size(stack_size: libc::c_uint) -> libc::c_int {
    let mut read_pipe: r#pipe = core::mem::zeroed();
    let mut write_pipe: r#pipe = core::mem::zeroed();
    let pid: libc::pid_t;

    FAIL_IF!((libc::pipe(read_pipe.fds.as_mut_ptr()) == -1) as libc::c_int);
    FAIL_IF!((libc::pipe(write_pipe.fds.as_mut_ptr()) == -1) as libc::c_int);

    pid = libc::fork();
    if pid == 0 {
        libc::close(read_pipe.read_fd);
        libc::close(write_pipe.write_fd);
        libc::exit(child(stack_size, read_pipe));
    }

    libc::close(read_pipe.write_fd);
    libc::close(write_pipe.read_fd);
    FAIL_IF!(sync_with_child(read_pipe, write_pipe));

    libc::kill(pid, libc::SIGUSR1);

    FAIL_IF!(wait_for_child(pid));

    libc::close(read_pipe.read_fd);
    libc::close(write_pipe.write_fd);

    0
}

#[no_mangle]
pub unsafe extern "C" fn test() -> libc::c_int {
    let mut i: libc::c_uint;
    let mut size: libc::c_uint;

    // Test with used stack from 1MB - 64K to 1MB + 64K
    // Increment by 64 to get more coverage of odd sizes
    i = 0;
    while i < 128 * _KB {
        size = i + 1 * _MB - 64 * _KB;
        FAIL_IF!(test_one_size(size));
        i += 64;
    }

    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            test,
            b"stack_expansion_signal\0".as_ptr() as *const libc::c_char,
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
