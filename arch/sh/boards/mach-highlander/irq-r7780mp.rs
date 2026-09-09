// SPDX-License-Identifier: GPL-2.0
/*
 * Renesas Solutions Highlander R7780MP Support.
 *
 * Copyright (C) 2002  Atom Create Engineering Co., Ltd.
 * Copyright (C) 2006  Paul Mundt
 * Copyright (C) 2007  Magnus Damm
 */

// Linux dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct IntcVect {
    pub irq: u32,
    pub vect: u32,
}

#[repr(C)]
pub struct IntcMaskReg {
    pub address: usize,
    pub set_reg: u32,
    pub width: u32,
    pub enum_id: [u32; 16],
}

extern "C" {
    fn __raw_readw(address: usize) -> u16;
    fn printk(level: u32, message: *const u8, ...);
    fn register_intc_controller(desc: *const IntcDesc);
}

#[repr(C)]
pub struct IntcDesc {
    pub name: *const u8,
    pub vectors: *const IntcVect,
    pub mask_registers: *const IntcMaskReg,
}

// Board-specific interrupt source identifiers.
#[repr(u32)]
enum InterruptSource {
    UNUSED = 0,
    CF,
    TP,
    SCIF1,
    SCIF0,
    SMBUS,
    RTC,
    AX88796,
    PSW,
    EXT1,
    EXT2,
    EXT4,
    EXT5,
    EXT6,
}

// The IRQ_* values and HL_NR_IRL are provided by mach/highlander.h.
extern "C" {
    static IRQ_CF: u32;
    static IRQ_TP: u32;
    static IRQ_SCIF1: u32;
    static IRQ_SCIF0: u32;
    static IRQ_SMBUS: u32;
    static IRQ_RTC: u32;
    static IRQ_AX88796: u32;
    static IRQ_PSW: u32;
    static IRQ_EXT1: u32;
    static IRQ_EXT2: u32;
    static IRQ_EXT4: u32;
    static IRQ_EXT5: u32;
    static IRQ_EXT6: u32;
}

// Equivalent of the INTC_IRQ(C, IRQ) initializer macro.
const fn intc_irq(source: InterruptSource, irq: u32) -> IntcVect {
    IntcVect { irq, vect: source as u32 }
}

static mut VECTORS: [IntcVect; 13] = [
    intc_irq(InterruptSource::CF, unsafe { IRQ_CF }),
    intc_irq(InterruptSource::TP, unsafe { IRQ_TP }),
    intc_irq(InterruptSource::SCIF1, unsafe { IRQ_SCIF1 }),
    intc_irq(InterruptSource::SCIF0, unsafe { IRQ_SCIF0 }),
    intc_irq(InterruptSource::SMBUS, unsafe { IRQ_SMBUS }),
    intc_irq(InterruptSource::RTC, unsafe { IRQ_RTC }),
    intc_irq(InterruptSource::AX88796, unsafe { IRQ_AX88796 }),
    intc_irq(InterruptSource::PSW, unsafe { IRQ_PSW }),
    intc_irq(InterruptSource::EXT1, unsafe { IRQ_EXT1 }),
    intc_irq(InterruptSource::EXT2, unsafe { IRQ_EXT2 }),
    intc_irq(InterruptSource::EXT4, unsafe { IRQ_EXT4 }),
    intc_irq(InterruptSource::EXT5, unsafe { IRQ_EXT5 }),
    intc_irq(InterruptSource::EXT6, unsafe { IRQ_EXT6 }),
];

static mut MASK_REGISTERS: [IntcMaskReg; 1] = [IntcMaskReg {
    address: 0xa4000000,
    set_reg: 0,
    width: 16,
    enum_id: [
        InterruptSource::SCIF0 as u32,
        InterruptSource::SCIF1 as u32,
        InterruptSource::RTC as u32,
        0,
        InterruptSource::CF as u32,
        0,
        InterruptSource::TP as u32,
        InterruptSource::SMBUS as u32,
        0,
        InterruptSource::EXT6 as u32,
        InterruptSource::EXT5 as u32,
        InterruptSource::EXT4 as u32,
        InterruptSource::EXT2 as u32,
        InterruptSource::EXT1 as u32,
        InterruptSource::PSW as u32,
        InterruptSource::AX88796 as u32,
    ],
}];

static mut IRL2IRQ: [u8; 15] = [
    0, unsafe { IRQ_CF as u8 }, unsafe { IRQ_TP as u8 }, unsafe { IRQ_SCIF1 as u8 },
    unsafe { IRQ_SCIF0 as u8 }, unsafe { IRQ_SMBUS as u8 }, unsafe { IRQ_RTC as u8 },
    unsafe { IRQ_EXT6 as u8 }, unsafe { IRQ_EXT5 as u8 }, unsafe { IRQ_EXT4 as u8 },
    unsafe { IRQ_EXT2 as u8 }, unsafe { IRQ_EXT1 as u8 }, 0,
    unsafe { IRQ_AX88796 as u8 }, unsafe { IRQ_PSW as u8 },
];

static mut INTC_DESC: IntcDesc = IntcDesc {
    name: b"r7780mp\0".as_ptr(),
    vectors: unsafe { VECTORS.as_ptr() },
    mask_registers: unsafe { MASK_REGISTERS.as_ptr() },
};

pub unsafe fn highlander_plat_irq_setup() -> *mut u8 {
    if (__raw_readw(0xa4000700) & 0xf000) == 0x2000 {
        printk(0, b"Using r7780mp interrupt controller.\n\0".as_ptr());
        register_intc_controller(&INTC_DESC);
        return IRL2IRQ.as_mut_ptr();
    }

    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
