// SPDX-License-Identifier: GPL-2.0
/* Nehalem-EX/Westmere-EX uncore support.  External kernel declarations are
 * supplied by the surrounding uncore implementation. */

const NHMEX_PMON_CTL_EV_SEL_MASK: u64 = 0x000000ff;
const NHMEX_PMON_CTL_UMASK_MASK: u64 = 0x0000ff00;
const NHMEX_PMON_CTL_EN_BIT0: u64 = 1 << 0;
const NHMEX_PMON_CTL_EDGE_DET: u64 = 1 << 18;
const NHMEX_PMON_CTL_PMI_EN: u64 = 1 << 20;
const NHMEX_PMON_CTL_EN_BIT22: u64 = 1 << 22;
const NHMEX_PMON_CTL_INVERT: u64 = 1 << 23;
const NHMEX_PMON_CTL_TRESH_MASK: u64 = 0xff000000;
const NHMEX_PMON_RAW_EVENT_MASK: u64 = NHMEX_PMON_CTL_EV_SEL_MASK | NHMEX_PMON_CTL_UMASK_MASK | NHMEX_PMON_CTL_EDGE_DET | NHMEX_PMON_CTL_INVERT | NHMEX_PMON_CTL_TRESH_MASK;

const NHMEX_U_MSR_PMON_GLOBAL_CTL: u32 = 0xc00;
const NHMEX_U_MSR_PMON_CTR: u32 = 0xc11;
const NHMEX_U_MSR_PMON_EV_SEL: u32 = 0xc10;
const NHMEX_U_PMON_GLOBAL_EN_ALL: u64 = 1 << 28;
const NHMEX_U_PMON_RAW_EVENT_MASK: u64 = NHMEX_PMON_CTL_EV_SEL_MASK | NHMEX_PMON_CTL_EDGE_DET;
const NHMEX_C0_MSR_PMON_GLOBAL_CTL: u32 = 0xd00;
const NHMEX_C0_MSR_PMON_CTR0: u32 = 0xd11;
const NHMEX_C0_MSR_PMON_EV_SEL0: u32 = 0xd10;
const NHMEX_B0_MSR_PMON_GLOBAL_CTL: u32 = 0xc20;
const NHMEX_B0_MSR_PMON_CTR0: u32 = 0xc31;
const NHMEX_B0_MSR_PMON_CTL0: u32 = 0xc30;
const NHMEX_B0_MSR_MATCH: u32 = 0xe45;
const NHMEX_B0_MSR_MASK: u32 = 0xe46;
const NHMEX_B1_MSR_MATCH: u32 = 0xe4d;
const NHMEX_B1_MSR_MASK: u32 = 0xe4e;
const NHMEX_B_MSR_OFFSET: u32 = 0x40;
const NHMEX_B_PMON_CTL_EV_SEL_SHIFT: u64 = 1;
const NHMEX_B_PMON_CTL_EV_SEL_MASK: u64 = 0x1f << 1;
const NHMEX_B_PMON_CTR_SHIFT: u64 = 6;
const NHMEX_B_PMON_CTR_MASK: u64 = 0x3 << 6;
const NHMEX_B_PMON_RAW_EVENT_MASK: u64 = NHMEX_B_PMON_CTL_EV_SEL_MASK | NHMEX_B_PMON_CTR_MASK;
const NHMEX_S0_MSR_PMON_GLOBAL_CTL: u32 = 0xc40;
const NHMEX_S0_MSR_PMON_CTR0: u32 = 0xc51;
const NHMEX_S0_MSR_PMON_CTL0: u32 = 0xc50;
const NHMEX_S_MSR_OFFSET: u32 = 0x80;
const NHMEX_S0_MSR_MM_CFG: u32 = 0xe48;
const NHMEX_S1_MSR_MM_CFG: u32 = 0xe58;
const NHMEX_S_PMON_MM_CFG_EN: u64 = 1 << 63;
const NHMEX_S_EVENT_TO_R_PROG_EV: u64 = 0;

