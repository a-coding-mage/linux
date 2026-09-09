// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * kernel/stop_machine.c
 *
 * Copyright (C) 2008, 2005 IBM Corporation.
 * Copyright (C) 2008, 2005 Rusty Russell rusty@rustcorp.com.au
 * Copyright (C) 2010 SUSE Linux Products GmbH
 * Copyright (C) 2010 Tejun Heo <tj@kernel.org>
 */

// Kernel dependencies supplied by other translation units are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct cpu_stop_done {
    pub nr_todo: atomic_t,
    pub ret: i32,
    pub completion: completion,
}

#[repr(C)]
pub struct cpu_stopper {
    pub thread: *mut task_struct,
    pub lock: raw_spinlock_t,
    pub enabled: bool,
    pub works: list_head,
    pub stop_work: cpu_stop_work,
    pub caller: usize,
    pub fn_: cpu_stop_fn_t,
}

static mut cpu_stopper: per_cpu<cpu_stopper> = DEFINE_PER_CPU!();
static mut stop_machine_initialized: bool = false;

pub unsafe fn print_stop_info(log_lvl: *const i8, task: *mut task_struct) {
    let stopper = per_cpu_ptr(&mut cpu_stopper, task_cpu(task));
    if task != (*stopper).thread { return; }
    printk(b"%sStopper: %pS <- %pS\n\0".as_ptr() as *const i8,
           log_lvl, (*stopper).fn_, (*stopper).caller as *mut core::ffi::c_void);
}

static mut stop_cpus_mutex: mutex = DEFINE_MUTEX!();
static mut stop_cpus_in_progress: bool = false;

unsafe fn cpu_stop_init_done(done: *mut cpu_stop_done, nr_todo: u32) {
    memset(done as *mut core::ffi::c_void, 0, core::mem::size_of::<cpu_stop_done>());
    atomic_set(&mut (*done).nr_todo, nr_todo);
    init_completion(&mut (*done).completion);
}

unsafe fn cpu_stop_signal_done(done: *mut cpu_stop_done) {
    if atomic_dec_and_test(&mut (*done).nr_todo) { complete(&mut (*done).completion); }
}

unsafe fn __cpu_stop_queue_work(stopper: *mut cpu_stopper, work: *mut cpu_stop_work) {
    list_add_tail(&mut (*work).list, &mut (*stopper).works);
}

unsafe fn cpu_stop_queue_work(cpu: u32, work: *mut cpu_stop_work) -> bool {
    let stopper = per_cpu_ptr(&mut cpu_stopper, cpu);
    let mut flags = 0usize;
    preempt_disable();
    raw_spin_lock_irqsave(&mut (*stopper).lock, &mut flags);
    let enabled = (*stopper).enabled;
    if enabled { __cpu_stop_queue_work(stopper, work); }
    else if !(*work).done.is_null() { cpu_stop_signal_done((*work).done); }
    raw_spin_unlock_irqrestore(&mut (*stopper).lock, flags);
    if enabled { wake_up_process((*stopper).thread); }
    preempt_enable();
    enabled
}

