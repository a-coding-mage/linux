/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies: linux/init.h, linux/clocksource.h, linux/irqreturn.h,
 * linux/linkage.h, xen/interface/xenpmu.h, xen/xen-ops.h, asm/page.h. */

#[cfg(feature = "CONFIG_XEN_PV")]
extern "C" {
    pub static xen_failsafe_callback: [core::ffi::c_char; 0];
    pub static xen_initial_gdt: *mut core::ffi::c_void;
    pub static xen_cpu_initialized_map: cpumask_var_t;

    pub fn xen_copy_trap_info(traps: *mut trap_info);
    pub fn xen_entry_SYSENTER_compat();
    pub fn xen_entry_SYSCALL_64();
    pub fn xen_entry_SYSCALL_compat();
    pub fn xen_setup_mfn_list_list();
    pub fn xen_build_mfn_list_list();
    pub fn xen_setup_machphys_mapping();
    pub fn xen_setup_kernel_pagetable(pgd: *mut pgd_t, max_pfn: libc::c_ulong);
    pub fn xen_reserve_special_pages();
    pub fn xen_pt_check_e820();
    pub fn xen_mm_pin_all();
    pub fn xen_mm_unpin_all();
    pub fn xen_relocate_p2m();
    pub fn xen_do_remap_nonram();
    pub fn xen_add_remap_nonram(maddr: phys_addr_t, paddr: phys_addr_t, size: libc::c_ulong);
    pub fn xen_chk_is_e820_usable(start: phys_addr_t, size: phys_addr_t, component: *const core::ffi::c_char);
    pub fn xen_chk_extra_mem(pfn: libc::c_ulong) -> libc::c_ulong;
    pub fn xen_inv_extra_mem();
    pub fn xen_remap_memory();
    pub fn xen_find_free_area(size: phys_addr_t) -> phys_addr_t;
    pub fn xen_memory_setup() -> *mut core::ffi::c_char;
    pub fn xen_arch_setup();
    pub fn xen_enable_syscall();
    pub fn xen_build_dynamic_phys_to_machine();
    pub fn xen_vmalloc_p2m_tree();
    pub fn xen_init_irq_ops();
    pub fn xen_setup_vcpu_info_placement();
    pub fn xen_init_apic();
    pub fn xen_irq_enable_direct();
    pub fn xen_irq_disable_direct();
    pub fn xen_save_fl_direct() -> libc::c_ulong;
    pub fn xen_read_cr2() -> libc::c_ulong;
    pub fn xen_read_cr2_direct() -> libc::c_ulong;
    pub fn xen_iret();
    pub fn xen_force_evtchn_callback();
    pub fn xen_pv_pre_suspend();
    pub fn xen_pv_post_suspend(suspend_cancelled: libc::c_int);
    pub fn xen_start_kernel(si: *mut start_info);
    pub fn set_pte_mfn(vaddr: libc::c_ulong, pfn: libc::c_ulong, flags: pgprot_t);
    pub fn xen_init_mmu_ops();
    pub fn __xen_mc_entry(args: usize) -> multicall_space;
    pub fn xen_mc_flush();
    pub fn xen_mc_callback(fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, data: *mut core::ffi::c_void);
    pub fn xen_mc_extend_args(op: libc::c_ulong, arg_size: usize) -> multicall_space;
    pub static is_xen_pmu: bool;
    pub fn xen_pmu_irq_handler(irq: libc::c_int, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn pmu_msr_chk_emulated(msr: u32, val: *mut u64, is_read: bool) -> bool;
    pub fn pmu_apic_update(reg: u32) -> libc::c_int;
    pub fn xen_read_pmc(counter: libc::c_int) -> u64;
    pub fn xen_hypercall_pv();
}

#[cfg(not(feature = "CONFIG_XEN_PV"))]
#[inline]
pub unsafe fn xen_pv_pre_suspend() {}
#[cfg(not(feature = "CONFIG_XEN_PV"))]
#[inline]
pub unsafe fn xen_pv_post_suspend(_suspend_cancelled: libc::c_int) {}

#[repr(C)]
pub struct multicall_space {
    pub mc: *mut multicall_entry,
    pub args: *mut core::ffi::c_void,
}

extern "C" {
    pub static xen_mc_irq_flags: libc::c_ulong;
    pub static xen_vcpu_info: vcpu_info;
    pub static xen_start_info: *mut start_info;
    pub static xen_dummy_shared_info: shared_info;
    pub static HYPERVISOR_shared_info: *mut shared_info;
}

#[inline]
pub unsafe fn xen_mc_batch() {
    let mut flags: libc::c_ulong = 0;
    local_irq_save(&mut flags);
    trace_xen_mc_batch(flags);
    __this_cpu_write_xen_mc_irq_flags(flags);
}

#[inline]
pub unsafe fn xen_mc_entry(args: usize) -> multicall_space {
    xen_mc_batch();
    __xen_mc_entry(args)
}

#[inline]
pub unsafe fn xen_mc_issue(flush: bool) {
    let flags = this_cpu_read_xen_mc_irq_flags();
    trace_xen_mc_issue(flush, flags);
    if flush { xen_mc_flush(); }
    local_irq_restore(flags);
}

extern "C" {
    pub fn xen_banner();
    pub fn xen_vcpu_restore();
    pub fn xen_hvm_init_shared_info();
    pub fn xen_unplug_emulated_devices();
    pub fn xen_setup_timer(cpu: libc::c_int);
    pub fn xen_setup_runstate_info(cpu: libc::c_int);
    pub fn xen_teardown_timer(cpu: libc::c_int);
    pub fn xen_setup_cpu_clockevents();
    pub fn xen_save_time_memory_area();
    pub fn xen_restore_time_memory_area();
    pub fn xen_init_time_ops();
    pub fn xen_hvm_init_time_ops();
    pub fn xen_vcpu_stolen(vcpu: libc::c_int) -> bool;
    pub fn xen_vcpu_setup(cpu: libc::c_int);
    pub fn xen_vcpu_info_reset(cpu: libc::c_int);
    pub fn xen_add_preferred_consoles();
    pub fn xen_panic_handler_init() -> libc::c_int;
    pub fn xen_cpuhp_setup(cpu_up_prepare_cb: Option<unsafe extern "C" fn(u32) -> libc::c_int>, cpu_dead_cb: Option<unsafe extern "C" fn(u32) -> libc::c_int>) -> libc::c_int;
    pub fn xen_pin_vcpu(cpu: libc::c_int);
    pub fn xen_emergency_restart();
    pub fn xen_hvm_post_suspend(suspend_cancelled: libc::c_int);
    pub fn xen_add_extra_mem(start_pfn: libc::c_ulong, n_pfns: libc::c_ulong);
    pub fn __set_phys_to_machine(pfn: libc::c_ulong, mfn: libc::c_ulong) -> bool;
    pub fn xen_hvm_init_mmu_ops();
    pub fn xen_hypercall_hvm();
    pub fn xen_hypercall_amd();
    pub fn xen_hypercall_intel();
    pub fn xen_hypercall_setfunc();
    pub fn __xen_hypercall_setfunc() -> *mut core::ffi::c_void;
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn xen_smp_init();
    pub fn xen_hvm_smp_init();
}
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline] pub unsafe fn xen_smp_init() {}
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline] pub unsafe fn xen_hvm_smp_init() {}

