// SPDX-License-Identifier: GPL-2.0
/* BPF extensible scheduler class: built-in idle CPU tracking policy. */

/* External kernel types, constants, macros and functions are supplied by the
 * surrounding kernel translation unit. */

#[repr(C)]
pub struct ScxIdleCpus { pub cpu: CpumaskVar, pub smt: CpumaskVar }

pub type CpumaskVar = *mut Cpumask;
pub type S32 = i32;
pub type U64 = u64;
pub type Bool = bool;

#[repr(C)] pub struct Cpumask { _private: [u8; 0] }
#[repr(C)] pub struct SchedDomain { pub span_weight: u32, pub groups: *mut SchedGroup }
#[repr(C)] pub struct SchedGroup { pub group_weight: u32 }
#[repr(C)] pub struct SchedExtOps { pub flags: u64, pub update_idle: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct TaskStruct { pub nr_cpus_allowed: u32, pub cpus_ptr: *const Cpumask, pub migration_disabled: i32, pub flags: u64, pub comm: [u8; 16], pub pid: i32 }
#[repr(C)] pub struct Rq { pub scx: ScxRq }
#[repr(C)] pub struct ScxRq { pub flags: u64, pub local_dsq: ScxDsq, pub in_select_cpu: bool }
#[repr(C)] pub struct ScxDsq { pub nr: u64 }
#[repr(C)] pub struct ScxSched { pub level: i32, pub children: ListHead, pub pcpu: *mut u8 }
#[repr(C)] pub struct ListHead { _private: [u8; 0] }
#[repr(C)] pub struct BpfProgAux { _private: [u8; 0] }

extern "C" {
    static mut scx_builtin_idle_enabled: u8;
    static mut scx_builtin_idle_per_node: u8;
    static mut scx_selcpu_topo_llc: u8;
    static mut scx_selcpu_topo_numa: u8;
    static mut scx_idle_global_masks: ScxIdleCpus;
    static mut scx_idle_node_masks: *mut *mut ScxIdleCpus;
    static mut nr_node_ids: i32;
    static mut nr_cpu_ids: i32;
    static mut cpu_none_mask: *const Cpumask;
    static mut current: *mut TaskStruct;
    static mut scx_root: *mut ScxSched;
}

extern "C" {
    fn static_branch_maybe(_: i32, _: *mut u8) -> bool;
    fn static_branch_likely(_: *mut u8) -> bool;
    fn static_branch_unlikely(_: *mut u8) -> bool;
    fn static_branch_enable_cpuslocked(_: *mut u8); fn static_branch_disable_cpuslocked(_: *mut u8); fn static_branch_disable(_: *mut u8);
    fn cpu_to_node(_: i32) -> i32; fn num_online_cpus() -> u32; fn num_possible_cpus() -> u32;
    fn sched_smt_active() -> bool; fn cpu_smt_mask(_: i32) -> *const Cpumask;
    fn cpumask_intersects(_: *const Cpumask, _: *const Cpumask) -> bool; fn cpumask_test_cpu(_: i32, _: *const Cpumask) -> bool;
    fn __cpumask_clear_cpu(_: i32, _: *mut Cpumask); fn cpumask_test_and_clear_cpu(_: i32, _: *mut Cpumask) -> bool;
    fn cpumask_any_and_distribute(_: *mut Cpumask, _: *const Cpumask) -> i32; fn cpumask_any_distribute(_: *const Cpumask) -> i32;
    fn cpumask_andnot(_: *mut Cpumask, _: *mut Cpumask, _: *const Cpumask); fn cpumask_and(_: *mut Cpumask, _: *const Cpumask, _: *const Cpumask) -> bool;
    fn cpumask_or(_: *mut Cpumask, _: *mut Cpumask, _: *const Cpumask); fn cpumask_subset(_: *const Cpumask, _: *const Cpumask) -> bool;
    fn cpumask_empty(_: *const Cpumask) -> bool; fn cpumask_clear(_: *mut Cpumask); fn cpumask_first(_: *const Cpumask) -> i32;
    fn cpumask_of_node(_: i32) -> *const Cpumask; fn preempt_disable(); fn preempt_enable(); fn smp_processor_id() -> i32;
    fn rcu_read_lock(); fn rcu_read_unlock(); fn cpu_of(_: *mut Rq) -> i32; fn cpus_share_cache(_: i32, _: i32) -> bool;
    fn cpu_rq(_: i32) -> *mut Rq; fn this_rq() -> *mut Rq; fn task_rq(_: *mut TaskStruct) -> *mut Rq;
    fn raw_spin_lock_irqsave(_: *mut u8, _: *mut usize); fn raw_spin_unlock_irqrestore(_: *mut u8, _: usize);
    fn scx_cpu_node_if_enabled(_: i32) -> i32; fn assign_cpu(_: i32, _: *mut Cpumask, _: bool);
    fn scx_cpu_valid(_: *mut ScxSched, _: i32, _: *mut u8) -> bool; fn scx_prog_sched(_: *const BpfProgAux) -> *mut ScxSched;
    fn scx_root_protected_live() -> *mut ScxSched; fn scx_has_subs() -> bool; fn scx_task_sched(_: *mut TaskStruct) -> *mut ScxSched;
    fn scx_error(_: *mut ScxSched, _: *const u8, ...); fn register_btf_kfunc_id_set(_: i32, _: *const u8) -> i32;
}

unsafe fn idle_cpumask(node: i32) -> *mut ScxIdleCpus {
    if node == -1 { &raw mut scx_idle_global_masks } else { *scx_idle_node_masks.add(node as usize) }
}

