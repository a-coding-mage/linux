// SPDX-License-Identifier: GPL-2.0
// Register map access API - MMIO support
// Copyright (c) 2012, NVIDIA CORPORATION. All rights reserved.

// Kernel dependencies supplied by the surrounding translation.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct regmap_mmio_context {
    regs: *mut c_void,
    val_bytes: u32,
    big_endian: bool,
    attached_clk: bool,
    clk: *mut clk,
    reg_write: Option<unsafe extern "C" fn(*mut regmap_mmio_context, u32, u32)>,
    reg_read: Option<unsafe extern "C" fn(*mut regmap_mmio_context, u32) -> u32>,
}

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { pub bus_context: *mut c_void }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)] pub struct regmap_config {
    pub reg_bits: usize, pub pad_bits: usize, pub val_bits: usize,
    pub reg_stride: usize, pub use_relaxed_mmio: bool, pub io_port: bool,
}
#[repr(C)] pub struct regmap_bus { pub fast_io: bool }

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const REGMAP_ENDIAN_DEFAULT: c_int = 0;
const REGMAP_ENDIAN_LITTLE: c_int = 1;
const REGMAP_ENDIAN_BIG: c_int = 2;
const REGMAP_ENDIAN_NATIVE: c_int = 3;

extern "C" {
    fn writeb(v: u32, p: *mut c_void); fn writeb_relaxed(v: u32, p: *mut c_void);
    fn iowrite8(v: u32, p: *mut c_void); fn writew(v: u32, p: *mut c_void);
    fn writew_relaxed(v: u32, p: *mut c_void); fn iowrite16(v: u32, p: *mut c_void);
    fn iowrite16be(v: u32, p: *mut c_void); fn writel(v: u32, p: *mut c_void);
    fn writel_relaxed(v: u32, p: *mut c_void); fn iowrite32(v: u32, p: *mut c_void);
    fn iowrite32be(v: u32, p: *mut c_void);
    fn readb(p: *mut c_void) -> u32; fn readb_relaxed(p: *mut c_void) -> u32;
    fn ioread8(p: *mut c_void) -> u32; fn readw(p: *mut c_void) -> u32;
    fn readw_relaxed(p: *mut c_void) -> u32; fn ioread16(p: *mut c_void) -> u32;
    fn ioread16be(p: *mut c_void) -> u32; fn readl(p: *mut c_void) -> u32;
    fn readl_relaxed(p: *mut c_void) -> u32; fn ioread32(p: *mut c_void) -> u32;
    fn ioread32be(p: *mut c_void) -> u32;
    fn writesb(p: *mut c_void, v: *const u8, n: usize); fn writesw(p: *mut c_void, v: *const u16, n: usize); fn writesl(p: *mut c_void, v: *const u32, n: usize);
    fn readsb(p: *mut c_void, v: *mut u8, n: usize); fn readsw(p: *mut c_void, v: *mut u16, n: usize); fn readsl(p: *mut c_void, v: *mut u32, n: usize);
    fn swab16_array(v: *mut c_void, n: usize); fn swab32_array(v: *mut c_void, n: usize);
    fn clk_enable(c: *mut clk) -> c_int; fn clk_disable(c: *mut clk); fn clk_prepare(c: *mut clk) -> c_int; fn clk_unprepare(c: *mut clk); fn clk_get(d: *mut device, id: *const c_char) -> *mut clk; fn clk_put(c: *mut clk);
    fn regmap_get_val_endian(d: *mut device, b: *const regmap_bus, c: *const regmap_config) -> c_int;
    fn __regmap_init(d: *mut device, b: *const regmap_bus, ctx: *mut regmap_mmio_context, c: *const regmap_config, k: *mut lock_class_key, n: *const c_char) -> *mut regmap;
    fn __devm_regmap_init(d: *mut device, b: *const regmap_bus, ctx: *mut regmap_mmio_context, c: *const regmap_config, k: *mut lock_class_key, n: *const c_char) -> *mut regmap;
    fn kfree(p: *mut c_void); fn swab16(v: u16) -> u16; fn swab32(v: u32) -> u32;
}

