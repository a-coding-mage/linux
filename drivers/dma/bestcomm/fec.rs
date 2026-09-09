// SPDX-License-Identifier: GPL-2.0-only
/*
 * Bestcomm FEC tasks driver
 *
 * Copyright (C) 2006-2007 Sylvain Munaut <tnt@246tNt.com>
 * Copyright (C) 2003-2004 MontaVista, Software, Inc.
 *                         ( by Dale Farnsworth <dfarnsworth@mvista.com> )
 */

// Kernel and BestComm declarations are supplied by other translation units.

extern "C" {
    static mut bcom_fec_rx_task: *mut u32;
    static mut bcom_fec_tx_task: *mut u32;
}

#[repr(C)]
pub struct bcom_fec_rx_var {
    pub enable: u32,
    pub fifo: u32,
    pub bd_base: u32,
    pub bd_last: u32,
    pub bd_start: u32,
    pub buffer_size: u32,
}

#[repr(C)]
pub struct bcom_fec_rx_inc {
    pub pad0: u16,
    pub incr_bytes: i16,
    pub pad1: u16,
    pub incr_dst: i16,
    pub pad2: u16,
    pub incr_dst_ma: i16,
}

#[repr(C)]
pub struct bcom_fec_tx_var {
    pub DRD: u32,
    pub fifo: u32,
    pub enable: u32,
    pub bd_base: u32,
    pub bd_last: u32,
    pub bd_start: u32,
    pub buffer_size: u32,
}

#[repr(C)]
pub struct bcom_fec_tx_inc {
    pub pad0: u16,
    pub incr_bytes: i16,
    pub pad1: u16,
    pub incr_src: i16,
    pub pad2: u16,
    pub incr_src_ma: i16,
}

#[repr(C)]
pub struct bcom_fec_priv {
    pub fifo: phys_addr_t,
    pub maxbufsize: i32,
}

#[no_mangle]
pub unsafe extern "C" fn bcom_fec_rx_init(queue_len: i32, fifo: phys_addr_t, maxbufsize: i32) -> *mut bcom_task {
    let tsk = bcom_task_alloc(queue_len, core::mem::size_of::<bcom_fec_bd>(), core::mem::size_of::<bcom_fec_priv>());
    if tsk.is_null() { return core::ptr::null_mut(); }
    (*tsk).flags = BCOM_FLAGS_NONE;
    let priv_ = (*tsk).priv_ as *mut bcom_fec_priv;
    (*priv_).fifo = fifo;
    (*priv_).maxbufsize = maxbufsize;
    if bcom_fec_rx_reset(tsk) != 0 { bcom_task_free(tsk); return core::ptr::null_mut(); }
    tsk
}

#[no_mangle]
pub unsafe extern "C" fn bcom_fec_rx_reset(tsk: *mut bcom_task) -> i32 {
    let priv_ = (*tsk).priv_ as *mut bcom_fec_priv;
    bcom_disable_task((*tsk).tasknum);
    let var = bcom_task_var((*tsk).tasknum) as *mut bcom_fec_rx_var;
    let inc = bcom_task_inc((*tsk).tasknum) as *mut bcom_fec_rx_inc;
    if bcom_load_image((*tsk).tasknum, bcom_fec_rx_task) != 0 { return -1; }
    (*var).enable = bcom_eng.regs_base + core::mem::offset_of!(mpc52xx_sdma, tcr) as u32 + ((*tsk).tasknum as usize * core::mem::size_of::<u16>()) as u32;
    (*var).fifo = (*priv_).fifo as u32;
    (*var).bd_base = (*tsk).bd_pa;
    (*var).bd_last = (*tsk).bd_pa + (((*tsk).num_bd - 1) * (*tsk).bd_size);
    (*var).bd_start = (*tsk).bd_pa;
    (*var).buffer_size = (*priv_).maxbufsize as u32;
    (*inc).incr_bytes = -(core::mem::size_of::<u32>() as i16);
    (*inc).incr_dst = core::mem::size_of::<u32>() as i16;
    (*inc).incr_dst_ma = core::mem::size_of::<u8>() as i16;
    (*tsk).index = 0; (*tsk).outdex = 0;
    memset_io((*tsk).bd, 0, (*tsk).num_bd * (*tsk).bd_size);
    bcom_set_task_pragma((*tsk).tasknum, BCOM_FEC_RX_BD_PRAGMA);
    bcom_set_task_auto_start((*tsk).tasknum, (*tsk).tasknum);
    out_8(&mut (*bcom_eng).regs.ipr[BCOM_INITIATOR_FEC_RX], BCOM_IPR_FEC_RX);
    out_be32(&mut (*bcom_eng).regs.IntPend, 1u32 << (*tsk).tasknum);
    0
}

