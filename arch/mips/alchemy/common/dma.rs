/*
 * A DMA channel allocator for Au1x00. API is modeled loosely off of
 * linux/kernel/dma.c.
 *
 * Copyright 2000, 2008 MontaVista Software Inc.
 * Author: MontaVista Software, Inc. <source@mvista.com>
 * Copyright (C) 2005 Ralf Baechle (ralf@linux-mips.org)
 *
 * This file is a direct Rust translation of the original implementation.
 */

// Linux and Au1x00 header dependencies are supplied by other translation units.

const DMA_CHANNEL_LEN: usize = 0x00000100;

#[repr(C)]
struct DmaDev {
    fifo_addr: u32,
    dma_mode: u32,
}

#[repr(C)]
pub struct DmaChan {
    pub dev_id: i32,
    pub irq: i32,
    pub irq_dev: *mut core::ffi::c_void,
    pub io: *mut core::ffi::c_void,
    pub dev_str: *const core::ffi::c_char,
    pub fifo_addr: u32,
    pub mode: u32,
}

extern "C" {
    static mut au1000_dma_spin_lock: core::ffi::c_void;

    static AU1000_UART0_PHYS_ADDR: u32;
    static AU1000_AC97_PHYS_ADDR: u32;
    static AU1000_UART3_PHYS_ADDR: u32;
    static AU1000_USB_UDC_PHYS_ADDR: u32;
    static AU1000_I2S_PHYS_ADDR: u32;
    static AU1100_SD0_PHYS_ADDR: u32;
    static AU1100_SD1_PHYS_ADDR: u32;
    static AU1000_DMA_PHYS_ADDR: usize;
    static DMA_DW8: u32;
    static DMA_DW16: u32;
    static DMA_DW32: u32;
    static DMA_DR: u32;
    static DMA_NC: u32;
    static DMA_DS: u32;
    static DMA_NUM_DEV: i32;
    static DMA_NUM_DEV_BANK2: i32;
    static NUM_AU1000_DMA_CHANNELS: i32;
    static ALCHEMY_CPU_AU1000: i32;
    static ALCHEMY_CPU_AU1500: i32;
    static ALCHEMY_CPU_AU1100: i32;
    static AU1000_DMA_INT_BASE: i32;
    static AU1500_DMA_INT_BASE: i32;
    static AU1100_DMA_INT_BASE: i32;

    fn alchemy_get_cputype() -> i32;
    fn get_dma_chan(dmanr: i32) -> *mut DmaChan;
    fn request_irq(irq: i32, handler: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32>, flags: usize, name: *const core::ffi::c_char, dev: *mut core::ffi::c_void) -> i32;
    fn free_irq(irq: i32, dev: *mut core::ffi::c_void);
    fn init_dma(dmanr: i32);
    fn disable_dma(dmanr: i32);
    fn printk(fmt: *const core::ffi::c_char, ...) -> i32;
}

#[no_mangle]
pub static mut au1000_dma_table: [DmaChan; 8] = [
    DmaChan { dev_id: -1, irq: 0, irq_dev: core::ptr::null_mut(), io: core::ptr::null_mut(), dev_str: core::ptr::null(), fifo_addr: 0, mode: 0 },
    DmaChan { dev_id: -1, irq: 0, irq_dev: core::ptr::null_mut(), io: core::ptr::null_mut(), dev_str: core::ptr::null(), fifo_addr: 0, mode: 0 },
    DmaChan { dev_id: -1, irq: 0, irq_dev: core::ptr::null_mut(), io: core::ptr::null_mut(), dev_str: core::ptr::null(), fifo_addr: 0, mode: 0 },
    DmaChan { dev_id: -1, irq: 0, irq_dev: core::ptr::null_mut(), io: core::ptr::null_mut(), dev_str: core::ptr::null(), fifo_addr: 0, mode: 0 },
    DmaChan { dev_id: -1, irq: 0, irq_dev: core::ptr::null_mut(), io: core::ptr::null_mut(), dev_str: core::ptr::null(), fifo_addr: 0, mode: 0 },
    DmaChan { dev_id: -1, irq: 0, irq_dev: core::ptr::null_mut(), io: core::ptr::null_mut(), dev_str: core::ptr::null(), fifo_addr: 0, mode: 0 },
    DmaChan { dev_id: -1, irq: 0, irq_dev: core::ptr::null_mut(), io: core::ptr::null_mut(), dev_str: core::ptr::null(), fifo_addr: 0, mode: 0 },
    DmaChan { dev_id: -1, irq: 0, irq_dev: core::ptr::null_mut(), io: core::ptr::null_mut(), dev_str: core::ptr::null(), fifo_addr: 0, mode: 0 },
];

