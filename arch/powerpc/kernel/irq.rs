// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Direct Rust translation of the PowerPC IRQ implementation source.
 * C headers and externally supplied kernel symbols are intentionally left as
 * external dependencies.
 */

// C includes omitted; their symbols remain external dependencies.

#[cfg(CONFIG_PPC32)]
pub static mut ppc_n_lost_interrupts: atomic_t = atomic_t { _opaque: 0 };

#[cfg(all(CONFIG_PPC32, CONFIG_TAU_INT))]
extern "C" {
    static mut tau_initialized: ::core::ffi::c_int;
    fn tau_interrupts(cpu: ::core::ffi::c_ulong) -> u32;
}

extern "C" {
    static mut irq_stat: irq_cpustat_t;
}

pub unsafe fn arch_show_interrupts(p: *mut seq_file, prec: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int;

    #[cfg(all(CONFIG_PPC32, CONFIG_TAU_INT))]
    {
        if tau_initialized != 0 {
            seq_printf(p, "%*s:\0".as_ptr() as *const _, prec, "TAU\0".as_ptr());
            for_each_online_cpu!(j, {
                seq_put_decimal_ull_width(p, " \0".as_ptr() as *const _, tau_interrupts(j as _), 10);
            });
            seq_puts(p, "  PowerPC             Thermal Assist (cpu temp)\n\0".as_ptr() as *const _);
        }
    }

    seq_printf(p, "%*s:\0".as_ptr() as *const _, prec, "LOC\0".as_ptr());
    for_each_online_cpu!(j, {
        seq_put_decimal_ull_width(p, " \0".as_ptr() as *const _, per_cpu!(irq_stat, j).timer_irqs_event, 10);
    });
    seq_printf(p, "  Local timer interrupts for timer event device\n\0".as_ptr() as *const _);

    seq_printf(p, "%*s:\0".as_ptr() as *const _, prec, "BCT\0".as_ptr());
    for_each_online_cpu!(j, {
        seq_put_decimal_ull_width(p, " \0".as_ptr() as *const _, per_cpu!(irq_stat, j).broadcast_irqs_event, 10);
    });
    seq_printf(p, "  Broadcast timer interrupts for timer event device\n\0".as_ptr() as *const _);

    seq_printf(p, "%*s:\0".as_ptr() as *const _, prec, "LOC\0".as_ptr());
    for_each_online_cpu!(j, {
        seq_put_decimal_ull_width(p, " \0".as_ptr() as *const _, per_cpu!(irq_stat, j).timer_irqs_others, 10);
    });
    seq_printf(p, "  Local timer interrupts for others\n\0".as_ptr() as *const _);

    seq_printf(p, "%*s:\0".as_ptr() as *const _, prec, "SPU\0".as_ptr());
    for_each_online_cpu!(j, {
        seq_put_decimal_ull_width(p, " \0".as_ptr() as *const _, per_cpu!(irq_stat, j).spurious_irqs, 10);
    });
    seq_printf(p, "  Spurious interrupts\n\0".as_ptr() as *const _);

    seq_printf(p, "%*s:\0".as_ptr() as *const _, prec, "PMI\0".as_ptr());
    for_each_online_cpu!(j, {
        seq_put_decimal_ull_width(p, " \0".as_ptr() as *const _, per_cpu!(irq_stat, j).pmu_irqs, 10);
    });
    seq_printf(p, "  Performance monitoring interrupts\n\0".as_ptr() as *const _);

    seq_printf(p, "%*s:\0".as_ptr() as *const _, prec, "MCE\0".as_ptr());
    for_each_online_cpu!(j, {
        seq_put_decimal_ull_width(p, " \0".as_ptr() as *const _, per_cpu!(irq_stat, j).mce_exceptions, 10);
    });
    seq_printf(p, "  Machine check exceptions\n\0".as_ptr() as *const _);

    #[cfg(CONFIG_PPC_BOOK3S_64)]
    if cpu_has_feature(CPU_FTR_HVMODE) {
        seq_printf(p, "%*s:\0".as_ptr() as *const _, prec, "HMI\0".as_ptr());
        for_each_online_cpu!(j, {
            seq_put_decimal_ull_width(p, " \0".as_ptr() as *const _, (*paca_ptrs[j as usize]).hmi_irqs, 10);
        });
        seq_printf(p, "  Hypervisor Maintenance Interrupts\n\0".as_ptr() as *const _);
    }

    seq_printf(p, "%*s:\0".as_ptr() as *const _, prec, "NMI\0".as_ptr());
    for_each_online_cpu!(j, {
        seq_put_decimal_ull_width(p, " \0".as_ptr() as *const _, per_cpu!(irq_stat, j).sreset_irqs, 10);
    });
    seq_printf(p, "  System Reset interrupts\n\0".as_ptr() as *const _);

    #[cfg(CONFIG_PPC_WATCHDOG)]
    {
        seq_printf(p, "%*s:\0".as_ptr() as *const _, prec, "WDG\0".as_ptr());
        for_each_online_cpu!(j, { seq_put_decimal_ull_width(p, " \0".as_ptr() as *const _, per_cpu!(irq_stat, j).soft_nmi_irqs, 10); });
        seq_printf(p, "  Watchdog soft-NMI interrupts\n\0".as_ptr() as *const _);
    }

    #[cfg(CONFIG_PPC_DOORBELL)]
    if cpu_has_feature(CPU_FTR_DBELL) {
        seq_printf(p, "%*s:\0".as_ptr() as *const _, prec, "DBL\0".as_ptr());
        for_each_online_cpu!(j, { seq_put_decimal_ull_width(p, " \0".as_ptr() as *const _, per_cpu!(irq_stat, j).doorbell_irqs, 10); });
        seq_printf(p, "  Doorbell interrupts\n\0".as_ptr() as *const _);
    }
    0
}

