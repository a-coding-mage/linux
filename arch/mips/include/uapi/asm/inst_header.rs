/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of the MIPS instruction-format UAPI header. */

/* The C header includes asm/bitfield.h; its bitfields are represented here
 * by the containing 32-bit word, preserving the instruction storage size. */

pub const MM_NOP16: u16 = 0x0c00;

macro_rules! opcode_constants {
    ($($name:ident = $value:expr),* $(,)?) => { $(pub const $name: u32 = $value;)* };
}

/* Major opcodes and the related opcode fields. */
opcode_constants! {
    spec_op=0, bcond_op=1, j_op=2, jal_op=3, beq_op=4, bne_op=5, blez_op=6,
    bgtz_op=7, addi_op=8, pop10_op=8, addiu_op=9, slti_op=10, sltiu_op=11,
    andi_op=12, ori_op=13, xori_op=14, lui_op=15, cop0_op=16, cop1_op=17,
    cop2_op=18, cop1x_op=19, beql_op=20, bnel_op=21, blezl_op=22, bgtzl_op=23,
    daddi_op=24, pop30_op=24, daddiu_op=25, ldl_op=26, ldr_op=27,
    spec2_op=28, jalx_op=29, mdmx_op=30, msa_op=30, spec3_op=31,
    lb_op=32, lh_op=33, lwl_op=34, lw_op=35, lbu_op=36, lhu_op=37,
    lwr_op=38, lwu_op=39, sb_op=40, sh_op=41, swl_op=42, sw_op=43,
    sdl_op=44, sdr_op=45, swr_op=46, cache_op=47, ll_op=48, lwc1_op=49,
    lwc2_op=50, bc6_op=50, pref_op=51, lld_op=52, ldc1_op=53, ldc2_op=54,
    pop66_op=54, ld_op=55, sc_op=56, swc1_op=57, swc2_op=58, balc6_op=58,
    major_3b_op=59, scd_op=60, sdc1_op=61, sdc2_op=62, pop76_op=62, sd_op=63
}

/* Instruction format records.  The C implementation uses compiler bitfields;
 * a raw word is the portable Rust representation of each format. */
macro_rules! format_structs { ($($name:ident),* $(,)?) => { $(#[repr(C)] #[derive(Copy, Clone, Default)] pub struct $name { pub word: u32 })* }; }
format_structs!(j_format,i_format,u_format,c_format,r_format,c0r_format,mfmc0_format,
    co_format,p_format,f_format,ma_format,b_format,ps_format,v_format,msa_mi10_format,
    dsp_format,mxu_lx_format,spec3_format,fb_format,fp0_format,mm_fp0_format,fp1_format,
    mm_fp1_format,mm_fp2_format,mm_fp3_format,mm_fp4_format,mm_fp5_format,fp6_format,
    mm_fp6_format,mm_i_format,mm_m_format,mm_x_format,mm_a_format,mm_b0_format,mm_b1_format,
    mm16_m_format,mm16_rb_format,mm16_r3_format,mm16_r5_format,loongson3_lswc2_format,
    loongson3_lsdc2_format,loongson3_lscsr_format,m16e_rr,m16e_jal,m16e_i64,m16e_ri64,
    m16e_ri,m16e_rri,m16e_i8);

#[repr(C)]
pub union mips_instruction {
    pub word: u32,
    pub halfword: [u16; 2],
    pub byte: [u8; 4],
    pub j_format: j_format, pub i_format: i_format, pub u_format: u_format,
    pub c_format: c_format, pub r_format: r_format, pub c0r_format: c0r_format,
    pub mfmc0_format: mfmc0_format, pub co_format: co_format, pub p_format: p_format,
    pub f_format: f_format, pub ma_format: ma_format, pub msa_mi10_format: msa_mi10_format,
    pub b_format: b_format, pub ps_format: ps_format, pub v_format: v_format,
    pub dsp_format: dsp_format, pub spec3_format: spec3_format, pub fb_format: fb_format,
    pub fp0_format: fp0_format, pub mm_fp0_format: mm_fp0_format, pub fp1_format: fp1_format,
    pub mm_fp1_format: mm_fp1_format, pub mm_fp2_format: mm_fp2_format,
    pub mm_fp3_format: mm_fp3_format, pub mm_fp4_format: mm_fp4_format,
    pub mm_fp5_format: mm_fp5_format, pub fp6_format: fp6_format, pub mm_fp6_format: mm_fp6_format,
    pub mm_i_format: mm_i_format, pub mm_m_format: mm_m_format, pub mm_x_format: mm_x_format,
    pub mm_a_format: mm_a_format, pub mm_b0_format: mm_b0_format, pub mm_b1_format: mm_b1_format,
    pub mm16_m_format: mm16_m_format, pub mm16_rb_format: mm16_rb_format,
    pub mm16_r3_format: mm16_r3_format, pub mm16_r5_format: mm16_r5_format,
    pub loongson3_lswc2_format: loongson3_lswc2_format,
    pub loongson3_lsdc2_format: loongson3_lsdc2_format,
    pub loongson3_lscsr_format: loongson3_lscsr_format, pub mxu_lx_format: mxu_lx_format,
}

#[repr(C)]
pub union mips16e_instruction { pub full: u16, pub rr: m16e_rr, pub jal: m16e_jal,
    pub i64: m16e_i64, pub ri64: m16e_ri64, pub ri: m16e_ri, pub rri: m16e_rri, pub i8: m16e_i8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
