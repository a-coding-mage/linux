// SPDX-License-Identifier: GPL-2.0
// Translated from get_func_ip_test.c. External libbpf/test skeleton symbols are
// declared here and supplied by the surrounding repository.

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: u32,
}

#[repr(C)]
pub struct bpf_kprobe_opts {
    pub offset: usize,
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct get_func_ip_test_bss {
    pub uprobe_trigger: c_ulong,
    pub test1_result: c_int,
    pub test2_result: c_int,
    pub test3_result: c_int,
    pub test4_result: c_int,
    pub test5_result: c_int,
    pub test6_result: c_int,
    pub test7_result: c_int,
    pub test8_result: c_int,
}

#[repr(C)]
pub struct get_func_ip_test_progs {
    pub test1: *mut bpf_program,
    pub test5: *mut bpf_program,
    #[cfg(target_arch = "x86_64")]
    pub test6: *mut bpf_program,
}

#[repr(C)]
pub struct get_func_ip_test_kconfig {
    pub CONFIG_X86_KERNEL_IBT: bool,
}

#[repr(C)]
pub struct get_func_ip_test {
    pub bss: *mut get_func_ip_test_bss,
    pub progs: get_func_ip_test_progs,
    #[cfg(target_arch = "x86_64")]
    pub kconfig: *mut get_func_ip_test_kconfig,
}

#[repr(C)]
pub struct get_func_ip_uprobe_test_bss {
    pub uprobe_trigger_body: c_ulong,
    pub test1_result: c_int,
}

#[repr(C)]
pub struct get_func_ip_uprobe_test {
    pub bss: *mut get_func_ip_uprobe_test_bss,
}

#[repr(C)]
pub struct get_func_ip_fsession_test_bss {
    pub test1_entry_result: c_int,
    pub test1_exit_result: c_int,
}

#[repr(C)]
pub struct get_func_ip_fsession_test_progs {
    pub test1: *mut bpf_program,
}

#[repr(C)]
pub struct get_func_ip_fsession_test {
    pub bss: *mut get_func_ip_fsession_test_bss,
    pub progs: get_func_ip_fsession_test_progs,
}

unsafe extern "C" {
    fn get_func_ip_test__open() -> *mut get_func_ip_test;
    fn get_func_ip_test__load(skel: *mut get_func_ip_test) -> c_int;
    fn get_func_ip_test__attach(skel: *mut get_func_ip_test) -> c_int;
    fn get_func_ip_test__destroy(skel: *mut get_func_ip_test);

    fn get_func_ip_uprobe_test__open_and_load() -> *mut get_func_ip_uprobe_test;
    fn get_func_ip_uprobe_test__attach(skel: *mut get_func_ip_uprobe_test) -> c_int;
    fn get_func_ip_uprobe_test__destroy(skel: *mut get_func_ip_uprobe_test);

    fn get_func_ip_fsession_test__open_and_load() -> *mut get_func_ip_fsession_test;
    fn get_func_ip_fsession_test__attach(skel: *mut get_func_ip_fsession_test) -> c_int;
    fn get_func_ip_fsession_test__destroy(skel: *mut get_func_ip_fsession_test);

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, topts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__attach_kprobe_opts(
        prog: *mut bpf_program,
        func_name: *const c_char,
        opts: *mut bpf_kprobe_opts,
    ) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ<T: PartialEq + Copy>(actual: T, expected: T, name: *const c_char) -> bool;
}

#[inline(never)]
fn uprobe_trigger() {}

unsafe fn test_function_entry() {
    let mut skel: *mut get_func_ip_test = core::ptr::null_mut();
    let mut err: c_int;
    let mut prog_fd: c_int;
    let mut topts: bpf_test_run_opts = core::mem::zeroed();

    skel = get_func_ip_test__open();
    if !ASSERT_OK_PTR(skel, c"get_func_ip_test__open".as_ptr()) {
        return;
    }

    err = get_func_ip_test__load(skel);
    if !ASSERT_OK(err, c"get_func_ip_test__load".as_ptr()) {
        get_func_ip_test__destroy(skel);
        return;
    }

    err = get_func_ip_test__attach(skel);
    if !ASSERT_OK(err, c"get_func_ip_test__attach".as_ptr()) {
        get_func_ip_test__destroy(skel);
        return;
    }

    (*(*skel).bss).uprobe_trigger = uprobe_trigger as usize as c_ulong;

    prog_fd = bpf_program__fd((*skel).progs.test1);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval, 0u32, c"test_run".as_ptr());

