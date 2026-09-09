// SPDX-License-Identifier: GPL-2.0
/*
 *  sun4m SMP support.
 *
 * Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
 */

// C dependencies: linux/clockchips.h, linux/interrupt.h, linux/profile.h,
// linux/delay.h, linux/sched/mm.h, linux/cpu.h, asm/cacheflush.h,
// asm/switch_to.h, asm/tlbflush.h, asm/timer.h, asm/oplib.h, irq.h, kernel.h.

const IRQ_IPI_SINGLE: i32 = 12;
const IRQ_IPI_MASK: i32 = 13;
const IRQ_IPI_RESCHED: i32 = 14;
const IRQ_CROSS_CALL: i32 = 15;

unsafe fn swap_ulong(ptr: *mut core::ffi::c_ulong, mut val: core::ffi::c_ulong) -> core::ffi::c_ulong {
    core::arch::asm!(
        "swap [{ptr}], {val}",
        ptr = inout(reg) ptr,
        val = inout(reg) val,
    );
    val
}

pub unsafe fn sun4m_cpu_pre_starting(_arg: *mut core::ffi::c_void) {}

pub unsafe fn sun4m_cpu_pre_online(_arg: *mut core::ffi::c_void) {
    let cpuid: i32 = hard_smp_processor_id();

    /* Allow master to continue. The master will then give us the
     * go-ahead by setting the smp_commenced_mask and will wait without
     * timeouts until our setup is completed fully (signified by
     * our bit being set in the cpu_online_mask).
     */
    swap_ulong(&mut cpu_callin_map[cpuid as usize], 1);

    /* XXX: What's up with all the flushes? */
    ((*local_ops).cache_all)();
    ((*local_ops).tlb_all)();

    /* Fix idle thread fields. */
    core::arch::asm!("ld [{0}], %g6", in(reg) &current_set[cpuid as usize], options(nostack));

    /* Attach to the address space of init_task. */
    mmgrab(&mut init_mm);
    (*current).active_mm = &mut init_mm;

    while !cpumask_test_cpu(cpuid, &smp_commenced_mask) {
        mb();
    }
}

/*
 *\tCycle through the processors asking the PROM to start each one.
 */
pub unsafe fn smp4m_boot_cpus() {
    sun4m_unmask_profile_irq();
    ((*local_ops).cache_all)();
}

pub unsafe fn smp4m_boot_one_cpu(i: i32, idle: *mut task_struct) -> i32 {
    let mut entry: *mut core::ffi::c_ulong = &mut sun4m_cpu_startup;
    let mut timeout: i32;
    let mut cpu_node: i32 = 0;

    cpu_find_by_mid(i, &mut cpu_node);
    current_set[i as usize] = task_thread_info(idle);

    /* See trampoline.S for details... */
    entry = entry.add(((i - 1) * 3) as usize);

    /*
     * Initialize the contexts table
     * Since the call to prom_startcpu() trashes the structure,
     * we need to re-initialize it for each cpu
     */
    smp_penguin_ctable.which_io = 0;
    smp_penguin_ctable.phys_addr = srmmu_ctx_table_phys as u32;
    smp_penguin_ctable.reg_size = 0;

    /* whirrr, whirrr, whirrrrrrrrr... */
    printk(KERN_INFO, "Starting CPU %d at %p\n", i, entry);
    ((*local_ops).cache_all)();
    prom_startcpu(cpu_node, &mut smp_penguin_ctable, 0, entry as *mut i8);

    /* wheee... it's going... */
    timeout = 0;
    while timeout < 10000 {
        if cpu_callin_map[i as usize] != 0 {
            break;
        }
        udelay(200);
        timeout += 1;
    }

    if cpu_callin_map[i as usize] == 0 {
        printk(KERN_ERR, "Processor %d is stuck.\n", i);
        return -ENODEV;
    }

    ((*local_ops).cache_all)();
    0
}

pub unsafe fn smp4m_smp_done() {
    let mut first: i32 = 0;
    let mut prev: *mut i32 = &mut first;
    let mut i: i32 = 0;

    /* setup cpu list for irq rotation */
    first = 0;
    for_each_online_cpu!(i) {
        *prev = i;
        prev = &mut cpu_data(i).next;
    }
    *prev = first;
    ((*local_ops).cache_all)();

    /* Ok, they are spinning and ready to go. */
}

unsafe fn sun4m_send_ipi(cpu: i32, level: i32) {
    sbus_writel(SUN4M_SOFT_INT(level), &mut (*sun4m_irq_percpu[cpu as usize]).set);
}

unsafe fn sun4m_ipi_resched(cpu: i32) { sun4m_send_ipi(cpu, IRQ_IPI_RESCHED); }
unsafe fn sun4m_ipi_single(cpu: i32) { sun4m_send_ipi(cpu, IRQ_IPI_SINGLE); }
unsafe fn sun4m_ipi_mask_one(cpu: i32) { sun4m_send_ipi(cpu, IRQ_IPI_MASK); }

