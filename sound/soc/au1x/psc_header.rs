/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Alchemy ALSA ASoC audio support.
 *
 * (c) 2007-2011 MSC Vertriebsges.m.b.H.,
 *	Manuel Lauss <manuel.lauss@gmail.com>
 */

use core::ffi::{c_int, c_ulong, c_void};

#[repr(C)]
pub struct au1xpsc_audio_data {
    pub mmio: *mut c_void,

    pub cfg: c_ulong,
    pub rate: c_ulong,

    pub dai_drv: snd_soc_dai_driver,

    pub pm: [c_ulong; 2],
    pub lock: mutex,
    pub dmaids: [c_int; 2],
}

/* easy access macros */
pub unsafe fn PSC_CTRL(x: *mut au1xpsc_audio_data) -> *mut c_void {
    unsafe { ((*x).mmio as *mut u8).add(PSC_CTRL_OFFSET as usize) as *mut c_void }
}

pub unsafe fn PSC_SEL(x: *mut au1xpsc_audio_data) -> *mut c_void {
    unsafe { ((*x).mmio as *mut u8).add(PSC_SEL_OFFSET as usize) as *mut c_void }
}

pub unsafe fn I2S_STAT(x: *mut au1xpsc_audio_data) -> *mut c_void {
    unsafe { ((*x).mmio as *mut u8).add(PSC_I2SSTAT_OFFSET as usize) as *mut c_void }
}

pub unsafe fn I2S_CFG(x: *mut au1xpsc_audio_data) -> *mut c_void {
    unsafe { ((*x).mmio as *mut u8).add(PSC_I2SCFG_OFFSET as usize) as *mut c_void }
}

pub unsafe fn I2S_PCR(x: *mut au1xpsc_audio_data) -> *mut c_void {
    unsafe { ((*x).mmio as *mut u8).add(PSC_I2SPCR_OFFSET as usize) as *mut c_void }
}

pub unsafe fn AC97_CFG(x: *mut au1xpsc_audio_data) -> *mut c_void {
    unsafe { ((*x).mmio as *mut u8).add(PSC_AC97CFG_OFFSET as usize) as *mut c_void }
}

pub unsafe fn AC97_CDC(x: *mut au1xpsc_audio_data) -> *mut c_void {
    unsafe { ((*x).mmio as *mut u8).add(PSC_AC97CDC_OFFSET as usize) as *mut c_void }
}

pub unsafe fn AC97_EVNT(x: *mut au1xpsc_audio_data) -> *mut c_void {
    unsafe { ((*x).mmio as *mut u8).add(PSC_AC97EVNT_OFFSET as usize) as *mut c_void }
}

pub unsafe fn AC97_PCR(x: *mut au1xpsc_audio_data) -> *mut c_void {
    unsafe { ((*x).mmio as *mut u8).add(PSC_AC97PCR_OFFSET as usize) as *mut c_void }
}

pub unsafe fn AC97_RST(x: *mut au1xpsc_audio_data) -> *mut c_void {
    unsafe { ((*x).mmio as *mut u8).add(PSC_AC97RST_OFFSET as usize) as *mut c_void }
}

pub unsafe fn AC97_STAT(x: *mut au1xpsc_audio_data) -> *mut c_void {
    unsafe { ((*x).mmio as *mut u8).add(PSC_AC97STAT_OFFSET as usize) as *mut c_void }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
