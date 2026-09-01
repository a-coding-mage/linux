// SPDX-License-Identifier: GPL-2.0
// Translated from arch/x86/include/asm/msr-index.h.

#[allow(non_snake_case)]
pub const fn BIT(n: u32) -> u64 { 1u64 << n }
#[allow(non_snake_case)]
pub const fn BIT_ULL(n: u32) -> u64 { 1u64 << n }
#[allow(non_snake_case)]
pub const fn GENMASK(h: u32, l: u32) -> u64 { (!0u64 << l) & (!0u64 >> (63 - h)) }
#[allow(non_snake_case)]
pub const fn GENMASK_ULL(h: u32, l: u32) -> u64 { GENMASK(h, l) }

/* SPDX-License-Identifier: GPL-2.0 */


/* CPU model specific register (MSR) numbers. */

/* x86-64 specific MSRs */
pub const MSR_EFER: u64 = 0xc0000080; /* extended feature register */
pub const MSR_STAR: u64 = 0xc0000081; /* legacy mode SYSCALL target */
pub const MSR_LSTAR: u64 = 0xc0000082; /* long mode SYSCALL target */
pub const MSR_CSTAR: u64 = 0xc0000083; /* compat mode SYSCALL target */
pub const MSR_SYSCALL_MASK: u64 = 0xc0000084; /* EFLAGS mask for syscall */
pub const MSR_FS_BASE: u64 = 0xc0000100; /* 64bit FS base */
pub const MSR_GS_BASE: u64 = 0xc0000101; /* 64bit GS base */
pub const MSR_KERNEL_GS_BASE: u64 = 0xc0000102; /* SwapGS GS shadow */
pub const MSR_TSC_AUX: u64 = 0xc0000103; /* Auxiliary TSC */

/* EFER bits: */
pub const _EFER_SCE: u64 = 0;  /* SYSCALL/SYSRET */
pub const _EFER_LME: u64 = 8;  /* Long mode enable */
pub const _EFER_LMA: u64 = 10; /* Long mode active (read-only) */
pub const _EFER_NX: u64 = 11; /* No execute enable */
pub const _EFER_SVME: u64 = 12; /* Enable virtualization */
pub const _EFER_LMSLE: u64 = 13; /* Long Mode Segment Limit Enable */
pub const _EFER_FFXSR: u64 = 14; /* Enable Fast FXSAVE/FXRSTOR */
pub const _EFER_TCE: u64 = 15; /* Enable Translation Cache Extensions */
pub const _EFER_AUTOIBRS: u64 = 21; /* Enable Automatic IBRS */

pub const EFER_SCE: u64 = (1<<_EFER_SCE);
pub const EFER_LME: u64 = (1<<_EFER_LME);
pub const EFER_LMA: u64 = (1<<_EFER_LMA);
pub const EFER_NX: u64 = (1<<_EFER_NX);
pub const EFER_SVME: u64 = (1<<_EFER_SVME);
pub const EFER_LMSLE: u64 = (1<<_EFER_LMSLE);
pub const EFER_FFXSR: u64 = (1<<_EFER_FFXSR);
pub const EFER_TCE: u64 = (1<<_EFER_TCE);
pub const EFER_AUTOIBRS: u64 = (1<<_EFER_ATOIBRS);

/*
 * Architectural memory types that are common to MTRRs, PAT, VMX MSRs, etc.
 * Most MSRs support/allow only a subset of memory types, but the values
 * themselves are common across all relevant MSRs.
 */
pub const X86_MEMTYPE_UC: u64 = 0;	/* Uncacheable, a.k.a. Strong Uncacheable */
pub const X86_MEMTYPE_WC: u64 = 1;	/* Write Combining */
/* RESERVED			2 */
/* RESERVED			3 */
pub const X86_MEMTYPE_WT: u64 = 4;	/* Write Through */
pub const X86_MEMTYPE_WP: u64 = 5;	/* Write Protected */
pub const X86_MEMTYPE_WB: u64 = 6;	/* Write Back */
pub const X86_MEMTYPE_UC_MINUS: u64 = 7;	/* Weak Uncacheabled (PAT only) */

/* FRED MSRs */
pub const MSR_IA32_FRED_RSP0: u64 = 0x1cc;			/* Level 0 stack pointer */
pub const MSR_IA32_FRED_RSP1: u64 = 0x1cd;			/* Level 1 stack pointer */
pub const MSR_IA32_FRED_RSP2: u64 = 0x1ce;			/* Level 2 stack pointer */
pub const MSR_IA32_FRED_RSP3: u64 = 0x1cf;			/* Level 3 stack pointer */
pub const MSR_IA32_FRED_STKLVLS: u64 = 0x1d0;			/* Exception stack levels */
pub const MSR_IA32_FRED_SSP0: u64 = MSR_IA32_PL0_SSP;	/* Level 0 shadow stack pointer */
pub const MSR_IA32_FRED_SSP1: u64 = 0x1d1;			/* Level 1 shadow stack pointer */
pub const MSR_IA32_FRED_SSP2: u64 = 0x1d2;			/* Level 2 shadow stack pointer */
pub const MSR_IA32_FRED_SSP3: u64 = 0x1d3;			/* Level 3 shadow stack pointer */
pub const MSR_IA32_FRED_CONFIG: u64 = 0x1d4;			/* Entrypoint and interrupt stack level */

/* Intel MSRs. Some also available on other CPUs */
pub const MSR_TEST_CTRL: u64 = 0x00000033;
pub const MSR_TEST_CTRL_SPLIT_LOCK_DETECT_BIT: u64 = 29;
pub const MSR_TEST_CTRL_SPLIT_LOCK_DETECT: u64 = BIT(MSR_TEST_CTRL_SPLIT_LOCK_DETECT_BIT);

pub const MSR_IA32_SPEC_CTRL: u64 = 0x00000048; /* Speculation Control */
pub const SPEC_CTRL_IBRS: u64 = BIT(0);	   /* Indirect Branch Restricted Speculation */
pub const SPEC_CTRL_STIBP_SHIFT: u64 = 1;	   /* Single Thread Indirect Branch Predictor (STIBP) bit */
pub const SPEC_CTRL_STIBP: u64 = BIT(SPEC_CTRL_STIBP_SHIFT);	/* STIBP mask */
pub const SPEC_CTRL_SSBD_SHIFT: u64 = 2;	   /* Speculative Store Bypass Disable bit */
pub const SPEC_CTRL_SSBD: u64 = BIT(SPEC_CTRL_SSBD_SHIFT);	/* Speculative Store Bypass Disable */
pub const SPEC_CTRL_RRSBA_DIS_S_SHIFT: u64 = 6;	   /* Disable RRSBA behavior */
pub const SPEC_CTRL_RRSBA_DIS_S: u64 = BIT(SPEC_CTRL_RRSBA_DIS_S_SHIFT);
pub const SPEC_CTRL_BHI_DIS_S_SHIFT: u64 = 10;	   /* Disable Branch History Injection behavior */
pub const SPEC_CTRL_BHI_DIS_S: u64 = BIT(SPEC_CTRL_BHI_DIS_S_SHIFT);

/* A mask for bits which the kernel toggles when controlling mitigations */
pub const SPEC_CTRL_MITIGATIONS_MASK: u64 = (SPEC_CTRL_IBRS | SPEC_CTRL_STIBP | SPEC_CTRL_SSBD  | SPEC_CTRL_RRSBA_DIS_S  | SPEC_CTRL_BHI_DIS_S);

pub const MSR_IA32_PRED_CMD: u64 = 0x00000049; /* Prediction Command */
pub const PRED_CMD_IBPB: u64 = BIT(0);	   /* Indirect Branch Prediction Barrier */
pub const PRED_CMD_SBPB: u64 = BIT(7);	   /* Selective Branch Prediction Barrier */

pub const MSR_PPIN_CTL: u64 = 0x0000004e;
pub const MSR_PPIN: u64 = 0x0000004f;

pub const MSR_IA32_PERFCTR0: u64 = 0x000000c1;
pub const MSR_IA32_PERFCTR1: u64 = 0x000000c2;
pub const MSR_FSB_FREQ: u64 = 0x000000cd;
pub const MSR_PLATFORM_INFO: u64 = 0x000000ce;
pub const MSR_PLATFORM_INFO_CPUID_FAULT_BIT: u64 = 31;
pub const MSR_PLATFORM_INFO_CPUID_FAULT: u64 = BIT_ULL(MSR_PLATFORM_INFO_CPUID_FAT_BIT);

pub const MSR_IA32_UMWAIT_CONTROL: u64 = 0xe1;
pub const MSR_IA32_UMWAIT_CONTROL_C02_DISABLE: u64 = BIT(0);
pub const MSR_IA32_UMWAIT_CONTROL_RESERVED: u64 = BIT(1);
/*
 * The time field is bit[31:2], but representing a 32bit value with
 * bit[1:0] zero.
 */
pub const MSR_IA32_UMWAIT_CONTROL_TIME_MASK: u64 = (!0x03);

/* Abbreviated from Intel SDM name IA32_CORE_CAPABILITIES */
pub const MSR_IA32_CORE_CAPS: u64 = 0x000000cf;
pub const MSR_IA32_CORE_CAPS_INTEGRITY_CAPS_BIT: u64 = 2;
pub const MSR_IA32_CORE_CAPS_INTEGRITY_CAPS: u64 = BIT(MSR_IA32_CORE_CAPS_INTEGRITY_CAPS_BIT);
pub const MSR_IA32_CORE_CAPS_SPLIT_LOCK_DETECT_BIT: u64 = 5;
pub const MSR_IA32_CORE_CAPS_SPLIT_LOCK_DETECT: u64 = BIT(MSR_IA32_CORE_CAPS_SPLIT_LOCK_DETECT_BIT);

pub const MSR_PKG_CST_CONFIG_CONTROL: u64 = 0x000000e2;
pub const NHM_C3_AUTO_DEMOTE: u64 = (1 << 25);
pub const NHM_C1_AUTO_DEMOTE: u64 = (1 << 26);
pub const ATM_LNC_C6_AUTO_DEMOTE: u64 = (1 << 25);
pub const SNB_C3_AUTO_UNDEMOTE: u64 = (1 << 27);
pub const SNB_C1_AUTO_UNDEMOTE: u64 = (1 << 28);

pub const MSR_MTRRcap: u64 = 0x000000fe;

pub const MSR_IA32_ARCH_CAPABILITIES: u64 = 0x0000010a;
pub const ARCH_CAP_RDCL_NO: u64 = BIT(0);	/* Not susceptible to Meltdown */
pub const ARCH_CAP_IBRS_ALL: u64 = BIT(1);	/* Enhanced IBRS support */
pub const ARCH_CAP_RSBA: u64 = BIT(2);	/* RET may use alternative branch predictors */
pub const ARCH_CAP_SKIP_VMENTRY_L1DFLUSH: u64 = BIT(3);	/* Skip L1D flush on vmentry */
pub const ARCH_CAP_SSB_NO: u64 = BIT(4);	/*
						 * Not susceptible to Speculative Store Bypass
						 * attack, so no Speculative Store Bypass
						 * control required.
						 */
pub const ARCH_CAP_MDS_NO: u64 = BIT(5);   /*
						  * Not susceptible to
						  * Microarchitectural Data
						  * Sampling (MDS) vulnerabilities.
						  */
pub const ARCH_CAP_PSCHANGE_MC_NO: u64 = BIT(6);	 /*
						  * The processor is not susceptible to a
						  * machine check error due to modifying the
						  * code page size along with either the
						  * physical address or cache type
						  * without TLB invalidation.
						  */
pub const ARCH_CAP_TSX_CTRL_MSR: u64 = BIT(7);	/* MSR for TSX control is available. */
pub const ARCH_CAP_TAA_NO: u64 = BIT(8);	/*
						 * Not susceptible to
						 * TSX Async Abort (TAA) vulnerabilities.
						 */
pub const ARCH_CAP_SBDR_SSDP_NO: u64 = BIT(13);	/*
						 * Not susceptible to SBDR and SSDP
						 * variants of Processor MMIO stale data
						 * vulnerabilities.
						 */
pub const ARCH_CAP_FBSDP_NO: u64 = BIT(14);	/*
						 * Not susceptible to FBSDP variant of
						 * Processor MMIO stale data
						 * vulnerabilities.
						 */
pub const ARCH_CAP_PSDP_NO: u64 = BIT(15);	/*
						 * Not susceptible to PSDP variant of
						 * Processor MMIO stale data
						 * vulnerabilities.
						 */
pub const ARCH_CAP_MCU_ENUM: u64 = BIT(16); /*
						 * Indicates the presence of microcode update
						 * feature enumeration and status information.
						 */
