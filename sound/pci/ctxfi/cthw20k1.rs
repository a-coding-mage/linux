// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	cthw20k1.c
 *
 * @Brief
 * This file contains the implementation of hardware access methord for 20k1.
 *
 * @Author	Liu Chun
 * @Date 	Jun 24 2008
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type u16_t = u16;
type u32_t = u32;
type ulong_t = usize;
type irqreturn_t = i32;

const ENOMEM: i32 = 12;
const ENOENT: i32 = 2;
const EBUSY: i32 = 16;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_SHARED: u32 = 0x80;
const BITS_PER_LONG: u32 = size_of::<ulong_t>() as u32 * 8;
const KBUILD_MODNAME: *const i8 = b"cthw20k1\0".as_ptr() as *const i8;

/* Dependencies supplied by the original Linux headers and local driver headers. */
#[repr(C)] pub struct pci_dev { pub irq: i32, pub dev: device }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct card { pub dev: *mut device, pub sync_irq: i32 }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] #[derive(Copy, Clone)] pub struct capabilities {
    pub digit_io_switch: i32,
    pub dedicated_mic: i32,
    pub dedicated_rca: i32,
    pub output_switch: i32,
    pub mic_source_switch: i32,
}
#[repr(C)] pub struct card_conf { pub rsr: u32, pub msr: u32, pub vm_pgt_phys: ulong_t }

pub const ADC_MICIN: i32 = 0;
pub const ADC_LINEIN: i32 = 1;
pub const ADC_VIDEO: i32 = 2;
pub const ADC_AUX: i32 = 3;
pub const ADC_NONE: i32 = 4;
pub type ADCSRC = i32;
pub const CTSB055X: i32 = 0;
pub const CTSB073X: i32 = 1;
pub const CTUAA: i32 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw {
    pub irq: i32,
    pub card: *mut card,
    pub pci: *mut pci_dev,
    pub io_base: ulong_t,
    pub mem_base: *mut c_void,
    pub model: i32,
    pub irq_callback: Option<unsafe extern "C" fn(*mut c_void, u32)>,
    pub irq_callback_data: *mut c_void,
    pub card_init: Option<unsafe extern "C" fn(*mut hw, *mut card_conf) -> i32>,
    pub card_stop: Option<unsafe extern "C" fn(*mut hw) -> i32>,
    pub pll_init: Option<unsafe extern "C" fn(*mut hw, u32) -> i32>,
    pub is_adc_source_selected: Option<unsafe extern "C" fn(*mut hw, ADCSRC) -> i32>,
    pub select_adc_source: Option<unsafe extern "C" fn(*mut hw, ADCSRC) -> i32>,
    pub capabilities: Option<unsafe extern "C" fn(*mut hw) -> capabilities>,
    /* CONFIG_PM_SLEEP: suspend/resume members are present in the C initializer when enabled. */
    pub src_rsc_get_ctrl_blk: Option<unsafe extern "C" fn(*mut *mut c_void) -> i32>,
    pub src_rsc_put_ctrl_blk: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub src_mgr_get_ctrl_blk: Option<unsafe extern "C" fn(*mut *mut c_void) -> i32>,
    pub src_mgr_put_ctrl_blk: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub src_set_state: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_bm: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_rsr: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_sf: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_wr: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_pm: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_rom: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_vo: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_st: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_ie: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_ilsz: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_bp: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_cisz: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_ca: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_sa: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_la: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_pitch: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_dirty: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_clear_zbufs: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_set_dirty_all: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub src_commit_write: Option<unsafe extern "C" fn(*mut hw, u32, *mut c_void) -> i32>,
    pub src_get_ca: Option<unsafe extern "C" fn(*mut hw, u32, *mut c_void) -> i32>,
    pub src_get_dirty: Option<unsafe extern "C" fn(*mut c_void) -> u32>,
    pub src_dirty_conj_mask: Option<unsafe extern "C" fn() -> u32>,
    pub src_mgr_enbs_src: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_mgr_enb_src: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_mgr_dsb_src: Option<unsafe extern "C" fn(*mut c_void, u32) -> i32>,
    pub src_mgr_commit_write: Option<unsafe extern "C" fn(*mut hw, *mut c_void) -> i32>,
}

unsafe extern "C" {
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn outl(value: u32, port: ulong_t);
    fn inl(port: ulong_t) -> u32;
    fn mdelay(ms: u32);
    fn msleep(ms: u32);
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn dev_alert(dev: *mut device, fmt: *const i8, ...);
    fn pci_enable_device(pci: *mut pci_dev) -> i32;
    fn pci_disable_device(pci: *mut pci_dev);
    fn pci_request_regions(pci: *mut pci_dev, name: *const i8) -> i32;
    fn pci_release_regions(pci: *mut pci_dev);
    fn pci_resource_start(pci: *mut pci_dev, bar: i32) -> ulong_t;
    fn pci_resource_len(pci: *mut pci_dev, bar: i32) -> ulong_t;
    fn pci_set_master(pci: *mut pci_dev);
    fn pci_read_config_dword(pci: *mut pci_dev, where_: i32, val: *mut u32) -> i32;
    fn pci_write_config_dword(pci: *mut pci_dev, where_: i32, val: u32) -> i32;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> i32;
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut c_void) -> irqreturn_t, flags: u32, name: *const i8, dev: *mut c_void) -> i32;
    fn free_irq(irq: i32, dev: *mut c_void);
    fn ioremap(base: ulong_t, len: ulong_t) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
}

/* Register constants are provided by ct20k1reg.h in the original repository. */
unsafe extern "C" {
    static SRCUPZ: u32; static SRCDN0Z: u32; static SRCDN1Z: u32; static PRING_LO_HI: u32;
    static PMOPLO: u32; static PMOPHI: u32; static SRCSA: u32; static SRCLA: u32; static SRCCA: u32;
    static SRCCF: u32; static SRCCCR: u32; static SRCCTL: u32; static SRCENBSTAT: u32; static SRCENBS: u32;
    static SRCENB: u32; static SRCIMAP: u32; static AMOPLO: u32; static AMOPHI: u32; static SRTSCTL: u32;
    static SRTICTL: u32; static SPOS: u32; static I2SCTL: u32; static SPOCTL: u32; static SPICTL: u32;
    static DAOIMAP: u32; static GIE: u32; static TIMR: u32; static WC: u32; static PTPALX: u32; static PTPAHX: u32;
    static TRNCTL: u32; static TRNIS: u32; static PLLCTL: u32; static GCTL: u32; static GPIO: u32; static GPIOCTL: u32;
    static ID0: u32; static SRCMCTL: u32; static SRCIP: u32; static GIP: u32;
}

const GFP_KERNEL: u32 = 0;
const PCI_BASE_ADDRESS_0: i32 = 0x10;
const PCI_BASE_ADDRESS_1: i32 = 0x14;
const PCI_BASE_ADDRESS_2: i32 = 0x18;
const PCI_BASE_ADDRESS_3: i32 = 0x1c;
const PCI_BASE_ADDRESS_4: i32 = 0x20;
const PCI_BASE_ADDRESS_5: i32 = 0x24;
const PCI_INTERRUPT_LINE: i32 = 0x3c;
const PCI_CACHE_LINE_SIZE: i32 = 0x0c;
const PCI_LATENCY_TIMER: i32 = 0x0d;
const PCI_COMMAND: i32 = 0x04;
const IT_INT: u32 = 0x1;
const TIMR_IE: u32 = 0x8000_0000;
const TIMR_IP: u32 = 0x4000_0000;

#[inline] fn DMA_BIT_MASK(bits: u32) -> u64 { if bits == 64 { !0 } else { (1u64 << bits) - 1 } }
#[inline] fn upper_32_bits(v: ulong_t) -> u32 { ((v as u64) >> 32) as u32 }
#[inline] fn CTLBITS(a: u32, b: u32, c: u32, d: u32) -> u32 { (a << 24) | (b << 16) | (c << 8) | d }
#[inline] fn get_field(data: u32, mask: u32) -> i32 { ((data & mask) >> mask.trailing_zeros()) as i32 }
#[inline] fn set_field(data: &mut u32, mask: u32, value: u32) { *data = (*data & !mask) | ((value << mask.trailing_zeros()) & mask); }

