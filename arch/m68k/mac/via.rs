// SPDX-License-Identifier: GPL-2.0
/*
 * 6522 Versatile Interface Adapter (VIA)
 *
 * There are two of these on the Mac II. Some IRQs are vectored
 * via them as are assorted bits and bobs - eg RTC, ADB.
 *
 * PRAM/RTC access algorithms are from the NetBSD RTC toolkit version 1.08b
 * by Erik Vogan and adapted to Linux by Joshua M. Thompson (funaho@jurai.org)
 */

// Linux and architecture headers supply the symbols referenced below.

extern "C" {
    static mut oss_present: bool;
    static mut macintosh_config: *mut MacintoshConfig;
    static mut VIA1_BASE: usize;
    static mut VIA2_BASE: usize;
    static mut RBV_BASE: usize;

    fn pr_debug(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn printk(fmt: *const u8, ...);
    fn panic(fmt: *const u8, ...) -> !;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn generic_handle_irq(irq: i32);
    fn irq_set_chained_handler(irq: i32, handler: unsafe extern "C" fn(*mut irq_desc));
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t, flags: u32, name: *const u8, dev_id: *mut core::ffi::c_void) -> i32;
    fn legacy_timer_tick(n: i32);
    fn clocksource_register_hz(cs: *mut clocksource, hz: u32) -> i32;
}

#[repr(C)] pub struct MacintoshConfig { pub via_type: i32, pub ident: i32, pub adb_type: i32 }
#[repr(C)] pub struct irq_desc { _private: [u8; 0] }
#[repr(C)] pub struct clocksource { pub name: *const u8, pub rating: i32, pub read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>, pub mask: u64, pub flags: u32 }
type irqreturn_t = i32;

extern "C" { static mut via1: *mut u8; }
#[no_mangle] pub static mut via2: *mut u8 = core::ptr::null_mut();
#[no_mangle] pub static mut rbv_present: i32 = 0;
#[no_mangle] pub static mut via_alt_mapping: i32 = 0;
static mut rbv_clear: u8 = 0;
static mut gIER: usize = 0; static mut gIFR: usize = 0; static mut gBufA: usize = 0; static mut gBufB: usize = 0;
static mut nubus_disabled: u8 = 0;

#[no_mangle] pub unsafe extern "C" fn via_debug_dump(){printk(b"VIA1: DDRA = 0x%02X DDRB = 0x%02X ACR = 0x%02X\n\0".as_ptr(),*via1.add(vDirA) as u32,*via1.add(vDirB) as u32,*via1.add(vACR) as u32);printk(b"         PCR = 0x%02X  IFR = 0x%02X IER = 0x%02X\n\0".as_ptr(),*via1.add(vPCR) as u32,*via1.add(vIFR) as u32,*via1.add(vIER) as u32);if via2.is_null(){return;}if rbv_present!=0{printk(b"VIA2:  IFR = 0x%02X  IER = 0x%02X\n\0".as_ptr(),*via2.add(rIFR) as u32,*via2.add(rIER) as u32);printk(b"      SIFR = 0x%02X SIER = 0x%02X\n\0".as_ptr(),*via2.add(rSIFR) as u32,*via2.add(rSIER) as u32);}else{printk(b"VIA2: DDRA = 0x%02X DDRB = 0x%02X ACR = 0x%02X\n\0".as_ptr(),*via2.add(vDirA) as u32,*via2.add(vDirB) as u32,*via2.add(vACR) as u32);printk(b"         PCR = 0x%02X  IFR = 0x%02X IER = 0x%02X\n\0".as_ptr(),*via2.add(vPCR) as u32,*via2.add(vIFR) as u32,*via2.add(vIER) as u32);}}

