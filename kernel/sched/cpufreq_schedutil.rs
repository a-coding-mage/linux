// SPDX-License-Identifier: GPL-2.0
/* CPUFreq governor based on scheduler-provided CPU utilization data. */
// Kernel dependencies from <uapi/linux/sched/types.h> and "sched.h" are external.

const IOWAIT_BOOST_MIN: u32 = SCHED_CAPACITY_SCALE / 8;

#[repr(C)]
pub struct sugov_tunables { pub attr_set: gov_attr_set, pub rate_limit_us: u32 }
#[repr(C)]
pub struct sugov_policy {
    pub policy: *mut cpufreq_policy, pub tunables: *mut sugov_tunables,
    pub tunables_hook: list_head, pub update_lock: raw_spinlock_t,
    pub last_freq_update_time: u64, pub freq_update_delay_ns: i64,
    pub next_freq: u32, pub cached_raw_freq: u32, pub irq_work: irq_work,
    pub work: kthread_work, pub work_lock: mutex, pub worker: kthread_worker,
    pub thread: *mut task_struct, pub work_in_progress: bool,
    pub limits_changed: bool, pub need_freq_update: bool,
}
#[repr(C)]
pub struct sugov_cpu {
    pub update_util: update_util_data, pub sg_policy: *mut sugov_policy,
    pub cpu: u32, pub iowait_boost_pending: bool, pub iowait_boost: u32,
    pub last_update: u64, pub util: c_ulong, pub bw_min: c_ulong,
    pub bw_max: c_ulong,
    #[cfg(CONFIG_NO_HZ_COMMON)] pub saved_idle_calls: c_ulong,
}

static mut sugov_cpu: per_cpu< sugov_cpu > = per_cpu::new();

unsafe fn sugov_update_rate_limit_us(p: *mut sugov_policy) {
    (*p).freq_update_delay_ns = ((*(*p).tunables).rate_limit_us as i64) * NSEC_PER_USEC as i64;
}
unsafe fn sugov_should_update_freq(p: *mut sugov_policy, time: u64) -> bool {
    if !cpufreq_this_cpu_can_update((*p).policy) { return false; }
    if unlikely(READ_ONCE((*p).limits_changed)) {
        WRITE_ONCE((*p).limits_changed, false); (*p).need_freq_update = true; smp_mb(); return true;
    } else if (*p).need_freq_update { return true; }
    (time.wrapping_sub((*p).last_freq_update_time) as i64) >= (*p).freq_update_delay_ns
}
unsafe fn sugov_update_next_freq(p: *mut sugov_policy, time: u64, next: u32) -> bool {
    if (*p).need_freq_update {
        (*p).need_freq_update = false;
        if (*p).next_freq == next && !cpufreq_driver_test_flags(CPUFREQ_NEED_UPDATE_LIMITS) { return false; }
    } else if (*p).next_freq == next { return false; }
    (*p).next_freq = next; (*p).last_freq_update_time = time; true
}
unsafe fn sugov_deferred_update(p: *mut sugov_policy) { if !(*p).work_in_progress { (*p).work_in_progress=true; irq_work_queue(&mut (*p).irq_work); } }
unsafe fn get_capacity_ref_freq(policy: *mut cpufreq_policy) -> c_ulong {
    let freq = arch_scale_freq_ref((*policy).cpu); if freq != 0 { return freq as c_ulong; }
    if arch_scale_freq_invariant() { return (*policy).cpuinfo.max_freq as c_ulong; }
    (*policy).cur as c_ulong + ((*policy).cur as c_ulong >> 2)
}
unsafe fn get_next_freq(p: *mut sugov_policy, util: c_ulong, max: c_ulong) -> u32 {
    let policy=(*p).policy; let freq=map_util_freq(util,get_capacity_ref_freq(policy),max);
    if freq == (*p).cached_raw_freq && !(*p).need_freq_update { return (*p).next_freq; }
    (*p).cached_raw_freq=freq; cpufreq_driver_resolve_freq(policy,freq)
}
pub unsafe fn sugov_effective_cpu_perf(_cpu: i32, mut actual: c_ulong, min: c_ulong, mut max: c_ulong) -> c_ulong {
    actual=map_util_perf(actual); if actual < max { max=actual; } if min > max { min } else { max }
}
unsafe fn sugov_get_util(c: *mut sugov_cpu, boost: c_ulong) {
    let mut min=0; let mut max=0; let mut util=scx_cpuperf_target((*c).cpu);
    if !scx_switched_all() { util += cpu_util_cfs_boost((*c).cpu); }
    util=effective_cpu_util((*c).cpu,util,&mut min,&mut max); util=if util>boost {util}else{boost};
    (*c).bw_min=min; (*c).bw_max=max; (*c).util=sugov_effective_cpu_perf((*c).cpu as i32,util,min,max);
}
unsafe fn sugov_iowait_reset(c:*mut sugov_cpu,time:u64,set:bool)->bool {
    if (time.wrapping_sub((*c).last_update) as i64) <= TICK_NSEC as i64 { return false; }
    (*c).iowait_boost=if set {IOWAIT_BOOST_MIN}else{0}; (*c).iowait_boost_pending=set; true
}
unsafe fn sugov_iowait_boost(c:*mut sugov_cpu,time:u64,flags:u32) {
    let set=(flags&SCHED_CPUFREQ_IOWAIT)!=0;
    if (*c).iowait_boost!=0 && sugov_iowait_reset(c,time,set) {return;} if !set{return;}
    if (*c).iowait_boost_pending{return;} (*c).iowait_boost_pending=true;
    if (*c).iowait_boost!=0 { (*c).iowait_boost=((*c).iowait_boost<<1).min(SCHED_CAPACITY_SCALE); } else {(*c).iowait_boost=IOWAIT_BOOST_MIN;}
}
unsafe fn sugov_iowait_apply(c:*mut sugov_cpu,time:u64,max_cap:c_ulong)->c_ulong {
    if (*c).iowait_boost==0{return 0;} if sugov_iowait_reset(c,time,false){return 0;}
    if !(*c).iowait_boost_pending {(*c).iowait_boost>>=1;if (*c).iowait_boost<IOWAIT_BOOST_MIN{(*c).iowait_boost=0;return 0;}}
    (*c).iowait_boost_pending=false; ((*c).iowait_boost as c_ulong*max_cap)>>SCHED_CAPACITY_SHIFT
}

