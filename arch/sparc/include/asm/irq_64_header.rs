/* SPDX-License-Identifier: GPL-2.0 */
/* irq.h: IRQ registers on the 64-bit Sparc.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1998 Jakub Jelinek (jj@ultra.linux.cz)
 */

// Dependencies supplied by the surrounding translation unit:
// linux/linkage.h, linux/kernel.h, linux/errno.h, linux/interrupt.h,
// asm/pil.h, and asm/ptrace.h.

/* IMAP/ICLR register defines */
pub const IMAP_VALID: u64 = 0x80000000u64; /* IRQ Enabled */
pub const IMAP_TID_UPA: u64 = 0x7c000000u64; /* UPA TargetID */
pub const IMAP_TID_JBUS: u64 = 0x7c000000u64; /* JBUS TargetID */
pub const IMAP_TID_SHIFT: u32 = 26;
pub const IMAP_AID_SAFARI: u64 = 0x7c000000u64; /* Safari AgentID */
pub const IMAP_AID_SHIFT: u32 = 26;
pub const IMAP_NID_SAFARI: u64 = 0x03e00000u64; /* Safari NodeID */
pub const IMAP_NID_SHIFT: u32 = 21;
pub const IMAP_IGN: u64 = 0x000007c0u64; /* IRQ Group Number */
pub const IMAP_INO: u64 = 0x0000003fu64; /* IRQ Number */
pub const IMAP_INR: u64 = 0x000007ffu64; /* Full interrupt number */

pub const ICLR_IDLE: u64 = 0x00000000u64; /* Idle state */
pub const ICLR_TRANSMIT: u64 = 0x00000001u64; /* Transmit state */
pub const ICLR_PENDING: u64 = 0x00000003u64; /* Pending state */

/* The largest number of unique interrupt sources we support.
 * If this needs to ever be larger than 255, you need to change
 * the type of ino_bucket->irq as appropriate.
 *
 * ino_bucket->irq allocation is made during {sun4v_,}build_irq().
 */
pub const NR_IRQS: usize = 2048;

extern "C" {
    pub fn irq_install_pre_handler(
        irq: ::core::ffi::c_int,
        func: Option<unsafe extern "C" fn(u32, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void)>,
        arg1: *mut ::core::ffi::c_void,
        arg2: *mut ::core::ffi::c_void,
    );
    pub fn build_irq(inofixup: ::core::ffi::c_int, iclr: u64, imap: u64) -> u32;
    pub fn sun4v_build_irq(devhandle: u32, devino: u32) -> u32;
    pub fn sun4v_build_virq(devhandle: u32, devino: u32) -> u32;
    pub fn sun4v_build_msi(devhandle: u32, irq_p: *mut u32, msi_devino_start: u32, msi_devino_end: u32) -> u32;
    pub fn sun4v_destroy_msi(irq: u32);
    pub fn sun4u_build_msi(portid: u32, irq_p: *mut u32, msi_devino_start: u32, msi_devino_end: u32, imap_base: u64, iclr_base: u64) -> u32;
    pub fn sun4u_destroy_msi(irq: u32);
    pub fn irq_alloc(dev_handle: u32, dev_ino: u32) -> u32;
    pub fn irq_free(irq: u32);
    pub fn fixup_irqs();
    pub fn arch_trigger_cpumask_backtrace(mask: *const cpumask, exclude_cpu: ::core::ffi::c_int);
}

#[allow(non_camel_case_types)]
pub enum cpumask {}

#[inline]
pub unsafe fn set_softint(bits: u64) {
    core::arch::asm!("wr {0}, 0x0, %set_softint", in(reg) bits);
}

#[inline]
pub unsafe fn clear_softint(bits: u64) {
    core::arch::asm!("wr {0}, 0x0, %clear_softint", in(reg) bits);
}

#[inline]
pub unsafe fn get_softint() -> u64 {
    let mut retval: u64;
    core::arch::asm!("rd %softint, {0}", out(reg) retval);
    retval
}

// #define irq_canonicalize(irq) (irq)
#[inline]
pub const fn irq_canonicalize<T>(irq: T) -> T { irq }

// #define arch_trigger_cpumask_backtrace arch_trigger_cpumask_backtrace

extern "C" {
    pub static mut hardirq_stack: [*mut ::core::ffi::c_void; NR_CPUS];
    pub static mut softirq_stack: [*mut ::core::ffi::c_void; NR_CPUS];
}

// NR_CPUS is supplied by the surrounding translation unit.
#[allow(non_upper_case_globals)]
pub const NO_IRQ: u32 = 0xffffffffu32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
