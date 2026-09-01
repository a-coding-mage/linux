// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C file:
// "bpf/libbpf.h", <test_progs.h>, <network_helpers.h>, and "cb_refs.skel.h".

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bpf_object_open_opts {
    pub kernel_log_buf: *mut c_char,
    pub kernel_log_size: usize,
    pub kernel_log_level: c_int,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *const c_void,
    pub data_size_in: usize,
    pub repeat: c_int,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cb_refs {
    pub obj: *mut bpf_object,
}

#[repr(C)]
struct cb_refs_test {
    prog_name: *const c_char,
    err_msg: *const c_char,
}

unsafe extern "C" {
    static pkt_v4: c_void;

    fn cb_refs__open_opts(opts: *const bpf_object_open_opts) -> *mut cb_refs;
    fn cb_refs__load(skel: *mut cb_refs) -> c_int;
    fn cb_refs__destroy(skel: *mut cb_refs);

    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;

    static mut stderr: *mut c_void;
}

static mut LOG_BUF: [c_char; 1024 * 1024] = [0; 1024 * 1024];

static CB_REFS_TESTS: [cb_refs_test; 4] = [
    cb_refs_test {
        prog_name: c"underflow_prog".as_ptr(),
        err_msg: c"release kfunc bpf_kfunc_call_test_release expects referenced PTR_TO_BTF_ID passed to R1".as_ptr(),
    },
    cb_refs_test {
        prog_name: c"leak_prog".as_ptr(),
        err_msg: c"Possibly NULL pointer passed to helper R2".as_ptr(),
    },
    cb_refs_test {
        prog_name: c"nested_cb".as_ptr(),
        err_msg: c"Unreleased reference id=4 alloc_insn=2".as_ptr(),
    }, /* alloc_insn=2{4,5} */
    cb_refs_test {
        prog_name: c"non_cb_transfer_ref".as_ptr(),
        err_msg: c"Unreleased reference id=4 alloc_insn=1".as_ptr(),
    }, /* alloc_insn=1{1,2} */
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cb_refs() {
    let opts = bpf_object_open_opts {
        kernel_log_buf: core::ptr::addr_of_mut!(LOG_BUF).cast::<c_char>(),
        kernel_log_size: core::mem::size_of_val(&*core::ptr::addr_of!(LOG_BUF)),
        kernel_log_level: 1,
    };
    let mut prog: *mut bpf_program;
    let mut skel: *mut cb_refs;
    let mut i: c_int;

    i = 0;
    while (i as usize) < CB_REFS_TESTS.len() {
        let mut run_opts = bpf_test_run_opts {
            data_in: core::ptr::addr_of!(pkt_v4).cast::<c_void>(),
            data_size_in: core::mem::size_of_val(&pkt_v4),
            repeat: 1,
        };

        skel = cb_refs__open_opts(&opts);
        if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"cb_refs__open_and_load".as_ptr()) {
            return;
        }

        prog = bpf_object__find_program_by_name(
            (*skel).obj,
            CB_REFS_TESTS[i as usize].prog_name,
        );
        bpf_program__set_autoload(prog, true);
        if !ASSERT_ERR(cb_refs__load(skel), c"cb_refs__load".as_ptr()) {
            bpf_prog_test_run_opts(bpf_program__fd(prog), &mut run_opts);
        }

        if !ASSERT_OK_PTR(
            strstr(
                core::ptr::addr_of!(LOG_BUF).cast::<c_char>(),
                CB_REFS_TESTS[i as usize].err_msg,
            )
            .cast::<c_void>(),
            c"expected error message".as_ptr(),
        ) {
            fprintf(
                stderr,
                c"Expected: %s\n".as_ptr(),
                CB_REFS_TESTS[i as usize].err_msg,
            );
            fprintf(
                stderr,
                c"Verifier: %s\n".as_ptr(),
                core::ptr::addr_of!(LOG_BUF).cast::<c_char>(),
            );
        }

        cb_refs__destroy(skel);
        i += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
