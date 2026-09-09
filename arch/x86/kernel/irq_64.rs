// SPDX-License-Identifier: GPL-2.0
/*
 *	Copyright (C) 1992, 1998 Linus Torvalds, Ingo Molnar
 *
 * This file contains the lowest level x86_64-specific interrupt
 * entry and irq statistics code. All the remaining irq logic
 * is done by the generic kernel/irq/ code and in the
 * x86_64-specific irq controller code. (e.g. i8259.c and
 * io_apic.c.)
 */

// Dependencies supplied by the kernel's other translation units are intentionally
// left as external names, as in the original C source.

#[no_mangle]
pub static mut hardirq_stack_inuse: bool = false;

#[no_mangle]
pub static mut irq_stack_backing_store: irq_stack;

#[cfg(feature = "CONFIG_VMAP_STACK")]
/*
 * VMAP the backing store with guard pages
 */
unsafe fn map_irq_stack(cpu: u32) -> i32 {
    let stack = per_cpu_ptr(&raw mut irq_stack_backing_store, cpu) as *mut u8;
    let mut pages: [*mut page; IRQ_STACK_SIZE / PAGE_SIZE] =
        [core::ptr::null_mut(); IRQ_STACK_SIZE / PAGE_SIZE];
    let va: *mut core::ffi::c_void;
    let mut i: usize;

    i = 0;
    while i < IRQ_STACK_SIZE / PAGE_SIZE {
        let pa: phys_addr_t =
            per_cpu_ptr_to_phys(stack.add(i << PAGE_SHIFT));

        pages[i] = pfn_to_page(pa >> PAGE_SHIFT);
        i += 1;
    }

    va = vmap(
        pages.as_mut_ptr(),
        IRQ_STACK_SIZE / PAGE_SIZE,
        VM_MAP,
        PAGE_KERNEL,
    );
    if va.is_null() {
        return -ENOMEM;
    }

    /* Store actual TOS to avoid adjustment in the hotpath */
    per_cpu!(hardirq_stack_ptr, cpu) = va.add(IRQ_STACK_SIZE - 8);
    0
}

#[cfg(not(feature = "CONFIG_VMAP_STACK"))]
/*
 * If VMAP stacks are disabled due to KASAN, just use the per cpu
 * backing store without guard pages.
 */
unsafe fn map_irq_stack(cpu: u32) -> i32 {
    let va = per_cpu_ptr(&raw mut irq_stack_backing_store, cpu) as *mut u8;

    /* Store actual TOS to avoid adjustment in the hotpath */
    per_cpu!(hardirq_stack_ptr, cpu) = va.add(IRQ_STACK_SIZE - 8);
    0
}

#[no_mangle]
pub unsafe extern "C" fn irq_init_percpu_irqstack(cpu: u32) -> i32 {
    if per_cpu!(hardirq_stack_ptr, cpu) != core::ptr::null_mut() {
        return 0;
    }
    map_irq_stack(cpu)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