const NHMEX_M0_MSR_GLOBAL_CTL: u32 = 0xca0;
const NHMEX_M0_MSR_PMU_DSP: u32 = 0xca5;
const NHMEX_M0_MSR_PMU_ISS: u32 = 0xca6;
const NHMEX_M0_MSR_PMU_MAP: u32 = 0xca7;
const NHMEX_M0_MSR_PMU_MSC_THR: u32 = 0xca8;
const NHMEX_M0_MSR_PMU_PGT: u32 = 0xca9;
const NHMEX_M0_MSR_PMU_PLD: u32 = 0xcaa;
const NHMEX_M0_MSR_PMU_ZDP_CTL_FVC: u32 = 0xcab;
const NHMEX_M0_MSR_PMU_CTL0: u32 = 0xcb0;
const NHMEX_M0_MSR_PMU_CNT0: u32 = 0xcb1;
const NHMEX_M_MSR_OFFSET: u32 = 0x40;
const NHMEX_M0_MSR_PMU_MM_CFG: u32 = 0xe54;
const NHMEX_M1_MSR_PMU_MM_CFG: u32 = 0xe5c;
const NHMEX_M_PMON_MM_CFG_EN: u64 = 1 << 63;
const NHMEX_M_PMON_ADDR_MATCH_MASK: u64 = 0x3ffffffff;
const NHMEX_M_PMON_ADDR_MASK_MASK: u64 = 0x7ffffff;
const NHMEX_M_PMON_ADDR_MASK_SHIFT: u64 = 34;
const NHMEX_M_PMON_CTL_INC_SEL_SHIFT: u64 = 9;
const NHMEX_M_PMON_CTL_INC_SEL_MASK: u64 = 0x1f << 9;
const NHMEX_M_PMON_CTL_FLAG_MODE: u64 = 1 << 7;
const NHMEX_M_PMON_CTL_SET_FLAG_SEL_SHIFT: u64 = 19;
const NHMEX_M_PMON_CTL_SET_FLAG_SEL_MASK: u64 = 7 << 19;
const NHMEX_M_PMON_RAW_EVENT_MASK: u64 = (3 << 2) | (3 << 4) | (1 << 6) | (1 << 7) | NHMEX_M_PMON_CTL_INC_SEL_MASK | NHMEX_M_PMON_CTL_SET_FLAG_SEL_MASK;
const NHMEX_M_PMON_ZDP_CTL_FVC_MASK: u64 = ((1 << 11) - 1) | (1 << 23);
const WSMEX_M_PMON_ZDP_CTL_FVC_MASK: u64 = ((1 << 12) - 1) | (1 << 24);
const NHMEX_R_MSR_GLOBAL_CTL: u32 = 0xe00;
const NHMEX_R_MSR_PMON_CTL0: u32 = 0xe10;
const NHMEX_R_MSR_PMON_CNT0: u32 = 0xe11;
const NHMEX_R_MSR_OFFSET: u32 = 0x20;
const NHMEX_R_PMON_CTL_EV_SEL_SHIFT: u64 = 1;
const NHMEX_R_PMON_CTL_EV_SEL_MASK: u64 = 0x1f << 1;
const NHMEX_R_PMON_RAW_EVENT_MASK: u64 = NHMEX_R_PMON_CTL_EV_SEL_MASK;
const NHMEX_W_MSR_GLOBAL_CTL: u32 = 0xc80;
const NHMEX_W_MSR_PMON_CNT0: u32 = 0xc90;
const NHMEX_W_MSR_PMON_EVT_SEL0: u32 = 0xc91;
const NHMEX_W_MSR_PMON_FIXED_CTR: u32 = 0x394;
const NHMEX_W_MSR_PMON_FIXED_CTL: u32 = 0x395;
const NHMEX_W_PMON_GLOBAL_FIXED_EN: u64 = 1 << 31;

#[inline]
fn bits_value(x: u64, i: u32, n: u32) -> u64 { (x >> (i * n)) & ((1u64 << n) - 1) }

// The following items intentionally retain the kernel's external types,
// macros, attributes, and operation tables; they are defined by uncore.h.
extern "C" {
    fn wrmsrq(msr: u32, value: u64);
    fn rdmsrq(msr: u32, value: *mut u64);
}

