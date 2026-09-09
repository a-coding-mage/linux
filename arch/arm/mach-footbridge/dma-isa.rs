// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 1999-2000 Russell King
 *
 *  ISA DMA primitives
 *  Taken from various sources, including:
 *   linux/include/asm/dma.h: Defines for using and allocating dma channels.
 *     Written by Hennus Bergman, 1992.
 *     High DMA channel support & info by Hannu Savolainen and John Boyd,
 *     Nov. 1992.
 *   arch/arm/kernel/dma-ebsa285.c
 *   Copyright (C) 1998 Phil Blundell
 */

const ISA_DMA_MASK: usize = 0;
const ISA_DMA_MODE: usize = 1;
const ISA_DMA_CLRFF: usize = 2;
const ISA_DMA_PGHI: usize = 3;
const ISA_DMA_PGLO: usize = 4;
const ISA_DMA_ADDR: usize = 5;
const ISA_DMA_COUNT: usize = 6;

static mut isa_dma_port: [[u32; 7]; 8] = [
    [0x0a, 0x0b, 0x0c, 0x487, 0x087, 0x00, 0x01],
    [0x0a, 0x0b, 0x0c, 0x483, 0x083, 0x02, 0x03],
    [0x0a, 0x0b, 0x0c, 0x481, 0x081, 0x04, 0x05],
    [0x0a, 0x0b, 0x0c, 0x482, 0x082, 0x06, 0x07],
    [0xd4, 0xd6, 0xd8, 0x000, 0x000, 0xc0, 0xc2],
    [0xd4, 0xd6, 0xd8, 0x48b, 0x08b, 0xc4, 0xc6],
    [0xd4, 0xd6, 0xd8, 0x489, 0x089, 0xc8, 0xca],
    [0xd4, 0xd6, 0xd8, 0x48a, 0x08a, 0xcc, 0xce],
];

unsafe fn isa_get_dma_residue(chan: u32, _dma: *mut dma_t) -> i32 {
    let io_port = isa_dma_port[chan as usize][ISA_DMA_COUNT];
    let mut count = 1 + inb(io_port);
    count |= inb(io_port) << 8;
    if chan < 4 { count as i32 } else { (count << 1) as i32 }
}

static mut isa_dma_dev: device = device {
    init_name: "fallback device",
    coherent_dma_mask: !(0 as dma_addr_t),
    dma_mask: core::ptr::null_mut(),
};

unsafe fn isa_enable_dma(chan: u32, dma: *mut dma_t) {
    if (*dma).invalid {
        let mut address: usize;
        let length: usize;
        let mode = (chan & 3) | (*dma).dma_mode;
        let direction = match (*dma).dma_mode & DMA_MODE_MASK {
            DMA_MODE_READ => DMA_FROM_DEVICE,
            DMA_MODE_WRITE => DMA_TO_DEVICE,
            DMA_MODE_CASCADE => DMA_BIDIRECTIONAL,
            _ => DMA_NONE,
        };
        if (*dma).sg.is_null() {
            (*dma).sg = &mut (*dma).buf;
            (*dma).sgcount = 1;
            (*dma).buf.length = (*dma).count;
            (*dma).buf.dma_address = dma_map_single(
                &mut isa_dma_dev, (*dma).addr, (*dma).count, direction);
        }
        address = (*dma).buf.dma_address as usize;
        length = (*dma).buf.length as usize - 1;
        outb((address >> 16) as u8, isa_dma_port[chan as usize][ISA_DMA_PGLO]);
        outb((address >> 24) as u8, isa_dma_port[chan as usize][ISA_DMA_PGHI]);
        if chan >= 4 { address >>= 1; }
        let length = if chan >= 4 { length >> 1 } else { length };
        outb(0, isa_dma_port[chan as usize][ISA_DMA_CLRFF]);
        outb(address as u8, isa_dma_port[chan as usize][ISA_DMA_ADDR]);
        outb((address >> 8) as u8, isa_dma_port[chan as usize][ISA_DMA_ADDR]);
        outb(length as u8, isa_dma_port[chan as usize][ISA_DMA_COUNT]);
        outb((length >> 8) as u8, isa_dma_port[chan as usize][ISA_DMA_COUNT]);
        outb(mode as u8, isa_dma_port[chan as usize][ISA_DMA_MODE]);
        (*dma).invalid = false;
    }
    outb((chan & 3) as u8, isa_dma_port[chan as usize][ISA_DMA_MASK]);
}

unsafe fn isa_disable_dma(chan: u32, _dma: *mut dma_t) {
    outb((chan | 4) as u8, isa_dma_port[chan as usize][ISA_DMA_MASK]);
}

static mut isa_dma_ops: dma_ops = dma_ops {
    type_: "ISA",
    enable: isa_enable_dma,
    disable: isa_disable_dma,
    residue: isa_get_dma_residue,
};

static mut dma_resources: [resource; 4] = [
    resource { name: "dma1", start: 0x0000, end: 0x000f },
    resource { name: "dma low page", start: 0x0080, end: 0x008f },
    resource { name: "dma2", start: 0x00c0, end: 0x00df },
    resource { name: "dma high page", start: 0x0480, end: 0x048f },
];

static mut isa_dma: [dma_t; 8] = [dma_t::ZERO; 8];

/* ISA DMA always starts at channel 0 */
unsafe fn isa_dma_init() -> i32 {
    outb(0xff, 0x0d); outb(0xff, 0xda);
    outb(0x55, 0x00); outb(0xaa, 0x00);
    if inb(0) == 0x55 && inb(0) == 0xaa {
        for chan in 0..8 {
            isa_dma[chan].d_ops = &mut isa_dma_ops;
            isa_disable_dma(chan as u32, core::ptr::null_mut());
        }
        for value in [0x40, 0x41, 0x42, 0x43] { outb(value, 0x0b); }
        outb(0xc0, 0xd6); for value in [0x41, 0x42, 0x43] { outb(value, 0xd6); }
        outb(0, 0xd4); outb(0x10, 0x08); outb(0x10, 0xd0);
        /* Documentation says these should instead use 0x3f. */
        for value in [0x30, 0x31, 0x32, 0x33] { outb(value, 0x40b); }
        for value in [0x31, 0x32, 0x33] { outb(value, 0x4d6); }
        for resource in &mut dma_resources { request_resource(&mut ioport_resource, resource); }
        for chan in 0..8 {
            let ret = isa_dma_add(chan as u32, &mut isa_dma[chan]);
            if ret != 0 { pr_err!("ISADMA{}: unable to register: {}\n", chan, ret); }
        }
        request_dma(DMA_ISA_CASCADE, "cascade");
    }
    dma_direct_set_offset(&mut isa_dma_dev, PHYS_OFFSET, BUS_OFFSET, SZ_256M);
    0
}

core_initcall!(isa_dma_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
