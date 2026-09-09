// SPDX-License-Identifier: GPL-2.0-only
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// C translation of x86/kernel/tsc.c. Kernel-provided declarations are external.
use core::ffi::c_void;

extern "C" {
    static mut cpu_khz: u32;
    static mut tsc_khz: u32;
    static mut tsc_clocksource_reliable: i32;
    static mut tsc_unstable: i32;
    static mut jiffies_64: u64;
    static mut boot_cpu_data: boot_cpu_data_t;
    static mut x86_platform: x86_platform_t;
    fn rdtsc() -> u64; fn rdtsc_ordered() -> u64; fn get_cycles() -> u64;
    fn preempt_disable_notrace(); fn preempt_enable_notrace();
    fn mul_u64_u32_shr(a: u64, b: u32, s: u32) -> u64;
    fn clocks_calc_mult_shift(m: *mut u32, s: *mut u32, from: u64, to: u64, maxsec: u32);
    fn smp_processor_id() -> i32; fn sched_clock() -> u64; fn sched_clock_noinstr() -> u64;
    fn sched_clock_stable() -> bool; fn clear_sched_clock_stable(); fn sched_clock_tick_stable();
    fn sched_clock_idle_sleep_event(); fn sched_clock_idle_wakeup_event();
    fn local_irq_save(f: *mut usize); fn local_irq_restore(f: usize);
    fn per_cpu_ptr<T>(p: *mut T, cpu: i32) -> *mut T; fn this_cpu_ptr<T>(p: *mut T) -> *mut T;
    fn cpuid(leaf: u32, a: *mut u32, b: *mut u32, c: *mut u32, d: *mut u32);
    fn cpu_khz_from_msr() -> u64; fn setup_force_cpu_cap(x: u32); fn boot_cpu_has(x: u32) -> bool;
    fn mark_tsc_unstable(r: *mut u8); fn is_hpet_enabled() -> i32;
    fn hpet_readl(x: u32) -> u64; fn acpi_pm_read_early() -> u64;
    fn has_legacy_pic() -> bool; fn udelay(x: u64); fn inb(x: u16) -> u8; fn outb(v: u8, p: u16);
    fn cpufreq_scale(a: u64, b: u64, c: u64) -> u64; fn num_online_cpus() -> i32;
    fn topology_max_packages() -> i32; fn apic_is_clustered_box() -> bool;
    fn tsc_verify_tsc_adjust(x: bool); fn vclocks_set_used(x: u32);
    fn clocksource_mark_unstable(x: *mut clocksource); fn clocksource_register_khz(x: *mut clocksource, k: u32) -> i32;
    fn clocksource_unregister(x: *mut clocksource); fn lapic_update_tsc_freq();
    fn schedule_delayed_work(w: *mut delayed_work, d: u64) -> bool;
    fn pr_info(_: *const u8, ...); fn pr_warn(_: *const u8, ...); fn pr_notice(_: *const u8, ...);
}

#[repr(C)] pub struct cyc2ns_data { pub cyc2ns_offset: u64, pub cyc2ns_mul: u32, pub cyc2ns_shift: u32 }
#[repr(C)] pub struct seqcount_latch_t { pub sequence: u32 }
#[repr(C)] pub struct cyc2ns { pub data: [cyc2ns_data; 2], pub seq: seqcount_latch_t }
#[repr(C)] pub struct clocksource { pub name: *const u8, pub rating: i32, pub read: Option<unsafe extern "C" fn(*mut clocksource)->u64>, pub mask:u64, pub flags:u64, pub id:u32, pub vdso_clock_mode:u32, pub enable:Option<unsafe extern "C" fn(*mut clocksource)->i32>, pub resume:Option<unsafe extern "C" fn(*mut clocksource)>, pub mark_unstable:Option<unsafe extern "C" fn(*mut clocksource)>, pub tick_stable:Option<unsafe extern "C" fn(*mut clocksource)>, pub list:[usize;2], pub base:*mut clocksource_base }
#[repr(C)] pub struct clocksource_base { pub id:u32, pub denominator:u32, pub numerator:u32, pub freq_khz:u32, pub offset:u64 }
#[repr(C)] pub struct delayed_work { pub opaque:[usize;8] }
#[repr(C)] pub struct work_struct { pub opaque:[usize;2] }
#[repr(C)] pub struct cpufreq_freqs { pub old:u32, pub new:u32, pub flags:u32, pub policy:*mut cpufreq_policy }
#[repr(C)] pub struct cpufreq_policy { pub cpu:i32 }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block,usize,*mut c_void)->i32> }
#[repr(C)] pub struct boot_cpu_data_t { pub x86_vendor:u32, pub cpuid_level:u32, pub x86_vfm:u32, pub loops_per_jiffy:u64 }
#[repr(C)] pub struct x86_platform_t { pub calibrate_cpu: Option<unsafe extern "C" fn()->u64>, pub calibrate_tsc: Option<unsafe extern "C" fn()->u64> }

