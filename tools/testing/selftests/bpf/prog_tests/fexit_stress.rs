// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */
/* Dependencies from C headers:
 * #include <test_progs.h>
 * #include "bpf_util.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct bpf_insn {
    pub code: u8,
    pub dst_reg_src_reg: u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub sz: usize,
    pub expected_attach_type: c_uint,
    pub attach_btf_id: c_uint,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
}

unsafe extern "C" {
    fn get_bpf_max_tramp_links() -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn libbpf_find_vmlinux_btf_id(name: *const c_char, attach_type: c_uint) -> c_int;
    fn bpf_prog_load(
        prog_type: c_uint,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: usize,
        opts: *const bpf_prog_load_opts,
    ) -> c_int;
    fn bpf_link_create(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: c_uint,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(actual: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

const BPF_REG_0: u8 = 0;
const BPF_PROG_TYPE_TRACING: c_uint = 26;
const BPF_TRACE_FEXIT: c_uint = 27;

const fn bpf_mov64_imm(dst: u8, imm: i32) -> bpf_insn {
    bpf_insn {
        code: 0xb7,
        dst_reg_src_reg: dst & 0x0f,
        off: 0,
        imm,
    }
}

const fn bpf_exit_insn() -> bpf_insn {
    bpf_insn {
        code: 0x95,
        dst_reg_src_reg: 0,
        off: 0,
        imm: 0,
    }
}

pub unsafe fn serial_test_fexit_stress() {
    let bpf_max_tramp_links: c_int;
    let mut err: c_int;
    let mut i: c_int;
    let fd: *mut c_int;
    let fexit_fd: *mut c_int;
    let link_fd: *mut c_int;

    bpf_max_tramp_links = unsafe { get_bpf_max_tramp_links() };
    if !unsafe {
        ASSERT_GE(
            bpf_max_tramp_links,
            1,
            c"bpf_max_tramp_links".as_ptr(),
        )
    } {
        return;
    }
    fd = unsafe {
        calloc(
            (bpf_max_tramp_links * 2) as usize,
            core::mem::size_of::<c_int>(),
        ) as *mut c_int
    };
    if !unsafe { ASSERT_OK_PTR(fd as *const c_void, c"fd".as_ptr()) } {
        return;
    }
    fexit_fd = fd;
    link_fd = unsafe { fd.add(bpf_max_tramp_links as usize) };

    let trace_program: [bpf_insn; 2] = [bpf_mov64_imm(BPF_REG_0, 0), bpf_exit_insn()];

    let mut trace_opts = bpf_prog_load_opts {
        sz: core::mem::size_of::<bpf_prog_load_opts>(),
        expected_attach_type: BPF_TRACE_FEXIT,
        attach_btf_id: 0,
    };

    let mut topts = bpf_test_run_opts {
        sz: core::mem::size_of::<bpf_test_run_opts>(),
    };

    err = unsafe {
        libbpf_find_vmlinux_btf_id(
            c"bpf_fentry_test1".as_ptr(),
            trace_opts.expected_attach_type,
        )
    };
    if !unsafe { ASSERT_GT(err, 0, c"find_vmlinux_btf_id".as_ptr()) } {
        goto_out(bpf_max_tramp_links, link_fd, fexit_fd, fd);
        return;
    }
    trace_opts.attach_btf_id = err as c_uint;

    i = 0;
    while i < bpf_max_tramp_links {
        unsafe {
            *fexit_fd.add(i as usize) = bpf_prog_load(
                BPF_PROG_TYPE_TRACING,
                core::ptr::null(),
                c"GPL".as_ptr(),
                trace_program.as_ptr(),
                trace_program.len(),
                &trace_opts,
            );
        }
        if !unsafe { ASSERT_GE(*fexit_fd.add(i as usize), 0, c"fexit load".as_ptr()) } {
            goto_out(bpf_max_tramp_links, link_fd, fexit_fd, fd);
            return;
        }
        unsafe {
            *link_fd.add(i as usize) = bpf_link_create(
                *fexit_fd.add(i as usize),
                0,
                BPF_TRACE_FEXIT,
                core::ptr::null(),
            );
        }
        if !unsafe { ASSERT_GE(*link_fd.add(i as usize), 0, c"fexit attach".as_ptr()) } {
            goto_out(bpf_max_tramp_links, link_fd, fexit_fd, fd);
            return;
        }
        i += 1;
    }

    err = unsafe { bpf_prog_test_run_opts(*fexit_fd.add(0), &mut topts) };
    unsafe {
        ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());
    }

    goto_out(bpf_max_tramp_links, link_fd, fexit_fd, fd);
}

unsafe fn goto_out(
    bpf_max_tramp_links: c_int,
    link_fd: *mut c_int,
    fexit_fd: *mut c_int,
    fd: *mut c_int,
) {
    let mut i: c_int = 0;

    while i < bpf_max_tramp_links {
        if unsafe { *link_fd.add(i as usize) } > 0 {
            unsafe {
                close(*link_fd.add(i as usize));
            }
        }
        if unsafe { *fexit_fd.add(i as usize) } > 0 {
            unsafe {
                close(*fexit_fd.add(i as usize));
            }
        }
        i += 1;
    }
    unsafe {
        free(fd as *mut c_void);
    }
}
