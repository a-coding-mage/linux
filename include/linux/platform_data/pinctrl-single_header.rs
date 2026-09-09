/* SPDX-License-Identifier: GPL-2.0 */

/**
 * irq:        optional wake-up interrupt
 * rearm:      optional soc specific rearm function
 *
 * Note that the irq and rearm setup should come from device
 * tree except for omap where there are still some dependencies
 * to the legacy PRM code.
 */
#[repr(C)]
pub struct pcs_pdata {
    pub irq: i32,
    pub rearm: Option<unsafe extern "C" fn()>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
