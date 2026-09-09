// SPDX-License-Identifier: GPL-2.0
/* smp.c: Sparc SMP support.
 *
 * Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
 * Copyright (C) 1998 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 * Copyright (C) 2004 Keith M Wesolowski (wesolows@foobazco.org)
 */

// C dependencies: asm/head.h, linux/kernel.h, linux/sched.h, linux/threads.h,
// linux/smp.h, linux/interrupt.h, linux/kernel_stat.h, linux/init.h,
// linux/spinlock.h, linux/mm.h, linux/fs.h, linux/seq_file.h, linux/cache.h,
// linux/delay.h, linux/profile.h, linux/cpu.h, asm/ptrace.h, linux/atomic.h,
// asm/irq.h, asm/page.h, asm/oplib.h, asm/cacheflush.h, asm/tlbflush.h,
// asm/cpudata.h, asm/timer.h, asm/leon.h, kernel.h, irq.h.

volatile static mut CPU_CALLIN_MAP: [c_ulong; NR_CPUS] = [0; NR_CPUS];

static mut SMP_COMMENCED_MASK: cpumask_t = CPU_MASK_NONE;

static mut SPARC32_IPI_OPS: *const sparc32_ipi_ops = core::ptr::null();

/* The only guaranteed locking primitive available on all Sparc
 * processors is 'ldstub [%reg + immediate], %dest_reg' which atomically
 * places the current byte at the effective address into dest_reg and
 * places 0xff there afterwards.  Pretty lame locking primitive
 * compared to the Alpha and the Intel no?  Most Sparcs have 'swap'
 * instruction which is much better...
 */

pub unsafe fn smp_store_cpu_info(id: c_int) {
    let mut cpu_node: c_int = 0;
    let mut mid: c_int;

    cpu_data(id).udelay_val = loops_per_jiffy;

    cpu_find_by_mid(id, &mut cpu_node);
    cpu_data(id).clock_tick = prom_getintdefault(cpu_node, "clock-frequency\0", 0);
    cpu_data(id).prom_node = cpu_node;
    mid = cpu_get_hwmid(cpu_node);

    if mid < 0 {
        printk(KERN_NOTICE "No MID found for CPU%d at node 0x%08x\0", id, cpu_node);
        mid = 0;
    }
    cpu_data(id).mid = mid;
}

pub unsafe fn smp_cpus_done(_max_cpus: c_uint) {
    let mut bogosum: c_ulong = 0;
    let mut cpu: c_int;
    let mut num: c_int = 0;

    for_each_online_cpu!(cpu) {
        num += 1;
        bogosum += cpu_data(cpu).udelay_val;
    }

    printk!("Total of %d processors activated (%lu.%02lu BogoMIPS).\n\0",
        num, bogosum / (500000 / HZ), (bogosum / (5000 / HZ)) % 100);

    match sparc_cpu_model {
        sun4m => smp4m_smp_done(),
        sun4d => smp4d_smp_done(),
        sparc_leon => leon_smp_done(),
        sun4e => { printk!("SUN4E\n\0"); BUG!(); },
        sun4u => { printk!("SUN4U\n\0"); BUG!(); },
        _ => { printk!("UNKNOWN!\n\0"); BUG!(); },
    }
}

pub unsafe fn cpu_panic() {
    printk!("CPU[%d]: Returns from cpu_idle!\n\0", smp_processor_id());
    panic!("SMP bolixed\n\0");
}

static mut SMP_PENGUIN_CTABLE: linux_prom_registers = linux_prom_registers { _opaque: 0 };

pub unsafe fn arch_smp_send_reschedule(cpu: c_int) {
    (*SPARC32_IPI_OPS).resched(cpu);
}

pub unsafe fn smp_send_stop() {}

pub unsafe fn arch_send_call_function_single_ipi(cpu: c_int) {
    (*SPARC32_IPI_OPS).single(cpu);
}

pub unsafe fn arch_send_call_function_ipi_mask(mask: *const cpumask) {
    let mut cpu: c_int;
    for_each_cpu!(cpu, mask) {
        (*SPARC32_IPI_OPS).mask_one(cpu);
    }
}

pub unsafe fn smp_resched_interrupt() {
    irq_enter();
    scheduler_ipi();
    local_cpu_data().irq_resched_count += 1;
    irq_exit();
}

pub unsafe fn smp_call_function_single_interrupt() {
    irq_enter();
    generic_smp_call_function_single_interrupt();
    local_cpu_data().irq_call_count += 1;
    irq_exit();
}

pub unsafe fn smp_call_function_interrupt() {
    irq_enter();
    generic_smp_call_function_interrupt();
    local_cpu_data().irq_call_count += 1;
    irq_exit();
}

