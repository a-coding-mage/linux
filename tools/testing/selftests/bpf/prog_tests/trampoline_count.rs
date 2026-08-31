// SPDX-License-Identifier: GPL-2.0-only
// C dependency intent: #define _GNU_SOURCE and #include <test_progs.h>

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const E2BIG: c_int = 7;

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct inst {
    obj: *mut bpf_object,
    link: *mut bpf_link,
}

unsafe extern "C" {
    fn bpf_object__open_file(file: *mut c_char, opts: *const c_void) -> *mut bpf_object;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *mut c_char,
    ) -> *mut bpf_program;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);
    fn libbpf_get_error(ptr: *const c_void) -> c_int;
    fn get_bpf_max_tramp_links() -> c_int;
    fn trigger_module_test_read(sz: c_int) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
}

const fn cstr(bytes: &'static [u8]) -> *mut c_char {
    bytes.as_ptr() as *mut c_char
}

unsafe fn load_prog(file: *mut c_char, name: *mut c_char, inst: *mut inst) -> *mut bpf_program {
    let obj: *mut bpf_object;
    let prog: *mut bpf_program;
    let err: c_int;

    obj = bpf_object__open_file(file, ptr::null());
    if !ASSERT_OK_PTR(obj.cast::<c_void>(), cstr(b"obj_open_file\0")) {
        return ptr::null_mut();
    }

    (*inst).obj = obj;

    err = bpf_object__load(obj);
    if !ASSERT_OK(err, cstr(b"obj_load\0")) {
        return ptr::null_mut();
    }

    prog = bpf_object__find_program_by_name(obj, name);
    if !ASSERT_OK_PTR(prog.cast::<c_void>(), cstr(b"obj_find_prog\0")) {
        return ptr::null_mut();
    }

    prog
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_trampoline_count() {
    let file: *mut c_char = cstr(b"test_trampoline_count.bpf.o\0");
    let progs: [*mut c_char; 3] = [
        cstr(b"fentry_test\0"),
        cstr(b"fmod_ret_test\0"),
        cstr(b"fexit_test\0"),
    ];
    let mut bpf_max_tramp_links: c_int;
    let mut i: c_int;
    let mut prog: *mut bpf_program;
    let mut link: *mut bpf_link;
    let inst: *mut inst;

    bpf_max_tramp_links = get_bpf_max_tramp_links();
    if !ASSERT_GE(
        bpf_max_tramp_links,
        1,
        cstr(b"bpf_max_tramp_links\0"),
    ) {
        return;
    }
    inst = calloc(
        (bpf_max_tramp_links + 1) as usize,
        core::mem::size_of::<inst>(),
    )
    .cast::<inst>();
    if !ASSERT_OK_PTR(inst.cast::<c_void>(), cstr(b"inst\0")) {
        return;
    }

    /* attach 'allowed' trampoline programs */
    i = 0;
    while i < bpf_max_tramp_links {
        prog = load_prog(
            file,
            progs[(i as usize) % progs.len()],
            inst.offset(i as isize),
        );
        if prog.is_null() {
            goto_cleanup(i, inst);
            return;
        }

        link = bpf_program__attach(prog);
        if !ASSERT_OK_PTR(link.cast::<c_void>(), cstr(b"attach_prog\0")) {
            goto_cleanup(i, inst);
            return;
        }

        (*inst.offset(i as isize)).link = link;
        i += 1;
    }

    /* and try 1 extra.. */
    prog = load_prog(file, cstr(b"fmod_ret_test\0"), inst.offset(i as isize));
    if prog.is_null() {
        goto_cleanup(i, inst);
        return;
    }

    /* ..that needs to fail */
    link = bpf_program__attach(prog);
    if !ASSERT_ERR_PTR(link.cast::<c_void>(), cstr(b"attach_prog\0")) {
        (*inst.offset(i as isize)).link = link;
        goto_cleanup(i, inst);
        return;
    }

    /* with E2BIG error */
    if !ASSERT_EQ(
        libbpf_get_error(link.cast::<c_void>()),
        -E2BIG,
        cstr(b"E2BIG\0"),
    ) {
        goto_cleanup(i, inst);
        return;
    }
    if !ASSERT_EQ(link, ptr::null_mut::<bpf_link>(), cstr(b"ptr_is_null\0")) {
        goto_cleanup(i, inst);
        return;
    }

    /* and finally execute the probe */
    ASSERT_OK(
        trigger_module_test_read(256),
        cstr(b"trigger_module_test_read\0"),
    );

    goto_cleanup(i, inst);
}

unsafe fn goto_cleanup(mut i: c_int, inst: *mut inst) {
    while i >= 0 {
        bpf_link__destroy((*inst.offset(i as isize)).link);
        bpf_object__close((*inst.offset(i as isize)).obj);
        i -= 1;
    }
    free(inst.cast::<c_void>());
}
