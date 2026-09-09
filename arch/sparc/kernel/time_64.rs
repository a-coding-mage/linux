// SPDX-License-Identifier: GPL-2.0
/* UltraSparc timer and TOD clock support. */

use core::ffi::c_void;

// Kernel dependencies supplied by the surrounding translation unit.
extern "C" {
    static mut prom_root_node: phandle;
    static mut tlb_type: i32;
    static mut this_is_starfire: bool;
    static mut tick_ops: *mut sparc64_tick_ops;
    fn prom_getproperty(node: phandle, name: *const u8, buf: *mut u8, len: usize) -> i32;
    fn prom_getint(node: phandle, name: *const u8) -> i32;
    fn prom_getintdefault(node: phandle, name: *const u8, def: i32) -> u64;
    fn prom_getchild(node: phandle) -> phandle;
    fn prom_getsibling(node: phandle) -> phandle;
    fn hard_smp_processor_id() -> i32;
    fn get_tick() -> u64;
    fn cpumask_of(cpu: i32) -> *mut c_void;
    fn smp_processor_id() -> i32;
    fn set_irq_regs(regs: *mut pt_regs) -> *mut pt_regs;
    fn clear_softint(mask: u64);
    fn irq_enter();
    fn irq_exit();
    fn kstat_incr_irq_this_cpu(irq: i32);
    fn clocksource_hz2mult(freq: u64, shift: u64) -> u64;
    fn clocksource_register_hz(cs: *mut clocksource, freq: u64) -> i32;
    fn clockevents_calc_mult_shift(evt: *mut clock_event_device, freq: u64, sec: u32);
    fn clockevent_delta2ns(delta: u64, evt: *const clock_event_device) -> u64;
    fn clockevents_register_device(evt: *mut clock_event_device) -> i32;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn wmb();
    fn flushi(addr: *const u32);
    fn barrier();
    fn platform_device_register(dev: *mut platform_device) -> i32;
    fn platform_driver_register(drv: *mut platform_driver) -> i32;
    fn readb(addr: *const u8) -> u8;
    fn writeb(val: u8, addr: *mut u8);
    fn cpufreq_register_notifier(nb: *mut notifier_block, val: u32) -> i32;
    fn cpufreq_scale(old: u64, ref_freq: u32, new: u32) -> u64;
    static mut __get_tick_patch: get_tick_patch;
    static mut __get_tick_patch_end: get_tick_patch;
}

type phandle = u32;
type u8_t = u8;
type u32_t = u32;
type pt_regs = c_void;

#[repr(C)] pub struct sparc64_tick_ops { pub name: *const u8, pub init_tick: Option<unsafe extern "C" fn()>, pub disable_irq: Option<unsafe extern "C" fn()>, pub get_tick: Option<unsafe extern "C" fn() -> u64>, pub add_tick: Option<unsafe extern "C" fn(u64) -> u64>, pub add_compare: Option<unsafe extern "C" fn(u64) -> i32>, pub get_frequency: Option<unsafe extern "C" fn() -> u64>, pub softint_mask: u64, pub offset: u64, pub ticks_per_nsec_quotient: u64, pub frequency: u64 }
#[repr(C)] pub struct freq_table { pub clock_tick_ref: u64, pub ref_freq: u32 }
#[repr(C)] pub struct clock_event_device { pub name: *const u8, pub features: u32, pub set_state_shutdown: Option<unsafe extern "C" fn(*mut clock_event_device)->i32>, pub set_next_event: Option<unsafe extern "C" fn(u64,*mut clock_event_device)->i32>, pub rating: u32, pub shift: u32, pub irq: i32, pub cpumask: *mut c_void, pub event_handler: Option<unsafe extern "C" fn(*mut clock_event_device)>, pub max_delta_ns:u64, pub max_delta_ticks:u64, pub min_delta_ns:u64, pub min_delta_ticks:u64, pub mult:u32 }
#[repr(C)] pub struct clocksource { pub name:*const u8, pub rating:u32, pub mask:u64, pub flags:u32, pub vdso_clock_mode:u32, pub read:Option<unsafe extern "C" fn(*mut clocksource)->u64>, pub mult:u32, pub shift:u32 }
#[repr(C)] pub struct resource { pub name:*const u8, pub start:u64, pub end:u64, pub flags:u64 }
#[repr(C)] pub struct device_node { pub parent:*mut device_node }
#[repr(C)] pub struct device { pub of_node:*mut device_node, pub platform_data:*mut c_void }
#[repr(C)] pub struct platform_device { pub name:*const u8, pub id:i32, pub resource:*mut resource, pub num_resources:u32, pub dev:device }
#[repr(C)] pub struct platform_driver { pub probe:Option<unsafe extern "C" fn(*mut platform_device)->i32> }
#[repr(C)] pub struct notifier_block { pub notifier_call:Option<unsafe extern "C" fn(*mut notifier_block,u64,*mut c_void)->i32> }
#[repr(C)] pub struct get_tick_patch { pub addr:u64, pub tick:*mut u32, pub stick:*mut u32 }

static mut rtc_lock: u32 = 0;
static mut cmos_regs: u64 = 0;
static mut tick_operations: sparc64_tick_ops = sparc64_tick_ops { name:b"tick\0".as_ptr(), init_tick:None, disable_irq:None, get_tick:None, add_tick:None, add_compare:None, get_frequency:None, softint_mask:1, offset:0, ticks_per_nsec_quotient:0, frequency:0 };
static mut sparc64_freq_table: freq_table = freq_table { clock_tick_ref:0, ref_freq:0 };

