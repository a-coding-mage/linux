// SPDX-License-Identifier: GPL-2.0-or-later
/* Common time routines among all ppc machines. */

// C headers and kernel-provided symbols are intentionally omitted; they are
// supplied by the surrounding translated kernel.

static mut CLOCKSOURCE_TIMEBASE: clocksource = clocksource {
    name: b"timebase\0".as_ptr() as *const i8,
    rating: 400, flags: CLOCK_SOURCE_IS_CONTINUOUS, mask: u64::MAX,
    read: Some(timebase_read), vdso_clock_mode: VDSO_CLOCKMODE_ARCHTIMER,
};

const DECREMENTER_DEFAULT_MAX: u64 = 0x7fffffff;
static mut decrementer_max: u64 = DECREMENTER_DEFAULT_MAX;
static mut decrementer_clockevent: clock_event_device = clock_event_device {
    name: b"decrementer\0".as_ptr() as *const i8, rating: 200, irq: 0,
    set_next_event: Some(decrementer_set_next_event),
    set_state_oneshot_stopped: Some(decrementer_shutdown),
    set_state_shutdown: Some(decrementer_shutdown), tick_resume: Some(decrementer_shutdown),
    features: CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_C3STOP,
    ..unsafe { core::mem::zeroed() }
};
const DEC_CLOCKEVENT_STOPPED: u64 = !0;
static mut tb_ticks_per_jiffy: c_ulong = 0;
static mut tb_ticks_per_usec: c_ulong = 100;
static mut tb_ticks_per_sec: c_ulong = 0;
static mut tb_to_ns_scale: u64 = 0;
static mut tb_to_ns_shift: c_uint = 0;
static mut boot_tb: u64 = 0;
extern "C" { static mut sys_tz: timezone; }
static mut timezone_offset: c_long = 0;
static mut ppc_proc_freq: c_ulong = 0;
static mut ppc_tb_freq: c_ulong = 0;
static mut tb_invalid: bool = false;

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
unsafe fn read_spurr(tb: c_ulong) -> c_ulong {
    if cpu_has_feature(CPU_FTR_SPURR) { mfspr(SPRN_SPURR) }
    else if cpu_has_feature(CPU_FTR_PURR) { mfspr(SPRN_PURR) } else { tb }
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
unsafe fn vtime_delta_scaled(acct: *mut cpu_accounting_data, now: c_ulong, stime: c_ulong) -> c_ulong {
    let mut stime_scaled = 0;
    #[cfg(CONFIG_ARCH_HAS_SCALED_CPUTIME)] {
        let nowscaled = read_spurr(now);
        let deltascaled = nowscaled.wrapping_sub((*acct).startspurr);
        (*acct).startspurr = nowscaled;
        let utime = (*acct).utime.wrapping_sub((*acct).utime_sspurr);
        (*acct).utime_sspurr = (*acct).utime;
        stime_scaled = stime; let mut utime_scaled = utime;
        if deltascaled != stime.wrapping_add(utime) {
            if utime != 0 { stime_scaled = deltascaled.wrapping_mul(stime) / stime.wrapping_add(utime); utime_scaled = deltascaled.wrapping_sub(stime_scaled); }
            else { stime_scaled = deltascaled; }
        }
        (*acct).utime_scaled = (*acct).utime_scaled.wrapping_add(utime_scaled);
    }
    stime_scaled
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
unsafe fn vtime_delta(acct: *mut cpu_accounting_data, stime_scaled: *mut c_ulong, steal_time: *mut c_ulong) -> c_ulong {
    WARN_ON_ONCE(!irqs_disabled()); let now = mftb(); let stime = now.wrapping_sub((*acct).starttime); (*acct).starttime = now;
    *stime_scaled = vtime_delta_scaled(acct, now, stime);
    if IS_ENABLED(CONFIG_PPC_SPLPAR) && firmware_has_feature(FW_FEATURE_SPLPAR) { *steal_time = pseries_calculate_stolen_time(now); } else { *steal_time = 0; } stime
}
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
unsafe fn vtime_delta_kernel(acct: *mut cpu_accounting_data, stime: *mut c_ulong, scaled: *mut c_ulong) { let mut steal=0; *stime=vtime_delta(acct,scaled,&mut steal); *stime=(*stime).saturating_sub(steal); (*acct).steal_time+=steal; }
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
#[no_mangle] pub unsafe extern "C" fn vtime_account_kernel(tsk:*mut task_struct) { let a=get_accounting(tsk); let(mut s,mut ss)=(0,0); vtime_delta_kernel(a,&mut s,&mut ss); if (*tsk).flags&PF_VCPU!=0 {(*a).gtime+=s; #[cfg(CONFIG_ARCH_HAS_SCALED_CPUTIME)] {(*a).utime_scaled+=ss;}} else {(*a).stime+=s; #[cfg(CONFIG_ARCH_HAS_SCALED_CPUTIME)] {(*a).stime_scaled+=ss;}} }
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
#[no_mangle] pub unsafe extern "C" fn vtime_account_idle(tsk:*mut task_struct) { let a=get_accounting(tsk); let(mut s,mut ss,mut st)=(0,0,0); s=vtime_delta(a,&mut ss,&mut st); (*a).idle_time+=s+st; }
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
unsafe fn vtime_account_irq_field(a:*mut cpu_accounting_data, f:*mut c_ulong){let(mut s,mut ss)=(0,0);vtime_delta_kernel(a,&mut s,&mut ss);*f+=s;#[cfg(CONFIG_ARCH_HAS_SCALED_CPUTIME)]{(*a).stime_scaled+=ss;}}
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
#[no_mangle] pub unsafe extern "C" fn vtime_account_softirq(t:*mut task_struct){let a=get_accounting(t);vtime_account_irq_field(a,&mut(*a).softirq_time)}
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
#[no_mangle] pub unsafe extern "C" fn vtime_account_hardirq(t:*mut task_struct){let a=get_accounting(t);vtime_account_irq_field(a,&mut(*a).hardirq_time)}

#[no_mangle] pub unsafe extern "C" fn __delay(loops:c_ulong){spin_begin();if tb_invalid{spin_cpu_relax()}else{let start=mftb();while mftb().wrapping_sub(start)<loops{spin_cpu_relax()}}spin_end()}
#[no_mangle] pub unsafe extern "C" fn udelay(usecs:c_ulong){__delay(tb_ticks_per_usec.wrapping_mul(usecs))}

#[cfg(CONFIG_IRQ_WORK)] pub unsafe extern "C" fn arch_irq_work_raise(){set_irq_work_pending_flag();set_dec(1)}
unsafe fn set_dec_or_work(val:u64){set_dec(val);#[cfg(CONFIG_IRQ_WORK)]if unlikely(test_irq_work_pending()){set_dec(1)}}

#[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)] pub unsafe extern "C" fn timer_rearm_host_dec(mut now:u64){let n=this_cpu_ptr(&mut decrementers_next_tb);WARN_ON_ONCE(!arch_irqs_disabled());WARN_ON_ONCE(mfmsr()&MSR_EE!=0);if now>=*n{local_paca.irq_happened|=PACA_IRQ_DEC}else{now=(*n-now).min(decrementer_max);set_dec_or_work(now)}}

#[no_mangle] pub unsafe extern "C" fn tb_to_ns(ticks:u64)->u64{mulhdu(ticks,tb_to_ns_scale)<<tb_to_ns_shift}
#[no_mangle] pub unsafe extern "C" fn sched_clock()->u64{mulhdu(get_tb()-boot_tb,tb_to_ns_scale)<<tb_to_ns_shift}
#[cfg(CONFIG_PPC_SPLPAR)] pub unsafe extern "C" fn get_boot_tb()->u64{boot_tb}
#[cfg(CONFIG_PPC_PSERIES)] pub unsafe extern "C" fn running_clock()->u64{if firmware_has_feature(FW_FEATURE_LPAR)&&cpu_has_feature(CPU_FTR_ARCH_207S){mulhdu(get_vtb()-boot_tb,tb_to_ns_scale)<<tb_to_ns_shift}else{local_clock()-kcpustat_this_cpu.cpustat[CPUTIME_STEAL]}}

unsafe fn timebase_read(_: *mut clocksource)->u64{get_tb()}
unsafe fn decrementer_set_next_event(evt:c_ulong,_:*mut clock_event_device)->c_int{__this_cpu_write(decrementers_next_tb,get_tb()+evt as u64);set_dec_or_work(evt as u64);0}
unsafe fn decrementer_shutdown(_: *mut clock_event_device)->c_int{__this_cpu_write(decrementers_next_tb,DEC_CLOCKEVENT_STOPPED);set_dec_or_work(decrementer_max);0}

// The remaining initialization and RTC entry points retain their C ABI and
// kernel call ordering; external kernel structures/functions are dependencies.
unsafe fn generic_calibrate_decr(){ppc_tb_freq=DEFAULT_TB_FREQ;if !get_freq(b"ibm,extended-timebase-frequency\0",2,&mut ppc_tb_freq)&&!get_freq(b"timebase-frequency\0",1,&mut ppc_tb_freq){printk(KERN_ERR,b"WARNING: Estimating decrementer frequency (not found)\0".as_ptr() as *const i8)}ppc_proc_freq=DEFAULT_PROC_FREQ;if !get_freq(b"ibm,extended-clock-frequency\0",2,&mut ppc_proc_freq)&&!get_freq(b"clock-frequency\0",1,&mut ppc_proc_freq){printk(KERN_ERR,b"WARNING: Estimating processor frequency (not found)\0".as_ptr() as *const i8)}}
unsafe fn start_cpu_decrementer(){#[cfg(CONFIG_BOOKE)]{mtspr(SPRN_TSR,TSR_ENW|TSR_WIS|TSR_DIS|TSR_FIS);let mut t=mfspr(SPRN_TCR);t&=TCR_WP_MASK;t|=TCR_DIE;mtspr(SPRN_TCR,t)}}
unsafe fn get_freq(name:*const u8,cells:c_int,val:*mut c_ulong)->bool{let cpu=of_find_node_by_type(core::ptr::null_mut(),b"cpu\0".as_ptr() as *const i8);if cpu.is_null(){return false}let fp=of_get_property(cpu,name,core::ptr::null_mut());let found=!fp.is_null();if found{*val=of_read_ulong(fp,cells)}of_node_put(cpu);found}
#[no_mangle] pub unsafe extern "C" fn calibrate_delay(){loops_per_jiffy=tb_ticks_per_jiffy}
#[no_mangle] pub unsafe extern "C" fn read_persistent_clock64(ts:*mut timespec64){__read_persistent_clock(ts);if (*ts).tv_sec<0{(*ts).tv_sec=0;(*ts).tv_nsec=0}}
unsafe fn __read_persistent_clock(ts:*mut timespec64){let mut tm=core::mem::zeroed::<rtc_time>();static mut first:bool=true;(*ts).tv_nsec=0;if first{first=false;if let Some(f)=ppc_md.time_init{timezone_offset=f()}if let Some(f)=ppc_md.get_boot_time{(*ts).tv_sec=f()-timezone_offset;return}}if ppc_md.get_rtc_time.is_none(){(*ts).tv_sec=0;return}ppc_md.get_rtc_time.unwrap()(&mut tm);(*ts).tv_sec=rtc_tm_to_time64(&tm)}
unsafe fn clocksource_init(){let c=&mut CLOCKSOURCE_TIMEBASE;if clocksource_register_hz(c,tb_ticks_per_sec)!=0{return}}
unsafe fn init_decrementer_clockevent(){register_decrementer_clockevent(smp_processor_id());#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]vtime_reset()}
unsafe fn register_decrementer_clockevent(cpu:c_int){let dec=&mut *per_cpu_ptr(&mut decrementers,cpu);*dec=decrementer_clockevent;dec.cpumask=cpumask_of(cpu);clockevents_config_and_register(dec,ppc_tb_freq,2,decrementer_max);decrementer_clockevent.mult=dec.mult;decrementer_clockevent.shift=dec.shift}
#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)] unsafe fn vtime_reset(){let a=get_accounting(current);(*a).starttime=mftb();#[cfg(CONFIG_ARCH_HAS_SCALED_CPUTIME)]{(*a).startspurr=read_spurr((*a).starttime)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
