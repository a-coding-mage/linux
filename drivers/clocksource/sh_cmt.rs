// SPDX-License-Identifier: GPL-2.0
/* SuperH Timer Support - CMT; Copyright (C) 2008 Magnus Damm */

// Kernel includes and symbols are supplied by the surrounding Rust kernel bindings.

#[repr(C)]
pub enum ShCmtModel { ShCmt16Bit, ShCmt32Bit, ShCmt48Bit, ShCmt0RcarGen2, ShCmt1RcarGen2 }

#[repr(C)]
pub struct ShCmtInfo {
    pub model: ShCmtModel, pub channels_mask: u32, pub width: usize,
    pub overflow_bit: u32, pub clear_bits: u32,
    pub read_control: unsafe extern "C" fn(*mut u8, usize) -> u32,
    pub write_control: unsafe extern "C" fn(*mut u8, usize, u32),
    pub read_count: unsafe extern "C" fn(*mut u8, usize) -> u32,
    pub write_count: unsafe extern "C" fn(*mut u8, usize, u32),
}

#[repr(C)] pub struct RawSpinLock { _private: [u8; 0] }
#[repr(C)] pub struct PlatformDevice { _private: [u8; 0] }
#[repr(C)] pub struct Clock { _private: [u8; 0] }
#[repr(C)] pub struct ClockEventDevice { _private: [u8; 0] }
#[repr(C)] pub struct ClockSource { _private: [u8; 0] }

#[repr(C)]
pub struct ShCmtChannel {
    pub cmt: *mut ShCmtDevice, pub index: u32, pub hwidx: u32,
    pub iostart: *mut u8, pub ioctrl: *mut u8, pub timer_bit: u32,
    pub flags: usize, pub match_value: u32, pub next_match_value: u32,
    pub max_match_value: u32, pub lock: RawSpinLock, pub ced: ClockEventDevice,
    pub cs: ClockSource, pub total_cycles: u64, pub cs_enabled: bool,
}
#[repr(C)]
pub struct ShCmtDevice {
    pub pdev: *mut PlatformDevice, pub info: *const ShCmtInfo, pub mapbase: *mut u8,
    pub clk: *mut Clock, pub rate: usize, pub reg_delay: u32, pub lock: RawSpinLock,
    pub channels: *mut ShCmtChannel, pub num_channels: u32, pub hw_channels: u32,
    pub has_clockevent: bool, pub has_clocksource: bool,
}

pub const SH_CMT16_CMCSR_CMF: u32=1<<7; pub const SH_CMT16_CMCSR_CMIE:u32=1<<6;
pub const SH_CMT16_CMCSR_CKS512:u32=3; pub const SH_CMT32_CMCSR_CMF:u32=1<<15;
pub const SH_CMT32_CMCSR_OVF:u32=1<<14; pub const SH_CMT32_CMCSR_WRFLG:u32=1<<13;
pub const SH_CMT32_CMCSR_CMTOUT_IE:u32=1<<7; pub const SH_CMT32_CMCSR_CMS:u32=1<<9;
pub const SH_CMT32_CMCSR_CMM:u32=1<<8; pub const SH_CMT32_CMCSR_CMR_IRQ:u32=2<<4;
pub const SH_CMT32_CMCSR_CKS_RCLK8:u32=4; pub const CMCSR:usize=0; pub const CMCNT:usize=1;
pub const CMCOR:usize=2; pub const CMCLKE:usize=0x1000;
pub const FLAG_CLOCKEVENT:usize=1; pub const FLAG_CLOCKSOURCE:usize=2;
pub const FLAG_REPROGRAM:usize=4; pub const FLAG_SKIPEVENT:usize=8;
pub const FLAG_IRQCONTEXT:usize=16;

extern "C" {
    fn ioread16(p:*mut u8)->u32; fn ioread32(p:*mut u8)->u32;
    fn iowrite16(v:u32,p:*mut u8); fn iowrite32(v:u32,p:*mut u8);
    fn udelay(v:u32); fn raw_spin_lock_irqsave(l:*mut RawSpinLock,f:*mut usize);
    fn raw_spin_unlock_irqrestore(l:*mut RawSpinLock,f:usize); fn raw_spin_lock_init(l:*mut RawSpinLock);
    fn dev_warn(p:*mut u8,s:*const u8,...); fn dev_err(p:*mut u8,s:*const u8,...);
    fn clockevent_state_oneshot(c:*mut ClockEventDevice)->bool; fn clockevent_state_shutdown(c:*mut ClockEventDevice)->bool;
    fn dev_pm_syscore_device(p:*mut u8,v:bool); fn clk_get_rate(c:*mut Clock)->usize;
}

