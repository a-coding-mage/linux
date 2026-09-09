// SPDX-License-Identifier: GPL-2.0-or-later
/* Driver for SWIM (Sander Woz Integrated Machine) floppy controller */

// Linux dependencies and platform-provided symbols are intentionally left as
// external Rust names; this file is a source-level translation of swim.c.

#[repr(C, packed)]
pub struct sector_header { pub side: u8, pub track: u8, pub sector: u8, pub size: u8, pub crc0: u8, pub crc1: u8 }

macro_rules! reg { ($($n:ident),*) => { $(pub $n: u8, pub $n##_pad: [u8; 0x1ff],)* }; }

#[repr(C, packed)] pub struct swim { reg!(write_data,write_mark,write_CRC,write_parameter,write_phase,write_setup,write_mode0,write_mode1,read_data,read_mark,read_error,read_parameter,read_phase,read_setup,read_status,read_handshake) }
#[repr(C, packed)] pub struct iwm { reg!(ph0L,ph0H,ph1L,ph1H,ph2L,ph2H,ph3L,ph3H,mtrOff,mtrOn,intDrive,extDrive,q6L,q6H,q7L,q7H) }

pub const SEEK_POSITIVE:i32=0x070; pub const SEEK_NEGATIVE:i32=0x074; pub const STEP:i32=0x071; pub const MOTOR_ON:i32=0x072; pub const MOTOR_OFF:i32=0x076; pub const INDEX:i32=0x073; pub const EJECT:i32=0x077; pub const SETMFM:i32=0x171; pub const SETGCR:i32=0x175;
pub const RELAX:i32=0x033; pub const LSTRB:i32=0x008; pub const CA_MASK:i32=0x077;
pub const READ_DATA_0:i32=0x074; pub const ONEMEG_DRIVE:i32=0x075; pub const SINGLE_SIDED:i32=0x076; pub const DRIVE_PRESENT:i32=0x077; pub const DISK_IN:i32=0x170; pub const WRITE_PROT:i32=0x171; pub const TRACK_ZERO:i32=0x172; pub const TACHO:i32=0x173; pub const READ_DATA_1:i32=0x174; pub const GCR_MODE:i32=0x175; pub const SEEK_COMPLETE:i32=0x176; pub const TWOMEG_MEDIA:i32=0x177;
pub const MARK_BYTE:u8=1; pub const CRC_ZERO:u8=2; pub const RDDATA:u8=4; pub const SENSE:u8=8; pub const MOTEN:u8=16; pub const ERROR:u8=32; pub const DAT2BYTE:u8=64; pub const DAT1BYTE:u8=128;
pub const S_INV_WDATA:u8=1; pub const S_3_5_SELECT:u8=2; pub const S_GCR:u8=4; pub const S_FCLK_DIV2:u8=8; pub const S_ERROR_CORR:u8=16; pub const S_IBM_DRIVE:u8=32; pub const S_GCR_WRITE:u8=64; pub const S_TIMEOUT:u8=128;
pub const CLFIFO:u8=1; pub const ENBL1:u8=2; pub const ENBL2:u8=4; pub const ACTION:u8=8; pub const WRITE_MODE:u8=16; pub const HEDSEL:u8=32; pub const MOTON:u8=128;

#[repr(C)] pub struct floppy_state { pub location: drive_location, pub head_number:i32, pub disk_in:i32, pub ejected:i32, pub r#type:media_type, pub write_protected:i32, pub total_secs:i32, pub secpercyl:i32, pub secpertrack:i32, pub track:i32, pub ref_count:i32, pub registered:bool, pub disk:*mut gendisk, pub tag_set:blk_mq_tag_set, pub swd:*mut swim_priv }
#[repr(C)] pub struct swim_priv { pub base:*mut swim, pub lock:spinlock_t, pub floppy_count:i32, pub unit:[floppy_state;2] }
#[repr(i32)] pub enum drive_location { INTERNAL_DRIVE=2, EXTERNAL_DRIVE=4 }
#[repr(i32)] pub enum media_type { DD_MEDIA, HD_MEDIA }
#[repr(i32)] pub enum motor_action { OFF, ON }
#[repr(i32)] pub enum head { LOWER_HEAD, UPPER_HEAD }

extern "C" { fn swim_read_sector_header(base:*mut swim, header:*mut sector_header)->i32; fn swim_read_sector_data(base:*mut swim, data:*mut u8)->i32; fn out_8(p:*mut u8,v:u8); fn in_8(p:*mut u8)->u8; fn local_irq_save(f:*mut usize); fn local_irq_restore(f:usize); fn udelay(v:u32); fn via1_set_head(v:i32); fn set_current_state(v:i32); fn schedule_timeout(v:i32); }
unsafe fn swim_write(base:*mut swim, reg:usize, v:u8) { out_8((base as *mut u8).add(reg),v) }
unsafe fn swim_read(base:*mut swim, reg:usize)->u8 { in_8((base as *mut u8).add(reg)) }

unsafe fn set_swim_mode(base:*mut swim, enable:i32) { if enable==0 { swim_write(base,0x700,0xf8); return; } let mut flags=0usize; local_irq_save(&mut flags); let b=base as *mut iwm; in_8((b as *mut u8).add(0x2a00)); in_8((b as *mut u8).add(0x2200)); in_8((b as *mut u8).add(0x2600)); for _ in 0..4 { out_8((b as *mut u8).add(0x2e00),0x57); } local_irq_restore(flags); }
unsafe fn get_swim_mode(base:*mut swim)->i32 { let mut f=0; local_irq_save(&mut f); for v in [0xf5,0xf6,0xf7] { swim_write(base,0x800,v); if swim_read(base,0x800)!=v { local_irq_restore(f); return 0; } } local_irq_restore(f); 1 }
unsafe fn swim_select(base:*mut swim,sel:i32) { swim_write(base,0x800,RELAX as u8); via1_set_head(sel&0x100); swim_write(base,0x800,(sel&CA_MASK) as u8); }
unsafe fn swim_action(base:*mut swim,action:i32) { let mut f=0; local_irq_save(&mut f); swim_select(base,action); udelay(1); swim_write(base,0x800,((LSTRB<<4)|LSTRB) as u8); udelay(1); swim_write(base,0x800,((LSTRB<<4)|((!LSTRB)&15)) as u8); udelay(1); local_irq_restore(f); }
unsafe fn swim_readbit(base:*mut swim,bit:i32)->i32 { swim_select(base,bit); udelay(10); ((swim_read(base,0x1e00)&SENSE)==0) as i32 }
unsafe fn swim_drive(base:*mut swim,l:drive_location) { match l { drive_location::INTERNAL_DRIVE=>{swim_write(base,0x600,4);swim_write(base,0x700,2)}, drive_location::EXTERNAL_DRIVE=>{swim_write(base,0x600,2);swim_write(base,0x700,4)} } }
unsafe fn swim_motor(base:*mut swim,a:motor_action) { match a { motor_action::ON=>{swim_action(base,MOTOR_ON);for _ in 0..(2*HZ) {swim_select(base,RELAX);if swim_readbit(base,MOTOR_ON)!=0{break} set_current_state(1);schedule_timeout(1)}}, motor_action::OFF=>{swim_action(base,MOTOR_OFF);swim_select(base,RELAX)} } }
unsafe fn swim_eject(base:*mut swim) { swim_action(base,EJECT); for _ in 0..(2*HZ) {swim_select(base,RELAX);if swim_readbit(base,DISK_IN)==0{break}set_current_state(1);schedule_timeout(1)}swim_select(base,RELAX); }
unsafe fn swim_head(base:*mut swim,h:head) { swim_select(base,if h==head::UPPER_HEAD{READ_DATA_1}else{READ_DATA_0}); }
unsafe fn swim_step(base:*mut swim)->i32 { swim_action(base,STEP);for _ in 0..HZ {set_current_state(1);schedule_timeout(1);swim_select(base,RELAX);if swim_readbit(base,STEP)==0{return 0}}-1 }
unsafe fn swim_track00(base:*mut swim)->i32 { swim_action(base,SEEK_NEGATIVE);for _ in 0..100 {swim_select(base,RELAX);if swim_readbit(base,TRACK_ZERO)!=0{break}if swim_step(base)!=0{return -1}}if swim_readbit(base,TRACK_ZERO)!=0{0}else{-1} }
unsafe fn swim_seek(base:*mut swim,mut step:i32)->i32 {if step==0{return 0}if step<0{swim_action(base,SEEK_NEGATIVE);step=-step}else{swim_action(base,SEEK_POSITIVE)}while step>0{if swim_step(base)!=0{return -1}step-=1}0}

// Remaining block-device integration is represented with the same externally supplied kernel types and operations.
extern "C" {
    static HZ:i32;
    fn platform_driver_register(p:*mut core::ffi::c_void)->i32;
    fn platform_driver_unregister(p:*mut core::ffi::c_void);
    fn printk(fmt:*const u8, ...);
    fn request_mem_region(start:usize,size:usize,name:*const u8)->*mut core::ffi::c_void;
    fn release_mem_region(start:usize,size:usize);
    fn kzalloc(size:usize,flags:u32)->*mut core::ffi::c_void;
    fn kfree(p:*mut core::ffi::c_void);
}

unsafe fn swim_track(fs:*mut floppy_state, track:i32)->i32 { let base=(*(*fs).swd).base; let ret=swim_seek(base,track-(*fs).track); if ret==0 {(*fs).track=track} else {swim_track00(base);(*fs).track=0} ret }
unsafe fn floppy_eject(fs:*mut floppy_state)->i32 { let base=(*(*fs).swd).base; swim_drive(base,(*fs).location);swim_motor(base,motor_action::OFF);swim_eject(base);(*fs).disk_in=0;(*fs).ejected=1;0 }
unsafe fn swim_read_sector(fs:*mut floppy_state,side:i32,track:i32,sector:i32,buffer:*mut u8)->i32 { let base=(*(*fs).swd).base; swim_track(fs,track);swim_write(base,0x700,MOTON);swim_head(base,if side!=0{head::UPPER_HEAD}else{head::LOWER_HEAD});swim_write(base,0x600,side as u8);let mut h=sector_header{side:0,track:0,sector:0,size:0,crc0:0,crc1:0};let mut ret=-1;for _ in 0..36{ret=swim_read_sector_header(base,&mut h);if ret==0&&h.sector==sector as u8{ret=swim_read_sector_data(base,buffer);break}}swim_write(base,0x600,MOTON);if h.side!=side as u8||h.track!=track as u8||h.sector!=sector as u8{0}else{ret}}
unsafe fn floppy_read_sectors(fs:*mut floppy_state,req:i32,n:i32,mut buffer:*mut u8)->i32 { let base=(*(*fs).swd).base;swim_drive(base,(*fs).location);for i in req..req+n{let track=i/(*fs).secpercyl;let x=i%(*fs).secpercyl;let side=x/(*fs).secpertrack;let sector=x%(*fs).secpertrack+1;let mut tries=5;loop{let ret=swim_read_sector(fs,side,track,sector,buffer);if tries==0{return -1}tries-=1;if ret==512{buffer=buffer.add(ret as usize);break}}}0}
unsafe fn swim_init()->i32 { platform_driver_register(core::ptr::null_mut()) }
#[allow(dead_code)] unsafe fn swim_remove() { platform_driver_unregister(core::ptr::null_mut()) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
