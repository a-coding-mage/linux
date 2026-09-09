/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014-15 Synopsys, Inc. (www.synopsys.com)
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

/* vineetg: March 2010 : local_irq_save( ) optimisation
 *  -Remove explicit mov of current status32 into reg, that is not needed
 *  -Use BIC  insn instead of INVERTED + AND
 *  -Conditionally disable interrupts (if they are not enabled, don't disable)
 */

/* Dependency: <asm/arcregs.h> */

/* status32 Reg bits related to Interrupt Handling */
pub const STATUS_E1_BIT: u32 = 1;
pub const STATUS_E2_BIT: u32 = 2;
pub const STATUS_A1_BIT: u32 = 3;
pub const STATUS_A2_BIT: u32 = 4;
pub const STATUS_AE_BIT: u32 = 5;

pub const STATUS_E1_MASK: u32 = 1 << STATUS_E1_BIT;
pub const STATUS_E2_MASK: u32 = 1 << STATUS_E2_BIT;
pub const STATUS_A1_MASK: u32 = 1 << STATUS_A1_BIT;
pub const STATUS_A2_MASK: u32 = 1 << STATUS_A2_BIT;
pub const STATUS_AE_MASK: u32 = 1 << STATUS_AE_BIT;
pub const STATUS_IE_MASK: u32 = STATUS_E1_MASK | STATUS_E2_MASK;

/* Other Interrupt Handling related Aux regs */
pub const AUX_IRQ_LEV: u32 = 0x200;
pub const AUX_IRQ_HINT: u32 = 0x201;
pub const AUX_IRQ_LV12: u32 = 0x43;
pub const AUX_IENABLE: u32 = 0x40c;
pub const AUX_ITRIGGER: u32 = 0x40d;
pub const AUX_IPULSE: u32 = 0x415;

pub const ISA_INIT_STATUS_BITS: u32 = STATUS_IE_MASK;

/*
 * IRQ Control functions. These operations carry a compiler memory barrier,
 * matching the "memory" clobber in the original inline assembly.
 */

/* Save IRQ state and disable IRQs */
#[inline(always)]
pub unsafe fn arch_local_irq_save() -> u64 {
    let mut temp: u64;
    let mut flags: u64;
    core::arch::asm!(
        "lr {flags}, [status32]",
        "bic {temp}, {flags}, {mask}",
        "and.f 0, {flags}, {mask}",
        "flag.nz {temp}",
        temp = out(reg) temp,
        flags = out(reg) flags,
        mask = const (STATUS_E1_MASK | STATUS_E2_MASK),
        options(nostack)
    );
    flags
}

/* restore saved IRQ state */
#[inline(always)]
pub unsafe fn arch_local_irq_restore(flags: u64) {
    core::arch::asm!("flag {0}", in(reg) flags, options(nostack));
}

/* Unconditionally Enable IRQs */
#[cfg(CONFIG_ARC_COMPACT_IRQ_LEVELS)]
unsafe extern "C" {
    pub fn arch_local_irq_enable();
}

#[cfg(not(CONFIG_ARC_COMPACT_IRQ_LEVELS))]
#[inline(always)]
pub unsafe fn arch_local_irq_enable() {
    let mut temp: u64;
    core::arch::asm!(
        "lr {temp}, [status32]",
        "or {temp}, {temp}, {mask}",
        "flag {temp}",
        temp = out(reg) temp,
        mask = const (STATUS_E1_MASK | STATUS_E2_MASK),
        options(nostack)
    );
}

/* Unconditionally Disable IRQs */
#[inline(always)]
pub unsafe fn arch_local_irq_disable() {
    let mut temp: u64;
    core::arch::asm!(
        "lr {temp}, [status32]",
        "and {temp}, {temp}, {mask}",
        "flag {temp}",
        temp = out(reg) temp,
        mask = const (!(STATUS_E1_MASK | STATUS_E2_MASK)),
        options(nostack)
    );
}

/* save IRQ state */
#[inline(always)]
pub unsafe fn arch_local_save_flags() -> u64 {
    let mut temp: u64;
    core::arch::asm!("lr {temp}, [status32]", temp = out(reg) temp, options(nostack));
    temp
}

/* Query IRQ state */
#[cfg(CONFIG_ARC_COMPACT_IRQ_LEVELS)]
const IRQ_DISABLED_MASK: u64 = (STATUS_E1_MASK | STATUS_E2_MASK) as u64;
#[cfg(not(CONFIG_ARC_COMPACT_IRQ_LEVELS))]
const IRQ_DISABLED_MASK: u64 = STATUS_E1_MASK as u64;

#[inline(always)]
pub const fn arch_irqs_disabled_flags(flags: u64) -> i32 {
    if (flags & IRQ_DISABLED_MASK) == 0 { 1 } else { 0 }
}

#[inline(always)]
pub unsafe fn arch_irqs_disabled() -> i32 {
    arch_irqs_disabled_flags(arch_local_save_flags())
}

/*
 * The assembler-only TRACE_ASM_IRQ_DISABLE, TRACE_ASM_IRQ_ENABLE, IRQ_DISABLE,
 * and IRQ_ENABLE macros are intentionally retained as comments: Rust has no
 * direct assembler-preprocessor macro equivalent in this header translation.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