pub unsafe fn smp_prepare_cpus(max_cpus: c_uint) {
    let mut i: c_int = 0;
    let mut cpuid: c_int = 0;
    let mut extra: c_int = 0;

    printk!("Entering SMP Mode...\n\0");

    while !cpu_find_by_instance(i, core::ptr::null_mut(), &mut cpuid) {
        if cpuid >= NR_CPUS { extra += 1; }
        i += 1;
    }
    if extra != 0 && max_cpus > (i - extra) as c_uint {
        printk!("Warning: NR_CPUS is too low to start all cpus\n\0");
    }

    smp_store_cpu_info(boot_cpu_id);
    match sparc_cpu_model {
        sun4m => smp4m_boot_cpus(),
        sun4d => smp4d_boot_cpus(),
        sparc_leon => leon_boot_cpus(),
        sun4e => { printk!("SUN4E\n\0"); BUG!(); },
        sun4u => { printk!("SUN4U\n\0"); BUG!(); },
        _ => { printk!("UNKNOWN!\n\0"); BUG!(); },
    }
}

/* Set this up early so that things like the scheduler can init
 * properly.  We use the same cpu mask for both the present and
 * possible cpu map.
 */
pub unsafe fn smp_setup_cpu_possible_map() {
    let mut instance: c_int = 0;
    let mut mid: c_int = 0;
    while !cpu_find_by_instance(instance, core::ptr::null_mut(), &mut mid) {
        if mid < NR_CPUS {
            set_cpu_possible(mid, true);
            set_cpu_present(mid, true);
        }
        instance += 1;
    }
}

pub unsafe fn smp_prepare_boot_cpu() {
    let cpuid = hard_smp_processor_id();
    if cpuid >= NR_CPUS {
        prom_printf!("Serious problem, boot cpu id >= NR_CPUS\n\0");
        prom_halt();
    }
    if cpuid != 0 {
        printk!("boot cpu id != 0, this could work but is untested\n\0");
    }
    (*current_thread_info()).cpu = cpuid;
    set_cpu_online(cpuid, true);
    set_cpu_possible(cpuid, true);
}

pub unsafe fn __cpu_up(cpu: c_uint, tidle: *mut task_struct) -> c_int {
    let mut ret: c_int = 0;
    match sparc_cpu_model {
        sun4m => ret = smp4m_boot_one_cpu(cpu, tidle),
        sun4d => ret = smp4d_boot_one_cpu(cpu, tidle),
        sparc_leon => ret = leon_boot_one_cpu(cpu, tidle),
        sun4e => { printk!("SUN4E\n\0"); BUG!(); },
        sun4u => { printk!("SUN4U\n\0"); BUG!(); },
        _ => { printk!("UNKNOWN!\n\0"); BUG!(); },
    }
    if ret == 0 {
        cpumask_set_cpu(cpu, &mut SMP_COMMENCED_MASK);
        while !cpu_online(cpu) { mb(); }
    }
    ret
}

unsafe fn arch_cpu_pre_starting(arg: *mut c_void) {
    (*local_ops).cache_all();
    (*local_ops).tlb_all();
    match sparc_cpu_model {
        sun4m => sun4m_cpu_pre_starting(arg),
        sun4d => sun4d_cpu_pre_starting(arg),
        sparc_leon => leon_cpu_pre_starting(arg),
        _ => BUG!(),
    }
}

unsafe fn arch_cpu_pre_online(arg: *mut c_void) {
    let cpuid = hard_smp_processor_id();
    register_percpu_ce(cpuid);
    calibrate_delay();
    smp_store_cpu_info(cpuid);
    (*local_ops).cache_all();
    (*local_ops).tlb_all();
    match sparc_cpu_model {
        sun4m => sun4m_cpu_pre_online(arg),
        sun4d => sun4d_cpu_pre_online(arg),
        sparc_leon => leon_cpu_pre_online(arg),
        _ => BUG!(),
    }
}

unsafe fn sparc_start_secondary(arg: *mut c_void) {
    arch_cpu_pre_starting(arg);
    let cpu = smp_processor_id();
    notify_cpu_starting(cpu);
    arch_cpu_pre_online(arg);
    set_cpu_online(cpu, true);
    local_irq_enable();
    wmb();
    cpu_startup_entry(CPUHP_AP_ONLINE_IDLE);
    BUG!();
}

pub unsafe fn smp_callin() { sparc_start_secondary(core::ptr::null_mut()); }

pub unsafe fn smp_bogo(m: *mut seq_file) {
    let mut i: c_int;
    for_each_online_cpu!(i) {
        seq_printf!(m, "Cpu%dBogo\t: %lu.%02lu\n\0", i,
            cpu_data(i).udelay_val / (500000 / HZ),
            (cpu_data(i).udelay_val / (5000 / HZ)) % 100);
    }
}

pub unsafe fn smp_info(m: *mut seq_file) {
    let mut i: c_int;
    seq_printf!(m, "State:\n\0");
    for_each_online_cpu!(i) {
        seq_printf!(m, "CPU%d\t\t: online\n\0", i);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