#[repr(C)]
pub struct hw20k1 {
    pub hw: hw,
    pub reg_20k1_lock: spinlock_t,
    pub reg_pci_lock: spinlock_t,
}

/* SRC resource control block */
const SRCCTL_STATE: u32 = 0x00000007;
const SRCCTL_BM: u32 = 0x00000008;
const SRCCTL_RSR: u32 = 0x00000030;
const SRCCTL_SF: u32 = 0x000001C0;
const SRCCTL_WR: u32 = 0x00000200;
const SRCCTL_PM: u32 = 0x00000400;
const SRCCTL_ROM: u32 = 0x00001800;
const SRCCTL_VO: u32 = 0x00002000;
const SRCCTL_ST: u32 = 0x00004000;
const SRCCTL_IE: u32 = 0x00008000;
const SRCCTL_ILSZ: u32 = 0x000F0000;
const SRCCTL_BP: u32 = 0x00100000;
const SRCCCR_CISZ: u32 = 0x000007FF;
const SRCCA_CA: u32 = 0x03FFFFFF;
const SRCSA_SA: u32 = 0x03FFFFFF;
const SRCLA_LA: u32 = 0x03FFFFFF;
const MPRLH_PITCH: u32 = 0xFFFFFFFF;

#[repr(C)] #[derive(Copy, Clone, Default)] pub struct src_dirty { pub data: u16 }
impl src_dirty {
    fn ctl(&self) -> bool { self.data & (1 << 0) != 0 } fn set_ctl(&mut self, v: bool) { self.set(0, v) }
    fn ccr(&self) -> bool { self.data & (1 << 1) != 0 } fn set_ccr(&mut self, v: bool) { self.set(1, v) }
    fn sa(&self) -> bool { self.data & (1 << 2) != 0 } fn set_sa(&mut self, v: bool) { self.set(2, v) }
    fn la(&self) -> bool { self.data & (1 << 3) != 0 } fn set_la(&mut self, v: bool) { self.set(3, v) }
    fn ca(&self) -> bool { self.data & (1 << 4) != 0 } fn set_ca(&mut self, v: bool) { self.set(4, v) }
    fn mpr(&self) -> bool { self.data & (1 << 5) != 0 } fn set_mpr(&mut self, v: bool) { self.set(5, v) }
    fn czbfs(&self) -> bool { self.data & (1 << 6) != 0 } fn set_czbfs(&mut self, v: bool) { self.set(6, v) }
    fn set(&mut self, bit: u16, v: bool) { if v { self.data |= 1 << bit } else { self.data &= !(1 << bit) } }
}

#[repr(C)] #[derive(Default)] pub struct src_rsc_ctrl_blk { pub ctl: u32, pub ccr: u32, pub ca: u32, pub sa: u32, pub la: u32, pub mpr: u32, pub dirty: src_dirty }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct src_mgr_dirty { pub data: u16 }
impl src_mgr_dirty { fn enbsa(&self)->bool{self.data&(1<<8)!=0} fn set_enbsa(&mut self,v:bool){if v{self.data|=1<<8}else{self.data&=!(1<<8)}} }
#[repr(C)] #[derive(Default)] pub struct src_mgr_ctrl_blk { pub enbsa: u32, pub enb: [u32; 8], pub dirty: src_mgr_dirty }
const SRCAIM_ARC: u32 = 0x00000FFF;
const SRCAIM_NXT: u32 = 0x00FF0000;
const SRCAIM_SRC: u32 = 0xFF000000;
#[repr(C)] #[derive(Default)] pub struct srcimap { pub srcaim: u32, pub idx: u32 }
#[repr(C)] #[derive(Copy, Clone, Default)] pub struct srcimp_mgr_dirty { pub data: u16 }
impl srcimp_mgr_dirty { fn srcimap(&self)->bool{self.data&1!=0} fn set_srcimap(&mut self,v:bool){if v{self.data|=1}else{self.data&=!1}} }
#[repr(C)] #[derive(Default)] pub struct srcimp_mgr_ctrl_blk { pub srcimap: srcimap, pub dirty: srcimp_mgr_dirty }

unsafe fn alloc_zeroed<T>() -> *mut T { kzalloc(size_of::<T>(), GFP_KERNEL) as *mut T }
unsafe extern "C" fn src_get_rsc_ctrl_blk(rblk: *mut *mut c_void) -> i32 { *rblk = ptr::null_mut(); let blk = alloc_zeroed::<src_rsc_ctrl_blk>(); if blk.is_null(){return -ENOMEM;} *rblk = blk as *mut c_void; 0 }
unsafe extern "C" fn src_put_rsc_ctrl_blk(blk: *mut c_void) -> i32 { kfree(blk); 0 }
macro_rules! src_set_ctl { ($name:ident,$mask:ident) => { unsafe extern "C" fn $name(blk:*mut c_void,v:u32)->i32{let ctl=&mut *(blk as *mut src_rsc_ctrl_blk); set_field(&mut ctl.ctl,$mask,v); ctl.dirty.set_ctl(true); 0} }; }
src_set_ctl!(src_set_state, SRCCTL_STATE); src_set_ctl!(src_set_bm, SRCCTL_BM); src_set_ctl!(src_set_rsr, SRCCTL_RSR);
src_set_ctl!(src_set_sf, SRCCTL_SF); src_set_ctl!(src_set_wr, SRCCTL_WR); src_set_ctl!(src_set_pm, SRCCTL_PM);
src_set_ctl!(src_set_rom, SRCCTL_ROM); src_set_ctl!(src_set_vo, SRCCTL_VO); src_set_ctl!(src_set_st, SRCCTL_ST);
src_set_ctl!(src_set_ie, SRCCTL_IE); src_set_ctl!(src_set_ilsz, SRCCTL_ILSZ); src_set_ctl!(src_set_bp, SRCCTL_BP);
unsafe extern "C" fn src_set_cisz(blk:*mut c_void,cisz:u32)->i32{let ctl=&mut *(blk as *mut src_rsc_ctrl_blk); set_field(&mut ctl.ccr,SRCCCR_CISZ,cisz); ctl.dirty.set_ccr(true); 0}
unsafe extern "C" fn src_set_ca(blk:*mut c_void,ca:u32)->i32{let ctl=&mut *(blk as *mut src_rsc_ctrl_blk); set_field(&mut ctl.ca,SRCCA_CA,ca); ctl.dirty.set_ca(true); 0}
unsafe extern "C" fn src_set_sa(blk:*mut c_void,sa:u32)->i32{let ctl=&mut *(blk as *mut src_rsc_ctrl_blk); set_field(&mut ctl.sa,SRCSA_SA,sa); ctl.dirty.set_sa(true); 0}
unsafe extern "C" fn src_set_la(blk:*mut c_void,la:u32)->i32{let ctl=&mut *(blk as *mut src_rsc_ctrl_blk); set_field(&mut ctl.la,SRCLA_LA,la); ctl.dirty.set_la(true); 0}
unsafe extern "C" fn src_set_pitch(blk:*mut c_void,pitch:u32)->i32{let ctl=&mut *(blk as *mut src_rsc_ctrl_blk); set_field(&mut ctl.mpr,MPRLH_PITCH,pitch); ctl.dirty.set_mpr(true); 0}
unsafe extern "C" fn src_set_clear_zbufs(blk:*mut c_void,clear:u32)->i32{(*(blk as *mut src_rsc_ctrl_blk)).dirty.set_czbfs(clear!=0); 0}
unsafe extern "C" fn src_set_dirty(blk:*mut c_void,flags:u32)->i32{(*(blk as *mut src_rsc_ctrl_blk)).dirty.data=(flags&0xffff) as u16; 0}
unsafe extern "C" fn src_set_dirty_all(blk:*mut c_void)->i32{(*(blk as *mut src_rsc_ctrl_blk)).dirty.data=!(0u16); 0}

