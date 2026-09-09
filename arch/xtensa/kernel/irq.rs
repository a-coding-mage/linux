// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/xtensa/kernel/irq.c
 *
 * Xtensa built-in interrupt controller and some generic functions copied
 * from i386.
 *
 * Copyright (C) 2002 - 2013 Tensilica, Inc.
 * Copyright (C) 1992, 1998 Linus Torvalds, Ingo Molnar
 *
 * Chris Zankel <chris@zankel.net>
 * Kevin Chea
 */

// Kernel and architecture declarations are supplied by the surrounding tree.

extern "C" {
    static mut nmi_count: PerCpu<u64>;

    fn generic_handle_domain_irq(domain: *mut irq_domain, hwirq: i32) -> i32;
    fn show_ipi_list(p: *mut seq_file, prec: i32);
    fn seq_printf(p: *mut seq_file, fmt: *const u8, ...);
    fn seq_puts(p: *mut seq_file, s: *const u8);
    fn irq_set_chip_and_handler_name(irq: u32, chip: *mut irq_chip,
        handler: unsafe extern "C" fn(), name: *const u8);
    fn irq_set_status_flags(irq: u32, flags: u32);
    fn irq_clear_status_flags(irq: u32, flags: u32);
    fn irqchip_init();
    fn xtensa_mx_init_legacy(arg: *mut core::ffi::c_void);
    fn xtensa_pic_init_legacy(arg: *mut core::ffi::c_void);
    fn ipi_init();
    fn smp_processor_id() -> u32;
    fn irq_get_irq_data(irq: u32) -> *mut irq_data;
    fn irqd_is_per_cpu(data: *mut irq_data) -> bool;
    fn irq_data_get_affinity_mask(data: *mut irq_data) -> *const cpumask;
    fn cpumask_test_cpu(cpu: u32, mask: *const cpumask) -> bool;
    fn cpumask_any_and(mask: *const cpumask, online: *const cpumask) -> u32;
    fn irq_set_affinity(irq: u32, mask: *const cpumask) -> i32;
    fn pr_info_ratelimited(fmt: *const u8, ...);
}

#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct irq_domain { pub host_data: *mut irq_chip }
#[repr(C)] pub struct irq_chip { _private: [u8; 0] }
#[repr(C)] pub struct irq_data { _private: [u8; 0] }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
pub struct PerCpu<T>(core::marker::PhantomData<T>);

#[allow(non_snake_case)]
pub unsafe extern "C" fn do_IRQ(hwirq: i32, _regs: *mut pt_regs) {
    #[cfg(CONFIG_DEBUG_STACKOVERFLOW)]
    {
        // Debugging check for stack overflow: is there less than 1KB free?
        let mut sp = current_stack_pointer;
        sp &= THREAD_SIZE - 1;
        if sp < core::mem::size_of::<thread_info>() + 1024 {
            printk(b"Stack overflow in do_IRQ: %ld\n\0".as_ptr(),
                sp - core::mem::size_of::<thread_info>());
        }
    }
    generic_handle_domain_irq(core::ptr::null_mut(), hwirq);
}

pub unsafe extern "C" fn arch_show_interrupts(p: *mut seq_file, prec: i32) -> i32 {
    #[cfg(CONFIG_SMP)] show_ipi_list(p, prec);
    #[cfg(XTENSA_FAKE_NMI)] {
        seq_printf(p, b"%*s:\0".as_ptr(), prec, b"NMI\0".as_ptr());
        // for_each_online_cpu(cpu)
        seq_puts(p, b" Non-maskable interrupts\n\0".as_ptr());
    }
    0
}

pub unsafe extern "C" fn xtensa_irq_domain_xlate(
    intspec: *const u32, intsize: u32, mut int_irq: u64, ext_irq: u64,
    out_hwirq: *mut u64, out_type: *mut u32,
) -> i32 {
    if intsize < 1 || intsize > 2 { return -22; }
    if intsize == 2 && *intspec.add(1) == 1 {
        int_irq = xtensa_map_ext_irq(ext_irq as u32) as u64;
        if int_irq < XCHAL_NUM_INTERRUPTS as u64 { *out_hwirq = int_irq; }
        else { return -22; }
    } else { *out_hwirq = int_irq; }
    *out_type = IRQ_TYPE_NONE;
    0
}

pub unsafe extern "C" fn xtensa_irq_map(d: *mut irq_domain, irq: u32, hw: u64) -> i32 {
    let irq_chip = (*d).host_data;
    let mask = 1u32 << hw;
    if mask & XCHAL_INTTYPE_MASK_SOFTWARE != 0 {
        irq_set_chip_and_handler_name(irq, irq_chip, handle_simple_irq, b"level\0".as_ptr());
        irq_set_status_flags(irq, IRQ_LEVEL);
    } else if mask & XCHAL_INTTYPE_MASK_EXTERN_EDGE != 0 {
        irq_set_chip_and_handler_name(irq, irq_chip, handle_edge_irq, b"edge\0".as_ptr());
        irq_clear_status_flags(irq, IRQ_LEVEL);
    } else if mask & XCHAL_INTTYPE_MASK_EXTERN_LEVEL != 0 {
        irq_set_chip_and_handler_name(irq, irq_chip, handle_level_irq, b"level\0".as_ptr());
        irq_set_status_flags(irq, IRQ_LEVEL);
    } else if mask & XCHAL_INTTYPE_MASK_TIMER != 0 {
        irq_set_chip_and_handler_name(irq, irq_chip, handle_percpu_irq, b"timer\0".as_ptr());
        irq_clear_status_flags(irq, IRQ_LEVEL);
    } else {
        irq_set_chip_and_handler_name(irq, irq_chip, handle_level_irq, b"level\0".as_ptr());
        irq_set_status_flags(irq, IRQ_LEVEL);
    }
    0
}

pub unsafe extern "C" fn xtensa_map_ext_irq(mut ext_irq: u32) -> u32 {
    let mut mask = XCHAL_INTTYPE_MASK_EXTERN_EDGE | XCHAL_INTTYPE_MASK_EXTERN_LEVEL;
    let mut i = 0;
    while mask != 0 { if mask & 1 != 0 && { let old = ext_irq; ext_irq -= 1; old == 0 } { return i; } i += 1; mask >>= 1; }
    XCHAL_NUM_INTERRUPTS
}

pub unsafe extern "C" fn xtensa_get_ext_irq_no(irq: u32) -> u32 {
    let mask = (XCHAL_INTTYPE_MASK_EXTERN_EDGE | XCHAL_INTTYPE_MASK_EXTERN_LEVEL) & ((1u32 << irq) - 1);
    mask.count_ones()
}

pub unsafe extern "C" fn init_IRQ() {
    #[cfg(CONFIG_USE_OF)] irqchip_init();
    #[cfg(all(not(CONFIG_USE_OF), CONFIG_HAVE_SMP))]
    xtensa_mx_init_legacy(core::ptr::null_mut());
    #[cfg(all(not(CONFIG_USE_OF), not(CONFIG_HAVE_SMP)))]
    xtensa_pic_init_legacy(core::ptr::null_mut());
    #[cfg(CONFIG_SMP)] ipi_init();
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe extern "C" fn migrate_irqs() {
    let cpu = smp_processor_id();
    // for_each_active_irq(i)
    // The active-IRQ iterator and nr_cpu_ids/cpu_all_mask are supplied by the kernel.
    let _ = cpu;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
