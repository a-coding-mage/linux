// SPDX-License-Identifier: GPL-2.0
/*
 * Xen SMP support
 *
 * This file implements the Xen versions of smp_ops.  SMP under Xen is
 * very straightforward.  Bringing a CPU up is simply a matter of
 * loading its initial context and setting it running.
 *
 * IPIs are handled through the Xen event mechanism.
 *
 * Because virtual CPUs can be scheduled onto any real CPU, there's no
 * useful topology information for the kernel to make use of.  As a
 * result, all CPUs are treated as if they're single-core and
 * single-threaded.
 */

// C headers and symbols are supplied by the surrounding kernel translation.

pub static mut xen_cpu_initialized_map: cpumask_var_t = core::ptr::null_mut();

static mut xen_irq_work: PerCpu<xen_common_irq> = PerCpu::new(xen_common_irq { irq: -1, name: core::ptr::null_mut() });
static mut xen_pmu_irq: PerCpu<xen_common_irq> = PerCpu::new(xen_common_irq { irq: -1, name: core::ptr::null_mut() });

extern "C" {
    fn xen_irq_work_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}

unsafe fn cpu_bringup() {
    let cpu: c_int;

    cr4_init();
    cpuhp_ap_sync_alive();
    cpu_init();
    fpu__init_cpu();
    touch_softlockup_watchdog();

    /* PVH runs in ring 0 and allows us to do native syscalls. Yay! */
    if !xen_feature(XENFEAT_supervisor_mode_kernel) {
        xen_enable_syscall();
    }

    cpu = smp_processor_id();
    identify_secondary_cpu(cpu);
    set_cpu_sibling_map(cpu);

    speculative_store_bypass_ht_init();
    xen_setup_cpu_clockevents();
    notify_cpu_starting(cpu);
    set_cpu_online(cpu, true);
    smp_mb();

    /* We can take interrupts now: we're officially "up". */
    local_irq_enable();
}

#[no_mangle]
pub unsafe extern "C" fn cpu_bringup_and_idle() -> ! {
    cpu_bringup();
    cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
}

pub unsafe extern "C" fn xen_smp_intr_free_pv(cpu: c_uint) {
    kfree(per_cpu!(xen_irq_work, cpu).name);
    per_cpu_mut!(xen_irq_work, cpu).name = core::ptr::null_mut();
    if per_cpu!(xen_irq_work, cpu).irq >= 0 {
        unbind_from_irqhandler(per_cpu!(xen_irq_work, cpu).irq, core::ptr::null_mut());
        per_cpu_mut!(xen_irq_work, cpu).irq = -1;
    }

    kfree(per_cpu!(xen_pmu_irq, cpu).name);
    per_cpu_mut!(xen_pmu_irq, cpu).name = core::ptr::null_mut();
    if per_cpu!(xen_pmu_irq, cpu).irq >= 0 {
        unbind_from_irqhandler(per_cpu!(xen_pmu_irq, cpu).irq, core::ptr::null_mut());
        per_cpu_mut!(xen_pmu_irq, cpu).irq = -1;
    }
}

pub unsafe extern "C" fn xen_smp_intr_init_pv(cpu: c_uint) -> c_int {
    let mut rc: c_int;
    let callfunc_name: *mut c_char;
    let mut pmu_name: *mut c_char;

    callfunc_name = kasprintf(GFP_KERNEL, c"irqwork%d".as_ptr(), cpu);
    per_cpu_mut!(xen_irq_work, cpu).name = callfunc_name;
    rc = bind_ipi_to_irqhandler(XEN_IRQ_WORK_VECTOR, cpu, xen_irq_work_interrupt,
        IRQF_PERCPU | IRQF_NOBALANCING, callfunc_name, core::ptr::null_mut());
    if rc < 0 { goto_fail(); return rc; }
    per_cpu_mut!(xen_irq_work, cpu).irq = rc;

    if is_xen_pmu {
        pmu_name = kasprintf(GFP_KERNEL, c"pmu%d".as_ptr(), cpu);
        per_cpu_mut!(xen_pmu_irq, cpu).name = pmu_name;
        rc = bind_virq_to_irqhandler(VIRQ_XENPMU, cpu, xen_pmu_irq_handler,
            IRQF_PERCPU | IRQF_NOBALANCING, pmu_name, core::ptr::null_mut());
        if rc < 0 { xen_smp_intr_free_pv(cpu); return rc; }
        per_cpu_mut!(xen_pmu_irq, cpu).irq = rc;
    }
    return 0;

    unsafe fn goto_fail() {}
}

unsafe fn xen_pv_smp_config() {
    let mut apicid: u32 = 0;
    topology_register_boot_apic(apicid);
    for i in 0..nr_cpu_ids {
        topology_register_apic(apicid, CPU_ACPIID_INVALID, true);
        apicid += 1;
        let _ = i;
    }
    /* Pretend to be a proper enumerated system */
    smp_found_config = 1;
}

