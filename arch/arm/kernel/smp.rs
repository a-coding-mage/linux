// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of arm/kernel/smp.c. */

// Linux and ARM header dependencies are supplied by the surrounding kernel.

static mut SECONDARY_DATA: secondary_data = secondary_data { ..unsafe { core::mem::zeroed() } };

#[repr(u32)]
enum ipi_msg_type { IPI_WAKEUP, IPI_TIMER, IPI_RESCHEDULE, IPI_CALL_FUNC,
    IPI_CPU_STOP, IPI_IRQ_WORK, IPI_COMPLETION, NR_IPI,
    IPI_CPU_BACKTRACE = NR_IPI, MAX_IPI }

static mut ipi_irq_base: i32 = 0;
static mut nr_ipi: i32 = NR_IPI as i32;
static mut ipi_desc: [*mut irq_desc; MAX_IPI as usize] = [core::ptr::null_mut(); MAX_IPI as usize];
static mut smp_ops: smp_operations = unsafe { core::mem::zeroed() };
static mut cpu_running: completion = unsafe { core::mem::zeroed() };

unsafe fn ipi_setup(cpu: i32) {
    if WARN_ON_ONCE(ipi_irq_base == 0) { return; }
    for i in 0..nr_ipi { enable_percpu_irq(ipi_irq_base + i, 0); }
}

pub unsafe fn smp_set_ops(ops: *const smp_operations) { if !ops.is_null() { smp_ops = *ops; } }

unsafe fn get_arch_pgd(pgd: *mut pgd_t) -> c_ulong {
    // CONFIG_ARM_LPAE selects __phys_to_pfn(virt_to_phys(pgd)); otherwise virt_to_phys(pgd).
    #[cfg(CONFIG_ARM_LPAE)] { return __phys_to_pfn(virt_to_phys(pgd)); }
    #[cfg(not(CONFIG_ARM_LPAE))] { virt_to_phys(pgd) }
}

unsafe fn secondary_biglittle_prepare(cpu: u32) -> i32 {
    // CONFIG_BIG_LITTLE && CONFIG_HARDEN_BRANCH_PREDICTOR supplies the vtable path.
    0
}
unsafe fn secondary_biglittle_init() {}

pub unsafe fn __cpu_up(cpu: u32, idle: *mut task_struct) -> i32 {
    if smp_ops.smp_boot_secondary.is_none() { return -ENOSYS; }
    let ret = secondary_biglittle_prepare(cpu); if ret != 0 { return ret; }
    SECONDARY_DATA.stack = task_stack_page(idle).add(THREAD_START_SP as usize);
    // CONFIG_ARM_MPU: SECONDARY_DATA.mpu_rgn_info = &mut mpu_rgn_info;
    // CONFIG_MMU: SECONDARY_DATA.pgdir = virt_to_phys(idmap_pgd); SECONDARY_DATA.swapper_pg_dir = get_arch_pgd(swapper_pg_dir);
    SECONDARY_DATA.task = idle;
    sync_cache_w(&mut SECONDARY_DATA);
    let mut ret = (smp_ops.smp_boot_secondary.unwrap())(cpu, idle);
    if ret == 0 {
        wait_for_completion_timeout(&mut cpu_running, msecs_to_jiffies(1000));
        if !cpu_online(cpu) { pr_crit!("CPU{}: failed to come online\n", cpu); ret = -EIO; }
    } else { pr_err!("CPU{}: failed to boot: {}\n", cpu, ret); }
    core::ptr::write_bytes(&mut SECONDARY_DATA as *mut _, 0, 1); ret
}

pub unsafe fn smp_init_cpus() { if let Some(f) = smp_ops.smp_init_cpus { f(); } }
pub unsafe fn platform_can_secondary_boot() -> i32 { smp_ops.smp_boot_secondary.is_some() as i32 }
pub unsafe fn platform_can_cpu_hotplug() -> i32 {
    // CONFIG_HOTPLUG_CPU
    if smp_ops.cpu_kill.is_some() { 1 } else { 0 }
}

