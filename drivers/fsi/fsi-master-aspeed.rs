// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) IBM Corporation 2018
// FSI master driver for AST2600
//
// Linux kernel dependencies and symbols supplied by other translation units
// are intentionally referenced as external Rust items.

use core::ffi::c_void;

#[repr(C)]
pub struct fsi_master_aspeed {
    pub master: fsi_master,
    pub lock: mutex,
    pub dev: *mut device,
    pub base: *mut c_void,
    pub clk: *mut clk,
    pub cfam_reset_gpio: *mut gpio_desc,
}

#[repr(C)] pub struct fsi_master { pub dev: device, pub n_links: i32, pub read: Option<unsafe extern "C" fn(*mut fsi_master,i32,u8,u32,*mut c_void,usize)->i32>, pub write: Option<unsafe extern "C" fn(*mut fsi_master,i32,u8,u32,*const c_void,usize)->i32>, pub send_break: Option<unsafe extern "C" fn(*mut fsi_master,i32)->i32>, pub term: Option<unsafe extern "C" fn(*mut fsi_master,i32,u8)->i32>, pub link_enable: Option<unsafe extern "C" fn(*mut fsi_master,i32,bool)->i32> }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct device { pub parent:*mut device, pub release:Option<unsafe extern "C" fn(*mut device)>, pub of_node:*mut c_void }
#[repr(C)] pub struct clk { _private:[u8;0] }
#[repr(C)] pub struct gpio_desc { _private:[u8;0] }
#[repr(C)] pub struct platform_device { _private:[u8;0] }

const CTRL_BASE:u32=0x80000000; const FSI_BASE:u32=0xa0000000;
const OPB_TRIGGER:u32=0x04; const OPB_CTRL_BASE:u32=0x08; const OPB_FSI_BASE:u32=0x0c; const OPB_CLK_SYNC:u32=0x3c; const OPB_IRQ_CLEAR:u32=0x40; const OPB_IRQ_MASK:u32=0x44; const OPB_IRQ_STATUS:u32=0x48;
const OPB0_SELECT:u32=0x10; const OPB0_RW:u32=0x14; const OPB0_XFER_SIZE:u32=0x18; const OPB0_FSI_ADDR:u32=0x1c; const OPB0_FSI_DATA_W:u32=0x20; const OPB0_STATUS:u32=0x80; const OPB0_FSI_DATA_R:u32=0x84;
const OPB0_WRITE_ORDER1:u32=0x4c; const OPB0_WRITE_ORDER2:u32=0x50; const OPB0_READ_ORDER1:u32=0x5c; const OPB_RETRY_COUNTER:u32=0x64;
const STATUS_ERR_ACK:u32=1<<2; const OPB1_XFER_ACK_EN:u32=1<<17; const OPB0_XFER_ACK_EN:u32=1<<16; const CMD_READ:u32=1; const CMD_WRITE:u32=0; const XFER_FULLWORD:u32=3; const XFER_HALFWORD:u32=1; const XFER_BYTE:u32=0;
const FSI_LINK_ENABLE_SETUP_TIME:u32=10; const FSI_DIVISOR_DEFAULT:u16=1; const FSI_DIVISOR_CABLED:u16=2; const OPB_POLL_TIMEOUT:u32=500;
static mut aspeed_fsi_divisor:u16=FSI_DIVISOR_DEFAULT;

extern "C" { fn writel(v:u32,p:*mut c_void); fn writel_relaxed(v:u32,p:*mut c_void); fn readl(p:*mut c_void)->u32; fn readl_poll_timeout(p:*mut c_void,v:*mut u32,cond:bool,delay:u32,timeout:u32)->i32; fn mdelay(v:u32); fn usleep_range(a:u32,b:u32); fn mutex_lock(m:*mut mutex); fn mutex_unlock(m:*mut mutex); fn opb_trace_write(a:u32,v:u32,s:u32,st:u32,r:u32); fn opb_trace_read(a:u32,s:u32,v:u32,st:u32,r:u32); fn opb_trace_error_enabled()->bool; fn trace_error(a:u32,b:u32,c:u32); fn be32_to_cpu(v:u32)->u32; fn cpu_to_be32(v:u32)->u32; }

