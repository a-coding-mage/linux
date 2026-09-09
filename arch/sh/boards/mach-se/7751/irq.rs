// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/se/7751/irq.c
 *
 * Copyright (C) 2000  Kazumoto Kojima
 *
 * Hitachi SolutionEngine Support.
 *
 * Modified for 7751 Solution Engine by
 * Ian da Silva and Jeremy Siegel, 2001.
 */

// Dependencies supplied by the surrounding kernel translation.

static mut IPR_IRQ_TABLE: [ipr_data; 1] = [
    ipr_data {
        irq: 13,
        offset: 3,
        shift: 3,
        priority: 2,
    },
    // Add additional entries here as drivers are added and tested.
];

static mut IPR_OFFSETS: [unsigned_long; 7] = [
    BCR_ILCRA,
    BCR_ILCRB,
    BCR_ILCRC,
    BCR_ILCRD,
    BCR_ILCRE,
    BCR_ILCRF,
    BCR_ILCRG,
];

static mut IPR_IRQ_DESC: ipr_desc = ipr_desc {
    ipr_offsets: IPR_OFFSETS.as_ptr(),
    nr_offsets: IPR_OFFSETS.len(),

    ipr_data: IPR_IRQ_TABLE.as_ptr(),
    nr_irqs: IPR_IRQ_TABLE.len(),

    chip: irq_chip {
        name: b"IPR-se7751\0".as_ptr() as *const core::ffi::c_char,
    },
};

/*
 * Initialize IRQ setting
 */
pub unsafe fn init_7751se_IRQ() {
    register_ipr_controller(&mut IPR_IRQ_DESC);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