#[cfg(feature = "CONFIG_PARAVIRT_SPINLOCKS")]
extern "C" {
    pub fn xen_init_spinlocks();
    pub fn xen_init_lock_cpu(cpu: libc::c_int);
    pub fn xen_uninit_lock_cpu(cpu: libc::c_int);
}
#[cfg(not(feature = "CONFIG_PARAVIRT_SPINLOCKS"))]
#[inline] pub unsafe fn xen_init_spinlocks() {}
#[cfg(not(feature = "CONFIG_PARAVIRT_SPINLOCKS"))]
#[inline] pub unsafe fn xen_init_lock_cpu(_cpu: libc::c_int) {}
#[cfg(not(feature = "CONFIG_PARAVIRT_SPINLOCKS"))]
#[inline] pub unsafe fn xen_uninit_lock_cpu(_cpu: libc::c_int) {}

#[cfg(feature = "CONFIG_XEN_DOM0")]
extern "C" { pub fn xen_init_vga(info: *const dom0_vga_console_info, size: usize, si: *mut screen_info); }
#[cfg(not(feature = "CONFIG_XEN_DOM0"))]
#[inline] pub unsafe fn xen_init_vga(_info: *const dom0_vga_console_info, _size: usize, _si: *mut screen_info) {}

#[cfg(feature = "CONFIG_XEN_EFI")]
extern "C" { pub fn xen_efi_init(boot_params: *mut boot_params); }
#[cfg(not(feature = "CONFIG_XEN_EFI"))]
#[inline] pub unsafe fn xen_efi_init(_boot_params: *mut boot_params) {}

