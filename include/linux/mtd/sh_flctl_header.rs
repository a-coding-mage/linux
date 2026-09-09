/* SPDX-License-Identifier: GPL-2.0
 *
 * SuperH FLCTL nand controller
 *
 * Copyright © 2008 Renesas Solutions Corp.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

use core::ffi::c_void;

/* FLCTL registers */
#[inline] pub unsafe fn FLCMNCR(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x0) }
#[inline] pub unsafe fn FLCMDCR(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x4) }
#[inline] pub unsafe fn FLCMCDR(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x8) }
#[inline] pub unsafe fn FLADR(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0xC) }
#[inline] pub unsafe fn FLADR2(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x3C) }
#[inline] pub unsafe fn FLDATAR(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x10) }
#[inline] pub unsafe fn FLDTCNTR(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x14) }
#[inline] pub unsafe fn FLINTDMACR(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x18) }
#[inline] pub unsafe fn FLBSYTMR(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x1C) }
#[inline] pub unsafe fn FLBSYCNT(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x20) }
#[inline] pub unsafe fn FLDTFIFO(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x24) }
#[inline] pub unsafe fn FLECFIFO(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x28) }
#[inline] pub unsafe fn FLTRCR(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x2C) }
#[inline] pub unsafe fn FLHOLDCR(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x38) }
#[inline] pub unsafe fn FL4ECCRESULT0(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x80) }
#[inline] pub unsafe fn FL4ECCRESULT1(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x84) }
#[inline] pub unsafe fn FL4ECCRESULT2(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x88) }
#[inline] pub unsafe fn FL4ECCRESULT3(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x8C) }
#[inline] pub unsafe fn FL4ECCCR(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x90) }
#[inline] pub unsafe fn FL4ECCCNT(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x94) }
#[inline] pub unsafe fn FLERRADR(f: *mut sh_flctl) -> *mut u8 { (*f).reg.cast::<u8>().add(0x98) }

pub const _4ECCCNTEN: u32 = 0x1 << 24; pub const _4ECCEN: u32 = 0x1 << 23;
pub const _4ECCCORRECT: u32 = 0x1 << 22; pub const SHBUSSEL: u32 = 0x1 << 20;
pub const SEL_16BIT: u32 = 0x1 << 19; pub const SNAND_E: u32 = 0x1 << 18;
pub const QTSEL_E: u32 = 0x1 << 17; pub const ENDIAN: u32 = 0x1 << 16;
pub const FCKSEL_E: u32 = 0x1 << 15; pub const ACM_SACCES_MODE: u32 = 0x01 << 10;
pub const NANWF_E: u32 = 0x1 << 9; pub const SE_D: u32 = 0x1 << 8;
pub const CE1_ENABLE: u32 = 0x1 << 4; pub const CE0_ENABLE: u32 = 0x1 << 3;
pub const TYPESEL_SET: u32 = 0x1;
pub const PULSE3: u32 = 0x1 << 27; pub const PULSE2: u32 = 0x1 << 17;
pub const PULSE1: u32 = 0x1 << 15; pub const PULSE0: u32 = 0x1 << 9;
pub const CLK_8B_0_5: u32 = PULSE1; pub const CLK_8B_1: u32 = 0;
pub const CLK_8B_1_5: u32 = PULSE1 | PULSE2; pub const CLK_8B_2: u32 = PULSE0;
pub const CLK_8B_3: u32 = PULSE0 | PULSE1 | PULSE2; pub const CLK_8B_4: u32 = PULSE0 | PULSE2;
pub const CLK_16B_6L_2H: u32 = PULSE0; pub const CLK_16B_9L_3H: u32 = PULSE0 | PULSE1 | PULSE2;
pub const CLK_16B_12L_4H: u32 = PULSE0 | PULSE2;

pub const ADRCNT2_E: u32 = 0x1 << 31; pub const ADRMD_E: u32 = 0x1 << 26;
pub const CDSRC_E: u32 = 0x1 << 25; pub const DOSR_E: u32 = 0x1 << 24;
pub const SELRW: u32 = 0x1 << 21; pub const DOADR_E: u32 = 0x1 << 20;
pub const ADRCNT_1: u32 = 0x00 << 18; pub const ADRCNT_2: u32 = 0x01 << 18;
pub const ADRCNT_3: u32 = 0x02 << 18; pub const ADRCNT_4: u32 = 0x03 << 18;
pub const DOCMD2_E: u32 = 0x1 << 17; pub const DOCMD1_E: u32 = 0x1 << 16;
pub const ESTERINTE: u32 = 0x1 << 24; pub const AC1CLR: u32 = 0x1 << 19;
pub const AC0CLR: u32 = 0x1 << 18; pub const DREQ0EN: u32 = 0x1 << 16;
pub const ECERB: u32 = 0x1 << 9; pub const STERB: u32 = 0x1 << 8; pub const STERINTE: u32 = 0x1 << 4;
pub const TRSTRT: u32 = 0x1; pub const TREND: u32 = 0x1 << 1; pub const HOLDEN: u32 = 0x1;
pub const _4ECCFA: u32 = 0x1 << 2; pub const _4ECCEND: u32 = 0x1 << 1; pub const _4ECCEXST: u32 = 0x1;
pub const LOOP_TIMEOUT_MAX: u32 = 0x00010000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flctl_ecc_res_t { FL_SUCCESS, FL_REPAIRABLE, FL_ERROR, FL_TIMEOUT }

pub enum dma_chan {}

#[repr(C)]
pub struct sh_flctl {
    pub chip: nand_chip, pub pdev: *mut platform_device, pub pm_qos: dev_pm_qos_request,
    pub reg: *mut c_void, pub fifo: resource_size_t, pub done_buff: [u8; 2048 + 64],
    pub read_bytes: i32, pub index: u32, pub seqin_column: i32, pub seqin_page_addr: i32,
    pub seqin_read_cmd: u32, pub erase1_page_addr: i32, pub erase_ADRCNT: u32,
    pub rw_ADRCNT: u32, pub flcmncr_base: u32, pub flintdmacr_base: u32,
    // C bit-fields; stored as their original unsigned word representation.
    pub page_size: u32, pub hwecc: u32, pub holden: u32, pub qos_request: u32,
    pub chan_fifo0_rx: *mut dma_chan, pub chan_fifo0_tx: *mut dma_chan,
    pub dma_complete: completion,
}

#[repr(C)]
pub struct sh_flctl_platform_data {
    pub parts: *mut mtd_partition, pub nr_parts: i32, pub flcmncr_val: c_ulong,
    pub has_hwecc: u32, pub use_holden: u32,
    pub slave_id_fifo0_tx: u32, pub slave_id_fifo0_rx: u32,
}

#[inline]
pub unsafe fn mtd_to_flctl(mtdinfo: *mut mtd_info) -> *mut sh_flctl {
    container_of(mtd_to_nand(mtdinfo), core::mem::offset_of!(sh_flctl, chip), core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
