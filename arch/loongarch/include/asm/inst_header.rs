/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of loongarch/include/asm/inst.h. */

pub const INSN_NOP: u32 = 0x03400000;
pub const INSN_BREAK: u32 = 0x002a0000;
pub const INSN_HVCL: u32 = 0x002b8000;
pub const ADDR_IMMMASK_LU52ID: u64 = 0xFFF0000000000000;
pub const ADDR_IMMMASK_LU32ID: u64 = 0x000FFFFF00000000;
pub const ADDR_IMMMASK_LU12IW: u64 = 0x00000000FFFFF000;
pub const ADDR_IMMMASK_ORI: u64 = 0x0000000000000FFF;
pub const ADDR_IMMMASK_ADDU16ID: u64 = 0x00000000FFFF0000;
pub const ADDR_IMMSHIFT_LU52ID: u32 = 52;
pub const ADDR_IMMSBIDX_LU52ID: u32 = 11;
pub const ADDR_IMMSHIFT_LU32ID: u32 = 32;
pub const ADDR_IMMSBIDX_LU32ID: u32 = 19;
pub const ADDR_IMMSHIFT_LU12IW: u32 = 12;
pub const ADDR_IMMSBIDX_LU12IW: u32 = 19;
pub const ADDR_IMMSHIFT_ORI: u32 = 0;
pub const ADDR_IMMSBIDX_ORI: u32 = 63;
pub const ADDR_IMMSHIFT_ADDU16ID: u32 = 16;
pub const ADDR_IMMSBIDX_ADDU16ID: u32 = 15;

#[inline] pub const fn sign_extend64(v: u64, bit: u32) -> u64 { ((v << (63 - bit)) as i64 >> (63 - bit)) as u64 }
#[inline] pub const fn addr_imm(addr: u64, mask: u64, shift: u32, bit: u32) -> u64 { sign_extend64((addr & mask) >> shift, bit) }

