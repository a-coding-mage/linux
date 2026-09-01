/* SPDX-License-Identifier: GPL-2.0-or-later */
/* -*- linux-c -*- *
 *
 * ALSA driver for the digigram lx6464es interface
 * low-level interface
 *
 * Copyright (c) 2009 Tim Blechmann <tim@klingt.org>
 */

/* Depends on linux/interrupt.h and lx_defs.h in the original C header. */

pub const REG_CRM_NUMBER: usize = 12;

#[repr(C)]
pub struct lx6464es {
    _unused: [u8; 0],
}

/* low-level register access */

/* dsp register access */
pub const eReg_BASE: ::core::ffi::c_int = 0;
pub const eReg_CSM: ::core::ffi::c_int = 1;
pub const eReg_CRM1: ::core::ffi::c_int = 2;
pub const eReg_CRM2: ::core::ffi::c_int = 3;
pub const eReg_CRM3: ::core::ffi::c_int = 4;
pub const eReg_CRM4: ::core::ffi::c_int = 5;
pub const eReg_CRM5: ::core::ffi::c_int = 6;
pub const eReg_CRM6: ::core::ffi::c_int = 7;
pub const eReg_CRM7: ::core::ffi::c_int = 8;
pub const eReg_CRM8: ::core::ffi::c_int = 9;
pub const eReg_CRM9: ::core::ffi::c_int = 10;
pub const eReg_CRM10: ::core::ffi::c_int = 11;
pub const eReg_CRM11: ::core::ffi::c_int = 12;
pub const eReg_CRM12: ::core::ffi::c_int = 13;
pub const eReg_ICR: ::core::ffi::c_int = 14;
pub const eReg_CVR: ::core::ffi::c_int = 15;
pub const eReg_ISR: ::core::ffi::c_int = 16;
pub const eReg_RXHTXH: ::core::ffi::c_int = 17;
pub const eReg_RXMTXM: ::core::ffi::c_int = 18;
pub const eReg_RHLTXL: ::core::ffi::c_int = 19;
pub const eReg_RESETDSP: ::core::ffi::c_int = 20;
pub const eReg_CSUF: ::core::ffi::c_int = 21;
pub const eReg_CSES: ::core::ffi::c_int = 22;
pub const eReg_CRESMSB: ::core::ffi::c_int = 23;
pub const eReg_CRESLSB: ::core::ffi::c_int = 24;
pub const eReg_ADMACESMSB: ::core::ffi::c_int = 25;
pub const eReg_ADMACESLSB: ::core::ffi::c_int = 26;
pub const eReg_CONFES: ::core::ffi::c_int = 27;
pub const eMaxPortLx: ::core::ffi::c_int = 28;

unsafe extern "C" {
    pub fn lx_dsp_reg_read(chip: *mut lx6464es, port: ::core::ffi::c_int) -> ::core::ffi::c_ulong;
    pub fn lx_dsp_reg_write(chip: *mut lx6464es, port: ::core::ffi::c_int, data: ::core::ffi::c_uint);
}

/* plx register access */
pub const ePLX_PCICR: ::core::ffi::c_int = 0;
pub const ePLX_MBOX0: ::core::ffi::c_int = 1;
pub const ePLX_MBOX1: ::core::ffi::c_int = 2;
pub const ePLX_MBOX2: ::core::ffi::c_int = 3;
pub const ePLX_MBOX3: ::core::ffi::c_int = 4;
pub const ePLX_MBOX4: ::core::ffi::c_int = 5;
pub const ePLX_MBOX5: ::core::ffi::c_int = 6;
pub const ePLX_MBOX6: ::core::ffi::c_int = 7;
pub const ePLX_MBOX7: ::core::ffi::c_int = 8;
pub const ePLX_L2PCIDB: ::core::ffi::c_int = 9;
pub const ePLX_IRQCS: ::core::ffi::c_int = 10;
pub const ePLX_CHIPSC: ::core::ffi::c_int = 11;
pub const eMaxPort: ::core::ffi::c_int = 12;

unsafe extern "C" {
    pub fn lx_plx_reg_read(chip: *mut lx6464es, port: ::core::ffi::c_int) -> ::core::ffi::c_ulong;
    pub fn lx_plx_reg_write(chip: *mut lx6464es, port: ::core::ffi::c_int, data: u32);
}

/* rhm */
#[repr(C)]
pub struct lx_rmh {
    pub cmd_len: u16,  /* length of the command to send (WORDs) */
    pub stat_len: u16, /* length of the status received (WORDs) */
    pub dsp_stat: u16, /* status type, RMP_SSIZE_XXX */
    pub cmd_idx: u16,  /* index of the command */
    pub cmd: [u32; REG_CRM_NUMBER],
    pub stat: [u32; REG_CRM_NUMBER],
}

