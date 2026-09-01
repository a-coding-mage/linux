// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Red Hat */

// C dependencies translated as external declarations:
// <test_progs.h>, <bpf/btf.h>, "bpf/libbpf_internal.h",
// "cgroup_helpers.h", and "bpf_util.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type __u32 = u32;
type __u64 = u64;
type __s32 = i32;

const ENOENT: c_int = 2;
const BTF_KIND_FUNC: c_int = 12;
const BPF_PROG_TYPE_TRACING: c_int = 26;
const BPF_TRACE_FENTRY: c_int = 0;
const BPF_REG_0: c_int = 0;
const BPF_JMP: u8 = 0x05;
const BPF_EXIT: u8 = 0x90;
const BPF_ALU64: u8 = 0x07;
const BPF_MOV: u8 = 0xb0;
const BPF_K: u8 = 0x00;

static MODULE_NAME: &[u8] = b"bpf_testmod\0";
static SYMBOL_NAME: &[u8] = b"bpf_fentry_shadow_test\0";

static mut module_name: *const c_char = MODULE_NAME.as_ptr() as *const c_char;
static mut symbol_name: *const c_char = SYMBOL_NAME.as_ptr() as *const c_char;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_btf_info {
    pub btf: __u64,
    pub btf_size: __u32,
    pub id: __u32,
    pub name: __u64,
    pub name_len: __u32,
    pub kernel_btf: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_insn {
    pub code: u8,
    pub regs: u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub sz: usize,
    pub expected_attach_type: c_int,
    pub attach_btf_id: __s32,
    pub attach_btf_obj_fd: c_int,
}

#[repr(C)]
pub struct test_env {
    pub has_testmod: bool,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut env: test_env;

    fn bpf_btf_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    fn bpf_btf_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_obj_get_info_by_fd(fd: c_int, info: *mut c_void, info_len: *__u32) -> c_int;
    fn btf__load_vmlinux_btf() -> *mut btf;
    fn btf_get_from_fd(fd: c_int, base_btf: *mut btf) -> *mut btf;
    fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: c_int) -> c_int;
    fn bpf_prog_load(
        prog_type: c_int,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: usize,
        opts: *mut bpf_prog_load_opts,
    ) -> c_int;
    fn bpf_link_create(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: c_int,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *const c_void) -> c_int;
    fn btf__free(btf: *mut btf);
    fn close(fd: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn log_err(fmt: *const c_char, ...);
    fn test__skip();
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
}

#[inline]
const fn ptr_to_u64<T>(ptr: *const T) -> __u64 {
    ptr as __u64
}

#[inline]
const fn bpf_insn_regs(dst_reg: c_int, src_reg: c_int) -> u8 {
    ((dst_reg as u8) & 0x0f) | (((src_reg as u8) & 0x0f) << 4)
}

#[inline]
const fn BPF_MOV64_IMM(dst_reg: c_int, imm: i32) -> bpf_insn {
    bpf_insn {
        code: BPF_ALU64 | BPF_MOV | BPF_K,
        regs: bpf_insn_regs(dst_reg, 0),
        off: 0,
        imm,
    }
}

#[inline]
const fn BPF_EXIT_INSN() -> bpf_insn {
    bpf_insn {
        code: BPF_JMP | BPF_EXIT,
        regs: 0,
        off: 0,
        imm: 0,
    }
}

unsafe fn get_bpf_testmod_btf_fd() -> c_int {
    let mut info: bpf_btf_info = mem::zeroed();
    let mut name: [c_char; 64] = [0; 64];
    let mut id: __u32 = 0;
    let mut len: __u32;
    let mut err: c_int;
    let mut fd: c_int;

    loop {
        err = bpf_btf_get_next_id(id, &mut id);
        if err != 0 {
            log_err(c"failed to iterate BTF objects".as_ptr());
            return err;
        }

        fd = bpf_btf_get_fd_by_id(id);
        if fd < 0 {
            if errno == ENOENT {
                continue; /* expected race: BTF was unloaded */
            }
            err = -errno;
            log_err(c"failed to get FD for BTF object #%d".as_ptr(), id);
            return err;
        }

        len = mem::size_of::<bpf_btf_info>() as __u32;
        memset(
            &mut info as *mut bpf_btf_info as *mut c_void,
            0,
            mem::size_of::<bpf_btf_info>(),
        );
        info.name = ptr_to_u64(name.as_mut_ptr());
        info.name_len = mem::size_of_val(&name) as __u32;

        err = bpf_obj_get_info_by_fd(
            fd,
            &mut info as *mut bpf_btf_info as *mut c_void,
            &mut len,
        );
        if err != 0 {
            err = -errno;
            log_err(c"failed to get info for BTF object #%d".as_ptr(), id);
            close(fd);
            return err;
        }

        if strcmp(name.as_ptr(), module_name) == 0 {
            return fd;
        }

        close(fd);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_module_fentry_shadow() {
    let mut vmlinux_btf: *mut btf = ptr::null_mut();
    let mut mod_btf: *mut btf = ptr::null_mut();
    let mut err: c_int;
    let mut i: c_int;
    let mut btf_fd: [c_int; 2] = [0; 2];
    let mut prog_fd: [c_int; 2] = [0; 2];
    let mut link_fd: [c_int; 2] = [0; 2];
    let mut btf_id: [__s32; 2] = [0; 2];

    if !env.has_testmod {
        test__skip();
        return;
    }

    let mut load_opts = bpf_prog_load_opts {
        sz: mem::size_of::<bpf_prog_load_opts>(),
        expected_attach_type: BPF_TRACE_FENTRY,
        attach_btf_id: 0,
        attach_btf_obj_fd: 0,
    };

    let trace_program: [bpf_insn; 2] = [
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];

    vmlinux_btf = btf__load_vmlinux_btf();
    if !ASSERT_OK_PTR(vmlinux_btf as *const c_void, c"load_vmlinux_btf".as_ptr()) {
        return;
    }

    btf_fd[1] = get_bpf_testmod_btf_fd();
    if !ASSERT_GE(btf_fd[1], 0, c"get_bpf_testmod_btf_fd".as_ptr()) {
        goto_out(vmlinux_btf, mod_btf, &mut btf_fd, &mut prog_fd, &mut link_fd);
        return;
    }

    mod_btf = btf_get_from_fd(btf_fd[1], vmlinux_btf);
    if !ASSERT_OK_PTR(mod_btf as *const c_void, c"btf_get_from_fd".as_ptr()) {
        goto_out(vmlinux_btf, mod_btf, &mut btf_fd, &mut prog_fd, &mut link_fd);
        return;
    }

    btf_id[0] = btf__find_by_name_kind(vmlinux_btf, symbol_name, BTF_KIND_FUNC);
    if !ASSERT_GT(btf_id[0], 0, c"btf_find_by_name".as_ptr()) {
        goto_out(vmlinux_btf, mod_btf, &mut btf_fd, &mut prog_fd, &mut link_fd);
        return;
    }

    btf_id[1] = btf__find_by_name_kind(mod_btf, symbol_name, BTF_KIND_FUNC);
    if !ASSERT_GT(btf_id[1], 0, c"btf_find_by_name".as_ptr()) {
        goto_out(vmlinux_btf, mod_btf, &mut btf_fd, &mut prog_fd, &mut link_fd);
        return;
    }

    i = 0;
    while i < 2 {
        let idx = i as usize;
        load_opts.attach_btf_id = btf_id[idx];
        load_opts.attach_btf_obj_fd = btf_fd[idx];
        prog_fd[idx] = bpf_prog_load(
            BPF_PROG_TYPE_TRACING,
            ptr::null(),
            c"GPL".as_ptr(),
            trace_program.as_ptr(),
            trace_program.len(),
            &mut load_opts,
        );
        if !ASSERT_GE(prog_fd[idx], 0, c"bpf_prog_load".as_ptr()) {
            goto_out(vmlinux_btf, mod_btf, &mut btf_fd, &mut prog_fd, &mut link_fd);
            return;
        }

        /* If the verifier incorrectly resolves addresses of the
         * shadowed functions and uses the same address for both the
         * vmlinux and the bpf_testmod functions, this will fail on
         * attempting to create two trampolines for the same address,
         * which is forbidden.
         */
        link_fd[idx] = bpf_link_create(prog_fd[idx], 0, BPF_TRACE_FENTRY, ptr::null());
        if !ASSERT_GE(link_fd[idx], 0, c"bpf_link_create".as_ptr()) {
            goto_out(vmlinux_btf, mod_btf, &mut btf_fd, &mut prog_fd, &mut link_fd);
            return;
        }

        i += 1;
    }

    err = bpf_prog_test_run_opts(prog_fd[0], ptr::null());
    ASSERT_OK(err, c"running test".as_ptr());

    goto_out(vmlinux_btf, mod_btf, &mut btf_fd, &mut prog_fd, &mut link_fd);
}

unsafe fn goto_out(
    vmlinux_btf: *mut btf,
    mod_btf: *mut btf,
    btf_fd: &mut [c_int; 2],
    prog_fd: &mut [c_int; 2],
    link_fd: &mut [c_int; 2],
) {
    btf__free(vmlinux_btf);
    btf__free(mod_btf);
    let mut i: c_int = 0;
    while i < 2 {
        let idx = i as usize;
        if btf_fd[idx] != 0 {
            close(btf_fd[idx]);
        }
        if prog_fd[idx] > 0 {
            close(prog_fd[idx]);
        }
        if link_fd[idx] > 0 {
            close(link_fd[idx]);
        }
        i += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
