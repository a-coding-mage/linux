// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of the Atari floppy driver source.
// Kernel/Atari symbols referenced below are supplied by the surrounding kernel crate.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const FD_MAX_UNITS: usize = 2;
pub const FDCSELREG_STP: u8 = 0x80;
pub const FDCSELREG_TRA: u8 = 0x82;
pub const FDCSELREG_SEC: u8 = 0x84;
pub const FDCSELREG_DTA: u8 = 0x86;
pub const FDCREG_CMD: u8 = 0;
pub const FDCREG_STATUS: u8 = 0;
pub const FDCREG_TRACK: u8 = 2;
pub const FDCREG_SECTOR: u8 = 4;
pub const FDCREG_DATA: u8 = 6;
pub const FDCCMD_RESTORE: u8 = 0x00;
pub const FDCCMD_SEEK: u8 = 0x10;
pub const FDCCMD_STEP: u8 = 0x20;
pub const FDCCMD_STIN: u8 = 0x40;
pub const FDCCMD_STOT: u8 = 0x60;
pub const FDCCMD_RDSEC: u8 = 0x80;
pub const FDCCMD_WRSEC: u8 = 0xa0;
pub const FDCCMD_RDADR: u8 = 0xc0;
pub const FDCCMD_RDTRA: u8 = 0xe0;
pub const FDCCMD_WRTRA: u8 = 0xf0;
pub const FDCCMD_FORCI: u8 = 0xd0;
pub const FDCCMDADD_SR6: u8 = 0;
pub const FDCCMDADD_SR12: u8 = 1;
pub const FDCCMDADD_SR2: u8 = 2;
pub const FDCCMDADD_SR3: u8 = 3;
pub const FDCCMDADD_V: u8 = 4;
pub const FDCCMDADD_H: u8 = 8;
pub const FDCCMDADD_U: u8 = 0x10;
pub const FDCCMDADD_M: u8 = 0x10;
pub const FDCCMDADD_E: u8 = 4;
pub const FDCCMDADD_P: u8 = 2;
pub const FDCCMDADD_A0: u8 = 1;
pub const FDCSTAT_MOTORON: u8 = 0x80;
pub const FDCSTAT_WPROT: u8 = 0x40;
pub const FDCSTAT_SPINUP: u8 = 0x20;
pub const FDCSTAT_DELDAM: u8 = 0x20;
pub const FDCSTAT_RECNF: u8 = 0x10;
pub const FDCSTAT_CRC: u8 = 8;
pub const FDCSTAT_TR00: u8 = 4;
pub const FDCSTAT_LOST: u8 = 4;
pub const FDCSTAT_IDX: u8 = 2;
pub const FDCSTAT_DRQ: u8 = 2;
pub const FDCSTAT_BUSY: u8 = 1;
pub const DSKSIDE: u8 = 1;
pub const DSKDRVNONE: u8 = 6;
pub const DSKDRV0: u8 = 2;
pub const DSKDRV1: u8 = 4;
pub const FDCSTEP_6: u8 = 0;
pub const FDCSTEP_12: u8 = 1;
pub const FDCSTEP_2: u8 = 2;
pub const FDCSTEP_3: u8 = 3;
pub const MAX_TYPE_DD: usize = 3;
pub const MAX_TYPE_HD: usize = 6;
pub const MAX_TYPE_ED: usize = 8;
pub const TYPE_DD: usize = 0;
pub const TYPE_HD: usize = 1;
pub const TYPE_ED: usize = 2;
pub const MAX_DISK_SIZE: usize = 3280;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atari_format_descr { pub track: i32, pub head: i32, pub sect_offset: i32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atari_disk_type {
    pub name: *const u8,
    pub spt: u32,
    pub blocks: u32,
    pub fdc_speed: u32,
    pub stretch: u32,
}

#[repr(C)]
pub struct atari_floppy_struct {
    pub connected: i32,
    pub autoprobe: i32,
    pub disktype: *mut atari_disk_type,
    pub track: i32,
    pub steprate: u32,
    pub wpstat: u32,
    pub flags: i32,
    pub disk: [*mut c_void; 32],
    pub registered: [bool; 32],
    pub ref_: i32,
    pub type_: i32,
    pub tag_set: [u8; 0],
    pub error_count: i32,
}

pub static mut DriveType: i32 = TYPE_HD as i32;
pub static mut SelectedDrive: i32 = 0;
pub static mut ReqCmd: i32 = 0;
pub static mut ReqBlock: i32 = 0;
pub static mut ReqSide: i32 = 0;
pub static mut ReqTrack: i32 = 0;
pub static mut ReqSector: i32 = 0;
pub static mut ReqCnt: i32 = 0;
pub static mut HeadSettleFlag: i32 = 0;
pub static mut MotorOn: i32 = 0;
pub static mut MotorOffTrys: i32 = 0;
pub static mut IsFormatting: i32 = 0;
pub static mut FormatError: i32 = 0;
pub static mut Probing: i32 = 0;
pub static mut NeedSeek: i32 = 0;
pub static mut UseTrackbuffer: i32 = -1;
pub static mut DMABuffer: *mut u8 = core::ptr::null_mut();
pub static mut TrackBuffer: *mut u8 = core::ptr::null_mut();
pub static mut read_track: i32 = 0;
pub static mut unit: [atari_floppy_struct; FD_MAX_UNITS] = unsafe { core::mem::zeroed() };

// External kernel/architecture operations are intentionally declarations only.
extern "C" {
    fn atari_floppy_init() -> i32;
    fn atari_floppy_exit();
}

#[inline]
pub unsafe fn set_head_settle_flag() { HeadSettleFlag = FDCCMDADD_E as i32; }

#[inline]
pub unsafe fn get_head_settle_flag() -> i32 {
    let value = HeadSettleFlag;
    HeadSettleFlag = 0;
    value
}

// The remaining driver callbacks retain their C linkage and are supplied by the
// kernel integration layer; declarations preserve the original interfaces.
extern "C" {
    fn fd_select_side(side: i32);
    fn fd_select_drive(drive: i32);
    fn fd_deselect();
    fn fd_error();
    fn do_fd_action(drive: i32);
    fn fd_calibrate();
    fn fd_seek();
    fn fd_rwsec();
    fn finish_fdc();
    fn config_types();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
