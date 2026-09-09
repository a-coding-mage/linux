/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of the RISC-V instruction header. */

pub const fn genmask(h: u32, l: u32) -> u32 { ((u32::MAX >> (31 - h)) & (u32::MAX << l)) }

pub const RV_INSN_FUNCT3_MASK: u32 = genmask(14, 12); pub const RV_INSN_FUNCT3_OPOFF: u32 = 12;
pub const RV_INSN_OPCODE_MASK: u32 = genmask(6, 0); pub const RV_INSN_OPCODE_OPOFF: u32 = 0;
pub const RV_INSN_FUNCT12_OPOFF: u32 = 20;
pub const RV_I_IMM_SIGN_OPOFF:u32=31; pub const RV_I_IMM_11_0_OPOFF:u32=20; pub const RV_I_IMM_SIGN_OFF:u32=12; pub const RV_I_IMM_11_0_OFF:u32=0; pub const RV_I_IMM_11_0_MASK:u32=genmask(11,0);
pub const RV_J_IMM_SIGN_OPOFF:u32=31; pub const RV_J_IMM_10_1_OPOFF:u32=21; pub const RV_J_IMM_11_OPOFF:u32=20; pub const RV_J_IMM_19_12_OPOFF:u32=12; pub const RV_J_IMM_SIGN_OFF:u32=20; pub const RV_J_IMM_10_1_OFF:u32=1; pub const RV_J_IMM_11_OFF:u32=11; pub const RV_J_IMM_19_12_OFF:u32=12; pub const RV_J_IMM_10_1_MASK:u32=genmask(9,0); pub const RV_J_IMM_11_MASK:u32=1; pub const RV_J_IMM_19_12_MASK:u32=genmask(7,0);
pub const RV_U_IMM_SIGN_OPOFF:u32=31; pub const RV_U_IMM_31_12_OPOFF:u32=0; pub const RV_U_IMM_31_12_MASK:u32=genmask(31,12);
pub const RV_B_IMM_SIGN_OPOFF:u32=31; pub const RV_B_IMM_10_5_OPOFF:u32=25; pub const RV_B_IMM_4_1_OPOFF:u32=8; pub const RV_B_IMM_11_OPOFF:u32=7; pub const RV_B_IMM_SIGN_OFF:u32=12; pub const RV_B_IMM_10_5_OFF:u32=5; pub const RV_B_IMM_4_1_OFF:u32=1; pub const RV_B_IMM_11_OFF:u32=11; pub const RV_B_IMM_10_5_MASK:u32=genmask(5,0); pub const RV_B_IMM_4_1_MASK:u32=genmask(3,0); pub const RV_B_IMM_11_MASK:u32=1;
pub const RVG_RS1_OPOFF:u32=15; pub const RVG_RS2_OPOFF:u32=20; pub const RVG_RD_OPOFF:u32=7; pub const RVG_RS1_MASK:u32=31; pub const RVG_RS2_MASK:u32=31; pub const RVG_RD_MASK:u32=31;
pub const RVC_J_IMM_SIGN_OPOFF:u32=12; pub const RVC_J_IMM_4_OPOFF:u32=11; pub const RVC_J_IMM_9_8_OPOFF:u32=9; pub const RVC_J_IMM_10_OPOFF:u32=8; pub const RVC_J_IMM_6_OPOFF:u32=7; pub const RVC_J_IMM_7_OPOFF:u32=6; pub const RVC_J_IMM_3_1_OPOFF:u32=3; pub const RVC_J_IMM_5_OPOFF:u32=2; pub const RVC_J_IMM_SIGN_OFF:u32=11; pub const RVC_J_IMM_4_OFF:u32=4; pub const RVC_J_IMM_9_8_OFF:u32=8; pub const RVC_J_IMM_10_OFF:u32=10; pub const RVC_J_IMM_6_OFF:u32=6; pub const RVC_J_IMM_7_OFF:u32=7; pub const RVC_J_IMM_3_1_OFF:u32=1; pub const RVC_J_IMM_5_OFF:u32=5; pub const RVC_J_IMM_4_MASK:u32=1; pub const RVC_J_IMM_9_8_MASK:u32=3; pub const RVC_J_IMM_10_MASK:u32=1; pub const RVC_J_IMM_6_MASK:u32=1; pub const RVC_J_IMM_7_MASK:u32=1; pub const RVC_J_IMM_3_1_MASK:u32=7; pub const RVC_J_IMM_5_MASK:u32=1;
pub const RVC_B_IMM_SIGN_OPOFF:u32=12; pub const RVC_B_IMM_4_3_OPOFF:u32=10; pub const RVC_B_IMM_7_6_OPOFF:u32=5; pub const RVC_B_IMM_2_1_OPOFF:u32=3; pub const RVC_B_IMM_5_OPOFF:u32=2; pub const RVC_B_IMM_SIGN_OFF:u32=8; pub const RVC_B_IMM_4_3_OFF:u32=3; pub const RVC_B_IMM_7_6_OFF:u32=6; pub const RVC_B_IMM_2_1_OFF:u32=1; pub const RVC_B_IMM_5_OFF:u32=5; pub const RVC_B_IMM_4_3_MASK:u32=3; pub const RVC_B_IMM_7_6_MASK:u32=3; pub const RVC_B_IMM_2_1_MASK:u32=3; pub const RVC_B_IMM_5_MASK:u32=1;
pub const RVC_INSN_FUNCT4_MASK:u32=0xf000; pub const RVC_INSN_FUNCT4_OPOFF:u32=12; pub const RVC_INSN_FUNCT3_MASK:u32=0xe000; pub const RVC_INSN_FUNCT3_OPOFF:u32=13; pub const RVC_INSN_J_RS1_MASK:u32=0xf80; pub const RVC_INSN_J_RS2_MASK:u32=0x7c; pub const RVC_INSN_OPCODE_MASK:u32=3;
pub const RVC_C0_RS1_OPOFF:u32=7; pub const RVC_C0_RS2_OPOFF:u32=2; pub const RVC_C0_RD_OPOFF:u32=2; pub const RVC_C1_RS1_OPOFF:u32=7; pub const RVC_C1_RS2_OPOFF:u32=2; pub const RVC_C1_RD_OPOFF:u32=7; pub const RVC_C2_RS1_OPOFF:u32=7; pub const RVC_C2_RS2_OPOFF:u32=2; pub const RVC_C2_RD_OPOFF:u32=7; pub const RVC_C2_RS1_MASK:u32=31;