pub const EXTRA_MEM_RATIO: libc::c_ulong = 10;

#[cfg(feature = "CONFIG_XEN_HAVE_VPMU")]
extern "C" {
    pub fn xen_pmu_init(cpu: libc::c_int);
    pub fn xen_pmu_finish(cpu: libc::c_int);
}
#[cfg(not(feature = "CONFIG_XEN_HAVE_VPMU"))]
#[inline] pub unsafe fn xen_pmu_init(_cpu: libc::c_int) {}
#[cfg(not(feature = "CONFIG_XEN_HAVE_VPMU"))]
#[inline] pub unsafe fn xen_pmu_finish(_cpu: libc::c_int) {}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn asm_cpu_bringup_and_idle();
    pub fn cpu_bringup_and_idle();
    pub fn xen_send_IPI_mask(mask: *const cpumask, vector: libc::c_int);
    pub fn xen_send_IPI_mask_allbutself(mask: *const cpumask, vector: libc::c_int);
    pub fn xen_send_IPI_allbutself(vector: libc::c_int);
    pub fn xen_send_IPI_all(vector: libc::c_int);
    pub fn xen_send_IPI_self(vector: libc::c_int);
    pub fn xen_smp_intr_init(cpu: u32) -> libc::c_int;
    pub fn xen_smp_intr_free(cpu: u32);
    pub fn xen_smp_intr_init_pv(cpu: u32) -> libc::c_int;
    pub fn xen_smp_intr_free_pv(cpu: u32);
    pub fn xen_smp_count_cpus();
    pub fn xen_smp_cpus_done(max_cpus: u32);
    pub fn xen_smp_send_reschedule(cpu: libc::c_int);
    pub fn xen_smp_send_call_function_ipi(mask: *const cpumask);
    pub fn xen_smp_send_call_function_single_ipi(cpu: libc::c_int);
    pub fn xen_cpu_bringup_again(stack: libc::c_ulong) -> !;
}

#[repr(C)]
pub struct xen_common_irq {
    pub irq: libc::c_int,
    pub name: *mut core::ffi::c_char,
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline] pub unsafe fn xen_smp_intr_init(_cpu: u32) -> libc::c_int { 0 }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline] pub unsafe fn xen_smp_intr_free(_cpu: u32) {}
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline] pub unsafe fn xen_smp_intr_init_pv(_cpu: u32) -> libc::c_int { 0 }
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline] pub unsafe fn xen_smp_intr_free_pv(_cpu: u32) {}
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline] pub unsafe fn xen_smp_count_cpus() {}

/* External types and helper operations are supplied by the translated kernel dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
