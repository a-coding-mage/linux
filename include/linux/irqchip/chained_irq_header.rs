/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Chained IRQ handlers support.
 *
 * Copyright (C) 2011 ARM Ltd.
 */

/* Dependency supplied by <linux/irq.h>. */
#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

/* Dependency supplied by <linux/irq.h>. */
#[repr(C)]
pub struct irq_chip {
    pub irq_eoi: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
}

/* Dependency supplied by <linux/irq.h>. */
#[repr(C)]
pub struct irq_desc {
    pub irq_data: irq_data,
}

/*
 * Entry/exit functions for chained handlers where the primary IRQ chip
 * may implement either fasteoi or level-trigger flow control.
 */
#[inline]
pub unsafe fn chained_irq_enter(chip: *mut irq_chip, desc: *mut irq_desc) {
    /* FastEOI controllers require no action on entry. */
    if (*chip).irq_eoi.is_some() {
        return;
    }

    if let Some(irq_mask_ack) = (*chip).irq_mask_ack {
        irq_mask_ack(&mut (*desc).irq_data);
    } else {
        if let Some(irq_mask) = (*chip).irq_mask {
            irq_mask(&mut (*desc).irq_data);
        }
        if let Some(irq_ack) = (*chip).irq_ack {
            irq_ack(&mut (*desc).irq_data);
        }
    }
}

#[inline]
pub unsafe fn chained_irq_exit(chip: *mut irq_chip, desc: *mut irq_desc) {
    if let Some(irq_eoi) = (*chip).irq_eoi {
        irq_eoi(&mut (*desc).irq_data);
    } else if let Some(irq_unmask) = (*chip).irq_unmask {
        irq_unmask(&mut (*desc).irq_data);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
