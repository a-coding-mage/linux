// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1992, 1998-2006 Linus Torvalds, Ingo Molnar
 * Copyright (C) 2005-2006, Thomas Gleixner, Russell King
 *
 * This file contains the dummy interrupt chip implementation
 */

/* Dependencies supplied by the surrounding kernel translation. */

/*
 * What should we do if we get a hw irq event on an illegal vector?
 * Each architecture has to answer this themselves.
 */
unsafe fn ack_bad(data: *mut irq_data) {
    let desc: *mut irq_desc = irq_data_to_desc(data);

    print_irq_desc((*data).irq, desc);
    ack_bad_irq((*data).irq);
}

/*
 * NOP functions
 */
unsafe fn noop(_data: *mut irq_data) {}

unsafe fn noop_ret(_data: *mut irq_data) -> u32 {
    0
}

/*
 * Generic no controller implementation
 */
pub static mut no_irq_chip: irq_chip = irq_chip {
    name: "none",
    irq_startup: Some(noop_ret),
    irq_shutdown: Some(noop),
    irq_enable: Some(noop),
    irq_disable: Some(noop),
    irq_ack: Some(ack_bad),
    flags: IRQCHIP_SKIP_SET_WAKE,
};

/*
 * Generic dummy implementation which can be used for
 * real dumb interrupt sources
 */
pub static mut dummy_irq_chip: irq_chip = irq_chip {
    name: "dummy",
    irq_startup: Some(noop_ret),
    irq_shutdown: Some(noop),
    irq_enable: Some(noop),
    irq_disable: Some(noop),
    irq_ack: Some(noop),
    irq_mask: Some(noop),
    irq_unmask: Some(noop),
    flags: IRQCHIP_SKIP_SET_WAKE,
};

/* EXPORT_SYMBOL_GPL(dummy_irq_chip); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