unsafe fn platform_cpu_kill(cpu: u32) -> i32 { smp_ops.cpu_kill.map_or(1, |f| f(cpu)) }
unsafe fn platform_cpu_disable(cpu: u32) -> i32 { smp_ops.cpu_disable.map_or(0, |f| f(cpu)) }
pub unsafe fn platform_can_hotplug_cpu(cpu: u32) -> i32 {
    if smp_ops.cpu_die.is_none() { return 0; }
    smp_ops.cpu_can_disable.map_or((cpu != 0) as i32, |f| f(cpu))
}
unsafe fn ipi_teardown(cpu: i32) { if WARN_ON_ONCE(ipi_irq_base == 0) { return; } for i in 0..nr_ipi { disable_percpu_irq(ipi_irq_base + i); } }

pub unsafe fn __cpu_disable() -> i32 {
    let cpu = smp_processor_id(); let ret = platform_cpu_disable(cpu); if ret != 0 { return ret; }
    // CONFIG_GENERIC_ARCH_TOPOLOGY: remove_cpu_topology(cpu);
    set_cpu_online(cpu, false); ipi_teardown(cpu); irq_migrate_all_off_this_cpu();
    flush_cache_louis(); local_flush_tlb_all(); 0
}
pub unsafe fn arch_cpuhp_cleanup_dead_cpu(cpu: u32) { pr_debug!("CPU{}: shutdown\n", cpu); clear_tasks_mm_cpumask(cpu); if platform_cpu_kill(cpu) == 0 { pr_err!("CPU{}: unable to kill\n", cpu); } }
pub unsafe fn arch_cpu_idle_dead() -> ! {
    let cpu = smp_processor_id(); idle_task_exit(); local_irq_disable(); flush_cache_louis(); cpuhp_ap_report_dead(); flush_cache_louis();
    if let Some(f) = smp_ops.cpu_die { f(cpu); }
    pr_warn!("CPU{}: smp_ops.cpu_die() returned, trying to resuscitate\n", cpu);
    // Assembly restores the idle stack and branches to secondary_start_kernel.
    core::hint::unreachable_unchecked()
}

unsafe fn smp_store_cpu_info(cpuid: u32) { let cpu_info = &mut per_cpu(cpu_data, cpuid); cpu_info.loops_per_jiffy = loops_per_jiffy; cpu_info.cpuid = read_cpuid_id(); store_cpu_topology(cpuid); check_cpu_icache_size(cpuid); }
unsafe fn set_current(cur: *mut task_struct) { asm!("mcr p15, 0, {0}, c13, c0, 3", in(reg) cur, options(nostack, preserves_flags)); }

pub unsafe fn secondary_start_kernel(task: *mut task_struct) {
    let mm = &mut init_mm; set_current(task); secondary_biglittle_init(); cpu_switch_mm(mm.pgd, mm); local_flush_bp_all(); enter_lazy_tlb(mm, current); local_flush_tlb_all();
    let cpu = smp_processor_id(); mmgrab(mm); (*current).active_mm = mm; cpumask_set_cpu(cpu, mm_cpumask(mm)); cpu_init();
    // !CONFIG_MMU: setup_vectors_base();
    pr_debug!("CPU{}: Booted secondary processor\n", cpu); trace_hardirqs_off(); if let Some(f) = smp_ops.smp_secondary_init { f(cpu); }
    notify_cpu_starting(cpu); ipi_setup(cpu); calibrate_delay(); smp_store_cpu_info(cpu); set_cpu_online(cpu, true); check_other_bugs(); complete(&mut cpu_running); local_irq_enable(); local_fiq_enable(); local_abt_enable(); cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
}

pub unsafe fn smp_cpus_done(_max_cpus: u32) { let mut bogosum = 0u64; for_each_online_cpu!(cpu, { bogosum += per_cpu(cpu_data, cpu).loops_per_jiffy; }); printk!(KERN_INFO "SMP: Total of %d processors activated (%lu.%02lu BogoMIPS).\n", num_online_cpus(), bogosum / (500000 / HZ), (bogosum / (5000 / HZ)) % 100); hyp_mode_check(); }
pub unsafe fn smp_prepare_boot_cpu() { set_my_cpu_offset(per_cpu_offset(smp_processor_id())); }
pub unsafe fn smp_prepare_cpus(mut max_cpus: u32) { let ncores = num_possible_cpus(); init_cpu_topology(); smp_store_cpu_info(smp_processor_id()); if max_cpus > ncores { max_cpus = ncores; } if ncores > 1 && max_cpus != 0 { init_cpu_present(cpu_possible_mask); if let Some(f) = smp_ops.smp_prepare_cpus { f(max_cpus); } } }