#[cfg(CONFIG_NO_HZ_COMMON)] unsafe fn sugov_hold_freq(c:*mut sugov_cpu)->bool {
    if scx_switched_all() || uclamp_rq_is_capped(cpu_rq((*c).cpu)){return false;}
    let calls=tick_nohz_get_idle_calls_cpu((*c).cpu); let ret=calls==(*c).saved_idle_calls; (*c).saved_idle_calls=calls; ret
}
#[cfg(not(CONFIG_NO_HZ_COMMON))] unsafe fn sugov_hold_freq(_c:*mut sugov_cpu)->bool {false}
unsafe fn ignore_dl_rate_limit(c:*mut sugov_cpu){if cpu_bw_dl(cpu_rq((*c).cpu))>(*c).bw_min{(*(*c).sg_policy).need_freq_update=true;}}
unsafe fn sugov_update_single_common(c:*mut sugov_cpu,time:u64,max_cap:c_ulong,flags:u32)->bool{
    sugov_iowait_boost(c,time,flags);(*c).last_update=time;ignore_dl_rate_limit(c);let p=(*c).sg_policy;if !sugov_should_update_freq(p,time){return false;}let b=sugov_iowait_apply(c,time,max_cap);sugov_get_util(c,b);true
}
unsafe fn sugov_update_single_freq(hook:*mut update_util_data,time:u64,flags:u32){
    let c=container_of!(hook,sugov_cpu,update_util);let p=(*c).sg_policy;let cached=(*p).cached_raw_freq;let max=arch_scale_cpu_capacity((*c).cpu) as c_ulong;if !sugov_update_single_common(c,time,max,flags){return;}
    let mut next=get_next_freq(p,(*c).util,max);if sugov_hold_freq(c)&&next<(*p).next_freq&&!(*p).need_freq_update{next=(*p).next_freq;(*p).cached_raw_freq=cached;}if !sugov_update_next_freq(p,time,next){return;}
    if (*(*p).policy).fast_switch_enabled{cpufreq_driver_fast_switch((*p).policy,next);}else{raw_spin_lock(&mut (*p).update_lock);sugov_deferred_update(p);raw_spin_unlock(&mut (*p).update_lock);}
}
unsafe fn sugov_update_single_perf(hook:*mut update_util_data,time:u64,flags:u32){if !arch_scale_freq_invariant(){sugov_update_single_freq(hook,time,flags);return;}let c=container_of!(hook,sugov_cpu,update_util);let p=(*c).sg_policy;let prev=(*c).util;let max=arch_scale_cpu_capacity((*c).cpu) as c_ulong;if !sugov_update_single_common(c,time,max,flags){return;}if sugov_hold_freq(c)&&(*c).util<prev{(*c).util=prev;}cpufreq_driver_adjust_perf((*p).policy,(*c).bw_min,(*c).util,(*c).bw_max,max);(*p).need_freq_update=false;(*p).last_freq_update_time=time;}
unsafe fn sugov_next_freq_shared(c:*mut sugov_cpu,time:u64)->u32{let p=(*c).sg_policy;let max=arch_scale_cpu_capacity((*c).cpu) as c_ulong;let mut util=0;for_each_cpu!(j,(*(*p).policy).cpus,{let jc=&mut per_cpu!(sugov_cpu,j);let b=sugov_iowait_apply(jc,time,max);sugov_get_util(jc,b);util=util.max((*jc).util);});get_next_freq(p,util,max)}
unsafe fn sugov_update_shared(hook:*mut update_util_data,time:u64,flags:u32){let c=container_of!(hook,sugov_cpu,update_util);let p=(*c).sg_policy;raw_spin_lock(&mut (*p).update_lock);sugov_iowait_boost(c,time,flags);(*c).last_update=time;ignore_dl_rate_limit(c);if sugov_should_update_freq(p,time){let n=sugov_next_freq_shared(c,time);if sugov_update_next_freq(p,time,n){if (*(*p).policy).fast_switch_enabled{cpufreq_driver_fast_switch((*p).policy,n);}else{sugov_deferred_update(p);}}}raw_spin_unlock(&mut (*p).update_lock);}
unsafe fn sugov_work(work:*mut kthread_work){let p=container_of!(work,sugov_policy,work);let f;let mut flags=0;raw_spin_lock_irqsave(&mut (*p).update_lock,&mut flags);f=(*p).next_freq;(*p).work_in_progress=false;raw_spin_unlock_irqrestore(&mut (*p).update_lock,flags);mutex_lock(&mut (*p).work_lock);__cpufreq_driver_target((*p).policy,f,CPUFREQ_RELATION_L);mutex_unlock(&mut (*p).work_lock);}
unsafe fn sugov_irq_work(w:*mut irq_work){let p=container_of!(w,sugov_policy,irq_work);kthread_queue_work(&mut (*p).worker,&mut (*p).work);}