unsafe fn scx_idle_test_and_clear_cpu(cpu: i32) -> bool {
    let node = scx_cpu_node_if_enabled(cpu); let masks = idle_cpumask(node);
    if sched_smt_active() {
        let smt = cpu_smt_mask(cpu); let idle_smts = (*masks).smt;
        if cpumask_intersects(smt, idle_smts) { cpumask_andnot(idle_smts, idle_smts, smt); }
        else if cpumask_test_cpu(cpu, idle_smts) { __cpumask_clear_cpu(cpu, idle_smts); }
    }
    cpumask_test_and_clear_cpu(cpu, (*masks).cpu)
}

unsafe fn pick_idle_cpu_in_node(allowed: *const Cpumask, node: i32, flags: u64) -> i32 {
    loop {
        let mut cpu;
        if sched_smt_active() {
            cpu = cpumask_any_and_distribute((*idle_cpumask(node)).smt, allowed);
            if cpu < nr_cpu_ids { if scx_idle_test_and_clear_cpu(cpu) { return cpu; } continue; }
            if flags & SCX_PICK_IDLE_CORE != 0 { return -16; }
        }
        cpu = cpumask_any_and_distribute((*idle_cpumask(node)).cpu, allowed);
        if cpu >= nr_cpu_ids { return -16; }
        if scx_idle_test_and_clear_cpu(cpu) { return cpu; }
    }
}

unsafe fn scx_pick_idle_cpu(allowed: *const Cpumask, node: i32, flags: u64) -> i32 {
    let cpu = pick_idle_cpu_in_node(allowed, node, flags); if cpu >= 0 { return cpu; }
    if node == -1 || flags & SCX_PICK_IDLE_IN_NODE != 0 { return -16; }
    pick_idle_cpu_from_online_nodes(allowed, node, flags)
}

#[cfg(feature = "CONFIG_NUMA")]
unsafe fn pick_idle_cpu_from_online_nodes(allowed: *const Cpumask, _node: i32, _flags: u64) -> i32 { let _ = allowed; -16 }
#[cfg(not(feature = "CONFIG_NUMA"))]
unsafe fn pick_idle_cpu_from_online_nodes(_: *const Cpumask, _: i32, _: u64) -> i32 { -16 }

unsafe fn llc_weight(_cpu: i32) -> u32 { 0 }
unsafe fn numa_weight(_cpu: i32) -> u32 { 0 }

pub unsafe extern "C" fn scx_idle_update_selcpu_topology(ops: *mut SchedExtOps) {
    let mut enable_llc = false; let mut enable_numa = false;
    let cpu = cpumask_first(core::ptr::null()); rcu_read_lock();
    let nr = llc_weight(cpu); if nr > 0 && nr < num_online_cpus() { enable_llc = true; }
    if (*ops).flags & SCX_OPS_BUILTIN_IDLE_PER_NODE == 0 { let nr = numa_weight(cpu); if nr > 0 && nr < num_online_cpus() { enable_numa = true; } }
    rcu_read_unlock();
    if enable_llc { static_branch_enable_cpuslocked(&raw mut scx_selcpu_topo_llc); } else { static_branch_disable_cpuslocked(&raw mut scx_selcpu_topo_llc); }
    if enable_numa { static_branch_enable_cpuslocked(&raw mut scx_selcpu_topo_numa); } else { static_branch_disable_cpuslocked(&raw mut scx_selcpu_topo_numa); }
}

unsafe fn task_affinity_all(p: *const TaskStruct) -> bool { (*p).nr_cpus_allowed >= num_possible_cpus() }

pub unsafe extern "C" fn scx_select_cpu_dfl(p: *mut TaskStruct, prev_cpu: i32, wake_flags: u64, allowed: *const Cpumask, flags: u64) -> i32 {
    let _ = wake_flags; let _ = flags; let _ = p; let _ = prev_cpu; let _ = allowed; -16
}

pub unsafe extern "C" fn scx_idle_init_masks() {}
pub unsafe extern "C" fn __scx_update_idle(_: *mut Rq, _: bool, _: bool) {}
pub unsafe extern "C" fn scx_idle_enable(_: *mut SchedExtOps) { static_branch_enable_cpuslocked(&raw mut scx_builtin_idle_enabled); }
pub unsafe extern "C" fn scx_idle_disable() { static_branch_disable(&raw mut scx_builtin_idle_enabled); static_branch_disable(&raw mut scx_builtin_idle_per_node); }

pub unsafe extern "C" fn scx_bpf_test_and_clear_cpu_idle(cpu: i32, _: *const BpfProgAux) -> bool { scx_idle_test_and_clear_cpu(cpu) }
pub unsafe extern "C" fn scx_bpf_pick_idle_cpu_node(m: *const Cpumask, node: i32, flags: u64, _: *const BpfProgAux) -> i32 { scx_pick_idle_cpu(m, node, flags) }
pub unsafe extern "C" fn scx_bpf_pick_idle_cpu(m: *const Cpumask, flags: u64, _: *const BpfProgAux) -> i32 { scx_pick_idle_cpu(m, -1, flags) }
pub unsafe extern "C" fn scx_bpf_get_idle_cpumask(_: *const BpfProgAux) -> *const Cpumask { (*idle_cpumask(-1)).cpu }
pub unsafe extern "C" fn scx_bpf_put_idle_cpumask(_: *const Cpumask) {}

const SCX_OPS_BUILTIN_IDLE_PER_NODE: u64 = 1 << 0;
const SCX_PICK_IDLE_CORE: u64 = 1 << 0;
const SCX_PICK_IDLE_IN_NODE: u64 = 1 << 1;

/* BTF kfunc registration tables and preprocessor-only annotations have no
 * executable Rust equivalent; their exported function declarations above are
 * retained as the source-level interface. */
pub unsafe extern "C" fn scx_idle_init() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