static ipi_types: [&str; NR_IPI as usize] = ["CPU wakeup interrupts", "Timer broadcast interrupts", "Rescheduling interrupts", "Function call interrupts", "CPU stop interrupts", "IRQ work interrupts", "completion interrupts"];
unsafe fn smp_cross_call(target: *const cpumask, ipinr: u32) { trace_ipi_raise(target, ipi_types[ipinr as usize]); __ipi_send_mask(ipi_desc[ipinr as usize], target); }
pub unsafe fn show_ipi_list(p: *mut seq_file, prec: i32) { for i in 0..NR_IPI as usize { if ipi_desc[i].is_null() { continue; } seq_printf!(p, "{:width$}{}:", "IPI", i, width=(prec-1) as usize); for_each_online_cpu!(cpu, { seq_printf!(p, "{:10} ", irq_desc_kstat_cpu(ipi_desc[i], cpu)); }); seq_printf!(p, " {}\n", ipi_types[i]); } }
pub unsafe fn arch_send_call_function_ipi_mask(mask: *const cpumask) { smp_cross_call(mask, IPI_CALL_FUNC); }
pub unsafe fn arch_send_wakeup_ipi_mask(mask: *const cpumask) { smp_cross_call(mask, IPI_WAKEUP); }
pub unsafe fn arch_send_call_function_single_ipi(cpu: i32) { smp_cross_call(cpumask_of(cpu), IPI_CALL_FUNC); }
pub unsafe fn arch_smp_send_reschedule(cpu: i32) { smp_cross_call(cpumask_of(cpu), IPI_RESCHEDULE); }

unsafe fn ipi_cpu_stop(cpu: u32) -> ! { local_fiq_disable(); if system_state <= SYSTEM_RUNNING { raw_spin_lock(&mut stop_lock); pr_crit!("CPU{}: stopping\n", cpu); dump_stack(); raw_spin_unlock(&mut stop_lock); } set_cpu_online(cpu, false); loop { cpu_relax(); wfe(); } }
static mut stop_lock: raw_spinlock_t = unsafe { core::mem::zeroed() };
static mut cpu_completion: *mut completion = core::ptr::null_mut();
pub unsafe fn register_ipi_completion(completion: *mut completion, cpu: i32) -> i32 { per_cpu(cpu_completion, cpu) = completion; IPI_COMPLETION as i32 }
unsafe fn ipi_complete(cpu: u32) { complete(per_cpu(cpu_completion, cpu)); }
unsafe fn do_handle_IPI(ipinr: i32) { let cpu = smp_processor_id(); if ipinr >= 0 && ipinr < NR_IPI as i32 { trace_ipi_entry(ipi_types[ipinr as usize]); } match ipinr as u32 {
    IPI_WAKEUP => {}, IPI_TIMER => { tick_receive_broadcast(); }, IPI_RESCHEDULE => scheduler_ipi(), IPI_CALL_FUNC => generic_smp_call_function_interrupt(), IPI_CPU_STOP => ipi_cpu_stop(cpu), IPI_IRQ_WORK => irq_work_run(), IPI_COMPLETION => ipi_complete(cpu), IPI_CPU_BACKTRACE => { printk_deferred_enter(); nmi_cpu_backtrace(get_irq_regs()); printk_deferred_exit(); }, _ => pr_crit!("CPU{}: Unknown IPI message 0x{:x}\n", cpu, ipinr) }
    if ipinr >= 0 && ipinr < NR_IPI as i32 { trace_ipi_exit(ipi_types[ipinr as usize]); }
}
pub unsafe fn handle_IPI(ipinr: i32, regs: *mut pt_regs) { let old = set_irq_regs(regs); irq_enter(); do_handle_IPI(ipinr); irq_exit(); set_irq_regs(old); }
unsafe fn ipi_handler(irq: i32, _data: *mut core::ffi::c_void) -> irqreturn_t { do_handle_IPI(irq - ipi_irq_base); IRQ_HANDLED }
pub unsafe fn set_smp_ipi_range(ipi_base: i32, n: i32) { WARN_ON(n < MAX_IPI as i32); nr_ipi = core::cmp::min(n, MAX_IPI as i32); for i in 0..nr_ipi { let err = request_percpu_irq(ipi_base+i, ipi_handler, "IPI", &mut irq_stat); WARN_ON(err != 0); ipi_desc[i as usize] = irq_to_desc(ipi_base+i); irq_set_status_flags(ipi_base+i, IRQ_HIDDEN); } ipi_irq_base=ipi_base; ipi_setup(smp_processor_id()); }
pub unsafe fn smp_send_stop() { let mut timeout = USEC_PER_SEC; let mut mask = core::mem::zeroed::<cpumask>(); cpumask_copy(&mut mask, cpu_online_mask); cpumask_clear_cpu(smp_processor_id(), &mut mask); if !cpumask_empty(&mask) { smp_cross_call(&mask, IPI_CPU_STOP); } while num_online_cpus() > 1 && timeout != 0 { timeout -= 1; udelay(1); } if num_online_cpus() > 1 { pr_warn!("SMP: failed to stop secondary CPUs\n"); } }
pub unsafe fn panic_smp_self_stop() -> ! { pr_debug!("CPU {} will stop doing anything useful since another CPU has paniced\n", smp_processor_id()); set_cpu_online(smp_processor_id(), false); loop { cpu_relax(); } }
unsafe fn raise_nmi(mask: *mut cpumask_t) { __ipi_send_mask(ipi_desc[IPI_CPU_BACKTRACE as usize], mask); }
pub unsafe fn arch_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: i32) { nmi_trigger_cpumask_backtrace(mask, exclude_cpu, raise_nmi); }

