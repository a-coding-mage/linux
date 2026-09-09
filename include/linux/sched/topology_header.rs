/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by other translated headers are intentionally external. */

/* SD_FLAG(name, mflags) generates the flag indexes and flag bits from sd_flags.h. */
#[repr(C)]
pub struct SdFlagDebug {
    pub meta_flags: u32,
    pub name: *mut core::ffi::c_char,
}

#[cfg(CONFIG_SCHED_SMT)]
extern "C" {
    pub fn cpu_smt_flags() -> core::ffi::c_int;
    pub fn tl_smt_mask(tl: *mut SchedDomainTopologyLevel, cpu: core::ffi::c_int) -> *const Cpumask;
}

#[cfg(CONFIG_SCHED_CLUSTER)]
extern "C" {
    pub fn cpu_cluster_flags() -> core::ffi::c_int;
    pub fn tl_cls_mask(tl: *mut SchedDomainTopologyLevel, cpu: core::ffi::c_int) -> *const Cpumask;
}

#[cfg(CONFIG_SCHED_MC)]
extern "C" {
    pub fn cpu_core_flags() -> core::ffi::c_int;
    pub fn tl_mc_mask(tl: *mut SchedDomainTopologyLevel, cpu: core::ffi::c_int) -> *const Cpumask;
}

extern "C" {
    pub static sd_flag_debug: SdFlagDebug;
    pub fn tl_pkg_mask(tl: *mut SchedDomainTopologyLevel, cpu: core::ffi::c_int) -> *const Cpumask;
    pub fn arch_asym_cpu_priority(cpu: core::ffi::c_int) -> core::ffi::c_int;
}

#[repr(C)]
pub struct SchedDomainAttr {
    pub relax_domain_level: core::ffi::c_int,
}

pub const SD_ATTR_INIT: SchedDomainAttr = SchedDomainAttr { relax_domain_level: -1 };

extern "C" {
    pub static mut sched_domain_level_max: core::ffi::c_int;
}

pub struct SchedGroup;

#[repr(C)]
pub union SchedDomainSharedIdle {
    pub nr_idle_scan: core::ffi::c_int,
    pub alloc_flags: core::ffi::c_int,
}

#[repr(C)]
pub struct SchedDomainShared {
    pub ref_: AtomicT,
    pub nr_busy_cpus: AtomicT,
    pub has_idle_cores: core::ffi::c_int,
    pub idle: SchedDomainSharedIdle,
    #[cfg(CONFIG_SCHED_CACHE)]
    pub util_avg: core::ffi::c_ulong,
    #[cfg(CONFIG_SCHED_CACHE)]
    pub capacity: core::ffi::c_ulong,
}

#[repr(C)]
pub struct SchedDomain {
    pub parent: *mut SchedDomain,
    pub child: *mut SchedDomain,
    pub groups: *mut SchedGroup,
    pub min_interval: core::ffi::c_ulong,
    pub max_interval: core::ffi::c_ulong,
    pub busy_factor: u32,
    pub imbalance_pct: u32,
    pub cache_nice_tries: u32,
    pub imb_numa_nr: u32,
    pub nohz_idle: core::ffi::c_int,
    pub flags: core::ffi::c_int,
    pub level: core::ffi::c_int,
    pub last_balance: core::ffi::c_ulong,
    pub balance_interval: u32,
    pub nr_balance_failed: u32,
    pub newidle_call: u32,
    pub newidle_success: u32,
    pub newidle_ratio: u32,
    pub newidle_stamp: u64,
    pub max_newidle_lb_cost: u64,
    pub last_decay_max_lb_cost: core::ffi::c_ulong,
    #[cfg(CONFIG_SCHED_CACHE)]
    pub llc_max: u32,
    #[cfg(CONFIG_SCHED_CACHE)]
    pub llc_counts: *mut u32,
    #[cfg(CONFIG_SCHED_CACHE)]
    pub llc_bytes: core::ffi::c_ulong,
    #[cfg(CONFIG_SCHEDSTATS)]
    pub lb_count: [u32; CPU_MAX_IDLE_TYPES],
    #[cfg(CONFIG_SCHEDSTATS)]
    pub lb_failed: [u32; CPU_MAX_IDLE_TYPES],
    #[cfg(CONFIG_SCHEDSTATS)]
    pub lb_balanced: [u32; CPU_MAX_IDLE_TYPES],
    #[cfg(CONFIG_SCHEDSTATS)]
    pub lb_imbalance_load: [u32; CPU_MAX_IDLE_TYPES],
    #[cfg(CONFIG_SCHEDSTATS)]
    pub lb_imbalance_util: [u32; CPU_MAX_IDLE_TYPES],
    #[cfg(CONFIG_SCHEDSTATS)]
    pub lb_imbalance_task: [u32; CPU_MAX_IDLE_TYPES],
    #[cfg(CONFIG_SCHEDSTATS)]
    pub lb_imbalance_misfit: [u32; CPU_MAX_IDLE_TYPES],
    #[cfg(CONFIG_SCHEDSTATS)]
    pub lb_gained: [u32; CPU_MAX_IDLE_TYPES],
    #[cfg(CONFIG_SCHEDSTATS)]
    pub lb_hot_gained: [u32; CPU_MAX_IDLE_TYPES],
    #[cfg(CONFIG_SCHEDSTATS)]
    pub lb_nobusyg: [u32; CPU_MAX_IDLE_TYPES],
    #[cfg(CONFIG_SCHEDSTATS)]
    pub lb_nobusyq: [u32; CPU_MAX_IDLE_TYPES],
    #[cfg(CONFIG_SCHEDSTATS)]
    pub alb_count: u32,
    #[cfg(CONFIG_SCHEDSTATS)]
    pub alb_failed: u32,
    #[cfg(CONFIG_SCHEDSTATS)]
    pub alb_pushed: u32,
    #[cfg(CONFIG_SCHEDSTATS)]
    pub sbe_count: u32,
    #[cfg(CONFIG_SCHEDSTATS)]
    pub sbe_balanced: u32,
    #[cfg(CONFIG_SCHEDSTATS)]
    pub sbe_pushed: u32,
    #[cfg(CONFIG_SCHEDSTATS)]
    pub sbf_count: u32,
    #[cfg(CONFIG_SCHEDSTATS)]
    pub sbf_balanced: u32,
    #[cfg(CONFIG_SCHEDSTATS)]
    pub sbf_pushed: u32,
    #[cfg(CONFIG_SCHEDSTATS)]
    pub ttwu_wake_remote: u32,
    #[cfg(CONFIG_SCHEDSTATS)]
    pub ttwu_move_affine: u32,
    #[cfg(CONFIG_SCHEDSTATS)]
    pub ttwu_move_balance: u32,
    pub name: *mut core::ffi::c_char,
    pub private_or_rcu: SchedDomainPrivate,
    pub shared: *mut SchedDomainShared,
    pub span_weight: u32,
}

