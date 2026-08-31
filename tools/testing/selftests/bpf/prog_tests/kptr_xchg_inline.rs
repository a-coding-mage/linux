// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */

// Dependencies from the original C includes:
// <test_progs.h>
// "linux/filter.h"
// "kptr_xchg_inline.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

pub const BPF_REG_0: c_int = 0;
pub const BPF_REG_1: c_int = 1;
pub const BPF_REG_2: c_int = 2;
pub const BPF_DW: c_int = 0x18;
pub const BPF_XCHG: c_int = 0xe0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_insn {
    pub code: u8,
    pub regs: u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
pub struct kptr_xchg_inline_progs {
    pub kptr_xchg_inline: *mut bpf_program,
}

#[repr(C)]
pub struct kptr_xchg_inline {
    pub progs: kptr_xchg_inline_progs,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn test__skip();
    fn kptr_xchg_inline__open_and_load() -> *mut kptr_xchg_inline;
    fn kptr_xchg_inline__destroy(skel: *mut kptr_xchg_inline);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn get_xlated_program(fd: c_int, insn: *mut *mut bpf_insn, cnt: *mut c_uint) -> c_int;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(cnt: c_uint, val: c_uint, name: *const c_char) -> bool;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn free(ptr: *mut c_void);

    // Function-like Rust declarations for C macro dependencies from linux/filter.h.
    fn BPF_MOV64_REG(dst: c_int, src: c_int) -> bpf_insn;
    fn BPF_ATOMIC_OP(size: c_int, op: c_int, dst: c_int, src: c_int, off: c_int) -> bpf_insn;
}

#[no_mangle]
pub unsafe extern "C" fn test_kptr_xchg_inline() {
    let mut skel: *mut kptr_xchg_inline;
    let mut insn: *mut bpf_insn = ptr::null_mut();
    let mut exp: bpf_insn;
    let mut cnt: c_uint = 0;
    let mut err: c_int;

    if !cfg!(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    )) {
        test__skip();
        return;
    }

    skel = kptr_xchg_inline__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_load".as_ptr()) {
        return;
    }

    err = get_xlated_program(
        bpf_program__fd((*skel).progs.kptr_xchg_inline),
        &mut insn,
        &mut cnt,
    );
    if !ASSERT_OK(err, c"prog insn".as_ptr()) {
        goto_out(insn, skel);
        return;
    }

    /* The original instructions are:
     * r1 = map[id:xxx][0]+0
     * r2 = 0
     * call bpf_kptr_xchg#yyy
     *
     * call bpf_kptr_xchg#yyy will be inlined as:
     * r0 = r2
     * r0 = atomic64_xchg((u64 *)(r1 +0), r0)
     */
    if !ASSERT_GT(cnt, 5, c"insn cnt".as_ptr()) {
        goto_out(insn, skel);
        return;
    }

    exp = BPF_MOV64_REG(BPF_REG_0, BPF_REG_2);
    if !ASSERT_OK(
        memcmp(
            insn.add(3) as *const c_void,
            &exp as *const bpf_insn as *const c_void,
            size_of::<bpf_insn>(),
        ),
        c"mov".as_ptr(),
    ) {
        goto_out(insn, skel);
        return;
    }

    exp = BPF_ATOMIC_OP(BPF_DW, BPF_XCHG, BPF_REG_1, BPF_REG_0, 0);
    if !ASSERT_OK(
        memcmp(
            insn.add(4) as *const c_void,
            &exp as *const bpf_insn as *const c_void,
            size_of::<bpf_insn>(),
        ),
        c"xchg".as_ptr(),
    ) {
        goto_out(insn, skel);
        return;
    }

    goto_out(insn, skel);
}

unsafe fn goto_out(insn: *mut bpf_insn, skel: *mut kptr_xchg_inline) {
    free(insn as *mut c_void);
    kptr_xchg_inline__destroy(skel);
}
