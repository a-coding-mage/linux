// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Linaro Ltd.
// Copyright (C) 2019 Socionext Inc.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Linux kernel dependencies are supplied by the surrounding build.
use core::ffi::c_void;

const M10V_XDACS: usize = 0x00;
const M10V_XDTBC: usize = 0x10;
const M10V_XDSSA: usize = 0x14;
const M10V_XDDSA: usize = 0x18;
const M10V_XDSAC: usize = 0x1C;
const M10V_XDDAC: usize = 0x20;
const M10V_XDDCC: usize = 0x24;
const M10V_XDDES: usize = 0x28;
const M10V_XDDPC: usize = 0x2C;
const M10V_XDDSD: usize = 0x30;

const M10V_XDACS_XE: u32 = 1 << 28;
const M10V_DEFBS: u32 = 0x3;
const M10V_DEFBL: u32 = 0xf;
const M10V_XDSAC_SBS: u32 = 0x3 << 16;
const M10V_XDSAC_SBL: u32 = 0xf << 8;
const M10V_XDDAC_DBS: u32 = 0x3 << 16;
const M10V_XDDAC_DBL: u32 = 0xf << 8;
const M10V_XDDES_CE: u32 = 1 << 28;
const M10V_XDDES_SE: u32 = 1 << 24;
const M10V_XDDES_SA: u32 = 1 << 15;
const M10V_XDDES_TF: u32 = 0xf << 20;
const M10V_XDDES_EI: u32 = 1 << 1;
const M10V_XDDES_TI: u32 = 1;
const M10V_XDDSD_IS_MASK: u32 = 0xf;
const M10V_XDDSD_IS_NORMAL: u32 = 0x8;

const fn field_prep(mask: u32, val: u32) -> u32 { (val << mask.trailing_zeros()) & mask }

#[repr(C)]
pub struct virt_dma_desc { pub node: list_head }
#[repr(C)]
pub struct virt_dma_chan { pub lock: spinlock_t }
#[repr(C)]
pub struct dma_device { pub channels: list_head, pub dev: *mut device, pub cap_mask: u64, pub src_addr_widths: u32, pub dst_addr_widths: u32 }
#[repr(C)] pub struct dma_chan { pub device_node: list_head, pub chan_id: i32 }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { pub of_node: *mut c_void }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
pub type dma_addr_t = u64;
pub type size_t = usize;
pub type irqreturn_t = i32;
pub type gfp_t = u32;
pub type dma_async_tx_descriptor = c_void;
pub type of_device_id = c_void;
pub type platform_driver = c_void;

#[repr(C)]
pub struct milbeaut_xdmac_desc { pub vd: virt_dma_desc, pub len: size_t, pub src: dma_addr_t, pub dst: dma_addr_t }
#[repr(C)]
pub struct milbeaut_xdmac_chan { pub vc: virt_dma_chan, pub md: *mut milbeaut_xdmac_desc, pub reg_ch_base: *mut u8 }
#[repr(C)]
pub struct milbeaut_xdmac_device { pub ddev: dma_device, pub reg_base: *mut u8, pub channels: [milbeaut_xdmac_chan; 0] }

