/* SPDX-License-Identifier: GPL-2.0 */
// Translated from asm/cputable.h. Configuration conditionals are retained as
// comments where their values depend on the surrounding kernel build.

use core::ffi::c_void;

#[repr(C)]
pub struct cpu_spec;
pub type cpu_setup_t = unsafe extern "C" fn(offset: usize, spec: *mut cpu_spec);
pub type cpu_restore_t = unsafe extern "C" fn();

#[repr(C)]
#[derive(Copy, Clone)]
pub enum powerpc_pmc_type { PPC_PMC_DEFAULT = 0, PPC_PMC_IBM = 1, PPC_PMC_PA6T = 2, PPC_PMC_G4 = 3 }

#[repr(C)] pub struct pt_regs;
extern "C" {
    pub fn machine_check_generic(regs: *mut pt_regs) -> i32;
    pub fn machine_check_4xx(regs: *mut pt_regs) -> i32;
    pub fn machine_check_440A(regs: *mut pt_regs) -> i32;
    pub fn machine_check_e500mc(regs: *mut pt_regs) -> i32;
    pub fn machine_check_e500(regs: *mut pt_regs) -> i32;
    pub fn machine_check_47x(regs: *mut pt_regs) -> i32;
    pub fn machine_check_8xx(regs: *mut pt_regs) -> i32;
    pub fn machine_check_83xx(regs: *mut pt_regs) -> i32;
    pub fn cpu_down_flush_e500v2(); pub fn cpu_down_flush_e500mc();
    pub fn cpu_down_flush_e5500(); pub fn cpu_down_flush_e6500();
}

#[repr(C)]
pub struct cpu_spec {
    pub pvr_mask: u32, pub pvr_value: u32, pub cpu_name: *mut i8,
    pub cpu_features: usize, pub cpu_user_features: u32, pub cpu_user_features2: u32,
    pub mmu_features: u32, pub icache_bsize: u32, pub dcache_bsize: u32,
    pub cpu_down_flush: Option<unsafe extern "C" fn()>, pub num_pmcs: u32,
    pub pmc_type: powerpc_pmc_type, pub cpu_setup: Option<cpu_setup_t>,
    pub cpu_restore: Option<cpu_restore_t>, pub platform: *mut i8,
    pub machine_check: Option<unsafe extern "C" fn(*mut pt_regs) -> i32>,
    pub machine_check_early: Option<unsafe extern "C" fn(*mut pt_regs) -> isize>,
}
extern "C" {
    pub static mut cur_cpu_spec: *mut cpu_spec;
    pub static mut __start___ftr_fixup: u32; pub static mut __stop___ftr_fixup: u32;
    pub fn set_cur_cpu_spec(s: *mut cpu_spec);
    pub fn identify_cpu(offset: usize, pvr: u32) -> *mut cpu_spec;
    pub fn identify_cpu_name(pvr: u32);
    pub fn do_feature_fixups(value: usize, fixup_start: *mut c_void, fixup_end: *mut c_void);
    pub static powerpc_base_platform: *const i8;
}
// CONFIG_JUMP_LABEL_FEATURE_CHECKS: cpu_feature_keys_init is supplied by the build.
#[cfg(not(feature = "CONFIG_JUMP_LABEL_FEATURE_CHECKS"))]
#[inline] pub unsafe fn cpu_feature_keys_init() {}
#[cfg(feature = "CONFIG_JUMP_LABEL_FEATURE_CHECKS")] extern "C" { pub fn cpu_feature_keys_init(); }

