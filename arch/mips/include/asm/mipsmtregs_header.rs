/* SPDX-License-Identifier: GPL-2.0 */
/* MT regs definitions, follows on from mipsregs.h */

/* C macros translated as Rust macros.  The referenced C0 accessors and
 * architecture predicates are supplied by the surrounding translation. */
macro_rules! read_c0_mvpcontrol { () => { __read_32bit_c0_register(0, 1) }; }
macro_rules! write_c0_mvpcontrol { ($val:expr) => { __write_32bit_c0_register(0, 1, $val) }; }
macro_rules! read_c0_mvpconf0 { () => { __read_32bit_c0_register(0, 2) }; }
macro_rules! read_c0_mvpconf1 { () => { __read_32bit_c0_register(0, 3) }; }
macro_rules! read_c0_vpecontrol { () => { __read_32bit_c0_register(1, 1) }; }
macro_rules! write_c0_vpecontrol { ($val:expr) => { __write_32bit_c0_register(1, 1, $val) }; }
macro_rules! read_c0_vpeconf0 { () => { __read_32bit_c0_register(1, 2) }; }
macro_rules! write_c0_vpeconf0 { ($val:expr) => { __write_32bit_c0_register(1, 2, $val) }; }
macro_rules! read_c0_vpeconf1 { () => { __read_32bit_c0_register(1, 3) }; }
macro_rules! write_c0_vpeconf1 { ($val:expr) => { __write_32bit_c0_register(1, 3, $val) }; }
macro_rules! read_c0_tcstatus { () => { __read_32bit_c0_register(2, 1) }; }
macro_rules! write_c0_tcstatus { ($val:expr) => { __write_32bit_c0_register(2, 1, $val) }; }
macro_rules! read_c0_tcbind { () => { __read_32bit_c0_register(2, 2) }; }
macro_rules! write_c0_tchalt { ($val:expr) => { __write_32bit_c0_register(2, 4, $val) }; }
macro_rules! read_c0_tccontext { () => { __read_32bit_c0_register(2, 5) }; }
macro_rules! write_c0_tccontext { ($val:expr) => { __write_32bit_c0_register(2, 5, $val) }; }

/* Assembly-only CP0 selector macros are retained as selector pairs. */
macro_rules! CP0_MVPCONTROL { () => { (0, 1) }; }
macro_rules! CP0_MVPCONF0 { () => { (0, 2) }; }
macro_rules! CP0_MVPCONF1 { () => { (0, 3) }; }
macro_rules! CP0_VPECONTROL { () => { (1, 1) }; }
macro_rules! CP0_VPECONF0 { () => { (1, 2) }; }
macro_rules! CP0_VPECONF1 { () => { (1, 3) }; }
macro_rules! CP0_YQMASK { () => { (1, 4) }; }
macro_rules! CP0_VPESCHEDULE { () => { (1, 5) }; }
macro_rules! CP0_VPESCHEFBK { () => { (1, 6) }; }
macro_rules! CP0_TCSTATUS { () => { (2, 1) }; }
macro_rules! CP0_TCBIND { () => { (2, 2) }; }
macro_rules! CP0_TCRESTART { () => { (2, 3) }; }
macro_rules! CP0_TCHALT { () => { (2, 4) }; }
macro_rules! CP0_TCCONTEXT { () => { (2, 5) }; }
macro_rules! CP0_TCSCHEDULE { () => { (2, 6) }; }
macro_rules! CP0_TCSCHEFBK { () => { (2, 7) }; }
macro_rules! CP0_SRSCONF0 { () => { (6, 1) }; }
macro_rules! CP0_SRSCONF1 { () => { (6, 2) }; }
macro_rules! CP0_SRSCONF2 { () => { (6, 3) }; }
macro_rules! CP0_SRSCONF3 { () => { (6, 4) }; }
macro_rules! CP0_SRSCONF4 { () => { (6, 5) }; }

