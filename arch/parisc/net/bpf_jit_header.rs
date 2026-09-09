/* SPDX-License-Identifier: GPL-2.0 */
/* Common functionality for PARISC32 and PARISC64 BPF JIT compilers. */

// Dependencies supplied by the surrounding kernel translation.
pub enum bpf_prog {}
extern "Rust" {
    static REG_SZ: usize;
}

pub const HPPA_JIT_DEBUG: i32 = 0;
pub const HPPA_JIT_REBOOT: i32 = 0;
pub const HPPA_JIT_DUMP: i32 = 0;
pub const OPTIMIZE_HPPA: i32 = 1;

#[inline(always)]
pub const fn hppa_r(nr: u32) -> u32 { nr }

pub const HPPA_REG_ZERO: i32 = 0;
pub const HPPA_REG_R1: i32 = 1;
pub const HPPA_REG_RP: i32 = 2;
pub const HPPA_REG_ARG7: i32 = 19;
pub const HPPA_REG_ARG6: i32 = 20;
pub const HPPA_REG_ARG5: i32 = 21;
pub const HPPA_REG_ARG4: i32 = 22;
pub const HPPA_REG_ARG3: i32 = 23;
pub const HPPA_REG_ARG2: i32 = 24;
pub const HPPA_REG_ARG1: i32 = 25;
pub const HPPA_REG_ARG0: i32 = 26;
pub const HPPA_REG_GP: i32 = 27;
pub const HPPA_REG_RET0: i32 = 28;
pub const HPPA_REG_RET1: i32 = 29;
pub const HPPA_REG_SP: i32 = 30;
pub const HPPA_REG_R31: i32 = 31;

// CONFIG_64BIT selects the following ABI register assignments.
#[cfg(CONFIG_64BIT)]
pub const HPPA_REG_TCC: i32 = 3;
#[cfg(not(CONFIG_64BIT))]
pub const HPPA_REG_TCC: i32 = 18;
pub const HPPA_REG_TCC_SAVED: i32 = if cfg!(CONFIG_64BIT) { 4 } else { 17 };
pub const HPPA_REG_TCC_IN_INIT: i32 = HPPA_REG_R31;
pub const HPPA_REG_T0: i32 = HPPA_REG_R1;
pub const HPPA_REG_T1: i32 = HPPA_REG_R31;
pub const HPPA_REG_T2: i32 = HPPA_REG_ARG4;
#[cfg(not(CONFIG_64BIT))]
pub const HPPA_REG_T3: i32 = HPPA_REG_ARG5;
#[cfg(not(CONFIG_64BIT))]
pub const HPPA_REG_T4: i32 = HPPA_REG_ARG6;
#[cfg(not(CONFIG_64BIT))]
pub const HPPA_REG_T5: i32 = HPPA_REG_ARG7;

#[repr(C)]
pub struct HppaJitContext {
    pub prog: *mut bpf_prog,
    pub insns: *mut u32,
    pub ninsns: i32,
    pub reg_seen_collect: i32,
    pub reg_seen: i32,
    pub body_len: i32,
    pub epilogue_offset: i32,
    pub prologue_len: i32,
    pub offset: *mut i32,
}

macro_rules! reg_set_seen { ($ctx:expr, $nr:expr) => { if $ctx.reg_seen_collect != 0 { $ctx.reg_seen |= 1i32 << $nr; } }; }
macro_rules! reg_set_seen_all { ($ctx:expr) => { if $ctx.reg_seen_collect != 0 { $ctx.reg_seen = -1; } }; }
macro_rules! reg_force_seen { ($ctx:expr, $nr:expr) => { $ctx.reg_seen |= 1i32 << $nr; }; }
macro_rules! reg_was_seen { ($ctx:expr, $nr:expr) => { $ctx.reg_seen & (1i32 << $nr) }; }
macro_rules! reg_all_seen { ($ctx:expr) => { $ctx.reg_seen == -1 }; }

pub const HPPA_INSN_SIZE: i32 = 4;
// REG_SIZE is REG_SZ in the kernel build.
pub const REG_SIZE: usize = REG_SZ;
pub const HPPA_BRANCH_DISPLACEMENT: i32 = 2;
pub const EXEC_NEXT_INSTR: i32 = 0;
pub const NOP_NEXT_INSTR: i32 = 1;

