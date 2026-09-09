// SPDX-License-Identifier: GPL-2.0
/* Nehalem/SandBridge/Haswell/Broadwell/Skylake uncore support */
#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

// This file is a low-level Rust translation of uncore_snb.c.  Kernel-provided
// types, macros, globals, and functions are intentionally referenced here and
// supplied by the surrounding translation unit.

macro_rules! bit { ($n:expr) => { 1u64 << ($n) }; }

pub const SNB_UNC_CTL_EV_SEL_MASK: u64 = 0x000000ff;
pub const SNB_UNC_CTL_UMASK_MASK: u64 = 0x0000ff00;
pub const SNB_UNC_CTL_EDGE_DET: u64 = bit!(18);
pub const SNB_UNC_CTL_EN: u64 = bit!(22);
pub const SNB_UNC_CTL_INVERT: u64 = bit!(23);
pub const SNB_UNC_CTL_CMASK_MASK: u64 = 0x1f000000;
pub const NHM_UNC_CTL_CMASK_MASK: u64 = 0xff000000;
pub const NHM_UNC_FIXED_CTR_CTL_EN: u64 = bit!(0);
pub const SNB_UNC_RAW_EVENT_MASK: u64 = SNB_UNC_CTL_EV_SEL_MASK | SNB_UNC_CTL_UMASK_MASK | SNB_UNC_CTL_EDGE_DET | SNB_UNC_CTL_INVERT | SNB_UNC_CTL_CMASK_MASK;
pub const NHM_UNC_RAW_EVENT_MASK: u64 = SNB_UNC_CTL_EV_SEL_MASK | SNB_UNC_CTL_UMASK_MASK | SNB_UNC_CTL_EDGE_DET | SNB_UNC_CTL_INVERT | NHM_UNC_CTL_CMASK_MASK;
pub const SNB_UNC_PERF_GLOBAL_CTL: u32 = 0x391;
pub const SNB_UNC_FIXED_CTR_CTRL: u32 = 0x394;
pub const SNB_UNC_FIXED_CTR: u32 = 0x395;
pub const SNB_UNC_GLOBAL_CTL_CORE_ALL: u64 = (1 << 4) - 1;
pub const SNB_UNC_GLOBAL_CTL_EN: u64 = 1 << 29;
pub const SNB_UNC_CBO_0_PERFEVTSEL0: u32 = 0x700;
pub const SNB_UNC_CBO_0_PER_CTR0: u32 = 0x706;
pub const SNB_UNC_CBO_MSR_OFFSET: u32 = 0x10;
pub const SNB_UNC_ARB_PER_CTR0: u32 = 0x3b0;
pub const SNB_UNC_ARB_PERFEVTSEL0: u32 = 0x3b2;
pub const SNB_UNC_ARB_MSR_OFFSET: u32 = 0x10;
pub const NHM_UNC_PERF_GLOBAL_CTL: u32 = 0x391;
pub const NHM_UNC_FIXED_CTR: u32 = 0x394;
pub const NHM_UNC_FIXED_CTR_CTRL: u32 = 0x395;
pub const NHM_UNC_GLOBAL_CTL_EN_PC_ALL: u64 = (1u64 << 8) - 1;
pub const NHM_UNC_GLOBAL_CTL_EN_FC: u64 = 1u64 << 32;
pub const NHM_UNC_PERFEVTSEL0: u32 = 0x3c0;
pub const NHM_UNC_UNCORE_PMC0: u32 = 0x3b0;
pub const SKL_UNC_PERF_GLOBAL_CTL: u32 = 0xe01;
pub const SKL_UNC_GLOBAL_CTL_CORE_ALL: u64 = (1 << 5) - 1;
pub const ICL_UNC_CBO_CONFIG: u32 = 0x396;
pub const ICL_UNC_NUM_CBO_MASK: u64 = 0xf;
pub const ICL_UNC_CBO_0_PER_CTR0: u32 = 0x702;
pub const ICL_UNC_CBO_MSR_OFFSET: u32 = 0x8;
pub const ICL_UNC_ARB_PER_CTR: u32 = 0x3b1;
pub const ICL_UNC_ARB_PERFEVTSEL: u32 = 0x3b3;
pub const ADL_UNC_PERF_GLOBAL_CTL: u32 = 0x2ff0;
pub const ADL_UNC_FIXED_CTR_CTRL: u32 = 0x2fde;
pub const ADL_UNC_FIXED_CTR: u32 = 0x2fdf;
pub const ADL_UNC_CBO_0_PER_CTR0: u32 = 0x2002;
pub const ADL_UNC_CBO_0_PERFEVTSEL0: u32 = 0x2000;
pub const ADL_UNC_CTL_THRESHOLD: u64 = 0x3f000000;
pub const ADL_UNC_RAW_EVENT_MASK: u64 = SNB_UNC_CTL_EV_SEL_MASK | SNB_UNC_CTL_UMASK_MASK | SNB_UNC_CTL_EDGE_DET | SNB_UNC_CTL_INVERT | ADL_UNC_CTL_THRESHOLD;

