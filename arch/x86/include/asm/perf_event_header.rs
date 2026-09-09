/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of x86/include/asm/perf_event.h. */

pub const INTEL_PMC_MAX_GENERIC: u32 = 32;
pub const INTEL_PMC_MAX_FIXED: u32 = 16;
pub const INTEL_PMC_IDX_FIXED: u32 = 32;
pub const X86_PMC_IDX_MAX: u32 = 64;
pub const MSR_ARCH_PERFMON_PERFCTR0: u32 = 0xc1;
pub const MSR_ARCH_PERFMON_PERFCTR1: u32 = 0xc2;
pub const MSR_ARCH_PERFMON_EVENTSEL0: u32 = 0x186;
pub const MSR_ARCH_PERFMON_EVENTSEL1: u32 = 0x187;

pub const ARCH_PERFMON_EVENTSEL_EVENT: u64 = 0xff;
pub const ARCH_PERFMON_EVENTSEL_UMASK: u64 = 0xff00;
pub const ARCH_PERFMON_EVENTSEL_USR: u64 = 1 << 16;
pub const ARCH_PERFMON_EVENTSEL_OS: u64 = 1 << 17;
pub const ARCH_PERFMON_EVENTSEL_EDGE: u64 = 1 << 18;
pub const ARCH_PERFMON_EVENTSEL_PIN_CONTROL: u64 = 1 << 19;
pub const ARCH_PERFMON_EVENTSEL_INT: u64 = 1 << 20;
pub const ARCH_PERFMON_EVENTSEL_ANY: u64 = 1 << 21;
pub const ARCH_PERFMON_EVENTSEL_ENABLE: u64 = 1 << 22;
pub const ARCH_PERFMON_EVENTSEL_INV: u64 = 1 << 23;
pub const ARCH_PERFMON_EVENTSEL_CMASK: u64 = 0xff000000;
pub const ARCH_PERFMON_EVENTSEL_BR_CNTR: u64 = 1 << 35;
pub const ARCH_PERFMON_EVENTSEL_EQ: u64 = 1 << 36;
pub const ARCH_PERFMON_EVENTSEL_RDPMC_USER_DISABLE: u64 = 1 << 37;
pub const ARCH_PERFMON_EVENTSEL_UMASK2: u64 = 0xff << 40;
pub const INTEL_FIXED_BITS_STRIDE: u32 = 4;
pub const INTEL_FIXED_0_KERNEL: u64 = 1;
pub const INTEL_FIXED_0_USER: u64 = 1 << 1;
pub const INTEL_FIXED_0_ANYTHREAD: u64 = 1 << 2;
pub const INTEL_FIXED_0_ENABLE_PMI: u64 = 1 << 3;
pub const INTEL_FIXED_0_RDPMC_USER_DISABLE: u64 = 1 << 33;
pub const INTEL_FIXED_3_METRICS_CLEAR: u64 = 1 << 2;
pub const HSW_IN_TX: u64 = 1 << 32;
pub const HSW_IN_TX_CHECKPOINTED: u64 = 1 << 33;
pub const ICL_EVENTSEL_ADAPTIVE: u64 = 1 << 34;
pub const ICL_FIXED_0_ADAPTIVE: u64 = 1 << 32;
pub const INTEL_FIXED_BITS_MASK: u64 = INTEL_FIXED_0_KERNEL | INTEL_FIXED_0_USER | INTEL_FIXED_0_ANYTHREAD | INTEL_FIXED_0_ENABLE_PMI | ICL_FIXED_0_ADAPTIVE | INTEL_FIXED_0_RDPMC_USER_DISABLE;
#[inline] pub const fn intel_fixed_bits_by_idx(idx: u32, bits: u64) -> u64 { bits << (idx * INTEL_FIXED_BITS_STRIDE) }
pub const AMD64_EVENTSEL_INT_CORE_ENABLE: u64 = 1 << 36;
pub const AMD64_EVENTSEL_GUESTONLY: u64 = 1 << 40;
pub const AMD64_EVENTSEL_HOSTONLY: u64 = 1 << 41;
pub const AMD64_EVENTSEL_HOST_GUEST_MASK: u64 = AMD64_EVENTSEL_HOSTONLY | AMD64_EVENTSEL_GUESTONLY;
pub const AMD64_EVENTSEL_INT_CORE_SEL_SHIFT: u32 = 37;
pub const AMD64_EVENTSEL_INT_CORE_SEL_MASK: u64 = 0xf << AMD64_EVENTSEL_INT_CORE_SEL_SHIFT;
pub const AMD64_EVENTSEL_EVENT: u64 = ARCH_PERFMON_EVENTSEL_EVENT | (0xf << 32);
pub const INTEL_ARCH_EVENT_MASK: u64 = ARCH_PERFMON_EVENTSEL_UMASK | ARCH_PERFMON_EVENTSEL_EVENT;
pub const AMD64_L3_SLICE_SHIFT: u32 = 48;
pub const AMD64_L3_SLICE_MASK: u64 = 0xf << AMD64_L3_SLICE_SHIFT;
pub const AMD64_L3_SLICEID_MASK: u64 = 0x7 << AMD64_L3_SLICE_SHIFT;
pub const AMD64_L3_THREAD_SHIFT: u32 = 56;
pub const AMD64_L3_THREAD_MASK: u64 = 0xff << AMD64_L3_THREAD_SHIFT;
pub const AMD64_L3_F19H_THREAD_MASK: u64 = 0x3 << AMD64_L3_THREAD_SHIFT;
pub const AMD64_L3_EN_ALL_CORES: u64 = 1 << 47;
pub const AMD64_L3_EN_ALL_SLICES: u64 = 1 << 46;
pub const AMD64_L3_COREID_SHIFT: u32 = 42;
pub const AMD64_L3_COREID_MASK: u64 = 0x7 << AMD64_L3_COREID_SHIFT;
pub const X86_RAW_EVENT_MASK: u64 = ARCH_PERFMON_EVENTSEL_EVENT | ARCH_PERFMON_EVENTSEL_UMASK | ARCH_PERFMON_EVENTSEL_EDGE | ARCH_PERFMON_EVENTSEL_INV | ARCH_PERFMON_EVENTSEL_CMASK;
pub const X86_ALL_EVENT_FLAGS: u64 = ARCH_PERFMON_EVENTSEL_EDGE | ARCH_PERFMON_EVENTSEL_INV | ARCH_PERFMON_EVENTSEL_CMASK | ARCH_PERFMON_EVENTSEL_ANY | ARCH_PERFMON_EVENTSEL_PIN_CONTROL | HSW_IN_TX | HSW_IN_TX_CHECKPOINTED;
pub const AMD64_RAW_EVENT_MASK: u64 = X86_RAW_EVENT_MASK | AMD64_EVENTSEL_EVENT;
pub const AMD64_RAW_EVENT_MASK_NB: u64 = AMD64_EVENTSEL_EVENT | ARCH_PERFMON_EVENTSEL_UMASK;
pub const AMD64_PERFMON_V2_EVENTSEL_EVENT_NB: u64 = AMD64_EVENTSEL_EVENT | (0x3 << 36);
pub const AMD64_PERFMON_V2_EVENTSEL_UMASK_NB: u64 = ARCH_PERFMON_EVENTSEL_UMASK | (0xf << 24);
pub const AMD64_PERFMON_V2_RAW_EVENT_MASK_NB: u64 = AMD64_PERFMON_V2_EVENTSEL_EVENT_NB | AMD64_PERFMON_V2_EVENTSEL_UMASK_NB;
pub const AMD64_PERFMON_V2_ENABLE_UMC: u64 = 1 << 31;
pub const AMD64_PERFMON_V2_EVENTSEL_EVENT_UMC: u64 = 0xff;
pub const AMD64_PERFMON_V2_EVENTSEL_RDWRMASK_UMC: u64 = 0x3 << 8;
pub const AMD64_PERFMON_V2_RAW_EVENT_MASK_UMC: u64 = AMD64_PERFMON_V2_EVENTSEL_EVENT_UMC | AMD64_PERFMON_V2_EVENTSEL_RDWRMASK_UMC;
pub const AMD64_NUM_COUNTERS: u32 = 4;
pub const AMD64_NUM_COUNTERS_CORE: u32 = 6;
pub const AMD64_NUM_COUNTERS_NB: u32 = 4;
pub const ARCH_PERFMON_UNHALTED_CORE_CYCLES_SEL: u32 = 0x3c;
pub const ARCH_PERFMON_UNHALTED_CORE_CYCLES_UMASK: u32 = 0;
pub const ARCH_PERFMON_UNHALTED_CORE_CYCLES_INDEX: u32 = 0;
pub const ARCH_PERFMON_UNHALTED_CORE_CYCLES_PRESENT: u32 = 1 << ARCH_PERFMON_UNHALTED_CORE_CYCLES_INDEX;
pub const ARCH_PERFMON_BRANCH_MISSES_RETIRED: u32 = 6;
pub const ARCH_PERFMON_EVENTS_COUNT: u32 = 7;