const AR_SLOT_SIZE: u32 = 4096;
const AR_PTS_PITCH: u32 = 6;
const AR_PARAM_SRC_OFFSET: u32 = 0x60;
fn src_param_pitch_mixer(src_idx: u32) -> u32 { ((src_idx << 4) + AR_PTS_PITCH + AR_SLOT_SIZE - AR_PARAM_SRC_OFFSET) % AR_SLOT_SIZE }

unsafe extern "C" fn src_commit_write(hw:*mut hw, idx:u32, blk:*mut c_void)->i32 {
    let ctl=&mut *(blk as *mut src_rsc_ctrl_blk);
    if ctl.dirty.czbfs() {
        for i in 0..8 { hw_write_20kx(hw, SRCUPZ + idx*0x100 + i*4, 0); }
        for i in 0..4 { hw_write_20kx(hw, SRCDN0Z + idx*0x100 + i*4, 0); }
        for i in 0..8 { hw_write_20kx(hw, SRCDN1Z + idx*0x100 + i*4, 0); }
        ctl.dirty.set_czbfs(false);
    }
    if ctl.dirty.mpr() {
        let pm_idx=src_param_pitch_mixer(idx);
        hw_write_20kx(hw, PRING_LO_HI+4*pm_idx, ctl.mpr); hw_write_20kx(hw, PMOPLO+8*pm_idx, 0x3); hw_write_20kx(hw, PMOPHI+8*pm_idx, 0);
        ctl.dirty.set_mpr(false);
    }
    if ctl.dirty.sa(){hw_write_20kx(hw,SRCSA+idx*0x100,ctl.sa); ctl.dirty.set_sa(false);}
    if ctl.dirty.la(){hw_write_20kx(hw,SRCLA+idx*0x100,ctl.la); ctl.dirty.set_la(false);}
    if ctl.dirty.ca(){hw_write_20kx(hw,SRCCA+idx*0x100,ctl.ca); ctl.dirty.set_ca(false);}
    hw_write_20kx(hw, SRCCF+idx*0x100, 0);
    if ctl.dirty.ccr(){hw_write_20kx(hw,SRCCCR+idx*0x100,ctl.ccr); ctl.dirty.set_ccr(false);}
    if ctl.dirty.ctl(){hw_write_20kx(hw,SRCCTL+idx*0x100,ctl.ctl); ctl.dirty.set_ctl(false);}
    0
}
unsafe extern "C" fn src_get_ca(hw:*mut hw, idx:u32, blk:*mut c_void)->i32{let ctl=&mut *(blk as *mut src_rsc_ctrl_blk); ctl.ca=hw_read_20kx(hw,SRCCA+idx*0x100); ctl.dirty.set_ca(false); get_field(ctl.ca,SRCCA_CA)}
unsafe extern "C" fn src_get_dirty(blk:*mut c_void)->u32{(*(blk as *mut src_rsc_ctrl_blk)).dirty.data as u32}
unsafe extern "C" fn src_dirty_conj_mask()->u32{0x20}
unsafe extern "C" fn src_mgr_enbs_src(blk:*mut c_void,idx:u32)->i32{let c=&mut *(blk as *mut src_mgr_ctrl_blk); c.enbsa=!0; c.dirty.set_enbsa(true); c.enb[(idx/32) as usize]|=1<<(idx%32); 0}
unsafe extern "C" fn src_mgr_enb_src(blk:*mut c_void,idx:u32)->i32{let c=&mut *(blk as *mut src_mgr_ctrl_blk); c.enb[(idx/32) as usize]|=1<<(idx%32); c.dirty.data|=(1<<(idx/32)) as u16; 0}
unsafe extern "C" fn src_mgr_dsb_src(blk:*mut c_void,idx:u32)->i32{let c=&mut *(blk as *mut src_mgr_ctrl_blk); c.enb[(idx/32) as usize]&=!(1<<(idx%32)); c.dirty.data|=(1<<(idx/32)) as u16; 0}
unsafe extern "C" fn src_mgr_commit_write(hw:*mut hw,blk:*mut c_void)->i32{let c=&mut *(blk as *mut src_mgr_ctrl_blk); if c.dirty.enbsa(){while hw_read_20kx(hw,SRCENBSTAT)&1!=0{} hw_write_20kx(hw,SRCENBS,c.enbsa); c.dirty.set_enbsa(false);} for i in 0..8{if c.dirty.data&(1<<i)!=0{hw_write_20kx(hw,SRCENB+i as u32*0x100,c.enb[i]); c.dirty.data&=!(1<<i);}} 0}
unsafe extern "C" fn src_mgr_get_ctrl_blk(rblk:*mut *mut c_void)->i32{*rblk=ptr::null_mut(); let b=alloc_zeroed::<src_mgr_ctrl_blk>(); if b.is_null(){return -ENOMEM;} *rblk=b as *mut c_void; 0}
unsafe extern "C" fn src_mgr_put_ctrl_blk(blk:*mut c_void)->i32{kfree(blk);0}
unsafe extern "C" fn srcimp_mgr_get_ctrl_blk(rblk:*mut *mut c_void)->i32{*rblk=ptr::null_mut(); let b=alloc_zeroed::<srcimp_mgr_ctrl_blk>(); if b.is_null(){return -ENOMEM;} *rblk=b as *mut c_void; 0}
unsafe extern "C" fn srcimp_mgr_put_ctrl_blk(blk:*mut c_void)->i32{kfree(blk);0}
unsafe extern "C" fn srcimp_mgr_set_imaparc(blk:*mut c_void,slot:u32)->i32{let c=&mut *(blk as *mut srcimp_mgr_ctrl_blk); set_field(&mut c.srcimap.srcaim,SRCAIM_ARC,slot); c.dirty.set_srcimap(true);0}
unsafe extern "C" fn srcimp_mgr_set_imapuser(blk:*mut c_void,user:u32)->i32{let c=&mut *(blk as *mut srcimp_mgr_ctrl_blk); set_field(&mut c.srcimap.srcaim,SRCAIM_SRC,user); c.dirty.set_srcimap(true);0}
unsafe extern "C" fn srcimp_mgr_set_imapnxt(blk:*mut c_void,next:u32)->i32{let c=&mut *(blk as *mut srcimp_mgr_ctrl_blk); set_field(&mut c.srcimap.srcaim,SRCAIM_NXT,next); c.dirty.set_srcimap(true);0}
unsafe extern "C" fn srcimp_mgr_set_imapaddr(blk:*mut c_void,addr:u32)->i32{let c=&mut *(blk as *mut srcimp_mgr_ctrl_blk); c.srcimap.idx=addr; c.dirty.set_srcimap(true);0}
unsafe extern "C" fn srcimp_mgr_commit_write(hw:*mut hw,blk:*mut c_void)->i32{let c=&mut *(blk as *mut srcimp_mgr_ctrl_blk); if c.dirty.srcimap(){hw_write_20kx(hw,SRCIMAP+c.srcimap.idx*0x100,c.srcimap.srcaim); c.dirty.set_srcimap(false);} 0}