unsafe fn tick_disable_protection() { core::arch::asm!("rd %tick, %g2", "add %g2, 6, %g2", "andn %g2, {0}, %g2", "wrpr %g2, 0, %tick", in(reg) 1u64); }
unsafe fn tick_disable_irq() { core::arch::asm!("wr {0}, 0x0, %tick_cmpr", "rd %tick_cmpr, %g0", in(reg) 1u64); }
unsafe fn tick_init_tick() { tick_disable_protection(); tick_disable_irq(); }
unsafe fn tick_get_tick() -> u64 { let ret:u64; core::arch::asm!("rd %tick, {0}", out(reg) ret); ret & !1 }
unsafe fn tick_add_tick(adj:u64)->u64 { let mut v:u64; core::arch::asm!("rd %tick, {0}", "add {0}, {1}, {0}", "wrpr {0}, 0, %tick", out(reg)v, in(reg)adj); v }
unsafe fn tick_add_compare(adj:u64)->i32 { let orig=tick_get_tick() & !1; let _=tick_add_tick(adj); ((tick_get_tick().wrapping_sub(orig.wrapping_add(adj)) as i64)>0) as i32 }
unsafe fn cpuid_to_freq(node:phandle, cpuid:i32)->u64 { if node==0{return 0}; let mut typ=[0u8;128]; let cpu=prom_getproperty(node,b"device_type\0".as_ptr(),typ.as_mut_ptr(),128)!=-1 && typ.starts_with(b"cpu"); let mut f=if cpu && (prom_getint(node,b"upa-portid\0".as_ptr())==cpuid || prom_getint(node,b"cpuid\0".as_ptr())==cpuid){prom_getintdefault(node,b"clock-frequency\0".as_ptr(),0)}else{0}; if f==0 {f=cpuid_to_freq(prom_getchild(node),cpuid)} if f==0 {f=cpuid_to_freq(prom_getsibling(node),cpuid)} f }
unsafe fn tick_get_frequency()->u64 { cpuid_to_freq(prom_root_node,hard_smp_processor_id()) }
unsafe fn stick_disable_irq() { core::arch::asm!("wr {0}, 0x0, %asr25", in(reg) 1u64); }
unsafe fn stick_init_tick() { if tlb_type!=1 {tick_disable_protection();tick_disable_irq();} stick_disable_irq(); }
unsafe fn stick_get_tick()->u64 { let v:u64;core::arch::asm!("rd %asr24, {0}",out(reg)v);v&!1 }
unsafe fn stick_add_tick(adj:u64)->u64 { let mut v:u64;core::arch::asm!("rd %asr24, {0}","add {0}, {1}, {0}","wr {0}, 0, %asr24",out(reg)v,in(reg)adj);v }
unsafe fn stick_add_compare(adj:u64)->i32 {let v=stick_get_tick()&!1;core::arch::asm!("wr {0}, 0, %asr25",in(reg)v.wrapping_add(adj));((stick_get_tick().wrapping_sub(v+adj)as i64)>0)as i32}
unsafe fn stick_get_frequency()->u64 {prom_getintdefault(prom_root_node,b"stick-frequency\0".as_ptr(),0)}
unsafe fn hbtick_get_tick()->u64 {stick_get_tick()}
unsafe fn hbtick_add_tick(adj:u64)->u64 {stick_add_tick(adj)}
unsafe fn hbtick_add_compare(adj:u64)->i32 {stick_add_compare(adj)}
unsafe fn hbtick_get_frequency()->u64 {stick_get_frequency()}
unsafe fn hbtick_init_tick(){tick_disable_protection();stick_disable_irq()}
unsafe fn hbtick_disable_irq(){stick_disable_irq()}

unsafe fn is_hummingbird()->bool { let v:u64;core::arch::asm!("rdpr %ver, {0}",out(reg)v);((v>>48)&0xffff)==0x17&&((v>>32)&0xffff)==0x13 }
pub unsafe extern "C" fn sparc64_get_clock_tick(_cpu:u32)->u64 { if sparc64_freq_table.clock_tick_ref!=0 {sparc64_freq_table.clock_tick_ref} else {0} }
unsafe fn sparc64_next_event(delta:u64,_evt:*mut clock_event_device)->i32 {if tick_add_compare(delta)!=0 {-62}else{0}}
unsafe fn sparc64_timer_shutdown(_evt:*mut clock_event_device)->i32 {tick_disable_irq();0}
pub unsafe extern "C" fn __delay(loops:u64){let b=get_tick();while get_tick().wrapping_sub(b)<loops{core::hint::spin_loop()}}
pub unsafe extern "C" fn udelay(usecs:u64){__delay(usecs)}
unsafe fn clocksource_tick_read(_cs:*mut clocksource)->u64{get_tick()}
pub unsafe extern "C" fn time_init_early(){if tlb_type==1&&is_hummingbird() {hbtick_init_tick()} else if tlb_type==1 {tick_init_tick()} else {stick_init_tick()}}
pub unsafe extern "C" fn time_init(){ }
pub unsafe extern "C" fn sched_clock()->u64 {barrier();get_tick()}
pub unsafe extern "C" fn delay_read_timer(v:*mut u64)->bool {*v=get_tick();true}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
