/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/linux/cpu.h - generic cpu definition
 *
 * This is mainly for topological representation. We define the
 * basic 'struct cpu' here, which can be embedded in per-arch
 * definitions of processors.
 *
 * Basic handling of the devices is done in drivers/base/cpu.c
 *
 * CPUs are exported via sysfs in the devices/system/cpu directory.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not redefined here.

pub struct device;
pub struct device_node;
pub struct attribute_group;
pub struct device_attribute;
pub struct bus_type;
pub struct task_struct;
pub enum cpuhp_state {}

#[repr(C)]
pub struct cpu {
    pub node_id: i32,
    pub hotpluggable: i32,
    pub dev: device,
}

extern "C" {
    pub fn boot_cpu_init();
    pub fn boot_cpu_hotplug_init();
    pub fn cpu_init();
    pub fn trap_init();

    pub fn register_cpu(cpu: *mut cpu, num: i32) -> i32;
    pub fn get_cpu_device(cpu: u32) -> *mut device;
    pub fn cpu_is_hotpluggable(cpu: u32) -> bool;
    pub fn arch_match_cpu_phys_id(cpu: i32, phys_id: u64) -> bool;
    pub fn arch_find_n_match_cpu_physical_id(
        cpun: *mut device_node,
        cpu: i32,
        thread: *mut u32,
    ) -> bool;

    pub fn cpu_add_dev_attr(attr: *mut device_attribute) -> i32;
    pub fn cpu_remove_dev_attr(attr: *mut device_attribute);
    pub fn cpu_add_dev_attr_group(attrs: *mut attribute_group) -> i32;
    pub fn cpu_remove_dev_attr_group(attrs: *mut attribute_group);

    pub fn cpu_show_meltdown(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_spectre_v1(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_spectre_v2(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_spec_store_bypass(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_l1tf(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_mds(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_tsx_async_abort(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_itlb_multihit(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_srbds(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_mmio_stale_data(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_retbleed(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_spec_rstack_overflow(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_gds(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_reg_file_data_sampling(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_ghostwrite(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_old_microcode(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_indirect_target_selection(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_tsa(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;
    pub fn cpu_show_vmscape(dev: *mut device, attr: *mut device_attribute, buf: *mut i8) -> ssize_t;

    pub fn cpu_device_create(
        parent: *mut device,
        drvdata: *mut core::ffi::c_void,
        groups: *const *const attribute_group,
        fmt: *const i8,
        ...,
    ) -> *mut device;
    pub fn arch_cpu_is_hotpluggable(cpu: i32) -> bool;
    pub fn arch_register_cpu(cpu: i32) -> i32;
    pub fn arch_unregister_cpu(cpu: i32);
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
extern "C" {
    pub fn unregister_cpu(cpu: *mut cpu);
    pub fn arch_cpu_probe(buf: *const i8, count: usize) -> ssize_t;
    pub fn arch_cpu_release(buf: *const i8, count: usize) -> ssize_t;
}

pub const CPU_ONLINE: u32 = 0x0002;
pub const CPU_UP_PREPARE: u32 = 0x0003;
pub const CPU_DEAD: u32 = 0x0007;
pub const CPU_DEAD_FROZEN: u32 = 0x0008;
pub const CPU_POST_DEAD: u32 = 0x0009;
pub const CPU_BROKEN: u32 = 0x000B;

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub static mut cpuhp_tasks_frozen: bool;
    pub fn add_cpu(cpu: u32) -> i32;
    pub fn cpu_device_up(dev: *mut device) -> i32;
    pub fn notify_cpu_starting(cpu: u32);
    pub fn cpu_maps_update_begin();
    pub fn cpu_maps_update_done();
    pub fn bringup_hibernate_cpu(sleep_cpu: u32) -> i32;
    pub fn bringup_nonboot_cpus(max_cpus: u32);
    pub fn arch_cpu_rescan_dead_smt_siblings() -> i32;
}

#[cfg(not(feature = "CONFIG_SMP"))]
pub const cpuhp_tasks_frozen: i32 = 0;

#[cfg(not(feature = "CONFIG_SMP"))]
pub fn cpu_maps_update_begin() {}
#[cfg(not(feature = "CONFIG_SMP"))]
pub fn cpu_maps_update_done() {}
#[cfg(not(feature = "CONFIG_SMP"))]
pub fn add_cpu(_cpu: u32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_SMP"))]
pub fn arch_cpu_rescan_dead_smt_siblings() -> i32 { 0 }

extern "C" {
    pub static cpu_subsys: bus_type;
    pub fn arch_cpu_idle();
    pub fn arch_cpu_idle_prepare();
    pub fn arch_cpu_idle_enter();
    pub fn arch_cpu_idle_exit();
    pub fn arch_tick_broadcast_enter();
    pub fn arch_tick_broadcast_exit();
    pub fn play_idle_precise(duration_ns: u64, latency_ns: u64);
    pub fn cpu_idle_poll_ctrl(enable: bool);
    pub fn cpu_in_idle(pc: usize) -> bool;
}

#[cfg(feature = "CONFIG_PM_SLEEP_SMP")]
extern "C" {
    pub fn freeze_secondary_cpus(primary: i32) -> i32;
    pub fn thaw_secondary_cpus();
}

#[cfg(feature = "CONFIG_PM_SLEEP_SMP")]
pub fn suspend_disable_secondary_cpus() -> i32 { unsafe { freeze_secondary_cpus(0) } }
#[cfg(feature = "CONFIG_PM_SLEEP_SMP")]
pub fn suspend_enable_secondary_cpus() { unsafe { thaw_secondary_cpus() } }
#[cfg(not(feature = "CONFIG_PM_SLEEP_SMP"))]
pub fn thaw_secondary_cpus() {}
#[cfg(not(feature = "CONFIG_PM_SLEEP_SMP"))]
pub fn suspend_disable_secondary_cpus() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PM_SLEEP_SMP"))]
pub fn suspend_enable_secondary_cpus() {}

extern "C" {
    pub fn cpu_startup_entry(state: cpuhp_state) -> !;
    pub fn arch_cpu_idle_dead() -> !;
}

#[cfg(feature = "CONFIG_ARCH_HAS_CPU_FINALIZE_INIT")]
extern "C" { pub fn arch_cpu_finalize_init(); }
#[cfg(not(feature = "CONFIG_ARCH_HAS_CPU_FINALIZE_INIT"))]
pub fn arch_cpu_finalize_init() {}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
extern "C" { pub fn cpuhp_report_idle_dead(); }
#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
pub fn cpuhp_report_idle_dead() {}

#[repr(C)]
pub enum cpu_attack_vectors {
    CPU_MITIGATE_USER_KERNEL,
    CPU_MITIGATE_USER_USER,
    CPU_MITIGATE_GUEST_HOST,
    CPU_MITIGATE_GUEST_GUEST,
    NR_CPU_ATTACK_VECTORS,
}

#[repr(C)]
pub enum smt_mitigations { SMT_MITIGATIONS_OFF, SMT_MITIGATIONS_AUTO, SMT_MITIGATIONS_ON }

#[cfg(feature = "CONFIG_CPU_MITIGATIONS")]
extern "C" {
    pub fn cpu_mitigations_off() -> bool;
    pub fn cpu_mitigations_auto_nosmt() -> bool;
    pub fn cpu_attack_vector_mitigated(v: cpu_attack_vectors) -> bool;
    pub static mut smt_mitigations: smt_mitigations;
}
#[cfg(not(feature = "CONFIG_CPU_MITIGATIONS"))]
pub fn cpu_mitigations_off() -> bool { true }
#[cfg(not(feature = "CONFIG_CPU_MITIGATIONS"))]
pub fn cpu_mitigations_auto_nosmt() -> bool { false }
#[cfg(not(feature = "CONFIG_CPU_MITIGATIONS"))]
pub fn cpu_attack_vector_mitigated(_v: cpu_attack_vectors) -> bool { false }
#[cfg(not(feature = "CONFIG_CPU_MITIGATIONS"))]
pub const smt_mitigations: smt_mitigations = smt_mitigations::SMT_MITIGATIONS_OFF;

extern "C" {
    pub fn arch_prctl_get_branch_landing_pad_state(t: *mut task_struct, state: *mut usize) -> i32;
    pub fn arch_prctl_set_branch_landing_pad_state(t: *mut task_struct, state: usize) -> i32;
    pub fn arch_prctl_lock_branch_landing_pad_state(t: *mut task_struct) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
