// SPDX-License-Identifier: GPL-2.0-only
/*
 *	Apple Peripheral System Controller (PSC)
 *
 *	The PSC is used on the AV Macs to control IO functions not handled
 *	by the VIAs (Ethernet, DSP, SCC).
 *
 * TO DO:
 *
 * Try to figure out what's going on in pIFR5 and pIFR6. There seem to be
 * persisant interrupt conditions in those registers and I have no idea what
 * they are. Granted it doesn't affect since we're not enabling any interrupts
 * on those levels at the moment, but it would be nice to know. I have a feeling
 * they aren't actually interrupt lines but data lines (to the DSP?)
 */

// Dependencies supplied by the surrounding kernel translation.

pub static mut psc: *mut u8 = core::ptr::null_mut();

/*
 * Debugging dump, used in various places to see what's going on.
 */

unsafe fn psc_debug_dump() {
    let mut i: i32;

    if psc.is_null() {
        return;
    }

    i = 0x30;
    while i < 0x70 {
        printk(KERN_DEBUG "PSC #%d:  IFR = 0x%02X IER = 0x%02X\n",
            i >> 4,
            psc_read_byte(pIFRbase + i),
            psc_read_byte(pIERbase + i));
        i += 0x10;
    }
}

/*
 * Try to kill all DMA channels on the PSC. Not sure how this his
 * supposed to work; this is code lifted from macmace.c and then
 * expanded to cover what I think are the other 7 channels.
 */

unsafe fn psc_dma_die_die_die() {
    let mut i: i32 = 0;

    while i < 9 {
        psc_write_word(PSC_CTL_BASE + (i << 4), 0x8800);
        psc_write_word(PSC_CTL_BASE + (i << 4), 0x1000);
        psc_write_word(PSC_CMD_BASE + (i << 5), 0x1100);
        psc_write_word(PSC_CMD_BASE + (i << 5) + 0x10, 0x1100);
        i += 1;
    }
}

/*
 * Initialize the PSC. For now this just involves shutting down all
 * interrupt sources using the IERs.
 */

pub unsafe fn psc_init() {
    let mut i: i32;

    if (*macintosh_config).ident != MAC_MODEL_C660
        && (*macintosh_config).ident != MAC_MODEL_Q840
    {
        psc = core::ptr::null_mut();
        return;
    }

    /*
     * The PSC is always at the same spot, but using psc
     * keeps things consistent with the psc_xxxx functions.
     */

    psc = PSC_BASE as *mut u8;

    pr_debug!("PSC detected at %p\n", psc);

    psc_dma_die_die_die();

    #[cfg(feature = "DEBUG_PSC")]
    psc_debug_dump();

    /*
     * Mask and clear all possible interrupts
     */

    i = 0x30;
    while i < 0x70 {
        psc_write_byte(pIERbase + i, 0x0F);
        psc_write_byte(pIFRbase + i, 0x0F);
        i += 0x10;
    }
}

/*
 * PSC interrupt handler. It's a lot like the VIA interrupt handler.
 */

unsafe fn psc_irq(desc: *mut irq_desc) {
    let offset: u32 = irq_desc_get_handler_data(desc) as u32;
    let irq: u32 = irq_desc_get_irq(desc);
    let pIFR: i32 = pIFRbase + offset as i32;
    let pIER: i32 = pIERbase + offset as i32;
    let mut irq_num: i32;
    let mut irq_bit: u8;
    let events: u8 = psc_read_byte(pIFR) & psc_read_byte(pIER) & 0xF;

    if events == 0 {
        return;
    }

    irq_num = (irq << 3) as i32;
    irq_bit = 1;
    loop {
        if events & irq_bit != 0 {
            psc_write_byte(pIFR, irq_bit);
            generic_handle_irq(irq_num as u32);
        }
        irq_num += 1;
        irq_bit <<= 1;
        if events < irq_bit {
            break;
        }
    }
}

/*
 * Register the PSC interrupt dispatchers for autovector interrupts 3-6.
 */

pub unsafe fn psc_register_interrupts() {
    irq_set_chained_handler_and_data(IRQ_AUTO_3, psc_irq, 0x30 as *mut core::ffi::c_void);
    irq_set_chained_handler_and_data(IRQ_AUTO_4, psc_irq, 0x40 as *mut core::ffi::c_void);
    irq_set_chained_handler_and_data(IRQ_AUTO_5, psc_irq, 0x50 as *mut core::ffi::c_void);
    irq_set_chained_handler_and_data(IRQ_AUTO_6, psc_irq, 0x60 as *mut core::ffi::c_void);
}

pub unsafe fn psc_irq_enable(irq: i32) {
    let irq_src: i32 = IRQ_SRC(irq);
    let irq_idx: i32 = IRQ_IDX(irq);
    let pIER: i32 = pIERbase + (irq_src << 4);

    psc_write_byte(pIER, ((1i32 << irq_idx) | 0x80) as u8);
}

pub unsafe fn psc_irq_disable(irq: i32) {
    let irq_src: i32 = IRQ_SRC(irq);
    let irq_idx: i32 = IRQ_IDX(irq);
    let pIER: i32 = pIERbase + (irq_src << 4);

    psc_write_byte(pIER, (1i32 << irq_idx) as u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