pub unsafe fn arch_irq_stat_cpu(cpu: ::core::ffi::c_uint) -> u64 {
    let mut sum = per_cpu!(irq_stat, cpu).timer_irqs_event;
    sum += per_cpu!(irq_stat, cpu).broadcast_irqs_event;
    sum += per_cpu!(irq_stat, cpu).pmu_irqs;
    sum += per_cpu!(irq_stat, cpu).mce_exceptions;
    sum += per_cpu!(irq_stat, cpu).spurious_irqs;
    sum += per_cpu!(irq_stat, cpu).timer_irqs_others;
    #[cfg(CONFIG_PPC_BOOK3S_64)] { sum += (*paca_ptrs[cpu as usize]).hmi_irqs; }
    sum += per_cpu!(irq_stat, cpu).sreset_irqs;
    #[cfg(CONFIG_PPC_WATCHDOG)] { sum += per_cpu!(irq_stat, cpu).soft_nmi_irqs; }
    #[cfg(CONFIG_PPC_DOORBELL)] { sum += per_cpu!(irq_stat, cpu).doorbell_irqs; }
    sum
}

#[inline]
unsafe fn check_stack_overflow(mut sp: ::core::ffi::c_ulong) {
    if !is_enabled!(CONFIG_DEBUG_STACKOVERFLOW) { return; }
    sp &= THREAD_SIZE - 1;
    if unlikely(sp < THREAD_SIZE / 4) {
        pr_err!("do_IRQ: stack overflow: %ld\n", sp);
        dump_stack();
    }
}

#[cfg(CONFIG_SOFTIRQ_ON_OWN_STACK)]
#[inline(always)]
unsafe fn call_do_softirq(sp: *const ::core::ffi::c_void) {
    // PowerPC inline assembly: temporarily switches r1, calls __do_softirq, restores r1.
    unsafe { core::arch::asm!("", in("r3") sp, options(nostack)); }
}

static_call_ret0!(ppc_get_irq, ppc_md.get_irq);

unsafe fn __do_irq(regs: *mut pt_regs, oldsp: ::core::ffi::c_ulong) {
    trace_irq_entry(regs);
    check_stack_overflow(oldsp);
    let irq = static_call!(ppc_get_irq)();
    if should_hard_irq_enable(regs) { do_hard_irq_enable(); }
    if unlikely(irq == 0) { this_cpu_inc!(irq_stat.spurious_irqs); } else { generic_handle_irq(irq); }
    trace_irq_exit(regs);
}