unsafe fn __opb_write(a:&mut fsi_master_aspeed, addr:u32,val:u32,size:u32)->i32 { let b=a.base; let mut reg=0; writel_relaxed(CMD_WRITE,b.add(OPB0_RW as usize)); writel_relaxed(size,b.add(OPB0_XFER_SIZE as usize)); writel_relaxed(addr,b.add(OPB0_FSI_ADDR as usize)); writel_relaxed(val,b.add(OPB0_FSI_DATA_W as usize)); writel_relaxed(1,b.add(OPB_IRQ_CLEAR as usize)); writel(1,b.add(OPB_TRIGGER as usize)); let ret=readl_poll_timeout(b.add(OPB_IRQ_STATUS as usize),&mut reg,(readl(b.add(OPB_IRQ_STATUS as usize))&OPB0_XFER_ACK_EN)!=0,0,OPB_POLL_TIMEOUT); let status=readl(b.add(OPB0_STATUS as usize)); opb_trace_write(addr,val,size,status,reg); if ret!=0{return ret} if status&STATUS_ERR_ACK!=0{return -5} 0 }
unsafe fn opb_writel(a:&mut fsi_master_aspeed,addr:u32,v:u32)->i32{__opb_write(a,addr,v,XFER_FULLWORD)}
unsafe fn __opb_read(a:&mut fsi_master_aspeed,addr:u32,size:u32,out:*mut c_void)->i32{let b=a.base;let mut reg=0;writel_relaxed(CMD_READ,b.add(OPB0_RW as usize));writel_relaxed(size,b.add(OPB0_XFER_SIZE as usize));writel_relaxed(addr,b.add(OPB0_FSI_ADDR as usize));writel_relaxed(1,b.add(OPB_IRQ_CLEAR as usize));writel(1,b.add(OPB_TRIGGER as usize));let ret=readl_poll_timeout(b.add(OPB_IRQ_STATUS as usize),&mut reg,true,0,OPB_POLL_TIMEOUT);let status=readl(b.add(OPB0_STATUS as usize));let result=readl(b.add(OPB0_FSI_DATA_R as usize));opb_trace_read(addr,size,result,readl(b.add(OPB0_STATUS as usize)),reg);if ret!=0{return ret}if status&STATUS_ERR_ACK!=0{return -5}if !out.is_null(){match size{0=>(*out as *mut u8)=result as u8,1=>(*out as *mut u16)=result as u16,3=>(*out as *mut u32)=result,_=>return -22}}0}
unsafe fn opb_readl(a:&mut fsi_master_aspeed,addr:u32,out:*mut u32)->i32{__opb_read(a,addr,3,out as *mut c_void)}

// The remaining driver entry points retain the C ABI and are supplied through the kernel-facing translation layer.
unsafe fn check_errors(a:&mut fsi_master_aspeed,err:i32)->i32{if err==-5{let _=opb_writel(a,CTRL_BASE+0x40,cpu_to_be32(0xffffffff));}err}
unsafe fn aspeed_master_read(m:*mut fsi_master,link:i32,id:u8,addr:u32,val:*mut c_void,size:usize)->i32{if id>3{return -22}let a=&mut *(m as *mut fsi_master_aspeed);let x=addr|((id as u32)<<21)+(link as u32)*0x10000;mutex_lock(&mut a.lock);let r=match size{4=>opb_readl(a,FSI_BASE+x,val as *mut u32),_=>-22};mutex_unlock(&mut a.lock);check_errors(a,r)}
unsafe fn aspeed_master_write(m:*mut fsi_master,link:i32,id:u8,addr:u32,val:*const c_void,size:usize)->i32{if id>3{return -22}let a=&mut *(m as *mut fsi_master_aspeed);let x=addr|((id as u32)<<21)+(link as u32)*0x10000;mutex_lock(&mut a.lock);let r=match size{4=>opb_writel(a,FSI_BASE+x,*(val as *const u32)),_=>-22};mutex_unlock(&mut a.lock);check_errors(a,r)}
unsafe fn aspeed_master_term(m:*mut fsi_master,link:i32,id:u8)->i32{let c=cpu_to_be32(0xecc00000);aspeed_master_write(m,link,id,4,&c as *const _ as *const c_void,4)}
unsafe fn aspeed_master_break(m:*mut fsi_master,link:i32)->i32{let c=cpu_to_be32(0xc0de0000);aspeed_master_write(m,link,0,0,&c as *const _ as *const c_void,4)}
unsafe fn aspeed_master_link_enable(m:*mut fsi_master,link:i32,enable:bool)->i32{let a=&mut *(m as *mut fsi_master_aspeed);mutex_lock(&mut a.lock);let r=opb_writel(a,CTRL_BASE+if enable{0x20}else{0x30}+4*((link/32) as u32),cpu_to_be32(0x80000000>>((link%32) as u32)));if enable&&r==0{mdelay(FSI_LINK_ENABLE_SETUP_TIME)}mutex_unlock(&mut a.lock);r}
unsafe fn aspeed_master_init(a:&mut fsi_master_aspeed)->i32{let r=cpu_to_be32(0xffffffff);let _=opb_writel(a,CTRL_BASE+0x40,r);let _=opb_writel(a,CTRL_BASE+0x40,r);let _=opb_writel(a,CTRL_BASE+0x44,cpu_to_be32(0x3));let _=opb_writel(a,CTRL_BASE+0x48,cpu_to_be32(0xffff0000));let _=opb_writel(a,CTRL_BASE+0x4c,cpu_to_be32(0xffffffff));mdelay(FSI_LINK_ENABLE_SETUP_TIME);let _=opb_writel(a,CTRL_BASE+0x30,r);0}

// Module registration, device probing, GPIO reset handling, tracing, and Linux
// object-management macros are represented by external integration code.
extern "C" { fn fsi_master_aspeed_probe(p:*mut platform_device)->i32; fn fsi_master_aspeed_remove(p:*mut platform_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
