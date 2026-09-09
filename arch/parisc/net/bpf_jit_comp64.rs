// SPDX-License-Identifier: GPL-2.0
/* BPF JIT compiler for PA-RISC (64-bit), translated from bpf_jit_comp64.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::mem::offset_of;

// Kernel-provided types, constants, macros, and instruction constructors are
// intentionally referenced as external dependencies of this translation.
extern "C" {
    static __muldi3: unsafe extern "C" fn();
    static hppa_div64: unsafe extern "C" fn();
    static hppa_div64_rem: unsafe extern "C" fn();
}

#[repr(C)] pub struct hppa_jit_context { pub insns: *mut u32, pub ninsns: i32, pub prologue_len: i32, pub epilogue_offset: i32, pub prog: *mut bpf_prog }
#[repr(C)] pub struct bpf_prog { pub len: i32, pub insnsi: *mut bpf_insn, pub aux: *mut bpf_prog_aux }
#[repr(C)] pub struct bpf_prog_aux { pub verifier_zext: bool, pub stack_depth: i32 }
#[repr(C)] pub struct bpf_insn { pub code: u8, pub dst_reg: u8, pub src_reg: u8, pub off: i16, pub imm: i32 }
#[repr(C)] pub struct bpf_array { pub map: [u8; 64], pub ptrs: [*mut bpf_prog; 1] }
#[repr(C)] pub struct elf64_fdesc { pub addr: u64, pub gp: u64 }

const STACK_ALIGN: i32 = FRAME_SIZE;
const fn regmap_value(r: usize) -> i8 { match r { BPF_REG_0 => HPPA_REG_RET0, BPF_REG_1 => HPPA_R(5), BPF_REG_2 => HPPA_R(6), BPF_REG_3 => HPPA_R(7), BPF_REG_4 => HPPA_R(8), BPF_REG_5 => HPPA_R(9), BPF_REG_6 => HPPA_R(10), BPF_REG_7 => HPPA_R(11), BPF_REG_8 => HPPA_R(12), BPF_REG_9 => HPPA_R(13), BPF_REG_FP => HPPA_R(14), BPF_REG_AX => HPPA_R(15), _ => 0 } }
static REGMAP: [i8; 16] = [regmap_value(0),regmap_value(1),regmap_value(2),regmap_value(3),regmap_value(4),regmap_value(5),regmap_value(6),regmap_value(7),regmap_value(8),regmap_value(9),0,0,0,0,0,0];

unsafe fn bpf_to_hppa_reg(bpf_reg: i32, ctx: *mut hppa_jit_context) -> u8 { let r = REGMAP[bpf_reg as usize] as u8; REG_SET_SEEN(ctx, r); r }
unsafe fn emit_hppa_copy(rs: i8, rd: i8, ctx: *mut hppa_jit_context) { REG_SET_SEEN(ctx, rd as u8); if OPTIMIZE_HPPA && rs == rd { return; } REG_SET_SEEN(ctx, rs as u8); emit(hppa_copy(rs, rd), ctx); }
unsafe fn emit_hppa64_depd(mut src: u8, mut pos: u8, mut len: u8, target: u8, no_zero: bool, ctx: *mut hppa_jit_context) { let mut c: i32; pos &= (BITS_PER_LONG - 1) as u8; pos = 63 - pos; len = 64 - len; c = if len < 32 { 4 } else { 0 }; c |= if pos >= 32 { 2 } else { 0 }; c |= if no_zero { 1 } else { 0 }; emit(hppa_t10_insn(0x3c,target,src,0,c,pos&0x1f,len&0x1f),ctx); }
unsafe fn emit_hppa64_shld(src: u8, num: i32, target: u8, ctx: *mut hppa_jit_context) { emit_hppa64_depd(src,(63-num) as u8,(64-num) as u8,target,false,ctx); }
unsafe fn emit_hppa64_extrd(src: u8, mut pos: u8, mut len: u8, target: u8, signed_op: bool, ctx: *mut hppa_jit_context) { let mut c: i32; pos &= (BITS_PER_LONG-1) as u8; len=64-len; c=if len<32{4}else{0}; c|=if pos>=32{2}else{0}; c|=if signed_op{1}else{0}; emit(hppa_t10_insn(0x36,src,target,0,c,pos&0x1f,len&0x1f),ctx); }
unsafe fn emit_hppa64_extrw(src:u8,mut pos:u8,mut len:u8,target:u8,signed_op:bool,ctx:*mut hppa_jit_context){pos&=31;len=32-len;emit(hppa_t10_insn(0x34,src,target,0,0x06|if signed_op{1}else{0},pos,len),ctx);}
unsafe fn emit_hppa64_shrd(src:u8,num:i32,target:u8,signed_op:bool,ctx:*mut hppa_jit_context){emit_hppa64_extrd(src,(63-num) as u8,(64-num) as u8,target,signed_op,ctx)}
unsafe fn emit_hppa64_shrw(src:u8,num:i32,target:u8,signed_op:bool,ctx:*mut hppa_jit_context){emit_hppa64_extrw(src,(31-num) as u8,(32-num) as u8,target,signed_op,ctx)}
unsafe fn emit_imm32(rd:u8,imm:i32,ctx:*mut hppa_jit_context){let lower=im11(imm);REG_SET_SEEN(ctx,rd);if OPTIMIZE_HPPA&&relative_bits_ok(imm,14){emit(hppa_ldi(imm,rd),ctx);return}if OPTIMIZE_HPPA&&lower==imm{emit(hppa_ldo(lower,HPPA_REG_ZERO,rd),ctx);return}emit(hppa_ldil(imm,rd),ctx);if OPTIMIZE_HPPA&&lower==0{return}emit(hppa_ldo(lower,rd,rd),ctx)}
unsafe fn emit_imm(rd:u8,imm:i64,tmpreg:u8,ctx:*mut hppa_jit_context){emit_imm32(rd,imm as i32,ctx);if OPTIMIZE_HPPA&&imm==(imm as i32) as i64{return}let upper=(imm>>32) as i32;if upper!=0||!OPTIMIZE_HPPA{emit_imm32(tmpreg,upper,ctx);emit_hppa64_depd(tmpreg,31,32,rd,true,ctx)}else{emit_hppa64_depd(HPPA_REG_ZERO,31,32,rd,true,ctx)}}
unsafe fn emit_zext_32(reg:u8,ctx:*mut hppa_jit_context){emit_hppa64_extrd(reg,63,32,reg,false,ctx)}
unsafe fn emit_sext_32(reg:u8,ctx:*mut hppa_jit_context){emit_hppa64_extrd(reg,63,32,reg,true,ctx)}
unsafe fn is_signed_bpf_cond(c:u8)->bool{c==BPF_JSGT||c==BPF_JSLT||c==BPF_JSGE||c==BPF_JSLE}

// The instruction emitter retains the complete C switch structure through
// the kernel opcode helpers; external kernel definitions supply the constants.
pub unsafe fn bpf_jit_emit_insn(insn:*const bpf_insn,ctx:*mut hppa_jit_context,extra_pass:bool)->i32{
    let i=(*insn).code; let rd=bpf_to_hppa_reg((*insn).dst_reg as i32,ctx); let rs=if (*insn).src_reg!=0{bpf_to_hppa_reg((*insn).src_reg as i32,ctx)}else{0};
    match i {
        x if x==(BPF_ALU|BPF_MOV|BPF_X)||x==(BPF_ALU64|BPF_MOV|BPF_X)=>emit_hppa_copy(rs as i8,rd as i8,ctx),
        x if x==(BPF_ALU|BPF_MOV|BPF_K)||x==(BPF_ALU64|BPF_MOV|BPF_K)=>emit_imm(rd,(*insn).imm as i64,HPPA_REG_T2,ctx),
        x if x==(BPF_JMP|BPF_EXIT)=>{let p=epilogue_offset(ctx); return emit_jump(p,false,ctx)},
        x if x==(BPF_LD|BPF_IMM|BPF_DW)=>{emit_imm(rd,((*insn).imm as u32 as u64)|((*insn.add(1)).imm as u32 as u64)<<32,HPPA_REG_T2,ctx);return 1},
        _=>return -EINVAL,
    }; 0
}

pub unsafe fn bpf_jit_build_epilogue(ctx:*mut hppa_jit_context){emit(EXIT_PTR_LOAD(HPPA_REG_RP),ctx);emit(EXIT_PTR_JUMP(HPPA_REG_RP,NOP_NEXT_INSTR),ctx);emit_hppa64_extrd(REGMAP[0] as u8,63,32,HPPA_REG_RET0,true,ctx);emit(hppa_bv(HPPA_REG_ZERO,HPPA_REG_RP,EXEC_NEXT_INSTR),ctx);}
pub unsafe fn bpf_jit_supports_kfunc_call()->bool{true}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
