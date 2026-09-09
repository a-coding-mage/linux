/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_void};

/* Dependency types supplied by other kernel translations. */
#[repr(C)]
pub struct hlist_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpuhp_state {
    CPUHP_INVALID = -1,
    CPUHP_OFFLINE = 0,
    CPUHP_CREATE_THREADS,
    CPUHP_PERF_X86_PREPARE,
    CPUHP_PERF_X86_AMD_UNCORE_PREP,
    CPUHP_PERF_POWER,
    CPUHP_PERF_SUPERH,
    CPUHP_X86_HPET_DEAD,
    CPUHP_X86_MCE_DEAD,
    CPUHP_VIRT_NET_DEAD,
    CPUHP_IBMVNIC_DEAD,
    CPUHP_SLUB_DEAD,
    CPUHP_DEBUG_OBJ_DEAD,
    CPUHP_MM_WRITEBACK_DEAD,
    CPUHP_MM_VMSTAT_DEAD,
    CPUHP_SOFTIRQ_DEAD,
    CPUHP_NET_MVNETA_DEAD,
    CPUHP_CPUIDLE_DEAD,
    CPUHP_ARM64_FPSIMD_DEAD,
    CPUHP_ARM_OMAP_WAKE_DEAD,
    CPUHP_IRQ_POLL_DEAD,
    CPUHP_BLOCK_SOFTIRQ_DEAD,
    CPUHP_BIO_DEAD,
    CPUHP_ACPI_CPUDRV_DEAD,
    CPUHP_S390_PFAULT_DEAD,
    CPUHP_BLK_MQ_DEAD,
    CPUHP_FS_BUFF_DEAD,
    CPUHP_PRINTK_DEAD,
    CPUHP_MM_MEMCQ_DEAD,
    CPUHP_PERCPU_CNT_DEAD,
    CPUHP_RADIX_DEAD,
    CPUHP_PAGE_ALLOC,
    CPUHP_NET_DEV_DEAD,
    CPUHP_IOMMU_IOVA_DEAD,
    CPUHP_AP_ARM_CACHE_B15_RAC_DEAD,
    CPUHP_AP_DTPM_CPU_DEAD,
    CPUHP_RANDOM_PREPARE,
    CPUHP_WORKQUEUE_PREP,
    CPUHP_POWER_NUMA_PREPARE,
    CPUHP_HRTIMERS_PREPARE,
    CPUHP_X2APIC_PREPARE,
    CPUHP_SMPCFD_PREPARE,
    CPUHP_RELAY_PREPARE,
    CPUHP_MD_RAID5_PREPARE,
    CPUHP_RCUTREE_PREP,
    CPUHP_CPUIDLE_COUPLED_PREPARE,
    CPUHP_POWERPC_PMAC_PREPARE,
    CPUHP_POWERPC_MMU_CTX_PREPARE,
    CPUHP_XEN_PREPARE,
    CPUHP_XEN_EVTCHN_PREPARE,
    CPUHP_ARM_SHMOBILE_SCU_PREPARE,
    CPUHP_SH_SH3X_PREPARE,
    CPUHP_TOPOLOGY_PREPARE,
    CPUHP_NET_IUCV_PREPARE,
    CPUHP_ARM_BL_PREPARE,
    CPUHP_TRACE_RB_PREPARE,
    CPUHP_MM_ZSWP_POOL_PREPARE,
    CPUHP_KVM_PPC_BOOK3S_PREPARE,
    CPUHP_ZCOMP_PREPARE,
    CPUHP_TIMERS_PREPARE,
    CPUHP_TMIGR_PREPARE,
    CPUHP_MIPS_SOC_PREPARE,
    CPUHP_BP_PREPARE_DYN,
    CPUHP_BP_PREPARE_DYN_END = CPUHP_BP_PREPARE_DYN as isize + 20,
    CPUHP_BP_KICK_AP,
    CPUHP_BRINGUP_CPU,
    CPUHP_AP_IDLE_DEAD,
    CPUHP_AP_OFFLINE,
    CPUHP_AP_CACHECTRL_STARTING,
    CPUHP_AP_SCHED_STARTING,
    CPUHP_AP_RCUTREE_DYING,
    CPUHP_AP_CPU_PM_STARTING,
    CPUHP_AP_IRQ_GIC_STARTING,
    CPUHP_AP_IRQ_HIP04_STARTING,
    CPUHP_AP_IRQ_APPLE_AIC_STARTING,
    CPUHP_AP_IRQ_ARMADA_XP_STARTING,
    CPUHP_AP_IRQ_BCM2836_STARTING,
    CPUHP_AP_IRQ_MIPS_GIC_STARTING,
    CPUHP_AP_IRQ_EIOINTC_STARTING,
    CPUHP_AP_IRQ_AVECINTC_STARTING,
    CPUHP_AP_IRQ_SIFIVE_PLIC_STARTING,
    CPUHP_AP_IRQ_ACLINT_SSWI_STARTING,
    CPUHP_AP_IRQ_RISCV_IMSIC_STARTING,
    CPUHP_AP_IRQ_RISCV_SBI_IPI_STARTING,
    CPUHP_AP_ARM_MVEBU_COHERENCY,
    CPUHP_AP_PERF_X86_AMD_UNCORE_STARTING,
    CPUHP_AP_PERF_X86_STARTING,
    CPUHP_AP_PERF_X86_AMD_IBS_STARTING,
    CPUHP_AP_PERF_XTENSA_STARTING,
    CPUHP_AP_ARM_VFP_STARTING,
    CPUHP_AP_ARM64_DEBUG_MONITORS_STARTING,
    CPUHP_AP_PERF_ARM_HW_BREAKPOINT_STARTING,
    CPUHP_AP_PERF_ARM_ACPI_STARTING,
    CPUHP_AP_PERF_ARM_STARTING,
    CPUHP_AP_PERF_RISCV_STARTING,
    CPUHP_AP_ARM_L2X0_STARTING,
    CPUHP_AP_EXYNOS4_MCT_TIMER_STARTING,
    CPUHP_AP_ARM_ARCH_TIMER_STARTING,
    CPUHP_AP_ARM_ARCH_TIMER_EVTSTRM_STARTING,
    CPUHP_AP_ARM_GLOBAL_TIMER_STARTING,
    CPUHP_AP_JCORE_TIMER_STARTING,
    CPUHP_AP_ARM_TWD_STARTING,
    CPUHP_AP_QCOM_TIMER_STARTING,
    CPUHP_AP_TEGRA_TIMER_STARTING,
    CPUHP_AP_ARMADA_TIMER_STARTING,
    CPUHP_AP_LOONGARCH_ARCH_TIMER_STARTING,
    CPUHP_AP_MIPS_GIC_TIMER_STARTING,
    CPUHP_AP_ARC_TIMER_STARTING,
    CPUHP_AP_REALTEK_TIMER_STARTING,
    CPUHP_AP_RISCV_TIMER_STARTING,
    CPUHP_AP_CLINT_TIMER_STARTING,
    CPUHP_AP_CSKY_TIMER_STARTING,
    CPUHP_AP_TI_GP_TIMER_STARTING,
    CPUHP_AP_HYPERV_TIMER_STARTING,
    CPUHP_AP_DUMMY_TIMER_STARTING,
    CPUHP_AP_ARM_XEN_STARTING,
    CPUHP_AP_ARM_XEN_RUNSTATE_STARTING,
    CPUHP_AP_ARM64_ISNDEP_STARTING,
    CPUHP_AP_SMPCFD_DYING,
    CPUHP_AP_HRTIMERS_DYING,
    CPUHP_AP_TICK_DYING,
    CPUHP_AP_X86_TBOOT_DYING,
    CPUHP_AP_ARM_CACHE_B15_RAC_DYING,
    CPUHP_AP_ONLINE,
    CPUHP_TEARDOWN_CPU,
    CPUHP_AP_ONLINE_IDLE,
    CPUHP_AP_HYPERV_ONLINE,
    CPUHP_AP_KVM_ONLINE,
    CPUHP_AP_SCHED_WAIT_EMPTY,
    CPUHP_AP_SMPBOOT_THREADS,
    CPUHP_AP_IRQ_AFFINITY_ONLINE,
    CPUHP_AP_BLK_MQ_ONLINE,
    CPUHP_AP_ARM_MVEBU_SYNC_CLOCKS,
    CPUHP_AP_ARM_CORESIGHT_ONLINE,
    CPUHP_AP_X86_INTEL_EPB_ONLINE,
    CPUHP_AP_PERF_ONLINE,
    CPUHP_AP_PERF_X86_ONLINE,
    CPUHP_AP_PERF_X86_UNCORE_ONLINE,
    CPUHP_AP_PERF_X86_AMD_UNCORE_ONLINE,
    CPUHP_AP_PERF_X86_AMD_POWER_ONLINE,
    CPUHP_AP_PERF_S390_CF_ONLINE,
    CPUHP_AP_PERF_S390_SF_ONLINE,
    CPUHP_AP_PERF_ARM_CCI_ONLINE,
    CPUHP_AP_PERF_ARM_CCN_ONLINE,
    CPUHP_AP_PERF_ARM_HISI_CPA_ONLINE,
    CPUHP_AP_PERF_ARM_HISI_DDRC_ONLINE,
    CPUHP_AP_PERF_ARM_HISI_HHA_ONLINE,
    CPUHP_AP_PERF_ARM_HISI_L3_ONLINE,
    CPUHP_AP_PERF_ARM_HISI_PA_ONLINE,
    CPUHP_AP_PERF_ARM_HISI_SLLC_ONLINE,
    CPUHP_AP_PERF_ARM_HISI_PCIE_PMU_ONLINE,
    CPUHP_AP_PERF_ARM_HNS3_PMU_ONLINE,
    CPUHP_AP_PERF_ARM_L2X0_ONLINE,
    CPUHP_AP_PERF_ARM_QCOM_L2_ONLINE,
    CPUHP_AP_PERF_ARM_QCOM_L3_ONLINE,
    CPUHP_AP_PERF_ARM_APM_XGENE_ONLINE,
    CPUHP_AP_PERF_ARM_CAVIUM_TX2_UNCORE_ONLINE,
    CPUHP_AP_PERF_ARM_MARVELL_CN10K_DDR_ONLINE,
    CPUHP_AP_PERF_ARM_MRVL_PEM_ONLINE,
    CPUHP_AP_PERF_POWERPC_NEST_IMC_ONLINE,
    CPUHP_AP_PERF_POWERPC_CORE_IMC_ONLINE,
    CPUHP_AP_PERF_POWERPC_THREAD_IMC_ONLINE,
    CPUHP_AP_PERF_POWERPC_TRACE_IMC_ONLINE,
    CPUHP_AP_PERF_POWERPC_HV_24x7_ONLINE,
    CPUHP_AP_PERF_POWERPC_HV_GPCI_ONLINE,
    CPUHP_AP_PERF_CSKY_ONLINE,
    CPUHP_AP_TMIGR_ONLINE,
    CPUHP_AP_WATCHDOG_ONLINE,
    CPUHP_AP_WORKQUEUE_ONLINE,
    CPUHP_AP_RANDOM_ONLINE,
    CPUHP_AP_RCUTREE_ONLINE,
    CPUHP_AP_KTHREADS_ONLINE,
    CPUHP_AP_BASE_CACHEINFO_ONLINE,
    CPUHP_AP_ONLINE_DYN,
    CPUHP_AP_ONLINE_DYN_END = CPUHP_AP_ONLINE_DYN as isize + 40,
    CPUHP_AP_X86_HPET_ONLINE,
    CPUHP_AP_X86_KVM_CLK_ONLINE,
    CPUHP_AP_ACTIVE,
    CPUHP_ONLINE,
}