    prog_fd = bpf_program__fd((*skel).progs.test5);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);

    ASSERT_OK(err, c"test_run".as_ptr());

    uprobe_trigger();

    ASSERT_EQ((*(*skel).bss).test1_result, 1, c"test1_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).test2_result, 1, c"test2_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).test3_result, 1, c"test3_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).test4_result, 1, c"test4_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).test5_result, 1, c"test5_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).test7_result, 1, c"test7_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).test8_result, 1, c"test8_result".as_ptr());

    get_func_ip_test__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn uprobe_trigger_body();
}

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
    ".globl uprobe_trigger_body",
    ".type uprobe_trigger_body, @function",
    "uprobe_trigger_body:",
    "	nop",
    "	ret",
);

#[cfg(target_arch = "x86_64")]
unsafe fn test_function_body_kprobe() {
    let mut skel: *mut get_func_ip_test = core::ptr::null_mut();
    let mut topts: bpf_test_run_opts = core::mem::zeroed();
    let mut kopts: bpf_kprobe_opts = core::mem::zeroed();
    let mut link6: *mut bpf_link = core::ptr::null_mut();
    let mut err: c_int;
    let mut prog_fd: c_int;

    skel = get_func_ip_test__open();
    if !ASSERT_OK_PTR(skel, c"get_func_ip_test__open".as_ptr()) {
        return;
    }

    /* test6 is x86_64 specific and is disabled by default,
     * enable it for body test.
     */
    bpf_program__set_autoload((*skel).progs.test6, true);

    err = get_func_ip_test__load(skel);
    if !ASSERT_OK(err, c"get_func_ip_test__load".as_ptr()) {
        bpf_link__destroy(link6);
        get_func_ip_test__destroy(skel);
        return;
    }

    kopts.offset = if (*(*skel).kconfig).CONFIG_X86_KERNEL_IBT {
        9
    } else {
        5
    };

    link6 = bpf_program__attach_kprobe_opts(
        (*skel).progs.test6,
        c"bpf_fentry_test6".as_ptr(),
        &mut kopts,
    );
    if !ASSERT_OK_PTR(link6, c"link6".as_ptr()) {
        bpf_link__destroy(link6);
        get_func_ip_test__destroy(skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.test1);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval, 0u32, c"test_run".as_ptr());

    ASSERT_EQ((*(*skel).bss).test6_result, 1, c"test6_result".as_ptr());

    bpf_link__destroy(link6);
    get_func_ip_test__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_function_body_uprobe() {
    let mut skel: *mut get_func_ip_uprobe_test = core::ptr::null_mut();
    let mut err: c_int;

    skel = get_func_ip_uprobe_test__open_and_load();
    if !ASSERT_OK_PTR(skel, c"get_func_ip_uprobe_test__open_and_load".as_ptr()) {
        return;
    }

    err = get_func_ip_uprobe_test__attach(skel);
    if !ASSERT_OK(err, c"get_func_ip_test__attach".as_ptr()) {
        get_func_ip_uprobe_test__destroy(skel);
        return;
    }

    (*(*skel).bss).uprobe_trigger_body = uprobe_trigger_body as usize as c_ulong;

    uprobe_trigger_body();

    ASSERT_EQ((*(*skel).bss).test1_result, 1, c"test1_result".as_ptr());

    get_func_ip_uprobe_test__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_function_body() {
    test_function_body_kprobe();
    test_function_body_uprobe();
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn test_function_body() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_get_func_ip_test() {
    test_function_entry();
    test_function_body();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_get_func_ip_fsession_test() {
    let mut skel: *mut get_func_ip_fsession_test = core::ptr::null_mut();
    let mut err: c_int;
    let mut topts: bpf_test_run_opts = core::mem::zeroed();

    skel = get_func_ip_fsession_test__open_and_load();
    if !ASSERT_OK_PTR(skel, c"get_func_ip_fsession_test__open_and_load".as_ptr()) {
        return;
    }

    err = get_func_ip_fsession_test__attach(skel);
    if !ASSERT_OK(err, c"get_func_ip_fsession_test__attach".as_ptr()) {
        get_func_ip_fsession_test__destroy(skel);
        return;
    }

    err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.test1), &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval, 0u32, c"test_run".as_ptr());

    ASSERT_EQ(
        (*(*skel).bss).test1_entry_result,
        1,
        c"test1_entry_result".as_ptr(),
    );
    ASSERT_EQ(
        (*(*skel).bss).test1_exit_result,
        1,
        c"test1_exit_result".as_ptr(),
    );

    get_func_ip_fsession_test__destroy(skel);
}
