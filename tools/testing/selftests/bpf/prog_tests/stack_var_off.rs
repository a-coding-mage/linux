// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, "test_stack_var_off.skel.h"

/* Test read and writes to the stack performed with offsets that are not
 * statically known.
 */

#[repr(C)]
pub struct test_stack_var_off__bss {
    pub test_pid: libc::pid_t,
    pub input: [i32; 2],
    pub probe_res: i32,
}

#[repr(C)]
pub struct test_stack_var_off {
    pub bss: *mut test_stack_var_off__bss,
}

unsafe extern "C" {
    fn test_stack_var_off__open_and_load() -> *mut test_stack_var_off;
    fn test_stack_var_off__attach(skel: *mut test_stack_var_off) -> libc::c_int;
    fn test_stack_var_off__destroy(skel: *mut test_stack_var_off);

    fn CHECK(
        condition: bool,
        tag: *const libc::c_char,
        format: *const libc::c_char,
        ...
    ) -> bool;
    fn ASSERT_OK(ret: libc::c_int, name: *const libc::c_char) -> bool;
}

pub unsafe extern "C" fn test_stack_var_off() {
    let duration: libc::c_int = 0;
    let skel: *mut test_stack_var_off;

    skel = unsafe { test_stack_var_off__open_and_load() };
    if unsafe {
        CHECK(
            skel.is_null(),
            c"skel_open".as_ptr(),
            c"failed to open skeleton\n".as_ptr(),
        )
    } {
        return;
    }

    /* Give pid to bpf prog so it doesn't trigger for anyone else. */
    unsafe {
        (*(*skel).bss).test_pid = libc::getpid();
    }
    /* Initialize the probe's input. */
    unsafe {
        (*(*skel).bss).input[0] = 2;
        (*(*skel).bss).input[1] = 42; /* This will be returned in probe_res. */
    }

    if !unsafe { ASSERT_OK(test_stack_var_off__attach(skel), c"skel_attach".as_ptr()) } {
        goto_cleanup(skel);
        return;
    }

    /* Trigger probe. */
    unsafe {
        libc::usleep(1);
    }

    if unsafe {
        CHECK(
            (*(*skel).bss).probe_res != 42,
            c"check_probe_res".as_ptr(),
            c"wrong probe res: %d\n".as_ptr(),
            (*(*skel).bss).probe_res,
        )
    } {
        goto_cleanup(skel);
        return;
    }

    goto_cleanup(skel);

    unsafe extern "C" fn goto_cleanup(skel: *mut test_stack_var_off) {
        unsafe {
            test_stack_var_off__destroy(skel);
        }
    }

    let _ = duration;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
