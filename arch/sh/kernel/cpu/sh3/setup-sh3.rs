// SPDX-License-Identifier: GPL-2.0
/*
 * Shared SH3 Setup code
 *
 *  Copyright (C) 2008  Magnus Damm
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/irq.h, linux/io.h, and asm/platform_early.h.

/* All SH3 devices are equipped with IRQ0->5 (except sh7708) */

#[allow(dead_code)]
const UNUSED: i32 = 0;

/* interrupt sources */
#[allow(dead_code)]
const IRQ0: i32 = 1;
#[allow(dead_code)]
const IRQ1: i32 = 2;
#[allow(dead_code)]
const IRQ2: i32 = 3;
#[allow(dead_code)]
const IRQ3: i32 = 4;
#[allow(dead_code)]
const IRQ4: i32 = 5;
#[allow(dead_code)]
const IRQ5: i32 = 6;

#[repr(C)]
pub struct intc_vect {
    pub irq: i32,
    pub vector: u32,
}

#[repr(C)]
pub struct intc_prio_reg {
    pub address: usize,
    pub pos: u32,
    pub width: u32,
    pub field_width: u32,
    pub fields: [i32; 4],
}

#[repr(C)]
pub struct intc_mask_reg {
    pub address: usize,
    pub pos: u32,
    pub width: u32,
    pub fields: [i32; 8],
}

#[repr(C)]
pub struct intc_sense_reg {
    pub address: usize,
    pub pos: u32,
    pub width: u32,
    pub fields: [i32; 8],
}

#[repr(C)]
pub struct intc_desc {
    _private: [u8; 0],
}

static mut vectors_irq0123: [intc_vect; 4] = [
    intc_vect { irq: IRQ0, vector: 0x600 },
    intc_vect { irq: IRQ1, vector: 0x620 },
    intc_vect { irq: IRQ2, vector: 0x640 },
    intc_vect { irq: IRQ3, vector: 0x660 },
];

static mut vectors_irq45: [intc_vect; 2] = [
    intc_vect { irq: IRQ4, vector: 0x680 },
    intc_vect { irq: IRQ5, vector: 0x6a0 },
];

static mut prio_registers: [intc_prio_reg; 2] = [
    intc_prio_reg {
        address: 0xa4000016,
        pos: 0,
        width: 16,
        field_width: 4,
        fields: [IRQ3, IRQ2, IRQ1, IRQ0], // IPRC
    },
    intc_prio_reg {
        address: 0xa4000018,
        pos: 0,
        width: 16,
        field_width: 4,
        fields: [UNUSED, UNUSED, IRQ5, IRQ4], // IPRD
    },
];

static mut ack_registers: [intc_mask_reg; 1] = [intc_mask_reg {
    address: 0xa4000004,
    pos: 0,
    width: 8,
    fields: [UNUSED, UNUSED, IRQ5, IRQ4, IRQ3, IRQ2, IRQ1, IRQ0], // IRR0
}];

static mut sense_registers: [intc_sense_reg; 1] = [intc_sense_reg {
    address: 0xa4000010,
    pos: 16,
    width: 2,
    fields: [UNUSED, UNUSED, IRQ5, IRQ4, IRQ3, IRQ2, IRQ1, IRQ0],
}];

// DECLARE_INTC_DESC_ACK(intc_desc_irq0123, "sh3-irq0123", vectors_irq0123,
//                       NULL, NULL, prio_registers, sense_registers,
//                       ack_registers);
// DECLARE_INTC_DESC_ACK(intc_desc_irq45, "sh3-irq45", vectors_irq45,
//                       NULL, NULL, prio_registers, sense_registers,
//                       ack_registers);
extern "C" {
    static intc_desc intc_desc_irq0123;
    static intc_desc intc_desc_irq45;
    fn register_intc_controller(desc: *const intc_desc);
    fn __raw_readw(address: usize) -> u16;
    fn __raw_writew(value: u16, address: usize);
    fn BUG() -> !;
}

const INTC_ICR1: usize = 0xa4000010;
const INTC_ICR1_IRQLVL: u16 = 1 << 14;

pub unsafe fn plat_irq_setup_pins(mode: i32) {
    if mode == IRQ_MODE_IRQ {
        __raw_writew(
            __raw_readw(INTC_ICR1) & !INTC_ICR1_IRQLVL,
            INTC_ICR1,
        );
        register_intc_controller(&intc_desc_irq0123);
        return;
    }
    BUG();
}

pub unsafe fn plat_irq_setup_sh3() {
    register_intc_controller(&intc_desc_irq45);
}

// Build-time platform constant supplied by the kernel headers.
extern "C" {
    static IRQ_MODE_IRQ: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