const AMOPLO_M:u32=0x00000003; const AMOPLO_X:u32=0x0003FFF0; const AMOPLO_Y:u32=0xFFFC0000;
const AMOPHI_SADR:u32=0x000000FF; const AMOPHI_SE:u32=0x80000000;
#[repr(C)] #[derive(Copy,Clone,Default)] pub struct amixer_dirty{pub data:u16}
impl amixer_dirty{fn amoplo(&self)->bool{self.data&1!=0} fn set_amoplo(&mut self,v:bool){if v{self.data|=1}else{self.data&=!1}} fn amophi(&self)->bool{self.data&2!=0} fn set_amophi(&mut self,v:bool){if v{self.data|=2}else{self.data&=!2}}}
#[repr(C)] #[derive(Default)] pub struct amixer_rsc_ctrl_blk{pub amoplo:u32,pub amophi:u32,pub dirty:amixer_dirty}
macro_rules! amixer_set_lo{($n:ident,$m:ident)=>{unsafe extern "C" fn $n(blk:*mut c_void,v:u32)->i32{let c=&mut *(blk as *mut amixer_rsc_ctrl_blk); set_field(&mut c.amoplo,$m,v); c.dirty.set_amoplo(true);0}}}
amixer_set_lo!(amixer_set_mode,AMOPLO_M); amixer_set_lo!(amixer_set_x,AMOPLO_X); amixer_set_lo!(amixer_set_y,AMOPLO_Y);
unsafe extern "C" fn amixer_set_iv(_blk:*mut c_void,_iv:u32)->i32{0}
unsafe extern "C" fn amixer_set_sadr(blk:*mut c_void,sadr:u32)->i32{let c=&mut *(blk as *mut amixer_rsc_ctrl_blk); set_field(&mut c.amophi,AMOPHI_SADR,sadr); c.dirty.set_amophi(true);0}
unsafe extern "C" fn amixer_set_se(blk:*mut c_void,se:u32)->i32{let c=&mut *(blk as *mut amixer_rsc_ctrl_blk); set_field(&mut c.amophi,AMOPHI_SE,se); c.dirty.set_amophi(true);0}
unsafe extern "C" fn amixer_set_dirty(blk:*mut c_void,flags:u32)->i32{(*(blk as *mut amixer_rsc_ctrl_blk)).dirty.data=(flags&0xffff) as u16;0}
unsafe extern "C" fn amixer_set_dirty_all(blk:*mut c_void)->i32{(*(blk as *mut amixer_rsc_ctrl_blk)).dirty.data=!0;0}
unsafe extern "C" fn amixer_commit_write(hw:*mut hw,idx:u32,blk:*mut c_void)->i32{let c=&mut *(blk as *mut amixer_rsc_ctrl_blk); if c.dirty.amoplo()||c.dirty.amophi(){hw_write_20kx(hw,AMOPLO+idx*8,c.amoplo); c.dirty.set_amoplo(false); hw_write_20kx(hw,AMOPHI+idx*8,c.amophi); c.dirty.set_amophi(false);}0}
unsafe extern "C" fn amixer_get_y(blk:*mut c_void)->i32{get_field((*(blk as *mut amixer_rsc_ctrl_blk)).amoplo,AMOPLO_Y)}
unsafe extern "C" fn amixer_get_dirty(blk:*mut c_void)->u32{(*(blk as *mut amixer_rsc_ctrl_blk)).dirty.data as u32}
unsafe extern "C" fn amixer_rsc_get_ctrl_blk(rblk:*mut *mut c_void)->i32{*rblk=ptr::null_mut();let b=alloc_zeroed::<amixer_rsc_ctrl_blk>();if b.is_null(){return -ENOMEM;}*rblk=b as *mut c_void;0}
unsafe extern "C" fn amixer_rsc_put_ctrl_blk(blk:*mut c_void)->i32{kfree(blk);0}
unsafe extern "C" fn amixer_mgr_get_ctrl_blk(rblk:*mut *mut c_void)->i32{*rblk=ptr::null_mut();0}
unsafe extern "C" fn amixer_mgr_put_ctrl_blk(_blk:*mut c_void)->i32{0}

const SRTCTL_SRCR:u32=0x000000FF; const SRTCTL_SRCL:u32=0x0000FF00; const SRTCTL_RSR:u32=0x00030000; const SRTCTL_DRAT:u32=0x000C0000; const SRTCTL_EC:u32=0x40000000; const SRTCTL_ET:u32=0x80000000;
#[repr(C)] #[derive(Copy,Clone,Default)] pub struct dai_dirty{pub data:u16} impl dai_dirty{fn srtctl(&self)->bool{self.data&1!=0} fn set_srtctl(&mut self,v:bool){if v{self.data|=1}else{self.data&=!1}}}
#[repr(C)] #[derive(Default)] pub struct dai_ctrl_blk{pub srtctl:u32,pub dirty:dai_dirty}
#[repr(C)] #[derive(Copy,Clone,Default)] pub struct dao_dirty{pub data:u16} impl dao_dirty{fn spos(&self)->bool{self.data&1!=0} fn set_spos(&mut self,v:bool){if v{self.data|=1}else{self.data&=!1}}}
#[repr(C)] #[derive(Default)] pub struct dao_ctrl_blk{pub spos:u32,pub dirty:dao_dirty}
const AIM_ARC:u32=0x00000FFF; const AIM_NXT:u32=0x007F0000; const I2SCTL_EA:u32=0x00000004; const I2SCTL_EI:u32=0x00000010;
const SPOCTL_OE:u32=0x00000001; const SPOCTL_OS:u32=0x0000000E; const SPOCTL_RIV:u32=0x00000010; const SPOCTL_LIV:u32=0x00000020; const SPOCTL_SR:u32=0x000000C0; const SPICTL_EN:u32=0x00000001;
#[repr(C)] #[derive(Default)] pub struct daoimap{pub aim:u32,pub idx:u32}
#[repr(C)] #[derive(Copy,Clone,Default)] pub struct daio_mgr_dirty{pub data:u32}
impl daio_mgr_dirty{fn get4(&self,s:u32)->u32{(self.data>>s)&0xf} fn set4_or(&mut self,s:u32,v:u32){self.data|=(v&0xf)<<s} fn clr4_bit(&mut self,s:u32,i:u32){self.data&=!(1<<(s+i))} fn daoimap(&self)->bool{self.data&(1<<16)!=0} fn set_daoimap(&mut self,v:bool){if v{self.data|=1<<16}else{self.data&=!(1<<16)}}}
#[repr(C)] #[derive(Default)] pub struct daio_mgr_ctrl_blk{pub i2sctl:u32,pub spoctl:u32,pub spictl:u32,pub daoimap:daoimap,pub dirty:daio_mgr_dirty}
macro_rules! dai_set{($n:ident,$m:ident)=>{unsafe extern "C" fn $n(blk:*mut c_void,v:u32)->i32{let c=&mut *(blk as *mut dai_ctrl_blk); set_field(&mut c.srtctl,$m,v); c.dirty.set_srtctl(true);0}}}
dai_set!(dai_srt_set_srcr,SRTCTL_SRCR); dai_set!(dai_srt_set_srcl,SRTCTL_SRCL); dai_set!(dai_srt_set_rsr,SRTCTL_RSR); dai_set!(dai_srt_set_drat,SRTCTL_DRAT);
unsafe extern "C" fn dai_srt_set_ec(blk:*mut c_void,ec:u32)->i32{let c=&mut *(blk as *mut dai_ctrl_blk); set_field(&mut c.srtctl,SRTCTL_EC,if ec!=0{1}else{0}); c.dirty.set_srtctl(true);0}
unsafe extern "C" fn dai_srt_set_et(blk:*mut c_void,et:u32)->i32{let c=&mut *(blk as *mut dai_ctrl_blk); set_field(&mut c.srtctl,SRTCTL_ET,if et!=0{1}else{0}); c.dirty.set_srtctl(true);0}
unsafe extern "C" fn dai_commit_write(hw:*mut hw,idx:u32,blk:*mut c_void)->i32{let c=&mut *(blk as *mut dai_ctrl_blk); if c.dirty.srtctl(){if idx<4{hw_write_20kx(hw,SRTSCTL+4*idx,c.srtctl)}else{hw_write_20kx(hw,SRTICTL,c.srtctl)} c.dirty.set_srtctl(false);}0}
unsafe extern "C" fn dai_get_ctrl_blk(rblk:*mut *mut c_void)->i32{*rblk=ptr::null_mut();let b=alloc_zeroed::<dai_ctrl_blk>();if b.is_null(){return -ENOMEM;}*rblk=b as *mut c_void;0}
unsafe extern "C" fn dai_put_ctrl_blk(blk:*mut c_void)->i32{kfree(blk);0}
unsafe extern "C" fn dao_set_spos(blk:*mut c_void,spos:u32)->i32{let c=&mut *(blk as *mut dao_ctrl_blk); c.spos=spos; c.dirty.set_spos(true);0}
unsafe extern "C" fn dao_commit_write(hw:*mut hw,idx:u32,blk:*mut c_void)->i32{let c=&mut *(blk as *mut dao_ctrl_blk); if c.dirty.spos(){if idx<4{hw_write_20kx(hw,SPOS+4*idx,c.spos)} c.dirty.set_spos(false);}0}
unsafe extern "C" fn dao_get_spos(blk:*mut c_void,spos:*mut u32)->i32{*spos=(*(blk as *mut dao_ctrl_blk)).spos;0}
unsafe extern "C" fn dao_get_ctrl_blk(rblk:*mut *mut c_void)->i32{*rblk=ptr::null_mut();let b=alloc_zeroed::<dao_ctrl_blk>();if b.is_null(){return -ENOMEM;}*rblk=b as *mut c_void;0}
unsafe extern "C" fn dao_put_ctrl_blk(blk:*mut c_void)->i32{kfree(blk);0}