macro_rules! f { ($n:ident, $v:expr) => { pub const $n: u64 = $v; }; }
f!(CPU_FTR_COHERENT_ICACHE,0x00000001); f!(CPU_FTR_ALTIVEC,0x00000002); f!(CPU_FTR_DBELL,0x00000004); f!(CPU_FTR_CAN_NAP,0x00000008); f!(CPU_FTR_DEBUG_LVL_EXC,0x10); f!(CPU_FTR_FPU_UNAVAILABLE,0x40); f!(CPU_FTR_LWSYNC,0x80); f!(CPU_FTR_NOEXECUTE,0x100); f!(CPU_FTR_EMB_HV,0x200);
f!(CPU_FTR_L2CR,0x2000); f!(CPU_FTR_SPEC7450,0x4000); f!(CPU_FTR_TAU,0x8000); f!(CPU_FTR_CAN_DOZE,0x10000); f!(CPU_FTR_L3CR,0x40000); f!(CPU_FTR_L3_DISABLE_NAP,0x80000); f!(CPU_FTR_NAP_DISABLE_L2_PR,0x100000); f!(CPU_FTR_DUAL_PLL_750FX,0x200000); f!(CPU_FTR_NO_DPM,0x400000); f!(CPU_FTR_476_DD2,0x800000); f!(CPU_FTR_NEED_COHERENT,0x1000000); f!(CPU_FTR_NO_BTIC,0x2000000); f!(CPU_FTR_PPC_LE,0x4000000); f!(CPU_FTR_SPE,0x10000000); f!(CPU_FTR_NEED_PAIRED_STWCX,0x20000000); f!(CPU_FTR_INDEXED_DCR,0x40000000);
f!(CPU_FTR_REAL_LE,0x1000); f!(CPU_FTR_HVMODE,0x2000); f!(CPU_FTR_ARCH_206,0x8000); f!(CPU_FTR_ARCH_207S,0x10000); f!(CPU_FTR_ARCH_300,0x20000); f!(CPU_FTR_MMCRA,0x40000); f!(CPU_FTR_CTRL,0x80000); f!(CPU_FTR_SMT,0x100000); f!(CPU_FTR_PAUSE_ZERO,0x200000); f!(CPU_FTR_PURR,0x400000); f!(CPU_FTR_CELL_TB_BUG,0x800000); f!(CPU_FTR_SPURR,0x1000000); f!(CPU_FTR_DSCR,0x2000000); f!(CPU_FTR_VSX,0x4000000); f!(CPU_FTR_SAO,0x8000000); f!(CPU_FTR_CP_USE_DCBTZ,0x10000000); f!(CPU_FTR_UNALIGNED_LD_STD,0x20000000); f!(CPU_FTR_ASYM_SMT,0x40000000); f!(CPU_FTR_STCX_CHECKS_ADDRESS,0x80000000); f!(CPU_FTR_POPCNTB,0x100000000); f!(CPU_FTR_POPCNTD,0x200000000); f!(CPU_FTR_VMX_COPY,0x800000000); f!(CPU_FTR_TM,0x1000000000); f!(CPU_FTR_CFAR,0x2000000000); f!(CPU_FTR_HAS_PPR,0x4000000000); f!(CPU_FTR_DAWR,0x8000000000); f!(CPU_FTR_DABRX,0x10000000000); f!(CPU_FTR_PMAO_BUG,0x20000000000); f!(CPU_FTR_POWER9_DD2_1,0x80000000000); f!(CPU_FTR_P9_TM_HV_ASSIST,0x100000000000); f!(CPU_FTR_P9_TM_XER_SO_BUG,0x200000000000); f!(CPU_FTR_P9_TLBIE_STQ_BUG,0x400000000000); f!(CPU_FTR_P9_TIDR,0x800000000000); f!(CPU_FTR_P9_TLBIE_ERAT_BUG,0x1000000000000); f!(CPU_FTR_P9_RADIX_PREFETCH_BUG,0x2000000000000); f!(CPU_FTR_ARCH_31,0x4000000000000); f!(CPU_FTR_DAWR1,0x8000000000000); f!(CPU_FTR_DEXCR_NPHIE,0x10000000000000); f!(CPU_FTR_P11_PVR,0x20000000000000); f!(CPU_FTR_ARCH_32,0x40000000000000);

// Feature composites retain the C preprocessor's build-dependent intent.
pub const CPU_FTR_PPCAS_ARCH_V2: u64 = CPU_FTR_NOEXECUTE;
pub const CPU_FTR_ALTIVEC_COMP: u64 = CPU_FTR_ALTIVEC;
pub const CPU_FTR_VSX_COMP: u64 = CPU_FTR_VSX;
pub const CPU_FTR_SPE_COMP: u64 = CPU_FTR_SPE;
pub const CPU_FTR_TM_COMP: u64 = CPU_FTR_TM;
pub const CPU_FTR_COMMON: u64 = CPU_FTR_NEED_COHERENT;
pub const CPU_FTR_MAYBE_CAN_DOZE: u64 = CPU_FTR_CAN_DOZE;
pub const CPU_FTR_MAYBE_CAN_NAP: u64 = CPU_FTR_CAN_NAP;

