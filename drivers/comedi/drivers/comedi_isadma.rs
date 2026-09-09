// SPDX-License-Identifier: GPL-2.0+
/*
 * COMEDI ISA DMA support functions
 * Copyright (c) 2014 H Hartley Sweeten <hsweeten@visionengravers.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comedi_device {
    pub hw_dev: *mut device,
    pub class_dev: *mut device,
    pub board_name: *const c_char,
}

#[repr(C)]
pub struct comedi_isadma_desc {
    pub chan: c_uint,
    pub mode: c_char,
    pub hw_addr: c_ulong,
    pub size: c_uint,
    pub maxsize: c_uint,
    pub virt_addr: *mut c_void,
}

#[repr(C)]
pub struct comedi_isadma {
    pub n_desc: c_int,
    pub dev: *mut device,
    pub chan: c_uint,
    pub chan2: c_uint,
    pub cur_dma: c_uint,
    pub desc: [comedi_isadma_desc; 0],
}

pub const COMEDI_ISADMA_READ: c_char = 0;
pub const DMA_MODE_READ: c_char = 0;
pub const DMA_MODE_WRITE: c_char = 1;
pub const GFP_KERNEL: c_uint = 0;

extern "C" {
    fn claim_dma_lock() -> c_ulong;
    fn release_dma_lock(flags: c_ulong);
    fn clear_dma_ff(chan: c_uint);
    fn set_dma_mode(chan: c_uint, mode: c_char);
    fn set_dma_addr(chan: c_uint, addr: c_ulong);
    fn set_dma_count(chan: c_uint, count: c_uint);
    fn enable_dma(chan: c_uint);
    fn disable_dma(chan: c_uint);
    fn get_dma_residue(chan: c_uint) -> c_uint;
    fn udelay(usecs: c_uint);
    fn request_dma(chan: c_uint, name: *const c_char) -> c_int;
    fn free_dma(chan: c_uint);
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn dma_alloc_coherent(dev: *mut device, size: usize, dma_handle: *mut c_ulong,
                          flags: c_uint) -> *mut c_void;
    fn dma_free_coherent(dev: *mut device, size: usize, cpu_addr: *mut c_void,
                         dma_handle: c_ulong);
    fn kzalloc_flex(size: usize) -> *mut comedi_isadma;
    fn kfree(ptr: *mut comedi_isadma);
    static mut isa_dma_bridge_buggy: bool;
}

pub unsafe fn comedi_isadma_program(desc: *mut comedi_isadma_desc) {
    let flags = claim_dma_lock();
    clear_dma_ff((*desc).chan);
    set_dma_mode((*desc).chan, (*desc).mode);
    set_dma_addr((*desc).chan, (*desc).hw_addr);
    set_dma_count((*desc).chan, (*desc).size);
    enable_dma((*desc).chan);
    release_dma_lock(flags);
}

pub unsafe fn comedi_isadma_disable(dma_chan: c_uint) -> c_uint {
    let flags = claim_dma_lock();
    disable_dma(dma_chan);
    let residue = get_dma_residue(dma_chan);
    release_dma_lock(flags);
    residue
}

pub unsafe fn comedi_isadma_disable_on_sample(dma_chan: c_uint, size: c_uint) -> c_uint {
    let mut stalled = 0;
    let mut residue = comedi_isadma_disable(dma_chan);
    while residue % size != 0 {
        // residue is a partial sample, enable DMA to allow more data
        let flags = claim_dma_lock();
        enable_dma(dma_chan);
        release_dma_lock(flags);
        udelay(2);
        let new_residue = comedi_isadma_disable(dma_chan);
        // is DMA stalled?
        if new_residue == residue {
            stalled += 1;
            if stalled > 10 {
                break;
            }
        } else {
            residue = new_residue;
            stalled = 0;
        }
    }
    residue
}

pub unsafe fn comedi_isadma_poll(dma: *mut comedi_isadma) -> c_uint {
    let desc = (*dma).desc.as_mut_ptr().add((*dma).cur_dma as usize);
    let flags = claim_dma_lock();
    clear_dma_ff((*desc).chan);
    if !isa_dma_bridge_buggy {
        disable_dma((*desc).chan);
    }
    let result = get_dma_residue((*desc).chan);
    // Read the counter again and choose higher value to avoid lower-byte rollover.
    let result1 = get_dma_residue((*desc).chan);
    if !isa_dma_bridge_buggy {
        enable_dma((*desc).chan);
    }
    release_dma_lock(flags);
    let result = if result < result1 { result1 } else { result };
    if result >= (*desc).size || result == 0 { return 0; }
    (*desc).size - result
}

pub unsafe fn comedi_isadma_set_mode(desc: *mut comedi_isadma_desc, dma_dir: c_char) {
    (*desc).mode = if dma_dir == COMEDI_ISADMA_READ { DMA_MODE_READ } else { DMA_MODE_WRITE };
}

pub unsafe fn comedi_isadma_alloc(dev: *mut comedi_device, n_desc: c_int,
                                  dma_chan1: c_uint, dma_chan2: c_uint,
                                  maxsize: c_uint, dma_dir: c_char) -> *mut comedi_isadma {
    if n_desc < 1 || n_desc > 2 { return core::ptr::null_mut(); }
    let dma = kzalloc_flex(core::mem::size_of::<comedi_isadma>() +
                           n_desc as usize * core::mem::size_of::<comedi_isadma_desc>());
    if dma.is_null() { return core::ptr::null_mut(); }
    (*dma).n_desc = n_desc;
    (*dma).dev = if !(*dev).hw_dev.is_null() { (*dev).hw_dev } else {
        if (*dev).class_dev.is_null() { comedi_isadma_free(dma); return core::ptr::null_mut(); }
        if dma_coerce_mask_and_coherent((*dev).class_dev, 1u64 << 24) != 0 {
            comedi_isadma_free(dma); return core::ptr::null_mut();
        }
        (*dev).class_dev
    };
    let chan2 = if dma_chan2 == 0 || dma_chan2 == dma_chan1 { dma_chan1 } else { dma_chan2 };
    if request_dma(dma_chan1, (*dev).board_name) != 0 { comedi_isadma_free(dma); return core::ptr::null_mut(); }
    (*dma).chan = dma_chan1;
    if chan2 != dma_chan1 && request_dma(chan2, (*dev).board_name) != 0 {
        comedi_isadma_free(dma); return core::ptr::null_mut();
    }
    (*dma).chan2 = chan2;
    for i in 0..n_desc as usize {
        let desc = &mut *(*dma).desc.as_mut_ptr().add(i);
        desc.chan = if i == 0 { dma_chan1 } else { chan2 };
        desc.maxsize = maxsize;
        desc.virt_addr = dma_alloc_coherent((*dma).dev, desc.maxsize as usize,
                                            &mut desc.hw_addr, GFP_KERNEL);
        if desc.virt_addr.is_null() { comedi_isadma_free(dma); return core::ptr::null_mut(); }
        comedi_isadma_set_mode(desc, dma_dir);
    }
    dma
}

pub unsafe fn comedi_isadma_free(dma: *mut comedi_isadma) {
    if dma.is_null() { return; }
    for i in 0..(*dma).n_desc as usize {
        let desc = &mut *(*dma).desc.as_mut_ptr().add(i);
        if !desc.virt_addr.is_null() {
            dma_free_coherent((*dma).dev, desc.maxsize as usize, desc.virt_addr, desc.hw_addr);
        }
    }
    if (*dma).chan2 != 0 && (*dma).chan2 != (*dma).chan { free_dma((*dma).chan2); }
    if (*dma).chan != 0 { free_dma((*dma).chan); }
    kfree(dma);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