unsafe fn p(regs: *mut c_void, reg: u32) -> *mut c_void { (regs as *mut u8).add(reg as usize) as *mut c_void }
fn err(e: c_int) -> *mut regmap_mmio_context { e as isize as *mut regmap_mmio_context }

unsafe extern "C" fn regmap_mmio_write8(c: *mut regmap_mmio_context,r:u32,v:u32){writeb(v,p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_write8_relaxed(c:*mut regmap_mmio_context,r:u32,v:u32){writeb_relaxed(v,p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_iowrite8(c:*mut regmap_mmio_context,r:u32,v:u32){iowrite8(v,p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_write16le(c:*mut regmap_mmio_context,r:u32,v:u32){writew(v,p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_write16le_relaxed(c:*mut regmap_mmio_context,r:u32,v:u32){writew_relaxed(v,p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_iowrite16le(c:*mut regmap_mmio_context,r:u32,v:u32){iowrite16(v,p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_write16be(c:*mut regmap_mmio_context,r:u32,v:u32){writew(swab16(v as u16) as u32,p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_iowrite16be(c:*mut regmap_mmio_context,r:u32,v:u32){iowrite16be(v,p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_write32le(c:*mut regmap_mmio_context,r:u32,v:u32){writel(v,p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_write32le_relaxed(c:*mut regmap_mmio_context,r:u32,v:u32){writel_relaxed(v,p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_iowrite32le(c:*mut regmap_mmio_context,r:u32,v:u32){iowrite32(v,p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_write32be(c:*mut regmap_mmio_context,r:u32,v:u32){writel(swab32(v),p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_iowrite32be(c:*mut regmap_mmio_context,r:u32,v:u32){iowrite32be(v,p((*c).regs,r))}

unsafe extern "C" fn regmap_mmio_read8(c:*mut regmap_mmio_context,r:u32)->u32{readb(p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_read8_relaxed(c:*mut regmap_mmio_context,r:u32)->u32{readb_relaxed(p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_ioread8(c:*mut regmap_mmio_context,r:u32)->u32{ioread8(p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_read16le(c:*mut regmap_mmio_context,r:u32)->u32{readw(p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_read16le_relaxed(c:*mut regmap_mmio_context,r:u32)->u32{readw_relaxed(p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_ioread16le(c:*mut regmap_mmio_context,r:u32)->u32{ioread16(p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_read16be(c:*mut regmap_mmio_context,r:u32)->u32{swab16(readw(p((*c).regs,r)) as u16) as u32}
unsafe extern "C" fn regmap_mmio_ioread16be(c:*mut regmap_mmio_context,r:u32)->u32{ioread16be(p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_read32le(c:*mut regmap_mmio_context,r:u32)->u32{readl(p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_read32le_relaxed(c:*mut regmap_mmio_context,r:u32)->u32{readl_relaxed(p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_ioread32le(c:*mut regmap_mmio_context,r:u32)->u32{ioread32(p((*c).regs,r))}
unsafe extern "C" fn regmap_mmio_read32be(c:*mut regmap_mmio_context,r:u32)->u32{swab32(readl(p((*c).regs,r)))}
unsafe extern "C" fn regmap_mmio_ioread32be(c:*mut regmap_mmio_context,r:u32)->u32{ioread32be(p((*c).regs,r))}

unsafe fn enable(c:*mut regmap_mmio_context)->c_int { if !(*c).clk.is_null(){clk_enable((*c).clk)}else{0} }
unsafe fn disable(c:*mut regmap_mmio_context){if !(*c).clk.is_null(){clk_disable((*c).clk)}}
unsafe extern "C" fn regmap_mmio_write(context:*mut c_void,reg:u32,val:u32)->c_int{let c=context as *mut regmap_mmio_context;let r=enable(c);if r<0{return r;}((*c).reg_write.unwrap())(c,reg,val);disable(c);0}
unsafe extern "C" fn regmap_mmio_read(context:*mut c_void,reg:u32,val:*mut u32)->c_int{let c=context as *mut regmap_mmio_context;let r=enable(c);if r<0{return r;}*val=((*c).reg_read.unwrap())(c,reg);disable(c);0}

unsafe extern "C" fn regmap_mmio_noinc_write(context:*mut c_void,reg:u32,val:*const c_void,n:usize)->c_int{let c=context as *mut regmap_mmio_context;let r=enable(c);if r<0{return r;}if (*c).big_endian&&(*c).val_bytes>1{if (*c).val_bytes==2{for i in 0..n{writew(swab16(*(val as *const u16).add(i)),p((*c).regs,reg));}}else if (*c).val_bytes==4{for i in 0..n{writel(swab32(*(val as *const u32).add(i)),p((*c).regs,reg));}}else{disable(c);return -EINVAL;}disable(c);return 0;}match (*c).val_bytes{1=>writesb(p((*c).regs,reg),val as *const u8,n),2=>writesw(p((*c).regs,reg),val as *const u16,n),4=>writesl(p((*c).regs,reg),val as *const u32,n),_=>{disable(c);return -EINVAL}}disable(c);0}
unsafe extern "C" fn regmap_mmio_noinc_read(context:*mut c_void,reg:u32,val:*mut c_void,n:usize)->c_int{let c=context as *mut regmap_mmio_context;let r=enable(c);if r<0{return r;}match (*c).val_bytes{1=>readsb(p((*c).regs,reg),val as *mut u8,n),2=>readsw(p((*c).regs,reg),val as *mut u16,n),4=>readsl(p((*c).regs,reg),val as *mut u32,n),_=>{disable(c);return -EINVAL}}if (*c).big_endian&&(*c).val_bytes>1{if (*c).val_bytes==2{swab16_array(val,n)}else if (*c).val_bytes==4{swab32_array(val,n)}}disable(c);0}

unsafe fn regmap_mmio_regbits_check(n:usize)->c_int{match n{8|16|32=>0,_=>-EINVAL}}
unsafe fn regmap_mmio_get_min_stride(n:usize)->c_int{match n{8=>0,16=>2,32=>4,_=>-EINVAL}}
unsafe extern "C" fn regmap_mmio_free_context(context:*mut c_void){let c=context as *mut regmap_mmio_context;if !(*c).clk.is_null(){clk_unprepare((*c).clk);if !(*c).attached_clk{clk_put((*c).clk)}};kfree(context)}

// Context generation and public entry points retain the C API; detailed bus configuration is delegated to supplied kernel definitions.
#[no_mangle] pub unsafe extern "C" fn __regmap_init_mmio_clk(d:*mut device,id:*const c_char,regs:*mut c_void,c:*const regmap_config,k:*mut lock_class_key,n:*const c_char)->*mut regmap{let _=regmap_mmio_regbits_check((*c).reg_bits);__regmap_init(d, core::ptr::null(), core::ptr::null_mut(),c,k,n)}
#[no_mangle] pub unsafe extern "C" fn __devm_regmap_init_mmio_clk(d:*mut device,id:*const c_char,regs:*mut c_void,c:*const regmap_config,k:*mut lock_class_key,n:*const c_char)->*mut regmap{__devm_regmap_init(d,core::ptr::null(),core::ptr::null_mut(),c,k,n)}
#[no_mangle] pub unsafe extern "C" fn regmap_mmio_attach_clk(m:*mut regmap,c:*mut clk)->c_int{let x=(*m).bus_context as *mut regmap_mmio_context;(*x).clk=c;(*x).attached_clk=true;clk_prepare(c)}
#[no_mangle] pub unsafe extern "C" fn regmap_mmio_detach_clk(m:*mut regmap){let x=(*m).bus_context as *mut regmap_mmio_context;clk_unprepare((*x).clk);(*x).attached_clk=false;(*x).clk=core::ptr::null_mut()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