macro_rules! op_enum { ($name:ident { $($n:ident = $v:expr),* $(,)? }) => { #[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum $name { $($n = $v),* } }; }
op_enum!(reg0i15_op { break_op=0x54, dbar_op=0x70e4 });
op_enum!(reg0i26_op { b_op=0x14, bl_op=0x15 });
op_enum!(reg1i20_op { lu12iw_op=0x0a, lu32id_op=0x0b, pcaddi_op=0x0c, pcalau12i_op=0x0d, pcaddu12i_op=0x0e, pcaddu18i_op=0x0f });
op_enum!(reg1i21_op { beqz_op=0x10, bnez_op=0x11, bceqz_op=0x12, bcnez_op=0x12 });
op_enum!(reg2_op { revb2h_op=0x0c, revb4h_op=0x0d, revb2w_op=0x0e, revbd_op=0x0f, revh2w_op=0x10, revhd_op=0x11, extwh_op=0x16, extwb_op=0x17, cpucfg_op=0x1b, iocsrrdb_op=0x19200, iocsrrdh_op=0x19201, iocsrrdw_op=0x19202, iocsrrdd_op=0x19203, iocsrwrb_op=0x19204, iocsrwrh_op=0x19205, iocsrwrw_op=0x19206, iocsrwrd_op=0x19207, llacqw_op=0xe15e0, screlw_op=0xe15e1, llacqd_op=0xe15e2, screld_op=0xe15e3 });
op_enum!(reg2i5_op { slliw_op=0x81, srliw_op=0x89, sraiw_op=0x91 });
op_enum!(reg2i6_op { sllid_op=0x41, srlid_op=0x45, sraid_op=0x49 });
op_enum!(reg2i12_op { sltui_op=9, addiw_op=0xa, addid_op=0xb, lu52id_op=0xc, andi_op=0xd, ori_op=0xe, xori_op=0xf, ldb_op=0xa0, ldh_op=0xa1, ldw_op=0xa2, ldd_op=0xa3, stb_op=0xa4, sth_op=0xa5, stw_op=0xa6, std_op=0xa7, ldbu_op=0xa8, ldhu_op=0xa9, ldwu_op=0xaa, flds_op=0xac, fsts_op=0xad, fldd_op=0xae, fstd_op=0xaf });
op_enum!(reg2i14_op { llw_op=0x20, scw_op=0x21, lld_op=0x22, scd_op=0x23, ldptrw_op=0x24, stptrw_op=0x25, ldptrd_op=0x26, stptrd_op=0x27 });
op_enum!(reg2i16_op { jirl_op=0x13, beq_op=0x16, bne_op=0x17, blt_op=0x18, bge_op=0x19, bltu_op=0x1a, bgeu_op=0x1b });
op_enum!(reg2bstrd_op { bstrinsd_op=2, bstrpickd_op=3 });
op_enum!(reg3sa2_op { alslw_op=2, alslwu_op=3, alsld_op=0x16 });

#[repr(C)] #[derive(Copy, Clone)] pub struct reg0i15_format { pub immediate: u32, pub opcode: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg0i26_format { pub immediate_h: u32, pub immediate_l: u32, pub opcode: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg1i20_format { pub rd:u32, pub immediate:u32, pub opcode:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg1i21_format { pub immediate_h:u32, pub rj:u32, pub immediate_l:u32, pub opcode:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg2_format { pub rd:u32, pub rj:u32, pub opcode:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg2i5_format { pub rd:u32,pub rj:u32,pub immediate:u32,pub opcode:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg2i6_format { pub rd:u32,pub rj:u32,pub immediate:u32,pub opcode:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg2i12_format { pub rd:u32,pub rj:u32,pub immediate:u32,pub opcode:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg2i14_format { pub rd:u32,pub rj:u32,pub immediate:u32,pub opcode:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg2i16_format { pub rd:u32,pub rj:u32,pub immediate:u32,pub opcode:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg2bstrd_format { pub rd:u32,pub rj:u32,pub lsbd:u32,pub msbd:u32,pub opcode:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg2csr_format { pub rd:u32,pub rj:u32,pub csr:u32,pub opcode:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg3_format { pub rd:u32,pub rj:u32,pub rk:u32,pub opcode:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg3sa2_format { pub rd:u32,pub rj:u32,pub rk:u32,pub immediate:u32,pub opcode:u32 }

#[repr(C)] pub union loongarch_instruction { pub word:u32, pub reg0i15_format:reg0i15_format, pub reg0i26_format:reg0i26_format, pub reg1i20_format:reg1i20_format, pub reg1i21_format:reg1i21_format, pub reg2_format:reg2_format, pub reg2i5_format:reg2i5_format, pub reg2i6_format:reg2i6_format, pub reg2i12_format:reg2i12_format, pub reg2i14_format:reg2i14_format, pub reg2i16_format:reg2i16_format, pub reg2bstrd_format:reg2bstrd_format, pub reg2csr_format:reg2csr_format, pub reg3_format:reg3_format, pub reg3sa2_format:reg3sa2_format }
pub const LOONGARCH_INSN_SIZE: usize = core::mem::size_of::<loongarch_instruction>();

#[repr(u32)] #[derive(Copy,Clone,Debug,PartialEq,Eq)] pub enum loongarch_gpr { LOONGARCH_GPR_ZERO=0, LOONGARCH_GPR_RA=1, LOONGARCH_GPR_TP=2, LOONGARCH_GPR_SP=3, LOONGARCH_GPR_A0=4, LOONGARCH_GPR_A1, LOONGARCH_GPR_A2, LOONGARCH_GPR_A3, LOONGARCH_GPR_A4, LOONGARCH_GPR_A5, LOONGARCH_GPR_A6, LOONGARCH_GPR_A7, LOONGARCH_GPR_T0=12, LOONGARCH_GPR_T1, LOONGARCH_GPR_T2, LOONGARCH_GPR_T3, LOONGARCH_GPR_T4, LOONGARCH_GPR_T5, LOONGARCH_GPR_T6, LOONGARCH_GPR_T7, LOONGARCH_GPR_T8, LOONGARCH_GPR_U0=21, LOONGARCH_GPR_FP, LOONGARCH_GPR_S0, LOONGARCH_GPR_S1, LOONGARCH_GPR_S2, LOONGARCH_GPR_S3, LOONGARCH_GPR_S4, LOONGARCH_GPR_S5, LOONGARCH_GPR_S6, LOONGARCH_GPR_S7, LOONGARCH_GPR_S8, LOONGARCH_GPR_MAX }

#[inline] pub fn is_imm_negative(val:u64, bit:u32)->bool { val & (1u64 << (bit-1)) != 0 }
#[inline] pub fn is_imm12_negative(val:u64)->bool { is_imm_negative(val,12) }
/* Bit-field accessors are represented by the source-level format structs above. */
extern "C" { pub fn simu_pc(regs:*mut core::ffi::c_void, insn:loongarch_instruction); pub fn simu_branch(regs:*mut core::ffi::c_void, insn:loongarch_instruction); pub fn insns_not_supported(insn:loongarch_instruction)->bool; pub fn insns_need_simulation(insn:loongarch_instruction)->bool; pub fn arch_simulate_insn(insn:loongarch_instruction, regs:*mut core::ffi::c_void); pub fn larch_insn_read(addr:*mut core::ffi::c_void, insnp:*mut u32)->i32; pub fn larch_insn_write(addr:*mut core::ffi::c_void, insn:u32)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