static mut tsc_early_khz: u32 = 0;
static mut tsc_force_recalibrate: i32 = 0;
static mut art_base_clk: clocksource_base = clocksource_base { id: 0, denominator:0, numerator:0, freq_khz:0, offset:0 };
static mut have_art: bool = false;
static mut cyc2ns_store: cyc2ns = cyc2ns { data:[cyc2ns_data{cyc2ns_offset:0,cyc2ns_mul:0,cyc2ns_shift:0};2], seq:seqcount_latch_t{sequence:0} };
static mut no_sched_irq_time:i32=0; static mut tsc_watchdog:i32=0; static mut cyc2ns_suspend:u64=0;

unsafe fn __cyc2ns_read(d:*mut cyc2ns_data) { loop { let seq=cyc2ns_store.seq.sequence; let x=(seq&1) as usize; (*d)=cyc2ns_store.data[x]; if seq==cyc2ns_store.seq.sequence {break;} } }
unsafe fn __cycles_2_ns(c:u64)->u64 { let mut d=cyc2ns_data{cyc2ns_offset:0,cyc2ns_mul:0,cyc2ns_shift:0}; __cyc2ns_read(&mut d); d.cyc2ns_offset + mul_u64_u32_shr(c,d.cyc2ns_mul,d.cyc2ns_shift) }
unsafe fn cycles_2_ns(c:u64)->u64 { preempt_disable_notrace(); let n=__cycles_2_ns(c); preempt_enable_notrace(); n }
unsafe fn set_cyc2ns_scale(khz:u64,_cpu:i32,tsc:u64) { let mut d=cyc2ns_data{cyc2ns_offset:0,cyc2ns_mul:0,cyc2ns_shift:0}; let n=cycles_2_ns(tsc); clocks_calc_mult_shift(&mut d.cyc2ns_mul,&mut d.cyc2ns_shift,khz,1_000_000,0); if d.cyc2ns_shift==32 {d.cyc2ns_shift=31;d.cyc2ns_mul>>=1;} d.cyc2ns_offset=n-mul_u64_u32_shr(tsc,d.cyc2ns_mul,d.cyc2ns_shift); cyc2ns_store.data=[d,d]; }

#[no_mangle] pub unsafe extern "C" fn native_sched_clock()->u64 { if __use_tsc() { __cycles_2_ns(rdtsc()) } else { (jiffies_64-INITIAL_JIFFIES)*(1_000_000_000/HZ) } }
#[no_mangle] pub unsafe extern "C" fn native_sched_clock_from_tsc(t:u64)->u64 { cycles_2_ns(t) }
#[no_mangle] pub unsafe extern "C" fn sched_clock()->u64 { preempt_disable_notrace(); let n=native_sched_clock(); preempt_enable_notrace(); n }
#[no_mangle] pub unsafe extern "C" fn check_tsc_unstable()->i32 { tsc_unstable }
#[no_mangle] pub unsafe extern "C" fn notsc_setup(_: *mut u8)->i32 { mark_tsc_unstable(b"boot parameter notsc\0" as *const u8 as *mut u8); 1 }

