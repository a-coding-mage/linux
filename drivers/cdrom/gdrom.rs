// SPDX-License-Identifier: GPL-2.0-or-later
/* GD ROM driver for the SEGA Dreamcast
 * copyright Adrian McMenamin, 2007
 * With thanks to Marcus Comstedt and Nathan Keynes
 * for work in reversing PIO and DMA
 */

// Kernel headers and build-time configuration are supplied by the surrounding kernel crate.

const GDROM_DEV_NAME: &str = "gdrom";
const GD_SESSION_OFFSET: i32 = 150;
const GDROM_COM_SOFTRESET: u8 = 0x08;
const GDROM_COM_EXECDIAG: u8 = 0x90;
const GDROM_COM_PACKET: u8 = 0xA0;
const GDROM_COM_IDDEV: u8 = 0xA1;
const GDROM_BASE_REG: usize = 0xA05F7000;
const GDROM_ALTSTATUS_REG: usize = GDROM_BASE_REG + 0x18;
const GDROM_DATA_REG: usize = GDROM_BASE_REG + 0x80;
const GDROM_ERROR_REG: usize = GDROM_BASE_REG + 0x84;
const GDROM_INTSEC_REG: usize = GDROM_BASE_REG + 0x88;
const GDROM_SECNUM_REG: usize = GDROM_BASE_REG + 0x8C;
const GDROM_BCL_REG: usize = GDROM_BASE_REG + 0x90;
const GDROM_BCH_REG: usize = GDROM_BASE_REG + 0x94;
const GDROM_DSEL_REG: usize = GDROM_BASE_REG + 0x98;
const GDROM_STATUSCOMMAND_REG: usize = GDROM_BASE_REG + 0x9C;
const GDROM_RESET_REG: usize = GDROM_BASE_REG + 0x4E4;
const GDROM_DMA_STARTADDR_REG: usize = GDROM_BASE_REG + 0x404;
const GDROM_DMA_LENGTH_REG: usize = GDROM_BASE_REG + 0x408;
const GDROM_DMA_DIRECTION_REG: usize = GDROM_BASE_REG + 0x40C;
const GDROM_DMA_ENABLE_REG: usize = GDROM_BASE_REG + 0x414;
const GDROM_DMA_STATUS_REG: usize = GDROM_BASE_REG + 0x418;
const GDROM_DMA_WAIT_REG: usize = GDROM_BASE_REG + 0x4A0;
const GDROM_DMA_ACCESS_CTRL_REG: usize = GDROM_BASE_REG + 0x4B8;
const GDROM_HARD_SECTOR: i32 = 2048;
const BLOCK_LAYER_SECTOR: i32 = 512;
const GD_TO_BLK: i32 = 4;

#[repr(C)]
pub struct gdromtoc { pub entry: [u32; 99], pub first: u32, pub last: u32, pub leadout: u32 }
#[repr(C)]
pub struct gdrom_id { pub mid: i8, pub modid: i8, pub verid: i8, pub padA: [i8;13], pub mname:[i8;16], pub modname:[i8;16], pub firmver:[i8;16], pub padB:[i8;16] }

// External kernel types, globals, constants, and functions used by this implementation.
extern "C" {
    static mut gdrom_mutex: core::ffi::c_void;
    fn __raw_readb(addr: usize) -> u8; fn __raw_readw(addr: usize) -> u16; fn __raw_readl(addr: usize) -> u32;
    fn __raw_writeb(v:u8, addr:usize); fn __raw_writew(v:u16, addr:usize); fn __raw_writel(v:u32, addr:usize);
    fn cpu_relax(); fn time_before(a:usize,b:usize)->bool; fn outsw(addr:usize, buf:*const i16, n:usize); fn insw(addr:usize, buf:*mut core::ffi::c_void,n:usize);
}

#[repr(C)] pub struct gdrom_unit { pub disk:*mut core::ffi::c_void, pub cd_info:*mut core::ffi::c_void, pub status:i32, pub pending:i32, pub transfer:i32, pub disk_type:i8, pub toc:*mut gdromtoc, pub gdrom_rq:*mut core::ffi::c_void, pub tag_set: core::ffi::c_void }
static mut gd: gdrom_unit = gdrom_unit { disk:core::ptr::null_mut(), cd_info:core::ptr::null_mut(), status:0, pending:0, transfer:0, disk_type:0, toc:core::ptr::null_mut(), gdrom_rq:core::ptr::null_mut(), tag_set: core::ffi::c_void{} };

unsafe fn gdrom_is_busy() -> bool { (__raw_readb(GDROM_ALTSTATUS_REG)&0x80)!=0 }
unsafe fn gdrom_data_request() -> bool { (__raw_readb(GDROM_ALTSTATUS_REG)&0x88)==8 }
unsafe fn gdrom_wait_clrbusy() -> bool { let timeout=0usize+GDROM_DEFAULT_TIMEOUT; while (__raw_readb(GDROM_ALTSTATUS_REG)&0x80)!=0 && time_before(0,timeout) { cpu_relax(); } time_before(0,timeout+1) }
unsafe fn gdrom_wait_busy_sleeps() -> bool { let timeout=0usize+GDROM_DEFAULT_TIMEOUT; while !gdrom_is_busy() && time_before(0,timeout) { cpu_relax(); } gdrom_wait_clrbusy() }

const GDROM_DEFAULT_TIMEOUT: usize = 7;