#[no_mangle]
pub unsafe extern "C" fn bcom_fec_rx_release(tsk: *mut bcom_task) { bcom_task_free(tsk); }

unsafe fn self_modified_drd(tasknum: i32) -> *mut u32 {
    let num_descs = bcom_task_num_descs(tasknum);
    let mut desc = bcom_task_desc(tasknum).add(num_descs as usize - 1);
    let mut drd_count = 0;
    for _ in 0..num_descs { if bcom_desc_is_drd(*desc) { drd_count += 1; if drd_count == 3 { break; } } desc = desc.sub(1); }
    desc
}

#[no_mangle]
pub unsafe extern "C" fn bcom_fec_tx_init(queue_len: i32, fifo: phys_addr_t) -> *mut bcom_task {
    let tsk = bcom_task_alloc(queue_len, core::mem::size_of::<bcom_fec_bd>(), core::mem::size_of::<bcom_fec_priv>());
    if tsk.is_null() { return core::ptr::null_mut(); }
    (*tsk).flags = BCOM_FLAGS_ENABLE_TASK;
    let priv_ = (*tsk).priv_ as *mut bcom_fec_priv; (*priv_).fifo = fifo;
    if bcom_fec_tx_reset(tsk) != 0 { bcom_task_free(tsk); return core::ptr::null_mut(); } tsk
}

#[no_mangle]
pub unsafe extern "C" fn bcom_fec_tx_reset(tsk: *mut bcom_task) -> i32 {
    let priv_ = (*tsk).priv_ as *mut bcom_fec_priv;
    bcom_disable_task((*tsk).tasknum);
    let var = bcom_task_var((*tsk).tasknum) as *mut bcom_fec_tx_var;
    let inc = bcom_task_inc((*tsk).tasknum) as *mut bcom_fec_tx_inc;
    if bcom_load_image((*tsk).tasknum, bcom_fec_tx_task) != 0 { return -1; }
    (*var).enable = bcom_eng.regs_base + core::mem::offset_of!(mpc52xx_sdma, tcr) as u32 + ((*tsk).tasknum as usize * core::mem::size_of::<u16>()) as u32;
    (*var).fifo = (*priv_).fifo as u32; (*var).DRD = bcom_sram_va2pa(self_modified_drd((*tsk).tasknum));
    (*var).bd_base = (*tsk).bd_pa; (*var).bd_last = (*tsk).bd_pa + (((*tsk).num_bd - 1) * (*tsk).bd_size); (*var).bd_start = (*tsk).bd_pa;
    (*inc).incr_bytes = -(core::mem::size_of::<u32>() as i16); (*inc).incr_src = core::mem::size_of::<u32>() as i16; (*inc).incr_src_ma = core::mem::size_of::<u8>() as i16;
    (*tsk).index = 0; (*tsk).outdex = 0; memset_io((*tsk).bd, 0, (*tsk).num_bd * (*tsk).bd_size);
    bcom_set_task_pragma((*tsk).tasknum, BCOM_FEC_TX_BD_PRAGMA); bcom_set_task_auto_start((*tsk).tasknum, (*tsk).tasknum);
    out_8(&mut (*bcom_eng).regs.ipr[BCOM_INITIATOR_FEC_TX], BCOM_IPR_FEC_TX); out_be32(&mut (*bcom_eng).regs.IntPend, 1u32 << (*tsk).tasknum); 0
}

#[no_mangle]
pub unsafe extern "C" fn bcom_fec_tx_release(tsk: *mut bcom_task) { bcom_task_free(tsk); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