pub const ARCH_CAP_FB_CLEAR: u64 = BIT(17);	/*
						 * VERW clears CPU fill buffer
						 * even on MDS_NO CPUs.
						 */
pub const ARCH_CAP_FB_CLEAR_CTRL: u64 = BIT(18);	/*
						 * MSR_IA32_MCU_OPT_CTRL[FB_CLEAR_DIS]
						 * bit available to control VERW
						 * behavior.
						 */
pub const ARCH_CAP_RRSBA: u64 = BIT(19);	/*
						 * Indicates RET may use predictors
						 * other than the RSB. With eIBRS
						 * enabled predictions in kernel mode
						 * are restricted to targets in
						 * kernel.
						 */
pub const ARCH_CAP_BHI_NO: u64 = BIT(20);	/*
						 * CPU is not affected by Branch
						 * History Injection.
						 */
pub const ARCH_CAP_XAPIC_DISABLE: u64 = BIT(21);	/*
						 * IA32_XAPIC_DISABLE_STATUS MSR
						 * supported
						 */
pub const ARCH_CAP_PBRSB_NO: u64 = BIT(24);	/*
						 * Not susceptible to Post-Barrier
						 * Return Stack Buffer Predictions.
						 */
pub const ARCH_CAP_GDS_CTRL: u64 = BIT(25);	/*
						 * CPU is vulnerable to Gather
						 * Data Sampling (GDS) and
						 * has controls for mitigation.
						 */
pub const ARCH_CAP_GDS_NO: u64 = BIT(26);	/*
						 * CPU is not vulnerable to Gather
						 * Data Sampling (GDS).
						 */
pub const ARCH_CAP_RFDS_NO: u64 = BIT(27);	/*
						 * Not susceptible to Register
						 * File Data Sampling.
						 */
pub const ARCH_CAP_RFDS_CLEAR: u64 = BIT(28);	/*
						 * VERW clears CPU Register
						 * File.
						 */
pub const ARCH_CAP_ITS_NO: u64 = BIT_ULL(62); /*
						     * Not susceptible to
						     * Indirect Target Selection.
						     * This bit is not set by
						     * HW, but is synthesized by
						     * VMMs for guests to know
						     * their affected status.
						     */

pub const MSR_IA32_FLUSH_CMD: u64 = 0x0000010b;
pub const L1D_FLUSH: u64 = BIT(0);	/*
						 * Writeback and invalidate the
						 * L1 data cache.
						 */

pub const MSR_IA32_BBL_CR_CTL: u64 = 0x00000119;
pub const MSR_IA32_BBL_CR_CTL3: u64 = 0x0000011e;

pub const MSR_IA32_TSX_CTRL: u64 = 0x00000122;
pub const TSX_CTRL_RTM_DISABLE: u64 = BIT(0);	/* Disable RTM feature */
pub const TSX_CTRL_CPUID_CLEAR: u64 = BIT(1);	/* Disable TSX enumeration */

pub const MSR_IA32_MCU_OPT_CTRL: u64 = 0x00000123;
pub const RNGDS_MITG_DIS: u64 = BIT(0);	/* SRBDS support */
pub const RTM_ALLOW: u64 = BIT(1);	/* TSX development mode */
pub const FB_CLEAR_DIS: u64 = BIT(3);	/* CPU Fill buffer clear disable */
pub const GDS_MITG_DIS: u64 = BIT(4);	/* Disable GDS mitigation */
pub const GDS_MITG_LOCKED: u64 = BIT(5);	/* GDS mitigation locked */

pub const MSR_IA32_SYSENTER_CS: u64 = 0x00000174;
pub const MSR_IA32_SYSENTER_ESP: u64 = 0x00000175;
pub const MSR_IA32_SYSENTER_EIP: u64 = 0x00000176;

pub const MSR_IA32_MCG_CAP: u64 = 0x00000179;
pub const MSR_IA32_MCG_STATUS: u64 = 0x0000017a;
pub const MSR_IA32_MCG_CTL: u64 = 0x0000017b;
pub const MSR_ERROR_CONTROL: u64 = 0x0000017f;
pub const MSR_IA32_MCG_EXT_CTL: u64 = 0x000004d0;

pub const MSR_OFFCORE_RSP_0: u64 = 0x000001a6;
pub const MSR_OFFCORE_RSP_1: u64 = 0x000001a7;
pub const MSR_TURBO_RATIO_LIMIT: u64 = 0x000001ad;
pub const MSR_TURBO_RATIO_LIMIT1: u64 = 0x000001ae;
pub const MSR_TURBO_RATIO_LIMIT2: u64 = 0x000001af;

pub const MSR_SNOOP_RSP_0: u64 = 0x00001328;
pub const MSR_SNOOP_RSP_1: u64 = 0x00001329;

pub const MSR_OMR_0: u64 = 0x000003e0;
pub const MSR_OMR_1: u64 = 0x000003e1;
pub const MSR_OMR_2: u64 = 0x000003e2;
pub const MSR_OMR_3: u64 = 0x000003e3;

pub const MSR_LBR_SELECT: u64 = 0x000001c8;
pub const MSR_LBR_TOS: u64 = 0x000001c9;

pub const MSR_IA32_POWER_CTL: u64 = 0x000001fc;
pub const MSR_IA32_POWER_CTL_BIT_EE: u64 = 19;

/* Abbreviated from Intel SDM name IA32_INTEGRITY_CAPABILITIES */
pub const MSR_INTEGRITY_CAPS: u64 = 0x000002d9;
pub const MSR_INTEGRITY_CAPS_ARRAY_BIST_BIT: u64 = 2;
pub const MSR_INTEGRITY_CAPS_ARRAY_BIST: u64 = BIT(MSR_INTEGRITY_CAPS_ARRAY_BIST_BIT);
pub const MSR_INTEGRITY_CAPS_PERIODIC_BIST_BIT: u64 = 4;
pub const MSR_INTEGRITY_CAPS_PERIODIC_BIST: u64 = BIT(MSR_INTEGRITY_CAPS_PERIODIC_BIST_BIT);
pub const MSR_INTEGRITY_CAPS_SBAF_BIT: u64 = 8;
pub const MSR_INTEGRITY_CAPS_SBAF: u64 = BIT(MSR_INTEGRITY_CAPS_SBAF_BIT);
pub const MSR_INTEGRITY_CAPS_SAF_GEN_MASK: u64 = GENMASK_ULL(10, 9);

pub const MSR_LBR_NHM_FROM: u64 = 0x00000680;
pub const MSR_LBR_NHM_TO: u64 = 0x000006c0;
pub const MSR_LBR_CORE_FROM: u64 = 0x00000040;
pub const MSR_LBR_CORE_TO: u64 = 0x00000060;

pub const MSR_LBR_INFO_0: u64 = 0x00000dc0; /* ... 0xddf for _31 */
pub const LBR_INFO_MISPRED: u64 = BIT_ULL(63);
pub const LBR_INFO_IN_TX: u64 = BIT_ULL(62);
pub const LBR_INFO_ABORT: u64 = BIT_ULL(61);
pub const LBR_INFO_CYC_CNT_VALID: u64 = BIT_ULL(60);
pub const LBR_INFO_CYCLES: u64 = 0xffff;
pub const LBR_INFO_BR_TYPE_OFFSET: u64 = 56;
pub const LBR_INFO_BR_TYPE: u64 = (0xf << LBR_INFO_BR_TYPE_OFFSET);
pub const LBR_INFO_BR_CNTR_OFFSET: u64 = 32;
pub const LBR_INFO_BR_CNTR_NUM: u64 = 4;
pub const LBR_INFO_BR_CNTR_BITS: u64 = 2;
pub const LBR_INFO_BR_CNTR_MASK: u64 = GENMASK_ULL(LBR_INFO_BR_CNTR_BITS - 1, 0);
pub const LBR_INFO_BR_CNTR_FULL_MASK: u64 = GENMASK_ULL(LBR_INFO_BR_CNTR_NUM * LBR_INFO_BR_CNTR_BITS - 1, 0);

pub const MSR_ARCH_LBR_CTL: u64 = 0x000014ce;
pub const ARCH_LBR_CTL_LBREN: u64 = BIT(0);
pub const ARCH_LBR_CTL_CPL_OFFSET: u64 = 1;
pub const ARCH_LBR_CTL_CPL: u64 = (0x3 << ARCH_LBR_CTL_CPL_OFFSET);
pub const ARCH_LBR_CTL_STACK_OFFSET: u64 = 3;
pub const ARCH_LBR_CTL_STACK: u64 = (0x1 << ARCH_LBR_CTL_STACK_OFFSET);
pub const ARCH_LBR_CTL_FILTER_OFFSET: u64 = 16;
pub const ARCH_LBR_CTL_FILTER: u64 = (0x7f << ARCH_LBR_CTL_FILTER_OFFSET);
pub const MSR_ARCH_LBR_DEPTH: u64 = 0x000014cf;
pub const MSR_ARCH_LBR_FROM_0: u64 = 0x00001500;
pub const MSR_ARCH_LBR_TO_0: u64 = 0x00001600;
pub const MSR_ARCH_LBR_INFO_0: u64 = 0x00001200;

pub const MSR_IA32_PEBS_ENABLE: u64 = 0x000003f1;
pub const MSR_PEBS_DATA_CFG: u64 = 0x000003f2;
pub const MSR_IA32_DS_AREA: u64 = 0x00000600;
pub const MSR_IA32_PERF_CAPABILITIES: u64 = 0x00000345;
pub const PERF_CAP_METRICS_IDX: u64 = 15;
pub const PERF_CAP_PT_IDX: u64 = 16;

pub const MSR_PEBS_LD_LAT_THRESHOLD: u64 = 0x000003f6;

pub const PERF_CAP_LBR_FMT: u64 = 0x3f;
pub const PERF_CAP_PEBS_TRAP: u64 = BIT_ULL(6);
pub const PERF_CAP_ARCH_REG: u64 = BIT_ULL(7);
pub const PERF_CAP_PEBS_FORMAT: u64 = 0xf00;
pub const PERF_CAP_FW_WRITES: u64 = BIT_ULL(13);
pub const PERF_CAP_PEBS_BASELINE: u64 = BIT_ULL(14);
pub const PERF_CAP_PEBS_TIMING_INFO: u64 = BIT_ULL(17);
pub const PERF_CAP_PEBS_MASK: u64 = (PERF_CAP_PEBS_TRAP | PERF_CAP_ARCH_REG |  PERF_CAP_PEBS_FORMAT | PERF_CAP_PEBS_BASEINE |  PERF_CAP_PEBS_TIMING_INFO);

/* Arch PEBS */
pub const MSR_IA32_PEBS_BASE: u64 = 0x000003f4;
pub const MSR_IA32_PEBS_INDEX: u64 = 0x000003f5;
pub const ARCH_PEBS_OFFSET_MASK: u64 = 0x7fffff;
pub const ARCH_PEBS_INDEX_WR_SHIFT: u64 = 4;

pub const ARCH_PEBS_RELOAD: u64 = 0xffffffff;
pub const ARCH_PEBS_CNTR_ALLOW: u64 = BIT_ULL(35);
pub const ARCH_PEBS_CNTR_GP: u64 = BIT_ULL(36);
pub const ARCH_PEBS_CNTR_FIXED: u64 = BIT_ULL(37);
pub const ARCH_PEBS_CNTR_METRICS: u64 = BIT_ULL(38);
pub const ARCH_PEBS_LBR_SHIFT: u64 = 40;
pub const ARCH_PEBS_LBR: u64 = (0x3 << ARCH_PEBS_LBR_SHIFT);
pub const ARCH_PEBS_VECR_XMM: u64 = BIT_ULL(49);
pub const ARCH_PEBS_GPR: u64 = BIT_ULL(61);
pub const ARCH_PEBS_AUX: u64 = BIT_ULL(62);
pub const ARCH_PEBS_EN: u64 = BIT_ULL(63);
pub const ARCH_PEBS_CNTR_MASK: u64 = (ARCH_PEBS_CNTR_GP | ARCH_PEBS_CNTR_FIXED |  ARCH_PEBS_CNTR_METRICS);