pub const PEBS_DATACFG_MEMINFO: u64 = 1; pub const PEBS_DATACFG_GP: u64 = 1 << 1; pub const PEBS_DATACFG_XMMS: u64 = 1 << 2; pub const PEBS_DATACFG_LBRS: u64 = 1 << 3; pub const PEBS_DATACFG_CNTR: u64 = 1 << 4; pub const PEBS_DATACFG_METRICS: u64 = 1 << 5;
pub const PEBS_DATACFG_LBR_SHIFT: u32 = 24; pub const PEBS_DATACFG_CNTR_SHIFT: u32 = 32; pub const PEBS_DATACFG_CNTR_MASK: u64 = 0xffff; pub const PEBS_DATACFG_FIX_SHIFT: u32 = 48; pub const PEBS_DATACFG_FIX_MASK: u64 = 0xff; pub const PEBS_UPDATE_DS_SW: u64 = 1 << 63;

/* C bit-field unions are represented by their raw storage plus the documented split view. */
#[repr(C)] pub union cpuid10_eax { pub split: cpuid10_eax_split, pub full: u32 }
#[repr(C)] pub struct cpuid10_eax_split { pub version_id: u32, pub num_counters: u32, pub bit_width: u32, pub mask_length: u32 }
#[repr(C)] pub union cpuid10_ebx { pub split: cpuid10_ebx_split, pub full: u32 }
#[repr(C)] pub struct cpuid10_ebx_split { pub no_unhalted_core_cycles: u32, pub no_instructions_retired: u32, pub no_unhalted_reference_cycles: u32, pub no_llc_reference: u32, pub no_llc_misses: u32, pub no_branch_instruction_retired: u32, pub no_branch_misses_retired: u32 }
#[repr(C)] pub union cpuid10_edx { pub split: cpuid10_edx_split, pub full: u32 }
#[repr(C)] pub struct cpuid10_edx_split { pub num_counters_fixed: u32, pub bit_width_fixed: u32, pub reserved1: u32, pub anythread_deprecated: u32, pub reserved2: u32 }
#[repr(C)] pub union cpuid35_eax { pub split: cpuid35_eax_split, pub full: u32 }
#[repr(C)] pub struct cpuid35_eax_split { pub leaf0:u32, pub cntr_subleaf:u32, pub acr_subleaf:u32, pub events_subleaf:u32, pub pebs_caps_subleaf:u32, pub pebs_cnts_subleaf:u32, pub reserved:u32 }
#[repr(C)] pub union cpuid35_ebx { pub split: cpuid35_ebx_split, pub full:u32 }
#[repr(C)] pub struct cpuid35_ebx_split { pub umask2:u32, pub eq:u32, pub rdpmc_user_disable:u32, pub reserved:u32 }
#[repr(C)] pub union cpuid28_eax { pub split:cpuid28_eax_split, pub full:u32 }
#[repr(C)] pub struct cpuid28_eax_split { pub lbr_depth_mask:u32, pub reserved:u32, pub lbr_deep_c_reset:u32, pub lbr_lip:u32 }
#[repr(C)] pub union cpuid28_ebx { pub split:cpuid28_ebx_split, pub full:u32 }
#[repr(C)] pub struct cpuid28_ebx_split { pub lbr_cpl:u32, pub lbr_filter:u32, pub lbr_call_stack:u32 }
#[repr(C)] pub union cpuid28_ecx { pub split:cpuid28_ecx_split, pub full:u32 }
#[repr(C)] pub struct cpuid28_ecx_split { pub lbr_mispred:u32, pub lbr_timed_lbr:u32, pub lbr_br_type:u32, pub reserved:u32, pub lbr_counters:u32 }
#[repr(C)] pub union cpuid_0x80000022_ebx { pub split:cpuid_0x80000022_ebx_split, pub full:u32 }
#[repr(C)] pub struct cpuid_0x80000022_ebx_split { pub num_core_pmc:u32, pub lbr_v2_stack_sz:u32, pub num_df_pmc:u32, pub num_umc_pmc:u32 }

