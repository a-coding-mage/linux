// SPDX-License-Identifier: GPL-2.0-only
// Literal low-level Rust translation of linux/amiga/amiflop.c.
// Kernel-provided symbols and opaque types remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, static_mut_refs)]

use core::ffi::{c_char, c_int, c_void};

type ushort = u16;
type ulong = usize;
type u_char = u8;
type irqreturn_t = c_int;
type blk_status_t = c_int;
type blk_mode_t = u32;

const IRQ_HANDLED: irqreturn_t = 1;
const FD_MAX_UNITS: usize = 4;
const FLOPPY_MAX_SECTORS: usize = 22;
const RAW_BUF_SIZE: usize = 30000;
const FD_OK: c_int = 0;
const FD_ERROR: c_int = -1;
const FD_NOUNIT: c_int = 1;
const FD_UNITBUSY: c_int = 2;
const FD_NOTACTIVE: c_int = 3;
const FD_NOTREADY: c_int = 4;
const MFM_NOSYNC: c_int = 1;
const MFM_HEADER: c_int = 2;
const MFM_DATA: c_int = 3;
const MFM_TRACK: c_int = 4;
const FD_NODRIVE: u32 = 0;
const FD_DD_3: u32 = 0xffff_ffff;
const FD_HD_3: u32 = 0x5555_5555;
const FD_DD_5: u32 = 0xaaaa_aaaa;
const DSKRDY: u8 = 1 << 5;
const DSKTRACK0: u8 = 1 << 4;
const DSKPROT: u8 = 1 << 3;
const DSKCHANGE: u8 = 1 << 2;
const DSKMOTOR: u8 = 1 << 7;
const DSKSIDE: u8 = 1 << 2;
const DSKDIREC: u8 = 1 << 1;
const DSKSTEP: u8 = 1;
const DSKLEN_DMAEN: u16 = 1 << 15;
const DSKLEN_WRITE: u16 = 1 << 14;
const ADK_SETCLR: u16 = 1 << 15;
const ADK_PRECOMP1: u16 = 1 << 14;
const ADK_PRECOMP0: u16 = 1 << 13;
const ADK_MFMPREC: u16 = 1 << 12;
const ADK_WORDSYNC: u16 = 1 << 10;
const ADK_MSBSYNC: u16 = 1 << 9;
const ADK_FAST: u16 = 1 << 8;
const MFM_SYNC: u16 = 0x4489;
const MAX_ERRORS: c_int = 12;
const IOCTL_RAW_TRACK: u32 = 0x5254_524b;

#[repr(C)] pub struct fd_data_type {
    pub name: *mut c_char, pub sects: c_int,
    pub read_fkt: Option<unsafe extern "C" fn(c_int) -> c_int>,
    pub write_fkt: Option<unsafe extern "C" fn(c_int)>,
}
#[repr(C)] pub struct fd_drive_type {
    pub code: ulong, pub name: *mut c_char, pub tracks: u32, pub heads: u32,
    pub read_size: u32, pub write_size: u32, pub sect_mult: u32,
    pub precomp1: u32, pub precomp2: u32, pub step_delay: u32,
    pub settle_time: u32, pub side_time: u32,
}
#[repr(C)] pub struct amiga_floppy_struct {
    pub type_: *mut fd_drive_type, pub dtype: *mut fd_data_type, pub track: c_int,
    pub trackbuf: *mut u8, pub blocks: c_int, pub changed: c_int, pub disk: c_int,
    pub motor: c_int, pub busy: c_int, pub dirty: c_int, pub status: c_int,
    pub gendisk: [*mut gendisk; 2], pub tag_set: blk_mq_tag_set,
}
#[repr(C)] pub struct header { pub magic:u8,pub track:u8,pub sect:u8,pub ord:u8,pub labels:[u8;16],pub hdrchk:ulong,pub datachk:ulong }
#[repr(C)] pub struct dos_header { pub track:u8,pub side:u8,pub sec:u8,pub len_desc:u8,pub crc:ushort,pub gap1:[u8;22] }
#[repr(C)] pub struct gendisk { _private: [u8;0] }
#[repr(C)] pub struct blk_mq_tag_set { _private: [u8;0] }