macro_rules! im11 { ($val:expr) => { (($val as u32) & 0x07ff) }; }

macro_rules! hppa_ldil { ($addr:expr, $reg:expr) => { hppa_t5_insn(0x08, $reg, (($addr as u32) >> 11)) }; }
macro_rules! hppa_addil { ($addr:expr, $reg:expr) => { hppa_t5_insn(0x0a, $reg, (($addr as u32) >> 11)) }; }
macro_rules! hppa_ldo { ($im14:expr, $reg:expr, $target:expr) => { hppa_t1_insn(0x0d, $reg, $target, $im14) }; }
macro_rules! hppa_ldi { ($im14:expr, $reg:expr) => { hppa_ldo!($im14, HPPA_REG_ZERO, $reg) }; }
macro_rules! hppa_or { ($r1:expr, $r2:expr, $target:expr) => { hppa_t6_insn(0x02, $r2, $r1, 0, 0, 0x09, $target) }; }
macro_rules! hppa_or_cond { ($r1:expr, $r2:expr, $cond:expr, $f:expr, $target:expr) => { hppa_t6_insn(0x02, $r2, $r1, $cond, $f, 0x09, $target) }; }
macro_rules! hppa_and { ($r1:expr, $r2:expr, $target:expr) => { hppa_t6_insn(0x02, $r2, $r1, 0, 0, 0x08, $target) }; }
macro_rules! hppa_and_cond { ($r1:expr, $r2:expr, $cond:expr, $f:expr, $target:expr) => { hppa_t6_insn(0x02, $r2, $r1, $cond, $f, 0x08, $target) }; }
macro_rules! hppa_xor { ($r1:expr, $r2:expr, $target:expr) => { hppa_t6_insn(0x02, $r2, $r1, 0, 0, 0x0a, $target) }; }
macro_rules! hppa_add { ($r1:expr, $r2:expr, $target:expr) => { hppa_t6_insn(0x02, $r2, $r1, 0, 0, 0x18, $target) }; }
macro_rules! hppa_addc { ($r1:expr, $r2:expr, $target:expr) => { hppa_t6_insn(0x02, $r2, $r1, 0, 0, 0x1c, $target) }; }
macro_rules! hppa_sub { ($r1:expr, $r2:expr, $target:expr) => { hppa_t6_insn(0x02, $r2, $r1, 0, 0, 0x10, $target) }; }
macro_rules! hppa_subb { ($r1:expr, $r2:expr, $target:expr) => { hppa_t6_insn(0x02, $r2, $r1, 0, 0, 0x14, $target) }; }
macro_rules! hppa_nop { () => { hppa_or!(0, 0, 0) }; }
macro_rules! hppa_addi { ($v:expr, $r:expr, $t:expr) => { hppa_t7_insn(0x2d, $r, $t, $v) }; }
macro_rules! hppa_subi { ($v:expr, $r:expr, $t:expr) => { hppa_t7_insn(0x25, $r, $t, $v) }; }
macro_rules! hppa_copy { ($r:expr, $t:expr) => { hppa_or!($r, HPPA_REG_ZERO, $t) }; }
macro_rules! hppa_ldw { ($v:expr, $r:expr, $t:expr) => { hppa_t1_insn(0x12, $r, $t, $v) }; }
macro_rules! hppa_ldb { ($v:expr, $r:expr, $t:expr) => { hppa_t1_insn(0x10, $r, $t, $v) }; }
macro_rules! hppa_ldh { ($v:expr, $r:expr, $t:expr) => { hppa_t1_insn(0x11, $r, $t, $v) }; }
macro_rules! hppa_stw { ($r:expr, $v:expr, $b:expr) => { hppa_t1_insn(0x1a, $b, $r, $v) }; }
macro_rules! hppa_stb { ($r:expr, $v:expr, $b:expr) => { hppa_t1_insn(0x18, $b, $r, $v) }; }
macro_rules! hppa_sth { ($r:expr, $v:expr, $b:expr) => { hppa_t1_insn(0x19, $b, $r, $v) }; }
macro_rules! hppa_stwma { ($r:expr, $v:expr, $b:expr) => { hppa_t1_insn(0x1b, $b, $r, $v) }; }
macro_rules! hppa_bv { ($r:expr, $b:expr, $n:expr) => { hppa_t11_insn(0x3a, $b, $r, 0x06, 0, $n) }; }
macro_rules! hppa_be { ($o:expr, $b:expr) => { hppa_t12_insn(0x38, $b, $o, 0x00, 1) }; }
macro_rules! hppa_be_l { ($o:expr, $b:expr, $n:expr) => { hppa_t12_insn(0x39, $b, $o, 0x00, $n) }; }
macro_rules! hppa_mtctl { ($r:expr, $cr:expr) => { hppa_t21_insn(0x00, $cr, $r, 0xc2, 0) }; }
macro_rules! hppa_mtsar { ($r:expr) => { hppa_mtctl!($r, 11) }; }
macro_rules! hppa_zdep { ($r:expr, $p:expr, $l:expr, $t:expr) => { hppa_t10_insn(0x35, $t, $r, 0, 2, $p, $l) }; }
macro_rules! hppa_shl { ($r:expr, $l:expr, $t:expr) => { hppa_zdep!($r, $l, $l, lo(rd)) }; }
macro_rules! hppa_depwz { ($r:expr, $p:expr, $l:expr, $t:expr) => { hppa_t10_insn(0x35, $t, $r, 0, 3, 31 - ($p), 32 - ($l)) }; }
macro_rules! hppa_depwz_sar { ($r:expr, $t:expr) => { hppa_t1_insn(0x35, $t, $r, 0) }; }
macro_rules! hppa_shrpw_sar { ($r:expr, $t:expr) => { hppa_t10_insn(0x34, $r, 0, 0, 0, 0, $t) }; }
macro_rules! hppa_shrpw { ($r1:expr, $r2:expr, $p:expr, $t:expr) => { hppa_t10_insn(0x34, $r2, $r1, 0, 2, 31 - ($p), $t) }; }
macro_rules! hppa_shd { ($r1:expr, $r2:expr, $p:expr, $t:expr) => { hppa_t10_insn(0x34, $r2, $r1, 0, 2, 31 - ($p), $t) }; }
macro_rules! hppa_extrws_sar { ($r:expr, $t:expr) => { hppa_t10_insn(0x34, $r, $t, 0, 5, 0, 0) }; }
macro_rules! hppa_extrws { ($r:expr, $p:expr, $l:expr, $t:expr) => { hppa_t10_insn(0x34, $r, $t, 0, 7, $p, $l) }; }
macro_rules! hppa_extru { ($r:expr, $p:expr, $l:expr, $t:expr) => { hppa_t10_insn(0x34, $r, $t, 0, 6, $p, 32 - ($l)) }; }
macro_rules! hppa_shr { ($r:expr, $l:expr, $t:expr) => { hppa_extru!($r, 31 - ($l), 32 - ($l), $t) }; }
macro_rules! hppa_bl { ($i:expr, $rp:expr) => { hppa_t12_insn(0x3a, $rp, $i, 0x00, 1) }; }
macro_rules! hppa_sh2add { ($r1:expr, $r2:expr, $t:expr) => { hppa_t6_insn(0x02, $r2, $r1, 0, 0, 0x1a, $t) }; }