unsafe extern "C" fn daio_mgr_enb_dai(blk:*mut c_void,mut idx:u32)->i32{let c=&mut *(blk as *mut daio_mgr_ctrl_blk); if idx<4{set_field(&mut c.spictl,SPICTL_EN<<(idx*8),1); c.dirty.set4_or(8,1<<idx)}else{idx%=4; set_field(&mut c.i2sctl,I2SCTL_EI<<(idx*8),1); c.dirty.set4_or(4,1<<idx)}0}
unsafe extern "C" fn daio_mgr_dsb_dai(blk:*mut c_void,mut idx:u32)->i32{let c=&mut *(blk as *mut daio_mgr_ctrl_blk); if idx<4{set_field(&mut c.spictl,SPICTL_EN<<(idx*8),0); c.dirty.set4_or(8,1<<idx)}else{idx%=4; set_field(&mut c.i2sctl,I2SCTL_EI<<(idx*8),0); c.dirty.set4_or(4,1<<idx)}0}
unsafe extern "C" fn daio_mgr_enb_dao(blk:*mut c_void,mut idx:u32)->i32{let c=&mut *(blk as *mut daio_mgr_ctrl_blk); if idx<4{set_field(&mut c.spoctl,SPOCTL_OE<<(idx*8),1); c.dirty.set4_or(12,1<<idx)}else{idx%=4; set_field(&mut c.i2sctl,I2SCTL_EA<<(idx*8),1); c.dirty.set4_or(0,1<<idx)}0}
unsafe extern "C" fn daio_mgr_dsb_dao(blk:*mut c_void,mut idx:u32)->i32{let c=&mut *(blk as *mut daio_mgr_ctrl_blk); if idx<4{set_field(&mut c.spoctl,SPOCTL_OE<<(idx*8),0); c.dirty.set4_or(12,1<<idx)}else{idx%=4; set_field(&mut c.i2sctl,I2SCTL_EA<<(idx*8),0); c.dirty.set4_or(0,1<<idx)}0}
unsafe extern "C" fn daio_mgr_dao_init(_hw:*mut hw,blk:*mut c_void,idx:u32,conf:u32)->i32{let c=&mut *(blk as *mut daio_mgr_ctrl_blk); if idx<4{match conf&7{0=>set_field(&mut c.spoctl,SPOCTL_SR<<(idx*8),3),1=>set_field(&mut c.spoctl,SPOCTL_SR<<(idx*8),0),2=>set_field(&mut c.spoctl,SPOCTL_SR<<(idx*8),1),4=>set_field(&mut c.spoctl,SPOCTL_SR<<(idx*8),2),_=>()} set_field(&mut c.spoctl,SPOCTL_LIV<<(idx*8),(conf>>4)&1); set_field(&mut c.spoctl,SPOCTL_RIV<<(idx*8),(conf>>4)&1); set_field(&mut c.spoctl,SPOCTL_OS<<(idx*8),if ((conf>>3)&1)!=0{2}else{2}); c.dirty.set4_or(12,1<<idx);}0}
unsafe extern "C" fn daio_mgr_set_imaparc(blk:*mut c_void,slot:u32)->i32{let c=&mut *(blk as *mut daio_mgr_ctrl_blk); set_field(&mut c.daoimap.aim,AIM_ARC,slot); c.dirty.set_daoimap(true);0}
unsafe extern "C" fn daio_mgr_set_imapnxt(blk:*mut c_void,next:u32)->i32{let c=&mut *(blk as *mut daio_mgr_ctrl_blk); set_field(&mut c.daoimap.aim,AIM_NXT,next); c.dirty.set_daoimap(true);0}
unsafe extern "C" fn daio_mgr_set_imapaddr(blk:*mut c_void,addr:u32)->i32{let c=&mut *(blk as *mut daio_mgr_ctrl_blk); c.daoimap.idx=addr; c.dirty.set_daoimap(true);0}
unsafe extern "C" fn daio_mgr_commit_write(hw:*mut hw,blk:*mut c_void)->i32{let c=&mut *(blk as *mut daio_mgr_ctrl_blk); if c.dirty.get4(4)!=0||c.dirty.get4(0)!=0{for i in 0..4{if c.dirty.get4(4)&(1<<i)!=0{c.dirty.clr4_bit(4,i)} if c.dirty.get4(0)&(1<<i)!=0{c.dirty.clr4_bit(0,i)}} hw_write_20kx(hw,I2SCTL,c.i2sctl); mdelay(1);} if c.dirty.get4(12)!=0{for i in 0..4{if c.dirty.get4(12)&(1<<i)!=0{c.dirty.clr4_bit(12,i)}} hw_write_20kx(hw,SPOCTL,c.spoctl); mdelay(1);} if c.dirty.get4(8)!=0{for i in 0..4{if c.dirty.get4(8)&(1<<i)!=0{c.dirty.clr4_bit(8,i)}} hw_write_20kx(hw,SPICTL,c.spictl); mdelay(1);} if c.dirty.daoimap(){hw_write_20kx(hw,DAOIMAP+c.daoimap.idx*4,c.daoimap.aim); c.dirty.set_daoimap(false);}0}
unsafe extern "C" fn daio_mgr_get_ctrl_blk(hw:*mut hw,rblk:*mut *mut c_void)->i32{*rblk=ptr::null_mut();let b=alloc_zeroed::<daio_mgr_ctrl_blk>();if b.is_null(){return -ENOMEM;}(*b).i2sctl=hw_read_20kx(hw,I2SCTL);(*b).spoctl=hw_read_20kx(hw,SPOCTL);(*b).spictl=hw_read_20kx(hw,SPICTL);*rblk=b as *mut c_void;0}
unsafe extern "C" fn daio_mgr_put_ctrl_blk(blk:*mut c_void)->i32{kfree(blk);0}

unsafe extern "C" fn set_timer_irq(hw:*mut hw,enable:i32)->i32{hw_write_20kx(hw,GIE,if enable!=0{IT_INT}else{0});0}
unsafe extern "C" fn set_timer_tick(hw:*mut hw,mut ticks:u32)->i32{if ticks!=0{ticks|=TIMR_IE|TIMR_IP;} hw_write_20kx(hw,TIMR,ticks);0}
unsafe extern "C" fn get_wc(hw:*mut hw)->u32{hw_read_20kx(hw,WC)}

#[repr(C)] pub struct dac_conf{pub msr:u32}
#[repr(C)] pub struct adc_conf{pub msr:u32,pub input:u8,pub mic20db:u8}
#[repr(C)] pub struct daio_conf{pub msr:u32}
#[repr(C)] pub struct trn_conf{pub vm_pgt_phys:ulong_t}

