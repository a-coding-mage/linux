// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/disasm.h.
// C include dependencies removed; externally supplied C symbols are declared below.

use std::os::raw::{c_char, c_int, c_uint};

#[repr(C)]
pub struct annotation_options {
    _private: [u8; 0],
}

#[repr(C)]
pub struct disasm_line {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct data_loc_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct type_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map_symbol {
    _private: [u8; 0],
}

// Present only when the original C build defines HAVE_LIBDW_SUPPORT.
#[repr(C)]
pub struct Dwarf_Die {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e_machine_and_e_flags {
    pub e_flags: u32,
    pub e_machine: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_objdump {
    pub comment_char: c_char,
    pub skip_functions_char: c_char,
    pub register_char: c_char,
    pub memory_ref_char: c_char,
    pub imm_char: c_char,
}

#[repr(C)]
pub struct arch {
    /// @name: name such as "x86" or "powerpc".
    pub name: *const c_char,
    pub instructions: *const ins,
    pub nr_instructions: usize,
    pub nr_instructions_allocated: usize,
    pub insn_suffix: *const c_char,
    pub model: c_uint,
    pub family: c_uint,
    /// @id: ELF machine and flags associated with arch.
    pub id: e_machine_and_e_flags,
    pub sorted_instructions: bool,
    pub objdump: arch_objdump,
    pub ins_is_fused:
        Option<unsafe extern "C" fn(arch: *const arch, ins1: *const c_char, ins2: *const c_char) -> bool>,
    pub associate_instruction_ops:
        Option<unsafe extern "C" fn(arch: *mut arch, name: *const c_char) -> *const ins_ops>,
    // Present only when the original C build defines HAVE_LIBDW_SUPPORT.
    pub update_insn_state: Option<
        unsafe extern "C" fn(
            state: *mut type_state,
            dloc: *mut data_loc_info,
            cu_die: *mut Dwarf_Die,
            dl: *mut disasm_line,
        ),
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins {
    pub name: *const c_char,
    pub ops: *const ins_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_operands_target {
    pub raw: *mut c_char,
    pub name: *mut c_char,
    pub sym: *mut symbol,
    pub addr: u64,
    pub offset: i64,
    pub offset_avail: bool,
    pub outside: bool,
    pub multi_regs: bool,
    pub mem_ref: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_operands_source {
    pub raw: *mut c_char,
    pub name: *mut c_char,
    pub addr: u64,
    pub multi_regs: bool,
    pub mem_ref: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_operands_locked {
    pub ins: ins,
    pub ops: *mut ins_operands,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_operands_jump {
    pub raw_comment: *mut c_char,
    pub raw_func_start: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ins_operands_union {
    pub source: ins_operands_source,
    pub locked: ins_operands_locked,
    pub jump: ins_operands_jump,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_operands {
    pub raw: *mut c_char,
    pub target: ins_operands_target,
    pub u: ins_operands_union,
}

#[repr(C)]
pub struct ins_ops {
    pub free: Option<unsafe extern "C" fn(ops: *mut ins_operands)>,
    pub parse: Option<
        unsafe extern "C" fn(
            arch: *const arch,
            ops: *mut ins_operands,
            ms: *mut map_symbol,
            dl: *mut disasm_line,
        ) -> c_int,
    >,
    pub scnprintf: Option<
        unsafe extern "C" fn(
            ins: *const ins,
            bf: *mut c_char,
            size: usize,
            ops: *mut ins_operands,
            max_ins_name: c_int,
        ) -> c_int,
    >,
    pub is_jump: bool,
    pub is_call: bool,
}

#[repr(C)]
pub struct annotate_args {
    pub arch: *const arch,
    pub ms: *mut map_symbol,
    pub options: *mut annotation_options,
    pub offset: i64,
    pub line: *mut c_char,
    pub line_nr: c_int,
    pub fileloc: *mut c_char,
}

unsafe extern "C" {
    pub fn arch__find(e_machine: u16, e_flags: u32, cpuid: *const c_char) -> *const arch;
    pub fn arch__is_x86(arch: *const arch) -> bool;
    pub fn arch__is_powerpc(arch: *const arch) -> bool;

    pub static call_ops: ins_ops;
    pub static dec_ops: ins_ops;
    pub static jump_ops: ins_ops;
    pub static mov_ops: ins_ops;
    pub static nop_ops: ins_ops;
    pub static lock_ops: ins_ops;
    pub static ret_ops: ins_ops;

    pub fn arch__associate_ins_ops(arch: *mut arch, name: *const c_char, ops: *const ins_ops)
        -> c_int;

    pub fn arch__new_arc(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    pub fn arch__new_arm(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    pub fn arch__new_arm64(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    pub fn arch__new_csky(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    pub fn arch__new_loongarch(
        id: *const e_machine_and_e_flags,
        cpuid: *const c_char,
    ) -> *const arch;
    pub fn arch__new_mips(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    pub fn arch__new_powerpc(
        id: *const e_machine_and_e_flags,
        cpuid: *const c_char,
    ) -> *const arch;
    pub fn arch__new_riscv64(
        id: *const e_machine_and_e_flags,
        cpuid: *const c_char,
    ) -> *const arch;
    pub fn arch__new_s390(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    pub fn arch__new_sparc(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    pub fn arch__new_x86(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;

    pub fn ins__find(arch: *const arch, name: *const c_char, dl: *mut disasm_line)
        -> *const ins_ops;

    pub fn ins__is_call(ins: *const ins) -> bool;
    pub fn ins__is_jump(ins: *const ins) -> bool;
    pub fn ins__is_fused(arch: *const arch, ins1: *const c_char, ins2: *const c_char) -> bool;
    pub fn ins__is_ret(ins: *const ins) -> bool;
    pub fn ins__is_lock(ins: *const ins) -> bool;

    pub fn check_ppc_insn(dl: *mut disasm_line) -> *const ins_ops;

    pub fn disasm_line__new(args: *mut annotate_args) -> *mut disasm_line;
    pub fn disasm_line__free(dl: *mut disasm_line);

    pub fn disasm_line__scnprintf(
        dl: *mut disasm_line,
        bf: *mut c_char,
        size: usize,
        raw: bool,
        max_ins_name: c_int,
    ) -> c_int;

    pub fn ins__raw_scnprintf(
        ins: *const ins,
        bf: *mut c_char,
        size: usize,
        ops: *mut ins_operands,
        max_ins_name: c_int,
    ) -> c_int;
    pub fn ins__scnprintf(
        ins: *const ins,
        bf: *mut c_char,
        size: usize,
        ops: *mut ins_operands,
        max_ins_name: c_int,
    ) -> c_int;
    pub fn call__scnprintf(
        ins: *const ins,
        bf: *mut c_char,
        size: usize,
        ops: *mut ins_operands,
        max_ins_name: c_int,
    ) -> c_int;
    pub fn jump__scnprintf(
        ins: *const ins,
        bf: *mut c_char,
        size: usize,
        ops: *mut ins_operands,
        max_ins_name: c_int,
    ) -> c_int;
    pub fn mov__scnprintf(
        ins: *const ins,
        bf: *mut c_char,
        size: usize,
        ops: *mut ins_operands,
        max_ins_name: c_int,
    ) -> c_int;

    pub fn jump__delete(ops: *mut ins_operands);

    pub fn symbol__disassemble(sym: *mut symbol, args: *mut annotate_args) -> c_int;

    pub fn expand_tabs(
        line: *mut c_char,
        storage: *mut *mut c_char,
        storage_len: *mut usize,
    ) -> *mut c_char;
}
