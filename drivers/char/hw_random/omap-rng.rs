/*
 * omap-rng.c - RNG driver for TI OMAP CPU family
 *
 * Author: Deepak Saxena <dsaxena@plexity.net>
 * Copyright 2005 (c) MontaVista Software, Inc.
 * Mostly based on original driver by Nokia Corporation.
 * Licensed under the GNU General Public License version 2.
 */

use core::{ffi::c_void, mem::size_of, ptr};

const RNG_REG_STATUS_RDY: u32 = 1 << 0;
const RNG_REG_INTACK_RDY_MASK: u32 = 1 << 0;
const RNG_REG_INTACK_SHUTDOWN_OFLO_MASK: u32 = 1 << 1;
const RNG_SHUTDOWN_OFLO_MASK: u32 = 1 << 1;
const RNG_CONTROL_STARTUP_CYCLES_SHIFT: u32 = 16;
const RNG_CONTROL_STARTUP_CYCLES_MASK: u32 = 0xffff << 16;
const RNG_CONTROL_ENABLE_TRNG_SHIFT: u32 = 10;
const RNG_CONTROL_ENABLE_TRNG_MASK: u32 = 1 << 10;
const RNG_CONFIG_MAX_REFIL_CYCLES_SHIFT: u32 = 16;
const RNG_CONFIG_MAX_REFIL_CYCLES_MASK: u32 = 0xffff << 16;
const RNG_CONFIG_MIN_REFIL_CYCLES_SHIFT: u32 = 0;
const RNG_CONFIG_MIN_REFIL_CYCLES_MASK: u32 = 0xff;
const RNG_CONTROL_STARTUP_CYCLES: u32 = 0xff;
const RNG_CONFIG_MIN_REFIL_CYCLES: u32 = 0x21;
const RNG_CONFIG_MAX_REFIL_CYCLES: u32 = 0x22;
const RNG_ALARMCNT_ALARM_TH_SHIFT: u32 = 0;
const RNG_ALARMCNT_ALARM_TH_MASK: u32 = 0xff;
const RNG_ALARMCNT_SHUTDOWN_TH_SHIFT: u32 = 16;
const RNG_ALARMCNT_SHUTDOWN_TH_MASK: u32 = 0x1f << 16;
const RNG_ALARM_THRESHOLD: u32 = 0xff;
const RNG_SHUTDOWN_THRESHOLD: u32 = 0x4;
const RNG_REG_FROENABLE_MASK: u32 = 0xffffff;
const RNG_REG_FRODETUNE_MASK: u32 = 0xffffff;
const OMAP2_RNG_OUTPUT_SIZE: u32 = 0x4;
const OMAP4_RNG_OUTPUT_SIZE: u32 = 0x8;
const EIP76_RNG_OUTPUT_SIZE: u32 = 0x10;
const RNG_DATA_FILL_TIMEOUT: i32 = 100;

#[repr(usize)]
#[derive(Copy, Clone)]
enum RngReg { Output0 = 0, Output1, Output2, Output3, Status, Intmask, Intack,
    Control, Config, Alarmcnt, Froenable, Frodetune, Alarmmask, Alarmstop, Rev, Sysconfig }

static REG_MAP_OMAP2: [u16; 16] = [0x0,0,0,0,0x4,0,0,0,0x28,0,0,0,0,0,0x3c,0x40];
static REG_MAP_OMAP4: [u16; 16] = [0x0,0x4,0,0,0x8,0xc,0x10,0x14,0x18,0x1c,0x20,0x24,0x28,0x2c,0x1FE0,0x1FE4];
static REG_MAP_EIP76: [u16; 16] = [0x0,0x4,0x8,0xc,0x10,0,0x10,0x14,0x18,0x1c,0x20,0x24,0x28,0x2c,0x7c,0];