unsafe fn xen_pv_smp_prepare_boot_cpu() {
    BUG_ON(smp_processor_id() != 0);
    native_smp_prepare_boot_cpu();
    if !xen_feature(XENFEAT_writable_page_tables) {
        /* We've switched to the "real" per-cpu gdt, so make
         * sure the old memory can be recycled. */
        make_lowmem_page_readwrite(xen_initial_gdt);
    }
    xen_setup_vcpu_info_placement();
    /*
     * The alternative logic (which patches the unlock/lock) runs before
     * the smp bootup up code is activated. Hence we need to set this up
     * the core kernel is being patched. Otherwise we will have only
     * modules patched but not core code.
     */
    xen_init_spinlocks();
}

unsafe fn xen_pv_smp_prepare_cpus(max_cpus: c_uint) {
    let mut cpu: c_uint;
    if ioapic_is_disabled {
        let m = if max_cpus == 0 { "The nosmp parameter is incompatible with Xen; use Xen dom0_max_vcpus=1 parameter" } else { "The noapic parameter is incompatible with Xen" };
        xen_raw_printk(m.as_ptr() as *const c_char);
        panic(m.as_ptr() as *const c_char);
    }
    xen_init_lock_cpu(0);
    smp_prepare_cpus_common();
    speculative_store_bypass_ht_init();
    xen_pmu_init(0);
    if xen_smp_intr_init(0) != 0 || xen_smp_intr_init_pv(0) != 0 { BUG(); }
    if !alloc_cpumask_var(&mut xen_cpu_initialized_map, GFP_KERNEL) { panic(c"could not allocate xen_cpu_initialized_map\0".as_ptr()); }
    cpumask_copy(xen_cpu_initialized_map, cpumask_of(0));
    while num_possible_cpus() > 1 && num_possible_cpus() > max_cpus {
        cpu = nr_cpu_ids - 1;
        while !cpu_possible(cpu) { cpu -= 1; }
        set_cpu_possible(cpu, false);
    }
    for_each_possible_cpu!(cpu, { set_cpu_present(cpu, true); });
}

unsafe fn cpu_initialize_context(cpu: c_uint, idle: *mut task_struct) -> c_int {
    if cpumask_test_and_set_cpu(cpu, xen_cpu_initialized_map) { return 0; }
    let ctxt = kzalloc_obj::<vcpu_guest_context>();
    if ctxt.is_null() {
        cpumask_clear_cpu(cpu, xen_cpu_initialized_map);
        return -ENOMEM;
    }
    let gdt = get_cpu_gdt_rw(cpu);
    (*ctxt).user_regs.eip = asm_cpu_bringup_and_idle as usize as c_ulong;
    (*ctxt).flags = VGCF_IN_KERNEL;
    (*ctxt).user_regs.eflags = 0x1000;
    (*ctxt).user_regs.ds = __USER_DS; (*ctxt).user_regs.es = __USER_DS;
    (*ctxt).user_regs.ss = __KERNEL_DS; (*ctxt).user_regs.cs = __KERNEL_CS;
    (*ctxt).user_regs.esp = task_pt_regs(idle) as usize as c_ulong;
    xen_copy_trap_info((*ctxt).trap_ctxt.as_mut_ptr());
    BUG_ON((gdt as usize & !PAGE_MASK as usize) != 0);
    let gdt_mfn = arbitrary_virt_to_mfn(gdt);
    make_lowmem_page_readonly(gdt);
    make_lowmem_page_readonly(mfn_to_virt(gdt_mfn));
    (*ctxt).gdt_frames[0] = gdt_mfn; (*ctxt).gdt_ents = GDT_ENTRIES;
    (*ctxt).kernel_ss = __KERNEL_DS; (*ctxt).kernel_sp = task_top_of_stack(idle);
    (*ctxt).gs_base_kernel = per_cpu_offset(cpu);
    (*ctxt).event_callback_eip = xen_asm_exc_xen_hypervisor_callback as usize as c_ulong;
    (*ctxt).failsafe_callback_eip = xen_failsafe_callback as usize as c_ulong;
    per_cpu_mut!(xen_cr3, cpu) = __pa(swapper_pg_dir);
    (*ctxt).ctrlreg[3] = xen_pfn_to_cr3(virt_to_gfn(swapper_pg_dir));
    if HYPERVISOR_vcpu_op(VCPUOP_initialise, xen_vcpu_nr(cpu), ctxt) != 0 { BUG(); }
    kfree(ctxt);
    0
}

unsafe fn xen_pv_kick_ap(cpu: c_uint, idle: *mut task_struct) -> c_int {
    let mut rc = common_cpu_up(cpu, idle); if rc != 0 { return rc; }
    xen_setup_runstate_info(cpu);
    (*per_cpu!(xen_vcpu, cpu)).evtchn_upcall_mask = 1;
    rc = cpu_initialize_context(cpu, idle); if rc != 0 { return rc; }
    xen_pmu_init(cpu);
    BUG_ON(HYPERVISOR_vcpu_op(VCPUOP_up, xen_vcpu_nr(cpu), core::ptr::null_mut()) != 0);
    0
}

