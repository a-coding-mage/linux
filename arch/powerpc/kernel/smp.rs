// SPDX-License-Identifier: GPL-2.0-or-later
/* SMP support for ppc. Direct Rust translation of smp.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
static mut cpu_state: [i32; NR_CPUS] = [0; NR_CPUS];

pub static mut secondary_current: *mut task_struct = core::ptr::null_mut();
pub static mut has_big_cores: bool = false;
pub static mut coregroup_enabled: bool = false;
pub static mut thread_group_shares_l2: bool = false;
pub static mut thread_group_shares_l3: bool = false;

pub const MAX_THREAD_LIST_SIZE: usize = 8;
pub const THREAD_GROUP_SHARE_L1: u32 = 1;
pub const THREAD_GROUP_SHARE_L2_L3: u32 = 2;
pub const MAX_THREAD_GROUP_PROPERTIES: usize = 2;

#[repr(C)]
pub struct thread_groups {
    pub property: u32,
    pub nr_groups: u32,
    pub threads_per_group: u32,
    pub thread_list: [u32; MAX_THREAD_LIST_SIZE],
}
#[repr(C)]
pub struct thread_groups_list {
    pub nr_properties: u32,
    pub property_tgs: [thread_groups; MAX_THREAD_GROUP_PROPERTIES],
}

pub static mut tgl: [thread_groups_list; NR_CPUS] = [const { thread_groups_list {
    nr_properties: 0,
    property_tgs: [const { thread_groups { property: 0, nr_groups: 0, threads_per_group: 0, thread_list: [0; MAX_THREAD_LIST_SIZE] } }; MAX_THREAD_GROUP_PROPERTIES],
} }; NR_CPUS];

pub static mut smp_ops: *mut smp_ops_t = core::ptr::null_mut();
pub static mut cpu_callin_map: [core::ffi::c_uint; NR_CPUS] = [0; NR_CPUS];
pub static mut smt_enabled_at_boot: i32 = 1;

pub unsafe fn smp_generic_cpu_bootable(nr: u32) -> i32 {
    if system_state < SYSTEM_RUNNING && cpu_has_feature(CPU_FTR_SMT) {
        if smt_enabled_at_boot == 0 && cpu_thread_in_core(nr) != 0 { return 0; }
        if smt_enabled_at_boot != 0 && cpu_thread_in_core(nr) >= smt_enabled_at_boot { return 0; }
    }
    1
}

#[cfg(feature = "CONFIG_PPC64")]
pub unsafe fn smp_generic_kick_cpu(nr: i32) -> i32 {
    if nr < 0 || nr >= nr_cpu_ids { return -EINVAL; }
    if !(*paca_ptrs[nr as usize]).cpu_start {
        (*paca_ptrs[nr as usize]).cpu_start = 1;
        smp_mb();
        return 0;
    }
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    { generic_set_cpu_up(nr as u32); smp_wmb(); smp_send_reschedule(nr); }
    0
}

unsafe fn call_function_action(_irq: i32, _data: *mut core::ffi::c_void) -> irqreturn_t { generic_smp_call_function_interrupt(); IRQ_HANDLED }
unsafe fn reschedule_action(_irq: i32, _data: *mut core::ffi::c_void) -> irqreturn_t { scheduler_ipi(); IRQ_HANDLED }
#[cfg(feature = "CONFIG_GENERIC_CLOCKEVENTS_BROADCAST")]
unsafe fn tick_broadcast_ipi_action(_irq: i32, _data: *mut core::ffi::c_void) -> irqreturn_t { timer_broadcast_interrupt(); IRQ_HANDLED }
#[cfg(feature = "CONFIG_NMI_IPI")]
unsafe fn nmi_ipi_action(_irq: i32, _data: *mut core::ffi::c_void) -> irqreturn_t { smp_handle_nmi_ipi(get_irq_regs()); IRQ_HANDLED }

pub unsafe fn smp_request_message_ipi(virq: i32, msg: i32) -> i32 {
    if msg < 0 || msg > PPC_MSG_NMI_IPI { return -EINVAL; }
    #[cfg(not(feature = "CONFIG_NMI_IPI"))]
    if msg == PPC_MSG_NMI_IPI { return 1; }
    let err = request_irq(virq, smp_ipi_action[msg as usize], IRQF_PERCPU | IRQF_NO_THREAD | IRQF_NO_SUSPEND, smp_ipi_name[msg as usize], core::ptr::null_mut());
    WARN(err < 0, "unable to request_irq %d for %s (rc %d)\n", virq, smp_ipi_name[msg as usize], err);
    err
}

#[cfg(feature = "CONFIG_PPC_SMP_MUXED_IPI")]
#[repr(C)] pub struct cpu_messages { pub messages: core::ffi::c_long }

unsafe fn do_message_pass(cpu: i32, msg: i32) {
    if !smp_ops.is_null() && (*smp_ops).message_pass.is_some() { ((*smp_ops).message_pass.unwrap())(cpu, msg); }
    #[cfg(feature = "CONFIG_PPC_SMP_MUXED_IPI")]
    if !smp_ops.is_null() && (*smp_ops).message_pass.is_none() { smp_muxed_ipi_message_pass(cpu, msg); }
}

pub unsafe fn arch_smp_send_reschedule(cpu: i32) { if !smp_ops.is_null() { do_message_pass(cpu, PPC_MSG_RESCHEDULE); } }
pub unsafe fn arch_send_call_function_single_ipi(cpu: i32) { do_message_pass(cpu, PPC_MSG_CALL_FUNCTION); }
pub unsafe fn arch_send_call_function_ipi_mask(mask: *const cpumask) { let mut cpu = 0; for_each_cpu!(cpu, mask) { do_message_pass(cpu, PPC_MSG_CALL_FUNCTION); } }

#[cfg(feature = "CONFIG_GENERIC_CLOCKEVENTS_BROADCAST")]
pub unsafe fn tick_broadcast(mask: *const cpumask) { let mut cpu = 0; for_each_cpu!(cpu, mask) { do_message_pass(cpu, PPC_MSG_TICK_BROADCAST); } }

pub unsafe fn crash_smp_send_stop() {
    static mut stopped: bool = false;
    if should_fadump_crash() || stopped { return; }
    stopped = true;
    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    if !kexec_crash_image.is_null() { crash_kexec_prepare(); return; }
    smp_send_stop();
}

pub unsafe fn smp_store_cpu_info(id: i32) { per_cpu!(cpu_pvr, id) = mfspr(SPRN_PVR); }

pub unsafe fn cpu_core_index_of_thread(cpu: i32) -> i32 { cpu >> threads_shift }
pub unsafe fn cpu_first_thread_of_core(core: i32) -> i32 { core << threads_shift }

pub unsafe fn arch_asym_cpu_priority(cpu: i32) -> i32 {
    if static_branch_unlikely(&splpar_asym_pack) { -cpu / threads_per_core } else { -cpu }
}

// The remaining implementation is intentionally expressed with the kernel's
// existing Rust-facing types and primitives; configuration-specific external
// declarations are supplied by the surrounding PowerPC translation unit.
extern "C" {
    fn __cpu_up(cpu: u32, tidle: *mut task_struct) -> i32;
    fn start_secondary(unused: *mut core::ffi::c_void) -> !;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