#[repr(C)]
pub union SchedDomainPrivate {
    pub private: *mut core::ffi::c_void,
    pub rcu: RcuHead,
}

#[inline]
pub unsafe fn sched_domain_span(sd: *mut SchedDomain) -> *mut Cpumask {
    let bitmap = (sd as *mut u8).add(core::mem::size_of::<SchedDomain>()) as *mut core::ffi::c_ulong;
    to_cpumask(bitmap)
}

extern "C" {
    pub fn partition_sched_domains(ndoms_new: core::ffi::c_int, doms_new: *mut CpumaskVar, dattr_new: *mut SchedDomainAttr);
    pub fn alloc_sched_domains(ndoms: u32) -> *mut CpumaskVar;
    pub fn free_sched_domains(doms: *mut CpumaskVar, ndoms: u32);
    pub fn cpus_equal_capacity(this_cpu: core::ffi::c_int, that_cpu: core::ffi::c_int) -> bool;
    pub fn cpus_share_cache(this_cpu: core::ffi::c_int, that_cpu: core::ffi::c_int) -> bool;
    pub fn cpus_share_resources(this_cpu: core::ffi::c_int, that_cpu: core::ffi::c_int) -> bool;
}

pub type SchedDomainMaskF = unsafe extern "C" fn(*mut SchedDomainTopologyLevel, core::ffi::c_int) -> *const Cpumask;
pub type SchedDomainFlagsF = unsafe extern "C" fn() -> core::ffi::c_int;

#[repr(C)]
pub struct SdData {
    pub sd: *mut *mut SchedDomain,
    pub sg: *mut *mut SchedGroup,
    pub sgc: *mut *mut SchedGroupCapacity,
}

#[repr(C)]
pub struct SchedDomainTopologyLevel {
    pub mask: Option<SchedDomainMaskF>,
    pub sd_flags: Option<SchedDomainFlagsF>,
    pub numa_level: core::ffi::c_int,
    pub data: SdData,
    pub name: *mut core::ffi::c_char,
}

extern "C" {
    pub fn set_sched_topology(tl: *mut SchedDomainTopologyLevel);
    pub fn sched_update_asym_prefer_cpu(cpu: core::ffi::c_int, old_prio: core::ffi::c_int, new_prio: core::ffi::c_int);
}

/* SDTL_INIT(maskfn, flagsfn, dname) initializes mask, flags, and a stringified name. */

#[cfg(all(CONFIG_ENERGY_MODEL, CONFIG_CPU_FREQ_GOV_SCHEDUTIL))]
extern "C" { pub fn rebuild_sched_domains_energy(); }

#[cfg(not(all(CONFIG_ENERGY_MODEL, CONFIG_CPU_FREQ_GOV_SCHEDUTIL)))]
#[inline]
pub unsafe fn rebuild_sched_domains_energy() {}

#[inline]
pub unsafe fn arch_scale_cpu_capacity(_cpu: core::ffi::c_int) -> core::ffi::c_ulong {
    SCHED_CAPACITY_SCALE
}

#[inline]
pub unsafe fn arch_scale_hw_pressure(_cpu: core::ffi::c_int) -> core::ffi::c_ulong { 0 }

#[inline]
pub unsafe fn arch_update_hw_pressure(_cpus: *const Cpumask, _capped_frequency: core::ffi::c_ulong) {}

#[inline]
pub unsafe fn arch_scale_freq_ref(_cpu: core::ffi::c_int) -> u32 { 0 }

#[inline]
pub unsafe fn task_node(p: *const TaskStruct) -> core::ffi::c_int {
    cpu_to_node(task_cpu(p))
}

#[cfg(CONFIG_SCHED_CACHE)]
extern "C" { pub fn sched_update_llc_bytes(cpu: u32); }

#[cfg(not(CONFIG_SCHED_CACHE))]
#[inline]
pub unsafe fn sched_update_llc_bytes(_cpu: u32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
