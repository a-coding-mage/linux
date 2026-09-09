/* SPDX-License-Identifier: GPL-2.0-only */
/* BPF JIT compiler for ARM64. Translated from the C header. */

// Dependency supplied by the surrounding ARM64 instruction definitions:
// asm/insn.h

macro_rules! A64_R { ($x:ident) => { AARCH64_INSN_REG_$x }; }
pub const A64_FP: _ = AARCH64_INSN_REG_FP;
pub const A64_LR: _ = AARCH64_INSN_REG_LR;
pub const A64_ZR: _ = AARCH64_INSN_REG_ZR;
pub const A64_SP: _ = AARCH64_INSN_REG_SP;

macro_rules! A64_VARIANT { ($sf:expr) => { if $sf { AARCH64_INSN_VARIANT_64BIT } else { AARCH64_INSN_VARIANT_32BIT } }; }
macro_rules! A64_COMP_BRANCH { ($sf:expr, $rt:expr, $offset:expr, $ty:ident) => { aarch64_insn_gen_comp_branch_imm(0, $offset, $rt, A64_VARIANT!($sf), AARCH64_INSN_BRANCH_COMP_$ty) }; }
macro_rules! A64_CBZ { ($sf:expr, $rt:expr, $imm:expr) => { A64_COMP_BRANCH!($sf, $rt, ($imm) << 2, ZERO) }; }
macro_rules! A64_CBNZ { ($sf:expr, $rt:expr, $imm:expr) => { A64_COMP_BRANCH!($sf, $rt, ($imm) << 2, NONZERO) }; }

macro_rules! A64_COND_BRANCH { ($cond:expr, $offset:expr) => { aarch64_insn_gen_cond_branch_imm(0, $offset, $cond) }; }
pub const A64_COND_EQ: _ = AARCH64_INSN_COND_EQ; pub const A64_COND_NE: _ = AARCH64_INSN_COND_NE;
pub const A64_COND_CS: _ = AARCH64_INSN_COND_CS; pub const A64_COND_HI: _ = AARCH64_INSN_COND_HI;
pub const A64_COND_LS: _ = AARCH64_INSN_COND_LS; pub const A64_COND_CC: _ = AARCH64_INSN_COND_CC;
pub const A64_COND_GE: _ = AARCH64_INSN_COND_GE; pub const A64_COND_GT: _ = AARCH64_INSN_COND_GT;
pub const A64_COND_LE: _ = AARCH64_INSN_COND_LE; pub const A64_COND_LT: _ = AARCH64_INSN_COND_LT;
macro_rules! A64_B_ { ($c:expr, $i:expr) => { A64_COND_BRANCH!($c, ($i) << 2) }; }

macro_rules! A64_BRANCH { ($o:expr, $ty:ident) => { aarch64_insn_gen_branch_imm(0, $o, AARCH64_INSN_BRANCH_$ty) }; }
macro_rules! A64_B { ($i:expr) => { A64_BRANCH!(($i) << 2, NOLINK) }; }
macro_rules! A64_BL { ($i:expr) => { A64_BRANCH!(($i) << 2, LINK) }; }
macro_rules! A64_BR { ($r:expr) => { aarch64_insn_gen_branch_reg($r, AARCH64_INSN_BRANCH_NOLINK) }; }
macro_rules! A64_BLR { ($r:expr) => { aarch64_insn_gen_branch_reg($r, AARCH64_INSN_BRANCH_LINK) }; }
macro_rules! A64_RET { ($r:expr) => { aarch64_insn_gen_branch_reg($r, AARCH64_INSN_BRANCH_RETURN) }; }