pub const RVG_OPCODE_FENCE:u32=0x0f; pub const RVG_OPCODE_AUIPC:u32=0x17; pub const RVG_OPCODE_BRANCH:u32=0x63; pub const RVG_OPCODE_JALR:u32=0x67; pub const RVG_OPCODE_JAL:u32=0x6f; pub const RVG_OPCODE_SYSTEM:u32=0x73; pub const RVG_SYSTEM_CSR_OFF:u32=20; pub const RVG_SYSTEM_CSR_MASK:u32=0x1fff;
pub const RVFDQ_FL_FS_WIDTH_OFF:u32=12; pub const RVFDQ_FL_FS_WIDTH_MASK:u32=7; pub const RVFDQ_FL_FS_WIDTH_W:u32=2; pub const RVFDQ_FL_FS_WIDTH_D:u32=3; pub const RVFDQ_LS_FS_WIDTH_Q:u32=4; pub const RVFDQ_OPCODE_FL:u32=7; pub const RVFDQ_OPCODE_FS:u32=0x27; pub const RVV_OPCODE_VECTOR:u32=0x57; pub const RVV_VL_VS_WIDTH_8:u32=0; pub const RVV_VL_VS_WIDTH_16:u32=5; pub const RVV_VL_VS_WIDTH_32:u32=6; pub const RVV_VL_VS_WIDTH_64:u32=7; pub const RVV_OPCODE_VL:u32=7; pub const RVV_OPCODE_VS:u32=0x27; pub const RVC_OPCODE_C0:u32=0; pub const RVC_OPCODE_C1:u32=1; pub const RVC_OPCODE_C2:u32=2;
pub const RVG_FUNCT3_JALR:u32=0; pub const RVG_FUNCT3_BEQ:u32=0; pub const RVG_FUNCT3_BNE:u32=1; pub const RVG_FUNCT3_BLT:u32=4; pub const RVG_FUNCT3_BGE:u32=5; pub const RVG_FUNCT3_BLTU:u32=6; pub const RVG_FUNCT3_BGEU:u32=7;
pub const RVC_FUNCT3_C_BEQZ:u32=6; pub const RVC_FUNCT3_C_BNEZ:u32=7; pub const RVC_FUNCT3_C_J:u32=5; pub const RVC_FUNCT3_C_JAL:u32=1; pub const RVC_FUNCT4_C_JR:u32=8; pub const RVC_FUNCT4_C_JALR:u32=9; pub const RVC_FUNCT4_C_EBREAK:u32=9; pub const RVG_FUNCT12_EBREAK:u32=1; pub const RVG_FUNCT12_SRET:u32=0x102;