// Register offsets and platform constants are supplied by asm/mac_via.h and related headers.
extern "C" {
    static rIER: usize; static rIFR: usize; static rSIFR: usize; static rBufB: usize;
    static vIER: usize; static vIFR: usize; static vBufA: usize; static vBufB: usize;
    static vDirA: usize; static vDirB: usize; static vT1CL: usize; static vT1CH: usize;
    static vT2CL: usize; static vT2CH: usize; static vACR: usize; static vPCR: usize;
    static rSIER: usize;
}

#[no_mangle] pub unsafe extern "C" fn via_init() {
    via1 = VIA1_BASE as *mut u8;
    pr_debug(b"VIA1 detected at %p\0".as_ptr(), via1);
    if oss_present { via2 = core::ptr::null_mut(); rbv_present = 0; } else {
        match (*macintosh_config).via_type {
            MAC_VIA_IICI => { via2 = RBV_BASE as *mut u8; pr_debug(b"VIA2 (RBV) detected at %p\0".as_ptr(), via2); rbv_present=1; rbv_clear = if (*macintosh_config).ident == MAC_MODEL_LCIII {0} else {0x80}; gIER=rIER; gIFR=rIFR; gBufA=rSIFR; gBufB=rBufB; }
            MAC_VIA_QUADRA | MAC_VIA_II => { via2=VIA2_BASE as *mut u8; pr_debug(b"VIA2 detected at %p\0".as_ptr(),via2); rbv_present=0; rbv_clear=0; gIER=vIER; gIFR=vIFR; gBufA=vBufA; gBufB=vBufB; }
            _ => panic(b"UNKNOWN VIA TYPE\0".as_ptr()),
        }
    }
    (*via1.add(vIER))=0x7f; (*via1.add(vIFR))=0x7f; (*via1.add(vT1CL))=0; (*via1.add(vT1CH))=0; (*via1.add(vT2CL))=0; (*via1.add(vT2CH))=0;
    *via1.add(vACR) &= !0xc0; *via1.add(vACR) &= !0x03;
    if (*macintosh_config).ident == MAC_MODEL_SE30 { *via1.add(vDirB) |= 0x40; *via1.add(vBufB) |= 0x40; }
    match (*macintosh_config).adb_type { MAC_ADB_IOP|MAC_ADB_II|MAC_ADB_PB1 => { *via1.add(vDirB) |= VIA1B_vRTCEnb|VIA1B_vRTCClk|VIA1B_vRTCData; *via1.add(vBufB) |= VIA1B_vRTCEnb|VIA1B_vRTCClk; }, _=>{} }
    if oss_present { return; }
    if (*macintosh_config).via_type==MAC_VIA_QUADRA && (*macintosh_config).adb_type!=MAC_ADB_PB1 && (*macintosh_config).adb_type!=MAC_ADB_PB2 && (*macintosh_config).ident!=MAC_MODEL_C660 && (*macintosh_config).ident!=MAC_MODEL_Q840 { via_alt_mapping=1; *via1.add(vDirB)|=0x40; *via1.add(vBufB)&=!0x40; } else { via_alt_mapping=0; }
    *via2.add(gIER)=0x7f; *via2.add(gIFR)=0x7f|rbv_clear;
    if rbv_present==0 { *via2.add(vT1CL)=0; *via2.add(vT1CH)=0; *via2.add(vT2CL)=0; *via2.add(vT2CH)=0; *via2.add(vACR)&=!0xc0; *via2.add(vACR)&=!0x03; }
    via_nubus_init(); if rbv_present!=0{return;} *via2.add(vPCR)=if (*macintosh_config).via_type==MAC_VIA_II {0x66} else {0x22};
}

unsafe fn via_nubus_init() { if (*macintosh_config).adb_type!=MAC_ADB_PB1 && (*macintosh_config).adb_type!=MAC_ADB_PB2 { if rbv_present==0 {*via2.add(vDirB)|=2;} *via2.add(gBufB)|=2; } match (*macintosh_config).via_type { MAC_VIA_II|MAC_VIA_QUADRA=>{}, MAC_VIA_IICI=>{*via2.add(rSIER)=0x7f;}, _=>{} } }