macro_rules! hppa_combt { ($r1:expr, $r2:expr, $a:expr, $c:expr, $n:expr) => { hppa_t11_insn(if cfg!(CONFIG_64BIT) { 0x27 } else { 0x20 }, $r2, $r1, $c, $a, $n) }; }
macro_rules! hppa_beq { ($a:expr, $b:expr, $t:expr) => { hppa_combt!($a, $b, $t, 1, NOP_NEXT_INSTR) }; }
macro_rules! hppa_blt { ($a:expr, $b:expr, $t:expr) => { hppa_combt!($a, $b, $t, 2, NOP_NEXT_INSTR) }; }
macro_rules! hppa_ble { ($a:expr, $b:expr, $t:expr) => { hppa_combt!($a, $b, $t, 3, NOP_NEXT_INSTR) }; }
macro_rules! hppa_bltu { ($a:expr, $b:expr, $t:expr) => { hppa_combt!($a, $b, $t, 4, NOP_NEXT_INSTR) }; }
macro_rules! hppa_bleu { ($a:expr, $b:expr, $t:expr) => { hppa_combt!($a, $b, $t, 5, NOP_NEXT_INSTR) }; }
macro_rules! hppa_combf { ($r1:expr, $r2:expr, $a:expr, $c:expr, $n:expr) => { hppa_t11_insn(if cfg!(CONFIG_64BIT) { 0x2f } else { 0x22 }, $r2, $r1, $c, $a, $n) }; }
macro_rules! hppa_bne { ($a:expr, $b:expr, $t:expr) => { hppa_combf!($a, $b, $t, 1, NOP_NEXT_INSTR) }; }
macro_rules! hppa_bge { ($a:expr, $b:expr, $t:expr) => { hppa_combf!($a, $b, $t, 2, NOP_NEXT_INSTR) }; }
macro_rules! hppa_bgt { ($a:expr, $b:expr, $t:expr) => { hppa_combf!($a, $b, $t, 3, NOP_NEXT_INSTR) }; }
macro_rules! hppa_bgeu { ($a:expr, $b:expr, $t:expr) => { hppa_combf!($a, $b, $t, 4, NOP_NEXT_INSTR) }; }
macro_rules! hppa_bgtu { ($a:expr, $b:expr, $t:expr) => { hppa_combf!($a, $b, $t, 5, NOP_NEXT_INSTR) }; }