#[inline(always)]
unsafe fn call_do_irq(regs: *mut pt_regs, sp: *mut ::core::ffi::c_void) {
    // PowerPC inline assembly: switches r1 to the IRQ stack and calls __do_irq.
    unsafe { core::arch::asm!("", in("r3") regs, in("r4") sp, options(nostack)); }
}

pub unsafe fn __do_IRQ(regs: *mut pt_regs) {
    let old_regs = set_irq_regs(regs);
    let cursp = (current_stack_pointer & !(THREAD_SIZE - 1)) as *mut ::core::ffi::c_void;
    let irqsp = hardirq_ctx[raw_smp_processor_id() as usize];
    if unlikely(cursp == irqsp) { __do_irq(regs, current_stack_pointer); } else { call_do_irq(regs, irqsp); }
    set_irq_regs(old_regs);
}

pub unsafe fn do_IRQ(regs: *mut pt_regs) { __do_IRQ(regs); }

unsafe fn alloc_vm_stack() -> *mut ::core::ffi::c_void {
    __vmalloc_node(THREAD_SIZE, THREAD_ALIGN, THREADINFO_GFP, NUMA_NO_NODE, _RET_IP_ as *mut _)
}

unsafe fn vmap_irqstack_init() {
    for_each_possible_cpu!(i, {
        softirq_ctx[i as usize] = alloc_vm_stack();
        hardirq_ctx[i as usize] = alloc_vm_stack();
    });
}

pub unsafe fn init_IRQ() {
    if is_enabled!(CONFIG_VMAP_STACK) { vmap_irqstack_init(); }
    if ppc_md.init_IRQ.is_some() { (ppc_md.init_IRQ.unwrap())(); }
    if !warn_on!(ppc_md.get_irq.is_none()) { static_call_update!(ppc_get_irq, ppc_md.get_irq); }
}

#[cfg(CONFIG_BOOKE)]
pub static mut critirq_ctx: [*mut ::core::ffi::c_void; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];
#[cfg(CONFIG_BOOKE)]
pub static mut dbgirq_ctx: [*mut ::core::ffi::c_void; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];
#[cfg(CONFIG_BOOKE)]
pub static mut mcheckirq_ctx: [*mut ::core::ffi::c_void; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];

pub static mut softirq_ctx: [*mut ::core::ffi::c_void; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];
pub static mut hardirq_ctx: [*mut ::core::ffi::c_void; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];

#[cfg(CONFIG_SOFTIRQ_ON_OWN_STACK)]
pub unsafe fn do_softirq_own_stack() { call_do_softirq(softirq_ctx[smp_processor_id() as usize]); }

pub unsafe fn virq_to_hw(virq: ::core::ffi::c_uint) -> irq_hw_number_t {
    let irq_data = irq_get_irq_data(virq);
    if warn_on!(irq_data.is_null()) { 0 } else { (*irq_data).hwirq }
}

#[cfg(CONFIG_SMP)]
pub unsafe fn irq_choose_cpu(mask: *const cpumask) -> ::core::ffi::c_int {
    let cpuid;
    if cpumask_equal(mask, cpu_online_mask) {
        static mut irq_rover: ::core::ffi::c_int = 0;
        static mut irq_rover_lock: raw_spinlock_t = raw_spinlock_t { _opaque: 0 };
        let mut flags: ::core::ffi::c_ulong = 0;
        raw_spin_lock_irqsave(&mut irq_rover_lock, &mut flags);
        irq_rover = cpumask_next_wrap(irq_rover, cpu_online_mask);
        cpuid = irq_rover;
        raw_spin_unlock_irqrestore(&mut irq_rover_lock, flags);
    } else {
        cpuid = cpumask_first_and(mask, cpu_online_mask);
        if cpuid >= nr_cpu_ids { return irq_choose_cpu(cpu_online_mask); }
    }
    get_hard_smp_processor_id(cpuid)
}

#[cfg(not(CONFIG_SMP))]
pub unsafe fn irq_choose_cpu(_mask: *const cpumask) -> ::core::ffi::c_int { hard_smp_processor_id() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
