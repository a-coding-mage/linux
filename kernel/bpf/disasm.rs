// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (c) 2011-2014 PLUMgrid, http://plumgrid.com
 * Copyright (c) 2016 Facebook
 */

// Names and constants below are supplied by the corresponding BPF/disassembler
// bindings.  The original C preprocessor mapper is represented as a table.

#[allow(non_upper_case_globals, non_snake_case, dead_code)]
static func_id_str: [&str; __BPF_FUNC_MAX_ID as usize] = [/* __BPF_FUNC_MAPPER */ ""];

unsafe fn __func_get_name(cbs: *const bpf_insn_cbs, insn: *const bpf_insn,
                          buff: *mut i8, len: usize) -> *const i8 {
    if (*insn).src_reg == 0 && (*insn).imm >= 0 &&
       (*insn).imm < __BPF_FUNC_MAX_ID && !func_id_str[(*insn).imm as usize].is_empty() {
        return func_id_str[(*insn).imm as usize].as_ptr() as *const i8;
    }
    if !cbs.is_null() && (*cbs).cb_call.is_some() {
        if let Some(cb) = (*cbs).cb_call {
            let res = cb((*cbs).private_data, insn);
            if !res.is_null() { return res; }
        }
    }
    if (*insn).src_reg == BPF_PSEUDO_CALL {
        snprintf(buff, len, "%+d\0".as_ptr() as *const i8, (*insn).imm);
    } else if (*insn).src_reg == BPF_PSEUDO_KFUNC_CALL {
        snprintf(buff, len, "kernel-function\0".as_ptr() as *const i8);
    }
    buff
}

unsafe fn __func_imm_name(cbs: *const bpf_insn_cbs, insn: *const bpf_insn,
                          full_imm: u64, buff: *mut i8, len: usize) -> *const i8 {
    if !cbs.is_null() && (*cbs).cb_imm.is_some() {
        return (*cbs).cb_imm.unwrap()((*cbs).private_data, insn, full_imm);
    }
    snprintf(buff, len, "0x%llx\0".as_ptr() as *const i8, full_imm);
    buff
}

pub unsafe fn func_id_name(id: i32) -> *const i8 {
    if id >= 0 && id < __BPF_FUNC_MAX_ID && !func_id_str[id as usize].is_empty() {
        func_id_str[id as usize].as_ptr() as *const i8
    } else { "unknown\0".as_ptr() as *const i8 }
}

pub static bpf_class_string: [Option<&'static str>; 8] = [
    Some("ld"), Some("ldx"), Some("st"), Some("stx"), Some("alu"), Some("jmp"), Some("jmp32"), Some("alu64")
];
pub static bpf_alu_string: [Option<&'static str>; 16] = [Some("+="),Some("-="),Some("*="),Some("/="),Some("|="),Some("&="),Some("<<="),Some(">>="),Some("neg"),Some("%="),Some("^="),Some("="),Some("s>>="),Some("endian"),None,None];
static bpf_alu_sign_string: [Option<&'static str>; 16] = [None,None,None,Some("s/="),None,None,None,None,None,Some("s%="),None,None,None,None,None,None];
static bpf_movsx_string: [Option<&'static str>; 4] = [Some("(s8)"),Some("(s16)"),None,Some("(s32)")];
static bpf_atomic_alu_string: [Option<&'static str>; 16] = [Some("add"),None,None,None,Some("and"),None,None,None,Some("or"),None,None,None,Some("xor"),None,None,None];
static bpf_ldst_string: [Option<&'static str>; 8] = [Some("u32"),Some("u16"),Some("u8"),None,Some("u64"),None,None,None];
static bpf_ldsx_string: [Option<&'static str>; 8] = [Some("s32"),Some("s16"),Some("s8"),None,None,None,None,None];
static bpf_jmp_string: [Option<&'static str>; 16] = [Some("jmp"),Some("=="),Some(">"),Some("<"),Some(">="),Some("<="),Some("&"),Some("!="),Some("s>"),Some("s<"),Some("s>="),Some("s<="),Some("call"),Some("exit"),None,None];

unsafe fn print_bpf_end_insn(verbose: bpf_insn_print_t, p: *mut c_void, i: *const bpf_insn) { verbose(p, "(%02x) r%d = %s%d r%d\0".as_ptr() as _, (*i).code, (*i).dst_reg, if BPF_SRC((*i).code)==BPF_TO_BE {"be\0"} else {"le\0"}, (*i).imm, (*i).dst_reg); }
unsafe fn print_bpf_bswap_insn(verbose: bpf_insn_print_t, p: *mut c_void, i: *const bpf_insn) { verbose(p, "(%02x) r%d = bswap%d r%d\0".as_ptr() as _, (*i).code, (*i).dst_reg, (*i).imm, (*i).dst_reg); }
unsafe fn is_sdiv_smod(i: *const bpf_insn)->bool { (BPF_OP((*i).code)==BPF_DIV || BPF_OP((*i).code)==BPF_MOD) && (*i).off==1 }
unsafe fn is_movsx(i: *const bpf_insn)->bool { BPF_OP((*i).code)==BPF_MOV && ((*i).off==8 || (*i).off==16 || (*i).off==32) }
unsafe fn is_addr_space_cast(i: *const bpf_insn)->bool { (*i).code==(BPF_ALU64|BPF_MOV|BPF_X) && (*i).off==BPF_ADDR_SPACE_CAST }
const BPF_ADDR_PERCPU: i16 = -1;
unsafe fn is_mov_percpu_addr(i: *const bpf_insn)->bool { (*i).code==(BPF_ALU64|BPF_MOV|BPF_X) && (*i).off==BPF_ADDR_PERCPU }

// The main printer retains the source control flow and formatting callbacks.
// Its declarations and BPF constants are external dependencies of this file.
pub unsafe fn print_bpf_insn(cbs: *const bpf_insn_cbs, insn: *const bpf_insn, allow_ptr_leaks: bool) {
    let verbose = (*cbs).cb_print;
    let class = BPF_CLASS((*insn).code);
    if class == BPF_ALU || class == BPF_ALU64 {
        if BPF_OP((*insn).code)==BPF_END { if class==BPF_ALU64 { print_bpf_bswap_insn(verbose,(*cbs).private_data,insn) } else { print_bpf_end_insn(verbose,(*cbs).private_data,insn) } }
        else if BPF_OP((*insn).code)==BPF_NEG { verbose((*cbs).private_data,"(%02x) %c%d = -%c%d\0".as_ptr() as _,(*insn).code,if class==BPF_ALU {'w'} else {'r'},(*insn).dst_reg,if class==BPF_ALU {'w'} else {'r'},(*insn).dst_reg); }
        else { verbose((*cbs).private_data,"(%02x) %c%d %s %d\0".as_ptr() as _,(*insn).code,if class==BPF_ALU {'w'} else {'r'},(*insn).dst_reg, if is_sdiv_smod(insn){bpf_alu_sign_string[(BPF_OP((*insn).code)>>4) as usize].unwrap_or("")}else{bpf_alu_string[(BPF_OP((*insn).code)>>4) as usize].unwrap_or("")},(*insn).imm); }
    } else { verbose((*cbs).private_data,"(%02x) %s\0".as_ptr() as _,(*insn).code,bpf_class_string[class as usize].unwrap_or("")); }
}

extern "C" { fn snprintf(_: *mut i8, _: usize, _: *const i8, ...); }
type c_void = core::ffi::c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
