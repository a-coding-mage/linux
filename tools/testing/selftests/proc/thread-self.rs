/*
 * Copyright © 2018 Alexey Dobriyan <adobriyan@gmail.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
// Test that /proc/thread-self gives correct TGID/PID.
// C source included assert.h, sched.h, stdio.h, unistd.h, sys/mman.h,
// sys/wait.h, and "proc.h".

use core::ffi::{c_char, c_int, c_long, c_void};

type pid_t = c_int;
type ssize_t = isize;
type size_t = usize;

const _SC_PAGESIZE: c_int = 30;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const CLONE_VM: c_int = 0x00000100;
const CLONE_SIGHAND: c_int = 0x00000800;
const CLONE_THREAD: c_int = 0x00010000;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

unsafe extern "C" {
    fn sys_getpid() -> pid_t;
    fn sys_gettid() -> pid_t;
    fn streq(s1: *const c_char, s2: *const c_char) -> bool;

    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t;
    fn exit(status: c_int) -> !;
    fn sysconf(name: c_int) -> c_long;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn clone(
        fn_: unsafe extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
        ...
    ) -> c_int;
    fn pause() -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn f(arg: *mut c_void) -> c_int {
    let mut buf1: [c_char; 64] = [0; 64];
    let mut buf2: [c_char; 64] = [0; 64];
    let pid: pid_t;
    let tid: pid_t;
    let rv: ssize_t;

    pid = unsafe { sys_getpid() };
    tid = unsafe { sys_gettid() };
    unsafe {
        snprintf(
            buf1.as_mut_ptr(),
            buf1.len(),
            c"%u/task/%u".as_ptr(),
            pid,
            tid,
        );
    }

    rv = unsafe {
        readlink(
            c"/proc/thread-self".as_ptr(),
            buf2.as_mut_ptr(),
            buf2.len(),
        )
    };
    assert!(rv == unsafe { strlen(buf1.as_ptr()) } as ssize_t);
    buf2[rv as usize] = b'\0' as c_char;
    assert!(unsafe { streq(buf1.as_ptr(), buf2.as_ptr()) });

    if !arg.is_null() {
        unsafe { exit(0) };
    }
    0
}

fn main() {
    let PAGE_SIZE: c_int = unsafe { sysconf(_SC_PAGESIZE) } as c_int;
    let pid: pid_t;
    let stack: *mut c_void;

    /* main thread */
    unsafe {
        f(core::ptr::null_mut());
    }

    stack = unsafe {
        mmap(
            core::ptr::null_mut(),
            (2 * PAGE_SIZE) as size_t,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        )
    };
    assert!(stack != MAP_FAILED);
    /* side thread */
    pid = unsafe {
        clone(
            f,
            (stack as *mut u8).add(PAGE_SIZE as usize) as *mut c_void,
            CLONE_THREAD | CLONE_SIGHAND | CLONE_VM,
            1usize as *mut c_void,
        )
    };
    assert!(pid > 0);
    unsafe {
        pause();
    }
}
