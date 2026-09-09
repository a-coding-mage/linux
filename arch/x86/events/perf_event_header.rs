/* Rust translation of the x86 performance-events header. */

#[repr(C)]
#[derive(Copy, Clone)]
pub union EventConstraintIndexMask { pub idxmsk: [usize; 1], pub idxmsk64: u64 }

#[repr(C)]
pub struct EventConstraint {
    pub idxmsk: EventConstraintIndexMask, pub code: u64, pub cmask: u64,
    pub weight: i32, pub overlap: i32, pub flags: i32, pub size: u32,
}

pub const EXTRA_REG_NONE: i32 = -1;
pub const EXTRA_REG_RSP_0: i32 = 0; pub const EXTRA_REG_RSP_1: i32 = 1;
pub const EXTRA_REG_LBR: i32 = 2; pub const EXTRA_REG_LDLAT: i32 = 3;
pub const EXTRA_REG_FE: i32 = 4; pub const EXTRA_REG_SNOOP_0: i32 = 5;
pub const EXTRA_REG_SNOOP_1: i32 = 6; pub const EXTRA_REG_OMR_0: i32 = 7;
pub const EXTRA_REG_OMR_1: i32 = 8; pub const EXTRA_REG_OMR_2: i32 = 9;
pub const EXTRA_REG_OMR_3: i32 = 10; pub const EXTRA_REG_MAX: usize = 11;

#[inline] pub unsafe fn constraint_match(c: *const EventConstraint, ecode: u64) -> bool {
    ((ecode & (*c).cmask).wrapping_sub((*c).code)) <= (*c).size as u64
}

/* PERF_ARCH-generated flags are supplied by the corresponding kernel header. */
#[inline] pub unsafe fn is_topdown_count(event: *const perf_event) -> bool { (*event).hw.flags & PERF_X86_EVENT_TOPDOWN != 0 }
#[inline] pub unsafe fn is_metric_event(event: *const perf_event) -> bool {
    let config = (*event).attr.config;
    (config & ARCH_PERFMON_EVENTSEL_EVENT) == 0 &&
        (config & INTEL_ARCH_EVENT_MASK) >= INTEL_TD_METRIC_RETIRING &&
        (config & INTEL_ARCH_EVENT_MASK) <= INTEL_TD_METRIC_MAX
}
#[inline] pub unsafe fn is_slots_event(event: *const perf_event) -> bool { ((*event).attr.config & INTEL_ARCH_EVENT_MASK) == INTEL_TD_SLOTS }
#[inline] pub unsafe fn is_topdown_event(event: *const perf_event) -> bool { is_metric_event(event) || is_slots_event(event) }
extern "C" { pub fn is_x86_event(event: *const perf_event) -> bool; }
#[inline] pub unsafe fn check_leader_group(leader: *const perf_event, flags: i32) -> bool { is_x86_event(leader) && ((*leader).hw.flags & flags != 0) }
#[inline] pub unsafe fn is_branch_counters_group(e: *const perf_event) -> bool { check_leader_group((*e).group_leader, PERF_X86_EVENT_BRANCH_COUNTERS) }
#[inline] pub unsafe fn is_pebs_counter_event_group(e: *const perf_event) -> bool { check_leader_group((*e).group_leader, PERF_X86_EVENT_PEBS_CNTR) }
#[inline] pub unsafe fn is_acr_event_group(e: *const perf_event) -> bool { check_leader_group((*e).group_leader, PERF_X86_EVENT_ACR) }
extern "C" { pub fn test_bit(nr: i32, addr: *const usize) -> bool; }
#[inline] pub unsafe fn is_acr_self_reload_event(e: *const perf_event) -> bool {
    let h = &(*e).hw; h.idx >= 0 && is_acr_event_group(e) && test_bit(h.idx, &h.config1 as *const _ as *const usize)
}

#[repr(C)] pub struct AmdNb { pub nb_id: i32, pub refcnt: i32, pub owners: [*mut perf_event; X86_PMC_IDX_MAX], pub event_constraints: [EventConstraint; X86_PMC_IDX_MAX] }
pub const PEBS_COUNTER_MASK: u64 = (1u64 << MAX_PEBS_EVENTS) - 1;
pub const PEBS_PMI_AFTER_EACH_RECORD: u64 = 1u64 << 60;
pub const PEBS_OUTPUT_OFFSET: u32 = 61;
pub const PEBS_OUTPUT_MASK: u64 = 3u64 << PEBS_OUTPUT_OFFSET;
pub const PEBS_OUTPUT_PT: u64 = 1u64 << PEBS_OUTPUT_OFFSET;
pub const PEBS_VIA_PT_MASK: u64 = PEBS_OUTPUT_PT | PEBS_PMI_AFTER_EACH_RECORD;
pub const X86_USER_RDPMC_NEVER_ENABLE: i32 = 0; pub const X86_USER_RDPMC_CONDITIONAL_ENABLE: i32 = 1; pub const X86_USER_RDPMC_ALWAYS_ENABLE: i32 = 2;