pub const MSR_IA32_RTIT_CTL: u64 = 0x00000570;
pub const RTIT_CTL_TRACEEN: u64 = BIT(0);
pub const RTIT_CTL_CYCLEACC: u64 = BIT(1);
pub const RTIT_CTL_OS: u64 = BIT(2);
pub const RTIT_CTL_USR: u64 = BIT(3);
pub const RTIT_CTL_PWR_EVT_EN: u64 = BIT(4);
pub const RTIT_CTL_FUP_ON_PTW: u64 = BIT(5);
pub const RTIT_CTL_FABRIC_EN: u64 = BIT(6);
pub const RTIT_CTL_CR3EN: u64 = BIT(7);
pub const RTIT_CTL_TOPA: u64 = BIT(8);
pub const RTIT_CTL_MTC_EN: u64 = BIT(9);
pub const RTIT_CTL_TSC_EN: u64 = BIT(10);
pub const RTIT_CTL_DISRETC: u64 = BIT(11);
pub const RTIT_CTL_PTW_EN: u64 = BIT(12);
pub const RTIT_CTL_BRANCH_EN: u64 = BIT(13);
pub const RTIT_CTL_EVENT_EN: u64 = BIT(31);
pub const RTIT_CTL_NOTNT: u64 = BIT_ULL(55);
pub const RTIT_CTL_MTC_RANGE_OFFSET: u64 = 14;
pub const RTIT_CTL_MTC_RANGE: u64 = (0x0f << RTIT_CTL_MTC_RANGE_OFFSET);
pub const RTIT_CTL_CYC_THRESH_OFFSET: u64 = 19;
pub const RTIT_CTL_CYC_THRESH: u64 = (0x0f << RTIT_CTL_CYC_THRESH_OFFSET);
pub const RTIT_CTL_PSB_FREQ_OFFSET: u64 = 24;
pub const RTIT_CTL_PSB_FREQ: u64 = (0x0f << RTIT_CTL_PSB_FREQ_OFFSET);
pub const RTIT_CTL_ADDR0_OFFSET: u64 = 32;
pub const RTIT_CTL_ADDR0: u64 = (0x0f << RTIT_CTL_ADDR0_OFFSET);
pub const RTIT_CTL_ADDR1_OFFSET: u64 = 36;
pub const RTIT_CTL_ADDR1: u64 = (0x0f << RTIT_CTL_ADDR1_OFFSET);
pub const RTIT_CTL_ADDR2_OFFSET: u64 = 40;
pub const RTIT_CTL_ADDR2: u64 = (0x0f << RTIT_CTL_ADDR2_OFFSET);
pub const RTIT_CTL_ADDR3_OFFSET: u64 = 44;
pub const RTIT_CTL_ADDR3: u64 = (0x0f << RTIT_CTL_ADDR3_OFFSET);
pub const MSR_IA32_RTIT_STATUS: u64 = 0x00000571;
pub const RTIT_STATUS_FILTEREN: u64 = BIT(0);
pub const RTIT_STATUS_CONTEXTEN: u64 = BIT(1);
pub const RTIT_STATUS_TRIGGEREN: u64 = BIT(2);
pub const RTIT_STATUS_BUFFOVF: u64 = BIT(3);
pub const RTIT_STATUS_ERROR: u64 = BIT(4);
pub const RTIT_STATUS_STOPPED: u64 = BIT(5);
pub const RTIT_STATUS_BYTECNT_OFFSET: u64 = 32;
pub const RTIT_STATUS_BYTECNT: u64 = (0x1ffff << RTIT_STATUS_BYTECNT_OFFSET);
pub const MSR_IA32_RTIT_ADDR0_A: u64 = 0x00000580;
pub const MSR_IA32_RTIT_ADDR0_B: u64 = 0x00000581;
pub const MSR_IA32_RTIT_ADDR1_A: u64 = 0x00000582;
pub const MSR_IA32_RTIT_ADDR1_B: u64 = 0x00000583;
pub const MSR_IA32_RTIT_ADDR2_A: u64 = 0x00000584;
pub const MSR_IA32_RTIT_ADDR2_B: u64 = 0x00000585;
pub const MSR_IA32_RTIT_ADDR3_A: u64 = 0x00000586;
pub const MSR_IA32_RTIT_ADDR3_B: u64 = 0x00000587;
pub const MSR_IA32_RTIT_CR3_MATCH: u64 = 0x00000572;
pub const MSR_IA32_RTIT_OUTPUT_BASE: u64 = 0x00000560;
pub const MSR_IA32_RTIT_OUTPUT_MASK: u64 = 0x00000561;

pub const MSR_MTRRfix64K_00000: u64 = 0x00000250;
pub const MSR_MTRRfix16K_80000: u64 = 0x00000258;
pub const MSR_MTRRfix16K_A0000: u64 = 0x00000259;
pub const MSR_MTRRfix4K_C0000: u64 = 0x00000268;
pub const MSR_MTRRfix4K_C8000: u64 = 0x00000269;
pub const MSR_MTRRfix4K_D0000: u64 = 0x0000026a;
pub const MSR_MTRRfix4K_D8000: u64 = 0x0000026b;
pub const MSR_MTRRfix4K_E0000: u64 = 0x0000026c;
pub const MSR_MTRRfix4K_E8000: u64 = 0x0000026d;
pub const MSR_MTRRfix4K_F0000: u64 = 0x0000026e;
pub const MSR_MTRRfix4K_F8000: u64 = 0x0000026f;
pub const MSR_MTRRdefType: u64 = 0x000002ff;

pub const MSR_IA32_CR_PAT: u64 = 0x00000277;

#[allow(non_snake_case, clippy::too_many_arguments)]
pub const fn PAT_VALUE(p0: u64, p1: u64, p2: u64, p3: u64, p4: u64, p5: u64, p6: u64, p7: u64) -> u64 {
    p0 | (p1 << 8) | (p2 << 16) | (p3 << 24) | (p4 << 32) | (p5 << 40) | (p6 << 48) | (p7 << 56)
}

pub const MSR_IA32_DEBUGCTLMSR: u64 = 0x000001d9;
pub const MSR_IA32_LASTBRANCHFROMIP: u64 = 0x000001db;
pub const MSR_IA32_LASTBRANCHTOIP: u64 = 0x000001dc;
pub const MSR_IA32_LASTINTFROMIP: u64 = 0x000001dd;
pub const MSR_IA32_LASTINTTOIP: u64 = 0x000001de;

pub const MSR_IA32_PASID: u64 = 0x00000d93;
pub const MSR_IA32_PASID_VALID: u64 = BIT_ULL(31);

/* DEBUGCTLMSR bits (others vary by model): */
pub const DEBUGCTLMSR_LBR_BIT: u64 = 0;	     /* last branch recording */
pub const DEBUGCTLMSR_LBR: u64 = (1 <<  DEBGCTLMSR_LBR_BIT);
pub const DEBUGCTLMSR_BTF_SHIFT: u64 = 1;
pub const DEBUGCTLMSR_BTF: u64 = (1 <<  1); /* single-step on branches */
pub const DEBUGCTLMSR_BUS_LOCK_DETECT: u64 = (1 <<  2);
pub const DEBUGCTLMSR_TR: u64 = (1 <<  6);
pub const DEBUGCTLMSR_BTS: u64 = (1 <<  7);
pub const DEBUGCTLMSR_BTINT: u64 = (1 <<  8);
pub const DEBUGCTLMSR_BTS_OFF_OS: u64 = (1 <<  9);
pub const DEBUGCTLMSR_BTS_OFF_USR: u64 = (1 << 10);
pub const DEBUGCTLMSR_FREEZE_LBRS_ON_PMI: u64 = (1 << 11);
pub const DEBUGCTLMSR_FREEZE_PERFMON_ON_PMI: u64 = (1 << 12);
pub const DEBUGCTLMSR_FREEZE_IN_SMM_BIT: u64 = 14;
pub const DEBUGCTLMSR_FREEZE_IN_SMM: u64 = (1 << DEBGCTLMSR_FREEZE_IN_SMM_BIT);
pub const DEBUGCTLMSR_RTM_DEBUG: u64 = BIT(15);

pub const MSR_PEBS_FRONTEND: u64 = 0x000003f7;

pub const MSR_IA32_MC0_CTL: u64 = 0x00000400;
pub const MSR_IA32_MC0_STATUS: u64 = 0x00000401;
pub const MSR_IA32_MC0_ADDR: u64 = 0x00000402;
pub const MSR_IA32_MC0_MISC: u64 = 0x00000403;

/* C-state Residency Counters */
pub const MSR_PKG_C3_RESIDENCY: u64 = 0x000003f8;
pub const MSR_PKG_C6_RESIDENCY: u64 = 0x000003f9;
pub const MSR_ATOM_PKG_C6_RESIDENCY: u64 = 0x000003fa;
pub const MSR_PKG_C7_RESIDENCY: u64 = 0x000003fa;
pub const MSR_CORE_C3_RESIDENCY: u64 = 0x000003fc;
pub const MSR_CORE_C6_RESIDENCY: u64 = 0x000003fd;
pub const MSR_CORE_C7_RESIDENCY: u64 = 0x000003fe;
pub const MSR_KNL_CORE_C6_RESIDENCY: u64 = 0x000003ff;
pub const MSR_PKG_C2_RESIDENCY: u64 = 0x0000060d;
pub const MSR_PKG_C8_RESIDENCY: u64 = 0x00000630;
pub const MSR_PKG_C9_RESIDENCY: u64 = 0x00000631;
pub const MSR_PKG_C10_RESIDENCY: u64 = 0x00000632;

/* Interrupt Response Limit */
pub const MSR_PKGC3_IRTL: u64 = 0x0000060a;
pub const MSR_PKGC6_IRTL: u64 = 0x0000060b;
pub const MSR_PKGC7_IRTL: u64 = 0x0000060c;
pub const MSR_PKGC8_IRTL: u64 = 0x00000633;
pub const MSR_PKGC9_IRTL: u64 = 0x00000634;
pub const MSR_PKGC10_IRTL: u64 = 0x00000635;

/* Run Time Average Power Limiting (RAPL) Interface */

pub const MSR_VR_CURRENT_CONFIG: u64 = 0x00000601;
pub const MSR_RAPL_POWER_UNIT: u64 = 0x00000606;

pub const MSR_PKG_POWER_LIMIT: u64 = 0x00000610;
pub const MSR_PKG_ENERGY_STATUS: u64 = 0x00000611;
pub const MSR_PKG_PERF_STATUS: u64 = 0x00000613;
pub const MSR_PKG_POWER_INFO: u64 = 0x00000614;

pub const MSR_DRAM_POWER_LIMIT: u64 = 0x00000618;
pub const MSR_DRAM_ENERGY_STATUS: u64 = 0x00000619;
pub const MSR_DRAM_PERF_STATUS: u64 = 0x0000061b;
pub const MSR_DRAM_POWER_INFO: u64 = 0x0000061c;

pub const MSR_PP0_POWER_LIMIT: u64 = 0x00000638;
pub const MSR_PP0_ENERGY_STATUS: u64 = 0x00000639;
pub const MSR_PP0_POLICY: u64 = 0x0000063a;
pub const MSR_PP0_PERF_STATUS: u64 = 0x0000063b;

pub const MSR_PP1_POWER_LIMIT: u64 = 0x00000640;
pub const MSR_PP1_ENERGY_STATUS: u64 = 0x00000641;
pub const MSR_PP1_POLICY: u64 = 0x00000642;

pub const MSR_AMD_RAPL_POWER_UNIT: u64 = 0xc0010299;
pub const MSR_AMD_CORE_ENERGY_STATUS: u64 = 0xc001029a;
pub const MSR_AMD_PKG_ENERGY_STATUS: u64 = 0xc001029b;

/* Config TDP MSRs */
pub const MSR_CONFIG_TDP_NOMINAL: u64 = 0x00000648;
pub const MSR_CONFIG_TDP_LEVEL_1: u64 = 0x00000649;
pub const MSR_CONFIG_TDP_LEVEL_2: u64 = 0x0000064A;
pub const MSR_CONFIG_TDP_CONTROL: u64 = 0x0000064B;
pub const MSR_TURBO_ACTIVATION_RATIO: u64 = 0x0000064C;

pub const MSR_PLATFORM_ENERGY_STATUS: u64 = 0x0000064D;
pub const MSR_SECONDARY_TURBO_RATIO_LIMIT: u64 = 0x00000650;

pub const MSR_PKG_WEIGHTED_CORE_C0_RES: u64 = 0x00000658;
pub const MSR_PKG_ANY_CORE_C0_RES: u64 = 0x00000659;
pub const MSR_PKG_ANY_GFXE_C0_RES: u64 = 0x0000065A;
pub const MSR_PKG_BOTH_CORE_GFXE_C0_RES: u64 = 0x0000065B;

pub const MSR_CORE_C1_RES: u64 = 0x00000660;
pub const MSR_MODULE_C6_RES_MS: u64 = 0x00000664;

pub const MSR_CC6_DEMOTION_POLICY_CONFIG: u64 = 0x00000668;
pub const MSR_MC6_DEMOTION_POLICY_CONFIG: u64 = 0x00000669;

pub const MSR_ATOM_CORE_RATIOS: u64 = 0x0000066a;
pub const MSR_ATOM_CORE_VIDS: u64 = 0x0000066b;
pub const MSR_ATOM_CORE_TURBO_RATIOS: u64 = 0x0000066c;
pub const MSR_ATOM_CORE_TURBO_VIDS: u64 = 0x0000066d;

