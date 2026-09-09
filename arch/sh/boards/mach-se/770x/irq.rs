// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/se/770x/irq.c
 *
 * Copyright (C) 2000  Kazumoto Kojima
 * Copyright (C) 2006  Nobuhiro Iwamatsu
 *
 * Hitachi SolutionEngine Support.
 *
 */

// Linux and architecture headers supplying the declarations used below.

#[cfg(CONFIG_CPU_SUBTYPE_SH7705)]
static mut IPR_IRQ_TABLE: &[ipr_data] = &[
    // Super I/O (Just mimic PC): keyboard, serial, printer, floppy, rtc, mouse, ide0.
    ipr_data { irq: 13, ipr_idx: 0, shift: 8, mask: 0x0f - 13 },
    ipr_data { irq: 5,  ipr_idx: 0, shift: 4, mask: 0x0f - 5 },
    ipr_data { irq: 10, ipr_idx: 1, shift: 0, mask: 0x0f - 10 },
    ipr_data { irq: 7,  ipr_idx: 2, shift: 4, mask: 0x0f - 7 },
    ipr_data { irq: 3,  ipr_idx: 2, shift: 0, mask: 0x0f - 3 },
    ipr_data { irq: 1,  ipr_idx: 3, shift: 12, mask: 0x0f - 1 },
    ipr_data { irq: 12, ipr_idx: 3, shift: 4, mask: 0x0f - 12 }, // LAN
    ipr_data { irq: 2,  ipr_idx: 4, shift: 8, mask: 0x0f - 2 }, // PCIRQ2
    ipr_data { irq: 6,  ipr_idx: 4, shift: 4, mask: 0x0f - 6 }, // PCIRQ1
    ipr_data { irq: 14, ipr_idx: 4, shift: 0, mask: 0x0f - 14 }, // PCIRQ0
    ipr_data { irq: 0,  ipr_idx: 5, shift: 12, mask: 0x0f },
    ipr_data { irq: 4,  ipr_idx: 5, shift: 4, mask: 0x0f - 4 },
    ipr_data { irq: 8,  ipr_idx: 6, shift: 12, mask: 0x0f - 8 },
    ipr_data { irq: 9,  ipr_idx: 6, shift: 8, mask: 0x0f - 9 },
    ipr_data { irq: 11, ipr_idx: 6, shift: 4, mask: 0x0f - 11 },
];

#[cfg(not(CONFIG_CPU_SUBTYPE_SH7705))]
static mut IPR_IRQ_TABLE: &[ipr_data] = &[
    ipr_data { irq: 14, ipr_idx: 0, shift: 8, mask: 0x0f - 14 },
    ipr_data { irq: 12, ipr_idx: 0, shift: 4, mask: 0x0f - 12 },
    ipr_data { irq: 8,  ipr_idx: 1, shift: 4, mask: 0x0f - 8 },
    ipr_data { irq: 6,  ipr_idx: 2, shift: 12, mask: 0x0f - 6 },
    ipr_data { irq: 5,  ipr_idx: 2, shift: 8, mask: 0x0f - 5 },
    ipr_data { irq: 4,  ipr_idx: 2, shift: 4, mask: 0x0f - 4 },
    ipr_data { irq: 3,  ipr_idx: 2, shift: 0, mask: 0x0f - 3 },
    ipr_data { irq: 1,  ipr_idx: 3, shift: 12, mask: 0x0f - 1 },
    #[cfg(CONFIG_STNIC)]
    ipr_data { irq: 10, ipr_idx: 3, shift: 4, mask: 0x0f - 10 }, // ST NIC / LAN
    ipr_data { irq: 0,  ipr_idx: 4, shift: 12, mask: 0x0f - 0 }, // PCIRQ3
    ipr_data { irq: 11, ipr_idx: 4, shift: 8, mask: 0x0f - 11 }, // PCIRQ2
    ipr_data { irq: 9,  ipr_idx: 4, shift: 4, mask: 0x0f - 9 }, // PCIRQ1
    ipr_data { irq: 7,  ipr_idx: 4, shift: 0, mask: 0x0f - 7 }, // PCIRQ0
    ipr_data { irq: 13, ipr_idx: 6, shift: 4, mask: 0x0f - 13 }, // SLOTIRQ2
    ipr_data { irq: 2,  ipr_idx: 6, shift: 0, mask: 0x0f - 2 }, // SLOTIRQ1
];

static mut IPR_OFFSETS: [c_ulong; 7] = [
    BCR_ILCRA, BCR_ILCRB, BCR_ILCRC, BCR_ILCRD,
    BCR_ILCRE, BCR_ILCRF, BCR_ILCRG,
];

static mut IPR_IRQ_DESC: ipr_desc = ipr_desc {
    ipr_offsets: IPR_OFFSETS.as_ptr(),
    nr_offsets: IPR_OFFSETS.len(),
    ipr_data: IPR_IRQ_TABLE.as_ptr(),
    nr_irqs: IPR_IRQ_TABLE.len(),
    chip: irq_chip { name: b"IPR-se770x\0".as_ptr() as *const c_char },
};

/* Initialize IRQ setting */
pub unsafe extern "C" fn init_se_IRQ() {
    /* Disable all interrupts */
    __raw_writew(0, BCR_ILCRA);
    __raw_writew(0, BCR_ILCRB);
    __raw_writew(0, BCR_ILCRC);
    __raw_writew(0, BCR_ILCRD);
    __raw_writew(0, BCR_ILCRE);
    __raw_writew(0, BCR_ILCRF);
    __raw_writew(0, BCR_ILCRG);

    register_ipr_controller(&mut IPR_IRQ_DESC);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