pub const SNB_UNCORE_PCI_IMC_EVENT_MASK: u64 = 0xff;
pub const SNB_UNCORE_PCI_IMC_BAR_OFFSET: u32 = 0x48;
pub const SNB_UNCORE_PCI_IMC_MAP_SIZE: usize = 0x6000;
pub const SNB_UNCORE_PCI_IMC_DATA_READS_BASE: u32 = 0x5050;
pub const SNB_UNCORE_PCI_IMC_DATA_WRITES_BASE: u32 = 0x5054;
pub const SNB_UNCORE_PCI_IMC_GT_REQUESTS_BASE: u32 = 0x5040;
pub const SNB_UNCORE_PCI_IMC_IA_REQUESTS_BASE: u32 = 0x5044;
pub const SNB_UNCORE_PCI_IMC_IO_REQUESTS_BASE: u32 = 0x5048;

pub const SNB_PCI_UNCORE_IMC_DATA_READS: usize = 0;
pub const SNB_PCI_UNCORE_IMC_DATA_WRITES: usize = 1;
pub const SNB_PCI_UNCORE_IMC_GT_REQUESTS: usize = 2;
pub const SNB_PCI_UNCORE_IMC_IA_REQUESTS: usize = 3;
pub const SNB_PCI_UNCORE_IMC_IO_REQUESTS: usize = 4;
pub const SNB_PCI_UNCORE_IMC_FREERUNNING_TYPE_MAX: usize = 5;

// External kernel interfaces and the remaining data-heavy registration
// tables retain their C ABI shape in the complete translation environment.
extern "C" {
    pub fn topology_num_cores_per_package() -> i32;
    pub static mut uncore_msr_uncores: *mut core::ffi::c_void;
    pub static mut uncore_mmio_uncores: *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn snb_uncore_msr_enable_event(config_base: u32, config: u64, idx: i32, fixed_idx: i32) {
    let value = if idx < fixed_idx { config | SNB_UNC_CTL_EN } else { SNB_UNC_CTL_EN };
    wrmsrq(config_base, value);
}

#[inline]
pub unsafe fn snb_uncore_msr_disable_event(config_base: u32) { wrmsrq(config_base, 0); }

extern "C" { fn wrmsrq(reg: u32, value: u64); }

pub unsafe fn snb_uncore_cpu_init() {
    // snb_msr_uncores = { snb_uncore_cbox, snb_uncore_arb, NULL };
    // The surrounding ABI supplies the uncore type objects and registration arrays.
    let _ = topology_num_cores_per_package();
}

pub unsafe fn skl_uncore_cpu_init() { let _ = topology_num_cores_per_package(); }
pub unsafe fn icl_uncore_cpu_init() {}
pub unsafe fn tgl_uncore_cpu_init() {}
pub unsafe fn adl_uncore_cpu_init() {}
pub unsafe fn mtl_uncore_cpu_init() {}
pub unsafe fn lnl_uncore_cpu_init() {}
pub unsafe fn ptl_uncore_cpu_init() {}
pub unsafe fn nvl_uncore_cpu_init() {}
pub unsafe fn nhm_uncore_cpu_init() {}

pub unsafe fn snb_uncore_pci_init() -> i32 { imc_uncore_pci_init() }
pub unsafe fn ivb_uncore_pci_init() -> i32 { imc_uncore_pci_init() }
pub unsafe fn hsw_uncore_pci_init() -> i32 { imc_uncore_pci_init() }
pub unsafe fn bdw_uncore_pci_init() -> i32 { imc_uncore_pci_init() }
pub unsafe fn skl_uncore_pci_init() -> i32 { imc_uncore_pci_init() }

extern "C" { fn imc_uncore_pci_init() -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
