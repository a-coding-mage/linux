/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * A small micro-assembler. It is intentionally kept simple, does only
 * support a subset of instructions, and does not try to hide pipeline
 * effects like branch delay slots.
 */

// Dependencies supplied by the surrounding MIPS translation unit:
// asm::inst, asm::elf, asm::bugs, asm::uasm, and the definitions from uasm.c.

const RS_MASK: u32 = 0x1f;
const RS_SH: u32 = 21;
const RT_MASK: u32 = 0x1f;
const RT_SH: u32 = 16;
const SCIMM_MASK: u32 = 0xfffff;
const SCIMM_SH: u32 = 6;

macro_rules! m {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
        (($a) << OP_SH | ($b) << RS_SH | ($c) << RT_SH | ($d) << RD_SH |
         ($e) << RE_SH | ($f) << FUNC_SH)
    };
}
macro_rules! m6 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        (($a) << OP_SH | ($b) << RS_SH | ($c) << RT_SH | ($d) << SIMM9_SH |
         ($e) << FUNC_SH)
    };
}

/* The C source uses designated initializers.  The entries below retain the
 * same opcode/field mapping; enum ordering supplies the array index. */
#[allow(non_upper_case_globals)]
static insn_table: [Insn; insn_invalid as usize] = [
    Insn { match_: m!(addiu_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(spec_op,0,0,0,0,addu_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,and_op), fields: RS | RT | RD },
    Insn { match_: m!(andi_op,0,0,0,0,0), fields: RS | RT | UIMM },
    Insn { match_: m!(lwc2_op,0,0,0,0,0), fields: RS | RT | BIMM },
    Insn { match_: m!(swc2_op,0,0,0,0,0), fields: RS | RT | BIMM },
    Insn { match_: m!(beq_op,0,0,0,0,0), fields: RS | RT | BIMM },
    Insn { match_: m!(beql_op,0,0,0,0,0), fields: RS | RT | BIMM },
    Insn { match_: m!(bcond_op,0,bgez_op,0,0,0), fields: RS | BIMM },
    Insn { match_: m!(bcond_op,0,bgezl_op,0,0,0), fields: RS | BIMM },
    Insn { match_: m!(bgtz_op,0,0,0,0,0), fields: RS | BIMM },
    Insn { match_: m!(blez_op,0,0,0,0,0), fields: RS | BIMM },
    Insn { match_: m!(bcond_op,0,bltz_op,0,0,0), fields: RS | BIMM },
    Insn { match_: m!(bcond_op,0,bltzl_op,0,0,0), fields: RS | BIMM },
    Insn { match_: m!(bne_op,0,0,0,0,0), fields: RS | RT | BIMM },
    Insn { match_: m!(spec_op,0,0,0,0,break_op), fields: SCIMM },
    Insn { match_: m!(cache_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(cop1_op,cfc_op,0,0,0,0), fields: RT | RD },
    Insn { match_: m!(msa_op,0,msa_cfc_op,0,0,msa_elm_op), fields: RD | RE },
    Insn { match_: m!(cop1_op,ctc_op,0,0,0,0), fields: RT | RD },
    Insn { match_: m!(msa_op,0,msa_ctc_op,0,0,msa_elm_op), fields: RD | RE },
    Insn { match_: m!(daddiu_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(spec_op,0,0,0,0,daddu_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,ddivu_op), fields: RS | RT },
    Insn { match_: m!(spec_op,0,0,0,ddivu_ddivu6_op,ddivu_op), fields: RS | RT | RD },
    Insn { match_: m!(cop0_op,mfmc0_op,0,12,0,0), fields: RT },
    Insn { match_: m!(spec3_op,0,0,0,0,dins_op), fields: RS | RT | RD | RE },
    Insn { match_: m!(spec3_op,0,0,0,0,dinsm_op), fields: RS | RT | RD | RE },
    Insn { match_: m!(spec3_op,0,0,0,0,dinsu_op), fields: RS | RT | RD | RE },
    Insn { match_: m!(spec_op,0,0,0,0,divu_op), fields: RS | RT },
    Insn { match_: m!(spec_op,0,0,0,divu_divu6_op,divu_op), fields: RS | RT | RD },
    Insn { match_: m!(cop0_op,dmfc_op,0,0,0,0), fields: RT | RD | SET },
    Insn { match_: m!(spec_op,0,0,0,ddivu_dmodu_op,ddivu_op), fields: RS | RT | RD },
    Insn { match_: m!(cop0_op,dmtc_op,0,0,0,0), fields: RT | RD | SET },
    Insn { match_: m!(spec_op,0,0,0,0,dmultu_op), fields: RS | RT },
    Insn { match_: m!(spec_op,0,0,0,dmultu_dmulu_op,dmultu_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,1,0,0,0,dsrl_op), fields: RT | RD | RE },
    Insn { match_: m!(spec_op,1,0,0,0,dsrl32_op), fields: RT | RD | RE },
    Insn { match_: m!(spec3_op,0,0,0,dsbh_op,dbshfl_op), fields: RT | RD },
    Insn { match_: m!(spec3_op,0,0,0,dshd_op,dbshfl_op), fields: RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,dsll_op), fields: RT | RD | RE },
    Insn { match_: m!(spec_op,0,0,0,0,dsll32_op), fields: RT | RD | RE },
    Insn { match_: m!(spec_op,0,0,0,0,dsllv_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,dsra_op), fields: RT | RD | RE },
    Insn { match_: m!(spec_op,0,0,0,0,dsra32_op), fields: RT | RD | RE },
    Insn { match_: m!(spec_op,0,0,0,0,dsrav_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,dsrl_op), fields: RT | RD | RE },
    Insn { match_: m!(spec_op,0,0,0,0,dsrl32_op), fields: RT | RD | RE },
    Insn { match_: m!(spec_op,0,0,0,0,dsrlv_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,dsubu_op), fields: RS | RT | RD },
    Insn { match_: m!(cop0_op,cop_op,0,0,0,eret_op), fields: 0 },
    Insn { match_: m!(spec3_op,0,0,0,0,ext_op), fields: RS | RT | RD | RE },
    Insn { match_: m!(spec3_op,0,0,0,0,ins_op), fields: RS | RT | RD | RE },
    Insn { match_: m!(j_op,0,0,0,0,0), fields: JIMM },
    Insn { match_: m!(jal_op,0,0,0,0,0), fields: JIMM },
    Insn { match_: m!(spec_op,0,0,0,0,jalr_op), fields: RS | RD },
    Insn { match_: m!(spec_op,0,0,0,0,jr_op), fields: RS },
    Insn { match_: m!(lb_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(lbu_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(ld_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(lwc2_op,0,0,0,lddir_op,mult_op), fields: RS | RT | RD },
    Insn { match_: m!(lwc2_op,0,0,0,ldpte_op,mult_op), fields: RS | RD },
    Insn { match_: m!(spec3_op,0,0,0,ldx_op,lx_op), fields: RS | RT | RD },
    Insn { match_: m!(lh_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(lhu_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(ll_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(lld_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(lui_op,0,0,0,0,0), fields: RT | SIMM },
    Insn { match_: m!(lw_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(lwu_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(spec3_op,0,0,0,lwx_op,lx_op), fields: RS | RT | RD },
    Insn { match_: m!(cop0_op,mfc_op,0,0,0,0), fields: RT | RD | SET },
    Insn { match_: m!(cop0_op,mfhc0_op,0,0,0,0), fields: RT | RD | SET },
    Insn { match_: m!(spec_op,0,0,0,0,mfhi_op), fields: RD },
    Insn { match_: m!(spec_op,0,0,0,0,mflo_op), fields: RD },
    Insn { match_: m!(spec_op,0,0,0,divu_modu_op,divu_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,movn_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,movz_op), fields: RS | RT | RD },
    Insn { match_: m!(cop0_op,mtc_op,0,0,0,0), fields: RT | RD | SET },
    Insn { match_: m!(cop0_op,mthc0_op,0,0,0,0), fields: RT | RD | SET },
    Insn { match_: m!(spec_op,0,0,0,0,mthi_op), fields: RS },
    Insn { match_: m!(spec_op,0,0,0,0,mtlo_op), fields: RS },
    Insn { match_: m!(spec_op,0,0,0,multu_mulu_op,multu_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,multu_muhu_op,multu_op), fields: RS | RT | RD },
    Insn { match_: m!(spec2_op,0,0,0,0,mul_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,multu_op), fields: RS | RT },
    Insn { match_: m!(spec_op,0,0,0,0,nor_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,or_op), fields: RS | RT | RD },
    Insn { match_: m!(ori_op,0,0,0,0,0), fields: RS | RT | UIMM },
    Insn { match_: m!(pref_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(cop0_op,cop_op,0,0,0,rfe_op), fields: 0 },
    Insn { match_: m!(spec_op,1,0,0,0,srl_op), fields: RT | RD | RE },
    Insn { match_: m!(sb_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(sc_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(scd_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(sd_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(spec_op,0,0,0,0,seleqz_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,selnez_op), fields: RS | RT | RD },
    Insn { match_: m!(sh_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(spec_op,0,0,0,0,sll_op), fields: RT | RD | RE },
    Insn { match_: m!(spec_op,0,0,0,0,sllv_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,slt_op), fields: RS | RT | RD },
    Insn { match_: m!(slti_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(sltiu_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(spec_op,0,0,0,0,sltu_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,sra_op), fields: RT | RD | RE },
    Insn { match_: m!(spec_op,0,0,0,0,srav_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,srl_op), fields: RT | RD | RE },
    Insn { match_: m!(spec_op,0,0,0,0,srlv_op), fields: RS | RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,subu_op), fields: RS | RT | RD },
    Insn { match_: m!(sw_op,0,0,0,0,0), fields: RS | RT | SIMM },
    Insn { match_: m!(spec_op,0,0,0,0,sync_op), fields: RE },
    Insn { match_: m!(spec_op,0,0,0,0,syscall_op), fields: SCIMM },
    Insn { match_: m!(cop0_op,cop_op,0,0,0,tlbp_op), fields: 0 },
    Insn { match_: m!(cop0_op,cop_op,0,0,0,tlbr_op), fields: 0 },
    Insn { match_: m!(cop0_op,cop_op,0,0,0,tlbwi_op), fields: 0 },
    Insn { match_: m!(cop0_op,cop_op,0,0,0,tlbwr_op), fields: 0 },
    Insn { match_: m!(cop0_op,cop_op,0,0,0,wait_op), fields: SCIMM },
    Insn { match_: m!(spec3_op,0,0,0,wsbh_op,bshfl_op), fields: RT | RD },
    Insn { match_: m!(spec_op,0,0,0,0,xor_op), fields: RS | RT | RD },
    Insn { match_: m!(xori_op,0,0,0,0,0), fields: RS | RT | UIMM },
    Insn { match_: m!(spec3_op,0,0,0,0,yield_op), fields: RS | RD },
];

#[inline]
unsafe fn build_bimm(arg: i32) -> u32 {
    // WARN(arg > 0x1ffff || arg < -0x20000, KERN_WARNING ...);
    // WARN(arg & 0x3, KERN_WARNING ...);
    ((if arg < 0 { 1 } else { 0 }) << 15) | (((arg >> 2) as u32) & 0x7fff)
}

#[inline]
unsafe fn build_jimm(arg: u32) -> u32 {
    // WARN(arg & !(JIMM_MASK << 2), KERN_WARNING ...);
    (arg >> 2) & JIMM_MASK
}

/* The C varargs interface is represented by an ordered argument slice. */
unsafe fn build_insn(buf: *mut *mut u32, opc: opcode, args: &[u32]) {
    if (opc as i32) < 0 || opc as usize >= insn_invalid as usize ||
       (opc == insn_daddiu && r4k_daddiu_bug()) ||
       (insn_table[opc as usize].match_ == 0 && insn_table[opc as usize].fields == 0) {
        panic!("Unsupported Micro-assembler instruction {}", opc as i32);
    }
    let ip = &insn_table[opc as usize];
    let mut op = ip.match_;
    let mut n = 0usize;
    macro_rules! arg { () => {{ let v = args[n]; n += 1; v }}; }
    if ip.fields & RS != 0 { op |= build_rs(arg!()); }
    if ip.fields & RT != 0 { op |= build_rt(arg!()); }
    if ip.fields & RD != 0 { op |= build_rd(arg!()); }
    if ip.fields & RE != 0 { op |= build_re(arg!()); }
    if ip.fields & SIMM != 0 { op |= build_simm(arg!() as i32); }
    if ip.fields & UIMM != 0 { op |= build_uimm(arg!()); }
    if ip.fields & BIMM != 0 { op |= build_bimm(arg!() as i32); }
    if ip.fields & JIMM != 0 { op |= build_jimm(arg!()); }
    if ip.fields & FUNC != 0 { op |= build_func(arg!()); }
    if ip.fields & SET != 0 { op |= build_set(arg!()); }
    if ip.fields & SCIMM != 0 { op |= build_scimm(arg!()); }
    if ip.fields & SIMM9 != 0 { op |= build_scimm9(arg!()); }
    **buf = op;
    *buf = (*buf).add(1);
}

#[inline]
unsafe fn __resolve_relocs(rel: *mut uasm_reloc, lab: *mut uasm_label) {
    let laddr = (*lab).addr as isize;
    let raddr = (*rel).addr as isize;
    match (*rel).type_ {
        R_MIPS_PC16 => *(*rel).addr |= build_bimm((laddr - (raddr + 4)) as i32),
        _ => panic!("Unsupported Micro-assembler relocation {}", (*rel).type_),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