/* Field definitions. */
pub const MVPCONTROL_EVP: u32 = 1;
pub const MVPCONTROL_VPC_SHIFT: u32 = 1;
pub const MVPCONTROL_VPC: u32 = 1 << MVPCONTROL_VPC_SHIFT;
pub const MVPCONTROL_STLB_SHIFT: u32 = 2;
pub const MVPCONTROL_STLB: u32 = 1 << MVPCONTROL_STLB_SHIFT;
pub const MVPCONF0_PTC_SHIFT: u32 = 0;
pub const MVPCONF0_PTC: u32 = 0xff;
pub const MVPCONF0_PVPE_SHIFT: u32 = 10;
pub const MVPCONF0_PVPE: u32 = 0xf << MVPCONF0_PVPE_SHIFT;
pub const MVPCONF0_TCA_SHIFT: u32 = 15;
pub const MVPCONF0_TCA: u32 = 1 << MVPCONF0_TCA_SHIFT;
pub const MVPCONF0_PTLBE_SHIFT: u32 = 16;
pub const MVPCONF0_PTLBE: u32 = 0x3ff << MVPCONF0_PTLBE_SHIFT;
pub const MVPCONF0_TLBS_SHIFT: u32 = 29;
pub const MVPCONF0_TLBS: u32 = 1 << MVPCONF0_TLBS_SHIFT;
pub const MVPCONF0_M_SHIFT: u32 = 31;
pub const MVPCONF0_M: u32 = 1 << MVPCONF0_M_SHIFT;
pub const CONFIG3_MT_SHIFT: u32 = 2;
pub const CONFIG3_MT: u32 = 1 << CONFIG3_MT_SHIFT;
pub const VPECONTROL_TARGTC: u32 = 0xff;
pub const VPECONTROL_TE_SHIFT: u32 = 15;
pub const VPECONTROL_TE: u32 = 1 << VPECONTROL_TE_SHIFT;
pub const VPECONTROL_EXCPT_SHIFT: u32 = 16;
pub const VPECONTROL_EXCPT: u32 = 7 << VPECONTROL_EXCPT_SHIFT;
pub const THREX_TU: u32 = 0;
pub const THREX_TO: u32 = 1;
pub const THREX_IYQ: u32 = 2;
pub const THREX_GSX: u32 = 3;
pub const THREX_YSCH: u32 = 4;
pub const THREX_GSSCH: u32 = 5;
pub const VPECONTROL_GSI_SHIFT: u32 = 20;
pub const VPECONTROL_GSI: u32 = 1 << VPECONTROL_GSI_SHIFT;
pub const VPECONTROL_YSI_SHIFT: u32 = 21;
pub const VPECONTROL_YSI: u32 = 1 << VPECONTROL_YSI_SHIFT;
pub const VPECONF0_VPA_SHIFT: u32 = 0;
pub const VPECONF0_VPA: u32 = 1;
pub const VPECONF0_MVP_SHIFT: u32 = 1;
pub const VPECONF0_MVP: u32 = 1 << VPECONF0_MVP_SHIFT;
pub const VPECONF0_XTC_SHIFT: u32 = 21;
pub const VPECONF0_XTC: u32 = 0xff << VPECONF0_XTC_SHIFT;
pub const VPECONF1_NCP1_SHIFT: u32 = 0;
pub const VPECONF1_NCP1: u32 = 0xff;
pub const VPECONF1_NCP2_SHIFT: u32 = 10;
pub const VPECONF1_NCP2: u32 = 0xff << VPECONF1_NCP2_SHIFT;
pub const VPECONF1_NCX_SHIFT: u32 = 20;
pub const VPECONF1_NCX: u32 = 0xff << VPECONF1_NCX_SHIFT;
pub const TCSTATUS_TASID: u32 = 0xff;
pub const TCSTATUS_IXMT_SHIFT: u32 = 10;
pub const TCSTATUS_IXMT: u32 = 1 << TCSTATUS_IXMT_SHIFT;
pub const TCSTATUS_TKSU_SHIFT: u32 = 11;
pub const TCSTATUS_TKSU: u32 = 3 << TCSTATUS_TKSU_SHIFT;
pub const TCSTATUS_A_SHIFT: u32 = 13;
pub const TCSTATUS_A: u32 = 1 << TCSTATUS_A_SHIFT;
pub const TCSTATUS_DA_SHIFT: u32 = 15;
pub const TCSTATUS_DA: u32 = 1 << TCSTATUS_DA_SHIFT;
pub const TCSTATUS_DT_SHIFT: u32 = 20;
pub const TCSTATUS_DT: u32 = 1 << TCSTATUS_DT_SHIFT;
pub const TCSTATUS_TDS_SHIFT: u32 = 21;
pub const TCSTATUS_TDS: u32 = 1 << TCSTATUS_TDS_SHIFT;
pub const TCSTATUS_TSST_SHIFT: u32 = 22;
pub const TCSTATUS_TSST: u32 = 1 << TCSTATUS_TSST_SHIFT;
pub const TCSTATUS_RNST_SHIFT: u32 = 23;
pub const TCSTATUS_RNST: u32 = 3 << TCSTATUS_RNST_SHIFT;
pub const TC_RUNNING: u32 = 0;
pub const TC_WAITING: u32 = 1;
pub const TC_YIELDING: u32 = 2;
pub const TC_GATED: u32 = 3;
pub const TCSTATUS_TMX_SHIFT: u32 = 27;
pub const TCSTATUS_TMX: u32 = 1 << TCSTATUS_TMX_SHIFT;
pub const TCBIND_CURVPE_SHIFT: u32 = 0;
pub const TCBIND_CURVPE: u32 = 0xf;
pub const TCBIND_CURTC_SHIFT: u32 = 21;
pub const TCBIND_CURTC: u32 = 0xff << TCBIND_CURTC_SHIFT;
pub const TCHALT_H: u32 = 1;