#[repr(C)] pub struct Device;
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct Clk;
#[repr(C)] pub struct Hwrng { pub priv_: usize, pub read: Option<unsafe extern "C" fn(*mut Hwrng,*mut c_void,usize,bool)->i32>, pub init: Option<unsafe extern "C" fn(*mut Hwrng)->i32>, pub cleanup: Option<unsafe extern "C" fn(*mut Hwrng)>, pub quality: u32, pub name: *const i8 }
#[repr(C)] pub struct OmapRngPdata { pub regs: *const u16, pub data_size: u32, pub data_present: unsafe extern "C" fn(*mut OmapRngDev)->u32, pub init: unsafe extern "C" fn(*mut OmapRngDev)->i32, pub cleanup: unsafe extern "C" fn(*mut OmapRngDev) }
#[repr(C)] pub struct OmapRngDev { pub base: *mut u8, pub dev: *mut Device, pub pdata: *const OmapRngPdata, pub rng: Hwrng, pub clk: *mut Clk, pub clk_reg: *mut Clk }

extern "C" {
    fn __raw_readl(p: *const u8) -> u32; fn __raw_writel(v:u32,p:*mut u8);
    fn udelay(v:u32); fn memcpy_fromio(dst:*mut c_void,src:*const u8,n:usize);
}
unsafe fn omap_rng_read(p:&mut OmapRngDev, r:usize)->u32 { __raw_readl(p.base.add((*p.pdata).regs.add(r).read() as usize)) }
unsafe fn omap_rng_write(p:&mut OmapRngDev,r:usize,v:u32){ __raw_writel(v,p.base.add((*p.pdata).regs.add(r).read() as usize)); }

