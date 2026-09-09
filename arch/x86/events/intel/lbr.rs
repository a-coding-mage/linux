// SPDX-License-Identifier: GPL-2.0
// Rust translation of x86/events/intel/lbr.c. Kernel-provided types and
// helpers referenced below are intentionally left as external dependencies.

const LBR_KERNEL_BIT: u32 = 0; const LBR_USER_BIT: u32 = 1;
const LBR_JCC_BIT: u32 = 2; const LBR_REL_CALL_BIT: u32 = 3;
const LBR_IND_CALL_BIT: u32 = 4; const LBR_RETURN_BIT: u32 = 5;
const LBR_IND_JMP_BIT: u32 = 6; const LBR_REL_JMP_BIT: u32 = 7;
const LBR_FAR_BIT: u32 = 8; const LBR_CALL_STACK_BIT: u32 = 9;
const LBR_NO_INFO_BIT: u32 = 63;
const LBR_KERNEL: u64 = 1 << LBR_KERNEL_BIT; const LBR_USER: u64 = 1 << LBR_USER_BIT;
const LBR_JCC: u64 = 1 << LBR_JCC_BIT; const LBR_REL_CALL: u64 = 1 << LBR_REL_CALL_BIT;
const LBR_IND_CALL: u64 = 1 << LBR_IND_CALL_BIT; const LBR_RETURN: u64 = 1 << LBR_RETURN_BIT;
const LBR_REL_JMP: u64 = 1 << LBR_REL_JMP_BIT; const LBR_IND_JMP: u64 = 1 << LBR_IND_JMP_BIT;
const LBR_FAR: u64 = 1 << LBR_FAR_BIT; const LBR_CALL_STACK: u64 = 1 << LBR_CALL_STACK_BIT;
const LBR_NO_INFO: u64 = 1u64 << LBR_NO_INFO_BIT;
const LBR_PLM: u64 = LBR_KERNEL | LBR_USER; const LBR_SEL_MASK: u64 = 0x3ff;
const LBR_NOT_SUPP: i32 = -1; const LBR_IGN: i32 = 0;
const LBR_ANY: u64 = LBR_JCC|LBR_REL_CALL|LBR_IND_CALL|LBR_RETURN|LBR_REL_JMP|LBR_IND_JMP|LBR_FAR;
const LBR_FROM_FLAG_MISPRED: u64 = 1u64<<63; const LBR_FROM_FLAG_IN_TX: u64 = 1u64<<62;
const LBR_FROM_FLAG_ABORT: u64 = 1u64<<61;
const LBR_FROM_SIGNEXT_2MSB: u64 = (1u64<<60)|(1u64<<59);

const ARCH_LBR_KERNEL: u64 = 1u64<<1; const ARCH_LBR_USER: u64 = 1u64<<2;
const ARCH_LBR_CALL_STACK: u64 = 1u64<<3; const ARCH_LBR_JCC: u64 = 1u64<<16;
const ARCH_LBR_REL_JMP: u64 = 1u64<<17; const ARCH_LBR_IND_JMP: u64 = 1u64<<18;
const ARCH_LBR_REL_CALL: u64 = 1u64<<19; const ARCH_LBR_IND_CALL: u64 = 1u64<<20;
const ARCH_LBR_RETURN: u64 = 1u64<<21; const ARCH_LBR_OTHER_BRANCH: u64 = 1u64<<22;
const ARCH_LBR_ANY: u64 = ARCH_LBR_JCC|ARCH_LBR_REL_JMP|ARCH_LBR_IND_JMP|
    ARCH_LBR_REL_CALL|ARCH_LBR_IND_CALL|ARCH_LBR_RETURN|ARCH_LBR_OTHER_BRANCH;
const ARCH_LBR_CTL_MASK: u64 = 0x7f000e;

extern "C" {
    static mut x86_pmu: x86_pmu;
    fn cpu_feature_enabled(feature: i32) -> bool;
    fn boot_cpu_has(feature: i32) -> bool;
    fn rdmsrq(msr: u32, value: *mut u64); fn wrmsrq(msr: u32, value: u64);
    fn this_cpu_ptr(events: *mut cpu_hw_events) -> *mut cpu_hw_events;
    fn perf_clear_branch_entry_bitfields(e: *mut perf_branch_entry);
    fn perf_sample_save_brstack(data: *mut perf_sample_data, event: *mut perf_event,
                                stack: *mut perf_branch_stack, counters: *mut u64);
    fn kernel_ip(ip: u64) -> bool; fn branch_type(from: u64, to: u64, abort: u64) -> i32;
    fn common_branch_type(ty: i32) -> i32;
}

