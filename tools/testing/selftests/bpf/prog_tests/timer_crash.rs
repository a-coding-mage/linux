// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, "timer_crash.skel.h"

const MODE_ARRAY: i32 = 0;
const MODE_HASH: i32 = 1;

#[repr(C)]
pub struct timer_crash_bss {
    pub pid: i32,
    pub crash_map: i32,
}

#[repr(C)]
pub struct timer_crash {
    pub bss: *mut timer_crash_bss,
}

const EOPNOTSUPP: i32 = 95;

unsafe extern "C" {
    static mut errno: i32;

    fn timer_crash__open_and_load() -> *mut timer_crash;
    fn timer_crash__attach(skel: *mut timer_crash) -> i32;
    fn timer_crash__destroy(skel: *mut timer_crash);

    fn test__skip();
    fn test__start_subtest(name: *const ::std::os::raw::c_char) -> bool;
    fn getpid() -> i32;
    fn usleep(usec: u32) -> i32;

    fn ASSERT_OK_PTR(ptr: *mut timer_crash, name: *const ::std::os::raw::c_char) -> bool;
    fn ASSERT_OK(err: i32, name: *const ::std::os::raw::c_char) -> bool;
}

unsafe fn test_timer_crash_mode(mode: i32) {
    let skel: *mut timer_crash;

    skel = timer_crash__open_and_load();
    if skel.is_null() && errno == EOPNOTSUPP {
        test__skip();
        return;
    }
    if !ASSERT_OK_PTR(skel, c"timer_crash__open_and_load".as_ptr()) {
        return;
    }
    (*(*skel).bss).pid = getpid();
    (*(*skel).bss).crash_map = mode;
    if !ASSERT_OK(timer_crash__attach(skel), c"timer_crash__attach".as_ptr()) {
        timer_crash__destroy(skel);
        return;
    }
    usleep(1);
    timer_crash__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_timer_crash() {
    if test__start_subtest(c"array".as_ptr()) {
        test_timer_crash_mode(MODE_ARRAY);
    }
    if test__start_subtest(c"hash".as_ptr()) {
        test_timer_crash_mode(MODE_HASH);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
