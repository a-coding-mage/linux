// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/dma.c
 *
 *  Copyright (C) 1995-2000 Russell King
 *
 *  Front-end to the DMA handling.  This handles the allocation/freeing
 *  of DMA channels, and provides a unified interface to the machines
 *  DMA facilities.
 */

// External kernel types, constants, functions, and macros supplied by other files.
pub const MAX_DMA_CHANNELS: usize = 8;

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_ops {
    pub request: Option<unsafe extern "C" fn(u32, *mut dma_t) -> i32>,
    pub free: Option<unsafe extern "C" fn(u32, *mut dma_t)>,
    pub enable: unsafe extern "C" fn(u32, *mut dma_t),
    pub disable: unsafe extern "C" fn(u32, *mut dma_t),
    pub setspeed: Option<unsafe extern "C" fn(u32, *mut dma_t, i32) -> i32>,
    pub residue: Option<unsafe extern "C" fn(u32, *mut dma_t) -> i32>,
}

#[repr(C)]
pub struct dma_t {
    pub d_ops: *mut dma_ops,
    pub buf: scatterlist,
    pub lock: i32,
    pub device_id: *const core::ffi::c_char,
    pub active: i32,
    pub invalid: i32,
    pub sg: *mut scatterlist,
    pub sgcount: i32,
    pub addr: *mut core::ffi::c_void,
    pub count: u64,
    pub dma_mode: u32,
    pub speed: i32,
}

extern "C" {
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn BUG() -> !;
    fn sg_init_table(sg: *mut scatterlist, nents: usize);
    fn proc_create_single(name: *const core::ffi::c_char, mode: u32, parent: *mut core::ffi::c_void,
                          show: unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32) -> *mut core::ffi::c_void;
    fn seq_printf(m: *mut seq_file, fmt: *const core::ffi::c_char, ...);
}

#[no_mangle]
pub static mut dma_spin_lock: u8 = 0;

static mut dma_chan: [*mut dma_t; MAX_DMA_CHANNELS] = [core::ptr::null_mut(); MAX_DMA_CHANNELS];

#[inline]
unsafe fn dma_channel(chan: u32) -> *mut dma_t {
    if chan >= MAX_DMA_CHANNELS as u32 { return core::ptr::null_mut(); }
    dma_chan[chan as usize]
}

pub unsafe extern "C" fn isa_dma_add(chan: u32, dma: *mut dma_t) -> i32 {
    if (*dma).d_ops.is_null() { return -22; }
    sg_init_table(&mut (*dma).buf, 1);
    if !dma_chan[chan as usize].is_null() { return -16; }
    dma_chan[chan as usize] = dma;
    0
}

pub unsafe extern "C" fn request_dma(chan: u32, device_id: *const core::ffi::c_char) -> i32 {
    let dma = dma_channel(chan);
    if dma.is_null() { pr_err(b"dma: trying to allocate DMA%d\0".as_ptr() as _, chan); return -22; }
    if core::ptr::replace(&mut (*dma).lock, 1) != 0 { return -16; }
    (*dma).device_id = device_id; (*dma).active = 0; (*dma).invalid = 1;
    let mut ret = 0;
    if let Some(request) = (*(*dma).d_ops).request { ret = request(chan, dma); }
    if ret != 0 { (*dma).lock = 0; }
    ret
}

pub unsafe extern "C" fn free_dma(chan: u32) {
    let dma = dma_channel(chan);
    if dma.is_null() { pr_err(b"dma: trying to free DMA%d\0".as_ptr() as _, chan); return; }
    if (*dma).active != 0 { pr_err(b"dma%d: freeing active DMA\n\0".as_ptr() as _, chan); ((*(*dma).d_ops).disable)(chan, dma); (*dma).active = 0; }
    if core::ptr::replace(&mut (*dma).lock, 0) != 0 { if let Some(free) = (*(*dma).d_ops).free { free(chan, dma); } return; }
    pr_err(b"dma%d: trying to free free DMA\n\0".as_ptr() as _, chan);
}