pub const INSN_MATCH_LB:u32=3; pub const INSN_MATCH_LH:u32=0x1003; pub const INSN_MATCH_LW:u32=0x2003; pub const INSN_MATCH_LD:u32=0x3003; pub const INSN_MATCH_LBU:u32=0x4003; pub const INSN_MATCH_LHU:u32=0x5003; pub const INSN_MATCH_LWU:u32=0x6003; pub const INSN_MATCH_SB:u32=0x23; pub const INSN_MATCH_SH:u32=0x1023; pub const INSN_MATCH_SW:u32=0x2023; pub const INSN_MATCH_SD:u32=0x3023; pub const INSN_MASK_LOADSTORE:u32=0x707f;
pub const INSN_16BIT_MASK:u32=3; pub const INSN_OPCODE_MASK:u32=0x7c; pub const INSN_OPCODE_SHIFT:u32=2; pub const INSN_OPCODE_SYSTEM:u32=28; pub const INSN_MASK_WFI:u32=0xffff_ffff; pub const INSN_MATCH_WFI:u32=0x10500073; pub const INSN_MASK_WRS:u32=0xffff_ffff; pub const INSN_MATCH_WRS:u32=0x00d00073;

pub const RVG_MATCH_AUIPC:u32=RVG_OPCODE_AUIPC; pub const RVG_MATCH_JALR:u32=(RVG_FUNCT3_JALR<<12)|RVG_OPCODE_JALR; pub const RVG_MATCH_JAL:u32=RVG_OPCODE_JAL;
pub const RVG_MATCH_FENCE:u32=RVG_OPCODE_FENCE; pub const RVG_MATCH_BEQ:u32=RVG_OPCODE_BRANCH; pub const RVG_MATCH_BNE:u32=(1<<12)|RVG_OPCODE_BRANCH; pub const RVG_MATCH_BLT:u32=(4<<12)|RVG_OPCODE_BRANCH; pub const RVG_MATCH_BGE:u32=(5<<12)|RVG_OPCODE_BRANCH; pub const RVG_MATCH_BLTU:u32=(6<<12)|RVG_OPCODE_BRANCH; pub const RVG_MATCH_BGEU:u32=(7<<12)|RVG_OPCODE_BRANCH;
pub const RVG_MATCH_EBREAK:u32=(1<<20)|RVG_OPCODE_SYSTEM; pub const RVG_MATCH_SRET:u32=(0x102<<20)|RVG_OPCODE_SYSTEM;
pub const RVC_MATCH_C_BEQZ:u32=(6<<13)|1; pub const RVC_MATCH_C_BNEZ:u32=(7<<13)|1; pub const RVC_MATCH_C_J:u32=(5<<13)|1; pub const RVC_MATCH_C_JAL:u32=(1<<13)|1; pub const RVC_MATCH_C_JR:u32=(8<<12)|2; pub const RVC_MATCH_C_JALR:u32=(9<<12)|2; pub const RVC_MATCH_C_EBREAK:u32=(9<<12)|2;
pub const RVG_MASK_AUIPC:u32=0x7f; pub const RVG_MASK_JALR:u32=0x707f; pub const RVG_MASK_JAL:u32=0x7f; pub const RVG_MASK_FENCE:u32=0x7f; pub const RVG_MASK_BRANCH:u32=0x707f; pub const RVC_MASK_C_JALR:u32=0xf07f; pub const RVC_MASK_C_JR:u32=0xf07f; pub const RVC_MASK_C_JAL:u32=0xe003; pub const RVC_MASK_C_J:u32=0xe003; pub const RVC_MASK_C_BEQZ:u32=0xe003; pub const RVC_MASK_C_BNEZ:u32=0xe003; pub const RVC_MASK_C_EBREAK:u32=0xffff; pub const RVG_MASK_EBREAK:u32=0xffff_ffff; pub const RVG_MASK_SRET:u32=0xffff_ffff;

