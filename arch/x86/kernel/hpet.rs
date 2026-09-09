// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level translation of hpet.c. Kernel-provided types, constants,
// macros and functions are intentionally referenced as external dependencies.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HpetMode { Unused, Legacy, Clockevt, Device }

#[repr(C)]
pub struct HpetChannel {
    pub evt: clock_event_device,
    pub num: u32, pub cpu: u32, pub irq: u32, pub in_use: u32,
    pub mode: HpetMode, pub boot_cfg: u32, pub name: [u8; 10],
}
#[repr(C)]
pub struct HpetBase { pub nr_channels: u32, pub nr_clockevents: u32,
    pub boot_cfg: u32, pub channels: *mut HpetChannel }

pub const HPET_MASK: u64 = CLOCKSOURCE_MASK(32);
pub const HPET_MIN_CYCLES: u32 = 128;
pub const HPET_MIN_PROG_DELTA: u32 = HPET_MIN_CYCLES + (HPET_MIN_CYCLES >> 1);

#[no_mangle] pub static mut hpet_address: usize = 0;
#[no_mangle] pub static mut hpet_blockid: u8 = 0;
#[no_mangle] pub static mut hpet_msi_disable: bool = false;
static mut hpet_virt_address: *mut u8 = core::ptr::null_mut();
static mut hpet_base: HpetBase = HpetBase { nr_channels: 0, nr_clockevents: 0, boot_cfg: 0, channels: core::ptr::null_mut() };
static mut hpet_legacy_int_enabled: bool = false;
static mut hpet_freq: usize = 0;
#[no_mangle] pub static mut boot_hpet_disable: bool = false;
#[no_mangle] pub static mut hpet_force_user: bool = false;
static mut hpet_verbose: bool = false;

#[inline] unsafe fn clockevent_to_channel(evt: *mut clock_event_device) -> *mut HpetChannel {
    (evt as *mut u8).sub(core::mem::offset_of!(HpetChannel, evt)) as *mut HpetChannel
}
#[no_mangle] pub unsafe fn hpet_readl(a: u32) -> u32 { readl(hpet_virt_address.add(a as usize)) }
unsafe fn hpet_writel(d: u32, a: u32) { writel(d, hpet_virt_address.add(a as usize)); }
unsafe fn hpet_set_mapping() { hpet_virt_address = ioremap(hpet_address, HPET_MMAP_SIZE); }
unsafe fn hpet_clear_mapping() { iounmap(hpet_virt_address); hpet_virt_address = core::ptr::null_mut(); }

unsafe fn hpet_setup(mut s: *mut u8) -> i32 {
    while !s.is_null() { let next = strchr(s, b','); if !next.is_null() { *next = 0; s = next.add(1); } else { s = core::ptr::null_mut(); }
        if !strncmp(b"disable\0".as_ptr(), s, 7) { boot_hpet_disable = true; }
        if !strncmp(b"force\0".as_ptr(), s, 5) { hpet_force_user = true; }
        if !strncmp(b"verbose\0".as_ptr(), s, 7) { hpet_verbose = true; }
    } 1
}
unsafe fn disable_hpet(_: *mut u8) -> i32 { boot_hpet_disable = true; 1 }
unsafe fn is_hpet_capable() -> bool { !boot_hpet_disable && hpet_address != 0 }
#[no_mangle] pub unsafe fn is_hpet_enabled() -> bool { is_hpet_capable() && hpet_legacy_int_enabled }

unsafe fn hpet_stop_counter() { let mut c=hpet_readl(HPET_CFG); c &= !HPET_CFG_ENABLE; hpet_writel(c,HPET_CFG); }
unsafe fn hpet_reset_counter() { hpet_writel(0,HPET_COUNTER); hpet_writel(0,HPET_COUNTER+4); }
unsafe fn hpet_start_counter() { let mut c=hpet_readl(HPET_CFG); c |= HPET_CFG_ENABLE; hpet_writel(c,HPET_CFG); }
unsafe fn hpet_restart_counter() { hpet_stop_counter(); hpet_reset_counter(); hpet_start_counter(); }
unsafe fn hpet_resume_device() { force_hpet_resume(); }
unsafe fn hpet_resume_counter(_: *mut clocksource) { hpet_resume_device(); hpet_restart_counter(); }
unsafe fn hpet_enable_legacy_int() { let mut c=hpet_readl(HPET_CFG); c|=HPET_CFG_LEGACY; hpet_writel(c,HPET_CFG); hpet_legacy_int_enabled=true; }

