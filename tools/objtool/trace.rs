// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2025, Oracle and/or its affiliates.
 */

// Translated from C source: <objtool/trace.h> supplies the external types,
// constants, globals, and tracing/disassembly helpers referenced here.

use core::ffi::{c_char, c_int, c_uint, c_void};

pub static mut trace: bool = false;
pub static mut trace_depth: c_int = 0;

const CFI_REG_NAME_MAXLEN: usize = 16;

unsafe extern "C" {
    static arch_reg_name: *const *const c_char;
    static mut stderr: *mut c_void;
    static mut objtool_disas_ctx: *mut c_void;

    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;

    fn disas_print_insn(
        file: *mut c_void,
        ctx: *mut c_void,
        insn: *mut instruction,
        depth: c_int,
        prefix: *const c_char,
    );
    fn insn_sym(insn: *mut instruction) -> *mut symbol;
}

unsafe extern "C" {
    fn TRACE(format: *const c_char, ...);
    fn TRACE_ADDR(insn: *mut instruction, format: *const c_char, ...);
    fn TRACE_ALT_INFO_NOADDR(
        insn: *mut instruction,
        prefix: *const c_char,
        format: *const c_char,
        ...
    );
}

unsafe extern "C" {
    static CFI_UNDEFINED: c_int;
    static CFI_CFA: c_int;
    static CFI_SP_INDIRECT: c_int;
    static CFI_BP_INDIRECT: c_int;
    static CFI_NUM_REGS: c_int;
    static ALT_TYPE_EX_TABLE: c_int;
    static ALT_TYPE_JUMP_TABLE: c_int;
    static INSN_NOP: c_int;
}

#[repr(C)]
pub struct cfi_reg {
    pub base: c_int,
    pub offset: c_int,
}

#[repr(C)]
pub struct cfi_state {
    pub cfa: cfi_reg,
    pub vals: *mut cfi_reg,
    pub regs: *mut cfi_reg,
    pub stack_size: c_int,
    pub drap: bool,
    pub drap_reg: c_int,
    pub drap_offset: c_int,
    pub bp_scratch: bool,
}

#[repr(C)]
pub struct insn_state {
    pub cfi: cfi_state,
    pub instr: c_int,
    pub uaccess_stack: c_uint,
}

#[repr(C)]
pub struct instruction {
    pub trace: c_int,
    pub offset: c_ulong,
    pub len: c_int,
    pub type_: c_int,
}

#[repr(C)]
pub struct alternative {
    pub insn: *mut instruction,
    pub type_: c_int,
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
    pub offset: c_ulong,
}

#[allow(non_camel_case_types)]
type c_ulong = u64;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

/*
 * Macros to trace CFI state attributes changes.
 */

unsafe fn trace_cfi_attr_bool(
    attr_name: *const c_char,
    prev: bool,
    next: bool,
) {
    if prev != next {
        TRACE(
            cstr!("%s=%s "),
            attr_name,
            if next { cstr!("true") } else { cstr!("false") },
        );
    }
}

unsafe fn trace_cfi_attr_num_int(
    attr_name: *const c_char,
    prev: c_int,
    next: c_int,
    fmt: *const c_char,
) {
    if prev != next {
        TRACE(cstr!("%s="), attr_name);
        TRACE(fmt, next);
        TRACE(cstr!(" "));
    }
}

unsafe fn trace_cfi_attr_num_uint(
    attr_name: *const c_char,
    prev: c_uint,
    next: c_uint,
    fmt: *const c_char,
) {
    if prev != next {
        TRACE(cstr!("%s="), attr_name);
        TRACE(fmt, next);
        TRACE(cstr!(" "));
    }
}

/*
 * Return the name of a register. Note that the same static buffer
 * is returned if the name is dynamically generated.
 */
