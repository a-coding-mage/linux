// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

// Dependencies from <bpf/bpf.h> and "disasm.h" are expected to be supplied by
// the surrounding translation unit/crate.
use crate::{
    bpf_insn, bpf_insn_cbs, print_bpf_insn, BPF_DW, BPF_IMM, BPF_LD, BPF_PSEUDO_CALL,
};
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct print_insn_context {
    pub scratch: [c_char; 16],
    pub buf: *mut c_char,
    pub sz: usize,
}

unsafe extern "C" {
    fn vsnprintf(s: *mut c_char, n: usize, format: *const c_char, arg: VaList) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

// Rust has no stable, portable file-local equivalent for C's va_list without
// relying on the platform ABI definition supplied elsewhere.
pub type VaList = *mut c_void;

unsafe extern "C" fn print_insn_cb(
    private_data: *mut c_void,
    fmt: *const c_char,
    mut args: ...
) {
    let ctx: *mut print_insn_context = private_data as *mut print_insn_context;

    vsnprintf((*ctx).buf, (*ctx).sz, fmt, args.as_va_list());
}

unsafe extern "C" fn print_call_cb(
    private_data: *mut c_void,
    insn: *const bpf_insn,
) -> *const c_char {
    let ctx: *mut print_insn_context = private_data as *mut print_insn_context;

    /* For pseudo calls verifier.c:jit_subprogs() hides original
     * imm to insn->off and changes insn->imm to be an index of
     * the subprog instead.
     */
    if (*insn).src_reg == BPF_PSEUDO_CALL {
        snprintf(
            (*ctx).scratch.as_mut_ptr(),
            (*ctx).scratch.len(),
            c"%+d".as_ptr(),
            (*insn).off as c_int,
        );
        return (*ctx).scratch.as_ptr();
    }

    core::ptr::null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn disasm_insn(
    insn: *mut bpf_insn,
    buf: *mut c_char,
    buf_sz: usize,
) -> *mut bpf_insn {
    let mut ctx: print_insn_context = print_insn_context {
        scratch: [0; 16],
        buf,
        sz: buf_sz,
    };
    let mut cbs: bpf_insn_cbs = bpf_insn_cbs {
        cb_print: Some(print_insn_cb),
        cb_call: Some(print_call_cb),
        private_data: &mut ctx as *mut print_insn_context as *mut c_void,
    };
    let mut tmp: *mut c_char;
    let mut pfx_end: *mut c_char;
    let mut sfx_start: *mut c_char;
    let double_insn: bool;
    let len: c_int;

    print_bpf_insn(&mut cbs, insn, true);
    /* We share code with kernel BPF disassembler, it adds '(FF) ' prefix
     * for each instruction (FF stands for instruction `code` byte).
     * Remove the prefix inplace, and also simplify call instructions.
     * E.g.: "(85) call foo#10" -> "call foo".
     */
    pfx_end = buf.offset(5);
    sfx_start = buf.offset(strlen(buf) as isize);
    tmp = strrchr(buf, b'#' as c_int);
    if strncmp(pfx_end, c"call ".as_ptr(), 5) == 0 && !tmp.is_null() {
        sfx_start = tmp;
    }
    len = sfx_start.offset_from(pfx_end) as c_int;
    memmove(
        buf as *mut c_void,
        pfx_end as *const c_void,
        len as usize,
    );
    *buf.offset(len as isize) = 0;
    double_insn = (*insn).code as c_int == (BPF_LD | BPF_IMM | BPF_DW);
    return insn.offset(if double_insn { 2 } else { 1 });
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
