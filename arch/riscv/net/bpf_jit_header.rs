/* SPDX-License-Identifier: GPL-2.0 */
/* Direct Rust translation of riscv/net/bpf_jit.h. */

use core::ffi::c_void;

pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type s32 = ::core::primitive::i32;

/* These types and constants are supplied by the surrounding kernel bindings. */
extern "C" {
    pub fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    pub fn pr_err(fmt: *const i8, ...);
}
#[repr(C)] pub struct bpf_prog { _private: [u8; 0] }
#[repr(C)] pub struct bpf_binary_header { _private: [u8; 0] }
#[repr(C)] pub struct bpf_insn { _private: [u8; 0] }
extern "C" { pub fn riscv_has_extension_likely(ext: u32) -> bool; }

#[inline] pub fn rvc_enabled() -> bool { cfg!(feature = "riscv_isa_c") }

pub const RV_REG_ZERO:u8=0; pub const RV_REG_RA:u8=1; pub const RV_REG_SP:u8=2;
pub const RV_REG_GP:u8=3; pub const RV_REG_TP:u8=4; pub const RV_REG_T0:u8=5;
pub const RV_REG_T1:u8=6; pub const RV_REG_T2:u8=7; pub const RV_REG_FP:u8=8;
pub const RV_REG_S1:u8=9; pub const RV_REG_A0:u8=10; pub const RV_REG_A1:u8=11;
pub const RV_REG_A2:u8=12; pub const RV_REG_A3:u8=13; pub const RV_REG_A4:u8=14;
pub const RV_REG_A5:u8=15; pub const RV_REG_A6:u8=16; pub const RV_REG_A7:u8=17;
pub const RV_REG_S2:u8=18; pub const RV_REG_S3:u8=19; pub const RV_REG_S4:u8=20;
pub const RV_REG_S5:u8=21; pub const RV_REG_S6:u8=22; pub const RV_REG_S7:u8=23;
pub const RV_REG_S8:u8=24; pub const RV_REG_S9:u8=25; pub const RV_REG_S10:u8=26;
pub const RV_REG_S11:u8=27; pub const RV_REG_T3:u8=28; pub const RV_REG_T4:u8=29;
pub const RV_REG_T5:u8=30; pub const RV_REG_T6:u8=31;

#[inline] pub fn is_creg(r:u8)->bool { (r==8)||(r>=9&&r<=15) }
#[repr(C)] pub struct rv_jit_context {
 pub prog:*mut bpf_prog, pub insns:*mut u16, pub ro_insns:*mut u16, pub ninsns:i32,
 pub prologue_len:i32, pub epilogue_offset:i32, pub offset:*mut i32, pub nexentries:i32,
 pub ex_insn_off:i32, pub ex_jmp_off:i32, pub flags:usize, pub stack_size:i32,
 pub tcc_offset:i32, pub arena_vm_start:u64, pub user_vm_start:u64,
}
#[repr(C)] pub struct rv_jit_data { pub header:*mut bpf_binary_header, pub ro_header:*mut bpf_binary_header, pub image:*mut u8, pub ro_image:*mut u8, pub ctx:rv_jit_context }

#[inline] pub unsafe fn ninsns_rvoff(n:i32)->i32 { n<<1 }
#[inline] pub unsafe fn bpf_fill_ill_insns(a:*mut c_void,n:u32){ memset(a,0,n as usize); }
#[inline] pub unsafe fn emit(x:u32,c:*mut rv_jit_context){ if !(*c).insns.is_null(){*(*c).insns.add((*c).ninsns as usize)=x as u16;*(*c).insns.add((*c).ninsns as usize+1)=(x>>16) as u16;}(*c).ninsns+=2; }
#[inline] pub unsafe fn emitc(x:u16,c:*mut rv_jit_context){if !(*c).insns.is_null(){*(*c).insns.add((*c).ninsns as usize)=x;}(*c).ninsns+=1;}
#[inline] pub unsafe fn epilogue_offset(c:*mut rv_jit_context)->i32{ninsns_rvoff((*c).epilogue_offset-(*c).ninsns)}

