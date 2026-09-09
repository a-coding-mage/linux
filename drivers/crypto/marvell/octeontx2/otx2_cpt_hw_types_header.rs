/* SPDX-License-Identifier: GPL-2.0-only */

pub const OTX2_CPT_PCI_PF_DEVICE_ID: u32 = 0xA0FD;
pub const OTX2_CPT_PCI_VF_DEVICE_ID: u32 = 0xA0FE;
pub const CN10K_CPT_PCI_PF_DEVICE_ID: u32 = 0xA0F2;
pub const CN10K_CPT_PCI_VF_DEVICE_ID: u32 = 0xA0F3;
pub const CPT_PCI_SUBSYS_DEVID_CN10K_A: u32 = 0xB900;
pub const CPT_PCI_SUBSYS_DEVID_CN10K_B: u32 = 0xBD00;
pub const OTX2_CPT_PF_MBOX_INT: u32 = 6;
pub const OTX2_CPT_MAX_ENGINE_GROUPS: u32 = 8;
pub const OTX2_CPT_INST_SIZE: u32 = 64;
pub const OTX2_CPT_VF_MSIX_VECTORS: u32 = 1;
pub const OTX2_CPT_VF_INTR_MBOX_MASK: u64 = 1;
pub const CN10K_CPT_VF_MBOX_REGION: u64 = 0xC0000;
pub const OTX2_CPT_LF_MSIX_VECTORS: u32 = 2;

pub const fn OTX2_CPT_PF_INT_VEC_E_MBOXX(x: u64, a: u64) -> u64 { x.wrapping_add(a) }
pub const fn OTX2_CPT_PF_MBOX_INTX(b: u64) -> u64 { 0x400 | (b << 3) }
pub const fn OTX2_CPT_PF_MBOX_INT_W1SX(b: u64) -> u64 { 0x420 | (b << 3) }
pub const fn OTX2_CPT_PF_MBOX_ENA_W1CX(b: u64) -> u64 { 0x440 | (b << 3) }
pub const fn OTX2_CPT_PF_MBOX_ENA_W1SX(b: u64) -> u64 { 0x460 | (b << 3) }
pub const fn OTX2_CPT_PF_GX_EN(b: u64) -> u64 { 0x600 | (b << 3) }
pub const fn OTX2_CPT_PF_EXE_DBG_CNTX(b: u64) -> u64 { 0x4001100 | (b << 3) }
pub const fn OTX2_CPT_PF_EXE_EPCI_INBX_CNT(b: u64) -> u64 { 0x4001200 | (b << 3) }
pub const fn OTX2_CPT_PF_EXE_EPCI_OUTBX_CNT(b: u64) -> u64 { 0x4001240 | (b << 3) }
pub const fn OTX2_CPT_PF_ENGX_UCODE_BASE(b: u64) -> u64 { 0x4002000 | (b << 3) }
pub const fn OTX2_CPT_PF_QX_CTL(b: u64) -> u64 { 0x8000000 | (b << 20) }
pub const fn OTX2_CPT_PF_QX_GMCTL(b: u64) -> u64 { 0x8000020 | (b << 20) }
pub const fn OTX2_CPT_PF_QX_CTL2(b: u64) -> u64 { 0x8000100 | (b << 20) }
pub const fn OTX2_CPT_PF_VFX_MBOXX(b: u64, c: u64) -> u64 { 0x8001000 | (b << 20) | (c << 8) }

pub const OTX2_CPT_RVU_FUNC_BLKADDR_SHIFT: u32 = 20;
pub const OTX2_CPT_LMT_LFBASE: u64 = 1u64 << OTX2_CPT_RVU_FUNC_BLKADDR_SHIFT;
pub const fn OTX2_CPT_LMT_LF_LMTLINEX(a: u64) -> u64 { OTX2_CPT_LMT_LFBASE | (a << 12) }