unsafe fn hpet_clkevt_set_state_periodic(evt: *mut clock_event_device) -> i32 {
    let ch=(*clockevent_to_channel(evt)).num; hpet_stop_counter();
    let mut delta=((NSEC_PER_SEC/HZ) as u64)*(*evt).mult as u64; delta >>= (*evt).shift;
    let now=hpet_readl(HPET_COUNTER); let cmp=now.wrapping_add(delta as u32); let mut c=hpet_readl(HPET_Tn_CFG(ch));
    c |= HPET_TN_ENABLE|HPET_TN_PERIODIC|HPET_TN_SETVAL|HPET_TN_32BIT; hpet_writel(c,HPET_Tn_CFG(ch)); hpet_writel(cmp,HPET_Tn_CMP(ch)); udelay(1); hpet_writel(delta as u32,HPET_Tn_CMP(ch)); hpet_start_counter(); 0
}
unsafe fn hpet_clkevt_set_state_oneshot(evt:*mut clock_event_device)->i32 { let ch=(*clockevent_to_channel(evt)).num; let mut c=hpet_readl(HPET_Tn_CFG(ch)); c&=!HPET_TN_PERIODIC; c|=HPET_TN_ENABLE|HPET_TN_32BIT; hpet_writel(c,HPET_Tn_CFG(ch)); 0 }
unsafe fn hpet_clkevt_set_state_shutdown(evt:*mut clock_event_device)->i32 { let ch=(*clockevent_to_channel(evt)).num; let mut c=hpet_readl(HPET_Tn_CFG(ch)); c&=!HPET_TN_ENABLE; hpet_writel(c,HPET_Tn_CFG(ch)); 0 }
unsafe fn hpet_clkevt_legacy_resume(_: *mut clock_event_device)->i32 { hpet_enable_legacy_int(); 0 }
unsafe fn hpet_clkevt_set_next_event(delta:usize,evt:*mut clock_event_device)->i32 { let ch=(*clockevent_to_channel(evt)).num; let cnt=hpet_readl(HPET_COUNTER).wrapping_add(delta as u32); hpet_writel(cnt,HPET_Tn_CMP(ch)); let res=(cnt.wrapping_sub(hpet_readl(HPET_COUNTER))) as i32; if res < HPET_MIN_CYCLES as i32 {-ETIME} else {0} }

unsafe fn hpet_init_clockevent(hc:*mut HpetChannel,rating:i32) { let e=&mut (*hc).evt; e.rating=rating; e.irq=(*hc).irq; e.name=(*hc).name.as_mut_ptr(); e.cpumask=cpumask_of((*hc).cpu); e.set_state_oneshot=Some(hpet_clkevt_set_state_oneshot); e.set_next_event=Some(hpet_clkevt_set_next_event); e.set_state_shutdown=Some(hpet_clkevt_set_state_shutdown); e.features=CLOCK_EVT_FEAT_ONESHOT; if (*hc).boot_cfg&HPET_TN_PERIODIC!=0 {e.features|=CLOCK_EVT_FEAT_PERIODIC;e.set_state_periodic=Some(hpet_clkevt_set_state_periodic);} }

#[no_mangle] pub unsafe fn hpet_enable() -> i32 {
    if !is_hpet_capable(){return 0;} hpet_set_mapping(); if hpet_virt_address.is_null(){return 0;}
    let period=hpet_readl(HPET_PERIOD); if period<HPET_MIN_PERIOD||period>HPET_MAX_PERIOD {hpet_clear_mapping();hpet_address=0;return 0;}
    hpet_freq=FSEC_PER_SEC/period as usize; let id=hpet_readl(HPET_ID); let channels=((id&HPET_ID_NUMBER)>>HPET_ID_NUMBER_SHIFT)+1;
    let p=kzalloc_objs::<HpetChannel>(channels as usize); if p.is_null(){hpet_clear_mapping();return 0;} hpet_base.channels=p;hpet_base.nr_channels=channels;
    let mut cfg=hpet_readl(HPET_CFG); hpet_base.boot_cfg=cfg; cfg&=!(HPET_CFG_ENABLE|HPET_CFG_LEGACY);hpet_writel(cfg,HPET_CFG);
    for i in 0..channels {let hc=p.add(i as usize);(*hc).num=i;let mut c=hpet_readl(HPET_Tn_CFG(i));(*hc).boot_cfg=c;(*hc).irq=(c&Tn_INT_ROUTE_CNF_MASK)>>Tn_INT_ROUTE_CNF_SHIFT;c&=!(HPET_TN_ENABLE|HPET_TN_LEVEL|HPET_TN_FSB);hpet_writel(c,HPET_Tn_CFG(i));}
    hpet_restart_counter(); clocksource_register_hz(&mut clocksource_hpet,hpet_freq as u32); if id&HPET_ID_LEGSUP!=0 {hpet_enable_legacy_int();(*p).mode=HpetMode::Legacy;return 1;} 0
}
#[no_mangle] pub unsafe fn hpet_disable(){if !is_hpet_capable()||hpet_virt_address.is_null(){return;}let mut c=hpet_base.boot_cfg&!HPET_CFG_ENABLE;hpet_writel(c,HPET_CFG);for i in 0..hpet_base.nr_channels{hpet_writel((*hpet_base.channels.add(i as usize)).boot_cfg,HPET_Tn_CFG(i));}if hpet_base.boot_cfg&HPET_CFG_ENABLE!=0{hpet_writel(hpet_base.boot_cfg,HPET_CFG);}}

// The remaining kernel-only registration, MSI, CPU-hotplug, and RTC-emulation
// entry points retain their C ABI and are supplied by the surrounding kernel.
extern "C" { static mut clocksource_hpet: clocksource; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