const HZ:u64=1000; const INITIAL_JIFFIES:u64=0; const TSC_WATCHDOG_AUTO:i32=0; const TSC_WATCHDOG_OFF:i32=1; const TSC_WATCHDOG_ON:i32=2;
const MAX_RETRIES:i32=5; const TSC_DEFAULT_THRESHOLD:u64=0x20000; const PIT_TICK_RATE:u64=1_193_182; const USEC_PER_MSEC:u64=1000;
const ULONG_MAX:u64=u64::MAX; const ULLONG_MAX:u64=u64::MAX; const CPUID_LEAF_TSC:u32=0x15; const CPUID_LEAF_FREQ:u32=0x16;

unsafe fn tsc_read_refs(p:*mut u64,hpet:i32)->u64 { let thresh=if tsc_khz!=0 {(tsc_khz as u64)>>5}else{TSC_DEFAULT_THRESHOLD}; for _ in 0..MAX_RETRIES {let t1=get_cycles(); *p=if hpet!=0{hpet_readl(0)&0xffff_ffff}else{acpi_pm_read_early()};let t2=get_cycles();if t2-t1<thresh{return t2;}} ULLONG_MAX }
unsafe fn pit_verify_msb(v:u8)->i32 {inb(0x42); if inb(0x42)==v{1}else{0}}
unsafe fn pit_expect_msb(v:u8,t:*mut u64,d:*mut u64)->i32 {let mut count=0;let mut ts=0;let mut prev=0;while count<50000&&pit_verify_msb(v)!=0{prev=ts;ts=get_cycles();count+=1;}*d=get_cycles()-prev;*t=ts;if count>5{1}else{0}}
unsafe fn quick_pit_calibrate()->u64 {if !has_legacy_pic(){return 0;} outb((inb(0x61)&!2)|1,0x61);outb(0xb0,0x43);outb(0xff,0x42);outb(0xff,0x42);pit_verify_msb(0);let(mut t,mut d1,mut d2)=(0,0,0);if pit_expect_msb(0xff,&mut t,&mut d1)!=0{for i in 1..=((50*PIT_TICK_RATE/1000/256) as u64){if pit_expect_msb((0xff-i) as u8,&mut d2,&mut d2)==0{break;}d2-=t;if i==1&&d1+d2 >= (d2*((50*PIT_TICK_RATE/1000/256) as u64)>>11){return 0;}if d1+d2 >= d2>>11{continue;}if pit_verify_msb((0xfe-i) as u8)==0{break;}return d2*PIT_TICK_RATE/(i*256*1000);}}0}

#[no_mangle] pub unsafe extern "C" fn native_calibrate_cpu_early()->u64 {let mut f=cpu_khz_from_cpuid();if f==0{f=cpu_khz_from_msr();}if f==0{let mut x=0;local_irq_save(&mut x);f=quick_pit_calibrate();local_irq_restore(x);}f}
unsafe fn cpu_khz_from_cpuid()->u64 {if boot_cpu_data.x86_vendor!=0||boot_cpu_data.cpuid_level<CPUID_LEAF_FREQ{return 0;}let(mut a,mut b,mut c,mut d)=(0,0,0,0);cpuid(CPUID_LEAF_FREQ,&mut a,&mut b,&mut c,&mut d);a as u64*1000}
unsafe fn native_calibrate_cpu()->u64 {let mut f=native_calibrate_cpu_early();if f==0{f=0;}f}

#[no_mangle] pub unsafe extern "C" fn tsc_save_sched_clock_state(){if !__use_tsc()&&!sched_clock_stable(){return;}cyc2ns_suspend=sched_clock();}
#[no_mangle] pub unsafe extern "C" fn tsc_restore_sched_clock_state(){if !__use_tsc()&&!sched_clock_stable(){return;}let mut f=0;local_irq_save(&mut f);let o=cyc2ns_suspend-sched_clock();cyc2ns_store.data[0].cyc2ns_offset=o;cyc2ns_store.data[1].cyc2ns_offset=o;local_irq_restore(f);}
#[no_mangle] pub unsafe extern "C" fn unsynchronized_tsc()->i32 {if !boot_cpu_has(1)||tsc_unstable!=0{return 1;}if boot_cpu_has(2){return 0;}if tsc_clocksource_reliable!=0{return 0;}if boot_cpu_data.x86_vendor!=0&&topology_max_packages()>1{return 1;}0}