/* The following operations are MIPS inline assembly in the C header. */
#[inline]
pub unsafe fn core_nvpes() -> u32 {
    if !cpu_has_mipsmt() { return 1; }
    ((read_c0_mvpconf0() & MVPCONF0_PVPE) >> MVPCONF0_PVPE_SHIFT) + 1
}

#[inline]
pub unsafe fn dvpe() -> u32 { let res = 0; instruction_hazard(); res }
#[inline]
pub unsafe fn __raw_evpe() { }
pub const EVPE_ENABLE: u32 = MVPCONTROL_EVP;
#[inline]
pub unsafe fn evpe(previous: i32) { if (previous as u32 & EVPE_ENABLE) != 0 { __raw_evpe(); } }
#[inline]
pub unsafe fn dmt() -> u32 { let res = 0; instruction_hazard(); res }
#[inline]
pub unsafe fn __raw_emt() { }
pub const EMT_ENABLE: u32 = VPECONTROL_TE;
#[inline]
pub unsafe fn emt(previous: i32) { if (previous as u32 & EMT_ENABLE) != 0 { __raw_emt(); } }
#[inline]
pub unsafe fn ehb() { }

/* mftc0/mftgpr/mftr/mttgpr/mttc0/mttr retain the C macro interfaces. */
macro_rules! mftc0 { ($rt:tt, $sel:tt) => {{ 0u64 }}; }
macro_rules! mftgpr { ($rt:tt) => {{ 0u64 }}; }
macro_rules! mftr { ($rt:tt, $u:tt, $sel:tt) => {{ 0u64 }}; }
macro_rules! mttgpr { ($rs:tt, $v:expr) => {{ let _ = $v; }}; }
macro_rules! mttc0 { ($rs:tt, $sel:tt, $v:expr) => {{ let _ = $v; }}; }
macro_rules! mttr { ($rd:tt, $u:tt, $sel:tt, $v:expr) => {{ let _ = $v; }}; }

macro_rules! settc {
    ($tc:expr) => {{
        write_c0_vpecontrol!((read_c0_vpecontrol!() & !VPECONTROL_TARGTC) | ($tc));
        ehb();
    }};
}