extern "C" {
    fn readl(addr: *mut u8) -> u32; fn readl_relaxed(addr: *mut u8) -> u32;
    fn writel(val: u32, addr: *mut u8); fn writel_relaxed(val: u32, addr: *mut u8);
    fn vchan_next_desc(vc: *mut virt_dma_chan) -> *mut virt_dma_desc;
    fn vchan_cookie_complete(vd: *mut virt_dma_desc); fn vchan_tx_prep(vc: *mut virt_dma_chan, vd: *mut virt_dma_desc, flags: usize) -> *mut dma_async_tx_descriptor;
    fn vchan_free_chan_resources(vc: *mut virt_dma_chan); fn vchan_terminate_vdesc(vd: *mut virt_dma_desc);
    fn vchan_get_all_descriptors(vc: *mut virt_dma_chan, head: *mut list_head); fn vchan_dma_desc_free_list(vc: *mut virt_dma_chan, head: *mut list_head);
    fn vchan_synchronize(vc: *mut virt_dma_chan); fn vchan_issue_pending(vc: *mut virt_dma_chan) -> bool;
    fn spin_lock(lock: *mut spinlock_t); fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize); fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn to_virt_chan(chan: *mut dma_chan) -> *mut virt_dma_chan; fn vchan_init(vc: *mut virt_dma_chan, ddev: *mut dma_device);
    fn platform_get_irq(pdev: *mut platform_device, n: i32) -> i32; fn platform_irq_count(pdev: *mut platform_device) -> i32;
    fn devm_request_irq(dev: *mut device, irq: i32, handler: unsafe extern "C" fn(i32,*mut c_void)->irqreturn_t, flags: u32, name: *const u8, data: *mut c_void) -> i32;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, n: i32) -> *mut u8;
    fn dma_async_device_register(ddev: *mut dma_device) -> i32; fn dma_async_device_unregister(ddev: *mut dma_device);
    fn of_dma_controller_register(node: *mut c_void, xlate: *mut c_void, data: *mut c_void) -> i32; fn of_dma_controller_free(node: *mut c_void);
    fn dmaengine_terminate_sync(chan: *mut dma_chan) -> i32; fn dma_cookie_status(_: *mut c_void, _: u32, _: *mut c_void) -> i32;
    fn kfree(p: *mut c_void); fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
}

unsafe fn to_desc(vd: *mut virt_dma_desc) -> *mut milbeaut_xdmac_desc { vd as *mut milbeaut_xdmac_desc }
unsafe fn to_chan(vc: *mut virt_dma_chan) -> *mut milbeaut_xdmac_chan { vc as *mut milbeaut_xdmac_chan }
unsafe fn next_desc(mc: *mut milbeaut_xdmac_chan) -> *mut milbeaut_xdmac_desc {
    let vd = vchan_next_desc(&mut (*mc).vc); if vd.is_null() { (*mc).md = core::ptr::null_mut(); return core::ptr::null_mut(); }
    (*vd).node.next = (*vd).node.next; (*mc).md = to_desc(vd); (*mc).md
}
unsafe fn chan_start(mc: *mut milbeaut_xdmac_chan, md: *mut milbeaut_xdmac_desc) {
    let b = (*mc).reg_ch_base; writel_relaxed((*md).len.wrapping_sub(1) as u32, b.add(M10V_XDTBC)); writel_relaxed((*md).src as u32, b.add(M10V_XDSSA)); writel_relaxed((*md).dst as u32, b.add(M10V_XDDSA));
    let mut v = readl_relaxed(b.add(M10V_XDSAC)); v &= !(M10V_XDSAC_SBS|M10V_XDSAC_SBL); v |= field_prep(M10V_XDSAC_SBS,M10V_DEFBS)|field_prep(M10V_XDSAC_SBL,M10V_DEFBL); writel_relaxed(v,b.add(M10V_XDSAC));
    v=readl_relaxed(b.add(M10V_XDDAC)); v&=!(M10V_XDDAC_DBS|M10V_XDDAC_DBL); v|=field_prep(M10V_XDDAC_DBS,M10V_DEFBS)|field_prep(M10V_XDDAC_DBL,M10V_DEFBL); writel_relaxed(v,b.add(M10V_XDDAC));
    v=readl_relaxed(b.add(M10V_XDDES)); v &= !(M10V_XDDES_CE|M10V_XDDES_SE|M10V_XDDES_TF|M10V_XDDES_EI|M10V_XDDES_TI); v |= field_prep(M10V_XDDES_CE,1)|field_prep(M10V_XDDES_SE,1)|field_prep(M10V_XDDES_TF,1)|field_prep(M10V_XDDES_EI,1)|field_prep(M10V_XDDES_TI,1); writel_relaxed(v,b.add(M10V_XDDES));
}
unsafe fn xdmac_start(mc:*mut milbeaut_xdmac_chan){let md=next_desc(mc);if !md.is_null(){chan_start(mc,md)}}