pub const MSR_CORE_PERF_LIMIT_REASONS: u64 = 0x00000690;
pub const MSR_GFX_PERF_LIMIT_REASONS: u64 = 0x000006B0;
pub const MSR_RING_PERF_LIMIT_REASONS: u64 = 0x000006B1;

/* Control-flow Enforcement Technology MSRs */
pub const MSR_IA32_U_CET: u64 = 0x000006a0; /* user mode cet */
pub const MSR_IA32_S_CET: u64 = 0x000006a2; /* kernel mode cet */
pub const CET_SHSTK_EN: u64 = BIT_ULL(0);
pub const CET_WRSS_EN: u64 = BIT_ULL(1);
pub const CET_ENDBR_EN: u64 = BIT_ULL(2);
pub const CET_LEG_IW_EN: u64 = BIT_ULL(3);
pub const CET_NO_TRACK_EN: u64 = BIT_ULL(4);
pub const CET_SUPPRESS_DISABLE: u64 = BIT_ULL(5);
pub const CET_RESERVED: u64 = (BIT_ULL(6) | BIT_ULL(7) | BIT_ULL(8) | BIT_ULL(9));
pub const CET_SUPPRESS: u64 = BIT_ULL(10);
pub const CET_WAIT_ENDBR: u64 = BIT_ULL(11);

pub const MSR_IA32_PL0_SSP: u64 = 0x000006a4; /* ring-0 shadow stack pointer */
pub const MSR_IA32_PL1_SSP: u64 = 0x000006a5; /* ring-1 shadow stack pointer */
pub const MSR_IA32_PL2_SSP: u64 = 0x000006a6; /* ring-2 shadow stack pointer */
pub const MSR_IA32_PL3_SSP: u64 = 0x000006a7; /* ring-3 shadow stack pointer */
pub const MSR_IA32_INT_SSP_TAB: u64 = 0x000006a8; /* exception shadow stack table */

/* Hardware P state interface */
pub const MSR_PPERF: u64 = 0x0000064e;
pub const MSR_PERF_LIMIT_REASONS: u64 = 0x0000064f;
pub const MSR_PM_ENABLE: u64 = 0x00000770;
pub const MSR_HWP_CAPABILITIES: u64 = 0x00000771;
pub const MSR_HWP_REQUEST_PKG: u64 = 0x00000772;
pub const MSR_HWP_INTERRUPT: u64 = 0x00000773;
pub const MSR_HWP_REQUEST: u64 = 0x00000774;
pub const MSR_HWP_STATUS: u64 = 0x00000777;

/* CPUID.6.EAX */
pub const HWP_BASE_BIT: u64 = (1<<7);
pub const HWP_NOTIFICATIONS_BIT: u64 = (1<<8);
pub const HWP_ACTIVITY_WINDOW_BIT: u64 = (1<<9);
pub const HWP_ENERGY_PERF_PREFERENCE_BIT: u64 = (1<<10);
pub const HWP_PACKAGE_LEVEL_REQUEST_BIT: u64 = (1<<11);

/* IA32_HWP_CAPABILITIES */
#[allow(non_snake_case)]
pub const fn HWP_HIGHEST_PERF(x: u64) -> u64 { (((x) >> 0) & 0xff) }
#[allow(non_snake_case)]
pub const fn HWP_GUARANTEED_PERF(x: u64) -> u64 { (((x) >> 8) & 0xff) }
#[allow(non_snake_case)]
pub const fn HWP_MOSTEFFICIENT_PERF(x: u64) -> u64 { (((x) >> 16) & 0xff) }
#[allow(non_snake_case)]
pub const fn HWP_LOWEST_PERF(x: u64) -> u64 { (((x) >> 24) & 0xff) }

/* IA32_HWP_REQUEST */
#[allow(non_snake_case)]
pub const fn HWP_MIN_PERF(x: u64) -> u64 { (x & 0xff) }
#[allow(non_snake_case)]
pub const fn HWP_MAX_PERF(x: u64) -> u64 { ((x & 0xff) << 8) }
#[allow(non_snake_case)]
pub const fn HWP_DESIRED_PERF(x: u64) -> u64 { ((x & 0xff) << 16) }
#[allow(non_snake_case)]
pub const fn HWP_ENERGY_PERF_PREFERENCE(x: u64) -> u64 { ((x & 0xff) << 24) }
pub const HWP_EPP_PERFORMANCE: u64 = 0x00;
pub const HWP_EPP_BALANCE_PERFORMANCE: u64 = 0x80;
pub const HWP_EPP_BALANCE_POWERSAVE: u64 = 0xC0;
pub const HWP_EPP_POWERSAVE: u64 = 0xFF;
#[allow(non_snake_case)]
pub const fn HWP_ACTIVITY_WINDOW(x: u64) -> u64 { ((x & 0xff3) << 32) }
#[allow(non_snake_case)]
pub const fn HWP_PACKAGE_CONTROL(x: u64) -> u64 { ((x & 0x1) << 42) }

/* IA32_HWP_STATUS */
#[allow(non_snake_case)]
pub const fn HWP_GUARANTEED_CHANGE(x: u64) -> u64 { (x & 0x1) }
#[allow(non_snake_case)]
pub const fn HWP_EXCURSION_TO_MINIMUM(x: u64) -> u64 { (x & 0x4) }

/* IA32_HWP_INTERRUPT */
#[allow(non_snake_case)]
pub const fn HWP_CHANGE_TO_GUARANTEED_INT(x: u64) -> u64 { (x & 0x1) }
#[allow(non_snake_case)]
pub const fn HWP_EXCURSION_TO_MINIMUM_INT(x: u64) -> u64 { (x & 0x2) }

pub const MSR_AMD64_MC0_MASK: u64 = 0xc0010044;

#[allow(non_snake_case)]
pub const fn MSR_IA32_MCx_CTL(x: u64) -> u64 { (MSR_IA32_MC0_CTL + 4*(x)) }
#[allow(non_snake_case)]
pub const fn MSR_IA32_MCx_STATUS(x: u64) -> u64 { (MSR_IA32_MC0_STATUS + 4*(x)) }
#[allow(non_snake_case)]
pub const fn MSR_IA32_MCx_ADDR(x: u64) -> u64 { (MSR_IA32_MC0_ADDR + 4*(x)) }
#[allow(non_snake_case)]
pub const fn MSR_IA32_MCx_MISC(x: u64) -> u64 { (MSR_IA32_MC0_MISC + 4*(x)) }

#[allow(non_snake_case)]
pub const fn MSR_AMD64_MCx_MASK(x: u64) -> u64 { (MSR_AMD64_MC0_MASK + (x)) }

/* These are consecutive and not in the normal 4er MCE bank block */
pub const MSR_IA32_MC0_CTL2: u64 = 0x00000280;
#[allow(non_snake_case)]
pub const fn MSR_IA32_MCx_CTL2(x: u64) -> u64 { (MSR_IA32_MC0_CTL2 + (x)) }

pub const MSR_P6_PERFCTR0: u64 = 0x000000c1;
pub const MSR_P6_PERFCTR1: u64 = 0x000000c2;
pub const MSR_P6_EVNTSEL0: u64 = 0x00000186;
pub const MSR_P6_EVNTSEL1: u64 = 0x00000187;

pub const MSR_KNC_PERFCTR0: u64 = 0x00000020;
pub const MSR_KNC_PERFCTR1: u64 = 0x00000021;
pub const MSR_KNC_EVNTSEL0: u64 = 0x00000028;
pub const MSR_KNC_EVNTSEL1: u64 = 0x00000029;

/* Alternative perfctr range with full access. */
pub const MSR_IA32_PMC0: u64 = 0x000004c1;

/* Auto-reload via MSR instead of DS area */
pub const MSR_RELOAD_PMC0: u64 = 0x000014c1;
pub const MSR_RELOAD_FIXED_CTR0: u64 = 0x00001309;

/* V6 PMON MSR range */
pub const MSR_IA32_PMC_V6_GP0_CTR: u64 = 0x1900;
pub const MSR_IA32_PMC_V6_GP0_CFG_A: u64 = 0x1901;
pub const MSR_IA32_PMC_V6_GP0_CFG_B: u64 = 0x1902;
pub const MSR_IA32_PMC_V6_GP0_CFG_C: u64 = 0x1903;
pub const MSR_IA32_PMC_V6_FX0_CTR: u64 = 0x1980;
pub const MSR_IA32_PMC_V6_FX0_CFG_B: u64 = 0x1982;
pub const MSR_IA32_PMC_V6_FX0_CFG_C: u64 = 0x1983;
pub const MSR_IA32_PMC_V6_STEP: u64 = 4;

/* KeyID partitioning between MKTME and TDX */
pub const MSR_IA32_MKTME_KEYID_PARTITIONING: u64 = 0x00000087;

/*
 * AMD64 MSRs. Not complete. See the architecture manual for a more
 * complete list.
 */
pub const MSR_AMD64_PATCH_LEVEL: u64 = 0x0000008b;
pub const MSR_AMD64_TSC_RATIO: u64 = 0xc0000104;
pub const MSR_AMD64_NB_CFG: u64 = 0xc001001f;
pub const MSR_AMD64_PATCH_LOADER: u64 = 0xc0010020;
pub const MSR_AMD_PERF_CTL: u64 = 0xc0010062;
pub const MSR_AMD_PERF_STATUS: u64 = 0xc0010063;
pub const MSR_AMD_PSTATE_DEF_BASE: u64 = 0xc0010064;
pub const MSR_AMD64_GUEST_TSC_FREQ: u64 = 0xc0010134;
pub const MSR_AMD64_OSVW_ID_LENGTH: u64 = 0xc0010140;
pub const MSR_AMD64_OSVW_STATUS: u64 = 0xc0010141;
pub const MSR_AMD_PPIN_CTL: u64 = 0xc00102f0;
pub const MSR_AMD_PPIN: u64 = 0xc00102f1;
pub const MSR_AMD64_CPUID_FN_7: u64 = 0xc0011002;
pub const MSR_AMD64_CPUID_FN_1: u64 = 0xc0011004;

pub const MSR_AMD64_CPUID_EXT_FEAT: u64 = 0xc0011005;
pub const MSR_AMD64_CPUID_EXT_FEAT_TOPOEXT_BIT: u64 = 54;
pub const MSR_AMD64_CPUID_EXT_FEAT_TOPOEXT: u64 = BIT_ULL(MSR_AMD64_CPUID_EXT_FEAT_TOPOEXT_BIT);

pub const MSR_AMD64_LS_CFG: u64 = 0xc0011020;
pub const MSR_AMD64_DC_CFG: u64 = 0xc0011022;
pub const MSR_AMD64_TW_CFG: u64 = 0xc0011023;

pub const MSR_AMD64_FP_CFG: u64 = 0xc0011028;
pub const MSR_AMD64_FP_CFG_ZEN1_DENORM_FIX_BIT: u64 = 9;

pub const MSR_AMD64_DE_CFG: u64 = 0xc0011029;
pub const MSR_AMD64_DE_CFG_LFENCE_SERIALIZE_BIT: u64 = 1;
pub const MSR_AMD64_DE_CFG_LFENCE_SERIALIZE: u64 = BIT_ULL(MSR_AMD64_DE_CFG_LFENCE_SERIAIZE_BIT);
pub const MSR_AMD64_DE_CFG_ZEN2_FP_BACKUP_FIX_BIT: u64 = 9;

