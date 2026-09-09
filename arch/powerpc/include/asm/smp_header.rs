/* SPDX-License-Identifier: GPL-2.0-or-later */
/* PowerPC-specific SMP declarations, translated from smp.h. */

#[cfg(not(__ASSEMBLER__))]
use core::ffi::c_char;

#[cfg(not(__ASSEMBLER__))]
extern "C" {
    pub static mut boot_cpuid: i32;
    pub static mut boot_cpu_hwid: i32;
    pub static mut boot_core_hwid: i32;
    pub static mut spinning_secondaries: i32;
    pub static mut cpu_to_phys_id: *mut u32;
    pub static mut coregroup_enabled: bool;
    pub fn cpu_to_chip_id(cpu: i32) -> i32;
    pub static mut chip_id_lookup_table: *mut i32;
    pub static mut secondary_current: *mut task_struct;
    pub fn start_secondary(unused: *mut core::ffi::c_void);
    pub fn smp_send_nmi_ipi(cpu: i32, f: Option<unsafe extern "C" fn(*mut pt_regs)>, delay_us: u64) -> i32;
    pub fn smp_send_safe_nmi_ipi(cpu: i32, f: Option<unsafe extern "C" fn(*mut pt_regs)>, delay_us: u64) -> i32;
    pub fn smp_send_debugger_break();
    pub fn start_secondary_resume() -> !;
    pub fn smp_generic_give_timebase();
    pub fn smp_generic_take_timebase();
    pub fn cpu_to_core_id(cpu: i32) -> i32;
    pub static mut has_big_cores: bool;
    pub static mut thread_group_shares_l2: bool;
    pub static mut thread_group_shares_l3: bool;
    pub fn smp_request_message_ipi(virq: i32, message: i32) -> i32;
    pub static mut smp_ipi_name: *mut *const c_char;
    pub fn smp_muxed_ipi_message_pass(cpu: i32, msg: i32);
    pub fn smp_muxed_ipi_set_message(cpu: i32, msg: i32);
    pub fn smp_ipi_demux() -> irqreturn_t;
    pub fn smp_ipi_demux_relaxed() -> irqreturn_t;
    pub fn smp_init_pSeries();
    pub fn smp_init_cell();
    pub fn smp_setup_cpu_maps();
    pub fn __cpu_disable() -> i32;
    pub fn __cpu_die(cpu: u32);
}

#[cfg(not(__ASSEMBLER__))]
#[repr(C)]
pub struct smp_ops_t {
    pub message_pass: Option<unsafe extern "C" fn(i32, i32)>,
    #[cfg(CONFIG_PPC_SMP_MUXED_IPI)]
    pub cause_ipi: Option<unsafe extern "C" fn(i32)>,
    pub cause_nmi_ipi: Option<unsafe extern "C" fn(i32) -> i32>,
    pub probe: Option<unsafe extern "C" fn()>,
    pub kick_cpu: Option<unsafe extern "C" fn(i32) -> i32>,
    pub prepare_cpu: Option<unsafe extern "C" fn(i32) -> i32>,
    pub setup_cpu: Option<unsafe extern "C" fn(i32)>,
    pub bringup_done: Option<unsafe extern "C" fn()>,
    pub take_timebase: Option<unsafe extern "C" fn()>,
    pub give_timebase: Option<unsafe extern "C" fn()>,
    pub cpu_disable: Option<unsafe extern "C" fn() -> i32>,
    pub cpu_die: Option<unsafe extern "C" fn(u32)>,
    pub cpu_bootable: Option<unsafe extern "C" fn(u32) -> i32>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_offline_self: Option<unsafe extern "C" fn()>,
}

#[cfg(CONFIG_HOTPLUG_CPU)]
extern "C" {
    pub fn generic_cpu_disable() -> i32;
    pub fn generic_cpu_die(cpu: u32);
    pub fn generic_set_cpu_dead(cpu: u32);
    pub fn generic_set_cpu_up(cpu: u32);
    pub fn generic_check_cpu_restart(cpu: u32) -> i32;
    pub fn is_cpu_dead(cpu: u32) -> i32;
}

#[cfg(not(CONFIG_HOTPLUG_CPU))]
#[inline(always)] pub unsafe fn generic_set_cpu_up(_: i32) {}

pub const PPC_MSG_CALL_FUNCTION: i32 = 0;
pub const PPC_MSG_RESCHEDULE: i32 = 1;
pub const PPC_MSG_TICK_BROADCAST: i32 = 2;
pub const PPC_MSG_NMI_IPI: i32 = 3;
pub const PPC_MSG_RM_HOST_ACTION: i32 = 4;
pub const NMI_IPI_ALL_OTHERS: i32 = -2;

#[cfg(CONFIG_NMI_IPI)]
extern "C" { pub fn smp_handle_nmi_ipi(regs: *mut pt_regs) -> i32; }
#[cfg(not(CONFIG_NMI_IPI))]
#[inline(always)] pub unsafe fn smp_handle_nmi_ipi(_: *mut pt_regs) -> i32 { 0 }

#[cfg(not(CONFIG_SMP))]
pub const thread_group_shares_l2: i32 = 0;
#[cfg(not(CONFIG_SMP))]
pub const thread_group_shares_l3: i32 = 0;

#[cfg(CONFIG_PPC64)]
extern "C" { pub fn get_hard_smp_processor_id(cpu: i32) -> i32; pub fn set_hard_smp_processor_id(cpu: i32, phys: i32); }
#[cfg(all(not(CONFIG_PPC64), not(CONFIG_SMP)))]
extern "C" { pub static mut boot_cpuid_phys: i32; }

#[cfg(any(CONFIG_PPC64, CONFIG_SMP, CONFIG_KEXEC_CORE))]
extern "C" { pub fn smp_release_cpus(); }
#[cfg(not(any(CONFIG_PPC64, CONFIG_SMP, CONFIG_KEXEC_CORE)))]
#[inline(always)] pub unsafe fn smp_release_cpus() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
