/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994 by Waldorf GMBH, written by Ralf Baechle
 * Copyright (C) 1995, 96, 97, 98, 99, 2000, 01, 02, 03 by Ralf Baechle
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left unresolved here: THREAD_SIZE, NR_CPUS, I8259A_IRQ_BASE, and cpumask.

pub const IRQ_STACK_SIZE: usize = THREAD_SIZE;
pub const IRQ_STACK_START: usize = IRQ_STACK_SIZE - 16;

extern "C" {
    pub static mut irq_stack: [*mut core::ffi::c_void; NR_CPUS];
}

/*
 * The highest address on the IRQ stack contains a dummy frame put down in
 * genex.S (handle_int & except_vec_vi_handler) which is structured as follows:
 *
 *   top ------------
 *       | task sp  | <- irq_stack[cpu] + IRQ_STACK_START
 *       ------------
 *       |          | <- First frame of IRQ context
 *       ------------
 *
 * task sp holds a copy of the task stack pointer where the struct pt_regs
 * from exception entry can be found.
 */

#[inline]
pub unsafe fn on_irq_stack(cpu: core::ffi::c_int, sp: core::ffi::c_ulong) -> bool {
    let low = irq_stack[cpu as usize] as core::ffi::c_ulong;
    let high = low.wrapping_add(IRQ_STACK_SIZE as core::ffi::c_ulong);

    low <= sp && sp <= high
}

#[cfg(CONFIG_I8259)]
#[inline]
pub const fn irq_canonicalize(irq: core::ffi::c_int) -> core::ffi::c_int {
    if irq == I8259A_IRQ_BASE + 2 {
        I8259A_IRQ_BASE + 9
    } else {
        irq
    }
}

#[cfg(not(CONFIG_I8259))]
#[inline]
pub const fn irq_canonicalize(irq: core::ffi::c_int) -> core::ffi::c_int {
    irq
}

pub const CP0_LEGACY_COMPARE_IRQ: core::ffi::c_int = 7;
pub const CP0_LEGACY_PERFCNT_IRQ: core::ffi::c_int = 7;

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

extern "C" {
    pub fn plat_irq_dispatch();
    pub fn do_IRQ(irq: core::ffi::c_uint);
    pub fn do_domain_IRQ(domain: *mut irq_domain, irq: core::ffi::c_uint);
    pub fn arch_init_irq();
    pub fn spurious_interrupt();

    pub static mut cp0_compare_irq: core::ffi::c_int;
    pub static mut cp0_compare_irq_shift: core::ffi::c_int;
    pub static mut cp0_perfcount_irq: core::ffi::c_int;
    pub static mut cp0_fdc_irq: core::ffi::c_int;

    pub fn get_c0_fdc_int() -> core::ffi::c_int;
    pub fn arch_trigger_cpumask_backtrace(
        mask: *const cpumask,
        exclude_cpu: core::ffi::c_int,
    );
}

// C macro alias: arch_trigger_cpumask_backtrace expands to itself.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
