// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1992, 1998 Linus Torvalds, Ingo Molnar
 *
 * This file contains the lowest level x86-specific interrupt
 * entry, irq-stacks and irq statistics code. All the remaining
 * irq logic is done by the generic kernel/irq/ code and
 * by the x86-specific irq controller code. (e.g. i8259.c and
 * io_apic.c.)
 */

// Linux and x86 header dependencies are supplied by other translation units.

#[cfg(CONFIG_DEBUG_STACKOVERFLOW)]
pub static mut sysctl_panic_on_stackoverflow: ::core::ffi::c_int = 0;

#[cfg(CONFIG_DEBUG_STACKOVERFLOW)]
unsafe fn check_stack_overflow() -> bool {
    let sp: usize = current_stack_pointer() & (THREAD_SIZE - 1);
    sp < (::core::mem::size_of::<thread_info>() + STACK_WARN)
}

#[cfg(CONFIG_DEBUG_STACKOVERFLOW)]
unsafe fn print_stack_overflow() {
    printk(KERN_WARNING, "low stack detected by irq handler\n");
    dump_stack();
    if sysctl_panic_on_stackoverflow != 0 {
        panic_kernel("low stack detected by irq handler - check messages\n");
    }
}

#[cfg(not(CONFIG_DEBUG_STACKOVERFLOW))]
#[inline]
unsafe fn check_stack_overflow() -> bool { false }

#[cfg(not(CONFIG_DEBUG_STACKOVERFLOW))]
#[inline]
unsafe fn print_stack_overflow() {}

// DEFINE_PER_CPU_CACHE_HOT(struct irq_stack *, softirq_stack_ptr);
pub static mut softirq_stack_ptr: *mut irq_stack = core::ptr::null_mut();

unsafe fn call_on_stack(func: *mut ::core::ffi::c_void, stack: *mut ::core::ffi::c_void) {
    let mut sp = stack;
    ::core::arch::asm!(
        "xchgl {sp}, %esp",
        CALL_NOSPEC,
        "movl {sp}, %esp",
        sp = inout(reg) sp,
        in("edi") func,
        options(nostack)
    );
}

#[inline]
unsafe fn current_stack() -> *mut ::core::ffi::c_void {
    (current_stack_pointer() & !(THREAD_SIZE - 1)) as *mut ::core::ffi::c_void
}

#[inline]
unsafe fn execute_on_irq_stack(overflow: bool, desc: *mut irq_desc) -> bool {
    let curstk = current_stack() as *mut irq_stack;
    let irqstk = __this_cpu_read_hardirq_stack_ptr();

    /*
     * this is where we switch to the IRQ stack. However, if we are
     * already using the IRQ stack (because we interrupted a hardirq
     * handler) we can't do that and just have to keep using the
     * current stack (which is the irq stack already after all)
     */
    if unlikely(curstk == irqstk) {
        return false;
    }

    let mut isp = (irqstk as *mut u8).add(core::mem::size_of::<irq_stack>()) as *mut u32;

    /* Save the next esp at the bottom of the stack */
    let prev_esp = irqstk as *mut u32;
    *prev_esp = current_stack_pointer() as u32;

    if unlikely(overflow) {
        call_on_stack(print_stack_overflow as *mut _, isp as *mut _);
    }

    let target = (*desc).handle_irq;
    ::core::arch::asm!(
        "xchgl {sp}, %esp",
        CALL_NOSPEC,
        "movl {sp}, %esp",
        sp = inout(reg) isp,
        in("eax") desc,
        in("edi") target,
        options(nostack)
    );
    true
}

/*
 * Allocate per-cpu stacks for hardirq and softirq processing
 */
pub unsafe fn irq_init_percpu_irqstack(cpu: u32) -> i32 {
    let node = cpu_to_node(cpu);
    let mut ph: *mut page;
    let mut ps: *mut page;

    if per_cpu_hardirq_stack_ptr(cpu) != core::ptr::null_mut() {
        return 0;
    }

    ph = alloc_pages_node(node, THREADINFO_GFP, THREAD_SIZE_ORDER);
    if ph.is_null() { return -12; /* -ENOMEM */ }
    ps = alloc_pages_node(node, THREADINFO_GFP, THREAD_SIZE_ORDER);
    if ps.is_null() {
        __free_pages(ph, THREAD_SIZE_ORDER);
        return -12; /* -ENOMEM */
    }

    set_per_cpu_hardirq_stack_ptr(cpu, page_address(ph));
    set_per_cpu_softirq_stack_ptr(cpu, page_address(ps));
    0
}

#[cfg(CONFIG_SOFTIRQ_ON_OWN_STACK)]
pub unsafe fn do_softirq_own_stack() {
    let irqstk = softirq_stack_ptr;
    /* build the stack frame on the softirq stack */
    let isp = (irqstk as *mut u8).add(core::mem::size_of::<irq_stack>()) as *mut u32;
    /* Push the previous esp onto the stack */
    let prev_esp = irqstk as *mut u32;
    *prev_esp = current_stack_pointer() as u32;
    call_on_stack(__do_softirq as *mut _, isp as *mut _);
}

pub unsafe fn __handle_irq(desc: *mut irq_desc, regs: *mut pt_regs) {
    let overflow = check_stack_overflow();
    if user_mode(regs) || !execute_on_irq_stack(overflow, desc) {
        if unlikely(overflow) { print_stack_overflow(); }
        generic_handle_irq_desc(desc);
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
