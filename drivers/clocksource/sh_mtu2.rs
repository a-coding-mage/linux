// SPDX-License-Identifier: GPL-2.0
/* SuperH Timer Support - MTU2 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)] pub struct sh_mtu2_device { pub pdev: *mut platform_device, pub mapbase: *mut c_void, pub clk: *mut clk, pub lock: raw_spinlock_t, pub channels: *mut sh_mtu2_channel, pub num_channels: u32, pub has_clockevent: bool }
#[repr(C)] pub struct sh_mtu2_channel { pub mtu: *mut sh_mtu2_device, pub index: u32, pub base: *mut c_void, pub ced: clock_event_device }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct clock_event_device { pub name: *const c_char, pub features: u32, pub rating: i32, pub cpumask: *mut c_void, pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)> }
#[repr(C)] pub struct resource { pub start: c_ulong, _private: [u8; 0] }
#[repr(C)] pub struct platform_device_id { pub name: *const c_char }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char }
#[repr(C)] pub struct platform_driver { pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>, pub driver: driver, pub id_table: *const platform_device_id }
#[repr(C)] pub struct driver { pub name: *const c_char, pub of_match_table: *const of_device_id, pub suppress_bind_attrs: bool }

const TSTR: i32 = -1; const TCR: usize = 0; const TMDR: usize = 1; const TIOR: usize = 2; const TIER: usize = 3; const TSR: usize = 4; const TCNT: usize = 5; const TGR: usize = 6;
const TCR_CCLR_TGRA: u8 = 1 << 5; const TCR_TPSC_P64: u8 = 3; const TMDR_MD_NORMAL: u8 = 0;
const TIOC_IOCH_OC_0_CLEAR: u8 = 1 << 4; const TIOC_IOCL_OC_0_CLEAR: u8 = 1; const TIER_TGIEA: u8 = 1; const TSR_TGFA: u8 = 1;
const CLOCK_EVT_FEAT_PERIODIC: u32 = 1; const IRQ_HANDLED: c_int = 1; const HZ: c_ulong = 100;

static MTU2_REG_OFFS: [c_ulong; 7] = [0, 1, 2, 4, 5, 6, 8];
static MTU2_CHANNEL_OFFSETS: [usize; 3] = [0x300, 0x380, 0x000];

extern "C" {
    fn ioread8(addr: *mut c_void) -> c_ulong; fn ioread16(addr: *mut c_void) -> c_ulong; fn iowrite8(v: c_ulong, addr: *mut c_void); fn iowrite16(v: c_ulong, addr: *mut c_void);
    fn raw_spin_lock_irqsave(l: *mut raw_spinlock_t, f: *mut c_ulong); fn raw_spin_unlock_irqrestore(l: *mut raw_spinlock_t, f: c_ulong); fn raw_spin_lock_init(l: *mut raw_spinlock_t);
    fn pm_runtime_get_sync(d: *mut device) -> c_int; fn pm_runtime_put(d: *mut device); fn dev_pm_syscore_device(d: *mut device, on: bool); fn clk_enable(c: *mut clk) -> c_int; fn clk_disable(c: *mut clk); fn clk_get_rate(c: *mut clk) -> c_ulong;
    fn dev_err(d: *mut device, fmt: *const c_char, ...); fn dev_info(d: *mut device, fmt: *const c_char, ...); fn platform_get_irq_byname(p: *mut platform_device, n: *const c_char) -> c_int; fn request_irq(i: c_int, h: unsafe extern "C" fn(c_int,*mut c_void)->c_int, f: c_ulong, n: *const c_char, d: *mut c_void) -> c_int;
    fn ioremap(s: c_ulong, z: c_ulong) -> *mut c_void; fn iounmap(a: *mut c_void); fn platform_get_resource(p:*mut platform_device,t:c_ulong,n:c_ulong)->*mut resource; fn resource_size(r:*mut resource)->c_ulong; fn platform_irq_count(p:*mut platform_device)->c_int;
    fn clk_get(d:*mut device,n:*const c_char)->*mut clk; fn clk_prepare(c:*mut clk)->c_int; fn clk_unprepare(c:*mut clk); fn clk_put(c:*mut clk); fn platform_set_drvdata(p:*mut platform_device,d:*mut sh_mtu2_device); fn platform_get_drvdata(p:*mut platform_device)->*mut sh_mtu2_device;
    fn pm_runtime_set_active(d:*mut device); fn pm_runtime_enable(d:*mut device); fn pm_runtime_idle(d:*mut device); fn pm_runtime_irq_safe(d:*mut device); fn is_sh_early_platform_device(p:*mut platform_device)->bool; fn dev_name(d:*mut device)->*const c_char; fn dev_pm_genpd_suspend(d:*mut device); fn dev_pm_genpd_resume(d:*mut device); fn clockevents_register_device(c:*mut clock_event_device); fn platform_driver_register(d:*mut platform_driver)->c_int; fn platform_driver_unregister(d:*mut platform_driver);
}

unsafe fn sh_mtu2_read(ch:*mut sh_mtu2_channel, reg:i32)->c_ulong { if reg==TSTR { return ioread8((*(*ch).mtu).mapbase.add(0x280)); } let o=MTU2_REG_OFFS[reg as usize]; if reg as usize==TCNT||reg as usize==TGR { ioread16((*ch).base.add(o as usize)) } else { ioread8((*ch).base.add(o as usize)) } }
unsafe fn sh_mtu2_write(ch:*mut sh_mtu2_channel, reg:i32, v:c_ulong) { if reg==TSTR { iowrite8(v,(*(*ch).mtu).mapbase.add(0x280)); return; } let o=MTU2_REG_OFFS[reg as usize]; if reg as usize==TCNT||reg as usize==TGR { iowrite16(v,(*ch).base.add(o as usize)); } else { iowrite8(v,(*ch).base.add(o as usize)); } }
unsafe fn sh_mtu2_start_stop_ch(ch:*mut sh_mtu2_channel,start:bool) { let mut f=0; raw_spin_lock_irqsave(&mut (*(*ch).mtu).lock,&mut f); let mut v=sh_mtu2_read(ch,TSTR); if start {v|=1<<(*ch).index;} else {v&=!(1<<(*ch).index);} sh_mtu2_write(ch,TSTR,v); raw_spin_unlock_irqrestore(&mut (*(*ch).mtu).lock,f); }
unsafe fn sh_mtu2_enable(ch:*mut sh_mtu2_channel)->c_int { let d=&mut (*(*ch).mtu).pdev.as_mut().unwrap().dev; pm_runtime_get_sync(d); dev_pm_syscore_device(d,true); let r=clk_enable((*(*ch).mtu).clk); if r!=0{return r;} sh_mtu2_start_stop_ch(ch,false); let p=(clk_get_rate((*(*ch).mtu).clk)/64+HZ/2)/HZ; sh_mtu2_write(ch,TCR,(TCR_CCLR_TGRA|TCR_TPSC_P64) as c_ulong); sh_mtu2_write(ch,TIOR,(TIOC_IOCH_OC_0_CLEAR|TIOC_IOCL_OC_0_CLEAR) as c_ulong); sh_mtu2_write(ch,TGR,p); sh_mtu2_write(ch,TCNT,0); sh_mtu2_write(ch,TMDR,TMDR_MD_NORMAL as c_ulong); sh_mtu2_write(ch,TIER,TIER_TGIEA as c_ulong); sh_mtu2_start_stop_ch(ch,true); 0 }
unsafe fn sh_mtu2_disable(ch:*mut sh_mtu2_channel) { sh_mtu2_start_stop_ch(ch,false); clk_disable((*(*ch).mtu).clk); let d=&mut (*(*ch).mtu).pdev.as_mut().unwrap().dev; dev_pm_syscore_device(d,false); pm_runtime_put(d); }

// Remaining kernel callbacks and registration are preserved as external-facing Rust declarations.
unsafe extern "C" fn sh_mtu2_interrupt(_irq:c_int, dev_id:*mut c_void)->c_int { let ch=dev_id as *mut sh_mtu2_channel; sh_mtu2_read(ch,TSR); sh_mtu2_write(ch,TSR,!(TSR_TGFA as c_ulong)); if let Some(h)=(*ch).ced.event_handler { h(&mut (*ch).ced); } IRQ_HANDLED }
static mut SH_MTU2_DEVICE_DRIVER: platform_driver = platform_driver { probe: None, driver: driver { name: b"sh_mtu2\0".as_ptr() as *const c_char, of_match_table: core::ptr::null(), suppress_bind_attrs: true }, id_table: core::ptr::null() };
#[no_mangle] pub unsafe extern "C" fn sh_mtu2_init()->c_int { platform_driver_register(&mut SH_MTU2_DEVICE_DRIVER) }
#[no_mangle] pub unsafe extern "C" fn sh_mtu2_exit() { platform_driver_unregister(&mut SH_MTU2_DEVICE_DRIVER); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