pub type Startup = Option<unsafe extern "C" fn(cpu: u32) -> i32>;
pub type Teardown = Option<unsafe extern "C" fn(cpu: u32) -> i32>;

extern "C" {
    pub fn __cpuhp_setup_state(state: cpuhp_state, name: *const c_char, invoke: bool,
        startup: Startup, teardown: Teardown, multi_instance: bool) -> i32;
    pub fn __cpuhp_setup_state_cpuslocked(state: cpuhp_state, name: *const c_char, invoke: bool,
        startup: Startup, teardown: Teardown, multi_instance: bool) -> i32;
    pub fn __cpuhp_state_add_instance(state: cpuhp_state, node: *mut hlist_node, invoke: bool) -> i32;
    pub fn __cpuhp_state_add_instance_cpuslocked(state: cpuhp_state, node: *mut hlist_node, invoke: bool) -> i32;
    pub fn __cpuhp_remove_state(state: cpuhp_state, invoke: bool);
    pub fn __cpuhp_remove_state_cpuslocked(state: cpuhp_state, invoke: bool);
    pub fn __cpuhp_state_remove_instance(state: cpuhp_state, node: *mut hlist_node, invoke: bool) -> i32;
    pub fn cpuhp_ap_sync_alive();
    pub fn arch_cpuhp_sync_state_poll();
    pub fn arch_cpuhp_cleanup_kick_cpu(cpu: u32);
    pub fn arch_cpuhp_kick_ap_alive(cpu: u32, tidle: *mut task_struct) -> i32;
    pub fn arch_cpuhp_init_parallel_bringup() -> bool;
    pub fn cpuhp_ap_report_dead();
    pub fn arch_cpuhp_cleanup_dead_cpu(cpu: u32);
}

