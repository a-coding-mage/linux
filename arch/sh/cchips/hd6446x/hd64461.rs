// SPDX-License-Identifier: GPL-2.0
/*
 *	Copyright (C) 2000 YAEGASHI Takeshi
 *	Hitachi HD64461 companion chip support
 */

// C dependencies supplied by the surrounding kernel translation.

/* This belongs in cpu specific */
const INTC_ICR1: usize = 0xA4140010;

unsafe fn hd64461_mask_irq(data: *mut irq_data) {
    let irq: c_uint = (*data).irq;
    let mut nimr: c_ushort;
    let mask: c_ushort = (1u32 << (irq - HD64461_IRQBASE)) as c_ushort;

    nimr = __raw_readw(HD64461_NIMR);
    nimr |= mask;
    __raw_writew(nimr, HD64461_NIMR);
}

unsafe fn hd64461_unmask_irq(data: *mut irq_data) {
    let irq: c_uint = (*data).irq;
    let mut nimr: c_ushort;
    let mask: c_ushort = (1u32 << (irq - HD64461_IRQBASE)) as c_ushort;

    nimr = __raw_readw(HD64461_NIMR);
    nimr &= !mask;
    __raw_writew(nimr, HD64461_NIMR);
}

unsafe fn hd64461_mask_and_ack_irq(data: *mut irq_data) {
    hd64461_mask_irq(data);

    // #ifdef CONFIG_HD64461_ENABLER
    #[cfg(CONFIG_HD64461_ENABLER)]
    {
        if (*data).irq == HD64461_IRQBASE + 13 {
            __raw_writeb(0x00, HD64461_PCC1CSCR);
        }
    }
}

static mut hd64461_irq_chip: irq_chip = irq_chip {
    name: "HD64461-IRQ",
    irq_mask: Some(hd64461_mask_irq),
    irq_mask_ack: Some(hd64461_mask_and_ack_irq),
    irq_unmask: Some(hd64461_unmask_irq),
};

unsafe fn hd64461_irq_demux(desc: *mut irq_desc) {
    let mut intv: c_ushort = __raw_readw(HD64461_NIRR);
    let mut ext_irq: c_uint = HD64461_IRQBASE;

    intv &= ((1u32 << HD64461_IRQ_NUM) - 1) as c_ushort;

    while intv != 0 {
        if (intv & 1) != 0 {
            generic_handle_irq(ext_irq);
        }
        intv >>= 1;
        ext_irq += 1;
    }
}

unsafe fn setup_hd64461() -> c_int {
    let mut irq_base: c_int;
    let mut i: c_int;

    printk(
        KERN_INFO,
        "HD64461 configured at 0x%x on irq %d(mapped into %d to %d)\n",
        HD64461_IOBASE,
        CONFIG_HD64461_IRQ,
        HD64461_IRQBASE,
        HD64461_IRQBASE + 15,
    );

    /* Should be at processor specific part.. */
    // #if defined(CONFIG_CPU_SUBTYPE_SH7709)
    #[cfg(CONFIG_CPU_SUBTYPE_SH7709)]
    {
        __raw_writew(0x2240, INTC_ICR1);
    }
    __raw_writew(0xffff, HD64461_NIMR);

    irq_base = irq_alloc_descs(HD64461_IRQBASE, HD64461_IRQBASE, 16, -1);
    if IS_ERR_VALUE(irq_base) {
        pr_err!("%s: failed hooking irqs for HD64461\n", "setup_hd64461");
        return irq_base;
    }

    i = 0;
    while i < 16 {
        irq_set_chip_and_handler(
            irq_base + i,
            &mut hd64461_irq_chip,
            handle_level_irq,
        );
        i += 1;
    }

    irq_set_chained_handler(CONFIG_HD64461_IRQ, hd64461_irq_demux);
    irq_set_irq_type(CONFIG_HD64461_IRQ, IRQ_TYPE_LEVEL_LOW);

    // #ifdef CONFIG_HD64461_ENABLER
    #[cfg(CONFIG_HD64461_ENABLER)]
    {
        printk(KERN_INFO, "HD64461: enabling PCMCIA devices\n");
        __raw_writeb(0x4c, HD64461_PCC1CSCIER);
        __raw_writeb(0x00, HD64461_PCC1CSCR);
    }

    0
}

// module_init(setup_hd64461);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
