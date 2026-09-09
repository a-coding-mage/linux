/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum pxad_chan_prio {
    PXAD_PRIO_HIGHEST = 0,
    PXAD_PRIO_NORMAL,
    PXAD_PRIO_LOW,
    PXAD_PRIO_LOWEST,
}

/**
 * struct pxad_param - dma channel request parameters
 * @drcmr: requestor line number
 * @prio: minimal mandatory priority of the channel
 *
 * If a requested channel is granted, its priority will be at least @prio,
 * ie. if PXAD_PRIO_LOW is required, the requested channel will be either
 * PXAD_PRIO_LOW, PXAD_PRIO_NORMAL or PXAD_PRIO_HIGHEST.
 */
#[repr(C)]
pub struct pxad_param {
    pub drcmr: u32,
    pub prio: pxad_chan_prio,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
