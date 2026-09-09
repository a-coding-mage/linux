// SPDX-License-Identifier: GPL-2.0-only
/*
 * Bestcomm ATA task driver
 *
 * Patterned after bestcomm/fec.c by Dale Farnsworth <dfarnsworth@mvista.com>
 *                                   2003-2004 (c) MontaVista, Software, Inc.
 *
 * Copyright (C) 2006-2007 Sylvain Munaut <tnt@246tNt.com>
 * Copyright (C) 2006      Freescale - John Rigby
 */

use core::mem::{size_of, offset_of};

/* Dependencies are supplied by the BestComm and platform support code. */

/* ======================================================================== */
/* Task image/var/inc                                                       */
/* ======================================================================== */

/* ata task image */
unsafe extern "C" {
    pub static mut bcom_ata_task: [u32; 0];
}

/* ata task vars that need to be set before enabling the task */
#[repr(C)]
pub struct bcom_ata_var {
    pub enable: u32,       /* (u16*) address of task's control register */
    pub bd_base: u32,      /* (struct bcom_bd*) beginning of ring buffer */
    pub bd_last: u32,      /* (struct bcom_bd*) end of ring buffer */
    pub bd_start: u32,     /* (struct bcom_bd*) current bd */
    pub buffer_size: u32,  /* size of receive buffer */
}

/* ata task incs that need to be set before enabling the task */
#[repr(C)]
pub struct bcom_ata_inc {
    pub pad0: u16,
    pub incr_bytes: i16,
    pub pad1: u16,
    pub incr_dst: i16,
    pub pad2: u16,
    pub incr_src: i16,
}

/* ======================================================================== */
/* Task support code                                                        */
/* ======================================================================== */

pub unsafe fn bcom_ata_init(queue_len: i32, maxbufsize: i32) -> *mut bcom_task {
    /* Prefetch breaks ATA DMA.  Turn it off for ATA DMA */
    bcom_disable_prefetch();

    let tsk = bcom_task_alloc(queue_len, size_of::<bcom_ata_bd>(), 0);
    if tsk.is_null() {
        return core::ptr::null_mut();
    }

    (*tsk).flags = BCOM_FLAGS_NONE;

    bcom_ata_reset_bd(tsk);

    let var = bcom_task_var((*tsk).tasknum) as *mut bcom_ata_var;
    let _inc = bcom_task_inc((*tsk).tasknum) as *mut bcom_ata_inc;

    if bcom_load_image((*tsk).tasknum, bcom_ata_task.as_ptr()) != 0 {
        bcom_task_free(tsk);
        return core::ptr::null_mut();
    }

    (*var).enable = (*bcom_eng).regs_base
        + offset_of!(mpc52xx_sdma, tcr[(*tsk).tasknum]);
    (*var).bd_base = (*tsk).bd_pa;
    (*var).bd_last = (*tsk).bd_pa + (((*tsk).num_bd - 1) * (*tsk).bd_size);
    (*var).bd_start = (*tsk).bd_pa;
    (*var).buffer_size = maxbufsize as u32;

    /* Configure some stuff */
    bcom_set_task_pragma((*tsk).tasknum, BCOM_ATA_PRAGMA);
    bcom_set_task_auto_start((*tsk).tasknum, (*tsk).tasknum);

    out_8(
        &mut (*bcom_eng).regs.as_mut().ipr[BCOM_INITIATOR_ATA_RX],
        BCOM_IPR_ATA_RX,
    );
    out_8(
        &mut (*bcom_eng).regs.as_mut().ipr[BCOM_INITIATOR_ATA_TX],
        BCOM_IPR_ATA_TX,
    );

    out_be32(&mut (*bcom_eng).regs.as_mut().IntPend, 1u32 << (*tsk).tasknum); /* Clear ints */

    tsk
}

pub unsafe fn bcom_ata_rx_prepare(tsk: *mut bcom_task) {
    let inc = bcom_task_inc((*tsk).tasknum) as *mut bcom_ata_inc;

    (*inc).incr_bytes = -(size_of::<u32>() as i16);
    (*inc).incr_src = 0;
    (*inc).incr_dst = size_of::<u32>() as i16;

    bcom_set_initiator((*tsk).tasknum, BCOM_INITIATOR_ATA_RX);
}

pub unsafe fn bcom_ata_tx_prepare(tsk: *mut bcom_task) {
    let inc = bcom_task_inc((*tsk).tasknum) as *mut bcom_ata_inc;

    (*inc).incr_bytes = -(size_of::<u32>() as i16);
    (*inc).incr_src = size_of::<u32>() as i16;
    (*inc).incr_dst = 0;

    bcom_set_initiator((*tsk).tasknum, BCOM_INITIATOR_ATA_TX);
}

pub unsafe fn bcom_ata_reset_bd(tsk: *mut bcom_task) {
    /* Reset all BD */
    memset_io((*tsk).bd, 0x00, (*tsk).num_bd * (*tsk).bd_size);

    (*tsk).index = 0;
    (*tsk).outdex = 0;

    let var = bcom_task_var((*tsk).tasknum) as *mut bcom_ata_var;
    (*var).bd_start = (*var).bd_base;
}

pub unsafe fn bcom_ata_release(tsk: *mut bcom_task) {
    /* Nothing special for the ATA tasks */
    bcom_task_free(tsk);
}

/* Exported symbols: EXPORT_SYMBOL_GPL */

unsafe extern "C" {
    static mut bcom_eng: *mut bcom_engine;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
