/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2010 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * Header file for Samsung SoC UART IRQ demux for S3C64XX and later
 */

#[repr(C)]
pub struct s3c_uart_irq {
    /* C: void __iomem *regs; */
    pub regs: *mut core::ffi::c_void,
    pub base_irq: u32,
    pub parent_irq: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
