// SPDX-License-Identifier: GPL-2.0-or-later
/* Freescale General-purpose Timers Module */

// Linux kernel dependencies and asm/fsl_gtm.h are supplied by the surrounding
// translation unit.

const fn gtcfr_stp(x: i32) -> u8 { if x & 1 != 0 { 1 << 5 } else { 1 << 1 } }
const fn gtcfr_rst(x: i32) -> u8 { if x & 1 != 0 { 1 << 4 } else { 1 } }
const GTMDR_ICLK_MASK: u16 = 3 << 1;
const GTMDR_ICLK_ICAS: u16 = 0 << 1;
const GTMDR_ICLK_ICLK: u16 = 1 << 1;
const GTMDR_ICLK_SLGO: u16 = 2 << 1;
const GTMDR_FRR: u16 = 1 << 3;
const GTMDR_ORI: u16 = 1 << 4;
const fn gtmdr_sps(x: u8) -> u16 { (x as u16) << 8 }

#[repr(C, packed)]
pub struct GtmTimersRegs {
    pub gtcfr1: u8, pub res0: [u8; 3], pub gtcfr2: u8, pub res1: [u8; 0xb],
    pub gtmdr1: u16, pub gtmdr2: u16, pub gtrfr1: u16, pub gtrfr2: u16,
    pub gtcpr1: u16, pub gtcpr2: u16, pub gtcnr1: u16, pub gtcnr2: u16,
    pub gtmdr3: u16, pub gtmdr4: u16, pub gtrfr3: u16, pub gtrfr4: u16,
    pub gtcpr3: u16, pub gtcpr4: u16, pub gtcnr3: u16, pub gtcnr4: u16,
    pub gtevr1: u16, pub gtevr2: u16, pub gtevr3: u16, pub gtevr4: u16,
    pub gtpsr1: u16, pub gtpsr2: u16, pub gtpsr3: u16, pub gtpsr4: u16,
    pub res2: [u8; 0x40],
}

#[repr(C)]
pub struct Gtm {
    pub clock: u32,
    pub regs: *mut GtmTimersRegs,
    pub timers: [GtmTimer; 4],
    pub lock: Spinlock,
    pub list_node: ListHead,
}

#[repr(C)]
pub struct GtmTimer {
    pub requested: bool, pub irq: u32, pub gtm: *mut Gtm,
    pub gtcfr: *mut u8, pub gtmdr: *mut u16, pub gtcnr: *mut u16,
    pub gtrfr: *mut u16, pub gtevr: *mut u16, pub gtpsr: *mut u16,
}
#[repr(C)] pub struct Spinlock { _opaque: [u8; 0] }
#[repr(C)] pub struct ListHead { _opaque: [u8; 0] }

extern "C" {
    static mut gtms: ListHead;
    fn spin_lock_irq(lock: *mut Spinlock); fn spin_unlock_irq(lock: *mut Spinlock);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn out_be16(p: *mut u16, v: u16); fn setbits8(p: *mut u8, v: u8);
    fn clrbits8(p: *mut u8, v: u8);
    fn clrsetbits_8(p: *mut u8, clear: u8, set: u8);
    fn clrsetbits_be16(p: *mut u16, clear: u16, set: u16);
    fn of_device_is_compatible(np: *mut DeviceNode, s: *const i8) -> bool;
    fn of_get_property(np: *mut DeviceNode, s: *const i8, size: *mut i32) -> *const u32;
    fn irq_of_parse_and_map(np: *mut DeviceNode, i: i32) -> u32;
    fn of_iomap(np: *mut DeviceNode, i: i32) -> *mut GtmTimersRegs;
    fn spin_lock_init(lock: *mut Spinlock); fn kfree(p: *mut Gtm);
}
#[repr(C)] pub struct DeviceNode { pub data: *mut core::ffi::c_void }

pub unsafe fn gtm_get_specific_timer16(gtm: *mut Gtm, timer: u32) -> *mut GtmTimer {
    if timer > 3 { return core::ptr::null_mut(); }
    spin_lock_irq(&mut (*gtm).lock); let ret;
    if (*gtm).timers[timer as usize].requested { ret = core::ptr::null_mut(); }
    else { (*gtm).timers[timer as usize].requested = true; ret = &mut (*gtm).timers[timer as usize]; }
    spin_unlock_irq(&mut (*gtm).lock); ret
}

pub unsafe fn gtm_put_timer16(tmr: *mut GtmTimer) { gtm_stop_timer16(tmr); let g = (*tmr).gtm; spin_lock_irq(&mut (*g).lock); (*tmr).requested = false; spin_unlock_irq(&mut (*g).lock); }