#[repr(C)] pub struct x86_pmu_capability { pub version:i32, pub num_counters_gp:i32, pub num_counters_fixed:i32, pub bit_width_gp:i32, pub bit_width_fixed:i32, pub events_mask:u32, pub events_mask_len:i32, pub pebs_ept:u32, pub mediated:u32 }
pub const INTEL_PMC_FIXED_RDPMC_BASE:u32=1<<30; pub const INTEL_PMC_FIXED_RDPMC_METRICS:u32=1<<29; pub const MSR_ARCH_PERFMON_FIXED_CTR_CTRL:u32=0x38d;
pub const MSR_ARCH_PERFMON_FIXED_CTR0:u32=0x309; pub const INTEL_PMC_IDX_FIXED_INSTRUCTIONS:u32=INTEL_PMC_IDX_FIXED; pub const MSR_ARCH_PERFMON_FIXED_CTR1:u32=0x30a; pub const INTEL_PMC_IDX_FIXED_CPU_CYCLES:u32=INTEL_PMC_IDX_FIXED+1; pub const MSR_ARCH_PERFMON_FIXED_CTR2:u32=0x30b; pub const INTEL_PMC_IDX_FIXED_REF_CYCLES:u32=INTEL_PMC_IDX_FIXED+2; pub const INTEL_PMC_MSK_FIXED_REF_CYCLES:u64=1<<INTEL_PMC_IDX_FIXED_REF_CYCLES; pub const MSR_ARCH_PERFMON_FIXED_CTR3:u32=0x30c; pub const INTEL_PMC_IDX_FIXED_SLOTS:u32=INTEL_PMC_IDX_FIXED+3; pub const INTEL_PMC_MSK_FIXED_SLOTS:u64=1<<INTEL_PMC_IDX_FIXED_SLOTS;
#[inline] pub const fn use_fixed_pseudo_encoding(code:u64)->bool {(code&0xff)==0}
pub const INTEL_PMC_IDX_FIXED_BTS:u32=INTEL_PMC_IDX_FIXED+15; pub const INTEL_PMC_IDX_METRIC_BASE:u32=INTEL_PMC_IDX_FIXED+16; pub const INTEL_PMC_IDX_TD_RETIRING:u32=INTEL_PMC_IDX_METRIC_BASE; pub const INTEL_PMC_IDX_TD_BAD_SPEC:u32=INTEL_PMC_IDX_METRIC_BASE+1; pub const INTEL_PMC_IDX_TD_FE_BOUND:u32=INTEL_PMC_IDX_METRIC_BASE+2; pub const INTEL_PMC_IDX_TD_BE_BOUND:u32=INTEL_PMC_IDX_METRIC_BASE+3; pub const INTEL_PMC_IDX_TD_HEAVY_OPS:u32=INTEL_PMC_IDX_METRIC_BASE+4; pub const INTEL_PMC_IDX_TD_BR_MISPREDICT:u32=INTEL_PMC_IDX_METRIC_BASE+5; pub const INTEL_PMC_IDX_TD_FETCH_LAT:u32=INTEL_PMC_IDX_METRIC_BASE+6; pub const INTEL_PMC_IDX_TD_MEM_BOUND:u32=INTEL_PMC_IDX_METRIC_BASE+7; pub const INTEL_PMC_IDX_METRIC_END:u32=INTEL_PMC_IDX_TD_MEM_BOUND; pub const INTEL_PMC_MSK_TOPDOWN:u64=(0xff<<INTEL_PMC_IDX_METRIC_BASE)|INTEL_PMC_MSK_FIXED_SLOTS;
pub const INTEL_TD_SLOTS:u32=0x400; pub const INTEL_TD_METRIC_RETIRING:u32=0x8000; pub const INTEL_TD_METRIC_BAD_SPEC:u32=0x8100; pub const INTEL_TD_METRIC_FE_BOUND:u32=0x8200; pub const INTEL_TD_METRIC_BE_BOUND:u32=0x8300; pub const INTEL_TD_METRIC_HEAVY_OPS:u32=0x8400; pub const INTEL_TD_METRIC_BR_MISPREDICT:u32=0x8500; pub const INTEL_TD_METRIC_FETCH_LAT:u32=0x8600; pub const INTEL_TD_METRIC_MEM_BOUND:u32=0x8700; pub const INTEL_TD_METRIC_MAX:u32=INTEL_TD_METRIC_MEM_BOUND; pub const INTEL_TD_METRIC_NUM:u32=8; pub const INTEL_TD_CFG_METRIC_CLEAR_BIT:u32=0; pub const INTEL_TD_CFG_METRIC_CLEAR:u64=1;
#[inline] pub fn is_metric_idx(idx:i32)->bool {(idx as u32).wrapping_sub(INTEL_PMC_IDX_METRIC_BASE)<INTEL_TD_METRIC_NUM}
#[inline] pub fn is_topdown_idx(idx:i32)->bool {is_metric_idx(idx)||idx==INTEL_PMC_IDX_FIXED_SLOTS as i32}
pub const GLOBAL_STATUS_COND_CHG:u64=1<<63; pub const GLOBAL_STATUS_BUFFER_OVF_BIT:u32=62; pub const GLOBAL_STATUS_BUFFER_OVF:u64=1<<62; pub const GLOBAL_STATUS_UNC_OVF:u64=1<<61; pub const GLOBAL_STATUS_ASIF:u64=1<<60; pub const GLOBAL_STATUS_COUNTERS_FROZEN:u64=1<<59; pub const GLOBAL_STATUS_LBRS_FROZEN_BIT:u32=58; pub const GLOBAL_STATUS_LBRS_FROZEN:u64=1<<58; pub const GLOBAL_STATUS_TRACE_TOPAPMI_BIT:u32=55; pub const GLOBAL_STATUS_TRACE_TOPAPMI:u64=1<<55; pub const GLOBAL_STATUS_ARCH_PEBS_THRESHOLD_BIT:u32=54; pub const GLOBAL_STATUS_ARCH_PEBS_THRESHOLD:u64=1<<54; pub const GLOBAL_STATUS_PERF_METRICS_OVF_BIT:u32=48; pub const GLOBAL_CTRL_EN_PERF_METRICS:u64=1<<48; pub const INTEL_PMC_IDX_FIXED_VLBR:u32=58; pub const INTEL_FIXED_VLBR_EVENT:u32=0x1b00;

