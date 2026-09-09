/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for using the Apple Descriptor-Based DMA controller
 * in Power Macintosh computers.
 *
 * Copyright (C) 1996 Paul Mackerras.
 */

/* The original declarations are guarded by __KERNEL__ and _ASM_DBDMA_H_. */

/*
 * DBDMA control/status registers.  All little-endian.
 */
#[repr(C)]
pub struct dbdma_regs {
    pub control: u32, /* lets you change bits in status */
    pub status: u32, /* DMA and device status bits (see below) */
    pub cmdptr_hi: u32, /* upper 32 bits of command address */
    pub cmdptr: u32, /* (lower 32 bits of) command address (phys) */
    pub intr_sel: u32, /* select interrupt condition bit */
    pub br_sel: u32, /* select branch condition bit */
    pub wait_sel: u32, /* select wait condition bit */
    pub xfer_mode: u32,
    pub data2ptr_hi: u32,
    pub data2ptr: u32,
    pub res1: u32,
    pub address_hi: u32,
    pub br_addr_hi: u32,
    pub res2: [u32; 3],
}

/* Bits in control and status registers */
pub const RUN: u32 = 0x8000;
pub const PAUSE: u32 = 0x4000;
pub const FLUSH: u32 = 0x2000;
pub const WAKE: u32 = 0x1000;
pub const DEAD: u32 = 0x0800;
pub const ACTIVE: u32 = 0x0400;
pub const BT: u32 = 0x0100;
pub const DEVSTAT: u32 = 0x00ff;

/*
 * DBDMA command structure.  These fields are all little-endian!
 */
#[repr(C)]
pub struct dbdma_cmd {
    pub req_count: u16, /* requested byte transfer count */
    pub command: u16, /* command word (has bit-fields) */
    pub phy_addr: u32, /* physical data address */
    pub cmd_dep: u32, /* command-dependent field */
    pub res_count: u16, /* residual count after completion */
    pub xfer_status: u16, /* transfer status */
}

/* DBDMA command values in command field */
pub const OUTPUT_MORE: u16 = 0;
pub const OUTPUT_LAST: u16 = 0x1000;
pub const INPUT_MORE: u16 = 0x2000;
pub const INPUT_LAST: u16 = 0x3000;
pub const STORE_WORD: u16 = 0x4000;
pub const LOAD_WORD: u16 = 0x5000;
pub const DBDMA_NOP: u16 = 0x6000;
pub const DBDMA_STOP: u16 = 0x7000;

/* Key values in command field */
pub const KEY_STREAM0: u16 = 0;
pub const KEY_STREAM1: u16 = 0x100;
pub const KEY_STREAM2: u16 = 0x200;
pub const KEY_STREAM3: u16 = 0x300;
pub const KEY_REGS: u16 = 0x500;
pub const KEY_SYSTEM: u16 = 0x600;
pub const KEY_DEVICE: u16 = 0x700;

/* Interrupt control values in command field */
pub const INTR_NEVER: u16 = 0;
pub const INTR_IFSET: u16 = 0x10;
pub const INTR_IFCLR: u16 = 0x20;
pub const INTR_ALWAYS: u16 = 0x30;

/* Branch control values in command field */
pub const BR_NEVER: u16 = 0;
pub const BR_IFSET: u16 = 0x4;
pub const BR_IFCLR: u16 = 0x8;
pub const BR_ALWAYS: u16 = 0xc;

/* Wait control values in command field */
pub const WAIT_NEVER: u16 = 0;
pub const WAIT_IFSET: u16 = 1;
pub const WAIT_IFCLR: u16 = 2;
pub const WAIT_ALWAYS: u16 = 3;

/* Align an address for a DBDMA command structure */
#[macro_export]
macro_rules! DBDMA_ALIGN {
    ($x:expr) => {
        ((($x as usize) + core::mem::size_of::<$crate::dbdma_cmd>() - 1)
            & !((core::mem::size_of::<$crate::dbdma_cmd>()) - 1))
    };
}

/* Useful macros.  These functions are supplied by the surrounding platform. */
unsafe extern "C" {
    fn out_le32(addr: *mut u32, value: u32);
    fn in_le32(addr: *const u32) -> u32;
}

#[macro_export]
macro_rules! DBDMA_DO_STOP {
    ($regs:expr) => {{
        unsafe {
            out_le32(&mut (*($regs)).control, ($crate::RUN | $crate::FLUSH) << 16);
            while (in_le32(&(*($regs)).status) & ($crate::ACTIVE | $crate::FLUSH)) != 0 {}
        }
    }};
}

#[macro_export]
macro_rules! DBDMA_DO_RESET {
    ($regs:expr) => {{
        unsafe {
            out_le32(
                &mut (*($regs)).control,
                ($crate::ACTIVE | $crate::DEAD | $crate::WAKE | $crate::FLUSH |
                    $crate::PAUSE | $crate::RUN) << 16,
            );
            while (in_le32(&(*($regs)).status) & $crate::RUN) != 0 {}
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