macro_rules! A64_SIZE { ($sf:expr) => { if $sf { AARCH64_INSN_SIZE_64 } else { AARCH64_INSN_SIZE_32 } }; }
macro_rules! A64_LS_REG { ($rt:expr,$rn:expr,$rm:expr,$size:ident,$ty:ident) => { aarch64_insn_gen_load_store_reg($rt,$rn,$rm,AARCH64_INSN_SIZE_$size,AARCH64_INSN_LDST_$ty_REG_OFFSET) }; }
macro_rules! A64_LS_IMM { ($rt:expr,$rn:expr,$imm:expr,$size:ident,$ty:ident) => { aarch64_insn_gen_load_store_imm($rt,$rn,$imm,AARCH64_INSN_SIZE_$size,AARCH64_INSN_LDST_$ty##_IMM_OFFSET) }; }

/* Register-offset and immediate-offset load/store aliases. */
macro_rules! A64_STRB { ($a:expr,$b:expr,$c:expr) => { A64_LS_REG!($a,$b,$c,8,STORE) }; }
macro_rules! A64_LDRB { ($a:expr,$b:expr,$c:expr) => { A64_LS_REG!($a,$b,$c,8,LOAD) }; }
macro_rules! A64_LDRSB { ($a:expr,$b:expr,$c:expr) => { A64_LS_REG!($a,$b,$c,8,SIGNED_LOAD) }; }
macro_rules! A64_STRH { ($a:expr,$b:expr,$c:expr) => { A64_LS_REG!($a,$b,$c,16,STORE) }; }
macro_rules! A64_LDRH { ($a:expr,$b:expr,$c:expr) => { A64_LS_REG!($a,$b,$c,16,LOAD) }; }
macro_rules! A64_LDRSH { ($a:expr,$b:expr,$c:expr) => { A64_LS_REG!($a,$b,$c,16,SIGNED_LOAD) }; }
macro_rules! A64_STR32 { ($a:expr,$b:expr,$c:expr) => { A64_LS_REG!($a,$b,$c,32,STORE) }; }
macro_rules! A64_LDR32 { ($a:expr,$b:expr,$c:expr) => { A64_LS_REG!($a,$b,$c,32,LOAD) }; }
macro_rules! A64_LDRSW { ($a:expr,$b:expr,$c:expr) => { A64_LS_REG!($a,$b,$c,32,SIGNED_LOAD) }; }
macro_rules! A64_STR64 { ($a:expr,$b:expr,$c:expr) => { A64_LS_REG!($a,$b,$c,64,STORE) }; }
macro_rules! A64_LDR64 { ($a:expr,$b:expr,$c:expr) => { A64_LS_REG!($a,$b,$c,64,LOAD) }; }

macro_rules! A64_LDR32LIT { ($t:expr,$o:expr) => { aarch64_insn_gen_load_literal(0,$o,$t,false) }; }
macro_rules! A64_LDR64LIT { ($t:expr,$o:expr) => { aarch64_insn_gen_load_literal(0,$o,$t,true) }; }
macro_rules! A64_LS_PAIR { ($a:expr,$b:expr,$c:expr,$o:expr,$ls:ident,$ty:ident) => { aarch64_insn_gen_load_store_pair($a,$b,$c,$o,AARCH64_INSN_VARIANT_64BIT,AARCH64_INSN_LDST_$ls##_PAIR_$ty) }; }
macro_rules! A64_PUSH { ($a:expr,$b:expr,$c:expr) => { A64_LS_PAIR!($a,$b,$c,-16,STORE,PRE_INDEX) }; }
macro_rules! A64_POP { ($a:expr,$b:expr,$c:expr) => { A64_LS_PAIR!($a,$b,$c,16,LOAD,POST_INDEX) }; }

macro_rules! A64_LSX { ($sf:expr,$rt:expr,$rn:expr,$rs:expr,$ty:ident) => { aarch64_insn_gen_load_store_ex($rt,$rn,$rs,A64_SIZE!($sf),AARCH64_INSN_LDST_$ty) }; }
macro_rules! A64_LDXR { ($sf:expr,$rt:expr,$rn:expr) => { A64_LSX!($sf,$rt,$rn,A64_ZR,LOAD_EX) }; }
macro_rules! A64_STXR { ($sf:expr,$rt:expr,$rn:expr,$rs:expr) => { A64_LSX!($sf,$rt,$rn,$rs,STORE_EX) }; }
macro_rules! A64_STLXR { ($sf:expr,$rt:expr,$rn:expr,$rs:expr) => { aarch64_insn_gen_load_store_ex($rt,$rn,$rs,A64_SIZE!($sf),AARCH64_INSN_LDST_STORE_REL_EX) }; }

macro_rules! A64_LDAR { ($rt:expr,$rn:expr,$size:ident) => { aarch64_insn_gen_load_acq_store_rel($rt,$rn,AARCH64_INSN_SIZE_$size,AARCH64_INSN_LDST_LOAD_ACQ) }; }
macro_rules! A64_STLR { ($rt:expr,$rn:expr,$size:ident) => { aarch64_insn_gen_load_acq_store_rel($rt,$rn,AARCH64_INSN_SIZE_$size,AARCH64_INSN_LDST_STORE_REL) }; }
macro_rules! A64_LDARB { ($a:expr,$b:expr) => { A64_LDAR!($a,$b,8) }; } macro_rules! A64_LDARH { ($a:expr,$b:expr) => { A64_LDAR!($a,$b,16) }; }
macro_rules! A64_LDAR32 { ($a:expr,$b:expr) => { A64_LDAR!($a,$b,32) }; } macro_rules! A64_LDAR64 { ($a:expr,$b:expr) => { A64_LDAR!($a,$b,64) }; }
macro_rules! A64_STLRB { ($a:expr,$b:expr) => { A64_STLR!($a,$b,8) }; } macro_rules! A64_STLRH { ($a:expr,$b:expr) => { A64_STLR!($a,$b,16) }; }
macro_rules! A64_STLR32 { ($a:expr,$b:expr) => { A64_STLR!($a,$b,32) }; } macro_rules! A64_STLR64 { ($a:expr,$b:expr) => { A64_STLR!($a,$b,64) }; }

macro_rules! A64_ST_OP { ($sf:expr,$rn:expr,$rs:expr,$op:ident) => { aarch64_insn_gen_atomic_ld_op(A64_ZR,$rn,$rs,A64_SIZE!($sf),AARCH64_INSN_MEM_ATOMIC_$op,AARCH64_INSN_MEM_ORDER_NONE) }; }
macro_rules! A64_STADD { ($s:expr,$n:expr,$r:expr) => { A64_ST_OP!($s,$n,$r,ADD) }; } macro_rules! A64_STCLR { ($s:expr,$n:expr,$r:expr) => { A64_ST_OP!($s,$n,$r,CLR) }; }
macro_rules! A64_STEOR { ($s:expr,$n:expr,$r:expr) => { A64_ST_OP!($s,$n,$r,EOR) }; } macro_rules! A64_STSET { ($s:expr,$n:expr,$r:expr) => { A64_ST_OP!($s,$n,$r,SET) }; }
macro_rules! A64_LD_OP_AL { ($sf:expr,$rt:expr,$rn:expr,$rs:expr,$op:ident) => { aarch64_insn_gen_atomic_ld_op($rt,$rn,$rs,A64_SIZE!($sf),AARCH64_INSN_MEM_ATOMIC_$op,AARCH64_INSN_MEM_ORDER_ACQREL) }; }
macro_rules! A64_LDADDAL { ($s:expr,$t:expr,$n:expr,$r:expr) => { A64_LD_OP_AL!($s,$t,$n,$r,ADD) }; } macro_rules! A64_LDCLRAL { ($s:expr,$t:expr,$n:expr,$r:expr) => { A64_LD_OP_AL!($s,$t,$n,$r,CLR) }; }
macro_rules! A64_LDEORAL { ($s:expr,$t:expr,$n:expr,$r:expr) => { A64_LD_OP_AL!($s,$t,$n,$r,EOR) }; } macro_rules! A64_LDSETAL { ($s:expr,$t:expr,$n:expr,$r:expr) => { A64_LD_OP_AL!($s,$t,$n,$r,SET) }; }
macro_rules! A64_SWPAL { ($s:expr,$t:expr,$n:expr,$r:expr) => { A64_LD_OP_AL!($s,$t,$n,$r,SWP) }; }
macro_rules! A64_CASAL { ($s:expr,$t:expr,$n:expr,$r:expr) => { aarch64_insn_gen_cas($t,$n,$r,A64_SIZE!($s),AARCH64_INSN_MEM_ORDER_ACQREL) }; }

macro_rules! A64_ADDSUB_IMM { ($s:expr,$d:expr,$n:expr,$i:expr,$ty:ident) => { aarch64_insn_gen_add_sub_imm($d,$n,$i,A64_VARIANT!($s),AARCH64_INSN_ADSB_$ty) }; }
macro_rules! A64_ADD_I { ($s:expr,$d:expr,$n:expr,$i:expr) => { A64_ADDSUB_IMM!($s,$d,$n,$i,ADD) }; } macro_rules! A64_SUB_I { ($s:expr,$d:expr,$n:expr,$i:expr) => { A64_ADDSUB_IMM!($s,$d,$n,$i,SUB) }; }
macro_rules! A64_ADDS_I { ($s:expr,$d:expr,$n:expr,$i:expr) => { A64_ADDSUB_IMM!($s,$d,$n,$i,ADD_SETFLAGS) }; } macro_rules! A64_SUBS_I { ($s:expr,$d:expr,$n:expr,$i:expr) => { A64_ADDSUB_IMM!($s,$d,$n,$i,SUB_SETFLAGS) }; }
macro_rules! A64_CMN_I { ($s:expr,$n:expr,$i:expr) => { A64_ADDS_I!($s,A64_ZR,$n,$i) }; } macro_rules! A64_CMP_I { ($s:expr,$n:expr,$i:expr) => { A64_SUBS_I!($s,A64_ZR,$n,$i) }; }
macro_rules! A64_MOV { ($s:expr,$d:expr,$n:expr) => { if ($d)==A64_SP || ($n)==A64_SP { A64_ADD_I!($s,$d,$n,0) } else { aarch64_insn_gen_move_reg($d,$n,A64_VARIANT!($s)) } }; }

macro_rules! A64_BITFIELD { ($s:expr,$d:expr,$n:expr,$r:expr,$i:expr,$ty:ident) => { aarch64_insn_gen_bitfield($d,$n,$r,$i,A64_VARIANT!($s),AARCH64_INSN_BITFIELD_MOVE_$ty) }; }
macro_rules! A64_SBFM { ($s:expr,$d:expr,$n:expr,$r:expr,$i:expr) => { A64_BITFIELD!($s,$d,$n,$r,$i,SIGNED) }; } macro_rules! A64_UBFM { ($s:expr,$d:expr,$n:expr,$r:expr,$i:expr) => { A64_BITFIELD!($s,$d,$n,$r,$i,UNSIGNED) }; }
macro_rules! A64_LSL { ($s:expr,$d:expr,$n:expr,$sh:expr) => {{ let sz = if $s {64} else {32}; A64_UBFM!($s,$d,$n,(0u32.wrapping_sub($sh as u32)) % sz,sz-1-$sh) }}; }
macro_rules! A64_LSR { ($s:expr,$d:expr,$n:expr,$sh:expr) => { A64_UBFM!($s,$d,$n,$sh,if $s {63} else {31}) }; } macro_rules! A64_ASR { ($s:expr,$d:expr,$n:expr,$sh:expr) => { A64_SBFM!($s,$d,$n,$sh,if $s {63} else {31}) }; }
macro_rules! A64_UXTH { ($s:expr,$d:expr,$n:expr) => { A64_UBFM!($s,$d,$n,0,15) }; } macro_rules! A64_UXTW { ($s:expr,$d:expr,$n:expr) => { A64_UBFM!($s,$d,$n,0,31) }; }
macro_rules! A64_SXTB { ($s:expr,$d:expr,$n:expr) => { A64_SBFM!($s,$d,$n,0,7) }; } macro_rules! A64_SXTH { ($s:expr,$d:expr,$n:expr) => { A64_SBFM!($s,$d,$n,0,15) }; } macro_rules! A64_SXTW { ($s:expr,$d:expr,$n:expr) => { A64_SBFM!($s,$d,$n,0,31) }; }

macro_rules! A64_MOVEW { ($s:expr,$d:expr,$i:expr,$sh:expr,$ty:ident) => { aarch64_insn_gen_movewide($d,$i,$sh,A64_VARIANT!($s),AARCH64_INSN_MOVEWIDE_$ty) }; }
macro_rules! A64_MOVN { ($s:expr,$d:expr,$i:expr,$sh:expr) => { A64_MOVEW!($s,$d,$i,$sh,INVERSE) }; } macro_rules! A64_MOVZ { ($s:expr,$d:expr,$i:expr,$sh:expr) => { A64_MOVEW!($s,$d,$i,$sh,ZERO) }; } macro_rules! A64_MOVK { ($s:expr,$d:expr,$i:expr,$sh:expr) => { A64_MOVEW!($s,$d,$i,$sh,KEEP) }; }
macro_rules! A64_ADDSUB_SREG { ($s:expr,$d:expr,$n:expr,$m:expr,$ty:ident) => { aarch64_insn_gen_add_sub_shifted_reg($d,$n,$m,0,A64_VARIANT!($s),AARCH64_INSN_ADSB_$ty) }; }
macro_rules! A64_ADD { ($s:expr,$d:expr,$n:expr,$m:expr) => { A64_ADDSUB_SREG!($s,$d,$n,$m,ADD) }; } macro_rules! A64_SUB { ($s:expr,$d:expr,$n:expr,$m:expr) => { A64_ADDSUB_SREG!($s,$d,$n,$m,SUB) }; } macro_rules! A64_SUBS { ($s:expr,$d:expr,$n:expr,$m:expr) => { A64_ADDSUB_SREG!($s,$d,$n,$m,SUB_SETFLAGS) }; }
macro_rules! A64_NEG { ($s:expr,$d:expr,$m:expr) => { A64_SUB!($s,$d,A64_ZR,$m) }; } macro_rules! A64_CMP { ($s:expr,$n:expr,$m:expr) => { A64_SUBS!($s,A64_ZR,$n,$m) }; }

macro_rules! A64_ADDSUB_EREG { ($s:expr,$d:expr,$n:expr,$m:expr,$e:ident,$sh:expr,$ty:ident) => { aarch64_insn_gen_add_sub_extended_reg($d,$n,$m,AARCH64_INSN_EXTEND_$e,$sh,A64_VARIANT!($s),AARCH64_INSN_ADSB_$ty) }; }
macro_rules! A64_ADD_EXT { ($s:expr,$d:expr,$n:expr,$m:expr,$e:ident,$sh:expr) => { A64_ADDSUB_EREG!($s,$d,$n,$m,$e,$sh,ADD) }; } macro_rules! A64_ADD_UXTW { ($d:expr,$n:expr,$m:expr) => { A64_ADD_EXT!(1,$d,$n,$m,UXTW,0) }; }
macro_rules! A64_DATA1 { ($s:expr,$d:expr,$n:expr,$ty:ident) => { aarch64_insn_gen_data1($d,$n,A64_VARIANT!($s),AARCH64_INSN_DATA1_$ty) }; }
macro_rules! A64_REV16 { ($s:expr,$d:expr,$n:expr) => { A64_DATA1!($s,$d,$n,REVERSE_16) }; } macro_rules! A64_REV32 { ($s:expr,$d:expr,$n:expr) => { A64_DATA1!($s,$d,$n,REVERSE_32) }; } macro_rules! A64_REV64 { ($d:expr,$n:expr) => { A64_DATA1!(1,$d,$n,REVERSE_64) }; }
macro_rules! A64_DATA2 { ($s:expr,$d:expr,$n:expr,$m:expr,$ty:ident) => { aarch64_insn_gen_data2($d,$n,$m,A64_VARIANT!($s),AARCH64_INSN_DATA2_$ty) }; }
macro_rules! A64_UDIV { ($s:expr,$d:expr,$n:expr,$m:expr) => { A64_DATA2!($s,$d,$n,$m,UDIV) }; } macro_rules! A64_SDIV { ($s:expr,$d:expr,$n:expr,$m:expr) => { A64_DATA2!($s,$d,$n,$m,SDIV) }; } macro_rules! A64_LSLV { ($s:expr,$d:expr,$n:expr,$m:expr) => { A64_DATA2!($s,$d,$n,$m,LSLV) }; } macro_rules! A64_LSRV { ($s:expr,$d:expr,$n:expr,$m:expr) => { A64_DATA2!($s,$d,$n,$m,LSRV) }; } macro_rules! A64_ASRV { ($s:expr,$d:expr,$n:expr,$m:expr) => { A64_DATA2!($s,$d,$n,$m,ASRV) }; }
macro_rules! A64_MADD { ($s:expr,$d:expr,$a:expr,$n:expr,$m:expr) => { aarch64_insn_gen_data3($d,$a,$n,$m,A64_VARIANT!($s),AARCH64_INSN_DATA3_MADD) }; } macro_rules! A64_MSUB { ($s:expr,$d:expr,$a:expr,$n:expr,$m:expr) => { aarch64_insn_gen_data3($d,$a,$n,$m,A64_VARIANT!($s),AARCH64_INSN_DATA3_MSUB) }; } macro_rules! A64_MUL { ($s:expr,$d:expr,$n:expr,$m:expr) => { A64_MADD!($s,$d,A64_ZR,$n,$m) }; }
macro_rules! A64_LOGIC_SREG { ($s:expr,$d:expr,$n:expr,$m:expr,$ty:ident) => { aarch64_insn_gen_logical_shifted_reg($d,$n,$m,0,A64_VARIANT!($s),AARCH64_INSN_LOGIC_$ty) }; }
macro_rules! A64_AND { ($s:expr,$d:expr,$n:expr,$m:expr) => { A64_LOGIC_SREG!($s,$d,$n,$m,AND) }; } macro_rules! A64_ORR { ($s:expr,$d:expr,$n:expr,$m:expr) => { A64_LOGIC_SREG!($s,$d,$n,$m,ORR) }; } macro_rules! A64_EOR { ($s:expr,$d:expr,$n:expr,$m:expr) => { A64_LOGIC_SREG!($s,$d,$n,$m,EOR) }; } macro_rules! A64_ANDS { ($s:expr,$d:expr,$n:expr,$m:expr) => { A64_LOGIC_SREG!($s,$d,$n,$m,AND_SETFLAGS) }; }
macro_rules! A64_TST { ($s:expr,$n:expr,$m:expr) => { A64_ANDS!($s,A64_ZR,$n,$m) }; } macro_rules! A64_MVN { ($s:expr,$d:expr,$m:expr) => { A64_LOGIC_SREG!($s,$d,A64_ZR,$m,ORN) }; }
macro_rules! A64_LOGIC_IMM { ($s:expr,$d:expr,$n:expr,$i:expr,$ty:ident) => {{ let imm64: u64 = if $s { $i as u64 } else { $i as u32 as u64 }; aarch64_insn_gen_logical_immediate(AARCH64_INSN_LOGIC_$ty,A64_VARIANT!($s),$n,$d,imm64) }}; }
macro_rules! A64_AND_I { ($s:expr,$d:expr,$n:expr,$i:expr) => { A64_LOGIC_IMM!($s,$d,$n,$i,AND) }; } macro_rules! A64_ORR_I { ($s:expr,$d:expr,$n:expr,$i:expr) => { A64_LOGIC_IMM!($s,$d,$n,$i,ORR) }; } macro_rules! A64_EOR_I { ($s:expr,$d:expr,$n:expr,$i:expr) => { A64_LOGIC_IMM!($s,$d,$n,$i,EOR) }; } macro_rules! A64_ANDS_I { ($s:expr,$d:expr,$n:expr,$i:expr) => { A64_LOGIC_IMM!($s,$d,$n,$i,AND_SETFLAGS) }; } macro_rules! A64_TST_I { ($s:expr,$n:expr,$i:expr) => { A64_ANDS_I!($s,A64_ZR,$n,$i) }; }

macro_rules! A64_HINT { ($x:expr) => { aarch64_insn_gen_hint($x) }; }
pub const A64_PACIASP: _ = A64_HINT!(AARCH64_INSN_HINT_PACIASP); pub const A64_AUTIASP: _ = A64_HINT!(AARCH64_INSN_HINT_AUTIASP);
pub const A64_BTI_C: _ = A64_HINT!(AARCH64_INSN_HINT_BTIC); pub const A64_BTI_J: _ = A64_HINT!(AARCH64_INSN_HINT_BTIJ); pub const A64_BTI_JC: _ = A64_HINT!(AARCH64_INSN_HINT_BTIJC); pub const A64_NOP: _ = A64_HINT!(AARCH64_INSN_HINT_NOP);
pub const A64_DMB_ISH: _ = aarch64_insn_gen_dmb(AARCH64_INSN_MB_ISH);
macro_rules! A64_ADR { ($d:expr,$o:expr) => { aarch64_insn_gen_adr(0,$o,$d,AARCH64_INSN_ADR_TYPE_ADR) }; }
macro_rules! A64_MRS_TPIDR_EL1 { ($t:expr) => { aarch64_insn_gen_mrs($t,AARCH64_INSN_SYSREG_TPIDR_EL1) }; } macro_rules! A64_MRS_TPIDR_EL2 { ($t:expr) => { aarch64_insn_gen_mrs($t,AARCH64_INSN_SYSREG_TPIDR_EL2) }; } macro_rules! A64_MRS_SP_EL0 { ($t:expr) => { aarch64_insn_gen_mrs($t,AARCH64_INSN_SYSREG_SP_EL0) }; }
pub const A64_SB: _ = aarch64_insn_get_sb_value();
pub const A64_DSB_NSH: _ = aarch64_insn_get_dsb_base_value() | 0x7 << 8;
pub const A64_ISB: _ = aarch64_insn_get_isb_value();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