#[repr(C)] pub struct pebs_basic { pub format_group:u32, pub retire_latency:u16, pub format_size:u16, pub ip:u64, pub applicable_counters:u64, pub tsc:u64 }
#[repr(C)] pub struct pebs_meminfo { pub address:u64, pub aux:u64, pub mem_latency:u64, pub tsx_tuning:u64 }
#[repr(C)] pub struct pebs_gprs { pub flags:u64,pub ip:u64,pub ax:u64,pub cx:u64,pub dx:u64,pub bx:u64,pub sp:u64,pub bp:u64,pub si:u64,pub di:u64,pub r8:u64,pub r9:u64,pub r10:u64,pub r11:u64,pub r12:u64,pub r13:u64,pub r14:u64,pub r15:u64 }
#[repr(C)] pub struct pebs_xmm { pub xmm:[u64;32] }
#[repr(C)] pub struct pebs_cntr_header { pub cntr:u32,pub fixed:u32,pub metrics:u32,pub reserved:u32 }
pub const INTEL_CNTR_METRICS:u32=3;
#[repr(C)] pub struct arch_pebs_index { pub whole:u64 }
#[repr(C)] pub struct arch_pebs_header { pub format:u64,pub rsvd6:u64 }
#[repr(C)] pub struct arch_pebs_basic { pub ip:u64,pub applicable_counters:u64,pub tsc:u64,pub retire:u16,pub valid:u8,pub rsvd:u64,pub rsvd2:u64,pub rsvd3:u64 }
#[repr(C)] pub struct arch_pebs_aux { pub address:u64,pub rsvd:u64,pub rsvd2:u64,pub rsvd3:u64,pub rsvd4:u64,pub aux:u64,pub instr_latency:u16,pub pad2:u16,pub cache_latency:u16,pub pad3:u16,pub tsx_tuning:u64 }
#[repr(C)] pub struct arch_pebs_gprs { pub flags:u64,pub ip:u64,pub ax:u64,pub cx:u64,pub dx:u64,pub bx:u64,pub sp:u64,pub bp:u64,pub si:u64,pub di:u64,pub r8:u64,pub r9:u64,pub r10:u64,pub r11:u64,pub r12:u64,pub r13:u64,pub r14:u64,pub r15:u64,pub ssp:u64,pub rsvd:u64 }
#[repr(C)] pub struct arch_pebs_xer_header { pub xstate:u64,pub rsvd:u64 }
pub const ARCH_PEBS_LBR_NAN:u32=0; pub const ARCH_PEBS_LBR_NUM_8:u32=1; pub const ARCH_PEBS_LBR_NUM_16:u32=2; pub const ARCH_PEBS_LBR_NUM_VAR:u32=3; pub const ARCH_PEBS_BASE_LBR_ENTRIES:u32=8;
#[repr(C)] pub struct arch_pebs_lbr_header { pub rsvd:u64,pub ctl:u64,pub depth:u64,pub ler_from:u64,pub ler_to:u64,pub ler_info:u64 }
#[repr(C)] pub struct arch_pebs_cntr_header { pub cntr:u32,pub fixed:u32,pub metrics:u32,pub reserved:u32 }

