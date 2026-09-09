/* SPDX-License-Identifier: GPL-2.0
 * Marvell OcteonTX CPT driver
 *
 * Source-level Rust translation of otx_cpt_hw_types.h.
 * C bitfields are represented by their containing hardware word; callers may
 * use the documented masks/shifts when accessing individual fields.
 */

pub const OTX_CPT_PCI_PF_DEVICE_ID: u32 = 0xa040;
pub const OTX_CPT_PCI_VF_DEVICE_ID: u32 = 0xa041;
pub const OTX_CPT_PCI_PF_SUBSYS_ID: u32 = 0xa340;
pub const OTX_CPT_PCI_VF_SUBSYS_ID: u32 = 0xa341;
pub const OTX_CPT_PF_PCI_CFG_BAR: u32 = 0;
pub const OTX_CPT_VF_PCI_CFG_BAR: u32 = 0;
pub const OTX_CPT_BAR_E_CPTX_VFX_BAR0_SIZE: u64 = 0x400000;
pub const OTX_CPT_PF_MBOX_INT: u32 = 3;
pub const OTX_CPT_PF_MSIX_VECTORS: u32 = 4;
pub const OTX_CPT_MAX_ENGINE_GROUPS: u32 = 8;
pub const OTX_CPT_INST_SIZE: u32 = 64;
pub const OTX_CPT_NEXT_CHUNK_PTR_SIZE: u32 = 8;
pub const OTX_CPT_VF_MSIX_VECTORS: u32 = 2;
pub const OTX_CPT_VF_INTR_MBOX_MASK: u64 = 1 << 0;
pub const OTX_CPT_VF_INTR_DOVF_MASK: u64 = 1 << 1;
pub const OTX_CPT_VF_INTR_IRDE_MASK: u64 = 1 << 2;
pub const OTX_CPT_VF_INTR_NWRP_MASK: u64 = 1 << 3;
pub const OTX_CPT_VF_INTR_SERR_MASK: u64 = 1 << 4;

#[inline] pub const fn OTX_CPT_BAR_E_CPTX_VFX_BAR0_OFFSET(a: u64, b: u64) -> u64 { 0x000020000000 + 0x1000000000 * a + 0x100000 * b }
#[inline] pub const fn OTX_CPT_PF_INT_VEC_E_MBOXX(x: u64, a: u64) -> u64 { x + a }

pub const OTX_CPT_PF_CONSTANTS: u64 = 0x0;
pub const OTX_CPT_PF_RESET: u64 = 0x100;
pub const OTX_CPT_PF_DIAG: u64 = 0x120;
pub const OTX_CPT_PF_BIST_STATUS: u64 = 0x160;
pub const OTX_CPT_PF_ECC0_CTL: u64 = 0x200;
pub const OTX_CPT_PF_ECC0_FLIP: u64 = 0x210;
pub const OTX_CPT_PF_ECC0_INT: u64 = 0x220;
pub const OTX_CPT_PF_ECC0_INT_W1S: u64 = 0x230;
pub const OTX_CPT_PF_ECC0_ENA_W1S: u64 = 0x240;
pub const OTX_CPT_PF_ECC0_ENA_W1C: u64 = 0x250;
pub const OTX_CPT_PF_EXEC_INT: u64 = 0x500;
pub const OTX_CPT_PF_EXEC_INT_W1S: u64 = 0x520;
pub const OTX_CPT_PF_EXEC_ENA_W1C: u64 = 0x540;
pub const OTX_CPT_PF_EXEC_ENA_W1S: u64 = 0x560;
pub const OTX_CPT_PF_EXEC_INFO: u64 = 0x700;
pub const OTX_CPT_PF_EXEC_BUSY: u64 = 0x800;
pub const OTX_CPT_PF_EXEC_INFO0: u64 = 0x900;
pub const OTX_CPT_PF_EXEC_INFO1: u64 = 0x910;
pub const OTX_CPT_PF_INST_REQ_PC: u64 = 0x10000;
pub const OTX_CPT_PF_INST_LATENCY_PC: u64 = 0x10020;
pub const OTX_CPT_PF_RD_REQ_PC: u64 = 0x10040;
pub const OTX_CPT_PF_RD_LATENCY_PC: u64 = 0x10060;
pub const OTX_CPT_PF_RD_UC_PC: u64 = 0x10080;
pub const OTX_CPT_PF_ACTIVE_CYCLES_PC: u64 = 0x10100;
pub const OTX_CPT_PF_EXE_CTL: u64 = 0x4000000;
pub const OTX_CPT_PF_EXE_STATUS: u64 = 0x4000008;
pub const OTX_CPT_PF_EXE_CLK: u64 = 0x4000010;
pub const OTX_CPT_PF_EXE_DBG_CTL: u64 = 0x4000018;
pub const OTX_CPT_PF_EXE_DBG_DATA: u64 = 0x4000020;
pub const OTX_CPT_PF_EXE_BIST_STATUS: u64 = 0x4000028;
pub const OTX_CPT_PF_EXE_REQ_TIMER: u64 = 0x4000030;
pub const OTX_CPT_PF_EXE_MEM_CTL: u64 = 0x4000038;
pub const OTX_CPT_PF_EXE_PERF_CTL: u64 = 0x4001000;