pub const MSR_AMD64_BU_CFG2: u64 = 0xc001102a;
pub const MSR_AMD64_IBSFETCHCTL: u64 = 0xc0011030;
pub const MSR_AMD64_IBSFETCHLINAD: u64 = 0xc0011031;
pub const MSR_AMD64_IBSFETCHPHYSAD: u64 = 0xc0011032;
pub const MSR_AMD64_IBSFETCH_REG_COUNT: u64 = 3;
pub const MSR_AMD64_IBSFETCH_REG_MASK: u64 = ((1<<MSR_AMD64_IBSFETCH_REG_COUNT)-1);
pub const MSR_AMD64_IBSOPCTL: u64 = 0xc0011033;
pub const MSR_AMD64_IBSOPRIP: u64 = 0xc0011034;
pub const MSR_AMD64_IBSOPDATA: u64 = 0xc0011035;
pub const MSR_AMD64_IBSOPDATA2: u64 = 0xc0011036;
pub const MSR_AMD64_IBSOPDATA3: u64 = 0xc0011037;
pub const MSR_AMD64_IBSDCLINAD: u64 = 0xc0011038;
pub const MSR_AMD64_IBSDCPHYSAD: u64 = 0xc0011039;
pub const MSR_AMD64_IBSOP_REG_COUNT: u64 = 7;
pub const MSR_AMD64_IBSOP_REG_MASK: u64 = ((1<<MSR_AMD64_IBSOP_REG_COUNT)-1);
pub const MSR_AMD64_IBSCTL: u64 = 0xc001103a;
pub const MSR_AMD64_IBSBRTARGET: u64 = 0xc001103b;
pub const MSR_AMD64_ICIBSEXTDCTL: u64 = 0xc001103c;
pub const MSR_AMD64_IBSOPDATA4: u64 = 0xc001103d;
pub const MSR_AMD64_IBSOPCTL2: u64 = 0xc001103e;
pub const MSR_AMD64_IBSFETCHCTL2: u64 = 0xc001103f;
pub const MSR_AMD64_IBS_REG_COUNT_MAX: u64 = 8; /* includes MSR_AMD64_IBSBRTARGET */
pub const MSR_AMD64_SVM_AVIC_DOORBELL: u64 = 0xc001011b;
pub const MSR_AMD64_VM_PAGE_FLUSH: u64 = 0xc001011e;
pub const MSR_AMD64_VIRT_SPEC_CTRL: u64 = 0xc001011f;
pub const MSR_AMD64_SEV_ES_GHCB: u64 = 0xc0010130;
pub const MSR_AMD64_SEV: u64 = 0xc0010131;
pub const MSR_AMD64_SEV_ENABLED_BIT: u64 = 0;
pub const MSR_AMD64_SEV_ENABLED: u64 = BIT_ULL(MSR_AMD64_SEV_ENABED_BIT);
pub const MSR_AMD64_SEV_ES_ENABLED_BIT: u64 = 1;
pub const MSR_AMD64_SEV_ES_ENABLED: u64 = BIT_ULL(MSR_AMD64_SEV_ES_ENABED_BIT);
pub const MSR_AMD64_SEV_SNP_ENABLED_BIT: u64 = 2;
pub const MSR_AMD64_SEV_SNP_ENABLED: u64 = BIT_ULL(MSR_AMD64_SEV_SNP_ENABED_BIT);
pub const MSR_AMD64_SNP_VTOM_BIT: u64 = 3;
pub const MSR_AMD64_SNP_VTOM: u64 = BIT_ULL(MSR_AMD64_SNP_VTOM_BIT);
pub const MSR_AMD64_SNP_REFLECT_VC_BIT: u64 = 4;
pub const MSR_AMD64_SNP_REFLECT_VC: u64 = BIT_ULL(MSR_AMD64_SNP_REFECT_VC_BIT);
pub const MSR_AMD64_SNP_RESTRICTED_INJ_BIT: u64 = 5;
pub const MSR_AMD64_SNP_RESTRICTED_INJ: u64 = BIT_ULL(MSR_AMD64_SNP_RESTRICTED_INJ_BIT);
pub const MSR_AMD64_SNP_ALT_INJ_BIT: u64 = 6;
pub const MSR_AMD64_SNP_ALT_INJ: u64 = BIT_ULL(MSR_AMD64_SNP_AT_INJ_BIT);
pub const MSR_AMD64_SNP_DEBUG_SWAP_BIT: u64 = 7;
pub const MSR_AMD64_SNP_DEBUG_SWAP: u64 = BIT_ULL(MSR_AMD64_SNP_DEBG_SWAP_BIT);
pub const MSR_AMD64_SNP_PREVENT_HOST_IBS_BIT: u64 = 8;
pub const MSR_AMD64_SNP_PREVENT_HOST_IBS: u64 = BIT_ULL(MSR_AMD64_SNP_PREVENT_HOST_IBS_BIT);
pub const MSR_AMD64_SNP_BTB_ISOLATION_BIT: u64 = 9;
pub const MSR_AMD64_SNP_BTB_ISOLATION: u64 = BIT_ULL(MSR_AMD64_SNP_BTB_ISOLATION_BIT);
pub const MSR_AMD64_SNP_VMPL_SSS_BIT: u64 = 10;
pub const MSR_AMD64_SNP_VMPL_SSS: u64 = BIT_ULL(MSR_AMD64_SNP_VMPL_SSS_BIT);
pub const MSR_AMD64_SNP_SECURE_TSC_BIT: u64 = 11;
pub const MSR_AMD64_SNP_SECURE_TSC: u64 = BIT_ULL(MSR_AMD64_SNP_SECRE_TSC_BIT);
pub const MSR_AMD64_SNP_VMGEXIT_PARAM_BIT: u64 = 12;
pub const MSR_AMD64_SNP_VMGEXIT_PARAM: u64 = BIT_ULL(MSR_AMD64_SNP_VMGEXIT_PARAM_BIT);
pub const MSR_AMD64_SNP_RESERVED_BIT13: u64 = BIT_ULL(13);
pub const MSR_AMD64_SNP_IBS_VIRT_BIT: u64 = 14;
pub const MSR_AMD64_SNP_IBS_VIRT: u64 = BIT_ULL(MSR_AMD64_SNP_IBS_VIRT_BIT);
pub const MSR_AMD64_SNP_RESERVED_BIT15: u64 = BIT_ULL(15);
pub const MSR_AMD64_SNP_VMSA_REG_PROT_BIT: u64 = 16;
pub const MSR_AMD64_SNP_VMSA_REG_PROT: u64 = BIT_ULL(MSR_AMD64_SNP_VMSA_REG_PROT_BIT);
pub const MSR_AMD64_SNP_SMT_PROT_BIT: u64 = 17;
pub const MSR_AMD64_SNP_SMT_PROT: u64 = BIT_ULL(MSR_AMD64_SNP_SMT_PROT_BIT);
pub const MSR_AMD64_SNP_SECURE_AVIC_BIT: u64 = 18;
pub const MSR_AMD64_SNP_SECURE_AVIC: u64 = BIT_ULL(MSR_AMD64_SNP_SECRE_AVIC_BIT);
pub const MSR_AMD64_SNP_RESERVED_BITS19_22: u64 = GENMASK_ULL(22, 19);
pub const MSR_AMD64_SNP_IBPB_ON_ENTRY_BIT: u64 = 23;
pub const MSR_AMD64_SNP_IBPB_ON_ENTRY: u64 = BIT_ULL(MSR_AMD64_SNP_IBPB_ON_ENTRY_BIT);
pub const MSR_AMD64_SNP_RESV_BIT: u64 = 24;
pub const MSR_AMD64_SNP_RESERVED_MASK: u64 = GENMASK_ULL(63, MSR_AMD64_SNP_RESV_BIT);
pub const MSR_AMD64_SAVIC_CONTROL: u64 = 0xc0010138;
pub const MSR_AMD64_SAVIC_EN_BIT: u64 = 0;
pub const MSR_AMD64_SAVIC_EN: u64 = BIT_ULL(MSR_AMD64_SAVIC_EN_BIT);
pub const MSR_AMD64_SAVIC_ALLOWEDNMI_BIT: u64 = 1;
pub const MSR_AMD64_SAVIC_ALLOWEDNMI: u64 = BIT_ULL(MSR_AMD64_SAVIC_AOWEDNMI_BIT);
pub const MSR_AMD64_RMP_BASE: u64 = 0xc0010132;
pub const MSR_AMD64_RMP_END: u64 = 0xc0010133;
pub const MSR_AMD64_RMP_CFG: u64 = 0xc0010136;
pub const MSR_AMD64_SEG_RMP_ENABLED_BIT: u64 = 0;
pub const MSR_AMD64_SEG_RMP_ENABLED: u64 = BIT_ULL(MSR_AMD64_SEG_RMP_ENABED_BIT);
#[allow(non_snake_case)]
pub const fn MSR_AMD64_RMP_SEGMENT_SHIFT(x: u64) -> u64 { (((x) & GENMASK_ULL(13, 8)) >> 8) }

pub const MSR_SVSM_CAA: u64 = 0xc001f000;

/* AMD Collaborative Processor Performance Control MSRs */
pub const MSR_AMD_CPPC_CAP1: u64 = 0xc00102b0;
pub const MSR_AMD_CPPC_ENABLE: u64 = 0xc00102b1;
pub const MSR_AMD_CPPC_CAP2: u64 = 0xc00102b2;
pub const MSR_AMD_CPPC_REQ: u64 = 0xc00102b3;
pub const MSR_AMD_CPPC_STATUS: u64 = 0xc00102b4;
pub const MSR_AMD_CPPC_REQ2: u64 = 0xc00102b5;

/* Masks for use with MSR_AMD_CPPC_CAP1 */
pub const AMD_CPPC_LOWEST_PERF_MASK: u64 = GENMASK(7, 0);
pub const AMD_CPPC_LOWNONLIN_PERF_MASK: u64 = GENMASK(15, 8);
pub const AMD_CPPC_NOMINAL_PERF_MASK: u64 = GENMASK(23, 16);
pub const AMD_CPPC_HIGHEST_PERF_MASK: u64 = GENMASK(31, 24);
pub const AMD_CPPC_FLOOR_PERF_CNT_MASK: u64 = GENMASK_ULL(39, 32);

/* Masks for use with MSR_AMD_CPPC_REQ */
pub const AMD_CPPC_MAX_PERF_MASK: u64 = GENMASK(7, 0);
pub const AMD_CPPC_MIN_PERF_MASK: u64 = GENMASK(15, 8);
pub const AMD_CPPC_DES_PERF_MASK: u64 = GENMASK(23, 16);
pub const AMD_CPPC_EPP_PERF_MASK: u64 = GENMASK(31, 24);

/* Masks for use with MSR_AMD_CPPC_REQ2 */
pub const AMD_CPPC_FLOOR_PERF_MASK: u64 = GENMASK(7, 0);

/* AMD Performance Counter Global Status and Control MSRs */
pub const MSR_AMD64_PERF_CNTR_GLOBAL_STATUS: u64 = 0xc0000300;
pub const MSR_AMD64_PERF_CNTR_GLOBAL_CTL: u64 = 0xc0000301;
pub const MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_CLR: u64 = 0xc0000302;
pub const MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_SET: u64 = 0xc0000303;

/* AMD Hardware Feedback Support MSRs */
pub const MSR_AMD_WORKLOAD_CLASS_CONFIG: u64 = 0xc0000500;
pub const MSR_AMD_WORKLOAD_CLASS_ID: u64 = 0xc0000501;
pub const MSR_AMD_WORKLOAD_HRST: u64 = 0xc0000502;

/* AMD Last Branch Record MSRs */
pub const MSR_AMD64_LBR_SELECT: u64 = 0xc000010e;

/* Zen4 */
pub const MSR_ZEN4_BP_CFG: u64 = 0xc001102e;
pub const MSR_ZEN4_BP_CFG_BP_SPEC_REDUCE_BIT: u64 = 4;
pub const MSR_ZEN4_BP_CFG_SHARED_BTB_FIX_BIT: u64 = 5;
pub const MSR_ZEN2_BP_CFG_BUG_FIX_BIT: u64 = 33;

/* Fam 19h MSRs */
pub const MSR_F19H_UMC_PERF_CTL: u64 = 0xc0010800;
pub const MSR_F19H_UMC_PERF_CTR: u64 = 0xc0010801;

/* Zen 2 */
pub const MSR_ZEN2_SPECTRAL_CHICKEN: u64 = 0xc00110e3;
pub const MSR_ZEN2_SPECTRAL_CHICKEN_BIT: u64 = 1;

/* Fam 17h MSRs */
pub const MSR_F17H_IRPERF: u64 = 0xc00000e9;

/* Fam 16h MSRs */
pub const MSR_F16H_L2I_PERF_CTL: u64 = 0xc0010230;
pub const MSR_F16H_L2I_PERF_CTR: u64 = 0xc0010231;
pub const MSR_F16H_DR1_ADDR_MASK: u64 = 0xc0011019;
pub const MSR_F16H_DR2_ADDR_MASK: u64 = 0xc001101a;
pub const MSR_F16H_DR3_ADDR_MASK: u64 = 0xc001101b;
pub const MSR_F16H_DR0_ADDR_MASK: u64 = 0xc0011027;

/* Fam 15h MSRs */
pub const MSR_F15H_CU_PWR_ACCUMULATOR: u64 = 0xc001007a;
pub const MSR_F15H_CU_MAX_PWR_ACCUMULATOR: u64 = 0xc001007b;
pub const MSR_F15H_PERF_CTL: u64 = 0xc0010200;
pub const MSR_F15H_PERF_CTL0: u64 = MSR_F15H_PERF_CTL;
pub const MSR_F15H_PERF_CTL1: u64 = (MSR_F15H_PERF_CTL + 2);
pub const MSR_F15H_PERF_CTL2: u64 = (MSR_F15H_PERF_CTL + 4);
pub const MSR_F15H_PERF_CTL3: u64 = (MSR_F15H_PERF_CTL + 6);
pub const MSR_F15H_PERF_CTL4: u64 = (MSR_F15H_PERF_CTL + 8);
pub const MSR_F15H_PERF_CTL5: u64 = (MSR_F15H_PERF_CTL + 10);

