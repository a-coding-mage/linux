// C dependencies from proc.h:
// assert.h, dirent.h, errno.h, stdbool.h, stdlib.h, string.h, unistd.h,
// sys/syscall.h.

pub type pid_t = i32;

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: i32;

    fn syscall(number: isize, ...) -> isize;
    fn strcmp(s1: *const i8, s2: *const i8) -> i32;
    fn strtoull(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> u64;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
}

// SYS_getpid and SYS_gettid are supplied by <sys/syscall.h> in the original C
// environment and are expected as translation-unit dependencies here.

unsafe fn c_assert(cond: bool) {
    assert!(cond);
}

pub unsafe fn sys_getpid() -> pid_t {
    unsafe { syscall(SYS_getpid) as pid_t }
}

pub unsafe fn sys_gettid() -> pid_t {
    unsafe { syscall(SYS_gettid) as pid_t }
}

pub unsafe fn streq(s1: *const i8, s2: *const i8) -> bool {
    unsafe { strcmp(s1, s2) == 0 }
}

pub unsafe fn xstrtoull(p: *const i8, end: *mut *mut i8) -> u64 {
    unsafe {
        if *p == b'0' as i8 {
            *end = p.add(1) as *mut i8;
            0
        } else if b'1' as i8 <= *p && *p <= b'9' as i8 {
            let val: u64;

            errno = 0;
            val = strtoull(p, end, 10);
            c_assert(errno == 0);
            val
        } else {
            c_assert(false);
            core::hint::unreachable_unchecked()
        }
    }
}

pub unsafe fn xreaddir(d: *mut DIR) -> *mut dirent {
    unsafe {
        let de: *mut dirent;

        errno = 0;
        de = readdir(d);
        c_assert(!de.is_null() || errno == 0);
        de
    }
}