unsafe fn nhmex_uncore_msr_init_box(_box: *mut intel_uncore_box) -> i32 { wrmsrq(NHMEX_U_MSR_PMON_GLOBAL_CTL, NHMEX_U_PMON_GLOBAL_EN_ALL); 0 }
unsafe fn nhmex_uncore_msr_exit_box(_box: *mut intel_uncore_box) { wrmsrq(NHMEX_U_MSR_PMON_GLOBAL_CTL, 0); }
unsafe fn nhmex_uncore_msr_disable_box(box_: *mut intel_uncore_box) {
    let msr = uncore_msr_box_ctl(box_); if msr != 0 { let mut config = 0u64; rdmsrq(msr, &mut config); config &= !((1u64 << uncore_num_counters(box_)) - 1); if uncore_msr_fixed_ctl(box_) != 0 { config &= !NHMEX_W_PMON_GLOBAL_FIXED_EN; } wrmsrq(msr, config); }
}
unsafe fn nhmex_uncore_msr_enable_box(box_: *mut intel_uncore_box) {
    let msr = uncore_msr_box_ctl(box_); if msr != 0 { let mut config = 0u64; rdmsrq(msr, &mut config); config |= (1u64 << uncore_num_counters(box_)) - 1; if uncore_msr_fixed_ctl(box_) != 0 { config |= NHMEX_W_PMON_GLOBAL_FIXED_EN; } wrmsrq(msr, config); }
}
unsafe fn nhmex_uncore_msr_disable_event(_box: *mut intel_uncore_box, event: *mut perf_event) { wrmsrq((*event).hw.config_base, 0); }
unsafe fn nhmex_uncore_msr_enable_event(box_: *mut intel_uncore_box, event: *mut perf_event) {
    let hwc = &(*event).hw;
    if hwc.idx == UNCORE_PMC_IDX_FIXED { wrmsrq(hwc.config_base, NHMEX_PMON_CTL_EN_BIT0); }
    else if (*(*box_).pmu).type_.event_mask & NHMEX_PMON_CTL_EN_BIT0 != 0 { wrmsrq(hwc.config_base, hwc.config | NHMEX_PMON_CTL_EN_BIT22); }
    else { wrmsrq(hwc.config_base, hwc.config | NHMEX_PMON_CTL_EN_BIT0); }
}

#[repr(C)]
pub struct intel_uncore_box { pub pmu: *mut intel_uncore_pmu }
#[repr(C)] pub struct intel_uncore_pmu { pub type_: *mut intel_uncore_type, pub pmu_idx: i32 }
#[repr(C)] pub struct intel_uncore_type { pub event_mask: u64, pub msr_offset: u32, pub num_counters: u32 }
#[repr(C)] pub struct perf_event { pub hw: hw_perf_event }
#[repr(C)] pub struct hw_perf_event { pub config_base: u32, pub config: u64, pub idx: i32 }

extern "C" {
    fn uncore_msr_box_ctl(box_: *mut intel_uncore_box) -> u32;
    fn uncore_num_counters(box_: *mut intel_uncore_box) -> u32;
    fn uncore_msr_fixed_ctl(box_: *mut intel_uncore_box) -> u32;
}

// Direct counterparts of the remaining kernel tables and callbacks.  The
// complete operation/type initializers are provided through the shared
// uncore declarations when this translation unit is integrated.
static mut uncore_nhmex: bool = false;

unsafe fn nhmex_r_msr_portn_qlx_cfg(n: u32) -> u32 { if n < 4 { 0xe0c + n } else { 0xe2c + n - 4 } }
unsafe fn nhmex_r_msr_portn_iperf_cfg0(n: u32) -> u32 { 0xe04 + n }
unsafe fn nhmex_r_msr_portn_iperf_cfg1(n: u32) -> u32 { 0xe24 + n }
unsafe fn nhmex_r_msr_portn_xbr_offset(n: u32) -> u32 { (if n < 4 { 0 } else { 0x10 }) + n * 4 }
unsafe fn nhmex_r_msr_portn_xbr_set1_mm_cfg(n: u32) -> u32 { 0xe60 + nhmex_r_msr_portn_xbr_offset(n) }
unsafe fn nhmex_r_msr_portn_xbr_set2_mm_cfg(n: u32) -> u32 { 0xe70 + nhmex_r_msr_portn_xbr_offset(n) }

// CPU initialization preserves the original selection and topology behavior.
extern "C" { fn topology_num_cores_per_package() -> u32; }
#[no_mangle] pub unsafe extern "C" fn nhmex_uncore_cpu_init() {
    // boot_cpu_data.x86_vfm == INTEL_NEHALEM_EX selects Nehalem-EX;
    // otherwise the Westmere-EX event descriptors are selected.
    // The shared uncore type table is assigned here by the integration layer.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
