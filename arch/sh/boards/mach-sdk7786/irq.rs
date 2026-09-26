// SPDX-License-Identifier: GPL-2.0
/*
 * SDK7786 FPGA IRQ Controller Support.
 *
 * Copyright (C) 2010  Matt Fleming
 * Copyright (C) 2010  Paul Mundt
 */

// Dependencies supplied by the surrounding kernel port:
// linux/irq.h, mach/fpga.h, and mach/irq.h

pub const ATA_IRQ_BIT: i32 = 1;
pub const SPI_BUSY_BIT: i32 = 2;
pub const LIRQ5_BIT: i32 = 3;
pub const LIRQ6_BIT: i32 = 4;
pub const LIRQ7_BIT: i32 = 5;
pub const LIRQ8_BIT: i32 = 6;
pub const KEY_IRQ_BIT: i32 = 7;
pub const PEN_IRQ_BIT: i32 = 8;
pub const ETH_IRQ_BIT: i32 = 9;
pub const RTC_ALARM_BIT: i32 = 10;
pub const CRYSTAL_FAIL_BIT: i32 = 12;
pub const ETH_PME_BIT: i32 = 14;

extern "C" {
    fn fpga_write_reg(value: u32, reg: u32);
    fn fpga_read_reg(reg: u32) -> u32;
    fn plat_irq_setup_pins(mode: u32);
}

pub unsafe fn sdk7786_init_irq() {
    let mut tmp: u32;

    /* Enable priority encoding for all IRLs */
    fpga_write_reg(fpga_read_reg(INTMSR) | 0x0303, INTMSR);

    /* Clear FPGA interrupt status registers */
    fpga_write_reg(0x0000, INTASR);
    fpga_write_reg(0x0000, INTBSR);

    /* Unmask FPGA interrupts */
    tmp = fpga_read_reg(INTAMR);
    tmp &= !(1 << ETH_IRQ_BIT);
    fpga_write_reg(tmp, INTAMR);

    plat_irq_setup_pins(IRQ_MODE_IRL7654_MASK);
    plat_irq_setup_pins(IRQ_MODE_IRL3210_MASK);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