pub const fn riscv_insn_is_auipc(c:u32)->bool{c&RVG_MASK_AUIPC==RVG_MATCH_AUIPC} pub const fn riscv_insn_is_jalr(c:u32)->bool{c&RVG_MASK_JALR==RVG_MATCH_JALR} pub const fn riscv_insn_is_jal(c:u32)->bool{c&0x7f==RVG_MATCH_JAL}
pub const fn riscv_insn_is_c_j(c:u32)->bool{c&RVC_MASK_C_J==RVC_MATCH_C_J} pub const fn riscv_insn_is_beq(c:u32)->bool{c&RVG_MASK_BRANCH==RVG_MATCH_BEQ} pub const fn riscv_insn_is_bne(c:u32)->bool{c&RVG_MASK_BRANCH==RVG_MATCH_BNE} pub const fn riscv_insn_is_blt(c:u32)->bool{c&RVG_MASK_BRANCH==RVG_MATCH_BLT} pub const fn riscv_insn_is_bge(c:u32)->bool{c&RVG_MASK_BRANCH==RVG_MATCH_BGE} pub const fn riscv_insn_is_bltu(c:u32)->bool{c&RVG_MASK_BRANCH==RVG_MATCH_BLTU} pub const fn riscv_insn_is_bgeu(c:u32)->bool{c&RVG_MASK_BRANCH==RVG_MATCH_BGEU}
pub const fn riscv_insn_is_c_beqz(c:u32)->bool{c&RVC_MASK_C_BEQZ==RVC_MATCH_C_BEQZ} pub const fn riscv_insn_is_c_bnez(c:u32)->bool{c&RVC_MASK_C_BNEZ==RVC_MATCH_C_BNEZ} pub const fn riscv_insn_is_c_ebreak(c:u32)->bool{c&RVC_MASK_C_EBREAK==RVC_MATCH_C_EBREAK} pub const fn riscv_insn_is_ebreak(c:u32)->bool{c==RVG_MATCH_EBREAK} pub const fn riscv_insn_is_sret(c:u32)->bool{c==RVG_MATCH_SRET} pub const fn riscv_insn_is_fence(c:u32)->bool{c&0x7f==RVG_MATCH_FENCE}
pub const fn riscv_insn_is_system(c:u32)->bool{c&RV_INSN_OPCODE_MASK==RVG_OPCODE_SYSTEM} pub const fn riscv_insn_is_branch(c:u32)->bool{c&RV_INSN_OPCODE_MASK==RVG_OPCODE_BRANCH}
pub const fn riscv_insn_is_c_jr(c:u32)->bool{c&RVC_MASK_C_JR==RVC_MATCH_C_JR && c&RVC_INSN_J_RS1_MASK!=0} pub const fn riscv_insn_is_c_jalr(c:u32)->bool{c&RVC_MASK_C_JALR==RVC_MATCH_C_JALR && c&RVC_INSN_J_RS1_MASK!=0}

pub const fn rv_x_mask(x:u32,s:u32,mask:u32)->u32 {(x>>s)&mask}
pub const fn rv_x(x:u32,s:u32,n:u32)->u32 { rv_x_mask(x,s,(1<<n)-1) }
pub const fn rv_imm_sign(x:u32)->u32 { 0u32.wrapping_sub((x>>31)&1) }
pub const fn rvc_imm_sign(x:u32)->u32 { 0u32.wrapping_sub((x>>12)&1) }
pub const fn insn_is_16bit(insn:u32)->bool {(insn&3)!=3}
pub const fn insn_len(insn:u32)->u32 {if insn_is_16bit(insn){2}else{4}}
pub const fn get_funct3(insn:u32)->u32 {(insn>>12)&7}
pub const fn rvc_rs1s(insn:u32)->u32 {8+rv_x(insn,7,3)}
pub const fn rvc_rs2s(insn:u32)->u32 {8+rv_x(insn,2,3)}
pub const fn rvc_rs2(insn:u32)->u32 {rv_x(insn,2,5)}

