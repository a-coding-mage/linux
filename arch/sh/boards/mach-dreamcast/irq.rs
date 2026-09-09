// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/dreamcast/irq.c
 *
 * Holly IRQ support for the Sega Dreamcast.
 *
 * Copyright (c) 2001, 2002 M. R. Brown <mrbrown@0xd6.org>
 *
 * This file is part of the LinuxDC project (www.linuxdc.org)
 */

// Dependencies supplied by the kernel and Dreamcast machine-specific headers.

/*
 * Dreamcast System ASIC Hardware Events -
 *
 * The Dreamcast's System ASIC (a.k.a. Holly) is responsible for receiving
 * hardware events from system peripherals and triggering an SH7750 IRQ.
 * Hardware events can trigger IRQs 13, 11, or 9 depending on which bits are
 * set in the Event Mask Registers (EMRs).  When a hardware event is
 * triggered, its corresponding bit in the Event Status Registers (ESRs) is
 * set, and that bit should be rewritten to the ESR to acknowledge that
 * event.
 *
 * There are three 32-bit ESRs located at 0xa05f6900 - 0xa05f6908.  Event
 * types can be found in arch/sh/include/mach-dreamcast/mach/sysasic.h.
 * There are three groups of EMRs that parallel the ESRs.  Each EMR group
 * corresponds to an IRQ, so 0xa05f6910 - 0xa05f6918 triggers IRQ 13,
 * 0xa05f6920 - 0xa05f6928 triggers IRQ 11, and 0xa05f6930 - 0xa05f6938
 * triggers IRQ 9.
 *
 * In the kernel, these events are mapped to virtual IRQs so that drivers can
 * respond to them as they would a normal interrupt.  In order to keep this
 * mapping simple, the events are mapped as:
 *
 * 6900/6910 - Events  0-31, IRQ 13
 * 6904/6924 - Events 32-63, IRQ 11
 * 6908/6938 - Events 64-95, IRQ  9
 */

const ESR_BASE: u32 = 0x005f6900;
const EMR_BASE: u32 = 0x005f6910;

/*
 * Helps us determine the EMR group that this event belongs to: 0 = 0x6910,
 * 1 = 0x6920, 2 = 0x6930; also determine the event offset.
 */
#[inline]
fn level(event: u32) -> u32 {
    (event - HW_EVENT_IRQ_BASE) / 32
}

/* Return the hardware event's bit position within the EMR/ESR */
#[inline]
fn event_bit(event: u32) -> u32 {
    (event - HW_EVENT_IRQ_BASE) & 31
}

/*
 * For each of these *_irq routines, the IRQ passed in is the virtual IRQ
 * (logically mapped to the corresponding bit for the hardware event).
 */

/* Disable the hardware event by masking its bit in its EMR */
#[inline]
unsafe fn disable_systemasic_irq(data: *mut irq_data) {
    let irq: u32 = (*data).irq;
    let emr: u32 = EMR_BASE + (level(irq) << 4) + (level(irq) << 2);
    let mut mask: u32;

    mask = inl(emr);
    mask &= !(1u32 << event_bit(irq));
    outl(mask, emr);
}

/* Enable the hardware event by setting its bit in its EMR */
#[inline]
unsafe fn enable_systemasic_irq(data: *mut irq_data) {
    let irq: u32 = (*data).irq;
    let emr: u32 = EMR_BASE + (level(irq) << 4) + (level(irq) << 2);
    let mut mask: u32;

    mask = inl(emr);
    mask |= 1u32 << event_bit(irq);
    outl(mask, emr);
}

/* Acknowledge a hardware event by writing its bit back to its ESR */
unsafe fn mask_ack_systemasic_irq(data: *mut irq_data) {
    let irq: u32 = (*data).irq;
    let esr: u32 = ESR_BASE + (level(irq) << 2);
    disable_systemasic_irq(data);
    outl(1u32 << event_bit(irq), esr);
}

pub static mut systemasic_int: irq_chip = irq_chip {
    name: "System ASIC",
    irq_mask: Some(disable_systemasic_irq),
    irq_mask_ack: Some(mask_ack_systemasic_irq),
    irq_unmask: Some(enable_systemasic_irq),
};

/*
 * Map the hardware event indicated by the processor IRQ to a virtual IRQ.
 */
pub unsafe fn systemasic_irq_demux(mut irq: i32) -> i32 {
    let emr: u32;
    let esr: u32;
    let mut status: u32;
    let level: u32;
    let mut j: u32;
    let mut bit: u32;

    match irq {
        13 + 16 => level = 0,
        11 + 16 => level = 1,
        9 + 16 => level = 2,
        _ => return irq,
    }
    emr = EMR_BASE + (level << 4) + (level << 2);
    esr = ESR_BASE + (level << 2);

    /* Mask the ESR to filter any spurious, unwanted interrupts */
    status = inl(esr);
    status &= inl(emr);

    /* Now scan and find the first set bit as the event to map */
    bit = 1;
    j = 0;
    while j < 32 {
        if status & bit != 0 {
            irq = (HW_EVENT_IRQ_BASE + j + (level << 5)) as i32;
            return irq;
        }
        bit <<= 1;
        j += 1;
    }

    /* Not reached */
    irq
}

pub unsafe fn systemasic_irq_init() {
    let irq_base: i32;
    let mut i: i32;

    irq_base = irq_alloc_descs(
        HW_EVENT_IRQ_BASE as i32,
        HW_EVENT_IRQ_BASE as i32,
        (HW_EVENT_IRQ_MAX - HW_EVENT_IRQ_BASE) as i32,
        -1,
    );
    if is_err_value(irq_base) {
        pr_err("%s: failed hooking irqs\n", "systemasic_irq_init");
        return;
    }

    i = HW_EVENT_IRQ_BASE as i32;
    while i < HW_EVENT_IRQ_MAX as i32 {
        irq_set_chip_and_handler(i, &mut systemasic_int, handle_level_irq);
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