// Sysfs and governor lifecycle declarations are retained below as kernel-facing Rust items.
extern "C" { static mut global_tunables:*mut sugov_tunables; }
unsafe fn sugov_policy_alloc(policy:*mut cpufreq_policy)->*mut sugov_policy{let p=kzalloc_obj::<sugov_policy>();if p.is_null(){return core::ptr::null_mut();}(*p).policy=policy;raw_spin_lock_init(&mut (*p).update_lock);p}
unsafe fn sugov_policy_free(p:*mut sugov_policy){kfree(p);}
unsafe fn sugov_tunables_free(kobj:*mut kobject){let a=to_gov_attr_set(kobj);kfree(to_sugov_tunables(a));}
unsafe fn rate_limit_us_show(a:*mut gov_attr_set,buf:*mut c_char)->isize{sysfs_emit(buf,"%u\n",(*to_sugov_tunables(a)).rate_limit_us)}
unsafe fn rate_limit_us_store(a:*mut gov_attr_set,buf:*const c_char,count:usize)->isize{let t=to_sugov_tunables(a);let mut v=0u32;if kstrtouint(buf,10,&mut v)!=0{return -EINVAL as isize;}(*t).rate_limit_us=v;list_for_each_entry!(p,&mut (*a).policy_list,tunables_hook,{sugov_update_rate_limit_us(p);});count as isize}
unsafe fn sugov_kthread_create(p:*mut sugov_policy)->i32{if (*(*p).policy).fast_switch_enabled{return 0;}kthread_init_work(&mut (*p).work,sugov_work);kthread_init_worker(&mut (*p).worker);let t=kthread_create(kthread_worker_fn,&mut (*p).worker,"sugov:%d",cpumask_first((*(*p).policy).related_cpus));if is_err(t){return ptr_err(t);}let ret=sched_setattr_nocheck(t,core::ptr::null_mut());if ret!=0{kthread_stop(t);return ret;}(*p).thread=t;init_irq_work(&mut (*p).irq_work,sugov_irq_work);mutex_init(&mut (*p).work_lock);wake_up_process(t);0}
unsafe fn sugov_kthread_stop(p:*mut sugov_policy){if (*(*p).policy).fast_switch_enabled{return;}kthread_flush_worker(&mut (*p).worker);kthread_stop((*p).thread);mutex_destroy(&mut (*p).work_lock);}
unsafe fn sugov_tunables_alloc(p:*mut sugov_policy)->*mut sugov_tunables{let t=kzalloc_obj::<sugov_tunables>();if !t.is_null(){gov_attr_set_init(&mut (*t).attr_set,&mut (*p).tunables_hook);if !have_governor_per_policy(){global_tunables=t;}}t}
unsafe fn sugov_clear_global_tunables(){if !have_governor_per_policy(){global_tunables=core::ptr::null_mut();}}
unsafe fn sugov_init(policy:*mut cpufreq_policy)->i32{if !(*policy).governor_data.is_null(){return -EBUSY;}cpufreq_enable_fast_switch(policy);let p=sugov_policy_alloc(policy);if p.is_null(){cpufreq_disable_fast_switch(policy);return -ENOMEM;}let ret=sugov_kthread_create(p);if ret!=0{sugov_policy_free(p);cpufreq_disable_fast_switch(policy);return ret;}let t=sugov_tunables_alloc(p);if t.is_null(){sugov_kthread_stop(p);sugov_policy_free(p);cpufreq_disable_fast_switch(policy);return -ENOMEM;}(*t).rate_limit_us=cpufreq_policy_transition_delay_us(policy);(*policy).governor_data=p;(*p).tunables=t;em_rebuild_sched_domains();0}
unsafe fn sugov_exit(policy:*mut cpufreq_policy){let p=(*policy).governor_data as *mut sugov_policy;let t=(*p).tunables;(*policy).governor_data=core::ptr::null_mut();sugov_clear_global_tunables();sugov_kthread_stop(p);sugov_policy_free(p);kfree(t);cpufreq_disable_fast_switch(policy);em_rebuild_sched_domains();}
unsafe fn sugov_start(policy:*mut cpufreq_policy)->i32{let p=(*policy).governor_data as *mut sugov_policy;sugov_update_rate_limit_us(p);(*p).last_freq_update_time=0;(*p).next_freq=0;(*p).work_in_progress=false;(*p).limits_changed=false;(*p).cached_raw_freq=0;(*p).need_freq_update=cpufreq_driver_test_flags(CPUFREQ_NEED_UPDATE_LIMITS);for_each_cpu!(cpu,(*policy).cpus,{let c=&mut per_cpu!(sugov_cpu,c);core::ptr::write_bytes(c,0,1);(*c).cpu=cpu;(*c).sg_policy=p;});for_each_cpu!(cpu,(*policy).cpus,{let c=&mut per_cpu!(sugov_cpu,c);cpufreq_add_update_util_hook(cpu,&mut (*c).update_util,sugov_update_single_freq);});0}
unsafe fn sugov_stop(policy:*mut cpufreq_policy){for_each_cpu!(cpu,(*policy).cpus,{cpufreq_remove_update_util_hook(cpu);});synchronize_rcu();}
unsafe fn sugov_limits(policy:*mut cpufreq_policy){let p=(*policy).governor_data as *mut sugov_policy;smp_wmb();WRITE_ONCE((*p).limits_changed,true);}
unsafe fn cpufreq_default_governor()->*mut cpufreq_governor{&mut schedutil_gov}
unsafe fn sugov_is_governor(policy:*mut cpufreq_policy)->bool{(*policy).governor==&mut schedutil_gov}
static mut schedutil_gov:cpufreq_governor=cpufreq_governor::new("schedutil",sugov_init,sugov_exit,sugov_start,sugov_stop,sugov_limits);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