unsafe fn cfi_reg_name(reg: c_uint) -> *const c_char {
    static mut RNAME_BUFFER: [c_char; CFI_REG_NAME_MAXLEN] = [0; CFI_REG_NAME_MAXLEN];
    let rname: *const c_char;

    if reg == CFI_UNDEFINED as c_uint {
        return cstr!("<undefined>");
    }
    if reg == CFI_CFA as c_uint {
        return cstr!("cfa");
    }
    if reg == CFI_SP_INDIRECT as c_uint {
        return cstr!("(sp)");
    }
    if reg == CFI_BP_INDIRECT as c_uint {
        return cstr!("(bp)");
    }

    if reg < CFI_NUM_REGS as c_uint {
        rname = *arch_reg_name.add(reg as usize);
        if !rname.is_null() {
            return rname;
        }
    }

    if snprintf(
        RNAME_BUFFER.as_mut_ptr(),
        CFI_REG_NAME_MAXLEN,
        cstr!("r%d"),
        reg,
    ) == -1
    {
        return cstr!("<error>");
    }

    RNAME_BUFFER.as_ptr()
}

/*
 * Functions and macros to trace CFI registers changes.
 */

unsafe fn trace_cfi_reg(
    prefix: *const c_char,
    reg: c_int,
    fmt: *const c_char,
    base_prev: c_int,
    offset_prev: c_int,
    base_next: c_int,
    offset_next: c_int,
) {
    let rname: *mut c_char;

    if base_prev == base_next && offset_prev == offset_next {
        return;
    }

    if !prefix.is_null() {
        TRACE(cstr!("%s:"), prefix);
    }

    if base_next == CFI_UNDEFINED {
        TRACE(cstr!("%1$s=<undef> "), cfi_reg_name(reg as c_uint));
    } else {
        rname = strdup(cfi_reg_name(reg as c_uint));
        TRACE(fmt, rname, cfi_reg_name(base_next as c_uint), offset_next);
        free(rname as *mut c_void);
    }
}

unsafe fn trace_cfi_reg_val(
    prefix: *const c_char,
    reg: c_int,
    base_prev: c_int,
    offset_prev: c_int,
    base_next: c_int,
    offset_next: c_int,
) {
    trace_cfi_reg(
        prefix,
        reg,
        cstr!("%1$s=%2$s%3$+d "),
        base_prev,
        offset_prev,
        base_next,
        offset_next,
    );
}

unsafe fn trace_cfi_reg_ref(
    prefix: *const c_char,
    reg: c_int,
    base_prev: c_int,
    offset_prev: c_int,
    base_next: c_int,
    offset_next: c_int,
) {
    trace_cfi_reg(
        prefix,
        reg,
        cstr!("%1$s=(%2$s%3$+d) "),
        base_prev,
        offset_prev,
        base_next,
        offset_next,
    );
}

#[no_mangle]
pub unsafe extern "C" fn trace_insn_state(
    insn: *mut instruction,
    sprev: *mut insn_state,
    snext: *mut insn_state,
) {
    let cprev: *mut cfi_state;
    let cnext: *mut cfi_state;
    let mut i: c_int;

    if memcmp(
        sprev as *const c_void,
        snext as *const c_void,
        core::mem::size_of::<insn_state>(),
    ) == 0
    {
        return;
    }

    cprev = &mut (*sprev).cfi;
    cnext = &mut (*snext).cfi;

    disas_print_insn(
        stderr,
        objtool_disas_ctx,
        insn,
        trace_depth - 1,
        cstr!("state: "),
    );

    /* print registers changes */
    trace_cfi_reg_val(
        core::ptr::null(),
        CFI_CFA,
        (*cprev).cfa.base,
        (*cprev).cfa.offset,
        (*cnext).cfa.base,
        (*cnext).cfa.offset,
    );
    i = 0;
    while i < CFI_NUM_REGS {
        let prev_val = (*cprev).vals.add(i as usize);
        let next_val = (*cnext).vals.add(i as usize);
        let prev_reg = (*cprev).regs.add(i as usize);
        let next_reg = (*cnext).regs.add(i as usize);

        trace_cfi_reg_val(
            core::ptr::null(),
            i,
            (*prev_val).base,
            (*prev_val).offset,
            (*next_val).base,
            (*next_val).offset,
        );
        trace_cfi_reg_ref(
            core::ptr::null(),
            i,
            (*prev_reg).base,
            (*prev_reg).offset,
            (*next_reg).base,
            (*next_reg).offset,
        );
        i += 1;
    }

    /* print attributes changes */
    trace_cfi_attr_num_int(
        cstr!("stack_size"),
        (*cprev).stack_size,
        (*cnext).stack_size,
        cstr!("%d"),
    );
    trace_cfi_attr_bool(cstr!("drap"), (*cprev).drap, (*cnext).drap);
    if (*cnext).drap {
        trace_cfi_reg_val(
            cstr!("drap"),
            (*cnext).drap_reg,
            (*cprev).drap_reg,
            (*cprev).drap_offset,
            (*cnext).drap_reg,
            (*cnext).drap_offset,
        );
    }
    trace_cfi_attr_bool(cstr!("bp_scratch"), (*cprev).bp_scratch, (*cnext).bp_scratch);
    trace_cfi_attr_num_int(cstr!("instr"), (*sprev).instr, (*snext).instr, cstr!("%d"));
    trace_cfi_attr_num_uint(
        cstr!("uaccess_stack"),
        (*sprev).uaccess_stack,
        (*snext).uaccess_stack,
        cstr!("%u"),
    );

    TRACE(cstr!("\n"));

    (*insn).trace = 1;
}

