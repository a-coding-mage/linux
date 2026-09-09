// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/drivers/dma/dma-g2.c
 *
 * G2 bus DMA support
 *
 * Copyright (C) 2003 - 2006  Paul Mundt
 */
// External Linux, SH architecture, machine sysasic, and DMA dependencies are
// supplied by the surrounding translation environment.

#[repr(C, align(32))]
struct g2_channel {
    g2_addr: ::core::ffi::c_ulong,    // G2 bus address
    root_addr: ::core::ffi::c_ulong,  // Root bus (SH-4) address
    size: ::core::ffi::c_ulong,       // Size (in bytes), 32-byte aligned
    direction: ::core::ffi::c_ulong,  // Transfer direction
    ctrl: ::core::ffi::c_ulong,       // Transfer control
    chan_enable: ::core::ffi::c_ulong, // Channel enable
    xfer_enable: ::core::ffi::c_ulong, // Transfer enable
    xfer_stat: ::core::ffi::c_ulong,  // Transfer status
}

#[repr(C, align(16))]
struct g2_status {
    g2_addr: ::core::ffi::c_ulong,
    root_addr: ::core::ffi::c_ulong,
    size: ::core::ffi::c_ulong,
    status: ::core::ffi::c_ulong,
}

#[repr(C, align(256))]
struct g2_dma_info {
    channel: [g2_channel; G2_NR_DMA_CHANNELS],
    pad1: [::core::ffi::c_ulong; G2_NR_DMA_CHANNELS],
    wait_state: ::core::ffi::c_ulong,
    pad2: [::core::ffi::c_ulong; 10],
    magic: ::core::ffi::c_ulong,
    status: [g2_status; G2_NR_DMA_CHANNELS],
}

static mut g2_dma: *mut g2_dma_info = 0xa05f7800 as *mut g2_dma_info;

#[inline]
unsafe fn g2_bytes_remaining(i: usize) -> ::core::ffi::c_ulong {
    ((*g2_dma).channel[i].size.wrapping_sub((*g2_dma).status[i].size)) & 0x0fffffff
}

unsafe fn g2_dma_interrupt(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t {
    let _ = irq;
    for i in 0..G2_NR_DMA_CHANNELS {
        if (*g2_dma).status[i].status & 0x20000000 != 0 {
            let bytes = g2_bytes_remaining(i);
            if likely(bytes == 0) {
                let info = dev_id as *mut dma_info;
                let chan = (*info).channels.add(i);
                wake_up(&mut (*chan).wait_queue);
                return IRQ_HANDLED;
            }
        }
    }
    IRQ_NONE
}

unsafe fn g2_enable_dma(chan: *mut dma_channel) -> ::core::ffi::c_int {
    let chan_nr = (*chan).chan as usize;
    (*g2_dma).channel[chan_nr].chan_enable = 1;
    (*g2_dma).channel[chan_nr].xfer_enable = 1;
    0
}

unsafe fn g2_disable_dma(chan: *mut dma_channel) -> ::core::ffi::c_int {
    let chan_nr = (*chan).chan as usize;
    (*g2_dma).channel[chan_nr].chan_enable = 0;
    (*g2_dma).channel[chan_nr].xfer_enable = 0;
    0
}

unsafe fn g2_xfer_dma(chan: *mut dma_channel) -> ::core::ffi::c_int {
    let chan_nr = (*chan).chan as usize;
    if (*chan).sar & 31 != 0 {
        printk!("g2dma: unaligned source 0x%lx\n", (*chan).sar);
        return -EINVAL;
    }
    if (*chan).dar & 31 != 0 {
        printk!("g2dma: unaligned dest 0x%lx\n", (*chan).dar);
        return -EINVAL;
    }
    // Align the count
    if (*chan).count & 31 != 0 {
        (*chan).count = ((*chan).count + (32 - 1)) & !(32 - 1);
    }
    // Fixup destination
    (*chan).dar = (*chan).dar.wrapping_add(0xa0800000);
    // Fixup direction
    (*chan).mode = !(*chan).mode;
    flush_icache_range((*chan).sar as ::core::ffi::c_ulong, (*chan).count);
    g2_disable_dma(chan);
    (*g2_dma).channel[chan_nr].g2_addr = (*chan).dar & 0x1fffffe0;
    (*g2_dma).channel[chan_nr].root_addr = (*chan).sar & 0x1fffffe0;
    (*g2_dma).channel[chan_nr].size = ((*chan).count & !31) | 0x80000000;
    (*g2_dma).channel[chan_nr].direction = (*chan).mode;
    /*
     * bit 0 - ???
     * bit 1 - if set, generate a hardware event on transfer completion
     * bit 2 - ??? something to do with suspend?
     */
    (*g2_dma).channel[chan_nr].ctrl = 5; /* ?? */
    g2_enable_dma(chan);
    pr_debug!("count, sar, dar, mode, ctrl, chan, xfer: %ld, 0x%08lx, 0x%08lx, %ld, %ld, %ld, %ld\n",
        (*g2_dma).channel[chan_nr].size, (*g2_dma).channel[chan_nr].root_addr,
        (*g2_dma).channel[chan_nr].g2_addr, (*g2_dma).channel[chan_nr].direction,
        (*g2_dma).channel[chan_nr].ctrl, (*g2_dma).channel[chan_nr].chan_enable,
        (*g2_dma).channel[chan_nr].xfer_enable);
    0
}

unsafe fn g2_get_residue(chan: *mut dma_channel) -> ::core::ffi::c_int {
    g2_bytes_remaining((*chan).chan as usize) as ::core::ffi::c_int
}

static mut g2_dma_ops: dma_ops = dma_ops { xfer: Some(g2_xfer_dma), get_residue: Some(g2_get_residue) };
static mut g2_dma_info: dma_info = dma_info {
    name: "g2_dmac",
    nr_channels: 4,
    ops: &mut g2_dma_ops,
    flags: DMAC_CHANNELS_TEI_CAPABLE,
};

unsafe fn g2_dma_init() -> ::core::ffi::c_int {
    let mut ret = request_irq(HW_EVENT_G2_DMA, Some(g2_dma_interrupt), 0, "g2 DMA handler", &mut g2_dma_info as *mut _ as *mut _);
    if unlikely(ret != 0) { return -EINVAL; }
    // Magic
    (*g2_dma).wait_state = 27;
    (*g2_dma).magic = 0x4659404f;
    ret = register_dmac(&mut g2_dma_info);
    if unlikely(ret != 0) { free_irq(HW_EVENT_G2_DMA, &mut g2_dma_info as *mut _ as *mut _); }
    ret
}

unsafe fn g2_dma_exit() {
    free_irq(HW_EVENT_G2_DMA, &mut g2_dma_info as *mut _ as *mut _);
    unregister_dmac(&mut g2_dma_info);
}

subsys_initcall!(g2_dma_init);
module_exit!(g2_dma_exit);

module_author!("Paul Mundt <lethal@linux-sh.org>");
module_description!("G2 bus DMA driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
