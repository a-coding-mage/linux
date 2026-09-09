/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by <linux/dmaengine.h>.

pub const MXS_DMA_CTRL_WAIT4END: u32 = 1u32 << 31;
pub const MXS_DMA_CTRL_WAIT4RDY: u32 = 1u32 << 30;

/*
 * The mxs dmaengine can do PIO transfers. We pass a pointer to the PIO words
 * in the second argument to dmaengine_prep_slave_sg when the direction is
 * set to DMA_TRANS_NONE. To make this clear and to prevent users from doing
 * the error prone casting we have this wrapper function
 */
#[inline]
pub unsafe fn mxs_dmaengine_prep_pio(
    chan: *mut dma_chan,
    pio: *mut u32,
    npio: core::ffi::c_uint,
    dir: dma_transfer_direction,
    flags: core::ffi::c_ulong,
) -> *mut dma_async_tx_descriptor {
    dmaengine_prep_slave_sg(chan, pio as *mut scatterlist, npio, dir, flags)
}

// External types and function supplied by <linux/dmaengine.h>.
extern "C" {
    pub type dma_async_tx_descriptor;
    pub type dma_chan;
    pub type scatterlist;
    pub type dma_transfer_direction;

    pub fn dmaengine_prep_slave_sg(
        chan: *mut dma_chan,
        sg: *mut scatterlist,
        sg_len: core::ffi::c_uint,
        dir: dma_transfer_direction,
        flags: core::ffi::c_ulong,
    ) -> *mut dma_async_tx_descriptor;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
