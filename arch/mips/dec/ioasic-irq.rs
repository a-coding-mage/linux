// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	DEC I/O ASIC interrupts.
 *
 *	Copyright (c) 2002, 2003, 2013  Maciej W. Rozycki
 */

// External kernel and DEC I/O ASIC dependencies are supplied by the surrounding build.

static mut ioasic_irq_base: ::core::ffi::c_int = 0;

unsafe fn unmask_ioasic_irq(d: *mut irq_data) {
    let mut simr: u32;

    simr = ioasic_read(IO_REG_SIMR);
    simr |= 1u32 << ((*d).irq - ioasic_irq_base as u32);
    ioasic_write(IO_REG_SIMR, simr);
}

unsafe fn mask_ioasic_irq(d: *mut irq_data) {
    let mut simr: u32;

    simr = ioasic_read(IO_REG_SIMR);
    simr &= !(1u32 << ((*d).irq - ioasic_irq_base as u32));
    ioasic_write(IO_REG_SIMR, simr);
}

unsafe fn ack_ioasic_irq(d: *mut irq_data) {
    mask_ioasic_irq(d);
    fast_iob();
}

static mut ioasic_irq_type: irq_chip = irq_chip {
    name: "IO-ASIC\0".as_ptr() as *const ::core::ffi::c_char,
    irq_ack: Some(ack_ioasic_irq),
    irq_mask: Some(mask_ioasic_irq),
    irq_mask_ack: Some(ack_ioasic_irq),
    irq_unmask: Some(unmask_ioasic_irq),
};

unsafe fn clear_ioasic_dma_irq(d: *mut irq_data) {
    let sir: u32;

    sir = !(1u32 << ((*d).irq - ioasic_irq_base as u32));
    ioasic_write(IO_REG_SIR, sir);
    fast_iob();
}

static mut ioasic_dma_irq_type: irq_chip = irq_chip {
    name: "IO-ASIC-DMA\0".as_ptr() as *const ::core::ffi::c_char,
    irq_ack: Some(clear_ioasic_dma_irq),
    irq_mask: Some(mask_ioasic_irq),
    irq_unmask: Some(unmask_ioasic_irq),
    irq_eoi: Some(clear_ioasic_dma_irq),
};

/*
 * I/O ASIC implements two kinds of DMA interrupts, informational and
 * error interrupts.
 *
 * The former do not stop DMA and should be cleared as soon as possible
 * so that if they retrigger before the handler has completed, usually as
 * a side effect of actions taken by the handler, then they are reissued.
 * These use the `handle_edge_irq' handler that clears the request right
 * away.
 *
 * The latter stop DMA and do not resume it until the interrupt has been
 * cleared.  This cannot be done until after a corrective action has been
 * taken and this also means they will not retrigger.  Therefore they use
 * the `handle_fasteoi_irq' handler that only clears the request on the
 * way out.
 *
 * This mask has `1' bits in the positions of informational interrupts.
 */
const IO_IRQ_DMA_INFO: u32 =
    IO_IRQ_MASK(IO_INR_SCC0A_RXDMA)
        | IO_IRQ_MASK(IO_INR_SCC1A_RXDMA)
        | IO_IRQ_MASK(IO_INR_ISDN_TXDMA)
        | IO_IRQ_MASK(IO_INR_ISDN_RXDMA)
        | IO_IRQ_MASK(IO_INR_ASC_DMA);

unsafe fn init_ioasic_irqs(base: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int;

    /* Mask interrupts. */
    ioasic_write(IO_REG_SIMR, 0);
    fast_iob();

    i = base;
    while i < base + IO_INR_DMA {
        irq_set_chip_and_handler(i, &raw mut ioasic_irq_type, handle_level_irq);
        i += 1;
    }
    while i < base + IO_IRQ_LINES {
        irq_set_chip_and_handler(
            i,
            &raw mut ioasic_dma_irq_type,
            if (1u32 << (i - base)) & IO_IRQ_DMA_INFO != 0 {
                handle_edge_irq
            } else {
                handle_fasteoi_irq
            },
        );
        i += 1;
    }

    ioasic_irq_base = base;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