pub const EXT_PERFMON_DEBUG_FEATURES:u32=0x80000022; pub const IBS_CPUID_FEATURES:u32=0x8000001b;
pub const IBS_CAPS_AVAIL:u32=1; pub const IBS_CAPS_FETCHSAM:u32=1<<1; pub const IBS_CAPS_OPSAM:u32=1<<2; pub const IBS_CAPS_RDWROPCNT:u32=1<<3; pub const IBS_CAPS_OPCNT:u32=1<<4; pub const IBS_CAPS_BRNTRGT:u32=1<<5; pub const IBS_CAPS_OPCNTEXT:u32=1<<6; pub const IBS_CAPS_RIPINVALIDCHK:u32=1<<7; pub const IBS_CAPS_OPBRNFUSE:u32=1<<8; pub const IBS_CAPS_FETCHCTLEXTD:u32=1<<9; pub const IBS_CAPS_OPDATA4:u32=1<<10; pub const IBS_CAPS_ZEN4:u32=1<<11; pub const IBS_CAPS_OPLDLAT:u32=1<<12; pub const IBS_CAPS_DIS:u32=1<<13; pub const IBS_CAPS_FETCHLAT:u32=1<<14; pub const IBS_CAPS_BIT63_FILTER:u32=1<<15; pub const IBS_CAPS_STRMST_RMTSOCKET:u32=1<<16; pub const IBS_CAPS_OPDTLBPGSIZE:u32=1<<19; pub const IBS_CAPS_DEFAULT:u32=IBS_CAPS_AVAIL|IBS_CAPS_FETCHSAM|IBS_CAPS_OPSAM;
pub const IBSCTL:u32=0x1cc; pub const IBSCTL_LVT_OFFSET_VALID:u64=1<<8; pub const IBSCTL_LVT_OFFSET_MASK:u32=0xf; pub const IBS_FETCH_L3MISSONLY:u64=1<<59; pub const IBS_FETCH_RAND_EN:u64=1<<57; pub const IBS_FETCH_VAL:u64=1<<49; pub const IBS_FETCH_ENABLE:u64=1<<48; pub const IBS_FETCH_CNT:u64=0xffff0000; pub const IBS_FETCH_MAX_CNT:u64=0xffff; pub const IBS_FETCH_2_DIS:u64=1; pub const IBS_FETCH_2_FETCHLAT_FILTER:u64=0xf<<1; pub const IBS_FETCH_2_FETCHLAT_FILTER_SHIFT:u32=1; pub const IBS_FETCH_2_EXCL_RIP_63_EQ_1:u64=1<<5; pub const IBS_FETCH_2_EXCL_RIP_63_EQ_0:u64=1<<6;
pub const IBS_OP_LDLAT_EN:u64=1<<63; pub const IBS_OP_LDLAT_THRSH:u64=0xf<<59; pub const IBS_OP_LDLAT_THRSH_SHIFT:u32=59; pub const IBS_OP_CUR_CNT:u64=0xfff80<<32; pub const IBS_OP_CUR_CNT_RAND:u64=0x7f<<32; pub const IBS_OP_CUR_CNT_EXT_MASK:u64=0x7f<<52; pub const IBS_OP_CNT_CTL:u64=1<<19; pub const IBS_OP_VAL:u64=1<<18; pub const IBS_OP_ENABLE:u64=1<<17; pub const IBS_OP_L3MISSONLY:u64=1<<16; pub const IBS_OP_MAX_CNT:u64=0xffff; pub const IBS_OP_MAX_CNT_EXT:u64=0x7fffff; pub const IBS_OP_MAX_CNT_EXT_MASK:u64=0x7f<<20; pub const IBS_RIP_INVALID:u64=1<<38; pub const IBS_OP_2_DIS:u64=1; pub const IBS_OP_2_EXCL_RIP_63_EQ_0:u64=1<<1; pub const IBS_OP_2_EXCL_RIP_63_EQ_1:u64=1<<2; pub const IBS_OP_2_STRM_ST_FILTER:u64=1<<3; pub const IBS_OP_2_STRM_ST_FILTER_SHIFT:u32=3;

