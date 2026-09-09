/* SPDX-License-Identifier: GPL-2.0 */
/* irq.h: IRQ registers on the Sparc.
 *
 * Copyright (C) 1995, 2007 David S. Miller (davem@davemloft.net)
 */

/* Allocated number of logical irq numbers.
 * sun4d boxes (ss2000e) should be OK with ~32.
 * Be on the safe side and make room for 64
 */
pub const NR_IRQS: usize = 64;

/* Dependency supplied by the surrounding kernel translation. */

#[allow(improper_ctypes)]
extern "C" {
    /// C declaration carries the `__init` section attribute.
    pub fn sun4d_init_sbi_irq();
}

macro_rules! irq_canonicalize {
    ($irq:expr) => {
        $irq
    };
}

pub(crate) use irq_canonicalize;

pub const NO_IRQ: u32 = 0xffff_ffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