#[inline] pub const fn OTX_CPT_PF_MBOX_INTX(b: u64) -> u64 { 0x400 | (b << 3) }
#[inline] pub const fn OTX_CPT_PF_MBOX_INT_W1SX(b: u64) -> u64 { 0x420 | (b << 3) }
#[inline] pub const fn OTX_CPT_PF_MBOX_ENA_W1CX(b: u64) -> u64 { 0x440 | (b << 3) }
#[inline] pub const fn OTX_CPT_PF_MBOX_ENA_W1SX(b: u64) -> u64 { 0x460 | (b << 3) }
#[inline] pub const fn OTX_CPT_PF_GX_EN(b: u64) -> u64 { 0x600 | (b << 3) }
#[inline] pub const fn OTX_CPT_PF_EXE_DBG_CNTX(b: u64) -> u64 { 0x4001100 | (b << 3) }
#[inline] pub const fn OTX_CPT_PF_EXE_EPCI_INBX_CNT(b: u64) -> u64 { 0x4001200 | (b << 3) }
#[inline] pub const fn OTX_CPT_PF_EXE_EPCI_OUTBX_CNT(b: u64) -> u64 { 0x4001240 | (b << 3) }
#[inline] pub const fn OTX_CPT_PF_ENGX_UCODE_BASE(b: u64) -> u64 { 0x4002000 | (b << 3) }
#[inline] pub const fn OTX_CPT_PF_QX_CTL(b: u64) -> u64 { 0x8000000 | (b << 20) }
#[inline] pub const fn OTX_CPT_PF_QX_GMCTL(b: u64) -> u64 { 0x8000020 | (b << 20) }
#[inline] pub const fn OTX_CPT_PF_QX_CTL2(b: u64) -> u64 { 0x8000100 | (b << 20) }
#[inline] pub const fn OTX_CPT_PF_VFX_MBOXX(b: u64, c: u64) -> u64 { 0x8001000 | (b << 20) | (c << 8) }

#[inline] pub const fn OTX_CPT_VQX_CTL(b: u64) -> u64 { 0x100 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_SADDR(b: u64) -> u64 { 0x200 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_DONE_WAIT(b: u64) -> u64 { 0x400 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_INPROG(b: u64) -> u64 { 0x410 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_DONE(b: u64) -> u64 { 0x420 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_DONE_ACK(b: u64) -> u64 { 0x440 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_DONE_INT_W1S(b: u64) -> u64 { 0x460 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_DONE_INT_W1C(b: u64) -> u64 { 0x468 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_DONE_ENA_W1S(b: u64) -> u64 { 0x470 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_DONE_ENA_W1C(b: u64) -> u64 { 0x478 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_MISC_INT(b: u64) -> u64 { 0x500 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_MISC_INT_W1S(b: u64) -> u64 { 0x508 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_MISC_ENA_W1S(b: u64) -> u64 { 0x510 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_MISC_ENA_W1C(b: u64) -> u64 { 0x518 | (b << 20) }
#[inline] pub const fn OTX_CPT_VQX_DOORBELL(b: u64) -> u64 { 0x600 | (b << 20) }
#[inline] pub const fn OTX_CPT_VFX_PF_MBOXX(b: u64, c: u64) -> u64 { 0x1000 | (b << 20) | (c << 3) }

#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OtxCptUcodeErrorCodeE { CPT_NO_UCODE_ERROR=0, ERR_OPCODE_UNSUPPORTED=1, ERR_SCATTER_GATHER_WRITE_LENGTH=2, ERR_SCATTER_GATHER_LIST=3, ERR_SCATTER_GATHER_NOT_SUPPORTED=4 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OtxCptCompE { CPT_COMP_E_NOTDONE=0, CPT_COMP_E_GOOD=1, CPT_COMP_E_FAULT=2, CPT_COMP_E_SWERR=3, CPT_COMP_E_HWERR=4, CPT_COMP_E_LAST_ENTRY=5 }
#[repr(u32)] #[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OtxCptVfIntVecE { CPT_VF_INT_VEC_E_MISC=0, CPT_VF_INT_VEC_E_DONE=1 }

#[repr(C)] #[derive(Copy, Clone)] pub struct OtxCptInstSFields { pub word0: u64, pub res_addr: u64, pub word2: u64, pub wq_ptr: u64, pub ei0: u64, pub ei1: u64, pub ei2: u64, pub ei3: u64 }
#[repr(C)] pub union OtxCptInstS { pub u: [u64; 8], pub s: OtxCptInstSFields }
#[repr(C)] #[derive(Copy, Clone)] pub struct OtxCptResSFields { pub word0: u64, pub reserved_64_127: u64 }
#[repr(C)] pub union OtxCptResS { pub u: [u64; 2], pub s: OtxCptResSFields }

#[repr(C)] #[derive(Copy, Clone)] pub struct OtxCptRegister { pub word0: u64 }
pub type OtxCptxPfBistStatus = OtxCptRegister;
pub type OtxCptxPfConstants = OtxCptRegister;
pub type OtxCptxPfExeBistStatus = OtxCptRegister;
pub type OtxCptxPfQxCtl = OtxCptRegister;
pub type OtxCptxVqxSaddr = OtxCptRegister;
pub type OtxCptxVqxMiscEnaW1s = OtxCptRegister;
pub type OtxCptxVqxDoorbell = OtxCptRegister;
pub type OtxCptxVqxInprog = OtxCptRegister;
pub type OtxCptxVqxMiscInt = OtxCptRegister;
pub type OtxCptxVqxDoneAck = OtxCptRegister;
pub type OtxCptxVqxDone = OtxCptRegister;
pub type OtxCptxVqxDoneWait = OtxCptRegister;
pub type OtxCptxVqxDoneEnaW1s = OtxCptRegister;
pub type OtxCptxVqxCtl = OtxCptRegister;
pub type OtxCptErrorCode = OtxCptRegister;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
