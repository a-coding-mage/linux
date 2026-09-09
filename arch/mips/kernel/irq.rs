/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Code to handle x86 style IRQs plus some generic interrupt stuff.
 *
 * Copyright (C) 1992 Linus Torvalds
 * Copyright (C) 1994 - 2000 Ralf Baechle
 */

// Declarations supplied by the Linux kernel headers are external dependencies.

extern "C" {
    static mut irq_stack: [*mut core::ffi::c_void; NR_CPUS];
    static mut irq_err_count: atomic_t;

    fn printk(format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn atomic_read(v: *const atomic_t) -> core::ffi::c_int;
    fn atomic_inc(v: *mut atomic_t);
    fn seq_printf(p: *mut seq_file, format: *const core::ffi::c_char, ...);
    fn get_order(size: usize) -> core::ffi::c_uint;
    fn irq_set_noprobe(irq: core::ffi::c_uint);
    fn clear_c0_status(mask: core::ffi::c_ulong);
    fn arch_init_irq();
    fn __get_free_pages(gfp_mask: core::ffi::c_ulong, order: core::ffi::c_uint)
        -> *mut core::ffi::c_void;
    fn pr_debug(format: *const core::ffi::c_char, ...);
    fn irq_enter();
    fn generic_handle_irq(irq: core::ffi::c_uint);
    fn irq_exit();
    fn dump_stack();
    fn generic_handle_domain_irq(domain: *mut irq_domain, hwirq: core::ffi::c_uint);
}

pub unsafe fn ack_bad_irq(irq: core::ffi::c_uint) {
    printk(b"unexpected IRQ # %d\n\0".as_ptr() as *const _, irq);
}

pub unsafe fn arch_show_interrupts(p: *mut seq_file, prec: core::ffi::c_int) -> core::ffi::c_int {
    seq_printf(
        p,
        b"%*s: %10u\n\0".as_ptr() as *const _,
        prec,
        b"ERR\0".as_ptr(),
        atomic_read(&irq_err_count),
    );
    0
}

pub unsafe fn spurious_interrupt() {
    atomic_inc(&mut irq_err_count);
}

pub unsafe fn init_IRQ() {
    let mut i: core::ffi::c_int;
    let order: core::ffi::c_uint = get_order(IRQ_STACK_SIZE);

    i = 0;
    while i < NR_IRQS {
        irq_set_noprobe(i as core::ffi::c_uint);
        i += 1;
    }

    // #ifdef condition: CONFIG/MIPS build-time symbol cpu_has_veic.
    if cpu_has_veic {
        clear_c0_status(ST0_IM);
    }

    arch_init_irq();

    // for_each_possible_cpu(i)
    for_each_possible_cpu!(i, {
        let s: *mut core::ffi::c_void = __get_free_pages(GFP_KERNEL, order);

        irq_stack[i as usize] = s;
        pr_debug(
            b"CPU%d IRQ stack at 0x%p - 0x%p\n\0".as_ptr() as *const _,
            i,
            irq_stack[i as usize],
            irq_stack[i as usize].add(IRQ_STACK_SIZE),
        );
    });
}

// #ifdef CONFIG_DEBUG_STACKOVERFLOW
#[inline]
pub unsafe fn check_stack_overflow() {
    let mut sp: usize;
    core::arch::asm!("move {0}, $sp", out(reg) sp);
    sp &= THREAD_MASK;

    /*
     * Check for stack overflow: is there less than STACK_WARN free?
     * STACK_WARN is defined as 1/8 of THREAD_SIZE by default.
     */
    if sp < core::mem::size_of::<thread_info>() + STACK_WARN {
        printk(
            b"do_IRQ: stack overflow: %ld\n\0".as_ptr() as *const _,
            sp.wrapping_sub(core::mem::size_of::<thread_info>()),
        );
        dump_stack();
    }
}

// #else: static inline void check_stack_overflow(void) {}

pub unsafe fn do_IRQ(irq: core::ffi::c_uint) {
    irq_enter();
    check_stack_overflow();
    generic_handle_irq(irq);
    irq_exit();
}

// #ifdef CONFIG_IRQ_DOMAIN
pub unsafe fn do_domain_IRQ(domain: *mut irq_domain, hwirq: core::ffi::c_uint) {
    irq_enter();
    check_stack_overflow();
    generic_handle_domain_irq(domain, hwirq);
    irq_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