unsafe extern "C" fn sh_cmt_read16(base:*mut u8, offs:usize)->u32 { ioread16(base.add(offs<<1)) }
unsafe extern "C" fn sh_cmt_read32(base:*mut u8, offs:usize)->u32 { ioread32(base.add(offs<<2)) }
unsafe extern "C" fn sh_cmt_write16(base:*mut u8, offs:usize, value:u32) { iowrite16(value,base.add(offs<<1)); }
unsafe extern "C" fn sh_cmt_write32(base:*mut u8, offs:usize, value:u32) { iowrite32(value,base.add(offs<<2)); }

#[inline] unsafe fn read_cmstr(ch:*mut ShCmtChannel)->u32 { let c=&*ch; let d=&*c.cmt; ((*d.info).read_control)(if !c.iostart.is_null(){c.iostart}else{d.mapbase},0) }
#[inline] unsafe fn write_cmstr(ch:*mut ShCmtChannel,v:u32) { let old=read_cmstr(ch); if v!=old { let c=&*ch; let d=&*c.cmt; ((*d.info).write_control)(if !c.iostart.is_null(){c.iostart}else{d.mapbase},0,v); udelay(d.reg_delay); } }
#[inline] unsafe fn read_cmcsr(ch:*mut ShCmtChannel)->u32 { let c=&*ch; ((*(*c.cmt).info).read_control)(c.ioctrl,CMCSR) }
#[inline] unsafe fn write_cmcsr(ch:*mut ShCmtChannel,v:u32) { if v!=read_cmcsr(ch) { let c=&*ch; ((*(*c.cmt).info).write_control)(c.ioctrl,CMCSR,v); udelay((*c.cmt).reg_delay); } }
#[inline] unsafe fn read_cmcnt(ch:*mut ShCmtChannel)->u32 { let c=&*ch; ((*(*c.cmt).info).read_count)(c.ioctrl,CMCNT) }
#[inline] unsafe fn write_cmcor(ch:*mut ShCmtChannel,v:u32) { let c=&*ch; if v!=((*(*c.cmt).info).read_count)(c.ioctrl,CMCOR) { ((*(*c.cmt).info).write_count)(c.ioctrl,CMCOR,v); udelay((*c.cmt).reg_delay); } }

unsafe fn get_counter(ch:*mut ShCmtChannel, wrapped:*mut u32)->u32 {
    let d=&*(*ch).cmt; let mut o1=read_cmcsr(ch)&(*d.info).overflow_bit; let (mut v1,mut v2,mut v3,mut o2);
    loop { o2=o1; v1=read_cmcnt(ch); v2=read_cmcnt(ch); v3=read_cmcnt(ch); o1=read_cmcsr(ch)&(*d.info).overflow_bit;
        if !((o1!=o2)||(v1>v2&&v1<v3)||(v2>v3&&v2<v1)||(v3>v1&&v3<v2)){break;} }
    *wrapped=o1; v2
}
unsafe fn start_stop(ch:*mut ShCmtChannel,start:bool) { let c=&*ch; let d=&*c.cmt; let mut f=0; raw_spin_lock_irqsave(&d.lock,&mut f); let mut v=read_cmstr(ch); if start {v|=1<<c.timer_bit}else{v&=!(1<<c.timer_bit)} write_cmstr(ch,v); raw_spin_unlock_irqrestore(&d.lock,f); }
unsafe fn enable(ch:*mut ShCmtChannel)->i32 { let c=&*ch; let d=&*c.cmt; dev_pm_syscore_device(core::ptr::null_mut(),true); start_stop(ch,false); if (*d.info).width==16 {write_cmcsr(ch,SH_CMT16_CMCSR_CMIE|SH_CMT16_CMCSR_CKS512)} else {write_cmcsr(ch,SH_CMT32_CMCSR_CMM|SH_CMT32_CMCSR_CMR_IRQ|SH_CMT32_CMCSR_CKS_RCLK8)} write_cmcor(ch,!0); let _=read_cmcnt(ch); start_stop(ch,true); 0 }
unsafe fn disable(ch:*mut ShCmtChannel) { start_stop(ch,false); write_cmcsr(ch,0); dev_pm_syscore_device(core::ptr::null_mut(),false); }

unsafe fn program_verify(ch:*mut ShCmtChannel,absolute:bool) { let c=&mut *ch; let mut w=0; let mut now=get_counter(ch,&mut w); c.flags|=FLAG_REPROGRAM; if w!=0 {c.flags|=FLAG_SKIPEVENT;return} if absolute {now=0} let mut delay=0; loop { let mut n=now.wrapping_add(c.next_match_value).wrapping_add(delay); if n>c.max_match_value{n=c.max_match_value} write_cmcor(ch,n); now=get_counter(ch,&mut w); if w!=0&&n>c.match_value {c.flags|=FLAG_SKIPEVENT;break} if w!=0||now<n {c.match_value=n;break} delay=if delay!=0{delay<<1}else{1}; if delay==0{break} } }
unsafe fn set_next(ch:*mut ShCmtChannel,delta:usize) { let c=&mut *ch; c.next_match_value=delta as u32; program_verify(ch,false); }

