// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta */
// C dependencies: <test_progs.h>, "clone_attach_btf_id.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u32 = c_uint;

#[repr(C)]
pub struct bpf_prog_info {
    pub attach_btf_id: __u32,
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub attach_btf_id: c_int,
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clone_attach_btf_id_progs {
    pub fentry_handler: *mut bpf_program,
}

#[repr(C)]
pub struct clone_attach_btf_id {
    pub obj: *mut bpf_object,
    pub progs: clone_attach_btf_id_progs,
}

extern "C" {
    static BPF_TRACE_FENTRY: c_int;

    fn bpf_prog_get_info_by_fd(
        prog_fd: c_int,
        info: *mut bpf_prog_info,
        info_len: *mut __u32,
    ) -> c_int;
    fn libbpf_find_vmlinux_btf_id(name: *const c_char, attach_type: c_int) -> c_int;
    fn clone_attach_btf_id__open() -> *mut clone_attach_btf_id;
    fn bpf_object__prepare(obj: *mut bpf_object) -> c_int;
    fn bpf_program__clone(prog: *mut bpf_program, opts: *const bpf_prog_load_opts) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn clone_attach_btf_id__destroy(skel: *mut clone_attach_btf_id);

    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

unsafe fn get_prog_attach_btf_id(prog_fd: c_int) -> c_int {
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut info_len: __u32 = core::mem::size_of::<bpf_prog_info>() as __u32;
    let err: c_int;

    err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len);
    if err != 0 {
        return err;
    }
    info.attach_btf_id as c_int
}

#[no_mangle]
pub unsafe extern "C" fn test_clone_attach_btf_id() {
    let skel: *mut clone_attach_btf_id;
    let mut fd1: c_int = -1;
    let mut fd2: c_int = -1;
    let mut err: c_int;
    let btf_id_test1: c_int;
    let btf_id_test2: c_int;

    btf_id_test1 = libbpf_find_vmlinux_btf_id(
        b"bpf_fentry_test1\0".as_ptr() as *const c_char,
        BPF_TRACE_FENTRY,
    );
    if !ASSERT_GT(btf_id_test1, 0, b"find_btf_id_test1\0".as_ptr() as *const c_char) {
        return;
    }

    btf_id_test2 = libbpf_find_vmlinux_btf_id(
        b"bpf_fentry_test2\0".as_ptr() as *const c_char,
        BPF_TRACE_FENTRY,
    );
    if !ASSERT_GT(btf_id_test2, 0, b"find_btf_id_test2\0".as_ptr() as *const c_char) {
        return;
    }

    skel = clone_attach_btf_id__open();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open\0".as_ptr() as *const c_char) {
        return;
    }

    err = bpf_object__prepare((*skel).obj);
    if !ASSERT_OK(err, b"obj_prepare\0".as_ptr() as *const c_char) {
        goto_out(skel, fd1, fd2);
        return;
    }

    /* Clone with no opts - callback resolves BTF from sec_name */
    fd1 = bpf_program__clone((*skel).progs.fentry_handler, core::ptr::null());
    if !ASSERT_GE(fd1, 0, b"clone_default\0".as_ptr() as *const c_char) {
        goto_out(skel, fd1, fd2);
        return;
    }
    ASSERT_EQ(
        get_prog_attach_btf_id(fd1),
        btf_id_test1,
        b"attach_btf_id_default\0".as_ptr() as *const c_char,
    );

    /*
     * Clone with attach_btf_id override pointing to a different
     * function. The BPF program never accesses arguments, so the
     * load succeeds regardless of signature mismatch.
     */
    let opts = bpf_prog_load_opts {
        attach_btf_id: btf_id_test2,
    };
    fd2 = bpf_program__clone((*skel).progs.fentry_handler, &opts);
    if !ASSERT_GE(fd2, 0, b"clone_override\0".as_ptr() as *const c_char) {
        goto_out(skel, fd1, fd2);
        return;
    }
    ASSERT_EQ(
        get_prog_attach_btf_id(fd2),
        btf_id_test2,
        b"attach_btf_id_override\0".as_ptr() as *const c_char,
    );

    goto_out(skel, fd1, fd2);
}

unsafe fn goto_out(skel: *mut clone_attach_btf_id, fd1: c_int, fd2: c_int) {
    if fd1 >= 0 {
        close(fd1);
    }
    if fd2 >= 0 {
        close(fd2);
    }
    clone_attach_btf_id__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