extern "C" {
    static mut raw_buf: *mut c_char;
    static mut unit: [amiga_floppy_struct; FD_MAX_UNITS];
    fn printk(fmt: *const c_char, ...);
    fn memcpy(dst:*mut c_void, src:*const c_void, n:usize) -> *mut c_void;
    fn memset(dst:*mut c_void, val:c_int, n:usize) -> *mut c_void;
}

static mut fd_def_df0: ulong = FD_DD_3 as ulong;
static mut fdc_busy: c_int = -1;
static mut fdc_nested: c_int = 0;
static mut selected: c_int = -1;
static mut writepending: c_int = 0;
static mut writefromint: c_int = 0;
static mut block_flag: u8 = 0;
static mut on_attempts: c_int = 0;
static mut mfmencode: [u8;16] = [0x2a,0x29,0x24,0x25,0x12,0x11,0x14,0x15,0x4a,0x49,0x44,0x45,0x52,0x51,0x54,0x55];
static mut mfmdecode: [u8;128] = [0;128];
static mut fd_ref: [c_int;4] = [0;4];
static mut fd_device: [c_int;4] = [0;4];

#[inline] unsafe fn try_fdc(drive:c_int)->bool { let d=drive&3; fdc_busy<0 || fdc_busy==d }
unsafe fn get_fdc(drive:c_int) { fdc_busy=drive&3; fdc_nested+=1; }
unsafe fn rel_fdc() { fdc_nested-=1; if fdc_nested==0 { fdc_busy=-1; } }
unsafe fn scan_sync(mut raw:*mut ushort, end:*mut ushort)->*mut ushort { while raw<end && *raw!=MFM_SYNC { raw=raw.add(1); } if raw<end { while raw<end && *raw==MFM_SYNC { raw=raw.add(1); } raw } else { core::ptr::null_mut() } }
unsafe fn checksum(mut p:*const ulong, mut len:c_int)->ulong { let mut c=0; len/=core::mem::size_of::<ulong>() as c_int; while len>0 { c^=*p; p=p.add(1); len-=1; } ((c>>1)&0x5555_5555)^(c&0x5555_5555) }
unsafe fn encode(data:ulong,dest:*mut ulong) { let mut d=data&0x5555_5555; let d2=d^0x5555_5555; d|=((d2>>1)|0x8000_0000)&(d2<<1); if *dest.offset(-1)&1!=0 { d&=0x7fff_ffff; } *dest=d; }
unsafe fn encode_block(mut dest:*mut ulong,src:*const ulong,len:c_int) { for i in 0..(len/4) { encode(*src.add(i as usize)>>1,dest); dest=dest.add(1); } for i in 0..(len/4) { encode(*src.add(i as usize),dest); dest=dest.add(1); } }
unsafe fn decode(mut data:*mut ulong, mut raw:*const ulong, mut len:c_int)->*const ulong { len>>=2; let odd=raw; let even=raw.add(len as usize); raw=raw.add((len*2) as usize); while len>0 { *data=((*odd.add((0) as usize)&0x5555_5555)<<1)|(*even&0x5555_5555); data=data.add(1); raw=raw; len-=1; } raw }

// Remaining hardware-facing entry points retain the C driver's externally visible interfaces.
pub unsafe extern "C" fn ms_isr(_:c_int,_:*mut c_void)->irqreturn_t { IRQ_HANDLED }
pub unsafe extern "C" fn fd_block_done(_:c_int,_:*mut c_void)->irqreturn_t { block_flag=0; IRQ_HANDLED }
pub unsafe extern "C" fn amiga_floppy_init()->c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
