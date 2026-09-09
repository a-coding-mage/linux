/* SPDX-License-Identifier: GPL-2.0 */
/*
 * pi1.h: Definitions for SGI PI1 parallel port
 */

use core::cell::UnsafeCell;

pub type VolatileU8 = UnsafeCell<u8>;

#[repr(C)]
pub struct pi1_regs {
    pub _data: [u8; 3],
    pub data: VolatileU8,
    pub _ctrl: [u8; 3],
    pub ctrl: VolatileU8,
    pub _status: [u8; 3],
    pub status: VolatileU8,
    pub _dmactrl: [u8; 3],
    pub dmactrl: VolatileU8,
    pub _intstat: [u8; 3],
    pub intstat: VolatileU8,
    pub _intmask: [u8; 3],
    pub intmask: VolatileU8,
    pub _timer1: [u8; 3],
    pub timer1: VolatileU8,
    pub _timer2: [u8; 3],
    pub timer2: VolatileU8,
    pub _timer3: [u8; 3],
    pub timer3: VolatileU8,
    pub _timer4: [u8; 3],
    pub timer4: VolatileU8,
}

pub const PI1_CTRL_STROBE_N: u8 = 0x01;
pub const PI1_CTRL_AFD_N: u8 = 0x02;
pub const PI1_CTRL_INIT_N: u8 = 0x04;
pub const PI1_CTRL_SLIN_N: u8 = 0x08;
pub const PI1_CTRL_IRQ_ENA: u8 = 0x10;
pub const PI1_CTRL_DIR: u8 = 0x20;
pub const PI1_CTRL_SEL: u8 = 0x40;

pub const PI1_STAT_DEVID: u8 = 0x03; // bits 0-1
pub const PI1_STAT_NOINK: u8 = 0x04; // SGI MODE only
pub const PI1_STAT_ERROR: u8 = 0x08;
pub const PI1_STAT_ONLINE: u8 = 0x10;
pub const PI1_STAT_PE: u8 = 0x20;
pub const PI1_STAT_ACK: u8 = 0x40;
pub const PI1_STAT_BUSY: u8 = 0x80;

pub const PI1_DMACTRL_FIFO_EMPTY: u8 = 0x01; // fifo empty R/O
pub const PI1_DMACTRL_ABORT: u8 = 0x02; // reset DMA and internal fifo W/O
pub const PI1_DMACTRL_STDMODE: u8 = 0x00; // bits 2-3
pub const PI1_DMACTRL_SGIMODE: u8 = 0x04; // bits 2-3
pub const PI1_DMACTRL_RICOHMODE: u8 = 0x08; // bits 2-3
pub const PI1_DMACTRL_HPMODE: u8 = 0x0c; // bits 2-3
pub const PI1_DMACTRL_BLKMODE: u8 = 0x10; // block mode
pub const PI1_DMACTRL_FIFO_CLEAR: u8 = 0x20; // clear fifo W/O
pub const PI1_DMACTRL_READ: u8 = 0x40; // read
pub const PI1_DMACTRL_RUN: u8 = 0x80; // pedal to the metal

pub const PI1_INTSTAT_ACK: u8 = 0x04;
pub const PI1_INTSTAT_FEMPTY: u8 = 0x08;
pub const PI1_INTSTAT_NOINK: u8 = 0x10;
pub const PI1_INTSTAT_ONLINE: u8 = 0x20;
pub const PI1_INTSTAT_ERR: u8 = 0x40;
pub const PI1_INTSTAT_PE: u8 = 0x80;

pub const PI1_INTMASK_ACK: u8 = 0x04;
pub const PI1_INTMASK_FIFO_EMPTY: u8 = 0x08;
pub const PI1_INTMASK_NOINK: u8 = 0x10;
pub const PI1_INTMASK_ONLINE: u8 = 0x20;
pub const PI1_INTMASK_ERR: u8 = 0x40;
pub const PI1_INTMASK_PE: u8 = 0x80;

pub const PI1_TIME1: u8 = 0x27;
pub const PI1_TIME2: u8 = 0x13;
pub const PI1_TIME3: u8 = 0x10;
pub const PI1_TIME4: u8 = 0x00;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
