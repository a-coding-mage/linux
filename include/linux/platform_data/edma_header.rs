/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  TI EDMA definitions
 *
 *  Copyright (C) 2006-2013 Texas Instruments.
 */

/*
 * This EDMA3 programming framework exposes two basic kinds of resource:
 *
 *  Channel	Triggers transfers, usually from a hardware event but
 *		also manually or by "chaining" from DMA completions.
 *		Each channel is coupled to a Parameter RAM (PaRAM) slot.
 *
 *  Slot	Each PaRAM slot holds a DMA transfer descriptor (PaRAM
 * 	"set"), source and destination addresses, a link to a
 * 	next PaRAM slot (if any), options for the transfer, and
 * 	instructions for updating those addresses.  There are
 * 	more than twice as many slots as event channels.
 *
 * Each PaRAM set describes a sequence of transfers, either for one large
 * buffer or for several discontiguous smaller buffers.  An EDMA transfer
 * is driven only from a channel, which performs the transfers specified
 * in its PaRAM slot until there are no more transfers.  When that last
 * transfer completes, the "link" field may be used to reload the channel's
 * PaRAM slot with a new transfer descriptor.
 *
 * The EDMA Channel Controller (CC) maps requests from channels into physical
 * Transfer Controller (TC) requests when the channel triggers (by hardware
 * or software events, or by chaining).  The two physical DMA channels provided
 * by the TCs are thus shared by many logical channels.
 *
 * DaVinci hardware also has a "QDMA" mechanism which is not currently
 * supported through this interface.  (DSP firmware uses it though.)
 */

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_event_q {
    EVENTQ_0 = 0,
    EVENTQ_1 = 1,
    EVENTQ_2 = 2,
    EVENTQ_3 = 3,
    EVENTQ_DEFAULT = -1,
}

macro_rules! EDMA_CTLR_CHAN {
    ($ctlr:expr, $chan:expr) => {
        (($ctlr << 16) | $chan)
    };
}

macro_rules! EDMA_CTLR {
    ($i:expr) => {
        ($i >> 16)
    };
}

macro_rules! EDMA_CHAN_SLOT {
    ($i:expr) => {
        ($i & 0xffff)
    };
}

macro_rules! EDMA_FILTER_PARAM {
    ($ctlr:expr, $chan:expr) => {
        [EDMA_CTLR_CHAN!($ctlr, $chan)]
    };
}

#[repr(C)]
pub struct edma_rsv_info {
    pub rsv_chans: *const [i16; 2],
    pub rsv_slots: *const [i16; 2],
}

pub struct dma_slave_map;

/* platform_data for EDMA driver */
#[repr(C)]
pub struct edma_soc_info {
    /*
     * Default queue is expected to be a low-priority queue.
     * This way, long transfers on the default queue started
     * by the codec engine will not cause audio defects.
     */
    pub default_queue: dma_event_q,

    /* Resource reservation for other cores */
    pub rsv: *mut edma_rsv_info,

    /* List of channels allocated for memcpy, terminated with -1 */
    pub memcpy_channels: *mut i32,

    pub queue_priority_mapping: *mut [i8; 2],
    pub xbar_chans: *const [i16; 2],

    pub slave_map: *const dma_slave_map,
    pub slavecnt: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