pub unsafe fn stop_one_cpu(cpu: u32, fn_: cpu_stop_fn_t, arg: *mut core::ffi::c_void) -> i32 {
    let mut done = core::mem::zeroed::<cpu_stop_done>();
    let mut work = cpu_stop_work { fn_, arg, done: &mut done, caller: _RET_IP!(), ..core::mem::zeroed() };
    cpu_stop_init_done(&mut done, 1);
    if !cpu_stop_queue_work(cpu, &mut work) { return -2; }
    cond_resched();
    wait_for_completion(&mut done.completion);
    done.ret
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum multi_stop_state { MULTI_STOP_NONE, MULTI_STOP_PREPARE, MULTI_STOP_DISABLE_IRQ, MULTI_STOP_RUN, MULTI_STOP_EXIT }

#[repr(C)]
pub struct multi_stop_data {
    pub fn_: cpu_stop_fn_t,
    pub data: *mut core::ffi::c_void,
    pub num_threads: u32,
    pub active_cpus: *const cpumask,
    pub state: multi_stop_state,
    pub thread_ack: atomic_t,
}

unsafe fn set_state(msdata: *mut multi_stop_data, newstate: multi_stop_state) {
    atomic_set(&mut (*msdata).thread_ack, (*msdata).num_threads);
    smp_wmb();
    WRITE_ONCE!((*msdata).state, newstate);
}

unsafe fn ack_state(msdata: *mut multi_stop_data) {
    if atomic_dec_and_test(&mut (*msdata).thread_ack) {
        set_state(msdata, core::mem::transmute((*msdata).state as u32 + 1));
    }
}

#[no_mangle]
pub unsafe extern "C" fn stop_machine_yield(_cpumask: *const cpumask) { cpu_relax(); }

unsafe fn multi_cpu_stop(data: *mut core::ffi::c_void) -> i32 {
    let msdata = data as *mut multi_stop_data;
    let mut newstate;
    let mut curstate = multi_stop_state::MULTI_STOP_NONE;
    let cpu = smp_processor_id();
    let mut err = 0;
    let cpumask;
    let mut flags = 0usize;
    let is_active;
    local_save_flags(&mut flags);
    if (*msdata).active_cpus.is_null() {
        cpumask = cpu_online_mask;
        is_active = cpu == cpumask_first(cpumask);
    } else { cpumask = (*msdata).active_cpus; is_active = cpumask_test_cpu(cpu, cpumask); }
    loop {
        stop_machine_yield(cpumask);
        newstate = READ_ONCE!((*msdata).state);
        if newstate as u32 != curstate as u32 {
            curstate = newstate;
            match curstate {
                multi_stop_state::MULTI_STOP_DISABLE_IRQ => { local_irq_disable(); hard_irq_disable(); }
                multi_stop_state::MULTI_STOP_RUN => if is_active { err = ((*msdata).fn_)(*msdata).data; },
                _ => {}
            }
            ack_state(msdata);
        } else if curstate as u32 > multi_stop_state::MULTI_STOP_PREPARE as u32 { touch_nmi_watchdog(); rcu_momentary_eqs(); }
        if curstate as u32 == multi_stop_state::MULTI_STOP_EXIT as u32 { break; }
    }
    local_irq_restore(flags);
    err
}

unsafe fn cpu_stop_queue_two_works(cpu1: i32, work1: *mut cpu_stop_work, cpu2: i32, work2: *mut cpu_stop_work) -> i32 {
    let stopper1 = per_cpu_ptr(&mut cpu_stopper, cpu1 as u32);
    let stopper2 = per_cpu_ptr(&mut cpu_stopper, cpu2 as u32);
    loop {
        preempt_disable(); raw_spin_lock_irq(&mut (*stopper1).lock); raw_spin_lock_nested(&mut (*stopper2).lock, SINGLE_DEPTH_NESTING);
        if !(*stopper1).enabled || !(*stopper2).enabled { raw_spin_unlock(&mut (*stopper2).lock); raw_spin_unlock_irq(&mut (*stopper1).lock); preempt_enable(); return -2; }
        if stop_cpus_in_progress { raw_spin_unlock(&mut (*stopper2).lock); raw_spin_unlock_irq(&mut (*stopper1).lock); preempt_enable(); while stop_cpus_in_progress { cpu_relax(); } continue; }
        __cpu_stop_queue_work(stopper1, work1); __cpu_stop_queue_work(stopper2, work2);
        raw_spin_unlock(&mut (*stopper2).lock); raw_spin_unlock_irq(&mut (*stopper1).lock);
        wake_up_process((*stopper1).thread); wake_up_process((*stopper2).thread); preempt_enable(); return 0;
    }
}

pub unsafe fn stop_two_cpus(cpu1: u32, cpu2: u32, fn_: cpu_stop_fn_t, arg: *mut core::ffi::c_void) -> i32 {
    let mut done = core::mem::zeroed::<cpu_stop_done>();
    let mut msdata = multi_stop_data { fn_, data: arg, num_threads: 2, active_cpus: cpumask_of(cpu1), state: multi_stop_state::MULTI_STOP_NONE, thread_ack: core::mem::zeroed() };
    let mut work1 = cpu_stop_work { fn_: multi_cpu_stop, arg: &mut msdata as *mut _ as *mut _, done: &mut done, caller: _RET_IP!(), ..core::mem::zeroed() };
    let mut work2 = work1;
    cpu_stop_init_done(&mut done, 2); set_state(&mut msdata, multi_stop_state::MULTI_STOP_PREPARE);
    let (a,b) = if cpu1 > cpu2 {(cpu2,cpu1)} else {(cpu1,cpu2)};
    if cpu_stop_queue_two_works(a as i32, &mut work1, b as i32, &mut work2) != 0 { return -2; }
    wait_for_completion(&mut done.completion); done.ret
}

pub unsafe fn stop_one_cpu_nowait(cpu: u32, fn_: cpu_stop_fn_t, arg: *mut core::ffi::c_void, work_buf: *mut cpu_stop_work) {
    *work_buf = cpu_stop_work { fn_, arg, caller: _RET_IP!(), ..core::mem::zeroed() };
    WARN_ON_ONCE!(!cpu_stop_queue_work(cpu, work_buf));
}

unsafe fn queue_stop_cpus_work(cpumask: *const cpumask, fn_: cpu_stop_fn_t, arg: *mut core::ffi::c_void, done: *mut cpu_stop_done) -> bool {
    let mut queued = false; preempt_disable(); stop_cpus_in_progress = true; barrier!();
    for_each_cpu!(cpu, cpumask, { let work = &mut (*per_cpu_ptr(&mut cpu_stopper, cpu)).stop_work; (*work).fn_ = fn_; (*work).arg = arg; (*work).done = done; (*work).caller = _RET_IP!(); if cpu_stop_queue_work(cpu, work) { queued = true; } });
    barrier!(); stop_cpus_in_progress = false; preempt_enable(); queued
}

unsafe fn __stop_cpus(cpumask: *const cpumask, fn_: cpu_stop_fn_t, arg: *mut core::ffi::c_void) -> i32 {
    let mut done = core::mem::zeroed::<cpu_stop_done>(); cpu_stop_init_done(&mut done, cpumask_weight(cpumask));
    if !queue_stop_cpus_work(cpumask, fn_, arg, &mut done) { return -2; } wait_for_completion(&mut done.completion); done.ret
}

unsafe fn stop_cpus(cpumask: *const cpumask, fn_: cpu_stop_fn_t, arg: *mut core::ffi::c_void) -> i32 { mutex_lock(&mut stop_cpus_mutex); let ret = __stop_cpus(cpumask, fn_, arg); mutex_unlock(&mut stop_cpus_mutex); ret }

unsafe fn cpu_stop_should_run(cpu: u32) -> i32 { let stopper = per_cpu_ptr(&mut cpu_stopper, cpu); let mut flags=0; raw_spin_lock_irqsave(&mut (*stopper).lock,&mut flags); let run=!list_empty(&(*stopper).works); raw_spin_unlock_irqrestore(&mut (*stopper).lock,flags); run as i32 }

unsafe fn cpu_stopper_thread(cpu: u32) {
    let stopper = per_cpu_ptr(&mut cpu_stopper, cpu);
    loop { let mut work: *mut cpu_stop_work = core::ptr::null_mut(); raw_spin_lock_irq(&mut (*stopper).lock); if !list_empty(&(*stopper).works) { work=list_first_entry(&mut (*stopper).works); list_del_init(&mut (*work).list); } raw_spin_unlock_irq(&mut (*stopper).lock);
        if work.is_null() { return; } let fn_=(*work).fn_; let done=(*work).done; (*stopper).caller=(*work).caller; (*stopper).fn_=fn_; preempt_count_inc(); let ret=fn_((*work).arg); if !done.is_null() { if ret != 0 {(*done).ret=ret;} cpu_stop_signal_done(done); } preempt_count_dec(); (*stopper).fn_=core::mem::zeroed(); (*stopper).caller=0; WARN_ONCE!(preempt_count()!=0, "cpu_stop: leaked preempt count");
    }
}

pub unsafe fn stop_machine_park(cpu: i32) { let stopper=per_cpu_ptr(&mut cpu_stopper,cpu as u32); (*stopper).enabled=false; kthread_park((*stopper).thread); }
unsafe fn cpu_stop_create(cpu: u32) { sched_set_stop_task(cpu, (*per_cpu_ptr(&mut cpu_stopper,cpu)).thread); }
unsafe fn cpu_stop_park(cpu: u32) { WARN_ON!(!list_empty(&(*per_cpu_ptr(&mut cpu_stopper,cpu)).works)); }
pub unsafe fn stop_machine_unpark(cpu: i32) { let stopper=per_cpu_ptr(&mut cpu_stopper,cpu as u32); (*stopper).enabled=true; kthread_unpark((*stopper).thread); }

static mut cpu_stop_threads: smp_hotplug_thread = smp_hotplug_thread { store: core::ptr::null_mut(), thread_should_run: cpu_stop_should_run, thread_fn: cpu_stopper_thread, thread_comm: b"migration/%u\0".as_ptr(), create: cpu_stop_create, park: cpu_stop_park, selfparking: true };

unsafe fn cpu_stop_init() -> i32 { for_each_possible_cpu!(cpu, { let s=per_cpu_ptr(&mut cpu_stopper,cpu); raw_spin_lock_init(&mut (*s).lock); INIT_LIST_HEAD!(&mut (*s).works); }); BUG_ON!(smpboot_register_percpu_thread(&mut cpu_stop_threads)); stop_machine_unpark(raw_smp_processor_id() as i32); stop_machine_initialized=true; 0 }
early_initcall!(cpu_stop_init);

pub unsafe fn stop_machine_cpuslocked(fn_: cpu_stop_fn_t, data: *mut core::ffi::c_void, cpus: *const cpumask) -> i32 { let mut msdata=multi_stop_data{fn_,data,num_threads:num_online_cpus(),active_cpus:cpus,state:multi_stop_state::MULTI_STOP_NONE,thread_ack:core::mem::zeroed()}; lockdep_assert_cpus_held!(); if !stop_machine_initialized { let mut flags=0; WARN_ON_ONCE!(msdata.num_threads!=1); local_irq_save(&mut flags); hard_irq_disable(); let ret=fn_(data); local_irq_restore(flags); return ret; } set_state(&mut msdata,multi_stop_state::MULTI_STOP_PREPARE); stop_cpus(cpu_online_mask,multi_cpu_stop,&mut msdata as *mut _ as *mut _) }
pub unsafe fn stop_machine(fn_: cpu_stop_fn_t, data: *mut core::ffi::c_void, cpus: *const cpumask) -> i32 { cpus_read_lock(); let ret=stop_machine_cpuslocked(fn_,data,cpus); cpus_read_unlock(); ret }

#[cfg(CONFIG_SCHED_SMT)]
pub unsafe fn stop_core_cpuslocked(cpu:u32, fn_:cpu_stop_fn_t, data:*mut core::ffi::c_void)->i32 { let smt_mask=cpu_smt_mask(cpu); let mut msdata=multi_stop_data{fn_,data,num_threads:cpumask_weight(smt_mask),active_cpus:smt_mask,state:multi_stop_state::MULTI_STOP_NONE,thread_ack:core::mem::zeroed()}; lockdep_assert_cpus_held!(); set_state(&mut msdata,multi_stop_state::MULTI_STOP_PREPARE); stop_cpus(smt_mask,multi_cpu_stop,&mut msdata as *mut _ as *mut _) }

pub unsafe fn stop_machine_from_inactive_cpu(fn_:cpu_stop_fn_t,data:*mut core::ffi::c_void,cpus:*const cpumask)->i32 { let mut msdata=multi_stop_data{fn_,data,num_threads:0,active_cpus:cpus,state:multi_stop_state::MULTI_STOP_NONE,thread_ack:core::mem::zeroed()}; let mut done=core::mem::zeroed::<cpu_stop_done>(); BUG_ON!(cpu_active(raw_smp_processor_id())); msdata.num_threads=num_active_cpus()+1; while !mutex_trylock(&mut stop_cpus_mutex) {cpu_relax();} set_state(&mut msdata,multi_stop_state::MULTI_STOP_PREPARE); cpu_stop_init_done(&mut done,num_active_cpus()); queue_stop_cpus_work(cpu_active_mask,multi_cpu_stop,&mut msdata as *mut _ as *mut _,&mut done); let ret=multi_cpu_stop(&mut msdata as *mut _ as *mut _); while !completion_done(&done.completion){cpu_relax();} mutex_unlock(&mut stop_cpus_mutex); if ret!=0 {ret} else {done.ret} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