pub const MSR_F15H_PERF_CTR: u64 = 0xc0010201;
pub const MSR_F15H_PERF_CTR0: u64 = MSR_F15H_PERF_CTR;
pub const MSR_F15H_PERF_CTR1: u64 = (MSR_F15H_PERF_CTR + 2);
pub const MSR_F15H_PERF_CTR2: u64 = (MSR_F15H_PERF_CTR + 4);
pub const MSR_F15H_PERF_CTR3: u64 = (MSR_F15H_PERF_CTR + 6);
pub const MSR_F15H_PERF_CTR4: u64 = (MSR_F15H_PERF_CTR + 8);
pub const MSR_F15H_PERF_CTR5: u64 = (MSR_F15H_PERF_CTR + 10);

pub const MSR_F15H_NB_PERF_CTL: u64 = 0xc0010240;
pub const MSR_F15H_NB_PERF_CTR: u64 = 0xc0010241;
pub const MSR_F15H_PTSC: u64 = 0xc0010280;
pub const MSR_F15H_IC_CFG: u64 = 0xc0011021;
pub const MSR_F15H_EX_CFG: u64 = 0xc001102c;

/* Fam 10h MSRs */
pub const MSR_FAM10H_MMIO_CONF_BASE: u64 = 0xc0010058;
pub const FAM10H_MMIO_CONF_ENABLE: u64 = (1<<0);
pub const FAM10H_MMIO_CONF_BUSRANGE_MASK: u64 = 0xf;
pub const FAM10H_MMIO_CONF_BUSRANGE_SHIFT: u64 = 2;
pub const FAM10H_MMIO_CONF_BASE_MASK: u64 = 0xfffffff;
pub const FAM10H_MMIO_CONF_BASE_SHIFT: u64 = 20;
pub const MSR_FAM10H_NODE_ID: u64 = 0xc001100c;

/* K8 MSRs */
pub const MSR_K8_TOP_MEM1: u64 = 0xc001001a;
pub const MSR_K8_TOP_MEM2: u64 = 0xc001001d;
pub const MSR_AMD64_SYSCFG: u64 = 0xc0010010;
pub const MSR_AMD64_SYSCFG_MEM_ENCRYPT_BIT: u64 = 23;
pub const MSR_AMD64_SYSCFG_MEM_ENCRYPT: u64 = BIT_ULL(MSR_AMD64_SYSCFG_MEM_ENCRYPT_BIT);
pub const MSR_AMD64_SYSCFG_SNP_EN_BIT: u64 = 24;
pub const MSR_AMD64_SYSCFG_SNP_EN: u64 = BIT_ULL(MSR_AMD64_SYSCFG_SNP_EN_BIT);
pub const MSR_AMD64_SYSCFG_SNP_VMPL_EN_BIT: u64 = 25;
pub const MSR_AMD64_SYSCFG_SNP_VMPL_EN: u64 = BIT_ULL(MSR_AMD64_SYSCFG_SNP_VMPL_EN_BIT);
pub const MSR_AMD64_SYSCFG_MFDM_BIT: u64 = 19;
pub const MSR_AMD64_SYSCFG_MFDM: u64 = BIT_ULL(MSR_AMD64_SYSCFG_MFDM_BIT);

pub const MSR_K8_INT_PENDING_MSG: u64 = 0xc0010055;
/* C1E active bits in int pending message */
pub const K8_INTP_C1E_ACTIVE_MASK: u64 = 0x18000000;
pub const MSR_K8_TSEG_ADDR: u64 = 0xc0010112;
pub const MSR_K8_TSEG_MASK: u64 = 0xc0010113;
pub const K8_MTRRFIXRANGE_DRAM_ENABLE: u64 = 0x00040000; /* MtrrFixDramEn bit    */
pub const K8_MTRRFIXRANGE_DRAM_MODIFY: u64 = 0x00080000; /* MtrrFixDramModEn bit */
pub const K8_MTRR_RDMEM_WRMEM_MASK: u64 = 0x18181818; /* Mask: RdMem|WrMem    */

/* K7 MSRs */
pub const MSR_K7_EVNTSEL0: u64 = 0xc0010000;
pub const MSR_K7_PERFCTR0: u64 = 0xc0010004;
pub const MSR_K7_EVNTSEL1: u64 = 0xc0010001;
pub const MSR_K7_PERFCTR1: u64 = 0xc0010005;
pub const MSR_K7_EVNTSEL2: u64 = 0xc0010002;
pub const MSR_K7_PERFCTR2: u64 = 0xc0010006;
pub const MSR_K7_EVNTSEL3: u64 = 0xc0010003;
pub const MSR_K7_PERFCTR3: u64 = 0xc0010007;
pub const MSR_K7_CLK_CTL: u64 = 0xc001001b;
pub const MSR_K7_HWCR: u64 = 0xc0010015;
pub const MSR_K7_HWCR_SMMLOCK_BIT: u64 = 0;
pub const MSR_K7_HWCR_SMMLOCK: u64 = BIT_ULL(MSR_K7_HWCR_SMMLOCK_BIT);
pub const MSR_K7_HWCR_IRPERF_EN_BIT: u64 = 30;
pub const MSR_K7_HWCR_IRPERF_EN: u64 = BIT_ULL(MSR_K7_HWCR_IRPERF_EN_BIT);
pub const MSR_K7_HWCR_CPUID_USER_DIS_BIT: u64 = 35;
pub const MSR_K7_HWCR_CPUID_USER_DIS: u64 = BIT_ULL(MSR_K7_HWCR_CPUID_USER_DIS_BIT);
pub const MSR_K7_FID_VID_CTL: u64 = 0xc0010041;
pub const MSR_K7_FID_VID_STATUS: u64 = 0xc0010042;
pub const MSR_K7_HWCR_CPB_DIS_BIT: u64 = 25;
pub const MSR_K7_HWCR_CPB_DIS: u64 = BIT_ULL(MSR_K7_HWCR_CPB_DIS_BIT);

/* K6 MSRs */
pub const MSR_K6_WHCR: u64 = 0xc0000082;
pub const MSR_K6_UWCCR: u64 = 0xc0000085;
pub const MSR_K6_EPMR: u64 = 0xc0000086;
pub const MSR_K6_PSOR: u64 = 0xc0000087;
pub const MSR_K6_PFIR: u64 = 0xc0000088;

/* Centaur-Hauls/IDT defined MSRs. */
pub const MSR_IDT_FCR1: u64 = 0x00000107;
pub const MSR_IDT_FCR2: u64 = 0x00000108;
pub const MSR_IDT_FCR3: u64 = 0x00000109;
pub const MSR_IDT_FCR4: u64 = 0x0000010a;

pub const MSR_IDT_MCR0: u64 = 0x00000110;
pub const MSR_IDT_MCR1: u64 = 0x00000111;
pub const MSR_IDT_MCR2: u64 = 0x00000112;
pub const MSR_IDT_MCR3: u64 = 0x00000113;
pub const MSR_IDT_MCR4: u64 = 0x00000114;
pub const MSR_IDT_MCR5: u64 = 0x00000115;
pub const MSR_IDT_MCR6: u64 = 0x00000116;
pub const MSR_IDT_MCR7: u64 = 0x00000117;
pub const MSR_IDT_MCR_CTRL: u64 = 0x00000120;

/* VIA Cyrix defined MSRs*/
pub const MSR_VIA_FCR: u64 = 0x00001107;
pub const MSR_VIA_LONGHAUL: u64 = 0x0000110a;
pub const MSR_VIA_RNG: u64 = 0x0000110b;
pub const MSR_VIA_BCR2: u64 = 0x00001147;

/* Transmeta defined MSRs */
pub const MSR_TMTA_LONGRUN_CTRL: u64 = 0x80868010;
pub const MSR_TMTA_LONGRUN_FLAGS: u64 = 0x80868011;
pub const MSR_TMTA_LRTI_READOUT: u64 = 0x80868018;
pub const MSR_TMTA_LRTI_VOLT_MHZ: u64 = 0x8086801a;

/* Intel defined MSRs. */
pub const MSR_IA32_P5_MC_ADDR: u64 = 0x00000000;
pub const MSR_IA32_P5_MC_TYPE: u64 = 0x00000001;
pub const MSR_IA32_TSC: u64 = 0x00000010;
pub const MSR_IA32_PLATFORM_ID: u64 = 0x00000017;
pub const MSR_IA32_EBL_CR_POWERON: u64 = 0x0000002a;
pub const MSR_EBC_FREQUENCY_ID: u64 = 0x0000002c;
pub const MSR_SMI_COUNT: u64 = 0x00000034;

/* Referred to as IA32_FEATURE_CONTROL in Intel's SDM. */
pub const MSR_IA32_FEAT_CTL: u64 = 0x0000003a;
pub const FEAT_CTL_LOCKED: u64 = BIT(0);
pub const FEAT_CTL_VMX_ENABLED_INSIDE_SMX: u64 = BIT(1);
pub const FEAT_CTL_VMX_ENABLED_OUTSIDE_SMX: u64 = BIT(2);
pub const FEAT_CTL_SGX_LC_ENABLED: u64 = BIT(17);
pub const FEAT_CTL_SGX_ENABLED: u64 = BIT(18);
pub const FEAT_CTL_LMCE_ENABLED: u64 = BIT(20);

pub const MSR_IA32_TSC_ADJUST: u64 = 0x0000003b;
pub const MSR_IA32_BNDCFGS: u64 = 0x00000d90;

pub const MSR_IA32_BNDCFGS_RSVD: u64 = 0x00000ffc;

pub const MSR_IA32_XFD: u64 = 0x000001c4;
pub const MSR_IA32_XFD_ERR: u64 = 0x000001c5;
pub const MSR_IA32_XSS: u64 = 0x00000da0;

pub const MSR_IA32_APICBASE: u64 = 0x0000001b;
pub const MSR_IA32_APICBASE_BSP: u64 = (1<<8);
pub const MSR_IA32_APICBASE_ENABLE: u64 = (1<<11);
pub const MSR_IA32_APICBASE_BASE: u64 = (0xfffff<<12);

pub const MSR_IA32_UCODE_WRITE: u64 = 0x00000079;

pub const MSR_IA32_MCU_ENUMERATION: u64 = 0x0000007b;
pub const MCU_STAGING: u64 = BIT(4);

pub const MSR_IA32_UCODE_REV: u64 = 0x0000008b;

/* Intel SGX Launch Enclave Public Key Hash MSRs */
pub const MSR_IA32_SGXLEPUBKEYHASH0: u64 = 0x0000008C;
pub const MSR_IA32_SGXLEPUBKEYHASH1: u64 = 0x0000008D;
pub const MSR_IA32_SGXLEPUBKEYHASH2: u64 = 0x0000008E;
pub const MSR_IA32_SGXLEPUBKEYHASH3: u64 = 0x0000008F;

pub const MSR_IA32_SMM_MONITOR_CTL: u64 = 0x0000009b;
pub const MSR_IA32_SMBASE: u64 = 0x0000009e;

pub const MSR_IA32_PERF_STATUS: u64 = 0x00000198;
pub const MSR_IA32_PERF_CTL: u64 = 0x00000199;
pub const INTEL_PERF_CTL_MASK: u64 = 0xffff;

/* AMD Branch Sampling configuration */
pub const MSR_AMD_DBG_EXTN_CFG: u64 = 0xc000010f;
pub const MSR_AMD_SAMP_BR_FROM: u64 = 0xc0010300;

pub const DBG_EXTN_CFG_LBRV2EN: u64 = BIT_ULL(6);

pub const MSR_IA32_MPERF: u64 = 0x000000e7;
pub const MSR_IA32_APERF: u64 = 0x000000e8;

pub const MSR_IA32_THERM_CONTROL: u64 = 0x0000019a;
pub const MSR_IA32_THERM_INTERRUPT: u64 = 0x0000019b;

pub const THERM_INT_HIGH_ENABLE: u64 = (1 << 0);
pub const THERM_INT_LOW_ENABLE: u64 = (1 << 1);
pub const THERM_INT_PLN_ENABLE: u64 = (1 << 24);

pub const MSR_IA32_THERM_STATUS: u64 = 0x0000019c;

pub const THERM_STATUS_PROCHOT: u64 = (1 << 0);
pub const THERM_STATUS_POWER_LIMIT: u64 = (1 << 10);

pub const MSR_THERM2_CTL: u64 = 0x0000019d;

pub const MSR_THERM2_CTL_TM_SELECT: u64 = (1 << 16);

pub const MSR_IA32_MISC_ENABLE: u64 = 0x000001a0;

