/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/interrupt.h, linux/ftrace.h, asm/traps.h,
// asm/head.h, and asm/io.h.

use core::ffi::c_void;

#[cfg(CONFIG_SPARC64)]
#[repr(C)]
pub struct seq_file { _private: [u8; 0] }

/* cpu.c */
extern "C" {
    pub static sparc_pmu_type: *const u8;
    pub static mut fsr_storage: core::ffi::c_uint;
    pub static mut ncpus_probed: core::ffi::c_int;
}

/* process{_32,_64}.c */
extern "C" {
    pub fn sparc_clone(regs: *mut pt_regs) -> core::ffi::c_long;
    pub fn sparc_fork(regs: *mut pt_regs) -> core::ffi::c_long;
    pub fn sparc_vfork(regs: *mut pt_regs) -> core::ffi::c_long;
    pub fn sparc_clone3(regs: *mut pt_regs) -> core::ffi::c_long;
}

#[cfg(CONFIG_SPARC64)]
extern "C" {
    /* setup_64.c */
    pub fn cpucap_info(file: *mut seq_file);

    /* sys_sparc_64.c */
    pub fn sys_kern_features() -> core::ffi::c_long;

    /* unaligned_64.c */
    pub fn kernel_unaligned_trap(regs: *mut pt_regs, insn: core::ffi::c_uint);
    pub fn handle_popc(insn: u32, regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn handle_lddfmna(regs: *mut pt_regs, sfar: core::ffi::c_ulong, sfsr: core::ffi::c_ulong);
    pub fn handle_stdfmna(regs: *mut pt_regs, sfar: core::ffi::c_ulong, sfsr: core::ffi::c_ulong);

    /* uprobes.c */
    pub fn uprobe_trap(regs: *mut pt_regs, trap_level: core::ffi::c_ulong);

    /* smp_64.c */
    pub fn smp_call_function_client(irq: core::ffi::c_int, regs: *mut pt_regs);
    pub fn smp_call_function_single_client(irq: core::ffi::c_int, regs: *mut pt_regs);
    pub fn smp_penguin_jailcell(irq: core::ffi::c_int, regs: *mut pt_regs);
    pub fn smp_receive_signal_client(irq: core::ffi::c_int, regs: *mut pt_regs);

    /* kgdb_64.c */
    pub fn smp_kgdb_capture_client(irq: core::ffi::c_int, regs: *mut pt_regs);

    /* signal32.c */
    pub fn do_sigreturn32(regs: *mut pt_regs);
    pub fn do_rt_sigreturn32(regs: *mut pt_regs);
    pub fn do_signal32(regs: *mut pt_regs);
    pub fn do_sys32_sigstack(u_ssptr: u32, u_ossptr: u32, sp: core::ffi::c_ulong) -> core::ffi::c_int;

    /* time_64.c */
    pub fn time_init_early();

    /* compat_audit.c */
    pub static mut sparc32_dir_class: [core::ffi::c_uint; 0];
    pub static mut sparc32_chattr_class: [core::ffi::c_uint; 0];
    pub static mut sparc32_write_class: [core::ffi::c_uint; 0];
    pub static mut sparc32_read_class: [core::ffi::c_uint; 0];
    pub static mut sparc32_signal_class: [core::ffi::c_uint; 0];
    pub fn sparc32_classify_syscall(syscall: core::ffi::c_uint) -> core::ffi::c_int;
}

#[cfg(CONFIG_SPARC64)]
pub unsafe fn kimage_addr_to_ra(p: *const c_void) -> core::ffi::c_ulong {
    let val = p as core::ffi::c_ulong;
    kern_base + (val - KERNBASE)
}

#[cfg(all(CONFIG_SPARC64, CONFIG_PCI))]
extern "C" {
    pub fn ali_sound_dma_hack(dev: *mut device, device_mask: u64) -> core::ffi::c_int;
}

#[cfg(all(CONFIG_SPARC64, not(CONFIG_PCI)))]
#[inline]
pub unsafe fn ali_sound_dma_hack(_dev: *mut device, _mask: u64) -> core::ffi::c_int { 0 }

#[cfg(CONFIG_SPARC32)]
extern "C" {
    /* setup_32.c */
    pub fn sparc32_start_kernel(rp: *mut linux_romvec);
    /* cpu.c */
    pub fn cpu_probe();
    /* traps_32.c */
    pub fn handle_hw_divzero(regs: *mut pt_regs, pc: core::ffi::c_ulong, npc: core::ffi::c_ulong, psr: core::ffi::c_ulong);
    /* irq_32.c */
    pub static mut static_irqaction: [irqaction; 0];
    pub static mut static_irq_count: core::ffi::c_int;
    pub static mut irq_action_lock: spinlock_t;
    pub fn unexpected_irq(irq: core::ffi::c_int, dev_id: *mut c_void, regs: *mut pt_regs);
    /* sun4m_irq.c */
    pub fn sun4m_init_IRQ();
    pub fn sun4m_unmask_profile_irq();
    pub fn sun4m_clear_profile_irq(cpu: core::ffi::c_int);
    /* sun4m_smp.c */
    pub fn sun4m_cpu_pre_starting(arg: *mut c_void);
    pub fn sun4m_cpu_pre_online(arg: *mut c_void);
    pub fn smp4m_boot_cpus();
    pub fn smp4m_boot_one_cpu(i: core::ffi::c_int, idle: *mut task_struct) -> core::ffi::c_int;
    pub fn smp4m_smp_done();
    pub fn smp4m_cross_call_irq();
    pub fn smp4m_percpu_timer_interrupt(regs: *mut pt_regs);
    /* sun4d_irq.c */
    pub static mut sun4d_imsk_lock: spinlock_t;
    pub fn sun4d_init_IRQ();
    pub fn sun4d_request_irq(irq: core::ffi::c_uint, handler: irq_handler_t, irqflags: core::ffi::c_ulong, devname: *const u8, dev_id: *mut c_void) -> core::ffi::c_int;
    pub fn show_sun4d_interrupts(file: *mut seq_file, v: *mut c_void) -> core::ffi::c_int;
    pub fn sun4d_distribute_irqs();
    pub fn sun4d_free_irq(irq: core::ffi::c_uint, dev_id: *mut c_void);
    /* sun4d_smp.c */
    pub fn sun4d_cpu_pre_starting(arg: *mut c_void);
    pub fn sun4d_cpu_pre_online(arg: *mut c_void);
    pub fn smp4d_boot_cpus();
    pub fn smp4d_boot_one_cpu(i: core::ffi::c_int, idle: *mut task_struct) -> core::ffi::c_int;
    pub fn smp4d_smp_done();
    pub fn smp4d_cross_call_irq();
    pub fn smp4d_percpu_timer_interrupt(regs: *mut pt_regs);
    /* leon_smp.c */
    pub fn leon_cpu_pre_starting(arg: *mut c_void);
    pub fn leon_cpu_pre_online(arg: *mut c_void);
    pub fn leonsmp_ipi_interrupt();
    pub fn leon_cross_call_irq();
    /* head_32.S */
    pub static mut t_nmi: [core::ffi::c_uint; 0];
    pub static mut linux_trap_ipi15_sun4d: [core::ffi::c_uint; 0];
    pub static mut linux_trap_ipi15_sun4m: [core::ffi::c_uint; 0];
    pub static mut trapbase: [tt_entry; 0];
    pub static mut trapbase_cpu1: [tt_entry; 0];
    pub static mut trapbase_cpu2: [tt_entry; 0];
    pub static mut trapbase_cpu3: [tt_entry; 0];
    pub static mut cputypval: [u8; 0];
    /* entry.S */
    pub static mut lvl14_save: [core::ffi::c_ulong; 4];
    pub static mut real_irq_entry: [core::ffi::c_uint; 0];
    pub static mut smp4d_ticker: [core::ffi::c_uint; 0];
    pub static mut patchme_maybe_smp_msg: [core::ffi::c_uint; 0];
    /* trampoline_32.S */
    pub static mut sun4m_cpu_startup: core::ffi::c_ulong;
    pub static mut sun4d_cpu_startup: core::ffi::c_ulong;
    pub fn floppy_hardint();
    /* signal_32.c */
    pub fn do_sigreturn(regs: *mut pt_regs);
    pub fn do_rt_sigreturn(regs: *mut pt_regs);
    pub fn do_notify_resume(regs: *mut pt_regs, orig_i0: core::ffi::c_ulong, thread_info_flags: core::ffi::c_ulong);
    pub fn do_sys_sigstack(ssptr: *mut sigstack, ossptr: *mut sigstack, sp: core::ffi::c_ulong) -> core::ffi::c_int;
    pub fn syscall_trace(regs: *mut pt_regs, syscall_exit_p: core::ffi::c_int) -> core::ffi::c_int;
    pub fn kernel_unaligned_trap(regs: *mut pt_regs, insn: core::ffi::c_uint);
    pub fn user_unaligned_trap(regs: *mut pt_regs, insn: core::ffi::c_uint);
    pub fn try_to_clear_window_buffer(regs: *mut pt_regs, who: core::ffi::c_int);
    pub fn auxio_probe();
    pub fn auxio_power_probe();
    pub static mut pcic_regs: *mut c_void;
    pub fn pcic_nmi(pend: core::ffi::c_uint, regs: *mut pt_regs);
    pub fn time_init();
}

// The following symbols and types are supplied by the included kernel headers.
extern "C" {
    pub static mut kern_base: core::ffi::c_ulong;
    pub static KERNBASE: core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