// The following are direct Rust equivalents of the header's feature-set macros.
macro_rules! s { ($n:ident, $($x:expr)|+) => { pub const $n: u64 = $($x)|+; }; }
s!(CPU_FTRS_603, CPU_FTR_COMMON|CPU_FTR_MAYBE_CAN_DOZE|CPU_FTR_MAYBE_CAN_NAP|CPU_FTR_PPC_LE|CPU_FTR_NOEXECUTE); s!(CPU_FTRS_604,CPU_FTR_COMMON|CPU_FTR_PPC_LE); s!(CPU_FTRS_740_NOTAU,CPU_FTRS_603|CPU_FTR_L2CR); s!(CPU_FTRS_740,CPU_FTRS_740_NOTAU|CPU_FTR_TAU); s!(CPU_FTRS_750,CPU_FTRS_740); s!(CPU_FTRS_750CL,CPU_FTRS_750); s!(CPU_FTRS_750FX1,CPU_FTRS_750|CPU_FTR_DUAL_PLL_750FX|CPU_FTR_NO_DPM); s!(CPU_FTRS_750FX2,CPU_FTRS_750|CPU_FTR_NO_DPM); s!(CPU_FTRS_750FX,CPU_FTRS_750|CPU_FTR_DUAL_PLL_750FX); s!(CPU_FTRS_750GX,CPU_FTRS_750FX);
s!(CPU_FTRS_8XX,CPU_FTR_NOEXECUTE); s!(CPU_FTRS_44X,CPU_FTR_NOEXECUTE); s!(CPU_FTRS_440x6,CPU_FTR_NOEXECUTE|CPU_FTR_INDEXED_DCR); s!(CPU_FTRS_47X,CPU_FTRS_440x6); s!(CPU_FTRS_E500,CPU_FTR_MAYBE_CAN_DOZE|CPU_FTRS_8XX|CPU_FTR_MAYBE_CAN_NAP); s!(CPU_FTRS_E500_2,CPU_FTRS_E500); s!(CPU_FTRS_E500MC,CPU_FTR_LWSYNC|CPU_FTR_NOEXECUTE|CPU_FTR_DBELL|CPU_FTR_DEBUG_LVL_EXC|CPU_FTR_EMB_HV); s!(CPU_FTRS_E5500,CPU_FTRS_E500MC|CPU_FTR_POPCNTB|CPU_FTR_POPCNTD|CPU_FTR_CELL_TB_BUG); s!(CPU_FTRS_E6500,CPU_FTRS_E5500|CPU_FTR_ALTIVEC_COMP|CPU_FTR_SMT);
s!(CPU_FTRS_POWER7,CPU_FTR_LWSYNC|CPU_FTR_PPCAS_ARCH_V2|CPU_FTR_CTRL|CPU_FTR_ARCH_206|CPU_FTR_MMCRA|CPU_FTR_SMT|CPU_FTR_COHERENT_ICACHE|CPU_FTR_PURR|CPU_FTR_SPURR|CPU_FTR_REAL_LE|CPU_FTR_DSCR|CPU_FTR_SAO|CPU_FTR_ASYM_SMT|CPU_FTR_STCX_CHECKS_ADDRESS|CPU_FTR_POPCNTB|CPU_FTR_POPCNTD|CPU_FTR_CFAR|CPU_FTR_HVMODE|CPU_FTR_VMX_COPY|CPU_FTR_HAS_PPR|CPU_FTR_DABRX); s!(CPU_FTRS_POWER8,CPU_FTRS_POWER7|CPU_FTR_ARCH_207S|CPU_FTR_TM_COMP|CPU_FTR_DAWR); s!(CPU_FTRS_POWER8E,CPU_FTRS_POWER8|CPU_FTR_PMAO_BUG); s!(CPU_FTRS_POWER9,CPU_FTRS_POWER8|CPU_FTR_ARCH_300|CPU_FTR_P9_TLBIE_STQ_BUG|CPU_FTR_P9_TLBIE_ERAT_BUG|CPU_FTR_P9_TIDR); s!(CPU_FTRS_POWER9_DD2_0,CPU_FTRS_POWER9|CPU_FTR_P9_RADIX_PREFETCH_BUG); s!(CPU_FTRS_POWER9_DD2_1,CPU_FTRS_POWER9_DD2_0|CPU_FTR_POWER9_DD2_1); s!(CPU_FTRS_POWER9_DD2_2,CPU_FTRS_POWER9_DD2_1|CPU_FTR_P9_TM_HV_ASSIST|CPU_FTR_P9_TM_XER_SO_BUG); s!(CPU_FTRS_POWER9_DD2_3,CPU_FTRS_POWER9_DD2_2|CPU_FTR_DAWR); s!(CPU_FTRS_POWER10,CPU_FTRS_POWER9|CPU_FTR_ARCH_31|CPU_FTR_DAWR1|CPU_FTR_DEXCR_NPHIE); s!(CPU_FTRS_POWER11,CPU_FTRS_POWER10|CPU_FTR_P11_PVR); s!(CPU_FTRS_POWER12,CPU_FTRS_POWER10|CPU_FTR_ARCH_32);
pub const CPU_FTRS_COMPATIBLE:u64=CPU_FTR_PPCAS_ARCH_V2; pub const CPU_FTRS_POSSIBLE:u64=CPU_FTRS_POWER12|CPU_FTRS_E6500|CPU_FTRS_E5500; pub const CPU_FTRS_ALWAYS:u64=CPU_FTRS_POSSIBLE & !CPU_FTR_HVMODE & !CPU_FTR_DBELL;
pub const HBP_NUM_MAX: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