pub const INSN_MATCH_C_LD:u32=0x6000; pub const INSN_MATCH_C_SD:u32=0xe000; pub const INSN_MATCH_C_LW:u32=0x4000; pub const INSN_MATCH_C_SW:u32=0xc000; pub const INSN_MATCH_C_LDSP:u32=0x6002; pub const INSN_MATCH_C_SDSP:u32=0xe002; pub const INSN_MATCH_C_LWSP:u32=0x4002; pub const INSN_MATCH_C_SWSP:u32=0xc002; pub const INSN_MASK_C:u32=0xe003;
pub const INSN_MATCH_C_LHU:u32=0x8400; pub const INSN_MATCH_C_LH:u32=0x8440; pub const INSN_MATCH_C_SH:u32=0x8c00; pub const INSN_MASK_C_LH:u32=0xfc43;
pub const INSN_MATCH_FLW:u32=0x2007; pub const INSN_MATCH_FLD:u32=0x3007; pub const INSN_MATCH_FLQ:u32=0x4007; pub const INSN_MATCH_FSW:u32=0x2027; pub const INSN_MATCH_FSD:u32=0x3027; pub const INSN_MATCH_FSQ:u32=0x4027; pub const INSN_MASK_FP:u32=0x707f;
pub const INSN_MATCH_CSRRW:u32=0x1073; pub const INSN_MATCH_CSRRS:u32=0x2073; pub const INSN_MATCH_CSRRC:u32=0x3073; pub const INSN_MATCH_CSRRWI:u32=0x5073; pub const INSN_MATCH_CSRRSI:u32=0x6073; pub const INSN_MATCH_CSRRCI:u32=0x7073; pub const INSN_MASK_CSR:u32=0x707f;
pub const MASK_FUNCT3:u32=0x7000; pub const SH_RD:u32=7; pub const SH_RS1:u32=15; pub const SH_RS2:u32=20; pub const SH_RS2C:u32=2; pub const MASK_RX:u32=0x1f;
pub const fn get_rs1(insn:u32, regs:*const u8)->u64 {unsafe{*((regs as usize + (((insn>>SH_RS1)&MASK_RX)<<3)) as *const u64)}}
pub const fn imm_i(insn:u32)->i32 {(insn as i32)>>20}
pub const fn imm_s(insn:u32)->i32 {((insn as i32)>>25<<5)|(((insn>>7)&0x1f) as i32)}

pub const fn rv_extract_jtype_imm(x:u32)->u32 {rv_x_mask(x,21,0x3ff)<<1 | rv_x_mask(x,20,1)<<11 | rv_x_mask(x,12,0xff)<<12 | rv_imm_sign(x)<<20}
pub const fn rv_extract_utype_imm(x:u32)->u32 {rv_x_mask(x,0,0xfffff000)}
pub const fn rv_extract_itype_imm(x:u32)->u32 {rv_x_mask(x,20,0xfff) | rv_imm_sign(x)<<12}
pub const fn rv_extract_btype_imm(x:u32)->u32 {rv_x_mask(x,8,0xf)<<1 | rv_x_mask(x,25,0x3f)<<5 | rv_x_mask(x,7,1)<<11 | rv_imm_sign(x)<<12}
pub const fn rvc_extract_jtype_imm(x:u32)->u32 {rv_x(x,3,3)<<1|rv_x(x,11,1)<<4|rv_x(x,2,1)<<5|rv_x(x,7,1)<<6|rv_x(x,6,1)<<7|rv_x(x,9,2)<<8|rv_x(x,8,1)<<10|rvc_imm_sign(x)<<11}
pub const fn rvc_extract_btype_imm(x:u32)->u32 {rv_x(x,3,2)<<1|rv_x(x,10,2)<<3|rv_x(x,2,1)<<5|rv_x(x,5,2)<<6|rvc_imm_sign(x)<<8}
pub const fn riscv_insn_extract_jtype_imm(insn:u32)->i32 {rv_extract_jtype_imm(insn) as i32}
pub unsafe fn riscv_insn_insert_jtype_imm(insn:*mut u32, imm:i32) {let v=imm as u32; *insn=(*insn & !0xfffff000) | (rv_x(v,1,10)<<21)|(rv_x(v,11,1)<<20)|(rv_x(v,12,8)<<12)|(rv_x(v,20,1)<<31);}
pub const fn riscv_insn_extract_utype_itype_imm(u:u32,i:u32)->i32 {(rv_extract_utype_imm(u) as i32).wrapping_add(rv_extract_itype_imm(i) as i32)}
pub unsafe fn riscv_insn_insert_utype_itype_imm(u:*mut u32,i:*mut u32,imm:i32) {let v=imm as u32; *u&=!0xfffff000; *i&=!(0xfff<<20); *u|=(v&0xfffff000)+((v&2048)<<1); *i|=(v&0xfff)<<20;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