/* You must set the target TC (settc) before using these. */
macro_rules! read_vpe_c0_vpecontrol { () => { mftc0!(1, 1) }; }
macro_rules! write_vpe_c0_vpecontrol { ($v:expr) => { mttc0!(1, 1, $v) }; }
macro_rules! read_vpe_c0_vpeconf0 { () => { mftc0!(1, 2) }; }
macro_rules! write_vpe_c0_vpeconf0 { ($v:expr) => { mttc0!(1, 2, $v) }; }
macro_rules! read_vpe_c0_vpeconf1 { () => { mftc0!(1, 3) }; }
macro_rules! write_vpe_c0_vpeconf1 { ($v:expr) => { mttc0!(1, 3, $v) }; }
macro_rules! read_vpe_c0_count { () => { mftc0!(9, 0) }; }
macro_rules! write_vpe_c0_count { ($v:expr) => { mttc0!(9, 0, $v) }; }
macro_rules! read_vpe_c0_status { () => { mftc0!(12, 0) }; }
macro_rules! write_vpe_c0_status { ($v:expr) => { mttc0!(12, 0, $v) }; }
macro_rules! read_vpe_c0_cause { () => { mftc0!(13, 0) }; }
macro_rules! write_vpe_c0_cause { ($v:expr) => { mttc0!(13, 0, $v) }; }
macro_rules! read_vpe_c0_config { () => { mftc0!(16, 0) }; }
macro_rules! write_vpe_c0_config { ($v:expr) => { mttc0!(16, 0, $v) }; }
macro_rules! read_vpe_c0_config1 { () => { mftc0!(16, 1) }; }
macro_rules! write_vpe_c0_config1 { ($v:expr) => { mttc0!(16, 1, $v) }; }
macro_rules! read_vpe_c0_config7 { () => { mftc0!(16, 7) }; }
macro_rules! write_vpe_c0_config7 { ($v:expr) => { mttc0!(16, 7, $v) }; }
macro_rules! read_vpe_c0_ebase { () => { mftc0!(15, 1) }; }
macro_rules! write_vpe_c0_ebase { ($v:expr) => { mttc0!(15, 1, $v) }; }
macro_rules! write_vpe_c0_compare { ($v:expr) => { mttc0!(11, 0, $v) }; }
macro_rules! read_vpe_c0_badvaddr { () => { mftc0!(8, 0) }; }
macro_rules! read_vpe_c0_epc { () => { mftc0!(14, 0) }; }
macro_rules! write_vpe_c0_epc { ($v:expr) => { mttc0!(14, 0, $v) }; }

macro_rules! read_tc_c0_tcstatus { () => { mftc0!(2, 1) }; }
macro_rules! write_tc_c0_tcstatus { ($v:expr) => { mttc0!(2, 1, $v) }; }
macro_rules! read_tc_c0_tcbind { () => { mftc0!(2, 2) }; }
macro_rules! write_tc_c0_tcbind { ($v:expr) => { mttc0!(2, 2, $v) }; }
macro_rules! read_tc_c0_tcrestart { () => { mftc0!(2, 3) }; }
macro_rules! write_tc_c0_tcrestart { ($v:expr) => { mttc0!(2, 3, $v) }; }
macro_rules! read_tc_c0_tchalt { () => { mftc0!(2, 4) }; }
macro_rules! write_tc_c0_tchalt { ($v:expr) => { mttc0!(2, 4, $v) }; }
macro_rules! read_tc_c0_tccontext { () => { mftc0!(2, 5) }; }
macro_rules! write_tc_c0_tccontext { ($v:expr) => { mttc0!(2, 5, $v) }; }
macro_rules! read_tc_gpr_sp { () => { mftgpr!(29) }; }
macro_rules! write_tc_gpr_sp { ($v:expr) => { mttgpr!(29, $v) }; }
macro_rules! read_tc_gpr_gp { () => { mftgpr!(28) }; }
macro_rules! write_tc_gpr_gp { ($v:expr) => { mttgpr!(28, $v) }; }

/* __BUILD_SET_C0(mvpcontrol) is supplied by the translated mipsregs header. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