pub unsafe extern "C" fn set_dma_sg(chan: u32, sg: *mut scatterlist, nr_sg: i32) { let dma=dma_channel(chan); if (*dma).active!=0 { pr_err(b"dma%d: altering DMA SG while DMA active\n\0".as_ptr() as _,chan); } (*dma).sg=sg; (*dma).sgcount=nr_sg; (*dma).invalid=1; }
pub unsafe extern "C" fn __set_dma_addr(chan: u32, addr: *mut core::ffi::c_void) { let dma=dma_channel(chan); if (*dma).active!=0 { pr_err(b"dma%d: altering DMA address while DMA active\n\0".as_ptr() as _,chan); } (*dma).sg=core::ptr::null_mut(); (*dma).addr=addr; (*dma).invalid=1; }
pub unsafe extern "C" fn set_dma_count(chan: u32, count: u64) { let dma=dma_channel(chan); if (*dma).active!=0 { pr_err(b"dma%d: altering DMA count while DMA active\n\0".as_ptr() as _,chan); } (*dma).sg=core::ptr::null_mut(); (*dma).count=count; (*dma).invalid=1; }
pub unsafe extern "C" fn set_dma_mode(chan: u32, mode: u32) { let dma=dma_channel(chan); if (*dma).active!=0 { pr_err(b"dma%d: altering DMA mode while DMA active\n\0".as_ptr() as _,chan); } (*dma).dma_mode=mode; (*dma).invalid=1; }

pub unsafe extern "C" fn enable_dma(chan:u32) { let dma=dma_channel(chan); if (*dma).lock==0 { pr_err(b"dma%d: trying to enable free DMA\n\0".as_ptr() as _,chan); BUG(); } if (*dma).active==0 { (*dma).active=1; ((*(*dma).d_ops).enable)(chan,dma); } }
pub unsafe extern "C" fn disable_dma(chan:u32) { let dma=dma_channel(chan); if (*dma).lock==0 { pr_err(b"dma%d: trying to disable free DMA\n\0".as_ptr() as _,chan); BUG(); } if (*dma).active==1 { (*dma).active=0; ((*(*dma).d_ops).disable)(chan,dma); } }
pub unsafe extern "C" fn dma_channel_active(chan:u32)->i32 { (*dma_channel(chan)).active }
pub unsafe extern "C" fn set_dma_page(chan:u32, _pagenr:i8) { pr_err(b"dma%d: trying to set_dma_page\n\0".as_ptr() as _,chan); }
pub unsafe extern "C" fn set_dma_speed(chan:u32, cycle_ns:i32) { let dma=dma_channel(chan); let mut ret=0; if let Some(f)=(*(*dma).d_ops).setspeed { ret=f(chan,dma,cycle_ns); } (*dma).speed=ret; }
pub unsafe extern "C" fn get_dma_residue(chan:u32)->i32 { let dma=dma_channel(chan); if let Some(f)=(*(*dma).d_ops).residue { return f(chan,dma); } 0 }

// CONFIG_PROC_FS controls this section in the kernel build.
#[cfg(feature = "CONFIG_PROC_FS")]
pub unsafe extern "C" fn proc_dma_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    for i in 0..MAX_DMA_CHANNELS {
        let dma = dma_channel(i as u32);
        if !dma.is_null() && (*dma).lock != 0 {
            seq_printf(m, b"%2d: %s\n\0".as_ptr() as _, i as i32, (*dma).device_id);
        }
    }
    0
}

#[cfg(feature = "CONFIG_PROC_FS")]
pub unsafe extern "C" fn proc_dma_init() -> i32 {
    proc_create_single(b"dma\0".as_ptr() as _, 0, core::ptr::null_mut(), proc_dma_show);
    0
}

// EXPORT_SYMBOL declarations are provided by the kernel build system.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