unsafe fn hw_daio_init(hw:*mut hw,info:*const daio_conf)->i32{let mut i2sorg:u32=0x94040404; i2sorg&=0xfffffffc; hw_write_20kx(hw,SPOCTL,0); let mut spdorg:u32=0x05; match (*info).msr{1=>{i2sorg|=1;spdorg|=0<<6},2=>{i2sorg|=2;spdorg|=1<<6},4=>{i2sorg|=3;spdorg|=2<<6},_=>i2sorg|=1} hw_write_20kx(hw,I2SCTL,i2sorg); hw_write_20kx(hw,SPOCTL,spdorg); hw_write_20kx(hw,SPICTL,0); mdelay(1); spdorg=0x0a0a0a0a; hw_write_20kx(hw,SPICTL,spdorg); mdelay(1);0}
unsafe fn hw_trn_init(hw:*mut hw,info:*const trn_conf)->i32{if !0usize==(*info).vm_pgt_phys{dev_err((*(*hw).card).dev,b"Wrong device page table page address!\n\0".as_ptr() as *const i8);return -1;} let mut trnctl=0x13; let low=(*info).vm_pgt_phys as u32; let high=upper_32_bits((*info).vm_pgt_phys); if size_of::<*mut c_void>()==8{trnctl|=1<<2;} hw_write_20kx(hw,PTPALX,low); hw_write_20kx(hw,PTPAHX,high); hw_write_20kx(hw,TRNCTL,trnctl); hw_write_20kx(hw,TRNIS,0x200c01);0}

const GCTL_EAC:u32=0x00000001; const GCTL_EAI:u32=0x00000002; const GCTL_DBP:u32=0x00000020; const GCTL_TBP:u32=0x00000080; const GCTL_FBP:u32=0x00000200; const GCTL_ET:u32=0x00000800; const GCTL_AID:u32=0x00100000;
unsafe extern "C" fn hw_pll_init(hw:*mut hw,rsr:u32)->i32{let pllctl=if rsr==48000{0x1480a001}else{0x1480a731}; let mut i=0; while i<3{if hw_read_20kx(hw,PLLCTL)==pllctl{break;} hw_write_20kx(hw,PLLCTL,pllctl); msleep(40); i+=1;} if i>=3{dev_alert((*(*hw).card).dev,b"PLL initialization failed!!!\n\0".as_ptr() as *const i8);return -EBUSY;}0}
unsafe fn hw_auto_init(hw:*mut hw)->i32{let mut gctl=hw_read_20kx(hw,GCTL); set_field(&mut gctl,GCTL_EAI,0); hw_write_20kx(hw,GCTL,gctl); set_field(&mut gctl,GCTL_EAI,1); hw_write_20kx(hw,GCTL,gctl); mdelay(10); for _ in 0..400000{gctl=hw_read_20kx(hw,GCTL); if get_field(gctl,GCTL_AID)!=0{break;}} if get_field(gctl,GCTL_AID)==0{dev_alert((*(*hw).card).dev,b"Card Auto-init failed!!!\n\0".as_ptr() as *const i8);return -EBUSY;}0}
unsafe fn i2c_unlock(hw:*mut hw)->i32{if hw_read_pci(hw,0xcc)&0xff==0xaa{return 0;} hw_write_pci(hw,0xcc,0x8c); hw_write_pci(hw,0xcc,0x0e); if hw_read_pci(hw,0xcc)&0xff==0xaa{return 0;} hw_write_pci(hw,0xcc,0xee); hw_write_pci(hw,0xcc,0xaa); if hw_read_pci(hw,0xcc)&0xff==0xaa{return 0;} -1}
unsafe fn i2c_lock(hw:*mut hw){if hw_read_pci(hw,0xcc)&0xff==0xaa{hw_write_pci(hw,0xcc,0)}}
unsafe fn i2c_write(hw:*mut hw,device:u32,addr:u32,data:u32){while hw_read_pci(hw,0xEC)&0x800000==0{} hw_write_pci(hw,0xE0,device); hw_write_pci(hw,0xE4,(data<<8)|(addr&0xff));}
unsafe fn hw_reset_dac(hw:*mut hw)->i32{if i2c_unlock(hw)!=0{return -1;} while hw_read_pci(hw,0xEC)&0x800000==0{} hw_write_pci(hw,0xEC,0x05); for _ in 0..2{msleep(100); let mut gpioorg=hw_read_20kx(hw,GPIO) as u16; gpioorg&=0xfffd; hw_write_20kx(hw,GPIO,gpioorg as u32); mdelay(1); hw_write_20kx(hw,GPIO,(gpioorg|0x2) as u32);} i2c_write(hw,0x00180080,0x01,0x80); i2c_write(hw,0x00180080,0x02,0x10); i2c_lock(hw);0}
unsafe fn hw_dac_init(hw:*mut hw,info:*const dac_conf)->i32{if (*hw).model==CTSB055X{let mut gpioorg=hw_read_20kx(hw,GPIO) as u16; gpioorg&=0xffbf; gpioorg|=2; hw_write_20kx(hw,GPIO,gpioorg as u32); return 0;} let mut gpioorg=hw_read_20kx(hw,GPIO) as u16; gpioorg&=0xffbf; hw_write_20kx(hw,GPIO,gpioorg as u32); hw_reset_dac(hw); if i2c_unlock(hw)!=0{return -1;} hw_write_pci(hw,0xEC,0x05); while hw_read_pci(hw,0xEC)&0x800000==0{} let data=match (*info).msr{1=>0x24,2=>0x25,4=>0x26,_=>0x24}; for reg in [0x06,0x09,0x0c,0x0f]{i2c_write(hw,0x00180080,reg,data);} i2c_lock(hw); gpioorg=hw_read_20kx(hw,GPIO) as u16; gpioorg|=0x40; hw_write_20kx(hw,GPIO,gpioorg as u32);0}