#[no_mangle] pub unsafe extern "C" fn via_l2_flush(_writeback:i32) { let mut flags=0; local_irq_save(&mut flags); *via2.add(gBufB)&=!VIA2B_vMode32; *via2.add(gBufB)|=VIA2B_vMode32; local_irq_restore(flags); }

#[no_mangle] pub unsafe extern "C" fn via_nubus_irq_startup(irq:i32) { let irq_idx=IRQ_IDX(irq); match (*macintosh_config).via_type { MAC_VIA_II|MAC_VIA_QUADRA=>{ if (*macintosh_config).via_type==MAC_VIA_II {*via2.add(vDirA)&=0xc0|!(1<<irq_idx);} else {*via2.add(vDirA)&=0x80|!(1<<irq_idx);} via_irq_enable(irq);}, _=>{} } }
#[no_mangle] pub unsafe extern "C" fn via_nubus_irq_shutdown(irq:i32) { match (*macintosh_config).via_type {MAC_VIA_II|MAC_VIA_QUADRA=>via_irq_enable(irq), MAC_VIA_IICI=>via_irq_disable(irq), _=>{}} }

const VIA_TIMER_1_INT:u8=1<<6;
#[no_mangle] pub unsafe extern "C" fn via1_irq(_desc:*mut irq_desc) { let mut events=*via1.add(vIFR)&*via1.add(vIER)&0x7f; if events==0{return;} let mut irq_num=IRQ_MAC_TIMER_1; let mut irq_bit=VIA_TIMER_1_INT; if events&irq_bit!=0 {let mut flags=0; local_irq_save(&mut flags);*via1.add(vIFR)=irq_bit;generic_handle_irq(irq_num);local_irq_restore(flags);events&=!irq_bit;if events==0{return;}} irq_num=VIA1_SOURCE_BASE;irq_bit=1; loop {if events&irq_bit!=0{*via1.add(vIFR)=irq_bit;generic_handle_irq(irq_num);}irq_num+=1;irq_bit<<=1;if events<irq_bit{break;}} }
unsafe fn via2_irq(_desc:*mut irq_desc) { let mut events=*via2.add(gIFR)&*via2.add(gIER)&0x7f;if events==0{return;}let mut irq_num=VIA2_SOURCE_BASE;let mut irq_bit=1;loop{if events&irq_bit!=0{*via2.add(gIFR)=irq_bit|rbv_clear;generic_handle_irq(irq_num);}irq_num+=1;irq_bit<<=1;if events<irq_bit{break;}} }
unsafe fn via_nubus_irq(_desc:*mut irq_desc) { let mut events=!(*via2.add(gBufA))&0x7f;if rbv_present!=0{events&=*via2.add(rSIER);}else{events&=!(*via2.add(vDirA));}if events==0{return;}loop{let mut irq=IRQ_NUBUS_F;let mut bit=0x40;while events!=0{if events&bit!=0{events&=!bit;generic_handle_irq(irq);}irq-=1;bit>>=1;}*via2.add(gIFR)=2|rbv_clear;events=!(*via2.add(gBufA))&0x7f;if rbv_present!=0{events&=*via2.add(rSIER);}else{events&=!(*via2.add(vDirA));}}}

