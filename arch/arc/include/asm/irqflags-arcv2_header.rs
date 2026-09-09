/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014-15 Synopsys, Inc. (www.synopsys.com)
 */

// Dependency supplied by the surrounding ARC platform code: arcregs.h.

/* status32 Bits */
pub const STATUS_AD_BIT: u32 = 19; // Disable Align chk: core supports non-aligned
pub const STATUS_IE_BIT: u32 = 31;

pub const STATUS_AD_MASK: usize = 1usize << STATUS_AD_BIT;
pub const STATUS_IE_MASK: usize = 1usize << STATUS_IE_BIT;

/* status32 Bits as encoded/expected by CLRI/SETI */
pub const CLRI_STATUS_IE_BIT: u32 = 4;

pub const CLRI_STATUS_E_MASK: usize = 0xF;
pub const CLRI_STATUS_IE_MASK: usize = 1usize << CLRI_STATUS_IE_BIT;

pub const AUX_USER_SP: u32 = 0x00D;
pub const AUX_IRQ_CTRL: u32 = 0x00E;
pub const AUX_IRQ_ACT: u32 = 0x043; // Active Intr across all levels
pub const AUX_IRQ_LVL_PEND: u32 = 0x200; // Pending Intr across all levels
pub const AUX_IRQ_HINT: u32 = 0x201; // For generating Soft Interrupts
pub const AUX_IRQ_PRIORITY: u32 = 0x206;
pub const ICAUSE: u32 = 0x40a;
pub const AUX_IRQ_SELECT: u32 = 0x40b;
pub const AUX_IRQ_ENABLE: u32 = 0x40c;

/* Was Intr taken in User Mode */
pub const AUX_IRQ_ACT_BIT_U: u32 = 31;

/*
 * Hardware supports 16 priorities (0 highest, 15 lowest)
 * Linux by default runs at 1, priority 0 reserved for NMI style interrupts
 */
pub const ARCV2_IRQ_DEF_PRIO: usize = 1;

/* Build-time CONFIG_ARC_USE_UNALIGNED_MEM_ACCESS selects STATUS_AD_MASK. */
pub const __AD_ENB: usize = 0; // CONFIG_ARC_USE_UNALIGNED_MEM_ACCESS: STATUS_AD_MASK

pub const ISA_INIT_STATUS_BITS: usize = STATUS_IE_MASK | __AD_ENB | (ARCV2_IRQ_DEF_PRIO << 1);

extern "C" {
    pub fn read_aux_reg(reg: u32) -> u32;
    pub fn write_aux_reg(reg: u32, value: u32);
}

/* Save IRQ state and disable IRQs */
#[inline]
pub unsafe fn arch_local_irq_save() -> i64 {
    let mut flags: usize;
    core::arch::asm!("clri", out(reg) flags, options(nostack, preserves_flags));
    flags as i64
}

/* restore saved IRQ state */
#[inline]
pub unsafe fn arch_local_irq_restore(flags: usize) {
    core::arch::asm!("seti {0}", in(reg) flags, options(nostack, preserves_flags));
}

/* Unconditionally Enable IRQs */
#[inline]
pub unsafe fn arch_local_irq_enable() {
    let irqact = read_aux_reg(AUX_IRQ_ACT);
    if irqact & 0xffff != 0 {
        write_aux_reg(AUX_IRQ_ACT, irqact & !0xffff);
    }
    core::arch::asm!("seti", options(nostack, preserves_flags));
}

/* Unconditionally Disable IRQs */
#[inline]
pub unsafe fn arch_local_irq_disable() {
    core::arch::asm!("clri", options(nostack, preserves_flags));
}

/* save IRQ state */
#[inline]
pub unsafe fn arch_local_save_flags() -> i64 {
    let mut temp: usize;
    core::arch::asm!("lr {0}, [status32]", out(reg) temp, options(nostack));

    /* To be compatible with irq_save()/irq_restore(), encode the irq bits as
     * expected by CLRI/SETI (this was needed to make CONFIG_TRACE_IRQFLAGS work)
     */
    temp = (1usize << 5)
        | (((temp & STATUS_IE_MASK != 0) as usize) << CLRI_STATUS_IE_BIT)
        | ((temp >> 1) & CLRI_STATUS_E_MASK);
    temp as i64
}

/* Query IRQ state */
#[inline]
pub fn arch_irqs_disabled_flags(flags: usize) -> i32 {
    (!(flags & CLRI_STATUS_IE_MASK != 0)) as i32
}

#[inline]
pub unsafe fn arch_irqs_disabled() -> i32 {
    arch_irqs_disabled_flags(arch_local_save_flags() as usize)
}

#[inline]
pub unsafe fn arc_softirq_trigger(irq: i32) {
    write_aux_reg(AUX_IRQ_HINT, irq as u32);
}

#[inline]
pub unsafe fn arc_softirq_clear(_irq: i32) {
    write_aux_reg(AUX_IRQ_HINT, 0);
}

// The __ASSEMBLER__ branch contains ARC assembler macros and is intentionally
// preserved here as source-level documentation rather than executable Rust.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