unsafe fn is_adc_input_selected_SB055x(_hw:*mut hw,_type:ADCSRC)->i32{0}
unsafe fn is_adc_input_selected_SBx(hw:*mut hw,type_:ADCSRC)->i32{let data=hw_read_20kx(hw,GPIO); match type_{ADC_MICIN=>((data&(1<<7)!=0)&&(data&(1<<8)!=0)) as i32,ADC_LINEIN=>((data&(1<<7)==0)&&(data&(1<<8)!=0)) as i32,ADC_NONE=>(data&(1<<8)==0) as i32,_=>0}}
unsafe fn is_adc_input_selected_hendrix(hw:*mut hw,type_:ADCSRC)->i32{let data=hw_read_20kx(hw,GPIO); match type_{ADC_MICIN=>if data&(1<<7)!=0{1}else{0},ADC_LINEIN=>if data&(1<<7)!=0{0}else{1},_=>0}}
unsafe extern "C" fn hw_is_adc_input_selected(hw:*mut hw,type_:ADCSRC)->i32{match (*hw).model{CTSB055X=>is_adc_input_selected_SB055x(hw,type_),CTSB073X|CTUAA=>is_adc_input_selected_hendrix(hw,type_),_=>is_adc_input_selected_SBx(hw,type_)}}
unsafe fn adc_input_select_SB055x(hw:*mut hw,type_:ADCSRC,boost:u8)->i32{let mut data=hw_read_20kx(hw,GPIO)&0xec73; match type_{ADC_MICIN=>{data|=(1<<7)|(1<<8)|(1<<9); if boost!=0{data|=1<<2}},ADC_LINEIN=>data|=1<<8,ADC_AUX=>data|=(1<<8)|(1<<12),ADC_NONE=>data|=1<<12,_=>return -1} hw_write_20kx(hw,GPIO,data);0}
unsafe fn adc_input_select_SBx(hw:*mut hw,type_:ADCSRC,boost:u8)->i32{if i2c_unlock(hw)!=0{return -1;} while hw_read_pci(hw,0xEC)&0x800000==0{} hw_write_pci(hw,0xEC,0x05); let mut data=hw_read_20kx(hw,GPIO); let i2c_data=match type_{ADC_MICIN=>{data|=(1<<7)|(1<<8);0x1},ADC_LINEIN=>{data&=!(1<<7);data|=1<<8;0x2},ADC_NONE=>{data&=!(1<<8);0},_=>{i2c_lock(hw);return -1}}; hw_write_20kx(hw,GPIO,data); i2c_write(hw,0x001a0080,0x2a,i2c_data); let v=if boost!=0{0xe7}else{0xcf}; i2c_write(hw,0x001a0080,0x1c,v); i2c_write(hw,0x001a0080,0x1e,v); i2c_lock(hw);0}
unsafe fn adc_input_select_hendrix(hw:*mut hw,type_:ADCSRC,boost:u8)->i32{if i2c_unlock(hw)!=0{return -1;} while hw_read_pci(hw,0xEC)&0x800000==0{} hw_write_pci(hw,0xEC,0x05); let mut data=hw_read_20kx(hw,GPIO); let i2c_data=match type_{ADC_MICIN=>{data|=1<<7;0x1},ADC_LINEIN=>{data&=!(1<<7);0x2},_=>{i2c_lock(hw);return -1}}; hw_write_20kx(hw,GPIO,data); i2c_write(hw,0x001a0080,0x2a,i2c_data); let v=if boost!=0{0xe7}else{0xcf}; i2c_write(hw,0x001a0080,0x1c,v); i2c_write(hw,0x001a0080,0x1e,v); i2c_lock(hw);0}
unsafe extern "C" fn hw_adc_input_select(hw:*mut hw,type_:ADCSRC)->i32{let state=(type_==ADC_MICIN) as u8; match (*hw).model{CTSB055X=>adc_input_select_SB055x(hw,type_,state),CTSB073X|CTUAA=>adc_input_select_hendrix(hw,type_,state),_=>adc_input_select_SBx(hw,type_,state)}}
unsafe fn adc_init_SB055x(hw:*mut hw,input:i32,mic20db:i32)->i32{adc_input_select_SB055x(hw,input,mic20db as u8)}
unsafe fn adc_init_SBx(hw:*mut hw,input:i32,mic20db:i32)->i32{let mut input_source:u16=0x100; let adcdata=match input{ADC_MICIN=>{input_source=0x180;0x1},ADC_LINEIN=>0x2,ADC_VIDEO=>0x4,ADC_AUX=>0x8,ADC_NONE=>{input_source=0;0},_=>0}; if i2c_unlock(hw)!=0{return -1;} while hw_read_pci(hw,0xEC)&0x800000==0{} hw_write_pci(hw,0xEC,0x05); i2c_write(hw,0x001a0080,0x0e,0x08); i2c_write(hw,0x001a0080,0x18,0x0a); i2c_write(hw,0x001a0080,0x28,0x86); i2c_write(hw,0x001a0080,0x2a,adcdata); let v=if mic20db!=0{0xf7}else{0xcf}; i2c_write(hw,0x001a0080,0x1c,v); i2c_write(hw,0x001a0080,0x1e,v); if hw_read_20kx(hw,ID0)&0x100==0{i2c_write(hw,0x001a0080,0x16,0x26);} i2c_lock(hw); let mut gpioorg=hw_read_20kx(hw,GPIO) as u16; gpioorg&=0xfe7f; gpioorg|=input_source; hw_write_20kx(hw,GPIO,gpioorg as u32);0}
unsafe fn hw_adc_init(hw:*mut hw,info:*const adc_conf)->i32{if (*hw).model==CTSB055X{adc_init_SB055x(hw,(*info).input as i32,(*info).mic20db as i32)}else{adc_init_SBx(hw,(*info).input as i32,(*info).mic20db as i32)}}
unsafe extern "C" fn hw_capabilities(hw:*mut hw)->capabilities{capabilities{digit_io_switch:(!((*hw).model==CTSB073X||(*hw).model==CTUAA)) as i32,dedicated_mic:0,dedicated_rca:0,output_switch:0,mic_source_switch:0}}

const UAA_CFG_PWRSTATUS:i32=0x44; const UAA_CFG_SPACE_FLAG:i32=0xA0; const UAA_CORE_CHANGE:usize=0x3FFC;
unsafe fn uaa_to_xfi(pci:*mut pci_dev)->i32{let io_base=pci_resource_start(pci,0); let mem_base=ioremap(io_base,pci_resource_len(pci,0)); if mem_base.is_null(){return -ENOENT;} let CTLX=CTLBITS('C' as u32,'T' as u32,'L' as u32,'X' as u32); let CTL_=CTLBITS('C' as u32,'T' as u32,'L' as u32,'-' as u32); let CTLF=CTLBITS('C' as u32,'T' as u32,'L' as u32,'F' as u32); let CTLi=CTLBITS('C' as u32,'T' as u32,'L' as u32,'i' as u32); let CTLA=CTLBITS('C' as u32,'T' as u32,'L' as u32,'A' as u32); let CTLZ=CTLBITS('C' as u32,'T' as u32,'L' as u32,'Z' as u32); let CTLL=CTLBITS('C' as u32,'T' as u32,'L' as u32,'L' as u32); let addr=(mem_base as *mut u8).add(UAA_CORE_CHANGE) as *mut c_void; let mut data=[0u32;4]; for i in 0..4{data[i]=readl(addr);} let is_uaa=if data[0]==CTLA{(data[1]==CTLZ&&data[2]==CTLL&&data[3]==CTLA)||(data[1]==CTLA&&data[2]==CTLZ&&data[3]==CTLL)}else if data[0]==CTLZ{data[1]==CTLL&&data[2]==CTLA&&data[3]==CTLA}else if data[0]==CTLL{data[1]==CTLA&&data[2]==CTLA&&data[3]==CTLZ}else{false}; if !is_uaa{iounmap(mem_base);return 0;} let(mut bar0,mut bar1,mut bar2,mut bar3,mut bar4,mut bar5,mut cmd,mut irq,mut cl_size,mut l_timer,mut pwr)=(0,0,0,0,0,0,0,0,0,0,0); pci_read_config_dword(pci,PCI_BASE_ADDRESS_0,&mut bar0); pci_read_config_dword(pci,PCI_BASE_ADDRESS_1,&mut bar1); pci_read_config_dword(pci,PCI_BASE_ADDRESS_2,&mut bar2); pci_read_config_dword(pci,PCI_BASE_ADDRESS_3,&mut bar3); pci_read_config_dword(pci,PCI_BASE_ADDRESS_4,&mut bar4); pci_read_config_dword(pci,PCI_BASE_ADDRESS_5,&mut bar5); pci_read_config_dword(pci,PCI_INTERRUPT_LINE,&mut irq); pci_read_config_dword(pci,PCI_CACHE_LINE_SIZE,&mut cl_size); pci_read_config_dword(pci,PCI_LATENCY_TIMER,&mut l_timer); pci_read_config_dword(pci,UAA_CFG_PWRSTATUS,&mut pwr); pci_read_config_dword(pci,PCI_COMMAND,&mut cmd); pci_write_config_dword(pci,UAA_CFG_SPACE_FLAG,0x87654321); pci_write_config_dword(pci,PCI_BASE_ADDRESS_0,bar5); pci_write_config_dword(pci,UAA_CFG_SPACE_FLAG,0x12345678); for (w,v) in [(PCI_BASE_ADDRESS_1,bar1),(PCI_BASE_ADDRESS_2,bar2),(PCI_BASE_ADDRESS_3,bar3),(PCI_BASE_ADDRESS_4,bar4),(PCI_INTERRUPT_LINE,irq),(PCI_CACHE_LINE_SIZE,cl_size),(PCI_LATENCY_TIMER,l_timer),(UAA_CFG_PWRSTATUS,pwr),(PCI_COMMAND,cmd)]{pci_write_config_dword(pci,w,v);} writel(CTLX,addr); writel(CTL_,addr); writel(CTLF,addr); writel(CTLi,addr); iounmap(mem_base);0}

