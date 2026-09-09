// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/irq.c
 *
 *  Copyright (C) 1992 Linus Torvalds
 *  Modifications for ARM processor Copyright (C) 1995-2000 Russell King.
 *
 *  Support for Dynamic Tick Timer Copyright (C) 2004-2005 Nokia Corporation.
 *  Dynamic Tick Timer written by Tony Lindgren <tony@atomide.com> and
 *  Tuukka Tikkanen <tuukka.tikkanen@elektrobit.com>.
 *
 *  This file contains the code used by various IRQ handling routines:
 *  asking for different IRQ's should be done through these routines
 *  instead of just grabbing them. Thus setups with different IRQ numbers
 *  shouldn't result in any weird surprises, and installing new handlers
 *  should be easier.
 *
 *  IRQ's are in fact implemented a bit like signal handlers for the kernel.
 *  Naturally it's not a 1:1 relation, but there are similarities.
 */

// C dependencies supplied by the surrounding kernel translation unit.

pub static mut irq_err_count: core::ffi::c_ulong = 0;

#[cfg(CONFIG_IRQSTACKS)]
pub static mut irq_stack_ptr: *mut u8 = core::ptr::null_mut();

#[cfg(CONFIG_IRQSTACKS)]
unsafe fn init_irq_stacks() {
    let mut stack: *mut u8;
    let mut cpu: core::ffi::c_int = 0;

    // for_each_possible_cpu(cpu)
    while cpu < nr_cpu_ids() {
        if !cfg!(CONFIG_VMAP_STACK) {
            stack = __get_free_pages(GFP_KERNEL, THREAD_SIZE_ORDER) as *mut u8;
        } else {
            stack = __vmalloc_node(
                THREAD_SIZE,
                THREAD_ALIGN,
                THREADINFO_GFP,
                NUMA_NO_NODE,
                __builtin_return_address(0),
            ) as *mut u8;
        }

        if WARN_ON(stack.is_null()) {
            break;
        }
        // per_cpu(irq_stack_ptr, cpu) = &stack[THREAD_SIZE]
        irq_stack_ptr = stack.add(THREAD_SIZE);
        cpu += 1;
    }
}

#[cfg(all(CONFIG_IRQSTACKS, CONFIG_SOFTIRQ_ON_OWN_STACK))]
unsafe extern "C" fn ____do_softirq(_arg: *mut core::ffi::c_void) {
    __do_softirq();
}

#[cfg(all(CONFIG_IRQSTACKS, CONFIG_SOFTIRQ_ON_OWN_STACK))]
pub unsafe fn do_softirq_own_stack() {
    call_with_stack(
        ____do_softirq,
        core::ptr::null_mut(),
        irq_stack_ptr,
    );
}

pub unsafe fn arch_show_interrupts(
    p: *mut seq_file,
    prec: core::ffi::c_int,
) -> core::ffi::c_int {
    #[cfg(CONFIG_FIQ)]
    show_fiq_list(p, prec);
    #[cfg(CONFIG_SMP)]
    show_ipi_list(p, prec);
    seq_printf(p, "%*s: %10lu\n", prec, "Err", irq_err_count);
    0
}

/*
 * handle_IRQ handles all hardware IRQ's.  Decoded IRQs should
 * not come via this function.  Instead, they should provide
 * their own 'handler'.  Used by platform code implementing C-based 1st
 * level decoding.
 */
pub unsafe fn handle_IRQ(irq: core::ffi::c_uint, regs: *mut pt_regs) {
    let desc: *mut irq_desc;

    /*
     * Some hardware gives randomly wrong interrupts.  Rather
     * than crashing, do something sensible.
     */
    if unlikely(irq == 0 || irq >= irq_get_nr_irqs()) {
        desc = core::ptr::null_mut();
    } else {
        desc = irq_to_desc(irq);
    }

    if likely(!desc.is_null()) {
        handle_irq_desc(desc);
    } else {
        ack_bad_irq(irq);
    }
}

pub unsafe fn init_IRQ() {
    let mut ret: core::ffi::c_int;

    #[cfg(CONFIG_IRQSTACKS)]
    init_irq_stacks();

    if cfg!(CONFIG_OF) && (*machine_desc).init_irq.is_none() {
        irqchip_init();
    } else {
        ((*machine_desc).init_irq.unwrap())();
    }

    if cfg!(all(CONFIG_OF, CONFIG_CACHE_L2X0))
        && ((*machine_desc).l2c_aux_mask != 0 || (*machine_desc).l2c_aux_val != 0)
    {
        if outer_cache.write_sec.is_none() {
            outer_cache.write_sec = (*machine_desc).l2c_write_sec;
        }
        ret = l2x0_of_init((*machine_desc).l2c_aux_val, (*machine_desc).l2c_aux_mask);
        if ret != 0 && ret != -ENODEV {
            pr_err("L2C: failed to init: %d\n", ret);
        }
    }

    uniphier_cache_init();
}

#[cfg(CONFIG_SPARSE_IRQ)]
pub unsafe fn arch_probe_nr_irqs() -> core::ffi::c_int {
    irq_set_nr_irqs(if (*machine_desc).nr_irqs != 0 {
        (*machine_desc).nr_irqs
    } else {
        NR_IRQS
    })
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