#[repr(C)] pub struct ErAccount { pub lock: raw_spinlock_t, pub config: u64, pub reg: u64, pub ref_: atomic_t }
#[repr(C)] pub struct IntelSharedRegs { pub regs: [ErAccount; EXTRA_REG_MAX], pub refcnt: i32, pub core_id: u32 }
pub const INTEL_EXCL_UNUSED: i32 = 0; pub const INTEL_EXCL_SHARED: i32 = 1; pub const INTEL_EXCL_EXCLUSIVE: i32 = 2;
#[repr(C)] pub struct IntelExclStates { pub state: [i32; X86_PMC_IDX_MAX], pub sched_started: bool }
#[repr(C)] pub union IntelExclMask { pub has_exclusive: [u16; 2], pub exclusive_present: u32 }
#[repr(C)] pub struct IntelExclCntrs { pub lock: raw_spinlock_t, pub states: [IntelExclStates; 2], pub mask: IntelExclMask, pub refcnt: i32, pub core_id: u32 }

pub const MAX_LBR_ENTRIES: usize = 32;
pub const LBR_FORMAT_32: i32=0; pub const LBR_FORMAT_LIP: i32=1; pub const LBR_FORMAT_EIP: i32=2; pub const LBR_FORMAT_EIP_FLAGS: i32=3; pub const LBR_FORMAT_EIP_FLAGS2: i32=4; pub const LBR_FORMAT_INFO: i32=5; pub const LBR_FORMAT_TIME: i32=6; pub const LBR_FORMAT_INFO2: i32=7; pub const LBR_FORMAT_MAX_KNOWN: i32=7;
pub const X86_PERF_KFREE_SHARED: usize=0; pub const X86_PERF_KFREE_EXCL: usize=1; pub const X86_PERF_KFREE_MAX: usize=2;

#[repr(C)] pub struct CpuHwEvents {
    pub events: [*mut perf_event; X86_PMC_IDX_MAX], pub active_mask: [usize; 1], pub dirty: [usize; 1], pub enabled: i32,
    pub n_events: i32, pub n_added: i32, pub n_txn: i32, pub n_txn_pair: i32, pub n_txn_metric: i32,
    pub assign: [i32; X86_PMC_IDX_MAX], pub tags: [u64; X86_PMC_IDX_MAX], pub event_list: [*mut perf_event; X86_PMC_IDX_MAX],
    pub event_constraint: [*mut EventConstraint; X86_PMC_IDX_MAX], pub n_excl: i32, pub n_late_setup: i32, pub txn_flags: u32, pub is_fake: i32,
    pub ds: *mut debug_store, pub ds_bts_vaddr: *mut core::ffi::c_void, pub pebs_vaddr: *mut core::ffi::c_void, pub pebs_enabled: u64,
    pub n_pebs: i32, pub n_large_pebs: i32, pub n_pebs_via_pt: i32, pub pebs_output: i32, pub pebs_data_cfg: u64, pub active_pebs_data_cfg: u64, pub pebs_record_size: i32,
    pub fixed_ctrl_val: u64, pub active_fixed_ctrl_val: u64, pub acr_cfg_b: [u64; X86_PMC_IDX_MAX], pub cfg_c_val: [u64; X86_PMC_IDX_MAX],
    pub lbr_users: i32, pub lbr_pebs_users: i32, pub lbr_stack: perf_branch_stack, pub lbr_entries: [perf_branch_entry; MAX_LBR_ENTRIES], pub lbr_counters: [u64; MAX_LBR_ENTRIES],
    pub lbr_sel: *mut ErAccount, pub br_sel: u64, pub last_task_ctx: *mut core::ffi::c_void, pub last_log_id: i32, pub lbr_select: i32, pub lbr_xsave: *mut core::ffi::c_void,
    pub intel_ctrl_guest_mask: u64, pub intel_ctrl_host_mask: u64, pub guest_switch_msrs: [perf_guest_switch_msr; X86_PMC_IDX_MAX], pub intel_cp_status: u64,
    pub shared_regs: *mut IntelSharedRegs, pub constraint_list: *mut EventConstraint, pub excl_cntrs: *mut IntelExclCntrs, pub excl_thread_id: i32, pub tfa_shadow: u64,
    pub n_metric: i32, pub amd_nb: *mut AmdNb, pub brs_active: i32, pub perf_ctr_virt_mask: u64, pub n_pair: i32, pub kfree_on_online: [*mut core::ffi::c_void; X86_PERF_KFREE_MAX], pub pmu: *mut pmu,
}