pub unsafe extern "C" fn milbeaut_xdmac_interrupt(_irq:i32, dev_id:*mut c_void)->irqreturn_t { let mc=dev_id as *mut milbeaut_xdmac_chan; spin_lock(&mut (*mc).vc.lock); writel_relaxed(0,(*mc).reg_ch_base.add(M10V_XDDSD)); let md=(*mc).md; if !md.is_null(){vchan_cookie_complete(&mut (*md).vd);xdmac_start(mc)} spin_unlock(&mut (*mc).vc.lock); 1 }
pub unsafe extern "C" fn milbeaut_xdmac_free_chan_resources(chan:*mut dma_chan){vchan_free_chan_resources(to_virt_chan(chan))}
pub unsafe extern "C" fn milbeaut_xdmac_prep_memcpy(chan:*mut dma_chan,dst:dma_addr_t,src:dma_addr_t,len:size_t,flags:usize)->*mut dma_async_tx_descriptor {let vc=to_virt_chan(chan);let md=kzalloc(core::mem::size_of::<milbeaut_xdmac_desc>(),0) as *mut milbeaut_xdmac_desc;if md.is_null(){return core::ptr::null_mut()}(*md).len=len;(*md).src=src;(*md).dst=dst;vchan_tx_prep(vc,&mut (*md).vd,flags)}
pub unsafe extern "C" fn milbeaut_xdmac_terminate_all(chan:*mut dma_chan)->i32 {let vc=to_virt_chan(chan);let mc=to_chan(vc);let mut flags=0;let mut head=list_head{next:core::ptr::null_mut(),prev:core::ptr::null_mut()};spin_lock_irqsave(&mut (*vc).lock,&mut flags);let mut v=readl((*mc).reg_ch_base.add(M10V_XDDES));v&=!M10V_XDDES_CE;writel(v,(*mc).reg_ch_base.add(M10V_XDDES));if !(*mc).md.is_null(){vchan_terminate_vdesc(&mut (*(*mc).md).vd);(*mc).md=core::ptr::null_mut()}vchan_get_all_descriptors(vc,&mut head);spin_unlock_irqrestore(&mut (*vc).lock,flags);vchan_dma_desc_free_list(vc,&mut head);0}
pub unsafe extern "C" fn milbeaut_xdmac_synchronize(chan:*mut dma_chan){vchan_synchronize(to_virt_chan(chan))}
pub unsafe extern "C" fn milbeaut_xdmac_issue_pending(chan:*mut dma_chan){let vc=to_virt_chan(chan);let mc=to_chan(vc);let mut flags=0;spin_lock_irqsave(&mut (*vc).lock,&mut flags);if vchan_issue_pending(vc)&&(*mc).md.is_null(){xdmac_start(mc)}spin_unlock_irqrestore(&mut (*vc).lock,flags)}
pub unsafe extern "C" fn milbeaut_xdmac_desc_free(vd:*mut virt_dma_desc){kfree(to_desc(vd) as *mut c_void)}

pub unsafe extern "C" fn enable_xdmac(mdev:*mut milbeaut_xdmac_device){let b=(*mdev).reg_base;let v=readl(b.add(M10V_XDACS))|M10V_XDACS_XE;writel(v,b.add(M10V_XDACS))}
pub unsafe extern "C" fn disable_xdmac(mdev:*mut milbeaut_xdmac_device){let b=(*mdev).reg_base;let v=readl(b.add(M10V_XDACS))&!M10V_XDACS_XE;writel(v,b.add(M10V_XDACS))}

// Device registration, OF matching, and module metadata are supplied by the
// surrounding kernel integration and retain the C driver's externally visible names.
#[no_mangle] pub static mut milbeaut_xdmac_match: [*const u8; 2] = [b"socionext,milbeaut-m10v-xdmac\0".as_ptr(), core::ptr::null()];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
