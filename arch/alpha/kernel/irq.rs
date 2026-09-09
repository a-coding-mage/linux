// SPDX-License-Identifier: GPL-2.0
/*
 *	linux/arch/alpha/kernel/irq.c
 *
 *	Copyright (C) 1995 Linus Torvalds
 *
 * This file contains the code used by various IRQ handling routines:
 * asking for different IRQ's should be done through these routines
 * instead of just grabbing them. Thus setups with different IRQ numbers
 * shouldn't result in any weird surprises, and installing new handlers
 * should be easier.
 */

// Linux and architecture headers from the original source supply the
// declarations referenced by this translation.

pub static mut irq_err_count: ::core::ffi::c_ulong = 0;
// DEFINE_PER_CPU(unsigned long, irq_pmi_count);
extern "C" {
    static mut irq_pmi_count: ::core::ffi::c_ulong;
}

pub unsafe fn ack_bad_irq(irq: ::core::ffi::c_uint) {
    irq_err_count = irq_err_count.wrapping_add(1);
    printk(KERN_CRIT, b"Unexpected IRQ trap at vector %u\n\0".as_ptr(), irq);
}

#[cfg(CONFIG_SMP)]
static mut irq_user_affinity: [::core::ffi::c_char; NR_IRQS as usize] = [0; NR_IRQS as usize];

#[cfg(CONFIG_SMP)]
pub unsafe fn irq_select_affinity(irq: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let data = irq_get_irq_data(irq);
    let chip: *mut irq_chip;
    static mut last_cpu: ::core::ffi::c_int = 0;
    let mut cpu = last_cpu + 1;

    if data.is_null() {
        return 1;
    }
    chip = irq_data_get_irq_chip(data);

    if (*chip).irq_set_affinity.is_none() || irq_user_affinity[irq as usize] != 0 {
        return 1;
    }

    while !cpu_possible(cpu)
        || !cpumask_test_cpu(cpu, irq_default_affinity)
    {
        cpu = if cpu < NR_CPUS - 1 { cpu + 1 } else { 0 };
    }
    last_cpu = cpu;

    irq_data_update_affinity(data, cpumask_of(cpu));
    ((*chip).irq_set_affinity.unwrap())(data, cpumask_of(cpu), false);
    0
}

pub unsafe fn arch_show_interrupts(
    p: *mut seq_file,
    _prec: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int;

    #[cfg(CONFIG_SMP)]
    {
        seq_puts(p, b" IPI: \0".as_ptr() as *const _);
        for_each_online_cpu!(j) {
            seq_printf(p, b"%10lu \0".as_ptr() as *const _, cpu_data[j as usize].ipi_count);
        }
        seq_putc(p, b'\n' as ::core::ffi::c_int);
    }
    seq_puts(p, b" PMI: \0".as_ptr() as *const _);
    for_each_online_cpu!(j) {
        seq_printf(p, b"%10lu \0".as_ptr() as *const _, per_cpu!(irq_pmi_count, j));
    }
    seq_puts(p, b" Performance Monitoring\n\0".as_ptr() as *const _);
    seq_printf(p, b" ERR: %10lu\n\0".as_ptr() as *const _, irq_err_count);
    0
}

/*
 * handle_irq handles all normal device IRQ's (the special
 * SMP cross-CPU interrupts have their own specific
 * handlers).
 */

pub const MAX_ILLEGAL_IRQS: ::core::ffi::c_uint = 16;

pub unsafe fn handle_irq(irq: ::core::ffi::c_int) {
    /*
     * We ack quickly, we don't want the irq controller
     * thinking we're snobs just because some other CPU has
     * disabled global interrupts (we have already done the
     * INT_ACK cycles, it's too late to try to pretend to the
     * controller that we aren't taking the interrupt).
     *
     * 0 return value means that this irq is already being
     * handled by some other CPU. (or is disabled)
     */
    static mut illegal_count: ::core::ffi::c_uint = 0;
    let desc = irq_to_desc(irq);

    if desc.is_null()
        || ((irq as ::core::ffi::c_uint) > ACTUAL_NR_IRQS
            && illegal_count < MAX_ILLEGAL_IRQS)
    {
        irq_err_count = irq_err_count.wrapping_add(1);
        illegal_count = illegal_count.wrapping_add(1);
        printk(
            KERN_CRIT,
            b"device_interrupt: invalid interrupt %d\n\0".as_ptr(),
            irq,
        );
        return;
    }

    irq_enter();
    generic_handle_irq_desc(desc);
    irq_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