static mut DMA_DEV_TABLE: [DmaDev; 16] = [
    DmaDev { fifo_addr: 0, dma_mode: 0 }, DmaDev { fifo_addr: 0, dma_mode: 0 },
    DmaDev { fifo_addr: 0, dma_mode: 0 }, DmaDev { fifo_addr: 0, dma_mode: 0 },
    DmaDev { fifo_addr: 0, dma_mode: 0 }, DmaDev { fifo_addr: 0, dma_mode: 0 },
    DmaDev { fifo_addr: 0, dma_mode: 0 }, DmaDev { fifo_addr: 0, dma_mode: 0 },
    DmaDev { fifo_addr: 0, dma_mode: 0 }, DmaDev { fifo_addr: 0, dma_mode: 0 },
    DmaDev { fifo_addr: 0, dma_mode: 0 }, DmaDev { fifo_addr: 0, dma_mode: 0 },
    DmaDev { fifo_addr: 0, dma_mode: 0 }, DmaDev { fifo_addr: 0, dma_mode: 0 },
    DmaDev { fifo_addr: 0, dma_mode: 0 }, DmaDev { fifo_addr: 0, dma_mode: 0 },
];

static mut DMA_DEV_TABLE_BANK2: [DmaDev; 4] = [
    DmaDev { fifo_addr: 0, dma_mode: 0 }, DmaDev { fifo_addr: 0, dma_mode: 0 },
    DmaDev { fifo_addr: 0, dma_mode: 0 }, DmaDev { fifo_addr: 0, dma_mode: 0 },
];

pub unsafe extern "C" fn au1000_dma_read_proc(buf: *mut i8, start: *mut *mut i8,
    fpos: isize, length: i32, eof: *mut i32, _data: *mut core::ffi::c_void) -> i32 {
    let mut i = 0;
    let mut len = 0;
    while i < 8 {
        let chan = get_dma_chan(i);
        if !chan.is_null() {
            len += sprintf(buf.add(len as usize), b"%2d: %s\n\0".as_ptr() as *const i8,
                i, (*chan).dev_str);
        }
        i += 1;
    }
    if fpos >= len as isize {
        *start = buf;
        *eof = 1;
        return 0;
    }
    *start = buf.add(fpos as usize);
    len -= fpos as i32;
    if len > length { return length; }
    *eof = 1;
    len
}

extern "C" { fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32; }

pub unsafe extern "C" fn request_au1000_dma(dev_id: i32, dev_str: *const i8,
    irqhandler: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32>,
    irqflags: usize, irq_dev_id: *mut core::ffi::c_void) -> i32 {
    let ndev = if alchemy_get_cputype() == ALCHEMY_CPU_AU1100 { 20 } else { 16 };
    if dev_id < 0 || dev_id >= ndev { return -22; }
    let mut i = 0;
    while i < 8 && (*au1000_dma_table.as_mut_ptr().add(i as usize)).dev_id >= 0 { i += 1; }
    if i == 8 { return -19; }
    let chan = au1000_dma_table.as_mut_ptr().add(i as usize);
    if let Some(handler) = irqhandler {
        (*chan).irq_dev = irq_dev_id;
        let ret = request_irq((*chan).irq, Some(handler), irqflags, dev_str, (*chan).irq_dev);
        if ret != 0 { (*chan).irq_dev = core::ptr::null_mut(); return ret; }
    } else { (*chan).irq_dev = core::ptr::null_mut(); }
    let _dev = if dev_id >= 16 { &DMA_DEV_TABLE_BANK2[(dev_id - 16) as usize] } else { &DMA_DEV_TABLE[dev_id as usize] };
    (*chan).io = (0usize + (i as usize) * DMA_CHANNEL_LEN) as *mut core::ffi::c_void;
    (*chan).dev_id = if dev_id >= 16 { dev_id - 16 } else { dev_id };
    (*chan).dev_str = dev_str;
    (*chan).fifo_addr = (*_dev).fifo_addr;
    (*chan).mode = (*_dev).dma_mode;
    init_dma(i);
    i
}

pub unsafe extern "C" fn free_au1000_dma(dmanr: u32) {
    let chan = get_dma_chan(dmanr as i32);
    if chan.is_null() { return; }
    disable_dma(dmanr as i32);
    if !(*chan).irq_dev.is_null() { free_irq((*chan).irq, (*chan).irq_dev); }
    (*chan).irq_dev = core::ptr::null_mut();
    (*chan).dev_id = -1;
}

pub unsafe extern "C" fn au1000_dma_init() -> i32 {
    let base = match alchemy_get_cputype() {
        x if x == ALCHEMY_CPU_AU1000 => AU1000_DMA_INT_BASE,
        x if x == ALCHEMY_CPU_AU1500 => AU1500_DMA_INT_BASE,
        x if x == ALCHEMY_CPU_AU1100 => AU1100_DMA_INT_BASE,
        _ => return 0,
    };
    for i in 0..8 { (*au1000_dma_table.as_mut_ptr().add(i)).irq = base + i as i32; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