#[inline] pub fn is_6b_int(v:isize)->bool{v>=-(1<<5)&&v<(1<<5)}
#[inline] pub fn is_7b_uint(v:usize)->bool{v<(1<<7)}
#[inline] pub fn is_8b_uint(v:usize)->bool{v<(1<<8)}
#[inline] pub fn is_9b_uint(v:usize)->bool{v<(1<<9)}
#[inline] pub fn is_10b_int(v:isize)->bool{v>=-(1<<9)&&v<(1<<9)}
#[inline] pub fn is_10b_uint(v:usize)->bool{v<(1<<10)}
#[inline] pub fn is_12b_int(v:isize)->bool{v>=-(1<<11)&&v<(1<<11)}
#[inline] pub fn is_13b_int(v:isize)->bool{v>=-(1<<12)&&v<(1<<12)}
#[inline] pub fn is_21b_int(v:isize)->bool{v>=-(1<<20)&&v<(1<<20)}

#[inline] pub fn rv_r_insn(f7:u8,rs2:u8,rs1:u8,f3:u8,rd:u8,op:u8)->u32{((f7 as u32)<<25)|((rs2 as u32)<<20)|((rs1 as u32)<<15)|((f3 as u32)<<12)|((rd as u32)<<7)|op as u32}
#[inline] pub fn rv_i_insn(i:u16,rs1:u8,f3:u8,rd:u8,op:u8)->u32{((i as u32)<<20)|((rs1 as u32)<<15)|((f3 as u32)<<12)|((rd as u32)<<7)|op as u32}
#[inline] pub fn rv_s_insn(i:u16,rs2:u8,rs1:u8,f3:u8,op:u8)->u32{rv_r_insn((i>>5) as u8,rs2,rs1,f3,(i&31) as u8,op)}
#[inline] pub fn rv_b_insn(i:u16,rs2:u8,rs1:u8,f3:u8,op:u8)->u32{let a=(((i&0x800)>>5)|((i&0x3f0)>>4)) as u8;let b=(((i&0xf)<<1)|((i&0x400)>>10)) as u8;((a as u32)<<25)|((rs2 as u32)<<20)|((rs1 as u32)<<15)|((f3 as u32)<<12)|((b as u32)<<7)|op as u32}
#[inline] pub fn rv_u_insn(i:u32,rd:u8,op:u8)->u32{(i<<12)|((rd as u32)<<7)|op as u32}
#[inline] pub fn rv_j_insn(i:u32,rd:u8,op:u8)->u32{let x=(i&0x80000)|((i&0x3ff)<<9)|((i&0x400)>>2)|((i&0x7f800)>>11);(x<<12)|((rd as u32)<<7)|op as u32}
#[inline] pub fn rv_amo_insn(f:u8,aq:u8,rl:u8,rs2:u8,rs1:u8,f3:u8,rd:u8,op:u8)->u32{rv_r_insn((f<<2)|(aq<<1)|rl,rs2,rs1,f3,rd,op)}