unsafe fn gtm_set_ref_timer16(tmr: *mut GtmTimer, frequency: i32, reference_value: i32, free_run: bool) -> i32 {
    let gtm = (*tmr).gtm; let num = tmr.offset_from((*gtm).timers.as_mut_ptr()) as i32;
    let mut prescaler = (*gtm).clock / frequency as u32; let mut iclk = GTMDR_ICLK_ICLK;
    let max = if (*tmr).gtpsr.is_null() { 256*256*16/256 } else { 256*256*16 };
    if prescaler > max { return -22; }
    if prescaler > max/16 { iclk=GTMDR_ICLK_SLGO; prescaler/=16; }
    let (psr,sps) = if prescaler <= 256 { (0, prescaler-1) } else { (255, prescaler/256-1) };
    let mut flags=0; spin_lock_irqsave(&mut (*gtm).lock,&mut flags);
    clrsetbits_8((*tmr).gtcfr, !(gtcfr_stp(num)|gtcfr_rst(num)), gtcfr_stp(num)|gtcfr_rst(num)); setbits8((*tmr).gtcfr,gtcfr_stp(num));
    if !(*tmr).gtpsr.is_null() { out_be16((*tmr).gtpsr,psr as u16); }
    clrsetbits_be16((*tmr).gtmdr,0xffff,iclk|gtmdr_sps(sps as u8)|GTMDR_ORI|if free_run{GTMDR_FRR}else{0}); out_be16((*tmr).gtcnr,0); out_be16((*tmr).gtrfr,reference_value as u16); out_be16((*tmr).gtevr,0xffff); clrbits8((*tmr).gtcfr,gtcfr_stp(num)); spin_unlock_irqrestore(&mut (*gtm).lock,flags); 0
}

pub unsafe fn gtm_set_timer16(tmr:*mut GtmTimer,usec:usize,reload:bool)->i32 { let mut freq=1_000_000i32; let bit=(usize::BITS-usec.leading_zeros()) as usize; let mut u=usec; if bit>15 { freq >>= bit-15; u >>= bit-15; } if freq==0{-22}else{gtm_set_ref_timer16(tmr,freq,u as i32,reload)} }
pub unsafe fn gtm_set_exact_timer16(tmr:*mut GtmTimer,usec:u16,reload:bool)->i32 { gtm_set_ref_timer16(tmr,1_000_000,usec as i32,reload) }
pub unsafe fn gtm_stop_timer16(tmr:*mut GtmTimer) { let g=(*tmr).gtm; let mut flags=0; spin_lock_irqsave(&mut (*g).lock,&mut flags); setbits8((*tmr).gtcfr,gtcfr_stp(tmr.offset_from((*g).timers.as_mut_ptr()) as i32)); out_be16((*tmr).gtevr,0xffff); spin_unlock_irqrestore(&mut (*g).lock,flags); }
pub unsafe fn gtm_ack_timer16(tmr:*mut GtmTimer,events:u16) { out_be16((*tmr).gtevr,events); }

unsafe fn gtm_set_shortcuts(np:*mut DeviceNode,timers:*mut GtmTimer,regs:*mut GtmTimersRegs) {
    let r=&mut *regs; let t=&mut *timers;
    t[0].gtcfr=&mut r.gtcfr1; t[0].gtmdr=&mut r.gtmdr1; t[0].gtcnr=&mut r.gtcnr1; t[0].gtrfr=&mut r.gtrfr1; t[0].gtevr=&mut r.gtevr1;
    t[1].gtcfr=&mut r.gtcfr1; t[1].gtmdr=&mut r.gtmdr2; t[1].gtcnr=&mut r.gtcnr2; t[1].gtrfr=&mut r.gtrfr2; t[1].gtevr=&mut r.gtevr2;
    t[2].gtcfr=&mut r.gtcfr2; t[2].gtmdr=&mut r.gtmdr3; t[2].gtcnr=&mut r.gtcnr3; t[2].gtrfr=&mut r.gtrfr3; t[2].gtevr=&mut r.gtevr3;
    t[3].gtcfr=&mut r.gtcfr2; t[3].gtmdr=&mut r.gtmdr4; t[3].gtcnr=&mut r.gtcnr4; t[3].gtrfr=&mut r.gtrfr4; t[3].gtevr=&mut r.gtevr4;
    if !of_device_is_compatible(np,b"fsl,cpm2-gtm\0".as_ptr() as *const i8) { t[0].gtpsr=&mut r.gtpsr1; t[1].gtpsr=&mut r.gtpsr2; t[2].gtpsr=&mut r.gtpsr3; t[3].gtpsr=&mut r.gtpsr4; }
}

#[no_mangle] pub unsafe extern "C" fn fsl_gtm_init() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
