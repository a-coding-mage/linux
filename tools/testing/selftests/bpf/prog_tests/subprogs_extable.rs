// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <test_progs.h>
// #include "test_subprogs_extable.skel.h"

#[repr(C)]
pub struct test_subprogs_extable_bss {
    pub triggered: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct test_subprogs_extable {
    pub bss: *mut test_subprogs_extable_bss,
}

unsafe extern "C" {
    fn test_subprogs_extable__open_and_load() -> *mut test_subprogs_extable;
    fn test_subprogs_extable__attach(skel: *mut test_subprogs_extable) -> ::std::os::raw::c_int;
    fn test_subprogs_extable__detach(skel: *mut test_subprogs_extable);
    fn test_subprogs_extable__destroy(skel: *mut test_subprogs_extable);

    fn trigger_module_test_read(read_sz: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    fn ASSERT_OK_PTR(ptr: *mut test_subprogs_extable, name: *const ::std::os::raw::c_char) -> bool;
    fn ASSERT_OK(err: ::std::os::raw::c_int, name: *const ::std::os::raw::c_char) -> bool;
    fn ASSERT_NEQ(
        actual: ::std::os::raw::c_int,
        expected: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    ) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn test_subprogs_extable() {
    let read_sz: ::std::os::raw::c_int = 456;
    let skel: *mut test_subprogs_extable;
    let mut err: ::std::os::raw::c_int;

    skel = unsafe { test_subprogs_extable__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel, c"skel_open_and_load".as_ptr()) } {
        return;
    }

    err = unsafe { test_subprogs_extable__attach(skel) };
    if !unsafe { ASSERT_OK(err, c"skel_attach".as_ptr()) } {
        unsafe { test_subprogs_extable__destroy(skel) };
        return;
    }

    /* trigger tracepoint */
    unsafe {
        ASSERT_OK(
            trigger_module_test_read(read_sz),
            c"trigger_read".as_ptr(),
        );
    }

    unsafe {
        ASSERT_NEQ(
            (*(*skel).bss).triggered,
            0,
            c"verify at least one program ran".as_ptr(),
        );
    }

    unsafe { test_subprogs_extable__detach(skel) };

    unsafe { test_subprogs_extable__destroy(skel) };
}