#[no_mangle] pub unsafe extern "C" fn via_register_interrupts(){if via_alt_mapping!=0{irq_set_chained_handler(IRQ_AUTO_1,via1_irq);irq_set_chained_handler(IRQ_AUTO_6,via1_irq);}else{irq_set_chained_handler(IRQ_AUTO_1,via1_irq);}irq_set_chained_handler(IRQ_AUTO_2,via2_irq);irq_set_chained_handler(IRQ_MAC_NUBUS,via_nubus_irq);}
#[no_mangle] pub unsafe extern "C" fn via_irq_enable(irq:i32){let s=IRQ_SRC(irq);let i=IRQ_IDX(irq);if s==1{*via1.add(vIER)=IER_SET_BIT(i);}else if s==2{if irq!=IRQ_MAC_NUBUS||nubus_disabled==0{*via2.add(gIER)=IER_SET_BIT(i);}}else if s==7{match (*macintosh_config).via_type{MAC_VIA_II|MAC_VIA_QUADRA=>{nubus_disabled&=!(1<<i);if nubus_disabled==0{*via2.add(gIER)=IER_SET_BIT(1);}},MAC_VIA_IICI=>*via2.add(rSIER)=IER_SET_BIT(i),_=>{}}}}
#[no_mangle] pub unsafe extern "C" fn via_irq_disable(irq:i32){let s=IRQ_SRC(irq);let i=IRQ_IDX(irq);if s==1{*via1.add(vIER)=IER_CLR_BIT(i);}else if s==2{*via2.add(gIER)=IER_CLR_BIT(i);}else if s==7{match (*macintosh_config).via_type{MAC_VIA_II|MAC_VIA_QUADRA=>{nubus_disabled|=1<<i;if nubus_disabled!=0{*via2.add(gIER)=IER_CLR_BIT(1);}},MAC_VIA_IICI=>*via2.add(rSIER)=IER_CLR_BIT(i),_=>{}}}}
#[no_mangle] pub unsafe extern "C" fn via1_set_head(head:i32){if head==0{*via1.add(vBufA)&=!VIA1A_vHeadSel;}else{*via1.add(vBufA)|=VIA1A_vHeadSel;}}
#[no_mangle] pub unsafe extern "C" fn via2_scsi_drq_pending()->i32{(*via2.add(gIFR)&(1<<IRQ_IDX(IRQ_MAC_SCSIDRQ))) as i32}

const VIA_CLOCK_FREQ:u32=783360; const VIA_TIMER_CYCLES:u32=VIA_CLOCK_FREQ/HZ; const VIA_TC:u32=VIA_TIMER_CYCLES-2; const VIA_TC_LOW:u8=(VIA_TC&0xff) as u8; const VIA_TC_HIGH:u8=(VIA_TC>>8) as u8;
static mut clk_total:u32=0; static mut clk_offset:u32=0;
unsafe extern "C" fn via_timer_handler(_irq:i32,_dev_id:*mut core::ffi::c_void)->irqreturn_t{clk_total+=VIA_TIMER_CYCLES;clk_offset=0;legacy_timer_tick(1);IRQ_HANDLED}
#[no_mangle] pub unsafe extern "C" fn via_init_clock(){if request_irq(IRQ_MAC_TIMER_1,via_timer_handler,IRQF_TIMER,b"timer\0".as_ptr(),core::ptr::null_mut())!=0{pr_err(b"Couldn't register %s interrupt\0".as_ptr(),b"timer\0".as_ptr());return;}*via1.add(vT1CL)=VIA_TC_LOW;*via1.add(vT1CH)=VIA_TC_HIGH;*via1.add(vACR)|=0x40;clocksource_register_hz(&mut mac_clk,VIA_CLOCK_FREQ);}
static mut mac_clk:clocksource=clocksource{name:b"via1\0".as_ptr(),rating:250,read:Some(mac_read_clk),mask:0xffff_ffff,flags:CLOCK_SOURCE_IS_CONTINUOUS};
unsafe extern "C" fn mac_read_clk(_cs:*mut clocksource)->u64{let mut flags=0;local_irq_save(&mut flags);let mut h=*via1.add(vT1CH);if h==0xff{h=0;}if h>0&&(*via1.add(vIFR)&VIA_TIMER_1_INT)!=0{clk_offset=VIA_TIMER_CYCLES;}let count=(h as u32)<<8;let ticks=VIA_TIMER_CYCLES-count+clk_offset+clk_total;local_irq_restore(flags);ticks as u64}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
