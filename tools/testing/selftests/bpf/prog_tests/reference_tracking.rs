// SPDX-License-Identifier: GPL-2.0
// Translated from C source using external declarations for test_progs/libbpf APIs.

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

type __u32 = u32;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

pub type libbpf_print_fn_t = Option<
    unsafe extern "C" fn(
        level: c_int,
        format: *const c_char,
        args: *mut c_void,
    ) -> c_int,
>;

#[repr(C)]
pub struct bpf_object_open_opts {
    pub sz: usize,
    pub object_name: *const c_char,
    pub relaxed_maps: bool,
}

extern "C" {
    fn bpf_object__open_file(
        path: *const c_char,
        opts: *const bpf_object_open_opts,
    ) -> *mut bpf_object;
    fn bpf_object__name(obj: *const bpf_object) -> *const c_char;
    fn bpf_object__next_program(
        obj: *const bpf_object,
        prog: *mut bpf_program,
    ) -> *mut bpf_program;
    fn bpf_object__find_program_by_name(
        obj: *const bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_program__name(prog: *const bpf_program) -> *const c_char;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn libbpf_set_print(fn_: libbpf_print_fn_t) -> libbpf_print_fn_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;

    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn CHECK(
        condition: c_int,
        tag: *const c_char,
        format: *const c_char,
        ...
    ) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
}

pub unsafe fn test_reference_tracking() {
    let file = b"test_sk_lookup_kern.bpf.o\0".as_ptr() as *const c_char;
    let obj_name = b"ref_track\0".as_ptr() as *const c_char;
    let open_opts = bpf_object_open_opts {
        sz: core::mem::size_of::<bpf_object_open_opts>(),
        object_name: obj_name,
        relaxed_maps: true,
    };
    let mut obj_iter: *mut bpf_object;
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut prog: *mut bpf_program;
    let _duration: __u32 = 0;
    let mut err: c_int = 0;

    obj_iter = bpf_object__open_file(file, &open_opts);
    if !ASSERT_OK_PTR(
        obj_iter as *mut c_void,
        b"obj_iter_open_file\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    if CHECK(
        strcmp(bpf_object__name(obj_iter), obj_name),
        b"obj_name\0".as_ptr() as *const c_char,
        b"wrong obj name '%s', expected '%s'\n\0".as_ptr() as *const c_char,
        bpf_object__name(obj_iter),
        obj_name,
    ) {
        bpf_object__close(obj);
        bpf_object__close(obj_iter);
        return;
    }

    // C source uses bpf_object__for_each_program(prog, obj_iter).
    prog = bpf_object__next_program(obj_iter, ptr::null_mut());
    while !prog.is_null() {
        let mut p: *mut bpf_program;
        let name: *const c_char;

        name = bpf_program__name(prog);
        if !test__start_subtest(name) {
            prog = bpf_object__next_program(obj_iter, prog);
            continue;
        }

        obj = bpf_object__open_file(file, &open_opts);
        if !ASSERT_OK_PTR(obj as *mut c_void, b"obj_open_file\0".as_ptr() as *const c_char) {
            bpf_object__close(obj);
            bpf_object__close(obj_iter);
            return;
        }

        /* all programs are not loaded by default, so just set
         * autoload to true for the single prog under test
         */
        p = bpf_object__find_program_by_name(obj, name);
        bpf_program__set_autoload(p, true);

        /* Expect verifier failure if test name has 'err' */
        if strncmp(name, b"err_\0".as_ptr() as *const c_char, core::mem::size_of_val(b"err_") - 1)
            == 0
        {
            let old_print_fn: libbpf_print_fn_t;

            old_print_fn = libbpf_set_print(None);
            err = (bpf_object__load(obj) == 0) as c_int;
            libbpf_set_print(old_print_fn);
        } else {
            err = bpf_object__load(obj);
        }
        ASSERT_OK(err, name);

        bpf_object__close(obj);
        obj = ptr::null_mut();

        prog = bpf_object__next_program(obj_iter, prog);
    }

    bpf_object__close(obj);
    bpf_object__close(obj_iter);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
