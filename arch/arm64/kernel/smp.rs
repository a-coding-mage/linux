// SPDX-License-Identifier: GPL-2.0-only
/*
 * SMP initialisation and IPI support
 * Based on arch/arm/kernel/smp.c
 *
 * Copyright (C) 2012 ARM Ltd.
 */

// C headers and configuration-dependent declarations are supplied by the
// surrounding kernel translation unit.

#[repr(C)]
pub struct secondary_data { pub task: *mut task_struct, pub status: long }
extern "C" {
    static mut secondary_data: secondary_data;
    static mut cpus_stuck_in_kernel: i32;
    static mut ipi_irq_base: i32;
    static mut nr_ipi: i32;
    static mut percpu_ipi_descs: bool;
    static mut crash_stop: bool;
    static mut irq_err_count: c_ulong;
}

#[repr(C)] pub struct ipi_descs { pub descs: [*mut irq_desc; MAX_IPI] }
static mut pcpu_ipi_desc: ipi_descs = ipi_descs { descs: [core::ptr::null_mut(); MAX_IPI] };

unsafe fn get_ipi_desc(cpu: i32, ipi: usize) -> *mut irq_desc {
    pcpu_ipi_desc.descs[ipi]
}

unsafe fn boot_secondary(cpu: u32, idle: *mut task_struct) -> i32 {
    let ops = get_cpu_ops(cpu);
    if !ops.is_null() && (*ops).cpu_boot.is_some() { return ((*ops).cpu_boot.unwrap())(cpu); }
    -EOPNOTSUPP
}

static mut cpu_running: completion = completion::new();

pub unsafe fn __cpu_up(cpu: u32, idle: *mut task_struct) -> i32 {
    secondary_data.task = idle;
    update_cpu_boot_status(CPU_MMU_OFF);
    let ret = boot_secondary(cpu, idle);
    if ret != 0 { if ret != -EPERM { pr_err!("CPU%u: failed to boot: %d\n", cpu, ret); } return ret; }
    wait_for_completion_timeout(&mut cpu_running, msecs_to_jiffies(5000));
    if cpu_online(cpu) { return 0; }
    pr_crit!("CPU%u: failed to come online\n", cpu);
    secondary_data.task = core::ptr::null_mut();
    let mut status = READ_ONCE(secondary_data.status);
    if status == CPU_MMU_OFF { status = READ_ONCE(__early_cpu_boot_status); }
    match status & CPU_BOOT_STATUS_MASK {
        CPU_KILL_ME => { if op_cpu_kill(cpu) == 0 { pr_crit!("CPU%u: died during early boot\n", cpu); } else { pr_crit!("CPU%u: may not have shut down cleanly\n", cpu); } }
        CPU_STUCK_IN_KERNEL => { pr_crit!("CPU%u: is stuck in kernel\n", cpu); if status & CPU_STUCK_REASON_52_BIT_VA != 0 { pr_crit!("CPU%u: does not support 52-bit VAs\n", cpu); } cpus_stuck_in_kernel += 1; }
        CPU_PANIC_KERNEL => panic!("CPU%u detected unsupported configuration\n", cpu),
        _ => { pr_err!("CPU%u: failed in unknown state : 0x%lx\n", cpu, status); cpus_stuck_in_kernel += 1; }
    }
    -EIO
}

unsafe fn init_gic_priority_masking() { if WARN_ON(!gic_enable_sre()) { return; } let cpuflags = read_sysreg(daif); WARN_ON(cpuflags & PSR_I_BIT == 0); WARN_ON(cpuflags & PSR_F_BIT == 0); gic_write_pmr(GIC_PRIO_IRQON | GIC_PRIO_PSR_I_SET); }

pub unsafe fn secondary_start_kernel() -> ! {
    let mpidr = read_cpuid_mpidr() & MPIDR_HWID_BITMASK; let mm = &mut init_mm; let cpu = smp_processor_id();
    mmgrab(mm); (*current).active_mm = mm; cpu_uninstall_idmap(); if system_uses_irq_prio_masking() { init_gic_priority_masking(); }
    rcutree_report_cpu_starting(cpu); trace_hardirqs_off(); check_local_cpu_capabilities();
    let ops = get_cpu_ops(cpu); if !ops.is_null() && (*ops).cpu_postboot.is_some() { ((*ops).cpu_postboot.unwrap())(); }
    cpuinfo_store_cpu(); store_cpu_topology(cpu); notify_cpu_starting(cpu); ipi_setup(cpu); numa_add_cpu(cpu);
    pr_info!("CPU%u: Booted secondary processor 0x%010lx [0x%08x]\n", cpu, mpidr, read_cpuid_id());
    update_cpu_boot_status(CPU_BOOT_SUCCESS); set_cpu_online(cpu, true); complete(&mut cpu_running); local_daif_restore(DAIF_PROCCTX); cpu_startup_entry(CPUHP_AP_ONLINE_IDLE)
}