#[no_mangle]
pub unsafe extern "C" fn trace_alt_begin(
    orig_insn: *mut instruction,
    alt: *mut alternative,
    alt_name: *mut c_char,
) {
    let alt_insn: *mut instruction;
    let mut suffix: [c_char; 2] = [0; 2];

    alt_insn = (*alt).insn;

    if (*alt).type_ == ALT_TYPE_EX_TABLE {
        /*
         * When there is an exception table then the instruction
         * at the original location is executed but it can cause
         * an exception. In that case, the execution will be
         * redirected to the alternative instruction.
         *
         * The instruction at the original location can have
         * instruction alternatives, so we just print the location
         * of the instruction that can cause the exception and
         * not the instruction itself.
         */
        TRACE_ALT_INFO_NOADDR(
            orig_insn,
            cstr!("/ "),
            cstr!("%s for instruction at 0x%lx <%s+0x%lx>"),
            alt_name,
            (*orig_insn).offset,
            (*insn_sym(orig_insn)).name,
            (*orig_insn).offset.wrapping_sub((*insn_sym(orig_insn)).offset),
        );
    } else {
        TRACE_ALT_INFO_NOADDR(orig_insn, cstr!("/ "), cstr!("%s"), alt_name);
    }

    if (*alt).type_ == ALT_TYPE_JUMP_TABLE {
        /*
         * For a jump alternative, if the default instruction is
         * a NOP then it is replaced with the jmp instruction,
         * otherwise it is replaced with a NOP instruction.
         */
        trace_depth += 1;
        if (*orig_insn).type_ == INSN_NOP {
            suffix[0] = if (*orig_insn).len == 5 { b'q' as c_char } else { 0 };
            TRACE_ADDR(
                orig_insn,
                cstr!("jmp%-3s %lx <%s+0x%lx>"),
                suffix.as_mut_ptr(),
                (*alt_insn).offset,
                (*insn_sym(alt_insn)).name,
                (*alt_insn).offset.wrapping_sub((*insn_sym(alt_insn)).offset),
            );
        } else {
            TRACE_ADDR(orig_insn, cstr!("nop%d"), (*orig_insn).len);
            trace_depth -= 1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn trace_alt_end(
    orig_insn: *mut instruction,
    alt: *mut alternative,
    alt_name: *mut c_char,
) {
    if (*alt).type_ == ALT_TYPE_JUMP_TABLE && (*orig_insn).type_ == INSN_NOP {
        trace_depth -= 1;
    }
    TRACE_ALT_INFO_NOADDR(orig_insn, cstr!("\\ "), cstr!("%s"), alt_name);
}