pub const MSR_IA32_TEMPERATURE_TARGET: u64 = 0x000001a2;

pub const MSR_MISC_FEATURE_CONTROL: u64 = 0x000001a4;
pub const MSR_MISC_PWR_MGMT: u64 = 0x000001aa;

pub const MSR_IA32_ENERGY_PERF_BIAS: u64 = 0x000001b0;
pub const ENERGY_PERF_BIAS_PERFORMANCE: u64 = 0;
pub const ENERGY_PERF_BIAS_BALANCE_PERFORMANCE: u64 = 4;
pub const ENERGY_PERF_BIAS_NORMAL: u64 = 6;
pub const ENERGY_PERF_BIAS_NORMAL_POWERSAVE: u64 = 7;
pub const ENERGY_PERF_BIAS_BALANCE_POWERSAVE: u64 = 8;
pub const ENERGY_PERF_BIAS_POWERSAVE: u64 = 15;

pub const MSR_IA32_PACKAGE_THERM_STATUS: u64 = 0x000001b1;

pub const PACKAGE_THERM_STATUS_PROCHOT: u64 = (1 << 0);
pub const PACKAGE_THERM_STATUS_POWER_LIMIT: u64 = (1 << 10);
pub const PACKAGE_THERM_STATUS_HFI_UPDATED: u64 = (1 << 26);

pub const MSR_IA32_PACKAGE_THERM_INTERRUPT: u64 = 0x000001b2;

pub const PACKAGE_THERM_INT_HIGH_ENABLE: u64 = (1 << 0);
pub const PACKAGE_THERM_INT_LOW_ENABLE: u64 = (1 << 1);
pub const PACKAGE_THERM_INT_PLN_ENABLE: u64 = (1 << 24);
pub const PACKAGE_THERM_INT_HFI_ENABLE: u64 = (1 << 25);

/* Thermal Thresholds Support */
pub const THERM_INT_THRESHOLD0_ENABLE: u64 = (1 << 15);
pub const THERM_SHIFT_THRESHOLD0: u64 = 8;
pub const THERM_MASK_THRESHOLD0: u64 = (0x7f << THERM_SHIFT_THRESHOLD0);
pub const THERM_INT_THRESHOLD1_ENABLE: u64 = (1 << 23);
pub const THERM_SHIFT_THRESHOLD1: u64 = 16;
pub const THERM_MASK_THRESHOLD1: u64 = (0x7f << THERM_SHIFT_THRESHOLD1);
pub const THERM_STATUS_THRESHOLD0: u64 = (1 << 6);
pub const THERM_LOG_THRESHOLD0: u64 = (1 << 7);
pub const THERM_STATUS_THRESHOLD1: u64 = (1 << 8);
pub const THERM_LOG_THRESHOLD1: u64 = (1 << 9);

/* MISC_ENABLE bits: architectural */
pub const MSR_IA32_MISC_ENABLE_FAST_STRING_BIT: u64 = 0;
pub const MSR_IA32_MISC_ENABLE_FAST_STRING: u64 = (1 << MSR_IA32_MISC_ENABE_FAST_STRING_BIT);
pub const MSR_IA32_MISC_ENABLE_TCC_BIT: u64 = 1;
pub const MSR_IA32_MISC_ENABLE_TCC: u64 = (1 << MSR_IA32_MISC_ENABE_TCC_BIT);
pub const MSR_IA32_MISC_ENABLE_EMON_BIT: u64 = 7;
pub const MSR_IA32_MISC_ENABLE_EMON: u64 = (1 << MSR_IA32_MISC_ENABE_EMON_BIT);
pub const MSR_IA32_MISC_ENABLE_BTS_UNAVAIL_BIT: u64 = 11;
pub const MSR_IA32_MISC_ENABLE_BTS_UNAVAIL: u64 = (1 << MSR_IA32_MISC_ENABE_BTS_UNAVAIL_BIT);
pub const MSR_IA32_MISC_ENABLE_PEBS_UNAVAIL_BIT: u64 = 12;
pub const MSR_IA32_MISC_ENABLE_PEBS_UNAVAIL: u64 = (1 << MSR_IA32_MISC_ENABE_PEBS_UNAVAIL_BIT);
pub const MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP_BIT: u64 = 16;
pub const MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP: u64 = (1 << MSR_IA32_MISC_ENABE_ENHANCED_SPEEDSTEP_BIT);
pub const MSR_IA32_MISC_ENABLE_MWAIT_BIT: u64 = 18;
pub const MSR_IA32_MISC_ENABLE_MWAIT: u64 = (1 << MSR_IA32_MISC_ENABE_MWAIT_BIT);
pub const MSR_IA32_MISC_ENABLE_LIMIT_CPUID_BIT: u64 = 22;
pub const MSR_IA32_MISC_ENABLE_LIMIT_CPUID: u64 = (1 << MSR_IA32_MISC_ENABE_LIMIT_CPUID_BIT);
pub const MSR_IA32_MISC_ENABLE_XTPR_DISABLE_BIT: u64 = 23;
pub const MSR_IA32_MISC_ENABLE_XTPR_DISABLE: u64 = (1 << MSR_IA32_MISC_ENABE_XTPR_DISABE_BIT);
pub const MSR_IA32_MISC_ENABLE_XD_DISABLE_BIT: u64 = 34;
pub const MSR_IA32_MISC_ENABLE_XD_DISABLE: u64 = (1 << MSR_IA32_MISC_ENABE_XD_DISABE_BIT);

/* MISC_ENABLE bits: model-specific, meaning may vary from core to core */
pub const MSR_IA32_MISC_ENABLE_X87_COMPAT_BIT: u64 = 2;
pub const MSR_IA32_MISC_ENABLE_X87_COMPAT: u64 = (1 << MSR_IA32_MISC_ENABE_X87_COMPAT_BIT);
pub const MSR_IA32_MISC_ENABLE_TM1_BIT: u64 = 3;
pub const MSR_IA32_MISC_ENABLE_TM1: u64 = (1 << MSR_IA32_MISC_ENABE_TM1_BIT);
pub const MSR_IA32_MISC_ENABLE_SPLIT_LOCK_DISABLE_BIT: u64 = 4;
pub const MSR_IA32_MISC_ENABLE_SPLIT_LOCK_DISABLE: u64 = (1 << MSR_IA32_MISC_ENABE_SPLIT_LOCK_DISABE_BIT);
pub const MSR_IA32_MISC_ENABLE_L3CACHE_DISABLE_BIT: u64 = 6;
pub const MSR_IA32_MISC_ENABLE_L3CACHE_DISABLE: u64 = (1 << MSR_IA32_MISC_ENABE_L3CACHE_DISABE_BIT);
pub const MSR_IA32_MISC_ENABLE_SUPPRESS_LOCK_BIT: u64 = 8;
pub const MSR_IA32_MISC_ENABLE_SUPPRESS_LOCK: u64 = (1 << MSR_IA32_MISC_ENABE_SUPPRESS_LOCK_BIT);
pub const MSR_IA32_MISC_ENABLE_PREFETCH_DISABLE_BIT: u64 = 9;
pub const MSR_IA32_MISC_ENABLE_PREFETCH_DISABLE: u64 = (1 << MSR_IA32_MISC_ENABE_PREFETCH_DISABE_BIT);
pub const MSR_IA32_MISC_ENABLE_FERR_BIT: u64 = 10;
pub const MSR_IA32_MISC_ENABLE_FERR: u64 = (1 << MSR_IA32_MISC_ENABE_FERR_BIT);
pub const MSR_IA32_MISC_ENABLE_FERR_MULTIPLEX_BIT: u64 = 10;
pub const MSR_IA32_MISC_ENABLE_FERR_MULTIPLEX: u64 = (1 << MSR_IA32_MISC_ENABE_FERR_MULTIPLEX_BIT);
pub const MSR_IA32_MISC_ENABLE_TM2_BIT: u64 = 13;
pub const MSR_IA32_MISC_ENABLE_TM2: u64 = (1 << MSR_IA32_MISC_ENABE_TM2_BIT);
pub const MSR_IA32_MISC_ENABLE_ADJ_PREF_DISABLE_BIT: u64 = 19;
pub const MSR_IA32_MISC_ENABLE_ADJ_PREF_DISABLE: u64 = (1 << MSR_IA32_MISC_ENABE_ADJ_PREF_DISABE_BIT);
pub const MSR_IA32_MISC_ENABLE_SPEEDSTEP_LOCK_BIT: u64 = 20;
pub const MSR_IA32_MISC_ENABLE_SPEEDSTEP_LOCK: u64 = (1 << MSR_IA32_MISC_ENABE_SPEEDSTEP_LOCK_BIT);
pub const MSR_IA32_MISC_ENABLE_L1D_CONTEXT_BIT: u64 = 24;
pub const MSR_IA32_MISC_ENABLE_L1D_CONTEXT: u64 = (1 << MSR_IA32_MISC_ENABE_L1D_CONTEXT_BIT);
pub const MSR_IA32_MISC_ENABLE_DCU_PREF_DISABLE_BIT: u64 = 37;
pub const MSR_IA32_MISC_ENABLE_DCU_PREF_DISABLE: u64 = (1 << MSR_IA32_MISC_ENABE_DC_PREF_DISABE_BIT);
pub const MSR_IA32_MISC_ENABLE_TURBO_DISABLE_BIT: u64 = 38;
pub const MSR_IA32_MISC_ENABLE_TURBO_DISABLE: u64 = (1 << MSR_IA32_MISC_ENABE_TURBO_DISABE_BIT);
pub const MSR_IA32_MISC_ENABLE_IP_PREF_DISABLE_BIT: u64 = 39;
pub const MSR_IA32_MISC_ENABLE_IP_PREF_DISABLE: u64 = (1 << MSR_IA32_MISC_ENABE_IP_PREF_DISABE_BIT);

/* MISC_FEATURES_ENABLES non-architectural features */
pub const MSR_MISC_FEATURES_ENABLES: u64 = 0x00000140;

pub const MSR_MISC_FEATURES_ENABLES_CPUID_FAULT_BIT: u64 = 0;
pub const MSR_MISC_FEATURES_ENABLES_CPUID_FAULT: u64 = BIT_ULL(MSR_MISC_FEATURES_ENABES_CPUID_FAT_BIT);
pub const MSR_MISC_FEATURES_ENABLES_RING3MWAIT_BIT: u64 = 1;

pub const MSR_IA32_TSC_DEADLINE: u64 = 0x000006E0;


pub const MSR_TSX_FORCE_ABORT: u64 = 0x0000010F;

pub const MSR_TFA_RTM_FORCE_ABORT_BIT: u64 = 0;
pub const MSR_TFA_RTM_FORCE_ABORT: u64 = BIT_ULL(MSR_TFA_RTM_FORCE_ABORT_BIT);
pub const MSR_TFA_TSX_CPUID_CLEAR_BIT: u64 = 1;
pub const MSR_TFA_TSX_CPUID_CLEAR: u64 = BIT_ULL(MSR_TFA_TSX_CPUID_CEAR_BIT);
pub const MSR_TFA_SDV_ENABLE_RTM_BIT: u64 = 2;
pub const MSR_TFA_SDV_ENABLE_RTM: u64 = BIT_ULL(MSR_TFA_SDV_ENABE_RTM_BIT);

/* P4/Xeon+ specific */
pub const MSR_IA32_MCG_EAX: u64 = 0x00000180;
pub const MSR_IA32_MCG_EBX: u64 = 0x00000181;
pub const MSR_IA32_MCG_ECX: u64 = 0x00000182;
pub const MSR_IA32_MCG_EDX: u64 = 0x00000183;
pub const MSR_IA32_MCG_ESI: u64 = 0x00000184;
pub const MSR_IA32_MCG_EDI: u64 = 0x00000185;
pub const MSR_IA32_MCG_EBP: u64 = 0x00000186;
pub const MSR_IA32_MCG_ESP: u64 = 0x00000187;
pub const MSR_IA32_MCG_EFLAGS: u64 = 0x00000188;
pub const MSR_IA32_MCG_EIP: u64 = 0x00000189;
pub const MSR_IA32_MCG_RESERVED: u64 = 0x0000018a;

