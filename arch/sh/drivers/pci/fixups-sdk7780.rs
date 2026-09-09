// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/drivers/pci/fixups-sdk7780.c
 *
 * PCI fixups for the SDK7780SE03
 *
 * Copyright (C) 2003  Lineo uSolutions, Inc.
 * Copyright (C) 2004 - 2006  Paul Mundt
 * Copyright (C) 2006  Nobuhiro Iwamatsu
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/pci.h, linux/io.h, linux/sh_intc.h, and pci-sh4.h.

// These values correspond to the C evt2irq() macro.
const IRQ_INTA: i32 = evt2irq(0xa20);
const IRQ_INTB: i32 = evt2irq(0xa40);
const IRQ_INTC: i32 = evt2irq(0xa60);
const IRQ_INTD: i32 = evt2irq(0xa80);

/* IDSEL [16][17][18][19][20][21][22][23][24][25][26][27][28][29][30][31] */
static mut sdk7780_irq_tab: [[i8; 16]; 4] = [
    /* INTA */
    [
        IRQ_INTA as i8, IRQ_INTD as i8, IRQ_INTC as i8, IRQ_INTD as i8,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ],
    /* INTB */
    [
        IRQ_INTB as i8, IRQ_INTA as i8, -1, IRQ_INTA as i8,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ],
    /* INTC */
    [
        IRQ_INTC as i8, IRQ_INTB as i8, -1, IRQ_INTB as i8,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ],
    /* INTD */
    [
        IRQ_INTD as i8, IRQ_INTC as i8, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ],
];

#[no_mangle]
pub unsafe extern "C" fn pcibios_map_platform_irq(
    _pdev: *const pci_dev,
    slot: u8,
    pin: u8,
) -> i32 {
    sdk7780_irq_tab[(pin - 1) as usize][slot as usize] as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