/* Configuration-dependent declarations remain external; the preprocessor conditions are build-time concerns. */
extern "C" { pub fn get_ibs_caps() -> u32; pub fn forward_event_to_ibs(event:*mut core::ffi::c_void)->i32; pub fn perf_events_lapic_init(); pub fn perf_get_x86_pmu_capability(cap:*mut x86_pmu_capability); pub fn perf_get_hw_event_config(hw_event:i32)->u64; pub fn perf_check_microcode(); pub fn perf_clear_dirty_counters(); pub fn x86_perf_rdpmc_index(event:*mut core::ffi::c_void)->i32; }
pub const PERF_EFLAGS_EXACT: u64=1<<3; pub const PERF_EFLAGS_VM:u64=1<<5;
#[repr(C)] pub struct x86_perf_regs { pub regs: [u8;0], pub xmm_regs:*mut u64 }
#[repr(C)] pub struct perf_guest_switch_msr { pub msr:u32,pub host:u64,pub guest:u64 }
#[repr(C)] pub struct x86_pmu_lbr { pub nr:u32,pub from:u32,pub to:u32,pub info:u32,pub has_callstack:bool }
extern "C" { pub fn perf_guest_get_msrs(nr:*mut i32,data:*mut core::ffi::c_void)->*mut perf_guest_switch_msr; pub fn x86_perf_get_lbr(lbr:*mut x86_pmu_lbr); pub fn intel_pt_handle_vmx(on:i32); pub fn amd_pmu_enable_virt(); pub fn amd_pmu_disable_virt(); }
pub const PERF_NEEDS_LOPWR_CB:u32=1;
extern "C" { pub fn perf_amd_brs_lopwr_cb(lopwr_in:bool); }
pub const arch_perf_out_copy_user: &str = "copy_from_user_nmi";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