unsafe fn __cpu_try_die(cpu: i32) { let ops = get_cpu_ops(cpu as u32); if !ops.is_null() && (*ops).cpu_die.is_some() { ((*ops).cpu_die.unwrap())(cpu as u32); } }

pub unsafe fn cpu_die_early() -> ! { let cpu=smp_processor_id(); pr_crit!("CPU%d: will not boot\n",cpu); set_cpu_present(cpu,0); rcutree_report_cpu_dead(); if IS_ENABLED(CONFIG_HOTPLUG_CPU) { update_cpu_boot_status(CPU_KILL_ME); __cpu_try_die(cpu); } update_cpu_boot_status(CPU_STUCK_IN_KERNEL); cpu_park_loop() }

unsafe fn is_mpidr_duplicate(cpu:u32, hwid:u64)->bool { let mut i=1; while i<cpu && i<NR_CPUS as u32 { if cpu_logical_map(i)==hwid{return true;} i+=1;} false }

pub unsafe fn smp_init_cpus() { if acpi_disabled { of_parse_and_init_cpus(); } else { acpi_parse_and_init_cpus(); } if !bootcpu_valid { pr_err!("missing boot CPU MPIDR, not enabling secondaries\n"); return; } if !setup_max_cpus { return; } let mut i=1; while i<nr_cpu_ids { if cpu_logical_map(i)!=INVALID_HWID && smp_cpu_setup(i)!=0 { set_cpu_logical_map(i,INVALID_HWID); } i+=1; } }

unsafe fn smp_cpu_setup(cpu:i32)->i32 { if init_cpu_ops(cpu)!=0{return -ENODEV;} let ops=get_cpu_ops(cpu as u32); if ops.is_null() || ((*ops).cpu_init.unwrap())(cpu as u32)!=0{return -ENODEV;} set_cpu_possible(cpu,true);0 }
static mut bootcpu_valid: bool=false; static mut cpu_count:u32=1;

pub unsafe fn arch_send_call_function_ipi_mask(mask:*const cpumask){smp_cross_call(mask,IPI_CALL_FUNC)}
pub unsafe fn arch_send_call_function_single_ipi(cpu:i32){smp_cross_call(cpumask_of(cpu),IPI_CALL_FUNC)}
pub unsafe fn arch_smp_send_reschedule(cpu:i32){smp_cross_call(cpumask_of(cpu),IPI_RESCHEDULE)}
pub unsafe fn tick_broadcast(mask:*const cpumask){smp_cross_call(mask,IPI_TIMER)}

unsafe fn ipi_should_be_nmi(ipi:i32)->bool { if !system_uses_irq_prio_masking(){return false;} matches!(ipi, IPI_CPU_STOP_NMI|IPI_CPU_BACKTRACE|IPI_KGDB_ROUNDUP) }
unsafe fn smp_cross_call(target:*const cpumask, ipinr:i32){trace_ipi_raise(target,ipi_types[ipinr as usize]); arm64_send_ipi(target,ipinr)}
unsafe fn arm64_send_ipi(mask:*const cpumask,nr:i32){ if !percpu_ipi_descs { __ipi_send_mask(get_ipi_desc(0,nr as usize),mask); } else { for_each_cpu!(cpu,mask){__ipi_send_single(get_ipi_desc(cpu,nr as usize),cpu);} } }

pub unsafe fn panic_smp_self_stop()->!{arm64_nmi_cpu_stop(core::ptr::null_mut(),false)}
pub unsafe fn arm64_nmi_cpu_stop(regs:*mut pt_regs,die_on_crash:bool)->!{let cpu=smp_processor_id();local_daif_mask();if crash_stop&&die_on_crash{__cpu_try_die(cpu as i32);}set_cpu_online(cpu,false);sdei_mask_local_cpu();cpu_park_loop()}

// Remaining platform-specific declarations and handlers retain their C
// interfaces; implementations are supplied by the kernel support layer.
extern "C" { fn ipi_setup(cpu:i32); fn op_cpu_kill(cpu:u32)->i32; fn of_parse_and_init_cpus(); fn acpi_parse_and_init_cpus(); fn smp_cpu_setup(cpu:i32)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
