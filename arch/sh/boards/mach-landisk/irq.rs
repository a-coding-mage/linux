// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/mach-landisk/irq.c
 *
 * I-O DATA Device, Inc. LANDISK Support
 *
 * Copyright (C) 2005-2007 kogiidena
 * Copyright (C) 2011 Nobuhiro Iwamatsu
 *
 * Copyright (C) 2001  Ian da Silva, Jeremy Siegel
 * Based largely on io_se.c.
 */

// Dependencies supplied by the Linux SH platform and LANDISK headers.

enum {
    UNUSED = 0,

    PCI_INTA, // PCI int A
    PCI_INTB, // PCI int B
    PCI_INTC, // PCI int C
    PCI_INTD, // PCI int D
    ATA,      // ATA
    FATA,     // CF
    POWER,    // Power switch
    BUTTON,   // Button switch
}

#[repr(C)]
pub struct IntcVect {
    pub irq: i32,
    pub enum_id: i32,
}

#[repr(C)]
pub struct IntcMaskReg {
    pub addr: usize,
    pub set_reg: i32,
    pub width: i32,
    pub enum_ids: [i32; 8],
}

#[repr(C)]
pub struct IntcDesc {
    pub name: *const core::ffi::c_char,
    pub vectors: *const IntcVect,
    pub nr_vectors: usize,
    pub mask_registers: *const IntcMaskReg,
    pub nr_mask_registers: usize,
}

unsafe extern "C" {
    fn register_intc_controller(desc: *const IntcDesc);
    fn __raw_writeb(value: u8, address: usize);
}

/* Vectors for LANDISK */
#[used]
static VECTORS_LANDISK: [IntcVect; 8] = [
    IntcVect { irq: IRQ_PCIINTA, enum_id: PCI_INTA },
    IntcVect { irq: IRQ_PCIINTB, enum_id: PCI_INTB },
    IntcVect { irq: IRQ_PCIINTC, enum_id: PCI_INTC },
    IntcVect { irq: IRQ_PCIINTD, enum_id: PCI_INTD },
    IntcVect { irq: IRQ_ATA,     enum_id: ATA },
    IntcVect { irq: IRQ_FATA,    enum_id: FATA },
    IntcVect { irq: IRQ_POWER,   enum_id: POWER },
    IntcVect { irq: IRQ_BUTTON,  enum_id: BUTTON },
];

/* IRLMSK mask register layout for LANDISK */
#[used]
static MASK_REGISTERS_LANDISK: [IntcMaskReg; 1] = [IntcMaskReg {
    addr: PA_IMASK,
    set_reg: 0,
    width: 8,
    enum_ids: [BUTTON, POWER, FATA, ATA, PCI_INTD, PCI_INTC, PCI_INTB, PCI_INTA],
}];

static LANDISK_NAME: &[u8] = b"landisk\0";

static INTC_DESC_LANDISK: IntcDesc = IntcDesc {
    name: LANDISK_NAME.as_ptr() as *const core::ffi::c_char,
    vectors: VECTORS_LANDISK.as_ptr(),
    nr_vectors: VECTORS_LANDISK.len(),
    mask_registers: MASK_REGISTERS_LANDISK.as_ptr(),
    nr_mask_registers: MASK_REGISTERS_LANDISK.len(),
};

/*
 * Initialize IRQ setting
 */
pub unsafe fn init_landisk_IRQ() {
    register_intc_controller(&INTC_DESC_LANDISK);
    __raw_writeb(0x00, PA_PWRINT_CLR);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