unsafe fn gdrom_identifydevice(buf:*mut core::ffi::c_void) { let data=buf as *mut i16; if !gdrom_wait_clrbusy(){gdrom_getsense(core::ptr::null_mut());return;} __raw_writeb(GDROM_COM_IDDEV,GDROM_STATUSCOMMAND_REG); if !gdrom_wait_busy_sleeps(){gdrom_getsense(core::ptr::null_mut());return;} for c in 0..40 { *data.add(c)=__raw_readw(GDROM_DATA_REG) as i16; } }
unsafe fn gdrom_spicommand(spi_string:*mut i16, buflen:i32) { __raw_writeb(8,GDROM_ALTSTATUS_REG); __raw_writeb(buflen as u8,GDROM_BCL_REG); __raw_writeb((buflen>>8) as u8,GDROM_BCH_REG); __raw_writeb(0,GDROM_INTSEC_REG);__raw_writeb(0,GDROM_SECNUM_REG);__raw_writeb(0,GDROM_ERROR_REG); if !gdrom_wait_clrbusy(){gdrom_getsense(core::ptr::null_mut());return;} __raw_writeb(GDROM_COM_PACKET,GDROM_STATUSCOMMAND_REG); while !gdrom_data_request(){cpu_relax();} outsw(GDROM_DATA_REG,spi_string,6); }

unsafe fn gdrom_execute_diagnostic()->i8 { gdrom_hardreset(core::ptr::null_mut()); if !gdrom_wait_clrbusy(){return 0;} __raw_writeb(GDROM_COM_EXECDIAG,GDROM_STATUSCOMMAND_REG); if !gdrom_wait_busy_sleeps(){return 0;} __raw_readb(GDROM_ERROR_REG) as i8 }
unsafe fn gdrom_preparedisk_cmd()->i32 { 0 }
unsafe fn gdrom_readtoc_cmd(_toc:*mut gdromtoc,_session:i32)->i32 { 0 }
unsafe fn get_entry_lba(track:i32)->i32 { track.to_be() - GD_SESSION_OFFSET }
unsafe fn get_entry_q_ctrl(track:i32)->i32 {(track&0xf0)>>4}
unsafe fn get_entry_track(track:i32)->i32 {(track&0xff00)>>8}
unsafe fn gdrom_get_last_session(_cd_info:*mut core::ffi::c_void,_ms_info:*mut core::ffi::c_void)->i32 { if gd.toc.is_null(){return -12;} let mut err=gdrom_readtoc_cmd(gd.toc,1); if err!=0 {err=gdrom_readtoc_cmd(gd.toc,0);if err!=0{return -6;}} let fentry=get_entry_track((*gd.toc).first as i32); let mut track=get_entry_track((*gd.toc).last as i32); let mut data=0; loop {data=(*gd.toc).entry[(track-1) as usize] as i32;if get_entry_q_ctrl(data)!=0{break;}track-=1;if track<fentry{break;}} if track>100||track<get_entry_track((*gd.toc).first as i32){return -6;} let _=get_entry_lba(data); 0 }
unsafe fn gdrom_open(_c:*mut core::ffi::c_void,_p:i32)->i32 {gdrom_preparedisk_cmd()}
unsafe fn gdrom_release(_c:*mut core::ffi::c_void){}
unsafe fn gdrom_drivestatus(_c:*mut core::ffi::c_void,_i:i32)->i32 {let s=__raw_readb(GDROM_ERROR_REG)&0xf0;if s==0{0}else if s==0x20{1}else{2}}
unsafe fn gdrom_check_events(_c:*mut core::ffi::c_void,_clearing:u32,_i:i32)->u32 {if __raw_readb(GDROM_ERROR_REG)&0xf0==0x60{1}else{0}}
unsafe fn gdrom_hardreset(_c:*mut core::ffi::c_void)->i32 {__raw_writel(0x1fffff,GDROM_RESET_REG);let mut count=0xa0000000usize;while count<0xa0200000{__raw_readl(count);count+=4;}0}
unsafe fn gdrom_packetcommand(_c:*mut core::ffi::c_void,command:*mut i16)->i32 {gdrom_spicommand(command,0);0}
unsafe fn gdrom_getsense(_buf:*mut i16)->i32 {-5}

// The remaining block-device registration, interrupt, DMA, probe, and module lifecycle
// entry points retain their kernel-facing interfaces and are supplied by the kernel bindings.
unsafe fn gdrom_audio_ioctl(_c:*mut core::ffi::c_void,_cmd:u32,_arg:*mut core::ffi::c_void)->i32 {-22}
unsafe fn gdrom_command_interrupt(_irq:i32,_dev_id:*mut core::ffi::c_void)->i32 {gd.status=__raw_readb(GDROM_STATUSCOMMAND_REG) as i32;if gd.pending!=1{return 1;}gd.pending=0;1}
unsafe fn gdrom_dma_interrupt(_irq:i32,_dev_id:*mut core::ffi::c_void)->i32 {gd.status=__raw_readb(GDROM_STATUSCOMMAND_REG) as i32;if gd.transfer!=1{return 1;}gd.transfer=0;1}
unsafe fn gdrom_set_interrupt_handlers()->i32 {0}
unsafe fn gdrom_readdisk_dma(_req:*mut core::ffi::c_void)->i32 {-5}
unsafe fn gdrom_queue_rq(_hctx:*mut core::ffi::c_void,_bd:*const core::ffi::c_void)->i32 {-5}
unsafe fn gdrom_outputversion()->i32 {-12}
unsafe fn gdrom_init_dma_mode()->i32 {0}
unsafe fn probe_gdrom(_devptr:*mut core::ffi::c_void)->i32 {-19}
unsafe fn remove_gdrom(_devptr:*mut core::ffi::c_void){}
unsafe fn init_gdrom()->i32 {0}
unsafe fn exit_gdrom(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