unsafe fn interrupt(ch:*mut ShCmtChannel) { let c=&mut *ch; write_cmcsr(ch,read_cmcsr(ch)&!SH_CMT32_CMCSR_CMF); if c.flags&FLAG_CLOCKSOURCE!=0 {c.total_cycles+=c.match_value as u64+1;} if c.flags&FLAG_REPROGRAM==0 {c.next_match_value=c.max_match_value;} c.flags|=FLAG_IRQCONTEXT; c.flags&=!FLAG_SKIPEVENT; if c.flags&FLAG_REPROGRAM!=0 {c.flags&=!FLAG_REPROGRAM;program_verify(ch,true)} c.flags&=!FLAG_IRQCONTEXT; }

unsafe fn start_clocksource(ch:*mut ShCmtChannel)->i32 { let c=&mut *ch; let mut r=0; if c.flags&(FLAG_CLOCKEVENT|FLAG_CLOCKSOURCE)==0 {r=enable(ch)} if r==0 {c.flags|=FLAG_CLOCKSOURCE;} r }
unsafe fn stop_clocksource(ch:*mut ShCmtChannel) { let c=&mut *ch; let f=c.flags&(FLAG_CLOCKEVENT|FLAG_CLOCKSOURCE); c.flags&=!FLAG_CLOCKSOURCE; if f!=0&&c.flags&(FLAG_CLOCKEVENT|FLAG_CLOCKSOURCE)==0{disable(ch)} }
unsafe fn start_clockevent(ch:*mut ShCmtChannel)->i32 { let c=&mut *ch; let r=if c.flags&(FLAG_CLOCKEVENT|FLAG_CLOCKSOURCE)==0{enable(ch)}else{0}; if r==0{c.flags|=FLAG_CLOCKEVENT} r }
unsafe fn stop_clockevent(ch:*mut ShCmtChannel) { let c=&mut *ch; let f=c.flags&(FLAG_CLOCKEVENT|FLAG_CLOCKSOURCE); c.flags&=!FLAG_CLOCKEVENT; if f!=0&&c.flags&(FLAG_CLOCKEVENT|FLAG_CLOCKSOURCE)==0{disable(ch)} if c.flags&FLAG_CLOCKSOURCE!=0{set_next(ch,c.max_match_value as usize)} }

#[repr(C)] pub struct PlatformDeviceId { pub name:*const u8, pub driver_data:usize }
#[repr(C)] pub struct OfDeviceId { pub compatible:*const u8, pub data:*const ShCmtInfo }
pub static SH_CMT_ID_TABLE:[PlatformDeviceId;3]=[
    PlatformDeviceId{name:b"sh-cmt-16\0".as_ptr(),driver_data:0},
    PlatformDeviceId{name:b"sh-cmt-32\0".as_ptr(),driver_data:1},
    PlatformDeviceId{name:core::ptr::null(),driver_data:0}];
pub static SH_CMT_OF_TABLE:[OfDeviceId;11]=[
    OfDeviceId{compatible:b"renesas,cmt-48\0".as_ptr(),data:core::ptr::null()},
    OfDeviceId{compatible:b"renesas,cmt-48-gen2\0".as_ptr(),data:core::ptr::null()},
    OfDeviceId{compatible:b"renesas,r8a7740-cmt1\0".as_ptr(),data:core::ptr::null()},
    OfDeviceId{compatible:b"renesas,sh73a0-cmt1\0".as_ptr(),data:core::ptr::null()},
    OfDeviceId{compatible:b"renesas,rcar-gen2-cmt0\0".as_ptr(),data:core::ptr::null()},
    OfDeviceId{compatible:b"renesas,rcar-gen2-cmt1\0".as_ptr(),data:core::ptr::null()},
    OfDeviceId{compatible:b"renesas,rcar-gen3-cmt0\0".as_ptr(),data:core::ptr::null()},
    OfDeviceId{compatible:b"renesas,rcar-gen3-cmt1\0".as_ptr(),data:core::ptr::null()},
    OfDeviceId{compatible:b"renesas,rcar-gen4-cmt0\0".as_ptr(),data:core::ptr::null()},
    OfDeviceId{compatible:b"renesas,rcar-gen4-cmt1\0".as_ptr(),data:core::ptr::null()},
    OfDeviceId{compatible:core::ptr::null(),data:core::ptr::null()}];

// Kernel platform setup/probe, registration callbacks, and module init/exit are represented
// as declarations because their implementations depend on external kernel types and APIs.
extern "C" { fn sh_cmt_setup(cmt:*mut ShCmtDevice,pdev:*mut PlatformDevice)->i32; fn sh_cmt_probe(pdev:*mut PlatformDevice)->i32; fn sh_cmt_init()->i32; fn sh_cmt_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