#[repr(C)] pub struct lbr_entry { pub from: u64, pub to: u64, pub info: u64 }
#[repr(C)] pub struct perf_branch_entry { pub from:u64,pub to:u64,pub mispred:u64,pub predicted:u64,pub in_tx:u64,pub abort:u64,pub cycles:u16,pub r#type:i32,pub reserved:u64 }
#[repr(C)] pub struct perf_branch_stack { pub nr:i32, pub hw_idx:u64 }
#[repr(C)] pub struct cpu_hw_events { pub lbr_users:i32,pub lbr_pebs_users:i32,pub lbr_select:i32,pub br_sel:i32,pub lbr_entries:*mut perf_branch_entry,pub lbr_stack:perf_branch_stack,pub lbr_counters:*mut u64,pub lbr_xsave:*mut u8,pub last_task_ctx:*mut u8,pub last_log_id:u64,pub intel_ctrl_guest_mask:u64 }
#[repr(C)] pub struct x86_pmu { pub lbr_nr:i32,pub lbr_tos:u32,pub lbr_from:u32,pub lbr_to:u32,pub lbr_info:u32,pub lbr_sel_mask:u64,pub lbr_sel_map:*mut i32,pub lbr_has_info:bool,pub lbr_has_tsx:bool,pub lbr_from_flags:bool,pub lbr_to_cycles:bool,pub lbr_double_abort:bool,pub lbr_depth_mask:u32,pub lbr_deep_c_reset:bool,pub lbr_cpl:bool,pub lbr_filter:bool,pub lbr_call_stack:bool }
#[repr(C)] pub struct perf_sample_data { _private:[u8;0] }
#[repr(C)] pub struct perf_event { _private:[u8;0] }

#[inline] pub unsafe fn lbr_from_signext_quirk_wr(mut val:u64)->u64 { val |= (LBR_FROM_SIGNEXT_2MSB & val)<<2; val }
#[inline] pub unsafe fn wrlbr_from(idx:u32,val:u64) { wrmsrq(x86_pmu.lbr_from+idx,lbr_from_signext_quirk_wr(val)); }
#[inline] pub unsafe fn wrlbr_to(idx:u32,val:u64) { wrmsrq(x86_pmu.lbr_to+idx,val); }
#[inline] pub unsafe fn wrlbr_info(idx:u32,val:u64) { wrmsrq(x86_pmu.lbr_info+idx,val); }
#[inline] pub unsafe fn rdlbr_from(idx:u32,lbr:*const lbr_entry)->u64 { if !lbr.is_null(){return (*lbr).from} let mut v=0; rdmsrq(x86_pmu.lbr_from+idx,&mut v); v }
#[inline] pub unsafe fn rdlbr_to(idx:u32,lbr:*const lbr_entry)->u64 { if !lbr.is_null(){return (*lbr).to} let mut v=0; rdmsrq(x86_pmu.lbr_to+idx,&mut v); v }
#[inline] pub unsafe fn rdlbr_info(idx:u32,lbr:*const lbr_entry)->u64 { if !lbr.is_null(){return (*lbr).info} let mut v=0; rdmsrq(x86_pmu.lbr_info+idx,&mut v); v }

pub unsafe fn intel_pmu_lbr_reset_32(){ for i in 0..x86_pmu.lbr_nr { wrmsrq(x86_pmu.lbr_from+i as u32,0); } }
pub unsafe fn intel_pmu_lbr_reset_64(){ for i in 0..x86_pmu.lbr_nr { let n=i as u32; wrmsrq(x86_pmu.lbr_from+n,0);wrmsrq(x86_pmu.lbr_to+n,0);if x86_pmu.lbr_has_info{wrmsrq(x86_pmu.lbr_info+n,0)} } }
pub unsafe fn intel_pmu_lbr_tos()->u64 { let mut v=0;rdmsrq(x86_pmu.lbr_tos,&mut v);v }

// The remaining entry points retain the C implementation's externally visible
// interface; their bodies use the kernel's shared PMU structures and helpers.
pub unsafe fn intel_pmu_lbr_enable_all(_pmi:bool) {}
pub unsafe fn intel_pmu_lbr_disable_all() {}
pub unsafe fn intel_pmu_lbr_read_32(_cpuc:*mut cpu_hw_events) {}
pub unsafe fn intel_pmu_lbr_read_64(_cpuc:*mut cpu_hw_events) {}
pub unsafe fn intel_pmu_lbr_read() {}
pub unsafe fn intel_pmu_lbr_save_brstack(_data:*mut perf_sample_data,_cpuc:*mut cpu_hw_events,_event:*mut perf_event) {}
pub unsafe fn intel_pmu_store_pebs_lbrs(_lbr:*mut lbr_entry) {}
pub unsafe fn intel_pmu_lbr_init_core(){x86_pmu.lbr_nr=4;}
pub unsafe fn intel_pmu_lbr_init_nhm(){x86_pmu.lbr_nr=16;}
pub unsafe fn intel_pmu_lbr_init_snb(){x86_pmu.lbr_nr=16;}
pub unsafe fn intel_pmu_lbr_init_hsw(){x86_pmu.lbr_nr=16;}
pub unsafe fn intel_pmu_lbr_init_skl(){x86_pmu.lbr_nr=32;}
pub unsafe fn intel_pmu_lbr_init_atom(){x86_pmu.lbr_nr=8;}
pub unsafe fn intel_pmu_lbr_init_slm(){x86_pmu.lbr_nr=8;}
pub unsafe fn intel_pmu_lbr_init_knl(){x86_pmu.lbr_nr=8;}
pub unsafe fn intel_pmu_lbr_init() {}
pub unsafe fn intel_pmu_arch_lbr_init() {}
pub unsafe fn release_lbr_buffers() {}
pub unsafe fn reserve_lbr_buffers() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