#[inline]
pub unsafe fn cpuhp_setup_state(state: cpuhp_state, name: *const c_char, startup: Startup, teardown: Teardown) -> i32 {
    __cpuhp_setup_state(state, name, true, startup, teardown, false)
}
#[inline]
pub unsafe fn cpuhp_setup_state_cpuslocked(state: cpuhp_state, name: *const c_char, startup: Startup, teardown: Teardown) -> i32 {
    __cpuhp_setup_state_cpuslocked(state, name, true, startup, teardown, false)
}
#[inline]
pub unsafe fn cpuhp_setup_state_nocalls(state: cpuhp_state, name: *const c_char, startup: Startup, teardown: Teardown) -> i32 {
    __cpuhp_setup_state(state, name, false, startup, teardown, false)
}
#[inline]
pub unsafe fn cpuhp_setup_state_nocalls_cpuslocked(state: cpuhp_state, name: *const c_char, startup: Startup, teardown: Teardown) -> i32 {
    __cpuhp_setup_state_cpuslocked(state, name, false, startup, teardown, false)
}

/* Multi-instance callback types are passed through the C void-pointer ABI. */
pub unsafe fn cpuhp_setup_state_multi(state: cpuhp_state, name: *const c_char,
    startup: Option<unsafe extern "C" fn(u32, *mut hlist_node) -> i32>,
    teardown: Option<unsafe extern "C" fn(u32, *mut hlist_node) -> i32>) -> i32 {
    __cpuhp_setup_state(state, name, false, core::mem::transmute(startup), core::mem::transmute(teardown), true)
}
#[inline]
pub unsafe fn cpuhp_state_add_instance(state: cpuhp_state, node: *mut hlist_node) -> i32 { __cpuhp_state_add_instance(state, node, true) }
#[inline]
pub unsafe fn cpuhp_state_add_instance_nocalls(state: cpuhp_state, node: *mut hlist_node) -> i32 { __cpuhp_state_add_instance(state, node, false) }
#[inline]
pub unsafe fn cpuhp_state_add_instance_nocalls_cpuslocked(state: cpuhp_state, node: *mut hlist_node) -> i32 { __cpuhp_state_add_instance_cpuslocked(state, node, false) }
#[inline]
pub unsafe fn cpuhp_remove_state(state: cpuhp_state) { __cpuhp_remove_state(state, true) }
#[inline]
pub unsafe fn cpuhp_remove_state_nocalls(state: cpuhp_state) { __cpuhp_remove_state(state, false) }
#[inline]
pub unsafe fn cpuhp_remove_state_nocalls_cpuslocked(state: cpuhp_state) { __cpuhp_remove_state_cpuslocked(state, false) }
#[inline]
pub unsafe fn cpuhp_remove_multi_state(state: cpuhp_state) { __cpuhp_remove_state(state, false) }
#[inline]
pub unsafe fn cpuhp_state_remove_instance(state: cpuhp_state, node: *mut hlist_node) -> i32 { __cpuhp_state_remove_instance(state, node, true) }
#[inline]
pub unsafe fn cpuhp_state_remove_instance_nocalls(state: cpuhp_state, node: *mut hlist_node) -> i32 { __cpuhp_state_remove_instance(state, node, false) }

#[cfg(feature = "CONFIG_SMP")]
extern "C" { pub fn cpuhp_online_idle(state: cpuhp_state); }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline] pub unsafe fn cpuhp_online_idle(_state: cpuhp_state) {}

#[cfg(not(feature = "CONFIG_HOTPLUG_CORE_SYNC_DEAD"))]
#[inline] pub unsafe fn cpuhp_ap_report_dead() {}
#[cfg(not(feature = "CONFIG_HOTPLUG_CORE_SYNC_DEAD"))]
#[inline] pub unsafe fn arch_cpuhp_cleanup_dead_cpu(_cpu: u32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
