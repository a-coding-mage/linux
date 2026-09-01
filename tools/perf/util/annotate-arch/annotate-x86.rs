// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/annotate-arch/annotate-x86.c.
// External types, globals, and functions are supplied by the surrounding perf
// sources.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type u64 = u64;
type s64 = i64;
type size_t = usize;

const INSN_OP_SOURCE: usize = 0;
const INSN_OP_TARGET: usize = 1;
const INSN_SEG_X86_GS: c_int = 1;
const DWARF_REG_PC: c_int = -1;
const DW_TAG_pointer_type: c_int = 0x0f;
const TSR_KIND_INVALID: c_int = 0;
const TSR_KIND_TYPE: c_int = 1;
const TSR_KIND_POINTER: c_int = 2;
const TSR_KIND_CONST: c_int = 3;
const TSR_KIND_PERCPU_BASE: c_int = 4;
const TSR_KIND_PERCPU_POINTER: c_int = 5;
const TSR_KIND_CANARY: c_int = 6;
const SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_CPUID_PARSING: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e_machine_and_e_flags {
    _data: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins_ops {
    _data: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ins {
    pub name: *const c_char,
    pub ops: *const ins_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct objdump_arch {
    pub comment_char: c_char,
    pub register_char: c_char,
    pub memory_ref_char: c_char,
    pub imm_char: c_char,
}

#[repr(C)]
pub struct arch {
    pub name: *const c_char,
    pub id: e_machine_and_e_flags,
    pub family: c_uint,
    pub model: c_uint,
    pub ins_is_fused:
        Option<unsafe extern "C" fn(*const arch, *const c_char, *const c_char) -> bool_>,
    pub instructions: *const ins,
    pub nr_instructions: size_t,
    pub sorted_instructions: bool_,
    pub objdump: objdump_arch,
    pub insn_suffix: *const c_char,
    pub update_insn_state: Option<
        unsafe extern "C" fn(*mut type_state, *mut data_loc_info, *mut Dwarf_Die, *mut disasm_line),
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dwarf_Die {
    _data: [u8; 0],
}

#[repr(C)]
pub struct type_state_reg {
    pub type_: Dwarf_Die,
    pub kind: c_int,
    pub ok: bool_,
    pub lifetime_active: bool_,
    pub lifetime_end: u64,
    pub copied_from: c_int,
    pub caller_saved: bool_,
    pub offset: u64,
    pub imm_value: u64,
}

#[repr(C)]
pub struct type_state_stack {
    pub type_: Dwarf_Die,
    pub kind: c_int,
    pub offset: c_int,
    pub ptr_offset: u64,
    pub compound: bool_,
}

#[repr(C)]
pub struct type_state {
    pub regs: [type_state_reg; 0],
    pub ret_reg: c_int,
    pub stack_reg: c_int,
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
    pub start: u64,
}

#[repr(C)]
pub struct map {
    _data: [u8; 0],
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct debug_info {
    pub dbg: *mut c_void,
}

#[repr(C)]
pub struct data_loc_info {
    pub arch: *const arch,
    pub fbreg: c_int,
    pub fb_cfa: bool_,
    pub ms: *mut map_symbol,
    pub di: *mut debug_info,
}

#[repr(C)]
pub struct annotated_op_loc {
    pub reg1: c_int,
    pub reg2: c_int,
    pub offset: u64,
    pub imm: bool_,
    pub mem_ref: bool_,
    pub segment: c_int,
    pub multi_regs: bool_,
}

#[repr(C)]
pub struct annotated_insn_loc {
    pub ops: [annotated_op_loc; 2],
}

#[repr(C)]
pub struct annotated_line {
    pub offset: u64,
}

#[repr(C)]
pub struct ins_name {
    pub name: *const c_char,
}

#[repr(C)]
pub struct target_op {
    pub sym: *mut symbol,
    pub name: *const c_char,
}

#[repr(C)]
pub struct disasm_ops {
    pub target: target_op,
}

#[repr(C)]
pub struct disasm_line {
    pub al: annotated_line,
    pub ins: ins_name,
    pub ops: disasm_ops,
}

unsafe extern "C" {
    static mov_ops: ins_ops;
    static call_ops: ins_ops;
    static dec_ops: ins_ops;
    static jump_ops: ins_ops;
    static lock_ops: ins_ops;
    static nop_ops: ins_ops;
    static ret_ops: ins_ops;
    static mut errno: c_int;

    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn zalloc(size: size_t) -> *mut c_void;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool_;
    fn annotate_get_insn_location(
        arch: *const arch,
        dl: *mut disasm_line,
        loc: *mut annotated_insn_loc,
    ) -> c_int;
    fn ins__is_call(ins: *mut ins_name) -> bool_;
    fn map__rip_2objdump(map: *mut map, rip: u64) -> u64;
    fn die_find_func_rettype(cu_die: *mut Dwarf_Die, name: *const c_char, die: *mut Dwarf_Die)
        -> bool_;
    fn pr_debug_dtp(format: *const c_char, ...);
    fn pr_debug_type_name(die: *mut Dwarf_Die, kind: c_int);
    fn has_reg_type(state: *mut type_state, reg: c_int) -> bool_;
    fn dwarf_tag(die: *mut Dwarf_Die) -> c_int;
    fn annotate_calc_pcrel(ms: *mut map_symbol, ip: u64, offset: u64, dl: *mut disasm_line) -> u64;
    fn get_global_var_info(
        dloc: *mut data_loc_info,
        addr: s64,
        name: *mut *const c_char,
        offset: *mut c_int,
    ) -> bool_;
    fn get_global_var_type(
        cu_die: *mut Dwarf_Die,
        dloc: *mut data_loc_info,
        ip: u64,
        addr: u64,
        offset: *mut c_int,
        type_die: *mut Dwarf_Die,
    ) -> bool_;
    fn __die_get_real_type(die: *mut Dwarf_Die, type_die: *mut Dwarf_Die) -> *mut Dwarf_Die;
    fn die_get_member_type(die: *mut Dwarf_Die, offset: u64, type_die: *mut Dwarf_Die)
        -> *mut Dwarf_Die;
    fn find_stack_state(state: *mut type_state, offset: c_int) -> *mut type_state_stack;
    fn die_get_cfa(dbg: *mut c_void, pc: u64, fbreg: *mut c_int, fboff: *mut c_int) -> c_int;
    fn dso__kernel(dso: *mut c_void) -> bool_;
    fn map__dso(map: *mut map) -> *mut c_void;
    fn die_deref_ptr_type(die: *mut Dwarf_Die, offset: u64, type_die: *mut Dwarf_Die) -> bool_;
    fn set_stack_state(
        stack: *mut type_state_stack,
        offset: c_int,
        kind: c_int,
        type_: *mut Dwarf_Die,
        ptr_offset: u64,
    );
    fn findnew_stack_state(
        state: *mut type_state,
        offset: c_int,
        kind: c_int,
        type_: *mut Dwarf_Die,
        ptr_offset: u64,
    ) -> *mut type_state_stack;
    fn assert_fail();
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

/*
 * x86 instruction nmemonic table to parse disasm lines for annotate.
 * This table is searched twice - one for exact match and another for
 * match without a size suffix (b, w, l, q) in case of AT&T syntax.
 *
 * So this table should not have entries with the suffix unless it's
 * a complete different instruction than ones without the suffix.
 */
static x86__instructions: [ins; 111] = unsafe {
    [
        ins { name: c!("adc"), ops: &mov_ops },
        ins { name: c!("add"), ops: &mov_ops },
        ins { name: c!("addsd"), ops: &mov_ops },
        ins { name: c!("and"), ops: &mov_ops },
        ins { name: c!("andpd"), ops: &mov_ops },
        ins { name: c!("andps"), ops: &mov_ops },
        ins { name: c!("bsr"), ops: &mov_ops },
        ins { name: c!("bt"), ops: &mov_ops },
        ins { name: c!("btr"), ops: &mov_ops },
        ins { name: c!("bts"), ops: &mov_ops },
        ins { name: c!("call"), ops: &call_ops },
        ins { name: c!("cmovae"), ops: &mov_ops },
        ins { name: c!("cmovbe"), ops: &mov_ops },
        ins { name: c!("cmove"), ops: &mov_ops },
        ins { name: c!("cmp"), ops: &mov_ops },
        ins { name: c!("cmpxch"), ops: &mov_ops },
        ins { name: c!("cmpxchg"), ops: &mov_ops },
        ins { name: c!("cs"), ops: &mov_ops },
        ins { name: c!("dec"), ops: &dec_ops },
        ins { name: c!("divsd"), ops: &mov_ops },
        ins { name: c!("divss"), ops: &mov_ops },
        ins { name: c!("gs"), ops: &mov_ops },
        ins { name: c!("imul"), ops: &mov_ops },
        ins { name: c!("inc"), ops: &dec_ops },
        ins { name: c!("ja"), ops: &jump_ops },
        ins { name: c!("jae"), ops: &jump_ops },
        ins { name: c!("jb"), ops: &jump_ops },
        ins { name: c!("jbe"), ops: &jump_ops },
        ins { name: c!("jc"), ops: &jump_ops },
        ins { name: c!("jcxz"), ops: &jump_ops },
        ins { name: c!("je"), ops: &jump_ops },
        ins { name: c!("jecxz"), ops: &jump_ops },
        ins { name: c!("jg"), ops: &jump_ops },
        ins { name: c!("jge"), ops: &jump_ops },
        ins { name: c!("jl"), ops: &jump_ops },
        ins { name: c!("jle"), ops: &jump_ops },
        ins { name: c!("jmp"), ops: &jump_ops },
        ins { name: c!("jna"), ops: &jump_ops },
        ins { name: c!("jnae"), ops: &jump_ops },
        ins { name: c!("jnb"), ops: &jump_ops },
        ins { name: c!("jnbe"), ops: &jump_ops },
        ins { name: c!("jnc"), ops: &jump_ops },
        ins { name: c!("jne"), ops: &jump_ops },
        ins { name: c!("jng"), ops: &jump_ops },
        ins { name: c!("jnge"), ops: &jump_ops },
        ins { name: c!("jnl"), ops: &jump_ops },
        ins { name: c!("jnle"), ops: &jump_ops },
        ins { name: c!("jno"), ops: &jump_ops },
        ins { name: c!("jnp"), ops: &jump_ops },
        ins { name: c!("jns"), ops: &jump_ops },
        ins { name: c!("jnz"), ops: &jump_ops },
        ins { name: c!("jo"), ops: &jump_ops },
        ins { name: c!("jp"), ops: &jump_ops },
        ins { name: c!("jpe"), ops: &jump_ops },
        ins { name: c!("jpo"), ops: &jump_ops },
        ins { name: c!("jrcxz"), ops: &jump_ops },
        ins { name: c!("js"), ops: &jump_ops },
        ins { name: c!("jz"), ops: &jump_ops },
        ins { name: c!("lea"), ops: &mov_ops },
        ins { name: c!("lock"), ops: &lock_ops },
        ins { name: c!("mov"), ops: &mov_ops },
        ins { name: c!("movapd"), ops: &mov_ops },
        ins { name: c!("movaps"), ops: &mov_ops },
        ins { name: c!("movdqa"), ops: &mov_ops },
        ins { name: c!("movdqu"), ops: &mov_ops },
        ins { name: c!("movsb"), ops: &mov_ops },
        ins { name: c!("movsd"), ops: &mov_ops },
        ins { name: c!("movsl"), ops: &mov_ops },
        ins { name: c!("movss"), ops: &mov_ops },
        ins { name: c!("movsw"), ops: &mov_ops },
        ins { name: c!("movupd"), ops: &mov_ops },
        ins { name: c!("movups"), ops: &mov_ops },
        ins { name: c!("movzb"), ops: &mov_ops },
        ins { name: c!("movzl"), ops: &mov_ops },
        ins { name: c!("movzw"), ops: &mov_ops },
        ins { name: c!("mulsd"), ops: &mov_ops },
        ins { name: c!("mulss"), ops: &mov_ops },
        ins { name: c!("nop"), ops: &nop_ops },
        ins { name: c!("or"), ops: &mov_ops },
        ins { name: c!("orps"), ops: &mov_ops },
        ins { name: c!("paddq"), ops: &mov_ops },
        ins { name: c!("pand"), ops: &mov_ops },
        ins { name: c!("pcmpeqb"), ops: &mov_ops },
        ins { name: c!("por"), ops: &mov_ops },
        ins { name: c!("rcl"), ops: &mov_ops },
        ins { name: c!("ret"), ops: &ret_ops },
        ins { name: c!("sbb"), ops: &mov_ops },
        ins { name: c!("sete"), ops: &mov_ops },
        ins { name: c!("sub"), ops: &mov_ops },
        ins { name: c!("subsd"), ops: &mov_ops },
        ins { name: c!("test"), ops: &mov_ops },
        ins { name: c!("tzcnt"), ops: &mov_ops },
        ins { name: c!("ucomisd"), ops: &mov_ops },
        ins { name: c!("ucomiss"), ops: &mov_ops },
        ins { name: c!("vaddsd"), ops: &mov_ops },
        ins { name: c!("vandpd"), ops: &mov_ops },
        ins { name: c!("vmovdqa"), ops: &mov_ops },
        ins { name: c!("vmovq"), ops: &mov_ops },
        ins { name: c!("vmovsd"), ops: &mov_ops },
        ins { name: c!("vmulsd"), ops: &mov_ops },
        ins { name: c!("vorpd"), ops: &mov_ops },
        ins { name: c!("vsubsd"), ops: &mov_ops },
        ins { name: c!("vucomisd"), ops: &mov_ops },
        ins { name: c!("xadd"), ops: &mov_ops },
        ins { name: c!("xbegin"), ops: &jump_ops },
        ins { name: c!("xchg"), ops: &mov_ops },
        ins { name: c!("xor"), ops: &mov_ops },
        ins { name: c!("xorpd"), ops: &mov_ops },
        ins { name: c!("xorps"), ops: &mov_ops },
    ]
};

unsafe extern "C" fn amd__ins_is_fused(
    arch: *const arch,
    ins1: *const c_char,
    ins2: *const c_char,
) -> bool_ {
    if !strstr(ins2, c!("jmp")).is_null() {
        return false;
    }

    /* Family >= 15h supports cmp/test + branch fusion */
    if (*arch).family >= 0x15
        && (strstarts(ins1, c!("test"))
            || (strstarts(ins1, c!("cmp")) && strstr(ins1, c!("xchg")).is_null()))
    {
        return true;
    }

    /* Family >= 19h supports some ALU + branch fusion */
    if (*arch).family >= 0x19
        && (strstarts(ins1, c!("add"))
            || strstarts(ins1, c!("sub"))
            || strstarts(ins1, c!("and"))
            || strstarts(ins1, c!("inc"))
            || strstarts(ins1, c!("dec"))
            || strstarts(ins1, c!("or"))
            || strstarts(ins1, c!("xor")))
    {
        return true;
    }

    false
}

unsafe extern "C" fn intel__ins_is_fused(
    arch: *const arch,
    ins1: *const c_char,
    ins2: *const c_char,
) -> bool_ {
    if (*arch).family != 6 || (*arch).model < 0x1e || !strstr(ins2, c!("jmp")).is_null() {
        return false;
    }

    if (*arch).model == 0x1e {
        /* Nehalem */
        if (!strstr(ins1, c!("cmp")).is_null() && strstr(ins1, c!("xchg")).is_null())
            || !strstr(ins1, c!("test")).is_null()
        {
            return true;
        }
    } else {
        /* Newer platform */
        if (!strstr(ins1, c!("cmp")).is_null() && strstr(ins1, c!("xchg")).is_null())
            || !strstr(ins1, c!("test")).is_null()
            || !strstr(ins1, c!("add")).is_null()
            || !strstr(ins1, c!("sub")).is_null()
            || !strstr(ins1, c!("and")).is_null()
            || !strstr(ins1, c!("inc")).is_null()
            || !strstr(ins1, c!("dec")).is_null()
        {
            return true;
        }
    }

    false
}

unsafe fn x86__cpuid_parse(arch: *mut arch, cpuid: *const c_char) -> c_int {
    let mut family: c_uint = 0;
    let mut model: c_uint = 0;
    let mut stepping: c_uint = 0;

    /*
     * cpuid = "GenuineIntel,family,model,stepping"
     */
    let ret = sscanf(
        cpuid,
        c!("%*[^,],%u,%u,%u"),
        &mut family,
        &mut model,
        &mut stepping,
    );
    if ret == 3 {
        (*arch).family = family;
        (*arch).model = model;
        (*arch).ins_is_fused = if strstarts(cpuid, c!("AuthenticAMD")) {
            Some(amd__ins_is_fused)
        } else {
            Some(intel__ins_is_fused)
        };
        return 0;
    }

    -1
}

// Original C code is guarded by HAVE_LIBDW_SUPPORT.
unsafe fn invalidate_reg_state(reg: *mut type_state_reg) {
    (*reg).kind = TSR_KIND_INVALID;
    (*reg).ok = false;
    (*reg).lifetime_active = false;
    (*reg).lifetime_end = 0;
    (*reg).copied_from = -1;
}

// Original C code is guarded by HAVE_LIBDW_SUPPORT.
unsafe extern "C" fn update_insn_state_x86(
    state: *mut type_state,
    dloc: *mut data_loc_info,
    cu_die: *mut Dwarf_Die,
    dl: *mut disasm_line,
) {
    let mut loc: annotated_insn_loc = core::mem::zeroed();
    let src = &mut loc.ops[INSN_OP_SOURCE] as *mut annotated_op_loc;
    let dst = &mut loc.ops[INSN_OP_TARGET] as *mut annotated_op_loc;
    let mut type_die: Dwarf_Die = core::mem::zeroed();
    let insn_offset: u32 = (*dl).al.offset as u32;
    let mut fbreg = (*dloc).fbreg;
    let mut fboff: c_int = 0;

    if annotate_get_insn_location((*dloc).arch, dl, &mut loc) < 0 {
        return;
    }

    if ins__is_call(&mut (*dl).ins) {
        let func = (*dl).ops.target.sym;
        let call_name: *const c_char;
        let call_addr: u64;

        /* Try to resolve the call target name */
        if !func.is_null() {
            call_name = (*func).name;
        } else {
            call_name = (*dl).ops.target.name;
        }

        /* __fentry__ will preserve all registers */
        if !call_name.is_null() && strcmp(call_name, c!("__fentry__")) == 0 {
            return;
        }

        if !call_name.is_null() {
            pr_debug_dtp(c!("call [%x] %s\n"), insn_offset, call_name);
        } else {
            pr_debug_dtp(c!("call [%x] <unknown>\n"), insn_offset);
        }

        /* Invalidate caller-saved registers after call */
        call_addr = map__rip_2objdump(
            (*(*dloc).ms).map,
            (*(*(*dloc).ms).sym).start + (*dl).al.offset,
        );
        let mut i: usize = 0;
        while i < (*state).regs.len() {
            let reg = (*state).regs.as_mut_ptr().add(i);

            if !(*reg).caller_saved {
                i += 1;
                continue;
            }
            /* Keep register valid within DWARF location lifetime */
            if (*reg).lifetime_active && call_addr < (*reg).lifetime_end {
                i += 1;
                continue;
            }
            invalidate_reg_state(reg);
            i += 1;
        }

        /* Update register with the return type (if any) */
        if !call_name.is_null() && die_find_func_rettype(cu_die, call_name, &mut type_die) {
            let tsr = (*state).regs.as_mut_ptr().add((*state).ret_reg as usize);
            (*tsr).type_ = type_die;
            (*tsr).kind = TSR_KIND_TYPE;
            (*tsr).offset = 0;
            (*tsr).ok = true;

            pr_debug_dtp(c!("call [%x] return -> reg%d"), insn_offset, (*state).ret_reg);
            pr_debug_type_name(&mut type_die, (*tsr).kind);
        }
        return;
    }

    if strncmp((*dl).ins.name, c!("add"), 3) == 0 {
        let mut imm_value: u64 = !0u64;
        let mut offset: c_int = 0;
        let mut var_name: *const c_char = ptr::null();
        let ms = (*dloc).ms;
        let ip = (*(*ms).sym).start + (*dl).al.offset;

        if !has_reg_type(state, (*dst).reg1) {
            return;
        }

        let tsr = (*state).regs.as_mut_ptr().add((*dst).reg1 as usize);
        (*tsr).copied_from = -1;
        (*tsr).lifetime_active = false;
        (*tsr).lifetime_end = 0;

        if (*src).imm {
            imm_value = (*src).offset;
        } else if has_reg_type(state, (*src).reg1)
            && (*(*state).regs.as_ptr().add((*src).reg1 as usize)).kind == TSR_KIND_CONST
        {
            imm_value = (*(*state).regs.as_ptr().add((*src).reg1 as usize)).imm_value;
        } else if (*src).reg1 == DWARF_REG_PC {
            let var_addr = annotate_calc_pcrel((*dloc).ms, ip, (*src).offset, dl);

            if get_global_var_info(dloc, var_addr as s64, &mut var_name, &mut offset)
                && strcmp(var_name, c!("this_cpu_off")) == 0
                && (*tsr).kind == TSR_KIND_CONST
            {
                (*tsr).kind = TSR_KIND_PERCPU_BASE;
                (*tsr).offset = 0;
                (*tsr).ok = true;
                imm_value = (*tsr).imm_value;
            }
        } else {
            return;
        }

        /* Ignore add to non-pointer or non-const types */
        if (*tsr).kind == TSR_KIND_POINTER
            || (dwarf_tag(&mut (*tsr).type_) == DW_TAG_pointer_type
                && (*src).reg1 != DWARF_REG_PC
                && (*tsr).kind == TSR_KIND_TYPE
                && !(*dst).mem_ref)
        {
            (*tsr).offset = (*tsr).offset.wrapping_add(imm_value);
            pr_debug_dtp(c!("add [%x] offset %#lx to reg%d"), insn_offset, imm_value, (*dst).reg1);
            pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
        }

        if (*tsr).kind == TSR_KIND_CONST {
            (*tsr).imm_value = (*tsr).imm_value.wrapping_add(imm_value);
        }

        if (*tsr).kind != TSR_KIND_PERCPU_BASE {
            return;
        }

        if get_global_var_type(cu_die, dloc, ip, imm_value, &mut offset, &mut type_die)
            && offset == 0
        {
            /*
             * This is not a pointer type, but it should be treated
             * as a pointer.
             */
            (*tsr).type_ = type_die;
            (*tsr).kind = TSR_KIND_PERCPU_POINTER;
            (*tsr).offset = 0;
            (*tsr).ok = true;

            pr_debug_dtp(c!("add [%x] percpu %#lx -> reg%d"), insn_offset, imm_value, (*dst).reg1);
            pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
        }
        return;
    }

    if strncmp((*dl).ins.name, c!("sub"), 3) == 0 {
        let mut imm_value: u64 = !0u64;

        if !has_reg_type(state, (*dst).reg1) {
            return;
        }

        let tsr = (*state).regs.as_mut_ptr().add((*dst).reg1 as usize);
        (*tsr).copied_from = -1;
        (*tsr).lifetime_active = false;
        (*tsr).lifetime_end = 0;

        if (*src).imm {
            imm_value = (*src).offset;
        } else if has_reg_type(state, (*src).reg1)
            && (*(*state).regs.as_ptr().add((*src).reg1 as usize)).kind == TSR_KIND_CONST
        {
            imm_value = (*(*state).regs.as_ptr().add((*src).reg1 as usize)).imm_value;
        }

        if (*tsr).kind == TSR_KIND_POINTER
            || (dwarf_tag(&mut (*tsr).type_) == DW_TAG_pointer_type
                && (*src).reg1 != DWARF_REG_PC
                && (*tsr).kind == TSR_KIND_TYPE
                && !(*dst).mem_ref)
        {
            (*tsr).offset = (*tsr).offset.wrapping_sub(imm_value);
            pr_debug_dtp(c!("sub [%x] offset %#lx to reg%d"), insn_offset, imm_value, (*dst).reg1);
            pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
        }

        if (*tsr).kind == TSR_KIND_CONST {
            (*tsr).imm_value = (*tsr).imm_value.wrapping_sub(imm_value);
        }

        return;
    }

    if strncmp((*dl).ins.name, c!("lea"), 3) == 0 {
        let sreg = (*src).reg1;

        if !has_reg_type(state, sreg) || !has_reg_type(state, (*dst).reg1) || !(*src).mem_ref {
            return;
        }

        let src_tsr = ptr::read((*state).regs.as_ptr().add(sreg as usize));
        let tsr = (*state).regs.as_mut_ptr().add((*dst).reg1 as usize);

        invalidate_reg_state(tsr);

        /* Case 1: Based on stack pointer or frame pointer */
        if sreg == fbreg || sreg == (*state).stack_reg {
            let offset = ((*src).offset as c_int).wrapping_sub(fboff);
            let stack = find_stack_state(state, offset);
            if stack.is_null() {
                return;
            }

            (*tsr).type_ = (*stack).type_;
            (*tsr).kind = TSR_KIND_POINTER;
            (*tsr).offset = (offset - (*stack).offset) as u64;
            (*tsr).ok = true;

            if sreg == fbreg {
                pr_debug_dtp(c!("lea [%x] address of -%#x(stack) -> reg%d"), insn_offset, -((*src).offset as c_int), (*dst).reg1);
            } else {
                pr_debug_dtp(c!("lea [%x] address of %#x(reg%d) -> reg%d"), insn_offset, (*src).offset as c_int, sreg, (*dst).reg1);
            }

            pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
        }
        /* Case 2: Based on a register holding a typed pointer */
        else if src_tsr.ok
            && (src_tsr.kind == TSR_KIND_POINTER
                || (dwarf_tag(&mut (*( &src_tsr as *const _ as *mut type_state_reg)).type_) == DW_TAG_pointer_type
                    && src_tsr.kind == TSR_KIND_TYPE))
        {
            if src_tsr.kind == TSR_KIND_TYPE
                && __die_get_real_type(&mut (*(*state).regs.as_mut_ptr().add(sreg as usize)).type_, &mut type_die).is_null()
            {
                return;
            }

            if src_tsr.kind == TSR_KIND_POINTER {
                type_die = (*(*state).regs.as_ptr().add(sreg as usize)).type_;
            }

            /* Check if the target type has a member at the new offset */
            if die_get_member_type(&mut type_die, (*src).offset.wrapping_add(src_tsr.offset), &mut type_die).is_null() {
                return;
            }

            (*tsr).type_ = src_tsr.type_;
            (*tsr).kind = src_tsr.kind;
            (*tsr).offset = (*src).offset.wrapping_add(src_tsr.offset);
            (*tsr).ok = true;

            pr_debug_dtp(c!("lea [%x] address of %s%#x(reg%d) -> reg%d"), insn_offset, if ((*src).offset as c_int) < 0 { c!("-") } else { c!("") }, ((*src).offset as c_int).abs(), sreg, (*dst).reg1);

            pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
        }
        return;
    }

    /* Invalidate register states for other ops which may change pointers */
    if has_reg_type(state, (*dst).reg1)
        && !(*dst).mem_ref
        && dwarf_tag(&mut (*(*state).regs.as_mut_ptr().add((*dst).reg1 as usize)).type_)
            == DW_TAG_pointer_type
    {
        if strncmp((*dl).ins.name, c!("imul"), 4) == 0
            || strncmp((*dl).ins.name, c!("mul"), 3) == 0
            || strncmp((*dl).ins.name, c!("idiv"), 4) == 0
            || strncmp((*dl).ins.name, c!("div"), 3) == 0
            || strncmp((*dl).ins.name, c!("shl"), 3) == 0
            || strncmp((*dl).ins.name, c!("shr"), 3) == 0
            || strncmp((*dl).ins.name, c!("sar"), 3) == 0
            || strncmp((*dl).ins.name, c!("and"), 3) == 0
            || strncmp((*dl).ins.name, c!("or"), 2) == 0
            || strncmp((*dl).ins.name, c!("neg"), 3) == 0
            || strncmp((*dl).ins.name, c!("inc"), 3) == 0
            || strncmp((*dl).ins.name, c!("dec"), 3) == 0
        {
            pr_debug_dtp(c!("%s [%x] invalidate reg%d\n"), (*dl).ins.name, insn_offset, (*dst).reg1);
            invalidate_reg_state((*state).regs.as_mut_ptr().add((*dst).reg1 as usize));
            return;
        }

        if strncmp((*dl).ins.name, c!("xor"), 3) == 0 && (*dst).reg1 == (*src).reg1 {
            /* xor reg, reg clears the register */
            pr_debug_dtp(c!("xor [%x] clear reg%d\n"), insn_offset, (*dst).reg1);

            let reg = (*state).regs.as_mut_ptr().add((*dst).reg1 as usize);
            (*reg).kind = TSR_KIND_CONST;
            (*reg).imm_value = 0;
            (*reg).ok = true;
            (*reg).lifetime_active = false;
            (*reg).lifetime_end = 0;
            (*reg).copied_from = -1;
            return;
        }
    }

    if strncmp((*dl).ins.name, c!("mov"), 3) != 0 {
        return;
    }

    if (*dloc).fb_cfa {
        let ip = (*(*(*dloc).ms).sym).start + (*dl).al.offset;
        let pc = map__rip_2objdump((*(*dloc).ms).map, ip);

        if die_get_cfa((*(*dloc).di).dbg, pc, &mut fbreg, &mut fboff) < 0 {
            fbreg = -1;
        }
    }

    /* Case 1. register to register or segment:offset to register transfers */
    if !(*src).mem_ref && !(*dst).mem_ref {
        if !has_reg_type(state, (*dst).reg1) {
            return;
        }

        let tsr = (*state).regs.as_mut_ptr().add((*dst).reg1 as usize);
        (*tsr).copied_from = -1;

        if dso__kernel(map__dso((*(*dloc).ms).map))
            && (*src).segment == INSN_SEG_X86_GS
            && (*src).imm
        {
            let ip = (*(*(*dloc).ms).sym).start + (*dl).al.offset;
            let var_addr = (*src).offset;
            let mut offset: c_int = 0;

            /*
             * In kernel, %gs points to a per-cpu region for the
             * current CPU.  Access with a constant offset should
             * be treated as a global variable access.
             */
            if var_addr == 40 {
                (*tsr).kind = TSR_KIND_CANARY;
                (*tsr).offset = 0;
                (*tsr).ok = true;

                pr_debug_dtp(c!("mov [%x] stack canary -> reg%d\n"), insn_offset, (*dst).reg1);
                return;
            }

            if !get_global_var_type(cu_die, dloc, ip, var_addr, &mut offset, &mut type_die)
                || die_get_member_type(&mut type_die, offset as u64, &mut type_die).is_null()
            {
                invalidate_reg_state(tsr);
                return;
            }

            (*tsr).type_ = type_die;
            (*tsr).kind = TSR_KIND_TYPE;
            (*tsr).offset = 0;
            (*tsr).ok = true;

            pr_debug_dtp(c!("mov [%x] this-cpu addr=%#lx -> reg%d"), insn_offset, var_addr, (*dst).reg1);
            pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
            return;
        }

        if (*src).imm {
            (*tsr).kind = TSR_KIND_CONST;
            (*tsr).imm_value = (*src).offset;
            (*tsr).offset = 0;
            (*tsr).ok = true;

            pr_debug_dtp(c!("mov [%x] imm=%#x -> reg%d\n"), insn_offset, (*tsr).imm_value, (*dst).reg1);
            return;
        }

        if !has_reg_type(state, (*src).reg1)
            || !(*(*state).regs.as_ptr().add((*src).reg1 as usize)).ok
        {
            invalidate_reg_state(tsr);
            return;
        }

        let src_reg = (*state).regs.as_ptr().add((*src).reg1 as usize);
        (*tsr).type_ = (*src_reg).type_;
        (*tsr).kind = (*src_reg).kind;
        (*tsr).imm_value = (*src_reg).imm_value;
        (*tsr).offset = (*src_reg).offset;
        (*tsr).lifetime_active = (*src_reg).lifetime_active;
        (*tsr).lifetime_end = (*src_reg).lifetime_end;
        (*tsr).ok = true;

        /* To copy back the variable type later (hopefully) */
        if (*tsr).kind == TSR_KIND_TYPE || (*tsr).kind == TSR_KIND_POINTER {
            (*tsr).copied_from = (*src).reg1;
        }

        pr_debug_dtp(c!("mov [%x] reg%d -> reg%d"), insn_offset, (*src).reg1, (*dst).reg1);
        pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
    }

    /* Case 2. memory to register transers */
    if (*src).mem_ref && !(*dst).mem_ref {
        let mut sreg = (*src).reg1;

        if !has_reg_type(state, (*dst).reg1) {
            return;
        }

        let tsr = (*state).regs.as_mut_ptr().add((*dst).reg1 as usize);
        (*tsr).copied_from = -1;

        loop {
            /* Check stack variables with offset */
            if sreg == fbreg || sreg == (*state).stack_reg {
                let offset = ((*src).offset as c_int).wrapping_sub(fboff);
                let stack = find_stack_state(state, offset);
                if stack.is_null() {
                    invalidate_reg_state(tsr);
                    return;
                } else if !(*stack).compound {
                    (*tsr).type_ = (*stack).type_;
                    (*tsr).kind = (*stack).kind;
                    (*tsr).offset = (*stack).ptr_offset;
                    (*tsr).ok = true;
                } else if !die_get_member_type(&mut (*stack).type_, (offset - (*stack).offset) as u64, &mut type_die).is_null() {
                    (*tsr).type_ = type_die;
                    (*tsr).kind = TSR_KIND_TYPE;
                    (*tsr).offset = 0;
                    (*tsr).ok = true;
                } else {
                    invalidate_reg_state(tsr);
                    return;
                }

                if sreg == fbreg {
                    pr_debug_dtp(c!("mov [%x] -%#x(stack) -> reg%d"), insn_offset, -offset, (*dst).reg1);
                } else {
                    pr_debug_dtp(c!("mov [%x] %#x(reg%d) -> reg%d"), insn_offset, offset, sreg, (*dst).reg1);
                }
                pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
                break;
            }
            /* And then dereference the pointer if it has one */
            else if has_reg_type(state, sreg)
                && (*(*state).regs.as_ptr().add(sreg as usize)).ok
                && (*(*state).regs.as_ptr().add(sreg as usize)).kind == TSR_KIND_TYPE
                && die_deref_ptr_type(&mut (*(*state).regs.as_mut_ptr().add(sreg as usize)).type_, (*src).offset.wrapping_add((*(*state).regs.as_ptr().add(sreg as usize)).offset), &mut type_die)
            {
                (*tsr).type_ = type_die;
                (*tsr).kind = TSR_KIND_TYPE;
                (*tsr).offset = 0;
                (*tsr).ok = true;

                pr_debug_dtp(c!("mov [%x] %#x(reg%d) -> reg%d"), insn_offset, (*src).offset as c_int, sreg, (*dst).reg1);
                pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
                break;
            }
            /* Handle dereference of TSR_KIND_POINTER registers */
            else if has_reg_type(state, sreg)
                && (*(*state).regs.as_ptr().add(sreg as usize)).ok
                && (*(*state).regs.as_ptr().add(sreg as usize)).kind == TSR_KIND_POINTER
                && !die_get_member_type(&mut (*(*state).regs.as_mut_ptr().add(sreg as usize)).type_, (*src).offset.wrapping_add((*(*state).regs.as_ptr().add(sreg as usize)).offset), &mut type_die).is_null()
            {
                (*tsr).type_ = (*(*state).regs.as_ptr().add(sreg as usize)).type_;
                (*tsr).kind = TSR_KIND_TYPE;
                (*tsr).offset = (*src).offset.wrapping_add((*(*state).regs.as_ptr().add(sreg as usize)).offset);
                (*tsr).ok = true;

                pr_debug_dtp(c!("mov [%x] addr %#x(reg%d) -> reg%d"), insn_offset, (*src).offset as c_int, sreg, (*dst).reg1);
                pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
                break;
            }
            /* Or check if it's a global variable */
            else if sreg == DWARF_REG_PC {
                let ms = (*dloc).ms;
                let ip = (*(*ms).sym).start + (*dl).al.offset;
                let mut offset: c_int = 0;

                let addr = annotate_calc_pcrel(ms, ip, (*src).offset, dl);

                if !get_global_var_type(cu_die, dloc, ip, addr, &mut offset, &mut type_die)
                    || die_get_member_type(&mut type_die, offset as u64, &mut type_die).is_null()
                {
                    invalidate_reg_state(tsr);
                    return;
                }

                (*tsr).type_ = type_die;
                (*tsr).kind = TSR_KIND_TYPE;
                (*tsr).offset = 0;
                (*tsr).ok = true;

                pr_debug_dtp(c!("mov [%x] global addr=%lx -> reg%d"), insn_offset, addr, (*dst).reg1);
                pr_debug_type_name(&mut type_die, (*tsr).kind);
                break;
            }
            /* And check percpu access with base register */
            else if has_reg_type(state, sreg)
                && (*(*state).regs.as_ptr().add(sreg as usize)).kind == TSR_KIND_PERCPU_BASE
            {
                let ip = (*(*(*dloc).ms).sym).start + (*dl).al.offset;
                let mut var_addr = (*src).offset;
                let mut offset: c_int = 0;

                if (*src).multi_regs {
                    let reg2 = if sreg == (*src).reg1 { (*src).reg2 } else { (*src).reg1 };

                    if has_reg_type(state, reg2)
                        && (*(*state).regs.as_ptr().add(reg2 as usize)).ok
                        && (*(*state).regs.as_ptr().add(reg2 as usize)).kind == TSR_KIND_CONST
                    {
                        var_addr = var_addr.wrapping_add((*(*state).regs.as_ptr().add(reg2 as usize)).imm_value);
                    }
                }

                /*
                 * In kernel, %gs points to a per-cpu region for the
                 * current CPU.  Access with a constant offset should
                 * be treated as a global variable access.
                 */
                if get_global_var_type(cu_die, dloc, ip, var_addr, &mut offset, &mut type_die)
                    && !die_get_member_type(&mut type_die, offset as u64, &mut type_die).is_null()
                {
                    (*tsr).type_ = type_die;
                    (*tsr).kind = TSR_KIND_TYPE;
                    (*tsr).offset = 0;
                    (*tsr).ok = true;

                    if (*src).multi_regs {
                        pr_debug_dtp(c!("mov [%x] percpu %#x(reg%d,reg%d) -> reg%d"), insn_offset, (*src).offset as c_int, (*src).reg1, (*src).reg2, (*dst).reg1);
                    } else {
                        pr_debug_dtp(c!("mov [%x] percpu %#x(reg%d) -> reg%d"), insn_offset, (*src).offset as c_int, sreg, (*dst).reg1);
                    }
                    pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
                } else {
                    invalidate_reg_state(tsr);
                }
                break;
            }
            /* And then dereference the calculated pointer if it has one */
            else if has_reg_type(state, sreg)
                && (*(*state).regs.as_ptr().add(sreg as usize)).ok
                && (*(*state).regs.as_ptr().add(sreg as usize)).kind == TSR_KIND_PERCPU_POINTER
                && !die_get_member_type(&mut (*(*state).regs.as_mut_ptr().add(sreg as usize)).type_, (*src).offset, &mut type_die).is_null()
            {
                (*tsr).type_ = type_die;
                (*tsr).kind = TSR_KIND_TYPE;
                (*tsr).offset = 0;
                (*tsr).ok = true;

                pr_debug_dtp(c!("mov [%x] pointer %#x(reg%d) -> reg%d"), insn_offset, (*src).offset as c_int, sreg, (*dst).reg1);
                pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
                break;
            }
            /* Or try another register if any */
            else if (*src).multi_regs && sreg == (*src).reg1 && (*src).reg1 != (*src).reg2 {
                sreg = (*src).reg2;
                continue;
            } else {
                let mut offset: c_int = 0;
                let mut var_name: *const c_char = ptr::null();

                /* it might be per-cpu variable (in kernel) access */
                if ((*src).offset as s64) < 0 {
                    if get_global_var_info(dloc, (*src).offset as s64, &mut var_name, &mut offset)
                        && strcmp(var_name, c!("__per_cpu_offset")) == 0
                    {
                        (*tsr).kind = TSR_KIND_PERCPU_BASE;
                        (*tsr).offset = 0;
                        (*tsr).ok = true;

                        pr_debug_dtp(c!("mov [%x] percpu base reg%d\n"), insn_offset, (*dst).reg1);
                        return;
                    }
                }

                invalidate_reg_state(tsr);
                break;
            }
        }
    }

    /* Case 3. register to memory transfers */
    if !(*src).mem_ref && (*dst).mem_ref {
        if !has_reg_type(state, (*src).reg1) || !(*(*state).regs.as_ptr().add((*src).reg1 as usize)).ok {
            return;
        }

        /* Check stack variables with offset */
        if (*dst).reg1 == fbreg || (*dst).reg1 == (*state).stack_reg {
            let offset = ((*dst).offset as c_int).wrapping_sub(fboff);

            let tsr = (*state).regs.as_mut_ptr().add((*src).reg1 as usize);

            let stack = find_stack_state(state, offset);
            if !stack.is_null() {
                /*
                 * The source register is likely to hold a type
                 * of member if it's a compound type.  Do not
                 * update the stack variable type since we can
                 * get the member type later by using the
                 * die_get_member_type().
                 */
                if !(*stack).compound {
                    set_stack_state(stack, offset, (*tsr).kind, &mut (*tsr).type_, (*tsr).offset);
                }
            } else {
                findnew_stack_state(state, offset, (*tsr).kind, &mut (*tsr).type_, (*tsr).offset);
            }

            if (*dst).reg1 == fbreg {
                pr_debug_dtp(c!("mov [%x] reg%d -> -%#x(stack)"), insn_offset, (*src).reg1, -offset);
            } else {
                pr_debug_dtp(c!("mov [%x] reg%d -> %#x(reg%d)"), insn_offset, (*src).reg1, offset, (*dst).reg1);
            }
            if (*tsr).offset != 0 {
                pr_debug_dtp(c!(" reg%d offset %#x ->"), (*src).reg1, (*tsr).offset);
            }

            pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
        }
        /*
         * Ignore other transfers since it'd set a value in a struct
         * and won't change the type.
         */
    }
    /* Case 4. memory to memory transfers (not handled for now) */
}

#[no_mangle]
pub unsafe extern "C" fn arch__new_x86(
    id: *const e_machine_and_e_flags,
    cpuid: *const c_char,
) -> *const arch {
    let arch = zalloc(size_of::<arch>()) as *mut arch;

    if arch.is_null() {
        return ptr::null();
    }

    (*arch).name = c!("x86");
    (*arch).id = *id;
    if !cpuid.is_null() {
        if x86__cpuid_parse(arch, cpuid) != 0 {
            errno = SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_CPUID_PARSING;
            return ptr::null();
        }
    }
    (*arch).instructions = x86__instructions.as_ptr();
    (*arch).nr_instructions = x86__instructions.len();

    #[cfg(debug_assertions)]
    {
        static mut sorted_check: bool = false;

        if !sorted_check {
            let mut i: usize = 0;
            while i < (*arch).nr_instructions - 1 {
                if strcmp(
                    (*(*arch).instructions.add(i)).name,
                    (*(*arch).instructions.add(i + 1)).name,
                ) > 0
                {
                    assert_fail();
                }
                i += 1;
            }
            sorted_check = true;
        }
    }

    (*arch).sorted_instructions = true;
    (*arch).objdump.comment_char = b'#' as c_char;
    (*arch).objdump.register_char = b'%' as c_char;
    (*arch).objdump.memory_ref_char = b'(' as c_char;
    (*arch).objdump.imm_char = b'$' as c_char;
    (*arch).insn_suffix = c!("bwlq");
    // Original C assignment is guarded by HAVE_LIBDW_SUPPORT.
    (*arch).update_insn_state = Some(update_insn_state_x86);
    arch
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
