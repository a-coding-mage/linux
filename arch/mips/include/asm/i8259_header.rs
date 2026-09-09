/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	include/asm-mips/i8259.h
 *
 *	i8259A interrupt definitions.
 *
 *	Copyright (C) 2003  Maciej W. Rozycki
 *	Copyright (C) 2003  Ralf Baechle <ralf@linux-mips.org>
 */

/* Dependencies supplied by the surrounding kernel translation. */

/* i8259A PIC registers */
pub const PIC_MASTER_CMD: u8 = 0x20;
pub const PIC_MASTER_IMR: u8 = 0x21;
pub const PIC_MASTER_ISR: u8 = PIC_MASTER_CMD;
pub const PIC_MASTER_POLL: u8 = PIC_MASTER_ISR;
pub const PIC_MASTER_OCW3: u8 = PIC_MASTER_ISR;
pub const PIC_SLAVE_CMD: u8 = 0xa0;
pub const PIC_SLAVE_IMR: u8 = 0xa1;

/* i8259A PIC related value */
pub const PIC_CASCADE_IR: i32 = 2;
pub const MASTER_ICW4_DEFAULT: u8 = 0x01;
pub const SLAVE_ICW4_DEFAULT: u8 = 0x01;
pub const PIC_ICW4_AEOI: u8 = 2;

extern "C" {
    pub static mut i8259A_lock: raw_spinlock_t;

    pub fn make_8259A_irq(irq: u32);

    pub fn init_i8259_irqs();
    /* The __init attribute is a build-time initialization annotation. */
    pub fn __init_i8259_irqs(node: *mut device_node) -> *mut irq_domain;

    /**
     * i8159_set_poll() - Override the i8259 polling function
     * @poll: pointer to platform-specific polling function
     *
     * Call this to override the generic i8259 polling function, which directly
     * accesses i8259 registers, with a platform specific one which may be faster
     * in cases where hardware provides a more optimal means of polling for an
     * interrupt.
     */
    pub fn i8259_set_poll(poll: Option<unsafe extern "C" fn() -> i32>);
}

/*
 * Do the traditional i8259 interrupt polling thing.  This is for the few
 * cases where no better interrupt acknowledge method is available and we
 * absolutely must touch the i8259.
 */
pub unsafe fn i8259_irq() -> i32 {
    let mut irq: i32;

    raw_spin_lock(&raw mut i8259A_lock);

    /* Perform an interrupt acknowledge cycle on controller 1. */
    outb(0x0C, PIC_MASTER_CMD); /* prepare for poll */
    irq = (inb(PIC_MASTER_CMD) & 7) as i32;
    if irq == PIC_CASCADE_IR {
        /*
         * Interrupt is cascaded so perform interrupt
         * acknowledge on controller 2.
         */
        outb(0x0C, PIC_SLAVE_CMD); /* prepare for poll */
        irq = ((inb(PIC_SLAVE_CMD) & 7) + 8) as i32;
    }

    if irq == 7 {
        /*
         * This may be a spurious interrupt.
         *
         * Read the interrupt status register (ISR). If the most
         * significant bit is not set then there is no valid
         * interrupt.
         */
        outb(0x0B, PIC_MASTER_ISR); /* ISR register */
        if !(inb(PIC_MASTER_ISR) & 0x80) != 0 {
            irq = -1;
        }
    }

    raw_spin_unlock(&raw mut i8259A_lock);

    if irq >= 0 { irq + I8259A_IRQ_BASE } else { irq }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