pub unsafe fn arch_irq_work_raise() { if arch_irq_work_has_interrupt() { smp_cross_call(cpumask_of(smp_processor_id()), IPI_IRQ_WORK); } }
pub unsafe fn tick_broadcast(mask: *const cpumask) { smp_cross_call(mask, IPI_TIMER); }

static mut l_p_j_ref: usize = 0;
static mut l_p_j_ref_freq: usize = 0;
static mut global_l_p_j_ref: usize = 0;
static mut global_l_p_j_ref_freq: usize = 0;
unsafe fn cpufreq_callback(_nb: *mut notifier_block, val: usize, data: *mut core::ffi::c_void) -> i32 {
    let freq = &mut *(data as *mut cpufreq_freqs); let cpus = (*freq.policy).cpus; let first = cpumask_first(cpus); if freq.flags & CPUFREQ_CONST_LOOPS != 0 { return NOTIFY_OK; }
    if per_cpu(l_p_j_ref, first) == 0 { for_each_cpu!(cpu, cpus, { per_cpu(l_p_j_ref, cpu) = per_cpu(cpu_data, cpu).loops_per_jiffy; per_cpu(l_p_j_ref_freq, cpu) = freq.old; }); if global_l_p_j_ref == 0 { global_l_p_j_ref = loops_per_jiffy; global_l_p_j_ref_freq = freq.old; } }
    if (val == CPUFREQ_PRECHANGE && freq.old < freq.new) || (val == CPUFREQ_POSTCHANGE && freq.old > freq.new) { loops_per_jiffy = cpufreq_scale(global_l_p_j_ref, global_l_p_j_ref_freq, freq.new); let lpj = cpufreq_scale(per_cpu(l_p_j_ref, first), per_cpu(l_p_j_ref_freq, first), freq.new); for_each_cpu!(cpu, cpus, { per_cpu(cpu_data, cpu).loops_per_jiffy = lpj; }); }
    NOTIFY_OK
}
static mut cpufreq_notifier: notifier_block = notifier_block { notifier_call: Some(cpufreq_callback) };
unsafe fn register_cpufreq_notifier() -> i32 { cpufreq_register_notifier(&mut cpufreq_notifier, CPUFREQ_TRANSITION_NOTIFIER) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
