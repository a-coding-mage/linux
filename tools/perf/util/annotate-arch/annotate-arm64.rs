// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/annotate-arch/annotate-arm64.c.
// C include dependencies are represented as C-compatible declarations below.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

const REG_EXTENDED: c_int = 1;
const SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_REGEXP: c_int = 1000;

#[repr(C)]
pub struct regex_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmatch_t {
    pub rm_so: isize,
    pub rm_eo: isize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e_machine_and_e_flags {
    pub e_machine: u16,
    pub e_flags: u32,
}

#[repr(C)]
pub struct objdump {
    pub comment_char: c_char,
    pub skip_functions_char: c_char,
}

#[repr(C)]
pub struct arch {
    pub name: *const c_char,
    pub id: e_machine_and_e_flags,
    pub objdump: objdump,
    pub associate_instruction_ops:
        Option<unsafe extern "C" fn(*mut arch, *const c_char) -> *const ins_ops>,
}

#[repr(C)]
pub struct operand {
    pub raw: *mut c_char,
    pub addr: u64,
    pub name: *mut c_char,
}

#[repr(C)]
pub struct ins_operands {
    pub raw: *mut c_char,
    pub source: operand,
    pub target: operand,
}

#[repr(C)]
pub struct map_symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct disasm_line {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ins_ops {
    pub parse: Option<
        unsafe extern "C" fn(
            *const arch,
            *mut ins_operands,
            *mut map_symbol,
            *mut disasm_line,
        ) -> c_int,
    >,
    pub scnprintf: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct arch_arm64 {
    pub arch: arch,
    pub call_insn: regex_t,
    pub jump_insn: regex_t,
}

unsafe extern "C" {
    static jump_ops: ins_ops;
    static call_ops: ins_ops;
    static ret_ops: ins_ops;
    static mut errno: c_int;

    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn free(ptr: *mut c_void);
    fn zalloc(size: usize) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: usize,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
    fn regfree(preg: *mut regex_t);
    fn arch__associate_ins_ops(arch: *mut arch, name: *const c_char, ops: *const ins_ops);
    fn mov__scnprintf();
}

unsafe extern "C" fn arm64_mov__parse(
    _arch: *const arch,
    ops: *mut ins_operands,
    _ms: *mut map_symbol,
    _dl: *mut disasm_line,
) -> c_int {
    let mut s = strchr((*ops).raw, b',' as c_int);
    let mut target: *mut c_char;
    let mut endptr: *mut c_char = core::ptr::null_mut();

    if s.is_null() {
        return -1;
    }

    *s = b'\0' as c_char;
    (*ops).source.raw = strdup((*ops).raw);
    *s = b',' as c_char;

    if (*ops).source.raw.is_null() {
        return -1;
    }

    s = s.add(1);
    target = s;
    (*ops).target.raw = strdup(target);
    if (*ops).target.raw.is_null() {
        goto_out_free_source(ops);
        return -1;
    }

    (*ops).target.addr = strtoull(target, &mut endptr, 16);
    if endptr == target {
        goto_out_free_target(ops);
        return -1;
    }

    s = strchr(endptr, b'<' as c_int);
    if s.is_null() {
        goto_out_free_target(ops);
        return -1;
    }
    endptr = strchr(s.add(1), b'>' as c_int);
    if endptr.is_null() {
        goto_out_free_target(ops);
        return -1;
    }

    *endptr = b'\0' as c_char;
    *s = b' ' as c_char;
    (*ops).target.name = strdup(s);
    *s = b'<' as c_char;
    *endptr = b'>' as c_char;
    if (*ops).target.name.is_null() {
        goto_out_free_target(ops);
        return -1;
    }

    0
}

unsafe fn goto_out_free_target(ops: *mut ins_operands) {
    zfree(&mut (*ops).target.raw as *mut *mut c_char as *mut *mut c_void);
    goto_out_free_source(ops);
}

unsafe fn goto_out_free_source(ops: *mut ins_operands) {
    zfree(&mut (*ops).source.raw as *mut *mut c_char as *mut *mut c_void);
}

static arm64_mov_ops: ins_ops = ins_ops {
    parse: Some(arm64_mov__parse),
    scnprintf: Some(mov__scnprintf),
};

unsafe extern "C" fn arm64__associate_instruction_ops(
    arch: *mut arch,
    name: *const c_char,
) -> *const ins_ops {
    let arm = arch as *mut arch_arm64;
    let ops: *const ins_ops;
    let mut match_: [regmatch_t; 2] = [
        regmatch_t { rm_so: 0, rm_eo: 0 },
        regmatch_t { rm_so: 0, rm_eo: 0 },
    ];

    if regexec(&(*arm).jump_insn, name, 2, match_.as_mut_ptr(), 0) == 0 {
        ops = &jump_ops;
    } else if regexec(&(*arm).call_insn, name, 2, match_.as_mut_ptr(), 0) == 0 {
        ops = &call_ops;
    } else if strcmp(name, c"ret".as_ptr()) == 0 {
        ops = &ret_ops;
    } else {
        ops = &arm64_mov_ops;
    }

    arch__associate_ins_ops(arch, name, ops);
    ops
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch__new_arm64(
    id: *const e_machine_and_e_flags,
    _cpuid: *const c_char,
) -> *const arch {
    let mut err: c_int;
    let arm = zalloc(core::mem::size_of::<arch_arm64>()) as *mut arch_arm64;
    let arch: *mut arch;

    if arm.is_null() {
        return core::ptr::null();
    }

    arch = &mut (*arm).arch;
    (*arch).name = c"arm64".as_ptr();
    (*arch).id = *id;
    (*arch).objdump.comment_char = b'/' as c_char;
    (*arch).objdump.skip_functions_char = b'+' as c_char;
    (*arch).associate_instruction_ops = Some(arm64__associate_instruction_ops);

    /* bl, blr */
    err = regcomp(&mut (*arm).call_insn, c"^blr?$".as_ptr(), REG_EXTENDED);
    if err != 0 {
        free(arm as *mut c_void);
        errno = SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_REGEXP;
        return core::ptr::null();
    }

    /* b, b.cond, br, cbz/cbnz, tbz/tbnz */
    err = regcomp(
        &mut (*arm).jump_insn,
        c"^[ct]?br?\\.?(cc|cs|eq|ge|gt|hi|hs|le|lo|ls|lt|mi|ne|pl|vc|vs)?n?z?$".as_ptr(),
        REG_EXTENDED,
    );
    if err != 0 {
        regfree(&mut (*arm).call_insn);
        free(arm as *mut c_void);
        errno = SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_REGEXP;
        return core::ptr::null();
    }

    arch
}