#[no_mangle] pub unsafe extern "C" fn omap_rng_do_read(rng:*mut Hwrng,data:*mut c_void,max:usize,wait:bool)->i32 {
    let p=&mut *((*rng).priv_ as *mut OmapRngDev); if max < (*p.pdata).data_size as usize{return 0;}
    let mut present=0; let mut i=0; while i<RNG_DATA_FILL_TIMEOUT {present=((*p.pdata).data_present)(p);if present!=0||!wait{break;}udelay(10);i+=1;}
    if present==0{return 0;} memcpy_fromio(data,p.base.add((*p.pdata).regs.add(RngReg::Output0 as usize).read() as usize),(*p.pdata).data_size as usize);
    if (*p.pdata).regs.add(RngReg::Intack as usize).read()!=0{omap_rng_write(p,RngReg::Intack as usize,RNG_REG_INTACK_RDY_MASK);} (*p.pdata).data_size as i32
}
unsafe extern "C" fn omap_rng_init(r:*mut Hwrng)->i32{let p=&mut *((*r).priv_ as *mut OmapRngDev);((*p.pdata).init)(p)}
unsafe extern "C" fn omap_rng_cleanup(r:*mut Hwrng){let p=&mut *((*r).priv_ as *mut OmapRngDev);((*p.pdata).cleanup)(p)}
unsafe extern "C" fn omap2_rng_data_present(p:*mut OmapRngDev)->u32{if omap_rng_read(&mut *p,RngReg::Status as usize)!=0{0}else{1}}
unsafe extern "C" fn omap2_rng_init(p:*mut OmapRngDev)->i32{omap_rng_write(&mut *p,RngReg::Sysconfig as usize,1);0}
unsafe extern "C" fn omap2_rng_cleanup(p:*mut OmapRngDev){omap_rng_write(&mut *p,RngReg::Sysconfig as usize,0)}
unsafe extern "C" fn omap4_rng_data_present(p:*mut OmapRngDev)->u32{omap_rng_read(&mut *p,RngReg::Status as usize)&RNG_REG_STATUS_RDY}
unsafe extern "C" fn eip76_rng_init(p:*mut OmapRngDev)->i32{let p=&mut *p;if omap_rng_read(p,RngReg::Control as usize)&RNG_CONTROL_ENABLE_TRNG_MASK!=0{return 0;}let mut v=0x5<<RNG_CONFIG_MIN_REFIL_CYCLES_SHIFT;v|=RNG_CONFIG_MAX_REFIL_CYCLES<<RNG_CONFIG_MAX_REFIL_CYCLES_SHIFT;omap_rng_write(p,RngReg::Config as usize,v);omap_rng_write(p,RngReg::Frodetune as usize,0);omap_rng_write(p,RngReg::Froenable as usize,RNG_REG_FROENABLE_MASK);omap_rng_write(p,RngReg::Control as usize,RNG_CONTROL_ENABLE_TRNG_MASK);0}
unsafe extern "C" fn omap4_rng_init(p:*mut OmapRngDev)->i32{let p=&mut *p;if omap_rng_read(p,RngReg::Control as usize)&RNG_CONTROL_ENABLE_TRNG_MASK!=0{return 0;}let mut v=RNG_CONFIG_MIN_REFIL_CYCLES<<RNG_CONFIG_MIN_REFIL_CYCLES_SHIFT|RNG_CONFIG_MAX_REFIL_CYCLES<<RNG_CONFIG_MAX_REFIL_CYCLES_SHIFT;omap_rng_write(p,RngReg::Config as usize,v);omap_rng_write(p,RngReg::Frodetune as usize,0);omap_rng_write(p,RngReg::Froenable as usize,RNG_REG_FROENABLE_MASK);v=RNG_ALARM_THRESHOLD|RNG_SHUTDOWN_THRESHOLD<<RNG_ALARMCNT_SHUTDOWN_TH_SHIFT;omap_rng_write(p,RngReg::Alarmcnt as usize,v);v=RNG_CONTROL_STARTUP_CYCLES<<RNG_CONTROL_STARTUP_CYCLES_SHIFT|RNG_CONTROL_ENABLE_TRNG_MASK;omap_rng_write(p,RngReg::Control as usize,v);0}
unsafe extern "C" fn omap4_rng_cleanup(p:*mut OmapRngDev){let p=&mut *p;let mut v=omap_rng_read(p,RngReg::Control as usize);v&=!RNG_CONTROL_ENABLE_TRNG_MASK;omap_rng_write(p,RngReg::Control as usize,v)}
unsafe extern "C" fn omap4_rng_irq(_irq:i32,dev_id:*mut c_void)->i32{let p=&mut *(dev_id as *mut OmapRngDev);omap_rng_write(p,RngReg::Alarmmask as usize,0);omap_rng_write(p,RngReg::Alarmstop as usize,0);let mut e=omap_rng_read(p,RngReg::Froenable as usize);let mut d=!e&RNG_REG_FRODETUNE_MASK;d|=omap_rng_read(p,RngReg::Frodetune as usize);e=RNG_REG_FROENABLE_MASK;omap_rng_write(p,RngReg::Frodetune as usize,d);omap_rng_write(p,RngReg::Froenable as usize,e);omap_rng_write(p,RngReg::Intack as usize,RNG_REG_INTACK_SHUTDOWN_OFLO_MASK);1}

static mut OMAP2_RNG_PDATA: OmapRngPdata=OmapRngPdata{regs:REG_MAP_OMAP2.as_ptr(),data_size:OMAP2_RNG_OUTPUT_SIZE,data_present:omap2_rng_data_present,init:omap2_rng_init,cleanup:omap2_rng_cleanup};
static mut OMAP4_RNG_PDATA: OmapRngPdata=OmapRngPdata{regs:REG_MAP_OMAP4.as_ptr(),data_size:OMAP4_RNG_OUTPUT_SIZE,data_present:omap4_rng_data_present,init:omap4_rng_init,cleanup:omap4_rng_cleanup};
static mut EIP76_RNG_PDATA: OmapRngPdata=OmapRngPdata{regs:REG_MAP_EIP76.as_ptr(),data_size:EIP76_RNG_OUTPUT_SIZE,data_present:omap4_rng_data_present,init:eip76_rng_init,cleanup:omap4_rng_cleanup};

/* Remaining platform-device, device-tree, power-management, clock, IRQ, and
 * module registration interfaces are external kernel dependencies. */
extern "C" { fn omap_rng_probe(p:*mut PlatformDevice)->i32; fn omap_rng_remove(p:*mut PlatformDevice); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