#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_ldd_reg { ($r:expr, $b:expr, $t:expr) => { hppa_t10_insn(0x03, $b, $r, 0, 0, 3 << 1, $t) }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_ldd_im5 { ($i:expr, $b:expr, $t:expr) => { hppa_t10_insn(0x03, $b, low_sign_unext($i, 5), 0, 1 << 2, 3 << 1, $t) }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_ldd_im16 { ($i:expr, $b:expr, $t:expr) => { hppa_t10_insn(0x14, $b, $t, 0, 0, 0, 0) | re_assemble_16($i) }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_std_im5 { ($s:expr, $i:expr, $b:expr) => { hppa_t10_insn(0x03, $b, $s, 0, 1 << 2, 0xB << 1, low_sign_unext($i, 5)) }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_std_im16 { ($s:expr, $i:expr, $b:expr) => { hppa_t10_insn(0x1c, $b, $s, 0, 0, 0, 0) | re_assemble_16($i) }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_bl_long { ($o:expr) => { hppa_t12_L_insn(0x3a, $o, 1) }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_mtsarcm { ($r:expr) => { hppa_t21_insn(0x00, 11, $r, 0xc6, 0) }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_shrpd_sar { ($r:expr, $t:expr) => { hppa_t10_insn(0x34, $r, 0, 0, 0, 1 << 4, $t) }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_shladd { ($r1:expr, $sa:expr, $r2:expr, $t:expr) => { hppa_t6_insn(0x02, $r2, $r1, 0, 0, (1 << 4) | (1 << 3) | $sa, $t) }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_depdz_sar { ($r:expr, $t:expr) => { hppa_t21_insn(0x35, $t, $r, 3 << 3, 0) }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa_extrd_sar { ($r:expr, $t:expr, $se:expr) => { hppa_t10_insn(0x34, $r, $t, 0, 0, 0, 0) | (2 << 11) | (($se & 1) << 10) | (1 << 9) | (1 << 8) }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_bve_l_rp { ($b:expr) => { (0x3a << 26) | ($b << 21) | 0xf000 }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_permh_3210 { ($r:expr, $t:expr) => { (0x3e << 26) | ($r << 21) | ($r << 16) | $t | 0x00006900 }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_hshl { ($r:expr, $sa:expr, $t:expr) => { (0x3e << 26) | ($r << 16) | ($sa << 6) | $t | 0x00008800 }; }
#[cfg(CONFIG_64BIT)]
macro_rules! hppa64_hshr_u { ($r:expr, $sa:expr, $t:expr) => { (0x3e << 26) | ($r << 21) | ($sa << 6) | $t | 0x0000c800 }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