#[repr(C)] pub struct ExtraReg { pub event: u32, pub msr: u32, pub config_mask: u64, pub valid_mask: u64, pub idx: i32, pub extra_msr_access: bool }
#[repr(C)] pub union PerfCapabilities { pub capabilities: u64, pub bits: u64 }
#[repr(C)] pub struct X86PmuQuirk { pub next: *mut X86PmuQuirk, pub func: Option<unsafe extern "C" fn()> }
#[repr(C)] pub union X86PmuConfig { pub value: u64, pub bits: u64 }
pub const PERF_PEBS_DATA_SOURCE_MAX: usize=0x100; pub const PERF_PEBS_DATA_SOURCE_MASK: u64=0xff; pub const PERF_PEBS_DATA_SOURCE_GRT_MAX: usize=0x10; pub const PERF_PEBS_DATA_SOURCE_GRT_MASK: u64=0xf;
pub const X86_HYBRID_PMU_ATOM_IDX: usize=0; pub const X86_HYBRID_PMU_CORE_IDX: usize=1; pub const X86_HYBRID_PMU_TINY_IDX: usize=2;
pub const NOT_HYBRID: i32=0; pub const HYBRID_SMALL: i32=1; pub const HYBRID_BIG: i32=2; pub const HYBRID_TINY: i32=4; pub const HYBRID_BIG_SMALL: i32=3; pub const HYBRID_SMALL_TINY: i32=5; pub const HYBRID_BIG_SMALL_TINY: i32=7;
#[repr(C)] pub struct ArchPebsCap { pub caps:u64, pub counters:u64, pub pdists:u64 }

/* The remaining declarations retain the kernel ABI and refer to external kernel types. */
extern "C" {
    pub static mut x86_pmu: x86_pmu;
    pub fn x86_get_static_pmu() -> *mut pmu; pub fn x86_get_pmu(cpu: u32) -> *mut pmu;
    pub fn x86_perf_event_set_period(event: *mut perf_event) -> i32;
    pub fn x86_perf_event_update(event: *mut perf_event) -> u64;
    pub fn x86_pmu_enable_all(added: i32); pub fn x86_pmu_stop(event: *mut perf_event, flags: i32);
    pub fn x86_pmu_handle_irq(regs: *mut pt_regs) -> i32; pub fn x86_pmu_disable_all();
    pub fn common_branch_type(ty: i32) -> i32; pub fn branch_type(from: usize,to:usize,abort:i32)->i32;
}

#[inline] pub unsafe fn intel_pmu_topdown_event_update(e: *mut perf_event, _val: *mut u64) -> u64 { x86_perf_event_update(e) }
#[inline] pub unsafe fn x86_pmu_config_addr(index: i32) -> u32 { x86_pmu.eventsel + match x86_pmu.addr_offset { Some(f) => f(index,true) as u32, None => index as u32 } }
#[inline] pub unsafe fn x86_pmu_event_addr(index: i32) -> u32 { x86_pmu.perfctr + match x86_pmu.addr_offset { Some(f) => f(index,false) as u32, None => index as u32 } }
#[inline] pub unsafe fn x86_pmu_fixed_ctr_addr(index: i32) -> u32 { x86_pmu.fixedctr + match x86_pmu.addr_offset { Some(f) => f(index,false) as u32, None => index as u32 } }
#[inline] pub unsafe fn x86_pmu_rdpmc_index(index: i32) -> i32 { match x86_pmu.rdpmc_index { Some(f)=>f(index), None=>index } }

/* Build-time CONFIG_CPU_SUP_AMD/INTEL and related feature guards are preserved by
 * exposing their declarations; unavailable configurations supply their kernel stubs. */
extern "C" {
    pub fn amd_pmu_init() -> i32; pub fn amd_brs_init() -> i32; pub fn intel_pmu_init() -> i32;
    pub fn intel_cpuc_prepare(cpuc:*mut CpuHwEvents,cpu:i32)->i32; pub fn intel_cpuc_finish(cpuc:*mut CpuHwEvents);
    pub fn intel_pmu_pebs_enable(event:*mut perf_event); pub fn intel_pmu_pebs_disable(event:*mut perf_event);
    pub fn intel_pmu_lbr_reset(); pub fn intel_pmu_lbr_add(event:*mut perf_event); pub fn intel_pmu_lbr_del(event:*mut perf_event);
    pub fn zhaoxin_pmu_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