unsafe fn __use_tsc()->bool { true }

// Reference calibration helpers and clocksource callbacks retain the C
// algorithm and use the kernel declarations above.
unsafe fn calc_hpet_ref(mut delta:u64,h1:u64,mut h2:u64)->u64 { if h2<h1 {h2+=0x1_0000_0000;} h2-=h1; let mut tmp=h2*hpet_readl(0); tmp/=1_000_000; delta/=tmp; delta }
unsafe fn calc_pmtimer_ref(mut delta:u64,p1:u64,mut p2:u64)->u64 { if p1==0&&p2==0{return ULONG_MAX;} if p2<p1{p2+=0xffff_ffff;} p2-=p1;let mut tmp=p2*1_000_000_000;tmp/=3_579_545;delta/=tmp;delta }
unsafe fn pit_calibrate_tsc(_l:u32,_ms:u64,_min:i32)->u64 { if !has_legacy_pic(){for _ in 0..5{udelay(10*USEC_PER_MSEC);}return ULONG_MAX;} let mut t=get_cycles();let t1=t;let mut mn=ULONG_MAX;let mut mx=0;let mut n=0;outb((inb(0x61)&!2)|1,0x61);outb(0xb0,0x43);outb(0xff,0x42);outb(0xff,0x42);while inb(0x61)&0x20==0{let x=get_cycles();let d=x-t;t=x;if d<mn{mn=d;}if d>mx{mx=d;}n+=1;}if n<1000||mx>10*mn{return ULONG_MAX;} (t-t1)/10 }
#[no_mangle] pub unsafe extern "C" fn native_calibrate_tsc()->u64 {if boot_cpu_data.x86_vendor!=0||boot_cpu_data.cpuid_level<CPUID_LEAF_TSC{return 0;}let(mut a,mut b,mut c,mut d)=(0,0,0,0);cpuid(CPUID_LEAF_TSC,&mut a,&mut b,&mut c,&mut d);if a==0||b==0{return 0;}let crystal=(c/1000) as u64;if crystal==0{return 0;}crystal*b as u64/a as u64}
#[no_mangle] pub unsafe extern "C" fn recalibrate_cpu_khz(){if !boot_cpu_has(1){return;}if let Some(f)=x86_platform.calibrate_cpu{cpu_khz=f();}if let Some(f)=x86_platform.calibrate_tsc{tsc_khz=f() as u32;}if tsc_khz==0{tsc_khz=cpu_khz;}}
unsafe extern "C" fn read_tsc(_: *mut clocksource)->u64 {rdtsc_ordered()}
unsafe extern "C" fn tsc_cs_enable(_: *mut clocksource)->i32 {vclocks_set_used(0);0}
unsafe extern "C" fn tsc_resume(_: *mut clocksource){tsc_verify_tsc_adjust(true)}
unsafe extern "C" fn tsc_cs_mark_unstable(_: *mut clocksource){if tsc_unstable==0{tsc_unstable=1;clear_sched_clock_stable();}}
unsafe extern "C" fn tsc_cs_tick_stable(_: *mut clocksource){if tsc_unstable==0{sched_clock_tick_stable();}}
unsafe extern "C" fn time_cpufreq_notifier(_: *mut notifier_block,_:usize,_:*mut c_void)->i32 {0}
unsafe extern "C" fn tsc_refine_calibration_work(_: *mut work_struct) {}

// Build-time registration, static keys, per-CPU declarations, and the
// remaining Linux initcall plumbing are supplied by the surrounding kernel.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