macro_rules! consts { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u64 = $v;)* }; }
consts! {
 OTX2_CPT_PF_CONSTANTS=0x0, OTX2_CPT_PF_RESET=0x100, OTX2_CPT_PF_DIAG=0x120, OTX2_CPT_PF_BIST_STATUS=0x160,
 OTX2_CPT_PF_ECC0_CTL=0x200, OTX2_CPT_PF_ECC0_FLIP=0x210, OTX2_CPT_PF_ECC0_INT=0x220, OTX2_CPT_PF_ECC0_INT_W1S=0x230,
 OTX2_CPT_PF_ECC0_ENA_W1S=0x240, OTX2_CPT_PF_ECC0_ENA_W1C=0x250, OTX2_CPT_PF_EXEC_INT=0x500, OTX2_CPT_PF_EXEC_INT_W1S=0x520,
 OTX2_CPT_PF_EXEC_ENA_W1C=0x540, OTX2_CPT_PF_EXEC_ENA_W1S=0x560, OTX2_CPT_PF_EXEC_INFO=0x700, OTX2_CPT_PF_EXEC_BUSY=0x800,
 OTX2_CPT_PF_EXEC_INFO0=0x900, OTX2_CPT_PF_EXEC_INFO1=0x910, OTX2_CPT_PF_INST_REQ_PC=0x10000, OTX2_CPT_PF_INST_LATENCY_PC=0x10020,
 OTX2_CPT_PF_RD_REQ_PC=0x10040, OTX2_CPT_PF_RD_LATENCY_PC=0x10060, OTX2_CPT_PF_RD_UC_PC=0x10080, OTX2_CPT_PF_ACTIVE_CYCLES_PC=0x10100,
 OTX2_CPT_PF_EXE_CTL=0x4000000, OTX2_CPT_PF_EXE_STATUS=0x4000008, OTX2_CPT_PF_EXE_CLK=0x4000010, OTX2_CPT_PF_EXE_DBG_CTL=0x4000018,
 OTX2_CPT_PF_EXE_DBG_DATA=0x4000020, OTX2_CPT_PF_EXE_BIST_STATUS=0x4000028, OTX2_CPT_PF_EXE_REQ_TIMER=0x4000030, OTX2_CPT_PF_EXE_MEM_CTL=0x4000038,
 OTX2_CPT_PF_EXE_PERF_CTL=0x4001000, OTX2_CPT_PF_EXE_PERF_EVENT_CNT=0x4001180,
 OTX2_CPT_LF_CTL=0x10, OTX2_CPT_LF_DONE_WAIT=0x30, OTX2_CPT_LF_INPROG=0x40, OTX2_CPT_LF_DONE=0x50, OTX2_CPT_LF_DONE_ACK=0x60,
 OTX2_CPT_LF_DONE_INT_ENA_W1S=0x90, OTX2_CPT_LF_DONE_INT_ENA_W1C=0xa0, OTX2_CPT_LF_MISC_INT=0xb0, OTX2_CPT_LF_MISC_INT_W1S=0xc0,
 OTX2_CPT_LF_MISC_INT_ENA_W1S=0xd0, OTX2_CPT_LF_MISC_INT_ENA_W1C=0xe0, OTX2_CPT_LF_Q_BASE=0xf0, OTX2_CPT_LF_Q_SIZE=0x100,
 OTX2_CPT_LF_Q_INST_PTR=0x110, OTX2_CPT_LF_Q_GRP_PTR=0x120, OTX2_CPT_LF_CTX_CTL=0x500, OTX2_CPT_LF_CTX_FLUSH=0x510, OTX2_CPT_LF_CTX_ERR=0x520,
 OTX2_RVU_VF_INT=0x20, OTX2_RVU_VF_INT_W1S=0x28, OTX2_RVU_VF_INT_ENA_W1S=0x30, OTX2_RVU_VF_INT_ENA_W1C=0x38
}
pub const fn OTX2_CPT_LF_NQX(a:u64)->u64 { 0x400 | (a<<3) }

#[repr(u32)] pub enum Otx2CptUcodeCompCodeE { OTX2_CPT_UCC_SUCCESS=0, OTX2_CPT_UCC_INVALID_OPCODE=1, OTX2_CPT_UCC_SG_WRITE_LENGTH=2, OTX2_CPT_UCC_SG_LIST=3, OTX2_CPT_UCC_SG_NOT_SUPPORTED=4 }
#[repr(u32)] pub enum Otx2CptCompE { OTX2_CPT_COMP_E_NOTDONE=0, OTX2_CPT_COMP_E_GOOD=1, OTX2_CPT_COMP_E_FAULT=2, OTX2_CPT_COMP_E_HWERR=4, OTX2_CPT_COMP_E_INSTERR=5, OTX2_CPT_COMP_E_WARN=6 }
#[repr(u32)] pub enum Otx2CptVfIntVecE { OTX2_CPT_VF_INT_VEC_E_MBOX=0 }
#[repr(u32)] pub enum Otx2CptLfIntVecE { OTX2_CPT_LF_INT_VEC_E_MISC=0, OTX2_CPT_LF_INT_VEC_E_DONE=1 }

/* C bitfields are represented as their containing little-endian u64 words. */
#[repr(C)] pub union otx2_cpt_inst_s { pub u: [u64;8], pub s: Otx2CptInstFields }
#[repr(C)] pub struct Otx2CptInstFields { pub word0:u64, pub res_addr:u64, pub word2:u64, pub word3:u64, pub ei0:u64, pub ei1:u64, pub ei2:u64, pub ei3:u64 }
#[repr(C)] pub union otx2_cpt_res_s { pub u:[u64;2], pub s:Cn9kCptResFields, pub cn10k:Cn10kCptResFields }
#[repr(C)] pub struct Cn9kCptResFields { pub word0:u64, pub reserved_64_127:u64 }
#[repr(C)] pub struct Cn10kCptResFields { pub word0:u64, pub esn:u64 }

macro_rules! reg_union { ($u:ident, $s:ident) => { #[repr(C)] pub union $u { pub u:u64, pub s:$s } #[repr(C)] pub struct $s { pub bits:u64 } }; }
reg_union!(otx2_cptx_af_constants1, Otx2CptxAfConstants1Fields);
reg_union!(otx2_cptx_lf_misc_int, Otx2CptxLfMiscIntFields);
reg_union!(otx2_cptx_lf_misc_int_ena_w1s, Otx2CptxLfMiscIntEnaW1sFields);
reg_union!(otx2_cptx_lf_ctl, Otx2CptxLfCtlFields);
reg_union!(otx2_cptx_lf_done_wait, Otx2CptxLfDoneWaitFields);
reg_union!(otx2_cptx_lf_done, Otx2CptxLfDoneFields);
reg_union!(otx2_cptx_lf_inprog, Otx2CptxLfInprogFields);
reg_union!(otx2_cptx_lf_q_base, Otx2CptxLfQBaseFields);
reg_union!(otx2_cptx_lf_q_size, Otx2CptxLfQSizeFields);
reg_union!(otx2_cptx_af_lf_ctrl, Otx2CptxAfLfCtrlFields);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