unsafe extern "C" {
    /* low-level dsp access */
    pub fn lx_dsp_get_version(chip: *mut lx6464es, rdsp_version: *mut u32) -> ::core::ffi::c_int;
    pub fn lx_dsp_get_clock_frequency(chip: *mut lx6464es, rfreq: *mut u32) -> ::core::ffi::c_int;
    pub fn lx_dsp_set_granularity(chip: *mut lx6464es, gran: u32) -> ::core::ffi::c_int;
    pub fn lx_dsp_read_async_events(chip: *mut lx6464es, data: *mut u32) -> ::core::ffi::c_int;
    pub fn lx_dsp_get_mac(chip: *mut lx6464es) -> ::core::ffi::c_int;

    /* low-level pipe handling */
    pub fn lx_pipe_allocate(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
        channels: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn lx_pipe_release(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn lx_pipe_sample_count(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
        rsample_count: *mut u64,
    ) -> ::core::ffi::c_int;
    pub fn lx_pipe_state(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
        rstate: *mut u16,
    ) -> ::core::ffi::c_int;
    pub fn lx_pipe_stop(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn lx_pipe_start(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn lx_pipe_pause(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn lx_pipe_wait_for_start(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn lx_pipe_wait_for_idle(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    /* low-level stream handling */
    pub fn lx_stream_set_format(
        chip: *mut lx6464es,
        runtime: *mut snd_pcm_runtime,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn lx_stream_state(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
        rstate: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn lx_stream_sample_position(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
        r_bytepos: *mut u64,
    ) -> ::core::ffi::c_int;

    pub fn lx_stream_set_state(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
        state: stream_state_t,
    ) -> ::core::ffi::c_int;

    /* low-level buffer handling */
    pub fn lx_buffer_ask(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
        r_needed: *mut u32,
        r_freed: *mut u32,
        size_array: *mut u32,
    ) -> ::core::ffi::c_int;
    pub fn lx_buffer_give(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
        buffer_size: u32,
        buf_address_lo: u32,
        buf_address_hi: u32,
        r_buffer_index: *mut u32,
    ) -> ::core::ffi::c_int;
    pub fn lx_buffer_free(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
        r_buffer_size: *mut u32,
    ) -> ::core::ffi::c_int;
    pub fn lx_buffer_cancel(
        chip: *mut lx6464es,
        pipe: u32,
        is_capture: ::core::ffi::c_int,
        buffer_index: u32,
    ) -> ::core::ffi::c_int;

    /* low-level gain/peak handling */
    pub fn lx_level_unmute(
        chip: *mut lx6464es,
        is_capture: ::core::ffi::c_int,
        unmute: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn lx_level_peaks(
        chip: *mut lx6464es,
        is_capture: ::core::ffi::c_int,
        channels: ::core::ffi::c_int,
        r_levels: *mut u32,
    ) -> ::core::ffi::c_int;

    /* interrupt handling */
    pub fn lx_interrupt(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t;
    pub fn lx_threaded_irq(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t;
    pub fn lx_irq_enable(chip: *mut lx6464es);
    pub fn lx_irq_disable(chip: *mut lx6464es);
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _unused: [u8; 0],
}

/* enum stream_state_t comes from lx_defs.h in the original C header. */
pub type stream_state_t = ::core::ffi::c_uint;

/* irqreturn_t comes from linux/interrupt.h in the original C header. */
pub type irqreturn_t = ::core::ffi::c_uint;

unsafe extern "C" {
    pub static SSTATE_RUN: stream_state_t;
    pub static SSTATE_PAUSE: stream_state_t;
    pub static SSTATE_STOP: stream_state_t;
}

#[inline]
pub unsafe fn lx_stream_start(
    chip: *mut lx6464es,
    pipe: u32,
    is_capture: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { lx_stream_set_state(chip, pipe, is_capture, SSTATE_RUN) }
}

#[inline]
pub unsafe fn lx_stream_pause(
    chip: *mut lx6464es,
    pipe: u32,
    is_capture: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { lx_stream_set_state(chip, pipe, is_capture, SSTATE_PAUSE) }
}

#[inline]
pub unsafe fn lx_stream_stop(
    chip: *mut lx6464es,
    pipe: u32,
    is_capture: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { lx_stream_set_state(chip, pipe, is_capture, SSTATE_STOP) }
}

/* Stream Format Header Defines (for LIN and IEEE754) */
pub const HEADER_FMT_BASE: u32 = HEADER_FMT_BASE_LIN;
pub const HEADER_FMT_BASE_LIN: u32 = 0xFED00000;
pub const HEADER_FMT_BASE_FLOAT: u32 = 0xFAD00000;
pub const HEADER_FMT_MONO: u32 = 0x00000080; /* bit 23 in header_lo. WARNING: old
                                             * bit 22 is ignored in float
                                             * format */
pub const HEADER_FMT_INTEL: u32 = 0x00008000;
pub const HEADER_FMT_16BITS: u32 = 0x00002000;
pub const HEADER_FMT_24BITS: u32 = 0x00004000;
pub const HEADER_FMT_UPTO11: u32 = 0x00000200; /* frequency is less or equ. to 11k.
                                               * */
pub const HEADER_FMT_UPTO32: u32 = 0x00000100; /* frequency is over 11k and less
                                               * then 32k.*/

pub const BIT_FMP_HEADER: ::core::ffi::c_int = 23;
pub const BIT_FMP_SD: ::core::ffi::c_int = 22;
pub const BIT_FMP_MULTICHANNEL: ::core::ffi::c_int = 19;

pub const START_STATE: ::core::ffi::c_int = 1;
pub const PAUSE_STATE: ::core::ffi::c_int = 0;

/* from PcxAll_e.h */
/* Start/Pause condition for pipes (PCXStartPipe, PCXPausePipe) */
pub const START_PAUSE_IMMEDIATE: ::core::ffi::c_int = 0;
pub const START_PAUSE_ON_SYNCHRO: ::core::ffi::c_int = 1;
pub const START_PAUSE_ON_TIME_CODE: ::core::ffi::c_int = 2;

/* Pipe / Stream state */
/* START_STATE and PAUSE_STATE are defined a second time in the C header. */

pub type dma_addr_t = usize;

#[inline]
pub unsafe fn unpack_pointer(ptr: dma_addr_t, r_low: *mut u32, r_high: *mut u32) {
    unsafe {
        *r_low = (ptr & 0xffffffff) as u32;
        #[cfg(target_pointer_width = "32")]
        {
            *r_high = 0;
        }
        #[cfg(not(target_pointer_width = "32"))]
        {
            *r_high = ((ptr as u64) >> 32) as u32;
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
