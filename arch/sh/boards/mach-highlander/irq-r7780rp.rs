// SPDX-License-Identifier: GPL-2.0
/*
 * Renesas Solutions Highlander R7780RP-1 Support.
 *
 * Copyright (C) 2002  Atom Create Engineering Co., Ltd.
 * Copyright (C) 2006  Paul Mundt
 * Copyright (C) 2008  Magnus Damm
 */

// Linux kernel dependencies supplied by the surrounding tree.

#[repr(u32)]
#[derive(Copy, Clone)]
enum BoardInterrupt {
    UNUSED = 0,

    /* board specific interrupt sources */
    AX88796, // Ethernet controller
    PSW,      // Push Switch
    CF,       // Compact Flash

    PCI_A,
    PCI_B,
    PCI_C,
    PCI_D,
}

#[repr(C)]
struct IntcVect {
    enum_id: BoardInterrupt,
    irq: u32,
}

#[repr(C)]
struct IntcMaskReg {
    address: usize,
    set_reg: u32,
    width: u32,
    enum_ids: [BoardInterrupt; 16],
}

#[repr(C)]
struct IntcDesc {
    name: *const u8,
    vectors: *const IntcVect,
    nr_vectors: usize,
    mask_registers: *const IntcMaskReg,
    nr_mask_registers: usize,
}

const IRQ_CF: u32 = 0;
const IRQ_PSW: u32 = 0;
const IRQ_AX88796: u32 = 0;
const HL_NR_IRL: usize = 14;

static mut VECTORS: [IntcVect; 7] = [
    IntcVect { enum_id: BoardInterrupt::PCI_A, irq: 65 }, // dirty: overwrite cpu vectors for pci
    IntcVect { enum_id: BoardInterrupt::PCI_B, irq: 66 },
    IntcVect { enum_id: BoardInterrupt::PCI_C, irq: 67 },
    IntcVect { enum_id: BoardInterrupt::PCI_D, irq: 68 },
    IntcVect { enum_id: BoardInterrupt::CF, irq: IRQ_CF },
    IntcVect { enum_id: BoardInterrupt::PSW, irq: IRQ_PSW },
    IntcVect { enum_id: BoardInterrupt::AX88796, irq: IRQ_AX88796 },
];

static mut MASK_REGISTERS: [IntcMaskReg; 1] = [IntcMaskReg {
    address: 0xa5000000,
    set_reg: 0,
    width: 16,
    enum_ids: [
        BoardInterrupt::PCI_A,
        BoardInterrupt::PCI_B,
        BoardInterrupt::PCI_C,
        BoardInterrupt::PCI_D,
        BoardInterrupt::CF,
        BoardInterrupt::UNUSED,
        BoardInterrupt::UNUSED,
        BoardInterrupt::UNUSED,
        BoardInterrupt::UNUSED,
        BoardInterrupt::UNUSED,
        BoardInterrupt::UNUSED,
        BoardInterrupt::UNUSED,
        BoardInterrupt::UNUSED,
        BoardInterrupt::UNUSED,
        BoardInterrupt::PSW,
        BoardInterrupt::AX88796,
    ],
}];

static mut IRL2IRQ: [u8; HL_NR_IRL] = [
    65, 66, 67, 68,
    IRQ_CF as u8, 0, 0, 0,
    0, 0, 0, 0,
    IRQ_AX88796 as u8, IRQ_PSW as u8,
];

static INTC_DESC: IntcDesc = IntcDesc {
    name: b"r7780rp\0".as_ptr(),
    vectors: unsafe { VECTORS.as_ptr() },
    nr_vectors: 7,
    mask_registers: unsafe { MASK_REGISTERS.as_ptr() },
    nr_mask_registers: 1,
};

extern "C" {
    fn __raw_readw(address: usize) -> u16;
    fn printk(format: *const u8, ...);
    fn register_intc_controller(desc: *const IntcDesc);
}

static KERN_INFO: &[u8] = b"<6>\0";

pub unsafe fn highlander_plat_irq_setup() -> *mut u8 {
    if __raw_readw(0xa5000600) != 0 {
        printk(
            b"Using r7780rp interrupt controller.\n\0".as_ptr(),
        );
        register_intc_controller(&INTC_DESC);
        return IRL2IRQ.as_mut_ptr();
    }

    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
