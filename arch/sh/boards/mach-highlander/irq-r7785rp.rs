// SPDX-License-Identifier: GPL-2.0
/*
 * Renesas Solutions Highlander R7785RP Support.
 *
 * Copyright (C) 2002  Atom Create Engineering Co., Ltd.
 * Copyright (C) 2006 - 2008  Paul Mundt
 * Copyright (C) 2007  Magnus Damm
 */
// C dependencies: linux/init.h, linux/irq.h, linux/io.h, mach/highlander.h

#[repr(i32)]
enum IrqSource {
    UNUSED = 0,

    /* FPGA specific interrupt sources */
    CF,
    SMBUS,
    TP,
    RTC,
    TH_ALERT,
    AX88796,

    /* external bus connector */
    EXT0,
    EXT1,
    EXT2,
    EXT3,
    EXT4,
    EXT5,
    EXT6,
    EXT7,
}

static mut VECTORS: [IntcVect; 14] = [
    INTC_IRQ!(CF, IRQ_CF),
    INTC_IRQ!(SMBUS, IRQ_SMBUS),
    INTC_IRQ!(TP, IRQ_TP),
    INTC_IRQ!(RTC, IRQ_RTC),
    INTC_IRQ!(TH_ALERT, IRQ_TH_ALERT),

    INTC_IRQ!(EXT0, IRQ_EXT0), INTC_IRQ!(EXT1, IRQ_EXT1),
    INTC_IRQ!(EXT2, IRQ_EXT2), INTC_IRQ!(EXT3, IRQ_EXT3),

    INTC_IRQ!(EXT4, IRQ_EXT4), INTC_IRQ!(EXT5, IRQ_EXT5),
    INTC_IRQ!(EXT6, IRQ_EXT6), INTC_IRQ!(EXT7, IRQ_EXT7),

    INTC_IRQ!(AX88796, IRQ_AX88796),
];

static mut MASK_REGISTERS: [IntcMaskReg; 2] = [
    IntcMaskReg {
        addr: 0xa4000010,
        set_reg: 0,
        clr_reg: 16,
        /* IRLMCR1 */
        enum_ids: [0, 0, 0, 0, CF, AX88796, SMBUS, TP,
                   RTC, 0, TH_ALERT, 0, 0, 0, 0, 0],
    },
    IntcMaskReg {
        addr: 0xa4000012,
        set_reg: 0,
        clr_reg: 16,
        /* IRLMCR2 */
        enum_ids: [0, 0, 0, 0, 0, 0, 0, 0,
                   EXT7, EXT6, EXT5, EXT4, EXT3, EXT2, EXT1, EXT0],
    },
];

static mut IRL2IRQ: [u8; HL_NR_IRL] = [
    0, IRQ_CF, IRQ_EXT4, IRQ_EXT5,
    IRQ_EXT6, IRQ_EXT7, IRQ_SMBUS, IRQ_TP,
    IRQ_RTC, IRQ_TH_ALERT, IRQ_AX88796, IRQ_EXT0,
    IRQ_EXT1, IRQ_EXT2, IRQ_EXT3,
];

static mut INTC_DESC: IntcDesc = DECLARE_INTC_DESC!(
    "r7785rp", VECTORS, None, MASK_REGISTERS, None, None
);

pub unsafe fn highlander_plat_irq_setup() -> *mut u8 {
    if (raw_readw(0xa4000158) & 0xf000) != 0x1000 {
        return core::ptr::null_mut();
    }

    printk!(KERN_INFO, "Using r7785rp interrupt controller.\n");

    raw_writew(0x0000, PA_IRLSSR1); // FPGA IRLSSR1(CF_CD clear)

    // Setup the FPGA IRL
    raw_writew(0x0000, PA_IRLPRA); // FPGA IRLA
    raw_writew(0xe598, PA_IRLPRB); // FPGA IRLB
    raw_writew(0x7060, PA_IRLPRC); // FPGA IRLC
    raw_writew(0x0000, PA_IRLPRD); // FPGA IRLD
    raw_writew(0x4321, PA_IRLPRE); // FPGA IRLE
    raw_writew(0xdcba, PA_IRLPRF); // FPGA IRLF

    register_intc_controller(&mut INTC_DESC);
    IRL2IRQ.as_mut_ptr()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