#[repr(C)]
struct smp_funcall {
    func: *mut core::ffi::c_void,
    arg1: core::ffi::c_ulong,
    arg2: core::ffi::c_ulong,
    arg3: core::ffi::c_ulong,
    arg4: core::ffi::c_ulong,
    arg5: core::ffi::c_ulong,
    processors_in: [core::ffi::c_ulong; SUN4M_NCPUS as usize],
    processors_out: [core::ffi::c_ulong; SUN4M_NCPUS as usize],
}

static mut ccall_info: smp_funcall = smp_funcall {
    func: core::ptr::null_mut(), arg1: 0, arg2: 0, arg3: 0, arg4: 0, arg5: 0,
    processors_in: [0; SUN4M_NCPUS as usize],
    processors_out: [0; SUN4M_NCPUS as usize],
};

static mut cross_call_lock: spinlock_t = spinlock_t::new();

/* Cross calls must be serialized, at least currently. */
unsafe fn sun4m_cross_call(func: *mut core::ffi::c_void, mut mask: cpumask_t,
                            arg1: core::ffi::c_ulong, arg2: core::ffi::c_ulong,
                            arg3: core::ffi::c_ulong, arg4: core::ffi::c_ulong) {
    let ncpus: i32 = SUN4M_NCPUS;
    let mut flags: core::ffi::c_ulong = 0;

    spin_lock_irqsave(&mut cross_call_lock, &mut flags);

    /* Init function glue. */
    ccall_info.func = func;
    ccall_info.arg1 = arg1;
    ccall_info.arg2 = arg2;
    ccall_info.arg3 = arg3;
    ccall_info.arg4 = arg4;
    ccall_info.arg5 = 0;

    /* Init receive/complete mapping, plus fire the IPI's off. */
    cpumask_clear_cpu(smp_processor_id(), &mut mask);
    cpumask_and(&mut mask, cpu_online_mask, &mask);
    for i in 0..ncpus {
        if cpumask_test_cpu(i, &mask) {
            ccall_info.processors_in[i as usize] = 0;
            ccall_info.processors_out[i as usize] = 0;
            sun4m_send_ipi(i, IRQ_CROSS_CALL);
        } else {
            ccall_info.processors_in[i as usize] = 1;
            ccall_info.processors_out[i as usize] = 1;
        }
    }

    let mut i: i32 = 0;
    loop {
        if cpumask_test_cpu(i, &mask) {
            while ccall_info.processors_in[i as usize] == 0 { barrier(); }
        }
        i += 1;
        if i >= ncpus { break; }
    }
    i = 0;
    loop {
        if cpumask_test_cpu(i, &mask) {
            while ccall_info.processors_out[i as usize] == 0 { barrier(); }
        }
        i += 1;
        if i >= ncpus { break; }
    }
    spin_unlock_irqrestore(&mut cross_call_lock, flags);
}

/* Running cross calls. */
pub unsafe fn smp4m_cross_call_irq() {
    let func: unsafe extern "C" fn(core::ffi::c_ulong, core::ffi::c_ulong,
                                    core::ffi::c_ulong, core::ffi::c_ulong,
                                    core::ffi::c_ulong) = core::mem::transmute(ccall_info.func);
    let i: i32 = smp_processor_id();

    ccall_info.processors_in[i as usize] = 1;
    func(ccall_info.arg1, ccall_info.arg2, ccall_info.arg3, ccall_info.arg4,
         ccall_info.arg5);
    ccall_info.processors_out[i as usize] = 1;
}

pub unsafe fn smp4m_percpu_timer_interrupt(regs: *mut pt_regs) {
    let old_regs: *mut pt_regs;
    let ce: *mut clock_event_device;
    let cpu: i32 = smp_processor_id();

    old_regs = set_irq_regs(regs);
    ce = &mut per_cpu!(sparc32_clockevent, cpu);

    if clockevent_state_periodic(ce) {
        sun4m_clear_profile_irq(cpu);
    } else {
        sparc_config.load_profile_irq(cpu, 0); /* Is this needless? */
    }

    irq_enter();
    ((*ce).event_handler)(ce);
    irq_exit();
    set_irq_regs(old_regs);
}

#[repr(C)]
struct sparc32_ipi_ops {
    cross_call: unsafe fn(*mut core::ffi::c_void, cpumask_t, core::ffi::c_ulong,
                          core::ffi::c_ulong, core::ffi::c_ulong, core::ffi::c_ulong),
    resched: unsafe fn(i32),
    single: unsafe fn(i32),
    mask_one: unsafe fn(i32),
}

static sun4m_ipi_ops: sparc32_ipi_ops = sparc32_ipi_ops {
    cross_call: sun4m_cross_call,
    resched: sun4m_ipi_resched,
    single: sun4m_ipi_single,
    mask_one: sun4m_ipi_mask_one,
};

pub unsafe fn sun4m_init_smp() {
    sparc32_ipi_ops = &sun4m_ipi_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