macro_rules! rfun { ($n:ident,$f:expr,$f3:expr,$op:expr)=>{#[inline] pub fn $n(rd:u8,rs1:u8,rs2:u8)->u32{rv_r_insn($f,rs2,rs1,$f3,rd,$op)}} }
macro_rules! ifun { ($n:ident,$f3:expr,$op:expr)=>{#[inline] pub fn $n(rd:u8,rs1:u8,i:u16)->u32{rv_i_insn(i,rs1,$f3,rd,$op)}} }
ifun!(rv_addi,0,0x13); ifun!(rv_andi,7,0x13); ifun!(rv_ori,6,0x13); ifun!(rv_xori,4,0x13); ifun!(rv_slli,1,0x13); ifun!(rv_srli,5,0x13);
#[inline] pub fn rv_srai(rd:u8,rs:u8,i:u16)->u32{rv_i_insn(0x400|i,rs,5,rd,0x13)}
rfun!(rv_add,0,0,0x33); rfun!(rv_sub,0x20,0,0x33); rfun!(rv_sltu,0,3,0x33); rfun!(rv_and,0,7,0x33); rfun!(rv_or,0,6,0x33); rfun!(rv_xor,0,4,0x33); rfun!(rv_sll,0,1,0x33); rfun!(rv_srl,0,5,0x33); rfun!(rv_sra,0x20,5,0x33); rfun!(rv_mul,1,0,0x33); rfun!(rv_mulhu,1,3,0x33); rfun!(rv_div,1,4,0x33); rfun!(rv_divu,1,5,0x33); rfun!(rv_rem,1,6,0x33); rfun!(rv_remu,1,7,0x33);
#[inline] pub fn rv_lui(rd:u8,i:u32)->u32{rv_u_insn(i,rd,0x37)} #[inline] pub fn rv_auipc(rd:u8,i:u32)->u32{rv_u_insn(i,rd,0x17)}
#[inline] pub fn rv_jal(rd:u8,i:u32)->u32{rv_j_insn(i,rd,0x6f)} #[inline] pub fn rv_jalr(rd:u8,rs:u8,i:u16)->u32{rv_i_insn(i,rs,0,rd,0x67)}
#[inline] pub fn rv_beq(a:u8,b:u8,i:u16)->u32{rv_b_insn(i,b,a,0,0x63)} #[inline] pub fn rv_bne(a:u8,b:u8,i:u16)->u32{rv_b_insn(i,b,a,1,0x63)}
#[inline] pub fn rv_bltu(a:u8,b:u8,i:u16)->u32{rv_b_insn(i,b,a,6,0x63)} #[inline] pub fn rv_bgeu(a:u8,b:u8,i:u16)->u32{rv_b_insn(i,b,a,7,0x63)}
#[inline] pub fn rv_blt(a:u8,b:u8,i:u16)->u32{rv_b_insn(i,b,a,4,0x63)} #[inline] pub fn rv_bge(a:u8,b:u8,i:u16)->u32{rv_b_insn(i,b,a,5,0x63)}
#[inline] pub fn rv_lb(rd:u8,i:u16,rs:u8)->u32{rv_i_insn(i,rs,0,rd,3)} #[inline] pub fn rv_lh(rd:u8,i:u16,rs:u8)->u32{rv_i_insn(i,rs,1,rd,3)} #[inline] pub fn rv_lw(rd:u8,i:u16,rs:u8)->u32{rv_i_insn(i,rs,2,rd,3)} #[inline] pub fn rv_lbu(rd:u8,i:u16,rs:u8)->u32{rv_i_insn(i,rs,4,rd,3)} #[inline] pub fn rv_lhu(rd:u8,i:u16,rs:u8)->u32{rv_i_insn(i,rs,5,rd,3)}
#[inline] pub fn rv_sb(rs:u8,i:u16,rt:u8)->u32{rv_s_insn(i,rt,rs,0,0x23)} #[inline] pub fn rv_sh(rs:u8,i:u16,rt:u8)->u32{rv_s_insn(i,rt,rs,1,0x23)} #[inline] pub fn rv_sw(rs:u8,i:u16,rt:u8)->u32{rv_s_insn(i,rt,rs,2,0x23)}

/* The remaining compressed encoders are direct bit-field translations. */
#[inline] pub fn rv_cr_insn(f:u8,rd:u8,rs:u8,op:u8)->u16{((f as u16)<<12)|((rd as u16)<<7)|((rs as u16)<<2)|op as u16}
#[inline] pub fn rv_ci_insn(f:u8,i:u32,rd:u8,op:u8)->u16{((f as u16)<<13)|(((i&0x20) as u16)<<7)|(((i&0x1f) as u16)<<2)|((rd as u16)<<7)|op as u16}
#[inline] pub fn rv_css_insn(f:u8,i:u32,rs:u8,op:u8)->u16{((f as u16)<<13)|((i as u16)<<7)|((rs as u16)<<2)|op as u16}
#[inline] pub fn rv_ciw_insn(f:u8,i:u32,rd:u8,op:u8)->u16{((f as u16)<<13)|((i as u16)<<5)|(((rd&7) as u16)<<2)|op as u16}
#[inline] pub fn rv_cl_insn(f:u8,hi:u32,rs:u8,lo:u32,rd:u8,op:u8)->u16{((f as u16)<<13)|((hi as u16)<<10)|(((rs&7) as u16)<<7)|((lo as u16)<<5)|(((rd&7) as u16)<<2)|op as u16}
#[inline] pub fn rv_cs_insn(f:u8,hi:u32,rs:u8,lo:u32,rt:u8,op:u8)->u16{rv_cl_insn(f,hi,rs,lo,rt,op)}
#[inline] pub fn rv_ca_insn(f:u8,rd:u8,f2:u8,rs:u8,op:u8)->u16{((f as u16)<<10)|(((rd&7) as u16)<<7)|((f2 as u16)<<5)|(((rs&7) as u16)<<2)|op as u16}
#[inline] pub fn rv_cb_insn(f:u8,i:u32,f2:u8,rd:u8,op:u8)->u16{rv_ca_insn(f,rd,f2,i as u8,op)|(((i&0x20) as u16)<<7)|(((i&0x1f) as u16)<<2)}

extern "C" { pub fn bpf_jit_build_prologue(ctx:*mut rv_jit_context,is_subprog:bool); pub fn bpf_jit_build_epilogue(ctx:*mut rv_jit_context); pub fn bpf_jit_emit_insn(insn:*const bpf_insn,ctx:*mut rv_jit_context,extra_pass:bool)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
