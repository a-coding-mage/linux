/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * eisa_eeprom.h - provide support for EISA adapters in PA-RISC machines
 *
 * Copyright (c) 2001, 2002 Daniel Engstrom <5116@telia.com>
 */

use core::ffi::c_void;

extern "C" {
    pub static mut eisa_eeprom_addr: *mut c_void;
}

pub const HPEE_MAX_LENGTH: usize = 0x2000; /* maximum eeprom length */

pub const fn HPEE_SLOT_INFO(slot: usize) -> usize { 20 + (48 * slot) }

#[repr(C, packed)]
pub struct eeprom_header {
    pub num_writes: u32, /* number of writes */
    pub flags: u8,       /* flags, usage? */
    pub ver_maj: u8,
    pub ver_min: u8,
    pub num_slots: u8,   /* number of EISA slots in system */
    pub csum: u16,       /* checksum, I don't know how to calculate this */
    pub pad: [u8; 10],
}

#[repr(C, packed)]
pub struct eeprom_eisa_slot_info {
    pub eisa_slot_id: u32,
    pub config_data_offset: u32,
    pub num_writes: u32,
    pub csum: u16,
    pub num_functions: u16,
    pub config_data_length: u16,

    /* bits 0..3 are the duplicate slot id */
    pub slot_info: u8,

    pub slot_features: u8,

    pub ver_min: u8,
    pub ver_maj: u8,

    pub function_info: u8,

    pub flags: u8,
    pub pad: [u8; 24],
}

pub const HPEE_SLOT_INFO_EMBEDDED: u8 = 0x10;
pub const HPEE_SLOT_INFO_VIRTUAL: u8 = 0x20;
pub const HPEE_SLOT_INFO_NO_READID: u8 = 0x40;
pub const HPEE_SLOT_INFO_DUPLICATE: u8 = 0x80;

pub const HPEE_SLOT_FEATURES_ENABLE: u8 = 0x01;
pub const HPEE_SLOT_FEATURES_IOCHK: u8 = 0x02;
pub const HPEE_SLOT_FEATURES_CFG_INCOMPLETE: u8 = 0x80;

pub const HPEE_FUNCTION_INFO_HAVE_TYPE: u8 = 0x01;
pub const HPEE_FUNCTION_INFO_HAVE_MEMORY: u8 = 0x02;
pub const HPEE_FUNCTION_INFO_HAVE_IRQ: u8 = 0x04;
pub const HPEE_FUNCTION_INFO_HAVE_DMA: u8 = 0x08;
pub const HPEE_FUNCTION_INFO_HAVE_PORT: u8 = 0x10;
pub const HPEE_FUNCTION_INFO_HAVE_PORT_INIT: u8 = 0x20;
/* I think there are two slighty different
 * versions of the function_info field
 * one int the fixed header and one optional
 * in the parsed slot data area */
pub const HPEE_FUNCTION_INFO_HAVE_FUNCTION: u8 = 0x01;
pub const HPEE_FUNCTION_INFO_F_DISABLED: u8 = 0x80;
pub const HPEE_FUNCTION_INFO_CFG_FREE_FORM: u8 = 0x40;

pub const HPEE_FLAG_BOARD_IS_ISA: u8 = 0x01; /* flag and minor version for isa board */

pub const HPEE_MEMORY_MAX_ENT: usize = 9;
/* memory descriptor: byte 0 */
pub const HPEE_MEMORY_WRITABLE: u8 = 0x01;
pub const HPEE_MEMORY_CACHABLE: u8 = 0x02;
pub const HPEE_MEMORY_TYPE_MASK: u8 = 0x18;
pub const HPEE_MEMORY_TYPE_SYS: u8 = 0x00;
pub const HPEE_MEMORY_TYPE_EXP: u8 = 0x08;
pub const HPEE_MEMORY_TYPE_VIR: u8 = 0x10;
pub const HPEE_MEMORY_TYPE_OTH: u8 = 0x18;
pub const HPEE_MEMORY_SHARED: u8 = 0x20;
pub const HPEE_MEMORY_MORE: u8 = 0x80;

/* memory descriptor: byte 1 */
pub const HPEE_MEMORY_WIDTH_MASK: u8 = 0x03;
pub const HPEE_MEMORY_WIDTH_BYTE: u8 = 0x00;
pub const HPEE_MEMORY_WIDTH_WORD: u8 = 0x01;
pub const HPEE_MEMORY_WIDTH_DWORD: u8 = 0x02;
pub const HPEE_MEMORY_DECODE_MASK: u8 = 0x0c;
pub const HPEE_MEMORY_DECODE_20BITS: u8 = 0x00;
pub const HPEE_MEMORY_DECODE_24BITS: u8 = 0x04;
pub const HPEE_MEMORY_DECODE_32BITS: u8 = 0x08;
/* byte 2 and 3 are a 16bit LE value
 * containing the memory size in kilobytes */
/* byte 4,5,6 are a 24bit LE value
 * containing the memory base address */

pub const HPEE_IRQ_MAX_ENT: usize = 7;
/* Interrupt entry: byte 0 */
pub const HPEE_IRQ_CHANNEL_MASK: u8 = 0xf;
pub const HPEE_IRQ_TRIG_LEVEL: u8 = 0x20;
pub const HPEE_IRQ_MORE: u8 = 0x80;
/* byte 1 seems to be unused */

pub const HPEE_DMA_MAX_ENT: usize = 4;
/* dma entry: byte 0 */
pub const HPEE_DMA_CHANNEL_MASK: u8 = 7;
pub const HPEE_DMA_SIZE_MASK: u8 = 0xc;
pub const HPEE_DMA_SIZE_BYTE: u8 = 0x0;
pub const HPEE_DMA_SIZE_WORD: u8 = 0x4;
pub const HPEE_DMA_SIZE_DWORD: u8 = 0x8;
pub const HPEE_DMA_SHARED: u8 = 0x40;
pub const HPEE_DMA_MORE: u8 = 0x80;
/* dma entry: byte 1 */
pub const HPEE_DMA_TIMING_MASK: u8 = 0x30;
pub const HPEE_DMA_TIMING_ISA: u8 = 0x0;
pub const HPEE_DMA_TIMING_TYPEA: u8 = 0x10;
pub const HPEE_DMA_TIMING_TYPEB: u8 = 0x20;
pub const HPEE_DMA_TIMING_TYPEC: u8 = 0x30;

pub const HPEE_PORT_MAX_ENT: usize = 20;
/* port entry byte 0 */
pub const HPEE_PORT_SIZE_MASK: u8 = 0x1f;
pub const HPEE_PORT_SHARED: u8 = 0x40;
pub const HPEE_PORT_MORE: u8 = 0x80;
/* byte 1 and 2 is a 16bit LE value
 * containing the start port number */

pub const HPEE_PORT_INIT_MAX_LEN: usize = 60; /* in bytes here */
/* port init entry byte 0 */
pub const HPEE_PORT_INIT_WIDTH_MASK: u8 = 0x3;
pub const HPEE_PORT_INIT_WIDTH_BYTE: u8 = 0x0;
pub const HPEE_PORT_INIT_WIDTH_WORD: u8 = 0x1;
pub const HPEE_PORT_INIT_WIDTH_DWORD: u8 = 0x2;
pub const HPEE_PORT_INIT_MASK: u8 = 0x4;
pub const HPEE_PORT_INIT_MORE: u8 = 0x80;

pub const HPEE_SELECTION_MAX_ENT: usize = 26;
pub const HPEE_TYPE_MAX_LEN: usize = 80;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