unsafe extern "C" fn ct_20k1_interrupt(_irq:i32,dev_id:*mut c_void)->irqreturn_t{let hw=dev_id as *mut hw; let status=hw_read_20kx(hw,GIP); if status==0{return IRQ_NONE;} if let Some(cb)=(*hw).irq_callback{cb((*hw).irq_callback_data,status);} hw_write_20kx(hw,GIP,status); IRQ_HANDLED}
unsafe fn hw_card_start(hw:*mut hw)->i32{let pci=(*hw).pci; let mut err=pci_enable_device(pci); if err<0{return err;} if dma_set_mask_and_coherent(&mut (*pci).dev,DMA_BIT_MASK(BITS_PER_LONG))!=0{dma_set_mask_and_coherent(&mut (*pci).dev,DMA_BIT_MASK(32));} if (*hw).io_base==0{err=pci_request_regions(pci,b"XFi\0".as_ptr() as *const i8); if err<0{pci_disable_device(pci);return err;} (*hw).io_base=if (*hw).model==CTUAA{pci_resource_start(pci,5)}else{pci_resource_start(pci,0)};} if (*hw).model==CTUAA{err=uaa_to_xfi(pci); if err!=0{pci_release_regions(pci);(*hw).io_base=0;pci_disable_device(pci);return err;}} if (*hw).irq<0{err=request_irq((*pci).irq,ct_20k1_interrupt,IRQF_SHARED,KBUILD_MODNAME,hw as *mut c_void); if err<0{dev_err((*(*hw).card).dev,b"XFi: Cannot get irq %d\n\0".as_ptr() as *const i8,(*pci).irq); pci_release_regions(pci);(*hw).io_base=0;pci_disable_device(pci);return err;} (*hw).irq=(*pci).irq; (*(*hw).card).sync_irq=(*hw).irq;} pci_set_master(pci);0}
unsafe extern "C" fn hw_card_stop(hw:*mut hw)->i32{hw_write_20kx(hw,TRNCTL,0); let data=hw_read_20kx(hw,PLLCTL); hw_write_20kx(hw,PLLCTL,data&!(0x0f<<12));0}
unsafe fn hw_card_shutdown(hw:*mut hw)->i32{if (*hw).irq>=0{free_irq((*hw).irq,hw as *mut c_void);} (*hw).irq=-1; iounmap((*hw).mem_base); (*hw).mem_base=ptr::null_mut(); if (*hw).io_base!=0{pci_release_regions((*hw).pci);} (*hw).io_base=0; pci_disable_device((*hw).pci);0}
unsafe extern "C" fn hw_card_init(hw:*mut hw,info:*mut card_conf)->i32{let mut err=hw_card_start(hw); if err!=0{return err;} err=hw_pll_init(hw,(*info).rsr); if err<0{return err;} err=hw_auto_init(hw); if err<0{return err;} let mut gctl=hw_read_20kx(hw,GCTL); set_field(&mut gctl,GCTL_EAC,1); set_field(&mut gctl,GCTL_DBP,1); set_field(&mut gctl,GCTL_TBP,1); set_field(&mut gctl,GCTL_FBP,1); set_field(&mut gctl,GCTL_ET,1); hw_write_20kx(hw,GCTL,gctl); mdelay(10); hw_write_20kx(hw,GIE,0); hw_write_20kx(hw,SRCIP,0); msleep(30); match (*hw).model{CTSB055X=>hw_write_20kx(hw,GPIOCTL,0x13fe),CTSB073X=>hw_write_20kx(hw,GPIOCTL,0x00e6),CTUAA=>hw_write_20kx(hw,GPIOCTL,0x00c2),_=>hw_write_20kx(hw,GPIOCTL,0x01e6)} let trn_info=trn_conf{vm_pgt_phys:(*info).vm_pgt_phys}; err=hw_trn_init(hw,&trn_info); if err<0{return err;} let daio_info=daio_conf{msr:(*info).msr}; err=hw_daio_init(hw,&daio_info); if err<0{return err;} let dac_info=dac_conf{msr:(*info).msr}; err=hw_dac_init(hw,&dac_info); if err<0{return err;} let adc_info=adc_conf{msr:(*info).msr,input:ADC_LINEIN as u8,mic20db:0}; err=hw_adc_init(hw,&adc_info); if err<0{return err;} let mut data=hw_read_20kx(hw,SRCMCTL); data|=1; hw_write_20kx(hw,SRCMCTL,data);0}
/* CONFIG_PM_SLEEP translated intent:
unsafe extern "C" fn hw_suspend(hw:*mut hw)->i32 { hw_card_stop(hw); if (*hw).model==CTUAA { pci_write_config_dword((*hw).pci,UAA_CFG_SPACE_FLAG,0); } 0 }
unsafe extern "C" fn hw_resume(hw:*mut hw,info:*mut card_conf)->i32 { hw_card_init(hw,info) }
*/

unsafe extern "C" fn hw_read_20kx(hw:*mut hw,reg:u32)->u32{outl(reg,(*hw).io_base+0x0); inl((*hw).io_base+0x4)}
unsafe extern "C" fn hw_write_20kx(hw:*mut hw,reg:u32,data:u32){outl(reg,(*hw).io_base+0x0); outl(data,(*hw).io_base+0x4)}
unsafe extern "C" fn hw_read_pci(hw:*mut hw,reg:u32)->u32{outl(reg,(*hw).io_base+0x10); inl((*hw).io_base+0x14)}
unsafe extern "C" fn hw_write_pci(hw:*mut hw,reg:u32,data:u32){outl(reg,(*hw).io_base+0x10); outl(data,(*hw).io_base+0x14)}

pub static mut ct20k1_preset: hw = hw {
    irq: -1, card: ptr::null_mut(), pci: ptr::null_mut(), io_base: 0, mem_base: ptr::null_mut(), model: 0, irq_callback: None, irq_callback_data: ptr::null_mut(),
    card_init: Some(hw_card_init), card_stop: Some(hw_card_stop), pll_init: Some(hw_pll_init), is_adc_source_selected: Some(hw_is_adc_input_selected), select_adc_source: Some(hw_adc_input_select), capabilities: Some(hw_capabilities),
    src_rsc_get_ctrl_blk: Some(src_get_rsc_ctrl_blk), src_rsc_put_ctrl_blk: Some(src_put_rsc_ctrl_blk), src_mgr_get_ctrl_blk: Some(src_mgr_get_ctrl_blk), src_mgr_put_ctrl_blk: Some(src_mgr_put_ctrl_blk),
    src_set_state: Some(src_set_state), src_set_bm: Some(src_set_bm), src_set_rsr: Some(src_set_rsr), src_set_sf: Some(src_set_sf), src_set_wr: Some(src_set_wr), src_set_pm: Some(src_set_pm), src_set_rom: Some(src_set_rom), src_set_vo: Some(src_set_vo), src_set_st: Some(src_set_st), src_set_ie: Some(src_set_ie), src_set_ilsz: Some(src_set_ilsz), src_set_bp: Some(src_set_bp), src_set_cisz: Some(src_set_cisz), src_set_ca: Some(src_set_ca), src_set_sa: Some(src_set_sa), src_set_la: Some(src_set_la), src_set_pitch: Some(src_set_pitch), src_set_dirty: Some(src_set_dirty), src_set_clear_zbufs: Some(src_set_clear_zbufs), src_set_dirty_all: Some(src_set_dirty_all), src_commit_write: Some(src_commit_write), src_get_ca: Some(src_get_ca), src_get_dirty: Some(src_get_dirty), src_dirty_conj_mask: Some(src_dirty_conj_mask), src_mgr_enbs_src: Some(src_mgr_enbs_src), src_mgr_enb_src: Some(src_mgr_enb_src), src_mgr_dsb_src: Some(src_mgr_dsb_src), src_mgr_commit_write: Some(src_mgr_commit_write),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_20k1_hw_obj(rhw:*mut *mut hw)->i32{*rhw=ptr::null_mut(); let hw20k1=alloc_zeroed::<hw20k1>(); if hw20k1.is_null(){return -ENOMEM;} spin_lock_init(&mut (*hw20k1).reg_20k1_lock); spin_lock_init(&mut (*hw20k1).reg_pci_lock); (*hw20k1).hw=ct20k1_preset; *rhw=&mut (*hw20k1).hw;0}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_20k1_hw_obj(hw:*mut hw)->i32{if (*hw).io_base!=0{hw_card_shutdown(hw);} let base=hw as *mut hw20k1; kfree(base as *mut c_void);0}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