/* Pentium IV performance counter MSRs */
pub const MSR_P4_BPU_PERFCTR0: u64 = 0x00000300;
pub const MSR_P4_BPU_PERFCTR1: u64 = 0x00000301;
pub const MSR_P4_BPU_PERFCTR2: u64 = 0x00000302;
pub const MSR_P4_BPU_PERFCTR3: u64 = 0x00000303;
pub const MSR_P4_MS_PERFCTR0: u64 = 0x00000304;
pub const MSR_P4_MS_PERFCTR1: u64 = 0x00000305;
pub const MSR_P4_MS_PERFCTR2: u64 = 0x00000306;
pub const MSR_P4_MS_PERFCTR3: u64 = 0x00000307;
pub const MSR_P4_FLAME_PERFCTR0: u64 = 0x00000308;
pub const MSR_P4_FLAME_PERFCTR1: u64 = 0x00000309;
pub const MSR_P4_FLAME_PERFCTR2: u64 = 0x0000030a;
pub const MSR_P4_FLAME_PERFCTR3: u64 = 0x0000030b;
pub const MSR_P4_IQ_PERFCTR0: u64 = 0x0000030c;
pub const MSR_P4_IQ_PERFCTR1: u64 = 0x0000030d;
pub const MSR_P4_IQ_PERFCTR2: u64 = 0x0000030e;
pub const MSR_P4_IQ_PERFCTR3: u64 = 0x0000030f;
pub const MSR_P4_IQ_PERFCTR4: u64 = 0x00000310;
pub const MSR_P4_IQ_PERFCTR5: u64 = 0x00000311;
pub const MSR_P4_BPU_CCCR0: u64 = 0x00000360;
pub const MSR_P4_BPU_CCCR1: u64 = 0x00000361;
pub const MSR_P4_BPU_CCCR2: u64 = 0x00000362;
pub const MSR_P4_BPU_CCCR3: u64 = 0x00000363;
pub const MSR_P4_MS_CCCR0: u64 = 0x00000364;
pub const MSR_P4_MS_CCCR1: u64 = 0x00000365;
pub const MSR_P4_MS_CCCR2: u64 = 0x00000366;
pub const MSR_P4_MS_CCCR3: u64 = 0x00000367;
pub const MSR_P4_FLAME_CCCR0: u64 = 0x00000368;
pub const MSR_P4_FLAME_CCCR1: u64 = 0x00000369;
pub const MSR_P4_FLAME_CCCR2: u64 = 0x0000036a;
pub const MSR_P4_FLAME_CCCR3: u64 = 0x0000036b;
pub const MSR_P4_IQ_CCCR0: u64 = 0x0000036c;
pub const MSR_P4_IQ_CCCR1: u64 = 0x0000036d;
pub const MSR_P4_IQ_CCCR2: u64 = 0x0000036e;
pub const MSR_P4_IQ_CCCR3: u64 = 0x0000036f;
pub const MSR_P4_IQ_CCCR4: u64 = 0x00000370;
pub const MSR_P4_IQ_CCCR5: u64 = 0x00000371;
pub const MSR_P4_ALF_ESCR0: u64 = 0x000003ca;
pub const MSR_P4_ALF_ESCR1: u64 = 0x000003cb;
pub const MSR_P4_BPU_ESCR0: u64 = 0x000003b2;
pub const MSR_P4_BPU_ESCR1: u64 = 0x000003b3;
pub const MSR_P4_BSU_ESCR0: u64 = 0x000003a0;
pub const MSR_P4_BSU_ESCR1: u64 = 0x000003a1;
pub const MSR_P4_CRU_ESCR0: u64 = 0x000003b8;
pub const MSR_P4_CRU_ESCR1: u64 = 0x000003b9;
pub const MSR_P4_CRU_ESCR2: u64 = 0x000003cc;
pub const MSR_P4_CRU_ESCR3: u64 = 0x000003cd;
pub const MSR_P4_CRU_ESCR4: u64 = 0x000003e0;
pub const MSR_P4_CRU_ESCR5: u64 = 0x000003e1;
pub const MSR_P4_DAC_ESCR0: u64 = 0x000003a8;
pub const MSR_P4_DAC_ESCR1: u64 = 0x000003a9;
pub const MSR_P4_FIRM_ESCR0: u64 = 0x000003a4;
pub const MSR_P4_FIRM_ESCR1: u64 = 0x000003a5;
pub const MSR_P4_FLAME_ESCR0: u64 = 0x000003a6;
pub const MSR_P4_FLAME_ESCR1: u64 = 0x000003a7;
pub const MSR_P4_FSB_ESCR0: u64 = 0x000003a2;
pub const MSR_P4_FSB_ESCR1: u64 = 0x000003a3;
pub const MSR_P4_IQ_ESCR0: u64 = 0x000003ba;
pub const MSR_P4_IQ_ESCR1: u64 = 0x000003bb;
pub const MSR_P4_IS_ESCR0: u64 = 0x000003b4;
pub const MSR_P4_IS_ESCR1: u64 = 0x000003b5;
pub const MSR_P4_ITLB_ESCR0: u64 = 0x000003b6;
pub const MSR_P4_ITLB_ESCR1: u64 = 0x000003b7;
pub const MSR_P4_IX_ESCR0: u64 = 0x000003c8;
pub const MSR_P4_IX_ESCR1: u64 = 0x000003c9;
pub const MSR_P4_MOB_ESCR0: u64 = 0x000003aa;
pub const MSR_P4_MOB_ESCR1: u64 = 0x000003ab;
pub const MSR_P4_MS_ESCR0: u64 = 0x000003c0;
pub const MSR_P4_MS_ESCR1: u64 = 0x000003c1;
pub const MSR_P4_PMH_ESCR0: u64 = 0x000003ac;
pub const MSR_P4_PMH_ESCR1: u64 = 0x000003ad;
pub const MSR_P4_RAT_ESCR0: u64 = 0x000003bc;
pub const MSR_P4_RAT_ESCR1: u64 = 0x000003bd;
pub const MSR_P4_SAAT_ESCR0: u64 = 0x000003ae;
pub const MSR_P4_SAAT_ESCR1: u64 = 0x000003af;
pub const MSR_P4_SSU_ESCR0: u64 = 0x000003be;
pub const MSR_P4_SSU_ESCR1: u64 = 0x000003bf; /* guess: not in manual */

pub const MSR_P4_TBPU_ESCR0: u64 = 0x000003c2;
pub const MSR_P4_TBPU_ESCR1: u64 = 0x000003c3;
pub const MSR_P4_TC_ESCR0: u64 = 0x000003c4;
pub const MSR_P4_TC_ESCR1: u64 = 0x000003c5;
pub const MSR_P4_U2L_ESCR0: u64 = 0x000003b0;
pub const MSR_P4_U2L_ESCR1: u64 = 0x000003b1;

pub const MSR_P4_PEBS_MATRIX_VERT: u64 = 0x000003f2;

/* Intel Core-based CPU performance counters */
pub const MSR_CORE_PERF_FIXED_CTR0: u64 = 0x00000309;
pub const MSR_CORE_PERF_FIXED_CTR1: u64 = 0x0000030a;
pub const MSR_CORE_PERF_FIXED_CTR2: u64 = 0x0000030b;
pub const MSR_CORE_PERF_FIXED_CTR3: u64 = 0x0000030c;
pub const MSR_CORE_PERF_FIXED_CTR_CTRL: u64 = 0x0000038d;
pub const MSR_CORE_PERF_GLOBAL_STATUS: u64 = 0x0000038e;
pub const MSR_CORE_PERF_GLOBAL_CTRL: u64 = 0x0000038f;
pub const MSR_CORE_PERF_GLOBAL_OVF_CTRL: u64 = 0x00000390;
pub const MSR_CORE_PERF_GLOBAL_STATUS_SET: u64 = 0x00000391;

pub const MSR_PERF_METRICS: u64 = 0x00000329;

/* PERF_GLOBAL_OVF_CTL bits */
pub const MSR_CORE_PERF_GLOBAL_OVF_CTRL_TRACE_TOPA_PMI_BIT: u64 = 55;
pub const MSR_CORE_PERF_GLOBAL_OVF_CTRL_TRACE_TOPA_PMI: u64 = (1 << MSR_CORE_PERF_GLOBA_OVF_CTRL_TRACE_TOPA_PMI_BIT);
pub const MSR_CORE_PERF_GLOBAL_OVF_CTRL_OVF_BUF_BIT: u64 = 62;
pub const MSR_CORE_PERF_GLOBAL_OVF_CTRL_OVF_BUF: u64 = (1 <<  MSR_CORE_PERF_GLOBA_OVF_CTRL_OVF_BF_BIT);
pub const MSR_CORE_PERF_GLOBAL_OVF_CTRL_COND_CHGD_BIT: u64 = 63;
pub const MSR_CORE_PERF_GLOBAL_OVF_CTRL_COND_CHGD: u64 = (1 << MSR_CORE_PERF_GLOBA_OVF_CTRL_COND_CHGD_BIT);

/* Geode defined MSRs */
pub const MSR_GEODE_BUSCONT_CONF0: u64 = 0x00001900;

/* Intel VT MSRs */
pub const MSR_IA32_VMX_BASIC: u64 = 0x00000480;
pub const MSR_IA32_VMX_PINBASED_CTLS: u64 = 0x00000481;
pub const MSR_IA32_VMX_PROCBASED_CTLS: u64 = 0x00000482;
pub const MSR_IA32_VMX_EXIT_CTLS: u64 = 0x00000483;
pub const MSR_IA32_VMX_ENTRY_CTLS: u64 = 0x00000484;
pub const MSR_IA32_VMX_MISC: u64 = 0x00000485;
pub const MSR_IA32_VMX_CR0_FIXED0: u64 = 0x00000486;
pub const MSR_IA32_VMX_CR0_FIXED1: u64 = 0x00000487;
pub const MSR_IA32_VMX_CR4_FIXED0: u64 = 0x00000488;
pub const MSR_IA32_VMX_CR4_FIXED1: u64 = 0x00000489;
pub const MSR_IA32_VMX_VMCS_ENUM: u64 = 0x0000048a;
pub const MSR_IA32_VMX_PROCBASED_CTLS2: u64 = 0x0000048b;
pub const MSR_IA32_VMX_EPT_VPID_CAP: u64 = 0x0000048c;
pub const MSR_IA32_VMX_TRUE_PINBASED_CTLS: u64 = 0x0000048d;
pub const MSR_IA32_VMX_TRUE_PROCBASED_CTLS: u64 = 0x0000048e;
pub const MSR_IA32_VMX_TRUE_EXIT_CTLS: u64 = 0x0000048f;
pub const MSR_IA32_VMX_TRUE_ENTRY_CTLS: u64 = 0x00000490;
pub const MSR_IA32_VMX_VMFUNC: u64 = 0x00000491;
pub const MSR_IA32_VMX_PROCBASED_CTLS3: u64 = 0x00000492;

pub const MSR_IA32_MCU_STAGING_MBOX_ADDR: u64 = 0x000007a5;

/* Resctrl MSRs: */
/* - Intel: */
pub const MSR_IA32_L3_QOS_CFG: u64 = 0xc81;
pub const MSR_IA32_L2_QOS_CFG: u64 = 0xc82;
pub const MSR_IA32_QM_EVTSEL: u64 = 0xc8d;
pub const MSR_IA32_QM_CTR: u64 = 0xc8e;
pub const MSR_IA32_PQR_ASSOC: u64 = 0xc8f;
pub const MSR_IA32_L3_CBM_BASE: u64 = 0xc90;
pub const MSR_RMID_SNC_CONFIG: u64 = 0xca0;
pub const MSR_IA32_L2_CBM_BASE: u64 = 0xd10;
pub const MSR_IA32_MBA_THRTL_BASE: u64 = 0xd50;

/* - AMD: */
pub const MSR_IA32_MBA_BW_BASE: u64 = 0xc0000200;
pub const MSR_IA32_SMBA_BW_BASE: u64 = 0xc0000280;
pub const MSR_IA32_L3_QOS_ABMC_CFG: u64 = 0xc00003fd;
pub const MSR_IA32_L3_QOS_EXT_CFG: u64 = 0xc00003ff;
pub const MSR_IA32_EVT_CFG_BASE: u64 = 0xc0000400;

/* AMD-V MSRs */
pub const MSR_VM_CR: u64 = 0xc0010114;
pub const MSR_VM_IGNNE: u64 = 0xc0010115;
pub const MSR_VM_HSAVE_PA: u64 = 0xc0010117;

pub const SVM_VM_CR_VALID_MASK: u64 = 0x001f;
pub const SVM_VM_CR_SVM_LOCK_MASK: u64 = 0x0008;
pub const SVM_VM_CR_SVM_DIS_MASK: u64 = 0x0010;

/* Hardware Feedback Interface */
pub const MSR_IA32_HW_FEEDBACK_PTR: u64 = 0x17d0;
pub const MSR_IA32_HW_FEEDBACK_CONFIG: u64 = 0x17d1;

/* x2APIC locked status */
pub const MSR_IA32_XAPIC_DISABLE_STATUS: u64 = 0xBD;
pub const LEGACY_XAPIC_DISABLED: u64 = BIT(0); /*
						* x2APIC mode is locked and
						* disabling x2APIC will cause
						* a #GP
						*/


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