unsafe fn xen_pv_poll_sync_state() { HYPERVISOR_sched_op(SCHEDOP_yield, core::ptr::null_mut()); }

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn xen_pv_cpu_disable() -> c_int { let cpu = smp_processor_id(); if cpu == 0 { return -EBUSY; } cpu_disable_common(); load_cr3(swapper_pg_dir); 0 }
#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn xen_pv_cpu_die(cpu: c_uint) { while HYPERVISOR_vcpu_op(VCPUOP_is_up, xen_vcpu_nr(cpu), core::ptr::null_mut()) != 0 { __set_current_state(TASK_UNINTERRUPTIBLE); schedule_timeout(HZ / 10); } }
#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn xen_pv_cleanup_dead_cpu(cpu: c_uint) { xen_smp_intr_free(cpu); xen_uninit_lock_cpu(cpu); xen_teardown_timer(cpu); xen_pmu_finish(cpu); }
#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn xen_pv_play_dead() -> ! { play_dead_common(); HYPERVISOR_vcpu_op(VCPUOP_down, xen_vcpu_nr(smp_processor_id()), core::ptr::null_mut()); xen_cpu_bringup_again(task_pt_regs(current) as usize as c_ulong); BUG(); }

#[cfg(not(CONFIG_HOTPLUG_CPU))]
unsafe fn xen_pv_cpu_disable() -> c_int { -ENOSYS }
#[cfg(not(CONFIG_HOTPLUG_CPU))]
unsafe fn xen_pv_cpu_die(_: c_uint) { BUG(); }
#[cfg(not(CONFIG_HOTPLUG_CPU))]
unsafe fn xen_pv_cleanup_dead_cpu(_: c_uint) { BUG(); }
#[cfg(not(CONFIG_HOTPLUG_CPU))]
unsafe fn xen_pv_play_dead() -> ! { BUG(); }

unsafe fn stop_self(_: *mut c_void) {
    let cpu = smp_processor_id();
    /* make sure we're not pinning something down */
    load_cr3(swapper_pg_dir);
    /* should set up a minimal gdt */
    set_cpu_online(cpu, false);
    HYPERVISOR_vcpu_op(VCPUOP_down, xen_vcpu_nr(cpu), core::ptr::null_mut());
    BUG();
}

unsafe fn xen_pv_stop_other_cpus(wait: c_int) { smp_call_function(stop_self, core::ptr::null_mut(), wait); }

unsafe fn xen_irq_work_interrupt_impl(_: c_int, _: *mut c_void) -> irqreturn_t {
    irq_work_run(); inc_irq_stat(IRQ_WORK); IRQ_HANDLED
}

pub unsafe extern "C" fn xen_smp_count_cpus() {
    let mut cpus: c_uint = 0;
    while cpus < nr_cpu_ids { if HYPERVISOR_vcpu_op(VCPUOP_is_up, cpus, core::ptr::null_mut()) < 0 { break; } cpus += 1; }
    pr_info(c"Xen PV: Detected %u vCPUS\n\0".as_ptr(), cpus);
    if cpus < nr_cpu_ids { set_nr_cpu_ids(cpus); }
}

static xen_smp_ops: smp_ops = smp_ops {
    smp_prepare_boot_cpu: Some(xen_pv_smp_prepare_boot_cpu),
    smp_prepare_cpus: Some(xen_pv_smp_prepare_cpus), smp_cpus_done: Some(xen_smp_cpus_done),
    kick_ap_alive: Some(xen_pv_kick_ap), cpu_die: Some(xen_pv_cpu_die),
    cleanup_dead_cpu: Some(xen_pv_cleanup_dead_cpu), poll_sync_state: Some(xen_pv_poll_sync_state),
    cpu_disable: Some(xen_pv_cpu_disable), play_dead: Some(xen_pv_play_dead),
    stop_other_cpus: Some(xen_pv_stop_other_cpus), smp_send_reschedule: Some(xen_smp_send_reschedule),
    send_call_func_ipi: Some(xen_smp_send_call_function_ipi),
    send_call_func_single_ipi: Some(xen_smp_send_call_function_single_ipi),
};

pub unsafe extern "C" fn xen_smp_init() {
    smp_ops = xen_smp_ops;
    x86_init.mpparse.find_mptable = x86_init_noop;
    x86_init.mpparse.early_parse_smp_cfg = x86_init_noop;
    if xen_initial_domain() { x86_init.mpparse.parse_smp_cfg = x86_init_noop; }
    else { x86_init.mpparse.parse_smp_cfg = xen_pv_smp_config; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
